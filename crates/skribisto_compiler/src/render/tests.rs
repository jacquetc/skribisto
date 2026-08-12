// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;
use common::entities::{Binder, BinderItem, BinderItemRole, BinderItemSubRole as SR, Work};
use skrib_format::{BinderWithItems, ItemWithContents};

use crate::preset::builtin_presets;

fn c(id: u64, role: ContentRole, data: &str) -> Content {
    Content {
        id,
        activated: true,
        role,
        data: data.to_string(),
        ..Default::default()
    }
}

fn iwc(id: u64, sub_role: SR, lang: &str, contents: Vec<Content>) -> ItemWithContents {
    ItemWithContents {
        item: BinderItem {
            id,
            role: BinderItemRole::Item,
            sub_role,
            // Explicit, like `is_exportable`/`activated` below and for the same reason:
            // `Default` is the **nil** uuid, and a nil uid is what a row carries only
            // before `with_identity` has ever run on it. Leaving it nil here would make
            // every fixture silently exercise the "this row has no durable identity"
            // path — so round-trip marks, which are keyed on it, would never be written
            // in any test.
            uid: common::uid::fixture_uid(id),
            // Split, not wrapped: `iwc(.., "", ..)` must mean "no language, so
            // inherit" — wrapping made it `[""]`, which reads as tagged.
            dict_language: language::parse_legacy_list(lang),
            is_exportable: true,
            activated: true,
            ..Default::default()
        },
        contents,
    }
}

fn gathered(items: Vec<ItemWithContents>, work_lang: &str) -> Gathered {
    Gathered {
        assets: Vec::new(),
        footnotes: Vec::new(),
        work: Work {
            id: 1,
            title: "My Novel".into(),
            author_name: "A. Writer".into(),
            dict_language: language::parse_legacy_list(work_lang),
            // Explicit, because `Work` derives `Default` and `bool::default()` is
            // `false` — a fixture leaning on `..Default::default()` here would test an
            // *unnumbered* manuscript while claiming to test an ordinary one. Same
            // trap `new_work_uc` guards against for real projects.
            number_chapters: true,
            ..Default::default()
        },
        tags: vec![],
        dict_words: vec![],
        text_replacement_rules: vec![],
        note_templates: vec![],
        smart_punctuation: None,
        trash_infos: vec![],
        paces: vec![],
        progress_snapshots: vec![],
        comments: vec![],
        binders: vec![BinderWithItems {
            binder: Binder {
                id: 10,
                ..Default::default()
            },
            items,
        }],
        work_info: None,
    }
}

fn preset(id: &str) -> Preset {
    builtin_presets().into_iter().find(|p| p.id == id).unwrap()
}

// ── Images: the sidecar, the scope, and the cover ────────────────────

/// A real 2×2 PNG, so a decoder anywhere downstream sees an image.
fn png() -> Vec<u8> {
    let mut buf = Vec::new();
    {
        let mut enc = png::Encoder::new(&mut buf, 2, 2);
        enc.set_color(png::ColorType::Rgba);
        enc.set_depth(png::BitDepth::Eight);
        let mut w = enc.write_header().unwrap();
        w.write_image_data(&[9u8, 9, 9, 255].repeat(4)).unwrap();
    }
    buf
}

fn asset(id: u64, hash: &str, is_cover: bool) -> common::entities::Asset {
    common::entities::Asset {
        id,
        content_hash: hash.into(),
        file_name: format!("{hash}.png"),
        mime_type: "image/png".into(),
        width: 2,
        height: 2,
        byte_size: png().len() as u64,
        alt: String::new(),
        is_cover,
        ..Default::default()
    }
}

/// A media directory holding `hashes`, plus a Gathered whose assets name them.
fn book_with_images(hashes: &[&str], cover: Option<&str>) -> (tempfile::TempDir, Gathered) {
    let dir = tempfile::tempdir().unwrap();
    let mut g = gathered(
        vec![
            iwc(
                100,
                SR::BookBegin,
                "en",
                vec![c(1, ContentRole::BookTitle, "My Novel")],
            ),
            iwc(
                101,
                SR::ChapterScene,
                "en",
                vec![c(
                    3,
                    ContentRole::SceneText,
                    &format!("A picture: ![a gull](assets/{}.png)", hashes[0]),
                )],
            ),
        ],
        "en",
    );
    for (i, h) in hashes.iter().enumerate() {
        std::fs::write(dir.path().join(format!("{h}.png")), png()).unwrap();
        g.assets.push(asset(200 + i as u64, h, Some(*h) == cover));
    }
    (dir, g)
}

fn req_with_media<'a>(
    g: &'a Gathered,
    include: &'a [u64],
    p: &'a Preset,
    f: ExportFormat,
    media: &'a std::path::Path,
) -> RenderRequest<'a> {
    RenderRequest {
        media_dir: media,
        gathered: g,
        include,
        preset: p,
        format: f,
        work_lang: "en",
        explicit_selection: false,
    }
}

#[test]
fn a_markdown_export_writes_its_images_beside_it() {
    // Before this the reference was emitted and no file was written, so
    // every exported Markdown/HTML/LaTeX image resolved to nothing.
    let (media, g) = book_with_images(&["aaa"], None);
    let out = tempfile::tempdir().unwrap();
    let path = out.path().join("book.md");
    let p = preset("neutral");
    render_to_file(
        &req_with_media(&g, &[100, 101], &p, ExportFormat::Markdown, media.path()),
        &path,
        &|_| {},
        &AtomicBool::new(false),
    )
    .unwrap();

    let text = std::fs::read_to_string(&path).unwrap();
    assert!(text.contains("assets/aaa.png"), "reference lost: {text}");
    let beside = out.path().join("assets/aaa.png");
    assert!(beside.exists(), "no file was written beside the document");
    assert_eq!(std::fs::read(&beside).unwrap(), png(), "wrong bytes");
}

#[test]
fn the_sidecar_mirrors_the_path_the_prose_names() {
    // The layout is built from the document's own reference rather than a
    // name this exporter invents, which is what makes rewriting the
    // reference — and getting that right per format — unnecessary.
    let (media, g) = book_with_images(&["aaa"], None);
    let out = tempfile::tempdir().unwrap();
    let p = preset("neutral");
    for (fmt, name) in [
        (ExportFormat::Djot, "book.dj"),
        (ExportFormat::Html, "book.html"),
        (ExportFormat::Latex, "book.tex"),
    ] {
        let path = out.path().join(name);
        render_to_file(
            &req_with_media(&g, &[100, 101], &p, fmt, media.path()),
            &path,
            &|_| {},
            &AtomicBool::new(false),
        )
        .unwrap();
        assert!(
            out.path().join("assets/aaa.png").exists(),
            "{fmt:?} wrote no sidecar"
        );
    }
}

#[test]
fn a_container_format_writes_nothing_beside_itself() {
    // DOCX carries its images inside the file; copies beside it would be
    // clutter nothing reads.
    let (media, g) = book_with_images(&["aaa"], None);
    let out = tempfile::tempdir().unwrap();
    let p = preset("neutral");
    render_to_file(
        &req_with_media(&g, &[100, 101], &p, ExportFormat::Docx, media.path()),
        &out.path().join("book.docx"),
        &|_| {},
        &AtomicBool::new(false),
    )
    .unwrap();
    assert!(!out.path().join("assets").exists());
}

#[test]
fn only_the_images_this_scope_names_are_written() {
    // Exporting one chapter must not drop the whole book's photographs into
    // the writer's folder.
    let (media, g) = book_with_images(&["aaa", "bbb"], None);
    let out = tempfile::tempdir().unwrap();
    let path = out.path().join("book.md");
    let p = preset("neutral");
    render_to_file(
        &req_with_media(&g, &[100, 101], &p, ExportFormat::Markdown, media.path()),
        &path,
        &|_| {},
        &AtomicBool::new(false),
    )
    .unwrap();
    assert!(
        out.path().join("assets/aaa.png").exists(),
        "named image missing"
    );
    assert!(
        !out.path().join("assets/bbb.png").exists(),
        "an image the export never mentions was copied out"
    );
}

#[test]
fn omitting_images_drops_the_reference_and_writes_no_files() {
    // A dangling reference is worse than no image: in LaTeX it is a build
    // failure, and everywhere else a broken picture.
    let (media, g) = book_with_images(&["aaa"], None);
    let out = tempfile::tempdir().unwrap();
    let path = out.path().join("book.md");
    let mut p = preset("neutral");
    p.image_handling = ImageHandling::Omit;
    render_to_file(
        &req_with_media(&g, &[100, 101], &p, ExportFormat::Markdown, media.path()),
        &path,
        &|_| {},
        &AtomicBool::new(false),
    )
    .unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    assert!(!text.contains("assets/aaa.png"), "{text}");
    assert!(!out.path().join("assets").exists());
}

#[test]
fn an_embedded_html_export_carries_its_images_and_leaves_nothing_beside() {
    let (media, g) = book_with_images(&["aaa"], None);
    let out = tempfile::tempdir().unwrap();
    let path = out.path().join("book.html");
    let mut p = preset("neutral");
    p.image_handling = ImageHandling::Embed;
    render_to_file(
        &req_with_media(&g, &[100, 101], &p, ExportFormat::Html, media.path()),
        &path,
        &|_| {},
        &AtomicBool::new(false),
    )
    .unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    assert!(
        text.contains("data:image/png;base64,"),
        "not inlined: {text}"
    );
    assert!(
        !out.path().join("assets").exists(),
        "a single-file export must leave nothing beside it"
    );
}

#[test]
fn a_reference_that_climbs_out_of_the_folder_is_not_written() {
    // Prose is user data and a project file can be hand-edited, so an
    // export must not be steerable into writing outside its own directory.
    let out = tempfile::tempdir().unwrap();
    let inside = out.path().join("sub");
    std::fs::create_dir(&inside).unwrap();
    assert_eq!(
        resolve_sidecar_path(&inside, "assets/a.png"),
        Some(inside.join("assets/a.png"))
    );
    assert_eq!(resolve_sidecar_path(&inside, "../escape.png"), None);
    assert_eq!(
        resolve_sidecar_path(&inside, "assets/../../escape.png"),
        None
    );
    assert_eq!(resolve_sidecar_path(&inside, "/etc/passwd"), None);
}

#[test]
fn a_cover_opens_the_book_and_is_not_confused_with_the_prose_image() {
    let (media, g) = book_with_images(&["aaa", "ccc"], Some("ccc"));
    let out = tempfile::tempdir().unwrap();
    let path = out.path().join("book.md");
    let p = preset("neutral");
    render_to_file(
        &req_with_media(&g, &[100, 101], &p, ExportFormat::Markdown, media.path()),
        &path,
        &|_| {},
        &AtomicBool::new(false),
    )
    .unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let cover = text.find("assets/ccc.png").expect("no cover in the output");
    let inline = text.find("assets/aaa.png").expect("no prose image");
    assert!(cover < inline, "the cover is not the first thing: {text}");
    // Its alt text is the book's title, not the word "cover".
    assert!(text.contains("![My Novel](assets/ccc.png)"), "{text}");
    // And it is written beside the document like any other reference.
    assert!(out.path().join("assets/ccc.png").exists());
}

#[test]
fn a_scoped_export_does_not_carry_the_books_cover() {
    // A cover opens a book. Exporting one chapter out of the middle is not
    // opening a book.
    let (media, g) = book_with_images(&["aaa", "ccc"], Some("ccc"));
    let out = tempfile::tempdir().unwrap();
    let path = out.path().join("chapter.md");
    let p = preset("neutral");
    render_to_file(
        &req_with_media(&g, &[101], &p, ExportFormat::Markdown, media.path()),
        &path,
        &|_| {},
        &AtomicBool::new(false),
    )
    .unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    assert!(!text.contains("assets/ccc.png"), "{text}");
}

#[test]
fn turning_the_cover_off_leaves_the_prose_untouched() {
    let (media, g) = book_with_images(&["aaa", "ccc"], Some("ccc"));
    let out = tempfile::tempdir().unwrap();
    let path = out.path().join("book.md");
    let mut p = preset("neutral");
    p.book_cover = false;
    render_to_file(
        &req_with_media(&g, &[100, 101], &p, ExportFormat::Markdown, media.path()),
        &path,
        &|_| {},
        &AtomicBool::new(false),
    )
    .unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    assert!(!text.contains("assets/ccc.png"), "{text}");
    assert!(
        text.contains("assets/aaa.png"),
        "the prose image went too: {text}"
    );
}

/// A flat one-book fixture: book title, one chapter with two scenes.
fn flat_book() -> Gathered {
    gathered(
        vec![
            iwc(
                100,
                SR::BookBegin,
                "en",
                vec![c(1, ContentRole::BookTitle, "My Novel")],
            ),
            iwc(
                101,
                SR::ChapterScene,
                "en",
                vec![
                    c(2, ContentRole::ChapterTitle, "Storms"),
                    c(3, ContentRole::SceneText, "The wind rose over the hills."),
                ],
            ),
            iwc(
                102,
                SR::Scene,
                "en",
                vec![c(4, ContentRole::SceneText, "She walked on into the dark.")],
            ),
        ],
        "en",
    )
}

/// A chapter carrying an epigraph, so the ordering and word-count rules have
/// something to bite on. The attribution rides inside the same blockquote, which is
/// what keeps it attached to its quotation through every writer.
fn book_with_epigraph() -> Gathered {
    gathered(
        vec![
            iwc(
                100,
                SR::BookBegin,
                "en",
                vec![c(1, ContentRole::BookTitle, "My Novel")],
            ),
            iwc(
                101,
                SR::ChapterScene,
                "en",
                vec![
                    c(2, ContentRole::ChapterTitle, "Storms"),
                    c(
                        5,
                        ContentRole::EpigraphText,
                        "> Salt is the only honest preservative.\n>\n> {alignment=right}\n> — M. Ferrand",
                    ),
                    c(3, ContentRole::SceneText, "The wind rose over the hills."),
                ],
            ),
        ],
        "en",
    )
}

fn req<'a>(
    g: &'a Gathered,
    include: &'a [u64],
    p: &'a Preset,
    f: ExportFormat,
) -> RenderRequest<'a> {
    RenderRequest {
        media_dir: std::path::Path::new(""),
        gathered: g,
        include,
        preset: p,
        format: f,
        work_lang: "en",
        explicit_selection: false,
    }
}

#[test]
fn renders_a_flat_book_to_html_with_headings_and_prose() {
    let g = flat_book();
    let p = preset("neutral");
    let html = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Html)).unwrap();
    assert!(html.contains("My Novel"), "book title: {html}");
    assert!(
        html.contains("Chapter 1 — Storms"),
        "chapter heading: {html}"
    );
    assert!(html.contains("The wind rose over the hills."), "{html}");
    assert!(html.contains("She walked on into the dark."), "{html}");
    // The prose must NOT become a heading.
    assert!(
        !html.contains("<h2>The wind"),
        "prose leaked into a heading: {html}"
    );
}

#[test]
fn every_text_format_renders_the_prose() {
    let g = flat_book();
    let p = preset("neutral");
    for f in [
        ExportFormat::Djot,
        ExportFormat::PlainText,
        ExportFormat::Markdown,
        ExportFormat::Html,
        ExportFormat::Latex,
    ] {
        let out = render_to_string(&req(&g, &[100, 101, 102], &p, f)).unwrap();
        assert!(out.contains("She walked on into the dark"), "{f:?}: {out}");
    }
}

#[test]
fn chapter_heading_follows_content_language_by_default() {
    // The built-in manuscript presets no longer force a language — headings follow each
    // scene's own resolved language (the fixture's scenes are "en").
    let g = flat_book();
    let p = preset("manuscript-fr");
    let html = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Html)).unwrap();
    assert!(
        html.contains("Chapter 1"),
        "content-language heading: {html}"
    );
    assert!(!html.contains("Chapitre 1"));
}

#[test]
fn a_fixed_heading_language_overrides_the_content_language() {
    // A preset MAY still pin a heading language (`HeadingLanguage::Fixed`); when it does,
    // that wins over the scene's own language.
    let g = flat_book();
    let p = Preset {
        heading_language: HeadingLanguage::Fixed("fr".to_string()),
        ..preset("manuscript-fr")
    };
    let html = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Html)).unwrap();
    assert!(html.contains("Chapitre 1"), "forced french heading: {html}");
    assert!(!html.contains("Chapter 1"));
}

#[test]
fn an_arabic_work_sets_rtl_direction() {
    let g = gathered(
        vec![iwc(
            200,
            SR::Scene,
            "ar",
            vec![c(9, ContentRole::SceneText, "نص عربي هنا.")],
        )],
        "ar",
    );
    let p = preset("neutral");
    let html = render_to_string(&req(&g, &[200], &p, ExportFormat::Html)).unwrap();
    assert!(
        html.contains("rtl"),
        "RTL direction should reach the HTML: {html}"
    );
}

/// `flat_book`, but the second scene's prose opens with an author-placed
/// minor break marker — in the ESCAPED form the editor really persists.
fn book_with_marker(marker: &str) -> Gathered {
    gathered(
        vec![
            iwc(
                100,
                SR::BookBegin,
                "en",
                vec![c(1, ContentRole::BookTitle, "My Novel")],
            ),
            iwc(
                101,
                SR::ChapterScene,
                "en",
                vec![
                    c(2, ContentRole::ChapterTitle, "Storms"),
                    c(3, ContentRole::SceneText, "The wind rose over the hills."),
                ],
            ),
            iwc(
                102,
                SR::Scene,
                "en",
                vec![c(
                    4,
                    ContentRole::SceneText,
                    &format!("{marker}\n\nShe walked on into the dark."),
                )],
            ),
        ],
        "en",
    )
}

#[test]
fn adjacent_scene_items_do_not_break_by_default() {
    // The binder is organisational: two adjacent scenes say nothing about typography,
    // so without a marker the prose must run straight on, even with a glyph configured.
    let g = flat_book();
    let mut p = preset("neutral");
    p.scene_break = SceneBreak::Glyph("###".to_string());
    p.major_scene_break = SceneBreak::Glyph("+++".to_string());
    let txt = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
    assert!(
        !txt.contains("###"),
        "no marker, so no break may appear: {txt}"
    );
    assert!(
        !txt.contains("+++"),
        "no marker, so no break may appear: {txt}"
    );
}

#[test]
fn an_authored_marker_renders_as_the_presets_glyph() {
    let g = book_with_marker("\\* \\* \\*");
    let mut p = preset("neutral");
    p.scene_break = SceneBreak::Glyph("###".to_string());
    let txt = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
    assert!(
        txt.contains("###"),
        "the marker must render as the glyph: {txt}"
    );
    assert!(
        !txt.contains("* * *"),
        "the marker itself must be consumed, not printed: {txt}"
    );
}

// ── pagination ──

/// Every chapter opens a page. The rule that makes a manuscript read as chapters
/// rather than as one unbroken column, and the one Shunn requires outright.
#[test]
fn each_chapter_opens_a_new_page() {
    let g = flat_book();
    let p = preset("manuscript-shunn");
    assert!(p.chapter_starts_page);
    let dj = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Djot)).unwrap();
    assert!(
        dj.contains("page_break_before=true"),
        "the chapter must open a page: {dj}"
    );
}

/// …and turning it off really turns it off, rather than being a knob that reads well
/// and does nothing.
#[test]
fn a_preset_that_declines_page_breaks_gets_none() {
    let g = flat_book();
    let p = Preset {
        book_starts_page: false,
        part_starts_page: false,
        chapter_starts_page: false,
        paratext_starts_page: false,
        book_title_page: false,
        ..preset("neutral")
    };
    let dj = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Djot)).unwrap();
    assert!(!dj.contains("page_break_before"), "{dj}");
}

/// A break on the very first block would open the document on a blank page in the
/// formats that take it literally. Exporting one chapter on its own is exactly that
/// case, and it is the common one — "Export Chapter" from the binder.
#[test]
fn the_first_block_of_an_export_never_carries_a_break() {
    let g = flat_book();
    let p = Preset {
        book_title_page: false,
        ..preset("manuscript-shunn")
    };
    // The chapter alone: its heading is the first thing in the document.
    let dj = render_to_string(&req(&g, &[101], &p, ExportFormat::Djot)).unwrap();
    assert!(
        !dj.contains("page_break_before"),
        "nothing above it to end a page on: {dj}"
    );
}

/// The break rides the *structure*, not the heading text. A preset that prints no
/// chapter heading at all still opens each chapter on its own page — the break simply
/// lands on the chapter's first paragraph instead of on a title.
#[test]
fn a_headingless_chapter_still_opens_a_page_on_its_prose() {
    let g = flat_book();
    let p = Preset {
        chapter_heading: HeadingScheme::None,
        book_title_page: false,
        ..preset("manuscript-shunn")
    };
    let dj = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Djot)).unwrap();
    let brk = dj
        .find("page_break_before")
        .unwrap_or_else(|| panic!("no break in: {dj}"));
    let prose = dj.find("The wind rose").expect("the chapter prose");
    assert!(brk < prose, "the break must lead the prose: {dj}");
}

/// A paratext has no heading to carry a break, so its own first paragraph opens the
/// page. A dedication sharing a page with the end of the copyright notice is not a
/// dedication.
#[test]
fn a_paratext_opens_its_own_page() {
    let mut g = flat_book();
    g.binders[0].items.push(iwc(
        103,
        SR::Paratext,
        "en",
        vec![c(9, ContentRole::ParatextText, "For my mother.")],
    ));
    let p = Preset {
        book_title_page: false,
        ..preset("neutral")
    };
    let dj = render_to_string(&req(&g, &[100, 101, 102, 103], &p, ExportFormat::Djot)).unwrap();
    let brk = dj.rfind("page_break_before").expect("a break");
    let dedication = dj.find("For my mother").expect("the paratext");
    assert!(brk < dedication, "{dj}");
}

/// The flowing formats keep the break too — they are files being written out, and the
/// style already decided there is a page boundary here.
#[test]
fn the_flowing_formats_carry_the_break_they_were_given() {
    let g = flat_book();
    let p = Preset {
        book_title_page: false,
        ..preset("manuscript-shunn")
    };
    let txt = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
    assert!(
        txt.contains('\u{000C}'),
        "a form feed is a page break in a .txt: {txt:?}"
    );

    let md = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Markdown)).unwrap();
    assert!(md.contains("break-before: page"), "{md}");
}

/// …and a style that declines them leaves both formats clean.
#[test]
fn a_style_without_breaks_leaves_no_trace_in_the_flowing_formats() {
    let g = flat_book();
    let p = Preset {
        book_starts_page: false,
        part_starts_page: false,
        chapter_starts_page: false,
        paratext_starts_page: false,
        book_title_page: false,
        ..preset("neutral")
    };
    let txt = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
    assert!(!txt.contains('\u{000C}'), "{txt:?}");
    let md = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Markdown)).unwrap();
    assert!(!md.contains("<div"), "{md}");
}

/// [`flat_book`] with an epigraph on its chapter. A local variant rather than a change
/// to the shared fixture, which a dozen other tests measure against.
fn flat_book_with_epigraph(epi: &str) -> Gathered {
    let mut g = flat_book();
    for it in &mut g.binders[0].items {
        if it.item.id == 101 {
            it.contents.push(c(9, ContentRole::EpigraphText, epi));
        }
    }
    g
}

// ── where the epigraph sits ──

/// The default is the documented convention: chapter title, then epigraph, then body.
#[test]
fn by_default_the_epigraph_follows_the_chapter_title() {
    let g = flat_book_with_epigraph("> A quotation.");
    let p = Preset {
        book_title_page: false,
        ..preset("neutral")
    };
    assert_eq!(p.epigraph_placement, EpigraphPlacement::AfterHeading);
    let txt = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
    let title = txt.find("Storms").expect("the chapter title");
    let epi = txt.find("A quotation").expect("the epigraph");
    let body = txt.find("The wind rose").expect("the prose");
    assert!(title < epi && epi < body, "{txt}");
}

/// …and the other placement really moves it above the title, rather than being a
/// setting that reads well and changes nothing.
#[test]
fn the_other_placement_puts_the_epigraph_above_the_title() {
    let g = flat_book_with_epigraph("> A quotation.");
    let p = Preset {
        epigraph_placement: EpigraphPlacement::BeforeHeading,
        book_title_page: false,
        ..preset("neutral")
    };
    let txt = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
    let title = txt.find("Storms").expect("the chapter title");
    let epi = txt.find("A quotation").expect("the epigraph");
    let body = txt.find("The wind rose").expect("the prose");
    assert!(epi < title && title < body, "{txt}");
}

/// With the epigraph leading, *it* opens the page — and the break has to ride the
/// quotation's own attribute line. An attribute line in front of a `>` block attaches
/// to no block at all, so a break written there would vanish silently.
#[test]
fn a_leading_epigraph_carries_the_chapters_page_break() {
    let g = flat_book_with_epigraph("> A quotation.");
    let p = Preset {
        epigraph_placement: EpigraphPlacement::BeforeHeading,
        book_title_page: false,
        ..preset("manuscript-shunn")
    };
    let dj = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Djot)).unwrap();
    let brk = dj
        .find("page_break_before")
        .unwrap_or_else(|| panic!("no break in:\n{dj}"));
    let title = dj.find("Storms").expect("the chapter title");
    assert!(
        brk < title,
        "the break opens the epigraph, not the title:\n{dj}"
    );
    // Inside the quotation, on the same line as the role it shares a block with.
    let line = dj
        .lines()
        .find(|l| l.contains("page_break_before"))
        .expect("the attribute line");
    assert!(
        line.trim_start().starts_with('>'),
        "the break must sit inside the quotation, not in front of it: {line:?}"
    );
    assert!(line.contains("semantic_role=epigraph"), "{line:?}");
}

/// An epigraph typed as a bare paragraph has no quotation to hang the break on, so it
/// takes it the ordinary way rather than losing it.
#[test]
fn a_bare_paragraph_epigraph_still_gets_its_break() {
    let g = flat_book_with_epigraph("No angle bracket here.");
    let p = Preset {
        epigraph_placement: EpigraphPlacement::BeforeHeading,
        book_title_page: false,
        ..preset("manuscript-shunn")
    };
    let dj = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Djot)).unwrap();
    let line = dj
        .lines()
        .find(|l| l.contains("page_break_before"))
        .unwrap_or_else(|| panic!("no break in:\n{dj}"));
    assert!(!line.trim_start().starts_with('>'), "{line:?}");
    assert!(
        dj.find("page_break_before") < dj.find("No angle bracket"),
        "{dj}"
    );
}

/// Only the first quotation opens the page: a chapter opens once, however many
/// epigraphs it carries.
#[test]
fn only_the_first_quotation_carries_the_break() {
    let g = flat_book_with_epigraph("> First.\n\n> Second.");
    let p = Preset {
        epigraph_placement: EpigraphPlacement::BeforeHeading,
        book_title_page: false,
        ..preset("manuscript-shunn")
    };
    let dj = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Djot)).unwrap();
    assert_eq!(dj.matches("page_break_before").count(), 1, "{dj}");
}

/// …while every quotation is still marked as an epigraph. Asserted on `mark_epigraph`
/// itself rather than on a rendered document: `render_to_string` round-trips through
/// the parser, which folds adjacent `>` groups into one frame, so the round-tripped
/// djot carries one role marker no matter how many the compiler wrote.
#[test]
fn every_quotation_is_marked_but_only_the_first_takes_the_extras() {
    let (out, marked) = mark_epigraph(
        "> First.\n\n> Second.",
        &["page_break_before=true".to_string()],
    );
    assert!(marked);
    assert_eq!(out.matches("semantic_role=epigraph").count(), 2, "{out}");
    assert_eq!(out.matches("page_break_before").count(), 1, "{out}");
}

/// An epigraph with no `>` at all reports that it took nothing, so the caller knows to
/// place the break itself.
#[test]
fn a_bare_paragraph_reports_that_it_carried_nothing() {
    let (out, marked) = mark_epigraph("Just a line.", &["page_break_before=true".to_string()]);
    assert!(!marked);
    assert!(!out.contains("page_break_before"), "{out}");
    assert!(!out.contains("semantic_role"), "{out}");
}

/// The *last* paratext of a run needs a break after it, not only one before. What
/// follows front matter is usually ordinary prose — a prologue, an opening scene —
/// with no structural opener of its own to break on, so a break-before rule alone
/// leaves the last page of the front matter running straight into the body.
#[test]
fn the_body_starts_a_page_after_the_last_paratext() {
    let mut g = flat_book();
    // Front matter, then a plain Scene: exactly the shape the bug showed up in.
    g.binders[0].items.insert(
        1,
        iwc(
            103,
            SR::Paratext,
            "en",
            vec![c(9, ContentRole::ParatextText, "For my mother.")],
        ),
    );
    g.binders[0].items.insert(
        2,
        iwc(
            104,
            SR::Scene,
            "en",
            vec![c(10, ContentRole::SceneText, "The prologue opens.")],
        ),
    );
    let p = Preset {
        book_title_page: false,
        ..preset("neutral")
    };
    let dj =
        render_to_string(&req(&g, &[100, 103, 104, 101, 102], &p, ExportFormat::Djot)).unwrap();
    let dedication = dj.find("For my mother").expect("the paratext");
    let prologue = dj.find("The prologue opens").expect("the scene after it");
    let after = dj[dedication..prologue]
        .find("page_break_before")
        .unwrap_or_else(|| panic!("nothing breaks between them:\n{dj}"));
    let _ = after;
}

// ── the title page ──

/// The title is centred and dropped down the page, not flush at the top left. This is
/// the whole visible difference between a title page and a first line.
#[test]
fn the_title_page_is_centred_and_dropped_down_the_page() {
    let g = flat_book();
    let p = preset("manuscript-shunn");
    let dj = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Djot)).unwrap();
    let title_line = dj
        .lines()
        .position(|l| l.contains("My Novel"))
        .expect("the title");
    let attrs = dj.lines().nth(title_line.saturating_sub(1)).unwrap_or("");
    assert!(
        attrs.contains("alignment=center"),
        "attrs were {attrs:?}\n{dj}"
    );
    assert!(attrs.contains("top_margin="), "attrs were {attrs:?}\n{dj}");
}

/// The body starts on a page of its own. Without this the first chapter runs on
/// underneath the byline, which is the complaint that started all of this.
#[test]
fn the_body_breaks_away_from_the_title_page() {
    let g = flat_book();
    let p = preset("manuscript-shunn");
    let dj = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Djot)).unwrap();
    let title = dj.find("My Novel").expect("the title");
    let brk = dj
        .find("page_break_before")
        .expect("a break after the title page");
    assert!(brk > title, "the break belongs below the title page: {dj}");
    // …and the title page itself never carries one: there is no page above it.
    assert!(
        !dj[..title].contains("page_break_before"),
        "nothing may break above the title: {dj}"
    );
}

/// Shunn puts the rounded word count at the top right, and it is the first thing an
/// editor looks at.
#[test]
fn a_submission_title_page_carries_the_rounded_word_count() {
    let g = flat_book();
    let p = preset("manuscript-shunn");
    assert!(p.title_page_word_count);
    let txt = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
    // The fixture is a dozen words, so Shunn's under-10k rule rounds to 100.
    assert!(txt.contains("about 100 words"), "{txt}");
    assert!(
        txt.find("about 100 words") < txt.find("My Novel"),
        "the count sits above the title: {txt}"
    );
}

/// A trade title page carries the title and the byline and nothing else — no editor
/// is reading it, and a word count on a finished book is noise.
#[test]
fn a_trade_title_page_has_no_word_count() {
    let g = flat_book();
    let p = Preset {
        book_title_page: true,
        title_page_word_count: false,
        ..preset("neutral")
    };
    let txt = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
    assert!(!txt.contains("words"), "{txt}");
    assert!(txt.contains("My Novel"), "{txt}");
}

/// The byline preposition is generated furniture, so it localizes; the name never does.
#[test]
fn the_byline_is_localized_and_the_name_is_not() {
    let g = flat_book();
    let p = Preset {
        heading_language: HeadingLanguage::Fixed("fr".into()),
        ..preset("manuscript-shunn")
    };
    let txt = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
    assert!(txt.contains("par A. Writer"), "{txt}");
}

// ── the author on the compiled title page ──

/// A book exported with a title-page preset carries the writer's name.
/// The whole point of storing `author_name`: it must reach the page.
#[test]
fn a_title_page_preset_prints_the_author_under_the_title() {
    let g = flat_book();
    let p = preset("manuscript-shunn");
    assert!(p.book_title_page, "fixture preset must have a title page");
    let txt = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
    assert!(txt.contains("A. Writer"), "the author must appear: {txt}");
    assert!(txt.contains("My Novel"), "the title must appear: {txt}");
    assert!(
        txt.find("My Novel") < txt.find("A. Writer"),
        "the author belongs under the title: {txt}"
    );
}

/// The name is **optional**, and blank must mean "omit" — not an empty line
/// where a name would be, and not the string "Untitled" or similar.
#[test]
fn an_empty_author_is_omitted_from_the_title_page() {
    let mut g = flat_book();
    g.work.author_name = String::new();
    let p = preset("manuscript-shunn");
    let txt = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
    assert!(txt.contains("My Novel"), "the title still appears: {txt}");
    assert!(
        !txt.contains("A. Writer"),
        "a cleared author must not linger: {txt}"
    );
}

/// An author whose name starts like an ordered-list marker must survive — see
/// [`escape_block_leading`] for why the escape goes before the `.`, not the letter.
#[test]
fn an_author_named_like_a_list_marker_is_not_eaten() {
    for name in ["A. Writer", "1. Writer", "i. Writer", "J.R.R. Writer"] {
        let mut g = flat_book();
        g.work.author_name = name.to_string();
        let p = preset("manuscript-shunn");
        let txt =
            render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
        assert!(
            txt.contains(name),
            "the author {name:?} must appear verbatim, got: {txt}"
        );
        assert!(
            !txt.contains('\\'),
            "no escape may leak onto the page for {name:?}: {txt}"
        );
    }
}

/// The escape helper itself, per marker family — a backslash belongs before
/// punctuation only.
#[test]
fn block_leading_escapes_pick_the_right_character() {
    // Single special character: escape it directly.
    assert_eq!(escape_block_leading("* Star"), "\\* Star");
    assert_eq!(escape_block_leading("# Sharp"), "\\# Sharp");
    // Ordered-list markers: escape the punctuation, not the alphanumeric.
    assert_eq!(escape_block_leading("A. Writer"), "A\\. Writer");
    assert_eq!(escape_block_leading("1. Thing"), "1\\. Thing");
    assert_eq!(escape_block_leading("i) Roman"), "i\\) Roman");
    // Nothing special: left alone.
    assert_eq!(escape_block_leading("Plain name"), "Plain name");
}

/// The running header degrades sensibly rather than printing a stray
/// separator when only one of the two halves is set.
#[test]
fn the_running_header_handles_a_missing_author_or_title() {
    assert_eq!(
        manuscript_header("My Novel", "A. Writer").as_deref(),
        Some("A. Writer / MY NOVEL")
    );
    assert_eq!(
        manuscript_header("My Novel", "  ").as_deref(),
        Some("MY NOVEL"),
        "no author: no leading separator"
    );
    assert_eq!(
        manuscript_header("", "A. Writer").as_deref(),
        Some("A. Writer"),
        "no title: no trailing separator"
    );
    assert_eq!(manuscript_header("", "").as_deref(), None);
}

#[test]
fn the_two_tiers_render_distinctly() {
    let mut p = preset("neutral");
    p.scene_break = SceneBreak::Glyph("###".to_string());
    p.major_scene_break = SceneBreak::Glyph("+++".to_string());

    let minor = render_to_string(&req(
        &book_with_marker("\\* \\* \\*"),
        &[100, 101, 102],
        &p,
        ExportFormat::PlainText,
    ))
    .unwrap();
    assert!(minor.contains("###") && !minor.contains("+++"), "{minor}");

    let major = render_to_string(&req(
        &book_with_marker("\\# # #"),
        &[100, 101, 102],
        &p,
        ExportFormat::PlainText,
    ))
    .unwrap();
    assert!(major.contains("+++") && !major.contains("###"), "{major}");
}

#[test]
fn a_marker_mid_scene_breaks_inside_one_item() {
    // The case a per-item flag or a marker item structurally cannot express:
    // a viewpoint shift inside one scene, with no binder change at all.
    let g = gathered(
        vec![iwc(
            100,
            SR::Scene,
            "en",
            vec![c(
                1,
                ContentRole::SceneText,
                "She closed the door.\n\n\\* \\* \\*\n\nDawn found him waiting.",
            )],
        )],
        "en",
    );
    let mut p = preset("neutral");
    p.scene_break = SceneBreak::Glyph("###".to_string());
    let txt = render_to_string(&req(&g, &[100], &p, ExportFormat::PlainText)).unwrap();
    assert!(txt.contains("She closed the door."), "{txt}");
    assert!(txt.contains("###"), "mid-scene break must render: {txt}");
    assert!(txt.contains("Dawn found him waiting."), "{txt}");
}

#[test]
fn every_accepted_spelling_of_a_marker_is_recognised() {
    // Guards the escaped forms specifically: these are the bytes that
    // actually reach `Content.data`, not idealised raw strings.
    let mut p = preset("neutral");
    p.scene_break = SceneBreak::Glyph("###".to_string());
    p.major_scene_break = SceneBreak::Glyph("+++".to_string());
    for (marker, expected) in [
        ("\\* \\* \\*", "###"),
        ("\\*\\*\\*", "###"),
        ("\\*", "###"),
        ("\\#", "###"),
        ("\\# # #", "+++"),
        ("\\#\\#\\#", "+++"),
    ] {
        let g = book_with_marker(marker);
        let txt =
            render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
        assert!(
            txt.contains(expected),
            "marker {marker:?} → {expected:?}: {txt}"
        );
    }
}

#[test]
fn prose_that_merely_resembles_a_marker_is_left_alone() {
    // Exact-match by intent: ordinary prose containing an asterisk must not
    // be silently eaten and replaced by a scene break.
    let g = book_with_marker("He was \\*emphatic\\* about it.");
    let mut p = preset("neutral");
    p.scene_break = SceneBreak::Glyph("###".to_string());
    let txt = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
    assert!(txt.contains("emphatic"), "prose must survive: {txt}");
    assert!(!txt.contains("###"), "prose must not become a break: {txt}");
}

#[test]
fn a_blank_line_break_is_no_longer_identical_to_none() {
    // `BlankLine` renders as real leading plus a suppressed indent on the following
    // paragraph — what most of the world's publishing traditions actually use.
    let g = book_with_marker("\\* \\* \\*");
    let mut blank = preset("neutral");
    blank.scene_break = SceneBreak::BlankLine;
    let mut none = preset("neutral");
    none.scene_break = SceneBreak::None;

    let with_blank =
        render_to_string(&req(&g, &[100, 101, 102], &blank, ExportFormat::Html)).unwrap();
    let with_none =
        render_to_string(&req(&g, &[100, 101, 102], &none, ExportFormat::Html)).unwrap();
    assert_ne!(
        with_blank, with_none,
        "BlankLine must differ from None:\n{with_blank}"
    );
    assert!(
        with_blank.contains("margin-top"),
        "BlankLine must open a real gap: {with_blank}"
    );
}

#[test]
fn a_break_suppresses_the_next_paragraphs_first_line_indent() {
    let g = book_with_marker("\\* \\* \\*");
    let mut p = preset("neutral");
    p.scene_break = SceneBreak::Glyph("###".to_string());
    let html = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Html)).unwrap();
    assert!(
        html.contains("text-indent: 0px"),
        "the paragraph after a break must not be indented: {html}"
    );
}

#[test]
fn a_glyph_break_is_centred() {
    let g = book_with_marker("\\* \\* \\*");
    let mut p = preset("neutral");
    p.scene_break = SceneBreak::Glyph("###".to_string());
    let html = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Html)).unwrap();
    assert!(
        html.contains("text-align: center"),
        "a dinkus is centred in print: {html}"
    );
}

#[test]
fn a_marker_does_not_count_as_prose_words() {
    let plain = flat_book();
    let marked = book_with_marker("\\* \\* \\*");
    let p = preset("neutral");
    let words_of = |g: &Gathered| {
        assemble(
            &req(g, &[100, 101, 102], &p, ExportFormat::PlainText),
            &|_| {},
            &AtomicBool::new(false),
        )
        .unwrap()
        .stats
        .words
    };
    let (a, b) = (words_of(&plain), words_of(&marked));
    assert_eq!(a, b, "a scene break is furniture, not three words");
}

#[test]
fn a_break_does_not_leak_across_a_chapter_boundary() {
    // A marker ending one chapter must not style the first paragraph of the next,
    // even when the preset emits no chapter heading to reset the flow.
    let g = gathered(
        vec![
            iwc(
                100,
                SR::ChapterScene,
                "en",
                vec![c(1, ContentRole::SceneText, "End of one.\n\n\\* \\* \\*")],
            ),
            iwc(
                101,
                SR::ChapterScene,
                "en",
                vec![c(2, ContentRole::SceneText, "Start of two.")],
            ),
        ],
        "en",
    );
    let mut p = preset("neutral");
    p.scene_break = SceneBreak::BlankLine;
    p.chapter_heading = HeadingScheme::None;
    let html = render_to_string(&req(&g, &[100, 101], &p, ExportFormat::Html)).unwrap();
    assert!(
        !html.contains("margin-top"),
        "a trailing break must not carry into the next chapter: {html}"
    );
}

#[test]
fn multi_paragraph_prose_survives_block_splitting_in_both_directions() {
    // `push_prose` now walks every block in both LTR and RTL. Nothing may be
    // lost or reordered by that — no existing test covered multi-block prose.
    for (lang, marker) in [("en", "\\* \\* \\*"), ("ar", "\\* \\* \\*")] {
        let g = gathered(
            vec![iwc(
                100,
                SR::Scene,
                lang,
                vec![c(
                    1,
                    ContentRole::SceneText,
                    &format!("One.\n\nTwo.\n\n{marker}\n\nThree.\n\nFour."),
                )],
            )],
            lang,
        );
        let mut p = preset("neutral");
        p.scene_break = SceneBreak::Glyph("###".to_string());
        let txt = render_to_string(&req(&g, &[100], &p, ExportFormat::PlainText)).unwrap();
        for para in ["One.", "Two.", "Three.", "Four."] {
            assert!(txt.contains(para), "lang={lang} lost {para}: {txt}");
        }
        assert!(txt.contains("###"), "lang={lang}: {txt}");
    }
}

#[test]
fn docx_options_map_the_manuscript_preset() {
    // Shunn: Times New Roman 12pt, double-spaced, 0.5" first-line indent, A4, ragged.
    let p = preset("manuscript-shunn");
    let o = docx_options(&p, "The Lighthouse", "Mara Vane", Default::default());
    assert_eq!(o.font_family.as_deref(), Some("Times New Roman"));
    assert_eq!(o.font_half_points, Some(24), "12pt → 24 half-points");
    assert_eq!(o.line_spacing_twips, Some(480), "double spacing");
    assert_eq!(o.first_line_indent_twips, Some(720), "0.5\" → 720 twips");
    assert_eq!(o.page_width_twips, Some(11906), "A4 width");
    assert_eq!(o.margin_left_twips, Some(1440), "1\" margins by default");
    assert!(!o.justify, "Shunn manuscripts are ragged-right");
    assert!(o.page_numbers, "manuscript pages are numbered");
    assert_eq!(
        o.running_header.as_deref(),
        Some("Mara Vane / THE LIGHTHOUSE"),
        "author / TITLE running header"
    );
}

/// The PDF arm carries an epigraph through Typst. It renders as a `#quote(block: true)`
/// like any blockquote, so what needs pinning is that the *whole* path survives it:
/// the epigraph reaches Typst, Typst compiles it, and the file is real. A malformed
/// block would fail the Typst compile rather than quietly drop the quotation, so a
/// PDF that exists and is larger than the same book without one is the honest signal
/// available from this side of the boundary.
#[cfg(feature = "pdf")]
#[test]
fn pdf_export_carries_an_epigraph() {
    let with = book_with_epigraph();
    let without = {
        let mut g = book_with_epigraph();
        g.binders[0].items[1]
            .contents
            .retain(|c| c.role != ContentRole::EpigraphText);
        g
    };
    let p = preset("manuscript-shunn");

    let render_bytes = |g: &Gathered, tag: &str| {
        let path = std::env::temp_dir().join(format!("skrib-epi-{tag}-{}.pdf", std::process::id()));
        render_to_file(
            &req(g, &[100, 101], &p, ExportFormat::Pdf),
            &path,
            &|_| {},
            &AtomicBool::new(false),
        )
        .unwrap_or_else(|e| panic!("{tag}: {e:#}"));
        let bytes = std::fs::read(&path).unwrap();
        let _ = std::fs::remove_file(&path);
        bytes
    };

    let a = render_bytes(&with, "with");
    let b = render_bytes(&without, "without");
    assert!(a.starts_with(b"%PDF-"), "valid PDF magic bytes");
    assert!(
        a.len() > b.len(),
        "the epigraph must reach the page: {} bytes with, {} without",
        a.len(),
        b.len()
    );
}

#[cfg(feature = "pdf")]
#[test]
fn pdf_export_writes_a_valid_pdf() {
    // manuscript-shunn names "Times New Roman" (no bundled bytes) → exercises the
    // EB Garamond substitution + a real embedded font.
    let g = flat_book();
    let p = preset("manuscript-shunn");
    let path = std::env::temp_dir().join(format!("skrib-export-{}.pdf", std::process::id()));
    let stats = render_to_file(
        &req(&g, &[100, 101, 102], &p, ExportFormat::Pdf),
        &path,
        &|_| {},
        &AtomicBool::new(false),
    )
    .unwrap();
    assert!(path.exists(), "pdf file should be written");
    let bytes = std::fs::read(&path).unwrap();
    assert!(bytes.starts_with(b"%PDF-"), "valid PDF magic bytes");
    assert!(bytes.len() > 500, "non-trivial PDF");
    assert!(stats.items >= 2);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn epub_export_writes_a_non_empty_file() {
    let g = flat_book();
    let p = preset("neutral");
    let path = std::env::temp_dir().join(format!("skrib-export-{}.epub", std::process::id()));
    let stats = render_to_file(
        &req(&g, &[100, 101, 102], &p, ExportFormat::Epub),
        &path,
        &|_| {},
        &AtomicBool::new(false),
    )
    .unwrap();
    assert!(path.exists(), "epub file should be written");
    assert!(
        std::fs::metadata(&path).unwrap().len() > 0,
        "epub should be non-empty"
    );
    assert!(stats.items >= 2);
    let _ = std::fs::remove_file(&path);
}

/// `flat_book`, plus one anchored comment with a reply on the first scene, and one
/// comment whose quoted words are not in the manuscript at all.
fn flat_book_with_comments() -> Gathered {
    use common::entities::{Comment, CommentAnchorKind, CommentReply};
    let mut g = flat_book();
    let scene = "The wind rose over the hills.";
    let (text, starts) = skrib_format::djot_plain_text(scene).expect("plain");
    let at = text.find("the hills").expect("fixture phrase");
    let start = text[..at].chars().count();
    let anchor = skribisto_model::comment_anchor::capture(
        &text,
        start,
        start + "the hills".chars().count(),
        skribisto_model::comment_anchor::block_of(&starts, start),
    );

    g.comments = vec![
        skrib_format::CommentWithReplies {
            comment: Comment {
                id: 900,
                uid: common::uid::fixture_uid(900),
                content: Some(3),
                kind: CommentAnchorKind::Range,
                author_name: "Mara Vane".into(),
                author_initials: "MV".into(),
                body: "Is this the right hill?".into(),
                range_start: anchor.start as u64,
                range_length: anchor.length as u64,
                quote_prefix: anchor.prefix.clone(),
                quote_exact: anchor.exact.clone(),
                quote_exact_truncated: anchor.exact_truncated,
                quote_suffix: anchor.suffix.clone(),
                block_ordinal_hint: anchor.block_ordinal as u64,
                replies: vec![901],
                ..Default::default()
            },
            replies: vec![CommentReply {
                id: 901,
                uid: common::uid::fixture_uid(901),
                author_name: "Editor".into(),
                author_initials: "E".into(),
                body: "It is.".into(),
                ..Default::default()
            }],
        },
        skrib_format::CommentWithReplies {
            comment: Comment {
                id: 910,
                uid: common::uid::fixture_uid(910),
                content: Some(3),
                kind: CommentAnchorKind::Range,
                author_name: "Mara Vane".into(),
                author_initials: "MV".into(),
                body: "About a sentence I deleted.".into(),
                quote_exact: "a sentence that is no longer anywhere".into(),
                range_length: 10,
                ..Default::default()
            },
            replies: vec![],
        },
    ];
    g
}

/// The payload the writers receive: anchored comments in, unplaceable ones counted and
/// dropped rather than written at a guess.
#[test]
fn the_comment_payload_carries_the_thread_and_counts_the_orphan() {
    let g = flat_book_with_comments();
    let p = preset("neutral");
    let r = req(&g, &[100, 101, 102], &p, ExportFormat::Docx);
    let built = assemble(&r, &|_| {}, &AtomicBool::new(false)).unwrap();

    let Payloads {
        comments: payload,
        orphaned,
        ..
    } = export_payloads(&r, &built).unwrap();
    assert_eq!(payload.len(), 1, "the placeable comment is written");
    assert_eq!(
        orphaned, 1,
        "the one whose words are gone is counted, not written"
    );

    let c = payload
        .get(&common::uid::fixture_uid(900).to_string())
        .expect("keyed by the comment's own uid");
    assert_eq!(c.author, "Mara Vane");
    assert_eq!(c.author_initials, "MV");
    assert_eq!(c.body, "Is this the right hill?");
    assert!(c.end > c.start, "a range comment must span real characters");
    assert_eq!(c.replies.len(), 1, "the thread's reply travels with it");
    assert_eq!(c.replies[0].author, "Editor");
    assert_eq!(c.replies[0].uid, common::uid::fixture_uid(901).to_string());

    // The range must land on the words the comment was made on, in the COMPILED
    // document — the whole point of rebasing.
    let text = built.doc.to_addressable_text().unwrap();
    let got: String = text
        .chars()
        .skip(c.start as usize)
        .take((c.end - c.start) as usize)
        .collect();
    assert_eq!(got, "the hills", "rebased onto the wrong words: {got:?}");
}

/// Two comments on the identical range come out with their marks in the same order as
/// their annotations.
///
/// The two payloads are sorted independently by whichever writer receives them, on keys
/// that do not agree: a comment ties on its uid, a mark on its name, and the name is a hash
/// of that uid. Two paragraph comments on one paragraph resolve to the identical extent —
/// `comment_anchor::resolve` always gives a paragraph comment the whole paragraph — so the
/// tie is ordinary, not contrived. Written in opposite orders, a reader matching by
/// position gives each comment the other's identity, and the editor's remark comes home on
/// the wrong thread.
#[test]
fn two_comments_on_one_range_keep_their_marks_in_step_with_their_annotations() {
    use common::entities::{Comment, CommentAnchorKind};

    let mut g = flat_book();
    let scene = "The wind rose over the hills.";
    let (text, starts) = skrib_format::djot_plain_text(scene).expect("plain");
    let anchor = skribisto_model::comment_anchor::capture(
        &text,
        0,
        text.chars().count(),
        skribisto_model::comment_anchor::block_of(&starts, 0),
    );
    // Two paragraph comments on the same paragraph: the same extent, by construction.
    let paragraph_comment = |id: u64| skrib_format::CommentWithReplies {
        comment: Comment {
            id,
            uid: common::uid::fixture_uid(id),
            content: Some(3),
            kind: CommentAnchorKind::Paragraph,
            author_name: "Mara Vane".into(),
            author_initials: "MV".into(),
            body: format!("Remark {id}"),
            range_start: anchor.start as u64,
            range_length: anchor.length as u64,
            quote_prefix: anchor.prefix.clone(),
            quote_exact: anchor.exact.clone(),
            quote_exact_truncated: anchor.exact_truncated,
            quote_suffix: anchor.suffix.clone(),
            block_ordinal_hint: anchor.block_ordinal as u64,
            ..Default::default()
        },
        replies: Vec::new(),
    };
    g.comments = vec![paragraph_comment(920), paragraph_comment(921)];

    let p = preset("neutral");
    let r = req(&g, &[100, 101, 102], &p, ExportFormat::Docx);
    let built = assemble(&r, &|_| {}, &AtomicBool::new(false)).unwrap();
    let Payloads {
        comments, marks, ..
    } = export_payloads(&r, &built).unwrap();

    assert_eq!(comments.len(), 2, "both comments are written");

    // The order each payload will actually be written in. Row marks share the payload and
    // are not what this is about.
    let comment_uids: Vec<String> = comments
        .in_document_order()
        .iter()
        .map(|c| c.uid.clone())
        .collect();
    let mark_names: Vec<String> = marks
        .in_document_order()
        .iter()
        .filter(|m| {
            m.name
                .starts_with(skribisto_model::round_trip::COMMENT_PREFIX)
        })
        .map(|m| m.name.clone())
        .collect();
    assert_eq!(mark_names.len(), 2, "both comments carry identity");

    let expected: Vec<String> = comment_uids
        .iter()
        .map(|u| {
            skribisto_model::round_trip::comment_mark_name(
                &u.parse::<uuid::Uuid>().expect("a uid round-trips"),
            )
        })
        .collect();
    assert_eq!(
        mark_names, expected,
        "the marks must be written in the same order as the comments they identify"
    );
}

/// A format that cannot bring comments home is not given any. Reading them and then
/// discarding them is deliberate (see `export_payloads`); writing them would not be.
#[test]
fn a_format_that_does_not_carry_comments_gets_an_empty_payload() {
    let g = flat_book_with_comments();
    let p = preset("neutral");
    for f in [ExportFormat::Epub, ExportFormat::Html, ExportFormat::Latex] {
        let r = req(&g, &[100, 101, 102], &p, f);
        let built = assemble(&r, &|_| {}, &AtomicBool::new(false)).unwrap();
        let Payloads {
            comments: payload,
            orphaned,
            ..
        } = export_payloads(&r, &built).unwrap();
        assert_eq!(payload.len(), 0, "{f:?} must carry no comments");
        assert_eq!(orphaned, 0, "{f:?} must not warn about them either");
    }
}

/// A writer who asked for a clean copy gets one — and is not warned about it.
///
/// The zero orphan count is the half worth pinning. The same book with comments *on*
/// reports one comment it could not place (the test above), so an implementation that
/// merely skipped writing the payload while still counting the failures would produce
/// "1 comment could not be placed" on an export that was exactly what was asked for.
#[test]
fn a_preset_with_comments_off_writes_none_and_warns_about_none() {
    let g = flat_book_with_comments();
    let mut p = preset("neutral");
    p.include_comments = false;
    for f in [ExportFormat::Docx, ExportFormat::Odt] {
        let r = req(&g, &[100, 101, 102], &p, f);
        let built = assemble(&r, &|_| {}, &AtomicBool::new(false)).unwrap();
        let Payloads {
            comments: payload,
            orphaned,
            ..
        } = export_payloads(&r, &built).unwrap();
        assert_eq!(payload.len(), 0, "{f:?} was asked for a clean copy");
        assert_eq!(
            orphaned, 0,
            "{f:?} must not report a deliberate omission as a dropped comment"
        );
    }
}

/// The default keeps doing what every export did before the switch existed.
///
/// Cheap, and it guards the one mistake this field could make silently: `#[serde(default)]`
/// on a bool is `false`, so a preset saved before the field existed would stop carrying
/// comments with nothing in the UI or the file to say why.
#[test]
fn comments_and_marks_default_to_on_for_every_shipped_preset() {
    for p in crate::preset::builtin_presets() {
        assert!(
            p.include_comments,
            "{} ships with comments off — deliberate? see the field's doc comment",
            p.id
        );
        assert!(p.include_round_trip_marks, "{} ships with marks off", p.id);
    }
    // A preset file as it was saved before these fields existed: a real one, with exactly
    // the two keys removed. Hand-writing a minimal JSON object would not do — most of
    // `Preset` has no serde default, so such a file would fail to load for reasons that
    // have nothing to do with what is under test.
    let mut saved = serde_json::to_value(preset("neutral")).expect("a preset serialises");
    let obj = saved.as_object_mut().expect("a preset is a JSON object");
    obj.remove("include_comments");
    obj.remove("include_round_trip_marks");
    let restored: crate::preset::Preset =
        serde_json::from_value(saved).expect("a preset file predating the fields still loads");
    assert!(restored.include_comments, "an absent key must read as on");
    assert!(restored.include_round_trip_marks, "likewise for marks");
}

// ── Round-trip marks ────────────────────────────────────────────────────────────────

/// Every exported row is named, at the first character of its own prose.
#[test]
fn every_exported_row_gets_a_mark_at_the_start_of_its_prose() {
    use skribisto_model::round_trip::{MarkName, parse_mark_name};

    let g = flat_book();
    let p = preset("neutral");
    let r = req(&g, &[100, 101, 102], &p, ExportFormat::Docx);
    let built = assemble(&r, &|_| {}, &AtomicBool::new(false)).unwrap();
    let payloads = export_payloads(&r, &built).unwrap();

    // 101 and 102 carry prose. 100 is a `BookBegin` whose only content is the book title,
    // which is a heading rather than a `Content` this export emits — so it has no window
    // and no mark, and that is right: there is no passage to point at.
    let rows: Vec<&text_document::DocumentMark> = payloads
        .marks
        .iter()
        .filter(|m| matches!(parse_mark_name(&m.name), Some(MarkName::Row { .. })))
        .collect();
    assert_eq!(rows.len(), 2, "one mark per row with prose: {rows:?}");

    let text = built.doc.to_addressable_text().unwrap();
    for (item_id, phrase) in [(101u64, "The wind rose"), (102, "She walked on")] {
        let uid = common::uid::fixture_uid(item_id);
        let mark = rows
            .iter()
            .find(|m| {
                matches!(
                    parse_mark_name(&m.name),
                    Some(MarkName::Row { ref uid_tag, .. })
                        if skribisto_model::round_trip::uid_matches(&uid, uid_tag)
                )
            })
            .unwrap_or_else(|| panic!("no mark for item {item_id}"));
        assert!(mark.is_point(), "a row mark names a position, not a span");
        let at: String = text.chars().skip(mark.start as usize).take(20).collect();
        assert!(
            at.starts_with(phrase),
            "item {item_id}'s mark landed on {at:?}, not on its own prose"
        );
    }
}

/// The digest in a row's name is the digest of that row's prose — which is what makes the
/// three-way "who changed this" comparison possible on re-import at all.
#[test]
fn a_rows_mark_carries_the_digest_of_its_own_prose() {
    use skribisto_model::round_trip::{MarkName, digest, parse_mark_name};

    let g = flat_book();
    let p = preset("neutral");
    let r = req(&g, &[101], &p, ExportFormat::Odt);
    let built = assemble(&r, &|_| {}, &AtomicBool::new(false)).unwrap();
    let payloads = export_payloads(&r, &built).unwrap();

    let mark = payloads
        .marks
        .iter()
        .find(|m| matches!(parse_mark_name(&m.name), Some(MarkName::Row { .. })))
        .expect("the chapter is marked");
    match parse_mark_name(&mark.name).unwrap() {
        MarkName::Row { digest: d, .. } => {
            assert_eq!(d, digest("The wind rose over the hills."));
        }
        other => panic!("parsed as {other:?}"),
    }
}

/// A scene and its synopsis are two `Content`s of **one** row.
///
/// Both are emitted when the preset keeps synopses, and both would mint a name from the same
/// `BinderItem.uid`. A bookmark name is unique in a document and `DocumentMarks` is keyed by
/// it, so the second would silently replace the first — leaving the row's identity anchored
/// in its summary rather than its prose, with nothing anywhere to say so.
#[test]
fn a_row_with_a_synopsis_is_marked_once_on_its_prose() {
    use skribisto_model::round_trip::{MarkName, parse_mark_name};

    let mut g = flat_book();
    g.binders[0].items[1].contents.push(c(
        9,
        ContentRole::SynopsisText,
        "A summary of the storm chapter.",
    ));
    let mut p = preset("neutral");
    p.include_synopses = true;
    let r = req(&g, &[101], &p, ExportFormat::Docx);
    let built = assemble(&r, &|_| {}, &AtomicBool::new(false)).unwrap();
    let payloads = export_payloads(&r, &built).unwrap();

    let rows: Vec<&text_document::DocumentMark> = payloads
        .marks
        .iter()
        .filter(|m| matches!(parse_mark_name(&m.name), Some(MarkName::Row { .. })))
        .collect();
    assert_eq!(rows.len(), 1, "one row, one mark: {rows:?}");

    let text = built.doc.to_addressable_text().unwrap();
    let at: String = text.chars().skip(rows[0].start as usize).take(13).collect();
    assert_eq!(
        at, "The wind rose",
        "the row's identity must sit on its prose, not on its synopsis"
    );
    match parse_mark_name(&rows[0].name).unwrap() {
        MarkName::Row { digest: d, .. } => assert_eq!(
            d,
            skribisto_model::round_trip::digest("The wind rose over the hills."),
            "the digest must be the prose's, not the synopsis's"
        ),
        other => panic!("parsed as {other:?}"),
    }
}

/// A placed comment is bracketed by a mark over exactly the characters it covers — the only
/// identity it has once an editor has saved the file, since neither Word nor LibreOffice
/// keeps the private uid attribute the writer also emits.
#[test]
fn a_placed_comment_gets_a_range_mark_over_its_own_characters() {
    use skribisto_model::round_trip::{MarkName, parse_mark_name, uid_matches};

    let g = flat_book_with_comments();
    let p = preset("neutral");
    let r = req(&g, &[100, 101, 102], &p, ExportFormat::Docx);
    let built = assemble(&r, &|_| {}, &AtomicBool::new(false)).unwrap();
    let payloads = export_payloads(&r, &built).unwrap();

    let comment_marks: Vec<&text_document::DocumentMark> = payloads
        .marks
        .iter()
        .filter(|m| matches!(parse_mark_name(&m.name), Some(MarkName::Comment { .. })))
        .collect();
    // One mark for the comment that found a home; the orphan is not in the file, so there
    // is nothing for a mark to name.
    assert_eq!(comment_marks.len(), 1, "{comment_marks:?}");

    let uid = common::uid::fixture_uid(900);
    match parse_mark_name(&comment_marks[0].name).unwrap() {
        MarkName::Comment { uid_tag } => assert!(uid_matches(&uid, &uid_tag)),
        other => panic!("parsed as {other:?}"),
    }

    let written = payloads.comments.get(&uid.to_string()).expect("written");
    assert_eq!(
        (comment_marks[0].start, comment_marks[0].end),
        (written.start, written.end),
        "the mark and the comment must bracket the same characters, or a returning file \
             re-anchors the comment somewhere the editor never put it"
    );

    let text = built.doc.to_addressable_text().unwrap();
    let covered: String = text
        .chars()
        .skip(comment_marks[0].start as usize)
        .take((comment_marks[0].end - comment_marks[0].start) as usize)
        .collect();
    assert_eq!(covered, "the hills");
}

#[test]
fn a_preset_with_marks_off_writes_none() {
    let g = flat_book_with_comments();
    let mut p = preset("neutral");
    p.include_round_trip_marks = false;
    for f in [ExportFormat::Docx, ExportFormat::Odt] {
        let r = req(&g, &[100, 101, 102], &p, f);
        let built = assemble(&r, &|_| {}, &AtomicBool::new(false)).unwrap();
        let payloads = export_payloads(&r, &built).unwrap();
        assert!(
            payloads.marks.is_empty(),
            "{f:?} was asked for a clean copy"
        );
        assert!(
            !payloads.comments.is_empty(),
            "{f:?} must still carry its comments — the two switches are separate questions"
        );
    }
}

#[test]
fn a_format_that_cannot_carry_marks_gets_none() {
    let g = flat_book();
    let p = preset("neutral");
    for f in [
        ExportFormat::Epub,
        ExportFormat::Html,
        ExportFormat::Latex,
        ExportFormat::Markdown,
    ] {
        assert!(!f.carries_round_trip_marks(), "{f:?}");
        let r = req(&g, &[100, 101, 102], &p, f);
        let built = assemble(&r, &|_| {}, &AtomicBool::new(false)).unwrap();
        assert!(
            export_payloads(&r, &built).unwrap().marks.is_empty(),
            "{f:?}"
        );
    }
}

/// A row created before identity was minted carries the nil uuid. Marking it would name
/// nothing — and worse, every such row would mint the *same* name, so the last one written
/// would be the only one in the file.
#[test]
fn a_row_with_no_durable_identity_is_left_unmarked() {
    use skribisto_model::round_trip::{MarkName, parse_mark_name};

    let mut g = flat_book();
    g.binders[0].items[1].item.uid = uuid::Uuid::nil();
    let p = preset("neutral");
    let r = req(&g, &[101, 102], &p, ExportFormat::Odt);
    let built = assemble(&r, &|_| {}, &AtomicBool::new(false)).unwrap();
    let payloads = export_payloads(&r, &built).unwrap();

    let rows: Vec<&text_document::DocumentMark> = payloads
        .marks
        .iter()
        .filter(|m| matches!(parse_mark_name(&m.name), Some(MarkName::Row { .. })))
        .collect();
    assert_eq!(rows.len(), 1, "only the identified row is marked: {rows:?}");
    match parse_mark_name(&rows[0].name).unwrap() {
        MarkName::Row { uid_tag, .. } => assert!(skribisto_model::round_trip::uid_matches(
            &common::uid::fixture_uid(102),
            &uid_tag
        )),
        other => panic!("parsed as {other:?}"),
    }
}

/// Marks reach the file, not merely the payload.
#[test]
fn a_real_export_writes_its_marks_into_both_containers() {
    let g = flat_book_with_comments();
    let p = preset("neutral");
    for (fmt, ext, part) in [
        (ExportFormat::Docx, "docx", "word/document.xml"),
        (ExportFormat::Odt, "odt", "content.xml"),
    ] {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(format!("marked.{ext}"));
        render_to_file(
            &req(&g, &[100, 101, 102], &p, fmt),
            &path,
            &|_| {},
            &AtomicBool::new(false),
        )
        .unwrap_or_else(|e| panic!("{fmt:?} export failed: {e:#}"));

        let bytes = std::fs::read(&path).unwrap();
        let mut zip = zip::ZipArchive::new(std::io::Cursor::new(bytes)).unwrap();
        let mut xml = String::new();
        std::io::Read::read_to_string(&mut zip.by_name(part).unwrap(), &mut xml).unwrap();

        let row = skribisto_model::round_trip::row_mark_name(
            &common::uid::fixture_uid(101),
            "The wind rose over the hills.",
        );
        let comment =
            skribisto_model::round_trip::comment_mark_name(&common::uid::fixture_uid(900));
        assert!(xml.contains(&row), "{fmt:?} lost the row mark {row}");
        assert!(
            xml.contains(&comment),
            "{fmt:?} lost the comment mark {comment}"
        );
    }
}

/// The counts reach `RenderStats` through a real export, for both carrying formats.
///
/// The payload builder has its own test; this one exists because a correct builder wired
/// to nothing would still pass that test, and the writer's toast reads these numbers.
#[test]
fn a_real_export_reports_what_it_wrote_and_what_it_dropped() {
    let g = flat_book_with_comments();
    let p = preset("neutral");
    for (fmt, ext) in [(ExportFormat::Docx, "docx"), (ExportFormat::Odt, "odt")] {
        let path =
            std::env::temp_dir().join(format!("skrib-comments-{}.{ext}", std::process::id()));
        let stats = render_to_file(
            &req(&g, &[100, 101, 102], &p, fmt),
            &path,
            &|_| {},
            &AtomicBool::new(false),
        )
        .unwrap_or_else(|e| panic!("{fmt:?} export failed: {e:#}"));

        assert_eq!(
            stats.comments_written, 1,
            "{fmt:?} should write the placeable comment"
        );
        assert_eq!(
            stats.comments_orphaned, 1,
            "{fmt:?} should report the one it could not place"
        );
        let _ = std::fs::remove_file(&path);
    }
}

/// A format that carries no comments reports no counts — so its toast stays silent
/// rather than warning about something the writer cannot act on.
#[test]
fn a_non_carrying_format_reports_no_comment_counts() {
    let g = flat_book_with_comments();
    let p = preset("neutral");
    let path = std::env::temp_dir().join(format!("skrib-nocomments-{}.epub", std::process::id()));
    let stats = render_to_file(
        &req(&g, &[100, 101, 102], &p, ExportFormat::Epub),
        &path,
        &|_| {},
        &AtomicBool::new(false),
    )
    .unwrap();
    assert_eq!(stats.comments_written, 0);
    assert_eq!(stats.comments_orphaned, 0);
    let _ = std::fs::remove_file(&path);
}

/// ODT reaches a real file through the whole pipeline, and the file is a real ODF
/// package.
///
/// The `mimetype`-first check is not decoration: ODF requires that entry to be the
/// **first** in the zip and **stored uncompressed**, and a package that gets it wrong
/// still unzips fine — it simply stops being recognised as a text document by the
/// applications this format exists to reach. A "non-empty file" assertion alone would
/// pass on exactly that failure.
#[test]
fn odt_export_writes_a_real_odf_package() {
    let g = flat_book();
    let p = preset("neutral");
    let path = std::env::temp_dir().join(format!("skrib-export-{}.odt", std::process::id()));
    let stats = render_to_file(
        &req(&g, &[100, 101, 102], &p, ExportFormat::Odt),
        &path,
        &|_| {},
        &AtomicBool::new(false),
    )
    .unwrap();
    assert!(path.exists(), "odt file should be written");
    assert!(stats.items >= 2);

    let bytes = std::fs::read(&path).unwrap();
    // The local-file-header signature, then the first entry's name.
    assert_eq!(&bytes[0..4], b"PK\x03\x04", "not a zip: {:?}", &bytes[0..4]);
    assert!(
        bytes.windows(8).take(64).any(|w| w == b"mimetype"),
        "`mimetype` must be the first entry of an ODF package"
    );
    assert!(
        bytes
            .windows(39)
            .any(|w| w == b"application/vnd.oasis.opendocument.text"),
        "the package must declare the OpenDocument Text media type"
    );
    let _ = std::fs::remove_file(&path);
}

/// The two formats that carry an editor's comments, and only those two. A format added
/// to this set without a reader to bring the comments home would send remarks on a
/// one-way trip — the reason LaTeX was considered and dropped.
#[test]
fn only_docx_and_odt_carry_comments() {
    for f in [ExportFormat::Docx, ExportFormat::Odt] {
        assert!(f.carries_comments(), "{f:?} must carry comments");
    }
    for f in [
        ExportFormat::Djot,
        ExportFormat::PlainText,
        ExportFormat::Markdown,
        ExportFormat::Html,
        ExportFormat::Latex,
        ExportFormat::Epub,
        ExportFormat::Pdf,
    ] {
        assert!(!f.carries_comments(), "{f:?} must NOT carry comments");
    }
}

#[test]
fn docx_export_writes_a_non_empty_file() {
    let g = flat_book();
    let p = preset("neutral");
    let path = std::env::temp_dir().join(format!("skrib-export-{}.docx", std::process::id()));
    let stats = render_to_file(
        &req(&g, &[100, 101, 102], &p, ExportFormat::Docx),
        &path,
        &|_| {},
        &AtomicBool::new(false),
    )
    .unwrap();
    assert!(path.exists(), "docx file should be written");
    assert!(
        std::fs::metadata(&path).unwrap().len() > 0,
        "docx should be non-empty"
    );
    assert!(stats.items >= 2);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn a_trashed_row_in_the_include_set_is_dropped_defensively() {
    let mut g = flat_book();
    g.binders[0].items[2].item.activated = false; // scene 102 trashed
    let p = preset("neutral");
    let txt = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
    assert!(txt.contains("The wind rose"));
    assert!(
        !txt.contains("She walked on"),
        "trashed scene must be excluded: {txt}"
    );
}

fn titled(mut iwc: ItemWithContents, title: &str) -> ItemWithContents {
    iwc.item.title = title.to_string();
    iwc
}

/// A book with a chapter (one scene) and a research note swept in after it.
fn book_with_note() -> Gathered {
    gathered(
        vec![
            iwc(
                100,
                SR::BookBegin,
                "en",
                vec![c(1, ContentRole::BookTitle, "My Novel")],
            ),
            iwc(
                101,
                SR::ChapterScene,
                "en",
                vec![
                    c(2, ContentRole::ChapterTitle, "Storms"),
                    c(3, ContentRole::SceneText, "The wind rose over the hills."),
                ],
            ),
            iwc(
                102,
                SR::Note,
                "en",
                vec![c(4, ContentRole::NoteText, "Research: local weather.")],
            ),
        ],
        "en",
    )
}

#[test]
fn a_swept_note_is_dropped_unless_the_preset_or_an_explicit_pick_keeps_it() {
    let g = book_with_note();
    let mut p = preset("neutral"); // include_notes = false
    // Swept into a whole-book export: the note is dropped by default.
    let txt = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
    assert!(txt.contains("The wind rose"), "{txt}");
    assert!(
        !txt.contains("Research"),
        "a swept note must be dropped by default: {txt}"
    );
    // The preset keeps notes → included.
    p.include_notes = true;
    let with = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::PlainText)).unwrap();
    assert!(
        with.contains("Research"),
        "include_notes should keep the note: {with}"
    );
    // An explicit pick (Export Note / a checked item) overrides the toggle being off.
    p.include_notes = false;
    let ids = [102u64];
    let explicit = RenderRequest {
        media_dir: std::path::Path::new(""),
        explicit_selection: true,
        ..req(&g, &ids, &p, ExportFormat::PlainText)
    };
    let only = render_to_string(&explicit).unwrap();
    assert!(
        only.contains("Research"),
        "an explicit note pick overrides the toggle: {only}"
    );
}

#[test]
fn scene_titles_are_emitted_only_when_the_preset_keeps_them() {
    let g = gathered(
        vec![
            titled(
                iwc(
                    300,
                    SR::Scene,
                    "en",
                    vec![c(1, ContentRole::SceneText, "Alpha prose.")],
                ),
                "Opening",
            ),
            titled(
                iwc(
                    301,
                    SR::Scene,
                    "en",
                    vec![c(2, ContentRole::SceneText, "Closing prose.")],
                ),
                "Ending",
            ),
        ],
        "en",
    );
    let mut p = preset("neutral"); // include_scene_titles = false
    let without = render_to_string(&req(&g, &[300, 301], &p, ExportFormat::Html)).unwrap();
    assert!(
        !without.contains("Opening"),
        "scene titles must not leak when off: {without}"
    );
    // Turned on: the titles head each scene (h1 here — no structural levels present).
    p.include_scene_titles = true;
    let with = render_to_string(&req(&g, &[300, 301], &p, ExportFormat::Html)).unwrap();
    assert!(
        with.contains("Opening") && with.contains("Ending"),
        "scene titles: {with}"
    );
    assert!(
        with.contains("<h1"),
        "scene titles head at h1 with no structural levels: {with}"
    );
}

#[test]
fn a_synopsis_does_not_steal_the_breaks_styling() {
    // A break at the end of a scene belongs to the NEXT scene's opening
    // paragraph. A synopsis is commentary sitting between them, and must not
    // consume the queued attributes on its way past.
    let g = gathered(
        vec![
            iwc(
                100,
                SR::Scene,
                "en",
                vec![
                    c(1, ContentRole::SceneText, "End of one.\n\n\\* \\* \\*"),
                    c(2, ContentRole::SynopsisText, "A synopsis line."),
                ],
            ),
            iwc(
                101,
                SR::Scene,
                "en",
                vec![c(3, ContentRole::SceneText, "Start of two.")],
            ),
        ],
        "en",
    );
    let mut p = preset("neutral");
    p.scene_break = SceneBreak::BlankLine;
    p.include_synopses = true;
    let html = render_to_string(&req(&g, &[100, 101], &p, ExportFormat::Html)).unwrap();
    let syn = html.split("A synopsis line").next().unwrap();
    assert!(
        !syn.ends_with("text-indent: 0px\">"),
        "the synopsis must not carry the break's styling: {html}"
    );
    assert!(
        html.contains("margin-top") && html.contains("Start of two."),
        "the next scene's opening paragraph must carry it: {html}"
    );
}

#[test]
fn a_multi_block_construct_survives_alongside_a_marker() {
    // `split("\\n\\n")` is not a Djot block split: a fenced block containing a
    // blank line is ONE construct spanning two chunks. Scanning for markers
    // must not tear it apart — hence contiguous non-marker blocks are flushed
    // as a single verbatim run. The fence holds a `#` so the scan really runs.
    let g = gathered(
        vec![iwc(
            100,
            SR::Scene,
            "en",
            vec![c(
                1,
                ContentRole::SceneText,
                "```\n# code a\n\ncode b\n```\n\n\\* \\* \\*\n\nAfter.",
            )],
        )],
        "en",
    );
    let mut p = preset("neutral");
    p.scene_break = SceneBreak::Glyph("###".to_string());
    let html = render_to_string(&req(&g, &[100], &p, ExportFormat::Html)).unwrap();
    assert!(
        html.matches("<pre>").count() == 1,
        "the fenced block must stay one construct: {html}"
    );
    assert!(html.contains("code a") && html.contains("code b"), "{html}");
    assert!(html.contains("###"), "the marker still renders: {html}");
}

#[test]
fn a_marker_only_scene_is_not_counted_as_an_emitted_item() {
    let g = gathered(
        vec![iwc(
            100,
            SR::Scene,
            "en",
            vec![c(1, ContentRole::SceneText, "\\* \\* \\*")],
        )],
        "en",
    );
    let mut p = preset("neutral");
    p.scene_break = SceneBreak::Glyph("###".to_string());
    let stats = assemble(
        &req(&g, &[100], &p, ExportFormat::PlainText),
        &|_| {},
        &AtomicBool::new(false),
    )
    .unwrap()
    .stats;
    assert_eq!(stats.items, 0, "a marker is furniture, not an emitted item");
}

#[test]
fn a_glyph_break_keeps_the_rows_direction_in_rtl() {
    let g = gathered(
        vec![iwc(
            100,
            SR::Scene,
            "ar",
            vec![c(
                1,
                ContentRole::SceneText,
                "\u{0623}.\n\n\\* \\* \\*\n\n\u{0628}.",
            )],
        )],
        "ar",
    );
    let mut p = preset("neutral");
    p.scene_break = SceneBreak::Glyph("###".to_string());
    let html = render_to_string(&req(&g, &[100], &p, ExportFormat::Html)).unwrap();
    let glyph_para = html
        .split("###")
        .next()
        .and_then(|s| s.rfind("<p").map(|i| s[i..].to_string()))
        .unwrap_or_default();
    assert!(
        glyph_para.contains("rtl"),
        "the glyph line must not fall back to LTR: {html}"
    );
}

#[test]
fn the_blank_line_gap_scales_with_body_size() {
    let g = book_with_marker("\\* \\* \\*");
    let gap = |pt: f32| {
        let mut p = preset("neutral");
        p.scene_break = SceneBreak::BlankLine;
        p.font_size_pt = pt;
        let html = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Html)).unwrap();
        html.split("margin-top: ")
            .nth(1)
            .and_then(|s| s.split("px").next())
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap_or(0)
    };
    assert!(
        gap(24.0) > gap(8.0),
        "a blank-line break must be about one line at the preset's own size"
    );
}

#[test]
fn every_builtin_preset_exports_every_format_with_both_tiers() {
    // The end-to-end guard: a manuscript carrying both tiers must survive the
    // whole stack — recogniser → assembled Djot → text-document parse → each
    // renderer — under every regional style we ship, not just in unit tests.
    let g = gathered(
        vec![
            iwc(
                100,
                SR::BookBegin,
                "en",
                vec![c(1, ContentRole::BookTitle, "My Novel")],
            ),
            iwc(
                101,
                SR::ChapterScene,
                "en",
                vec![
                    c(2, ContentRole::ChapterTitle, "Storms"),
                    c(
                        3,
                        ContentRole::SceneText,
                        "The wind rose.\n\n\\* \\* \\*\n\nShe waited.",
                    ),
                ],
            ),
            iwc(
                102,
                SR::Scene,
                "en",
                vec![c(4, ContentRole::SceneText, "\\# # #\n\nA year passed.")],
            ),
        ],
        "en",
    );
    let text_formats = [
        ExportFormat::PlainText,
        ExportFormat::Markdown,
        ExportFormat::Html,
        ExportFormat::Djot,
    ];
    // A unique directory per run: a fixed shared path would let two
    // concurrent `cargo test` invocations overwrite each other's output and
    // let the first to finish delete files the second is still asserting on.
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();
    for p in builtin_presets() {
        for f in text_formats {
            let out = render_to_string(&req(&g, &[100, 101, 102], &p, f))
                .unwrap_or_else(|e| panic!("{} / {f:?}: {e:#}", p.id));
            // No trailing period in the needles: Markdown/Djot escape it as
            // `\.`, which is correct output, not a loss.
            assert!(out.contains("The wind rose"), "{} / {f:?}: {out}", p.id);
            assert!(out.contains("She waited"), "{} / {f:?}: {out}", p.id);
            assert!(out.contains("A year passed"), "{} / {f:?}: {out}", p.id);
        }
        // PDF rides the opt-in `pdf` feature (Typst is a heavy dependency),
        // so it joins the matrix only when that feature is on.
        #[cfg(feature = "pdf")]
        let binary = vec![ExportFormat::Docx, ExportFormat::Epub, ExportFormat::Pdf];
        #[cfg(not(feature = "pdf"))]
        let binary = vec![ExportFormat::Docx, ExportFormat::Epub];
        for f in binary {
            let path = dir.join(format!("{}-{f:?}", p.id));
            let stats = render_to_file(
                &req(&g, &[100, 101, 102], &p, f),
                &path,
                &|_| {},
                &AtomicBool::new(false),
            )
            .unwrap_or_else(|e| panic!("{} / {f:?}: {e:#}", p.id));
            assert!(
                std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0) > 0,
                "{} / {f:?} wrote nothing",
                p.id
            );
            assert!(stats.words > 0, "{} / {f:?} counted no words", p.id);
        }
    }
}

#[test]
fn empty_prose_emits_no_block_separator() {
    // A Scene whose SceneText row exists but holds only whitespace must add
    // nothing at all to the assembled Djot — not even a bare separator.
    let g = gathered(
        vec![iwc(
            100,
            SR::Scene,
            "en",
            vec![c(1, ContentRole::SceneText, "   \n\n  ")],
        )],
        "en",
    );
    let p = preset("neutral");
    let out = render_to_string(&req(&g, &[100], &p, ExportFormat::Djot)).unwrap();
    assert!(
        out.trim().is_empty(),
        "empty prose must emit nothing: {out:?}"
    );
}

#[test]
fn the_blank_line_gap_accounts_for_paragraph_spacing() {
    // A preset that already spaces paragraphs supplies part of the gap, so
    // the break must add only the remainder rather than doubling it.
    let g = book_with_marker("\\* \\* \\*");
    let gap = |spacing_pt: f32| {
        let mut p = preset("neutral");
        p.scene_break = SceneBreak::BlankLine;
        p.paragraph_spacing_pt = spacing_pt;
        let html = render_to_string(&req(&g, &[100, 101, 102], &p, ExportFormat::Html)).unwrap();
        html.split("margin-top: ")
            .nth(1)
            .and_then(|s| s.split("px").next())
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap_or(0)
    };
    assert!(
        gap(6.0) < gap(0.0),
        "an already-spaced preset must not get the full extra gap"
    );
}

// ─────────────────────────────────────────────────────────────────────────
// Paratexts
// ─────────────────────────────────────────────────────────────────────────

/// A chapter, a paratext between it and the next, and a second chapter — the shape
/// that proves a paratext neither takes a number nor disturbs the ones around it.
fn book_with_paratext() -> Gathered {
    gathered(
        vec![
            iwc(
                100,
                SR::BookBegin,
                "en",
                vec![c(1, ContentRole::BookTitle, "My Novel")],
            ),
            iwc(
                101,
                SR::ChapterScene,
                "en",
                vec![c(2, ContentRole::SceneText, "The wind rose.")],
            ),
            ItemWithContents {
                item: BinderItem {
                    id: 102,
                    role: BinderItemRole::Item,
                    sub_role: SR::Paratext,
                    title: "Acknowledgements".into(),
                    dict_language: language::parse_legacy_list("en"),
                    is_exportable: true,
                    activated: true,
                    ..Default::default()
                },
                contents: vec![c(
                    3,
                    ContentRole::ParatextText,
                    "With thanks to the archivists.",
                )],
            },
            iwc(
                103,
                SR::ChapterScene,
                "en",
                vec![c(4, ContentRole::SceneText, "She walked on.")],
            ),
        ],
        "en",
    )
}

/// A paratext exports its **prose and nothing else**. Its title is a binder label the
/// writer chose to find it by — "Copyright", "front matter (draft)" — not a line of
/// the book, so writing it into the export would put the writer's private filing
/// vocabulary on the page. A heading, if one is wanted, is written in the prose where
/// the writer controls its wording.
#[test]
fn a_paratext_exports_its_prose_and_not_its_title() {
    let g = book_with_paratext();
    let p = preset("neutral");
    let out = render_to_string(&req(&g, &[100, 101, 102, 103], &p, ExportFormat::Djot)).unwrap();
    assert!(
        out.contains("With thanks to the archivists."),
        "prose: {out}"
    );
    assert!(
        !out.contains("Acknowledgements"),
        "the binder label must not reach the book: {out}"
    );
}

/// The chapters either side of an interleaved paratext keep their own numbering.
#[test]
fn a_paratext_between_chapters_does_not_renumber_them() {
    let g = book_with_paratext();
    let p = preset("neutral");
    let out = render_to_string(&req(&g, &[100, 101, 102, 103], &p, ExportFormat::Djot)).unwrap();
    assert!(out.contains("Chapter 1"), "{out}");
    assert!(out.contains("Chapter 2"), "{out}");
    assert!(!out.contains("Chapter 3"), "only two chapters exist: {out}");
}

/// A paratext is the author's, but it is not the manuscript. Its words must not move
/// the count, or every pace goal in the project drifts by the length of the front
/// matter with nothing looking wrong.
#[test]
fn a_paratext_adds_no_words_to_the_manuscript() {
    let p = preset("neutral");
    let words = |g: &Gathered, include: &[u64]| {
        assemble(
            &req(g, include, &p, ExportFormat::Djot),
            &|_| {},
            &AtomicBool::new(false),
        )
        .unwrap()
        .stats
        .words
    };
    let g = book_with_paratext();
    assert_eq!(
        words(&g, &[100, 101, 102, 103]),
        words(&g, &[100, 101, 103]),
        "the paratext must not be counted"
    );
}

/// The preset can leave it out — the clean-submission case, where an editor wants the
/// manuscript and not the acknowledgements.
#[test]
fn the_preset_can_drop_a_paratext() {
    let g = book_with_paratext();
    let mut p = preset("neutral");
    assert!(p.include_paratexts, "shipped on by default");

    p.include_paratexts = false;
    let out = render_to_string(&req(&g, &[100, 101, 102, 103], &p, ExportFormat::Djot)).unwrap();
    assert!(!out.contains("Acknowledgements"), "dropped: {out}");
    assert!(!out.contains("archivists"), "body dropped too: {out}");
    assert!(
        out.contains("The wind rose."),
        "the manuscript stays: {out}"
    );
}

/// A preset saved before the field existed must keep its paratexts, for the same
/// reason `include_epigraphs` must: a bare serde default reads absence as `false`.
#[test]
fn a_preset_saved_before_paratexts_still_keeps_them() {
    let mut v = serde_json::to_value(preset("neutral")).unwrap();
    let obj = v.as_object_mut().unwrap();
    assert!(
        obj.remove("include_paratexts").is_some(),
        "field must serialize"
    );
    let old: Preset = serde_json::from_value(v).expect("an older preset must still load");
    assert!(old.include_paratexts, "absence must read as on");
}

// ─────────────────────────────────────────────────────────────────────────
// Heading numbers
// ─────────────────────────────────────────────────────────────────────────

/// A five-chapter book, of which only the fifth is exported.
fn five_chapter_book() -> Gathered {
    let mut items = vec![iwc(
        100,
        SR::BookBegin,
        "en",
        vec![c(1, ContentRole::BookTitle, "My Novel")],
    )];
    for n in 1..=5u64 {
        items.push(iwc(
            100 + n,
            SR::ChapterScene,
            "en",
            vec![
                c(200 + n, ContentRole::ChapterTitle, &format!("Chapter {n}")),
                c(300 + n, ContentRole::SceneText, &format!("Scene {n}.")),
            ],
        ));
    }
    gathered(items, "en")
}

/// Exporting one chapter must report the number it carries in the manuscript, not its
/// position in the export. A writer sending chapter five to a reader was handed a
/// document that called itself chapter one.
#[test]
fn a_scoped_export_numbers_a_chapter_as_the_book_does() {
    let g = five_chapter_book();
    let p = preset("neutral");
    // Exactly what `ScopeKind::Chapter` resolves for the fifth chapter.
    let out = render_to_string(&req(&g, &[105], &p, ExportFormat::Djot)).unwrap();
    assert!(
        out.contains("Chapter 5"),
        "the fifth chapter must be numbered five: {out}"
    );
    assert!(
        !out.contains("Chapter 1"),
        "and must not be renumbered from one: {out}"
    );
}

/// A title that already says the number must not have it prepended again — the exact
/// shape the bug report showed, "Chapter 1 — Chapter 5".
#[test]
fn a_number_is_not_repeated_by_a_title_that_already_carries_it() {
    let g = five_chapter_book();
    let p = preset("neutral");
    let out = render_to_string(&req(&g, &[105], &p, ExportFormat::Djot)).unwrap();
    assert!(
        !out.contains("Chapter 5 — Chapter 5"),
        "the number must not appear twice: {out}"
    );
    let heading = out.lines().find(|l| l.starts_with('#')).expect("a heading");
    assert_eq!(heading, "# Chapter 5", "got {heading:?}");
}

/// A real title still gets the number in front of it — the de-duplication above must
/// not swallow titles generally.
#[test]
fn a_real_title_still_follows_its_number() {
    let mut g = five_chapter_book();
    g.binders[0].items[5].contents[0].data = "The Storm".to_string();
    let p = preset("neutral");
    let out = render_to_string(&req(&g, &[105], &p, ExportFormat::Djot)).unwrap();
    assert!(out.contains("Chapter 5 — The Storm"), "{out}");
}

/// A whole-project export is unchanged: its first row *is* the first row, so nothing
/// is seeded and the chapters read 1..5 exactly as before.
#[test]
fn a_full_export_still_numbers_from_one() {
    let g = five_chapter_book();
    let p = preset("neutral");
    let out = render_to_string(&req(
        &g,
        &[100, 101, 102, 103, 104, 105],
        &p,
        ExportFormat::Djot,
    ))
    .unwrap();
    for n in 1..=5 {
        assert!(
            out.contains(&format!("Chapter {n}")),
            "chapter {n} missing: {out}"
        );
    }
}

/// Blank every chapter title in `five_chapter_book`.
///
/// That fixture titles its chapters the literal "Chapter 1".."Chapter 5" — deliberately,
/// because that is what this app's own new-project template writes and what the
/// redundancy guard exists to collapse. It is the wrong fixture for asserting *about
/// numerals*, though: "Chapter 4 — Chapter 5" contains the substring "Chapter 5" as a
/// title, so a test checking "the numeral 5 is gone" would read a title as a number.
fn untitle_chapters(g: &mut Gathered) {
    for item in g.binders[0].items.iter_mut() {
        for c in item.contents.iter_mut() {
            if c.role == ContentRole::ChapterTitle {
                c.data.clear();
            }
        }
    }
}

/// **The seed-vs-selection bug.** The old design counted twice: incrementally over the
/// rows an export had already been filtered to, and again — unfiltered — to seed a
/// scoped export. Mark chapter three non-exportable and the two answered differently
/// for the *same* chapter: a full book export called chapter five "Chapter 4", while
/// exporting that chapter alone called it "Chapter 5".
///
/// Now there is one pass over the whole manuscript, so both agree. Which answer they
/// agree *on* is the second half of the fix: a chapter the writer took out of the book
/// holds no number and leaves no gap, so five chapters minus one read 1..4.
#[test]
fn a_non_exportable_chapter_cannot_make_two_exports_disagree() {
    let mut g = five_chapter_book();
    untitle_chapters(&mut g);
    g.binders[0].items[3].item.is_exportable = false; // the third chapter (id 103)
    let p = preset("neutral");

    let heading_of = |out: &str| {
        out.lines()
            .find(|l| l.starts_with('#'))
            .unwrap_or_default()
            .to_string()
    };

    // Scoped: chapter five on its own.
    let scoped = render_to_string(&req(&g, &[105], &p, ExportFormat::Djot)).unwrap();
    // Full: every exportable chapter. 103 is absent, exactly as `push_swept` builds it.
    let full =
        render_to_string(&req(&g, &[100, 101, 102, 104, 105], &p, ExportFormat::Djot)).unwrap();

    assert_eq!(
        heading_of(&scoped),
        "# Chapter 4",
        "the scoped export must not count a chapter that is not in the book: {scoped}"
    );
    assert!(
        full.contains("Chapter 4") && !full.contains("Chapter 5"),
        "the full export numbers the survivors 1..4 with no gap: {full}"
    );
}

/// A prologue must not take "Chapter 1" from the real first chapter — and must not
/// print a number of its own either. It keeps its title, its prose and its heading.
#[test]
fn an_unnumbered_chapter_neither_prints_nor_consumes_a_number() {
    let mut g = five_chapter_book();
    untitle_chapters(&mut g);
    // The first chapter becomes the prologue, titled as one.
    g.binders[0].items[1].item.exclude_from_numbering = true;
    g.binders[0].items[1].contents[0].data = "Prologue".to_string();
    let p = preset("neutral");
    let out = render_to_string(&req(
        &g,
        &[100, 101, 102, 103, 104, 105],
        &p,
        ExportFormat::Djot,
    ))
    .unwrap();

    assert!(
        out.contains("# Prologue"),
        "the prologue keeps its own heading: {out}"
    );
    assert!(
        !out.contains("Chapter 1 — Prologue"),
        "and carries no numeral of its own: {out}"
    );
    // The chapter after it is chapter one, and the last is chapter four.
    assert!(
        out.contains("Chapter 1"),
        "the chapter after a prologue is chapter one: {out}"
    );
    assert!(
        out.contains("Chapter 4") && !out.contains("Chapter 5"),
        "…and the rest shift down with it: {out}"
    );
}

/// `Work.number_chapters = false` has to reach the exported file, not merely the UI.
/// The style still asks for `NumberAndTitle`; the manuscript overrides it.
#[test]
fn a_work_with_numbering_off_exports_titles_without_numerals() {
    let mut g = five_chapter_book();
    g.work.number_chapters = false;
    // Give the chapters real titles, so there is something left once numbers go.
    for n in 1..=5usize {
        g.binders[0].items[n].contents[0].data = format!("Title {n}");
    }
    let p = preset("neutral");
    assert_eq!(p.chapter_heading, HeadingScheme::NumberAndTitle);
    let out = render_to_string(&req(
        &g,
        &[100, 101, 102, 103, 104, 105],
        &p,
        ExportFormat::Djot,
    ))
    .unwrap();
    assert!(out.contains("# Title 1"), "titles survive: {out}");
    assert!(
        !out.contains("Chapter 1") && !out.contains(" — "),
        "no numeral and no separator anywhere: {out}"
    );
}

/// **The hole the first attempt at "numbering off" left open.**
///
/// Gating only the *schemes* was not enough. `TitleOnly` falls back to the numeral when
/// a title is blank, and a Book's scheme is hardcoded rather than read from the preset,
/// so it never passed through the clamp at all — an untitled book or chapter still
/// printed "Book 1" / "Chapter 3" into a manuscript whose writer had switched numbering
/// off. The gate now lives at the number map, so no scheme has anything to fall back to.
#[test]
fn numbering_off_prints_no_numeral_even_for_untitled_rows() {
    let mut g = five_chapter_book();
    untitle_chapters(&mut g);
    // …and an untitled book, which is the case that reached the exporter unclamped.
    for c in g.binders[0].items[0].contents.iter_mut() {
        c.data.clear();
    }
    g.work.number_chapters = false;
    let ids = [100, 101, 102, 103, 104, 105];

    for scheme in [
        HeadingScheme::Numbered,
        HeadingScheme::TitleOnly,
        HeadingScheme::NumberAndTitle,
    ] {
        let mut p = preset("neutral");
        p.chapter_heading = scheme;
        p.part_heading = scheme;
        // Force the Book opener down the heading path rather than the title page.
        p.book_title_page = false;
        let out = render_to_string(&req(&g, &ids, &p, ExportFormat::Djot)).unwrap();
        for word in ["Chapter", "Part", "Book"] {
            assert!(
                !out.contains(word),
                "{scheme:?} leaked a generated {word} into an unnumbered manuscript: {out}"
            );
        }
    }
}

/// The converse, so the gate cannot be "fixed" by simply never numbering: with the
/// switch on, an untitled chapter is still opened by its number.
#[test]
fn numbering_on_still_names_an_untitled_chapter_by_its_number() {
    let mut g = five_chapter_book();
    untitle_chapters(&mut g);
    let p = preset("neutral");
    let out = render_to_string(&req(&g, &[105], &p, ExportFormat::Djot)).unwrap();
    let heading = out.lines().find(|l| l.starts_with('#')).expect("a heading");
    assert_eq!(heading, "# Chapter 5", "got {heading:?}");
}

/// `part_resets_chapter` is a `Work` setting, and it works.
#[test]
fn a_part_restarts_chapters_when_the_work_asks_it_to() {
    let mut g = gathered(
        vec![
            iwc(100, SR::BookBegin, "en", vec![]),
            iwc(101, SR::Part, "en", vec![]),
            iwc(
                102,
                SR::ChapterScene,
                "en",
                vec![c(1, ContentRole::SceneText, "A.")],
            ),
            iwc(
                103,
                SR::ChapterScene,
                "en",
                vec![c(2, ContentRole::SceneText, "B.")],
            ),
            iwc(104, SR::Part, "en", vec![]),
            iwc(
                105,
                SR::ChapterScene,
                "en",
                vec![c(3, ContentRole::SceneText, "C.")],
            ),
        ],
        "en",
    );
    let p = preset("neutral");
    let ids = [100, 101, 102, 103, 104, 105];

    // Default: chapters run on across the part boundary.
    let out = render_to_string(&req(&g, &ids, &p, ExportFormat::Djot)).unwrap();
    assert!(out.contains("Chapter 3"), "continuous by default: {out}");

    // Opted in: the second part opens on chapter one again.
    g.work.part_resets_chapter = true;
    let out = render_to_string(&req(&g, &ids, &p, ExportFormat::Djot)).unwrap();
    assert!(!out.contains("Chapter 3"), "chapters restart: {out}");
    assert!(out.contains("Part 2"), "…but parts do not: {out}");
}

/// A title of nothing but spaces is no title, not a title made of spaces. It used to
/// reach the composer and render "Chapter 5 —    ", dangling dash and all.
#[test]
fn a_blank_title_does_not_leave_a_dangling_separator() {
    let mut g = five_chapter_book();
    g.binders[0].items[5].contents[0].data = "   ".to_string();
    let p = preset("neutral");
    let out = render_to_string(&req(&g, &[105], &p, ExportFormat::Djot)).unwrap();
    let heading = out.lines().find(|l| l.starts_with('#')).expect("a heading");
    assert_eq!(heading, "# Chapter 5", "got {heading:?}");
}

/// The normalized guard, end to end: the shapes the old byte-comparison let through.
#[test]
fn a_title_restating_its_number_is_collapsed_however_it_is_spelled() {
    for title in ["chapter 5", "Chapter 5.", "Chapter\u{00A0}5", "5"] {
        let mut g = five_chapter_book();
        g.binders[0].items[5].contents[0].data = title.to_string();
        let p = preset("neutral");
        let out = render_to_string(&req(&g, &[105], &p, ExportFormat::Djot)).unwrap();
        let heading = out.lines().find(|l| l.starts_with('#')).expect("a heading");
        assert_eq!(heading, "# Chapter 5", "title {title:?} gave {heading:?}");
    }
}

/// …and a title naming a *different* number is left visibly doubled on purpose. It is
/// not a duplicate, it is the writer's own count disagreeing with where the row now
/// sits, and hiding half of it would hide the disagreement.
#[test]
fn a_title_naming_a_different_number_is_left_visible() {
    let mut g = five_chapter_book();
    g.binders[0].items[5].contents[0].data = "Chapter 3".to_string();
    let p = preset("neutral");
    let out = render_to_string(&req(&g, &[105], &p, ExportFormat::Djot)).unwrap();
    assert!(out.contains("Chapter 5 — Chapter 3"), "{out}");
}

/// LaTeX must not print its own counter in front of a heading this compiler already
/// numbered. `article`'s default `secnumdepth` of 3 numbers `\section`, so a scoped
/// chapter export rendered "1  Chapter 5" — the export's local counter and the
/// manuscript's real number, disagreeing, side by side. The Typst backend has always
/// suppressed its own numbering for exactly this reason; LaTeX now does too.
#[test]
fn latex_does_not_number_a_heading_this_compiler_already_numbered() {
    let mut g = five_chapter_book();
    untitle_chapters(&mut g);
    let p = preset("neutral");
    let out = render_to_string(&req(&g, &[105], &p, ExportFormat::Latex)).unwrap();
    assert!(
        out.contains("\\setcounter{secnumdepth}{-1}"),
        "the preamble must suppress LaTeX's own numbering: {out}"
    );
    assert!(
        out.contains("Chapter 5"),
        "…while the manuscript's own number survives: {out}"
    );
}

/// The separator is the style's to choose.
#[test]
fn the_heading_separator_comes_from_the_preset() {
    let mut g = five_chapter_book();
    g.binders[0].items[5].contents[0].data = "The Storm".to_string();
    let mut p = preset("neutral");
    p.heading_separator = ": ".to_string();
    let out = render_to_string(&req(&g, &[105], &p, ExportFormat::Djot)).unwrap();
    assert!(out.contains("Chapter 5: The Storm"), "{out}");
}

/// A second book restarts its chapter numbering; a part inside one book does not.
/// Trade practice runs chapters continuously across the parts of a book.
#[test]
fn a_new_book_restarts_chapters_but_a_new_part_does_not() {
    let g = gathered(
        vec![
            iwc(
                100,
                SR::BookBegin,
                "en",
                vec![c(1, ContentRole::BookTitle, "One")],
            ),
            iwc(
                101,
                SR::ChapterScene,
                "en",
                vec![c(2, ContentRole::SceneText, "a")],
            ),
            iwc(
                102,
                SR::Part,
                "en",
                vec![c(3, ContentRole::PartTitle, "Second Part")],
            ),
            iwc(
                103,
                SR::ChapterScene,
                "en",
                vec![c(4, ContentRole::SceneText, "b")],
            ),
            iwc(
                104,
                SR::BookBegin,
                "en",
                vec![c(5, ContentRole::BookTitle, "Two")],
            ),
            iwc(
                105,
                SR::ChapterScene,
                "en",
                vec![c(6, ContentRole::SceneText, "c")],
            ),
        ],
        "en",
    );
    let p = preset("neutral");

    // The chapter after the part is the book's second, not its first.
    let after_part = render_to_string(&req(&g, &[103], &p, ExportFormat::Djot)).unwrap();
    assert!(
        after_part.contains("Chapter 2"),
        "a part must not restart chapters: {after_part}"
    );

    // The chapter in the second book is that book's first.
    let second_book = render_to_string(&req(&g, &[105], &p, ExportFormat::Djot)).unwrap();
    assert!(
        second_book.contains("Chapter 1"),
        "a new book must restart chapters: {second_book}"
    );
}

// ─────────────────────────────────────────────────────────────────────────
// Epigraphs
// ─────────────────────────────────────────────────────────────────────────

/// Heading, then epigraph, then prose — CMOS §13.36's order, and the order the
/// editor page shows, so what the writer sees is what the export writes.
#[test]
fn an_epigraph_renders_between_the_heading_and_the_prose() {
    let g = book_with_epigraph();
    let p = preset("neutral");
    let out = render_to_string(&req(&g, &[100, 101], &p, ExportFormat::Djot)).unwrap();
    let heading = out.find("Storms").expect("chapter heading");
    let epi = out.find("Salt is the only").expect("epigraph");
    let prose = out.find("The wind rose").expect("prose");
    assert!(
        heading < epi && epi < prose,
        "expected heading < epigraph < prose, got {heading}/{epi}/{prose} in:\n{out}"
    );
}

/// Quoted matter is not the author's word count. An epigraph that moved the total
/// would inflate every pace goal and progress snapshot in the project, silently.
#[test]
fn an_epigraph_adds_no_words_to_the_manuscript() {
    let p = preset("neutral");

    let with = book_with_epigraph();
    let without = {
        let mut g = book_with_epigraph();
        g.binders[0].items[1]
            .contents
            .retain(|c| c.role != ContentRole::EpigraphText);
        g
    };

    let words = |g: &Gathered| {
        assemble(
            &req(g, &[100, 101], &p, ExportFormat::Djot),
            &|_| {},
            &AtomicBool::new(false),
        )
        .unwrap()
        .stats
        .words
    };
    assert_eq!(
        words(&with),
        words(&without),
        "the epigraph must not be counted"
    );
    assert!(words(&with) > 0, "the fixture must count its real prose");
}

/// The preset toggle actually removes it — the clean-submission case.
#[test]
fn the_preset_can_drop_the_epigraph() {
    let g = book_with_epigraph();
    let mut p = preset("neutral");
    assert!(
        p.include_epigraphs,
        "epigraphs ship by default: authored matter is finished-book content"
    );

    p.include_epigraphs = false;
    let out = render_to_string(&req(&g, &[100, 101], &p, ExportFormat::Djot)).unwrap();
    assert!(!out.contains("Salt is the only"), "dropped: {out}");
    assert!(out.contains("The wind rose"), "prose stays: {out}");
}

/// A preset written before the field existed has no `include_epigraphs` key. Plain
/// `#[serde(default)]` would read that absence as `false` and silently strip
/// epigraphs from every custom style while the built-ins kept them — a divergence
/// nobody would think to look for.
#[test]
fn a_preset_saved_before_the_field_existed_still_keeps_epigraphs() {
    // Exactly what one looks like: a real preset serialized, with the key that did
    // not exist yet removed.
    let mut v = serde_json::to_value(preset("neutral")).unwrap();
    let obj = v.as_object_mut().unwrap();
    assert!(
        obj.remove("include_epigraphs").is_some(),
        "the field must be serialized, or this test proves nothing"
    );

    let old: Preset = serde_json::from_value(v).expect("an older preset must still load");
    assert!(
        old.include_epigraphs,
        "absence must read as on, not as a silent opt-out"
    );
}

/// The first line after an epigraph carries no first-line indent (New Hart's Rule),
/// queued through the same mechanism a scene break uses.
#[test]
fn the_paragraph_after_an_epigraph_is_not_indented() {
    let g = book_with_epigraph();
    let mut p = preset("neutral");
    p.first_line_indent_in = 0.5;
    let out = render_to_string(&req(&g, &[100, 101], &p, ExportFormat::Djot)).unwrap();
    let prose = out.find("The wind rose").expect("prose");
    let before = &out[..prose];
    let attrs = before
        .rfind('{')
        .expect("an attribute block before the prose");
    assert!(
        before[attrs..].contains("text_indent=0"),
        "the prose after an epigraph must carry text_indent=0, got: {:?}",
        &before[attrs..]
    );
}

/// Switching the plain-text export to the indented walk changes every `.txt` that
/// contains a blockquote, not only the epigraphs — a quoted letter inside a scene
/// starts arriving indented too. That is the correct reading of a blockquote in a
/// format with no markup, so it is pinned deliberately rather than left to be
/// discovered as a surprise by someone whose manuscript already used one.
#[test]
fn the_plain_text_export_also_indents_a_blockquote_inside_ordinary_prose() {
    let g = gathered(
        vec![iwc(
            300,
            SR::Scene,
            "en",
            vec![c(
                1,
                ContentRole::SceneText,
                "She unfolded it.\n\n> Come at once. Bring the key.\n\nThe hand was her \
                     mother's.",
            )],
        )],
        "en",
    );
    let p = preset("neutral");
    let txt = render_to_string(&req(&g, &[300], &p, ExportFormat::PlainText)).unwrap();

    let quoted = txt
        .lines()
        .find(|l| l.contains("Come at once"))
        .expect("the quoted letter must survive");
    assert!(
        quoted.starts_with(' '),
        "a blockquote in ordinary prose is set in too, got {quoted:?}"
    );
    for flush in ["She unfolded it.", "The hand was her"] {
        assert!(
            txt.lines()
                .any(|l| l.contains(flush) && !l.starts_with(' ')),
            "surrounding prose must stay flush ({flush}): {txt}"
        );
    }
}

/// The end of the chain: an epigraph must reach HTML as marked-up front matter, not
/// as an anonymous blockquote. This is what the whole `semantic_role` path exists for,
/// and it is the only test that exercises compiler → djot → document → writer whole.
#[test]
fn an_epigraph_reaches_html_as_semantic_markup() {
    let g = book_with_epigraph();
    let p = preset("neutral");
    let html = render_to_string(&req(&g, &[100, 101], &p, ExportFormat::Html)).unwrap();
    assert!(
        html.contains(r#"epub:type="epigraph""#),
        "the epigraph must be marked: {html}"
    );
    assert!(
        html.contains(r#"role="doc-epigraph""#),
        "and reachable by assistive technology: {html}"
    );
    assert!(html.contains("Salt is the only"), "with its text: {html}");
}

/// The marker is the compiler's doing, so a scene's own blockquote — a quoted letter,
/// a diary page — must not acquire it.
#[test]
fn a_quotation_inside_scene_prose_is_not_marked_as_an_epigraph() {
    let g = gathered(
        vec![iwc(
            300,
            SR::Scene,
            "en",
            vec![c(
                1,
                ContentRole::SceneText,
                "She unfolded it.\n\n> Come at once.\n\nThe hand was her mother's.",
            )],
        )],
        "en",
    );
    let p = preset("neutral");
    let html = render_to_string(&req(&g, &[300], &p, ExportFormat::Html)).unwrap();
    assert!(html.contains("<blockquote>"), "still a quotation: {html}");
    assert!(!html.contains("epigraph"), "but not an epigraph: {html}");
}

/// Several quotations on one node arrive as one marked blockquote holding them all:
/// this parser folds `>` groups separated by a blank line into a single frame. What
/// matters is that none of the text escapes the marked quote and the marker is not
/// repeated — an epigraph is one piece of front matter however many quotations the
/// author put in it.
#[test]
fn every_quotation_in_an_epigraph_field_is_marked() {
    let g = gathered(
        vec![iwc(
            400,
            SR::ChapterScene,
            "en",
            vec![
                c(1, ContentRole::ChapterTitle, "Two Quotes"),
                c(
                    2,
                    ContentRole::EpigraphText,
                    "> First quotation.\n\n> Second quotation.",
                ),
                c(3, ContentRole::SceneText, "Body."),
            ],
        )],
        "en",
    );
    let p = preset("neutral");
    let html = render_to_string(&req(&g, &[400], &p, ExportFormat::Html)).unwrap();
    assert_eq!(
        html.matches(r#"epub:type="epigraph""#).count(),
        1,
        "one marked quote, not one per quotation: {html}"
    );
    for quote in ["First quotation.", "Second quotation."] {
        assert!(html.contains(quote), "{quote} missing: {html}");
    }
    assert!(
        !html.contains("semantic_role"),
        "the marker must be consumed, never rendered as text: {html}"
    );
}

/// A Part or a Book has no prose of its own, so its epigraph must NOT queue an
/// indent reset: the queue would outlive the row and land on the next row's opening
/// paragraph, which is a different paragraph and is entitled to its indent. The
/// binder is organisational, so nothing guarantees a heading row comes between them
/// to clear it.
#[test]
fn a_proseless_rows_epigraph_does_not_suppress_the_next_rows_indent() {
    let g = gathered(
        vec![
            iwc(
                200,
                SR::Part,
                "en",
                vec![
                    c(1, ContentRole::PartTitle, "Part One"),
                    c(2, ContentRole::EpigraphText, "> A part-level epigraph."),
                ],
            ),
            // Deliberately a bare Scene, not a ChapterScene: no structural heading
            // follows, so nothing clears a leaked queue.
            iwc(
                201,
                SR::Scene,
                "en",
                vec![c(
                    3,
                    ContentRole::SceneText,
                    "The wind rose over the hills.",
                )],
            ),
        ],
        "en",
    );
    let mut p = preset("neutral");
    p.first_line_indent_in = 0.5;
    let out = render_to_string(&req(&g, &[200, 201], &p, ExportFormat::Djot)).unwrap();

    let prose = out.find("The wind rose").expect("prose");
    let before = &out[..prose];
    let leaked = before
        .rfind('{')
        .is_some_and(|a| before[a..].contains("text_indent=0"));
    assert!(
        !leaked,
        "the Part's epigraph must not reach the next row's paragraph: {out}"
    );
}

/// An RTL row's epigraph must carry the direction attribute like any other prose on
/// that row — the bug that appears the moment the epigraph is pushed straight into
/// the buffer instead of through `push_prose`.
#[test]
fn an_epigraph_inherits_its_rows_direction() {
    let mut g = book_with_epigraph();
    g.binders[0].items[1].item.dict_language = language::parse_legacy_list("he");
    g.work.dict_language = language::parse_legacy_list("he");
    let p = preset("neutral");
    let out = render_to_string(&req(&g, &[100, 101], &p, ExportFormat::Djot)).unwrap();
    let epi = out.find("Salt is the only").expect("epigraph");
    let before = &out[..epi];
    // The nearest `{` is the semantic marker, which sits inside the quote directly
    // above its text; the direction attribute is emitted by `push_prose` for the
    // block as a whole, so look for it across everything preceding the quotation.
    assert!(
        before.contains("direction=rtl"),
        "an RTL row's epigraph must be marked rtl, got: {before:?}"
    );
}

/// `.txt` has no markup for quoted matter, so the epigraph must arrive indented —
/// the whole reason the plain-text export uses the indented walk.
#[test]
fn the_plain_text_export_sets_the_epigraph_in() {
    let g = book_with_epigraph();
    let p = preset("neutral");
    let txt = render_to_string(&req(&g, &[100, 101], &p, ExportFormat::PlainText)).unwrap();
    let line = txt
        .lines()
        .find(|l| l.contains("Salt is the only"))
        .expect("the epigraph must be in the plain text");
    assert!(
        line.starts_with(' '),
        "the epigraph line must be indented, got {line:?}"
    );
    assert!(
        txt.lines()
            .any(|l| l.contains("The wind rose") && !l.starts_with(' ')),
        "the body prose must stay flush: {txt}"
    );
}
// ── Footnotes ──────────────────────────────────────────────────

fn note(id: u64, content: u64, label: &str, body: &str) -> skrib_format::FootnoteWithContent {
    skrib_format::FootnoteWithContent {
        footnote: common::entities::Footnote {
            id,
            content: Some(content),
            label: label.into(),
            body: body.into(),
            ..Default::default()
        },
    }
}

/// A note's body must reach the compiled document, or every writer renders a
/// marker pointing at text that is not there.
#[test]
fn a_notes_body_is_compiled_into_the_document() {
    let mut g = gathered(
        vec![iwc(
            100,
            SR::Scene,
            "",
            vec![c(1, ContentRole::SceneText, "Prose[^n1] here.")],
        )],
        "en",
    );
    g.footnotes = vec![note(7000, 1, "n1", "The note body.")];

    let p = preset("neutral");
    let out = render_to_string(&req(&g, &[100], &p, ExportFormat::Djot)).unwrap();
    assert!(out.contains("[^n1]"), "reference lost: {out}");
    assert!(out.contains("[^n1]:"), "definition never emitted: {out}");
    assert!(out.contains("The note body"), "body lost: {out}");
}

/// `include_footnotes` off drops the bodies.
#[test]
fn a_preset_can_leave_the_notes_out() {
    let mut g = gathered(
        vec![iwc(
            100,
            SR::Scene,
            "",
            vec![c(1, ContentRole::SceneText, "Prose[^n1] here.")],
        )],
        "en",
    );
    g.footnotes = vec![note(7000, 1, "n1", "UNIQUEBODY.")];

    let mut p = preset("neutral");
    p.include_footnotes = false;
    let out = render_to_string(&req(&g, &[100], &p, ExportFormat::Djot)).unwrap();
    assert!(!out.contains("UNIQUEBODY"), "the body survived: {out}");
}

/// **The regression class `ef2a98a0` fixed for chapters, for notes.** Exporting
/// one chapter must number its notes as the whole book numbers them — the
/// number a reader would find, and the number the editor's badge shows.
#[test]
fn a_scoped_export_numbers_its_notes_as_the_book_does() {
    let mut g = gathered(
        vec![
            iwc(
                100,
                SR::Scene,
                "",
                vec![c(1, ContentRole::SceneText, "First[^a].")],
            ),
            iwc(
                101,
                SR::Scene,
                "",
                vec![c(2, ContentRole::SceneText, "Second[^b].")],
            ),
        ],
        "en",
    );
    g.footnotes = vec![note(7000, 1, "a", "One."), note(7001, 2, "b", "Two.")];

    let p = preset("neutral");
    // The second scene alone. Its note is the book's second note.
    let out = render_to_string(&req(&g, &[101], &p, ExportFormat::Html)).unwrap();
    assert!(
        out.contains("<sup>2</sup>"),
        "a scoped export renumbered from one: {out}"
    );
    assert!(
        !out.contains("<sup>1</sup>"),
        "the first note leaked into a scope that excludes it: {out}"
    );
}

/// **The scope-drift regression `label_homes` closes.** A duplicated scene
/// (a raw text copy that never remints its footnote labels) can leave the
/// same label referenced from two different items. Exporting only the
/// *later* one — item 100, the label's true first home in the whole
/// manuscript, is deliberately left out of scope — must still print the
/// number the whole-manuscript collapse gives it, not a scope-local
/// recount starting from whichever occurrence the export happens to see
/// first.
#[test]
fn a_label_duplicated_across_two_items_keeps_its_whole_manuscript_number_when_scoped() {
    let mut g = gathered(
        vec![
            iwc(
                100,
                SR::Scene,
                "",
                vec![c(1, ContentRole::SceneText, "First[^a].")],
            ),
            iwc(
                150,
                SR::Scene,
                "",
                vec![c(2, ContentRole::SceneText, "Also[^a] here.")],
            ),
            iwc(
                200,
                SR::Scene,
                "",
                vec![c(3, ContentRole::SceneText, "Second[^b].")],
            ),
        ],
        "en",
    );
    g.footnotes = vec![note(7000, 1, "a", "One."), note(7001, 3, "b", "Two.")];

    let p = preset("neutral");
    let out = render_to_string(&req(&g, &[150, 200], &p, ExportFormat::Html)).unwrap();
    assert!(
        out.contains("<sup>1</sup>"),
        "the scoped export must print the same number 1 the editor's whole-manuscript \
             badge shows for [^a], not a scope-local recount: {out}"
    );
    assert!(
        !out.contains("<sup>2</sup>"),
        "no citation in this scope may be forced to number 2 by a scope-local collapse: {out}"
    );
    assert!(
        out.contains("<sup>3</sup>"),
        "the unambiguous note lost its number: {out}"
    );
}

/// A note whose reference sits outside the exported scope is not printed: there
/// would be nothing pointing at it.
#[test]
fn an_unreferenced_note_is_not_printed_in_a_scoped_export() {
    let mut g = gathered(
        vec![
            iwc(
                100,
                SR::Scene,
                "",
                vec![c(1, ContentRole::SceneText, "First[^a].")],
            ),
            iwc(
                101,
                SR::Scene,
                "",
                vec![c(2, ContentRole::SceneText, "Second.")],
            ),
        ],
        "en",
    );
    g.footnotes = vec![note(7000, 1, "a", "ONLYINSCENEONE.")];

    let p = preset("neutral");
    let out = render_to_string(&req(&g, &[101], &p, ExportFormat::Djot)).unwrap();
    assert!(
        !out.contains("ONLYINSCENEONE"),
        "a note nothing in this export references was printed: {out}"
    );
}
