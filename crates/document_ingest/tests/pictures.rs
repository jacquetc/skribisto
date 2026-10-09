// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The pictures an OpenDocument file and a Markdown file bring, read from files built here.
//! (A Word file's are pinned in `docx_references_and_structure.rs`, beside the notes and
//! comments whose places a picture decides.)
//!
//! Every picture is made in the test with the `image` crate's encoders. What is checked is
//! what the editor will read: every row's Djot goes through `text-document`, and a picture
//! has to arrive as an image fragment naming the file the project will hold.

use std::io::Write;
use std::path::Path;

use document_ingest::pictures::PictureRefusal;
use document_ingest::{ImportDiagnostic, ScannerRegistry, SourceBlock, SourceDocument};
use text_document::{FragmentContent, TextDocument};

fn picture_bytes(format: image::ImageFormat, width: u32, height: u32, colour: [u8; 4]) -> Vec<u8> {
    let picture = image::DynamicImage::ImageRgba8(image::RgbaImage::from_pixel(
        width,
        height,
        image::Rgba(colour),
    ));
    let mut out = std::io::Cursor::new(Vec::new());
    picture.write_to(&mut out, format).expect("encode");
    out.into_inner()
}

fn png(width: u32, height: u32) -> Vec<u8> {
    picture_bytes(image::ImageFormat::Png, width, height, [30, 60, 90, 255])
}

/// Two frames, red then blue.
fn animated_gif() -> Vec<u8> {
    let mut out = Vec::new();
    {
        let mut encoder = image::codecs::gif::GifEncoder::new(&mut out);
        let frames = [[255, 0, 0, 255], [0, 0, 255, 255]].map(|colour| {
            image::Frame::new(image::RgbaImage::from_pixel(5, 3, image::Rgba(colour)))
        });
        encoder.encode_frames(frames).expect("encode");
    }
    out
}

const SVG: &[u8] =
    b"<?xml version=\"1.0\"?><svg xmlns=\"http://www.w3.org/2000/svg\" width=\"4\" height=\"4\"/>";

/// An `.odt` whose text is `body`, holding `members` beside its content.
fn odt(body: &str, members: &[(&str, &[u8])]) -> Vec<u8> {
    let content = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<office:document-content
  xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0"
  xmlns:style="urn:oasis:names:tc:opendocument:xmlns:style:1.0"
  xmlns:text="urn:oasis:names:tc:opendocument:xmlns:text:1.0"
  xmlns:draw="urn:oasis:names:tc:opendocument:xmlns:drawing:1.0"
  xmlns:svg="urn:oasis:names:tc:opendocument:xmlns:svg-compatible:1.0"
  xmlns:xlink="http://www.w3.org/1999/xlink"
  xmlns:dc="http://purl.org/dc/elements/1.1/"
  office:version="1.3">
  <office:body>
    <office:text>
{body}
    </office:text>
  </office:body>
</office:document-content>"#
    );
    let mut zip = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
    let stored =
        zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    zip.start_file("mimetype", stored).expect("mimetype");
    zip.write_all(b"application/vnd.oasis.opendocument.text")
        .expect("mimetype");
    zip.start_file("content.xml", stored).expect("content");
    zip.write_all(content.as_bytes()).expect("content");
    for (name, bytes) in members {
        zip.start_file(*name, stored).expect("member");
        zip.write_all(bytes).expect("member");
    }
    zip.finish().expect("finish").into_inner()
}

/// A picture frame as LibreOffice writes one, shown at `width` × `height` inches.
fn frame(images: &[&str], title: &str, width: &str, height: &str) -> String {
    let images: String = images
        .iter()
        .map(|href| {
            format!(
                r#"<draw:image xlink:href="{href}" xlink:type="simple" xlink:show="embed" xlink:actuate="onLoad"/>"#
            )
        })
        .collect();
    let title = if title.is_empty() {
        String::new()
    } else {
        format!("<svg:title>{title}</svg:title>")
    };
    format!(
        r#"<draw:frame draw:name="Image1" text:anchor-type="as-char" svg:width="{width}" svg:height="{height}">{images}{title}</draw:frame>"#
    )
}

fn scan(name: &str, bytes: &[u8]) -> SourceDocument {
    ScannerRegistry::with_builtin_scanners().scan_bytes(Path::new(name), bytes)
}

/// The one prose block of a document.
fn prose(doc: &SourceDocument) -> &str {
    let prose: Vec<&str> = doc
        .blocks
        .iter()
        .filter_map(|b| match b {
            SourceBlock::Prose { djot, .. } => Some(djot.as_str()),
            _ => None,
        })
        .collect();
    assert_eq!(prose.len(), 1, "one prose block: {:#?}", doc.blocks);
    prose[0]
}

/// Every picture the editor reads in `djot`: its name, its description, its size.
fn pictures_read(djot: &str) -> Vec<(String, String, u32, u32)> {
    let doc = TextDocument::new();
    doc.set_djot(djot)
        .and_then(|op| {
            op.wait_timeout(std::time::Duration::from_secs(30))
                .expect("the text-document operation did not finish within 30 s")
        })
        .expect("the stored Djot parses");
    doc.blocks()
        .iter()
        .flat_map(|b| b.fragments())
        .filter_map(|f| match f {
            FragmentContent::Image {
                name,
                alt,
                width,
                height,
                ..
            } => Some((name, alt, width, height)),
            _ => None,
        })
        .collect()
}

/// The text the editor reads, a picture one character.
fn text_of(djot: &str) -> String {
    skrib_format::djot_plain_text(djot).expect("parses").0
}

fn refusals(doc: &SourceDocument) -> Vec<(String, PictureRefusal)> {
    doc.diagnostics
        .iter()
        .filter_map(|d| match d {
            ImportDiagnostic::ImageNotIngested { target, reason, .. } => {
                Some((target.clone(), *reason))
            }
            _ => None,
        })
        .collect()
}

fn asset(bytes: &[u8]) -> String {
    format!("assets/{}.png", blake3::hash(bytes).to_hex())
}

// ── OpenDocument ────────────────────────────────────────────────────────────────────

/// A picture LibreOffice stores comes with the document, at the size its frame shows it and
/// with the description in the frame's title. 2.0833 by 1.0417 inches is 200 by 100 pixels.
#[test]
fn a_picture_stored_in_an_odt_comes_with_it() {
    let map = png(40, 20);
    let body = format!(
        "<text:p>The map {} was old.</text:p>",
        frame(
            &["Pictures/1000.png"],
            "The harbour, 1890",
            "2.0833in",
            "1.0417in"
        )
    );
    let doc = scan("chapter.odt", &odt(&body, &[("Pictures/1000.png", &map)]));
    let djot = prose(&doc);
    assert_eq!(text_of(djot), "The map \u{FFFC} was old.");
    assert_eq!(
        pictures_read(djot),
        vec![(asset(&map), "The harbour, 1890".to_string(), 200, 100)]
    );
    assert_eq!(doc.pictures.len(), 1);
    assert_eq!(doc.pictures[0].bytes, map);
    assert_eq!(doc.pictures[0].file_name, "1000.png");
    assert!(refusals(&doc).is_empty(), "{:?}", doc.diagnostics);
}

/// LibreOffice keeps an SVG drawing with a PNG beside it for readers that cannot draw SVG.
/// The editor is one, so the PNG is the picture, and nothing was lost to report.
#[test]
fn an_svg_frame_arrives_as_the_png_beside_it() {
    let fallback = png(6, 6);
    let body = format!(
        "<text:p>A plan: {}</text:p>",
        frame(&["Pictures/plan.svg", "Pictures/plan.png"], "", "", "")
    );
    let doc = scan(
        "chapter.odt",
        &odt(
            &body,
            &[("Pictures/plan.svg", SVG), ("Pictures/plan.png", &fallback)],
        ),
    );
    let djot = prose(&doc);
    assert_eq!(
        pictures_read(djot),
        vec![(asset(&fallback), String::new(), 6, 6)],
        "no size in the frame: the picture's own"
    );
    assert!(refusals(&doc).is_empty(), "{:?}", doc.diagnostics);
}

/// A picture that cannot come leaves nothing in the text, and a comment after it keeps its
/// words: the text counts no character for it. Each is said once.
#[test]
fn a_picture_left_out_of_an_odt_moves_no_comment() {
    let body = format!(
        r#"<text:p>See {}{}{}{}the <office:annotation office:name="a0"><dc:creator>Editor</dc:creator><text:p>Which one?</text:p></office:annotation>figure<office:annotation-end office:name="a0"/>.{}</text:p>"#,
        frame(&["Pictures/plan.svg"], "", "", ""),
        frame(&["https://example.com/figure.png"], "", "", ""),
        frame(&["../elsewhere/figure.png"], "", "", ""),
        frame(&["Pictures/gone.png"], "", "", ""),
        frame(&["Pictures/plan.svg"], "", "", ""),
    );
    let bytes = odt(&body, &[("Pictures/plan.svg", SVG)]);
    let plan = document_ingest::scan_and_plan(
        &ScannerRegistry::with_builtin_scanners(),
        &[(Path::new("chapter.odt").to_path_buf(), bytes.clone())],
        skribisto_model::CreateType::Chapter,
        common::entities::ChapterMode::Folder,
        0,
    );
    let row = &plan.rows[0];
    let text = text_of(&row.djot);
    assert_eq!(text, "See the figure.");
    let comment = &row.comments[0];
    let words: String = text
        .chars()
        .skip(comment.anchor.start)
        .take(comment.anchor.length)
        .collect();
    assert_eq!(words, "figure", "the comment keeps its word");
    assert!(!row.djot.contains("!["), "{}", row.djot);
    assert!(!row.djot.contains("elsewhere"), "{}", row.djot);

    assert_eq!(
        refusals(&scan("chapter.odt", &bytes)),
        vec![
            (
                "Pictures/plan.svg".to_string(),
                PictureRefusal::Unsupported { format: "SVG" }
            ),
            (
                "https://example.com/figure.png".to_string(),
                PictureRefusal::Remote
            ),
            (
                "../elsewhere/figure.png".to_string(),
                PictureRefusal::Outside
            ),
            ("Pictures/gone.png".to_string(), PictureRefusal::Missing),
        ]
    );
}

#[test]
fn an_animated_picture_in_an_odt_arrives_as_its_first_frame() {
    let body = format!(
        "<text:p>{}</text:p>",
        frame(&["Pictures/spin.gif"], "", "", "")
    );
    let doc = scan(
        "chapter.odt",
        &odt(&body, &[("Pictures/spin.gif", &animated_gif())]),
    );
    assert_eq!(doc.pictures.len(), 1);
    let picture = &doc.pictures[0];
    assert_eq!(picture.mime_type, "image/png");
    assert_eq!(picture.file_name, "spin.png");
    let first = image::load_from_memory(&picture.bytes).expect("a PNG");
    assert_eq!(first.to_rgba8().get_pixel(0, 0).0, [255, 0, 0, 255]);
    assert!(
        doc.diagnostics.iter().any(|d| matches!(
            d,
            ImportDiagnostic::PictureFirstFrameOnly { target, .. } if target == "Pictures/spin.gif"
        )),
        "{:?}",
        doc.diagnostics
    );
    assert_eq!(
        pictures_read(prose(&doc)),
        vec![(
            format!("assets/{}.png", picture.content_hash),
            String::new(),
            5,
            3
        )]
    );
}

// ── Markdown ────────────────────────────────────────────────────────────────────────

/// A Markdown document written into `folder`, scanned the way the importer scans a file it
/// read from disk.
fn scan_markdown(folder: &Path, markdown: &str) -> SourceDocument {
    let path = folder.join("chapter.md");
    std::fs::write(&path, markdown).expect("document");
    let bytes = std::fs::read(&path).expect("read back");
    ScannerRegistry::with_builtin_scanners().scan_file(&path, &bytes)
}

/// A picture beside a Markdown file comes with it, at its own size since Markdown states
/// none, with its description kept word for word, punctuation included. The same file
/// named two ways is one picture.
#[test]
fn a_picture_beside_a_markdown_file_comes_with_it() {
    let folder = tempfile::tempdir().expect("scratch");
    std::fs::create_dir(folder.path().join("images")).expect("images");
    let map = png(30, 12);
    std::fs::write(folder.path().join("images").join("old map.png"), &map).expect("picture");
    let description = "Anna's map [draft] *v2* at 10:30";
    let doc = scan_markdown(
        folder.path(),
        &format!(
            "The map ![{}](images/old%20map.png) was old, ![again](./images/old%20map.png) and older.\n",
            "Anna's map \\[draft\\] \\*v2\\* at 10:30"
        ),
    );
    let djot = prose(&doc);
    assert_eq!(
        pictures_read(djot),
        vec![
            (asset(&map), description.to_string(), 30, 12),
            (asset(&map), "again".to_string(), 30, 12),
        ],
        "{djot}"
    );
    assert_eq!(
        text_of(djot),
        "The map \u{FFFC} was old, \u{FFFC} and older."
    );
    assert_eq!(doc.pictures.len(), 1, "one file, one picture");
    assert_eq!(doc.pictures[0].file_name, "old map.png");
    assert!(refusals(&doc).is_empty(), "{:?}", doc.diagnostics);
}

/// Only files under the document's own folder are read. A path climbing out, an absolute
/// path, an address on the web: each is refused before anything is opened, said once, and
/// leaves nothing in the text, so no path from the writer's disk reaches the project.
#[test]
fn a_markdown_file_cannot_reach_outside_its_folder() {
    let outer = tempfile::tempdir().expect("scratch");
    let folder = outer.path().join("book");
    std::fs::create_dir(&folder).expect("book");
    let secret = outer.path().join("secret.png");
    std::fs::write(&secret, png(2, 2)).expect("a picture outside");
    let absolute = secret.to_string_lossy().replace('\\', "/");
    let doc = scan_markdown(
        &folder,
        &format!(
            "One ![a](../secret.png) two ![b]({absolute}) three ![c](https://example.com/x.png) four ![d](gone.png) five ![e](../secret.png).\n"
        ),
    );
    let djot = prose(&doc);
    assert_eq!(text_of(djot), "One  two  three  four  five .");
    assert!(doc.pictures.is_empty());
    assert!(!djot.contains("secret"), "{djot}");
    assert_eq!(
        refusals(&doc),
        vec![
            ("../secret.png".to_string(), PictureRefusal::Outside),
            (absolute, PictureRefusal::Outside),
            (
                "https://example.com/x.png".to_string(),
                PictureRefusal::Remote
            ),
            ("gone.png".to_string(), PictureRefusal::Missing),
        ]
    );
}

/// A symbolic link is never followed, to a file or to a folder: either could lead anywhere
/// on the disk, and the rule is the document's own folder.
#[cfg(unix)]
#[test]
fn a_markdown_file_cannot_reach_through_a_link() {
    let outer = tempfile::tempdir().expect("scratch");
    let folder = outer.path().join("book");
    let private = outer.path().join("private");
    std::fs::create_dir(&folder).expect("book");
    std::fs::create_dir(&private).expect("private");
    std::fs::write(private.join("photo.png"), png(2, 2)).expect("private picture");
    std::os::unix::fs::symlink(private.join("photo.png"), folder.join("photo.png"))
        .expect("a link to a file");
    std::os::unix::fs::symlink(&private, folder.join("album")).expect("a link to a folder");
    let doc = scan_markdown(&folder, "A ![x](photo.png) and ![y](album/photo.png).\n");
    assert!(doc.pictures.is_empty());
    assert_eq!(
        refusals(&doc),
        vec![
            ("photo.png".to_string(), PictureRefusal::Outside),
            ("album/photo.png".to_string(), PictureRefusal::Outside),
        ]
    );
}

/// A file with no folder to stand on (bytes handed over alone) looks for nothing on disk:
/// its pictures are missing, not looked up wherever the app was started.
#[test]
fn a_markdown_file_with_no_folder_reads_nothing_beside_it() {
    let doc = scan("chapter.md", b"A ![x](images/x.png).\n");
    assert_eq!(
        refusals(&doc),
        vec![("images/x.png".to_string(), PictureRefusal::Missing)]
    );
}

/// A picture written into the document itself, as a `data:` address, needs no folder.
#[test]
fn a_picture_written_into_a_markdown_file_comes_with_it() {
    let bytes = png(3, 5);
    let encoded = base64(&bytes);
    let doc = scan(
        "chapter.md",
        format!("Inline ![dot](data:image/png;base64,{encoded}) here.\n").as_bytes(),
    );
    assert_eq!(
        pictures_read(prose(&doc)),
        vec![(asset(&bytes), "dot".to_string(), 3, 5)]
    );
}

/// A heading becomes a title: it keeps its picture's description as words, as it always
/// has, and the picture is named. Nothing a title does not hold is stored for it.
#[test]
fn a_picture_in_a_markdown_heading_is_named() {
    let folder = tempfile::tempdir().expect("scratch");
    std::fs::write(folder.path().join("crest.png"), png(2, 2)).expect("picture");
    let doc = scan_markdown(
        folder.path(),
        "# The ![crest](crest.png) Harbour\n\nText.\n",
    );
    assert_eq!(
        refusals(&doc),
        vec![("crest.png".to_string(), PictureRefusal::InHeading)]
    );
    assert!(doc.pictures.is_empty(), "never read");
    assert!(document_ingest::pictures::referenced_by(&doc).is_empty());
}

/// What a document's rows will need from the project: every picture its stored text names,
/// once, and nothing its headings showed.
#[test]
fn a_document_names_the_pictures_its_text_holds() {
    let folder = tempfile::tempdir().expect("scratch");
    let first = png(2, 2);
    let second = png(3, 3);
    std::fs::write(folder.path().join("a.png"), &first).expect("picture");
    std::fs::write(folder.path().join("b.png"), &second).expect("picture");
    let doc = scan_markdown(
        folder.path(),
        "![](a.png)\n\n# Two\n\n![](b.png) and ![](a.png)\n",
    );
    assert_eq!(
        document_ingest::pictures::referenced_by(&doc),
        vec![asset(&first), asset(&second)]
    );
}

fn base64(bytes: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in bytes.chunks(3) {
        let n = chunk
            .iter()
            .enumerate()
            .fold(0u32, |acc, (i, b)| acc | (u32::from(*b) << (16 - 8 * i)));
        for i in 0..4 {
            if i <= chunk.len() {
                out.push(char::from(ALPHABET[((n >> (18 - 6 * i)) & 63) as usize]));
            } else {
                out.push('=');
            }
        }
    }
    out
}

// ── What stays behind, and what it leaves ───────────────────────────────────────────────

/// A flat OpenDocument file whose text is `body`: its pictures are written inside it.
fn fodt(body: &str) -> Vec<u8> {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<office:document
  xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0"
  xmlns:text="urn:oasis:names:tc:opendocument:xmlns:text:1.0"
  xmlns:draw="urn:oasis:names:tc:opendocument:xmlns:drawing:1.0"
  xmlns:svg="urn:oasis:names:tc:opendocument:xmlns:svg-compatible:1.0"
  xmlns:xlink="http://www.w3.org/1999/xlink"
  xmlns:dc="http://purl.org/dc/elements/1.1/"
  office:mimetype="application/vnd.oasis.opendocument.text">
  <office:body><office:text>{body}</office:text></office:body>
</office:document>"#
    )
    .into_bytes()
}

/// `picture` inside a footnote of a paragraph, and inside a comment on the next one.
fn in_a_note_and_a_comment(picture: &str) -> String {
    format!(
        concat!(
            r#"<text:p>Text<text:note text:id="n1" text:note-class="footnote">"#,
            r#"<text:note-citation>1</text:note-citation><text:note-body>"#,
            r#"<text:p>Note {p} end.</text:p></text:note-body></text:note> after.</text:p>"#,
            r#"<text:p><office:annotation><dc:creator>A</dc:creator>"#,
            r#"<text:p>Comment {p} end.</text:p></office:annotation>Commented.</text:p>"#,
        ),
        p = picture
    )
}

/// A note and a comment keep their words and no picture, so a picture in one is named with
/// that reason. It used to vanish without a word: its description stayed behind as words of
/// the note, and in a flat file, whose pictures are written inside it, the note and the
/// comment took every character of the picture's base64 as their text.
#[test]
fn a_picture_in_an_odt_note_or_comment_is_named_and_leaves_no_text() {
    let map = png(4, 4);
    let framed = frame(&["Pictures/1000.png"], "The harbour, 1890", "1in", "1in");
    let doc = scan(
        "notes.odt",
        &odt(
            &in_a_note_and_a_comment(&framed),
            &[("Pictures/1000.png", &map)],
        ),
    );
    assert_eq!(doc.footnotes.len(), 1);
    assert_eq!(text_of(&doc.footnotes[0].body), "Note end.");
    assert_eq!(doc.annotations.len(), 1);
    assert_eq!(text_of(&doc.annotations[0].body), "Comment end.");
    assert_eq!(
        refusals(&doc),
        vec![("Pictures/1000.png".to_string(), PictureRefusal::InNote)],
        "named once, with the reason, however many notes show it"
    );
    assert!(doc.pictures.is_empty());

    let written_in = format!(
        r#"<draw:frame svg:width="1in" svg:height="1in"><draw:image><office:binary-data>{}</office:binary-data></draw:image></draw:frame>"#,
        base64(&map)
    );
    let doc = scan("notes.fodt", &fodt(&in_a_note_and_a_comment(&written_in)));
    assert_eq!(text_of(&doc.footnotes[0].body), "Note end.");
    assert_eq!(text_of(&doc.annotations[0].body), "Comment end.");
    assert_eq!(
        refusals(&doc),
        vec![
            ("picture".to_string(), PictureRefusal::InNote),
            ("picture".to_string(), PictureRefusal::InNote),
        ]
    );
}

/// A picture that cannot come is taken out of the text, and the text around it keeps its
/// shape. Taken out alone, a picture on a line of its own inside a paragraph left a blank
/// line there, and the paragraph arrived as two; a picture opening a line left what followed
/// opening it instead, so a caption became a list item.
#[test]
fn a_picture_that_cannot_come_leaves_its_paragraph_as_it_was() {
    let folder = tempfile::tempdir().expect("scratch");
    for (markdown, expected) in [
        (
            "Here is the map:\n![The map](gone.png)\nAs you can see, it is old.\n",
            "Here is the map: As you can see, it is old.",
        ),
        ("![](gone.png) - the harbour, 1890\n", "- the harbour, 1890"),
        ("![](gone.png) 1. Arrival\n", "1. Arrival"),
        ("![](gone.png) # of ships\n", "# of ships"),
        ("Before ![](gone.png) after.\n", "Before  after."),
    ] {
        let doc = scan_markdown(folder.path(), markdown);
        let djot = prose(&doc);
        assert_eq!(text_of(djot), expected, "{markdown:?} became {djot:?}");
        let paragraphs = TextDocument::new();
        paragraphs
            .set_djot(djot)
            .and_then(|op| {
                op.wait_timeout(std::time::Duration::from_secs(30))
                    .expect("the text-document operation did not finish within 30 s")
            })
            .expect("the stored Djot parses");
        assert_eq!(
            paragraphs.blocks().len(),
            1,
            "{markdown:?}: one paragraph, as it was: {djot:?}"
        );
        assert_eq!(
            refusals(&doc),
            vec![("gone.png".to_string(), PictureRefusal::Missing)]
        );
    }
}

/// A picture a Markdown file writes as HTML is dropped with the rest of the HTML, and named
/// with that reason; the HTML block holding it is reported too. Both went without a word.
#[test]
fn a_picture_written_as_html_is_named() {
    let folder = tempfile::tempdir().expect("scratch");
    std::fs::write(folder.path().join("map.png"), png(2, 2)).expect("picture");
    let doc = scan_markdown(
        folder.path(),
        "Text <img alt=\"a map\" src=\"map.png\"> more.\n\n<p>\n<img\n  src='../private/photo.jpg'>\n</p>\n\n<IMG SRC=map.png>\n",
    );
    assert_eq!(
        refusals(&doc),
        vec![
            ("map.png".to_string(), PictureRefusal::InHtml),
            ("../private/photo.jpg".to_string(), PictureRefusal::InHtml),
        ],
        "each named once, a tag spread over lines too"
    );
    assert!(
        doc.diagnostics
            .iter()
            .any(|d| matches!(d, ImportDiagnostic::RawHtmlDropped { count: 2, .. })),
        "{:?}",
        doc.diagnostics
    );
    assert!(doc.pictures.is_empty(), "nothing read for them");
}

/// A `data:` address carries the picture in it. One that cannot come is named by its type,
/// not by the address: the whole address, megabytes of it, reached the review's line.
#[test]
fn a_refused_picture_written_into_the_file_is_named_by_its_type() {
    let folder = tempfile::tempdir().expect("scratch");
    let metafile = base64(&[1u8; 3000]);
    let doc = scan_markdown(
        folder.path(),
        &format!("A drawing ![](data:image/x-emf;base64,{metafile}) here.\n"),
    );
    assert_eq!(
        refusals(&doc),
        vec![(
            "data:image/x-emf".to_string(),
            PictureRefusal::Unsupported { format: "unknown" }
        )]
    );
}

/// A folder of the picture's path swapped for a link to another while the picture is read.
/// Each name was checked, then the path looked up and opened again, so a swap in between was
/// followed by the look-up and the open alike, and the file outside came in: measured, a few
/// reads in a million. What the kernel says was opened is held to the path checked.
#[cfg(target_os = "linux")]
#[test]
fn a_folder_swapped_for_a_link_mid_read_never_lets_the_outside_in() {
    use document_ingest::pictures::DocumentFolder;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};

    let scratch = tempfile::tempdir().expect("scratch");
    let root = scratch.path().join("book");
    let outside = scratch.path().join("private");
    std::fs::create_dir_all(root.join("real")).expect("folder");
    std::fs::create_dir_all(&outside).expect("outside");
    std::fs::write(root.join("real").join("map.png"), b"inside").expect("inside");
    std::fs::write(outside.join("map.png"), b"outside").expect("outside");
    std::os::unix::fs::symlink(&outside, root.join("link")).expect("link");
    std::fs::rename(root.join("real"), root.join("images")).expect("images");
    let folder = DocumentFolder::of_document(&root.join("chapter.md")).expect("folder");

    // `images` turns from the real folder into the link and back, as fast as it can.
    let stop = Arc::new(AtomicBool::new(false));
    let swapper = {
        let root = root.clone();
        let stop = stop.clone();
        std::thread::spawn(move || {
            let mut real = true;
            while !stop.load(Ordering::Relaxed) {
                let (away, back) = if real {
                    ("real", "link")
                } else {
                    ("link", "real")
                };
                let _ = std::fs::rename(root.join("images"), root.join(away));
                let _ = std::fs::rename(root.join(back), root.join("images"));
                real = !real;
            }
        })
    };
    let segments = ["images".to_string(), "map.png".to_string()];
    let started = std::time::Instant::now();
    let mut reads = 0usize;
    let mut outside_read = 0usize;
    while started.elapsed() < std::time::Duration::from_secs(2) {
        reads += 1;
        if folder
            .read(&segments)
            .is_ok_and(|bytes| bytes == b"outside")
        {
            outside_read += 1;
        }
    }
    stop.store(true, Ordering::Relaxed);
    let _ = swapper.join();
    assert_eq!(
        outside_read, 0,
        "the outside file came in, in {reads} reads"
    );
}

/// A group holds frames, not pictures of its own: LibreOffice groups a photograph with the
/// shape of its caption, say. Its picture was looked for among the group's own images, found
/// none, and vanished without a word; it comes as the picture of any frame does.
#[test]
fn a_picture_in_an_odt_group_comes_with_it() {
    let map = png(40, 20);
    let body = format!(
        "<text:p>Before <draw:g>{}<draw:custom-shape/></draw:g> after.</text:p>",
        frame(
            &["Pictures/1000.png"],
            "The harbour, 1890",
            "2.0833in",
            "1.0417in"
        )
    );
    let doc = scan("grouped.odt", &odt(&body, &[("Pictures/1000.png", &map)]));
    let djot = prose(&doc);
    assert_eq!(text_of(djot), "Before \u{FFFC} after.");
    assert_eq!(
        pictures_read(djot),
        vec![(asset(&map), "The harbour, 1890".to_string(), 200, 100)]
    );
    assert!(refusals(&doc).is_empty(), "{:?}", doc.diagnostics);
}
