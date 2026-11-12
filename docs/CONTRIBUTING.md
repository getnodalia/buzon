# Contributing

Bug reports, translations, design feedback and code are all welcome. Everyone
whose work is in the app is named in [CREDITS.md](CREDITS.md) and in the
About window.

## Ways in

- **Report a bug:** [open an issue](https://github.com/getnodalia/buzon/issues).
  The version, your distribution and what the server is (Gmail, iCloud, Proton
  Bridge, a self-hosted box) narrow most problems down fast. For anything the
  app did over the network, *Main Menu → Export Log* writes a redacted log of
  every command and answer.
- **Suggest a feature:** describe what you are trying to do, rather than the
  control you imagine.
- **Translate:** no Rust needed; see [po/README.md](../po/README.md).
- **Code:** send a pull request; see below.
- **Security:** please report privately, following [SECURITY.md](SECURITY.md).

## Pull requests

- Branch from `main` and keep the change to one subject.
- Build and run it before opening the pull request. [BUILDING.md](BUILDING.md)
  has the dependencies. `cargo fmt --check` and
  `cargo clippy --all-targets -- -D warnings` must pass.
- Match the surrounding code: comments say *why*, not *what*, and user-facing
  strings go through `i18n()`. New or changed strings mean the template needs
  a refresh: `tools/update-pot.sh`.
- Read [AGENTS.md](../AGENTS.md) before larger changes: it holds the project's
  rules for names, the Nodalia interface and secrets.
- There is no CLA. By opening a pull request you agree your contribution ships
  under the [AGPL-3.0-or-later](../COPYING).

## Commits

Subjects follow [Conventional Commits](https://www.conventionalcommits.org):
`type(area): summary`, such as `fix(reader): keep the caret in a reply's body`.
Keep the subject to 72 characters, with no full stop. The body, when there is
one, says briefly why the change exists.

## Documentation

Documentation lives in `docs/`, one subject per file, and the
[README](../README.md) is the front door. A change that documents something
goes where that something already lives:

| What you have | Where it goes |
| --- | --- |
| A feature | [FEATURES.md](FEATURES.md) |
| How to set something up, or use it | [DOCUMENTATION.md](DOCUMENTATION.md) |
| A keyboard change | [KEYBOARD_SHORTCUTS.md](KEYBOARD_SHORTCUTS.md) |
| A package or install caveat | [INSTALLING.md](INSTALLING.md) |
| A build dependency or step | [BUILDING.md](BUILDING.md) |
| Credit | [`data/CONTRIBUTORS`](../data/CONTRIBUTORS) or [`data/TRANSLATORS`](../data/TRANSLATORS) for the name, [CREDITS.md](CREDITS.md) for the work |
| A licence or trademark note | [LICENSE.md](LICENSE.md) |
| What changed in a release | [CHANGELOG.md](../CHANGELOG.md) and [RELEASE_NOTES.md](RELEASE_NOTES.md) |
