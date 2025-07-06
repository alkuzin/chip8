// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025-present chip8 emulator project and contributors

//! Emulator builtin disassembler main module.

use crate::emulator::{EmulatorResult, opcode::OpCode};

/// Opcode decodable trait.
pub trait Decodable {
    /// Get opcode mnemonic.
    ///
    /// # Returns
    /// - Opcode assembly mnemonic string representation.
    fn decode(&self) -> String;
}

/// Display assembly mnemonics of specified binary file.
///
/// # Parameters
/// - `program_data` - given program data bytes.
///
/// # Returns
/// - `Ok`  - in case of success.
/// - `Err` - otherwise.
pub fn disassemble(program_data: &[u16]) -> EmulatorResult<()> {
    for (i, bytes) in program_data.iter().enumerate() {
        let opcode = OpCode::new(*bytes).decode();
        // TODO: make 0x200 and other magic numbers as global const.
        let addr = 0x200 + i * 2;

        println!("<{addr:#06X}>  |{bytes:04X}|  {opcode}");
    }

    Ok(())
}
