// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025-present chip8 emulator project and contributors

//! Emulator builtin disassembler main module.

use crate::{emulator::{opcode::OpCode}, config::Config};
use std::{fs::File, process, io::Read};

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
/// - `filename` - given binary file name.
pub fn disassemble(filename: &String) {
    let name = Config::name();

    match File::open(filename) {
        Err(error) => {
            println!("{name}: error during opening of '{filename}': {error}");
            process::exit(1);
        }
        Ok(mut file) => {
            let mut temp_buffer = Vec::new();

            if let Err(error) = file.read_to_end(&mut temp_buffer) {
                println!("{name}: error read '{filename}' to buffer: {error}");
                process::exit(1);
            }

            let bytes_len = temp_buffer.len();

            if bytes_len % 2 != 0 {
                println!("{name}: error: buffer should have even byte length");
                process::exit(1);
            }

            // Convert file bytes to vector of u16 Big-Endian bytes.
            let mut buffer: Vec<u16> = Vec::with_capacity(bytes_len / 2);

            for chunk in temp_buffer.chunks_exact(2) {
                let opcode = u16::from_be_bytes([chunk[0], chunk[1]]);
                buffer.push(opcode);
            }

            for (i, bytes) in buffer.iter().enumerate() {
                let opcode = OpCode::new(*bytes).decode();
                // TODO: make 0x200 and other magic numbers as global const.
                let addr = 0x200 + i * 2;

                println!("<{addr:#06X}>  |{bytes:04X}|  {opcode}");
            }
        }
    }
}