// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The pictures a document brings, turned into what a project keeps.
//!
//! A project stores a picture as a file named by the blake3 of its bytes, beside an `Asset`
//! row holding its type and size, and the prose names it `assets/<hash>.<ext>`
//! (`skrib_format::media`). Everything here produces exactly that, with no store and no
//! window: the scanners call it as they meet a picture, and what they write into the prose
//! is the name it will have in the project.
//!
//! ## What is kept, what is converted, what is refused
//!
//! The format is read from the bytes, never from a name: a `.png` that is really a JPEG is a
//! JPEG, and an extension is only ever something a file claims.
//!
//! * **PNG, JPEG and WebP** are kept byte for byte. Those are the three the editor paints.
//! * **TIFF, GIF and BMP** are converted to PNG, which is lossless, so nothing the file showed
//!   is lost on the way. An animated GIF becomes its first frame, and says so
//!   ([`crate::ImportDiagnostic::PictureFirstFrameOnly`]).
//! * **Anything else** (a Windows metafile, an SVG drawing, a PDF page) is refused and named,
//!   since the editor could not show it ([`PictureRefusal::Unsupported`]).
//!
//! ## What a crafted picture cannot do
//!
//! A picture is data from somebody else's file, so every limit here is checked before the
//! work it bounds. Its bytes are capped before they are read ([`MAX_PICTURE_BYTES`]); its
//! dimensions are read from its header, and checked against [`MAX_PICTURE_SIDE`] and
//! [`MAX_PICTURE_PIXELS`], before a single pixel is decoded; and the one decode that happens,
//! for a format that has to be converted, runs under the `image` crate's own allocation
//! ceiling. A kept picture is never decoded at all: its header says how large it is, and the
//! editor decodes it when it paints it, as it does a picture inserted by hand.
//!
//! ## Its size, as the editor shows it
//!
//! A JPEG may carry an EXIF orientation, which the editor applies when it paints it
//! (`teksilo::canvas::RasterIcon::decode_jpeg`), so a portrait photograph stored on its side
//! shows upright. For the four orientations that turn it a quarter (5 to 8), its width and
//! height are swapped here, so the size the prose asks for is the size it is shown at. The
//! editor applies no orientation to a PNG or a WebP, and neither does this. A converted
//! picture is turned upright before it is written, so its PNG needs none.
//!
//! ## Where the bytes come from
//!
//! A Word or OpenDocument file holds its pictures inside it ([`PackagePictures`]). A Markdown
//! file names files beside it, and [`DocumentFolder`] reads those, only inside the document's
//! own folder: never an absolute path, never `..`, never through a symbolic link, and only a
//! regular file. A picture on the web is never fetched. Those rules and the format check
//! together are what stop a document from pulling a private file into a project the writer
//! may later share: a path is refused before it is opened, and whatever is read is kept only
//! if it is a picture.

use std::collections::HashMap;
use std::io::{Cursor, Read};
use std::path::{Component, Path, PathBuf};

use image::metadata::Orientation;
use image::{DynamicImage, ImageDecoder, ImageFormat, ImageReader, Limits};

/// The most bytes one picture may hold, as the document stores it.
///
/// A photograph from a camera is a few megabytes and a scan at print resolution rarely passes
/// thirty. Sixty-four leaves room for both, and keeps one crafted member from filling memory
/// before its header has even been read.
pub const MAX_PICTURE_BYTES: u64 = 64 << 20;

/// The longest side a picture may have, in pixels.
///
/// Past what any printed page needs at any resolution a writer would use, and the size past
/// which a graphics card refuses a texture outright, so a picture this wide could not be shown
/// anyway.
pub const MAX_PICTURE_SIDE: u32 = 16_384;

/// The most pixels a picture may have: 64 million, a 50-megapixel camera with room to spare.
///
/// The editor decodes every picture it paints, four bytes a pixel, so this is also what keeps
/// one picture under a quarter of a gigabyte on screen.
pub const MAX_PICTURE_PIXELS: u64 = 64_000_000;

/// The most bytes the pictures of one document may hold together: a gigabyte.
///
/// A document's pictures are held in memory from the moment they are read until the
/// importer stores them, so this is what bounds that memory for a document naming a great
/// many large files. A hundred camera photographs fit several times over; a document that
/// needs more is imported in parts.
pub const MAX_DOCUMENT_PICTURE_BYTES: u64 = 1 << 30;

/// The most the decoder may hold while converting one picture: the pixels of the largest one
/// allowed, twice over, which leaves room for a decoder's own working buffers.
///
/// Twice over because a conversion holds its picture twice at its peak, whatever the format:
/// the TIFF decoder decodes into a buffer of its own and copies it out, and turning a picture
/// upright builds the turned copy beside the original. So a picture whose decoded pixels pass
/// half of this is refused before any of them are allocated ([`MAX_DECODED_BYTES`]).
pub const MAX_DECODE_BYTES: u64 = 2 * 4 * MAX_PICTURE_PIXELS;

/// The most one converted picture's decoded pixels may hold: half of [`MAX_DECODE_BYTES`],
/// the other half being the decoder's own copy or the turned one. The largest picture allowed
/// at eight bits a channel fits exactly; one at sixteen bits, or in floating point, has to be
/// smaller.
pub const MAX_DECODED_BYTES: u64 = MAX_DECODE_BYTES / 2;

/// Why a picture the document shows does not come with it.
///
/// Each one is its own sentence in the review, because each asks something different of the
/// writer: convert it, fetch it, move it, or accept that it stays behind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PictureRefusal {
    /// A format the editor cannot show, named as the bytes identify it (`"EMF"`, `"SVG"`),
    /// or `"unknown"` when they identify nothing.
    Unsupported { format: &'static str },
    /// An address on the web. Nothing is downloaded while importing.
    Remote,
    /// A file outside the document and its own folder: an absolute path, a path climbing out
    /// with `..`, a symbolic link, or a Word or OpenDocument picture linked from the disk
    /// rather than stored in the file.
    Outside,
    /// The file the document names is not there.
    Missing,
    /// Larger than a project accepts: past [`MAX_PICTURE_BYTES`], [`MAX_PICTURE_SIDE`] or
    /// [`MAX_PICTURE_PIXELS`].
    TooLarge,
    /// Bytes that claim a picture format and do not read as one, or a name that is not a
    /// regular file.
    Unreadable,
    /// A picture in a heading. A heading becomes a title, which holds no picture.
    InHeading,
    /// A picture in a footnote, an endnote or a comment. A note and a comment keep their
    /// words and their four marks, and no picture.
    InNote,
    /// A picture a Markdown file writes as HTML (`<img src="…">`) rather than as Markdown.
    /// The HTML in a Markdown file is left out whole, and what it shows with it.
    InHtml,
    /// The document's pictures already come to [`MAX_DOCUMENT_PICTURE_BYTES`], and this one
    /// would pass it.
    DocumentFull,
}

impl PictureRefusal {
    /// The name the review translates this refusal under. See
    /// [`crate::ImportDiagnostic::key`].
    pub fn key(&self) -> &'static str {
        match self {
            PictureRefusal::Unsupported { .. } => "image-not-ingested-format",
            PictureRefusal::Remote => "image-not-ingested-remote",
            PictureRefusal::Outside => "image-not-ingested-outside",
            PictureRefusal::Missing => "image-not-ingested-missing",
            PictureRefusal::TooLarge => "image-not-ingested-too-large",
            PictureRefusal::Unreadable => "image-not-ingested-unreadable",
            PictureRefusal::InHeading => "image-not-ingested-heading",
            PictureRefusal::InNote => "image-not-ingested-note",
            PictureRefusal::InHtml => "image-not-ingested-html",
            PictureRefusal::DocumentFull => "image-not-ingested-document-full",
        }
    }

    /// An untranslated account, for logs and tests.
    pub fn describe(&self) -> String {
        match self {
            PictureRefusal::Unsupported { format } => {
                format!("a format the editor cannot show ({format})")
            }
            PictureRefusal::Remote => "on the web, and never downloaded".to_string(),
            PictureRefusal::Outside => "outside the document and its folder".to_string(),
            PictureRefusal::Missing => "not found".to_string(),
            PictureRefusal::TooLarge => "larger than a project accepts".to_string(),
            PictureRefusal::Unreadable => "not readable as a picture".to_string(),
            PictureRefusal::InHeading => "in a heading, which holds no picture".to_string(),
            PictureRefusal::InNote => "in a note or a comment, which holds no picture".to_string(),
            PictureRefusal::InHtml => "written as HTML, which is left out".to_string(),
            PictureRefusal::DocumentFull => {
                "past what one document's pictures may hold together".to_string()
            }
        }
    }
}

/// A picture as a project keeps it: the bytes a save writes, and what its `Asset` row says.
#[derive(Clone, PartialEq, Eq)]
pub struct IngestedPicture {
    /// blake3 of [`Self::bytes`], hex. The picture's identity and the stem of its file name.
    pub content_hash: String,
    /// `image/png`, `image/jpeg` or `image/webp`: the three the editor paints.
    pub mime_type: &'static str,
    /// Its width as the editor shows it: for a JPEG turned a quarter by its EXIF orientation,
    /// the height of the pixels as stored.
    pub width: u32,
    pub height: u32,
    /// The name the document gave it, reduced to a file name, with the extension of the
    /// format it is stored in. Provenance, and what an export names the file; never a path,
    /// so a writer's folders never reach the project.
    pub file_name: String,
    pub bytes: Vec<u8>,
}

impl std::fmt::Debug for IngestedPicture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IngestedPicture")
            .field("content_hash", &self.content_hash)
            .field("mime_type", &self.mime_type)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("file_name", &self.file_name)
            .field("bytes", &format_args!("{} bytes", self.bytes.len()))
            .finish()
    }
}

impl IngestedPicture {
    /// The extension its file is stored under, from its type.
    pub fn extension(&self) -> String {
        skrib_format::media::extension_for(self.mime_type)
    }

    /// Where it lives in a bundle, and what the prose calls it: `assets/<hash>.<ext>`.
    pub fn relative_path(&self) -> String {
        skrib_format::media::asset_relpath(&self.content_hash, &self.extension())
    }

    /// The name of its file in a project's media directory, `<hash>.<ext>`: the one a save
    /// reads it back under (`work_management::work_io::read_asset_bytes`).
    pub fn stored_name(&self) -> String {
        format!("{}.{}", self.content_hash, self.extension())
    }

    /// Its row in a bundle's `assets.ron`, for a converter that writes a bundle itself.
    ///
    /// `stamp` is the creation time the bundle records, written as the bundle writes every
    /// other one. The bytes go into `WorkBundle::asset_bytes` under
    /// [`Self::content_hash`]; a bundle listing an asset without them is refused on write.
    pub fn asset_file(&self, file_id: u64, stamp: &str) -> skrib_format::AssetFile {
        skrib_format::AssetFile {
            file_id,
            created_at: stamp.to_string(),
            updated_at: stamp.to_string(),
            content_hash: self.content_hash.clone(),
            file_name: self.file_name.clone(),
            mime_type: self.mime_type.to_string(),
            width: self.width,
            height: self.height,
            byte_size: self.bytes.len() as u64,
            alt: String::new(),
            is_cover: false,
            path: self.relative_path(),
        }
    }
}

/// A picture's identity: the blake3 of the bytes a project stores, in hex. The name of its
/// file and of its `Asset` row, and the same hash the editor names an inserted picture by
/// (`teksilo_ui::shared::images::examine_bytes`), so a picture imported and the same picture
/// inserted by hand are one picture.
pub fn content_hash(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

/// What [`ingest`] made of a picture.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ingested {
    pub picture: IngestedPicture,
    /// It was an animation, and only its first frame was kept.
    pub first_frame_only: bool,
}

/// Turn one picture's bytes into what a project keeps, or say why not.
///
/// `name` is what the document called it, a path or a member name; only its last segment is
/// kept (see [`IngestedPicture::file_name`]).
pub fn ingest(bytes: Vec<u8>, name: &str) -> Result<Ingested, PictureRefusal> {
    if bytes.len() as u64 > MAX_PICTURE_BYTES {
        return Err(PictureRefusal::TooLarge);
    }
    let Some(format) = sniff(&bytes) else {
        return Err(PictureRefusal::Unsupported {
            format: unsupported_format_name(&bytes),
        });
    };
    match format {
        Sniffed::Png => keep(bytes, ImageFormat::Png, "image/png", name),
        Sniffed::Jpeg => keep(bytes, ImageFormat::Jpeg, "image/jpeg", name),
        Sniffed::Webp => keep(bytes, ImageFormat::WebP, "image/webp", name),
        Sniffed::Gif => convert_gif(&bytes, name),
        // A TIFF may hold several pages, as a GIF holds several frames; the first is the
        // picture, and the decoder reads no other.
        Sniffed::Tiff => convert(&bytes, ImageFormat::Tiff, name),
        Sniffed::Bmp => convert(&bytes, ImageFormat::Bmp, name),
    }
}

/// The formats read from a picture's first bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Sniffed {
    Png,
    Jpeg,
    Webp,
    Gif,
    Tiff,
    Bmp,
}

/// The format `bytes` are, from their signature alone.
fn sniff(bytes: &[u8]) -> Option<Sniffed> {
    if bytes.starts_with(&[0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a]) {
        return Some(Sniffed::Png);
    }
    // Every JPEG opens with a start-of-image marker followed by another marker, which is the
    // test the editor's own sniff applies (`teksilo::canvas::ImageFormat::sniff`).
    if bytes.starts_with(&[0xff, 0xd8, 0xff]) {
        return Some(Sniffed::Jpeg);
    }
    if bytes.len() >= 12 && bytes.starts_with(b"RIFF") && &bytes[8..12] == b"WEBP" {
        return Some(Sniffed::Webp);
    }
    if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") {
        return Some(Sniffed::Gif);
    }
    if bytes.starts_with(b"II*\0") || bytes.starts_with(b"MM\0*") {
        return Some(Sniffed::Tiff);
    }
    // Two bytes, then the file's own length and a reserved zero: checked in full, since `BM`
    // alone opens a great deal of text.
    if bytes.len() >= 14 && bytes.starts_with(b"BM") && bytes[6..10] == [0, 0, 0, 0] {
        return Some(Sniffed::Bmp);
    }
    None
}

/// A name for a format this cannot keep, so the review can say what the picture was.
fn unsupported_format_name(bytes: &[u8]) -> &'static str {
    // An enhanced metafile opens with record type 1 and carries ` EMF` forty bytes in.
    if bytes.len() >= 44 && bytes.starts_with(&[1, 0, 0, 0]) && &bytes[40..44] == b" EMF" {
        return "EMF";
    }
    // A Windows metafile: the placeable header's key, or a bare header's type and size.
    if bytes.starts_with(&[0xd7, 0xcd, 0xc6, 0x9a])
        || bytes.starts_with(&[1, 0, 9, 0])
        || bytes.starts_with(&[2, 0, 9, 0])
    {
        return "WMF";
    }
    if bytes.starts_with(b"%PDF") {
        return "PDF";
    }
    if bytes.starts_with(b"%!PS") || bytes.starts_with(&[0xc5, 0xd0, 0xd3, 0xc6]) {
        return "EPS";
    }
    if bytes.len() >= 12 && &bytes[4..8] == b"ftyp" {
        return match &bytes[8..12] {
            b"avif" | b"avis" => "AVIF",
            b"heic" | b"heix" | b"mif1" | b"msf1" | b"hevc" => "HEIC",
            _ => "unknown",
        };
    }
    if bytes.starts_with(&[0, 0, 1, 0]) {
        return "ICO";
    }
    // SVG is text: an XML declaration or the element itself, after whatever white space or
    // byte-order mark leads it. Only the first kilobyte is looked at.
    let head = &bytes[..bytes.len().min(1024)];
    let text = String::from_utf8_lossy(head);
    let text = text.trim_start_matches('\u{feff}').trim_start();
    if text.starts_with("<svg") || (text.starts_with("<?xml") && text.contains("<svg")) {
        return "SVG";
    }
    "unknown"
}

/// The `image` crate's limits, as every read here sets them.
fn limits() -> Limits {
    let mut limits = Limits::default();
    limits.max_image_width = Some(MAX_PICTURE_SIDE);
    limits.max_image_height = Some(MAX_PICTURE_SIDE);
    limits.max_alloc = Some(MAX_DECODE_BYTES);
    limits
}

/// A decoder error as the refusal it is: a limit is a picture too large, anything else one
/// that does not read.
fn refusal(error: image::ImageError) -> PictureRefusal {
    match error {
        image::ImageError::Limits(_) => PictureRefusal::TooLarge,
        _ => PictureRefusal::Unreadable,
    }
}

/// Refuse dimensions past the pixel ceiling, or with no pixels at all.
fn check_size(width: u32, height: u32) -> Result<(), PictureRefusal> {
    if width == 0 || height == 0 {
        return Err(PictureRefusal::Unreadable);
    }
    if width > MAX_PICTURE_SIDE
        || height > MAX_PICTURE_SIDE
        || u64::from(width) * u64::from(height) > MAX_PICTURE_PIXELS
    {
        return Err(PictureRefusal::TooLarge);
    }
    Ok(())
}

/// Whether `orientation` turns a picture a quarter, so that its sides swap.
fn turns_a_quarter(orientation: Orientation) -> bool {
    matches!(
        orientation,
        Orientation::Rotate90
            | Orientation::Rotate270
            | Orientation::Rotate90FlipH
            | Orientation::Rotate270FlipH
    )
}

/// Keep `bytes` as they are, reading only their header.
fn keep(
    bytes: Vec<u8>,
    format: ImageFormat,
    mime_type: &'static str,
    name: &str,
) -> Result<Ingested, PictureRefusal> {
    let (width, height) = {
        let mut reader = ImageReader::with_format(Cursor::new(&bytes[..]), format);
        reader.limits(limits());
        let mut decoder = reader.into_decoder().map_err(refusal)?;
        let (width, height) = decoder.dimensions();
        check_size(width, height)?;
        // Only a JPEG is turned by the editor, so only a JPEG's sides are swapped here. An
        // orientation that cannot be read is no orientation: the editor reads none either.
        let turned = format == ImageFormat::Jpeg
            && decoder.orientation().map(turns_a_quarter).unwrap_or(false);
        if turned {
            (height, width)
        } else {
            (width, height)
        }
    };
    let content_hash = content_hash(&bytes);
    Ok(Ingested {
        picture: IngestedPicture {
            content_hash,
            mime_type,
            width,
            height,
            file_name: file_name_for(name, mime_type),
            bytes,
        },
        first_frame_only: false,
    })
}

/// Decode `bytes` as `format`, turn it upright, and keep it as PNG.
fn convert(bytes: &[u8], format: ImageFormat, name: &str) -> Result<Ingested, PictureRefusal> {
    let mut reader = ImageReader::with_format(Cursor::new(bytes), format);
    reader.limits(limits());
    let mut decoder = reader.into_decoder().map_err(refusal)?;
    let (width, height) = decoder.dimensions();
    check_size(width, height)?;
    let orientation = decoder.orientation().unwrap_or(Orientation::NoTransforms);
    // Refused before a pixel is allocated: past this, the decoder's own copy or the turned
    // one would take the conversion past its ceiling (see `MAX_DECODE_BYTES`).
    let decoded = decoder.total_bytes();
    if decoded > MAX_DECODED_BYTES {
        return Err(PictureRefusal::TooLarge);
    }
    // What `ImageReader::decode` does before it decodes: reserve the whole output buffer, then
    // hand the decoder what is left, so its own working buffers are held to the remainder
    // rather than to the whole ceiling a second time.
    let mut budget = limits();
    budget
        .reserve(decoded)
        .map_err(|_| PictureRefusal::TooLarge)?;
    decoder.set_limits(budget).map_err(refusal)?;
    let mut image = DynamicImage::from_decoder(decoder).map_err(refusal)?;
    image.apply_orientation(orientation);
    as_png(image, name, false)
}

/// A GIF's first frame, as PNG, and whether the file holds a second.
///
/// Only the first frame is decoded, however many the file holds. Whether there is a second
/// is read from the file's structure ([`holds_a_second_frame`]), never by decoding it: a
/// decoded frame is the size of the whole picture however little of it the frame covers,
/// and the frame iterator composes each onto a canvas it keeps besides, so learning that a
/// second frame existed used to hold the picture three times over.
fn convert_gif(bytes: &[u8], name: &str) -> Result<Ingested, PictureRefusal> {
    let mut ingested = convert(bytes, ImageFormat::Gif, name)?;
    ingested.first_frame_only = holds_a_second_frame(bytes);
    Ok(ingested)
}

/// Whether the GIF `bytes` holds more than one frame, read from its blocks alone: the
/// header, the colour table, then extensions and image descriptors, each skipped by the
/// lengths it states, until a second image descriptor or the end.
///
/// A file whose structure breaks before a second frame is taken to hold one, as a decoder
/// would stop at the break.
fn holds_a_second_frame(bytes: &[u8]) -> bool {
    /// The data sub-blocks starting at `at`, skipped: each a length byte and that many bytes,
    /// ending at a zero length. Where the next block starts, or `None` past the end.
    fn skip_sub_blocks(bytes: &[u8], mut at: usize) -> Option<usize> {
        loop {
            let length = usize::from(*bytes.get(at)?);
            at += 1;
            if length == 0 {
                return Some(at);
            }
            at += length;
        }
    }
    /// How many bytes a colour table takes, from the packed field that announces it.
    fn colour_table(packed: u8) -> usize {
        if packed & 0x80 == 0 {
            0
        } else {
            3 << ((packed & 0x07) + 1)
        }
    }
    // Header (6) and logical screen descriptor (7), then the global colour table.
    let Some(&packed) = bytes.get(10) else {
        return false;
    };
    let mut at = 13 + colour_table(packed);
    let mut frames = 0usize;
    loop {
        match bytes.get(at) {
            // An extension: its label, then its sub-blocks.
            Some(0x21) => match skip_sub_blocks(bytes, at + 2) {
                Some(next) => at = next,
                None => return false,
            },
            // An image descriptor: nine bytes, a local colour table, the LZW code size, then
            // the frame's data as sub-blocks.
            Some(0x2c) => {
                frames += 1;
                if frames == 2 {
                    return true;
                }
                let Some(&packed) = bytes.get(at + 9) else {
                    return false;
                };
                match skip_sub_blocks(bytes, at + 10 + colour_table(packed) + 1) {
                    Some(next) => at = next,
                    None => return false,
                }
            }
            // The trailer, the end of the bytes, or something no GIF writes.
            _ => return false,
        }
    }
}

/// Encode a decoded picture as PNG, the way a converted one is kept.
fn as_png(
    picture: DynamicImage,
    name: &str,
    first_frame_only: bool,
) -> Result<Ingested, PictureRefusal> {
    // PNG holds eight and sixteen bits a channel; a TIFF may decode to floating point.
    let picture = match picture {
        DynamicImage::ImageRgb32F(_) | DynamicImage::ImageRgba32F(_) => {
            DynamicImage::ImageRgba16(picture.to_rgba16())
        }
        other => other,
    };
    let (width, height) = (picture.width(), picture.height());
    check_size(width, height)?;
    let mut png = Cursor::new(Vec::new());
    picture
        .write_to(&mut png, ImageFormat::Png)
        .map_err(|_| PictureRefusal::Unreadable)?;
    let bytes = png.into_inner();
    if bytes.len() as u64 > MAX_PICTURE_BYTES {
        return Err(PictureRefusal::TooLarge);
    }
    let content_hash = content_hash(&bytes);
    Ok(Ingested {
        picture: IngestedPicture {
            content_hash,
            mime_type: "image/png",
            width,
            height,
            file_name: file_name_for(name, "image/png"),
            bytes,
        },
        first_frame_only,
    })
}

/// The last segment of `name`, with the extension the stored format is known by.
///
/// A converted TIFF is a PNG now, and a file mislabelled `.png` that is really a JPEG is a
/// JPEG: the name an export writes has to say what the bytes are. An extension already right
/// for the format is left as the writer spelled it (`.jpeg`, `.JPG`).
fn file_name_for(name: &str, mime_type: &str) -> String {
    let segment = name
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or_default()
        .chars()
        .filter(|c| !c.is_control())
        .collect::<String>();
    let segment = segment.trim();
    let (stem, extension) = match segment.rsplit_once('.') {
        Some((stem, extension)) if !stem.is_empty() => (stem, Some(extension)),
        _ => (segment, None),
    };
    let stem = if stem.is_empty() { "picture" } else { stem };
    let fits = |extension: &str| {
        let extension = extension.to_ascii_lowercase();
        match mime_type {
            "image/jpeg" => extension == "jpg" || extension == "jpeg",
            "image/png" => extension == "png",
            "image/webp" => extension == "webp",
            _ => false,
        }
    };
    match extension {
        Some(extension) if fits(extension) => format!("{stem}.{extension}"),
        _ => format!("{stem}.{}", skrib_format::media::extension_for(mime_type)),
    }
}

// ---------------------------------------------------------------------------
// One document's pictures
// ---------------------------------------------------------------------------

/// Where a picture landed, as the prose refers to it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlacedPicture {
    /// `assets/<hash>.<ext>`, the name the prose gives it.
    pub src: String,
    /// Its size as the editor shows it, for a reference that states none of its own.
    pub width: u32,
    pub height: u32,
}

/// What asking for one picture came to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Brought {
    pub result: Result<PlacedPicture, PictureRefusal>,
    /// This is the first time the document named this picture. A refusal and a kept first
    /// frame are reported then, and only then: a logo on every chapter is one finding.
    pub first_time: bool,
    /// It was an animation and only its first frame was kept.
    pub first_frame_only: bool,
}

/// Every picture one document brings, each once.
///
/// Asked once per place the document shows a picture, and keyed by where the bytes come from,
/// so a picture shown on every chapter is read and checked once. Kept by content, so two
/// names for the same bytes are one picture: the project would store one file either way.
#[derive(Debug)]
pub struct DocumentPictures {
    pictures: Vec<IngestedPicture>,
    by_hash: HashMap<String, usize>,
    by_source: HashMap<String, Result<usize, PictureRefusal>>,
    /// Bytes held so far, against `budget`.
    held: u64,
    budget: u64,
}

impl Default for DocumentPictures {
    fn default() -> Self {
        Self::with_budget(MAX_DOCUMENT_PICTURE_BYTES)
    }
}

impl DocumentPictures {
    pub fn new() -> Self {
        Self::default()
    }

    /// A document whose pictures may hold `budget` bytes together, rather than
    /// [`MAX_DOCUMENT_PICTURE_BYTES`]. For a test, which cannot afford a gigabyte.
    pub fn with_budget(budget: u64) -> Self {
        DocumentPictures {
            pictures: Vec::new(),
            by_hash: HashMap::new(),
            by_source: HashMap::new(),
            held: 0,
            budget,
        }
    }

    /// The picture at `source`, read with `read` the first time it is asked for.
    ///
    /// `source` is whatever names the bytes unambiguously in this document: the member of the
    /// package, the path under the document's folder. `name` is the name to keep for it.
    pub fn bring(
        &mut self,
        source: &str,
        name: &str,
        read: impl FnOnce() -> Result<Vec<u8>, PictureRefusal>,
    ) -> Brought {
        if let Some(known) = self.by_source.get(source) {
            return Brought {
                result: known.map(|index| self.placed(index)),
                first_time: false,
                first_frame_only: false,
            };
        }
        // Read even with the budget spent: one picture at a time is never more than
        // `MAX_PICTURE_BYTES`, and one the document already holds costs nothing more.
        let outcome = read().and_then(|bytes| ingest(bytes, name));
        let (stored, first_frame_only) = match outcome {
            Ok(ingested) => {
                let hash = ingested.picture.content_hash.clone();
                let size = ingested.picture.bytes.len() as u64;
                match self.by_hash.get(&hash) {
                    Some(index) => (Ok(*index), ingested.first_frame_only),
                    None if self.held.saturating_add(size) > self.budget => {
                        (Err(PictureRefusal::DocumentFull), false)
                    }
                    None => {
                        self.pictures.push(ingested.picture);
                        let index = self.pictures.len() - 1;
                        self.by_hash.insert(hash, index);
                        self.held = self.held.saturating_add(size);
                        (Ok(index), ingested.first_frame_only)
                    }
                }
            }
            Err(refused) => (Err(refused), false),
        };
        self.by_source.insert(source.to_string(), stored);
        Brought {
            result: stored.map(|index| self.placed(index)),
            first_time: true,
            first_frame_only,
        }
    }

    fn placed(&self, index: usize) -> PlacedPicture {
        let picture = &self.pictures[index];
        PlacedPicture {
            src: picture.relative_path(),
            width: picture.width,
            height: picture.height,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.pictures.is_empty()
    }

    /// Every picture kept, in the order the document first showed each.
    pub fn into_pictures(self) -> Vec<IngestedPicture> {
        self.pictures
    }
}

/// The `assets/…` pictures a document's stored text names, each once: what a row that holds
/// that text will need in the project.
///
/// Its prose, its epigraphs and its footnotes, which are everything a scanned document hands
/// the plan. A picture a scanner kept that none of them names (one in a heading, say) is not
/// here, and nothing needs to store it.
pub fn referenced_by(document: &crate::SourceDocument) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut add = |djot: &str| {
        for path in skrib_format::media::referenced_paths(djot) {
            if !out.contains(&path) {
                out.push(path);
            }
        }
    };
    for block in &document.blocks {
        match block {
            crate::SourceBlock::Prose { djot, .. } | crate::SourceBlock::Epigraph { djot, .. } => {
                add(djot)
            }
            crate::SourceBlock::Heading { .. } | crate::SourceBlock::SceneBreak { .. } => {}
        }
    }
    for note in &document.footnotes {
        add(&note.body);
    }
    out
}

// ---------------------------------------------------------------------------
// Where the bytes come from
// ---------------------------------------------------------------------------

/// What a picture reference names, before anything is read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reference {
    /// On the web.
    Remote,
    /// Somewhere outside the document and its folder.
    Outside,
    /// Carried in the reference itself, a `data:` address.
    Inline(Result<Vec<u8>, PictureRefusal>),
    /// A path below the document's folder, as its segments. Never empty, never `..`.
    Relative(Vec<String>),
    /// Nothing at all.
    Empty,
}

/// A picture reference as the review names it: as the document wrote it, except that a
/// `data:` address is named by its type alone and anything longer than a path is cut.
///
/// A `data:` address carries the picture itself, as many megabytes as it has, and the
/// review shows the name on a line of its own: the whole address arrived in the plan and on
/// screen when such a picture could not come.
pub fn reference_label(target: &str) -> String {
    const LONGEST: usize = 200;
    let target = target.trim();
    if let Some((scheme, rest)) = target.split_once(':')
        && scheme.eq_ignore_ascii_case("data")
    {
        let kind: String = rest
            .split([',', ';'])
            .next()
            .unwrap_or_default()
            .trim()
            .chars()
            .take(64)
            .collect();
        return format!("data:{kind}");
    }
    if target.chars().count() <= LONGEST {
        return target.to_string();
    }
    let mut cut: String = target.chars().take(LONGEST).collect();
    cut.push('…');
    cut
}

/// Read a picture reference as the document wrote it: a URL, possibly percent-encoded, or a
/// relative path.
pub fn classify_reference(target: &str) -> Reference {
    let target = target.trim();
    if target.is_empty() {
        return Reference::Empty;
    }
    if let Some((scheme, rest)) = scheme_of(target) {
        return match scheme.to_ascii_lowercase().as_str() {
            "http" | "https" | "ftp" | "ftps" => Reference::Remote,
            "data" => Reference::Inline(decode_data_url(rest)),
            // `file:` and every other scheme name something this document does not hold.
            _ => Reference::Outside,
        };
    }
    // A Windows path spelled with backslashes, a UNC share, or an absolute path.
    if target.starts_with('/') || target.starts_with('\\') {
        return Reference::Outside;
    }
    let decoded = percent_decode(target);
    // A fragment or a query means nothing to a file on disk.
    let path = decoded
        .split(['#', '?'])
        .next()
        .unwrap_or_default()
        .to_string();
    let mut segments = Vec::new();
    for segment in path.split('/') {
        match segment {
            "" | "." => continue,
            ".." => return Reference::Outside,
            // A backslash is a separator on Windows and a drive letter or a stream name
            // follows a colon there: neither is a name inside this folder.
            s if s.contains(['\\', ':', '\0']) => return Reference::Outside,
            s => segments.push(s.to_string()),
        }
    }
    if segments.is_empty() {
        return Reference::Empty;
    }
    Reference::Relative(segments)
}

/// `scheme` and what follows its colon, when `target` opens with one. A single letter is a
/// drive (`C:\`), not a scheme.
fn scheme_of(target: &str) -> Option<(&str, &str)> {
    let (scheme, rest) = target.split_once(':')?;
    let valid = scheme.len() > 1
        && scheme.starts_with(|c: char| c.is_ascii_alphabetic())
        && scheme
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.'));
    if valid {
        Some((scheme, rest))
    } else if scheme.len() == 1 && scheme.starts_with(|c: char| c.is_ascii_alphabetic()) {
        // `C:\photos\x.png`, `C:/photos/x.png`: absolute on Windows.
        Some(("file", rest))
    } else {
        None
    }
}

/// `%xx` escapes decoded, when what they decode to is UTF-8; `text` unchanged otherwise.
fn percent_decode(text: &str) -> String {
    if !text.contains('%') {
        return text.to_string();
    }
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%'
            && index + 2 < bytes.len()
            && let (Some(high), Some(low)) = (hex(bytes[index + 1]), hex(bytes[index + 2]))
        {
            out.push(high * 16 + low);
            index += 3;
            continue;
        }
        out.push(bytes[index]);
        index += 1;
    }
    String::from_utf8(out).unwrap_or_else(|_| text.to_string())
}

fn hex(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

/// The bytes a `data:` address carries, after its colon.
fn decode_data_url(rest: &str) -> Result<Vec<u8>, PictureRefusal> {
    let Some((header, payload)) = rest.split_once(',') else {
        return Err(PictureRefusal::Unreadable);
    };
    // Base64 is four characters for three bytes: past this many, the bytes would be past
    // the ceiling, and nothing is decoded to find out.
    if payload.len() as u64 > MAX_PICTURE_BYTES / 3 * 4 + 4 {
        return Err(PictureRefusal::TooLarge);
    }
    if header
        .split(';')
        .any(|part| part.trim().eq_ignore_ascii_case("base64"))
    {
        decode_base64(&percent_decode(payload)).ok_or(PictureRefusal::Unreadable)
    } else {
        Ok(percent_decode(payload).into_bytes())
    }
}

/// Standard base64, white space ignored and padding optional, as a `data:` address and an
/// OpenDocument `office:binary-data` element write it. `None` for anything else.
pub fn decode_base64(text: &str) -> Option<Vec<u8>> {
    decode_base64_parts(std::iter::once(text))
}

/// A picture written as base64 in `parts`, the text nodes of one element in order, decoded
/// only once it is known to fit: past [`MAX_PICTURE_BYTES`] it is refused as too large
/// without a byte of it being decoded or the text being gathered into one string.
///
/// For OpenDocument's `office:binary-data`, whose size nothing else bounds: a flat `.fodt` is
/// read whole, however large, and its pictures are written inside it.
pub fn decode_base64_picture<'a, I>(parts: I) -> Result<Vec<u8>, PictureRefusal>
where
    I: Iterator<Item = &'a str> + Clone,
{
    let digits: u64 = parts
        .clone()
        .map(|part| {
            part.bytes()
                .filter(|byte| !byte.is_ascii_whitespace() && *byte != b'=')
                .count() as u64
        })
        .sum();
    // Four characters for three bytes.
    if digits / 4 * 3 > MAX_PICTURE_BYTES {
        return Err(PictureRefusal::TooLarge);
    }
    decode_base64_parts(parts).ok_or(PictureRefusal::Unreadable)
}

/// [`decode_base64`] over text in several parts, as the text nodes of one element hold it.
fn decode_base64_parts<'a>(parts: impl Iterator<Item = &'a str> + Clone) -> Option<Vec<u8>> {
    fn value(c: u8) -> Option<u32> {
        match c {
            b'A'..=b'Z' => Some(u32::from(c - b'A')),
            b'a'..=b'z' => Some(u32::from(c - b'a') + 26),
            b'0'..=b'9' => Some(u32::from(c - b'0') + 52),
            b'+' => Some(62),
            b'/' => Some(63),
            _ => None,
        }
    }
    let length: usize = parts.clone().map(str::len).sum();
    let mut out = Vec::with_capacity(length / 4 * 3);
    let mut buffer = 0u32;
    let mut bits = 0u32;
    let mut padding = 0usize;
    for byte in parts.flat_map(str::bytes) {
        if byte.is_ascii_whitespace() {
            continue;
        }
        if byte == b'=' {
            padding += 1;
            continue;
        }
        // A character after the padding is not base64.
        if padding > 0 {
            return None;
        }
        buffer = (buffer << 6) | value(byte)?;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((buffer >> bits) as u8);
            buffer &= (1 << bits) - 1;
        }
    }
    if padding > 2 {
        return None;
    }
    Some(out)
}

/// The folder a document was read from, where the pictures it names by relative path live.
///
/// Holds the folder's canonical path, and reads nothing outside it: every name is resolved
/// one segment at a time from there, a segment that is a symbolic link is refused rather than
/// followed, and only a regular file is read. See the module doc for why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentFolder {
    root: PathBuf,
}

impl DocumentFolder {
    /// The folder holding the document at `path`, or `None` when `path` names no folder this
    /// can stand on: a relative path, whose folder would depend on where the app happened to
    /// be started, or one that is not there.
    pub fn of_document(path: &Path) -> Option<Self> {
        if !path.is_absolute() {
            return None;
        }
        let parent = path.parent()?;
        let root = std::fs::canonicalize(parent).ok()?;
        root.is_dir().then_some(DocumentFolder { root })
    }

    /// The folder itself, as it was resolved.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// The bytes of the regular file at `segments` below the folder.
    pub fn read(&self, segments: &[String]) -> Result<Vec<u8>, PictureRefusal> {
        let mut path = self.root.clone();
        let last = segments.len().saturating_sub(1);
        let mut found: Option<std::fs::Metadata> = None;
        for (index, segment) in segments.iter().enumerate() {
            // `classify_reference` already refused these; a caller passing its own segments
            // is held to the same rule.
            let mut components = Path::new(segment).components();
            let single = matches!(
                (components.next(), components.next()),
                (Some(Component::Normal(_)), None)
            );
            if !single {
                return Err(PictureRefusal::Outside);
            }
            path.push(segment);
            let metadata = match std::fs::symlink_metadata(&path) {
                Ok(metadata) => metadata,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                    return Err(PictureRefusal::Missing);
                }
                Err(_) => return Err(PictureRefusal::Unreadable),
            };
            if metadata.file_type().is_symlink() {
                return Err(PictureRefusal::Outside);
            }
            if index < last {
                if !metadata.is_dir() {
                    return Err(PictureRefusal::Missing);
                }
            } else {
                found = Some(metadata);
            }
        }
        let metadata = found.ok_or(PictureRefusal::Missing)?;
        if !metadata.is_file() {
            return Err(PictureRefusal::Unreadable);
        }
        if metadata.len() > MAX_PICTURE_BYTES {
            return Err(PictureRefusal::TooLarge);
        }
        let file = std::fs::File::open(&path).map_err(|_| PictureRefusal::Unreadable)?;
        // The name was checked a moment ago and opened now. Should it have been replaced in
        // between, by a link to a file elsewhere, the file opened is not the one checked.
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            let opened = file.metadata().map_err(|_| PictureRefusal::Unreadable)?;
            if opened.dev() != metadata.dev() || opened.ino() != metadata.ino() {
                return Err(PictureRefusal::Outside);
            }
        }
        // That comparison holds the last name to the file opened, and not the folders above
        // it: a folder of the path swapped for a link to another between being checked and
        // the file below it being looked up is followed by both the look-up and the open,
        // which then agree about a file outside. Measured, a writer's folder being swapped
        // this way under a running import leaked the outside file a few times in a million
        // reads. The kernel's own name for what was opened settles it where it can be asked.
        #[cfg(target_os = "linux")]
        {
            use std::os::fd::AsRawFd;
            if let Ok(opened_at) = std::fs::read_link(format!("/proc/self/fd/{}", file.as_raw_fd()))
                && opened_at != path
            {
                return Err(PictureRefusal::Outside);
            }
        }
        let mut bytes = Vec::new();
        file.take(MAX_PICTURE_BYTES + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| PictureRefusal::Unreadable)?;
        if bytes.len() as u64 > MAX_PICTURE_BYTES {
            return Err(PictureRefusal::TooLarge);
        }
        Ok(bytes)
    }
}

/// The pictures stored inside a zip container, a `.docx` or an `.odt`.
///
/// Read through a guard of its own ([`skrib_format::zip_guard`]), so a picture member built
/// to unpack past its budget is refused as that one picture, and the text beside it still
/// imports.
#[cfg(any(feature = "odt", feature = "docx"))]
pub struct PackagePictures<'a> {
    archive: Option<zip::ZipArchive<Cursor<&'a [u8]>>>,
    guard: skrib_format::zip_guard::ZipGuard,
}

#[cfg(any(feature = "odt", feature = "docx"))]
impl<'a> PackagePictures<'a> {
    /// The pictures of the container `bytes`. Not a zip at all is a container holding none.
    pub fn open(bytes: &'a [u8]) -> Self {
        let limits = crate::sources::DOCUMENT_ZIP_LIMITS;
        match skrib_format::zip_guard::ZipGuard::open(limits, Cursor::new(bytes)) {
            Ok((archive, guard)) => PackagePictures {
                archive: Some(archive),
                guard,
            },
            Err(_) => PackagePictures {
                archive: None,
                guard: skrib_format::zip_guard::ZipGuard::new(limits, bytes.len() as u64),
            },
        }
    }

    /// The bytes of the member `name`.
    pub fn read(&mut self, name: &str) -> Result<Vec<u8>, PictureRefusal> {
        let archive = self.archive.as_mut().ok_or(PictureRefusal::Missing)?;
        let mut entry = match archive.by_name(name) {
            Ok(entry) => entry,
            Err(zip::result::ZipError::FileNotFound) => return Err(PictureRefusal::Missing),
            Err(_) => return Err(PictureRefusal::Unreadable),
        };
        if entry.is_dir() {
            return Err(PictureRefusal::Unreadable);
        }
        if entry.size() > MAX_PICTURE_BYTES {
            return Err(PictureRefusal::TooLarge);
        }
        let mut out = Capped {
            bytes: Vec::new(),
            over: false,
        };
        match self.guard.copy(&mut entry, &mut out) {
            Ok(_) => Ok(out.bytes),
            Err(_) if out.over => Err(PictureRefusal::TooLarge),
            Err(error) if skrib_format::zip_guard::refused(&error).is_some() => {
                Err(PictureRefusal::TooLarge)
            }
            Err(_) => Err(PictureRefusal::Unreadable),
        }
    }
}

/// A buffer that refuses to grow past [`MAX_PICTURE_BYTES`], whatever the member's header
/// declared: the declared size is checked first, and this is what holds a header that lied.
#[cfg(any(feature = "odt", feature = "docx"))]
struct Capped {
    bytes: Vec<u8>,
    over: bool,
}

#[cfg(any(feature = "odt", feature = "docx"))]
impl std::io::Write for Capped {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if self.bytes.len() as u64 + buf.len() as u64 > MAX_PICTURE_BYTES {
            self.over = true;
            return Err(std::io::Error::other("a picture past its ceiling"));
        }
        self.bytes.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests;
