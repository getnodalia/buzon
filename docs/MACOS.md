# Buzón on macOS: the plan

Buzón draws mail and the composer with WebKitGTK, which does not exist for macOS (Homebrew's
`webkitgtk` is Linux-only and GTK 3). Everything else Buzón uses has a macOS build: GTK 4 (Homebrew
has 4.24), Poppler, OpenSSL, gettext. So the port is the web layer.

## What was proved

`tools/macos-spike` (run by `.github/workflows/macos-spike.yml` on a macOS runner) lays a system
`WKWebView` over a placeholder GTK widget. On macOS 15, arm64, with GTK 4.24 it:

- shows the window and puts the web view over the placeholder, following it as it moves,
- receives `window.webkit.messageHandlers.buzon.postMessage(...)` from JavaScript,
- runs `evaluateJavaScript` and returns its result,
- takes a snapshot.

The embedding is a child `NSView` of the GTK window's content view (`gdk_macos_surface_get_native_window`,
through the `gdk4-macos` crate). Its limits are known: the web view is above everything GTK draws, so
a popover or menu that opens over it has to hide it (or the popover has to be drawn away from it).

## What Buzón uses of WebKitGTK

About 80 call sites in 31 files, in `ui/message_view`, `ui/rich_editor`, `ui/print_preview`,
`ui/send_morph`, `ui/web_signin` and `desktop/`. The HTML, CSS and JavaScript they load stay as they
are: `WKWebView` runs the same web content.

| WebKitGTK | Uses | WKWebView |
|---|---|---|
| `load_html`, `load_uri` | 7 | `loadHTMLString:baseURL:`, `loadRequest:` |
| `evaluate_javascript` | 19 | `evaluateJavaScript:completionHandler:` |
| `UserContentManager` script messages | 13 | `WKUserContentController` + `WKScriptMessageHandler` |
| `load-changed` | 9 | `WKNavigationDelegate` |
| `decide-policy` | 2 | `decidePolicyForNavigationAction` |
| `context-menu` | 3 | `NSView menuForEvent` / custom menu |
| `snapshot` | 5 | `takeSnapshotWithConfiguration:` |
| `print` | 10 | `printOperationWithPrintInfo:` |
| `execute_editing_command` | 6 | JavaScript `document.execCommand` |
| zoom, background colour | 6 | `pageZoom`, `drawsBackground` / CSS |
| spell checking, editor state | few | `NSSpellChecker`, JavaScript |

## Phases

1. **Web layer.** One `web` module with the operations in the table, implemented over WebKitGTK on
   Linux (no change in behaviour: the Linux build is checked screenshot by screenshot) and over
   `WKWebView` on macOS. The call sites move to it one file at a time.
2. **Builds on macOS.** Put the Linux-only parts behind `cfg(target_os = "linux")`: Wayland
   start-up notice, the tray (`ksni`), the D-Bus global menu and Contacts (`zbus`). The Secret
   Service becomes the macOS Keychain (`keyring`'s `apple-native`), gettext comes from Homebrew.
   A `release-macos.yml` job builds on a macOS runner.
3. **The app.** Native window controls (AGENTS.md: macOS and Windows always use the system's),
   the menu bar, `.app` bundle with its icon, bundled GTK libraries.
4. **DMG.** Unsigned, with the Buzón background from `nodalia-design/tools/dmg-background`, called
   from `release-auto.yml`. Open it the first time with right-click → **Open**.

Phase 1 is most of the work.
