// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Every XML file this crate reads, parsed within bounds.
//!
//! `roxmltree` descends once per nested element and has no ceiling of its own; on
//! a long operation's 2 MiB thread a debug build aborts at about 130 nested
//! elements, and an abort takes every window with it. So each document is first
//! measured by `skrib_format::xml_depth::check`, a loop that cannot overflow on
//! the input it exists to refuse, and only then parsed, on a thread with a stack
//! deep enough for anything under the ceiling. The deepest real Scrivener manifest
//! nests 49 elements; the ceiling is 256.
//!
//! A DTD is allowed, as the other importers allow one: a tool has been seen adding
//! a DOCTYPE to a Scrivener file, and refusing a project over a declaration
//! nothing here reads would be a refusal with no benefit. An entity declaration
//! is not: Scrivener never writes one, and `roxmltree` bounds only the references
//! *inside* entity values (its billion-laughs guard), never the references the
//! document itself makes. One 64 KiB entity named four thousand times turns a
//! 78 KB manifest into a 256 MiB title, and a manifest within the size ceiling
//! into more memory than the computer has. So a document declaring an entity is
//! refused on its text, before any parser expands anything, by the rule the
//! Plume Creator and Manuskript readers follow
//! (`skrib_format::xml_depth::check_entities`). `roxmltree` resolves no external
//! entity. The node count is capped too, so a manifest of empty elements cannot
//! take memory past what any real project needs, which is why the parse is this
//! crate's own rather than `xml_depth::parse`: that one sets no node cap.
//!
//! What is parsed is walked with loops, never by recursion.

use skrib_format::xml_depth;

/// The most nodes one document may hold. The largest real manifest (2,509
/// binder items) holds about a hundred thousand.
pub(crate) const MAX_NODES: u32 = 2_000_000;

/// Why an XML document was not parsed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum XmlProblem {
    /// Not UTF-8.
    NotText,
    /// Nested past the ceiling; refused before any parser saw it.
    TooDeep { depth: u32, line: u32 },
    /// Declares an entity, which no Scrivener file does; refused before any
    /// parser could expand it.
    DeclaresEntities,
    /// Not well-formed, or past the node cap. The parser's own words.
    Malformed(String),
}

impl XmlProblem {
    /// The problem, as a notice carries it.
    pub(crate) fn problem(&self) -> crate::outcome::ReadProblem {
        use crate::outcome::ReadProblem;
        match self {
            XmlProblem::NotText => ReadProblem::NotText,
            XmlProblem::TooDeep { depth, line } => ReadProblem::TooDeep {
                depth: *depth,
                line: *line,
            },
            XmlProblem::DeclaresEntities => ReadProblem::DeclaresEntities,
            XmlProblem::Malformed(reason) => ReadProblem::Malformed {
                reason: reason.clone(),
            },
        }
    }
}

/// `bytes` as text: UTF-8, a byte-order mark dropped.
pub(crate) fn text_of(bytes: &[u8]) -> Result<&str, XmlProblem> {
    let bytes = bytes.strip_prefix(b"\xEF\xBB\xBF").unwrap_or(bytes);
    std::str::from_utf8(bytes).map_err(|_| XmlProblem::NotText)
}

/// Parse `text`, which `part` names in a refusal.
pub(crate) fn parse<'a>(part: &str, text: &'a str) -> Result<roxmltree::Document<'a>, XmlProblem> {
    xml_depth::check_entities(part, text.as_bytes()).map_err(|_| XmlProblem::DeclaresEntities)?;
    xml_depth::check(part, text.as_bytes()).map_err(|too_deep| XmlProblem::TooDeep {
        depth: u32::try_from(too_deep.depth).unwrap_or(u32::MAX),
        line: u32::try_from(too_deep.line).unwrap_or(u32::MAX),
    })?;
    xml_depth::on_parser_stack(move || {
        let options = roxmltree::ParsingOptions {
            allow_dtd: true,
            nodes_limit: MAX_NODES,
            ..roxmltree::ParsingOptions::default()
        };
        roxmltree::Document::parse_with_options(text, options)
    })
    .map_err(|e| XmlProblem::Malformed(e.to_string()))?
    .map_err(|e| XmlProblem::Malformed(e.to_string()))
}

/// The first child element of `node` named `name`.
pub(crate) fn child<'a, 'i>(
    node: roxmltree::Node<'a, 'i>,
    name: &str,
) -> Option<roxmltree::Node<'a, 'i>> {
    node.children()
        .find(|c| c.is_element() && c.tag_name().name() == name)
}

/// Every child element of `node` named `name`, in order.
pub(crate) fn children<'a, 'i>(
    node: roxmltree::Node<'a, 'i>,
    name: &'a str,
) -> impl Iterator<Item = roxmltree::Node<'a, 'i>> + 'a
where
    'i: 'a,
{
    node.children()
        .filter(move |c| c.is_element() && c.tag_name().name() == name)
}

/// The text inside `node`, all of it: text and CDATA sections joined, markup
/// inside skipped. `None` when there is none.
pub(crate) fn text(node: roxmltree::Node<'_, '_>) -> Option<String> {
    let mut out = String::new();
    let mut any = false;
    for c in node.children() {
        if c.is_text()
            && let Some(t) = c.text()
        {
            out.push_str(t);
            any = true;
        }
    }
    any.then_some(out)
}

/// The text of the child element `name`, when present.
pub(crate) fn child_text(node: roxmltree::Node<'_, '_>, name: &str) -> Option<String> {
    child(node, name).and_then(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_document_past_the_ceiling_is_refused_before_it_is_parsed() {
        let deep = format!("{}{}", "<a>".repeat(400), "</a>".repeat(400));
        assert!(matches!(
            parse("deep.xml", &deep),
            Err(XmlProblem::TooDeep { .. })
        ));
    }

    #[test]
    fn a_document_at_the_ceiling_parses_even_in_a_debug_build() {
        let depth = xml_depth::MAX_DEPTH;
        let doc = format!("{}{}", "<a>".repeat(depth), "</a>".repeat(depth));
        // Run where a long operation runs: a 2 MiB thread.
        let parsed = std::thread::Builder::new()
            .stack_size(2 << 20)
            .spawn(move || parse("at-ceiling.xml", &doc).is_ok())
            .map(|handle| handle.join());
        assert!(matches!(parsed, Ok(Ok(true))));
    }

    #[test]
    fn an_entity_bomb_is_refused_not_expanded() {
        let bomb = r#"<?xml version="1.0"?>
<!DOCTYPE lolz [
 <!ENTITY lol "lol">
 <!ENTITY lol2 "&lol;&lol;&lol;&lol;&lol;&lol;&lol;&lol;&lol;&lol;">
 <!ENTITY lol3 "&lol2;&lol2;&lol2;&lol2;&lol2;&lol2;&lol2;&lol2;&lol2;&lol2;">
 <!ENTITY lol4 "&lol3;&lol3;&lol3;&lol3;&lol3;&lol3;&lol3;&lol3;&lol3;&lol3;">
 <!ENTITY lol5 "&lol4;&lol4;&lol4;&lol4;&lol4;&lol4;&lol4;&lol4;&lol4;&lol4;">
 <!ENTITY lol6 "&lol5;&lol5;&lol5;&lol5;&lol5;&lol5;&lol5;&lol5;&lol5;&lol5;">
 <!ENTITY lol7 "&lol6;&lol6;&lol6;&lol6;&lol6;&lol6;&lol6;&lol6;&lol6;&lol6;">
 <!ENTITY lol8 "&lol7;&lol7;&lol7;&lol7;&lol7;&lol7;&lol7;&lol7;&lol7;&lol7;">
 <!ENTITY lol9 "&lol8;&lol8;&lol8;&lol8;&lol8;&lol8;&lol8;&lol8;&lol8;&lol8;">
]>
<ScrivenerProject Version="2.0"><Binder><BinderItem><Title>&lol9;</Title></BinderItem></Binder></ScrivenerProject>"#;
        assert!(parse("bomb.scrivx", bomb).is_err());
    }

    /// No entity nests inside another here, so the parser's billion-laughs guard
    /// never trips: one entity named from the document itself, over and over, is
    /// what it lets through, and what is refused.
    #[test]
    fn an_entity_named_over_and_over_is_refused_before_it_is_expanded() {
        let value = "a".repeat(1 << 10);
        let doc = format!(
            "<!DOCTYPE r [<!ENTITY e \"{value}\">]><r a=\"{refs}\">{refs}</r>",
            refs = "&e;".repeat(1 << 10)
        );
        assert_eq!(
            parse("amplified.xml", &doc).err(),
            Some(XmlProblem::DeclaresEntities)
        );
        // A comment that mentions a declaration is no declaration without a DTD.
        let quiet = "<r><!-- <!ENTITY is not declared here --><![CDATA[<!ENTITY]]></r>";
        assert!(parse("quiet.xml", quiet).is_ok());
    }

    #[test]
    fn a_harmless_doctype_is_allowed() {
        let doc = r#"<?xml version="1.0"?><!DOCTYPE Styles []><Styles><Style/></Styles>"#;
        assert!(parse("styles.xml", doc).is_ok());
    }

    #[test]
    fn text_joins_text_and_cdata() {
        let doc = "<a>one <![CDATA[<two>]]> three</a>";
        let parsed = parse("t.xml", doc);
        let joined = parsed.ok().and_then(|d| text(d.root_element()));
        assert_eq!(joined.as_deref(), Some("one <two> three"));
    }
}
