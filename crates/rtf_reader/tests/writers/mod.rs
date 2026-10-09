// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Test-only RTF writers: one small model of styled paragraphs, written the way three real
//! writers write it, so a round trip through the reader can be compared with what went in.
//!
//! * **Cocoa** (macOS, Scrivener for Mac): formatting switched inline without groups,
//!   paragraphs ended by a backslash before a line feed, `\uc0` once and then bare `\uN`,
//!   characters of code page 1252 as `\'hh`, astral characters as two positive surrogates.
//! * **Windows** (Scrivener for Windows, Qt): `\uc1`, one group per run with every toggle
//!   stated, `\par` at the start of the next line, `\uN` followed by the code page byte, and
//!   values above 32767 written negative.
//! * **Word**: `\uN?` fallbacks, a group per run naming only what is on, `\par`, and raw line
//!   breaks dropped in the middle of the text, as a writer wrapping long lines does.

#![allow(dead_code)]

/// A backslash, kept apart from the letters after it: see `common` for why.
const BACKSLASH: char = '\\';

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Style {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strike: bool,
    /// 0 baseline, 1 superscript, 2 subscript.
    pub vertical: u8,
}

/// One paragraph: runs of text, each with its style. Never empty: an empty paragraph at the
/// end of a story has no mark to end it in two of the three writers.
pub type Paragraph = Vec<(String, Style)>;

/// The code page 1252 byte for `c`, when it has one above 0x7F.
fn cp1252(c: char) -> Option<u8> {
    let mut buffer = [0u8; 4];
    let (bytes, _, errors) = encoding_rs::WINDOWS_1252.encode(c.encode_utf8(&mut buffer));
    if errors || bytes.len() != 1 {
        return None;
    }
    bytes.first().copied().filter(|b| *b >= 0x80)
}

/// A control word followed by the space that ends it.
fn word(out: &mut String, name: &str) {
    out.push(BACKSLASH);
    out.push_str(name);
    out.push(' ');
}

/// `\uN` with `N` as given.
fn unicode(out: &mut String, value: i32) {
    out.push(BACKSLASH);
    out.push('u');
    out.push_str(&value.to_string());
}

/// The UTF-16 units of `c`.
fn units(c: char) -> Vec<u16> {
    let mut buffer = [0u16; 2];
    c.encode_utf16(&mut buffer).to_vec()
}

/// Text every writer writes the same way: ASCII, with the three characters RTF reserves
/// escaped. `None` for a character the writer must spell its own way.
fn plain(out: &mut String, c: char) -> bool {
    match c {
        '\\' | '{' | '}' => {
            out.push(BACKSLASH);
            out.push(c);
            true
        }
        '\t' => {
            word(out, "tab");
            true
        }
        ' '..='~' => {
            out.push(c);
            true
        }
        _ => false,
    }
}

fn toggles(out: &mut String, from: Style, to: Style) {
    let pairs = [
        (from.bold, to.bold, "b", "b0"),
        (from.italic, to.italic, "i", "i0"),
        (from.underline, to.underline, "ul", "ulnone"),
        (from.strike, to.strike, "strike", "strike0"),
    ];
    for (was, is, on, off) in pairs {
        if was != is {
            word(out, if is { on } else { off });
        }
    }
    if from.vertical != to.vertical {
        word(
            out,
            match to.vertical {
                1 => "super",
                2 => "sub",
                _ => "nosupersub",
            },
        );
    }
}

pub fn cocoa(paragraphs: &[Paragraph]) -> Vec<u8> {
    let mut out = String::from(
        "{\\rtf1\\ansi\\ansicpg1252\\cocoartf2822\n\\cocoatextscaling0\\cocoaplatform0{\\fonttbl\\f0\\fswiss\\fcharset0 Helvetica;}\n{\\colortbl;\\red255\\green255\\blue255;}\n{\\*\\expandedcolortbl;;}\n\\pard\\tx560\\pardirnatural\\partightenfactor0\n\n\\f0\\fs24 \\cf0 ",
    );
    word(&mut out, "uc0");
    let mut current = Style::default();
    for paragraph in paragraphs {
        for (text, style) in paragraph {
            toggles(&mut out, current, *style);
            current = *style;
            for c in text.chars() {
                if plain(&mut out, c) {
                    continue;
                }
                match cp1252(c) {
                    Some(byte) => out.push_str(&format!("\\'{byte:02x}")),
                    None => {
                        for unit in units(c) {
                            unicode(&mut out, i32::from(unit));
                            out.push(' ');
                        }
                    }
                }
            }
        }
        out.push_str("\\\n");
    }
    out.push('}');
    out.into_bytes()
}

pub fn windows(paragraphs: &[Paragraph]) -> Vec<u8> {
    let mut out = String::from(
        "{\\rtf1\\ansi\\ansicpg1252\\uc1\\deff0{\\fonttbl{\\f0\\fmodern\\fcharset0\\fprq2 Cambria;}}\n{\\colortbl;\\red0\\green0\\blue0;}\n\\pard\\plain ",
    );
    for (index, paragraph) in paragraphs.iter().enumerate() {
        if index > 0 {
            out.push_str("\n\\par\\plain ");
        }
        for (text, style) in paragraph {
            out.push_str(&format!(
                "{{\\f0\\fs24\\b{}\\i{}",
                u8::from(style.bold),
                u8::from(style.italic)
            ));
            out.push_str(if style.underline { "\\ul" } else { "\\ul0" });
            out.push_str(if style.strike {
                "\\strike"
            } else {
                "\\strike0"
            });
            out.push_str(match style.vertical {
                1 => "\\super ",
                2 => "\\sub ",
                _ => "\\nosupersub ",
            });
            for c in text.chars() {
                if plain(&mut out, c) {
                    continue;
                }
                let fallback = cp1252(c).unwrap_or(b'?');
                for unit in units(c) {
                    out.push_str("\\loch\\af0\\hich\\af0\\dbch\\af0\\uc1");
                    // Values above 32767 as the signed 16-bit numbers the specification
                    // describes.
                    unicode(&mut out, i32::from(unit as i16));
                    out.push_str(&format!("\\'{fallback:02x}"));
                }
            }
            out.push('}');
        }
    }
    out.push('}');
    out.into_bytes()
}

/// Word, with a raw line break after every `wrap` characters of output: readers ignore raw
/// line ends, and a writer wrapping its lines can put one anywhere.
pub fn word_processor(paragraphs: &[Paragraph], wrap: usize) -> Vec<u8> {
    let mut out = String::from(
        "{\\rtf1\\ansi\\ansicpg1252\\deff0\\deflang1033{\\fonttbl{\\f0\\froman\\fcharset0 Times New Roman;}}\r\n{\\*\\generator Word;}\\uc1\\pard\\plain\\f0\\fs24 ",
    );
    for paragraph in paragraphs {
        for (text, style) in paragraph {
            out.push('{');
            let mut open = String::new();
            for (on, name) in [
                (style.bold, "b"),
                (style.italic, "i"),
                (style.underline, "ul"),
                (style.strike, "strike"),
                (style.vertical == 1, "super"),
                (style.vertical == 2, "sub"),
            ] {
                if on {
                    open.push(BACKSLASH);
                    open.push_str(name);
                }
            }
            if !open.is_empty() {
                out.push_str(&open);
                out.push(' ');
            }
            let mut since_wrap = 0usize;
            for c in text.chars() {
                let before = out.len();
                if !plain(&mut out, c) {
                    for unit in units(c) {
                        unicode(&mut out, i32::from(unit));
                        out.push('?');
                    }
                }
                since_wrap += out.len() - before;
                if wrap > 0 && since_wrap >= wrap {
                    // Never inside an escape: always between whole characters.
                    out.push_str("\r\n");
                    since_wrap = 0;
                }
            }
            out.push('}');
        }
        out.push_str("\\par\r\n");
    }
    out.push('}');
    out.into_bytes()
}
