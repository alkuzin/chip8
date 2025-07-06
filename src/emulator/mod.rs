// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025-present chip8 emulator project and contributors

//! Emulator main module.

use crate::emulator::cpu::Cpu;
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
pub struct Emulator {
    /// Emulated CPU.
    cpu: Cpu,
}

impl Emulator {
    /// Construct new `Emulator` object.
    ///
    /// # Returns
    /// - New `Emulator` object.
    pub fn new() -> Self {
        let cpu = Cpu::new();

        Self { cpu }
    }

    /// Extract program data from binary file.
    ///
    /// # Parameters
    /// - `filename` - given binary file name.
    ///
    /// # Returns
    /// - Program data bytes - in case of success.
    /// - `Err`              - otherwise.
    fn extract_program(&self, filename: &String) -> EmulatorResult<Vec<u8>> {
        match File::open(filename) {
            Ok(mut file) => {
                let mut buffer = Vec::new();

                if let Err(error) = file.read_to_end(&mut buffer) {
                    return Err(format!(
                        "Error read '{filename}' to buffer: {error}"
                    ));
                }

                if buffer.len() % 2 != 0 {
                    return Err(
                        "Buffer should have even byte length".to_string()
                    );
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
            Mode::Emulator => self.emulate(&program_data),
            Mode::Disassembler => disasm::disassemble(&program_data),
        }
    }

    /// Emulate platform.
    ///
    /// # Parameters
    /// - `program_data` - given program data bytes.
    ///
    /// # Returns
    /// - `Ok`  - in case of success.
    /// - `Err` - otherwise.
    fn emulate(&mut self, program_data: &[u8]) -> EmulatorResult<()> {
        self.cpu.load_program(program_data);
        self.cpu.run();

        Ok(())
    }
}
