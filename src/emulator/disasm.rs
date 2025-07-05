// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025-present chip8 emulator project and contributors

//! Emulator builtin disassembler main module.

/// Opcode decodable trait.
pub trait Decodable {
    /// Get opcode mnemonic.
    ///
    /// # Returns
    /// - Opcode assembly mnemonic string representation.
    fn decode(&self) -> String;
}

/// CHIP-8 opcode struct.
pub struct OpCode {
    /// Opcode raw bytes.
    raw: u16,
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
        Self { raw }
    }

    /// Extract opcode class.
    ///
    /// # Returns
    /// - Opcode class.
    #[inline(always)]
    pub fn class(&self) -> u16 {
        self.raw & 0xF000
    }

    /// Extract memory address from opcode.
    ///
    /// # Returns
    /// - Memory address from opcode.
    #[inline(always)]
    pub fn addr(&self) -> u16 {
        self.raw & 0x0FFF
    }

    /// Get 0xxx opcode class mnemonic.
    ///
    /// # Returns
    /// - Opcode assembly mnemonic string representation.
    fn decode_0xxx(&self) -> String {
        match self.raw {
            0x00E0 => String::from("CLS"),
            0x00EE => String::from("RET"),
            _      => format!("SYS {:03X}", self.addr()),
        }
    }
}

impl Decodable for OpCode {
    /// Get opcode mnemonic.
    ///
    /// # Returns
    /// - Opcode assembly mnemonic string representation.
    fn decode(&self) -> String {
        let opcode_class = self.class();

        match opcode_class {
            0x0000 => self.decode_0xxx(),
            _      => format!("UNKNOWN: {:04X}", self.raw)
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_class_method() {
        let class = OpCode::new(0x0123).class();
        assert_eq!(0x0000, class);

        let class = OpCode::new(0x6123).class();
        assert_eq!(0x6000, class);

        let class = OpCode::new(0xF123).class();
        assert_eq!(0xF000, class);
    }

    #[test]
    fn test_addr_method() {
        let addr = OpCode::new(0x0123).addr();
        assert_eq!(0x123, addr);

        let addr = OpCode::new(0x0000).addr();
        assert_eq!(0x000, addr);

        let addr = OpCode::new(0xDEAD).addr();
        assert_eq!(0xEAD, addr);
    }

    #[test]
    fn test_unknown_opcode() {
        let disasm_str = OpCode::new(0xFFFF).decode();

        assert_eq!("UNKNOWN: FFFF", disasm_str)
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
}