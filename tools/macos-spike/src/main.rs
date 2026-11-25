// SPDX-License-Identifier: AGPL-3.0-or-later
//! Opens a GTK 4 window with a web view laid over part of it, runs a script and a message round
//! trip through it, takes a snapshot, and exits 0 only when every step worked. Run by CI on macOS.

#[cfg(all(target_os = "macos", feature = "app"))]
mod app;

fn main() {
    #[cfg(all(target_os = "macos", feature = "app"))]
    std::process::exit(app::run());
    #[cfg(not(all(target_os = "macos", feature = "app")))]
    {
        eprintln!("This spike runs on macOS, built with --features app.");
        std::process::exit(2);
    }
}
