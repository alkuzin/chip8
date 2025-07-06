// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025-present chip8 emulator project and contributors

//! Emulator builtin disassembler main module.

use crate::emulator::{EmulatorResult, cpu::START_ADDR, opcode::OpCode};

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
pub fn disassemble(program_data: &[u8]) -> EmulatorResult<()> {
    // Convert file bytes to vector of u16 big endian bytes.
    let mut buffer: Vec<u16> = Vec::with_capacity(program_data.len() / 2);

    for chunk in program_data.chunks_exact(2) {
        buffer.push(u16::from_be_bytes([chunk[0], chunk[1]]));
    }

    for (i, bytes) in buffer.iter().enumerate() {
        let opcode = OpCode::new(*bytes).decode();
        let addr = START_ADDR + i * 2;

        println!("<{addr:#05X}>  |{bytes:04X}|  {opcode}");
    }

    Ok(())
}
