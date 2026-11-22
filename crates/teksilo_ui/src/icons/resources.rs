// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The file-type icons of the Resources dock. Line-style, `stroke="currentColor"`, on
//! the 16 px grid the other panels use, and the app's own: the desktop's icon theme is
//! a Linux convention, and a resource has to look the same on every system. `res!`
//! needs a literal path per call site, so there is one function per icon.

use teksilo::res;
use teksilo::widgets::IconWidget;

use crate::resources::file_kind::FileKind;

/// The icon for a resource of `kind`.
pub fn for_kind(kind: FileKind) -> IconWidget {
    match kind {
        FileKind::Picture => IconWidget::from_svg_icon(res!("assets/icons/resources/picture.svg")),
        FileKind::Pdf => IconWidget::from_svg_icon(res!("assets/icons/resources/pdf.svg")),
        FileKind::Audio => IconWidget::from_svg_icon(res!("assets/icons/resources/audio.svg")),
        FileKind::Video => IconWidget::from_svg_icon(res!("assets/icons/resources/video.svg")),
        FileKind::Text => IconWidget::from_svg_icon(res!("assets/icons/resources/document.svg")),
        FileKind::Ebook => IconWidget::from_svg_icon(res!("assets/icons/resources/ebook.svg")),
        FileKind::Archive => IconWidget::from_svg_icon(res!("assets/icons/resources/archive.svg")),
        FileKind::Other => IconWidget::from_svg_icon(res!("assets/icons/resources/file.svg")),
        FileKind::WebLink => IconWidget::from_svg_icon(res!("assets/icons/resources/web-link.svg")),
    }
}

/// Add a web link, on the dock's header.
pub fn add_link() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/resources/web-link.svg"))
}
