// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! RTF bytes to tokens.
//!
//! RTF is read as bytes, never as UTF-8: raw 8-bit text is legal RTF in the document's code
//! page, and a writer puts such bytes in real files (a macOS attachment marker, a stray
//! control byte). Every token consumes at least one byte, which is what bounds the reader's
//! work by the input's length.
//!
//! Lexing rules, from the RTF 1.9.1 specification (pages 7 to 10):
//!
//! * A control word is a backslash, ASCII letters, an optional signed number, and an
//!   optional space that belongs to it. The number is saturated, never overflowed: a writer
//!   bug or a crafted `\u` with eleven digits must not fail the document.
//! * A raw carriage return or line feed is not text. After a backslash it is a paragraph
//!   mark, which is how macOS writes every paragraph end.
//! * `\binN` is followed by N raw bytes that may hold braces and backslashes.
//! * A NUL byte is not text either: RichEdit ends a document with `}\r\n\0`.

/// One token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Token<'a> {
    Open,
    Close,
    /// A control word and its parameter, saturated to the `i32` range.
    Word {
        name: &'a [u8],
        param: Option<i32>,
    },
    /// A control symbol: a backslash and one character that is not a letter.
    Symbol(u8),
    /// `\'hh`.
    Hex(u8),
    /// `\'` without two hexadecimal digits after it.
    BadHex,
    /// A backslash before a carriage return or a line feed: a paragraph mark.
    Par,
    /// A run of text bytes.
    Text(&'a [u8]),
    /// `\binN`: `declared` is N as written, `data` what the input held of it.
    Bin {
        declared: i64,
        data: &'a [u8],
    },
}

/// The lead bytes of a double-byte code page, whose next byte belongs to the same character
/// whatever it is. In raw (unescaped) text that byte can be a backslash or a brace, so it
/// must be taken before tokenising goes on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LeadBytes {
    /// Shift_JIS: 0x81 to 0x9F and 0xE0 to 0xFC.
    ShiftJis,
    /// GBK, Big5 and the Korean pages: 0x81 to 0xFE.
    Wide,
}

impl LeadBytes {
    fn is_lead(self, byte: u8) -> bool {
        match self {
            LeadBytes::ShiftJis => matches!(byte, 0x81..=0x9F | 0xE0..=0xFC),
            LeadBytes::Wide => matches!(byte, 0x81..=0xFE),
        }
    }
}

pub(crate) struct Lexer<'a> {
    input: &'a [u8],
    pos: usize,
    /// Tokens returned so far.
    pub(crate) tokens: usize,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a [u8], start: usize) -> Self {
        Lexer {
            input,
            pos: start.min(input.len()),
            tokens: 0,
        }
    }

    /// The next token, reading raw text with `lead` as the current code page's lead bytes.
    pub(crate) fn next(&mut self, lead: Option<LeadBytes>) -> Option<Token<'a>> {
        loop {
            let byte = *self.input.get(self.pos)?;
            let token = match byte {
                b'{' => {
                    self.pos += 1;
                    Token::Open
                }
                b'}' => {
                    self.pos += 1;
                    Token::Close
                }
                b'\r' | b'\n' | 0 => {
                    self.pos += 1;
                    continue;
                }
                b'\\' => self.control(),
                _ => self.text(lead),
            };
            self.tokens += 1;
            return Some(token);
        }
    }

    fn text(&mut self, lead: Option<LeadBytes>) -> Token<'a> {
        let start = self.pos;
        while let Some(&byte) = self.input.get(self.pos) {
            match byte {
                b'{' | b'}' | b'\\' | b'\r' | b'\n' | 0 => break,
                _ if lead.is_some_and(|l| l.is_lead(byte)) => {
                    self.pos = (self.pos + 2).min(self.input.len());
                }
                _ => self.pos += 1,
            }
        }
        Token::Text(self.input.get(start..self.pos).unwrap_or_default())
    }

    fn control(&mut self) -> Token<'a> {
        // The backslash.
        self.pos += 1;
        let Some(&first) = self.input.get(self.pos) else {
            // A backslash as the input's last byte stands for nothing.
            return Token::Symbol(0);
        };
        if first.is_ascii_alphabetic() {
            return self.word();
        }
        self.pos += 1;
        match first {
            b'\'' => {
                let digit = |b: Option<&u8>| b.and_then(|b| char::from(*b).to_digit(16));
                match (
                    digit(self.input.get(self.pos)),
                    digit(self.input.get(self.pos + 1)),
                ) {
                    (Some(high), Some(low)) => {
                        self.pos += 2;
                        // Two hexadecimal digits are at most 0xFF.
                        Token::Hex(u8::try_from(high * 16 + low).unwrap_or(u8::MAX))
                    }
                    _ => Token::BadHex,
                }
            }
            b'\r' | b'\n' => Token::Par,
            other => Token::Symbol(other),
        }
    }

    fn word(&mut self) -> Token<'a> {
        let start = self.pos;
        while self
            .input
            .get(self.pos)
            .is_some_and(|b| b.is_ascii_alphabetic())
        {
            self.pos += 1;
        }
        let name = self.input.get(start..self.pos).unwrap_or_default();

        let negative = self.input.get(self.pos) == Some(&b'-')
            && self
                .input
                .get(self.pos + 1)
                .is_some_and(|b| b.is_ascii_digit());
        if negative {
            self.pos += 1;
        }
        let mut value: i64 = 0;
        let mut has_digits = false;
        while let Some(digit) = self.input.get(self.pos).filter(|b| b.is_ascii_digit()) {
            has_digits = true;
            value = value
                .saturating_mul(10)
                .saturating_add(i64::from(digit - b'0'));
            self.pos += 1;
        }
        if negative {
            value = -value;
        }
        // The space that ends a control word is part of it.
        if self.input.get(self.pos) == Some(&b' ') {
            self.pos += 1;
        }

        if name == b"bin" {
            let declared = if has_digits { value } else { 0 };
            let wanted = usize::try_from(declared.max(0)).unwrap_or(usize::MAX);
            let end = self.pos.saturating_add(wanted).min(self.input.len());
            let data = self.input.get(self.pos..end).unwrap_or_default();
            self.pos = end;
            return Token::Bin { declared, data };
        }

        let param = has_digits.then(|| {
            i32::try_from(value.clamp(i64::from(i32::MIN), i64::from(i32::MAX))).unwrap_or_default()
        });
        Token::Word { name, param }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Written apart from the digits after it: a backslash, a `u` and four hexadecimal digits
    /// in a source file are too easily taken for an escape by the tools that handle it.
    const BACKSLASH: char = '\\';

    fn tokens(input: &[u8]) -> Vec<Token<'_>> {
        let mut lexer = Lexer::new(input, 0);
        std::iter::from_fn(|| lexer.next(None)).collect()
    }

    #[test]
    fn a_control_word_takes_its_parameter_and_one_space() {
        assert_eq!(
            tokens(br"\b0 x\fs-2y"),
            vec![
                Token::Word {
                    name: b"b",
                    param: Some(0)
                },
                Token::Text(b"x"),
                Token::Word {
                    name: b"fs",
                    param: Some(-2)
                },
                Token::Text(b"y"),
            ]
        );
    }

    #[test]
    fn a_parameter_too_long_saturates() {
        assert_eq!(
            tokens(format!("{}u99999999999 ", BACKSLASH).as_bytes()),
            vec![Token::Word {
                name: b"u",
                param: Some(i32::MAX)
            }]
        );
        assert_eq!(
            tokens(format!("{}u-99999999999 ", BACKSLASH).as_bytes()),
            vec![Token::Word {
                name: b"u",
                param: Some(i32::MIN)
            }]
        );
    }

    #[test]
    fn a_word_ends_at_any_non_letter_non_digit() {
        assert_eq!(
            tokens(br"\i0.\b0,"),
            vec![
                Token::Word {
                    name: b"i",
                    param: Some(0)
                },
                Token::Text(b"."),
                Token::Word {
                    name: b"b",
                    param: Some(0)
                },
                Token::Text(b","),
            ]
        );
    }

    #[test]
    fn a_backslash_before_a_line_end_is_a_paragraph_mark() {
        assert_eq!(
            tokens(b"one\\\ntwo\\\r\nthree"),
            vec![
                Token::Text(b"one"),
                Token::Par,
                Token::Text(b"two"),
                Token::Par,
                Token::Text(b"three"),
            ]
        );
    }

    #[test]
    fn raw_line_ends_and_nul_are_not_text() {
        assert_eq!(
            tokens(b"a\r\nb\0"),
            vec![Token::Text(b"a"), Token::Text(b"b")]
        );
    }

    #[test]
    fn hex_escapes_and_their_truncated_forms() {
        assert_eq!(tokens(br"\'e9"), vec![Token::Hex(0xE9)]);
        assert_eq!(tokens(br"\'A0"), vec![Token::Hex(0xA0)]);
        assert_eq!(
            tokens(br"\'a}"),
            vec![Token::BadHex, Token::Text(b"a"), Token::Close]
        );
        assert_eq!(tokens(br"\'"), vec![Token::BadHex]);
    }

    #[test]
    fn bin_data_may_hold_braces_and_is_cut_at_the_input_end() {
        assert_eq!(
            tokens(br"\bin3 {\}x"),
            vec![
                Token::Bin {
                    declared: 3,
                    data: br"{\}"
                },
                Token::Text(b"x")
            ]
        );
        assert_eq!(
            tokens(br"\bin99 ab"),
            vec![Token::Bin {
                declared: 99,
                data: b"ab"
            }]
        );
    }

    #[test]
    fn a_double_byte_character_keeps_a_trail_byte_that_looks_like_a_backslash() {
        // 0x83 0x5C is one Shift_JIS character whose second byte is a backslash.
        let input = [0x83u8, b'\\', b'x'];
        let mut lexer = Lexer::new(&input, 0);
        assert_eq!(
            lexer.next(Some(LeadBytes::ShiftJis)),
            Some(Token::Text(&input[..]))
        );
    }

    #[test]
    fn a_lone_backslash_at_the_end_stands_for_nothing() {
        assert_eq!(tokens(b"a\\"), vec![Token::Text(b"a"), Token::Symbol(0)]);
    }
}
