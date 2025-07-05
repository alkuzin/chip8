// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025-present chip8 emulator project and contributors

//! Emulator main module.

mod disasm;

/// Emulator operation mode.
#[derive(Debug)]
pub enum Mode {
    Emulator,
    Disassembler,
}

/// Run an emulator.
pub fn run(mode: Mode, filename: String) {
    println!("Mode: {mode:#?}, Filename: {filename}");
}
