// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Word documents `docx-rs` would never finish reading, and the same documents whole.
//!
//! `docx-rs` 0.4.22 reads a part through an event reader that answers every call past
//! the end of its input with one more `EndDocument`, and most of its readers are loops
//! that leave only at their own element's end tag. A part cut short, one with a syntax
//! error or an ill-formed tag some reader reads past, or a style sheet or relationships
//! part whose root is not the element its loop waits for, left the import spinning on
//! one thread at full speed, for good, with no cancel reaching it. Each such document is
//! now refused before `docx-rs` sees it, as a file that could not be read, naming the
//! part; and each is paired here with the same document whole, which is read as before.
//!
//! Every scan runs on a thread of its own and is waited for a bounded time, so a
//! regression fails the test instead of hanging the suite.

use std::io::Write;
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

use document_ingest::{ImportDiagnostic, ScannerRegistry, SourceBlock, SourceDocument};

/// How long a scan may take before it is taken to be reading for ever. Each of these
/// documents is read in milliseconds; the margin is for a loaded machine.
const PATIENCE: Duration = Duration::from_secs(30);

/// Scan every document at once, each as the `.docx` file `novel.docx` on a thread of its
/// own, and wait for all of them together for [`PATIENCE`]: the shape's name and its
/// document, or why there is none.
///
/// A scan that never returns is left running on its thread: there is no way to stop it
/// from outside, which is the defect itself. It ends with the test binary. Waiting for
/// all of them together is what lets one run name every shape that hangs.
fn scan_all(
    documents: Vec<(&'static str, Vec<u8>)>,
) -> Vec<(&'static str, Result<SourceDocument, String>)> {
    let started = std::time::Instant::now();
    let waiting: Vec<(&'static str, Option<mpsc::Receiver<SourceDocument>>)> = documents
        .into_iter()
        .map(|(shape, bytes)| {
            let (sender, receiver) = mpsc::channel();
            let spawned = std::thread::Builder::new()
                .name(format!("scan {shape}"))
                .spawn(move || {
                    let doc = ScannerRegistry::with_builtin_scanners()
                        .scan_bytes(Path::new("novel.docx"), &bytes);
                    // The receiver is gone only once the test has already failed.
                    let _ = sender.send(doc);
                });
            (shape, spawned.ok().map(|_| receiver))
        })
        .collect();
    waiting
        .into_iter()
        .map(|(shape, receiver)| {
            let Some(receiver) = receiver else {
                return (
                    shape,
                    Err("the scan thread could not be started".to_string()),
                );
            };
            let left = PATIENCE.saturating_sub(started.elapsed());
            let result = match receiver.recv_timeout(left) {
                Ok(doc) => Ok(doc),
                Err(mpsc::RecvTimeoutError::Timeout) => Err(format!(
                    "the scan did not return within {PATIENCE:?}: docx-rs is reading for ever"
                )),
                Err(mpsc::RecvTimeoutError::Disconnected) => Err("the scan panicked".to_string()),
            };
            (shape, result)
        })
        .collect()
}

// ---------------------------------------------------------------------------
// A package
// ---------------------------------------------------------------------------

const W: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
const A: &str = "http://schemas.openxmlformats.org/drawingml/2006/main";
const REL: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
const PACKAGE_RELS: &str = "http://schemas.openxmlformats.org/package/2006/relationships";

/// The words every whole document carries in its body.
const WORDS: &str = "The words that must arrive.";

/// A `.docx` built member by member.
#[derive(Clone)]
struct Docx {
    body: String,
    /// The main part's relationships in full, or `None` for the default: one
    /// `Relationships` element holding [`Docx::relationships`].
    document_rels: Option<String>,
    relationships: Vec<String>,
    /// The custom properties part, when the package names one.
    custom: Option<String>,
    members: Vec<(String, String)>,
}

impl Docx {
    /// A document whose body is `body`, followed by a paragraph holding [`WORDS`].
    fn with_body(body: &str) -> Self {
        Docx {
            body: format!("{body}<w:p><w:r><w:t>{WORDS}</w:t></w:r></w:p>"),
            document_rels: None,
            relationships: Vec::new(),
            custom: None,
            members: Vec::new(),
        }
    }

    fn plain() -> Self {
        Self::with_body("")
    }

    /// The main part exactly as given, in place of the one built from the body.
    fn with_document(mut self, document: &str) -> Self {
        self.members
            .push(("word/document.xml".to_string(), document.to_string()));
        self
    }

    /// The part `target`, related from the main part as `kind`, holding `xml`.
    fn related(mut self, kind: &str, target: &str, xml: &str) -> Self {
        let id = format!("rId{}", 10 + self.relationships.len());
        self.relationships.push(format!(
            "<Relationship Id=\"{id}\" Type=\"{REL}/{kind}\" Target=\"{target}\"/>"
        ));
        self.members
            .push((format!("word/{target}"), xml.to_string()));
        self
    }

    /// Any other member.
    fn member(mut self, name: &str, xml: &str) -> Self {
        self.members.push((name.to_string(), xml.to_string()));
        self
    }

    fn document_rels(mut self, xml: &str) -> Self {
        self.document_rels = Some(xml.to_string());
        self
    }

    fn custom(mut self, xml: &str) -> Self {
        self.custom = Some(xml.to_string());
        self
    }

    fn bytes(&self) -> Vec<u8> {
        let content_types = "<?xml version=\"1.0\" encoding=\"UTF-8\"?><Types \
            xmlns=\"http://schemas.openxmlformats.org/package/2006/content-types\"><Default \
            Extension=\"rels\" ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/>\
            <Default Extension=\"xml\" ContentType=\"application/xml\"/></Types>";
        let custom_rel = if self.custom.is_some() {
            format!(
                "<Relationship Id=\"rId2\" Type=\"{REL}/custom-properties\" \
                 Target=\"docProps/custom.xml\"/>"
            )
        } else {
            String::new()
        };
        let package = format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><Relationships xmlns=\"{PACKAGE_RELS}\">\
             <Relationship Id=\"rId1\" Type=\"{REL}/officeDocument\" Target=\"word/document.xml\"/>\
             {custom_rel}</Relationships>"
        );
        let document_rels = self.document_rels.clone().unwrap_or_else(|| {
            format!(
                "<?xml version=\"1.0\" encoding=\"UTF-8\"?><Relationships xmlns=\"{PACKAGE_RELS}\">\
                 {}</Relationships>",
                self.relationships.concat()
            )
        });
        let document = format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><w:document xmlns:w=\"{W}\"><w:body>{}\
             </w:body></w:document>",
            self.body
        );

        let mut members: Vec<(String, String)> = vec![
            ("[Content_Types].xml".into(), content_types.into()),
            ("_rels/.rels".into(), package),
            ("word/_rels/document.xml.rels".into(), document_rels),
        ];
        if !self
            .members
            .iter()
            .any(|(name, _)| name == "word/document.xml")
        {
            members.push(("word/document.xml".into(), document));
        }
        if let Some(custom) = &self.custom {
            members.push(("docProps/custom.xml".into(), custom.clone()));
        }
        members.extend(self.members.iter().cloned());

        let mut out = std::io::Cursor::new(Vec::new());
        let mut writer = zip::ZipWriter::new(&mut out);
        let options = zip::write::SimpleFileOptions::default();
        for (name, xml) in &members {
            writer.start_file(name.as_str(), options).expect("member");
            writer.write_all(xml.as_bytes()).expect("member bytes");
        }
        writer.finish().expect("finish");
        out.into_inner()
    }
}

fn relationships(inner: &str) -> String {
    format!(
        "<?xml version=\"1.0\"?><Relationships xmlns=\"{PACKAGE_RELS}\">{inner}</Relationships>"
    )
}

fn styles(inner: &str) -> String {
    format!("<?xml version=\"1.0\"?><w:styles xmlns:w=\"{W}\">{inner}</w:styles>")
}

const A_STYLE: &str = "<w:style w:type=\"paragraph\" w:styleId=\"Normal\"><w:name w:val=\"Normal\"/>\
     <w:rPr><w:sz w:val=\"24\"/></w:rPr></w:style>";

fn header(inner: &str) -> String {
    format!("<?xml version=\"1.0\"?><w:hdr xmlns:w=\"{W}\">{inner}</w:hdr>")
}

fn footer(inner: &str) -> String {
    format!("<?xml version=\"1.0\"?><w:ftr xmlns:w=\"{W}\">{inner}</w:ftr>")
}

const A_PARAGRAPH: &str = "<w:p><w:r><w:t>Running head</w:t></w:r></w:p>";

fn comments(inner: &str) -> String {
    format!("<?xml version=\"1.0\"?><w:comments xmlns:w=\"{W}\">{inner}</w:comments>")
}

const A_COMMENT: &str = "<w:comment w:id=\"0\" w:author=\"Ada\"><w:p><w:r><w:t>A note.</w:t></w:r>\
     </w:p></w:comment>";

fn numbering(inner: &str) -> String {
    format!("<?xml version=\"1.0\"?><w:numbering xmlns:w=\"{W}\">{inner}</w:numbering>")
}

const A_DEFINITION: &str = "<w:abstractNum w:abstractNumId=\"0\"><w:lvl w:ilvl=\"0\">\
     <w:start w:val=\"1\"/></w:lvl></w:abstractNum>";

fn theme(inner: &str) -> String {
    format!(
        "<?xml version=\"1.0\"?><a:theme xmlns:a=\"{A}\"><a:themeElements>{inner}</a:themeElements></a:theme>"
    )
}

const A_FONT_SCHEME: &str = "<a:fontScheme name=\"Office\"><a:majorFont><a:latin typeface=\"Serif\"/>\
     </a:majorFont><a:minorFont><a:latin typeface=\"Serif\"/></a:minorFont></a:fontScheme>";

fn custom_properties(inner: &str) -> String {
    format!(
        "<?xml version=\"1.0\"?><Properties xmlns:vt=\"http://schemas.openxmlformats.org/\
         officeDocument/2006/docPropsVTypes\">{inner}</Properties>"
    )
}

const A_PROPERTY: &str = "<property name=\"draft\"><vt:lpwstr>third</vt:lpwstr></property>";

// ---------------------------------------------------------------------------
// The shapes
// ---------------------------------------------------------------------------

/// One way to make `docx-rs` read for ever: the document, the part it must be refused
/// for, and the same document whole.
struct Shape {
    name: &'static str,
    part: &'static str,
    broken: Docx,
    whole: Docx,
}

fn document_cut_after(body: &str) -> String {
    format!("<?xml version=\"1.0\"?><w:document xmlns:w=\"{W}\"><w:body>{body}")
}

fn shapes() -> Vec<Shape> {
    let whole_body = format!("<w:p><w:r><w:t>{WORDS}</w:t></w:r></w:p>");
    vec![
        Shape {
            name: "the main part cut short inside a paragraph",
            part: "word/document.xml",
            broken: Docx::plain()
                .with_document(&document_cut_after(&format!("<w:p><w:r><w:t>{WORDS}</w:t>"))),
            whole: Docx::plain(),
        },
        Shape {
            name: "the main part cut short after its last paragraph",
            part: "word/document.xml",
            broken: Docx::plain().with_document(&document_cut_after(&whole_body)),
            whole: Docx::plain(),
        },
        Shape {
            name: "a syntax error inside a content control",
            part: "word/document.xml",
            broken: Docx::with_body("<w:sdt><w:sdtContent><w:tbl><!- ></w:tbl></w:sdtContent></w:sdt>"),
            whole: Docx::with_body(
                "<w:sdt><w:sdtContent><w:tbl><w:tr><w:tc><w:p/></w:tc></w:tr></w:tbl>\
                 </w:sdtContent></w:sdt>",
            ),
        },
        Shape {
            name: "a mismatched end tag inside a tracked move",
            part: "word/document.xml",
            broken: Docx::with_body("<w:p><w:ins><w:moveFrom><w:r></w:moveFrom></w:ins></w:p>"),
            whole: Docx::with_body(
                "<w:p><w:ins><w:moveFrom><w:r><w:t>Moved away.</w:t></w:r></w:moveFrom></w:ins></w:p>",
            ),
        },
        Shape {
            name: "the main part's relationships, empty",
            part: "word/_rels/document.xml.rels",
            broken: Docx::plain().document_rels(""),
            whole: Docx::plain(),
        },
        Shape {
            name: "the main part's relationships under another root",
            part: "word/_rels/document.xml.rels",
            broken: Docx::plain().document_rels("<?xml version=\"1.0\"?><Types/>"),
            whole: Docx::plain(),
        },
        Shape {
            name: "the main part's relationships cut short",
            part: "word/_rels/document.xml.rels",
            broken: Docx::plain()
                .document_rels(&format!("<?xml version=\"1.0\"?><Relationships xmlns=\"{PACKAGE_RELS}\">")),
            whole: Docx::plain(),
        },
        Shape {
            name: "an empty style sheet",
            part: "word/styles.xml",
            broken: Docx::plain().related("styles", "styles.xml", ""),
            whole: Docx::plain().related("styles", "styles.xml", &styles(A_STYLE)),
        },
        Shape {
            name: "a style sheet under another root",
            part: "word/styles.xml",
            broken: Docx::plain().related(
                "styles",
                "styles.xml",
                &format!("<?xml version=\"1.0\"?><w:docDefaults xmlns:w=\"{W}\"/>"),
            ),
            whole: Docx::plain().related("styles", "styles.xml", &styles(A_STYLE)),
        },
        Shape {
            name: "a style sheet cut short inside a style",
            part: "word/styles.xml",
            broken: Docx::plain().related(
                "styles",
                "styles.xml",
                &format!(
                    "<?xml version=\"1.0\"?><w:styles xmlns:w=\"{W}\"><w:style w:type=\"paragraph\" \
                     w:styleId=\"Normal\"><w:name w:val=\"Normal\"/>"
                ),
            ),
            whole: Docx::plain().related("styles", "styles.xml", &styles(A_STYLE)),
        },
        Shape {
            name: "a header cut short inside a paragraph",
            part: "word/header1.xml",
            broken: Docx::plain().related(
                "header",
                "header1.xml",
                &format!("<?xml version=\"1.0\"?><w:hdr xmlns:w=\"{W}\"><w:p>"),
            ),
            whole: Docx::plain().related("header", "header1.xml", &header(A_PARAGRAPH)),
        },
        Shape {
            name: "a header's relationships under another root",
            part: "word/_rels/header1.xml.rels",
            broken: Docx::plain()
                .related("header", "header1.xml", &header(A_PARAGRAPH))
                .member("word/_rels/header1.xml.rels", "<?xml version=\"1.0\"?><Types/>"),
            whole: Docx::plain()
                .related("header", "header1.xml", &header(A_PARAGRAPH))
                .member("word/_rels/header1.xml.rels", &relationships("")),
        },
        Shape {
            name: "a footer's relationships cut short",
            part: "word/_rels/footer1.xml.rels",
            broken: Docx::plain()
                .related("footer", "footer1.xml", &footer(A_PARAGRAPH))
                .member(
                    "word/_rels/footer1.xml.rels",
                    &format!("<?xml version=\"1.0\"?><Relationships xmlns=\"{PACKAGE_RELS}\">"),
                ),
            whole: Docx::plain()
                .related("footer", "footer1.xml", &footer(A_PARAGRAPH))
                .member("word/_rels/footer1.xml.rels", &relationships("")),
        },
        Shape {
            name: "comments cut short inside a comment",
            part: "word/comments.xml",
            broken: Docx::plain().related(
                "comments",
                "comments.xml",
                &format!("<?xml version=\"1.0\"?><w:comments xmlns:w=\"{W}\"><w:comment w:id=\"0\"><w:p>"),
            ),
            whole: Docx::plain().related("comments", "comments.xml", &comments(A_COMMENT)),
        },
        Shape {
            name: "numbering cut short inside a definition",
            part: "word/numbering.xml",
            broken: Docx::plain().related(
                "numbering",
                "numbering.xml",
                &format!(
                    "<?xml version=\"1.0\"?><w:numbering xmlns:w=\"{W}\"><w:abstractNum \
                     w:abstractNumId=\"0\">"
                ),
            ),
            whole: Docx::plain().related("numbering", "numbering.xml", &numbering(A_DEFINITION)),
        },
        Shape {
            name: "a theme cut short inside its font scheme",
            part: "word/theme/theme1.xml",
            broken: Docx::plain().related(
                "theme",
                "theme/theme1.xml",
                &format!(
                    "<?xml version=\"1.0\"?><a:theme xmlns:a=\"{A}\"><a:themeElements>\
                     <a:fontScheme name=\"Office\"><a:majorFont>"
                ),
            ),
            whole: Docx::plain().related("theme", "theme/theme1.xml", &theme(A_FONT_SCHEME)),
        },
        Shape {
            name: "a custom property cut short",
            part: "docProps/custom.xml",
            broken: Docx::plain().custom(
                "<?xml version=\"1.0\"?><Properties><property name=\"draft\"><vt:lpwstr>third",
            ),
            whole: Docx::plain().custom(&custom_properties(A_PROPERTY)),
        },
        Shape {
            name: "a custom property with a syntax error",
            part: "docProps/custom.xml",
            broken: Docx::plain().custom(&custom_properties(
                "<property name=\"draft\"><!- ></property>",
            )),
            whole: Docx::plain().custom(&custom_properties(A_PROPERTY)),
        },
    ]
}

fn prose(doc: &SourceDocument) -> String {
    doc.blocks
        .iter()
        .map(SourceBlock::plain_text)
        .collect::<Vec<_>>()
        .join("\n")
}

/// Every shape is refused as a file that could not be read, naming the part, and
/// nothing of it is imported. Each of them used to keep the import reading for ever.
#[test]
fn a_part_docx_rs_would_never_finish_reading_is_refused_by_name() {
    let shapes = shapes();
    let parts: Vec<&str> = shapes.iter().map(|shape| shape.part).collect();
    let scanned = scan_all(
        shapes
            .into_iter()
            .map(|shape| (shape.name, shape.broken.bytes()))
            .collect(),
    );
    let mut wrong = Vec::new();
    for ((shape, scanned), part) in scanned.into_iter().zip(parts) {
        let doc = match scanned {
            Ok(doc) => doc,
            Err(why) => {
                wrong.push(format!("{shape}: {why}"));
                continue;
            }
        };
        match doc.diagnostics.as_slice() {
            [ImportDiagnostic::FileUnreadable { reason, .. }] if reason.contains(part) => {}
            other => wrong.push(format!(
                "{shape}: expected one unreadable-file refusal naming {part}, got {other:?}"
            )),
        }
        if !doc.blocks.is_empty() {
            wrong.push(format!("{shape}: a refused file contributes nothing"));
        }
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}

/// The same documents whole are read, their words arriving, so the refusal is of the
/// damage and never of the construct.
#[test]
fn the_same_documents_whole_are_read() {
    let scanned = scan_all(
        shapes()
            .into_iter()
            .map(|shape| (shape.name, shape.whole.bytes()))
            .collect(),
    );
    let mut wrong = Vec::new();
    for (shape, scanned) in scanned {
        let doc = match scanned {
            Ok(doc) => doc,
            Err(why) => {
                wrong.push(format!("{shape}: {why}"));
                continue;
            }
        };
        if doc.diagnostics.iter().any(|d| {
            matches!(
                d,
                ImportDiagnostic::FileUnreadable { .. } | ImportDiagnostic::NestedTooDeep { .. }
            )
        }) {
            wrong.push(format!(
                "{shape}: a whole document is read, got {:?}",
                doc.diagnostics
            ));
        }
        if !prose(&doc).contains(WORDS) {
            wrong.push(format!("{shape}: the words arrive: {:?}", prose(&doc)));
        }
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}

/// A part the import rewrites before `docx-rs` reads it, cut short inside a no-break
/// hyphen, a soft hyphen or a symbol spelled with an end tag rather than as an empty
/// element. Nothing holds the notes, or a comments part the document does not name, to
/// their end, since `docx-rs` reads neither with a reader that waits for one; the rewrite
/// waited for the element's end tag, which never came, and the import read for ever. Each
/// is now read in a moment and its body's words arrive.
///
/// With it, an element spelled with an end tag and holding another of its own name: the
/// rewrite stopped at the inner end tag and left the outer one behind, a stray end tag in
/// a document that had none.
#[test]
fn a_part_cut_short_inside_a_hyphen_or_a_symbol_is_read_at_once() {
    let cut = |root: &str, inner: &str| {
        format!("<?xml version=\"1.0\"?><w:{root} xmlns:w=\"{W}\">{inner}")
    };
    let scanned = scan_all(vec![
        (
            "footnotes cut short inside a no-break hyphen",
            Docx::plain()
                .related(
                    "footnotes",
                    "footnotes.xml",
                    &cut(
                        "footnotes",
                        "<w:footnote w:id=\"1\"><w:p><w:r><w:t>twenty</w:t><w:noBreakHyphen>",
                    ),
                )
                .bytes(),
        ),
        (
            "endnotes cut short inside a symbol",
            Docx::plain()
                .related(
                    "endnotes",
                    "endnotes.xml",
                    &cut(
                        "endnotes",
                        "<w:endnote w:id=\"1\"><w:p><w:r><w:sym w:font=\"Symbol\" w:char=\"F061\">",
                    ),
                )
                .bytes(),
        ),
        (
            "comments the document does not name, cut short inside a soft hyphen",
            Docx::plain()
                .member(
                    "word/comments.xml",
                    &cut(
                        "comments",
                        "<w:comment w:id=\"0\"><w:p><w:r><w:t>soft</w:t><w:softHyphen>",
                    ),
                )
                .bytes(),
        ),
        (
            "a no-break hyphen holding another",
            Docx::with_body(
                "<w:p><w:r><w:t>twenty</w:t><w:noBreakHyphen><w:noBreakHyphen>\
                 </w:noBreakHyphen></w:noBreakHyphen><w:t>one</w:t></w:r></w:p>",
            )
            .bytes(),
        ),
    ]);
    let mut wrong = Vec::new();
    for (shape, scanned) in scanned {
        let doc = match scanned {
            Ok(doc) => doc,
            Err(why) => {
                wrong.push(format!("{shape}: {why}"));
                continue;
            }
        };
        if doc
            .diagnostics
            .iter()
            .any(|d| matches!(d, ImportDiagnostic::FileUnreadable { .. }))
        {
            wrong.push(format!("{shape}: read, got {:?}", doc.diagnostics));
        }
        if !prose(&doc).contains(WORDS) {
            wrong.push(format!("{shape}: the words arrive: {:?}", prose(&doc)));
        }
        if shape.starts_with("a no-break hyphen") && !prose(&doc).contains("twenty\u{2011}one") {
            wrong.push(format!(
                "{shape}: one hyphen, in its word: {:?}",
                prose(&doc)
            ));
        }
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}
