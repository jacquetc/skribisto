// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The two RTF shapes Scrivener writes, for the fixtures' documents.
//!
//! Scrivener for Mac writes Cocoa RTF: a flat font table, `\cocoartf` in the
//! header, and a backslash before a line break ending each paragraph. Scrivener
//! for Windows (and the Linux beta) writes through Qt: `\uc1\deff0` in the
//! header, every font declared `\fmodern`, each run in a group of its own, and
//! `\par` between paragraphs. A project touched on both platforms holds both, so
//! the builders pick the shape per document from the project's platform.
//!
//! Text is written as ASCII; anything else is escaped as `\'hh` in code page
//! 1252 or, beyond it, as a Unicode escape built at run time (never spelled out
//! in this file).

/// Which writer a document's RTF imitates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dialect {
    Cocoa,
    Qt,
}

/// Escape one paragraph's text for RTF.
pub fn escape(text: &str) -> String {
    let mut out = String::new();
    for c in text.chars() {
        match c {
            '\\' | '{' | '}' => {
                out.push('\\');
                out.push(c);
            }
            ' '..='~' => out.push(c),
            _ => {
                let code = c as u32;
                if (0xA0..=0xFF).contains(&code) {
                    out.push_str(&format!("\\'{code:02x}"));
                } else {
                    // A Unicode escape: backslash, "u", the code as a signed
                    // 16-bit number, and a fallback character.
                    let value = if code > 0x7FFF {
                        code as i64 - 0x1_0000
                    } else {
                        code as i64
                    };
                    out.push('\\');
                    out.push('u');
                    out.push_str(&format!("{value}?"));
                }
            }
        }
    }
    out
}

/// A document of `paragraphs` in the given dialect.
pub fn document(dialect: Dialect, paragraphs: &[String]) -> String {
    match dialect {
        Dialect::Cocoa => cocoa(paragraphs),
        Dialect::Qt => qt(paragraphs),
    }
}

fn cocoa(paragraphs: &[String]) -> String {
    let mut out = String::from(
        "{\\rtf1\\ansi\\ansicpg1252\\cocoartf2761\n\\cocoatextscaling0\\cocoaplatform0{\\fonttbl\\f0\\fswiss\\fcharset0 Helvetica;}\n{\\colortbl;\\red255\\green255\\blue255;}\n{\\*\\expandedcolortbl;;}\n\\pard\\tx720\\tx1440\\tx2160\\pardirnatural\\partightenfactor0\n\n\\f0\\fs24 \\cf0 ",
    );
    let body: Vec<String> = paragraphs.iter().map(|p| escape(p)).collect();
    out.push_str(&body.join("\\\n"));
    out.push('}');
    out
}

fn qt(paragraphs: &[String]) -> String {
    let mut out = String::from(
        "{\\rtf1\\ansi\\ansicpg1252\\uc1\\deff0\n{\\fonttbl{\\f0\\fmodern\\fcharset0\\fprq2 Cochineal;}}\n{\\colortbl;\\red0\\green0\\blue0;\\red255\\green255\\blue255;}\n\\paperw12240\\paperh15840\\margl1800\\margr1800\\margt1440\\margb1440\\fet2\\ftnbj\\aenddoc\n\\pgnrestart\\pgnstarts0\n",
    );
    for (i, paragraph) in paragraphs.iter().enumerate() {
        if i > 0 {
            out.push_str("\\par");
        }
        out.push_str("\\pard\\plain \\ltrch\\loch {\\f0\\fs24\\b0\\i0 ");
        out.push_str(&escape(paragraph));
        out.push_str("}\n");
    }
    out.push_str("\\par}");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn braces_and_backslashes_are_escaped() {
        assert_eq!(escape("a{b}\\c"), "a\\{b\\}\\\\c");
    }

    #[test]
    fn each_dialect_starts_as_its_writer_does() {
        let paras = vec!["One.".to_string(), "Two.".to_string()];
        assert!(
            document(Dialect::Cocoa, &paras).starts_with("{\\rtf1\\ansi\\ansicpg1252\\cocoartf")
        );
        assert!(document(Dialect::Qt, &paras).contains("\\uc1\\deff0"));
        assert!(document(Dialect::Qt, &paras).contains("\\par"));
    }
}
