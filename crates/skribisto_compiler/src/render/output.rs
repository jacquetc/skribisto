// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Turning an assembled document into the file the writer asked for.
//!
//! Everything past the point where the manuscript exists as one resolved djot
//! string: the per-format options, the image payloads, the comment marks, and
//! `render_to_file` itself. Eight formats, but only the last few hundred bytes
//! of each differ — the shared work happens upstream in the assembly, which is
//! why this is not one module per format.

use super::*;

/// Render to `path`, honouring `progress` (0..1) and `cancel`. Handles every format: text
/// formats are assembled + written; DOCX is written by text-document's own writer.
pub fn render_to_file(
    req: &RenderRequest,
    path: &Path,
    progress: &dyn Fn(f32),
    cancel: &AtomicBool,
) -> Result<RenderStats> {
    let built = assemble(req, progress, cancel)?;
    let (doc, mut stats, langs) = (&built.doc, built.stats, &built.langs);
    if cancel.load(Ordering::Relaxed) {
        return Err(anyhow!("operation cancelled"));
    }
    // Resolved once, before the format arms, so the two writers that carry comments consume
    // the same payload rather than each rebasing for itself — two rebases of one document
    // could disagree, and the disagreement would show up as a comment landing in different
    // places in the `.docx` and the `.odt` of the same export.
    let Payloads {
        comments,
        marks,
        orphaned: comments_orphaned,
    } = export_payloads(req, &built)?;
    stats.comments_written = comments.len();
    stats.comments_orphaned = comments_orphaned;
    match req.format {
        f if f.is_text() => {
            let handling = req.preset.image_handling;
            let text = text_render(doc, f, handling, &images_for_html(req, &built))?;
            fs::write(path, text).map_err(|e| anyhow!("writing '{}': {e}", path.display()))?;
            write_sidecar_images(req, &built, path, f, handling)?;
        }
        ExportFormat::Docx => {
            let out = path.to_string_lossy().into_owned();
            let mut opts = docx_options(
                req.preset,
                &req.gathered.work.title,
                &req.gathered.work.author_name,
                collect_images(req.gathered, req.media_dir, Some(&built.image_refs)),
            );
            opts.comments = comments;
            opts.marks = marks;
            doc.to_docx_with_options(&out, opts)?
                .wait()
                .map_err(|e| anyhow!("writing DOCX '{out}': {e:#}"))?;
        }
        ExportFormat::Odt => {
            let out = path.to_string_lossy().into_owned();
            let mut opts = odt_options(
                req.preset,
                &req.gathered.work.title,
                &req.gathered.work.author_name,
                collect_images(req.gathered, req.media_dir, Some(&built.image_refs)),
            );
            opts.comments = comments;
            opts.marks = marks;
            doc.to_odt_with_options(&out, opts)?
                .wait()
                .map_err(|e| anyhow!("writing ODT '{out}': {e:#}"))?;
        }
        ExportFormat::Epub => {
            let out = path.to_string_lossy().into_owned();
            let w = &req.gathered.work;
            // The book's language drives the EPUB `dc:language` + reading direction; fall back
            // to the work-level language when the document carries none. `dc:language` takes
            // ONE tag, so this must be the primary, not the whole (possibly multi-tag) field.
            let primary = skribisto_model::language::primary(&w.dict_language);
            let lang = if primary.is_empty() {
                req.work_lang.to_string()
            } else {
                primary.to_string()
            };
            let opts = EpubExportOptions {
                title: w.title.clone(),
                author: w.author_name.clone(),
                rtl: is_rtl_row(req.preset, &lang),
                language: lang,
                images: collect_images(req.gathered, req.media_dir, Some(&built.image_refs)),
                cover: cover_image(req),
            };
            doc.to_epub_with_options(&out, opts)?
                .wait()
                .map_err(|e| anyhow!("writing EPUB '{out}': {e:#}"))?;
        }
        ExportFormat::Pdf => {
            let out = path.to_string_lossy().into_owned();
            let w = &req.gathered.work;
            // Same single-tag requirement as the EPUB arm above.
            let primary = skribisto_model::language::primary(&w.dict_language);
            let lang = if primary.is_empty() {
                req.work_lang.to_string()
            } else {
                primary.to_string()
            };
            // Effective languages of the included rows (computed once by `assemble`) drive
            // which RTL faces to embed.
            let (page_w, page_h) = pdf_page_mm(req.preset.page_size);
            let m = &req.preset.margin;
            let in_to_mm = |i: f32| i * 25.4;
            let opts = PdfExportOptions {
                page_width_mm: page_w,
                page_height_mm: page_h,
                margin_top_mm: in_to_mm(m.top_in),
                margin_bottom_mm: in_to_mm(m.bottom_in),
                margin_left_mm: in_to_mm(m.left_in),
                margin_right_mm: in_to_mm(m.right_in),
                // The family name must match the bytes actually fed (substitute-aware).
                font_family: crate::fonts::pdf_body_family(req.preset),
                font_bytes: crate::fonts::pdf_font_bytes(req.preset, langs),
                images: collect_images(req.gathered, req.media_dir, Some(&built.image_refs)),
                font_size_pt: req.preset.font_size_pt,
                // Typst `leading` (the gap *between* lines, not a line-height multiple — Typst
                // has no direct multiple), in em. Approximated as `multiple - 0.35`, anchoring
                // single on Typst's own 0.65em default and scaling up for 1½ / double so a
                // "double-spaced" manuscript reads visibly more open than 1½.
                line_spacing: match req.preset.line_spacing {
                    LineSpacing::Single => 0.65,
                    LineSpacing::OneAndHalf => 1.15,
                    LineSpacing::Double => 1.65,
                },
                first_line_indent_mm: (req.preset.first_line_indent_in > 0.0)
                    .then(|| in_to_mm(req.preset.first_line_indent_in)),
                paragraph_spacing_pt: (req.preset.paragraph_spacing_pt > 0.0)
                    .then_some(req.preset.paragraph_spacing_pt),
                justify: req.preset.justify,
                base_rtl: is_rtl_row(req.preset, &lang),
                lang: Some(lang),
                title: (!w.title.trim().is_empty()).then(|| w.title.clone()),
                author: (!w.author_name.trim().is_empty()).then(|| w.author_name.clone()),
                include_preamble: true,
            };
            doc.to_pdf_with_options(&out, opts)?
                .wait()
                .map_err(|e| anyhow!("writing PDF '{out}': {e:#}"))?;
        }
        other => return Err(anyhow!("{other:?} export is not implemented yet")),
    }
    progress(1.0);
    Ok(stats)
}

/// A preset [`PageSize`] as (width, height) in millimetres — the unit `PdfExportOptions` uses.
pub(super) fn pdf_page_mm(size: PageSize) -> (f32, f32) {
    match size {
        PageSize::A4 => (210.0, 297.0),
        PageSize::Letter => (215.9, 279.4),
        PageSize::A5 => (148.0, 210.0),
    }
}

/// One inch in twips (DOCX's twentieth-of-a-point length unit).
pub(super) const TWIPS_PER_IN: f32 = 1440.0;

/// Map a [`Preset`] onto DOCX page geometry + base typography (all in DOCX units), plus a
/// manuscript running header from the Work's author + title. Only DOCX (and later PDF) honour
/// these; the free text formats ignore them. This is where the manuscript presets become
/// *effective* — page size, margins, font, double-spacing, first-line indent, ragged/justified
/// alignment, and page-numbered header all flow from here; per-block RTL is emitted by the
/// exporter itself from each block's direction, so it needs no option.
/// The editor-facing comment payload for one assembled document, plus how many comments
/// could not be placed.
///
/// Empty for every format that does not [carry comments](ExportFormat::carries_comments) —
/// the two writers that do are the two an editor marks up and returns, and a payload built
/// for any other format would be work done to be discarded.
///
/// # What is and is not counted as a failure
///
/// Three populations arrive here and only two of them are the writer's problem:
///
/// * A comment on a row **outside the export scope** — or on a synopsis a preset omits — is
///   not in this document at all. `comment_rebase::place_comments` leaves it out of its
///   result entirely, so it never reaches the count. Warning about it would fire on every
///   "Export Chapter 5" that has notes anywhere else in the book.
/// * A comment **already orphaned at rest** has no valid quote to place, so it resolves to an
///   orphan and is counted. The writer has seen it flagged in the dock too.
/// * A comment that **failed to rebase** — its quote resolved in the editor but not against
///   the compiled text — is counted, and is the surprising one worth telling the writer
///   about, because it looks perfectly healthy in the margin.
///
/// A counted comment is dropped from the payload rather than written at a guessed position:
/// the whole anchor model exists to avoid a comment that is *mostly* right.
///
/// A preset with [`include_comments`](crate::preset::Preset::include_comments) off returns the
/// same empty payload **and a zero count**. That is not laziness about the count: a comment left
/// out because the writer asked for a clean copy was not *dropped*, and reporting it as one
/// would put "3 comments could not be placed" on a perfectly correct export. A warning that
/// fires when nothing is wrong is how writers learn to stop reading warnings.
/// Everything a comment-carrying writer is handed beside the document itself.
pub(crate) struct Payloads {
    pub comments: text_document::DocumentComments,
    /// Round-trip marks — the bookmarks that let a returning file be recognised as this
    /// project's work. See [`skribisto_model::round_trip`] for what the names say.
    pub marks: text_document::DocumentMarks,
    /// Comments that belonged in this export and could not be placed.
    pub orphaned: usize,
}

/// Build both payloads from **one** pass over the compiled document.
///
/// One pass, not two, because a row's window is found by a forward heuristic search (see
/// [`comment_rebase::locate_windows`]) and two independent searches are free to disagree. They
/// must not: a row mark is anchored at the start of the very window its own comments are rebased
/// into, so a disagreement would put a row's identity and its notes in different places in the
/// same file — and the file is the only thing the reader gets.
pub(super) fn export_payloads(req: &RenderRequest, built: &Assembled) -> Result<Payloads> {
    let mut out = Payloads {
        comments: text_document::DocumentComments::new(),
        marks: text_document::DocumentMarks::new(),
        orphaned: 0,
    };

    let want_comments = req.format.carries_comments()
        && req.preset.include_comments
        && !req.gathered.comments.is_empty();
    let want_marks = req.format.carries_round_trip_marks() && req.preset.include_round_trip_marks;
    if !want_comments && !want_marks {
        return Ok(out);
    }

    // The ADDRESSABLE text and the block starts that index it. Pairing an offset with
    // `to_plain_text()` instead is the classic form of this bug in this codebase: that string
    // is the human-readable export, it omits each table's `U+FFFC` anchor, and every offset
    // after a table lands two characters out in it.
    let text = built.doc.to_addressable_text()?;
    let starts: Vec<usize> = built
        .doc
        .blocks()
        .into_iter()
        .map(|b| b.position())
        .collect();
    let windows = comment_rebase::locate_windows(&text, &starts, &built.emitted);

    if want_marks {
        for m in row_marks(built, &windows) {
            out.marks.insert(m);
        }
    }
    if want_comments {
        comment_payload_into(req, built, &text, &starts, &windows, &mut out, want_marks)?;
    }
    Ok(out)
}

/// One point mark per exported `BinderItem`, at the first character of its prose.
///
/// **Per item, not per `Content`** — a scene and its own synopsis are two contents of one row,
/// and two marks naming the same uid would collide (a bookmark name is unique in a document, and
/// `DocumentMarks` is keyed by it, so the second would silently replace the first). The item's
/// first emitted content wins, which is its manuscript prose whenever it has any.
///
/// A row whose uid is still nil is skipped. That is a row created before `with_identity` ran;
/// it has no durable identity to write, and a mark naming the nil uuid would claim every such
/// row was the same one.
pub(super) fn row_marks(
    built: &Assembled,
    windows: &std::collections::HashMap<u64, comment_rebase::Window>,
) -> Vec<text_document::DocumentMark> {
    let mut seen: std::collections::HashSet<uuid::Uuid> = std::collections::HashSet::new();
    let mut out = Vec::new();
    for e in &built.emitted {
        if e.item_uid.is_nil() || !seen.insert(e.item_uid) {
            continue;
        }
        let Some(w) = windows.get(&e.content_id) else {
            // The row's prose could not be located in the compiled text. Nothing to anchor to,
            // and re-import falls back to matching this row by type and title.
            continue;
        };
        out.push(text_document::DocumentMark::point(
            w.lo as u32,
            skribisto_model::round_trip::row_mark_name(&e.item_uid, &e.djot_plain()),
        ));
    }
    out
}

/// Rebase every comment into its row's window and fill `out`.
///
/// `with_marks` additionally emits one **range** mark per comment that found a home, over the
/// very characters the comment covers. That is what lets a returning file re-anchor a comment
/// from a position the editor's own application maintained through their edits, instead of
/// re-matching a quote against prose they may have rewritten — and it is the only identity a
/// comment has once it comes back, since neither Word nor LibreOffice preserves the private uid
/// attribute the writer also emits.
///
/// An orphaned comment gets no mark: it is not written into the file at all, so there would be
/// nothing for the mark to name.
pub(super) fn comment_payload_into(
    req: &RenderRequest,
    built: &Assembled,
    text: &str,
    starts: &[usize],
    windows: &std::collections::HashMap<u64, comment_rebase::Window>,
    out: &mut Payloads,
    with_marks: bool,
) -> Result<()> {
    // The row's own Djot, per Content, so an anchor can be rebuilt against the text it was
    // captured on.
    let djot_by_content: std::collections::HashMap<u64, &str> = built
        .emitted
        .iter()
        .map(|e| (e.content_id, e.djot.as_str()))
        .collect();

    let to_place: Vec<comment_rebase::CommentToPlace> = req
        .gathered
        .comments
        .iter()
        .filter_map(|cwr| {
            let content_id = cwr.comment.content?;
            let row_djot = djot_by_content.get(&content_id).copied().unwrap_or("");
            Some(comment_rebase::CommentToPlace {
                comment_id: cwr.comment.id,
                content_id,
                anchor: comment_rebase::anchor_of(&cwr.comment, row_djot),
                is_paragraph: cwr.comment.kind == common::entities::CommentAnchorKind::Paragraph,
            })
        })
        .collect();

    let placements = comment_rebase::place_comments_in(text, starts, windows, &to_place);
    let by_id: std::collections::HashMap<u64, &skrib_format::CommentWithReplies> = req
        .gathered
        .comments
        .iter()
        .map(|c| (c.comment.id, c))
        .collect();

    // Every comment that got a mark, collected rather than emitted inline so its **ordinal**
    // can be worked out once the whole set is known — see the emission below.
    let mut anchored: Vec<(u32, u32, uuid::Uuid)> = Vec::new();

    for p in &placements {
        let Some(cwr) = by_id.get(&p.comment_id) else {
            continue;
        };
        if comment_rebase::orphan_reason(p).is_some() {
            out.orphaned += 1;
            continue;
        }
        match p.resolution {
            skribisto_model::comment_anchor::Resolution::Anchored { start, length } => {
                out.comments.insert(text_document::DocumentComment {
                    start: start as u32,
                    end: (start + length) as u32,
                    uid: cwr.comment.uid.to_string(),
                    author: cwr.comment.author_name.clone(),
                    author_initials: cwr.comment.author_initials.clone(),
                    date: cwr.comment.created_at.to_rfc3339(),
                    resolved: cwr.comment.resolved,
                    body: cwr.comment.body.clone(),
                    replies: cwr
                        .replies
                        .iter()
                        .map(|r| text_document::CommentReply {
                            uid: r.uid.to_string(),
                            author: r.author_name.clone(),
                            author_initials: r.author_initials.clone(),
                            date: r.created_at.to_rfc3339(),
                            body: r.body.clone(),
                        })
                        .collect(),
                });
                if with_marks && !cwr.comment.uid.is_nil() {
                    anchored.push((start as u32, (start + length) as u32, cwr.comment.uid));
                }
            }
            // Already counted and skipped above.
            skribisto_model::comment_anchor::Resolution::Orphan(_) => {}
        }
    }

    // The marks, ordered to agree with the comments they carry the identity of.
    //
    // The two payloads are sorted independently by whichever writer receives them, and a
    // comment breaks a tie on its `uid` while a mark breaks one on its `name` — which is
    // `fnv(uid)`, so the two orders are unrelated. Two comments on the identical range (two
    // paragraph comments on one paragraph, since `comment_anchor::resolve` gives every
    // paragraph comment the whole paragraph) could therefore be written with their annotations
    // in one order and their marks in the other, and a reader matching them by position would
    // hand each the other's identity: the editor's remark comes home on the wrong thread.
    //
    // So the ordinal is stated here, from the same order the comment payload will be sorted
    // into. `DocumentComment.uid` is the uuid's *string*, so that is what the rank is taken on.
    //
    // ⚠ One tie this cannot break: a comment with a **nil** uid gets no mark at all, so if one
    // shares an exact range with a real comment, the returning file has two annotations there
    // and one mark, and the reader gives the mark to whichever annotation comes first. Nil uids
    // are a legacy shape (rows made before identity was minted) and the overlap needs both at
    // once, but it is a real hole and not a solved one.
    anchored.sort_by(|a, b| {
        a.0.cmp(&b.0)
            .then(a.1.cmp(&b.1))
            .then(a.2.to_string().cmp(&b.2.to_string()))
    });
    let mut ordinal = 0u32;
    for (i, (start, end, uid)) in anchored.iter().enumerate() {
        ordinal = if i > 0 && (anchored[i - 1].0, anchored[i - 1].1) == (*start, *end) {
            ordinal + 1
        } else {
            0
        };
        out.marks.insert(
            text_document::DocumentMark::range(
                *start,
                *end,
                skribisto_model::round_trip::comment_mark_name(uid),
            )
            .with_ordinal(ordinal),
        );
    }
    Ok(())
}

/// The ODT counterpart of [`docx_options`], mapping the same preset onto ODF's own
/// options struct.
///
/// A near-copy on purpose, and deliberately **not** a shared generic over the two. The
/// structs are field-for-field alike today because ODF and OOXML happen to want the same
/// page and typography knobs in the same units, not because either format guarantees it —
/// their vocabularies are unrelated, and the first knob one grows without the other would
/// turn a clever abstraction into a worse copy. The duplication is small, local and
/// obvious; the coupling would not be.
pub(super) fn odt_options(
    preset: &Preset,
    work_title: &str,
    work_author: &str,
    images: text_document::ExportImages,
) -> text_document::OdtExportOptions {
    let (page_w, page_h) = match preset.page_size {
        PageSize::A4 => (11906u32, 16838u32),
        PageSize::Letter => (12240, 15840),
        PageSize::A5 => (8391, 11906),
    };
    let m = &preset.margin;
    let in_to_twips = |i: f32| (i * TWIPS_PER_IN).round() as i32;
    text_document::OdtExportOptions {
        images,
        page_width_twips: Some(page_w),
        page_height_twips: Some(page_h),
        margin_top_twips: Some(in_to_twips(m.top_in)),
        margin_bottom_twips: Some(in_to_twips(m.bottom_in)),
        margin_left_twips: Some(in_to_twips(m.left_in)),
        margin_right_twips: Some(in_to_twips(m.right_in)),
        font_family: (!preset.font_family.trim().is_empty()).then(|| preset.font_family.clone()),
        font_half_points: Some((preset.font_size_pt * 2.0).round().max(2.0) as usize),
        line_spacing_twips: Some(match preset.line_spacing {
            LineSpacing::Single => 240,
            LineSpacing::OneAndHalf => 360,
            LineSpacing::Double => 480,
        }),
        first_line_indent_twips: (preset.first_line_indent_in > 0.0)
            .then(|| in_to_twips(preset.first_line_indent_in)),
        paragraph_spacing_after_twips: (preset.paragraph_spacing_pt > 0.0)
            .then(|| (preset.paragraph_spacing_pt * 20.0).round() as i32),
        justify: preset.justify,
        page_numbers: true,
        running_header: manuscript_header(work_title, work_author),
        heading_styles: Vec::new(),
        // As in `docx_options`: the comment payload is filled by the export use case, the
        // only layer that has resolved an anchor against the compiled document.
        ..Default::default()
    }
}

pub(super) fn docx_options(
    preset: &Preset,
    work_title: &str,
    work_author: &str,
    images: text_document::ExportImages,
) -> DocxExportOptions {
    let (page_w, page_h) = match preset.page_size {
        // Twips = inch × 1440. A4 = 210×297 mm, A5 = 148×210 mm, US Letter = 8.5×11 in.
        PageSize::A4 => (11906u32, 16838u32),
        PageSize::Letter => (12240, 15840),
        PageSize::A5 => (8391, 11906),
    };
    let m = &preset.margin;
    let in_to_twips = |i: f32| (i * TWIPS_PER_IN).round() as i32;
    DocxExportOptions {
        images,
        page_width_twips: Some(page_w),
        page_height_twips: Some(page_h),
        margin_top_twips: Some(in_to_twips(m.top_in)),
        margin_bottom_twips: Some(in_to_twips(m.bottom_in)),
        margin_left_twips: Some(in_to_twips(m.left_in)),
        margin_right_twips: Some(in_to_twips(m.right_in)),
        font_family: (!preset.font_family.trim().is_empty()).then(|| preset.font_family.clone()),
        font_half_points: Some((preset.font_size_pt * 2.0).round().max(2.0) as usize),
        line_spacing_twips: Some(match preset.line_spacing {
            LineSpacing::Single => 240,
            LineSpacing::OneAndHalf => 360,
            LineSpacing::Double => 480,
        }),
        first_line_indent_twips: (preset.first_line_indent_in > 0.0)
            .then(|| in_to_twips(preset.first_line_indent_in)),
        paragraph_spacing_after_twips: (preset.paragraph_spacing_pt > 0.0)
            .then(|| (preset.paragraph_spacing_pt * 20.0).round() as i32),
        justify: preset.justify,
        page_numbers: true,
        running_header: manuscript_header(work_title, work_author),
        // Empty ⇒ `DocxExportOptions::resolved_heading_styles` falls back to
        // `DocxHeadingStyle::default_ramp` scaled off the body size, which is
        // exactly the output this function produced before the field existed.
        heading_styles: Vec::new(),
        // Every remaining option keeps its default. Spelled `..Default::default()`
        // rather than field-by-field on purpose: this struct belongs to
        // `text-document`, which grows a field whenever a new DOCX capability
        // lands (comments, most recently). Enumerating them exhaustively here
        // means each such addition breaks *this* crate's build for no reason —
        // and the compile error names a struct in another repository, which is a
        // poor place to send the next reader.
        //
        // The comment payload is populated by the export use case, which is the
        // only layer that has resolved a comment's anchor against the compiled
        // document; a preset knows nothing about anchors and must not guess.
        ..Default::default()
    }
}

/// The right-aligned running header text — `"Author / TITLE"`, or one side, or `None` when both
/// are blank (the page number is emitted regardless).
pub(super) fn manuscript_header(title: &str, author: &str) -> Option<String> {
    let (title, author) = (title.trim(), author.trim());
    match (author.is_empty(), title.is_empty()) {
        (true, true) => None,
        (false, true) => Some(author.to_string()),
        (true, false) => Some(title.to_uppercase()),
        (false, false) => Some(format!("{author} / {}", title.to_uppercase())),
    }
}

/// The `assets/…` path of the book's cover, if it has one.
///
/// Only the path — the *inline* cover (every format but EPUB) rides the ordinary
/// image pipeline from here on, so its bytes are collected exactly like any
/// other picture the compiled book names.
pub(super) fn cover_relpath(req: &RenderRequest) -> Option<String> {
    let asset = req.gathered.assets.iter().find(|a| a.is_cover)?;
    Some(skrib_format::media::asset_relpath(
        &asset.content_hash,
        &skrib_format::media::extension_for(&asset.mime_type),
    ))
}

/// The book's cover, if it has one and its file is still there.
///
/// Read outside [`collect_images`] because a cover is not inline content: no
/// prose references it, so a scope-restricted image set would never contain it —
/// and a cover belongs to the *book*, not to whichever chapters this export
/// happens to include.
pub(super) fn cover_image(req: &RenderRequest) -> Option<text_document::ExportImage> {
    if req.media_dir.as_os_str().is_empty() {
        return None;
    }
    let asset = req.gathered.assets.iter().find(|a| a.is_cover)?;
    let ext = skrib_format::media::extension_for(&asset.mime_type);
    let bytes = std::fs::read(req.media_dir.join(format!("{}.{ext}", asset.content_hash))).ok()?;
    Some(text_document::ExportImage::new(
        bytes,
        asset.mime_type.clone(),
    ))
}

/// The bytes an HTML export would need to inline, and nothing more.
///
/// Only [`ImageHandling::Embed`] on HTML reads image files at render time; every
/// other combination either references the files (which the sidecar writes) or
/// drops them. Collecting unconditionally would make a plain Markdown export of
/// an illustrated book read every photograph off disk for nothing.
pub(super) fn images_for_html(
    req: &RenderRequest,
    built: &Assembled,
) -> text_document::ExportImages {
    if req.format != ExportFormat::Html || req.preset.image_handling != ImageHandling::Embed {
        return text_document::ExportImages::new();
    }
    collect_images(req.gathered, req.media_dir, Some(&built.image_refs))
}

/// Write the referenced image files beside the exported document.
///
/// The layout on disk is built from the document's own references rather than
/// from a name this function invents: the prose says `assets/<hash>.png`, so the
/// file is written to `assets/<hash>.png` relative to the output. That is what
/// makes rewriting unnecessary — and rewriting is the part that would have to be
/// separately correct for Djot's link syntax, HTML's URL escaping and LaTeX's
/// backslashes.
///
/// Only the formats that reference an image by path get a sidecar. DOCX, EPUB
/// and PDF carry the bytes inside the file; writing the pictures beside them too
/// would litter the writer's folder with copies nothing reads.
pub(super) fn write_sidecar_images(
    req: &RenderRequest,
    built: &Assembled,
    output: &Path,
    format: ExportFormat,
    handling: ImageHandling,
) -> Result<()> {
    if built.image_refs.is_empty() || !format.references_images() {
        return Ok(());
    }
    match handling {
        ImageHandling::Omit => return Ok(()),
        // A single-file HTML carries its own bytes; the whole point is that
        // there is nothing beside it to lose.
        ImageHandling::Embed if format == ExportFormat::Html => return Ok(()),
        _ => {}
    }
    let Some(parent) = output.parent() else {
        return Ok(());
    };
    let images = collect_images(req.gathered, req.media_dir, Some(&built.image_refs));
    for (relpath, image) in images.iter() {
        let Some(target) = resolve_sidecar_path(parent, relpath) else {
            // A reference that climbs out of the output folder is not written.
            // Prose is user data and a project file can be edited by hand, so
            // this has to be impossible rather than unlikely.
            continue;
        };
        if let Some(dir) = target.parent() {
            fs::create_dir_all(dir).map_err(|e| anyhow!("creating '{}': {e}", dir.display()))?;
        }
        fs::write(&target, &image.bytes)
            .map_err(|e| anyhow!("writing '{}': {e}", target.display()))?;
    }
    Ok(())
}

/// Resolve a document-relative asset path under `parent`, or `None` if it would
/// escape.
///
/// Rejects rather than sanitizes: a reference containing `..` or an absolute
/// root is not a path this exporter should be guessing the intent of.
pub(super) fn resolve_sidecar_path(parent: &Path, relpath: &str) -> Option<std::path::PathBuf> {
    let candidate = Path::new(relpath);
    if candidate.is_absolute() {
        return None;
    }
    for part in candidate.components() {
        match part {
            std::path::Component::Normal(_) => {}
            // `.` is harmless but arrives only from a malformed reference;
            // everything else (`..`, a root, a Windows prefix) escapes.
            _ => return None,
        }
    }
    Some(parent.join(candidate))
}
