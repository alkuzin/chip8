// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025-present chip8 emulator project and contributors

//! Emulated CPU related declarations.

use crate::emulator::opcode::OpCode;

/// CHIP-8 RAM size (4 KB).
const RAM_SIZE: usize = 4096;

/// CHIP-8 stack size (number levels of nested subroutines).
const STACK_SIZE: usize = 16;

/// CHIP-8 general-purpose registers count.
const REGISTER_COUNT: usize = 16;

/// Program start memory address of most CHIP-8 programs.
pub const START_ADDR: usize = 0x200;

/// Emulated CPU main struct.
pub struct Cpu {
    /// CHIP-8 RAM.
    memory: [u8; RAM_SIZE],
    /// General purpose registers.
    registers: [u8; REGISTER_COUNT],
    /// I-register that storing memory addresses.
    register_i: u16,
    /// Program counter - used to store the currently executing address.
    pc: u16,
    /// Stack pointer - used to point to the topmost level of the stack.
    sp: u8,
    /// Execution stack.
    stack: [u16; STACK_SIZE],
    /// Delay timer register.
    dt: u8,
    /// Sound timer register.
    st: u8,
}

impl Cpu {
    /// Construct new `Cpu` object.
    ///
    /// # Returns
    /// - New `Cpu` object.
    pub fn new() -> Self {
        let memory = [0u8; RAM_SIZE];
        let registers = [0u8; REGISTER_COUNT];
        let stack = [0u16; STACK_SIZE];
        let pc = START_ADDR as u16;

        Self { memory, registers, register_i: 0, pc, sp: 0, stack, dt: 0, st: 0 }
    }

    /// Load program to RAM.
    ///
    /// # Parameters
    /// - `program_data` - given program data bytes.
    pub fn load_program(&mut self, program_data: &[u8]) {
        let program_size = program_data.len();
        let load_range = START_ADDR..START_ADDR + program_size;
        let memory_slice = &mut self.memory[load_range];

        memory_slice.copy_from_slice(program_data);
    }

    /// Run a CPU.
    pub fn run(&mut self) {
        while self.pc != RAM_SIZE as u16 {
            let opcode = self.fetch();
            println!("Opcode: {}", opcode.raw);

            self.pc += 2;
        }
    }

    /// Extract next opcode from memory.
    ///
    /// # Returns
    /// - Next opcode from memory.
    fn fetch(&self) -> OpCode {
        let pos = self.pc as usize;
        let raw = u16::from_be_bytes([self.memory[pos], self.memory[pos + 1]]);

        OpCode::new(raw)
    }
}
