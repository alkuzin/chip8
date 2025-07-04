// SPDX-License-Identifier: GPL-3.0-or-later
// Date: 2025-07-04
// Author: Alexander Kuzin <alkuzindev@gmail.com>

//! Emulator main module.

use crate::config::Config;

/// Display emulator entry.
fn print_entry() {
    let name = Config::name();
    let version = Config::version();
    let description = Config::description();
    let license = Config::license();
    let repository = Config::repository();
    let authors = Config::authors();

    println!("\n{} ({}) - {}", name, version, description);
    println!("Repository: {}\nCreated by {}", repository, authors);
    println!("Running under {} license.", license);
}

/// Run an emulator.
pub fn run() {
    print_entry();
}
