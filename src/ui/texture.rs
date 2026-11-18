// SPDX-License-Identifier: AGPL-3.0-or-later
//! A decoded pixbuf as a texture. `gdk::Texture::for_pixbuf` is deprecated
//! since GTK 4.20, so the pixels are handed over as a memory texture.

use gtk::gdk;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::prelude::*;

pub(crate) fn from_pixbuf(pixbuf: &Pixbuf) -> gdk::Texture {
    let format = if pixbuf.has_alpha() {
        gdk::MemoryFormat::R8g8b8a8
    } else {
        gdk::MemoryFormat::R8g8b8
    };
    gdk::MemoryTexture::new(
        pixbuf.width(),
        pixbuf.height(),
        format,
        &pixbuf.read_pixel_bytes(),
        pixbuf.rowstride() as usize,
    )
    .upcast()
}
