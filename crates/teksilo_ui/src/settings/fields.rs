// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The settings window's own small vocabulary of view helpers.
//!
//! Nothing here knows which page it is on: a field label, a slider with a live
//! readout, the breadcrumb frame a pane's body sits in, the centred placeholder
//! a category with no settings yet shows. They sit together because they are
//! narrower than [`crate::shared`] — which is for what several *features* spell
//! the same way — and wider than any one pane.

use skribisto_model::counting::CountingMethodSetting;
use teksilo::canvas::svg::SvgIcon;
use teksilo::i18n::{LocalizedString, localized};
use teksilo::prelude::*;
use teksilo::widgets::{
    Breadcrumb, BreadcrumbItem, Center, Divider, Expand, FixedSize, GroupHeader, HStack,
    IconWidget, Padding, ScrollArea, Slider, TextWidget, VStack,
};

use super::nav::Sec;

// ── Small view helpers ───────────────────────────────────────────────────────

/// A left-column field label (dimmed, small) — matches the design's `--tx2`.
/// `pub(crate)` so `panes::backup` shares the exact same row-label style as
/// every built-in pane.
pub(crate) use crate::shared::text::{field_label, hint};

/// A lowercase-titled section header + trailing rule (design's group headers).
/// `pub(crate)` so the backup panes reuse the identical group-header treatment.
pub(crate) fn group(text: LocalizedString) -> GroupHeader {
    GroupHeader::new(text)
        .style(TextStyleRole::SmallBold)
        .color(TextRole::Secondary)
}

/// A slider with a live value readout on its right, in a fixed 300 px cell.
pub(crate) fn slider_field(
    value: Signal<f32>,
    min: f32,
    max: f32,
    step: f32,
    fmt: impl Fn(f32) -> String + 'static,
) -> impl Widget + 'static {
    slider_field_inner(value, min, max, step, fmt, None)
}

/// Like [`slider_field`], with a plain tooltip on the slider (for short
/// explanations that used to be body-copy hints under the control).
pub(crate) fn slider_field_tipped(
    value: Signal<f32>,
    min: f32,
    max: f32,
    step: f32,
    fmt: impl Fn(f32) -> String + 'static,
    tip: LocalizedString,
) -> impl Widget + 'static {
    slider_field_inner(value, min, max, step, fmt, Some(tip))
}

fn slider_field_inner(
    value: Signal<f32>,
    min: f32,
    max: f32,
    step: f32,
    fmt: impl Fn(f32) -> String + 'static,
    tip: Option<LocalizedString>,
) -> impl Widget + 'static {
    let seed = fmt(value.get());
    let text = value.map(move |v| fmt(*v));
    let readout = TextWidget::new(LocalizedString::literal(seed))
        .text(text)
        .style(TextStyleRole::Small)
        .color(TextRole::Secondary);
    let mut slider = Slider::new(value, min, max).step(step);
    if let Some(tip) = tip {
        slider = slider.tooltip(tip);
    }
    FixedSize::new().width(300.0).child(
        HStack::new()
            .spacing(12.0)
            .child(Expand::horizontal().child(slider))
            .child(FixedSize::new().width(52.0).child(readout)),
    )
}

/// The breadcrumb trail shown at the top of a pane (`Parent › Current`).
pub(crate) fn crumb(parent: Option<LocalizedString>, current: LocalizedString) -> Breadcrumb {
    let mut b = Breadcrumb::new();
    if let Some(p) = parent {
        b = b.item(BreadcrumbItem::new(p));
    }
    b.item(BreadcrumbItem::current(current))
}

/// The right-pane frame: breadcrumb header · rule · scrollable content.
pub(crate) fn pane_frame(
    breadcrumb: impl Widget + 'static,
    content: impl Widget + 'static,
) -> impl Widget {
    VStack::new()
        .spacing(0.0)
        .child(
            FixedSize::new()
                .height(46.0)
                .child(Padding::symmetric(13.0, 22.0).child(breadcrumb)),
        )
        .child(Expand::horizontal().child(Divider::new()))
        .child(
            Expand::vertical()
                .child(ScrollArea::new().child(Padding::symmetric(20.0, 24.0).child(content))),
        )
}

/// The centered placeholder body for a not-yet-implemented category.
pub(crate) fn empty_content(icon: &'static SvgIcon) -> impl Widget {
    Center::new().child(
        VStack::new()
            .spacing(10.0)
            .child(
                IconWidget::from_svg_icon(icon)
                    .icon_size(40.0)
                    .color(TextRole::Disabled),
            )
            .child(
                TextWidget::new(tr!(settings_empty_title()))
                    .style(TextStyleRole::Body)
                    .color(TextRole::Secondary),
            )
            .child(hint(tr!(settings_empty_hint()))),
    )
}

/// A section's displayed title — static, except the open project's, which reads
/// "Work: `<title>`".
///
/// Composed per resolve rather than once, so a runtime locale switch reaches the
/// "Work" half of it (the project's own title is data and stays as typed).
pub(crate) fn section_title(sec: Sec, work_title: &str) -> LocalizedString {
    match sec {
        Sec::Work => {
            let title = work_title.to_string();
            localized(move || format!("{}: {}", tr!(settings_sec_work()).resolve_now(), title))
        }
        other => other.label(),
    }
}

/// A full empty pane (breadcrumb + placeholder).
pub(crate) fn empty_pane(
    parent: Option<LocalizedString>,
    current: LocalizedString,
    icon: &'static SvgIcon,
) -> impl Widget {
    pane_frame(crumb(parent, current), empty_content(icon))
}

/// `CountingMethodSetting` ⟷ the `RadioGroup`'s `usize` selection. The order here
/// is the radio order in `goals_pane`; keep the two in step.
pub(crate) fn method_to_index(m: CountingMethodSetting) -> usize {
    match m {
        CountingMethodSetting::Auto => 0,
        CountingMethodSetting::Whitespace => 1,
        CountingMethodSetting::UnicodeWords => 2,
        CountingMethodSetting::CjkHybrid => 3,
    }
}

pub(crate) fn index_to_method(i: usize) -> CountingMethodSetting {
    match i {
        1 => CountingMethodSetting::Whitespace,
        2 => CountingMethodSetting::UnicodeWords,
        3 => CountingMethodSetting::CjkHybrid,
        _ => CountingMethodSetting::Auto,
    }
}
