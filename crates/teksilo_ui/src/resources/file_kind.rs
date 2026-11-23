// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What sort of thing a resource is, for the eye: which icon its row draws, whether it
//! has a thumbnail, whether it opens in the image viewer, and the word a screen reader
//! hears for its type.
//!
//! Read from the file's extension, with the stored media type as a fallback, and never
//! asked whether a file is safe to open: that is [`super::open_safety`]'s question, which
//! has its own, stricter list. A name can claim anything.
//!
//! Not a view-model: a classification and its words.

use teksilo::prelude::*;

/// The families a resource row tells apart.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FileKind {
    Picture,
    Pdf,
    Audio,
    Video,
    /// Plain text, Markdown, RTF, and the word processors' documents and spreadsheets.
    Text,
    /// An e-book.
    Ebook,
    Archive,
    /// Anything else on disk.
    Other,
    /// An address on the web, not a file.
    WebLink,
}

/// The lowercase extension of `file_name`, without its dot; empty when it has none.
///
/// Trailing dots and spaces are dropped first: Windows ignores them when it opens a
/// file, so `notes.exe.` is an `exe` there, and must be one here too.
pub fn extension(file_name: &str) -> String {
    let name = file_name.trim_end_matches(['.', ' ']);
    let name = name.rsplit(['/', '\\']).next().unwrap_or(name);
    match name.rsplit_once('.') {
        Some((stem, ext)) if !stem.is_empty() => ext.to_ascii_lowercase(),
        _ => String::new(),
    }
}

/// The family of a file called `file_name`, whose media type is `mime` (possibly empty,
/// which a linked file's always is).
pub fn kind_of(file_name: &str, mime: &str) -> FileKind {
    let by_name = match extension(file_name).as_str() {
        "png" | "jpg" | "jpeg" | "webp" | "gif" | "tif" | "tiff" | "bmp" | "heic" | "heif"
        | "avif" | "svg" => Some(FileKind::Picture),
        "pdf" => Some(FileKind::Pdf),
        "mp3" | "m4a" | "aac" | "ogg" | "oga" | "opus" | "flac" | "wav" | "aiff" | "aif" => {
            Some(FileKind::Audio)
        }
        "mp4" | "m4v" | "mov" | "webm" | "mkv" | "avi" => Some(FileKind::Video),
        "txt" | "text" | "md" | "markdown" | "rtf" | "doc" | "docx" | "odt" | "ods" | "odp"
        | "xls" | "xlsx" | "ppt" | "pptx" | "csv" | "pages" | "numbers" | "key" | "html"
        | "htm" => Some(FileKind::Text),
        "epub" | "mobi" | "azw3" => Some(FileKind::Ebook),
        "zip" | "7z" | "rar" | "tar" | "gz" | "tgz" | "bz2" | "xz" => Some(FileKind::Archive),
        _ => None,
    };
    by_name.unwrap_or_else(|| {
        let mime = mime.to_ascii_lowercase();
        if mime.starts_with("image/") {
            FileKind::Picture
        } else if mime == "application/pdf" {
            FileKind::Pdf
        } else if mime.starts_with("audio/") {
            FileKind::Audio
        } else if mime.starts_with("video/") {
            FileKind::Video
        } else if mime.starts_with("text/") {
            FileKind::Text
        } else {
            FileKind::Other
        }
    })
}

/// The extensions of the formats the editor decodes, lower-case: what
/// [`is_viewable_picture`] accepts, and what a file dialog asking for pictures offers.
pub const VIEWABLE_PICTURE_EXTENSIONS: [&str; 4] = ["png", "jpg", "jpeg", "webp"];

/// Whether the image viewer and the thumbnails can draw this file: the formats the
/// editor decodes, and nothing else. A `.heic` is a picture to the eye and still opens in
/// another application.
pub fn is_viewable_picture(file_name: &str) -> bool {
    VIEWABLE_PICTURE_EXTENSIONS.contains(&extension(file_name).as_str())
}

impl FileKind {
    /// The word a screen reader hears for this family, in a row's name.
    pub fn label(self) -> LocalizedString {
        match self {
            Self::Picture => tr!(resources_kind_picture()),
            Self::Pdf => tr!(resources_kind_pdf()),
            Self::Audio => tr!(resources_kind_audio()),
            Self::Video => tr!(resources_kind_video()),
            Self::Text => tr!(resources_kind_document()),
            Self::Ebook => tr!(resources_kind_ebook()),
            Self::Archive => tr!(resources_kind_archive()),
            Self::Other => tr!(resources_kind_file()),
            Self::WebLink => tr!(resources_kind_web_link()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_extension_ignores_case_and_what_windows_ignores() {
        assert_eq!(extension("Map.PDF"), "pdf");
        assert_eq!(extension("notes.exe."), "exe");
        assert_eq!(extension("notes.exe . "), "exe");
        assert_eq!(extension(".bashrc"), "");
        assert_eq!(extension("README"), "");
        assert_eq!(extension("/home/a.b/notes"), "");
    }

    #[test]
    fn a_file_is_sorted_by_its_name_then_its_media_type() {
        assert_eq!(kind_of("Lighthouse.jpg", ""), FileKind::Picture);
        assert_eq!(kind_of("Village map.pdf", ""), FileKind::Pdf);
        assert_eq!(kind_of("Keeper.mp3", ""), FileKind::Audio);
        assert_eq!(kind_of("Harbour.mov", ""), FileKind::Video);
        assert_eq!(kind_of("Chapter notes.docx", ""), FileKind::Text);
        assert_eq!(kind_of("Ship's log.epub", ""), FileKind::Ebook);
        assert_eq!(kind_of("Letters.zip", ""), FileKind::Archive);
        assert_eq!(kind_of("scan", "image/png"), FileKind::Picture);
        assert_eq!(
            kind_of("blob.bin", "application/octet-stream"),
            FileKind::Other
        );
    }

    #[test]
    fn only_what_the_editor_decodes_is_viewable() {
        assert!(is_viewable_picture("a.PNG"));
        assert!(is_viewable_picture("a.jpeg"));
        assert!(!is_viewable_picture("a.heic"));
        assert!(!is_viewable_picture("a.svg"));
    }
}
