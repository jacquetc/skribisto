// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What the Gallery shows, as plain data: the notes of a folder turned into tiles.
//!
//! A gallery is a gallery: its tiles are pictures. A note with three pictures is one tile
//! in *First image* (its first, with a badge saying how many more) and three in *All
//! images*. A note with no picture is left out, unless the writer asks for *Notes without
//! images*, when it is a tile of its opening words. In *Nested* a subfolder is a tile of
//! its own, its name over the first four pictures found in it and anywhere beneath it; in
//! *Flat* there are no folder tiles, and every picture below the folder is shown.
//!
//! ## Picture resources
//!
//! A picture **resource** (a photograph attached to a note, not put into its text) counts
//! as a picture of the item it is attached to, after the pictures of its text: it is in
//! the badge's count in First image, a tile of its own in All images, a preview of its
//! folder's tile in Nested. A resource attached to a folder is that folder's picture: a
//! preview of its tile in Nested, a tile of its own in Flat, and, for the folder the
//! Gallery shows, tiles ahead of everything else ([`GalleryNote::is_folder`] is false for
//! that one row, which stands for the folder itself). A resource on a folder and on items
//! inside it is one picture of that folder's tile.
//!
//! The order is the order of the notes the caller hands in, which is the Overview's:
//! binder order, or the Overview's sort, with its search and filters applied
//! (`models::overview_rows_model::shaped_subtree`). This module does not reorder.

use crate::models::ResourceRow;
use crate::shared::images::ImageRef;

/// How many pictures a folder tile previews.
pub const FOLDER_PREVIEWS: usize = 4;

/// How many characters of a text note a tile shows.
pub const EXCERPT_CHARS: usize = 280;

/// One picture of a note: one in its text, or a picture resource attached to it.
#[derive(Debug, Clone, PartialEq)]
pub struct GalleryPicture {
    /// The picture as a reference: a text's own, or a resource's `resource:<uid>` with its
    /// description and its stored size.
    pub image: ImageRef,
    /// The resource it is, `None` for a picture in a note's text.
    pub resource: Option<Box<ResourceRow>>,
}

impl GalleryPicture {
    /// A picture in a note's text.
    pub fn in_text(image: ImageRef) -> Self {
        Self {
            image,
            resource: None,
        }
    }

    /// A picture resource, `index`th among its item's pictures.
    pub fn of_resource(row: ResourceRow, index: usize) -> Self {
        let size = (row.width > 0 && row.height > 0).then(|| {
            (
                u32::try_from(row.width).unwrap_or(u32::MAX),
                u32::try_from(row.height).unwrap_or(u32::MAX),
            )
        });
        let alt = if row.description.trim().is_empty() {
            row.title.clone()
        } else {
            row.description.clone()
        };
        Self {
            image: ImageRef {
                index,
                src: crate::resources::links::resource_href(row.uid),
                alt,
                size,
            },
            resource: Some(Box::new(row)),
        }
    }

    pub fn is_resource(&self) -> bool {
        self.resource.is_some()
    }
}

/// One row of the folder's subtree, with what the Gallery needs from it.
#[derive(Debug, Clone, PartialEq)]
pub struct GalleryNote {
    pub item_id: u64,
    pub title: String,
    /// Depth below the folder the Gallery shows: `0` for its own children.
    pub depth: usize,
    /// A folder (any `Folder` role), shown as a folder tile in Nested.
    pub is_folder: bool,
    /// Its pictures: those of its text in document order, then its picture resources.
    /// A folder's are its picture resources only.
    pub pictures: Vec<GalleryPicture>,
    /// Its opening words, for a tile when it has no picture.
    pub excerpt: String,
}

/// The Gallery's three switches.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GalleryOptions {
    /// One tile per picture, rather than one per note.
    pub all_images: bool,
    /// Subfolders as tiles one can enter, rather than everything at once.
    pub nested: bool,
    /// Text-only notes as tiles of their opening words.
    pub text_notes: bool,
}

impl Default for GalleryOptions {
    fn default() -> Self {
        Self {
            all_images: false,
            nested: true,
            text_notes: false,
        }
    }
}

/// One tile.
#[derive(Debug, Clone, PartialEq)]
pub enum GalleryTile {
    Picture {
        item_id: u64,
        title: String,
        picture: GalleryPicture,
        /// Pictures of the same note this tile does not show: the badge in First image,
        /// always `0` in All images.
        more: usize,
    },
    Text {
        item_id: u64,
        title: String,
        excerpt: String,
    },
    Folder {
        item_id: u64,
        title: String,
        /// Up to [`FOLDER_PREVIEWS`] pictures from the folder and anywhere beneath it, in
        /// order.
        previews: Vec<GalleryPicture>,
        /// Every picture of the folder and beneath it.
        pictures: usize,
    },
}

impl GalleryTile {
    /// The note or folder the tile stands for.
    pub fn item_id(&self) -> u64 {
        match self {
            Self::Picture { item_id, .. }
            | Self::Text { item_id, .. }
            | Self::Folder { item_id, .. } => *item_id,
        }
    }

    pub fn title(&self) -> &str {
        match self {
            Self::Picture { title, .. } | Self::Text { title, .. } | Self::Folder { title, .. } => {
                title
            }
        }
    }

    /// The picture the tile shows, when it shows exactly one.
    pub fn image(&self) -> Option<&ImageRef> {
        match self {
            Self::Picture { picture, .. } => Some(&picture.image),
            _ => None,
        }
    }

    /// The resource the tile shows, when it shows one picture resource.
    pub fn resource(&self) -> Option<&ResourceRow> {
        match self {
            Self::Picture { picture, .. } => picture.resource.as_deref(),
            _ => None,
        }
    }
}

/// The tiles for `notes` under `options`.
pub fn build_tiles(notes: &[GalleryNote], options: GalleryOptions) -> Vec<GalleryTile> {
    let mut tiles = Vec::new();
    for (at, note) in notes.iter().enumerate() {
        if options.nested && note.depth > 0 {
            // Inside a subfolder: that folder's tile stands for it.
            continue;
        }
        if note.is_folder {
            if options.nested {
                let beneath = subtree(notes, at);
                // A resource on the folder and on a note inside it is one file: counted
                // once, and one preview. A picture in two notes' texts is a copy in each.
                let mut seen = std::collections::HashSet::new();
                let pictures: Vec<&GalleryPicture> = note
                    .pictures
                    .iter()
                    .chain(beneath.iter().flat_map(|n| n.pictures.iter()))
                    .filter(|p| p.resource.as_ref().is_none_or(|r| seen.insert(r.id)))
                    .collect();
                tiles.push(GalleryTile::Folder {
                    item_id: note.item_id,
                    title: note.title.clone(),
                    previews: pictures
                        .iter()
                        .take(FOLDER_PREVIEWS)
                        .map(|p| (*p).clone())
                        .collect(),
                    pictures: pictures.len(),
                });
            } else if !note.pictures.is_empty() {
                // A folder's own synopsis is not a note's text, so in Flat it is never a
                // text tile; its picture resources are pictures like any other.
                note_tiles(
                    note,
                    GalleryOptions {
                        text_notes: false,
                        ..options
                    },
                    &mut tiles,
                );
            }
            continue;
        }
        note_tiles(note, options, &mut tiles);
    }
    tiles
}

fn note_tiles(note: &GalleryNote, options: GalleryOptions, tiles: &mut Vec<GalleryTile>) {
    match note.pictures.split_first() {
        Some((first, rest)) if !options.all_images => tiles.push(GalleryTile::Picture {
            item_id: note.item_id,
            title: note.title.clone(),
            picture: first.clone(),
            more: rest.len(),
        }),
        Some(_) => tiles.extend(note.pictures.iter().map(|picture| GalleryTile::Picture {
            item_id: note.item_id,
            title: note.title.clone(),
            picture: picture.clone(),
            more: 0,
        })),
        None if options.text_notes => tiles.push(GalleryTile::Text {
            item_id: note.item_id,
            title: note.title.clone(),
            excerpt: note.excerpt.clone(),
        }),
        None => {}
    }
}

/// The rows strictly beneath `notes[at]`: those after it that are deeper, up to the first
/// that is not.
fn subtree(notes: &[GalleryNote], at: usize) -> &[GalleryNote] {
    let depth = notes[at].depth;
    let rest = &notes[at + 1..];
    let end = rest
        .iter()
        .position(|n| n.depth <= depth)
        .unwrap_or(rest.len());
    &rest[..end]
}

/// The opening of `plain`, at most [`EXCERPT_CHARS`] characters, its runs of white space
/// folded to one space, cut at a word when one ends near the limit.
pub fn excerpt(plain: &str) -> String {
    let folded: String = plain
        .replace('\u{fffc}', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    if folded.chars().count() <= EXCERPT_CHARS {
        return folded;
    }
    let cut: String = folded.chars().take(EXCERPT_CHARS).collect();
    let at_word = cut
        .rfind(' ')
        .filter(|&i| i > EXCERPT_CHARS / 2)
        .map_or(cut.as_str(), |i| &cut[..i]);
    format!("{}…", at_word.trim_end())
}

/// Whether any note under the folder has text but no picture: what makes the empty
/// Gallery worth a hint about *Notes without images*.
pub fn has_text_only_notes(notes: &[GalleryNote]) -> bool {
    notes
        .iter()
        .any(|n| !n.is_folder && n.pictures.is_empty() && !n.excerpt.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ResourceKind;

    fn picture(src: &str) -> GalleryPicture {
        GalleryPicture::in_text(ImageRef {
            index: 0,
            src: src.to_string(),
            alt: String::new(),
            size: Some((100, 50)),
        })
    }

    /// A picture resource whose title is `name`, so the tiles below can name it.
    fn resource(name: &str) -> GalleryPicture {
        GalleryPicture::of_resource(
            ResourceRow {
                // One resource per name: the same name in two notes is one file on both.
                id: name.bytes().map(u64::from).sum(),
                uid: uuid::Uuid::from_u128(name.len() as u128),
                title: name.to_string(),
                kind: ResourceKind::StoredFile,
                file_name: format!("{name}.jpg"),
                width: 1600,
                height: 900,
                ..Default::default()
            },
            0,
        )
    }

    fn note(id: u64, depth: usize, pictures: &[&str], excerpt: &str) -> GalleryNote {
        GalleryNote {
            item_id: id,
            title: format!("note {id}"),
            depth,
            is_folder: false,
            pictures: pictures
                .iter()
                .map(|s| match s.strip_prefix("res:") {
                    Some(name) => resource(name),
                    None => picture(s),
                })
                .collect(),
            excerpt: excerpt.to_string(),
        }
    }

    fn folder(id: u64, depth: usize) -> GalleryNote {
        GalleryNote {
            item_id: id,
            title: format!("folder {id}"),
            depth,
            is_folder: true,
            pictures: Vec::new(),
            excerpt: String::new(),
        }
    }

    /// A folder of mixed notes with a subfolder holding five pictures over two notes.
    fn folder_of_research() -> Vec<GalleryNote> {
        vec![
            note(1, 0, &["a", "b", "c"], ""),
            note(2, 0, &[], "a quotation typed by hand"),
            folder(3, 0),
            note(4, 1, &["d", "e"], ""),
            note(5, 1, &["f", "g", "h"], ""),
            note(6, 0, &["i"], ""),
        ]
    }

    /// The name a picture goes by in these tests: its source, or `res:` and the
    /// resource's title.
    fn name(p: &GalleryPicture) -> String {
        match &p.resource {
            Some(row) => format!("res:{}", row.title),
            None => p.image.src.clone(),
        }
    }

    fn srcs(tiles: &[GalleryTile]) -> Vec<String> {
        tiles
            .iter()
            .map(|t| match t {
                GalleryTile::Picture { picture, more, .. } => format!("{}+{more}", name(picture)),
                GalleryTile::Text { item_id, .. } => format!("text {item_id}"),
                GalleryTile::Folder {
                    previews, pictures, ..
                } => format!(
                    "folder[{}]/{pictures}",
                    previews.iter().map(name).collect::<Vec<_>>().join("")
                ),
            })
            .collect()
    }

    #[test]
    fn first_image_nested_shows_one_tile_per_note_and_folder_tiles() {
        let tiles = build_tiles(&folder_of_research(), GalleryOptions::default());
        assert_eq!(srcs(&tiles), ["a+2", "folder[defg]/5", "i+0"]);
    }

    #[test]
    fn all_images_flat_shows_every_picture_beneath_in_order() {
        let options = GalleryOptions {
            all_images: true,
            nested: false,
            text_notes: false,
        };
        let tiles = build_tiles(&folder_of_research(), options);
        assert_eq!(
            srcs(&tiles),
            [
                "a+0", "b+0", "c+0", "d+0", "e+0", "f+0", "g+0", "h+0", "i+0"
            ]
        );
    }

    #[test]
    fn text_notes_appear_only_when_asked_for() {
        let notes = folder_of_research();
        assert!(
            !srcs(&build_tiles(&notes, GalleryOptions::default())).contains(&"text 2".to_string())
        );
        let options = GalleryOptions {
            text_notes: true,
            ..GalleryOptions::default()
        };
        assert_eq!(
            srcs(&build_tiles(&notes, options)),
            ["a+2", "text 2", "folder[defg]/5", "i+0"]
        );
        assert!(has_text_only_notes(&notes));
    }

    #[test]
    fn a_folder_tile_previews_from_its_whole_subtree_and_stops_at_its_end() {
        let notes = vec![
            folder(1, 0),
            folder(2, 1),
            note(3, 2, &["deep"], ""),
            note(4, 1, &["near"], ""),
            note(5, 0, &["after"], ""),
        ];
        let tiles = build_tiles(&notes, GalleryOptions::default());
        assert_eq!(srcs(&tiles), ["folder[deepnear]/2", "after+0"]);
    }

    #[test]
    fn an_empty_folder_is_still_a_tile_to_enter() {
        let tiles = build_tiles(&[folder(1, 0)], GalleryOptions::default());
        assert_eq!(srcs(&tiles), ["folder[]/0"]);
    }

    /// The research folder again, with picture resources: one on a note that has
    /// pictures in its text, one on a note that has none, and one on the subfolder.
    fn research_with_resources() -> Vec<GalleryNote> {
        let mut notes = folder_of_research();
        notes[0].pictures.push(resource("Map"));
        notes[1].pictures.push(resource("Quay"));
        notes[2].pictures.push(resource("Gull"));
        notes
    }

    #[test]
    fn a_picture_resource_counts_as_a_picture_of_its_item_in_first_image() {
        let tiles = build_tiles(&research_with_resources(), GalleryOptions::default());
        // Note 1 now has four pictures; note 2 is a picture tile, no longer a text note;
        // the subfolder's own resource leads its previews and its count.
        assert_eq!(
            srcs(&tiles),
            ["a+3", "res:Quay+0", "folder[res:Gulldef]/6", "i+0"]
        );
        assert!(
            tiles[1].resource().is_some(),
            "the tile knows it is a resource"
        );
        assert!(tiles[0].resource().is_none());
    }

    /// A resource attached to a folder and to a note inside it is one picture of the
    /// folder's tile, in its count and in its previews; a picture repeated in two notes'
    /// texts is still two.
    #[test]
    fn a_resource_on_a_folder_and_inside_it_is_counted_once() {
        let notes = vec![
            GalleryNote {
                pictures: vec![resource("Map")],
                ..folder(1, 0)
            },
            note(2, 1, &["res:Map", "a"], ""),
            note(3, 1, &["res:Map", "a", "res:Gull"], ""),
        ];
        let tiles = build_tiles(&notes, GalleryOptions::default());
        assert_eq!(srcs(&tiles), ["folder[res:Mapaares:Gull]/4"]);
    }

    #[test]
    fn a_picture_resource_is_a_tile_of_its_own_in_all_images() {
        let options = GalleryOptions {
            all_images: true,
            ..GalleryOptions::default()
        };
        let tiles = build_tiles(&research_with_resources(), options);
        assert_eq!(
            srcs(&tiles),
            [
                "a+0",
                "b+0",
                "c+0",
                "res:Map+0",
                "res:Quay+0",
                "folder[res:Gulldef]/6",
                "i+0"
            ]
        );
    }

    #[test]
    fn in_flat_a_folders_resources_are_tiles_and_its_notes_follow() {
        let options = GalleryOptions {
            all_images: true,
            nested: false,
            text_notes: true,
        };
        let tiles = build_tiles(&research_with_resources(), options);
        assert_eq!(
            srcs(&tiles),
            [
                "a+0",
                "b+0",
                "c+0",
                "res:Map+0",
                "res:Quay+0",
                "res:Gull+0",
                "d+0",
                "e+0",
                "f+0",
                "g+0",
                "h+0",
                "i+0"
            ]
        );
        // A folder with no picture is still no tile in Flat, Notes without images or not.
        let bare = build_tiles(&folder_of_research(), options);
        assert!(!srcs(&bare).iter().any(|s| s.starts_with("text 3")));
    }

    #[test]
    fn a_note_whose_only_picture_is_a_resource_is_not_a_text_note() {
        let notes = vec![note(1, 0, &["res:Map"], "words too")];
        assert!(!has_text_only_notes(&notes));
        let options = GalleryOptions {
            text_notes: true,
            ..GalleryOptions::default()
        };
        assert_eq!(srcs(&build_tiles(&notes, options)), ["res:Map+0"]);
    }

    #[test]
    fn a_resource_picture_is_laid_out_from_its_stored_size_and_named_by_its_uid() {
        let p = resource("Map");
        assert_eq!(p.image.size, Some((1600, 900)));
        assert!(p.image.src.starts_with("resource:"), "{}", p.image.src);
        assert_eq!(p.image.alt, "Map", "no description: its title");
        let unknown = GalleryPicture::of_resource(ResourceRow::default(), 2);
        assert_eq!(
            unknown.image.size, None,
            "no stored size: 4:3 until it decodes"
        );
        assert_eq!(unknown.image.index, 2);
    }

    #[test]
    fn an_excerpt_folds_white_space_and_cuts_at_a_word() {
        assert_eq!(excerpt("  a\n\n quote\u{fffc}here "), "a quote here");
        let long = "word ".repeat(200);
        let cut = excerpt(&long);
        assert!(cut.ends_with("word…"), "{cut}");
        assert!(cut.chars().count() <= EXCERPT_CHARS + 1);
        // One long unbroken run is cut where it must be.
        let run = "x".repeat(1000);
        assert_eq!(excerpt(&run).chars().count(), EXCERPT_CHARS + 1);
    }
}
