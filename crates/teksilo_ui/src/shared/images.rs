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

use teksilo::prelude::{
    AsyncRuntimeHandle, EventContext, EventContextAsyncExt, EventContextToastExt, lit,
    spawn_blocking, tr,
};
use teksilo::widgets::Toast;
use teksilo::widgets::rich_text::{EditorHandle, WeakEditorHandle};

use crate::toast_scope::ToastWorkExt;

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
///
/// The [`Display`](std::fmt::Display) text is what the writer reads, in a toast: each
/// variant is a translated sentence, never a message meant for a log.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageError {
    /// The file could not be opened or read.
    Unreadable(String),
    /// The file is not a picture the editor paints, or its header claims more than it may.
    Undecodable(String),
    /// A picture past [`document_ingest::pictures::MAX_PICTURE_BYTES`], never read.
    TooLarge(String),
    /// The picture could not be written into this media directory.
    NotWritten(String),
    /// No project is open, so there is no media directory to store the picture in.
    NoProject,
    /// The picture is stored but its `Asset` row could not be created.
    NotRecorded,
}

impl std::fmt::Display for ImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::Unreadable(p) => tr!(image_unreadable(file = p.clone())),
            Self::Undecodable(p) => tr!(image_undecodable(file = p.clone())),
            Self::TooLarge(p) => tr!(image_too_large(
                file = p.clone(),
                limit = (document_ingest::pictures::MAX_PICTURE_BYTES >> 20) as i64
            )),
            Self::NotWritten(p) => tr!(image_not_stored(folder = p.clone())),
            Self::NoProject => tr!(image_no_project()),
            Self::NotRecorded => tr!(image_not_recorded()),
        };
        f.write_str(&text.resolve_now())
    }
}

/// How much of a file is read to tell whether it is a picture: more than any signature
/// the editor knows needs (a WebP's is the longest, at twelve bytes).
const SNIFF_BYTES: u64 = 64;

/// Open the file at `path` and read its first bytes, if they are those of a picture the
/// editor paints and the file is within the size a picture may be.
///
/// The file comes back open after them, so [`examine`] reads the rest from where this
/// stopped. A recording of two gigabytes dropped on a text is asked whether it is a
/// picture too: its first bytes alone tell, and a picture past the size a project accepts
/// ([`document_ingest::pictures::MAX_PICTURE_BYTES`]) is refused before the rest is read.
fn open_picture(path: &Path) -> Result<(std::fs::File, Vec<u8>), ImageError> {
    use std::io::Read;
    let display = || path.display().to_string();
    let unreadable = |_| ImageError::Unreadable(display());
    let mut file = std::fs::File::open(path).map_err(unreadable)?;
    let mut bytes = Vec::new();
    (&mut file)
        .take(SNIFF_BYTES)
        .read_to_end(&mut bytes)
        .map_err(unreadable)?;
    if teksilo::canvas::ImageFormat::sniff(&bytes).is_none() {
        return Err(ImageError::Undecodable(display()));
    }
    let size = file.metadata().map_err(unreadable)?.len();
    if size > document_ingest::pictures::MAX_PICTURE_BYTES {
        return Err(ImageError::TooLarge(display()));
    }
    Ok((file, bytes))
}

/// Whether the file at `path` opens like a picture the editor paints, within the size a
/// picture may be: its first bytes and its length, nothing decoded.
///
/// Cheap enough for the interface thread, which is where a drop on a text sorts its
/// files into pictures and resources. A file that passes can still fail [`examine`] (a
/// truncated photograph, a header claiming more pixels than a picture may have), so the
/// sorting is only final once that has run.
pub fn looks_like_picture(path: &Path) -> bool {
    open_picture(path).is_ok()
}

/// Examine a chosen file without storing anything.
///
/// Decoding here is not wasted work: it is the only way to learn the image's
/// true pixel dimensions (a file's own metadata can disagree with its content,
/// and a JPEG's EXIF rotation changes which side is which), and it is also the
/// check that the bytes are an image at all — better to find out before writing
/// them into the project than at the first paint.
///
/// It reads up to 64 MiB and decodes every pixel, so it runs away from the interface
/// thread (see [`off_the_interface`]). The file's first bytes are read alone first
/// ([`looks_like_picture`] asks the same), and [`examine_bytes`] checks the header's
/// claims before decoding a pixel.
pub fn examine(path: &Path) -> Result<PendingImage, ImageError> {
    use std::io::Read;
    let (mut file, mut bytes) = open_picture(path)?;
    let display = path.display().to_string();
    let cap = document_ingest::pictures::MAX_PICTURE_BYTES;
    // Read to the cap and one byte past it, not to the length seen when it was opened: a
    // file still being written can have grown since.
    (&mut file)
        .take(cap + 1 - bytes.len() as u64)
        .read_to_end(&mut bytes)
        .map_err(|_| ImageError::Unreadable(display.clone()))?;
    if bytes.len() as u64 > cap {
        return Err(ImageError::TooLarge(display));
    }
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
///
/// The header is checked against the ceilings an import applies
/// ([`document_ingest::pictures::displayable_size`]) before anything is decoded: a few
/// crafted bytes can claim billions of pixels.
pub fn examine_bytes(bytes: Vec<u8>, file_name: String) -> Option<PendingImage> {
    let format = teksilo::canvas::ImageFormat::sniff(&bytes)?;
    document_ingest::pictures::displayable_size(&bytes).ok()?;
    let decoded = teksilo::canvas::RasterIcon::decode(&bytes).ok()?;
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
    let Ok(decoded) = teksilo::canvas::RasterIcon::decode(&image.bytes) else {
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

pub(crate) fn encode_png(icon: &teksilo::canvas::RasterIcon) -> Option<Vec<u8>> {
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

/// [`apply_policy`], then [`store`] into `media_dir`: everything a picture the writer
/// chose needs before it can be recorded, all of it file work for a background thread.
pub fn prepare(
    image: PendingImage,
    policy: SizePolicy,
    media_dir: &Path,
) -> Result<PendingImage, ImageError> {
    let image = apply_policy(image, policy);
    store(&image, media_dir)?;
    Ok(image)
}

/// Run `work` away from the interface thread, then `done` back on it, with what `work`
/// returned and a fresh context of the same window.
///
/// Reading a picture of 64 MiB, decoding it and writing it into the project take as long
/// as the picture is large, and on the interface thread they froze the window for that
/// long. `done` gets `None` when the worker died before returning (a panic, which
/// `spawn_blocking` catches rather than letting it reach the interface). With no async
/// runtime installed, as in a headless test, both run here, one after the other.
pub fn off_the_interface<T: Send + 'static>(
    ctx: &mut EventContext,
    work: impl FnOnce() -> T + Send + 'static,
    done: impl FnOnce(Option<T>, &mut EventContext) + 'static,
) {
    if ctx.app_state::<AsyncRuntimeHandle>().is_none() {
        let out = work();
        done(Some(out), ctx);
        return;
    }
    ctx.spawn_local_with(async move { spawn_blocking(work).await.ok() }, done)
        .detach();
}

/// Where a picture prepared away from the interface goes once it is ready, and what it
/// was asked for, so that what changed meanwhile can be told.
#[derive(Debug, Clone)]
pub struct Landing {
    /// The text it goes into, held weakly: a tab closed while the picture was read takes
    /// its editor with it, and the picture is dropped rather than keeping the editor
    /// alive to receive it.
    pub editor: WeakEditorHandle,
    /// Where in that text, `None` for wherever the caret is when it lands.
    pub at: Option<usize>,
    /// The Work open when it was asked for. A window that has moved to another project
    /// since drops it: the picture belongs to the project it was chosen in.
    pub work_id: Option<u64>,
}

impl Landing {
    /// A landing in `handle`'s text at `at`, for the Work `ids` has open now.
    pub fn new(handle: &EditorHandle, at: Option<usize>, ids: &crate::app_ids::AppIds) -> Self {
        Self {
            editor: handle.downgrade(),
            at,
            work_id: ids.work_id.get(),
        }
    }

    /// The editor, if it is still there, still takes writes, and the window still has
    /// the same Work open; else `None`, and why on standard error.
    pub fn target(&self, ids: &crate::app_ids::AppIds) -> Option<EditorHandle> {
        if ids.work_id.get() != self.work_id {
            eprintln!("images: the project changed while a picture was read; it was not inserted");
            return None;
        }
        let Some(handle) = self.editor.upgrade() else {
            eprintln!("images: the text was closed while a picture was read; it was not inserted");
            return None;
        };
        crate::shared::editor_writes::accepts_writes(&handle).then_some(handle)
    }
}

/// Record `image` (already [`prepare`]d into `stored_in`) as an `Asset` of the open Work,
/// unless one with the same bytes already is, and put it into the text `landing` names,
/// described as `alt`. Back on the interface thread, after the file work.
///
/// `Ok(None)` when the picture has nowhere to go any more (see [`Landing::target`]):
/// nothing is recorded then, and the stored file stays unreferenced, as harmless as the
/// one a crash between storing and inserting leaves. `media_dir` is where the project
/// keeps its pictures now: a project moved meanwhile (a Save As) gets the picture stored
/// again in its new place, which the editor and the next save read from.
pub fn land(
    app_ctx: &frontend::AppContext,
    ids: &crate::app_ids::AppIds,
    media_dir: &Path,
    stored_in: &Path,
    landing: &Landing,
    image: &PendingImage,
    alt: &str,
) -> Result<Option<EditorHandle>, ImageError> {
    let Some(handle) = landing.target(ids) else {
        return Ok(None);
    };
    if media_dir.as_os_str().is_empty() {
        return Err(ImageError::NoProject);
    }
    if media_dir != stored_in {
        store(image, media_dir)?;
    }
    let work_id = ids.work_id.get().unwrap_or_default();
    if !work_has_asset(app_ctx, work_id, &image.content_hash) {
        frontend::commands::asset_commands::create_asset(
            app_ctx,
            ids.stack_id.get(),
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
                // Inserting a picture into the prose never makes it the cover; that is a
                // separate, deliberate choice made from the image itself.
            },
            work_id,
            -1,
        )
        .map_err(|_| ImageError::NotRecorded)?;
    }
    // Registered before the insertion relays out, or the first paint draws nothing.
    handle.add_image_resource(&image.relative_path(), &image.mime_type, &image.bytes);
    match landing.at {
        // The text may have shortened while the picture was read.
        Some(at) => {
            let at = at.min(handle.to_plain_text().chars().count());
            handle.select_range(at, at);
        }
        // Under "Always forward", after a selection rather than over it.
        None => crate::shared::editor_writes::collapse_selection_before_insert(&handle),
    }
    handle.insert_djot(&image.djot(alt, image.width, image.height));
    Ok(Some(handle))
}

/// Say why a picture did not go in, in a toast of the Work it was asked for.
pub fn report(ctx: &mut EventContext, error: &ImageError, work_id: Option<u64>) {
    ctx.show_toast(Toast::error(lit!(error.to_string())).scoped_id("images.failed", work_id));
}

/// Put the picture file at `path` into `handle`'s text at `at`, described as `alt`:
/// examined and stored away from the interface thread, then recorded as an `Asset` of
/// the open Work unless one with the same bytes already is, and referenced (see
/// [`land`]). Kept at its own size: a picture already in the project (a resource) was
/// never asked about its size the way a fresh insertion is. A failure is reported in a
/// toast.
///
/// The bytes are a copy. From here on the picture is the text's, and the file it came
/// from can change or go without the prose losing it.
#[allow(clippy::too_many_arguments)]
pub fn insert_picture_file(
    ctx: &mut EventContext,
    app_ctx: std::rc::Rc<frontend::AppContext>,
    ids: crate::app_ids::AppIds,
    media_dir: impl Fn() -> PathBuf + 'static,
    handle: &EditorHandle,
    at: usize,
    path: PathBuf,
    alt: String,
) {
    let landing = Landing::new(handle, Some(at), &ids);
    let stored_in = media_dir();
    if stored_in.as_os_str().is_empty() {
        report(ctx, &ImageError::NoProject, landing.work_id);
        return;
    }
    let dir = stored_in.clone();
    // A worker that died before answering read nothing the writer can be told about but
    // the file it was reading.
    let shown = path.display().to_string();
    off_the_interface(
        ctx,
        move || examine(&path).and_then(|image| prepare(image, SizePolicy::KeepOriginal, &dir)),
        move |prepared, c| {
            let landed = prepared
                .unwrap_or(Err(ImageError::Unreadable(shown)))
                .and_then(|image| {
                    land(
                        &app_ctx,
                        &ids,
                        &media_dir(),
                        &stored_in,
                        &landing,
                        &image,
                        &alt,
                    )
                });
            if let Err(e) = landed {
                report(c, &e, landing.work_id);
            }
        },
    );
}

/// Whether the Work `work_id` already records a picture with these bytes.
fn work_has_asset(app_ctx: &frontend::AppContext, work_id: u64, content_hash: &str) -> bool {
    use frontend::common::direct_access::work::WorkRelationshipField;
    let ids = frontend::commands::work_commands::get_work_relationship(
        app_ctx,
        &work_id,
        &WorkRelationshipField::Assets,
    )
    .unwrap_or_default();
    frontend::commands::asset_commands::get_asset_multi(app_ctx, &ids)
        .unwrap_or_default()
        .into_iter()
        .flatten()
        .any(|a| a.content_hash == content_hash)
}

/// Supplies an editor with image bytes its document does not have.
///
/// Handed to `RichTextEditor::on_image_missing`, which asks when it meets a name
/// it cannot resolve. That is exactly what pasting a picture into a *second*
/// editor produces: the clipboard carries the reference, because an image's
/// pixels live on the document that owns it and a fragment is not a document.
/// The same hook covers a drop and an undo that brings a deleted image back.
///
/// Cheap to clone and safe to hold: the files under the directory are
/// content-addressed, so a name either resolves to the same bytes forever or to
/// nothing at all.
///
/// It holds what answers where the directory is, never the answer. A document can
/// stay open while its project moves (a Save As, a new folder project's first
/// save), and a picture added after that exists only in the new place, so a path
/// kept from when the document opened left that picture blank wherever it was
/// pasted, until the project was closed.
#[derive(Clone)]
pub struct ImageSource {
    media_dir: std::rc::Rc<dyn Fn() -> PathBuf>,
}

impl std::fmt::Debug for ImageSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImageSource")
            .field("media_dir", &self.media_dir())
            .finish()
    }
}

impl ImageSource {
    /// A source that asks `media_dir` where the project's pictures are each time it
    /// needs one.
    pub fn following(media_dir: std::rc::Rc<dyn Fn() -> PathBuf>) -> Self {
        Self { media_dir }
    }

    /// A source that always looks in `media_dir`.
    pub fn at(media_dir: PathBuf) -> Self {
        Self::following(std::rc::Rc::new(move || media_dir.clone()))
    }

    /// Where the project's pictures are now.
    pub fn media_dir(&self) -> PathBuf {
        (self.media_dir)()
    }

    /// The callback `RichTextEditor::on_image_missing` wants.
    pub fn resolver(&self) -> impl Fn(&str) -> Option<(String, Vec<u8>)> + 'static {
        let source = self.clone();
        move |name: &str| {
            // The prose names `assets/<hash>.<ext>`; the media directory holds
            // the file under that basename. Only the basename is used, so a
            // name that tried to walk out of the project cannot.
            let base = Path::new(name).file_name()?;
            let bytes = std::fs::read(source.media_dir().join(base)).ok()?;
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
    doc: &teksilo::text_document::TextDocument,
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
                teksilo::text_document::ResourceType::Image,
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
        self.scaled_within(percent, None)
    }

    /// The size this reference is drawn at in a line `measure` logical pixels wide.
    ///
    /// The layout never draws a picture wider than its line: one stored wider is drawn at
    /// the line's width, its height in proportion (text-typeset's cap). `None` for the
    /// measure means it is not known, and the stored size is the answer.
    pub fn shown_size(&self, measure: Option<f32>) -> Option<(u32, u32)> {
        let (w, h) = self.size?;
        match measure {
            Some(m) if m.is_finite() && m >= 1.0 && f64::from(w) > f64::from(m) => {
                let m = f64::from(m);
                let ratio = m / f64::from(w.max(1));
                Some((
                    (m.floor() as u32).max(1),
                    ((f64::from(h) * ratio).round() as u32).max(1),
                ))
            }
            _ => Some((w, h)),
        }
    }

    /// [`Self::scaled`], from the size the picture is drawn at in a line `measure` wide
    /// ([`Self::shown_size`]) rather than the stored one, which the layout may have capped.
    /// So "50" halves what the writer sees, even for a picture stored far wider than the
    /// page.
    pub fn scaled_within(&self, percent: u32, measure: Option<f32>) -> Option<(u32, u32)> {
        let (w, h) = self.shown_size(measure)?;
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

    /// What a refused picture tells the writer is a sentence of their language, naming
    /// the file, and never an empty name: a project that is not open says so instead.
    #[test]
    fn every_refusal_reads_as_a_translated_sentence() {
        let file = "/home/a/poster.png".to_string();
        let cases = [
            (
                ImageError::Unreadable(file.clone()),
                tr!(image_unreadable(file = file.clone())),
            ),
            (
                ImageError::Undecodable(file.clone()),
                tr!(image_undecodable(file = file.clone())),
            ),
            (
                ImageError::NotWritten("/home/a/media".into()),
                tr!(image_not_stored(folder = "/home/a/media".to_string())),
            ),
            (ImageError::NoProject, tr!(image_no_project())),
            (ImageError::NotRecorded, tr!(image_not_recorded())),
        ];
        for (error, expected) in cases {
            let text = error.to_string();
            assert_eq!(text, expected.resolve_now(), "{error:?}");
            assert!(!text.is_empty() && !text.contains('{'), "{text}");
        }
        assert!(
            ImageError::Unreadable(file.clone())
                .to_string()
                .contains(&file)
        );
        assert!(
            ImageError::NotWritten("/home/a/media".into())
                .to_string()
                .contains("/home/a/media")
        );
    }

    #[test]
    fn non_image_bytes_are_refused_before_anything_is_stored() {
        assert!(examine_bytes(b"just some text".to_vec(), "notes.txt".into()).is_none());
    }

    /// A file dropped on a text is asked whether it is a picture on the interface thread
    /// ([`looks_like_picture`]), and examined afterwards: a large file that is none is told
    /// by its first bytes, and a picture past the size a project accepts is refused
    /// without being read. Both files here are sparse, four gigabytes long: read whole,
    /// either would take that much memory.
    #[test]
    fn a_large_file_is_judged_without_being_read_whole() {
        let dir = tempfile::tempdir().unwrap();
        let huge = 4u64 << 30;
        let video = dir.path().join("rushes.mkv");
        {
            let mut file = std::fs::File::create(&video).unwrap();
            std::io::Write::write_all(&mut file, &[0x1a, 0x45, 0xdf, 0xa3]).unwrap();
            file.set_len(huge).unwrap();
        }
        assert!(matches!(examine(&video), Err(ImageError::Undecodable(_))));
        assert!(!looks_like_picture(&video));
        let picture = dir.path().join("poster.png");
        {
            let mut file = std::fs::File::create(&picture).unwrap();
            std::io::Write::write_all(&mut file, &png(4, 4)).unwrap();
            file.set_len(huge).unwrap();
        }
        assert!(matches!(examine(&picture), Err(ImageError::TooLarge(_))));
        assert!(!looks_like_picture(&picture));
        // Just past the cap is refused as well, and a picture within it still comes in.
        let cap = document_ingest::pictures::MAX_PICTURE_BYTES;
        std::fs::File::options()
            .write(true)
            .open(&picture)
            .unwrap()
            .set_len(cap + 1)
            .unwrap();
        assert!(matches!(examine(&picture), Err(ImageError::TooLarge(_))));
        assert!(!looks_like_picture(&picture));
        let small = dir.path().join("small.png");
        std::fs::write(&small, png(6, 5)).unwrap();
        assert!(looks_like_picture(&small));
        let image = examine(&small).expect("a picture within the cap");
        assert_eq!((image.width, image.height), (6, 5));
        assert_eq!(image.file_name, "small.png");
    }

    /// Sorting a drop decodes nothing: a file that opens like a PNG passes, whatever
    /// follows its signature, and only examining it finds that nothing does.
    #[test]
    fn telling_a_picture_by_its_first_bytes_decodes_nothing() {
        let dir = tempfile::tempdir().unwrap();
        let broken = dir.path().join("broken.png");
        let mut bytes = png(4, 4);
        bytes.truncate(20);
        std::fs::write(&broken, &bytes).unwrap();
        assert!(looks_like_picture(&broken));
        assert!(matches!(examine(&broken), Err(ImageError::Undecodable(_))));
        assert!(!looks_like_picture(&dir.path().join("gone.png")));
    }

    /// A picture whose header passes the ceilings an import applies is decoded; one
    /// wider than any the editor can show is refused before it is.
    #[test]
    fn a_picture_past_the_import_ceilings_is_refused_before_decoding() {
        let side = document_ingest::pictures::MAX_PICTURE_SIDE;
        assert!(examine_bytes(png(side, 1), "strip.png".into()).is_some());
        assert!(examine_bytes(png(side + 1, 1), "strip.png".into()).is_none());
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
    fn a_percentage_is_of_the_size_the_line_lets_a_picture_be_drawn_at() {
        let wide = ImageRef {
            index: 0,
            src: "assets/x.png".into(),
            alt: String::new(),
            size: Some((4000, 1000)),
        };
        // A 4000 px picture in an 800 px line is drawn 800 × 200.
        assert_eq!(wide.shown_size(Some(800.0)), Some((800, 200)));
        assert_eq!(wide.scaled_within(50, Some(800.0)), Some((400, 100)));
        // Unknown measure: the stored size, as before the cap.
        assert_eq!(wide.scaled_within(50, None), Some((2000, 500)));
        // A picture narrower than its line is drawn at its own size.
        assert_eq!(wide.shown_size(Some(5000.0)), Some((4000, 1000)));
        // A nonsense measure is ignored, never a division by zero.
        assert_eq!(wide.shown_size(Some(0.0)), Some((4000, 1000)));
        assert_eq!(wide.shown_size(Some(f32::NAN)), Some((4000, 1000)));
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

    /// Putting a picture read away from the interface into its text, against the real
    /// backend: what lands, where, and what is dropped when its text or its Work is gone.
    #[cfg(not(feature = "mocks"))]
    mod landing {
        use super::*;
        use teksilo::text_document::TextDocument;
        use teksilo::widgets::rich_text::RichTextEditor;

        fn editor(text: &str) -> RichTextEditor {
            let doc = TextDocument::new();
            doc.set_plain_text(text).expect("plain text");
            RichTextEditor::editor(doc)
        }

        fn assets(p: &crate::test_support::RealProject) -> Vec<frontend::asset::dtos::AssetDto> {
            use frontend::common::direct_access::work::WorkRelationshipField;
            let ids = frontend::commands::work_commands::get_work_relationship(
                &p.app_ctx,
                &p.work_id,
                &WorkRelationshipField::Assets,
            )
            .expect("the Work's assets");
            frontend::commands::asset_commands::get_asset_multi(&p.app_ctx, &ids)
                .expect("the assets")
                .into_iter()
                .flatten()
                .collect()
        }

        /// A picture prepared in the background goes in at its offset, cut back to the
        /// text's length when the text shortened meanwhile, and is recorded once however
        /// often it goes in.
        #[test]
        fn a_prepared_picture_lands_at_its_offset_within_the_text() {
            let p = crate::test_support::RealProject::empty_novel();
            let media = tempfile::tempdir().unwrap();
            let image = prepare(
                examine_bytes(png(5, 4), "gull.png".into()).unwrap(),
                SizePolicy::KeepOriginal,
                media.path(),
            )
            .expect("stored");
            let editor = editor("Gulls");
            let handle = editor.handle();
            let landing = Landing::new(&handle, Some(2), &p.ids);
            let landed = land(
                &p.app_ctx,
                &p.ids,
                media.path(),
                media.path(),
                &landing,
                &image,
                "a gull",
            )
            .expect("landed");
            assert!(landed.is_some());
            let text = handle.to_plain_text();
            assert_eq!(text.chars().nth(2), Some('\u{FFFC}'), "{text:?}");
            assert!(handle.to_djot().contains(&image.relative_path()));
            assert!(handle.has_image_resource(&image.relative_path()));

            let past_the_end = Landing::new(&handle, Some(500), &p.ids);
            land(
                &p.app_ctx,
                &p.ids,
                media.path(),
                media.path(),
                &past_the_end,
                &image,
                "",
            )
            .expect("landed at the end");
            assert_eq!(handle.to_plain_text().matches('\u{FFFC}').count(), 2);
            let recorded = assets(&p);
            assert_eq!(recorded.len(), 1, "the same bytes are one Asset");
            assert_eq!(recorded[0].content_hash, image.content_hash);
        }

        /// A text closed, or a window moved to another project, while the picture was
        /// read: nothing goes in and nothing is recorded.
        #[test]
        fn a_picture_whose_text_or_work_is_gone_is_dropped() {
            let p = crate::test_support::RealProject::empty_novel();
            let media = tempfile::tempdir().unwrap();
            let image = prepare(
                examine_bytes(png(3, 3), "x.png".into()).unwrap(),
                SizePolicy::KeepOriginal,
                media.path(),
            )
            .unwrap();
            let closed = {
                let editor = editor("Gone");
                Landing::new(&editor.handle(), Some(0), &p.ids)
            };
            let outcome = land(
                &p.app_ctx,
                &p.ids,
                media.path(),
                media.path(),
                &closed,
                &image,
                "",
            );
            assert_eq!(outcome.map(|h| h.is_some()), Ok(false));

            let editor = editor("Still here");
            let handle = editor.handle();
            let landing = Landing::new(&handle, Some(0), &p.ids);
            p.ids.work_id.set(Some(p.work_id + 1000));
            let outcome = land(
                &p.app_ctx,
                &p.ids,
                media.path(),
                media.path(),
                &landing,
                &image,
                "",
            );
            p.ids.work_id.set(Some(p.work_id));
            assert_eq!(outcome.map(|h| h.is_some()), Ok(false));
            assert_eq!(handle.to_plain_text(), "Still here");
            assert!(
                assets(&p).is_empty(),
                "nothing recorded for a dropped picture"
            );
        }

        /// A project moved while the picture was read (a Save As) gets it stored again in
        /// its new place, where the editor and the next save look for it.
        #[test]
        fn a_project_moved_meanwhile_gets_the_picture_in_its_new_place() {
            let p = crate::test_support::RealProject::empty_novel();
            let old = tempfile::tempdir().unwrap();
            let new = tempfile::tempdir().unwrap();
            let image = prepare(
                examine_bytes(png(3, 2), "x.png".into()).unwrap(),
                SizePolicy::KeepOriginal,
                old.path(),
            )
            .unwrap();
            let editor = editor("");
            let landing = Landing::new(&editor.handle(), None, &p.ids);
            land(
                &p.app_ctx,
                &p.ids,
                new.path(),
                old.path(),
                &landing,
                &image,
                "",
            )
            .unwrap();
            let base = format!("{}.png", image.content_hash);
            assert!(new.path().join(base).is_file());
        }

        fn picture_file(dir: &Path) -> PathBuf {
            let path = dir.join("lighthouse.png");
            std::fs::write(&path, png(7, 3)).unwrap();
            path
        }

        /// Inserting a picture file goes through the async runtime when the app has one:
        /// nothing is in the text when the call returns, and the picture is once the
        /// background work is done and its completion delivered.
        #[test]
        fn a_picture_file_is_read_in_the_background_then_lands_in_its_text() {
            use teksilo::async_rt::TeksiloAppBuilderAsyncExt;
            let p = crate::test_support::RealProject::empty_novel();
            let files = tempfile::tempdir().unwrap();
            let media = tempfile::tempdir().unwrap();
            let source = picture_file(files.path());
            let editor = editor("Light");
            let handle = editor.handle();
            let mut app = teksilo::app::TeksiloAppBuilder::new()
                .install_async()
                .build_headless();
            let rt = app
                .tree
                .app_context()
                .app_state::<AsyncRuntimeHandle>()
                .expect("the runtime")
                .clone();
            let dir = media.path().to_path_buf();
            app.tree
                .run_with_event_context(&mut teksilo::core::NoopWindowOps, |ctx| {
                    insert_picture_file(
                        ctx,
                        p.app_ctx.clone(),
                        p.ids.clone(),
                        move || dir.clone(),
                        &handle,
                        5,
                        source,
                        "a lighthouse".into(),
                    );
                });
            assert_eq!(
                handle.to_plain_text(),
                "Light",
                "nothing decoded on the caller's turn"
            );
            let started = std::time::Instant::now();
            while rt.completions().pending_len() == 0 {
                assert!(
                    started.elapsed().as_secs() < 30,
                    "the background work never finished"
                );
                rt.tick();
                std::thread::sleep(std::time::Duration::from_millis(5));
            }
            assert_eq!(rt.completions().drain_into(&mut app.tree), 1);
            assert_eq!(handle.to_plain_text(), "Light\u{FFFC}");
            assert!(handle.to_djot().contains("![a lighthouse](assets/"));
            assert_eq!(assets(&p).len(), 1);
        }

        /// Without a runtime (a headless tree, as most tests build) the same call runs
        /// straight through and the picture is in the text when it returns.
        #[test]
        fn without_a_runtime_a_picture_file_lands_at_once() {
            let p = crate::test_support::RealProject::empty_novel();
            let files = tempfile::tempdir().unwrap();
            let media = tempfile::tempdir().unwrap();
            let source = picture_file(files.path());
            let editor = editor("Light");
            let handle = editor.handle();
            let mut tree = teksilo::core::widget_tree::WidgetTree::new();
            let dir = media.path().to_path_buf();
            tree.run_with_event_context(&mut teksilo::core::NoopWindowOps, |ctx| {
                insert_picture_file(
                    ctx,
                    p.app_ctx.clone(),
                    p.ids.clone(),
                    move || dir.clone(),
                    &handle,
                    0,
                    source,
                    String::new(),
                );
            });
            assert_eq!(handle.to_plain_text(), "\u{FFFC}Light");
            assert_eq!(assets(&p).len(), 1);
        }
    }
}
