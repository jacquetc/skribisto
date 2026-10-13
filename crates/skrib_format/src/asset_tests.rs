// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Assets through a full bundle round trip, in both project shapes.
//!
//! The interesting cases are not "does a byte array survive a map" — they are
//! the ones where the two writers' own design works against binary content: the
//! zip rebuilds the archive from a fresh staging directory on every save, and
//! the exploded writer deletes any file it does not expect. Both are why the
//! format version has a floor for asset-bearing projects, and both are what
//! these tests exercise.

use std::collections::BTreeMap;

use super::bundle::*;
use super::media::{asset_relpath, extension_for};

/// A bundle with two images, built directly rather than through `from_entities`
/// so a test can state exactly what it is round-tripping.
pub(crate) fn bundle_with_assets() -> WorkBundle {
    let mut b = super::tests::build_bundle(ShapeTag::Folder);
    let png = vec![0x89, b'P', b'N', b'G', 1, 2, 3, 4];
    let jpg = vec![0xff, 0xd8, 0xff, 9, 9];
    b.assets = vec![
        AssetFile {
            file_id: 50,
            created_at: "2026-01-01T00:00:00Z".into(),
            updated_at: "2026-01-01T00:00:00Z".into(),
            content_hash: "hash-cover".into(),
            file_name: "cover.png".into(),
            mime_type: "image/png".into(),
            width: 640,
            height: 480,
            byte_size: png.len() as u64,
            alt: "the cover".into(),
            path: asset_relpath("hash-cover", &extension_for("image/png")),
        },
        AssetFile {
            file_id: 51,
            created_at: "2026-01-01T00:00:00Z".into(),
            updated_at: "2026-01-01T00:00:00Z".into(),
            content_hash: "hash-map".into(),
            file_name: "map.jpg".into(),
            mime_type: "image/jpeg".into(),
            width: 800,
            height: 600,
            byte_size: jpg.len() as u64,
            alt: String::new(),
            path: asset_relpath("hash-map", &extension_for("image/jpeg")),
        },
    ];
    b.asset_bytes = BTreeMap::from([
        ("hash-cover".to_string(), png),
        ("hash-map".to_string(), jpg),
    ]);
    b
}

fn round_trip(shape: super::SkribShape) -> WorkBundle {
    let dir = tempfile::tempdir().expect("tmp");
    let target = dir.path().join("Novel.skrib");
    let path = target.to_string_lossy().into_owned();
    super::write_bundle(&path, shape, &bundle_with_assets()).expect("write");
    super::read_bundle(&path).expect("read")
}

#[test]
fn an_image_survives_an_exploded_folder_round_trip() {
    let back = round_trip(super::SkribShape::ExplodedFolder);
    assert_eq!(back.assets.len(), 2);
    assert_eq!(back.asset_bytes.get("hash-cover").map(|b| b.len()), Some(8));
    assert_eq!(back.asset_bytes.get("hash-map").map(|b| b.len()), Some(5));
}

#[test]
fn an_image_survives_a_zip_round_trip() {
    // The zip writer rebuilds the archive from a fresh staging directory, so
    // anything not modelled in `WorkBundle` vanishes on the next save. This is
    // the test that says assets are modelled.
    let back = round_trip(super::SkribShape::ZipFile);
    assert_eq!(back.assets.len(), 2);
    assert_eq!(
        back.asset_bytes.get("hash-cover").map(|b| b.as_slice()),
        Some([0x89, b'P', b'N', b'G', 1, 2, 3, 4].as_slice())
    );
}

#[test]
fn asset_metadata_survives_intact() {
    let back = round_trip(super::SkribShape::ZipFile);
    let cover = back
        .assets
        .iter()
        .find(|a| a.content_hash == "hash-cover")
        .expect("cover");
    assert_eq!(cover.file_name, "cover.png");
    assert_eq!(cover.mime_type, "image/png");
    assert_eq!((cover.width, cover.height), (640, 480));
    assert_eq!(cover.alt, "the cover");
    assert_eq!(cover.path, "assets/hash-cover.png");
}

#[test]
fn a_replaced_image_does_not_leave_its_predecessor_behind() {
    // Assets are content-addressed, so replacing one writes a *new* filename.
    // Without the prune the old blob would stay on disk forever, and a project
    // would accumulate every version of every picture ever swapped out.
    let dir = tempfile::tempdir().expect("tmp");
    let target = dir.path().join("Novel.skrib");
    let path = target.to_string_lossy().into_owned();

    super::write_bundle(
        &path,
        super::SkribShape::ExplodedFolder,
        &bundle_with_assets(),
    )
    .expect("first write");
    assert!(target.join("assets/hash-cover.png").exists());

    let mut swapped = bundle_with_assets();
    swapped.assets.retain(|a| a.content_hash != "hash-cover");
    swapped.asset_bytes.remove("hash-cover");
    super::write_bundle(&path, super::SkribShape::ExplodedFolder, &swapped).expect("second write");

    assert!(
        !target.join("assets/hash-cover.png").exists(),
        "the orphaned blob was left behind"
    );
    assert!(
        target.join("assets/hash-map.jpg").exists(),
        "the surviving image must not be pruned with it"
    );
}

#[test]
fn a_project_with_images_raises_its_read_floor() {
    // An older build has no assets field and both writers rebuild from what the
    // bundle holds, so its first save would delete every image. The floor makes
    // that a refusal to open instead.
    let with = bundle_with_assets();
    assert_eq!(super::version_gate::compute_min_read_version(&with), 8);
}

#[test]
fn a_project_without_images_keeps_the_lower_floor() {
    // The floor is content-driven: adding the *capability* must not lock every
    // existing project out of every older build.
    let without = super::tests::build_bundle(ShapeTag::Folder);
    assert!(
        super::version_gate::compute_min_read_version(&without) < 8,
        "an image-free project should still open in an older build"
    );
}

#[test]
fn image_bytes_never_reach_the_content_fingerprint() {
    // `asset_bytes` is `#[serde(skip)]`, and the fingerprint RON-serialises the
    // whole bundle on every save. Were the bytes included, each image would be
    // encoded as a bracketed decimal list several times its own size, on the
    // hottest path in the format.
    let mut a = bundle_with_assets();
    let before = super::content_fingerprint(&a);

    // Same metadata, different bytes: the hash is what identifies content, and
    // it lives in the metadata, so the fingerprint still changes when an image
    // really changes — via `content_hash`, not via the pixels.
    a.asset_bytes
        .insert("hash-cover".to_string(), vec![7, 7, 7, 7, 7, 7, 7, 7]);
    assert_eq!(
        super::content_fingerprint(&a),
        before,
        "raw bytes must not participate in the fingerprint"
    );

    a.assets[0].content_hash = "hash-cover-v2".into();
    assert_ne!(
        super::content_fingerprint(&a),
        before,
        "a changed content hash must change the fingerprint"
    );
}

#[test]
fn an_asset_with_no_bytes_fails_the_write_rather_than_writing_a_dangling_row() {
    // `from_entities` drops such a row before it gets here; if one ever reaches
    // the writer it must be loud, because a bundle listing an asset it does not
    // contain reads back as a hard error on the *next* open.
    let mut b = bundle_with_assets();
    b.asset_bytes.remove("hash-cover");
    let dir = tempfile::tempdir().expect("tmp");
    let path = dir
        .path()
        .join("Novel.skrib")
        .to_string_lossy()
        .into_owned();
    let err = super::write_bundle(&path, super::SkribShape::ExplodedFolder, &b)
        .expect_err("a missing blob must not write silently");
    assert!(
        format!("{err:#}").contains("cover.png"),
        "the error should name the asset: {err:#}"
    );
}

// ── a cover per Book ──

/// The sample project with its Book carrying picture 50 as its cover. `bytes` says
/// whether the picture's bytes reach the writer.
fn bundle_with_a_book_cover(shape: ShapeTag, bytes: bool) -> (WorkBundle, u64) {
    let mut book_id = 0;
    let asset_bytes = if bytes {
        BTreeMap::from([("hash-of-cover".to_string(), vec![0x89, b'P', b'N', b'G'])])
    } else {
        BTreeMap::new()
    };
    let bundle = super::tests::build_bundle_with_bytes(shape, asset_bytes, |s| {
        let book = s
            .binders
            .iter_mut()
            .flat_map(|b| b.items.iter_mut())
            .find(|i| i.item.sub_role == common::entities::BinderItemSubRole::Book)
            .expect("the sample project has a Book");
        book.item.cover = Some(50);
        book_id = book.item.id;
    });
    (bundle, book_id)
}

fn cover_of(bundle: &WorkBundle, item_id: u64) -> Option<u64> {
    bundle
        .binders
        .iter()
        .flat_map(|b| b.items.iter())
        .find(|i| i.item.file_id == item_id)
        .expect("item")
        .item
        .cover_id
}

/// A Book's cover is written, read back in both shapes, and reaches the loaded project as
/// a link to the right picture. Deliberately a non-default value end to end: a fixture
/// leaving it `None` would pass whether or not the field round-tripped.
#[test]
fn a_books_cover_survives_a_round_trip_in_both_shapes() {
    for (tag, shape) in [
        (ShapeTag::Zip, super::SkribShape::ZipFile),
        (ShapeTag::Folder, super::SkribShape::ExplodedFolder),
    ] {
        let (bundle, book) = bundle_with_a_book_cover(tag, true);
        assert_eq!(cover_of(&bundle, book), Some(50));

        let dir = tempfile::tempdir().expect("tmp");
        let path = dir
            .path()
            .join("Novel.skrib")
            .to_string_lossy()
            .into_owned();
        super::write_bundle(&path, shape, &bundle).expect("write");
        let back = super::read_bundle(&path).expect("read");
        assert_eq!(cover_of(&back, book), Some(50), "{shape:?}");

        let loaded = super::bundle_to_loaded(back, &path).expect("load");
        let item = loaded
            .binders
            .iter()
            .flat_map(|b| b.items.iter())
            .find(|i| i.item.id == book)
            .expect("loaded book");
        assert_eq!(item.cover_id, Some(50), "{shape:?}");
        assert!(loaded.assets.iter().any(|a| a.id == 50), "{shape:?}");
    }
}

/// A picture whose bytes could not be read is dropped from the bundle; its Book is saved
/// without a cover rather than pointing at a file id nothing in the bundle carries.
#[test]
fn a_cover_whose_picture_is_not_written_is_not_saved() {
    let (bundle, book) = bundle_with_a_book_cover(ShapeTag::Zip, false);
    assert!(bundle.assets.is_empty());
    assert_eq!(cover_of(&bundle, book), None);
}

/// A v14 asset still carries `is_cover`. It reads, and the flag is simply not kept: no
/// migration, by decision (see `FORMAT_VERSION`).
#[test]
fn a_v14_cover_flag_reads_and_is_dropped() {
    let v14 = r#"AssetFile(
        file_id: 7,
        created_at: "2026-01-01T00:00:00Z",
        updated_at: "2026-01-01T00:00:00Z",
        content_hash: "abc",
        file_name: "front.png",
        mime_type: "image/png",
        width: 2,
        height: 2,
        byte_size: 4,
        alt: "",
        is_cover: true,
        path: "assets/abc.png",
    )"#;
    let a: AssetFile = ron::from_str(v14).expect("a v14 asset still parses");
    assert_eq!(a.file_id, 7);
    assert_eq!(a.content_hash, "abc");
}
