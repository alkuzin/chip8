// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025-present chip8 emulator project and contributors

//! Emulator main module.

mod disasm;

use crate::config::Config;

/// Display emulator entry.
fn print_entry() {
    let name = Config::name();
    let version = Config::version();
    let description = Config::description();
    let license = Config::license();
    let repository = Config::repository();
    let authors = Config::authors();

    println!("\n{name} ({version}) - {description}");
    println!("Repository: {repository}\nCreated by {authors}");
    println!("Running under {license} license.");
}

/// Run an emulator.
pub fn run() {
    print_entry();
}
