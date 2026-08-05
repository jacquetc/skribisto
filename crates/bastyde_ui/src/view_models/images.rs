// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `ImagesViewModel` — putting a picture into a manuscript, and getting it back.
//!
//! Inserting an image is six steps that have to happen in one order, because
//! each depends on the last: read the file, identify and decode it (which is
//! also the only honest way to learn its real dimensions), hash the bytes,
//! write them into the project's media directory, record an `Asset` row, and
//! finally put a reference into the prose.
//!
//! The reference is Djot — `![alt](assets/<hash>.<ext>){width=W height=H}` —
//! not a direct call into the editor's image API. That is deliberate: Djot is
//! what `Content.data` stores and what a reload parses, so inserting through it
//! means the thing on screen and the thing that survives a save are produced by
//! the same code path. An insertion route that bypassed it would work until the
//! writer closed the project.
//!
//! ## Why the hash is the name
//!
//! An asset is addressed by the blake3 of its bytes. Two consequences the rest
//! of the design leans on: inserting the same photograph twice costs one copy,
//! and an unchanged image can be recognised on save without reading it back.
//! The original filename is kept for provenance and for naming the file an
//! export writes — never as identity.

use std::path::{Path, PathBuf};

/// What an image would cost, and what the writer chose to do about it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizePolicy {
    /// Store the writer's file byte for byte.
    KeepOriginal,
    /// Re-encode so neither side exceeds [`DOWNSCALE_MAX_EDGE`].
    Downscale,
}

/// Longest edge, in pixels, that "optimise" reduces an image to.
///
/// Comfortably above any realistic print use of a reference photograph inside a
/// manuscript, and far below what a modern camera produces.
pub const DOWNSCALE_MAX_EDGE: u32 = 2560;

/// Above this many pixels, inserting asks before storing the original.
///
/// A threshold in *pixels* rather than bytes: bytes vary by an order of
/// magnitude with the encoder's quality setting, while pixel count is what
/// actually predicts the memory and decode cost the project will carry.
pub const LARGE_IMAGE_PIXELS: u64 = 4_000_000;

/// A file the writer chose, examined but not yet stored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingImage {
    pub bytes: Vec<u8>,
    pub mime_type: String,
    pub width: u32,
    pub height: u32,
    pub file_name: String,
    pub content_hash: String,
}

impl PendingImage {
    /// Whether this image is big enough to be worth asking about.
    pub fn is_large(&self) -> bool {
        u64::from(self.width) * u64::from(self.height) > LARGE_IMAGE_PIXELS
    }

    /// The bundle-relative name this image will be stored under.
    pub fn relative_path(&self) -> String {
        skrib_format::media::asset_relpath(
            &self.content_hash,
            &skrib_format::media::extension_for(&self.mime_type),
        )
    }

    /// The Djot that references it, with its display size.
    ///
    /// Size is written even at natural dimensions: it is what a later resize
    /// edits, and a reference with no size would lay out at whatever the file
    /// happens to be, which for a phone photo overruns the column.
    pub fn djot(&self, alt: &str, width: u32, height: u32) -> String {
        format!(
            "![{}]({}){{width={width} height={height}}}",
            escape_djot_alt(alt),
            self.relative_path()
        )
    }
}

use skrib_format::media::escape_djot_alt;

/// Errors an insertion can fail with, each phrased for a writer rather than a log.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageError {
    Unreadable(String),
    Undecodable(String),
    NotWritten(String),
}

impl std::fmt::Display for ImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unreadable(p) => write!(f, "could not read {p}"),
            Self::Undecodable(p) => write!(f, "{p} is not a PNG, JPEG or WebP"),
            Self::NotWritten(p) => write!(f, "could not store the image in {p}"),
        }
    }
}

/// Examine a chosen file without storing anything.
///
/// Decoding here is not wasted work: it is the only way to learn the image's
/// true pixel dimensions (a file's own metadata can disagree with its content,
/// and a JPEG's EXIF rotation changes which side is which), and it is also the
/// check that the bytes are an image at all — better to find out before writing
/// them into the project than at the first paint.
pub fn examine(path: &Path) -> Result<PendingImage, ImageError> {
    let display = path.display().to_string();
    let bytes = std::fs::read(path).map_err(|_| ImageError::Unreadable(display.clone()))?;
    examine_bytes(
        bytes,
        path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("image")
            .to_string(),
    )
    .ok_or(ImageError::Undecodable(display))
}

/// The same, for bytes that did not come from a file — a paste, or a drop that
/// carried its payload inline.
pub fn examine_bytes(bytes: Vec<u8>, file_name: String) -> Option<PendingImage> {
    let format = bastyde::canvas::ImageFormat::sniff(&bytes)?;
    let decoded = bastyde::canvas::RasterIcon::decode(&bytes).ok()?;
    Some(PendingImage {
        content_hash: blake3::hash(&bytes).to_hex().to_string(),
        mime_type: format.mime_type().to_string(),
        width: decoded.width(),
        height: decoded.height(),
        file_name,
        bytes,
    })
}

/// Apply the writer's size choice, returning the image as it will be stored.
///
/// Downscaling re-encodes as PNG: it is lossless, so optimising twice cannot
/// degrade an image the way repeated JPEG re-encoding would, and the caller has
/// already decided the pixel count is what needs reducing.
pub fn apply_policy(image: PendingImage, policy: SizePolicy) -> PendingImage {
    if policy == SizePolicy::KeepOriginal {
        return image;
    }
    let Ok(decoded) = bastyde::canvas::RasterIcon::decode(&image.bytes) else {
        return image;
    };
    let Some(small) = decoded.downsample_to_max(DOWNSCALE_MAX_EDGE) else {
        return image; // already within bounds
    };
    let Some(png) = encode_png(&small) else {
        return image;
    };
    PendingImage {
        content_hash: blake3::hash(&png).to_hex().to_string(),
        mime_type: "image/png".to_string(),
        width: small.width(),
        height: small.height(),
        file_name: image.file_name,
        bytes: png,
    }
}

fn encode_png(icon: &bastyde::canvas::RasterIcon) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    {
        let mut enc = png::Encoder::new(&mut out, icon.width(), icon.height());
        enc.set_color(png::ColorType::Rgba);
        enc.set_depth(png::BitDepth::Eight);
        let mut writer = enc.write_header().ok()?;
        writer.write_image_data(icon.pixels()).ok()?;
    }
    Some(out)
}

/// Write an image into the project's media directory.
///
/// Content-addressed, so a file already there under this name holds these
/// bytes and re-writing it would be pure cost — which is also what makes
/// inserting the same photograph twice free.
pub fn store(image: &PendingImage, media_dir: &Path) -> Result<PathBuf, ImageError> {
    let dir_display = media_dir.display().to_string();
    std::fs::create_dir_all(media_dir).map_err(|_| ImageError::NotWritten(dir_display.clone()))?;
    let ext = skrib_format::media::extension_for(&image.mime_type);
    let path = media_dir.join(format!("{}.{ext}", image.content_hash));
    if !path.exists() {
        std::fs::write(&path, &image.bytes).map_err(|_| ImageError::NotWritten(dir_display))?;
    }
    Ok(path)
}

/// Make `image` the book's cover, or clear the cover when it is `None`.
///
/// At most one asset per Work carries the mark, so setting one clears the rest —
/// enforced here, at the only place that sets it, rather than by a constraint
/// the schema cannot express.
///
/// The previous cover's *bytes* are left alone. It may still be referenced by
/// the prose, and even if it is not, reclaiming storage is always an explicit
/// act in this design (see [`store`]); silently deleting a picture because the
/// writer chose a different cover would be the one irreversible step in an
/// otherwise reversible feature.
pub fn set_cover(
    app_ctx: &frontend::AppContext,
    stack_id: Option<u64>,
    work_id: Option<u64>,
    image: Option<&PendingImage>,
) -> Result<(), ()> {
    use frontend::commands::asset_commands;

    let existing = asset_commands::get_all_asset(app_ctx).map_err(|_| ())?;

    // Demote first. If the chosen picture is already in the project (the same
    // photograph inserted earlier, or the same cover chosen twice), promoting it
    // afterwards is what leaves exactly one marked.
    let keep = image.map(|i| i.content_hash.clone());
    for asset in existing.iter().filter(|a| a.is_cover) {
        if Some(&asset.content_hash) == keep.as_ref() {
            continue;
        }
        let mut dto = to_update(asset);
        dto.is_cover = false;
        asset_commands::update_asset(app_ctx, stack_id, &dto).map_err(|_| ())?;
    }

    let Some(image) = image else {
        return Ok(());
    };

    if let Some(asset) = existing
        .iter()
        .find(|a| a.content_hash == image.content_hash)
    {
        if !asset.is_cover {
            let mut dto = to_update(asset);
            dto.is_cover = true;
            asset_commands::update_asset(app_ctx, stack_id, &dto).map_err(|_| ())?;
        }
        return Ok(());
    }

    asset_commands::create_asset(
        app_ctx,
        stack_id,
        &frontend::asset::dtos::CreateAssetDto {
            created_at: Default::default(),
            updated_at: Default::default(),
            content_hash: image.content_hash.clone(),
            file_name: image.file_name.clone(),
            mime_type: image.mime_type.clone(),
            width: u64::from(image.width),
            height: u64::from(image.height),
            byte_size: image.bytes.len() as u64,
            alt: String::new(),
            is_cover: true,
        },
        work_id.unwrap_or_default(),
        -1,
    )
    .map(|_| ())
    .map_err(|_| ())
}

/// An `UpdateAssetDto` carrying an asset's current values, for a caller about to
/// change exactly one of them.
fn to_update(a: &frontend::asset::dtos::AssetDto) -> frontend::asset::dtos::UpdateAssetDto {
    frontend::asset::dtos::UpdateAssetDto {
        id: a.id,
        created_at: a.created_at,
        updated_at: a.updated_at,
        content_hash: a.content_hash.clone(),
        file_name: a.file_name.clone(),
        mime_type: a.mime_type.clone(),
        width: a.width,
        height: a.height,
        byte_size: a.byte_size,
        alt: a.alt.clone(),
        is_cover: a.is_cover,
    }
}

/// Supplies an editor with image bytes its document does not have.
///
/// Handed to `RichTextEditor::on_image_missing`, which asks when it meets a name
/// it cannot resolve. That is exactly what pasting a picture into a *second*
/// editor produces: the clipboard carries the reference, because an image's
/// pixels live on the document that owns it and a fragment is not a document.
/// The same hook covers a drop and an undo that brings a deleted image back.
///
/// Cheap to clone and safe to hold: it is a path, and the files under it are
/// content-addressed, so a name either resolves to the same bytes forever or to
/// nothing at all.
#[derive(Clone, Debug)]
pub struct ImageSource {
    media_dir: std::rc::Rc<PathBuf>,
}

impl ImageSource {
    pub fn new(media_dir: PathBuf) -> Self {
        Self {
            media_dir: std::rc::Rc::new(media_dir),
        }
    }

    /// The callback `RichTextEditor::on_image_missing` wants.
    pub fn resolver(&self) -> impl Fn(&str) -> Option<(String, Vec<u8>)> + 'static {
        let dir = self.media_dir.clone();
        move |name: &str| {
            // The prose names `assets/<hash>.<ext>`; the media directory holds
            // the file under that basename. Only the basename is used, so a
            // name that tried to walk out of the project cannot.
            let base = Path::new(name).file_name()?;
            let bytes = std::fs::read(dir.join(base)).ok()?;
            Some((mime_for_extension(name).to_string(), bytes))
        }
    }
}

/// Register every image a document's prose references, reading each from the
/// media directory.
///
/// Called when a document is opened, not only when one is inserted: after a
/// reload the anchors exist but the resource table is empty, so without this a
/// reopened project shows every image as a correctly-sized blank.
pub fn register_referenced(
    doc: &bastyde::text_document::TextDocument,
    djot: &str,
    media_dir: &Path,
) -> usize {
    let mut registered = 0;
    for relpath in referenced_paths(djot) {
        if doc.resource(&relpath).ok().flatten().is_some() {
            continue;
        }
        // The bundle path is `assets/<hash>.<ext>`; the media directory holds
        // the file under that same basename.
        let Some(base) = Path::new(&relpath).file_name() else {
            continue;
        };
        let Ok(bytes) = std::fs::read(media_dir.join(base)) else {
            // A picture whose file has gone is shown as its alt text rather
            // than failing the open — the prose still names it, so restoring
            // the file restores the image.
            continue;
        };
        if doc
            .add_resource(
                bastyde::text_document::ResourceType::Image,
                &relpath,
                mime_for_extension(&relpath),
                &bytes,
            )
            .is_ok()
        {
            registered += 1;
        }
    }
    registered
}

/// One image reference as the prose stores it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageRef {
    /// Position among the document's images, in document order. The identity
    /// that matters: the same picture may appear twice at two sizes, so a `src`
    /// alone cannot say which one the writer clicked.
    pub index: usize,
    pub src: String,
    pub alt: String,
    /// Display size from the reference's `{width=… height=…}`, `None` when it
    /// carries none — an image inserted by this app always does, one typed by
    /// hand may not.
    pub size: Option<(u32, u32)>,
}

impl ImageRef {
    /// This reference, rewritten with a new description and size.
    pub fn djot(&self, alt: &str, size: Option<(u32, u32)>) -> String {
        let mut out = format!("![{}]({})", escape_djot_alt(alt), self.src);
        if let Some((w, h)) = size {
            out.push_str(&format!("{{width={w} height={h}}}"));
        }
        out
    }

    /// The size scaled to `percent` of what it is now, never below one pixel.
    ///
    /// A percentage rather than two independent numbers because an image whose
    /// aspect ratio is edited by hand is almost always a mistake, and the one
    /// case where it is not — cropping — is not something a size field can do.
    pub fn scaled(&self, percent: u32) -> Option<(u32, u32)> {
        let (w, h) = self.size?;
        let f = f64::from(percent) / 100.0;
        Some((
            ((f64::from(w) * f).round() as u32).max(1),
            ((f64::from(h) * f).round() as u32).max(1),
        ))
    }
}

/// Every image reference in this Djot, in document order.
///
/// A scan rather than a parse, for the reason [`referenced_paths`] gives — and
/// on the same shape, so the two agree about what an image is. Unlike that one
/// it keeps every reference, including images pointing outside the project: a
/// writer can still describe and resize a picture they linked by hand.
pub fn parse_images(djot: &str) -> Vec<ImageRef> {
    let mut out: Vec<ImageRef> = Vec::new();
    let mut rest = djot;
    while let Some(open) = rest.find("![") {
        rest = &rest[open + 2..];
        let Some(close) = rest.find("](") else { break };
        let alt = unescape_djot_alt(&rest[..close]);
        let after = &rest[close + 2..];
        let Some(end) = after.find(')') else { break };
        let src = after[..end].to_string();
        let tail = &after[end + 1..];
        out.push(ImageRef {
            index: out.len(),
            src,
            alt,
            size: parse_size(tail),
        });
        rest = tail;
    }
    out
}

/// `{width=W height=H}` when `tail` opens with exactly that, else `None`.
///
/// Only an attribute block *immediately* following the reference counts: a `{`
/// later in the paragraph belongs to something else.
fn parse_size(tail: &str) -> Option<(u32, u32)> {
    let attrs = tail.strip_prefix('{')?;
    let attrs = &attrs[..attrs.find('}')?];
    let mut width = None;
    let mut height = None;
    for pair in attrs.split_whitespace() {
        match pair.split_once('=') {
            Some(("width", v)) => width = v.parse().ok(),
            Some(("height", v)) => height = v.parse().ok(),
            _ => {}
        }
    }
    Some((width?, height?))
}

/// Undo [`escape_djot_alt`], so a description round-trips through an edit.
fn unescape_djot_alt(alt: &str) -> String {
    let mut out = String::with_capacity(alt.len());
    let mut chars = alt.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(next) = chars.next() {
                out.push(next);
            }
        } else {
            out.push(c);
        }
    }
    out
}

/// The image at character `offset` in a document, given its addressable plain
/// text and its Djot.
///
/// An image is one `U+FFFC` in the plain text, so the number of them before
/// `offset` is the image's index — and images appear in the same order in both
/// views. That indirection is what makes this exact where matching on the `src`
/// would not be: a document may hold one picture in three places.
///
/// Depends on the plain text *keeping* its sentinels, which is what
/// `PlainTextExportOptions::addressable` guarantees and the presentation view
/// deliberately does not.
pub fn image_at(plain_text: &str, offset: usize, djot: &str) -> Option<ImageRef> {
    let mut index = 0usize;
    for (i, c) in plain_text.chars().enumerate() {
        if i >= offset {
            break;
        }
        if c == '\u{FFFC}' {
            index += 1;
        }
    }
    // The character *at* the offset must itself be an image, or the click landed
    // on ordinary text and there is nothing to edit.
    if plain_text.chars().nth(offset) != Some('\u{FFFC}') {
        return None;
    }
    parse_images(djot).into_iter().nth(index)
}

/// Every `assets/...` path referenced by an image in this Djot.
///
/// Lives in `skrib_format` because the exporter needs exactly the same answer:
/// which of a project's pictures a given piece of prose actually names.
pub use skrib_format::media::referenced_paths;

fn mime_for_extension(path: &str) -> &'static str {
    match path.rsplit('.').next().unwrap_or("") {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        "gif" => "image/gif",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn png(w: u32, h: u32) -> Vec<u8> {
        let mut buf = Vec::new();
        {
            let mut enc = png::Encoder::new(&mut buf, w, h);
            enc.set_color(png::ColorType::Rgba);
            enc.set_depth(png::BitDepth::Eight);
            let mut wr = enc.write_header().unwrap();
            wr.write_image_data(&[10u8, 20, 30, 255].repeat((w * h) as usize))
                .unwrap();
        }
        buf
    }

    #[test]
    fn examining_reads_the_real_dimensions_and_type() {
        let img = examine_bytes(png(40, 30), "photo.png".into()).expect("decodes");
        assert_eq!((img.width, img.height), (40, 30));
        assert_eq!(img.mime_type, "image/png");
        assert_eq!(img.file_name, "photo.png");
        assert_eq!(img.content_hash.len(), 64, "blake3 hex");
    }

    #[test]
    fn non_image_bytes_are_refused_before_anything_is_stored() {
        assert!(examine_bytes(b"just some text".to_vec(), "notes.txt".into()).is_none());
    }

    #[test]
    fn the_same_bytes_always_produce_the_same_name() {
        // What makes re-inserting a photograph free, and an unchanged asset
        // recognisable on save without reading it back.
        let a = examine_bytes(png(8, 8), "a.png".into()).unwrap();
        let b = examine_bytes(png(8, 8), "renamed.png".into()).unwrap();
        assert_eq!(a.content_hash, b.content_hash);
        assert_eq!(a.relative_path(), b.relative_path());
    }

    #[test]
    fn a_stored_image_lands_under_its_hash_and_is_not_rewritten() {
        let dir = tempfile::tempdir().unwrap();
        let img = examine_bytes(png(4, 4), "x.png".into()).unwrap();

        let path = store(&img, dir.path()).expect("stored");
        assert_eq!(
            path.file_name().unwrap().to_str().unwrap(),
            format!("{}.png", img.content_hash)
        );
        let first = std::fs::metadata(&path).unwrap().modified().unwrap();

        let again = store(&img, dir.path()).expect("stored again");
        assert_eq!(again, path);
        assert_eq!(std::fs::metadata(&path).unwrap().modified().unwrap(), first);
    }

    #[test]
    fn the_djot_reference_carries_alt_and_display_size() {
        let img = examine_bytes(png(4, 4), "x.png".into()).unwrap();
        let djot = img.djot("a blue square", 200, 150);
        assert!(djot.starts_with("![a blue square](assets/"));
        assert!(djot.ends_with("{width=200 height=150}"));
    }

    #[test]
    fn alt_text_cannot_break_out_of_its_brackets() {
        let img = examine_bytes(png(4, 4), "x.png".into()).unwrap();
        let djot = img.djot("a ] bracket and a \\ slash", 10, 10);
        // Re-reading it must find the whole description, not a truncated one.
        let alt_end = djot.find("](").expect("closes");
        assert!(djot[..alt_end].contains("\\]"), "{djot}");
        assert!(djot[..alt_end].contains("\\\\"), "{djot}");
    }

    #[test]
    fn only_a_big_image_is_worth_asking_about() {
        let small = examine_bytes(png(100, 100), "s.png".into()).unwrap();
        assert!(!small.is_large());
        let mut big = small.clone();
        big.width = 4000;
        big.height = 3000;
        assert!(big.is_large());
    }

    #[test]
    fn keeping_the_original_changes_nothing_at_all() {
        let img = examine_bytes(png(64, 64), "x.png".into()).unwrap();
        let after = apply_policy(img.clone(), SizePolicy::KeepOriginal);
        assert_eq!(after, img);
    }

    #[test]
    fn downscaling_bounds_the_long_edge_and_renames_the_asset() {
        // A different image is different bytes, so it must get a different
        // content hash — otherwise the original and the optimised copy would
        // collide in the media directory.
        let img = examine_bytes(png(3000, 1500), "big.png".into()).unwrap();
        let small = apply_policy(img.clone(), SizePolicy::Downscale);
        assert_eq!(small.width, DOWNSCALE_MAX_EDGE);
        assert_eq!(small.height, DOWNSCALE_MAX_EDGE / 2);
        assert_ne!(small.content_hash, img.content_hash);
        assert_eq!(small.file_name, "big.png", "provenance is kept");
    }

    #[test]
    fn downscaling_an_already_small_image_leaves_it_alone() {
        let img = examine_bytes(png(64, 64), "x.png".into()).unwrap();
        let after = apply_policy(img.clone(), SizePolicy::Downscale);
        assert_eq!(after.content_hash, img.content_hash);
    }

    // ── editing an image already in the prose ───────────────────────────

    const PROSE: &str = "Before ![a gull](assets/aa.png){width=320 height=240} after.";

    #[test]
    fn a_reference_parses_back_into_its_parts() {
        let images = parse_images(PROSE);
        assert_eq!(images.len(), 1);
        assert_eq!(images[0].src, "assets/aa.png");
        assert_eq!(images[0].alt, "a gull");
        assert_eq!(images[0].size, Some((320, 240)));
    }

    #[test]
    fn an_image_with_no_attributes_reports_no_size() {
        // Hand-typed Djot carries none; the editor's own insertions always do.
        let images = parse_images("![x](assets/a.png) and {not=an attribute block}");
        assert_eq!(images.len(), 1);
        assert_eq!(images[0].size, None);
    }

    #[test]
    fn a_description_survives_a_round_trip_through_an_edit() {
        // The escape exists so a `]` cannot close the description early; the
        // unescape is what makes editing one idempotent rather than piling up
        // backslashes every time the writer opens the dialog.
        for alt in [
            "a gull",
            "the sign [Revised]",
            "a back\\slash",
            "brackets ] and [ both",
        ] {
            let written = ImageRef {
                index: 0,
                src: "assets/a.png".into(),
                alt: String::new(),
                size: None,
            }
            .djot(alt, None);
            let parsed = parse_images(&written);
            assert_eq!(parsed.len(), 1, "{written:?}");
            assert_eq!(parsed[0].alt, alt, "{written:?}");
        }
    }

    #[test]
    fn the_image_at_a_position_is_found_by_order_not_by_name() {
        // The same picture twice at two sizes: a `src` cannot say which one was
        // clicked, so the index has to.
        let djot = "a ![one](assets/a.png){width=10 height=10} b \
                    ![two](assets/a.png){width=99 height=99} c";
        // Plain text: "a \u{FFFC} b \u{FFFC} c" — the addressable view, images
        // included.
        let plain = "a \u{FFFC} b \u{FFFC} c";
        let first = image_at(plain, 2, djot).expect("first image");
        assert_eq!((first.index, first.size), (0, Some((10, 10))));
        let second = image_at(plain, 6, djot).expect("second image");
        assert_eq!((second.index, second.size), (1, Some((99, 99))));
        // And a click on ordinary text edits nothing.
        assert_eq!(image_at(plain, 0, djot), None);
    }

    #[test]
    fn resizing_keeps_the_aspect_ratio_and_never_reaches_zero() {
        let img = parse_images(PROSE).remove(0);
        assert_eq!(img.scaled(50), Some((160, 120)));
        assert_eq!(img.scaled(200), Some((640, 480)));
        // A ratio that would round to nothing still leaves a visible image
        // rather than a zero-sized one the layout cannot place.
        let tiny = ImageRef {
            index: 0,
            src: "a".into(),
            alt: String::new(),
            size: Some((3, 1)),
        };
        assert_eq!(tiny.scaled(1), Some((1, 1)));
    }

    #[test]
    fn rewriting_an_image_changes_only_what_was_asked() {
        let img = parse_images(PROSE).remove(0);
        let out = img.djot("a herring gull", img.scaled(50));
        assert_eq!(
            out,
            "![a herring gull](assets/aa.png){width=160 height=120}"
        );
    }

    #[test]
    fn referenced_paths_finds_every_image_once() {
        let djot = "Before ![one](assets/aaa.png){width=1 height=1} and \
                    ![two](assets/bbb.jpg) and ![again](assets/aaa.png).";
        assert_eq!(
            referenced_paths(djot),
            vec!["assets/aaa.png".to_string(), "assets/bbb.jpg".to_string()]
        );
    }

    #[test]
    fn referenced_paths_ignores_links_and_outside_references() {
        // A plain link is not an image, and an image pointing outside the
        // project is not ours to resolve.
        let djot = "[a link](assets/not-an-image.png) and ![web](https://x/y.png)";
        assert!(referenced_paths(djot).is_empty());
    }

    #[test]
    fn referenced_paths_survives_malformed_markup() {
        for djot in ["![unclosed", "![a](", "![a](assets/x.png", "!["] {
            let _ = referenced_paths(djot);
        }
    }

    #[test]
    fn mime_is_derived_from_the_stored_extension() {
        assert_eq!(mime_for_extension("assets/a.png"), "image/png");
        assert_eq!(mime_for_extension("assets/a.jpg"), "image/jpeg");
        assert_eq!(mime_for_extension("assets/a.webp"), "image/webp");
    }
}
