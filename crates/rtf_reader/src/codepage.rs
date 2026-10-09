// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Code pages: which one a stretch of 8-bit text is in, and how to read it.
//!
//! A byte of text, raw or written `\'hh`, is a character in the code page of the font it is
//! written in (RTF 1.9.1, page 13): `\cpgN` when the font table states one, else the page
//! its `\fcharsetN` implies (page 19), else the document's `\ansicpgN`. Bytes are buffered
//! and decoded together, because a double-byte page spells one character with two `\'hh`.
//!
//! **Character set 0 means the document's page here, not always 1252.** macOS declares every
//! font `\fcharset0` and states the page it really wrote in `\ansicpgN`: a Russian system
//! writes `\ansicpg1251` and Cyrillic `\'hh` in `\fcharset0` fonts. Windows writers put
//! anything outside their page in `\uN` form, so reading character set 0 as the document's
//! page costs them nothing.
//!
//! `encoding_rs` reads the Windows pages, the double-byte ones, Mac Roman and Mac Cyrillic.
//! Code pages 437 and 850 and the Symbol font are tables here. Any other page's bytes above
//! 0x7F are read as U+FFFD and counted.

use encoding_rs::Encoding;

use crate::lexer::LeadBytes;

/// The code page Symbol-font text is read in: not a real code page, the number Windows
/// gives the font's own encoding.
pub(crate) const SYMBOL: u16 = 42;

/// The code page a font's `\fcharsetN` names, or `None` for "the document's own".
pub(crate) fn for_charset(charset: i32) -> Option<u16> {
    Some(match charset {
        // ANSI and DEFAULT: see the module documentation.
        0 | 1 => return None,
        2 => SYMBOL,
        77 => 10000,
        78 => 10001,
        79 => 10003,
        80 => 10008,
        81 => 10002,
        83 => 10005,
        84 => 10004,
        85 => 10006,
        86 => 10081,
        87 => 10021,
        88 => 10029,
        89 => 10007,
        128 => 932,
        129 => 949,
        130 => 1361,
        134 => 936,
        136 => 950,
        161 => 1253,
        162 => 1254,
        163 => 1258,
        177 => 1255,
        178 => 1256,
        186 => 1257,
        204 => 1251,
        222 => 874,
        238 => 1250,
        254 => 437,
        255 => 850,
        _ => return None,
    })
}

/// How to read one code page.
#[derive(Debug, Clone, Copy)]
enum Decoder {
    Encoding(&'static Encoding),
    /// A single-byte page: ASCII below 0x80, this table above.
    Table(&'static [u16; 128]),
    Symbol,
    Unsupported,
}

fn decoder(code_page: u16) -> Decoder {
    use encoding_rs as e;
    let encoding = match code_page {
        437 => return Decoder::Table(&CP437),
        850 => return Decoder::Table(&CP850),
        SYMBOL => return Decoder::Symbol,
        866 => e::IBM866,
        874 => e::WINDOWS_874,
        932 => e::SHIFT_JIS,
        936 => e::GBK,
        949 => e::EUC_KR,
        950 => e::BIG5,
        1250 => e::WINDOWS_1250,
        1251 => e::WINDOWS_1251,
        1252 => e::WINDOWS_1252,
        1253 => e::WINDOWS_1253,
        1254 => e::WINDOWS_1254,
        1255 => e::WINDOWS_1255,
        1256 => e::WINDOWS_1256,
        1257 => e::WINDOWS_1257,
        1258 => e::WINDOWS_1258,
        10000 => e::MACINTOSH,
        10007 => e::X_MAC_CYRILLIC,
        20866 => e::KOI8_R,
        20932 | 51932 => e::EUC_JP,
        21866 => e::KOI8_U,
        28591 => e::WINDOWS_1252,
        28592 => e::ISO_8859_2,
        28593 => e::ISO_8859_3,
        28594 => e::ISO_8859_4,
        28595 => e::ISO_8859_5,
        28596 => e::ISO_8859_6,
        28597 => e::ISO_8859_7,
        28598 => e::ISO_8859_8,
        28599 => e::WINDOWS_1254,
        28603 => e::ISO_8859_13,
        28605 => e::ISO_8859_15,
        54936 => e::GB18030,
        65001 => e::UTF_8,
        _ => return Decoder::Unsupported,
    };
    Decoder::Encoding(encoding)
}

/// The lead bytes of `code_page`, when it spells characters with two bytes.
pub(crate) fn lead_bytes(code_page: u16) -> Option<LeadBytes> {
    match code_page {
        932 => Some(LeadBytes::ShiftJis),
        936 | 949 | 950 | 54936 => Some(LeadBytes::Wide),
        _ => None,
    }
}

/// What decoding one buffer found.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct Decoded {
    /// Characters read as U+FFFD because the bytes spell nothing in the page.
    pub undecodable: usize,
    /// Bytes read as U+FFFD because the page has no table here.
    pub unsupported: usize,
}

/// Append `bytes`, read in `code_page`, to `out`.
pub(crate) fn decode(bytes: &[u8], code_page: u16, out: &mut String) -> Decoded {
    let mut found = Decoded::default();
    match decoder(code_page) {
        Decoder::Encoding(encoding) => {
            // Never the BOM-sniffing `decode`: these bytes are the middle of a document,
            // and three that happen to spell a byte-order mark are text.
            let (text, had_errors) = encoding.decode_without_bom_handling(bytes);
            if had_errors {
                found.undecodable = text.chars().filter(|c| *c == '\u{FFFD}').count();
            }
            out.push_str(&text);
        }
        Decoder::Table(table) => {
            for &byte in bytes {
                out.push(match byte.checked_sub(0x80) {
                    None => char::from(byte),
                    Some(high) => table
                        .get(usize::from(high))
                        .and_then(|u| char::from_u32(u32::from(*u)))
                        .unwrap_or('\u{FFFD}'),
                });
            }
        }
        Decoder::Symbol => {
            for &byte in bytes {
                out.push(symbol_byte(byte));
            }
        }
        Decoder::Unsupported => {
            for &byte in bytes {
                if byte.is_ascii() {
                    out.push(char::from(byte));
                } else {
                    out.push('\u{FFFD}');
                    found.unsupported += 1;
                }
            }
        }
    }
    found
}

/// A byte in the Symbol font: the character the font shows there, else the private-use
/// character Word gives it (U+F000 plus the byte), which is what a `\uN` in a Symbol font
/// names too.
fn symbol_byte(byte: u8) -> char {
    if byte < 0x20 {
        return char::from(byte);
    }
    symbol_character(byte)
        .or_else(|| char::from_u32(0xF000 | u32::from(byte)))
        .unwrap_or('\u{FFFD}')
}

/// A private-use character Word writes for a Symbol-font glyph (U+F020 to U+F0FF), read as
/// the character the font shows there. Other characters are returned unchanged.
pub(crate) fn symbol_private_use(c: char) -> char {
    let code = u32::from(c);
    if !(0xF020..=0xF0FF).contains(&code) {
        return c;
    }
    u8::try_from(code - 0xF000)
        .ok()
        .and_then(symbol_character)
        .unwrap_or(c)
}

/// The character Word's Symbol font shows at `code`, as Unicode's own mapping for that font
/// gives it (`VENDORS/APPLE/SYMBOL.TXT`), from `0x20` on. `None` where the font holds
/// nothing, or a piece of a drawn sign Unicode has no character for.
///
/// The same table `document_ingest`'s DOCX scanner reads `w:sym` with.
fn symbol_character(code: u8) -> Option<char> {
    #[rustfmt::skip]
    const FROM_0X20: [u16; 224] = [
        0x0020, 0x0021, 0x2200, 0x0023, 0x2203, 0x0025, 0x0026, 0x220B, 0x0028, 0x0029, 0x2217, 0x002B, 0x002C, 0x2212, 0x002E, 0x002F,
        0x0030, 0x0031, 0x0032, 0x0033, 0x0034, 0x0035, 0x0036, 0x0037, 0x0038, 0x0039, 0x003A, 0x003B, 0x003C, 0x003D, 0x003E, 0x003F,
        0x2245, 0x0391, 0x0392, 0x03A7, 0x0394, 0x0395, 0x03A6, 0x0393, 0x0397, 0x0399, 0x03D1, 0x039A, 0x039B, 0x039C, 0x039D, 0x039F,
        0x03A0, 0x0398, 0x03A1, 0x03A3, 0x03A4, 0x03A5, 0x03C2, 0x03A9, 0x039E, 0x03A8, 0x0396, 0x005B, 0x2234, 0x005D, 0x22A5, 0x005F,
        0x0000, 0x03B1, 0x03B2, 0x03C7, 0x03B4, 0x03B5, 0x03C6, 0x03B3, 0x03B7, 0x03B9, 0x03D5, 0x03BA, 0x03BB, 0x03BC, 0x03BD, 0x03BF,
        0x03C0, 0x03B8, 0x03C1, 0x03C3, 0x03C4, 0x03C5, 0x03D6, 0x03C9, 0x03BE, 0x03C8, 0x03B6, 0x007B, 0x007C, 0x007D, 0x223C, 0x0000,
        0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
        0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
        0x20AC, 0x03D2, 0x2032, 0x2264, 0x2044, 0x221E, 0x0192, 0x2663, 0x2666, 0x2665, 0x2660, 0x2194, 0x2190, 0x2191, 0x2192, 0x2193,
        0x00B0, 0x00B1, 0x2033, 0x2265, 0x00D7, 0x221D, 0x2202, 0x2022, 0x00F7, 0x2260, 0x2261, 0x2248, 0x2026, 0x23D0, 0x23AF, 0x21B5,
        0x2135, 0x2111, 0x211C, 0x2118, 0x2297, 0x2295, 0x2205, 0x2229, 0x222A, 0x2283, 0x2287, 0x2284, 0x2282, 0x2286, 0x2208, 0x2209,
        0x2220, 0x2207, 0x00AE, 0x00A9, 0x2122, 0x220F, 0x221A, 0x22C5, 0x00AC, 0x2227, 0x2228, 0x21D4, 0x21D0, 0x21D1, 0x21D2, 0x21D3,
        0x25CA, 0x27E8, 0x00AE, 0x00A9, 0x2122, 0x2211, 0x239B, 0x239C, 0x239D, 0x23A1, 0x23A2, 0x23A3, 0x23A7, 0x23A8, 0x23A9, 0x23AA,
        0x0000, 0x27E9, 0x222B, 0x2320, 0x23AE, 0x2321, 0x239E, 0x239F, 0x23A0, 0x23A4, 0x23A5, 0x23A6, 0x23AB, 0x23AC, 0x23AD, 0x0000,
    ];
    let unicode = *FROM_0X20.get(usize::from(code.checked_sub(0x20)?))?;
    (unicode != 0)
        .then(|| char::from_u32(u32::from(unicode)))
        .flatten()
}

/// Code page 437, 0x80 to 0xFF, from the Unicode mapping of the page.
#[rustfmt::skip]
const CP437: [u16; 128] = [
    0x00C7, 0x00FC, 0x00E9, 0x00E2, 0x00E4, 0x00E0, 0x00E5, 0x00E7,
    0x00EA, 0x00EB, 0x00E8, 0x00EF, 0x00EE, 0x00EC, 0x00C4, 0x00C5,
    0x00C9, 0x00E6, 0x00C6, 0x00F4, 0x00F6, 0x00F2, 0x00FB, 0x00F9,
    0x00FF, 0x00D6, 0x00DC, 0x00A2, 0x00A3, 0x00A5, 0x20A7, 0x0192,
    0x00E1, 0x00ED, 0x00F3, 0x00FA, 0x00F1, 0x00D1, 0x00AA, 0x00BA,
    0x00BF, 0x2310, 0x00AC, 0x00BD, 0x00BC, 0x00A1, 0x00AB, 0x00BB,
    0x2591, 0x2592, 0x2593, 0x2502, 0x2524, 0x2561, 0x2562, 0x2556,
    0x2555, 0x2563, 0x2551, 0x2557, 0x255D, 0x255C, 0x255B, 0x2510,
    0x2514, 0x2534, 0x252C, 0x251C, 0x2500, 0x253C, 0x255E, 0x255F,
    0x255A, 0x2554, 0x2569, 0x2566, 0x2560, 0x2550, 0x256C, 0x2567,
    0x2568, 0x2564, 0x2565, 0x2559, 0x2558, 0x2552, 0x2553, 0x256B,
    0x256A, 0x2518, 0x250C, 0x2588, 0x2584, 0x258C, 0x2590, 0x2580,
    0x03B1, 0x00DF, 0x0393, 0x03C0, 0x03A3, 0x03C3, 0x00B5, 0x03C4,
    0x03A6, 0x0398, 0x03A9, 0x03B4, 0x221E, 0x03C6, 0x03B5, 0x2229,
    0x2261, 0x00B1, 0x2265, 0x2264, 0x2320, 0x2321, 0x00F7, 0x2248,
    0x00B0, 0x2219, 0x00B7, 0x221A, 0x207F, 0x00B2, 0x25A0, 0x00A0,
];

/// Code page 850, 0x80 to 0xFF, from the Unicode mapping of the page.
#[rustfmt::skip]
const CP850: [u16; 128] = [
    0x00C7, 0x00FC, 0x00E9, 0x00E2, 0x00E4, 0x00E0, 0x00E5, 0x00E7,
    0x00EA, 0x00EB, 0x00E8, 0x00EF, 0x00EE, 0x00EC, 0x00C4, 0x00C5,
    0x00C9, 0x00E6, 0x00C6, 0x00F4, 0x00F6, 0x00F2, 0x00FB, 0x00F9,
    0x00FF, 0x00D6, 0x00DC, 0x00F8, 0x00A3, 0x00D8, 0x00D7, 0x0192,
    0x00E1, 0x00ED, 0x00F3, 0x00FA, 0x00F1, 0x00D1, 0x00AA, 0x00BA,
    0x00BF, 0x00AE, 0x00AC, 0x00BD, 0x00BC, 0x00A1, 0x00AB, 0x00BB,
    0x2591, 0x2592, 0x2593, 0x2502, 0x2524, 0x00C1, 0x00C2, 0x00C0,
    0x00A9, 0x2563, 0x2551, 0x2557, 0x255D, 0x00A2, 0x00A5, 0x2510,
    0x2514, 0x2534, 0x252C, 0x251C, 0x2500, 0x253C, 0x00E3, 0x00C3,
    0x255A, 0x2554, 0x2569, 0x2566, 0x2560, 0x2550, 0x256C, 0x00A4,
    0x00F0, 0x00D0, 0x00CA, 0x00CB, 0x00C8, 0x0131, 0x00CD, 0x00CE,
    0x00CF, 0x2518, 0x250C, 0x2588, 0x2584, 0x00A6, 0x00CC, 0x2580,
    0x00D3, 0x00DF, 0x00D4, 0x00D2, 0x00F5, 0x00D5, 0x00B5, 0x00FE,
    0x00DE, 0x00DA, 0x00DB, 0x00D9, 0x00FD, 0x00DD, 0x00AF, 0x00B4,
    0x00AD, 0x00B1, 0x2017, 0x00BE, 0x00B6, 0x00A7, 0x00F7, 0x00B8,
    0x00B0, 0x00A8, 0x00B7, 0x00B9, 0x00B3, 0x00B2, 0x25A0, 0x00A0,
];

#[cfg(test)]
mod tests {
    use super::*;

    fn read(bytes: &[u8], code_page: u16) -> (String, Decoded) {
        let mut out = String::new();
        let found = decode(bytes, code_page, &mut out);
        (out, found)
    }

    #[test]
    fn windows_1252_reads_the_curly_quotes_as_quotes_not_control_characters() {
        assert_eq!(read(&[0x93, b'q', 0x94], 1252).0, "\u{201C}q\u{201D}");
    }

    #[test]
    fn mac_roman_and_the_double_byte_pages() {
        assert_eq!(read(b"caf\x8e", 10000).0, "caf\u{E9}");
        // An arrow typed in a Japanese font: two bytes, one character.
        assert_eq!(read(&[0x81, 0xCB], 932).0, "\u{21D2}");
    }

    #[test]
    fn the_pages_held_as_tables() {
        assert_eq!(read(&[0x82], 437).0, "\u{E9}");
        assert_eq!(read(&[0x9B], 850).0, "\u{F8}");
        assert_eq!(read(b"a", SYMBOL).0, "\u{3B1}");
    }

    #[test]
    fn an_unknown_page_keeps_ascii_and_counts_the_rest() {
        let (text, found) = read(b"ab\xC0", 10001);
        assert_eq!(text, "ab\u{FFFD}");
        assert_eq!(found.unsupported, 1);
    }

    #[test]
    fn bytes_spelling_a_bom_are_text() {
        assert_eq!(read(&[0xEF, 0xBB, 0xBF], 1252).0, "\u{EF}\u{BB}\u{BF}");
    }

    #[test]
    fn a_symbol_font_private_use_character_is_read_as_its_glyph() {
        assert_eq!(symbol_private_use('\u{F061}'), '\u{3B1}');
        assert_eq!(symbol_private_use('x'), 'x');
    }
}
