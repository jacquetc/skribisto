// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Reading a zip archive nobody vouches for, bounded before and while it inflates.
//!
//! # The failure this prevents
//!
//! A zip member's sizes are claims the archive makes about itself. A deflate
//! stream expands about a thousand times at most, so a few megabytes of archive can
//! unpack to gigabytes; and a zip64 extra field lets a header declare a member of
//! any size up to 2^64 bytes, whatever the member really holds. Nothing downstream
//! stops either:
//!
//! * `zip` 8.6 inflates a member to the end of its stream whatever its header
//!   declared: its deflate decoder is never handed the declared size
//!   (`Decompressor::new` in its `compression.rs` passes it to LZMA and the legacy
//!   methods only), so reading a bomb to the end fills memory with all of it.
//! * `docx-rs` 0.4.22 reserves a member's declared size before it reads a byte of
//!   it (`Vec::with_capacity`, in its `reader/read_zip.rs`), and so does the
//!   dictionary installer's own reader. A zip64 header declaring an exabyte makes
//!   that one reservation fail, and a failed allocation is not a panic: it aborts
//!   the process, it cannot be caught, and every unsaved document in every window
//!   dies with it.
//!
//! Every archive a writer can hand Skribisto is read through here: a `.docx` or an
//! `.odt` being imported, a `.plume` or `.msk` project, a `.skrib` itself (a project
//! travels by mail and shared drives like any other file, and so does its backup),
//! and a downloaded dictionary.
//!
//! # Two checks, in this order
//!
//! 1. **The central directory, before anything inflates**
//!    ([`ZipGuard::check_directory`]): the entry count, every member the reader
//!    will read declaring no more than its budget, and all of them together no
//!    more than the archive may unpack to. This is what refuses a zip64 member
//!    declaring an exabyte before `docx-rs` can try to reserve it.
//! 2. **Every byte while it inflates** ([`ZipGuard::copy`], [`ZipGuard::read`]): a
//!    declared size decides nothing on its own, so each member is read through a
//!    reader that stops one byte past its budget, and after each member the
//!    archive as a whole is held to the same ratio.
//!
//! # The budget
//!
//! A member may inflate to the larger of [`RATIO_FLOOR_BYTES`] and [`MAX_RATIO`]
//! times its compressed size, never past the reader's ceiling for one member
//! ([`ZipLimits::max_member_bytes`]) nor past what is left of its ceiling for the
//! archive ([`ZipLimits::max_total_bytes`]). Real content does not come near the
//! ratio: prose deflates three or four times, markup rather more, and pictures not
//! at all. A bomb's whole trick is a ratio in the thousands.
//!
//! **A compressed size is a claim too**, so it is never taken past the length of
//! the archive holding it. Otherwise a member claiming a terabyte of compressed
//! data would give itself a budget to match, and members laid over the same bytes
//! (the overlapping-entry bomb, whose members each point into one shared stream)
//! would each count those bytes again towards the archive's ratio.
//!
//! # What a refusal is
//!
//! A typed [`ZipRefused`], wherever in an `anyhow` chain it ends up
//! ([`refused()`](crate::zip_guard::refused) finds it), so a reader's caller can tell
//! the writer why in their own language.
//! A long operation reports its failure as text, so the value also has a spelling
//! that survives that trip ([`ZipRefused::failure_message`]).

use std::fmt;
use std::io::{Read, Seek, SeekFrom, Write};

use anyhow::Context;
use zip::ZipArchive;
use zip::read::ZipFile;
use zip::result::ZipError;

use crate::errors::SkribFormatError;

/// Above [`RATIO_FLOOR_BYTES`], refuse a member, or an archive, that has expanded
/// more than this many times over the bytes consumed to produce it.
///
/// A zip bomb's whole trick is a ratio in the thousands. Real content does not come
/// close: Djot prose deflates around 3-4x, XML and RON manifests rather more, and a
/// picture, already compressed, not at all (`zip_io::zip_dir` stores them).
pub const MAX_RATIO: u64 = 200;

/// Below this much unpacked, the ratio is not consulted: a few small, highly
/// compressible manifests can legitimately show a large ratio, and refusing a 2 KB
/// document over it would be absurd.
pub const RATIO_FLOOR_BYTES: u64 = 64 << 20;

/// What one kind of archive may hold. Each reader sets its own, next to the reader,
/// with the real documents that justify it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZipLimits {
    /// The most entries the central directory may list.
    pub max_entries: usize,
    /// The most bytes one member may unpack to, whatever its ratio.
    pub max_member_bytes: u64,
    /// The most bytes the members read may unpack to, together.
    pub max_total_bytes: u64,
}

/// What a refused failure message starts with when it has to cross a boundary
/// that only carries text; see [`ZipRefused::failure_message`].
const FAILURE_TAG: &str = "zip-refused:";

/// Why an archive was refused, before it was read or while it was.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZipRefused {
    /// The central directory lists more entries than the archive may hold.
    TooManyEntries { entries: usize, limit: usize },
    /// One member unpacks, or declares that it unpacks, past what it may.
    MemberTooLarge {
        /// The member, as the archive names it: `content.xml`, `word/document.xml`.
        part: String,
        /// The bytes it may unpack to: its own budget, which is at least
        /// [`RATIO_FLOOR_BYTES`] unless the reader's ceiling for one member is lower.
        limit: u64,
        /// What its header declared, when the header is what was refused. `None`
        /// when the member was refused while it inflated.
        declared: Option<u64>,
    },
    /// The members read unpack, or declare that they unpack, past what the archive
    /// may, together: its ceiling, or its ratio once past the floor.
    ArchiveTooLarge { limit: u64 },
}

impl ZipRefused {
    /// The member refused, or `None` when it was the archive as a whole.
    pub fn part(&self) -> Option<&str> {
        match self {
            ZipRefused::MemberTooLarge { part, .. } => Some(part),
            ZipRefused::TooManyEntries { .. } | ZipRefused::ArchiveTooLarge { .. } => None,
        }
    }

    /// The ceiling a size refusal went past, in whole mebibytes (never less than
    /// one), or `None` for [`ZipRefused::TooManyEntries`]. Rounded down, so "more
    /// than this many" stays true of what was refused.
    pub fn limit_mib(&self) -> Option<u64> {
        match self {
            ZipRefused::MemberTooLarge { limit, .. } | ZipRefused::ArchiveTooLarge { limit } => {
                Some((limit >> 20).max(1))
            }
            ZipRefused::TooManyEntries { .. } => None,
        }
    }

    /// This refusal as one line of text a reader can turn back into the value.
    ///
    /// A long operation reports its failure as a string (`OperationStatus::Failed`),
    /// so a typed error cannot reach the UI through it as a type. The numbers come
    /// first and the part last, so a part name holding a colon still reads back whole.
    pub fn failure_message(&self) -> String {
        match self {
            ZipRefused::TooManyEntries { entries, limit } => {
                format!("{FAILURE_TAG}entries:{entries}:{limit}")
            }
            ZipRefused::MemberTooLarge {
                part,
                limit,
                declared,
            } => format!(
                "{FAILURE_TAG}member:{limit}:{}:{part}",
                declared.map_or_else(String::new, |d| d.to_string())
            ),
            ZipRefused::ArchiveTooLarge { limit } => format!("{FAILURE_TAG}archive:{limit}"),
        }
    }

    /// Recover a refusal from a failure message, or `None` when the message is about
    /// something else.
    pub fn from_failure_message(message: &str) -> Option<Self> {
        let rest = message.strip_prefix(FAILURE_TAG)?;
        let (kind, rest) = rest.split_once(':')?;
        match kind {
            "entries" => {
                let (entries, limit) = rest.split_once(':')?;
                Some(ZipRefused::TooManyEntries {
                    entries: entries.parse().ok()?,
                    limit: limit.parse().ok()?,
                })
            }
            "member" => {
                let (limit, rest) = rest.split_once(':')?;
                let (declared, part) = rest.split_once(':')?;
                let declared = match declared {
                    "" => None,
                    number => Some(number.parse().ok()?),
                };
                Some(ZipRefused::MemberTooLarge {
                    part: part.to_string(),
                    limit: limit.parse().ok()?,
                    declared,
                })
            }
            "archive" => Some(ZipRefused::ArchiveTooLarge {
                limit: rest.parse().ok()?,
            }),
            _ => None,
        }
    }
}

impl fmt::Display for ZipRefused {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZipRefused::TooManyEntries { entries, limit } => write!(
                f,
                "the archive lists {entries} entries, more than the {limit} it may hold, \
                 so it is refused unread"
            ),
            ZipRefused::MemberTooLarge {
                part,
                limit,
                declared: Some(declared),
            } => write!(
                f,
                "{part} declares that it unpacks to {declared} bytes, past the {limit} it \
                 may, so the archive is refused unread"
            ),
            ZipRefused::MemberTooLarge {
                part,
                limit,
                declared: None,
            } => write!(
                f,
                "{part} unpacks past the {limit} bytes it may, which only an archive built \
                 to exhaust memory does, so it is refused"
            ),
            ZipRefused::ArchiveTooLarge { limit } => write!(
                f,
                "the archive unpacks past the {limit} bytes it may, which only an archive \
                 built to exhaust memory does, so it is refused"
            ),
        }
    }
}

impl std::error::Error for ZipRefused {}

/// The refusal inside `error`, wherever in its chain it sits.
///
/// Sees through [`SkribFormatError::Unreadable`] as well: that variant's own
/// `source` skips its first cause so a chain does not print it twice, which would
/// otherwise hide a refusal raised with no context of its own.
pub fn refused(error: &anyhow::Error) -> Option<&ZipRefused> {
    error.chain().find_map(|cause| {
        cause.downcast_ref::<ZipRefused>().or_else(|| {
            match cause.downcast_ref::<SkribFormatError>() {
                Some(SkribFormatError::Unreadable(inner)) => inner
                    .chain()
                    .find_map(|inner| inner.downcast_ref::<ZipRefused>()),
                _ => None,
            }
        })
    })
}

/// A running account of what one archive has unpacked, against its [`ZipLimits`].
///
/// One per archive opened: the ratio and the ceiling for the whole archive are
/// counted across every member read through it.
#[derive(Debug, Clone)]
pub struct ZipGuard {
    limits: ZipLimits,
    /// The archive's own length: no compressed size is taken past it.
    archive_len: u64,
    /// Bytes unpacked so far, every member together.
    written: u64,
    /// Compressed bytes those came from, never counted past `archive_len`.
    compressed: u64,
}

impl ZipGuard {
    /// A guard for an archive of `archive_len` bytes.
    pub fn new(limits: ZipLimits, archive_len: u64) -> Self {
        ZipGuard {
            limits,
            archive_len,
            written: 0,
            compressed: 0,
        }
    }

    /// Open `reader` as an archive, and a guard measured to it.
    ///
    /// Opening parses the central directory and nothing else; no member is read.
    pub fn open<R: Read + Seek>(
        limits: ZipLimits,
        mut reader: R,
    ) -> Result<(ZipArchive<R>, ZipGuard), ZipError> {
        let archive_len = reader.seek(SeekFrom::End(0))?;
        reader.seek(SeekFrom::Start(0))?;
        let archive = ZipArchive::new(reader)?;
        Ok((archive, ZipGuard::new(limits, archive_len)))
    }

    /// The limits this guard holds an archive to.
    pub fn limits(&self) -> ZipLimits {
        self.limits
    }

    /// The most bytes a member of `compressed` bytes may unpack to, on its own.
    fn member_budget(&self, compressed: u64) -> u64 {
        let compressed = compressed.min(self.archive_len);
        RATIO_FLOOR_BYTES
            .max(compressed.saturating_mul(MAX_RATIO))
            .min(self.limits.max_member_bytes)
    }

    /// Refuse the archive before anything in it is inflated: when it lists more
    /// entries than it may, when a member `wanted` selects declares more than its
    /// budget, or when those members declare more than the archive may, together.
    ///
    /// `wanted` is asked of each entry's name, so a reader that reads three members
    /// of a thousand checks those three: the rest will never be inflated, and a
    /// picture this reader never opens is no reason to refuse the document. Only
    /// what `wanted` accepts costs a read of its local header.
    pub fn check_directory<R: Read + Seek>(
        &self,
        archive: &mut ZipArchive<R>,
        mut wanted: impl FnMut(&str) -> bool,
    ) -> Result<(), ZipRefused> {
        if archive.len() > self.limits.max_entries {
            return Err(ZipRefused::TooManyEntries {
                entries: archive.len(),
                limit: self.limits.max_entries,
            });
        }
        let mut declared_total: u64 = 0;
        for index in 0..archive.len() {
            let Some(name) = archive.name_for_index(index) else {
                continue;
            };
            if !wanted(name) {
                continue;
            }
            // A member whose header cannot be read cannot be inflated either, by this
            // reader or any other: the read that would have used it fails the same way.
            let Ok(entry) = archive.by_index_raw(index) else {
                continue;
            };
            if entry.is_dir() {
                continue;
            }
            let declared = entry.size();
            let budget = self.member_budget(entry.compressed_size());
            if declared > budget {
                return Err(ZipRefused::MemberTooLarge {
                    part: entry.name().to_string(),
                    limit: budget,
                    declared: Some(declared),
                });
            }
            declared_total = declared_total.saturating_add(declared);
            if declared_total > self.limits.max_total_bytes {
                return Err(ZipRefused::ArchiveTooLarge {
                    limit: self.limits.max_total_bytes,
                });
            }
        }
        Ok(())
    }

    /// Inflate `entry` into `out`, refusing it once it goes past its budget, and
    /// return how many bytes it unpacked to.
    ///
    /// Its header's claim is checked first, so a member declaring more than it may
    /// is refused without inflating a byte. The read itself is what bounds it: it
    /// stops one byte past the budget whatever the header said.
    ///
    /// An `Err` is either a [`ZipRefused`] or the member failing to read (a damaged
    /// stream, a checksum that does not match), with the member named.
    pub fn copy<R: Read, W: Write>(
        &mut self,
        entry: &mut ZipFile<'_, R>,
        out: &mut W,
    ) -> anyhow::Result<u64> {
        let part = entry.name().to_string();
        let own = self.member_budget(entry.compressed_size());
        let left = self.limits.max_total_bytes.saturating_sub(self.written);
        let too_large = |declared| {
            if own <= left {
                ZipRefused::MemberTooLarge {
                    part: part.clone(),
                    limit: own,
                    declared,
                }
            } else {
                ZipRefused::ArchiveTooLarge {
                    limit: self.limits.max_total_bytes,
                }
            }
        };
        let budget = own.min(left);
        if entry.size() > budget {
            return Err(anyhow::Error::new(too_large(Some(entry.size()))));
        }
        let copied = std::io::copy(&mut (&mut *entry).take(budget.saturating_add(1)), out)
            .with_context(|| format!("{part} could not be read"))?;
        if copied > budget {
            return Err(anyhow::Error::new(too_large(None)));
        }

        self.written = self.written.saturating_add(copied);
        self.compressed = self
            .compressed
            .saturating_add(entry.compressed_size())
            .min(self.archive_len);
        let ratio_limit = RATIO_FLOOR_BYTES.max(self.compressed.saturating_mul(MAX_RATIO));
        if self.written > ratio_limit {
            return Err(anyhow::Error::new(ZipRefused::ArchiveTooLarge {
                limit: ratio_limit,
            }));
        }
        Ok(copied)
    }

    /// Inflate `entry` into memory, as [`ZipGuard::copy`] bounds it.
    ///
    /// Nothing is reserved from the header's claim: the buffer grows with what the
    /// member really holds, which the budget bounds.
    pub fn read<R: Read>(&mut self, entry: &mut ZipFile<'_, R>) -> anyhow::Result<Vec<u8>> {
        let mut out = Vec::new();
        self.copy(entry, &mut out)?;
        Ok(out)
    }

    /// Inflate the member `name` into memory, or `None` when the archive has no
    /// member of that name.
    pub fn read_named<R: Read + Seek>(
        &mut self,
        archive: &mut ZipArchive<R>,
        name: &str,
    ) -> anyhow::Result<Option<Vec<u8>>> {
        let mut entry = match archive.by_name(name) {
            Ok(entry) => entry,
            Err(ZipError::FileNotFound) => return Ok(None),
            Err(error) => {
                return Err(anyhow::Error::new(error).context(format!("{name} could not be read")));
            }
        };
        self.read(&mut entry).map(Some)
    }

    /// Inflate every member of `archive` without keeping a byte of it, refusing the
    /// archive as soon as one goes past its budget or fails to read.
    ///
    /// For an archive handed whole to a reader that does its own reading, and
    /// reserves what the headers declare (`docx-rs`): once this has passed, every
    /// member that reader can open unpacks to no more than its budget, and all of
    /// them together to no more than the archive's. A member that cannot be opened
    /// at all (encrypted, or in a method this build does not read) is passed over,
    /// since that reader cannot open it either.
    pub fn verify_all<R: Read + Seek>(
        &mut self,
        archive: &mut ZipArchive<R>,
    ) -> anyhow::Result<()> {
        for index in 0..archive.len() {
            let Ok(mut entry) = archive.by_index(index) else {
                continue;
            };
            if entry.is_dir() {
                continue;
            }
            self.copy(&mut entry, &mut std::io::sink())?;
        }
        Ok(())
    }
}

/// Crafted archives for the tests of every reader held to this module: bombs,
/// headers declaring what their members do not hold, and the zip64 fields that
/// carry such a claim. Built by hand rather than by `zip::ZipWriter`, which writes
/// only honest archives.
///
/// Behind the `hostile-fixtures` feature, which only the dev-dependencies of the
/// crates that read zips turn on.
#[cfg(any(test, feature = "hostile-fixtures"))]
pub mod fixtures;

#[cfg(test)]
#[path = "zip_guard_tests.rs"]
mod tests;
