//! Settings → Date and Time.

use super::super::{PrefInput, ViewSender};
use crate::i18n::i18n;

/// The page, and the rows on it that the window reaches later.
#[derive(Debug)]
pub(in crate::ui::preferences) struct DateTimePage {
    pub(in crate::ui::preferences) page: ndl::PreferencesPage,
    pub(in crate::ui::preferences) date_style_row: ndl::ComboRow,
    pub(in crate::ui::preferences) clock_style_row: ndl::ComboRow,
}

impl DateTimePage {
    pub(in crate::ui::preferences) fn build(sender: &ViewSender) -> Self {
        relm4::view! {
            page = ndl::PreferencesPage {
                add = &ndl::PreferencesGroup {
                    set_title: &i18n("Date and Time"),
                    set_description: Some(
                        i18n("Dates follow the system unless a format is chosen here.").as_str()
                    ),

                    #[name = "date_style_row"]
                    ndl::ComboRow {
                        set_title: &i18n("Date format"),
                        connect_selected_notify[sender] => move |row| {
                            sender.input(PrefInput::ChangeDateStyle(row.selected()));
                        },
                    },

                    #[name = "clock_style_row"]
                    ndl::ComboRow {
                        set_title: &i18n("Clock"),
                        connect_selected_notify[sender] => move |row| {
                            sender.input(PrefInput::ChangeClockStyle(row.selected()));
                        },
                    },
                },
            }
        }
        Self {
            page,
            date_style_row,
            clock_style_row,
        }
    }
}
