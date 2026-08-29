// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The TOML schema both subcommands read.
//!
//! **One file per bundled example**, describing the whole of it: the work's own
//! settings, how to cut the source text into chapters, the paratext, the tag
//! vocabulary, the story bible, and every chapter's editorial metadata. `convert`
//! reads the first half, `enrich` the second, and both parse the *same* struct with
//! `deny_unknown_fields` — so a key misspelt in a section this subcommand happens not
//! to use is still an error, rather than silently doing nothing until someone
//! notices a missing synopsis months later.
//!
//! Everything here is **editorial description written by the Skribisto project**,
//! never text lifted from the book. See each example's `NOTICE`.

use serde::Deserialize;
use uuid::Uuid;

/// One bundled example, end to end.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Spec {
    pub book: Book,
    #[serde(default)]
    pub source: Source,
    #[serde(default)]
    pub paratext: Vec<Paratext>,
    #[serde(default)]
    pub tag: Vec<Tag>,
    /// The project's workflow ladder, **in ladder order** — that order is the ladder, so
    /// the sequence here is the data, not presentation.
    #[serde(default)]
    pub status: Vec<Status>,
    #[serde(default)]
    pub bible: Bible,
    #[serde(default)]
    pub character: Vec<Entry>,
    #[serde(default)]
    pub place: Vec<Entry>,
    #[serde(default)]
    pub object: Vec<Entry>,
    #[serde(default)]
    pub chapter: Vec<Chapter>,
}

/// The `Work` row itself.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Book {
    pub title: String,
    pub author: String,
    /// `Work.unique_id`. **Pinned in the file, not minted**: it is what every
    /// per-project settings file keys on, so regenerating the bundle with a fresh
    /// one would orphan the reader's remembered desk, expand state and backups.
    pub unique_id: String,
    /// BCP-47 dictionary tags, e.g. `["fr-FR-x-classique"]`.
    #[serde(default)]
    pub language: Vec<String>,
    /// The single RFC-3339 stamp written as every row's `created_at`/`updated_at`.
    ///
    /// Fixed rather than "now" so the tool is **reproducible**: regenerating an
    /// unchanged example must produce an unchanged file, or every run is a
    /// whole-bundle diff in review and nobody can see what actually changed.
    pub timestamp: String,
    #[serde(default = "yes")]
    pub chapter_flat: bool,
    #[serde(default = "yes")]
    pub number_chapters: bool,
    /// The manuscript binder's name.
    pub manuscript_binder: String,
    /// The `Folder/Book` row every chapter hangs under.
    pub book_folder: String,
    /// `Folder/Paratext` wrapping the `section = "front"` pages. Empty puts them
    /// flat at the top of the binder instead.
    #[serde(default)]
    pub front_matter: String,
    /// The same for `section = "back"`.
    #[serde(default)]
    pub back_matter: String,
    /// That row's synopsis (Markdown).
    #[serde(default)]
    pub synopsis: String,
}

/// How to cut the plain-text source into chapters — `convert` only.
///
/// A chapter always starts at **a line holding nothing but a Roman numeral**; that
/// is not a knob because it is not a variable. Both public-domain sources this tool
/// was written against (and every Project Gutenberg French novel of the period spot
/// checked alongside them) mark chapters exactly that way, and a knob nobody varies
/// is a lie about how general the tool is.
#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    /// Drop from a line starting with any of these up to and including the next
    /// line containing `]` — Project Gutenberg's `[Illustration: …]` blocks, which
    /// run to two lines when the caption wraps.
    #[serde(default)]
    pub drop_blocks_starting_with: Vec<String>,
    /// Everything from the first line starting with any of these is back matter
    /// (a table of contents, a transcriber's correction list, a printer's mark)
    /// and is discarded.
    #[serde(default)]
    pub stop_at_lines_starting_with: Vec<String>,
    /// Literal, ordered substitutions applied to the whole source before parsing —
    /// `["--", "—"]` for a transcription that spells the em dash as two hyphens.
    #[serde(default)]
    pub replace: Vec<[String; 2]>,
    /// Turn the typewriter apostrophe into U+2019. Every transcription checked
    /// uses `'`, and French prose set with it looks wrong in a book.
    #[serde(default)]
    pub curly_apostrophes: bool,
    /// Treat a paragraph opening `[n] ` as footnote *n*'s body, and the matching
    /// `[n]` in the prose above it as the reference.
    #[serde(default)]
    pub footnotes: bool,
}

/// A front- or back-matter page: not part of the book body, excluded from every
/// statistic, exported in stream order like a scene.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Paratext {
    /// `"front"` or `"back"`.
    pub section: String,
    pub title: String,
    /// Markdown, converted to Djot on the way in.
    #[serde(default)]
    pub body: String,
}

/// A `BinderTag`. `discoverable` is what puts an item's title and aliases into the
/// mention index and the point-of-view picker.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tag {
    pub name: String,
    #[serde(default)]
    pub color: String,
    #[serde(default)]
    pub details: String,
    #[serde(default)]
    pub discoverable: bool,
}

/// One rung of the workflow ladder.
///
/// A rung is *two* things, and only one of them is the writer's: the `name` and the order
/// are theirs, while `category` picks from a closed app-owned set that owns the glyph and
/// the per-theme colour. That is why there is no `color` here as there is on [`Tag`] — a
/// status colour cannot be stored data and still clear WCAG against both themes.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Status {
    pub name: String,
    /// One of `Planned` / `Drafting` / `NeedsWork` / `Revised` / `Final`. Spelled exactly
    /// as the enum, and a typo is a parse error rather than a silent default.
    pub category: common::entities::StatusCategory,
    #[serde(default)]
    pub details: String,
}

/// Where the story bible goes. An empty `binder` skips it entirely.
#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Bible {
    /// Binder name, created if absent and reused if present.
    #[serde(default)]
    pub binder: String,
    /// Optional `Folder/Note` to group each kind under. Empty means "flat in the
    /// binder", which is what the Starforgers bible has always looked like.
    #[serde(default)]
    pub character_folder: String,
    #[serde(default)]
    pub place_folder: String,
    #[serde(default)]
    pub object_folder: String,
    /// Tag applied to every entry of that kind, on top of the entry's own `tags`.
    #[serde(default)]
    pub character_tag: String,
    #[serde(default)]
    pub place_tag: String,
    #[serde(default)]
    pub object_tag: String,
}

/// One story-bible entry: a person, a place or a thing.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Entry {
    /// The note's title, and the primary name the mention scan matches.
    pub name: String,
    /// The other names the prose uses. Also how a `[[chapter]]` may refer to it.
    #[serde(default)]
    pub aliases: Vec<String>,
    /// A short factual note (Markdown). Original wording — never lifted from the book.
    #[serde(default)]
    pub note: String,
    /// Extra tags by name, on top of the kind's own.
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Editorial metadata for one writing row.
///
/// Addressed by exactly one of `number` (the *n*th prose row of the manuscript, in
/// stream order — what `convert` produces) or `file_id` (an existing row's id — how
/// a bundle this tool did not build, like Starforgers, is addressed).
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Chapter {
    #[serde(default)]
    pub number: Option<u32>,
    #[serde(default)]
    pub file_id: Option<u64>,
    /// The chapter's title. `convert` writes it and checks it against the heading
    /// actually found in the source; `enrich` ignores it.
    #[serde(default)]
    pub title: String,
    /// Markdown.
    #[serde(default)]
    pub synopsis: String,
    /// Point-of-view characters, by `name` or alias.
    #[serde(default)]
    pub pov: Vec<String>,
    /// Story-bible entries this chapter names, pinned as `References` — the same
    /// relationship the Inspector's Cast section pins by hand.
    #[serde(default)]
    pub cast: Vec<String>,
    #[serde(default)]
    pub places: Vec<String>,
    #[serde(default)]
    pub objects: Vec<String>,
    /// Tags by name.
    #[serde(default)]
    pub tags: Vec<String>,
    /// The coloured binder label.
    #[serde(default)]
    pub label: String,
    /// This chapter's rung, **by name**, matching one of the `[[status]]` entries. Empty
    /// means "no status", which is a real state and the default.
    ///
    /// A name with no matching rung is a hard error at build time, not a silent drop — the
    /// same discipline the story-bible cast lists already follow, and for the same reason:
    /// a typo that quietly does nothing is discovered by a reader, months later, as an
    /// absence.
    #[serde(default)]
    pub status: String,
}

impl Spec {
    /// Parse, then check what serde cannot: that the cross-references between
    /// sections actually resolve.
    ///
    /// Every one of these was a real failure of the JSON-driven predecessor, which
    /// reported its misses only as a lower count in its own summary line — a number
    /// nobody compares against anything.
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let text =
            std::fs::read_to_string(path).map_err(|e| anyhow::anyhow!("reading {path}: {e}"))?;
        let spec: Spec =
            toml::from_str(&text).map_err(|e| anyhow::anyhow!("parsing {path}: {e}"))?;
        spec.validate()?;
        Ok(spec)
    }

    fn validate(&self) -> anyhow::Result<()> {
        let mut names: Vec<&str> = Vec::new();
        for e in self.entries() {
            names.push(&e.name);
            names.extend(e.aliases.iter().map(String::as_str));
        }
        let mut seen = std::collections::BTreeSet::new();
        for n in &names {
            anyhow::ensure!(
                seen.insert(*n),
                "story-bible name or alias {n:?} is used twice — a chapter naming it \
                 could not say which entry it meant"
            );
        }
        let tags: std::collections::BTreeSet<&str> =
            self.tag.iter().map(|t| t.name.as_str()).collect();
        let check_tag = |t: &str, whose: &str| -> anyhow::Result<()> {
            anyhow::ensure!(
                t.is_empty() || tags.contains(t),
                "{whose} names tag {t:?}, which no [[tag]] declares"
            );
            Ok(())
        };
        check_tag(&self.bible.character_tag, "[bible].character_tag")?;
        check_tag(&self.bible.place_tag, "[bible].place_tag")?;
        check_tag(&self.bible.object_tag, "[bible].object_tag")?;
        for e in self.entries() {
            for t in &e.tags {
                check_tag(t, &format!("[[…]] {:?}", e.name))?;
            }
        }
        for c in &self.chapter {
            anyhow::ensure!(
                c.number.is_some() ^ c.file_id.is_some(),
                "[[chapter]] {:?} must set exactly one of `number` or `file_id`",
                c.title
            );
            for t in &c.tags {
                check_tag(t, &format!("[[chapter]] {:?}", c.title))?;
            }
            for n in c
                .pov
                .iter()
                .chain(&c.cast)
                .chain(&c.places)
                .chain(&c.objects)
            {
                anyhow::ensure!(
                    names.contains(&n.as_str()),
                    "[[chapter]] {:?} names {n:?}, which is not a story-bible entry \
                     or alias",
                    c.title
                );
            }
        }
        for p in &self.paratext {
            anyhow::ensure!(
                p.section == "front" || p.section == "back",
                "[[paratext]] {:?}: section must be \"front\" or \"back\", not {:?}",
                p.title,
                p.section
            );
        }
        Ok(())
    }

    /// Every story-bible entry, in the order the bible is built.
    pub fn entries(&self) -> impl Iterator<Item = &Entry> {
        self.character.iter().chain(&self.place).chain(&self.object)
    }
}

fn yes() -> bool {
    true
}

/// A deterministic `uid`, derived from the project identity plus a stable key.
///
/// Fixtures must not call `common::uid::new_uid`: a random v4 per run would give
/// every row a new identity on every regeneration, so the bundle would differ
/// byte-for-byte each time — unreviewable in a diff — and any reader who had the
/// example open would find every remembered tab, bookmark and expand state pointing
/// at rows that no longer exist.
///
/// Stamped as UUID version 8 (RFC 9562's "custom" version), which is what a
/// name-derived-but-not-v3/v5 value honestly is.
pub fn uid_for(project: &str, kind: &str, key: &str) -> Uuid {
    let mut hasher = blake3::Hasher::new();
    for part in [project, kind, key] {
        hasher.update(part.as_bytes());
        hasher.update(b"\x00");
    }
    let mut bytes = [0u8; 16];
    bytes.copy_from_slice(&hasher.finalize().as_bytes()[..16]);
    bytes[6] = (bytes[6] & 0x0f) | 0x80;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    Uuid::from_bytes(bytes)
}
