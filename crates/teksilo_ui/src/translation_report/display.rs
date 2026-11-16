// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The display pass: from what a catalogue *contains* to what the screen *shows*.
//!
//! A writer reporting a translation types the words they saw. A catalogue holds more
//! than that, and a search over the raw source would miss exactly the strings people
//! report most:
//!
//! - **The mnemonic `&`.** `A&nnuler` is shown as "Annuler". `&&` is a literal `&`.
//! - **Invisible marks.** The Arabic and Hebrew catalogues carry 189 LRM/RLM marks
//!   between the words a reader sees.
//! - **Tooltip markup.** A rich tooltip is Markdown: `[scènes](:wm-scene)` shows as
//!   "scènes", and `*where you put it*` without its asterisks.
//! - **Placeables.** `{ $count } éléments` shows a number where the source has a
//!   variable, and `{"{"}`-style string literals show their content.
//! - **Selectors.** One message is several sentences, one per plural form.
//!
//! Two strings come out of each form: [`Form::shown`], what the result list prints (a
//! variable is shown as `{ $name }`, so the writer sees where the number goes), and
//! [`Form::search`], what the matcher folds (a variable is [`WILD`], typography is
//! flattened by [`normalize_for_search`], which the query goes through too).

use fluent_syntax::ast::{Expression, InlineExpression, Pattern, PatternElement};

/// Stands in for a variable placeable in [`Form::search`]: whatever the writer typed
/// in that position (a number, a name, another message) is accepted there.
///
/// A private-use code point, so it cannot occur in a catalogue or in a query.
pub(crate) const WILD: char = '\u{E000}';

/// How many sentences one message may expand to. Each select expression multiplies
/// the count; past this, the remaining selectors keep their default variant only.
const MAX_FORMS: usize = 32;

/// One sentence a message can show.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Form {
    /// As printed in the result list: marks and markup removed, a variable as
    /// `{ $name }`.
    pub shown: String,
    /// As matched: [`shown`](Self::shown) with each variable replaced by [`WILD`]
    /// and the typography flattened.
    pub search: String,
}

impl Form {
    /// Whether this form has a variable in it.
    pub fn has_wild(&self) -> bool {
        self.search.contains(WILD)
    }
}

/// What kind of markup the catalogue's text carries beyond Fluent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Markup {
    /// Plain labels and messages.
    Plain,
    /// A rich tooltip's Markdown: links and emphasis.
    Markdown,
}

/// One piece of a sentence before rendering.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Piece {
    Text(String),
    Variable(String),
}

/// Every sentence `pattern` can show, at most [`MAX_FORMS`].
pub(crate) fn forms(pattern: &Pattern<&str>, markup: Markup) -> Vec<Form> {
    let mut out: Vec<Form> = expand(pattern)
        .into_iter()
        .map(|pieces| render(&pieces, markup))
        .collect();
    // Two variants that read the same ("[one] { $n } fichier" and "[few] { $n }
    // fichier") are one sentence on screen.
    let mut seen = std::collections::HashSet::new();
    out.retain(|f| seen.insert(f.search.clone()));
    out
}

/// The sentences of a pattern, as piece lists. Selects multiply; once the product
/// would pass [`MAX_FORMS`], further selects keep only their default variant.
fn expand(pattern: &Pattern<&str>) -> Vec<Vec<Piece>> {
    let mut acc: Vec<Vec<Piece>> = vec![Vec::new()];
    for element in &pattern.elements {
        match element {
            PatternElement::TextElement { value } => {
                for sentence in &mut acc {
                    push_text(sentence, value);
                }
            }
            PatternElement::Placeable { expression } => {
                let alternatives = expand_expression(expression, acc.len());
                acc = product(acc, alternatives);
            }
        }
    }
    acc
}

/// The alternatives one placeable contributes. `already` is how many sentences the
/// pattern holds so far, which decides whether a select may still multiply them.
fn expand_expression(expression: &Expression<&str>, already: usize) -> Vec<Vec<Piece>> {
    match expression {
        Expression::Inline(inline) => expand_inline(inline, already),
        Expression::Select { variants, .. } => {
            let room = MAX_FORMS / already.max(1);
            let chosen: Vec<_> = if variants.len() <= room {
                variants.iter().collect()
            } else {
                variants.iter().filter(|v| v.default).collect()
            };
            chosen
                .into_iter()
                .flat_map(|variant| expand(&variant.value))
                .collect()
        }
    }
}

fn expand_inline(inline: &InlineExpression<&str>, already: usize) -> Vec<Vec<Piece>> {
    match inline {
        InlineExpression::StringLiteral { value } => {
            vec![vec![Piece::Text(unescape_literal(value))]]
        }
        InlineExpression::NumberLiteral { value } => vec![vec![Piece::Text(value.to_string())]],
        InlineExpression::VariableReference { id } => {
            vec![vec![Piece::Variable(id.name.to_string())]]
        }
        InlineExpression::Placeable { expression } => expand_expression(expression, already),
        // None of the shipped catalogues uses a function, a message or a term
        // reference; each would show text the catalogue line does not contain, so it
        // is a position anything may fill, like a variable.
        InlineExpression::FunctionReference { id, .. } => {
            vec![vec![Piece::Variable(id.name.to_string())]]
        }
        InlineExpression::MessageReference { id, .. } => {
            vec![vec![Piece::Variable(id.name.to_string())]]
        }
        InlineExpression::TermReference { id, .. } => {
            vec![vec![Piece::Variable(format!("-{}", id.name))]]
        }
    }
}

fn product(acc: Vec<Vec<Piece>>, alternatives: Vec<Vec<Piece>>) -> Vec<Vec<Piece>> {
    if alternatives.is_empty() {
        return acc;
    }
    let mut out = Vec::with_capacity(acc.len() * alternatives.len());
    for prefix in &acc {
        for alternative in &alternatives {
            let mut sentence = prefix.clone();
            for piece in alternative {
                match piece {
                    Piece::Text(t) => push_text(&mut sentence, t),
                    Piece::Variable(_) => sentence.push(piece.clone()),
                }
            }
            out.push(sentence);
        }
    }
    out
}

/// Append text, merging with a preceding text piece so markup split across a
/// placeable boundary is still seen whole.
fn push_text(sentence: &mut Vec<Piece>, text: &str) {
    if let Some(Piece::Text(last)) = sentence.last_mut() {
        last.push_str(text);
    } else {
        sentence.push(Piece::Text(text.to_string()));
    }
}

/// Fluent string literal escapes: `\\`, `\"`, `\uXXXX`, `\UXXXXXX`.
fn unescape_literal(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut chars = raw.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('u') => push_code_point(&mut out, &mut chars, 4),
            Some('U') => push_code_point(&mut out, &mut chars, 6),
            Some(other) => out.push(other),
            None => out.push('\\'),
        }
    }
    out
}

fn push_code_point(out: &mut String, chars: &mut std::str::Chars<'_>, digits: usize) {
    let hex: String = chars.by_ref().take(digits).collect();
    match u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32) {
        Some(c) => out.push(c),
        // A malformed escape is shown as written, which is what a reader would see.
        None => {
            out.push('\\');
            out.push_str(&hex);
        }
    }
}

/// Render a piece list into its two strings.
fn render(pieces: &[Piece], markup: Markup) -> Form {
    let mut shown = String::new();
    let mut search = String::new();
    // The mnemonic is the first single `&` of the whole label, wherever the
    // placeables fall, so the flag is carried across text pieces.
    let mut mnemonic_taken = false;
    for piece in pieces {
        match piece {
            Piece::Text(text) => {
                let text = strip_mnemonic(text, &mut mnemonic_taken);
                let text = strip_invisible(&text);
                let text = match markup {
                    Markup::Plain => text,
                    Markup::Markdown => strip_markdown(&text),
                };
                shown.push_str(&text);
                search.push_str(&text);
            }
            Piece::Variable(name) => {
                shown.push_str("{ $");
                shown.push_str(name);
                shown.push_str(" }");
                search.push(WILD);
            }
        }
    }
    Form {
        shown: collapse_spaces(&shown),
        search: normalize_for_search(&search),
    }
}

/// Remove the mnemonic marker: the first `&` followed by a character other than a
/// space or another `&`. `&&` is a literal ampersand wherever it stands.
fn strip_mnemonic(text: &str, taken: &mut bool) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '&' {
            out.push(c);
            continue;
        }
        match chars.peek() {
            Some('&') => {
                chars.next();
                out.push('&');
            }
            Some(next) if !*taken && !next.is_whitespace() => *taken = true,
            _ => out.push('&'),
        }
    }
    out
}

/// Whether `c` is a formatting mark a reader cannot see: bidi marks, embeddings and
/// isolates, zero-width characters, the soft hyphen and the byte-order mark.
fn is_invisible(c: char) -> bool {
    matches!(
        c,
        '\u{00AD}'
            | '\u{061C}'
            | '\u{200B}'..='\u{200F}'
            | '\u{202A}'..='\u{202E}'
            | '\u{2060}'..='\u{2064}'
            | '\u{2066}'..='\u{2069}'
            | '\u{FEFF}'
    )
}

fn strip_invisible(text: &str) -> String {
    text.chars().filter(|c| !is_invisible(*c)).collect()
}

/// The Markdown a rich tooltip renders: a link shows its label, emphasis its text.
fn strip_markdown(text: &str) -> String {
    let unlinked = strip_links(text);
    strip_emphasis(&unlinked)
}

/// `[label](target)` → `label`. A bracket not followed by a parenthesised target
/// is literal text.
fn strip_links(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(open) = rest.find('[') {
        out.push_str(&rest[..open]);
        let after = &rest[open + 1..];
        let link = after.find(']').and_then(|close| {
            let label = &after[..close];
            let tail = &after[close + 1..];
            let target_end = tail.strip_prefix('(')?.find(')')?;
            Some((label, &tail[target_end + 2..]))
        });
        match link {
            Some((label, remainder)) if !label.contains('[') => {
                out.push_str(label);
                rest = remainder;
            }
            _ => {
                out.push('[');
                rest = after;
            }
        }
    }
    out.push_str(rest);
    out
}

/// `**x**` and `*x*` → `x`, when the delimiters hug the text as Markdown requires.
/// An asterisk with a space on its inner side ("5 * 3") is literal.
fn strip_emphasis(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut keep = vec![true; chars.len()];
    let mut i = 0;
    while i < chars.len() {
        if chars[i] != '*' {
            i += 1;
            continue;
        }
        let run = chars[i..].iter().take_while(|c| **c == '*').count();
        let opens = chars.get(i + run).is_some_and(|c| !c.is_whitespace());
        if !opens {
            i += run;
            continue;
        }
        // The matching closing run of the same length, hugging the text.
        let mut j = i + run;
        let mut closed = None;
        while j < chars.len() {
            if chars[j] == '*' {
                let close_run = chars[j..].iter().take_while(|c| **c == '*').count();
                if close_run == run && !chars[j - 1].is_whitespace() {
                    closed = Some(j);
                    break;
                }
                j += close_run;
            } else {
                j += 1;
            }
        }
        match closed {
            Some(j) => {
                for k in 0..run {
                    keep[i + k] = false;
                    keep[j + k] = false;
                }
                i = j + run;
            }
            None => i += run,
        }
    }
    chars
        .into_iter()
        .zip(keep)
        .filter_map(|(c, k)| k.then_some(c))
        .collect()
}

/// Every run of white space as one space, trimmed. No-break and thin spaces are
/// white space too: French puts one before every `:` and inside every `« »`.
fn collapse_spaces(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// What both sides of a match go through: marks removed, typographic quotes,
/// apostrophes, dashes and the ellipsis flattened to what a keyboard types, and the
/// spacing collapsed. Case and diacritics are left alone: the matcher folds those,
/// per language.
pub(crate) fn normalize_for_search(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            c if is_invisible(c) => {}
            '\u{2018}' | '\u{2019}' | '\u{201A}' | '\u{201B}' | '\u{02BC}' | '\u{2032}' => {
                out.push('\'')
            }
            '\u{201C}' | '\u{201D}' | '\u{201E}' | '\u{201F}' | '\u{00AB}' | '\u{00BB}'
            | '\u{2039}' | '\u{203A}' | '\u{2033}' | '\u{300C}' | '\u{300D}' => out.push('"'),
            '\u{2010}'..='\u{2015}' | '\u{2212}' => out.push('-'),
            '\u{2026}' => out.push_str("..."),
            c => out.push(c),
        }
    }
    collapse_spaces(&out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use fluent_syntax::ast::Entry;

    /// The forms of the message `id` in `source`.
    fn forms_of(source: &str, markup: Markup) -> Vec<Form> {
        let resource = match fluent_syntax::parser::parse(source) {
            Ok(r) => r,
            Err((r, _)) => r,
        };
        resource
            .body
            .iter()
            .find_map(|e| match e {
                Entry::Message(m) => m.value.as_ref().map(|v| forms(v, markup)),
                _ => None,
            })
            .unwrap_or_default()
    }

    fn shown(source: &str) -> Vec<String> {
        forms_of(source, Markup::Plain)
            .into_iter()
            .map(|f| f.shown)
            .collect()
    }

    #[test]
    fn the_mnemonic_is_removed_and_a_double_ampersand_kept() {
        assert_eq!(shown("k = A&nnuler"), ["Annuler"]);
        assert_eq!(shown("k = Search && Replace"), ["Search & Replace"]);
        assert_eq!(shown("k = Fish & chips"), ["Fish & chips"]);
        // Only the first single ampersand is the mnemonic.
        assert_eq!(shown("k = &Save &as"), ["Save &as"]);
    }

    #[test]
    fn invisible_marks_are_removed() {
        assert_eq!(
            shown("k = \u{200F}مرحبا\u{200E} Skribisto\u{200F}"),
            ["مرحبا Skribisto"]
        );
    }

    #[test]
    fn a_variable_shows_as_itself_and_searches_as_a_wildcard() {
        let f = &forms_of("k = { $count } éléments", Markup::Plain)[0];
        assert_eq!(f.shown, "{ $count } éléments");
        assert_eq!(f.search, format!("{WILD} éléments"));
        assert!(f.has_wild());
    }

    #[test]
    fn a_string_literal_shows_its_content() {
        assert_eq!(shown(r#"k = a{", "}b"#), ["a, b"]);
        assert_eq!(shown(r#"k = {" "}:"#), [":"]);
        assert_eq!(shown(r#"k = {"{"}x{"}"}"#), ["{x}"]);
    }

    #[test]
    fn every_variant_of_a_select_is_a_form() {
        let f = shown("k = { $n ->\n    [one] { $n } fichier\n   *[other] { $n } fichiers\n}");
        assert_eq!(f, ["{ $n } fichier", "{ $n } fichiers"]);
    }

    #[test]
    fn two_variants_that_read_the_same_are_one_form() {
        let f = shown(
            "k = { $n ->\n    [one] { $n } plik\n    [few] { $n } plik\n   *[other] { $n } plików\n}",
        );
        assert_eq!(f, ["{ $n } plik", "{ $n } plików"]);
    }

    #[test]
    fn nested_selects_stop_multiplying_past_the_cap() {
        // 6 × 6 × 6 would be 216 sentences; past the cap only defaults are kept.
        let select =
            |v: &str| format!("{{ ${v} ->\n [a] a\n [b] b\n [c] c\n [d] d\n [e] e\n *[f] f\n}}");
        let source = format!("k = {} {} {}", select("x"), select("y"), select("z"));
        let f = forms_of(&source, Markup::Plain);
        assert!(f.len() <= MAX_FORMS, "{}", f.len());
        assert!(!f.is_empty());
    }

    #[test]
    fn tooltip_markdown_shows_its_text() {
        let f = forms_of(
            "k = A scene holds *your* writing and a [synopsis](:wm-synopsis), **always**.",
            Markup::Markdown,
        );
        assert_eq!(
            f[0].shown,
            "A scene holds your writing and a synopsis, always."
        );
    }

    #[test]
    fn a_literal_asterisk_or_bracket_survives_the_markdown_pass() {
        assert_eq!(strip_markdown("5 * 3 = 15"), "5 * 3 = 15");
        assert_eq!(strip_markdown("[draft] notes"), "[draft] notes");
        assert_eq!(strip_markdown("a [b](c) [d]"), "a b [d]");
    }

    #[test]
    fn markdown_is_left_alone_in_plain_catalogues() {
        assert_eq!(shown("k = Use *asterisks*"), ["Use *asterisks*"]);
    }

    #[test]
    fn spaces_quotes_dashes_and_the_ellipsis_flatten_for_search() {
        assert_eq!(
            normalize_for_search("«\u{202F}Annuler\u{202F}»\u{00A0}: l’élément — fin…"),
            "\" Annuler \" : l'élément - fin..."
        );
        assert_eq!(normalize_for_search("  a \t b  "), "a b");
    }

    #[test]
    fn the_shown_text_keeps_its_typography() {
        assert_eq!(shown("k = l’élément…"), ["l’élément…"]);
        assert_eq!(
            forms_of("k = l’élément…", Markup::Plain)[0].search,
            "l'élément..."
        );
    }
}
