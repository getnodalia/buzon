# Installing

Every release is on [GitHub Releases](https://github.com/getnodalia/buzon/releases) with a
`SHA256SUMS-linux` file. Check what you download:

```sh
sha256sum -c --ignore-missing SHA256SUMS-linux
```

## Linux

### AppImage (any distribution)

`Buzon-x86_64.AppImage` carries every library it needs, down to the C library, so it runs on any
x86-64 Linux.

```sh
chmod +x Buzon-x86_64.AppImage     # a browser download is not executable
./Buzon-x86_64.AppImage
```

Run it once from a terminal first; on that first start Buzón writes its own launcher and icons
under `~/.local/share`, so it appears in the dash and the dock with its icon and under a single
entry. If you integrate it with an AppImage manager such as Gear Lever and the manager refuses or
shows a generic icon, install it from the terminal as above and launch it once: Buzón repairs the
launcher the manager wrote.

Buzón tells you when a new version is out, and **Check for Updates…** (Help menu, global menu or
command palette) installs it in place. Keep one copy of the AppImage (for example
`~/Applications`, not also `~/Aplicaciones`).

### Fedora

```sh
sudo dnf install ./buzon-<version>-1.fc43.x86_64.rpm
```

### Ubuntu and Debian

Buzón needs GTK 4.20, which Ubuntu 26.04 has and Debian 13 does not. Use the AppImage on older
systems.

```sh
sudo apt install ./buzon_<version>-1_amd64.deb
```

### From source

[BUILDING.md](BUILDING.md). `./install.sh` installs the binary, icons, translations and launcher
into `~/.local`; `./uninstall.sh` removes them (`--purge` also removes settings and the mail
cache).

At runtime Buzón needs a Secret Service provider (gnome-keyring, KeePassXC or similar): passwords
and the key of the mail cache live there, never in a file.

## macOS

There is no macOS build yet. Buzón shows mail in WebKitGTK, which does not exist for macOS, so a
macOS version first needs that part ported to the system's own web view. None of the release
files is signed by a paid certificate; once the DMG exists, open the app the first time with
right-click → **Open**.
