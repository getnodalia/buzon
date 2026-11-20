#!/usr/bin/env bash
#
# Guard the Linux desktop identity for Buzón (#18): the GTK application id,
# the .desktop file name, StartupWMClass and Icon= must all be the same string.
# Runs on any host (no GTK, no AppImage). Exit 1 on mismatch.
#
# Optional: APPDIR=<AppDir> also checks the assembled AppImage tree (icons,
# desktop basename, StartupWMClass, file modes) before packing.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
MAIN="$ROOT/src/main.rs"
DESKTOP_STABLE="$ROOT/data/dev.danielmiguelt.Buzon.desktop"

fail() { echo "check-desktop-identity: $*" >&2; exit 1; }

[ -f "$MAIN" ] || fail "missing $MAIN"
[ -f "$DESKTOP_STABLE" ] || fail "missing $DESKTOP_STABLE"

APP_ID="$(
    sed -n 's/^[[:space:]]*"\(dev\.danielmiguelt\.Buzon\)".*/\1/p' "$MAIN" | head -1
)"
[ -n "$APP_ID" ] || fail "could not read APP_ID from $MAIN"
[ "$APP_ID" = "dev.danielmiguelt.Buzon" ] || fail "unexpected APP_ID '$APP_ID'"

base="$(basename "$DESKTOP_STABLE" .desktop)"
[ "$base" = "$APP_ID" ] || fail "desktop basename '$base' != APP_ID '$APP_ID'"

wmclass="$(awk -F= '/^StartupWMClass=/{print $2; exit}' "$DESKTOP_STABLE")"
[ "$wmclass" = "$APP_ID" ] || fail "StartupWMClass='$wmclass' != APP_ID '$APP_ID'"

icon="$(awk -F= '/^Icon=/{print $2; exit}' "$DESKTOP_STABLE")"
[ "$icon" = "$APP_ID" ] || fail "Icon='$icon' != APP_ID '$APP_ID'"

# Icon= must be a theme name, never a path (integrators copy theme icons out
# of the AppDir; absolute paths break on every other machine).
case "$icon" in
    /*|*/*) fail "Icon= must be a theme name, not a path ('$icon')" ;;
esac

for size in 256x256 512x512; do
    [ -f "$ROOT/data/icons/hicolor/$size/apps/$APP_ID.png" ] \
        || fail "missing data/icons/hicolor/$size/apps/$APP_ID.png"
done
[ -f "$ROOT/data/icons/hicolor/scalable/apps/$APP_ID.svg" ] \
    || fail "missing scalable app icon"
[ -f "$ROOT/data/icons/hicolor/symbolic/apps/$APP_ID-symbolic.svg" ] \
    || fail "missing symbolic app icon"

# Exec must be the binary name (AppImage / packages remap it), not a user path.
exec_line="$(awk -F= '/^Exec=/{print $2; exit}' "$DESKTOP_STABLE")"
case "$exec_line" in
    buzon\ *|buzon) ;;
    *) fail "Exec= must start with 'buzon', got '$exec_line'" ;;
esac

check_appdir() {
    local appdir="$1"
    local desk icon_file mode

    [ -d "$appdir" ] || fail "APPDIR '$appdir' is not a directory"

    # Exactly one top-level desktop, named after the app id (integrators and
    # GNOME Wayland primary matching both care about the basename).
    desk="$(printf '%s\n' "$appdir"/*.desktop)"
    [ -f "$desk" ] || fail "no top-level .desktop in $appdir"
    case "$desk" in
        *" "*) fail "more than one top-level .desktop in $appdir" ;;
    esac
    [ "$(basename "$desk")" = "$APP_ID.desktop" ] \
        || fail "AppDir desktop is '$(basename "$desk")', expected '$APP_ID.desktop'"

    wmclass="$(awk -F= '/^StartupWMClass=/{print $2; exit}' "$desk")"
    [ "$wmclass" = "$APP_ID" ] \
        || fail "AppDir StartupWMClass='$wmclass' != APP_ID '$APP_ID'"

    icon="$(awk -F= '/^Icon=/{print $2; exit}' "$desk")"
    [ "$icon" = "$APP_ID" ] || fail "AppDir Icon='$icon' != APP_ID '$APP_ID'"
    case "$icon" in
        /*|*/*) fail "AppDir Icon= must be a theme name, not a path ('$icon')" ;;
    esac

    for size in 256x256 512x512; do
        icon_file="$appdir/share/icons/hicolor/$size/apps/$APP_ID.png"
        [ -f "$icon_file" ] || fail "AppDir missing $icon_file"
    done
    [ -f "$appdir/share/icons/hicolor/scalable/apps/$APP_ID.svg" ] \
        || fail "AppDir missing scalable app icon"
    [ -f "$appdir/share/icons/hicolor/symbolic/apps/$APP_ID-symbolic.svg" ] \
        || fail "AppDir missing symbolic app icon"

    # Desktop entry must be world-readable; the AppImage itself is checked
    # after packing.
    mode="$(stat -c '%a' "$desk" 2>/dev/null || stat -f '%OLp' "$desk")"
    case "$mode" in
        644|664|744|755) ;;
        *) fail "AppDir desktop mode is $mode (expected world-readable)" ;;
    esac

    echo "check-desktop-identity: AppDir ok ($appdir)"
}

if [ -n "${APPDIR:-}" ]; then
    check_appdir "$APPDIR"
fi

echo "check-desktop-identity: ok ($APP_ID)"
