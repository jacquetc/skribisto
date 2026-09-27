// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Markdown and HTML ceilings, and the converters that honour them.
//!
//! Every conversion here runs on a thread with the 2 MiB stack a long operation
//! gets, in the debug build the suite runs in. A stack overflow is not a panic, so
//! a regression does not fail a test: it aborts the test binary, which is the
//! signal. Before the ceilings, `markdown_to_djot_and_text` aborted such a thread
//! at 477 nested blockquotes and `markdown_to_html` before 300, and
//! `html_to_djot_and_text` returned nothing, without a word of warning, for a
//! paragraph under 200 nested `<div>`s.

use super::*;
use crate::convert::{
    ConvertedDjot, html_to_djot_and_text, markdown_to_djot_and_text, markdown_to_html,
};
use proptest::prelude::*;

/// The stack `std::thread::spawn` gives a long operation's worker by default.
const LONG_OPERATION_STACK: usize = 2 << 20;

/// Run `work` on a thread with a long operation's stack.
fn on_a_long_operation_stack<T: Send + 'static>(work: impl FnOnce() -> T + Send + 'static) -> T {
    std::thread::Builder::new()
        .stack_size(LONG_OPERATION_STACK)
        .spawn(work)
        .expect("spawn the conversion thread")
        .join()
        .expect("the conversion must not unwind")
}

fn markdown_on_a_long_operation_stack(markdown: String) -> ConvertedDjot {
    on_a_long_operation_stack(move || markdown_to_djot_and_text(&markdown)).expect("the conversion")
}

fn html_on_a_long_operation_stack(html: String) -> ConvertedDjot {
    on_a_long_operation_stack(move || html_to_djot_and_text(&html)).expect("the conversion")
}

/// `levels` blockquotes, one inside the other, around a paragraph.
fn quoted_markdown(levels: usize) -> String {
    format!("Opening words.\n\n{}Deep words.\n", "> ".repeat(levels))
}

/// `levels` of `open`, a paragraph inside the innermost, then the matching `close`s.
fn nested_html(open: &str, close: &str, levels: usize) -> String {
    format!(
        "<p>Opening words.</p>{}<p>Deep words.</p>{}",
        open.repeat(levels),
        close.repeat(levels)
    )
}

// ---------------------------------------------------------------------------
// Markdown
// ---------------------------------------------------------------------------

#[test]
fn ordinary_markdown_passes() {
    for text in [
        "# A chapter\n\nThe ferry was *late*.\n\n* * *\n\nShe waited.\n",
        "> He said it plainly.\n>\n> Then he left.\n",
        "- one\n  - two\n    - three\n      - four\n",
        "1. First\n   1. Inside\n      1. Deeper\n",
        "- [ ] a task\n- [x] a done one\n",
        "A note.[^1]\n\n[^1]: The note.\n",
        "| a | b |\n|---|---|\n| c | d |\n",
        "```\n    indented code, with > and - in it\n```\n",
        "",
    ] {
        assert!(check_markdown(text).is_ok(), "should accept: {text:?}");
    }
}

/// Every container a Markdown line can open, repeated on one line: at the ceiling
/// it passes, one more is refused, and the refusal names the line.
#[test]
fn every_container_a_markdown_line_can_open_is_counted_to_the_ceiling() {
    for marker in [
        "> ", ">", "- ", "* ", "+ ", "1. ", "7) ", "[^a]: ", "- [ ] ",
    ] {
        let at = format!("{}deep\n", marker.repeat(MAX_MARKDOWN_DEPTH));
        assert!(check_markdown(&at).is_ok(), "{marker:?} at the ceiling");
        let past = format!("Before.\n\n{}deep\n", marker.repeat(MAX_MARKDOWN_DEPTH + 1));
        let refused = check_markdown(&past).expect_err(marker);
        assert_eq!(refused.line, 3, "{marker:?}");
        assert!(
            refused.depth > MAX_MARKDOWN_DEPTH,
            "{marker:?}: {refused:?}"
        );
    }
}

/// A thematic break is a run of one mark only. Mixed, the marks are bullets, and a
/// run of them nests a list item inside each.
#[test]
fn only_matching_marks_make_a_thematic_break() {
    let breaks = format!("{}\n{}\n", "- ".repeat(500), "* ".repeat(500));
    assert!(check_markdown(&breaks).is_ok());
    let bullets = format!("{}* * *\n", "- ".repeat(MAX_MARKDOWN_DEPTH + 1));
    assert!(check_markdown(&bullets).is_err());
}

/// A list item continues on lines indented past its marker, so every column of
/// indentation in front of a marker may be one more level; a tab reaches four.
#[test]
fn indentation_before_a_marker_counts_a_level_per_column() {
    let spaces = format!("{}- deep\n", " ".repeat(MAX_MARKDOWN_DEPTH - 1));
    assert!(check_markdown(&spaces).is_ok());
    let spaces = format!("{}- deep\n", " ".repeat(MAX_MARKDOWN_DEPTH));
    assert!(check_markdown(&spaces).is_err());
    let tabs = format!("{}- deep\n", "\t".repeat(MAX_MARKDOWN_DEPTH / 4));
    assert!(check_markdown(&tabs).is_err(), "a tab counts four columns");
    // Indentation in front of text opens nothing.
    let text = format!("{}deep\n", " ".repeat(10 * MAX_MARKDOWN_DEPTH));
    assert!(check_markdown(&text).is_ok());
}

/// What a converter reads in place of a refused document: every word, a line to a
/// paragraph, with nothing left to nest.
#[test]
fn markdown_without_nesting_keeps_every_word_and_nothing_else() {
    let deep = format!(
        "{}- [ ] *First* words\n{}1. second words\n\n    indented words\n| cell | words |\n",
        "> ".repeat(300),
        "  ".repeat(300)
    );
    let flat = markdown_without_nesting(&deep);
    assert_eq!(
        flat,
        "*First* words\n\nsecond words\n\nindented words\n\n| cell | words |"
    );
    for line in flat.split('\n') {
        assert!(
            crate::djot_depth::line_start(line, crate::djot_depth::Grammar::Markdown, 1).containers
                <= 1,
            "{line:?} opens nothing but a table row"
        );
    }
}

/// The at-ceiling document converts as it is, from a long operation's stack; one
/// level more, and five thousand, keep their words as plain text and say so.
#[test]
fn markdown_at_and_past_the_ceiling_converts_from_a_two_mebibyte_thread() {
    let at = markdown_on_a_long_operation_stack(quoted_markdown(MAX_MARKDOWN_DEPTH));
    assert!(!at.flattened, "at the ceiling the markup is kept");
    assert!(
        at.djot.contains(&"> ".repeat(MAX_MARKDOWN_DEPTH)),
        "{:?}",
        at.djot
    );
    assert!(at.text.contains("Deep words."));
    assert!(crate::djot_depth::check(&at.djot).is_ok());

    for levels in [MAX_MARKDOWN_DEPTH + 1, 500, 5_000] {
        let past = markdown_on_a_long_operation_stack(quoted_markdown(levels));
        assert!(past.flattened, "{levels}: the writer is told");
        assert_eq!(past.text, "Opening words.\nDeep words.", "{levels}");
        assert!(crate::djot_depth::check(&past.djot).is_ok(), "{levels}");
    }
}

#[test]
fn markdown_to_html_past_the_ceiling_keeps_its_words_from_a_two_mebibyte_thread() {
    let at = on_a_long_operation_stack(|| markdown_to_html(&quoted_markdown(MAX_MARKDOWN_DEPTH)))
        .expect("convert");
    assert!(at.contains("<blockquote"), "{at:.200}");
    assert!(at.contains("Deep words."));
    for levels in [MAX_MARKDOWN_DEPTH + 1, 300, 5_000] {
        let past = on_a_long_operation_stack(move || markdown_to_html(&quoted_markdown(levels)))
            .expect("convert");
        assert_eq!(past, "<p>Opening words.</p><p>Deep words.</p>", "{levels}");
    }
    let escaped = on_a_long_operation_stack(|| {
        markdown_to_html(&format!("{}a < b & c", "> ".repeat(MAX_MARKDOWN_DEPTH + 1)))
    })
    .expect("convert");
    assert_eq!(escaped, "<p>a &lt; b &amp; c</p>");
}

// ---------------------------------------------------------------------------
// HTML
// ---------------------------------------------------------------------------

/// A Qt rich-text document, the shape Plume, Manuskript and older Skribisto
/// projects hold, with a hundred paragraphs.
fn qt_document(paragraphs: usize) -> String {
    format!(
        "<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 4.0//EN\" \"http://www.w3.org/TR/REC-html40/strict.dtd\">\n\
         <html><head><meta name=\"qrichtext\" content=\"1\" /><title>A title</title>\
         <style type=\"text/css\">\np, li {{ white-space: pre-wrap; }}\n</style></head>\
         <body style=\" font-family:'Sans'; font-size:10pt;\">\n{}\
         <table border=\"1\"><tr><td><p>In a cell.</p></td></tr></table>\
         <ul><li>One</li><li>Two<br />wrapped</li></ul></body></html>",
        "<p style=\" margin-top:0px;\"><span style=\" font-weight:600;\">Bold</span> and \
         <a href=\"x.html?a=1&amp;b=2\">a link</a>.</p>\n"
            .repeat(paragraphs)
    )
}

#[test]
fn ordinary_html_passes() {
    for html in [
        qt_document(1_000),
        "<p>a</p>".repeat(10_000),
        format!("<ul>{}</ul>", "<li>unclosed item".repeat(10_000)),
        format!("<div>{}</div>", "<p>unclosed paragraph".repeat(10_000)),
        format!("<p>{}</p>", "word<br>".repeat(10_000)),
        format!("<table>{}</table>", "<tr><td>a<td>b".repeat(10_000)),
    ] {
        assert!(check_html(&html).is_ok(), "should accept: {html:.80}");
    }
}

/// At the ceiling it passes, one more element is refused, naming the line, for
/// block elements, inline ones, lists, quotations and unknown elements alike.
#[test]
fn nested_elements_are_counted_to_the_ceiling() {
    for (open, close) in [
        ("<div>", "</div>"),
        ("<span>", "</span>"),
        ("<blockquote>", "</blockquote>"),
        ("<b>", "</b>"),
        ("<x-y>", "</x-y>"),
        ("<section>", "</section>"),
    ] {
        let levels = |n: usize| format!("{}words{}", open.repeat(n), close.repeat(n));
        assert!(check_html(&levels(MAX_HTML_DEPTH)).is_ok(), "{open}");
        let past = format!("<p>a</p>\n\n{}", levels(MAX_HTML_DEPTH + 1));
        let refused = check_html(&past).expect_err(open);
        assert_eq!(refused.line, 3, "{open}");
        assert!(refused.depth > MAX_HTML_DEPTH, "{open}: {refused:?}");
    }
    // A list item is a level of its own.
    let lists = |n: usize| format!("{}words{}", "<ul><li>".repeat(n), "</li></ul>".repeat(n));
    assert!(check_html(&lists(MAX_HTML_DEPTH / 2)).is_ok());
    assert!(check_html(&lists(MAX_HTML_DEPTH / 2 + 1)).is_err());
    // A table counts for the body and row the tree builder adds inside it.
    let tables = |n: usize| format!("{}words", "<table><td>".repeat(n));
    assert!(check_html(&tables(MAX_HTML_DEPTH / 4)).is_ok());
    assert!(check_html(&tables(MAX_HTML_DEPTH / 4 + 1)).is_err());
}

/// An end tag closes only what it names, so a stray or misnested one lowers
/// nothing; neither does a start tag that pretends to close itself.
#[test]
fn only_an_end_tag_naming_the_last_open_element_closes_it() {
    let stray = format!("{}words", "<div></span>".repeat(MAX_HTML_DEPTH + 1));
    assert!(check_html(&stray).is_err());
    let self_closing = format!("{}words", "<div/>".repeat(MAX_HTML_DEPTH + 1));
    assert!(check_html(&self_closing).is_err());
    let misnested = format!("{}words", "<b><i></b>".repeat(MAX_HTML_DEPTH));
    assert!(check_html(&misnested).is_err());
}

/// Tags are found where the tokenizer finds them, and nowhere else: a comment, an
/// attribute value and the text of a raw text element hide what is inside them, and
/// nothing after them.
#[test]
fn tags_are_found_where_the_tokenizer_finds_them() {
    let deep = "<div>".repeat(MAX_HTML_DEPTH + 1);
    let hidden = [
        format!("<!-- {deep} -->words"),
        format!("<p title=\"{deep}\">words</p>"),
        format!("<p title='{deep}'>words</p>"),
        format!("<textarea>{deep}</textarea>words"),
        format!("<title>{deep}</title>words"),
        "<p><![CDATA[ x ]]>words</p>".to_string(),
    ];
    for html in hidden {
        assert!(check_html(&html).is_ok(), "hidden: {html:.60}");
    }
    let visible = [
        // A comment closed at once, by `<!-->` or `<!--->`.
        format!("<!-->{deep}"),
        format!("<!--->{deep}"),
        // A quote inside an attribute name opens nothing.
        format!("<p a\"b>{deep}\"</p>"),
        // An unquoted value ends at the `>`.
        format!("<p a=b>{deep}"),
        // A comment inside raw text is text: the element ends at its own end tag.
        format!("<textarea><!--</textarea>{deep}-->"),
        format!("<title><p title=\"</title>{deep}\">"),
        // A CDATA section outside foreign content is a bogus comment, ended by `>`.
        format!("<![CDATA[>{deep}]]>"),
    ];
    for html in visible {
        assert!(check_html(&html).is_err(), "visible: {html:.60}");
    }
}

/// Where the tokenizer's reading depends on the tree builder, every tag counts and
/// none closes.
#[test]
fn after_svg_math_select_or_script_every_tag_counts_and_none_closes() {
    for opener in ["<svg>", "<math>", "<select>"] {
        let html = format!("{opener}{}", "<g></g>".repeat(MAX_HTML_DEPTH));
        assert!(check_html(&html).is_err(), "{opener}");
        let commented = format!("{opener}{}", "<!-- <g> -->".repeat(MAX_HTML_DEPTH));
        assert!(
            check_html(&commented).is_err(),
            "{opener}: even inside a comment"
        );
    }
    // The converter removes `<script>` blocks before the scan, and so does the public
    // check, so only HTML that reached the scan some other way can hold one.
    let script = format!("<script>{}</script>words", "<g></g>".repeat(MAX_HTML_DEPTH));
    assert!(check_html(&script).is_ok());
    assert!(check_clean_html(&script).is_err());
}

/// What a converter reads in place of a refused document: every word, a block to a
/// paragraph, with nothing left to nest.
#[test]
fn html_without_nesting_keeps_every_word_and_nothing_else() {
    let deep = format!(
        "<title>Not prose</title>{}<p>First <b>words</b> &amp; more</p><div>second<br>third</div>\
         <noscript>hidden</noscript><textarea>kept a &lt; b</textarea>{}",
        "<blockquote><div>".repeat(300),
        "</div></blockquote>".repeat(300)
    );
    let flat = html_without_nesting(&deep);
    assert_eq!(
        flat,
        "<p>First words &amp; more</p>\n<p>second</p>\n<p>third</p>\n<p>kept a &lt; b</p>\n"
    );
    assert!(check_html(&flat).is_ok());
}

/// The at-ceiling document converts as it is; one element more, and thousands,
/// keep every word as plain text and say so. Before the ceiling, a paragraph under
/// 200 nested `<div>`s or lists converted to nothing at all.
#[test]
fn html_at_and_past_the_ceiling_converts_from_a_two_mebibyte_thread() {
    let at = html_on_a_long_operation_stack(nested_html(
        "<blockquote>",
        "</blockquote>",
        MAX_HTML_DEPTH - 1,
    ));
    assert!(!at.flattened, "at the ceiling the markup is kept");
    assert!(at.djot.contains("> > > "), "{:.200}", at.djot);
    assert_eq!(at.text, "Opening words.\nDeep words.");
    // Tables nested in cells, with the rows spelled out: five levels each.
    let tables = html_on_a_long_operation_stack(nested_html(
        "<table><tr><td>",
        "</td></tr></table>",
        (MAX_HTML_DEPTH - 1) / 5,
    ));
    assert_eq!(tables.text, "Opening words.\nDeep words.");

    for (open, close) in [
        ("<blockquote>", "</blockquote>"),
        ("<div>", "</div>"),
        ("<ul><li>", "</li></ul>"),
        ("<span>", "</span>"),
        ("<table><tr><td>", "</td></tr></table>"),
    ] {
        for levels in [MAX_HTML_DEPTH + 1, 200, 5_000] {
            let past = html_on_a_long_operation_stack(nested_html(open, close, levels));
            assert!(past.flattened, "{open} × {levels}: the writer is told");
            assert_eq!(
                past.text, "Opening words.\nDeep words.",
                "{open} × {levels}"
            );
            assert!(crate::djot_depth::check(&past.djot).is_ok());
        }
    }
}

// ---------------------------------------------------------------------------
// Properties
// ---------------------------------------------------------------------------

/// A Markdown document of lines that open containers and end in a word of their
/// own: in some documents every line stays well within the ceiling, in others lines
/// come just either side of it, and in the rest they pass it many times over.
fn markdown_soup() -> impl Strategy<Value = (String, Vec<String>)> {
    let token = prop::sample::select(vec![
        "> ", ">", "- ", "* ", "+ ", "1. ", "2) ", "- [ ] ", " ", "  ", "\t",
    ]);
    prop_oneof![Just(40usize), Just(MAX_MARKDOWN_DEPTH / 2), Just(300)].prop_flat_map(move |max| {
        let line = prop::collection::vec(token.clone(), 0..max);
        prop::collection::vec((line, 1usize..6), 1..24).prop_map(|lines| {
            let mut text = String::new();
            let mut words = Vec::new();
            for (tokens, times) in lines {
                for _ in 0..times {
                    let word = format!("w{}", words.len());
                    text.push_str(&tokens.concat());
                    text.push_str(&word);
                    text.push_str("\n\n");
                    words.push(word);
                }
            }
            (text, words)
        })
    })
}

/// An HTML document of random elements, closed or not, misnested or not, with a
/// word of its own in every text run: in some documents they rarely nest, in others
/// runs of them repeat until they nest far past the ceiling.
fn html_soup() -> impl Strategy<Value = (String, Vec<String>)> {
    let piece = prop::sample::select(vec![
        "<div>",
        "</div>",
        "<p>",
        "</p>",
        "<span>",
        "</span>",
        "<b>",
        "</b>",
        "<i>",
        "</i>",
        "<em>",
        "<blockquote>",
        "</blockquote>",
        "<ul>",
        "</ul>",
        "<ol>",
        "<li>",
        "</li>",
        "<table>",
        "</table>",
        "<tr>",
        "<td>",
        "</td>",
        "<dl>",
        "<dd>",
        "<dt>",
        "<h2>",
        "</h2>",
        "<section>",
        "</section>",
        "<a href=\"x>y\">",
        "</a>",
        "<x-y>",
        "</x-y>",
        "<br>",
        "<hr>",
        "<img src=\"a.png\">",
        "<!-- <div> -->",
        "<pre>",
        "</pre>",
        "<strong>",
        "</strong>",
        "<u>",
        "</u>",
        "<title>x</title>",
        "<textarea>",
        "</textarea>",
        "<svg>",
        "<select>",
    ]);
    prop_oneof![
        (Just(4usize), Just(1usize)),
        (Just(8usize), Just(3usize)),
        (Just(40usize), Just(40usize)),
    ]
    .prop_flat_map(move |(pieces, repeats)| {
        prop::collection::vec(
            (prop::collection::vec(piece.clone(), 0..pieces), 0..repeats),
            1..60,
        )
        .prop_map(|runs| {
            let mut html = String::new();
            let mut words = Vec::new();
            for (pieces, repeat) in runs {
                for _ in 0..=repeat {
                    html.push_str(&pieces.concat());
                }
                let word = format!("w{}", words.len());
                html.push(' ');
                html.push_str(&word);
                html.push(' ');
                words.push(word);
            }
            (html, words)
        })
    })
}

/// The words of `text`, as whole words.
fn has_word(text: &str, word: &str) -> bool {
    text.split(|c: char| !c.is_alphanumeric())
        .any(|candidate| candidate == word)
}

proptest! {
    #![proptest_config(ProptestConfig { cases: 128, ..ProptestConfig::default() })]

    /// One line of `levels` Markdown container markers nests exactly that deep:
    /// accepted exactly when that is within the ceiling.
    #[test]
    fn a_line_of_markdown_containers_is_refused_exactly_past_the_ceiling(
        markers in prop::collection::vec(
            prop::sample::select(vec!["> ", ">", "- ", "* ", "+ ", "1. ", "3) ", "[^a]: "]),
            1..(2 * MAX_MARKDOWN_DEPTH)),
    ) {
        let levels = markers.len();
        let text = format!("{}deep\n", markers.concat());
        prop_assert_eq!(check_markdown(&text).is_ok(), levels <= MAX_MARKDOWN_DEPTH);
    }

    /// Whatever the nesting, the conversion comes back from a long operation's
    /// stack with every word, as markup when it could keep it and as plain text,
    /// reported, when it could not.
    #[test]
    fn markdown_keeps_every_word_from_a_two_mebibyte_thread((text, words) in markdown_soup()) {
        let passed = check_markdown(&text).is_ok();
        let converted = markdown_on_a_long_operation_stack(text);
        for word in &words {
            prop_assert!(has_word(&converted.text, word), "{} is missing", word);
        }
        if !passed {
            prop_assert!(converted.flattened);
        }
        prop_assert!(crate::djot_depth::check(&converted.djot).is_ok());
    }

    /// The same for HTML, including the words the parser used to drop without a
    /// word when they sat too deep.
    #[test]
    fn html_keeps_every_word_from_a_two_mebibyte_thread((html, words) in html_soup()) {
        let passed = check_html(&html).is_ok();
        let converted = html_on_a_long_operation_stack(html);
        for word in &words {
            prop_assert!(has_word(&converted.text, word), "{} is missing", word);
        }
        if !passed {
            prop_assert!(converted.flattened);
        }
        prop_assert!(crate::djot_depth::check(&converted.djot).is_ok());
    }
}
