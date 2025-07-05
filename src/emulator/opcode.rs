// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025-present chip8 emulator project and contributors

//! CHIP-8 opcode related declarations module.

use crate::emulator::disasm::Decodable;

/// CHIP-8 opcode struct.
pub struct OpCode {
    /// Opcode raw bytes.
    pub raw: u16,
    /// Opcode class.
    pub class: u8,
    /// Memory address from opcode.
    pub addr: u16,
    /// First register Vx from opcode.
    pub reg_x: u8,
    /// Second register Vy from opcode.
    pub reg_y: u8,
    /// Byte from opcode.
    pub byte: u8,
    /// Nibble from opcode.
    pub nibble: u8,
}

impl OpCode {
    /// Construct new `OpCode` object.
    ///
    /// # Parameters
    /// - `raw` - given opcode raw bytes.
    ///
    /// # Returns
    /// - New `OpCode` object.
    pub fn new(raw: u16) -> Self {
        let addr = raw & 0x0FFF;
        let class = ((raw & 0xF000) >> 12) as u8;
        let reg_x = ((raw & 0x0F00) >> 8) as u8;
        let reg_y = ((raw & 0x00F0) >> 4) as u8;
        let byte = (raw & 0x00FF) as u8;
        let nibble = (raw & 0x000F) as u8;

        Self {
            raw,
            addr,
            class,
            reg_x,
            reg_y,
            byte,
            nibble,
        }
    }

    /// Handle unknown opcode.
    ///
    /// # Returns
    /// - Unknown opcode assembly mnemonic.
    #[inline(always)]
    pub fn unknown(&self) -> String {
        format!("UNKNOWN: {:04X}", self.raw)
    }

    /// Get 0xxx opcode class mnemonic.
    ///
    /// # Returns
    /// - Opcode assembly mnemonic string representation.
    fn decode_0xxx(&self) -> String {
        let addr = self.addr;

        match self.raw {
            0x00E0 => "CLS".to_string(),
            0x00EE => "RET".to_string(),
            _ => format!("SYS {addr:#03X}"),
        }
    }

    /// Get nnn opcode class mnemonic.
    ///
    /// # Returns
    /// - Opcode assembly mnemonic string representation.
    fn decode_nnn(&self) -> String {
        let addr = self.addr;

        match self.class {
            0x1 => format!("JP {addr:#03X}"),
            0x2 => format!("CALL {addr:#03X}"),
            0xA => format!("LD I, {addr:#03X}"),
            0xB => format!("JP V0, {addr:#03X}"),
            _ => self.unknown(),
        }
    }

    /// Get xkk opcode class mnemonic.
    ///
    /// # Returns
    /// - Opcode assembly mnemonic string representation.
    fn decode_xkk(&self) -> String {
        let reg_x = self.reg_x;
        let byte = self.byte;

        match self.class {
            0x3 => format!("SE V{reg_x}, {byte:#02X}"),
            0x4 => format!("SNE V{reg_x}, {byte:#02X}"),
            0x6 => format!("LD V{reg_x}, {byte:#02X}"),
            0x7 => format!("ADD V{reg_x}, {byte:#02X}"),
            0xC => format!("RND V{reg_x}, {byte:#02X}"),
            _ => self.unknown(),
        }
    }

    /// Get xy opcode class mnemonic.
    ///
    /// # Returns
    /// - Opcode assembly mnemonic string representation.
    fn decode_xy(&self) -> String {
        let reg_x = self.reg_x;
        let reg_y = self.reg_y;
        let nibble = self.nibble;

        match self.class {
            0x5 => match nibble {
                0x0 => format!("SE V{reg_x}, V{reg_y}"),
                _ => self.unknown(),
            },
            0x8 => match nibble {
                0x0 => format!("LD V{reg_x}, V{reg_y}"),
                0x1 => format!("OR V{reg_x}, V{reg_y}"),
                0x2 => format!("AND V{reg_x}, V{reg_y}"),
                0x3 => format!("XOR V{reg_x}, V{reg_y}"),
                0x4 => format!("ADD V{reg_x}, V{reg_y}"),
                0x5 => format!("SUB V{reg_x}, V{reg_y}"),
                0x6 => format!("SHR V{reg_x} {{, V{reg_y}}}"),
                0x7 => format!("SUBN V{reg_x}, V{reg_y}"),
                0xE => format!("SHL V{reg_x} {{, V{reg_y}}}"),
                _ => self.unknown(),
            },
            0x9 => match nibble {
                0x0 => format!("SNE V{reg_x}, V{reg_y}"),
                _ => self.unknown(),
            },
            0xD => format!("DRW V{reg_x}, V{reg_y}, {nibble:#02X}"),
            _ => self.unknown(),
        }
    }

    /// Get Ex opcode class mnemonic.
    ///
    /// # Returns
    /// - Opcode assembly mnemonic string representation.
    fn decode_ex(&self) -> String {
        let reg_x = self.reg_x;

        match self.byte {
            0x9E => format!("SKP V{reg_x}"),
            0xA1 => format!("SKNP V{reg_x}"),
            _ => self.unknown(),
        }
    }

    /// Get Fx opcode class mnemonic.
    ///
    /// # Returns
    /// - Opcode assembly mnemonic string representation.
    fn decode_fx(&self) -> String {
        let reg_x = self.reg_x;

        match self.byte {
            0x07 => format!("LD V{reg_x}, DT"),
            0x0A => format!("LD V{reg_x}, K"),
            0x15 => format!("LD DT, V{reg_x}"),
            0x18 => format!("LD ST, V{reg_x}"),
            0x1E => format!("ADD I, V{reg_x}"),
            0x29 => format!("LD F, V{reg_x}"),
            0x33 => format!("LD B, V{reg_x}"),
            0x55 => format!("LD [I], V{reg_x}"),
            0x65 => format!("LD V{reg_x}, [I]"),
            _ => self.unknown(),
        }
    }
}

impl Decodable for OpCode {
    /// Get opcode mnemonic.
    ///
    /// # Returns
    /// - Opcode assembly mnemonic string representation.
    fn decode(&self) -> String {
        match self.class {
            0x0 => self.decode_0xxx(),
            0x1 => self.decode_nnn(),
            0x2 => self.decode_nnn(),
            0x3 => self.decode_xkk(),
            0x4 => self.decode_xkk(),
            0x5 => self.decode_xy(),
            0x6 => self.decode_xkk(),
            0x7 => self.decode_xkk(),
            0x8 => self.decode_xy(),
            0x9 => self.decode_xy(),
            0xA => self.decode_nnn(),
            0xB => self.decode_nnn(),
            0xC => self.decode_xkk(),
            0xD => self.decode_xy(),
            0xE => self.decode_ex(),
            0xF => self.decode_fx(),
            _ => self.unknown(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_class_extraction() {
        let class = OpCode::new(0x0123).class;
        assert_eq!(0x0, class);

        let class = OpCode::new(0x6123).class;
        assert_eq!(0x6, class);

        let class = OpCode::new(0xF123).class;
        assert_eq!(0xF, class);
    }

    #[test]
    fn test_addr_extraction() {
        let addr = OpCode::new(0x0123).addr;
        assert_eq!(0x123, addr);

        let addr = OpCode::new(0x0000).addr;
        assert_eq!(0x000, addr);

        let addr = OpCode::new(0xDEAD).addr;
        assert_eq!(0xEAD, addr);
    }

    #[test]
    fn test_reg_x_extraction() {
        let reg_x = OpCode::new(0x0123).reg_x;
        assert_eq!(0x1, reg_x);

        let reg_x = OpCode::new(0x0000).reg_x;
        assert_eq!(0x0, reg_x);

        let reg_x = OpCode::new(0xDEAD).reg_x;
        assert_eq!(0xE, reg_x);
    }

    #[test]
    fn test_reg_y_extraction() {
        let reg_y = OpCode::new(0x0123).reg_y;
        assert_eq!(0x2, reg_y);

        let reg_y = OpCode::new(0x0000).reg_y;
        assert_eq!(0x0, reg_y);

        let reg_y = OpCode::new(0xDEAD).reg_y;
        assert_eq!(0xA, reg_y);
    }

    #[test]
    fn test_byte_extraction() {
        let byte = OpCode::new(0x0123).byte;
        assert_eq!(0x23, byte);

        let byte = OpCode::new(0x0000).byte;
        assert_eq!(0x00, byte);

        let byte = OpCode::new(0xDEAD).byte;
        assert_eq!(0xAD, byte);
    }

    #[test]
    fn test_nibble_extraction() {
        let nibble = OpCode::new(0x0123).nibble;
        assert_eq!(0x3, nibble);

        let nibble = OpCode::new(0x6456).nibble;
        assert_eq!(0x6, nibble);

        let nibble = OpCode::new(0xF789).nibble;
        assert_eq!(0x9, nibble);
    }

    #[test]
    fn test_decode_0xxx() {
        let disasm_str = OpCode::new(0x00E0).decode();
        assert_eq!("CLS", disasm_str);

        let disasm_str = OpCode::new(0x00EE).decode();
        assert_eq!("RET", disasm_str);

        let disasm_str = OpCode::new(0x0123).decode();
        assert_eq!("SYS 123", disasm_str);
    }

    #[test]
    fn test_decode_nnn() {
        let disasm_str = OpCode::new(0x1123).decode();
        assert_eq!("JP 123", disasm_str);

        let disasm_str = OpCode::new(0x2123).decode();
        assert_eq!("CALL 123", disasm_str);

        let disasm_str = OpCode::new(0xA123).decode();
        assert_eq!("LD I, 123", disasm_str);

        let disasm_str = OpCode::new(0xB123).decode();
        assert_eq!("JP V0, 123", disasm_str);
    }

    #[test]
    fn test_decode_xkk() {
        let disasm_str = OpCode::new(0x3123).decode();
        assert_eq!("SE V1, 23", disasm_str);

        let disasm_str = OpCode::new(0x4223).decode();
        assert_eq!("SNE V2, 23", disasm_str);

        let disasm_str = OpCode::new(0x6323).decode();
        assert_eq!("LD V3, 23", disasm_str);

        let disasm_str = OpCode::new(0x7423).decode();
        assert_eq!("ADD V4, 23", disasm_str);

        let disasm_str = OpCode::new(0xC523).decode();
        assert_eq!("RND V5, 23", disasm_str);
    }

    #[test]
    fn test_decode_xy_correct() {
        let disasm_str = OpCode::new(0x5120).decode();
        assert_eq!("SE V1, V2", disasm_str);

        let disasm_str = OpCode::new(0x8120).decode();
        assert_eq!("LD V1, V2", disasm_str);

        let disasm_str = OpCode::new(0x8121).decode();
        assert_eq!("OR V1, V2", disasm_str);

        let disasm_str = OpCode::new(0x8122).decode();
        assert_eq!("AND V1, V2", disasm_str);

        let disasm_str = OpCode::new(0x8123).decode();
        assert_eq!("XOR V1, V2", disasm_str);

        let disasm_str = OpCode::new(0x8124).decode();
        assert_eq!("ADD V1, V2", disasm_str);

        let disasm_str = OpCode::new(0x8125).decode();
        assert_eq!("SUB V1, V2", disasm_str);

        let disasm_str = OpCode::new(0x8126).decode();
        assert_eq!("SHR V1 {, V2}", disasm_str);

        let disasm_str = OpCode::new(0x8127).decode();
        assert_eq!("SUBN V1, V2", disasm_str);

        let disasm_str = OpCode::new(0x812E).decode();
        assert_eq!("SHL V1 {, V2}", disasm_str);

        let disasm_str = OpCode::new(0x9120).decode();
        assert_eq!("SNE V1, V2", disasm_str);

        let disasm_str = OpCode::new(0xD123).decode();
        assert_eq!("DRW V1, V2, 03", disasm_str);
    }

    #[test]
    fn test_decode_xy_incorrect() {
        let disasm_str = OpCode::new(0x5121).decode();
        assert_eq!("UNKNOWN: 5121", disasm_str);

        let disasm_str = OpCode::new(0x8128).decode();
        assert_eq!("UNKNOWN: 8128", disasm_str);

        let disasm_str = OpCode::new(0x9121).decode();
        assert_eq!("UNKNOWN: 9121", disasm_str);
    }

    #[test]
    fn test_decode_ex() {
        let disasm_str = OpCode::new(0xE19E).decode();
        assert_eq!("SKP V1", disasm_str);

        let disasm_str = OpCode::new(0xE1A1).decode();
        assert_eq!("SKNP V1", disasm_str);

        let disasm_str = OpCode::new(0xE1AA).decode();
        assert_eq!("UNKNOWN: E1AA", disasm_str);
    }

    #[test]
    fn test_decode_fx() {
        let disasm_str = OpCode::new(0xF607).decode();
        assert_eq!("LD V6, DT", disasm_str);

        let disasm_str = OpCode::new(0xF60A).decode();
        assert_eq!("LD V6, K", disasm_str);

        let disasm_str = OpCode::new(0xF615).decode();
        assert_eq!("LD DT, V6", disasm_str);

        let disasm_str = OpCode::new(0xF618).decode();
        assert_eq!("LD ST, V6", disasm_str);

        let disasm_str = OpCode::new(0xF61E).decode();
        assert_eq!("ADD I, V6", disasm_str);

        let disasm_str = OpCode::new(0xF629).decode();
        assert_eq!("LD F, V6", disasm_str);

        let disasm_str = OpCode::new(0xF633).decode();
        assert_eq!("LD B, V6", disasm_str);

        let disasm_str = OpCode::new(0xF655).decode();
        assert_eq!("LD [I], V6", disasm_str);

        let disasm_str = OpCode::new(0xF665).decode();
        assert_eq!("LD V6, [I]", disasm_str);

        let disasm_str = OpCode::new(0xF6FF).decode();
        assert_eq!("UNKNOWN: F6FF", disasm_str);
    }
}
