// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Turning a Manuskript body into the Djot a `.skrib` stores.
//!
//! **Conversion happens here, at the format boundary, and nowhere else.** A
//! body's markup language is a property of the file it was read from — the
//! item's `type` says which — and that fact is available to the readers and to
//! nothing downstream. So the readers convert, [`crate::model::OutlineItem::text`]
//! holds Djot, and the mapper passes it through untouched.
//!
//! Getting that boundary wrong is not a loud failure, which is why it is stated
//! this plainly. Converting twice reads Djot as Markdown and silently turns
//! `*bold*` into `_italic_`; not converting at all hands raw HTML to a Markdown
//! parser, which returns an empty document and loses the scene.
//!
//! Manuskript itself purged the `txt`, `t2t` and `html` types in 0.3.0 and
//! coerces any survivor to `md` on load, flattening an HTML body through its own
//! `HTML2PlainText` and discarding the markup. The markup is kept here instead,
//! so the emphasis and the paragraphs of a 2016 project survive.

/// What a body converted to, and anything the writer should be told about it.
pub struct Converted {
    pub djot: String,
    /// `None` when the conversion was clean.
    pub notice: Option<String>,
}

/// Convert one body, given the item `type` the file declared.
///
/// `what` names the source in any notice: a member path, or the row's title.
pub fn to_djot(declared_type: &str, body: &str, what: &str) -> Converted {
    if body.trim().is_empty() {
        return Converted {
            djot: String::new(),
            notice: None,
        };
    }
    // Only the pre-0.3.0 `html` type is markup of another kind. `txt` and `t2t`
    // are read as Markdown, which is what Manuskript's own coercion to `md`
    // amounts to, and which leaves plain text plain.
    let converted = if declared_type == "html" {
        skrib_format::html_to_djot(body)
    } else {
        skrib_format::markdown_to_djot(body)
    };
    match converted {
        Ok(djot) => Converted { djot, notice: None },
        Err(e) => Converted {
            // Keeping the source is better than keeping nothing: the writer can
            // see their words and clean up the markup themselves.
            djot: body.to_string(),
            notice: Some(format!(
                "The text of '{what}' could not be converted ({e}); it was kept exactly as it \
                 was written."
            )),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_markdown_body_becomes_djot() {
        let out = to_djot("md", "A **strong** word.", "a scene");
        assert!(out.notice.is_none());
        assert!(out.djot.contains("*strong*"), "{}", out.djot);
    }

    /// The 2016 case. Handing this to a Markdown parser returns an empty
    /// document and the scene is gone.
    #[test]
    fn an_html_body_keeps_its_words_and_its_markup() {
        let out = to_djot("html", "<p>A <b>strong</b> word.</p>", "a scene");
        assert!(out.notice.is_none());
        assert!(out.djot.contains("strong"), "{}", out.djot);
        assert!(!out.djot.trim().is_empty(), "an html body must not vanish");
    }

    /// Converting twice is the other half of the same mistake: Djot's `*x*` is
    /// strong, Markdown's is emphasis, so a second pass demotes bold to italic.
    #[test]
    fn converting_an_html_body_once_keeps_bold_bold() {
        let once = to_djot("html", "<p><b>Bold</b></p>", "a scene").djot;
        assert!(once.contains("*Bold*"), "Djot strong: {once}");
        let twice = to_djot("md", &once, "a scene").djot;
        assert!(
            twice.contains("_Bold_"),
            "a second pass demotes it, which is why there is only ever one: {twice}"
        );
    }

    /// Pre-0.3.0 plain types are read as Markdown, exactly as Manuskript's own
    /// coercion to `md` does.
    #[test]
    fn the_other_legacy_types_are_read_as_markdown() {
        for declared in ["txt", "t2t", "md", ""] {
            let out = to_djot(declared, "Plain words.", "a scene");
            assert!(
                out.djot.contains("Plain words."),
                "{declared}: {}",
                out.djot
            );
            assert!(out.notice.is_none());
        }
    }

    #[test]
    fn an_empty_body_converts_to_nothing_and_says_nothing() {
        for declared in ["md", "html"] {
            let out = to_djot(declared, "   \n ", "a scene");
            assert_eq!(out.djot, "");
            assert!(out.notice.is_none());
        }
    }
}
