// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Documents whose DTD declares thousands of entities and names one of them
//! hundreds of thousands of times.
//!
//! Before any parser reads a Word part or an OpenDocument file, the XML depth
//! check (`skrib_format::xml_depth::check`) measures it, entities included, and it
//! looks every reference up among the declared names. When that lookup walked the
//! declarations, the check cost declarations times references: the Word document
//! here, 2.6 MB of XML, kept the import busy for a minute in a debug build, and a
//! part within the importer's size limits (512 MiB) would have kept it busy for
//! days. Nothing is refused meanwhile, and Cancel waits for the file to end.
//!
//! Each scan here runs as the importer runs it, on a thread with a long
//! operation's stack, and has to end well within [`BUDGET`]; a regression fails
//! the test when the budget runs out rather than when the scan finally ends.

use std::io::Write;
use std::path::Path;
use std::time::Duration;

use document_ingest::{ImportDiagnostic, ScannerRegistry, SourceDocument};

/// The stack `std::thread::spawn` gives a long operation's worker by default.
const LONG_OPERATION_STACK: usize = 2 * 1024 * 1024;

/// How many entities the DTD declares.
const DECLARATIONS: usize = 50_000;
/// How many times the body names the last of them.
const REFERENCES: usize = 200_000;

/// How long a scan may take. The lookup that walked the declarations took a
/// minute over each of these documents in a debug build.
const BUDGET: Duration = Duration::from_secs(20);

/// The DTD's declarations, and the run of references naming the last one
/// declared: the name a search in declaration order finds last.
fn entities() -> (String, String) {
    let mut declarations = String::new();
    for i in 0..DECLARATIONS {
        declarations.push_str(&format!("<!ENTITY e{i} 'x'>"));
    }
    let references = format!("&e{};", DECLARATIONS - 1).repeat(REFERENCES);
    (declarations, references)
}

fn zip(members: &[(&str, &[u8])]) -> Vec<u8> {
    let mut out = std::io::Cursor::new(Vec::new());
    let mut writer = zip::ZipWriter::new(&mut out);
    let options = zip::write::SimpleFileOptions::default();
    for (name, bytes) in members {
        writer.start_file(*name, options).expect("member");
        writer.write_all(bytes).expect("member bytes");
    }
    writer.finish().expect("finish");
    out.into_inner()
}

/// Scan `bytes` as the file `name` on a thread with a long operation's stack, and
/// return what the scan produced, or panic once [`BUDGET`] has run out. The scan's
/// thread is left running then; the test binary ends it.
fn scan_within_budget(name: &'static str, bytes: Vec<u8>) -> SourceDocument {
    let file_bytes = bytes.len();
    let (done, outcome) = std::sync::mpsc::channel();
    std::thread::Builder::new()
        .stack_size(LONG_OPERATION_STACK)
        .spawn(move || {
            let doc = ScannerRegistry::with_builtin_scanners().scan_bytes(Path::new(name), &bytes);
            // The receiver is gone only once the budget ran out; nothing waits then.
            let _ = done.send(doc);
        })
        .expect("spawn the scan thread");
    match outcome.recv_timeout(BUDGET) {
        Ok(scanned) => scanned,
        Err(std::sync::mpsc::RecvTimeoutError::Timeout) => panic!(
            "scanning {name} ({file_bytes} bytes; {DECLARATIONS} entities declared, one named \
             {REFERENCES} times) took more than {BUDGET:?}"
        ),
        Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
            panic!("the scan of {name} unwound")
        }
    }
}

/// Reported unreadable, and nothing else: neither importer expands an entity a
/// document declares, so neither can read these, and the entities are one level
/// of text, far from the nesting ceiling.
fn assert_unreadable(shape: &str, doc: &SourceDocument) {
    assert!(
        matches!(
            doc.diagnostics.as_slice(),
            [ImportDiagnostic::FileUnreadable { .. }]
        ),
        "{shape}: expected the file to be reported unreadable, got {:?}",
        doc.diagnostics
    );
    assert!(doc.blocks.is_empty(), "{shape}: nothing is read from it");
}

const W: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
const REL: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";

/// A `.docx` whose `word/document.xml` carries `doctype` before its root and
/// holds one paragraph of `text`.
fn docx(doctype: &str, text: &str) -> Vec<u8> {
    let content_types = "<?xml version=\"1.0\" encoding=\"UTF-8\"?><Types \
        xmlns=\"http://schemas.openxmlformats.org/package/2006/content-types\"><Default \
        Extension=\"rels\" ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/>\
        <Default Extension=\"xml\" ContentType=\"application/xml\"/></Types>";
    let package = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><Relationships \
         xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\"><Relationship \
         Id=\"rId1\" Type=\"{REL}/officeDocument\" Target=\"word/document.xml\"/></Relationships>"
    );
    let document = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>{doctype}<w:document xmlns:w=\"{W}\">\
         <w:body><w:p><w:r><w:t>Chapter One</w:t></w:r></w:p>\
         <w:p><w:r><w:t>{text}</w:t></w:r></w:p></w:body></w:document>"
    );
    let document_rels = "<?xml version=\"1.0\" encoding=\"UTF-8\"?><Relationships \
        xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\"></Relationships>";
    zip(&[
        ("[Content_Types].xml", content_types.as_bytes()),
        ("_rels/.rels", package.as_bytes()),
        ("word/document.xml", document.as_bytes()),
        ("word/_rels/document.xml.rels", document_rels.as_bytes()),
    ])
}

/// The Word importer measures every part `docx-rs` will read before it reads any
/// of them (`refuse_unreadable_parts`), the main document first. `docx-rs` then
/// stops at the first reference it cannot resolve.
#[test]
fn a_docx_declaring_thousands_of_entities_is_scanned_in_a_moment() {
    let (declarations, references) = entities();
    let bytes = docx(
        &format!("<!DOCTYPE w:document [{declarations}]>"),
        &references,
    );

    let doc = scan_within_budget("entities.docx", bytes);

    assert_unreadable("docx", &doc);
}

const OFFICE: &str = "urn:oasis:names:tc:opendocument:xmlns:office:1.0";
const TEXT: &str = "urn:oasis:names:tc:opendocument:xmlns:text:1.0";

/// The OpenDocument importer refuses a DTD, but through `xml_depth::parse`, which
/// measures the document before the parser can refuse it.
#[test]
fn a_flat_odt_declaring_thousands_of_entities_is_scanned_in_a_moment() {
    let (declarations, references) = entities();
    let bytes = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><!DOCTYPE office:document [{declarations}]>\
         <office:document xmlns:office=\"{OFFICE}\" xmlns:text=\"{TEXT}\" office:version=\"1.3\">\
         <office:body><office:text><text:p>{references}</text:p></office:text></office:body>\
         </office:document>"
    )
    .into_bytes();

    let doc = scan_within_budget("entities.fodt", bytes);

    assert_unreadable("fodt", &doc);
}
