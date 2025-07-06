// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025-present chip8 emulator project and contributors

//! Emulated CPU related declarations.

use crate::emulator::disasm::Decodable;
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
    /// Current executing opcode.
    opcode: OpCode,
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
        let opcode = OpCode::new(0);

        Self { memory, registers, register_i: 0, pc, sp: 0, stack, dt: 0, st: 0, opcode }
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
            self.fetch();
            self.execute();

            self.pc += 2;
        }
    }

    /// Extract next opcode from memory.
    #[inline(always)]
    fn fetch(&mut self) {
        let pos = self.pc as usize;
        let raw = u16::from_be_bytes([self.memory[pos], self.memory[pos + 1]]);

        self.opcode = OpCode::new(raw);
    }

    /// Execute CPU instruction.
    #[inline(always)]
    fn execute(&mut self) {
        let opcode = &self.opcode;

        println!("Executing: |{:04X}|   {}", opcode.raw, opcode.decode());

        match opcode.class {
            0x0 => self.execute_0xxx(),
            0x1 => self.execute_0nnn(),
            0x2 => self.execute_0nnn(),
            0x3 => unimplemented!(),
            0x4 => unimplemented!(),
            0x5 => unimplemented!(),
            0x6 => unimplemented!(),
            0x7 => unimplemented!(),
            0x8 => unimplemented!(),
            0x9 => unimplemented!(),
            0xA => self.execute_0nnn(),
            0xB => self.execute_0nnn(),
            0xC => unimplemented!(),
            0xD => unimplemented!(),
            0xE => unimplemented!(),
            0xF => unimplemented!(),
            _ => self.unknown(),
        }
    }

    /// Handle unknown instruction.
    ///
    /// # Parameters
    /// - `opcode` - given unknown opcode to handle.
    #[inline(always)]
    fn unknown(&self) {
        let opcode = &self.opcode;

        println!("UNKNOWN: |{:04X}|   {}", opcode.raw, opcode.decode());
        std::process::exit(1); // TODO: replace with Err(...).
    }

    /// Execute CPU 0xxx opcode class instructions.
    #[inline(always)]
    fn execute_0xxx(&mut self) {
        match self.opcode.raw {
            0x00E0 => self.clear_display(),
            0x00EE => self.ret(),
            _ => self.sys(self.opcode.addr),
        }
    }

    /// Clear the display.
    #[inline(always)]
    fn clear_display(&self) {
        unimplemented!()
    }

    /// Return from a subroutine.
    #[inline(always)]
    fn ret(&mut self) {
        // The interpreter sets the program counter to the address at the top of
        // the stack, then subtracts 1 from the stack pointer.
        if self.sp > 0 {
            self.sp -= 1;
            self.pc = self.stack[self.sp as usize];
        }
    }

    /// Jump to a machine code routine at specified address.
    ///
    /// # Parameters
    /// - `addr` - given memory address to jump.
    #[inline(always)]
    fn sys(&self, _: u16) {
        // This instruction is only used on the old computers
        // on which Chip-8 was originally implemented.
        // It is ignored by modern interpreters.
    }

    /// Execute CPU 0nnn opcode class instructions.
    #[inline(always)]
    fn execute_0nnn(&mut self) {
        let addr = self.opcode.addr;

        match self.opcode.class {
            0x1 => self.jump(addr),
            0x2 => self.call(addr),
            0xA => self.set_reg_i(addr),
            0xB => self.jump_by_offset(addr),
            _ => self.unknown(),
        }
    }

    /// Jump to specified location.
    ///
    /// # Parameters
    /// - `addr` - given memory address to jump.
    #[inline(always)]
    fn jump(&mut self, addr: u16) {
        self.pc = addr;
    }

    /// Call subroutine at specified address.
    ///
    /// # Parameters
    /// - `addr` - given memory address to call.
    #[inline(always)]
    fn call(&mut self, addr: u16) {
        self.sp += 1;
        self.stack[self.sp as usize] = self.pc;
        self.pc = addr;
    }

    /// Set register I.
    ///
    /// # Parameters
    /// - `addr` - given memory address.
    #[inline(always)]
    fn set_reg_i(&mut self, addr: u16) {
        self.register_i = addr;
    }

    /// Jump to location by offset.
    ///
    /// # Parameters
    /// - `addr` - given memory address.
    #[inline(always)]
    fn jump_by_offset(&mut self, addr: u16) {
        self.pc = self.registers[0] as u16 + addr;
    }
}
