// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025-present chip8 emulator project and contributors

//! Command line arguments handling functions.

use crate::{config::Config, emulator::Mode};
use std::{env, process};

/// Handle command line arguments.
///
/// # Returns
///
/// Tuple of:
/// - Emulator mode.
/// - Target filename.
pub fn handle_args() -> (Mode, String) {
    let args: Vec<String> = env::args().collect();
    let argc = args.len();
    let name = Config::name();

    if argc < 2 {
        println!("{name}: Use '-h' or '--help' for usage.");
        process::exit(1);
    }

    let mut mode = Mode::Emulator;
    let mut filename: String = Default::default();

    for (i, arg) in args[1..argc].iter().enumerate() {
        match arg.as_str() {
            "-h" | "--help" => help(),
            "-v" | "--version" => version(),
            "-d" | "--disasm" => {
                mode = Mode::Disassembler;
                filename = get_filename(&args, i + 2);
                break;
            }
            "-e" | "--emulator" => {
                mode = Mode::Emulator;
                filename = get_filename(&args, i + 2);
                break;
            }
            _ => {
                println!("{name}: unknown option '{arg}'");
                process::exit(1);
            }
        }
    }

    if filename.is_empty() {
        println!("{name}: filename is empty");
        process::exit(1);
    }

    (mode, filename)
}

/// Print list of available commands
fn help() {
    println!("{}\n{}", Config::title(), Config::help());
    process::exit(0);
}

/// Print current version of the program
fn version() {
    let title = Config::title();
    let name = Config::name();
    let version = Config::version();
    let description = Config::description();
    let license = Config::license();
    let repository = Config::repository();
    let authors = Config::authors();

    println!("{title}\n\n{name} ({version}) - {description}");
    println!("Repository: {repository}\nCreated by {authors}");
    println!("Running under {license} license.");

    process::exit(0);
}

/// Extract filename from command line arguments.
///
/// # Parameters
/// - `args` - given command line arguments.
/// - `pos`  - given position of filename in arguments list.
///
/// # Returns
/// - Target filename.
fn get_filename(args: &[String], pos: usize) -> String {
    let arg = match args.get(pos) {
        Some(arg) => arg,
        None => {
            println!("{}: target path is not set", Config::name());
            process::exit(1);
        }
    };

    String::from(arg)
}
