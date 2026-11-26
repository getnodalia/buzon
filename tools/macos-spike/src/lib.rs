// SPDX-License-Identifier: AGPL-3.0-or-later
//! The macOS spike's web view layer: a `WKWebView` laid over a region of a GTK window.
//!
//! WebKitGTK does not exist for macOS, so Buzón's mail view and composer cannot use it there. GTK 4
//! has no widget for a foreign native view, so the web view is a child `NSView` of the window's
//! content view, placed over a placeholder GTK widget and moved with it.

#[cfg(target_os = "macos")]
pub mod webview;
