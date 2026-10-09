// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Every picture here is built in the test, with the `image` crate's own encoders or byte by
//! byte: no file from anywhere else is read, and none is committed.

use super::*;
use image::{Rgba, RgbaImage};

/// A `width` × `height` picture of one colour, encoded as `format`.
pub(crate) fn encoded(format: ImageFormat, width: u32, height: u32, colour: [u8; 4]) -> Vec<u8> {
    let picture = DynamicImage::ImageRgba8(RgbaImage::from_pixel(width, height, Rgba(colour)));
    // JPEG and BMP hold no alpha channel worth testing; give them the colour without it.
    let picture = match format {
        ImageFormat::Jpeg => DynamicImage::ImageRgb8(picture.to_rgb8()),
        _ => picture,
    };
    let mut out = Cursor::new(Vec::new());
    picture.write_to(&mut out, format).expect("encode");
    out.into_inner()
}

pub(crate) fn png(width: u32, height: u32) -> Vec<u8> {
    encoded(ImageFormat::Png, width, height, [200, 30, 30, 255])
}

/// A JPEG carrying an EXIF block whose orientation tag says `orientation`, inserted right
/// after the start-of-image marker where a camera writes it.
pub(crate) fn jpeg_with_orientation(width: u32, height: u32, orientation: u16) -> Vec<u8> {
    let plain = encoded(ImageFormat::Jpeg, width, height, [10, 120, 200, 255]);
    // A little-endian TIFF header, one IFD, one entry: tag 0x0112, type SHORT, count 1.
    let mut tiff = Vec::new();
    tiff.extend_from_slice(b"II*\0");
    tiff.extend_from_slice(&8u32.to_le_bytes());
    tiff.extend_from_slice(&1u16.to_le_bytes());
    tiff.extend_from_slice(&0x0112u16.to_le_bytes());
    tiff.extend_from_slice(&3u16.to_le_bytes());
    tiff.extend_from_slice(&1u32.to_le_bytes());
    tiff.extend_from_slice(&orientation.to_le_bytes());
    tiff.extend_from_slice(&[0, 0]);
    tiff.extend_from_slice(&0u32.to_le_bytes());
    let mut app1 = b"Exif\0\0".to_vec();
    app1.extend_from_slice(&tiff);
    let length = (app1.len() + 2) as u16;
    let mut out = plain[..2].to_vec();
    out.extend_from_slice(&[0xff, 0xe1]);
    out.extend_from_slice(&length.to_be_bytes());
    out.extend_from_slice(&app1);
    out.extend_from_slice(&plain[2..]);
    out
}

/// An animated GIF of `frames` frames, each its own colour.
pub(crate) fn gif(width: u32, height: u32, colours: &[[u8; 4]]) -> Vec<u8> {
    let mut out = Vec::new();
    {
        let mut encoder = image::codecs::gif::GifEncoder::new(&mut out);
        let frames = colours
            .iter()
            .map(|colour| image::Frame::new(RgbaImage::from_pixel(width, height, Rgba(*colour))));
        encoder.encode_frames(frames).expect("encode gif");
    }
    out
}

/// CRC-32 as PNG checksums its chunks, so a hand-built header reads as a real one.
fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = 0xffff_ffffu32;
    for &byte in bytes {
        crc ^= u32::from(byte);
        for _ in 0..8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ 0xedb8_8320
            } else {
                crc >> 1
            };
        }
    }
    !crc
}

fn chunk(kind: &[u8; 4], data: &[u8]) -> Vec<u8> {
    let mut out = (data.len() as u32).to_be_bytes().to_vec();
    let mut body = kind.to_vec();
    body.extend_from_slice(data);
    out.extend_from_slice(&body);
    out.extend_from_slice(&crc32(&body).to_be_bytes());
    out
}

/// A PNG whose header claims `width` × `height` and whose data is one empty stored block:
/// a few dozen bytes asking for as many pixels as the header says.
pub(crate) fn png_claiming(width: u32, height: u32) -> Vec<u8> {
    let mut out = vec![0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a];
    let mut ihdr = width.to_be_bytes().to_vec();
    ihdr.extend_from_slice(&height.to_be_bytes());
    ihdr.extend_from_slice(&[8, 6, 0, 0, 0]);
    out.extend_from_slice(&chunk(b"IHDR", &ihdr));
    out.extend_from_slice(&chunk(b"IDAT", &[0x78, 0x01, 0x01, 0x00, 0x00, 0xff, 0xff]));
    out.extend_from_slice(&chunk(b"IEND", &[]));
    out
}

#[test]
fn a_png_is_kept_byte_for_byte_and_named_by_its_hash() {
    let bytes = png(40, 30);
    let ingested = ingest(bytes.clone(), "word/media/image1.png").expect("a PNG comes in");
    let picture = ingested.picture;
    assert_eq!(picture.bytes, bytes, "a kept format is never re-encoded");
    assert_eq!((picture.width, picture.height), (40, 30));
    assert_eq!(picture.mime_type, "image/png");
    assert_eq!(
        picture.content_hash,
        blake3::hash(&bytes).to_hex().to_string()
    );
    assert_eq!(
        picture.relative_path(),
        format!("assets/{}.png", picture.content_hash)
    );
    assert_eq!(picture.file_name, "image1.png", "the name, never the path");
    assert!(!ingested.first_frame_only);
}

#[test]
fn a_jpeg_and_a_webp_are_kept_as_they_are() {
    let jpeg = encoded(ImageFormat::Jpeg, 16, 8, [1, 2, 3, 255]);
    let kept = ingest(jpeg.clone(), "photo.jpeg")
        .expect("a JPEG comes in")
        .picture;
    assert_eq!(kept.bytes, jpeg);
    assert_eq!(kept.mime_type, "image/jpeg");
    assert_eq!((kept.width, kept.height), (16, 8));
    assert_eq!(
        kept.file_name, "photo.jpeg",
        "a right extension keeps its spelling"
    );

    let webp = encoded(ImageFormat::WebP, 12, 7, [1, 2, 3, 255]);
    let kept = ingest(webp.clone(), "a.webp")
        .expect("a WebP comes in")
        .picture;
    assert_eq!(kept.bytes, webp);
    assert_eq!(kept.mime_type, "image/webp");
    assert_eq!((kept.width, kept.height), (12, 7));
}

/// Orientations 5 to 8 turn the picture a quarter, so the editor shows its stored height as
/// its width: the size recorded is the size shown. 1 to 4 keep the sides where they are.
#[test]
fn a_jpeg_turned_a_quarter_by_its_exif_swaps_its_sides() {
    for orientation in [5u16, 6, 7, 8] {
        let bytes = jpeg_with_orientation(40, 20, orientation);
        let kept = ingest(bytes.clone(), "portrait.jpg")
            .expect("comes in")
            .picture;
        assert_eq!(
            (kept.width, kept.height),
            (20, 40),
            "orientation {orientation} turns the picture a quarter"
        );
        assert_eq!(
            kept.bytes, bytes,
            "the orientation is the editor's to apply"
        );
    }
    for orientation in [1u16, 2, 3, 4] {
        let kept = ingest(jpeg_with_orientation(40, 20, orientation), "x.jpg")
            .expect("comes in")
            .picture;
        assert_eq!(
            (kept.width, kept.height),
            (40, 20),
            "orientation {orientation}"
        );
    }
}

#[test]
fn a_tiff_and_a_bmp_are_converted_to_png() {
    for (format, name) in [
        (ImageFormat::Tiff, "scan.tif"),
        (ImageFormat::Bmp, "old.bmp"),
    ] {
        let bytes = encoded(format, 9, 5, [0, 128, 0, 255]);
        let converted = ingest(bytes, name).expect("comes in").picture;
        assert_eq!(converted.mime_type, "image/png", "{name}");
        assert_eq!((converted.width, converted.height), (9, 5), "{name}");
        assert!(
            converted.bytes.starts_with(&[0x89, b'P', b'N', b'G']),
            "{name}"
        );
        assert!(
            converted.file_name.ends_with(".png"),
            "{name} is named for what it holds now: {}",
            converted.file_name
        );
        let decoded = image::load_from_memory(&converted.bytes).expect("a real PNG");
        assert_eq!(decoded.to_rgba8().get_pixel(4, 2).0, [0, 128, 0, 255]);
    }
}

#[test]
fn an_animated_gif_arrives_as_its_first_frame() {
    let bytes = gif(
        6,
        4,
        &[[255, 0, 0, 255], [0, 0, 255, 255], [0, 255, 0, 255]],
    );
    let ingested = ingest(bytes, "spinner.gif").expect("comes in");
    assert!(
        ingested.first_frame_only,
        "the animation is said to be lost"
    );
    let picture = ingested.picture;
    assert_eq!(picture.mime_type, "image/png");
    assert_eq!(picture.file_name, "spinner.png");
    assert_eq!((picture.width, picture.height), (6, 4));
    let decoded = image::load_from_memory(&picture.bytes).expect("a real PNG");
    assert_eq!(
        decoded.to_rgba8().get_pixel(0, 0).0,
        [255, 0, 0, 255],
        "the first frame, not a later one"
    );

    let still = ingest(gif(6, 4, &[[1, 2, 3, 255]]), "still.gif").expect("comes in");
    assert!(!still.first_frame_only, "a GIF of one frame loses nothing");
}

/// Whether a GIF has a second frame is read from its blocks, past its colour tables,
/// extensions and frame data, without decoding a frame; a file that breaks before one holds
/// one.
#[test]
fn a_second_frame_is_found_from_the_blocks_alone() {
    let two = gif(6, 4, &[[255, 0, 0, 255], [0, 0, 255, 255]]);
    let one = gif(6, 4, &[[255, 0, 0, 255]]);
    assert!(holds_a_second_frame(&two));
    assert!(!holds_a_second_frame(&one));

    // The first frame's data, cut short: no second frame to find.
    let second = two
        .iter()
        .enumerate()
        .filter(|(_, b)| **b == 0x2c)
        .map(|(at, _)| at)
        .nth(1)
        .expect("a second image descriptor");
    assert!(!holds_a_second_frame(&two[..second - 3]));
    assert!(!holds_a_second_frame(b"GIF89a"));
    assert!(!holds_a_second_frame(&[]));
}

/// A reference is named as the document wrote it, except a `data:` address, which carries
/// the picture itself and is named by its type, and a reference too long to be a path.
#[test]
fn a_reference_is_named_short() {
    assert_eq!(reference_label(" images/map.png "), "images/map.png");
    assert_eq!(
        reference_label(&format!("data:image/png;base64,{}", "A".repeat(1_000_000))),
        "data:image/png"
    );
    assert_eq!(reference_label("DATA:,plain"), "data:");
    let long = "a/".repeat(300);
    let named = reference_label(&long);
    assert_eq!(named.chars().count(), 201);
    assert!(named.ends_with('…'));
}

/// A header claiming a picture larger than a project accepts is refused on its header: the
/// few dozen bytes below would ask a decoder for tens of gigabytes.
#[test]
fn a_crafted_huge_picture_is_refused_before_it_is_decoded() {
    for (width, height) in [(100_000, 100_000), (20_000, 2), (10_000, 10_000)] {
        let bytes = png_claiming(width, height);
        assert!(bytes.len() < 100, "a tiny file");
        let started = std::time::Instant::now();
        assert_eq!(
            ingest(bytes, "bomb.png").map(|i| i.picture.width),
            Err(PictureRefusal::TooLarge),
            "{width} × {height}"
        );
        assert!(
            started.elapsed() < std::time::Duration::from_secs(5),
            "refused on its header, not after a decode"
        );
    }
}

/// A GIF's canvas is declared in its header too, and a converted format is decoded, so the
/// same ceiling has to hold before its frames are read.
#[test]
fn a_gif_claiming_a_huge_canvas_is_refused() {
    let mut bytes = gif(4, 4, &[[1, 2, 3, 255]]);
    // The logical screen's width and height, little-endian, right after the signature.
    bytes[6..8].copy_from_slice(&60_000u16.to_le_bytes());
    bytes[8..10].copy_from_slice(&60_000u16.to_le_bytes());
    assert_eq!(
        ingest(bytes, "canvas.gif").map(|i| i.picture.width),
        Err(PictureRefusal::TooLarge)
    );
}

#[test]
fn a_file_past_the_byte_ceiling_is_refused_unread() {
    let mut bytes = png(2, 2);
    bytes.resize(MAX_PICTURE_BYTES as usize + 1, 0);
    assert_eq!(
        ingest(bytes, "huge.png").map(|i| i.picture.width),
        Err(PictureRefusal::TooLarge)
    );
}

#[test]
fn a_format_the_editor_cannot_show_is_named() {
    let mut emf = vec![1, 0, 0, 0];
    emf.resize(40, 0);
    emf.extend_from_slice(b" EMF");
    emf.resize(120, 0);
    let cases: Vec<(Vec<u8>, &str)> = vec![
        (emf, "EMF"),
        (vec![0xd7, 0xcd, 0xc6, 0x9a, 0, 0, 0, 0], "WMF"),
        (
            b"<?xml version=\"1.0\"?>\n<svg xmlns=\"http://www.w3.org/2000/svg\"/>".to_vec(),
            "SVG",
        ),
        (b"  <svg width=\"1\"></svg>".to_vec(), "SVG"),
        (b"%PDF-1.7\n".to_vec(), "PDF"),
        (b"just some words".to_vec(), "unknown"),
    ];
    for (bytes, format) in cases {
        assert_eq!(
            ingest(bytes, "x").map(|i| i.picture.width),
            Err(PictureRefusal::Unsupported { format }),
            "{format}"
        );
    }
}

/// The name a file claims is never trusted: a PNG called `.jpg` is a PNG, and says so.
#[test]
fn the_format_is_read_from_the_bytes_not_the_name() {
    let kept = ingest(png(3, 3), "lying.jpg").expect("comes in").picture;
    assert_eq!(kept.mime_type, "image/png");
    assert_eq!(kept.file_name, "lying.png");
}

#[test]
fn a_picture_that_does_not_read_is_refused_not_fatal() {
    let mut truncated = png(8, 8);
    truncated.truncate(20);
    assert_eq!(
        ingest(truncated, "cut.png").map(|i| i.picture.width),
        Err(PictureRefusal::Unreadable)
    );
    let mut broken_tiff = b"II*\0".to_vec();
    broken_tiff.extend_from_slice(&[0xff; 12]);
    assert_eq!(
        ingest(broken_tiff, "broken.tif").map(|i| i.picture.width),
        Err(PictureRefusal::Unreadable)
    );
}

#[test]
fn a_kept_name_is_a_file_name_never_a_path() {
    assert_eq!(
        file_name_for("/home/writer/private/map.png", "image/png"),
        "map.png"
    );
    assert_eq!(
        file_name_for("C:\\Users\\me\\map.PNG", "image/png"),
        "map.PNG"
    );
    assert_eq!(file_name_for("", "image/jpeg"), "picture.jpg");
    assert_eq!(file_name_for("noext", "image/webp"), "noext.webp");
    assert_eq!(file_name_for(".hidden", "image/png"), ".hidden.png");
}

#[test]
fn one_picture_named_twice_is_read_once_and_kept_once() {
    let mut pictures = DocumentPictures::new();
    let bytes = png(5, 5);
    let mut reads = 0;
    let first = pictures.bring("word/media/image1.png", "image1.png", || {
        reads += 1;
        Ok(bytes.clone())
    });
    assert!(first.first_time);
    let again = pictures.bring("word/media/image1.png", "image1.png", || {
        reads += 1;
        Ok(bytes.clone())
    });
    assert!(!again.first_time, "reported once, however often shown");
    assert_eq!(reads, 1, "read once");
    assert_eq!(first.result, again.result);

    // The same bytes under another name are the same picture: one file in the project.
    let other = pictures.bring("word/media/image7.png", "image7.png", || Ok(bytes.clone()));
    assert!(other.first_time);
    assert_eq!(other.result, first.result);

    let refused = pictures.bring("gone.png", "gone.png", || Err(PictureRefusal::Missing));
    assert_eq!(refused.result, Err(PictureRefusal::Missing));
    let kept = pictures.into_pictures();
    assert_eq!(kept.len(), 1);
    assert_eq!(
        kept[0].file_name, "image1.png",
        "the first name it was shown under"
    );
}

#[test]
fn a_reference_is_read_before_anything_is_opened() {
    use Reference::*;
    assert_eq!(classify_reference("https://example.com/a.png"), Remote);
    assert_eq!(classify_reference("HTTP://example.com/a.png"), Remote);
    assert_eq!(classify_reference("ftp://example.com/a.png"), Remote);
    assert_eq!(classify_reference("file:///home/me/a.png"), Outside);
    assert_eq!(classify_reference("/home/me/a.png"), Outside);
    assert_eq!(classify_reference("C:\\Users\\me\\a.png"), Outside);
    assert_eq!(classify_reference("C:/Users/me/a.png"), Outside);
    assert_eq!(classify_reference("\\\\server\\share\\a.png"), Outside);
    assert_eq!(classify_reference("../a.png"), Outside);
    assert_eq!(classify_reference("images/../../a.png"), Outside);
    assert_eq!(classify_reference("images/..%2F..%2Fa.png"), Outside);
    assert_eq!(classify_reference("images\\a.png"), Outside);
    assert_eq!(classify_reference(""), Empty);
    assert_eq!(classify_reference("./"), Empty);
    assert_eq!(
        classify_reference("./images/./My%20Photo.png"),
        Relative(vec!["images".into(), "My Photo.png".into()])
    );
    assert_eq!(
        classify_reference("a.png#part"),
        Relative(vec!["a.png".into()])
    );
    assert_eq!(
        classify_reference("data:image/png;base64,iVBORw0KGgo="),
        Inline(Ok(vec![0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a]))
    );
    assert_eq!(
        classify_reference("data:image/svg+xml,%3Csvg%3E"),
        Inline(Ok(b"<svg>".to_vec()))
    );
    assert_eq!(
        classify_reference("data:image/png;base64,***"),
        Inline(Err(PictureRefusal::Unreadable))
    );
}

#[test]
fn base64_reads_as_a_data_address_and_an_odf_file_write_it() {
    assert_eq!(decode_base64("aGVsbG8="), Some(b"hello".to_vec()));
    assert_eq!(decode_base64("aGVs\n bG8"), Some(b"hello".to_vec()));
    assert_eq!(decode_base64(""), Some(Vec::new()));
    assert_eq!(
        decode_base64("aGVsbG8=x"),
        None,
        "nothing after the padding"
    );
    assert_eq!(decode_base64("a-b"), None);
}

#[test]
fn a_folder_reads_its_own_files_and_nothing_else() {
    let outer = tempfile::tempdir().expect("scratch");
    let folder = outer.path().join("book");
    std::fs::create_dir_all(folder.join("images")).expect("folders");
    std::fs::write(folder.join("images").join("map.png"), png(3, 2)).expect("picture");
    std::fs::write(outer.path().join("secret.png"), png(1, 1)).expect("outside picture");
    std::fs::create_dir(folder.join("dir.png")).expect("a folder named like a picture");
    let document = folder.join("chapter.md");
    std::fs::write(&document, "# One").expect("document");

    let at = DocumentFolder::of_document(&document).expect("an absolute document path");
    let read = |reference: &str| match classify_reference(reference) {
        Reference::Relative(segments) => at.read(&segments),
        other => panic!("{reference} classified as {other:?}"),
    };
    assert_eq!(read("images/map.png"), Ok(png(3, 2)));
    assert_eq!(read("images/none.png"), Err(PictureRefusal::Missing));
    assert_eq!(read("missing/map.png"), Err(PictureRefusal::Missing));
    assert_eq!(read("dir.png"), Err(PictureRefusal::Unreadable));
    assert_eq!(read("chapter.md/x.png"), Err(PictureRefusal::Missing));

    // A caller handing its own segments is held to the same rule as a classified reference.
    assert_eq!(
        at.read(&["..".to_string(), "secret.png".to_string()]),
        Err(PictureRefusal::Outside)
    );

    assert_eq!(DocumentFolder::of_document(Path::new("chapter.md")), None);
}

#[cfg(unix)]
#[test]
fn a_symbolic_link_is_never_followed() {
    let outer = tempfile::tempdir().expect("scratch");
    let folder = outer.path().join("book");
    std::fs::create_dir_all(&folder).expect("folder");
    let private = outer.path().join("private");
    std::fs::create_dir_all(&private).expect("private folder");
    std::fs::write(private.join("photo.png"), png(2, 2)).expect("private picture");
    std::os::unix::fs::symlink(private.join("photo.png"), folder.join("linked.png"))
        .expect("file link");
    std::os::unix::fs::symlink(&private, folder.join("pictures")).expect("folder link");
    let document = folder.join("chapter.md");
    std::fs::write(&document, "text").expect("document");

    let at = DocumentFolder::of_document(&document).expect("folder");
    assert_eq!(
        at.read(&["linked.png".to_string()]),
        Err(PictureRefusal::Outside)
    );
    assert_eq!(
        at.read(&["pictures".to_string(), "photo.png".to_string()]),
        Err(PictureRefusal::Outside)
    );
}

#[test]
fn a_file_past_the_ceiling_is_refused_before_it_is_read() {
    let folder = tempfile::tempdir().expect("scratch");
    let big = folder.path().join("big.png");
    // Sparse: the length is what is checked, and nothing is written to reach it.
    std::fs::File::create(&big)
        .and_then(|file| file.set_len(MAX_PICTURE_BYTES + 1))
        .expect("a long file");
    let document = folder.path().join("a.md");
    std::fs::write(&document, "text").expect("document");
    let at = DocumentFolder::of_document(&document).expect("folder");
    assert_eq!(
        at.read(&["big.png".to_string()]),
        Err(PictureRefusal::TooLarge)
    );
}

/// What a converter writing a bundle itself records for a picture, which has to be exactly
/// what a save writes for the same picture (`skrib_format::from_entities`).
#[test]
fn a_bundle_row_names_the_file_a_save_would_write() {
    let picture = ingest(png(4, 3), "media/fig.png")
        .expect("comes in")
        .picture;
    let row = picture.asset_file(7, "2026-09-28T00:00:00Z");
    assert_eq!(row.file_id, 7);
    assert_eq!(row.content_hash, picture.content_hash);
    assert_eq!(row.path, picture.relative_path());
    assert_eq!(
        row.path,
        skrib_format::media::asset_relpath(
            &picture.content_hash,
            &skrib_format::media::extension_for(&row.mime_type)
        )
    );
    assert_eq!((row.width, row.height), (4, 3));
    assert_eq!(row.byte_size, picture.bytes.len() as u64);
    assert_eq!(row.file_name, "fig.png");
    assert_eq!(
        picture.stored_name(),
        format!("{}.png", picture.content_hash)
    );
}

#[cfg(any(feature = "odt", feature = "docx"))]
#[test]
fn a_package_reads_its_members_and_refuses_what_it_lacks() {
    use std::io::Write;
    let mut archive = Vec::new();
    {
        let mut zip = zip::ZipWriter::new(Cursor::new(&mut archive));
        let options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored);
        zip.start_file("Pictures/a.png", options).expect("member");
        zip.write_all(&png(2, 3)).expect("bytes");
        zip.add_directory("Pictures/sub/", options)
            .expect("directory");
        zip.finish().expect("archive");
    }
    let mut package = PackagePictures::open(&archive);
    assert_eq!(package.read("Pictures/a.png"), Ok(png(2, 3)));
    assert_eq!(package.read("Pictures/b.png"), Err(PictureRefusal::Missing));
    assert_eq!(
        package.read("Pictures/sub/"),
        Err(PictureRefusal::Unreadable)
    );

    let mut not_a_zip = PackagePictures::open(b"plain text");
    assert_eq!(not_a_zip.read("a.png"), Err(PictureRefusal::Missing));
}

/// A document's pictures together are bounded, so one naming a great many large files cannot
/// fill memory before any of them is stored. What comes before the bound still comes, and a
/// picture already held is no more bytes to hold.
#[test]
fn a_document_holds_its_pictures_up_to_a_budget() {
    let first = png(20, 20);
    let second = png(21, 21);
    let mut pictures = DocumentPictures::with_budget(first.len() as u64 + 10);
    assert!(
        pictures
            .bring("a.png", "a.png", || Ok(first.clone()))
            .result
            .is_ok()
    );
    assert!(
        pictures
            .bring("same-bytes.png", "same.png", || Ok(first.clone()))
            .result
            .is_ok(),
        "the same picture again costs nothing"
    );
    let refused = pictures.bring("b.png", "b.png", || Ok(second.clone()));
    assert_eq!(refused.result, Err(PictureRefusal::DocumentFull));
    assert!(refused.first_time, "reported");
    assert_eq!(pictures.into_pictures().len(), 1);
}
