pub mod chip8_disassembler {
    use std::str;

    struct OpCode {
        code: u16,
        code_string: &'static str,
        bit_mask: u16,
        mnemonic: &'static str,
    }

    const OP_CODES: [OpCode; 44] = [
        OpCode {
            code: 0x00E0,
            code_string: "00E0",
            bit_mask: 0xFFFF,
            mnemonic: "CLS",
        },
        OpCode {
            code: 0x00EE,
            code_string: "00EE",
            bit_mask: 0x00FF,
            mnemonic: "RET",
        },
        OpCode {
            code: 0x1000,
            code_string: "1NNN",
            bit_mask: 0xF000,
            mnemonic: "JP NNN",
        },
        OpCode {
            code: 0x2000,
            code_string: "2NNN",
            bit_mask: 0xF000,
            mnemonic: "CALL NNN",
        },
        OpCode {
            code: 0x3000,
            code_string: "3XKK",
            bit_mask: 0xF000,
            mnemonic: "SE VX, KK",
        },
        OpCode {
            code: 0x4000,
            code_string: "4XKK",
            bit_mask: 0xF000,
            mnemonic: "SNE VX, KK",
        },
        OpCode {
            code: 0x5000,
            code_string: "5XY0",
            bit_mask: 0xF00F,
            mnemonic: "SE VX, VY",
        },
        OpCode {
            code: 0x6000,
            code_string: "6XKK",
            bit_mask: 0xF000,
            mnemonic: "LD VX, KK",
        },
        OpCode {
            code: 0x7000,
            code_string: "7XKK",
            bit_mask: 0xF000,
            mnemonic: "ADD VX, KK",
        },
        OpCode {
            code: 0x8000,
            code_string: "8XY0",
            bit_mask: 0xF00F,
            mnemonic: "LD VX, VY",
        },
        OpCode {
            code: 0x8001,
            code_string: "8XY1",
            bit_mask: 0xF00F,
            mnemonic: "OR VX, VY",
        },
        OpCode {
            code: 0x8002,
            code_string: "8XY2",
            bit_mask: 0xF00F,
            mnemonic: "AND VX, VY",
        },
        OpCode {
            code: 0x8003,
            code_string: "8XY3",
            bit_mask: 0xF00F,
            mnemonic: "XOR VX, VY",
        },
        OpCode {
            code: 0x8004,
            code_string: "8XY4",
            bit_mask: 0xF00F,
            mnemonic: "ADD VX, VY",
        },
        OpCode {
            code: 0x8005,
            code_string: "8XY5",
            bit_mask: 0xF00F,
            mnemonic: "SUB VX, VY",
        },
        OpCode {
            code: 0x8006,
            code_string: "8XY6",
            bit_mask: 0xF00F,
            mnemonic: "SHR VX",
        },
        OpCode {
            code: 0x8007,
            code_string: "8XY7",
            bit_mask: 0xF00F,
            mnemonic: "SUBN VX, VY",
        },
        OpCode {
            code: 0x800E,
            code_string: "8XYE",
            bit_mask: 0xF00F,
            mnemonic: "SHL VX",
        },
        OpCode {
            code: 0x9000,
            code_string: "9XY0",
            bit_mask: 0xF00F,
            mnemonic: "SNE VX, VY",
        },
        OpCode {
            code: 0xA000,
            code_string: "ANNN",
            bit_mask: 0xF000,
            mnemonic: "LD I, NNN",
        },
        OpCode {
            code: 0xB000,
            code_string: "BNNN",
            bit_mask: 0xF000,
            mnemonic: "JP V0, NNN",
        },
        OpCode {
            code: 0xC000,
            code_string: "CXKK",
            bit_mask: 0xF000,
            mnemonic: "RND VX, KK",
        },
        OpCode {
            code: 0xD000,
            code_string: "DXYN",
            bit_mask: 0xF000,
            mnemonic: "DRW VX, VY, N",
        },
        OpCode {
            code: 0xE09E,
            code_string: "EX9E",
            bit_mask: 0xF0FF,
            mnemonic: "SKP VX",
        },
        OpCode {
            code: 0xE0A1,
            code_string: "EXA1",
            bit_mask: 0xF0FF,
            mnemonic: "SKNP VX",
        },
        OpCode {
            code: 0xF007,
            code_string: "FX07",
            bit_mask: 0xF0FF,
            mnemonic: "LD VX, DT",
        },
        OpCode {
            code: 0xF00A,
            code_string: "FX0A",
            bit_mask: 0xF0FF,
            mnemonic: "LD VX, K",
        },
        OpCode {
            code: 0xF015,
            code_string: "FX15",
            bit_mask: 0xF0FF,
            mnemonic: "LD DT, VX",
        },
        OpCode {
            code: 0xF018,
            code_string: "FX18",
            bit_mask: 0xF0FF,
            mnemonic: "LD ST, VX",
        },
        OpCode {
            code: 0xF01E,
            code_string: "FX1E",
            bit_mask: 0xF0FF,
            mnemonic: "ADD I, VX",
        },
        OpCode {
            code: 0xF029,
            code_string: "FX29",
            bit_mask: 0xF0FF,
            mnemonic: "LD F, VX",
        },
        OpCode {
            code: 0xF033,
            code_string: "FX33",
            bit_mask: 0xF0FF,
            mnemonic: "LD B, VX",
        },
        OpCode {
            code: 0xF055,
            code_string: "FX55",
            bit_mask: 0xF0FF,
            mnemonic: "LD [I], VX",
        },
        OpCode {
            code: 0xF065,
            code_string: "FX65",
            bit_mask: 0xF0FF,
            mnemonic: "LD VX, [I]",
        },
        OpCode {
            code: 0x00C0,
            code_string: "00CN",
            bit_mask: 0xFFF0,
            mnemonic: "SCD N",
        },
        OpCode {
            code: 0x00FB,
            code_string: "00FB",
            bit_mask: 0xFFFF,
            mnemonic: "SCR",
        },
        OpCode {
            code: 0x00FC,
            code_string: "00FC",
            bit_mask: 0xFFFF,
            mnemonic: "SCL",
        },
        OpCode {
            code: 0x00FD,
            code_string: "00FD",
            bit_mask: 0xFFFF,
            mnemonic: "EXIT",
        },
        OpCode {
            code: 0x00FE,
            code_string: "00FE",
            bit_mask: 0xFFFF,
            mnemonic: "LOW",
        },
        OpCode {
            code: 0x00FF,
            code_string: "00FF",
            bit_mask: 0xFFFF,
            mnemonic: "HIGH",
        },
        OpCode {
            code: 0xF030,
            code_string: "FX30",
            bit_mask: 0xF0FF,
            mnemonic: "LD HF, VX",
        },
        OpCode {
            code: 0xF075,
            code_string: "FX75",
            bit_mask: 0xF0FF,
            mnemonic: "LD R, VX",
        },
        OpCode {
            code: 0xF085,
            code_string: "FX85",
            bit_mask: 0xF0FF,
            mnemonic: "LD VX, R",
        },
        OpCode {
            code: 0x0000,
            code_string: "0NNN",
            bit_mask: 0xF000,
            mnemonic: "SYS NNN",
        },
    ];

    fn hex_to_u16(hex: &str) -> u16 {
        u16::from_str_radix(hex, 16).unwrap()
    }

    fn get_op_code_from_hex(code: u16) -> Result<&'static OpCode, &'static str> {
        for op_code in OP_CODES.iter() {
            if op_code.code == code & op_code.bit_mask {
                return Ok(op_code);
            }
        }
        return Err("Invalid OpCode");
    }

    /// Dissasemble a OpCode from a string
    /// # Arguments
    /// * `code` - A string slice that holds the OpCode
    /// # Returns
    /// * A string with the dissasembled OpCode
    /// # Example
    /// ```
    /// let code = "00E0";
    /// let result = dissasemble_op_code_from_str(code).expect("fail");
    /// assert_eq!(result, "CLS");
    /// ```
    pub fn dissasemble_op_code_from_str(code: &str) -> Result<String, &str> {
        let code = hex_to_u16(code);
        return dissasemble_op_code_from_u16(code);
    }

    /// Dissasemble a OpCode from a u16
    /// # Arguments
    /// * `code` - A u16 that holds the OpCode
    /// # Returns
    /// * A string with the dissasembled OpCode
    /// # Example
    /// ```
    /// let code = 0x00E0;
    /// let result = dissasemble_op_code_from_u16(code).expect("fail");
    /// assert_eq!(result, "CLS");
    /// ```
    pub fn dissasemble_op_code_from_u16(code: u16) -> Result<String, &'static str> {
        let op_code = get_op_code_from_hex(code)?;
        let mut ret = String::from(op_code.mnemonic);
        for i in 0..4 {
            let c = op_code.code_string.chars().nth(i).unwrap();
            if c == 'X' || c == 'Y' || c == 'N' || c == 'K' {
                let value = match i {
                    0 => (code & 0xF000) >> 12,
                    1 => (code & 0x0F00) >> 8,
                    2 => (code & 0x00F0) >> 4,
                    3 => code & 0x000F,
                    _ => 0,
                };
                ret = ret.replacen(&c.to_string(), &value.to_string(), 1);
            }
        }
        return Ok(ret);
    }

    /// Dissasemble a OpCode from a slice of bytes
    /// # Arguments
    /// * `bytes` - A slice of bytes that holds the OpCode
    /// # Returns
    /// * A vector of strings with the dissasembled OpCode
    /// # Example
    /// ```
    /// let bytes = [0x00, 0xE0, 0x00, 0xEE, 0x81, 0x22];
    /// let result = dissasemble_op_code_from_bytes(&bytes).expect("fail");
    /// assert_eq!(result[0], "CLS");
    /// assert_eq!(result[1], "RET");
    /// assert_eq!(result[2], "AND V1, V2");
    /// ```
    pub fn dissasemble_op_code_from_bytes(bytes: &[u8]) -> Result<Vec<String>, &str> {
        if bytes.len() % 2 != 0 {
            return Err("Invalid OpCode");
        }
        let mut ret = Vec::new();
        for i in (0..bytes.len()).step_by(2) {
            let code = (bytes[i] as u16) << 8 | bytes[i + 1] as u16;
            let str = dissasemble_op_code_from_u16(code)?;
            ret.push(str);
        }
        return Ok(ret);
    }

    mod tests {
        use super::*;

        #[test]
        fn test_dissasemble_op_code() {
            assert_eq!(dissasemble_op_code_from_str("00E0").expect("fail"), "CLS");
            assert_eq!(dissasemble_op_code_from_str("00EE").expect("fail"), "RET");
            assert_eq!(
                dissasemble_op_code_from_str("1123").expect("fail"),
                "JP 123"
            );
            assert_eq!(
                dissasemble_op_code_from_str("2123").expect("fail"),
                "CALL 123"
            );
            assert_eq!(
                dissasemble_op_code_from_str("3112").expect("fail"),
                "SE V1, 12"
            );
            assert_eq!(
                dissasemble_op_code_from_str("4112").expect("fail"),
                "SNE V1, 12"
            );
            assert_eq!(
                dissasemble_op_code_from_str("5120").expect("fail"),
                "SE V1, V2"
            );
            assert_eq!(
                dissasemble_op_code_from_str("6112").expect("fail"),
                "LD V1, 12"
            );
            assert_eq!(
                dissasemble_op_code_from_str("7112").expect("fail"),
                "ADD V1, 12"
            );
            assert_eq!(
                dissasemble_op_code_from_str("8120").expect("fail"),
                "LD V1, V2"
            );
            assert_eq!(
                dissasemble_op_code_from_str("8121").expect("fail"),
                "OR V1, V2"
            );
            assert_eq!(
                dissasemble_op_code_from_str("8122").expect("fail"),
                "AND V1, V2"
            );
        }
        #[test]
        fn test_dissasemble_op_code_from_bytes() {
            let bytes = [0x00, 0xE0, 0x00, 0xEE, 0x81, 0x22];
            let result = dissasemble_op_code_from_bytes(&bytes).expect("fail");
            assert_eq!(result[0], "CLS");
            assert_eq!(result[1], "RET");
            assert_eq!(result[2], "AND V1, V2");
        }
    }
}
