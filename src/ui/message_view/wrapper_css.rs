//! The wrapper document's stylesheet: the cards, their headers, chips and
//! banners, in the reader's grounds and accent. One `format!` so the bytes
//! stay exactly what the page has always received.

use super::frame::StatusColors;

/// The wrapper's stylesheet: the cards, their headers and chips, in the
/// reader's grounds and accent. The marks and verdicts wear the theme's
/// status colors, as the app's own widgets do (#619).
// Each a color or a fragment dropped into the one `format!`.
#[allow(clippy::too_many_arguments)]
pub(super) fn wrapper_css(
    scheme: &str,
    page: &str,
    bg: &str,
    chrome: &str,
    accent: &str,
    status: &StatusColors,
    toggle_color: &str,
    plain_css: &str,
) -> String {
    let StatusColors {
        error,
        warning,
        success,
    } = status;
    format!(
        "\
         :root{{color-scheme:{scheme};}}\
         body{{margin:0;padding:0;background:{page};font:14px/1.55 system-ui,sans-serif;}}\
         body.buzon-conv{{padding:14px;}}\
         iframe.buzon-frame{{width:100%;border:0;display:block;background:{bg};\
         visibility:hidden;}}\
         iframe.buzon-frame.buzon-live{{visibility:visible;}}\
         .buzon-pan{{overflow-x:auto;position:relative;}}\
         iframe.buzon-frame.anim{{transition:height 240ms cubic-bezier(0.4,0,0.2,1);}}\
         @media (prefers-reduced-motion:reduce){{iframe.buzon-frame.anim{{transition:none;}}}}\
         .buzon-msg{{background:{bg};\
         border-radius:12px;overflow:hidden;margin:0 0 14px;}}\
         .buzon-msg:last-child{{margin-bottom:0;}}\
         .buzon-msg{{user-select:none;}}\
         body:not(.buzon-conv) .buzon-msg{{border-radius:0;margin:0;}}\
         .buzon-msg.selected{{box-shadow:0 0 0 2px {accent};}}\
         .buzon-fbar{{display:none;}}\
         /* Folded, a card keeps its header exactly as it is open and\
         shows the preview under it, lined up with the sender's name\
         (16px padding, 26px circle, 8px gap). */\
         .buzon-msg.buzon-folded>.buzon-fbar{{display:-webkit-box;-webkit-box-orient:vertical;\
         overflow:hidden;opacity:0.6;font-size:0.92em;line-height:1.35;overflow-wrap:anywhere;\
         margin:-6px 0 12px;padding:0;padding-inline:50px 16px;position:relative;z-index:2;\
         cursor:pointer;user-select:none;}}\
         .buzon-msg.buzon-folded>:not(.buzon-fbar):not(.buzon-msg-hdr){{display:none;}}\
         /* Contents, not a box: a slot that is not drawn then takes no\
         gap in the meta row either (#616). */\
         .buzon-marks{{display:contents;}}\
         .buzon-mark{{display:inline-flex;opacity:0.55;line-height:0;\
         align-self:center;flex:none;}}\
         .buzon-mark svg{{width:16px;height:16px;}}\
         .buzon-mark svg,.buzon-mark svg *{{fill:currentColor;}}\
         .buzon-mark-none{{width:16px;height:16px;}}\
         /* An unstarred star and a missing paperclip keep their place, so\
         the marks of a conversation's cards stand in the same columns, but\
         only while some message on the page fills that column: otherwise\
         they are empty space between the chips and the date (#616). */\
         .buzon-mark-star{{color:#e5a50a;opacity:1;}}\
         .buzon-mark-high{{color:{error};opacity:1;}}\
         .buzon-msg:not(.starred) .buzon-mark-star{{visibility:hidden;}}\
         body:not(:has(.buzon-msg.starred)) .buzon-mark-star,\
         body:not(:has(.buzon-mark-clip)) .buzon-mark-none{{display:none;}}\
         body.buzon-folds .buzon-msg-hdr{{cursor:pointer;}}\
         @media print{{.buzon-msg.buzon-folded>:not(.buzon-fbar){{display:revert !important;}}\
         .buzon-msg.buzon-folded>.buzon-fbar{{display:none !important;}}}}\
         .buzon-msg-hdr{{cursor:pointer;}}\
         .buzon-msg-hdr{{padding:12px 16px;cursor:default;user-select:none;\
         position:sticky;top:0;z-index:1;background-color:{bg};}}\
         /* 8px between items on a line, but only 2px between wrapped\
         lines — the flex gap otherwise pads rows apart too. */\
         .buzon-hdr-line{{display:flex;column-gap:8px;row-gap:2px;\
         align-items:baseline;flex-wrap:nowrap;\
         min-width:0;position:relative;}}\
         .buzon-hdr-meta{{display:flex;gap:8px;align-items:baseline;flex:none;\
         margin-inline-start:auto;}}\
         /* Narrow pane: the meta group (folder chip, recipients chip, date)\
         drops to its own line as one unit — no mid-text wrapping — while\
         the sender and the \u{22ef} hold the first line. */\
         @media (max-width:620px){{\
         /* Tighten the right-hand insets (card gutter + header padding)\
         so the \u{22ef} sits nearer the pane's edge — but the\
         combined inset never drops below 20px. */\
         body.buzon-conv{{padding:10px;}}\
         body.buzon-conv .buzon-msg-hdr{{padding:12px 10px;}}\
         .buzon-msg.buzon-folded>.buzon-fbar{{padding-inline:44px 10px;}}\
         .buzon-hdr-line{{flex-wrap:wrap;}}\
         .buzon-hdr-meta{{order:10;flex-basis:100%;margin-inline-start:34px;\
         justify-content:flex-start;}}\
         /* Below the wrapped meta line the recipients need a touch\
         more air than the wide layout's 2px. */\
         .buzon-rcpt{{margin-top:6px;}}\
         }}\
         body:not(.buzon-conv) .buzon-msg-hdr{{background-color:{chrome};}}\
         {plain_css}\
         .buzon-ava{{width:26px;height:26px;border-radius:50%;flex:none;align-self:center;\
         display:flex;align-items:center;justify-content:center;color:#fff;\
         font-size:0.8em;font-weight:700;}}\
         /* Zero flex-basis: the name and address only absorb free space\
         (up to their natural width), so their length can never force\
         the \u{22ef}'s room onto another line. Under pressure the address\
         gives way first (tiny grow share), fading out at its right\
         edge; the name ellipsizes only after that. */\
         .buzon-from{{font-weight:700;flex:99 1 0;max-width:max-content;min-width:0;\
         white-space:nowrap;overflow:hidden;text-overflow:ellipsis;}}\
         .buzon-tags{{display:inline-flex;gap:4px;align-items:center;flex:0 0 auto;}}\
         .buzon-tags:empty{{display:none;}}\
         /* A card's own attachments (#213): chips beneath the body. */\
         .buzon-atts{{display:flex;flex-wrap:wrap;gap:6px;padding:2px 14px 12px;}}\
         .buzon-atts:empty{{display:none;}}\
         /* The Unsubscribe banner between a list message's header and\
         body: a line of text and one button. */\
         .buzon-tr{{display:flex;align-items:center;gap:10px;padding:8px 14px 9px;\
         font-size:0.9em;border-bottom:1px solid rgba(128,128,128,0.25);}}\
         .buzon-tr:empty{{display:none;}}\
         .buzon-tr-text{{flex:1 1 auto;min-width:0;opacity:0.8;}}\
         .buzon-tr-btn{{flex:none;border:none;border-radius:6px;\
         padding:3px 12px;background:color-mix(in srgb,currentColor 10%,transparent);\
         color:inherit;font:inherit;font-size:0.95em;font-weight:600;cursor:pointer;}}\
         .buzon-tr-btn:hover{{background:color-mix(in srgb,currentColor 15%,transparent);}}\
         .buzon-tr-btn:active{{background:color-mix(in srgb,currentColor 30%,transparent);}}\
         .buzon-unsub,.buzon-mdn{{display:flex;align-items:center;gap:10px;padding:8px 14px 9px;\
         font-size:0.9em;border-bottom:1px solid rgba(128,128,128,0.25);}}\
         .buzon-unsub:empty,.buzon-mdn:empty{{display:none;}}\
         .buzon-unsub-text{{flex:1 1 auto;min-width:0;opacity:0.8;}}\
         .buzon-unsub-btn{{flex:none;border:none;border-radius:6px;\
         padding:3px 12px;background:color-mix(in srgb,currentColor 10%,transparent);\
         color:inherit;font:inherit;font-size:0.95em;font-weight:600;cursor:pointer;}}\
         .buzon-unsub-btn:hover{{background:color-mix(in srgb,currentColor 15%,transparent);}}\
         .buzon-unsub-btn:active{{background:color-mix(in srgb,currentColor 30%,transparent);}}\
         /* The invitation banner (#223): the meeting at the top of\
         the card, and the answer buttons under it. */\
         .buzon-inv{{display:flex;gap:10px;padding:10px 14px 11px;font-size:0.9em;\
         border-bottom:1px solid rgba(128,128,128,0.25);}}\
         .buzon-inv:empty{{display:none;}}\
         .buzon-inv-icon{{flex:none;padding-top:1px;opacity:0.7;}}\
         .buzon-inv-icon svg{{width:20px;height:20px;display:block;}}\
         .buzon-inv-icon svg,.buzon-inv-icon svg *{{fill:currentColor;}}\
         .buzon-inv-body{{flex:1 1 auto;min-width:0;}}\
         .buzon-inv-head{{display:flex;align-items:baseline;gap:8px;flex-wrap:wrap;}}\
         .buzon-inv-title{{font-weight:700;font-size:1.05em;}}\
         .buzon-inv-title.off{{text-decoration:line-through;opacity:0.7;}}\
         .buzon-inv-kind{{font-size:0.85em;text-transform:uppercase;\
         letter-spacing:0.04em;opacity:0.6;}}\
         .buzon-inv-row{{opacity:0.8;margin-top:2px;}}\
         .buzon-inv-note{{margin-top:4px;opacity:0.8;font-style:italic;}}\
         .buzon-inv-acts{{display:flex;flex-wrap:wrap;gap:6px;margin-top:8px;}}\
         .buzon-inv-btn{{border:1px solid rgba(128,128,128,0.4);border-radius:6px;\
         padding:3px 12px;background:rgba(128,128,128,0.1);color:inherit;font:inherit;\
         font-size:0.95em;font-weight:600;cursor:pointer;}}\
         .buzon-inv-btn:hover{{background:rgba(128,128,128,0.22);}}\
         /* The answer already given is marked, so a meeting opened\
         again shows where it stands at a glance. */\
         .buzon-inv-btn.on{{border-color:{accent};color:{accent};\
         background:color-mix(in srgb, {accent} 16%, transparent);}}\
         .buzon-attw{{display:inline-flex;align-items:stretch;max-width:100%;\
         border:1px solid rgba(128,128,128,0.35);border-radius:8px;overflow:hidden;\
         background:rgba(128,128,128,0.08);}}\
         .buzon-attc{{display:inline-flex;align-items:center;gap:6px;padding:4px 8px;\
         border:none;background:none;color:inherit;font:inherit;font-size:0.85em;\
         cursor:pointer;min-width:0;max-width:320px;}}\
         .buzon-attc:hover{{background:rgba(128,128,128,0.18);}}\
         .buzon-attc svg{{width:16px;height:16px;flex:none;display:block;}}\
         .buzon-attc svg,.buzon-attc svg *{{fill:currentColor;}}\
         .buzon-attc.ftype-pdf svg{{color:#e01b24;}}\
         .buzon-attc.ftype-doc svg{{color:#3584e4;}}\
         .buzon-attc.ftype-sheet svg{{color:#2ec27e;}}\
         .buzon-attc.ftype-slides svg{{color:#ff7800;}}\
         .buzon-attc.ftype-archive svg{{color:#e5a50a;}}\
         .buzon-attc.ftype-audio svg{{color:#9141ac;}}\
         .buzon-attc.ftype-video svg{{color:#e6689e;}}\
         .buzon-attc.ftype-calendar svg{{color:#009e8f;}}\
         .buzon-attc.ftype-image svg{{color:#1c9bd8;}}\
         .buzon-attc.ftype-generic svg{{color:#8c8c8c;}}\
         .buzon-attn{{white-space:nowrap;overflow:hidden;text-overflow:ellipsis;min-width:0;}}\
         .buzon-attz{{opacity:0.55;white-space:nowrap;flex:none;}}\
         .buzon-attsave{{display:inline-flex;align-items:center;padding:0 7px;border:none;\
         border-inline-start:1px solid rgba(128,128,128,0.35);background:none;color:inherit;\
         cursor:pointer;opacity:0.6;}}\
         .buzon-attsave:hover{{opacity:1;background:rgba(128,128,128,0.18);}}\
         .buzon-attsave svg{{width:14px;height:14px;display:block;}}\
         .buzon-attsave svg,.buzon-attsave svg *{{fill:currentColor;}}\
         .buzon-atts.flash .buzon-attw{{animation:buzon-att-flash 1.2s ease-out;}}\
         @keyframes buzon-att-flash{{0%,40%{{box-shadow:0 0 0 3px rgba(53,132,228,0.7);}}\
         100%{{box-shadow:0 0 0 3px rgba(53,132,228,0);}}}}\
         .buzon-tag{{font-size:0.72em;font-weight:700;line-height:1.4;padding:1px 7px;\
         border-radius:9999px;white-space:nowrap;}}\
         .buzon-label{{max-width:16em;\
         overflow:hidden;text-overflow:ellipsis;}}\
         .buzon-addr{{opacity:0.55;font-size:0.9em;flex:1 1 0;max-width:max-content;\
         min-width:0;white-space:nowrap;overflow:hidden;text-overflow:clip;\
         position:relative;}}\
         /* The fade exists only while something actually collides with\
         the address (`clipped` is set by measurement, below). */\
         .buzon-addr.clipped{{\
         -webkit-mask-image:linear-gradient(to right,#000 calc(100% - 18px),transparent);\
         mask-image:linear-gradient(to right,#000 calc(100% - 18px),transparent);}}\
         /* Hovering a clipped address reveals it in full: an overlay\
         pinned to the span's spot, floating over whatever crowded it\
         out, gone when the cursor leaves. */\
         .buzon-addr.clipped:hover{{-webkit-mask-image:none;mask-image:none;\
         z-index:6;}}\
         /* Addresses act as links: click composes to them, right-click\
         offers Copy / New Message. */\
         /* Sender-authentication seal (#88): invisible until a\
         verdict arrives; a pass wears Bazaar's fixed blue (never the\
         system accent), problems the theme's warning and error colors.\
         Sized in em and anchored to the text baseline, then eased\
         down to the name's optical centre with an em transform\
         (margins can't move it: WebKit synthesizes the missing\
         baseline before they apply) — a fixed centred pixel box sat\
         visibly low whenever a GNOME text scaling factor shrank the\
         type around it. */\
         .buzon-verify{{display:none;background:none;border:none;\
         padding:0 2px;margin-inline-start:2px;cursor:pointer;line-height:0;\
         align-self:baseline;transform:translateY(0.18em);flex:none;}}\
         .buzon-verify.on{{display:inline-flex;}}\
         .buzon-verify svg{{width:0.95em;height:0.95em;display:block;}}\
         .buzon-verify svg,.buzon-verify svg *{{fill:currentColor;}}\
         .buzon-verify.trust-pass{{color:#3584e4;}}\
         .buzon-verify.trust-unverified{{color:currentColor;opacity:0.4;}}\
         .buzon-verify.trust-suspicious{{color:{warning};}}\
         .buzon-verify.trust-fail{{color:{error};}}\
         /* The OpenPGP chip (#133), a labelled pill since #300: the\
         verdict reads in words on a wash of its color. Text of its\
         own, so it sits on the name's baseline without the seal's\
         nudge. */\
         .buzon-pgp{{display:none;font:inherit;font-size:0.78em;font-weight:600;\
         border:none;border-radius:999px;padding-block:0.1em;padding-inline:0.45em 0.6em;\
         margin-inline-start:4px;cursor:pointer;align-items:center;gap:0.3em;\
         align-self:baseline;white-space:nowrap;flex:none;color:inherit;\
         background:rgba(128,128,128,0.16);}}\
         .buzon-pgp.on{{display:inline-flex;}}\
         .buzon-pgp svg{{width:1.1em;height:1.1em;display:none;flex:none;}}\
         .buzon-pgp svg,.buzon-pgp svg *{{fill:currentColor;}}\
         .buzon-pgp.enc svg:first-child{{display:block;}}\
         .buzon-pgp.sig svg:nth-child(2){{display:block;}}\
         .buzon-pgp.pgp-good{{color:{success};background:color-mix(in srgb,{success} 16%,transparent);}}\
         .buzon-pgp.pgp-warn{{color:{warning};background:color-mix(in srgb,{warning} 16%,transparent);}}\
         .buzon-pgp.pgp-bad{{color:{error};background:color-mix(in srgb,{error} 14%,transparent);}}\
         .buzon-pgp:hover{{filter:brightness(1.1);}}\
         /* The sensitivity label (#411): the OpenPGP pill's shape in a\
         neutral wash, since a label is a statement, not a verdict.\
         Zero flex-basis and a small share, like the address: the\
         name fills first and a long label ellipsizes before it. */\
         .buzon-sens{{display:none;font-size:0.78em;font-weight:600;\
         border-radius:999px;padding-block:0.1em;padding-inline:0.45em 0.6em;\
         margin-inline-start:4px;align-items:center;gap:0.3em;\
         align-self:baseline;white-space:nowrap;flex:1 1 0;min-width:3.5em;max-width:max-content;\
         background:rgba(128,128,128,0.16);cursor:default;}}\
         .buzon-sens.on{{display:inline-flex;}}\
         .buzon-sens svg{{width:1em;height:1em;flex:none;}}\
         .buzon-sens svg,.buzon-sens svg *{{fill:var(--buzon-sens,currentColor);}}\
         .buzon-sens-text{{overflow:hidden;text-overflow:ellipsis;}}\
         /* The icons alone (Settings, #300): the chip as it was before\
         it had words, sitting on the name's line like the seal. */\
         body[data-buzon-pgpicons] .buzon-pgp{{background:none;padding:0 2px;\
         margin-inline-start:2px;gap:1px;font-size:inherit;line-height:0;\
         transform:translateY(0.18em);}}\
         body[data-buzon-pgpicons] .buzon-pgp svg{{width:0.95em;height:0.95em;}}\
         body[data-buzon-pgpicons] .buzon-pgp-text{{display:none;}}\
         .buzon-mail{{cursor:pointer;}}\
         .buzon-mail:hover{{text-decoration:underline;}}\
         /* Styled after the app's own context menus (context_menu.rs +\
         .context-menu-list in styles.css): same padding, min width,\
         row shape and weight, so both menu families read as one. */\
         .buzon-mailmenu-scrim{{position:fixed;left:0;top:0;right:0;bottom:0;\
         z-index:98;}}\
         .buzon-mailmenu{{position:fixed;z-index:99;background:{bg};\
         border:1px solid rgba(128,128,128,0.2);border-radius:12px;\
         box-shadow:0 6px 24px rgba(0,0,0,0.22);padding:6px;\
         min-width:190px;font-size:0.95em;}}\
         .buzon-mailmenu button{{display:block;width:100%;text-align:start;\
         background:none;border:none;padding:8px 10px;border-radius:6px;\
         color:inherit;cursor:pointer;font:inherit;font-weight:normal;}}\
         .buzon-mailmenu button:hover{{background:rgba(128,128,128,0.14);}}\
         .buzon-addr.clipped:hover::after{{content:attr(data-addr);\
         position:absolute;inset-inline-start:-6px;top:50%;transform:translateY(-50%);\
         background:{bg};border-radius:6px;padding:1px 6px;\
         white-space:nowrap;text-decoration:underline;}}\
         body:not(.buzon-conv) .buzon-addr.clipped:hover::after{{\
         background:{chrome};}}\
         .buzon-date{{opacity:0.55;font-size:0.85em;flex:none;white-space:nowrap;}}\
         .buzon-read{{display:none;flex:none;align-self:center;margin-inline-end:-3px;}}\
         .buzon-read.asked,.buzon-read.read{{display:inline-flex;}}\
         .buzon-read svg{{width:1.05em;height:1.05em;display:block;}}\
         .buzon-read svg,.buzon-read svg *{{fill:currentColor;}}\
         .buzon-read.asked{{opacity:0.55;}}\
         .buzon-read.read{{color:{success};}}\
         .buzon-dot{{width:8px;height:8px;border-radius:50%;background:#3584e4;\
         flex:none;align-self:center;}}\
         .buzon-end{{height:1px;}}\
         .buzon-quote{{display:block;margin:0 16px 12px;padding:0 7px;\
         font:inherit;font-size:0.7em;line-height:1.45;letter-spacing:0.06em;\
         color:inherit;opacity:0.6;background:rgba(128,128,128,0.16);\
         border:0;border-radius:999px;cursor:pointer;}}\
         .buzon-quote:hover{{opacity:0.95;background:rgba(128,128,128,0.28);}}\
         .buzon-quote.open{{opacity:0.95;position:absolute;inset-inline-start:0;z-index:2;}}\
         /* The card's menu. Always visible and OUT OF FLOW: absolutely\
         pinned to the header's right edge, so no amount of header content\
         can push or wrap it. The line reserves the button's footprint as\
         fixed right padding instead. */\
         .buzon-hdr-line{{padding-inline-end:38px;}}\
         /* Anchored to the FIRST row's centre (the 26px avatar/sender\
         line), not the line box's — when the meta row wraps beneath,\
         the box grows downward but the button must not move. */\
         .buzon-acts-toggle{{display:block;\
         position:absolute;inset-inline-end:0;top:13px;transform:translateY(-50%);\
         color:{toggle_color};background:none;border:none;border-radius:6px;\
         padding:4px 8px;margin:0;cursor:pointer;\
         opacity:0.65;transition:opacity 120ms ease,background 120ms ease;}}\
         /* `svg *` too: the embedded symbolic paths carry their own fill\
         attribute, which a rule on the svg element alone can't beat. */\
         .buzon-acts-toggle svg{{width:14px;height:14px;display:block;}}\
         .buzon-acts-toggle svg,.buzon-acts-toggle svg *{{fill:currentColor;}}\
         .buzon-acts-toggle:hover{{opacity:1;background:rgba(128,128,128,0.18);}}\
         .buzon-folder{{padding:0.05em 0.45em;border-radius:0.7em;\
         font-size:0.78em;opacity:0.75;border:1px solid currentColor;\
         white-space:nowrap;flex:none;}}\
         .buzon-rcpt-toggle{{font:inherit;font-size:0.78em;color:inherit;\
         background:color-mix(in srgb,currentColor 10%,transparent);\
         border:none;border-radius:6px;padding:0.15em 0.6em;cursor:pointer;\
         white-space:nowrap;flex:none;transition:background 120ms ease;}}\
         .buzon-rcpt-toggle:hover{{background:color-mix(in srgb,currentColor 15%,transparent);}}\
         .buzon-rcpt-toggle.open{{background:color-mix(in srgb,currentColor 20%,transparent);}}\
         /* Indented past the avatar (26px + 8px gap), so the recipients\
         align with the sender's name — as does the wrapped meta line. */\
         .buzon-rcpt{{font-size:0.85em;opacity:0.75;\
         user-select:text;overflow-wrap:anywhere;margin:2px 0 0;margin-inline-start:34px;}}\
         .buzon-rcpt div{{margin-top:2px;}}\
         .buzon-loading{{opacity:0.5;padding:16px;}}\
         /* The Copied pill after Ctrl+C: a toast the page draws itself,\
         since the reader has no toast overlay of its own. */\
         .buzon-copied{{position:fixed;left:50%;bottom:22px;transform:translateX(-50%) translateY(8px);\
         padding:8px 18px;border-radius:999px;background:rgba(40,40,40,0.92);color:#fff;\
         font:inherit;font-size:0.9em;box-shadow:0 2px 10px rgba(0,0,0,0.25);\
         opacity:0;pointer-events:none;transition:opacity 180ms ease,transform 180ms ease;z-index:50;}}\
         .buzon-copied.on{{opacity:1;transform:translateX(-50%) translateY(0);}}\
         /* Right to left (#366): what logical properties cannot say. */\
         html[dir=rtl] .buzon-addr.clipped{{\
         -webkit-mask-image:linear-gradient(to left,#000 calc(100% - 18px),transparent);\
         mask-image:linear-gradient(to left,#000 calc(100% - 18px),transparent);}}\
         html[dir=rtl] .buzon-addr.clipped:hover{{-webkit-mask-image:none;mask-image:none;}}\
         /* Each chip's words in their own direction, as a GTK label\
         has them: an English 1 recipient in a right-to-left line\
         otherwise reads recipient 1. */\
         html[dir=rtl] :is(.buzon-rcpt-toggle,.buzon-date,\
         .buzon-folder,.buzon-rcpt div){{unicode-bidi:plaintext;}}\
         "
    )
}
