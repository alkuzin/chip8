// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025-present chip8 emulator project and contributors

//! Emulator main module.

use std::{fs::File, io::Read};

mod cpu;
mod disasm;
mod opcode;

/// Emulator operation mode.
#[derive(Debug)]
pub enum Mode {
    Emulator,
    Disassembler,
}

/// Result wrapper for emulator.
pub type EmulatorResult<T> = Result<T, String>;

/// Emulator main struct.
pub struct Emulator;

impl Emulator {
    /// Construct new `Emulator` object.
    ///
    /// # Returns
    /// - New `Emulator` object.
    pub fn new() -> Self {
        Self {}
    }

    /// Extract program data from binary file.
    ///
    /// # Parameters
    /// - `filename` - given binary file name.
    ///
    /// # Returns
    /// - Program data bytes - in case of success.
    /// - `Err`              - otherwise.
    fn extract_program(&self, filename: &String) -> EmulatorResult<Vec<u16>> {
        match File::open(filename) {
            Ok(mut file) => {
                let mut temp_buffer = Vec::new();

                if let Err(error) = file.read_to_end(&mut temp_buffer) {
                    return Err(format!(
                        "Error read '{filename}' to buffer: {error}"
                    ));
                }

                let bytes_len = temp_buffer.len();

                if bytes_len % 2 != 0 {
                    return Err(
                        "Buffer should have even byte length".to_string()
                    );
                }

                // Convert file bytes to vector of u16 big endian bytes.
                let mut buffer: Vec<u16> = Vec::with_capacity(bytes_len / 2);

                for chunk in temp_buffer.chunks_exact(2) {
                    buffer.push(u16::from_be_bytes([chunk[0], chunk[1]]));
                }

                Ok(buffer)
            }
            Err(error) => {
                Err(format!("Error during opening of '{filename}': {error}"))
            }
        }
    }

    /// Run an emulator.
    ///
    /// # Parameters
    /// - `mode`     - given emulator operation mode.
    /// - `filename` - given target filename.
    ///
    /// # Returns
    /// - `Ok`  - in case of success.
    /// - `Err` - otherwise.
    pub fn run(&mut self, mode: Mode, filename: String) -> EmulatorResult<()> {
        let program_data = self.extract_program(&filename)?;

        match mode {
            Mode::Emulator => unimplemented!(),
            Mode::Disassembler => disasm::disassemble(&program_data),
        }
    }
}
