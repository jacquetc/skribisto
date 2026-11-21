// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Icons for the image viewer's toolbar. Line-style, `stroke="currentColor"`, on the 16 px
//! grid the other toolbars use, so they follow the theme. `res!` embeds each asset at
//! compile time and needs a literal path per call site, so there is one function per icon.

use teksilo::res;
use teksilo::widgets::IconWidget;

/// The previous image: a left chevron.
pub fn previous() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/viewer/previous.svg"))
}

/// The next image: a right chevron.
pub fn next() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/viewer/next.svg"))
}

/// Zoom out: a magnifier with a minus.
pub fn zoom_out() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/viewer/zoom-out.svg"))
}

/// Zoom in: a magnifier with a plus.
pub fn zoom_in() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/viewer/zoom-in.svg"))
}

/// Fit the window: four corners closing in on a frame.
pub fn fit() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/viewer/fit.svg"))
}

/// The picture's own size: "1:1".
pub fn actual_size() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/viewer/actual-size.svg"))
}

/// Open the note the picture is in: the binder's own note glyph, so the button names the
/// thing it opens the way the binder draws it.
pub fn open_note() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/binder/note.svg"))
}

/// Open in another application: a frame with an arrow leaving it.
pub fn open_external() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/viewer/open-external.svg"))
}

/// Save a copy: an arrow down into a tray.
pub fn save_copy() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/viewer/save-copy.svg"))
}

/// Close the viewer: a cross.
pub fn close() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/viewer/close.svg"))
}
