// SPDX-License-Identifier: AGPL-3.0-or-later
//! A notice or question over a window, built with its parent and presented
//! later, as the app's dialogs have always been written. An
//! `ndl::AlertDialog` underneath: a sheet inside a Nodalia window, or a small
//! window of its own over any other.

use std::cell::RefCell;
use std::ops::Deref;

use gtk::prelude::*;

thread_local! {
    /// The dialogs on screen, newest last, for the showcase to find.
    static SHOWING: RefCell<Vec<MessageDialog>> = const { RefCell::new(Vec::new()) };
}

#[derive(Clone)]
pub(crate) struct MessageDialog {
    alert: ndl::AlertDialog,
    parent: Option<gtk::Window>,
}

impl Deref for MessageDialog {
    type Target = ndl::AlertDialog;

    fn deref(&self) -> &ndl::AlertDialog {
        &self.alert
    }
}

impl MessageDialog {
    pub(crate) fn new(
        parent: Option<&impl IsA<gtk::Window>>,
        heading: Option<&str>,
        body: Option<&str>,
    ) -> Self {
        Self {
            alert: ndl::AlertDialog::new(heading, body),
            parent: parent.map(|p| p.as_ref().clone()),
        }
    }

    /// The window the dialog was built over.
    pub(crate) fn transient_for(&self) -> Option<gtk::Window> {
        self.parent.clone()
    }

    pub(crate) fn present(&self) {
        let me = self.clone();
        SHOWING.with(|s| s.borrow_mut().push(me));
        let alert = self.alert.clone();
        self.alert.dialog().connect_closed(move |_| {
            SHOWING.with(|s| {
                s.borrow_mut()
                    .retain(|d| d.alert.dialog() != alert.dialog())
            });
        });
        self.alert.present(self.parent.as_ref());
    }

    /// Calls `f` with the response id; with `Some(detail)`, only for that one.
    pub(crate) fn connect_response<F: Fn(&Self, &str) + 'static>(
        &self,
        detail: Option<&str>,
        f: F,
    ) {
        let parent = self.parent.clone();
        self.alert.connect_response(detail, move |alert, id| {
            let me = MessageDialog {
                alert: alert.clone(),
                parent: parent.clone(),
            };
            f(&me, id);
        });
    }

    /// The widest the dialog's content gets; the height follows the content.
    pub(crate) fn set_size_request(&self, width: i32, _height: i32) {
        if width > 0 {
            self.alert.dialog().set_content_width(width);
        }
    }

    pub(crate) fn is_visible(&self) -> bool {
        self.alert.dialog().is_visible() && self.alert.dialog().parent().is_some()
    }

    /// The dialog itself, for capturing it.
    pub(crate) fn widget(&self) -> gtk::Widget {
        self.alert.dialog().clone().upcast()
    }

    /// The newest dialog on screen.
    pub(crate) fn showing() -> Option<Self> {
        SHOWING.with(|s| s.borrow().last().cloned())
    }
}
