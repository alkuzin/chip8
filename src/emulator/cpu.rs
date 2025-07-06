// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025-present chip8 emulator project and contributors

//! Emulated CPU related declarations.

/// CHIP-8 RAM size (4 KB).
const RAM_SIZE: usize = 4096;

/// Start memory address of most CHIP-8 programs.
const PROGRAM_START_ADDR: usize = 0x200;

/// Emulated CPU main struct.
pub struct Cpu {
    /// CHIP-8 RAM.
    memory: [u8; RAM_SIZE],
}

impl Cpu {
    /// Construct new `Cpu` object.
    ///
    /// # Returns
    /// - New `Cpu` object.
    pub fn new() -> Self {
        let memory = [0u8; RAM_SIZE];

        Self { memory }
    }

    /// Load program to RAM.
    ///
    /// # Parameters
    /// - `program_data` - given program data bytes.
    pub fn load_program(&mut self, program_data: &[u8]) {
        let program_size = program_data.len();
        let memory_slice = &mut self.memory[0x200..0x200 + program_size];

        memory_slice.copy_from_slice(program_data);
    }

    /// Run a CPU.
    pub fn run(&mut self) {
        unimplemented!()
    }
}
