# Buzón

Buzón is the email client of the Nodalia suite: native, written in Rust, with no telemetry,
remote content blocked by default and credentials kept in the system keyring.

## What's new in 0.1.7

- Fixes a 0.1.6 stylesheet bug that left the sidebar transparent; the AppImage dock icon uses an absolute PNG path so GNOME can show it without waiting on the icon cache

## What's new in 0.1.6

- On AppImage, a Gear Lever / Nodalix launcher with a generic mail icon is repaired in place so GNOME shows Buzón's own icon; the collapsed sidebar Add button no longer clips; the UI paints from Nodalia `--ndl-*` tokens

## What's new in 0.1.5

- On Linux, the AppImage window groups under the correct GNOME dock icon again, ships with the execute bit set, and no longer loses icons to a theme-name clash

## What's new in 0.1.4

- Every language is now fully translated: Catalan, French, German, Greek, Hungarian, Italian, Polish, Portuguese (Brazil and Portugal), Russian and Spanish, including settings, address books, mail merge, calendar invitations and AI agent access

## What's new in 0.1.3

- A new app icon: a sharper portrait of Rowland Hill with a clean cut-out

## What's new in 0.1.2

- Updates from inside the app: Buzón tells you when a new version is out, and the AppImage installs it with one click
- Check for Updates… in the Help menu, the global menu and the command palette

## What's new in 0.1.1

- The welcome wizard, notices, console mode and avatars are drawn in the Nodalia design, with every color from its tokens and no gradients
- The default theme is called Nodalia, and its thumbnail shows Nodalia's real colors

## What's new in 0.1.0

### First release:

- Several accounts at once over IMAP, POP3 or JMAP, with one combined inbox
- The whole mailbox synced and searchable
- Rich text, Markdown, HTML or plain text, chosen per message
- An Outbox for mail that cannot go out yet, and sending later
- OpenPGP with the GnuPG already on your computer
- Filters, tags, conversations and single-key shortcuts
- Drawn in the Nodalia design: rounded panels, the app menu in the header and in the top bar's global menu, and round window controls on Linux that you can swap for your desktop's own
