// SPDX-License-Identifier: GPL-3.0-or-later
// Date: 2025-07-04
// Author: Alexander Kuzin <alkuzindev@gmail.com>

//! Emulator entry point.

mod config;
mod emulator;

fn main() {
    emulator::run();
}
