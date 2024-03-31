struct Chip8 {
    /*
     Memory Map:
     +---------------+= 0xFFF (4095) End of Chip-8 RAM
     |               |
     |               |
     | 0x200 to 0xFFF|
     |     Chip-8    |
     | Program / Data|
     |     Space     |
     |               |
     |               |
     +- - - - - - - -+= 0x600 (1536) Start of ETI 660 Chip-8 programs
     |               |
     |               |
     +---------------+= 0x200 (512) Start of most Chip-8 programs
     | 0x000 to 0x1FF|
     | Reserved for  |
     |  interpreter  |
     +---------------+= 0x000 (0) Start of Chip-8 RAM
    */
    memory: [u8; 4096],
    v: [u8; 16],
    i: u16,
    pc: u16,

    /*
     The display memmory
     *********************
     *(0,0)        (63,0)*
     *                   *
     *(0,31)      (63,31)*
     *********************
    */
    gfx: [u8; 64 * 32],
    extended_gfx_mode: bool,
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    sp: u16,
    /*
     The computers which originally used the Chip-8 Language had a 16-key hexadecimal keypad with the following layout:
     1 2 3 4
     4 5 6 D
     7 8 9 E
     A 0 B F
    */
    key: [u8; 16],
}

const CHIP8_FONTSET: [u8; 240] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
    //Super Chip-8 Font
    0xF0, 0xF0, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0xF0, 0xF0, //0
    0x20, 0x20, 0x60, 0x60, 0x20, 0x20, 0x20, 0x20, 0x70, 0x70, //1
    0xF0, 0xF0, 0x10, 0x10, 0xF0, 0xF0, 0x80, 0x80, 0xF0, 0xF0, //2
    0xF0, 0xF0, 0x10, 0x10, 0xF0, 0xF0, 0x10, 0x10, 0xF0, 0xF0, //3
    0x90, 0x90, 0x90, 0x90, 0xF0, 0xF0, 0x10, 0x10, 0x10, 0x10, //4
    0xF0, 0xF0, 0x80, 0x80, 0xF0, 0xF0, 0x10, 0x10, 0xF0, 0xF0, //5
    0xF0, 0xF0, 0x80, 0x80, 0xF0, 0xF0, 0x90, 0x90, 0xF0, 0xF0, //6
    0xF0, 0xF0, 0x10, 0x10, 0x20, 0x20, 0x40, 0x40, 0x40, 0x40, //7
    0xF0, 0xF0, 0x90, 0x90, 0xF0, 0xF0, 0x90, 0x90, 0xF0, 0xF0, //8
    0xF0, 0xF0, 0x90, 0x90, 0xF0, 0xF0, 0x10, 0x10, 0xF0, 0xF0, //9
    0xF0, 0xF0, 0x90, 0x90, 0xF0, 0xF0, 0x90, 0x90, 0x90, 0x90, //A
    0xE0, 0xE0, 0x90, 0x90, 0xE0, 0xE0, 0x90, 0x90, 0xE0, 0xE0, //B
    0xF0, 0xF0, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0xF0, 0xF0, //C
    0xE0, 0xE0, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0xE0, 0xE0, //D
    0xF0, 0xF0, 0x80, 0x80, 0xF0, 0xF0, 0x80, 0x80, 0xF0, 0xF0, //E
    0xF0, 0xF0, 0x80, 0x80, 0xF0, 0xF0, 0x80, 0x80, 0x80, 0x80, //F
];

impl Chip8 {
    pub fn new() -> Self {
        let mut c = Self {
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: 0x200,
            gfx: [0; 64 * 32],
            extended_gfx_mode: false,
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            key: [0; 16],
        };
        c.memory[..CHIP8_FONTSET.len()].copy_from_slice(&CHIP8_FONTSET);
        return c;
    }

    pub fn load_game(&mut self, game: Vec<u8>) {
        for (i, byte) in game.iter().enumerate() {
            self.memory[i + 0x200] = *byte;
        }
    }

    pub fn load_game_from_file(&mut self, path: &str) -> Result<(), std::io::Error> {
        let game = std::fs::read(path)?;
        self.load_game(game);
        return Ok(());
    }

    pub fn set_key(&mut self, key: u8, state: bool) {
        self.key[key as usize] = state as u8;
    }

    pub fn get_key(&self, key: u8) -> bool {
        return self.key[key as usize] == 1;
    }

    pub fn get_gfx(&self) -> &[u8] {
        return &self.gfx;
    }

    /**********************************************************************************************
     * CHIP-8 has 35 opcodes, which are all two bytes long and stored big-endian.
     * The opcodes are listed below, in hexadecimal and with the following symbols:
     *
     * OPCODE     DISC (Instructions marked with (*) are new in SUPER-CHIP.)
     * --------------------------------------------------------------------------------------------
     * 00CN*    Scroll display N lines down
     * 0NNN        RCA 1802 program at address NNN. Not necessary for most ROMs.
     * 00E0     Clears the screen.
     * 00EE     Returns from a subroutine.
     * 00FB*    Scroll display 4 pixels right
     * 00FC*    Scroll display 4 pixels left
     * 00FD*    Exit CHIP interpreter
     * 00FE*    Disable extended screen mode
     * 00FF*    Enable extended screen mode for full-screen graphics
     * 1NNN     Jumps to address NNN.
     * 2NNN     Calls subroutine at NNN.
     * 3XNN     Skips the next instruction if VX equals NN.
     * 4XNN     Skips the next instruction if VX doesn't equal NN.
     * 5XY0     Skips the next instruction if VX equals VY.
     * 6XNN     Sets VX to NN.
     * 7XNN     Adds NN to VX.
     * 8XY0     Sets VX to the value of VY.
     * 8XY1     Sets VX to VX or VY. (Bitwise OR operation) VF is reset to 0.
     * 8XY2     Sets VX to VX and VY. (Bitwise AND operation) VF is reset to 0.
     * 8XY3     Sets VX to VX xor VY. VF is reset to 0.
     * 8XY4     Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't.
     * 8XY5     VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
     * 8XY6     Shifts VX right by one. VF is set to the value of the least significant bit of VX before the shift.
     * 8XY7     Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
     * 8XYE     Shifts VX left by one. VF is set to the value of the most significant bit of VX before the shift.
     * 9XY0     Skips the next instruction if VX doesn't equal VY.
     * ANNN     Sets I to the address NNN.
     * BNNN     Jumps to the address NNN plus V0.
     * CXNN     Sets VX to the result of a bitwise and operation on a random number and NN.
     * DXYN*    Show N-byte sprite from M(I) at coords (VX,VY), VF :=
     *          collision. If N=0 and extended mode, show 16x16 sprite.
     * EX9E     Skips the next instruction if the key stored in VX is pressed.
     * EXA1     Skips the next instruction if the key stored in VX isn't pressed.
     * FX07     Sets VX to the value of the delay timer.
     * FX0A     A key press is awaited, and then stored in VX. (Blocking Operation. All instruction halted until next key event)
     * FX15     Sets the delay timer to VX.
     * FX18     Sets the sound timer to VX.
     * FX1E     Adds VX to I.[3]
     * FX29     sets I to the location of the sprite for the character in VX. Characters 0-F  are represented by a 4x5 font.
     * FX30*    Point I to 10-byte font sprite for digit VX (0..9)
     * FX33     Stores the binary-coded decimal representation of VX, with the most significant of three digits at the address in I.
     * FX55     Stores V0 to VX (including VX) in memory starting at address I.[4]
     * FX65     Fills V0 to VX (including VX) with values from memory starting at address I.
     * FX75*    Store V0..VX in RPL user flags (X <= 7)
     * FX85*    Read V0..VX from RPL user flags (X <= 7)
     **********************************************************************************************/
    pub fn emulate_cycle(&mut self) {
        let opcode =
            (self.memory[self.pc as usize] as u16) << 8 | self.memory[self.pc as usize + 1] as u16;
        self.pc += 2;
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
        match opcode & 0xF000 {
            0x0000 => {
                if (opcode & 0x00F0) == 0x00C0 {
                    let screen_width = if self.extended_gfx_mode { 128 } else { 64 };
                    let screen_height = if self.extended_gfx_mode { 64 } else { 32 };
                    let n = (opcode & 0x000F) as u16;
                    for y in screen_width..n {
                        let start = y * screen_height;
                        for x in start..start + screen_width {
                            self.gfx[x as usize] = self.gfx[(x - screen_width * n) as usize];
                        }
                    }
                    self.gfx[..(screen_width * n) as usize].fill(0);
                    return;
                }
                match opcode & 0x00FF {
                    0x00E0 => self.gfx = [0; 64 * 32],
                    0x00EE => {
                        self.sp -= 1;
                        self.pc = self.stack[self.sp as usize];
                    }
                    0x00FB => {
                        let screen_width = if self.extended_gfx_mode { 128 } else { 64 };
                        let screen_height = if self.extended_gfx_mode { 64 } else { 32 };
                        for y in 0..screen_height {
                            let start = y * screen_width;
                            for x in start..start + screen_width - 4 {
                                self.gfx[x as usize] = self.gfx[(x + 4) as usize];
                            }
                            self.gfx[start as usize..(start + 4) as usize].fill(0);
                        }
                    }
                    0x00FC => {
                        let screen_width = if self.extended_gfx_mode { 128 } else { 64 };
                        let screen_height = if self.extended_gfx_mode { 64 } else { 32 };
                        for y in 0..screen_height {
                            let start = y * screen_width;
                            for x in start..start + screen_width - 4 {
                                self.gfx[x as usize] = self.gfx[(x + 4) as usize];
                            }
                            let ms = start + screen_width - 4;
                            self.gfx[ms as usize..(ms + 4) as usize].fill(0);
                        }
                    }
                    0x00FD => self.pc = 0x200,
                    0x00FE => self.extended_gfx_mode = false,
                    0x00FF => self.extended_gfx_mode = true,
                    _ => panic!("Unknown opcode: {:#X}", opcode),
                }
            }
            0x1000 => self.pc = opcode & 0x0FFF,
            0x2000 => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = opcode & 0x0FFF;
            }
            0x3000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let byte = (opcode & 0x00FF) as u8;
                if self.v[x] == byte {
                    self.pc += 2;
                }
            }
            0x4000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let byte = (opcode & 0x00FF) as u8;
                if self.v[x] != byte {
                    self.pc += 2;
                }
            }
            0x5000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                if self.v[x] == self.v[y] {
                    self.pc += 2;
                }
            }
            0x6000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let byte = (opcode & 0x00FF) as u8;
                self.v[x] = byte;
            }
            0x7000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let byte = (opcode & 0x00FF) as u8;
                self.v[x] = self.v[x].wrapping_add(byte);
            }
            0x8000 => match opcode & 0x000F {
                0x0000 => {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    let y = ((opcode & 0x00F0) >> 4) as usize;
                    self.v[x] = self.v[y];
                }
                0x0001 => {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    let y = ((opcode & 0x00F0) >> 4) as usize;
                    self.v[x] |= self.v[y];
                    self.v[0xF] = 0;
                }
                0x0002 => {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    let y = ((opcode & 0x00F0) >> 4) as usize;
                    self.v[x] &= self.v[y];
                    self.v[0xF] = 0;
                }
                0x0003 => {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    let y = ((opcode & 0x00F0) >> 4) as usize;
                    self.v[x] ^= self.v[y];
                }
                0x0004 => {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    let y = ((opcode & 0x00F0) >> 4) as usize;
                    let sum = self.v[x] as u16 + self.v[y] as u16;
                    self.v[0xF] = if sum > 0xFF { 1 } else { 0 };
                    self.v[x] = sum as u8;
                }
                0x0005 => {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    let y = ((opcode & 0x00F0) >> 4) as usize;
                    self.v[0xF] = if self.v[x] > self.v[y] { 1 } else { 0 };
                    self.v[x] = self.v[x].wrapping_sub(self.v[y]);
                }
                0x0006 => {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    self.v[0xF] = self.v[x] & 0x1;
                    self.v[x] >>= 1;
                }
                0x0007 => {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    let y = ((opcode & 0x00F0) >> 4) as usize;
                    self.v[0xF] = if self.v[x] > self.v[y] { 0 } else { 1 };
                    self.v[x] = self.v[y].wrapping_sub(self.v[x]);
                }
                0x000E => {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    self.v[0xF] = (self.v[x] & 0x80) >> 7;
                    self.v[x] <<= 1;
                }
                _ => panic!("Unknown opcode: {:#X}", opcode),
            },
            0x9000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                if self.v[x] != self.v[y] {
                    self.pc += 2;
                }
            }
            0xA000 => self.i = opcode & 0x0FFF,
            0xB000 => self.pc = (opcode & 0x0FFF) + self.v[0] as u16,
            0xC000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let byte = (opcode & 0x00FF) as u8;
                self.v[x] = byte & rand::random::<u8>();
            }
            0xD000 => {
                let x = self.v[((opcode & 0x0F00) >> 8) as usize] as u16;
                let y = self.v[((opcode & 0x00F0) >> 4) as usize] as u16;
                let height = opcode & 0x000F;
                self.v[0xF] = 0;
                for yline in 0..height {
                    let pixel = self.memory[self.i as usize + yline as usize];
                    for xline in 0..8 as u16 {
                        if (pixel & (0x80 >> xline)) != 0 {
                            if self.gfx[(x + xline + ((y + yline) * 64)) as usize] == 1 {
                                self.v[0xF] = 1;
                            }
                            self.gfx[(x + xline + ((y + yline) * 64)) as usize] ^= 1;
                        }
                    }
                }
            }
            0xE000 => match opcode & 0x00FF {
                0x009E => {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    if self.key[self.v[x] as usize] == 1 {
                        self.pc += 2;
                    }
                }
                0x00A1 => {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    if self.key[self.v[x] as usize] == 0 {
                        self.pc += 2;
                    }
                }
                _ => panic!("Unknown opcode: {:#X}", opcode),
            },
            0xF000 => match opcode & 0x00FF {
                0x0007 => {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    self.v[x] = self.delay_timer;
                }
                0x000A => {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    let mut key_press = false;
                    for i in 0..16 {
                        if self.key[i] != 0 {
                            self.v[x] = i as u8;
                            key_press = true;
                        }
                    }
                    if !key_press {
                        self.pc -= 2;
                    }
                }
                0x0015 => {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    self.delay_timer = self.v[x];
                }
                0x0018 => {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    self.sound_timer = self.v[x];
                }
                0x001E => {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    self.i += self.v[x] as u16;
                }
                0x0029 => {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    self.i = self.v[x] as u16 * 5;
                }
                0x0030 => {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    self.i = 0x50 + self.v[x] as u16 * 10;
                }
                0x0033 => {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    self.memory[self.i as usize] = self.v[x] / 100;
                    self.memory[self.i as usize + 1] = (self.v[x] / 10) % 10;
                    self.memory[self.i as usize + 2] = self.v[x] % 10;
                }
                0x0055 => {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    for i in 0..=x {
                        self.memory[self.i as usize + i] = self.v[i];
                    }
                }
                0x0065 => {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    for i in 0..=x {
                        self.v[i] = self.memory[self.i as usize + i];
                    }
                }
                0x0075 => {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    for i in 0..=x {
                        self.memory[0x5F0 + i] = self.v[i];
                    }
                }
                0x0085 => {
                    let x = ((opcode & 0x0F00) >> 8) as usize;
                    for i in 0..=x {
                        self.v[i] = self.memory[0x5F0 + i];
                    }
                }
                _ => panic!("Unknown opcode: {:#X}", opcode),
            },
            _ => panic!("Unknown opcode: {:#X}", opcode),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let c = Chip8::new();
        assert_eq!(c.memory.len(), 4096);
    }
    #[test]
    fn load_game() {
        let mut c = Chip8::new();
        c.load_game(vec![0x12, 0x34]);
        assert_eq!(c.memory[0x200], 0x12);
        assert_eq!(c.memory[0x201], 0x34);
    }
    #[test]
    fn test_key() {
        let mut c = Chip8::new();
        c.set_key(0, true);
        assert_eq!(c.get_key(0), true);
        assert_eq!(c.get_key(1), false);
    }
    #[test]
    fn test_opcode_00cn() {
        let mut c = Chip8::new();
        let opcode = 0x00C4;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.gfx = [1; 64 * 32];
        c.emulate_cycle();
        let mut exp = [0; 64 * 32];
        exp[64 * 4..].fill(1);
        assert_eq!(c.gfx, exp);
        c.gfx = [0; 64 * 32];
        c.pc = 0x200;
        c.emulate_cycle();
        assert_eq!(c.gfx, [0; 64 * 32]);
    }
    #[test]
    fn test_opcode_00e0() {
        let mut c = Chip8::new();
        let opcode = 0x00E0;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.gfx = [1; 64 * 32];
        c.emulate_cycle();
        assert_eq!(c.gfx, [0; 64 * 32]);
    }
    #[test]
    fn test_opcode_00ee() {
        let mut c = Chip8::new();
        let opcode = 0x00EE;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.stack[0] = 0x234;
        c.sp = 1;
        c.emulate_cycle();
        assert_eq!(c.pc, 0x234);
        assert_eq!(c.sp, 0);
    }
    #[test]
    fn test_opcode_00fb() {
        let mut c = Chip8::new();
        let opcode = 0x00FB;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.gfx = [1; 64 * 32];
        c.emulate_cycle();
        let mut ex = [1; 64 * 32];
        for y in 0..32 {
            let start = y * 64;
            ex[start as usize..(start + 4) as usize].fill(0);
        }
        assert_eq!(c.gfx, ex);
    }
    #[test]
    fn test_opcode_00fc() {
        let mut c = Chip8::new();
        let opcode = 0x00FC;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.gfx = [1; 64 * 32];
        c.emulate_cycle();
        let mut ex = [1; 64 * 32];
        for y in 0..32 {
            let start = y * 64;
            ex[(start + 60) as usize..(start + 64) as usize].fill(0);
        }
        assert_eq!(c.gfx, ex);
    }
    #[test]
    fn test_opcode_00fd() {
        let mut c = Chip8::new();
        let opcode = 0x00FD;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.pc = 0x200;
        c.emulate_cycle();
        assert_eq!(c.pc, 0x200);
    }
    #[test]
    fn test_opcode_00fe() {
        let mut c = Chip8::new();
        let opcode = 0x00FE;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.extended_gfx_mode = true;
        c.emulate_cycle();
        assert_eq!(c.extended_gfx_mode, false);
    }
    #[test]
    fn test_opcode_00ff() {
        let mut c = Chip8::new();
        let opcode = 0x00FF;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.extended_gfx_mode = false;
        c.emulate_cycle();
        assert_eq!(c.extended_gfx_mode, true);
    }
    #[test]
    fn test_opcode_1nnn() {
        let mut c = Chip8::new();
        let opccode = 0x1234;
        c.memory[0x200] = (opccode >> 8) as u8;
        c.memory[0x201] = (opccode & 0xFF) as u8;
        c.emulate_cycle();
        assert_eq!(c.pc, 0x234);
    }
    #[test]
    fn test_opcode_2nnn() {
        let mut c = Chip8::new();
        let opcode = 0x2234;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.emulate_cycle();
        assert_eq!(c.pc, 0x234);
        assert_eq!(c.stack[0], 0x202);
        assert_eq!(c.sp, 1);
    }
    #[test]
    fn test_opcode_3xkk() {
        let mut c = Chip8::new();
        c.v[0] = 0x11;
        let opcode = 0x3012;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.emulate_cycle();
        assert_eq!(c.pc, 0x202);
        c.v[0] = 0x12;
        c.pc = 0x200;
        c.emulate_cycle();
        assert_eq!(c.pc, 0x204);
    }
    #[test]
    fn test_opcode_4xkk() {
        let mut c = Chip8::new();
        c.v[0] = 0x12;
        let opcode = 0x4012;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.emulate_cycle();
        assert_eq!(c.pc, 0x202);
        c.pc = 0x200;
        c.v[0] = 0x34;
        c.emulate_cycle();
        assert_eq!(c.pc, 0x204);
    }
    #[test]
    fn test_opcode_5xy0() {
        let mut c = Chip8::new();
        c.v[0] = 0x12;
        c.v[1] = 0x12;
        let opcode = 0x5010;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.emulate_cycle();
        assert_eq!(c.pc, 0x204);
        c.pc = 0x200;
        c.v[1] = 0x34;
        c.emulate_cycle();
        assert_eq!(c.pc, 0x202);
    }
    #[test]
    fn test_opcode_6xkk() {
        let mut c = Chip8::new();
        let opcode = 0x6012;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.emulate_cycle();
        assert_eq!(c.v[0], 0x12);
    }
    #[test]
    fn test_opcode_7xkk() {
        let mut c = Chip8::new();
        c.v[0] = 0x12;
        let opcode = 0x7012;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.emulate_cycle();
        assert_eq!(c.v[0], 0x24);
    }
    #[test]
    fn test_opcode_8xy0() {
        let mut c = Chip8::new();
        c.v[1] = 0x12;
        let opcode = 0x8010;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.emulate_cycle();
        assert_eq!(c.v[0], 0x12);
    }
    #[test]
    fn test_opcode_8xy1() {
        let mut c = Chip8::new();
        c.v[0] = 0x12;
        c.v[1] = 0x34;
        c.v[0xF] = 1;
        let opcode = 0x8011;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.emulate_cycle();
        assert_eq!(c.v[0], 0x34 | 0x12);
        assert_eq!(c.v[0xF], 0);
    }
    #[test]
    fn test_opcode_8xy2() {
        let mut c = Chip8::new();
        c.v[0] = 0x12;
        c.v[1] = 0x34;
        c.v[0xF] = 1;
        let opcode = 0x8012;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.emulate_cycle();
        assert_eq!(c.v[0], 0x12 & 0x34);
        assert_eq!(c.v[0xF], 0);
    }
    #[test]
    fn test_opcode_8xy3() {
        let mut c = Chip8::new();
        c.v[0] = 0x12;
        c.v[1] = 0x34;
        let opcode = 0x8013;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.emulate_cycle();
        assert_eq!(c.v[0], 0x26);
    }
    #[test]
    fn test_opcode_8xy4() {
        let mut c = Chip8::new();
        c.v[0] = 0x12;
        c.v[1] = 0x34;
        let opcode = 0x8014;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.emulate_cycle();
        assert_eq!(c.v[0], 0x46);
        assert_eq!(c.v[0xF], 0);
        c.v[0] = 0xFF;
        c.v[1] = 0x01;
        c.pc = 0x200;
        c.emulate_cycle();
        assert_eq!(c.v[0], 0x00);
        assert_eq!(c.v[0xF], 1);
    }
    #[test]
    fn test_opcode_8xy5() {
        let mut c = Chip8::new();
        c.v[0] = 0x34;
        c.v[1] = 0x12;
        let opcode = 0x8015;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.emulate_cycle();
        assert_eq!(c.v[0], 0x22);
        assert_eq!(c.v[0xF], 1);
        c.v[0] = 0x01;
        c.v[1] = 0x02;
        c.pc = 0x200;
        c.emulate_cycle();
        assert_eq!(c.v[0], 0xFF);
        assert_eq!(c.v[0xF], 0);
    }
    #[test]
    fn test_opcode_8xy6() {
        let mut c = Chip8::new();
        c.v[0] = 0b00001010;
        let opcode = 0x8006;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.emulate_cycle();
        assert_eq!(c.v[0], 0b0101);
        assert_eq!(c.v[0xF], 0);
        c.v[0] = 0b00001011;
        c.pc = 0x200;
        c.emulate_cycle();
        assert_eq!(c.v[0], 0b0101);
        assert_eq!(c.v[0xF], 1);
    }
    #[test]
    fn test_opcode_8xy7() {
        let mut c = Chip8::new();
        c.v[0] = 0x34;
        c.v[1] = 0x12;
        let opcode = 0x8017;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.emulate_cycle();
        assert_eq!(c.v[0], 0xDE);
        assert_eq!(c.v[0xF], 0);
        c.v[0] = 0x01;
        c.v[1] = 0x02;
        c.pc = 0x200;
        c.emulate_cycle();
        assert_eq!(c.v[0], 0x01);
        assert_eq!(c.v[0xF], 1);
    }
    #[test]
    fn test_opcode_8xye() {
        let mut c = Chip8::new();
        c.v[0] = 0b10100000;
        let opcode = 0x800E;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.emulate_cycle();
        assert_eq!(c.v[0], 0b01000000);
        assert_eq!(c.v[0xF], 1);
        c.v[0] = 0b00110000;
        c.pc = 0x200;
        c.emulate_cycle();
        assert_eq!(c.v[0], 0b01100000);
        assert_eq!(c.v[0xF], 0);
    }
    #[test]
    fn test_opcode_9xy0() {
        let mut c = Chip8::new();
        c.v[0] = 0x12;
        c.v[1] = 0x12;
        let opcode = 0x9010;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.emulate_cycle();
        assert_eq!(c.pc, 0x202);
        c.pc = 0x200;
        c.v[1] = 0x34;
        c.emulate_cycle();
        assert_eq!(c.pc, 0x204);
    }
    #[test]
    fn test_opcode_annn() {
        let mut c = Chip8::new();
        let opcode = 0xA234;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.emulate_cycle();
        assert_eq!(c.i, 0x234);
    }
    #[test]
    fn test_opcode_bnnn() {
        let mut c = Chip8::new();
        c.v[0] = 0x12;
        let opcode = 0xB234;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.emulate_cycle();
        assert_eq!(c.pc, 0x246);
    }
    #[test]
    fn test_opcode_cxkk() {
        let mut c = Chip8::new();
        let opcode = 0xC012;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.emulate_cycle();
        //assert_ne!(c.v[0], 0x00);
    }
    #[test]
    fn test_opcode_dxyn() {
        let mut c = Chip8::new();
        c.i = 0x205;
        c.memory[0x205] = 0b11110000;
        let opcode = 0xD001;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.v[0] = 0;
        c.v[1] = 0;
        c.emulate_cycle();
        assert_eq!(c.v[0xF], 0);
        assert_eq!(c.gfx[0], 1);
        assert_eq!(c.gfx[1], 1);
        assert_eq!(c.gfx[2], 1);
        assert_eq!(c.gfx[3], 1);
        assert_eq!(c.gfx[4], 0);
        assert_eq!(c.gfx[5], 0);
        assert_eq!(c.gfx[6], 0);
        assert_eq!(c.gfx[7], 0);
    }
    #[test]
    fn test_opcode_ex9e() {
        let mut c = Chip8::new();
        c.v[2] = 3;
        c.key[3] = 1;
        let opcode = 0xE29E;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.emulate_cycle();
        assert_eq!(c.pc, 0x204);
        c.pc = 0x200;
        c.v[2] = 3;
        c.key[3] = 0;
        c.emulate_cycle();
        assert_eq!(c.pc, 0x202);
    }
    #[test]
    fn test_opcode_exa1() {
        let mut c = Chip8::new();
        c.v[0] = 0;
        c.key[0] = 1;
        let opcode = 0xE0A1;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.emulate_cycle();
        assert_eq!(c.pc, 0x202);
        c.pc = 0x200;
        c.v[0] = 0;
        c.key[0] = 0;
        c.emulate_cycle();
        assert_eq!(c.pc, 0x204);
    }
    #[test]
    fn test_opcode_fx07() {
        let mut c = Chip8::new();
        c.delay_timer = 0x12;
        let opcode = 0xF007;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.emulate_cycle();
        assert_eq!(c.v[0], 0x11);
    }
    #[test]
    fn test_opcode_fx0a() {
        let mut c = Chip8::new();
        c.key[0] = 1;
        let opcode = 0xF00A;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.emulate_cycle();
        assert_eq!(c.v[0], 0);
        c.pc = 0x200;
        c.emulate_cycle();
        assert_eq!(c.v[0], 0);
        c.key[0] = 0;
        c.pc = 0x200;
        c.emulate_cycle();
        assert_eq!(c.v[0], 0);
        c.key[0] = 1;
        c.pc = 0x200;
        c.emulate_cycle();
        assert_eq!(c.v[0], 0);
    }
    #[test]
    fn test_opcode_fx15() {
        let mut c = Chip8::new();
        c.v[0] = 0x12;
        let opcode = 0xF015;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.emulate_cycle();
        assert_eq!(c.delay_timer, 0x12);
    }
    #[test]
    fn test_opcode_fx18() {
        let mut c = Chip8::new();
        c.v[0] = 0x12;
        let opcode = 0xF018;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.emulate_cycle();
        assert_eq!(c.sound_timer, 0x12);
    }
    #[test]
    fn test_opcode_fx1e() {
        let mut c = Chip8::new();
        c.v[0] = 0x12;
        c.i = 0x34;
        let opcode = 0xF01E;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.emulate_cycle();
        assert_eq!(c.i, 0x46);
    }
    #[test]
    fn test_opcode_fx29() {
        let mut c = Chip8::new();
        c.v[0] = 0x5;
        let opcode = 0xF029;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.emulate_cycle();
        assert_eq!(c.i, 0x19);
    }
    #[test]
    fn test_opcode_fx30() {
        let mut c = Chip8::new();
        c.v[0] = 0x2;
        let opcode = 0xF030;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.emulate_cycle();
        assert_eq!(c.i, 0x64);
    }
    #[test]
    fn test_opcode_fx33() {
        let mut c = Chip8::new();
        c.v[0] = 123;
        let opcode = 0xF033;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.i = 0x200;
        c.emulate_cycle();
        assert_eq!(c.memory[0x200], 1);
        assert_eq!(c.memory[0x201], 2);
        assert_eq!(c.memory[0x202], 3);
    }
    #[test]
    fn test_opcode_fx55() {
        let mut c = Chip8::new();
        c.v[0] = 0x12;
        c.v[1] = 0x34;
        c.v[2] = 0x56;
        let opcode = 0xF255;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.i = 0x200;
        c.emulate_cycle();
        assert_eq!(c.memory[0x200], 0x12);
        assert_eq!(c.memory[0x201], 0x34);
        assert_eq!(c.memory[0x202], 0x56);
    }
    #[test]
    fn test_opcode_fx65() {
        let mut c = Chip8::new();
        c.memory[0x205] = 0x12;
        c.memory[0x206] = 0x34;
        c.memory[0x207] = 0x56;
        let opcode = 0xF265;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.i = 0x205;
        c.emulate_cycle();
        assert_eq!(c.v[0], 0x12);
        assert_eq!(c.v[1], 0x34);
        assert_eq!(c.v[2], 0x56);
    }
    #[test]
    fn test_opcode_fx75() {
        let mut c = Chip8::new();
        c.v[0] = 0x12;
        c.v[1] = 0x34;
        c.v[2] = 0x56;
        let opcode = 0xF275;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.i = 0x5F0;
        c.emulate_cycle();
        assert_eq!(c.memory[0x5F0], 0x12);
        assert_eq!(c.memory[0x5F1], 0x34);
        assert_eq!(c.memory[0x5F2], 0x56);
    }
    #[test]
    fn test_opcode_fx85() {
        let mut c = Chip8::new();
        c.memory[0x5F0] = 0x12;
        c.memory[0x5F1] = 0x34;
        c.memory[0x5F2] = 0x56;
        let opcode = 0xF285;
        c.memory[0x200] = (opcode >> 8) as u8;
        c.memory[0x201] = (opcode & 0xFF) as u8;
        c.i = 0x5F0;
        c.emulate_cycle();
        assert_eq!(c.v[0], 0x12);
        assert_eq!(c.v[1], 0x34);
        assert_eq!(c.v[2], 0x56);
    }
    #[test]
    fn test_test_rom() {
        let mut c = Chip8::new();
        c.load_game_from_file("c8_test.c8")
            .expect("Failed to load game");
        for _i in 0..500 {
            c.emulate_cycle();
        }
        for y in 0..32 {
            for x in 0..64 {
                print!("{}", c.gfx[y * 64 + x]);
            }
            println!();
        }
    }
}
