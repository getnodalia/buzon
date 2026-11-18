// SPDX-License-Identifier: AGPL-3.0-or-later
//! In-app updates: a look at GitHub Releases, a question, and a verified install.
//!
//! The work is `nodalia-update`'s, shared with the other Nodalia apps; this file is what is
//! Buzón's own: where its releases live, the menu item, and the two windows. Nothing is installed
//! without a click. Only the AppImage replaces itself; for the Flatpak and the distribution
//! packages the check at start says nothing, because their package manager already does, and a
//! check asked for from the menu offers the release page. A failed check at start goes to the
//! log, not to a toast nobody asked for.

use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use gtk::prelude::*;
use nodalia_update as nu;
use relm4::prelude::*;

use crate::i18n::{i18n, i18n_f};
use crate::ui::message_dialog::MessageDialog;

use super::{AppModel, AppMsg};

pub(crate) const APP: nu::App = nu::App {
    name: "Buzón",
    repo: "getnodalia/buzon",
    bundle: "Buzón.app",
    version: env!("CARGO_PKG_VERSION"),
    edition: None,
};

/// An automatic check is skipped if one ran less than this long ago.
const EVERY: u64 = 24 * 60 * 60;

/// How long after start the automatic check waits, so it never competes with the first sync.
const STARTUP_DELAY: u32 = 30;

/// What the update windows hold between messages.
#[derive(Default)]
pub(crate) struct UpdateUi {
    /// A check or an install is running: a second click does nothing.
    busy: bool,
    /// The release the open question is about.
    release: Option<nu::Release>,
    /// The window shown while the release downloads.
    progress: Option<MessageDialog>,
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}

/// Whether an automatic check is due, given when the last one ran.
fn due(last: Option<u64>, now: u64) -> bool {
    last.is_none_or(|last| now.saturating_sub(last) >= EVERY)
}

/// Where the time of the last automatic check is kept.
fn stamp_path() -> PathBuf {
    gtk::glib::user_cache_dir()
        .join("buzon")
        .join("update-checked")
}

fn read_stamp() -> Option<u64> {
    std::fs::read_to_string(stamp_path())
        .ok()
        .and_then(|s| s.trim().parse().ok())
}

fn write_stamp(at: u64) {
    let path = stamp_path();
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    if let Err(err) = std::fs::write(&path, at.to_string()) {
        tracing::debug!(%err, "could not record the update check");
    }
}

/// The language the update texts are worded in: Spanish for a Spanish session, English for the
/// rest (the release notes are written in those two).
fn language() -> nu::Language {
    let code = crate::i18n::locale_env(true).next().unwrap_or_default();
    nu::Language::from_code(code.get(..2).unwrap_or_default())
}

/// The check at start, a while after the window shows: only for the AppImage, which can replace
/// itself, and at most once a day.
pub(crate) fn schedule_startup_check(sender: &ComponentSender<AppModel>) {
    if cfg!(feature = "beta") || super::demo::demo_mode() {
        return;
    }
    if !nu::install_kind(&APP).can_apply_in_place() || !due(read_stamp(), now()) {
        return;
    }
    let sender = sender.clone();
    gtk::glib::timeout_add_seconds_local_once(STARTUP_DELAY, move || {
        sender.input(AppMsg::CheckForUpdates { user: false });
    });
}

impl AppModel {
    /// The messages the router sends here (see `AppModel::route`).
    pub(super) fn on_updates(&mut self, msg: AppMsg, sender: ComponentSender<Self>) {
        match msg {
            AppMsg::CheckForUpdates { user } => self.check_for_updates(&sender, user),
            AppMsg::UpdateChecked { user, result } => self.update_checked(&sender, user, result),
            AppMsg::InstallUpdate => self.install_update(&sender),
            AppMsg::OpenReleasePage => self.open_release_page(),
            AppMsg::UpdateInstalled(result) => self.update_installed(result),
            _ => tracing::warn!("a message not for the updater reached it"),
        }
    }

    fn toast_text(&self, text: &str) {
        self.show_toast(&ndl::Toast::new(text));
    }

    fn check_for_updates(&mut self, sender: &ComponentSender<Self>, user: bool) {
        if self.updates.busy {
            return;
        }
        self.updates.busy = true;
        if !user {
            write_stamp(now());
        }
        let language = language();
        let sender = sender.clone();
        gtk::glib::spawn_future_local(async move {
            let result = gtk::gio::spawn_blocking(|| nu::check(&APP))
                .await
                .map_err(|_| i18n("The check for updates stopped."))
                .and_then(|r| r.map_err(|e| e.describe(language)));
            sender.input(AppMsg::UpdateChecked { user, result });
        });
    }

    fn update_checked(
        &mut self,
        sender: &ComponentSender<Self>,
        user: bool,
        result: Result<nu::Check, String>,
    ) {
        self.updates.busy = false;
        match result {
            Ok(nu::Check::UpToDate) if user => {
                self.toast_text(&i18n_f(
                    "Buzón {version} is the latest version",
                    &[("version", APP.version)],
                ));
            }
            Ok(nu::Check::UpToDate) => {}
            Ok(nu::Check::Available(release)) => self.offer_update(sender, release, user),
            Err(why) if user => self.toast_text(&why),
            Err(why) => tracing::info!(%why, "automatic update check failed"),
        }
    }

    fn offer_update(&mut self, sender: &ComponentSender<Self>, release: nu::Release, user: bool) {
        let in_place = nu::install_kind(&APP).can_apply_in_place() && release.asset.is_some();
        if !in_place && !user {
            return;
        }
        let heading = i18n_f(
            "Buzón {version} is available",
            &[("version", release.version.as_str())],
        );
        let dialog = MessageDialog::new(Some(&self.window), Some(&heading), None);
        dialog.set_size_request(440, -1);
        dialog.set_extra_child(Some(&offer_body(&release.notes, in_place)));
        dialog.add_response("later", &i18n("Later"));
        let main = if in_place { "install" } else { "page" };
        let label = if in_place {
            i18n("Install and Restart")
        } else {
            i18n("Open Download Page")
        };
        dialog.add_response(main, &label);
        dialog.set_response_appearance(main, ndl::ResponseAppearance::Suggested);
        dialog.set_default_response(Some(main));
        dialog.set_close_response("later");
        self.updates.release = Some(release);

        let sender = sender.clone();
        dialog.connect_response(None, move |_, response| match response {
            "install" => sender.input(AppMsg::InstallUpdate),
            "page" => sender.input(AppMsg::OpenReleasePage),
            _ => {}
        });
        dialog.present();
    }

    fn open_release_page(&self) {
        let Some(release) = &self.updates.release else {
            return;
        };
        gtk::UriLauncher::new(&release.page).launch(
            Some(&self.window),
            gtk::gio::Cancellable::NONE,
            |result| {
                if let Err(err) = result {
                    tracing::warn!(?err, "could not open the release page");
                }
            },
        );
    }

    /// Download, verify and install the release the question was about, then restart.
    fn install_update(&mut self, sender: &ComponentSender<Self>) {
        let Some(release) = self.updates.release.clone() else {
            return;
        };
        if self.updates.busy {
            return;
        }
        self.updates.busy = true;
        let heading = i18n_f(
            "Downloading Buzón {version}…",
            &[("version", release.version.as_str())],
        );
        let dialog = MessageDialog::new(Some(&self.window), Some(&heading), None);
        let spinner = gtk::Spinner::builder()
            .spinning(true)
            .margin_top(12)
            .build();
        dialog.set_extra_child(Some(&spinner));
        dialog.present();
        self.updates.progress = Some(dialog);

        let language = language();
        let sender = sender.clone();
        gtk::glib::spawn_future_local(async move {
            let result = gtk::gio::spawn_blocking(move || {
                let package = nu::download(&APP, &release, |_, _| {})?;
                nu::apply(&APP, &package, || Ok(()))
            })
            .await
            .map_err(|_| i18n("The update stopped."))
            .and_then(|r| r.map_err(|e| e.describe(language)));
            sender.input(AppMsg::UpdateInstalled(result));
        });
    }

    fn update_installed(&mut self, result: Result<(), String>) {
        self.updates.busy = false;
        if let Some(dialog) = self.updates.progress.take() {
            dialog.dialog().force_close();
        }
        match result {
            // The new AppImage is starting and waits for this process to end: leave now, the way
            // Quit does, so held messages are kept for the next start.
            Ok(()) => super::window::exit_now(&self.outgoing, &self.merges),
            Err(why) => self.toast_text(&why),
        }
    }
}

/// The release notes in the reader's language, scrolling on their own so the heading and the
/// buttons never leave the window, then a hint when the app cannot replace itself.
fn offer_body(notes: &str, in_place: bool) -> gtk::Box {
    let column = gtk::Box::new(gtk::Orientation::Vertical, 12);
    let markup = nu::notes_markup(notes, language());
    if !markup.is_empty() {
        let label = gtk::Label::builder()
            .use_markup(true)
            .wrap(true)
            .wrap_mode(gtk::pango::WrapMode::WordChar)
            .xalign(0.0)
            .yalign(0.0)
            .halign(gtk::Align::Fill)
            .build();
        label.set_markup(&markup);
        let scroller = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .propagate_natural_height(true)
            .max_content_height(260)
            .child(&label)
            .build();
        column.append(&scroller);
    }
    if !in_place {
        let hint = gtk::Label::builder()
            .label(i18n(
                "Update Buzón the way you installed it: with your software center, your package \
                 manager or the new release's package.",
            ))
            .wrap(true)
            .xalign(0.0)
            .build();
        hint.add_css_class("dim-label");
        column.append(&hint);
    }
    column
}

/// The new copy an update starts must not hand itself to the old one, which is about to exit:
/// wait (a few seconds at most) for that process to end before the application registers.
pub(crate) fn wait_for_the_old_version() {
    let Some(pid) = std::env::var(nu::RELAUNCH_FROM)
        .ok()
        .and_then(|p| p.trim().parse::<u32>().ok())
    else {
        return;
    };
    let proc = PathBuf::from(format!("/proc/{pid}"));
    for _ in 0..100 {
        if !proc.exists() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    // Single-threaded here, before GTK: removing the variable keeps a later restart honest.
    std::env::remove_var(nu::RELAUNCH_FROM);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_releases_are_buzons_own() {
        assert_eq!(APP.repo, "getnodalia/buzon");
        assert_eq!(APP.version, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn an_automatic_check_runs_once_a_day() {
        assert!(due(None, 1_000));
        assert!(!due(Some(1_000), 1_000 + EVERY - 1));
        assert!(due(Some(1_000), 1_000 + EVERY));
        assert!(!due(Some(2_000), 1_000), "a clock that went back");
    }
}
