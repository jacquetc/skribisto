// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Leading `---` / `+++` metadata blocks.
//!
//! Stripped **before** anything else looks at the text, because leaving one in
//! is actively destructive rather than merely unhelpful: `text-document`'s
//! Markdown reader has both metadata-block flags off, so a leading
//! `---\ntitle: My Book\n---` parses as a setext heading and lands in the prose
//! as `## title: My Book`. Verified, not assumed.
//!
//! Parsing is flat `key: value` scalars and nothing more. Writing tools put
//! titles, authors and ordering in front matter; nested YAML belongs to static
//! site generators. A key whose value is a list or a map is reported and skipped
//! rather than mangled, so the writer learns their `tags: [a, b]` did not come
//! across instead of finding out later.

use crate::block::SourceMetadata;
use crate::diagnostics::ImportDiagnostic;

/// What was found at the top of the file, and where the body starts.
pub struct FrontMatter {
    pub metadata: SourceMetadata,
    /// Byte offset in the original text where the body begins. Callers slice
    /// from here so every downstream byte offset stays relative to one string.
    pub body_offset: usize,
    pub diagnostics: Vec<ImportDiagnostic>,
}

impl FrontMatter {
    fn none() -> Self {
        FrontMatter {
            metadata: SourceMetadata::default(),
            body_offset: 0,
            diagnostics: Vec::new(),
        }
    }
}

/// Split `text` into its front matter and the offset of the body.
///
/// A fence must be the very first line — `---` further down is a thematic break
/// or a setext underline, and treating it as metadata would eat a scene break.
pub fn split(text: &str, path: &str) -> FrontMatter {
    let Some((fence, rest_offset)) = opening_fence(text) else {
        return FrontMatter::none();
    };

    // Find the closing fence: a line that is exactly the fence (YAML also allows
    // `...` to end a document, which some exporters emit).
    let mut cursor = rest_offset;
    let mut inner_end = None;
    let mut body_offset = text.len();
    while cursor < text.len() {
        let line_end = text[cursor..]
            .find('\n')
            .map(|i| cursor + i)
            .unwrap_or(text.len());
        let line = text[cursor..line_end].trim_end();
        if line == fence || (fence == "---" && line == "...") {
            inner_end = Some(cursor);
            body_offset = (line_end + 1).min(text.len());
            break;
        }
        cursor = line_end + 1;
    }

    // An unterminated fence is not front matter — it is a document that happens
    // to start with a thematic break. Treating it as metadata would swallow the
    // whole manuscript.
    let Some(inner_end) = inner_end else {
        return FrontMatter::none();
    };

    let mut metadata = SourceMetadata::default();
    let mut diagnostics = Vec::new();
    for line in text[rest_offset..inner_end].lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let Some((key, value)) = split_pair(trimmed, fence) else {
            continue;
        };
        let key = key.trim().to_ascii_lowercase();
        let value = unquote(value.trim());

        if value.is_empty() {
            // `tags:` followed by an indented list, or a nested map. The keys we
            // care about are all scalars, so this is a real skip worth naming.
            diagnostics.push(ImportDiagnostic::FrontMatterNotFlat {
                path: path.to_string(),
                key: key.clone(),
            });
            continue;
        }

        match key.as_str() {
            "title" => metadata.title = Some(value.to_string()),
            "author" => metadata.author = Some(value.to_string()),
            "order" | "weight" | "position" => {
                if let Ok(n) = value.parse::<i64>() {
                    metadata.order_hint = Some(n);
                }
            }
            _ => {}
        }
        metadata.raw.insert(key, value.to_string());
    }

    FrontMatter {
        metadata,
        body_offset,
        diagnostics,
    }
}

/// The opening fence and the offset just past its line, if `text` starts with one.
fn opening_fence(text: &str) -> Option<(&'static str, usize)> {
    for fence in ["---", "+++"] {
        if let Some(rest) = text.strip_prefix(fence) {
            // The fence must be alone on its line: `----` is a setext underline
            // and `--- something` is prose.
            let line_end = rest.find('\n')?;
            if rest[..line_end].trim().is_empty() {
                return Some((fence, fence.len() + line_end + 1));
            }
        }
    }
    None
}

/// `key: value` for YAML fences, `key = value` for TOML ones.
fn split_pair<'a>(line: &'a str, fence: &str) -> Option<(&'a str, &'a str)> {
    let sep = if fence == "+++" { '=' } else { ':' };
    line.split_once(sep)
}

fn unquote(value: &str) -> &str {
    let bytes = value.as_bytes();
    if bytes.len() >= 2
        && ((bytes[0] == b'"' && bytes[bytes.len() - 1] == b'"')
            || (bytes[0] == b'\'' && bytes[bytes.len() - 1] == b'\''))
    {
        &value[1..value.len() - 1]
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yaml_front_matter_is_read_and_removed() {
        let text = "---\ntitle: The Storm\nauthor: Jane\norder: 3\n---\n\nProse begins.";
        let fm = split(text, "a.md");
        assert_eq!(fm.metadata.title.as_deref(), Some("The Storm"));
        assert_eq!(fm.metadata.author.as_deref(), Some("Jane"));
        assert_eq!(fm.metadata.order_hint, Some(3));
        assert_eq!(&text[fm.body_offset..], "\nProse begins.");
    }

    #[test]
    fn toml_front_matter_uses_equals() {
        let text = "+++\ntitle = \"The Storm\"\n+++\nProse.";
        let fm = split(text, "a.md");
        assert_eq!(fm.metadata.title.as_deref(), Some("The Storm"));
        assert_eq!(&text[fm.body_offset..], "Prose.");
    }

    /// The case that matters most: a `---` that is a scene break, not metadata.
    /// Swallowing it would eat the manuscript up to the next one.
    #[test]
    fn an_unterminated_opening_fence_is_not_front_matter() {
        let text = "---\n\nProse after a thematic break.";
        let fm = split(text, "a.md");
        assert_eq!(fm.body_offset, 0);
        assert_eq!(fm.metadata, SourceMetadata::default());
    }

    #[test]
    fn a_thematic_break_further_down_is_left_alone() {
        let text = "Prose.\n\n---\n\nMore prose.";
        assert_eq!(split(text, "a.md").body_offset, 0);
    }

    #[test]
    fn a_non_scalar_value_is_reported_rather_than_mangled() {
        let text = "---\ntitle: A\ntags:\n  - one\n  - two\n---\nProse.";
        let fm = split(text, "a.md");
        assert_eq!(fm.metadata.title.as_deref(), Some("A"));
        assert!(fm.diagnostics.iter().any(
            |d| matches!(d, ImportDiagnostic::FrontMatterNotFlat { key, .. } if key == "tags")
        ));
    }

    #[test]
    fn quotes_are_stripped_from_values() {
        let text = "---\ntitle: \"The Storm\"\n---\nx";
        assert_eq!(
            split(text, "a.md").metadata.title.as_deref(),
            Some("The Storm")
        );
    }
}
