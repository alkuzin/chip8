// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025-present chip8 emulator project and contributors

//! Emulator entry point.

// Allow unused code.
#![allow(dead_code)]

mod args;
mod config;
mod emulator;

fn main() {
    let (mode, filename) = args::handle_args();
    emulator::run(mode, filename);
}
