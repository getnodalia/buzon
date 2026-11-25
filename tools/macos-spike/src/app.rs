// SPDX-License-Identifier: AGPL-3.0-or-later
//! The GTK side of the spike.

use std::cell::Cell;
use std::ffi::c_void;
use std::rc::Rc;
use std::time::Duration;

use buzon_macos_spike::webview::NativeWebView;
use gtk::glib;
use gtk::prelude::*;

const PAGE: &str = r#"<!doctype html>
<html><head><meta charset="utf-8"><title>Buzón spike</title>
<style>body{font:16px -apple-system,sans-serif;margin:24px;background:#fff;color:#111}
h1{color:#d4241b}</style></head>
<body onload="window.webkit.messageHandlers.buzon.postMessage('hello from JavaScript')">
<h1>WKWebView inside GTK 4</h1>
<p>This is HTML drawn by the system's own web view, laid over a GTK widget.</p>
</body></html>"#;

const GOT_MESSAGE: u8 = 1;
const GOT_RESULT: u8 = 2;
const GOT_SNAPSHOT: u8 = 4;
const ALL: u8 = GOT_MESSAGE | GOT_RESULT | GOT_SNAPSHOT;

/// Runs the spike; the exit code is 0 when the window, the message, the script result and the
/// snapshot all worked.
pub fn run() -> i32 {
    let steps = Rc::new(Cell::new(0u8));
    let app = gtk::Application::builder()
        .application_id("dev.danielmiguelt.Buzon.MacosSpike")
        .build();
    let shared = steps.clone();
    app.connect_activate(move |app| build(app, shared.clone()));
    app.run_with_args::<&str>(&[]);
    let done = steps.get();
    println!("SPIKE steps: {done:#05b} of {ALL:#05b}");
    if done == ALL {
        0
    } else {
        1
    }
}

fn build(app: &gtk::Application, steps: Rc<Cell<u8>>) {
    let placeholder = gtk::Box::new(gtk::Orientation::Vertical, 0);
    placeholder.set_hexpand(true);
    placeholder.set_vexpand(true);
    placeholder.set_size_request(400, 300);

    let side = gtk::Label::new(Some("GTK side"));
    side.set_width_request(200);
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    row.append(&side);
    row.append(&placeholder);

    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Buzón macOS spike")
        .default_width(900)
        .default_height(600)
        .child(&row)
        .build();
    window.present();

    // The native window exists once the window is mapped.
    let win = window.clone();
    let app = app.clone();
    glib::timeout_add_local_once(Duration::from_millis(800), move || {
        let Some(surface) = win
            .surface()
            .and_then(|s| s.downcast::<gdk_macos::MacosSurface>().ok())
        else {
            println!("SPIKE: the window has no macOS surface");
            app.quit();
            return;
        };
        let ns_window = surface.native() as *mut c_void;
        let got = steps.clone();
        let view = unsafe {
            NativeWebView::attach(ns_window, move |message| {
                println!("SPIKE message: {message}");
                if message == "hello from JavaScript" {
                    got.set(got.get() | GOT_MESSAGE);
                }
            })
        };
        let Some(view) = view else {
            println!("SPIKE: could not attach the web view");
            app.quit();
            return;
        };
        let view = Rc::new(view);

        // The web view follows the placeholder, whatever moves or resizes it.
        let follow = view.clone();
        let place = placeholder.clone();
        win.add_tick_callback(move |win, _| {
            if let Some(b) = place.compute_bounds(win) {
                follow.set_frame(
                    f64::from(b.x()),
                    f64::from(b.y()),
                    f64::from(b.width()),
                    f64::from(b.height()),
                );
            }
            glib::ControlFlow::Continue
        });
        view.load_html(PAGE);

        let script = view.clone();
        let got = steps.clone();
        glib::timeout_add_local_once(Duration::from_millis(2500), move || {
            script.evaluate_javascript("document.title + ' / ' + (6 * 7)", move |result| {
                println!("SPIKE script: {result:?}");
                if result.as_deref() == Ok("Buzón spike / 42") {
                    got.set(got.get() | GOT_RESULT);
                }
            });
        });

        let shot = view.clone();
        let got = steps.clone();
        glib::timeout_add_local_once(Duration::from_millis(3500), move || {
            shot.snapshot(move |size| {
                println!("SPIKE snapshot: {size:?}");
                if size.is_some_and(|(w, h)| w > 0.0 && h > 0.0) {
                    got.set(got.get() | GOT_SNAPSHOT);
                }
            });
        });

        glib::timeout_add_local_once(Duration::from_millis(6000), move || app.quit());
    });
}
