// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `.scrivtemplate`: a project zip written as text inside an XML wrapper.
//!
//! ```xml
//! <ScrivenerTemplate Version="1.0">
//!     <Title>…</Title><Category>…</Category>
//!     <ProjectZipData Type="Base64">UEsDBBQ…</ProjectZipData>
//! </ScrivenerTemplate>
//! ```
//!
//! Scrivener 3 writes `Type="Base64"`. Scrivener 2 wrote no `Type` and encoded the
//! zip in hexadecimal, which decoded as base64 gives garbage. So: `Base64` is
//! base64; no `Type` is hexadecimal when the text is all hexadecimal digits and
//! starts with a zip signature, and base64 otherwise; any other `Type` is refused.
//! Whatever the encoding, the result must start with a zip signature, which is
//! what catches a truncated download.

use base64::Engine;

use crate::model::TemplateEncoding;
use crate::outcome::TemplateDamage;
use crate::xml::{self, child, child_text, text};

/// The largest template read.
pub(crate) const MAX_TEMPLATE_BYTES: u64 = 512 << 20;

/// A decoded template: its project zip, how it was spelled, its title.
pub(crate) struct Decoded {
    pub(crate) zip: Vec<u8>,
    pub(crate) encoding: TemplateEncoding,
    pub(crate) title: Option<String>,
}

/// Decode a `.scrivtemplate`'s bytes.
pub(crate) fn decode(file: &str, bytes: &[u8]) -> Result<Decoded, TemplateDamage> {
    let source = xml::text_of(bytes).map_err(|_| TemplateDamage::NotXml)?;
    let doc = xml::parse(file, source).map_err(|_| TemplateDamage::NotXml)?;
    let root = doc.root_element();
    if root.tag_name().name() != "ScrivenerTemplate" {
        return Err(TemplateDamage::NotATemplate);
    }
    let title = child_text(root, "Title")
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty());
    let data = child(root, "ProjectZipData").ok_or(TemplateDamage::NoProjectData)?;
    let raw: Vec<u8> = text(data)
        .unwrap_or_default()
        .bytes()
        .filter(|b| !b.is_ascii_whitespace())
        .collect();
    if raw.is_empty() {
        return Err(TemplateDamage::NoProjectData);
    }
    let (zip, encoding) = match data.attribute("Type") {
        Some("Base64") => (base64_decode(&raw)?, TemplateEncoding::Base64),
        Some(other) => {
            return Err(TemplateDamage::UnknownEncoding {
                encoding: other.to_string(),
            });
        }
        None if raw.len() >= 8
            && raw.iter().all(u8::is_ascii_hexdigit)
            && raw[..8].eq_ignore_ascii_case(b"504b0304") =>
        {
            (hex_decode(&raw)?, TemplateEncoding::Hex)
        }
        None => (base64_decode(&raw)?, TemplateEncoding::UntypedBase64),
    };
    if !(zip.starts_with(b"PK\x03\x04") || zip.starts_with(b"PK\x05\x06")) {
        return Err(TemplateDamage::NotAZip);
    }
    Ok(Decoded {
        zip,
        encoding,
        title,
    })
}

fn base64_decode(raw: &[u8]) -> Result<Vec<u8>, TemplateDamage> {
    base64::engine::general_purpose::STANDARD
        .decode(raw)
        .map_err(|_| TemplateDamage::Undecodable)
}

fn hex_decode(raw: &[u8]) -> Result<Vec<u8>, TemplateDamage> {
    if !raw.len().is_multiple_of(2) {
        return Err(TemplateDamage::Undecodable);
    }
    raw.chunks(2)
        .map(|pair| {
            let hi = (pair[0] as char).to_digit(16);
            let lo = (pair[1] as char).to_digit(16);
            match (hi, lo) {
                (Some(hi), Some(lo)) => {
                    u8::try_from(hi * 16 + lo).map_err(|_| TemplateDamage::Undecodable)
                }
                _ => Err(TemplateDamage::Undecodable),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const ZIP_HEAD: &[u8] = b"PK\x05\x06\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";

    fn wrap(attr: &str, data: &str) -> Vec<u8> {
        format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<ScrivenerTemplate Version=\"1.0\">\n    <Title>Starter</Title>\n    <ProjectZipData{attr}>{data}</ProjectZipData>\n</ScrivenerTemplate>\n"
        )
        .into_bytes()
    }

    #[test]
    fn base64_and_hex_templates_both_decode() {
        let b64 = base64::engine::general_purpose::STANDARD.encode(ZIP_HEAD);
        let decoded = decode("t.scrivtemplate", &wrap(" Type=\"Base64\"", &b64));
        assert!(matches!(
            decoded,
            Ok(Decoded {
                encoding: TemplateEncoding::Base64,
                ..
            })
        ));
        let hex: String = ZIP_HEAD.iter().map(|b| format!("{b:02x}")).collect();
        // A hex template's zip starts with a local header, not an empty
        // directory; build one that does.
        let hex = format!("504b0304{}", &hex[8..]);
        let decoded = decode("t.scrivtemplate", &wrap("", &hex));
        assert!(matches!(
            decoded,
            Ok(Decoded {
                encoding: TemplateEncoding::Hex,
                ..
            })
        ));
    }

    #[test]
    fn a_truncated_or_unknown_template_is_refused() {
        assert_eq!(
            decode("t.scrivtemplate", &wrap(" Type=\"Base64\"", "UEsDBA")).err(),
            Some(TemplateDamage::Undecodable)
        );
        assert_eq!(
            decode("t.scrivtemplate", &wrap(" Type=\"Rot13\"", "abc")).err(),
            Some(TemplateDamage::UnknownEncoding {
                encoding: "Rot13".to_string()
            })
        );
        assert_eq!(
            decode(
                "t.scrivtemplate",
                &wrap(" Type=\"Base64\"", "aGVsbG8gd29ybGQ=")
            )
            .err(),
            Some(TemplateDamage::NotAZip)
        );
        assert_eq!(
            decode("t.scrivtemplate", b"<Other/>").err(),
            Some(TemplateDamage::NotATemplate)
        );
    }
}
