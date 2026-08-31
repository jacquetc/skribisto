// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! One convention, and the drift tests that keep it.
//!
//! **A label that can hold user text, in a row, must truncate.** Titles, note names,
//! tag names, theme names, file paths — anything a writer typed — have no length, and
//! a row is a fixed width.
//!
//! ## Why wrapping is not the safe default here
//!
//! `TextOverflow::Wrap` is `TextWidget`'s default, and it sounds harmless: text too
//! wide for its box moves to the next line. It does not work in a row, for two reasons
//! that compound.
//!
//! A wrapping label asked to measure under an **unbounded** width has no basis on which
//! to wrap, so it measures as a single line at its full intrinsic width — and an
//! `HStack` proposes exactly that to its children. The label therefore reports the
//! whole string, the stack is over-constrained, and everything after it in the row (a
//! rename button, a rule, an options menu, a trailing action) is pushed past the edge.
//! Worse, `Wrap` reports *rigid*: the deficit has nowhere to go, so the text does not
//! shrink, it simply paints outside its parent. That is how a long chapter title came
//! to paint out of the outline dock and across the editor beside it.
//!
//! An ellipsis mode fixes both halves at once. It measures single-line against whatever
//! width it is offered, and it advertises a shrink weight — so an over-constrained
//! stack takes the deficit out of the label, and the label says so with a "…".
//!
//! ## `max_lines(1)` is not the way to say "one line"
//!
//! It reads like it should be, and it is the one trap here worth naming. `max_lines`
//! caps how many lines are *painted*; the widget still measures in `Wrap` mode, so it
//! still reports the full width and still over-constrains the row. The text is then cut
//! with no ellipsis at all, which is the worst outcome available: the row is broken
//! *and* the reader cannot tell the name they are looking at is not the whole name. Use
//! `TextWidget::single_line`, which is the shorthand for
//! `overflow(TextOverflow::Ellipsis(EllipsisMode::Trailing))`. `max_lines` above one —
//! clamping a synopsis preview to six lines inside a box of known width — is what it is
//! for and stays legal.
//!
//! ## Where this does not apply
//!
//! Prose in a bounded container is meant to wrap: a settings description, a panel's
//! body, a tooltip (teksilo bounds one at `TOOLTIP_MAX_WIDTH` and wraps there, so
//! eliding a tooltip would throw away the very text it exists to show). The rule is
//! about *rows*, and the blanket part of it — the drift test below — is scoped to the
//! two row widgets, where there is no such thing as a legitimate wrap.

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    /// Every `.rs` under this crate's `src/`.
    ///
    /// A directory walk rather than a hand-listed set, for the reason
    /// `settings_keys`' own drift test gives: what these look for is a site added in a
    /// file nobody thought to add here, and a fixed list is blind to exactly that.
    fn sources() -> Vec<(PathBuf, String)> {
        fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
            let Ok(entries) = std::fs::read_dir(dir) else {
                return;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    walk(&path, out);
                } else if path.extension().is_some_and(|e| e == "rs") {
                    out.push(path);
                }
            }
        }
        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut files = Vec::new();
        walk(&src, &mut files);
        files.sort();
        files
            .into_iter()
            .filter_map(|p| {
                let text = std::fs::read_to_string(&p).ok()?;
                Some((p, text))
            })
            .collect()
    }

    /// A line that is only a comment. Both scanners skip these, rather than skipping
    /// *this file* — the guard has to keep working when the prose that names the
    /// pattern moves, which is precisely how `settings_keys`' first guard stopped
    /// covering anything at all.
    fn is_comment(line: &str) -> bool {
        line.trim_start().starts_with("//")
    }

    /// Does `line` contain `needle` as **code** rather than inside a string literal?
    ///
    /// The scanners below are written in the crate they scan, so each one names the
    /// pattern it hunts for and would otherwise report itself — which is not a
    /// hypothetical, it is what both did on their first run. Skipping this file
    /// instead would be the mistake `settings_keys`' first guard made: the prose
    /// moves, and the guard silently stops covering the file it lives in.
    fn code_contains(line: &str, needle: &str) -> bool {
        line.match_indices(needle)
            .any(|(at, _)| !line[..at].ends_with('"'))
    }

    /// `path` as the repo sees it, so a failure names something clickable.
    fn rel(p: &Path) -> String {
        p.strip_prefix(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .parent()
                .unwrap(),
        )
        .unwrap_or(p)
        .display()
        .to_string()
    }

    /// **Every row widget truncates its label.**
    ///
    /// `StandardListItem` and `StandardTreeItem` are rows by construction — there is no
    /// row whose label may legitimately wrap — so the rule is blanket rather than a
    /// judgement per site. It was not: the outline, the trash tree, the export scope
    /// picker, Go To, the corkboard's move target, the destination picker, the settings
    /// tree, the Welcome examples list and four settings panes all shipped without it,
    /// and none of it showed until a project arrived whose chapter titles were a
    /// hundred characters long rather than "Chapter 1".
    ///
    /// The one exemption is principled rather than a list: a chain that sets
    /// `label_slot` replaces the rendered label with a widget of its own, which then
    /// owns its own overflow. `search::dock`'s excerpt row is the one that does.
    #[test]
    fn every_row_widget_truncates_its_label() {
        const WINDOW: usize = 30;
        let mut missing = Vec::new();
        for (path, text) in sources() {
            let lines: Vec<&str> = text.lines().collect();
            for (i, line) in lines.iter().enumerate() {
                if is_comment(line) {
                    continue;
                }
                if !(code_contains(line, "StandardListItem::new(")
                    || code_contains(line, "StandardTreeItem::new("))
                {
                    continue;
                }
                let end = (i + WINDOW).min(lines.len());
                let chain = lines[i..end].join("\n");
                if chain.contains(".label_overflow(") || chain.contains(".label_slot(") {
                    continue;
                }
                missing.push(format!("{}:{}", rel(&path), i + 1));
            }
        }
        assert!(
            missing.is_empty(),
            "these row widgets never set `label_overflow`, so a long title reports its \
             full width, over-constrains the row and paints outside it — see this \
             module's docs:\n  {}",
            missing.join("\n  ")
        );
    }

    /// **Nothing says "one line" with `max_lines(1)`.**
    ///
    /// See this module's docs: it caps the painted lines and changes neither the
    /// measurement nor the shrink weight, so it breaks the row *and* hides that the
    /// text was cut. Eight sites used it — the two "Appears in" lists, both corkboard
    /// card labels, the cast picker — and every one of them meant `single_line`.
    #[test]
    fn nothing_uses_max_lines_of_one() {
        let mut found = Vec::new();
        for (path, text) in sources() {
            for (i, line) in text.lines().enumerate() {
                if !is_comment(line) && code_contains(line, ".max_lines(1)") {
                    found.push(format!("{}:{}", rel(&path), i + 1));
                }
            }
        }
        assert!(
            found.is_empty(),
            "`max_lines(1)` measures as `Wrap` — full intrinsic width, no shrink — and \
             then cuts the text with no ellipsis. Use `.single_line()`:\n  {}",
            found.join("\n  ")
        );
    }

    /// **A focus ring makes its content unbounded; `Expand` is what holds it.**
    ///
    /// Not a rule anyone can guess from the widget names, and the reason "Appears in
    /// the manuscript" painted across the pane beside it. `ZStack::layout_response`
    /// takes its width from an *unspecified* pass on purpose — the comment there
    /// explains that using the bounded pass would truncate a shrinkable label to a
    /// `MinSize`'s minimum during intrinsic measurement — so anything inside
    /// `crate::widgets::with_focus_ring` reports its full natural width however narrow
    /// the column around it, and a plain stack then places it at that width.
    ///
    /// Three things that look like they should close it and do not are pinned here
    /// too, because each cost a round of measurement: the `ZStack` does pass a shrink
    /// weight up, but a stack only distributes a deficit along its own **main** axis,
    /// so a column never compresses it; `Shrinkable` therefore changes nothing; and a
    /// `ColumnFlow` clamps only the child it holds directly, not one behind an
    /// intervening stack.
    #[test]
    fn a_focus_ring_reports_its_content_unbounded_and_expand_is_what_holds_it() {
        use teksilo::prelude::*;
        use teksilo::widgets::{
            ColumnFlow, Expand, HStack, Shrinkable, Spacer, TextWidget, VStack, ZStack,
        };

        // Long enough that its natural width is far past the box it is laid out in.
        let long = "Dans lequel Phileas Fogg et Passepartout s’acceptent réciproquement, l’un \
             comme maître, l’autre comme domestique"
            .to_string();
        const BOX: f32 = 200.0;

        /// The right-most edge anything in `w` reaches, laid out in a `BOX`-wide box.
        ///
        /// A real text backend, for the reason `tabs`' own narrow-window test gives:
        /// the no-backend fallback reports a rigid size, so a `single_line` label would
        /// spill exactly as it never does in the live app.
        fn widest(w: impl Widget + 'static) -> f32 {
            let mut tree =
                teksilo::core::widget_tree::WidgetTree::new().with_text_backend(std::rc::Rc::new(
                    std::cell::RefCell::new(teksilo::canvas::MockTextBackend::new()),
                ));
            let id = tree.add(w);
            tree.layout(SizeProposal::exact(BOX, 400.0));
            fn walk(
                tree: &teksilo::core::widget_tree::WidgetTree,
                id: teksilo::prelude::WidgetId,
                out: &mut f32,
            ) {
                let b = tree.bounds(id);
                *out = out.max(b.origin().x + b.size().width);
                for c in tree.children(id) {
                    walk(tree, c, out);
                }
            }
            let mut out = 0.0;
            walk(&tree, id, &mut out);
            out
        }

        // One backlink row: an elided title, a spacer, and the confirm control.
        let row = || {
            HStack::new()
                .child(TextWidget::new(lit!(long.clone())).single_line())
                .child(Expand::horizontal().child(Spacer::new()))
                .child(TextWidget::new(lit!("x".to_string())))
        };
        let ringed = || ZStack::new().child(row()).child(Spacer::new());

        // Bare, the row behaves: the label truncates and the row fits its column.
        assert!(
            widest(VStack::new().child(row())) <= BOX + 0.5,
            "a plain row does not fit its box, so the rest of this test proves nothing"
        );

        // Wrapped in a focus ring, the very same row reports its full natural width —
        // and neither a shrink weight nor a flow one stack away takes it back.
        for (what, w) in [
            ("VStack(ring)", widest(VStack::new().child(ringed()))),
            (
                "VStack(Shrinkable(ring))",
                widest(VStack::new().child(Shrinkable::new().min_width(40.0).child(ringed()))),
            ),
            (
                "ColumnFlow(VStack(ring))",
                widest(
                    ColumnFlow::new()
                        .min_column_width(120.0)
                        .max_columns(1)
                        .child(VStack::new().child(ringed())),
                ),
            ),
        ] {
            assert!(
                w > BOX * 2.0,
                "{what} now bounds a focus ring's content ({w} in a {BOX}px box). If the \
                 framework changed, `BacklinksList`'s `Expand` wrapper can be \
                 reconsidered — but read this test's docs first."
            );
        }

        // `Expand` is what closes it, and it needs no width to do so.
        assert!(
            widest(VStack::new().child(Expand::horizontal().child(ringed()))) <= BOX + 0.5,
            "`Expand::horizontal` no longer holds a focus ring to the width it is given"
        );
    }
}
