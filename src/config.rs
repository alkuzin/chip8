// SPDX-License-Identifier: GPL-3.0-or-later
// Date: 2025-07-04
// Author: Alexander Kuzin <alkuzindev@gmail.com>

//! Emulator configuration data.

// Allow unused code for this module.
#![allow(dead_code)]

/// Software configuration data struct.
pub struct Config;

impl Config {
    /// Get software version.
    ///
    /// # Returns
    /// String representation of software version.
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
    /// String representation of software name.
    pub const fn name() -> &'static str {
        env!("CARGO_PKG_NAME")
    }

    /// Get software description.
    ///
    /// # Returns
    /// String representation of software description.
    pub const fn description() -> &'static str {
        env!("CARGO_PKG_DESCRIPTION")
    }

    /// Get software license.
    ///
    /// # Returns
    /// String representation of the software license.
    pub const fn license() -> &'static str {
        env!("CARGO_PKG_LICENSE")
    }

    /// Get software authors.
    ///
    /// # Returns
    /// String representation of the software authors.
    pub const fn authors() -> &'static str {
        env!("CARGO_PKG_AUTHORS")
    }

    /// Get software repository link.
    ///
    /// # Returns
    /// String representation of the software repository link.
    pub const fn repository() -> &'static str {
        env!("CARGO_PKG_REPOSITORY")
    }
}
