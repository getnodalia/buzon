# Credits

Buzón is maintained by [Daniel Miguel Tejedor](https://github.com/danielmigueltejedor).
Much of its code was written by [Hyprlab](https://github.com/hyprlab) and the
people below. This page says what each contributor's work was; the names alone live in
[`data/CONTRIBUTORS`](../data/CONTRIBUTORS) and
[`data/TRANSLATORS`](../data/TRANSLATORS), which is what the app's About window
shows. Want to join them? See [CONTRIBUTING.md](CONTRIBUTING.md).

## Code

- [**Alfonso Lizárraga**](https://github.com/alfonsolzrg): sending
  to recipients with punctuated or accented names, the startup message list,
  message-list rebuild performance, the unread dot, and the Attachments-row
  setting.
- [**Chris Pouliot**](https://github.com/chrispouliot): Proton
  Bridge connections, with IMAP STARTTLS and locally signed certificates.
- [**Isaac**](https://github.com/thecalamityjoe87): PDF first-page thumbnails
  in the attachment gallery and drawer, the fix for attachments not opening
  (wrong O_NOFOLLOW constant + portal-based launching), the reader header's
  "To:" line, the preference to always load remote content, the shared
  GNOME-styled right-click context menus, swipe-to-archive/delete on
  message rows, the uninstall script, and the dock badge coming back after
  the screen is unlocked.
- [**Alexander Lubovenko**](https://github.com/typedev): Gmail
  conversations: showing a message once rather than once per label, answering it
  from whichever label already holds its body or attachments, and fetching a
  conversation's bodies in one request instead of one apiece; listing small
  attachments sent from web Gmail that the inline-image heuristic hid;
  fetching a labelled message's attachments once instead of once per label;
  rejoining filenames split across RFC 2047 encoded-words; and pictures
  dropped or pasted from a file manager landing in the message, keeping their
  filename, with resizing by handle or menu and an optional recompress on send;
  and the non-ASCII part header that took an account's mail thread down.
- [**frenchy82**](https://github.com/frenchy82): the French translation,
  Buzón's first, and the report that found the labels the app was showing in
  English despite having the translation.
- [**Laszlo Lang**](https://github.com/7system7): the
  Hungarian translation.
- [**Ilya Semenkovich**](https://github.com/iliasen): the Russian translation, two
  reader tooltips that could not be translated, and the About menu entry that
  could not be either.
- [**Paulo Fino**](https://github.com/somepaulo): the Portuguese (Portugal)
  and Brazilian Portuguese translations, the request for a language chooser, and
  the report that the chosen language never reached the Flatpak.
- [**Daniel Miguel**](https://github.com/danielmigueltejedor): the Spanish
  translation; the counters, today's date, special folder names and settings
  choices that could not be translated; and layouts that make room for
  longer translated labels.
- [**Christian Lauinger**](https://github.com/ChrisLauinger77): the German translation;
  the report that GNOME Online Accounts mail accounts ignored the server's
  security settings; the Debian package build the official package grew
  from; and the launcher's Contacts and Attachments actions.
- [**Tomasz Bojanowski**](https://github.com/TomaszBojanowski): the
  Polish translation.
- [**Anton Palgunov**](https://github.com/Toxblh): sender avatars from GNOME
  Contacts photos, and GNOME Online Accounts refinements: custom server ports
  (IPv6 included), pausing an account while its Mail service is off in GNOME
  Settings, OAuth-aware connection tests, and a timeout on stalled IMAP
  connections.
- [**Yiannis Ioannides**](https://github.com/yioannides): the Greek translation; the
  `--user` flag in the Flatpak install instructions, so a local install no
  longer asks for root; and a long run of requests and design feedback that
  shaped tags, split replies, the reader's own font and colours, Empty Trash
  and the reply panel's fields.
- [**Gonzalo Vasco López**](https://github.com/gonzisdev):
  folded message cards that load their message only when they open, which
  keeps a long conversation from using all the memory.
- [**Isak Samsten**](https://github.com/isaksamsten):
  recipient lookups in LDAP directories. The composer asks the directories
  as an address is typed, once the typing pauses, and drops an answer that
  arrives for text since changed, as the #307 branch did. Also the connection
  for AI agents: the server inside the app, the relay an agent's client
  starts, and drafts that open in the composer for the user to check.
- [**Felix Günther**](https://github.com/fegue): saving contacts and
  reading LDAP directories in the Flatpak, which finds Evolution Data Server
  by asking the session bus, and an address book picker that names each book
  and leaves out the read-only ones.
- [**Salem Sayed Abdel Gawad**](https://github.com/salemsayed): a deadline on every
  IMAP read and write, so a connection that stops answering in the middle of
  a command is dropped and reconnected instead of holding up the account's
  syncing; and background syncing that steps aside for a message being
  opened, with a probe that measures how long its body takes to arrive.
- [**Yo'av Moshe**](https://github.com/bjesus):
  a test suite that no longer depends on the list-preview setting of the
  person running it.
- [**Fabio Rotondo**](https://github.com/fsoft72):
  the Buzón address book with vCard import, filters on whether the sender is
  in Contacts, the per-message Light / Dark Mode switch, tag-colored rows,
  marking a message read from its unread dot, a search that stays open
  while its folder scope is picked, a window that no longer opens blank
  while the keyring is locked, the reminder for a forgotten attachment, and
  Esc taking back a message in Undo Send.
- [**Elliot**](https://github.com/Elliot-32):
  a startup crash when a folder's name has a non-ASCII character near its
  start.
- [**jolupa**](https://github.com/jolupa): the
  Catalan translation.
- [**Jimmy**](https://github.com/amivaleo): the
  Italian translation.
- [**Christian Dandachi**](https://github.com/Dandiccf):
  Reply All leaving out every address of the account, the bodies a
  remembered conversation lacked, a verified sender marked suspicious
  over its reply address, and a conversation that opened at its top
  instead of its new mail.
- [**Comodino**](https://github.com/llComodino):
  waiting for a keyring that asks the user first, such as a security key
  that wants a tap.
- [**Samuel Edlmeier**](https://github.com/samuel-edlmeier):
  the German translation brought up to date, in the impersonal style GNOME
  uses.
- [**Konstantin Adamov**](https://github.com/XRayAdams):
  a message body shown before its remote images have loaded, instead of
  staying blank until the window was resized.
- [**Artur Heinze**](https://github.com/aheinze):
  Reply and New Message opening while the contact service hangs, with
  recipient suggestions loaded in the background.
- [**Valentin Morice**](https://github.com/valentin-morice): the count centered in
  the sidebar's unread chip, a setting to hide the account chip in the
  message header, a message opening as one change on screen, sooner, and
  folder counts that start from the last ones the server gave.
- [**James Reilly**](https://github.com/hanthor): the gap above a message
  card reached with `w` or `b`, Drafts shown from the cache after a send from a
  Microsoft account, OpenPGP checks of the Bcc keys, the signer and unchecked
  keys, a warning before opening a program or a script, trackers taken off
  links, owner-only exports and local mail files with the Flatpak attachment
  purge, a settings export without OAuth client secrets, and the encrypted
  mail cache.
- [**Alp Sahin**](https://github.com/AlpSha):
  text in the accent's foreground color on the accent, and a theme picked in
  Settings shown over the colors of a user's `gtk.css`.
- [**FLASH911**](https://github.com/FLASH911):
  the crash in Add to Calendar on a long non-Latin invitation title, the
  composer menu entries left in the system's language, the Russian
  translation completed, Insert Template in the composer, several names on
  one send-as address, and a sender name for each signature.

## Reports, design and ideas

Not every contribution is code.

Thanks to
[**p-mitana**](https://github.com/p-mitana) for a thorough round of design
feedback (reader, composer and GNOME-HIG suggestions, and a string of sharp
bug reports) that shaped the 1.15 releases; to
[**7system7**](https://github.com/7system7) for the HTML-signature and `mid:`
link requests and [**EmmanuelP**](https://github.com/EmmanuelP) for tracking
down the deletes that failed on Zimbra, both in 1.22; to
[**7system7**](https://github.com/7system7) again for Send Later and cloud
attachments, [**yioannides**](https://github.com/yioannides) for Empty Trash,
the reply-panel fields and the single-message card, and
[**EmmanuelP**](https://github.com/EmmanuelP) for the quote-folding and
Quote-button reports, all in 1.25; to
[**yioannides**](https://github.com/yioannides) for the sidebar
consolidation proposal, picture avatars and the reply-target and menu
reports, [**7system7**](https://github.com/7system7) for the preview
charset and tag-refresh reports, [**Peter Weiss**](https://github.com/peterweissdk)
for the Move To button and [**frenchy82**](https://github.com/frenchy82) for
Not Spam, all in 1.27; to [**yioannides**](https://github.com/yioannides)
for the Inboxes and notification report, [**Peter Weiss**](https://github.com/peterweissdk)
for conversation moves, [**taprobane99**](https://github.com/taprobane99)
for the clock-format report and [**somePaulo**](https://github.com/somepaulo)
for the language chooser, plain-text composing and monospace requests,
all in 1.28; to [**EmmanuelP**](https://github.com/EmmanuelP) for the
undo and sent-copy requests, [**yioannides**](https://github.com/yioannides)
for the recipient-rule report, [**7system7**](https://github.com/7system7)
for the links report and [**frenchy82**](https://github.com/frenchy82) for the
French update, all in 1.32; to [**p-mitana**](https://github.com/p-mitana)
for the reply-target, thread-selection, reply-placement and per-message
attachment reports, with [**yioannides**](https://github.com/yioannides)
weighing in, all in 1.33; and to [**EmmanuelP**](https://github.com/EmmanuelP)
for the meeting-invitations request and
[**yioannides**](https://github.com/yioannides) for the repository cleanup this documentation
follows, both in 1.36; to [**Romain Ferrali**](https://github.com/rferrali)
for the Ctrl+Enter, signature placement, hidden folders and forwarded
attachments requests, [**aia832003**](https://github.com/aia832003) for the
unified-inbox conversation report and
[**7system7**](https://github.com/7system7) for the file-only preview
report, all in 1.38; and to everyone who files issues and ideas.

## Translations

Every language Buzón speaks was given to it by somebody:

- **Catalan:** [jolupa](https://github.com/jolupa)
- **French:** [frenchy82](https://github.com/frenchy82)
- **German:** [Christian Lauinger](https://github.com/ChrisLauinger77) and
  [Samuel Edlmeier](https://github.com/samuel-edlmeier)
- **Greek:** [Yiannis Ioannides](https://github.com/yioannides), with the bulk
  of one round's strings generated by [somePaulo](https://github.com/somepaulo)
- **Hungarian:** [Laszlo Lang](https://github.com/7system7)
- **Italian:** [Jimmy](https://github.com/amivaleo)
- **Polish:** [Tomasz Bojanowski](https://github.com/TomaszBojanowski)
- **Portuguese (Portugal and Brazil):** [Paulo Fino](https://github.com/somepaulo)
- **Russian:** [Ilya Semenkovich](https://github.com/iliasen) and
  [FLASH911](https://github.com/FLASH911)
- **Spanish:** [Daniel Miguel](https://github.com/danielmigueltejedor)

Adding or updating one is the easiest way in: see [po/README.md](../po/README.md).

## Packaging

- [**bennypowers**](https://github.com/bennypowers): the Gentoo ebuild, in
  [his overlay](https://github.com/bennypowers/gentoo-overlay).
- [**clementpoiret**](https://github.com/clementpoiret): the NixOS flake, in
  [buzon-flake](https://github.com/clementpoiret/buzon-flake).

## Third-party work

The brand marks, sender logos, new-mail sounds and appearance themes Buzón
bundles are listed with their sources and licences in
[LICENSE.md](LICENSE.md).
