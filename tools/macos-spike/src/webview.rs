// SPDX-License-Identifier: AGPL-3.0-or-later
//! A `WKWebView` placed over a rectangle of a GTK window.

use std::ffi::c_void;

use block2::RcBlock;
use objc2::rc::Retained;
use objc2::runtime::{AnyObject, ProtocolObject};
use objc2::{define_class, msg_send, DefinedClass, MainThreadMarker, MainThreadOnly};
use objc2_app_kit::{NSImage, NSView, NSWindow};
use objc2_foundation::{NSError, NSObject, NSObjectProtocol, NSPoint, NSRect, NSSize, NSString};
use objc2_web_kit::{
    WKScriptMessage, WKScriptMessageHandler, WKUserContentController, WKWebView,
    WKWebViewConfiguration,
};

/// The name JavaScript posts to: `window.webkit.messageHandlers.buzon.postMessage(...)`.
pub const HANDLER_NAME: &str = "buzon";

struct HandlerIvars {
    on_message: Box<dyn Fn(String)>,
}

define_class!(
    // The receiver of `postMessage`, which WebKit calls on the main thread.
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "BuzonScriptHandler"]
    #[ivars = HandlerIvars]
    struct ScriptHandler;

    unsafe impl NSObjectProtocol for ScriptHandler {}

    unsafe impl WKScriptMessageHandler for ScriptHandler {
        #[unsafe(method(userContentController:didReceiveScriptMessage:))]
        fn did_receive(&self, _controller: &WKUserContentController, message: &WKScriptMessage) {
            let body = unsafe { message.body() };
            (self.ivars().on_message)(describe(&body));
        }
    }
);

impl ScriptHandler {
    fn new(mtm: MainThreadMarker, on_message: Box<dyn Fn(String)>) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(HandlerIvars { on_message });
        unsafe { msg_send![super(this), init] }
    }
}

/// A JavaScript value as text: strings as they are, anything else by its description.
fn describe(value: &AnyObject) -> String {
    match value.downcast_ref::<NSString>() {
        Some(text) => text.to_string(),
        None => format!("{value:?}"),
    }
}

/// A web view that lives in a window's content view.
pub struct NativeWebView {
    view: Retained<WKWebView>,
    content: Retained<NSView>,
    _handler: Retained<ScriptHandler>,
}

impl NativeWebView {
    /// Adds a web view to `ns_window` (an `NSWindow *`), hidden until [`set_frame`](Self::set_frame)
    /// gives it a place. `on_message` receives what JavaScript posts to [`HANDLER_NAME`].
    ///
    /// Returns `None` off the main thread, or when the window has no content view.
    ///
    /// # Safety
    ///
    /// `ns_window` must point to a live `NSWindow`.
    pub unsafe fn attach(
        ns_window: *mut c_void,
        on_message: impl Fn(String) + 'static,
    ) -> Option<Self> {
        let mtm = MainThreadMarker::new()?;
        let window: &NSWindow = unsafe { (ns_window as *const NSWindow).as_ref()? };
        let content = window.contentView()?;

        let config = unsafe { WKWebViewConfiguration::new(mtm) };
        let handler = ScriptHandler::new(mtm, Box::new(on_message));
        unsafe {
            config.userContentController().addScriptMessageHandler_name(
                ProtocolObject::from_ref(&*handler),
                &NSString::from_str(HANDLER_NAME),
            );
        }
        let view = unsafe {
            WKWebView::initWithFrame_configuration(WKWebView::alloc(mtm), NSRect::ZERO, &config)
        };
        view.setHidden(true);
        content.addSubview(&view);
        Some(Self {
            view,
            content,
            _handler: handler,
        })
    }

    /// Puts the web view over `x`, `y`, `width`, `height`, in GTK's coordinates: points, from the
    /// window's top left corner.
    pub fn set_frame(&self, x: f64, y: f64, width: f64, height: f64) {
        // AppKit's own origin is at the bottom left unless the content view says it is flipped.
        let y = if self.content.isFlipped() {
            y
        } else {
            self.content.bounds().size.height - y - height
        };
        self.view
            .setFrame(NSRect::new(NSPoint::new(x, y), NSSize::new(width, height)));
        self.view.setHidden(width < 1.0 || height < 1.0);
    }

    /// Shows or hides the web view: it sits above everything GTK draws, so a popover that opens
    /// over it has to hide it.
    pub fn set_visible(&self, visible: bool) {
        self.view.setHidden(!visible);
    }

    pub fn load_html(&self, html: &str) {
        let html = NSString::from_str(html);
        unsafe { self.view.loadHTMLString_baseURL(&html, None) };
    }

    /// Runs `script` and hands `done` its result as text, or the error's description.
    pub fn evaluate_javascript(
        &self,
        script: &str,
        done: impl Fn(Result<String, String>) + 'static,
    ) {
        let script = NSString::from_str(script);
        let block = RcBlock::new(move |result: *mut AnyObject, error: *mut NSError| {
            if let Some(error) = unsafe { error.as_ref() } {
                done(Err(error.localizedDescription().to_string()));
            } else {
                match unsafe { result.as_ref() } {
                    Some(value) => done(Ok(describe(value))),
                    None => done(Ok(String::new())),
                }
            }
        });
        unsafe {
            self.view
                .evaluateJavaScript_completionHandler(&script, Some(&block));
        }
    }

    /// Renders what the web view shows; `done` gets its size in points, or `None` when it fails.
    pub fn snapshot(&self, done: impl Fn(Option<(f64, f64)>) + 'static) {
        let block = RcBlock::new(move |image: *mut NSImage, _error: *mut NSError| {
            done(unsafe { image.as_ref() }.map(|image| {
                let size = image.size();
                (size.width, size.height)
            }));
        });
        unsafe {
            self.view
                .takeSnapshotWithConfiguration_completionHandler(None, &block);
        }
    }
}
