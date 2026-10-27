// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Find and replace inside one editor — the banner over the writing column.
//!
//! Distinct from the project-wide search dock: this one is scoped to the open
//! document, keeps the caret, and paints its matches through the same highlight
//! the caret band uses ([`highlight_of`]).

use super::*;

use teksilo::widgets::Shrinkable;

/// The narrowest the replacement field gets when the replace row runs short of room:
/// enough to see what is being typed into it.
const REPLACE_FIELD_MIN_WIDTH: f32 = 120.0;

/// The narrowest the Preserve-case checkbox gets: its box's own 24 px hit area, so
/// the box itself is never cut and only the label gives way.
const PRESERVE_CASE_MIN_WIDTH: f32 = 24.0;

/// A highlight format from a background `SurfaceRole` and an optional foreground
/// `TextRole` — so highlighted text keeps its contrast (a strong current-match
/// background needs an on-that-surface text colour, or the glyphs disappear into
/// it). Theme colours are f32 components (0..1); document highlight colours are
/// `u8` (0..255). Shared with the search preview's find-highlight session.
pub(crate) fn highlight_of(
    bg: SurfaceRole,
    fg: Option<TextRole>,
    colors: &teksilo::tokens::ColorTokens,
) -> HighlightFormat {
    let to_u8 = |v: f32| (v * 255.0).round().clamp(0.0, 255.0) as u8;
    let to_doc = |c: teksilo::tokens::Color| {
        DocColor::rgba(to_u8(c.r()), to_u8(c.g()), to_u8(c.b()), to_u8(c.a()))
    };
    HighlightFormat {
        background_color: Some(to_doc(bg.resolve(colors))),
        foreground_color: fg.map(|role| to_doc(role.resolve(colors))),
        ..Default::default()
    }
}

/// Shift+Enter in a find-bar field: back to the previous match.
///
/// Claimed in the *preview* of the field's wrapper, before the field sees the key. A
/// field submits on Enter whatever the modifiers, so a handler further up heard
/// Shift+Enter only after the field had acted on it: the query field had stepped
/// forward, the replace field had replaced. On each field's wrapper rather than on the
/// banner's row, so a focused button keeps Shift+Enter as its own.
fn shift_enter_steps_back(
    find: FindViewModel,
) -> impl FnMut(&WidgetEvent, &mut EventContext) -> EventResponse + 'static {
    move |ev, ctx| match ev {
        WidgetEvent::KeyDown {
            key: Key::Enter,
            modifiers,
            ..
        } if modifiers.shift() => {
            find.prev(ctx);
            EventResponse::Handled
        }
        _ => EventResponse::Ignored,
    }
}

/// The per-editor find banner: a raised strip with a query field, an "N of M"
/// counter, previous / next / close controls, and Escape-to-close. Bound to the
/// tab's [`FindViewModel`], which owns the [`FindSession`](teksilo::widgets::rich_text::FindSession)
/// that highlights the matches and the editor handle that scrolls the current one
/// into view.
///
/// A custom widget (not a plain function) because `build` needs a context: it
/// resolves the two highlight colours from the theme, creates the find session
/// lazily, and wires the reactive layer — re-run the query when it or the options
/// change, and re-derive the matches once per frame if an edit staled them.
pub(super) struct FindBanner {
    pub(super) find: FindViewModel,
    pub(super) child_id: Option<WidgetId>,
}

impl FindBanner {
    pub(super) fn new(find: FindViewModel) -> Self {
        Self {
            find,
            child_id: None,
        }
    }
}

impl std::fmt::Debug for FindBanner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FindBanner").finish()
    }
}

impl Widget for FindBanner {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Rebuild when the banner is (re)opened — that is the one moment we can
        // grab keyboard focus for the field (this widget is otherwise a dormant,
        // build-once child of the visibility gate).
        self.find.focus_seq_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );

        // Create the find session with theme-resolved highlight colours — the
        // current match in the accent colour with on-accent text (a solid, legible
        // highlight), the rest in a subtle accent tint that keeps the normal text
        // colour. Both paint-only, so they stay out of the accessibility tree.
        let colors = &ctx.theme().colors;
        let current = highlight_of(SurfaceRole::Accent, Some(TextRole::OnAccent), colors);
        let other = highlight_of(SurfaceRole::AccentSubtle, None, colors);
        self.find.ensure_session(current, other);

        // Reactive layer: re-run the query when it or the match options change,
        // and re-derive once per frame if an edit moved the matched offsets.
        let f = self.find.clone();
        ctx.effect(&self.find.query_signal(), move |_| f.refresh_query());
        let f = self.find.clone();
        ctx.effect(&self.find.case_sensitive_signal(), move |_| {
            f.refresh_query()
        });
        let f = self.find.clone();
        ctx.effect(&self.find.whole_word_signal(), move |_| f.refresh_query());
        // Find lives under `VisibleWhen`: when the banner is closed the
        // whole subtree is dormant, but `frame_tick` still fires. Skip
        // the re-derive while hidden so a closed banner does not keep
        // a multi-tab project busy.
        let f = self.find.clone();
        let active = ctx.activation_signal(ctx.self_id());
        let tick = ctx.frame_tick();
        ctx.effect(&tick, move |_| {
            if active.get() {
                f.tick();
            }
        });

        // "N of M" / "No results" — empty while the query is empty.
        let label = self
            .find
            .query_signal()
            .zip(&self.find.current_signal())
            .zip(&self.find.count_signal())
            .map(|((query, current), total)| {
                if query.trim().is_empty() {
                    String::new()
                } else if *total == 0 {
                    tr!(find_no_results()).resolve_now()
                } else {
                    tr!(find_count(current = *current as i64, total = *total as i64)).resolve_now()
                }
            });

        let prev = self.find.clone();
        let next = self.find.clone();
        let close = self.find.clone();
        let submit = self.find.clone();
        let esc = self.find.clone();
        let alt_a = self.find.clone();
        let rc_submit = self.find.clone();
        let rc_btn = self.find.clone();
        let ra_btn = self.find.clone();

        // The query field is built as a standalone widget so we can hold its id
        // and steer the on-open autofocus straight at it — the replace-mode toggle
        // sits to its left and would otherwise win `first_focusable_descendant`.
        let query_id = ctx.add(
            MaxSize::width(FIND_FIELD_MAX_WIDTH)
                .child(
                    TextInput::new(self.find.query_signal())
                        .placeholder(tr!(find_placeholder()))
                        .on_submit_fn(move |ctx| submit.submit(ctx)),
                )
                .on_key_preview(shift_enter_steps_back(self.find.clone())),
        );

        // Whether the replace half is on offer at all. It is not over a **page** of
        // documents — a stream's rows, a note's In-prose reading — because rewriting
        // across many documents is Search & Replace's job: it has the preview and the
        // scope picker, and a Replace All here would leave one undo per row. Finding is
        // untouched; only the rewriting half goes. Reactive rather than read once,
        // because a split changes a page's row count under an open banner.
        let may_replace = self.find.single_document_signal();
        let replace_open = self
            .find
            .replace_mode_signal()
            .zip(&may_replace)
            .map(|(mode, single)| *mode && *single);

        // The find row. A left replace-mode toggle (⇄) discloses the replace row;
        // then a capped-width query field, the count, the prev/next chevrons, and
        // the Match-case / Whole-word toggles; a spacer pushes Close to the trailing
        // edge. All buttons are flat (`.toolbar()` = ghost).
        let find_row = HStack::new()
            .spacing(3.0)
            .child(VisibleWhen::new(
                may_replace,
                IconButton::new(crate::icons::find::replace_icon())
                    .toolbar()
                    .toggle(self.find.replace_mode_signal())
                    .tooltip(tr!(find_replace_toggle())),
            ))
            .child(query_id)
            .child(
                Padding::symmetric(0.0, 6.0).child(
                    TextWidget::new(lit!(""))
                        .text(label)
                        .style(TextStyleRole::Small)
                        .color(TextRole::Secondary),
                ),
            )
            .child(
                IconButton::new(crate::icons::find::nav_prev_icon())
                    .toolbar()
                    .tooltip(tr!(find_previous()))
                    .on_activate_fn(move |ctx| prev.prev(ctx)),
            )
            .child(
                IconButton::new(crate::icons::find::nav_next_icon())
                    .toolbar()
                    .tooltip(tr!(find_next()))
                    .on_activate_fn(move |ctx| next.next(ctx)),
            )
            .child(
                IconButton::new(crate::icons::find::case_icon())
                    .toolbar()
                    .toggle(self.find.case_sensitive_signal())
                    .tooltip(tr!(find_opt_case())),
            )
            .child(
                IconButton::new(crate::icons::find::whole_word_icon())
                    .toolbar()
                    .toggle(self.find.whole_word_signal())
                    .tooltip(tr!(find_opt_whole_word())),
            )
            .child(Expand::horizontal().child(FixedSize::new().height(1.0)))
            .child(
                IconButton::clear()
                    .toolbar()
                    .tooltip(tr!(find_close()))
                    .on_activate_fn(move |ctx| close.close_and_refocus(ctx)),
            );

        // The replace row (disclosed when replace mode is on): a capped-width
        // replacement field (Enter = replace the current match), Replace / Replace
        // All actions, and the Preserve-case toggle.
        //
        // The field and the checkbox are `Shrinkable` because the banner is as wide
        // as its widest row, and a banner wider than its pane widens the whole tab
        // past the pane's edge, the prose with it. A `Checkbox` in a row measures at
        // its whole label and reports no way to shrink, so in a narrow pane (a
        // 512 px one between the binder and the Inspector, in French) this row ran
        // 120 px past the edge. The field gives way twice as fast as the label, so
        // the label stays readable longer; neither goes below a usable floor.
        let replace_row = HStack::new()
            .spacing(6.0)
            .child(
                Shrinkable::new()
                    .shrink(2.0)
                    .min_width(REPLACE_FIELD_MIN_WIDTH)
                    .child(
                        MaxSize::width(FIND_FIELD_MAX_WIDTH)
                            .child(
                                TextInput::new(self.find.replacement_signal())
                                    .placeholder(tr!(find_replace_placeholder()))
                                    .on_submit_fn(move |ctx| rc_submit.replace_current(ctx)),
                            )
                            .on_key_preview(shift_enter_steps_back(self.find.clone())),
                    ),
            )
            .child(
                Button::new(tr!(find_replace()))
                    .variant(ButtonVariant::Tinted)
                    .on_activate_fn(move |ctx| rc_btn.replace_current(ctx)),
            )
            .child(
                Button::new(tr!(find_replace_all()))
                    .variant(ButtonVariant::Filled)
                    .on_activate_fn(move |ctx| ra_btn.replace_all(ctx)),
            )
            .child(Shrinkable::new().min_width(PRESERVE_CASE_MIN_WIDTH).child(
                Checkbox::new(self.find.preserve_case_signal()).label(tr!(find_preserve_case())),
            ))
            .child(Expand::horizontal().child(FixedSize::new().height(1.0)));

        // Enter (in a field) navigates / replaces via `on_submit`, and Shift+Enter in
        // either field steps back (`shift_enter_steps_back`); the row handles the
        // rest: Alt+A replaces all, Escape closes (find-bar conventions).
        //
        // Alt+A is claimed in the row's *preview*, ahead of the focused field: a key
        // reaches the row's bubbling handler only after the field has had it, and a
        // field types the letter an Alt+A carries. It matches `Key::A`, which is how
        // a letter arrives (it never matched as `Key::Character('a')`, so the chord
        // did nothing), and not with Ctrl, which is AltGr on Windows: AltGr+A types a
        // letter on several layouts (ą in Polish). On macOS, Option+A composes a
        // character (å) and arrives as that, so the chord is Windows' and Linux's.
        let keyed = VStack::new()
            .spacing(4.0)
            .child(find_row)
            .child(VisibleWhen::new(replace_open, replace_row))
            .on_key_preview(move |ev, ctx| match ev {
                WidgetEvent::KeyDown {
                    key: Key::A,
                    modifiers,
                    ..
                } if modifiers.alt() && !modifiers.ctrl() && alt_a.replace_mode_signal().get() => {
                    alt_a.replace_all(ctx);
                    EventResponse::Handled
                }
                _ => EventResponse::Ignored,
            })
            .on_key(move |ev, ctx| match ev {
                WidgetEvent::KeyDown {
                    key: Key::Escape, ..
                } => {
                    esc.close_and_refocus(ctx);
                    EventResponse::Handled
                }
                _ => EventResponse::Ignored,
            });

        let banner = teksu!(
            Panel {
                background: SurfaceRole::Raised
                corner_radius: 0.0
                padding: 0.0
                child: Padding::symmetric(6.0, 6.0) {
                    child: keyed
                }
            }
        );
        let root = ctx.add(banner);
        // Autofocus the query field when the banner is open — drill into the query
        // field's own subtree, not the whole row, so focus lands on the SearchField
        // and never on the replace-mode toggle to its left. Fires on open via the
        // `focus_seq` rebuild binding above; harmless on a closed-state rebuild
        // (focus is a no-op on a dormant subtree).
        if self.find.visible_signal().get()
            && let Some(field) = ctx.first_focusable_descendant(query_id)
        {
            ctx.focus(field);
        }
        self.child_id = Some(root);
        self.child_id.into_iter().collect()
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.child_id
            .and_then(|id| ctx.child_size(id, proposal))
            .unwrap_or(Size::new(0.0, 0.0))
            .into()
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        for child in children.iter_mut() {
            child.origin = Point::new(bounds.x, bounds.y);
            child.size = bounds.size();
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.child_id.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use teksilo::core::widget_tree::WidgetTree;

    fn doc(text: &str) -> teksilo::text_document::TextDocument {
        let d = teksilo::text_document::TextDocument::new();
        let _ = d.set_plain_text(text);
        d
    }

    /// The width the banner reports when it is laid out in `width` pixels.
    fn reported_width(query: &str, width: f32) -> f32 {
        let vm = crate::search::FindViewModel::new(doc("the cat and the hat")).for_item(7);
        vm.open();
        vm.query_signal().set(query.into());
        let mut tree = WidgetTree::new();
        let id = tree.add(FindBanner::new(vm));
        tree.layout(SizeProposal::exact(width, 600.0));
        let _ = tree.render();
        tree.bounds(id).width
    }

    /// **The banner may not be wider than the space it is given.**
    ///
    /// Its own row is a query field, a count, two chevrons, two option toggles and a
    /// close button, and only the field is width-capped. In a narrow window the rest
    /// still added up past the tab, and a `VStack` reports the widest child it has:
    /// the banner then decided how wide the whole editor was, and the margin lane --
    /// the last thing in the row beneath it -- was pushed off the right-hand edge.
    ///
    /// Reported as the lane changing width while a search was typed and vanishing
    /// when nothing matched. Nothing was wrong with the lane. The count reads "",
    /// then "1 of 3", then the longest of the three, "No results", so the overhang
    /// grew as the query was typed and was widest exactly when it found nothing.
    /// `VisibleWhen`'s own note records this same geometry doing the same thing when
    /// the banner was closed, which is why that gate is not a `Switcher`.
    #[test]
    fn the_banner_never_claims_more_width_than_it_is_given() {
        for width in [320.0, 400.0, 622.0, 900.0] {
            for query in ["", "cat", "zzqqxx"] {
                let got = reported_width(query, width);
                assert!(
                    got <= width + 0.5,
                    "at {width}px with query {query:?} the banner claimed {got}px"
                );
            }
        }
    }

    /// The three states of the count are the three widths that mattered, so they are
    /// asserted against each other rather than only against the bound: whatever the
    /// row does inside, the banner presents one width to its parent.
    #[test]
    fn the_count_does_not_change_what_the_banner_claims() {
        let narrow = 400.0;
        let empty = reported_width("", narrow);
        let hits = reported_width("cat", narrow);
        let none = reported_width("zzqqxx", narrow);
        assert!(
            (empty - hits).abs() < 0.5 && (hits - none).abs() < 0.5,
            "empty={empty} hits={hits} none={none}"
        );
    }
}
