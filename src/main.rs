// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025-present chip8 emulator project and contributors

//! Emulator entry point.

mod config;
mod emulator;

fn main() {
    emulator::run();
}
