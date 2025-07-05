// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025-present chip8 emulator project and contributors

//! Emulator configuration data.

/// Software configuration data struct.
pub struct Config;

impl Config {
    /// Get software version.
    ///
    /// # Returns
    /// - String representation of software version.
    pub const fn version() -> &'static str {
        concat!(
            "v",
            env!("CARGO_PKG_VERSION_MAJOR"),
            ".",
            env!("CARGO_PKG_VERSION_MINOR"),
            ".",
            env!("CARGO_PKG_VERSION_PATCH"),
        )
    }

    /// Get software name.
    ///
    /// # Returns
    /// - String representation of software name.
    pub const fn name() -> &'static str {
        env!("CARGO_PKG_NAME")
    }

    /// Get software description.
    ///
    /// # Returns
    /// - String representation of software description.
    pub const fn description() -> &'static str {
        env!("CARGO_PKG_DESCRIPTION")
    }

    /// Get software license.
    ///
    /// # Returns
    /// - String representation of the software license.
    pub const fn license() -> &'static str {
        env!("CARGO_PKG_LICENSE")
    }

    /// Get software authors.
    ///
    /// # Returns
    /// - String representation of the software authors.
    pub const fn authors() -> &'static str {
        env!("CARGO_PKG_AUTHORS")
    }

    /// Get software repository link.
    ///
    /// # Returns
    /// - String representation of the software repository link.
    pub const fn repository() -> &'static str {
        env!("CARGO_PKG_REPOSITORY")
    }

    /// Get software title.
    ///
    /// # Returns
    /// - String representation of the software title.
    pub const fn title() -> &'static str {
        r#"
 ██████╗██╗  ██╗██╗██████╗  █████╗
██╔════╝██║  ██║██║██╔══██╗██╔══██╗
██║     ███████║██║██████╔╝╚█████╔╝
██║     ██╔══██║██║██╔═══╝ ██╔══██╗
╚██████╗██║  ██║██║██║     ╚█████╔╝
 ╚═════╝╚═╝  ╚═╝╚═╝╚═╝      ╚════╝
        "#
    }

    /// Get software list of available commands.
    ///
    /// # Returns
    /// - List of available commands.
    pub const fn help() -> &'static str {
        r#"
USAGE
       chip8 [options] <file>


DESCRIPTION

       chip8 - CHIP-8 interpreted programming language emulator

OPTIONS

        -d,    --disasm     run in disassembler mode
        -e,    --emulator   run in emulator mode
        -h,    --help       display options list
        -v,    --version    display version of hexd
        "#
    }
}
