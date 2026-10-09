// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Archives no honest zip writer makes: names that climb out of the archive,
//! two members under one name, links, encrypted members, compression methods the
//! importer does not read.
//!
//! Written by hand, byte by byte, because `zip::ZipWriter` refuses to write any of
//! them. Every member is stored (not compressed) with a correct CRC, so an archive
//! is refused for the reason under test and never for a broken checksum.

/// One member of a hand-built archive.
#[derive(Debug, Clone)]
pub struct RawEntry {
    /// The name exactly as the archive will list it.
    pub name: String,
    pub data: Vec<u8>,
    /// A Unix mode to record (`0o120777` for a link), when any.
    pub unix_mode: Option<u32>,
    /// Mark the member as encrypted.
    pub encrypted: bool,
    /// The compression method recorded (0 = stored). The data is still stored;
    /// only the method field says otherwise.
    pub method: u16,
    /// The unpacked size the headers claim, when it is not the truth.
    pub declared_size: Option<u32>,
}

impl RawEntry {
    pub fn file(name: &str, data: &[u8]) -> Self {
        RawEntry {
            name: name.to_string(),
            data: data.to_vec(),
            unix_mode: None,
            encrypted: false,
            method: 0,
            declared_size: None,
        }
    }

    pub fn link(name: &str, target: &str) -> Self {
        RawEntry {
            unix_mode: Some(0o120_777),
            ..RawEntry::file(name, target.as_bytes())
        }
    }
}

/// CRC-32 (IEEE), as zip records it.
pub fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &byte in data {
        crc ^= u32::from(byte);
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}

/// An archive of `entries`, in order, duplicates and all.
pub fn zip(entries: &[RawEntry]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut central = Vec::new();
    for entry in entries {
        let offset = out.len() as u32;
        let crc = crc32(&entry.data);
        let size = entry.data.len() as u32;
        let declared = entry.declared_size.unwrap_or(size);
        let name = entry.name.as_bytes();
        let flags: u16 = 0x0800 | if entry.encrypted { 1 } else { 0 };
        // Local file header.
        out.extend_from_slice(b"PK\x03\x04");
        out.extend_from_slice(&20u16.to_le_bytes());
        out.extend_from_slice(&flags.to_le_bytes());
        out.extend_from_slice(&entry.method.to_le_bytes());
        out.extend_from_slice(&[0, 0, 0x21, 0]); // time, date
        out.extend_from_slice(&crc.to_le_bytes());
        out.extend_from_slice(&size.to_le_bytes());
        out.extend_from_slice(&declared.to_le_bytes());
        out.extend_from_slice(&(name.len() as u16).to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(name);
        out.extend_from_slice(&entry.data);
        // Central directory header.
        let made_by: u16 = if entry.unix_mode.is_some() {
            0x0314
        } else {
            0x0014
        };
        let external: u32 = entry.unix_mode.map_or(0, |mode| mode << 16);
        central.extend_from_slice(b"PK\x01\x02");
        central.extend_from_slice(&made_by.to_le_bytes());
        central.extend_from_slice(&20u16.to_le_bytes());
        central.extend_from_slice(&flags.to_le_bytes());
        central.extend_from_slice(&entry.method.to_le_bytes());
        central.extend_from_slice(&[0, 0, 0x21, 0]);
        central.extend_from_slice(&crc.to_le_bytes());
        central.extend_from_slice(&size.to_le_bytes());
        central.extend_from_slice(&declared.to_le_bytes());
        central.extend_from_slice(&(name.len() as u16).to_le_bytes());
        central.extend_from_slice(&0u16.to_le_bytes()); // extra
        central.extend_from_slice(&0u16.to_le_bytes()); // comment
        central.extend_from_slice(&0u16.to_le_bytes()); // disk
        central.extend_from_slice(&0u16.to_le_bytes()); // internal attributes
        central.extend_from_slice(&external.to_le_bytes());
        central.extend_from_slice(&offset.to_le_bytes());
        central.extend_from_slice(name);
    }
    let directory_offset = out.len() as u32;
    let directory_len = central.len() as u32;
    out.extend_from_slice(&central);
    // Past 65,535 members the classic record cannot count them: a zip64 record
    // and its locator carry the count, as every archiver writes them.
    let count = if entries.len() > usize::from(u16::MAX) - 1 {
        let record = out.len() as u64;
        out.extend_from_slice(b"PK\x06\x06");
        out.extend_from_slice(&44u64.to_le_bytes());
        out.extend_from_slice(&45u16.to_le_bytes());
        out.extend_from_slice(&45u16.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(&(entries.len() as u64).to_le_bytes());
        out.extend_from_slice(&(entries.len() as u64).to_le_bytes());
        out.extend_from_slice(&u64::from(directory_len).to_le_bytes());
        out.extend_from_slice(&u64::from(directory_offset).to_le_bytes());
        out.extend_from_slice(b"PK\x06\x07");
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(&record.to_le_bytes());
        out.extend_from_slice(&1u32.to_le_bytes());
        u16::MAX
    } else {
        entries.len() as u16
    };
    out.extend_from_slice(b"PK\x05\x06");
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&count.to_le_bytes());
    out.extend_from_slice(&count.to_le_bytes());
    out.extend_from_slice(&directory_len.to_le_bytes());
    out.extend_from_slice(&directory_offset.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc32_matches_the_standard_check_value() {
        assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
    }
}
