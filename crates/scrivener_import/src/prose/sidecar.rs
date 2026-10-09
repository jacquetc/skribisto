// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The files beside a document that hold its linked comments and footnotes.
//!
//! Two generations of one mechanism:
//!
//! * **`content.comments`** (format 2.0) and **`<ID>.comments`** (format 1.5): one
//!   `<Comment ID=… Color=… [Footnote="Yes"] [Author=…]>` per comment or footnote, its body a
//!   whole RTF document in CDATA (Windows may write it as plain XML text). The document's RTF
//!   anchors each with a `HYPERLINK "scrivcmt://<ID>"` field over the words it marks. Only
//!   `Footnote="Yes"` tells a footnote from a comment.
//! * **`<ID>.links`** (format 1.0): `<Comment IsFootnote=… [ID=…]>` entries that carry their
//!   place as a `<Range>location,length</Range>` into the document's text (variant A), and
//!   from Scrivener for Mac 2.5 also an `ID` and the same `scrivcmt://` field (variant B, where
//!   the field is the anchor and the range only a cross-check). The same file lists the
//!   document's links to other items as ranges (`<TextLinks><LinkID Range=…>n</LinkID>`).
//!
//! Nothing here decides where anything goes: the entries are handed to the dialect pass,
//! which places them in the text.

use std::collections::HashMap;

use crate::xml;

/// The most entries read from one file: more than any real document holds by far.
const MAX_ENTRIES: usize = 100_000;

/// One comment or footnote a sidecar holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Entry {
    /// The UUID its field names, upper-cased; `None` for a range-only `.links` entry.
    pub(crate) id: Option<String>,
    /// `Footnote="Yes"` or `IsFootnote="Yes"`.
    pub(crate) footnote: bool,
    /// `Author`, when written: absent from almost every real entry.
    pub(crate) author: Option<String>,
    /// Whether it carries a colour: a comment's colour is not kept, and the report says so.
    pub(crate) coloured: bool,
    /// Its body, RTF (or, rarely, plain text), as the XML decoded it.
    pub(crate) body: String,
    /// A `.links` entry's place: location and length into the document's text.
    pub(crate) range: Option<(u64, u64)>,
}

/// A link to another item that a `.links` file anchors by range.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RangeLink {
    pub(crate) range: (u64, u64),
    /// The item it names: a number, as written.
    pub(crate) target: String,
}

/// Everything one sidecar holds.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Sidecar {
    pub(crate) entries: Vec<Entry>,
    pub(crate) links: Vec<RangeLink>,
}

impl Sidecar {
    /// Every entry by the UUID its field names (upper-cased, as read), the first of two
    /// that share one. Built once for a text whose fields may name thousands: a search per
    /// field would cost the square of their number.
    pub(crate) fn ids(&self) -> HashMap<&str, usize> {
        let mut out = HashMap::with_capacity(self.entries.len());
        for (index, entry) in self.entries.iter().enumerate() {
            if let Some(id) = entry.id.as_deref() {
                out.entry(id).or_insert(index);
            }
        }
        out
    }
}

/// Read a `content.comments` or `<ID>.comments` file.
pub(crate) fn read_comments(bytes: &[u8]) -> Result<Sidecar, xml::XmlProblem> {
    let text = xml::text_of(bytes)?;
    let doc = xml::parse("content.comments", text)?;
    let mut sidecar = Sidecar::default();
    for comment in doc
        .descendants()
        .filter(|n| n.is_element() && n.tag_name().name() == "Comment")
        .take(MAX_ENTRIES)
    {
        sidecar.entries.push(Entry {
            id: comment
                .attribute("ID")
                .map(|id| id.trim().to_ascii_uppercase()),
            footnote: yes(comment.attribute("Footnote")),
            author: comment
                .attribute("Author")
                .map(str::trim)
                .filter(|a| !a.is_empty())
                .map(str::to_string),
            coloured: comment.attribute("Color").is_some(),
            body: xml::text(comment).unwrap_or_default(),
            range: None,
        });
    }
    Ok(sidecar)
}

/// Read a format 1.0 `<ID>.links` file.
pub(crate) fn read_links(bytes: &[u8]) -> Result<Sidecar, xml::XmlProblem> {
    let text = xml::text_of(bytes)?;
    let doc = xml::parse("links", text)?;
    let mut sidecar = Sidecar::default();
    for node in doc
        .descendants()
        .filter(|n| n.is_element())
        .take(MAX_ENTRIES)
    {
        match node.tag_name().name() {
            "Comment" => {
                let range = xml::child_text(node, "Range").as_deref().and_then(range_of);
                sidecar.entries.push(Entry {
                    id: node
                        .attribute("ID")
                        .map(|id| id.trim().to_ascii_uppercase()),
                    footnote: yes(node.attribute("IsFootnote")),
                    author: node
                        .attribute("Author")
                        .map(str::trim)
                        .filter(|a| !a.is_empty())
                        .map(str::to_string),
                    coloured: xml::child(node, "Color").is_some(),
                    body: xml::child_text(node, "RTF").unwrap_or_default(),
                    range,
                });
            }
            "LinkID" => {
                if let (Some(range), Some(target)) = (
                    node.attribute("Range").and_then(range_of),
                    xml::text(node).map(|t| t.trim().to_string()),
                ) && !target.is_empty()
                {
                    sidecar.links.push(RangeLink { range, target });
                }
            }
            _ => {}
        }
    }
    Ok(sidecar)
}

fn yes(value: Option<&str>) -> bool {
    value.is_some_and(|v| v.trim().eq_ignore_ascii_case("yes"))
}

/// `location,length`, both counts of UTF-16 code units.
fn range_of(text: &str) -> Option<(u64, u64)> {
    let (location, length) = text.split_once(',')?;
    Some((location.trim().parse().ok()?, length.trim().parse().ok()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_comments_file_tells_footnotes_from_comments() {
        let file = r#"<?xml version="1.0" encoding="UTF-8"?>
<Comments Version="1.0">
    <Comment ID="306420e0-2997-499b-ac4b-2c5e6355c0f2" Color="0.99 0.95 0.75"><![CDATA[{\rtf1 A remark.}]]></Comment>
    <Comment ID="D2B48EB8-F262-4B8F-BD35-52400284BA5C" Footnote="Yes" Number="1" Author="Ann"><![CDATA[{\rtf1 A note.}]]></Comment>
</Comments>"#;
        let sidecar = read_comments(file.as_bytes()).expect("parses");
        assert_eq!(sidecar.entries.len(), 2);
        assert!(!sidecar.entries[0].footnote);
        assert!(sidecar.entries[0].coloured);
        assert!(sidecar.entries[1].footnote);
        assert_eq!(sidecar.entries[1].author.as_deref(), Some("Ann"));
        assert_eq!(
            sidecar
                .ids()
                .get("306420E0-2997-499B-AC4B-2C5E6355C0F2")
                .copied(),
            Some(0),
            "a field's UUID is found whatever case the file wrote it in"
        );
        assert_eq!(sidecar.entries[1].body, r"{\rtf1 A note.}");
    }

    #[test]
    fn a_links_file_reads_its_ranges_and_its_text_links() {
        let file = r#"<?xml version="1.0" encoding="UTF-8"?>
<Links Version="1.0">
    <CommentsAndFootnotes>
        <Comment IsFootnote="Yes" Collapsed="No">
            <Range>159,16</Range>
            <Color>0.992 0.929 0.525</Color>
            <RTF><![CDATA[{\rtf1 Light with match.}]]></RTF>
        </Comment>
        <Comment IsFootnote="No" ID="EE92DA9C-0000-0000-0000-000000000001"><Range>4,6</Range><RTF>{\rtf1 plain}</RTF></Comment>
    </CommentsAndFootnotes>
    <ScrivenerLinks><TextLinks><LinkID Range="246,6">119</LinkID></TextLinks></ScrivenerLinks>
</Links>"#;
        let sidecar = read_links(file.as_bytes()).expect("parses");
        assert_eq!(sidecar.entries.len(), 2);
        assert_eq!(sidecar.entries[0].range, Some((159, 16)));
        assert!(sidecar.entries[0].footnote);
        assert_eq!(sidecar.entries[0].id, None);
        assert_eq!(sidecar.entries[1].body, r"{\rtf1 plain}", "plain XML text");
        assert_eq!(
            sidecar.links,
            [RangeLink {
                range: (246, 6),
                target: "119".into()
            }]
        );
    }
}
