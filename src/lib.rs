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
            0x0000 => match opcode & 0x00FF {
                0x00E0 => self.gfx = [0; 64 * 32],
                0x00EE => {
                    self.sp -= 1;
                    self.pc = self.stack[self.sp as usize];
                }
                _ => panic!("Unknown opcode: {:#X}", opcode),
            },
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
}
