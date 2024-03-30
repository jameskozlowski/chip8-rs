struct Chip8 {
    memory: [u8; 4096],
    v: [u8; 16],
    i: u16,
    pc: u16,
    gfx: [u8; 64 * 32],
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    sp: u16,
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

    pub fn emulate_cycle(&mut self) {
        let opcode =
            (self.memory[self.pc as usize] as u16) << 8 | self.memory[self.pc as usize + 1] as u16;
        self.pc += 2;
        match opcode & 0xF000 {
            0x0000 => {
                if (opcode & 0x00F0) == 0x00C0 {
                    panic!("opcode not implemented: {:#X}", opcode)
                }
                match opcode & 0x00FF {
                    0x00E0 => self.gfx = [0; 64 * 32],
                    0x00EE => {
                        self.sp -= 1;
                        self.pc = self.stack[self.sp as usize];
                    }
                    0x00FB => panic!("opcode not implemented: {:#X}", opcode),
                    0x00FC => panic!("opcode not implemented: {:#X}", opcode),
                    0x00FD => panic!("opcode not implemented: {:#X}", opcode),
                    0x00FE => panic!("opcode not implemented: {:#X}", opcode),
                    0x00FF => panic!("opcode not implemented: {:#X}", opcode),
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
                    self.v[0xF] = if self.v[x] > self.v[y] { 1 } else { 0 };
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
    fn test_opcode_00e0() {
        let mut c = Chip8::new();
        c.memory[0x200] = 0x00;
        c.memory[0x201] = 0xE0;
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
        assert_eq!(c.v[0xF], 1);
        c.v[0] = 0x01;
        c.v[1] = 0x02;
        c.pc = 0x200;
        c.emulate_cycle();
        assert_eq!(c.v[0], 0x01);
        assert_eq!(c.v[0xF], 0);
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
        assert_ne!(c.v[0], 0x00);
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
        assert_eq!(c.v[0], 0x12);
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
}
