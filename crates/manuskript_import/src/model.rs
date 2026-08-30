// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The intermediate, in-memory Manuskript model produced by the readers and
//! consumed by the mapper.
//!
//! It is **version-neutral**. The format-1 readers and the format-0 readers both
//! normalise into these shapes — the `summarySentance` misspelling of 2016, the
//! `persos`/`subplots` plot attributes, the `txt`/`t2t`/`html` item types, the flat
//! `perso.xml` character table — so the mapper only ever sees one shape and has no
//! idea which generation it came from.
//!
//! What it deliberately does **not** do is resolve anything. A label is still the
//! integer Manuskript stored, a POV is still a character id, `compile` is still the
//! raw check state, and none of the computed fields are here at all. Resolution is
//! the mapper's job, in one place, where the vocabularies it resolves against are
//! also being built.

/// A whole project, however it was stored.
#[derive(Debug, Clone, Default)]
pub struct Project {
    pub info: Info,
    /// The project's own name, from the folder or the `.msk` stem.
    ///
    /// The title fallback for a project whose `infos.txt` carries none, which is
    /// every project the writer never filled that field in for. Without it such a
    /// project arrives called "Imported Manuskript project", having had a perfectly
    /// good name on disk all along.
    pub source_name: String,
    pub summary: Summary,
    /// The label vocabulary, in file order. **Referenced 1-based**: an item's
    /// `label: 3` is `labels[2]`, because Manuskript seeds its model with an empty
    /// first row meaning "no label". See [`Project::label_at`].
    pub labels: Vec<Label>,
    /// The status vocabulary, same 1-based indexing as [`Self::labels`].
    pub statuses: Vec<String>,
    /// The outline's top-level items, in order.
    pub outline: Vec<OutlineItem>,
    pub characters: Vec<Character>,
    pub world: Vec<WorldItem>,
    pub plots: Vec<Plot>,
    pub settings: Settings,
    /// Every recorded revision, flat, each naming the item it belongs to.
    pub revisions: Vec<Revision>,
    /// What the readers could not account for, in the writer's language.
    pub notices: Vec<String>,
}

impl Project {
    /// The label an item's stored index names, or `None` for "no label".
    ///
    /// Index 0 is Manuskript's "none" row and never reaches a file: `outlineToMMD`
    /// omits a falsy value, so absent and `0` are the same state. An index past
    /// the end of the vocabulary is also `None` — that is a project whose
    /// `labels.txt` was edited out from under its items, and inventing a label for
    /// it would be worse than leaving the row unmarked.
    pub fn label_at(&self, index: usize) -> Option<&Label> {
        index.checked_sub(1).and_then(|i| self.labels.get(i))
    }

    /// The status name an item's stored index names, on the same 1-based rule.
    pub fn status_at(&self, index: usize) -> Option<&str> {
        index
            .checked_sub(1)
            .and_then(|i| self.statuses.get(i))
            .map(String::as_str)
    }
}

/// `infos.txt`. Every field is optional; the writer omits an empty one.
#[derive(Debug, Clone, Default)]
pub struct Info {
    pub title: String,
    pub subtitle: String,
    /// Manuskript's own spelling, which has never been corrected: the on-disk key
    /// is `Serie`, not `Series`.
    pub serie: String,
    pub volume: String,
    pub genre: String,
    pub license: String,
    pub author: String,
    pub email: String,
}

/// `summary.txt` — the snowflake ladder, each rung a longer telling of the one
/// before it.
#[derive(Debug, Clone, Default)]
pub struct Summary {
    pub situation: String,
    pub sentence: String,
    pub paragraph: String,
    pub page: String,
    pub full: String,
}

impl Summary {
    pub fn is_empty(&self) -> bool {
        [
            &self.situation,
            &self.sentence,
            &self.paragraph,
            &self.page,
            &self.full,
        ]
        .iter()
        .all(|s| s.trim().is_empty())
    }

    /// The rungs in ladder order, paired with their on-disk key.
    pub fn rungs(&self) -> [(&'static str, &str); 5] {
        [
            ("Situation", self.situation.as_str()),
            ("Sentence", self.sentence.as_str()),
            ("Paragraph", self.paragraph.as_str()),
            ("Page", self.page.as_str()),
            ("Full", self.full.as_str()),
        ]
    }
}

/// One row of `labels.txt`: a name and, usually, a colour.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Label {
    pub name: String,
    /// `#rrggbb` in format 1, `#aarrggbb` in format 0. `None` when the row carried
    /// none, which `status.txt` rows never do and `labels.txt` rows should not.
    pub color: Option<String>,
}

/// What an outline row is. Manuskript has exactly these two and no more — no
/// chapter, no scene, no part.
///
/// `Text` is the default because that is what a row is when nothing says
/// otherwise: a folder is a folder by holding a `folder.txt`, which is a positive
/// statement the reader has to find.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutlineKind {
    /// A directory holding `folder.txt` and its children.
    Folder,
    /// A single `.md` file. Always a leaf.
    #[default]
    Text,
}

/// One outline row, with its children.
#[derive(Debug, Clone, Default)]
pub struct OutlineItem {
    /// Manuskript's `ID`. A join key only: POV, plot links and revisions all
    /// address rows by it, and it is re-minted by Manuskript itself on collision,
    /// so nothing durable may be built on it.
    pub id: Option<String>,
    pub title: String,
    pub kind: OutlineKindOpt,
    pub summary_sentence: String,
    pub summary_full: String,
    /// A `Character::id`, not a name.
    pub pov: Option<String>,
    pub notes: String,
    /// The stored 1-based index into `labels.txt`; `None` for no label.
    pub label: Option<usize>,
    /// The stored 1-based index into `status.txt`; `None` for no status.
    pub status: Option<usize>,
    /// The raw check state. `0` excludes; anything else includes, subject to
    /// ancestors — resolving that inheritance is [`OutlineItem::compiles`]'s job,
    /// not this field's.
    pub compile: Option<i64>,
    pub set_goal: Option<i64>,
    /// A freedesktop icon-theme name. Kept so the mapper can report it rather than
    /// drop it silently; Skribisto has no per-row icon.
    pub custom_icon: String,
    /// The body, **as Djot**, converted by the reader that produced it.
    ///
    /// Djot and not the source's own markup because only the reader knows which
    /// markup that was: the item's `type` says `md` on a modern project and
    /// `html` on a pre-0.3.0 one, and that fact does not survive into this model.
    /// See [`crate::prose`], which is the only place the conversion happens.
    ///
    /// Empty for a folder, which cannot hold prose.
    pub text: String,
    pub children: Vec<OutlineItem>,
}

/// [`OutlineKind`], defaulting to `Text`.
///
/// A newtype rather than `Option`, because "we could not tell" is not a state the
/// mapper should have to handle: a row read from a `folder.txt` is a folder and a
/// row read from a `.md` is text, whatever the `type:` key says or fails to say.
pub type OutlineKindOpt = OutlineKind;

impl OutlineItem {
    pub fn is_folder(&self) -> bool {
        self.kind == OutlineKind::Folder
    }

    /// Whether this row reaches the compiled book, given its ancestors.
    ///
    /// `inherited` is the answer for its parent. Manuskript walks up on every
    /// query (`outlineItem.py::compile`): a row is excluded when its own check
    /// state is `0` **or** when any ancestor's is. Flattening the stored field to
    /// a boolean without this walk exports scenes the writer switched off.
    pub fn compiles(&self, inherited: bool) -> bool {
        inherited && self.compile != Some(0)
    }

    /// Every row of this subtree, depth-first, paired with the resolved compile
    /// flag it inherits.
    pub fn walk<'a>(&'a self, inherited: bool, out: &mut Vec<(&'a OutlineItem, bool)>) {
        let mine = self.compiles(inherited);
        out.push((self, mine));
        for child in &self.children {
            child.walk(mine, out);
        }
    }
}

/// A character. Field names here are the enum's; the on-disk keys are the
/// human-readable labels in `characterMap` and are mapped in the reader.
#[derive(Debug, Clone, Default)]
pub struct Character {
    pub id: Option<String>,
    pub name: String,
    /// `0` minor, `1` secondary, `2` main.
    pub importance: Option<u8>,
    /// Whether Manuskript offers this character in its POV picker. Added in
    /// 0.12.0, so absent in an older project, which is not the same as `false`.
    pub pov_enabled: Option<bool>,
    /// `#rrggbb`, the swatch beside the name. The **first** `Color` in the file:
    /// any later one is a user field, and the reader keeps that distinction.
    pub color: String,
    pub motivation: String,
    pub goal: String,
    pub conflict: String,
    pub epiphany: String,
    pub summary_sentence: String,
    pub summary_paragraph: String,
    pub summary_full: String,
    pub notes: String,
    /// The writer's own fields, in file order: every key that is not one of the
    /// built-ins and not the swatch.
    pub infos: Vec<(String, String)>,
}

/// One node of the worldbuilding tree.
#[derive(Debug, Clone, Default)]
pub struct WorldItem {
    pub id: Option<String>,
    pub name: String,
    pub description: String,
    /// Manuskript's own prompt: what the reader might love about it.
    pub passion: String,
    /// And what conflict it may cause.
    pub conflict: String,
    pub children: Vec<WorldItem>,
}

/// A plot, with its resolution steps.
#[derive(Debug, Clone, Default)]
pub struct Plot {
    pub id: Option<String>,
    pub name: String,
    /// `0` minor, `1` secondary, `2` main — the same scale as a character's.
    pub importance: Option<u8>,
    /// `Character::id`s, from the comma-joined `characters` attribute.
    pub characters: Vec<String>,
    pub description: String,
    pub result: String,
    pub summary: String,
    pub steps: Vec<PlotStep>,
}

/// One beat of a plot.
#[derive(Debug, Clone, Default)]
pub struct PlotStep {
    pub id: Option<String>,
    pub name: String,
    /// Free text; Manuskript's own field name for it is `meta`.
    pub meta: String,
    pub summary: String,
}

/// One recorded past state of one outline row's prose.
#[derive(Debug, Clone)]
pub struct Revision {
    /// The `OutlineItem::id` this belongs to.
    pub item_id: String,
    /// Unix seconds, as Manuskript stores it.
    pub timestamp: i64,
    /// The whole body at that moment, as Markdown. Snapshots, not diffs.
    pub text: String,
}

/// The handful of `settings.txt` keys that describe the project rather than the
/// window.
#[derive(Debug, Clone, Default)]
pub struct Settings {
    /// The spellcheck locale, e.g. `en_US`. The only signal in the project of what
    /// language it is written in.
    pub dict: Option<String>,
    /// Whether the project was keeping revisions. Informational: `revisions.xml`
    /// is read when it is there, whatever this says.
    pub revisions_keep: Option<bool>,
}

/// Normalise a stored colour to `#rrggbb`.
///
/// Format 1 writes six digits and format 0 writes eight, alpha first. Skribisto's
/// tag colours have no alpha, so the alpha channel is dropped rather than
/// misread as red.
pub fn normalise_color(raw: &str) -> Option<String> {
    let hex = raw.strip_prefix('#')?;
    if !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    match hex.len() {
        6 => Some(format!("#{}", hex.to_ascii_lowercase())),
        8 => Some(format!("#{}", hex[2..].to_ascii_lowercase())),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(compile: Option<i64>, children: Vec<OutlineItem>) -> OutlineItem {
        OutlineItem {
            compile,
            children,
            ..OutlineItem::default()
        }
    }

    #[test]
    fn a_six_digit_colour_is_kept_and_an_eight_digit_one_loses_its_alpha() {
        assert_eq!(normalise_color("#ff0000").as_deref(), Some("#ff0000"));
        assert_eq!(normalise_color("#FF0000").as_deref(), Some("#ff0000"));
        // Format 0 wrote #aarrggbb; the alpha must not be read as red.
        assert_eq!(normalise_color("#ffff0000").as_deref(), Some("#ff0000"));
        assert_eq!(normalise_color("#00123456").as_deref(), Some("#123456"));
    }

    #[test]
    fn a_colour_that_is_not_one_is_no_colour() {
        assert!(normalise_color("").is_none());
        assert!(normalise_color("red").is_none());
        assert!(normalise_color("#12345").is_none());
        assert!(normalise_color("#zzzzzz").is_none());
    }

    #[test]
    fn a_vocabulary_is_indexed_one_based_with_zero_meaning_none() {
        let p = Project {
            labels: vec![
                Label {
                    name: "Idea".into(),
                    color: Some("#ffff00".into()),
                },
                Label {
                    name: "Chapter".into(),
                    color: Some("#0000ff".into()),
                },
            ],
            statuses: vec!["TODO".into(), "First draft".into()],
            ..Project::default()
        };
        assert_eq!(p.label_at(1).map(|l| l.name.as_str()), Some("Idea"));
        assert_eq!(p.label_at(2).map(|l| l.name.as_str()), Some("Chapter"));
        assert_eq!(p.status_at(2), Some("First draft"));
        // Manuskript's "none" row, which never reaches a file.
        assert!(p.label_at(0).is_none());
        assert!(p.status_at(0).is_none());
        // Past the end: a vocabulary edited out from under its items.
        assert!(p.label_at(3).is_none());
        assert!(p.status_at(99).is_none());
    }

    #[test]
    fn compile_is_inherited_so_a_switched_off_folder_excludes_its_children() {
        let tree = item(Some(0), vec![item(Some(2), vec![item(Some(2), vec![])])]);
        let mut out = Vec::new();
        tree.walk(true, &mut out);
        assert_eq!(out.len(), 3);
        assert!(
            out.iter().all(|(_, compiles)| !compiles),
            "an excluded ancestor excludes all"
        );
    }

    #[test]
    fn an_included_ancestor_leaves_each_child_to_its_own_state() {
        let tree = item(Some(2), vec![item(Some(0), vec![]), item(Some(2), vec![])]);
        let mut out = Vec::new();
        tree.walk(true, &mut out);
        assert_eq!(
            out.iter().map(|(_, c)| *c).collect::<Vec<_>>(),
            [true, false, true]
        );
    }

    /// `compile` is a check state, not a bool: `1` is Qt's partially-checked and
    /// only `0` means excluded.
    #[test]
    fn only_zero_excludes() {
        assert!(item(Some(2), vec![]).compiles(true));
        assert!(item(Some(1), vec![]).compiles(true));
        assert!(item(None, vec![]).compiles(true));
        assert!(!item(Some(0), vec![]).compiles(true));
    }

    #[test]
    fn a_summary_knows_when_it_has_nothing_to_say() {
        assert!(Summary::default().is_empty());
        let s = Summary {
            page: "  ".into(),
            ..Summary::default()
        };
        assert!(s.is_empty(), "whitespace is not content");
        let s = Summary {
            full: "A book.".into(),
            ..Summary::default()
        };
        assert!(!s.is_empty());
        assert_eq!(s.rungs()[4], ("Full", "A book."));
    }
}
