// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Writing djot: rows in, marked-up text out.
//!
//! The lowest layer of the compiler and the only one that knows the syntax.
//! Every function here appends to a `String` and none of them read the store,
//! so a change to how a heading or a scene break is spelled lands in one place
//! and reaches all eight formats at once.

use super::*;

/// Flatten the frozen tree into the included rows, in document order, each with its
/// resolved language. `include` decides membership; document order decides sequence (so a
/// Choose… selection still exports top-to-bottom). `activated` is a defensive guard — a
/// trashed item never exports even if its id reaches the set.
pub(super) fn flatten<'a>(req: &'a RenderRequest) -> Vec<Row<'a>> {
    let want: std::collections::HashSet<u64> = req.include.iter().copied().collect();
    let mut rows = Vec::new();
    // `tags_in_binder` speaks lists; the renderer speaks one tag per row. Built once,
    // outside the loop, and via the shared parser so an empty work language yields no
    // tags rather than a list holding one empty string.
    let work_langs = language::parse_legacy_list(req.work_lang);
    for bwi in &req.gathered.binders {
        let items: Vec<BinderItem> = bwi.items.iter().map(|iwc| iwc.item.clone()).collect();
        let mut langs = std::collections::HashMap::new();
        language::tags_in_binder(&work_langs, &items, &mut langs);
        for iwc in &bwi.items {
            if want.contains(&iwc.item.id) && iwc.item.activated {
                let lang = langs
                    .get(&iwc.item.id)
                    .map(|l| language::primary(l).to_string())
                    .filter(|l| !l.is_empty())
                    .unwrap_or_else(|| req.work_lang.to_string());
                rows.push(Row {
                    item: &iwc.item,
                    contents: &iwc.contents,
                    lang,
                });
            }
        }
    }
    rows
}

// ─────────────────────────────────────────────────────────────────────────────
// Djot builders
// ─────────────────────────────────────────────────────────────────────────────

/// The `direction` attribute pair for a row, or `None` for LTR. The single
/// encoding of the key — [`dir_pair`] and every attribute set below build from
/// it, so the spelling lives in one place.
pub(super) fn dir_pair(rtl: bool) -> Option<&'static str> {
    rtl.then_some("direction=rtl")
}

/// One `{k=v k=v}` line for a block, or nothing when it carries no attributes.
///
/// A Djot block gets exactly **one** attribute set, so every key that applies to a block
/// has to be merged here — a second `{…}` line is parsed as its own empty block, which is
/// how an attribute can appear in the output and still do nothing at all.
pub(super) fn attr_line(rtl: bool, extra: &[String]) -> String {
    let mut attrs: Vec<&str> = Vec::new();
    attrs.extend(dir_pair(rtl));
    attrs.extend(extra.iter().map(String::as_str));
    if attrs.is_empty() {
        String::new()
    } else {
        format!("{{{}}}\n", attrs.join(" "))
    }
}

pub(super) fn push_heading(out: &mut String, level: u8, text: &str, rtl: bool, extra: &[String]) {
    out.push_str(&attr_line(rtl, extra));
    for _ in 0..level.clamp(1, 6) {
        out.push('#');
    }
    out.push(' ');
    out.push_str(text.trim());
    out.push_str("\n\n");
}

pub(super) fn push_para(out: &mut String, text: &str, rtl: bool, extra: &[String]) {
    out.push_str(&attr_line(rtl, extra));
    out.push_str(text.trim());
    out.push_str("\n\n");
}

/// Append a row's prose, block by block. The prose itself is already Djot and
/// goes in verbatim — the exporter generates furniture, it never rewrites what
/// the author wrote.
///
/// Every block is inspected when `scan_markers` is set: a scene-break marker the
/// author placed is replaced by the preset's rendering for its tier (and does
/// not count as prose), while an ordinary block is emitted with whatever block
/// attributes apply — RTL direction, plus anything a preceding break queued for
/// it. They are merged into **one** `{...}` line, because a block carries a
/// single attribute set.
///
/// Returns `(words, emitted_prose)` — the word count with markers excluded, and
/// whether any actual prose block reached the document. A scene whose entire
/// content is a break marker emits furniture but no prose, and must not be
/// counted as an emitted item.
pub(super) fn push_prose(
    out: &mut String,
    djot: &str,
    rtl: bool,
    preset: &Preset,
    scan_markers: bool,
    pending: &mut Vec<String>,
    lead: &[String],
) -> (usize, bool) {
    let trimmed = djot.trim();
    // Fast path — and a correctness guard, not just an optimisation. Splitting
    // on `"\n\n"` is not a Djot block split: a list with an indented
    // continuation, or a fenced block containing a blank line, is one construct
    // spanning several chunks. When there is nothing to scan for and no
    // attribute to attach, the prose goes in verbatim exactly as it always did,
    // so those constructs cannot be disturbed at all.
    if trimmed.is_empty() {
        return (0, false);
    }
    let could_hold_marker = scan_markers && scene_break::might_contain_marker(trimmed);
    if !rtl && pending.is_empty() && lead.is_empty() && !could_hold_marker {
        out.push_str(trimmed);
        out.push_str("\n\n");
        return (trimmed.split_whitespace().count(), true);
    }
    let mut words = 0usize;
    let mut emitted_prose = false;
    // Non-marker blocks are flushed as one contiguous, verbatim run rather than
    // one at a time. `split("\n\n")` is not a Djot block split — a fenced block
    // containing a blank line is one construct spanning several chunks — so
    // rejoining a run reverses the split exactly and cannot disturb it. Only the
    // first block of a run carries the queued attributes.
    let blocks: Vec<&str> = trimmed.split("\n\n").collect();
    let mut offsets: Vec<(usize, usize)> = Vec::with_capacity(blocks.len());
    let mut cursor = 0usize;
    for b in &blocks {
        offsets.push((cursor, cursor + b.len()));
        cursor += b.len() + 2;
    }
    let mut run_start: Option<usize> = None;
    // `lead` (a queued page break) belongs to the first block that actually reaches the
    // output, and to that one alone — hence the flag rather than a plain closure capture.
    let mut lead_pending = !lead.is_empty();
    let flush = |out: &mut String,
                 range: Option<(usize, usize)>,
                 pending: &mut Vec<String>,
                 lead_pending: &mut bool| {
        let Some((a, b)) = range else { return false };
        let text = trimmed[a..b.min(trimmed.len())].trim_end();
        if text.trim().is_empty() {
            return false;
        }
        let mut attrs: Vec<String> = Vec::new();
        attrs.extend(dir_pair(rtl).map(str::to_string));
        if *lead_pending {
            attrs.extend(lead.iter().cloned());
            *lead_pending = false;
        }
        // Only manuscript prose inherits what a break queued. A synopsis or a
        // note between two scenes is commentary, not the flow the break divides.
        if scan_markers {
            attrs.append(pending);
        }
        if !attrs.is_empty() {
            out.push('{');
            out.push_str(&attrs.join(" "));
            out.push_str("}\n");
        }
        out.push_str(text);
        out.push_str("\n\n");
        true
    };
    for (i, block) in blocks.iter().enumerate() {
        let tier = if scan_markers {
            scene_break::tier_of_djot_block(block)
        } else {
            None
        };
        let Some(tier) = tier else {
            // RTL needs its attribute on every block, so it cannot batch.
            if rtl {
                if flush(out, Some(offsets[i]), pending, &mut lead_pending) {
                    emitted_prose = true;
                }
            } else {
                run_start.get_or_insert(offsets[i].0);
            }
            words += block.split_whitespace().count();
            continue;
        };
        if flush(
            out,
            run_start.take().map(|a| (a, offsets[i].0)),
            pending,
            &mut lead_pending,
        ) {
            emitted_prose = true;
        }
        let style = match tier {
            SceneBreakTier::Minor => &preset.scene_break,
            SceneBreakTier::Major => &preset.major_scene_break,
        };
        push_scene_break(out, style, preset, rtl, pending);
    }
    if flush(
        out,
        run_start.take().map(|a| (a, trimmed.len())),
        pending,
        &mut lead_pending,
    ) {
        emitted_prose = true;
    }
    (words, emitted_prose)
}

/// The gap a `BlankLine` break opens above the paragraph that follows it, in
/// logical pixels — one empty line *at this preset's body size*, so it stays
/// "about a blank line" whether the manuscript is set at 8pt or 24pt.
///
/// It has to be a real margin: Djot cannot express "one blank line" on its own,
/// since consecutive blank lines collapse into the ordinary block separator.
/// 96 px = 72 pt, hence the 4/3.
pub(super) fn blank_line_gap_px(preset: &Preset) -> i64 {
    let multiple = match preset.line_spacing {
        LineSpacing::Single => 1.0,
        LineSpacing::OneAndHalf => 1.5,
        LineSpacing::Double => 2.0,
    };
    // A preset that already spaces paragraphs contributes part of the gap on its
    // own; only the remainder is needed here, or a spaced preset would open a
    // visibly double break.
    let line_px = (preset.font_size_pt as f64) * 4.0 / 3.0 * multiple;
    let already_px = (preset.paragraph_spacing_pt as f64) * 4.0 / 3.0;
    (line_px - already_px).round().max(1.0) as i64
}

/// Render a scene break the author marked, and queue the block attributes that
/// belong on the paragraph after it.
///
/// Both visible tiers suppress that paragraph's first-line indent, which is what
/// print typography actually does at a scene break — the indent would otherwise
/// read as an ordinary new paragraph and undo the separation.
pub(super) fn push_scene_break(
    out: &mut String,
    sep: &SceneBreak,
    preset: &Preset,
    rtl: bool,
    pending: &mut Vec<String>,
) {
    // The most recent break wins: two markers in a row must not stack their
    // attributes onto the same following paragraph.
    pending.clear();
    match sep {
        // Rendered as nothing at all, so the following paragraph is ordinary too.
        SceneBreak::None => {}
        SceneBreak::BlankLine => {
            pending.push(format!("top_margin={}", blank_line_gap_px(preset)));
            pending.push("text_indent=0".to_string());
        }
        SceneBreak::Glyph(g) => {
            // Centred, as a dinkus is set in print — and carrying the row's own
            // direction, so the glyph line does not fall back to LTR inside an
            // otherwise right-to-left manuscript.
            let mut attrs = vec!["alignment=center".to_string()];
            attrs.extend(dir_pair(rtl).map(str::to_string));
            out.push('{');
            out.push_str(&attrs.join(" "));
            out.push_str("}\n");
            // Escape a leading Djot block-marker so the glyph stays literal text — an
            // unescaped `#` is a heading, `* * *` / `---` a (dropped) thematic break.
            out.push_str(&escape_block_leading(g.trim()));
            out.push_str("\n\n");
            pending.push("text_indent=0".to_string());
        }
    }
}

/// The title page: the word count, then the title about a third of the way down, then the
/// byline under it.
///
/// The vertical drop is a real `top_margin` on the title block rather than a run of empty
/// paragraphs. Empty paragraphs are what a word processor's user would do and what every
/// format then renders differently — and a blank Djot block is not even representable,
/// since consecutive blank lines collapse into the ordinary block separator.
///
/// It normally carries no page break — it is the top of the document, and the *body's*
/// first block is what breaks away from it. `break_above` is the one exception: a cover
/// page sits above it, and the two must not share a sheet.
pub(super) fn render_title_page(
    req: &RenderRequest,
    preset: &Preset,
    rtl: bool,
    words: usize,
    break_above: bool,
) -> String {
    let w = &req.gathered.work;
    let lang = match &preset.heading_language {
        HeadingLanguage::Fixed(l) => l.clone(),
        HeadingLanguage::Auto => req.work_lang.to_string(),
    };
    let mut page = String::new();
    // Consumed by whichever block turns out to be first — which one that is
    // depends on the preset and on whether the Work has a title at all, so it
    // cannot be decided here.
    let mut first: Vec<String> = if break_above {
        vec!["page_break_before=true".to_string()]
    } else {
        Vec::new()
    };

    // Upper right, the manuscript-submission convention. Suppressed for an empty
    // manuscript: "about 0 words" is a statement no title page should make.
    if preset.title_page_word_count && words > 0 {
        let mut attrs = std::mem::take(&mut first);
        attrs.push("alignment=right".to_string());
        push_para(
            &mut page,
            &escape_block_leading(&headings::word_count_note(&lang, words, preset.digit_style)),
            rtl,
            &attrs,
        );
    }

    if !w.title.trim().is_empty() {
        let mut attrs = std::mem::take(&mut first);
        attrs.push("alignment=center".to_string());
        attrs.push(format!("top_margin={}", title_drop_px(preset)));
        push_heading(&mut page, 1, w.title.trim(), rtl, &attrs);
    }

    // The author's name is *data*, never markup — escaped so a name that happens to start
    // like a list marker survives intact. The preposition is generated furniture and so is
    // localized; the name itself never is.
    if !w.author_name.trim().is_empty() {
        let mut attrs = std::mem::take(&mut first);
        attrs.push("alignment=center".to_string());
        let byline = format!("{} {}", headings::by(&lang), w.author_name.trim());
        push_para(&mut page, &escape_block_leading(&byline), rtl, &attrs);
    }
    page
}

/// How far down the page a title page's title sits, in logical pixels.
///
/// About a third of the way into the *text* area — the traditional placement, and what
/// Shunn asks for on a novel's title page. Derived from the preset's own page size and
/// margins so it stays a third whether the manuscript is A4 or A5.
pub(super) fn title_drop_px(preset: &Preset) -> i64 {
    let page_h_in = match preset.page_size {
        PageSize::A4 => 297.0 / 25.4,
        PageSize::Letter => 11.0,
        PageSize::A5 => 210.0 / 25.4,
    };
    let text_h_in =
        (page_h_in - preset.margin.top_in as f64 - preset.margin.bottom_in as f64).max(0.0);
    // 96 logical pixels to the inch, the unit the block attributes use.
    ((text_h_in / 3.0) * 96.0).round().max(0.0) as i64
}

/// Emit one row's epigraph, returning whether anything reached the output.
///
/// `lead` is a page break this epigraph is opening the chapter with, if any. It rides the
/// quotation's own attribute line rather than a line in front of it — see
/// [`mark_epigraph`] — and falls back to an ordinary block attribute when the epigraph was
/// typed as a bare paragraph with no `>` to hang it on.
pub(super) fn push_epigraph(
    out: &mut String,
    epi: &str,
    lead: &[String],
    rtl: bool,
    preset: &Preset,
    pending_attrs: &mut Vec<String>,
    row: &Row<'_>,
) -> bool {
    // Mark the quote as an epigraph, not merely as quoted text. `semantic_role` rides the
    // blockquote's first block — djot block attributes are the only channel — and
    // text-document lifts it onto the frame, which is what lets EPUB emit
    // `epub:type="epigraph"`, DOCX give it a named style and Typst use its own attribution
    // slot. Without it every writer sees an ordinary blockquote and none can say what it is.
    let (marked, is_quote) = mark_epigraph(epi, lead);
    // Never scanned for break markers: an epigraph is quoted matter, not the scene flow a
    // break divides. Its words are not the manuscript's either, so the count is discarded
    // exactly as the synopsis's is — an epigraph must not move a pace target.
    let (_, emitted) = push_prose(
        out,
        &marked,
        rtl,
        preset,
        false,
        pending_attrs,
        // Already inside the quotation when there was one to put it in.
        if is_quote { &[] } else { lead },
    );
    if emitted {
        // New Hart's Rule: the first line after a heading, an epigraph or a section break
        // carries no first-line indent. Queued the same way a scene break queues it, and
        // consumed by this row's own prose — `push_prose` only lets manuscript prose
        // (`scan_markers`) inherit the queue, so a synopsis cannot swallow it by mistake.
        //
        // **Only when this row has prose of its own.** A Part or a Book has none, so the
        // queue would outlive the row and land on whatever prose came next — a following
        // Scene's opening paragraph, which is a different paragraph entirely and is
        // entitled to its indent. Relying on "the next structural heading clears it" is not
        // enough: the binder is organisational, so nothing guarantees a heading row comes
        // next.
        if main_prose_role(&row.item.sub_role)
            .is_some_and(|role| content_of(row.contents, role).is_some())
        {
            pending_attrs.push("text_indent=0".to_string());
        }
    }
    emitted
}

/// Take a queued page break for the block about to be emitted, if there is one and if
/// there is anything above it to end a page on.
///
/// Both flags are cleared either way. A break the document's very first block cannot use
/// is *spent*, not carried forward: it was asked for "before this content", and this
/// content is already at the top.
///
/// Deliberately not offered to an epigraph. An epigraph's blocks live inside a blockquote,
/// and a page break in there is at best awkward and at worst — in Typst, where the quote
/// is a single content argument — malformed. A row whose heading is suppressed lets the
/// break fall through to its prose instead, which is the paragraph a reader would call
/// the top of the page anyway.
pub(super) fn take_break(pending: &mut bool, anything_above: &mut bool) -> Vec<String> {
    let take = *pending && *anything_above;
    *pending = false;
    *anything_above = true;
    if take {
        vec!["page_break_before=true".to_string()]
    } else {
        Vec::new()
    }
}

/// Escape a leading Djot block marker so the line is read as a plain paragraph.
///
/// Two different families of marker can hijack the start of a line, and they need
/// *different* escapes — a backslash only escapes punctuation in Djot, so putting one
/// in front of a letter or digit leaves a literal backslash on the page:
///
/// * a **single special character** (`#` heading, `>` quote, `*`/`-`/`+` bullet, …).
///   Escape the character itself: `\* Star` → `* Star`.
/// * a **two-character list marker** — an alphanumeric followed by `.` or `)`, which
///   Djot reads as an ordered list (numeric `1.`, alphabetic `A.`, roman `i.`).
///   Escape the *punctuation*, not the leading character: `A\. Writer` → `A. Writer`,
///   whereas `\A. Writer` renders the backslash literally and `A. Writer` silently
///   loses the `A.` — the marker is consumed and only "Writer" survives.
pub(super) fn escape_block_leading(s: &str) -> String {
    let mut chars = s.chars();
    match (chars.next(), chars.next()) {
        // Ordered-list marker: escape the `.`/`)` that makes it one.
        (Some(c), Some(p)) if c.is_alphanumeric() && (p == '.' || p == ')') => {
            let mut out = String::with_capacity(s.len() + 1);
            out.push(c);
            out.push('\\');
            out.push_str(&s[c.len_utf8()..]);
            out
        }
        (Some(c), _) if "#*-_+>~:|=`".contains(c) => format!("\\{s}"),
        _ => s.to_string(),
    }
}

/// Whether a row's text lays out right-to-left, honouring a forced preset direction.
pub(super) fn is_rtl_row(preset: &Preset, lang: &str) -> bool {
    match preset.direction {
        DirectionMode::ForceRtl => true,
        DirectionMode::ForceLtr => false,
        DirectionMode::Auto => language::is_rtl(lang),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Small helpers
// ─────────────────────────────────────────────────────────────────────────────

/// The manuscript's numbering, computed once over the **whole** gathered tree.
///
/// This replaces a two-pass design that got the same question wrong two different ways: a
/// running counter bumped inside the row loop (which only ever saw rows a given export had
/// already been filtered down to), plus a `seed_counters` replay for scoped exports (which
/// walked the tree unfiltered). Mark a chapter non-exportable and the two disagreed —
/// "Export Book" renumbered every later chapter, "Export Chapter 7" did not. There is now
/// one pass, in [`skribisto_model::numbering`], and the renderer only looks its answer up.
///
/// Deliberately built from `req.gathered`, never from `req.include`: the export's own
/// selection must not change what number a chapter carries.
/// Empty when the manuscript does not number (`Work.number_chapters`) — the gate lives
/// *here*, at the single source of the numbers, rather than downstream at the schemes.
///
/// Gating the schemes alone was not enough and shipped a hole: `TitleOnly` falls back to
/// the numeral when a title is blank (`title.or(numbered)`), and a Book's scheme is
/// hardcoded rather than read from the preset, so it never passed through a clamp at all.
/// An untitled book or chapter therefore still printed "Book 1" / "Chapter 3" into the
/// exported file of a manuscript whose writer had switched numbering off. With no numbers
/// in the map there is nothing for any scheme to fall back to, and the guarantee holds for
/// every level and every scheme by construction.
pub(super) fn manuscript_numbers(req: &RenderRequest) -> HashMap<u64, Numbered> {
    if !req.gathered.work.number_chapters {
        return HashMap::new();
    }
    numbering::number_map(
        &crate::item_metas(req.gathered),
        NumberingRules {
            part_resets_chapter: req.gathered.work.part_resets_chapter,
        },
    )
}

/// The footnote definitions this export must carry, and what each marker prints.
pub(super) struct CompiledNotes {
    /// `[^label]: body` blocks, in the order they are read.
    pub(super) definitions: Vec<String>,
    /// Label → marker, from the manuscript's own numbering.
    pub(super) markers: std::collections::HashMap<String, String>,
}
