// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025-present chip8 emulator project and contributors

//! Emulator entry point.

// Allow unused code.
#![allow(dead_code)]

mod args;
mod config;
mod emulator;

use crate::{config::Config, emulator::Emulator};

fn main() {
    let (mode, filename) = args::handle_args();
    let mut emulator = Emulator::new();

    if let Err(error) = emulator.run(mode, filename) {
        // TODO: add custom error macro.
        let name = Config::name();

        println!("{name}: {error}");
    }
}
