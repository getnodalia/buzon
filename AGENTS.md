# Buzón: instructions for agents

Buzón is the email client of the Nodalia suite: Rust, GTK4 and relm4, drawn in the Nodalia Design
Language (NDL). It syncs IMAP, POP3 and JMAP accounts, sends over SMTP, and renders mail in
WebKitGTK. `CLAUDE.md` imports this file; edit this one only.

## Identity

- The app is **Buzón** in the UI, **Buzon** in identifiers. App ID: `dev.danielmiguelt.Buzon`. It
  must match the `.desktop` file, the metainfo, the GResource prefix, the D-Bus names and the
  icon names. Binary and crate: `buzon`. Settings and cache live in `~/.config/buzon` and
  `~/.cache/buzon`; environment variables start with `BUZON_`.
- Buzón is its own app. Never describe it as a fork or a port of something else in the README,
  the website, the UI or release notes.
- That never removes a copyright notice. Code by other authors keeps its notices in the source, in
  `NOTICE` and in the About window's credits, as the AGPL requires.

## Licence

- **AGPL-3.0-or-later** (`COPYING`). Everything merged here is under it.
- Dependencies and assets must be compatible with it. Never add one whose licence you cannot prove.
- Third-party marks, logos, sounds and themes are listed with their licences in `docs/LICENSE.md`.

## Accounts and secrets

- Never bundle someone else's OAuth client. Buzón ships no Google client; a Microsoft or Dropbox
  client is supplied at build time through `BUZON_*` variables or by the user, never committed.
- Passwords, tokens and the cache key live in the Secret Service. Never write them to disk, logs
  or error strings.

## Nodalia Design

- The interface uses `nodalia-gtk` (`use nodalia_gtk as ndl;`), pinned by `rev` from
  [nodalia-design](https://github.com/danielmigueltejedor/nodalia-design). Until the move is
  finished some views still use libadwaita; new code uses `ndl::`, and colours come only from
  `--ndl-*` tokens. Buzón's accent is `#D4241B`.
- A hamburger menu in the header holds Preferences, Keyboard shortcuts and About. The light and
  dark selector lives only in Preferences.
- NDL's round window controls are drawn **on Linux only**, unless the user picks the desktop's own
  in Preferences. macOS and Windows always use native controls. Dialogs use NDL controls too
  (Preferences: close only), never a GNOME titlebar.
- On Linux the app menu is exported over D-Bus to the shell's top-bar global menu. The in-window
  menu bar shows only when the desktop has no global menu.
- No decorative gradients or glows.

## Code

- Rust stable, edition 2021 for now. `cargo clippy --all-targets -- -D warnings` and
  `cargo fmt --check` are the bar.
- Never panic on input: no `unwrap()`/`expect()` outside tests. Mail, headers, MIME parts, server
  replies and attachments are hostile; return errors and show them.
- Never block the GTK main thread: network, IMAP, SMTP and database work run off it.
- A new source file carries the SPDX header `SPDX-License-Identifier: AGPL-3.0-or-later`.
- Commits: conventional commits (`feat:`, `fix:`, `refactor:`, `chore:`, `docs:`).

## CI and releases

- Workflows run on standard GitHub-hosted runners. Renovate is weekly and grouped; there is no
  Dependabot.
- `main` changes through pull requests. A pull request merges once its required checks pass and
  its reviews are resolved.
- Each release ships a macOS DMG with its own designed install window (background in Buzón's
  colour), built in CI, unsigned.
