// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The capture menu's own arithmetic: which tags "Add as note" offers, in what order.
//!
//! **One gesture, one tag.** Picking a tag is unavoidable anyway, because a tag flagged
//! discoverable is what puts an entry into the mention index at all: without one it
//! matches nothing, appears in no roster, and contributes nothing. So the menu makes
//! that single choice answer three questions at once, and asks nothing else. The tag
//! carries where the note is filed (`BinderTag::creates_in`) and what shape it starts in
//! (`BinderTag::note_template`), both set on the Tags page or answered once on first use.
//!
//! Exactly one tag, never several: two tags could name two destinations, and the whole
//! point of the gesture is that the destination is never in doubt. A note that is
//! genuinely two things gets its second tag in the Details segment a moment later.
//!
//! ## Three tiers, and why each exists
//!
//! Discoverable tags first: they are the story-bible ones, and the common case. Then the
//! few other tags this writer actually reaches for, which is only knowable by watching,
//! so it comes from [`crate::models::NoteCaptureService`] rather than from the palette.
//! Then one **All tags** submenu holding everything, discoverable first.
//!
//! One overflow, not two. An earlier shape had the discoverable remainder and the
//! non-discoverable remainder in separate submenus, which meant a writer hunting a tag
//! they could not see had to know which of the two held it. Same information, one place
//! to look.
//!
//! **Untagged always sits last, and is always present.** A writer capturing a stray
//! thought has not decided they are building a story bible, and a menu that offers no
//! way through without picking one got the moment wrong. It is also what makes a
//! brand-new project work with no special case: a project with no tags shows a menu with
//! Untagged in it and nothing else.

use uuid::Uuid;

use crate::models::TagRow;

/// One tag, as the capture menu offers it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CaptureTag {
    pub id: u64,
    pub uid: Uuid,
    pub name: String,
    /// `#rrggbb`, for the row's colour dot.
    pub color: String,
    pub discoverable: bool,
}

impl From<&TagRow> for CaptureTag {
    fn from(r: &TagRow) -> Self {
        Self {
            id: r.id,
            uid: r.uid,
            name: r.name.clone(),
            color: r.color.clone(),
            discoverable: r.discoverable,
        }
    }
}

/// What the submenu renders, already ordered.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CaptureMenu {
    /// Discoverable tags, most recently used first, capped.
    pub primary: Vec<CaptureTag>,
    /// Other tags in recent use, capped. Empty for a project that has none.
    pub recent: Vec<CaptureTag>,
    /// Every tag, discoverable first, each group in palette order. The overflow.
    pub all: Vec<CaptureTag>,
}

impl CaptureMenu {
    /// Whether "All tags" would say anything the tiers above have not already said. A
    /// project with four tags shows all four inline; a submenu repeating them is chrome
    /// pretending to be a choice.
    pub fn needs_overflow(&self) -> bool {
        self.all.len() > self.primary.len() + self.recent.len()
    }
}

/// Order the palette into the three tiers.
///
/// `recents` is most-recently-used first, as
/// [`crate::models::NoteCaptureService::recent_tags`] returns it, and may hold uids that
/// no longer name anything: a tag deleted since it was last used is skipped here rather
/// than pruned there, because the palette is the authority on what exists and that file
/// never sees a delete.
///
/// Pure: no store, no widgets, so every rule above is testable without a project.
pub fn build_menu(rows: &[TagRow], recents: &[Uuid], cap: usize) -> CaptureMenu {
    let rank = |uid: &Uuid| recents.iter().position(|r| r == uid);

    // Palette order is already alphabetical and deterministic (`sort_rows`), so a tag
    // nobody has used yet keeps a stable place rather than moving between builds.
    let mut discoverable: Vec<CaptureTag> = rows
        .iter()
        .filter(|r| r.discoverable)
        .map(Into::into)
        .collect();
    let mut others: Vec<CaptureTag> = rows
        .iter()
        .filter(|r| !r.discoverable)
        .map(Into::into)
        .collect();

    // Used tags float in use order; the rest keep palette order behind them.
    let float = |v: &mut Vec<CaptureTag>| {
        v.sort_by_key(|t| (rank(&t.uid).is_none(), rank(&t.uid).unwrap_or(usize::MAX)));
    };
    float(&mut discoverable);
    float(&mut others);

    let primary: Vec<CaptureTag> = discoverable.iter().take(cap).cloned().collect();
    // Only tags actually in use reach the second tier. An unused one has nothing to say
    // for itself here and waits in the overflow, one hop away.
    let recent: Vec<CaptureTag> = others
        .iter()
        .filter(|t| rank(&t.uid).is_some())
        .take(cap)
        .cloned()
        .collect();

    let mut all = discoverable;
    all.extend(others);

    CaptureMenu {
        primary,
        recent,
        all,
    }
}

/// One row of the rendered submenu, in order.
///
/// The tiers above say *which* tags go where; this says what the writer actually sees,
/// including the dividers. Separately, because a divider is not a tag and the rule about
/// it is easy to get wrong in a way nothing shows: a separator is content-free, so it is
/// pruned from the accessibility tree and a stray one is invisible to every test that
/// reads a menu the way the rest of this crate does.
#[derive(Debug, Clone, PartialEq)]
pub enum CaptureEntry {
    /// File under this tag.
    Tag(CaptureTag),
    /// Open the full palette, discoverable first.
    AllTags,
    /// File under no tag at all. Always present, always last.
    Untagged,
    /// A rule between two groups. Never first, never last, never doubled.
    Separator,
}

impl CaptureMenu {
    /// Every row of the submenu, in the order it renders.
    ///
    /// **A separator divides two groups.** Every tier here is optional, so each one's
    /// divider has to ask whether anything precedes it: a project whose tags are all
    /// undiscoverable has an empty first tier, and a surface handed no palette at all has
    /// every tier empty but still shows Untagged. Emitting the dividers unconditionally
    /// draws a rule across the top of the menu, which is what this exists to prevent.
    pub fn entries(&self) -> Vec<CaptureEntry> {
        let mut out: Vec<CaptureEntry> = Vec::new();
        let group = |out: &mut Vec<CaptureEntry>, rows: Vec<CaptureEntry>| {
            if rows.is_empty() {
                return;
            }
            if !out.is_empty() {
                out.push(CaptureEntry::Separator);
            }
            out.extend(rows);
        };
        group(
            &mut out,
            self.primary
                .iter()
                .cloned()
                .map(CaptureEntry::Tag)
                .collect(),
        );
        group(
            &mut out,
            self.recent.iter().cloned().map(CaptureEntry::Tag).collect(),
        );
        if self.needs_overflow() {
            group(&mut out, vec![CaptureEntry::AllTags]);
        }
        group(&mut out, vec![CaptureEntry::Untagged]);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tag(id: u64, name: &str, discoverable: bool) -> TagRow {
        TagRow {
            id,
            uid: Uuid::from_u128(id as u128),
            name: name.to_string(),
            color: "#2e7d32".to_string(),
            details: String::new(),
            discoverable,
            creates_in: None,
            note_template: None,
        }
    }

    fn uid(n: u64) -> Uuid {
        Uuid::from_u128(n as u128)
    }

    /// **A divider divides.** With no palette at all the whole submenu is Untagged, so
    /// there is nothing to divide and no rule to draw. This is the shape every surface
    /// built without a tab around it passes, and the corkboard card passes it in a real,
    /// shipping project.
    #[test]
    fn an_empty_menu_is_untagged_alone_with_no_divider() {
        assert_eq!(
            CaptureMenu::default().entries(),
            vec![CaptureEntry::Untagged],
            "one row, no rule above it"
        );
    }

    /// A project whose tags are **all undiscoverable**: the first tier is empty, so the
    /// recents tier must not lead with a divider.
    #[test]
    fn an_empty_first_tier_does_not_leave_a_divider_at_the_top() {
        let rows = vec![tag(1, "research", false)];
        let entries = build_menu(&rows, &[uid(1)], 5).entries();
        assert_eq!(
            entries.first(),
            Some(&CaptureEntry::Tag(CaptureTag {
                id: 1,
                uid: uid(1),
                name: "research".to_string(),
                color: "#2e7d32".to_string(),
                discoverable: false,
            })),
            "the menu opens on a tag, not on a rule: {entries:?}"
        );
        assert_eq!(entries.last(), Some(&CaptureEntry::Untagged));
    }

    /// Every tier present: a rule between each pair and nowhere else.
    #[test]
    fn dividers_fall_between_the_tiers_and_nowhere_else() {
        let rows = vec![
            tag(1, "character", true),
            tag(2, "research", false),
            tag(3, "timeline", false),
            tag(4, "workflow", false),
        ];
        // `research` used, so it floats into tier two; the rest only reachable under All.
        let entries = build_menu(&rows, &[uid(2)], 5).entries();
        assert_ne!(entries.first(), Some(&CaptureEntry::Separator));
        assert_ne!(entries.last(), Some(&CaptureEntry::Separator));
        assert!(
            !entries
                .windows(2)
                .any(|w| w == [CaptureEntry::Separator, CaptureEntry::Separator]),
            "never two rules in a row: {entries:?}"
        );
        assert_eq!(
            entries
                .iter()
                .filter(|e| **e == CaptureEntry::Separator)
                .count(),
            3,
            "discoverable | recent | All tags | Untagged: {entries:?}"
        );
    }

    /// Untagged is last, always, whatever the tiers did.
    #[test]
    fn untagged_is_always_the_last_row() {
        for rows in [
            Vec::new(),
            vec![tag(1, "character", true)],
            vec![tag(1, "character", true), tag(2, "research", false)],
        ] {
            let entries = build_menu(&rows, &[], 5).entries();
            assert_eq!(entries.last(), Some(&CaptureEntry::Untagged), "{entries:?}");
        }
    }

    /// The palette a fresh project gets from the Basic preset: three discoverable tags
    /// and nothing else. Everything fits inline, so there is nothing to overflow.
    #[test]
    fn a_basic_palette_is_entirely_first_class() {
        let rows = vec![
            tag(1, "character", true),
            tag(2, "item", true),
            tag(3, "place", true),
        ];
        let m = build_menu(&rows, &[], 5);
        assert_eq!(m.primary.len(), 3);
        assert!(m.recent.is_empty());
        assert!(!m.needs_overflow(), "nothing is hidden, so no submenu");
    }

    /// A project with no tags at all still yields a menu: empty tiers, and the caller
    /// renders Untagged alone. Nothing here special-cases it.
    #[test]
    fn a_tagless_project_yields_empty_tiers_rather_than_an_error() {
        let m = build_menu(&[], &[], 5);
        assert!(m.primary.is_empty() && m.recent.is_empty() && m.all.is_empty());
        assert!(!m.needs_overflow());
    }

    /// Use order beats palette order inside the first tier, so the tag a writer keeps
    /// reaching for rises to the top rather than staying wherever the alphabet put it.
    #[test]
    fn recently_used_discoverable_tags_float_to_the_front() {
        let rows = vec![
            tag(1, "character", true),
            tag(2, "item", true),
            tag(3, "place", true),
        ];
        let m = build_menu(&rows, &[uid(3)], 5);
        assert_eq!(m.primary[0].name, "place");
    }

    /// The second tier is *used* other tags only. An unused one has nothing to say for
    /// itself there and waits in the overflow.
    #[test]
    fn an_unused_other_tag_stays_out_of_the_second_tier() {
        let rows = vec![
            tag(1, "character", true),
            tag(2, "research", false),
            tag(3, "worldbuilding", false),
        ];
        let m = build_menu(&rows, &[uid(2)], 5);
        assert_eq!(m.recent.len(), 1);
        assert_eq!(m.recent[0].name, "research");
        assert!(m.needs_overflow(), "worldbuilding is reachable, not lost");
    }

    /// Both tiers are capped, and everything past the cap is still in "All tags". The
    /// menu is a shortlist, never a filter.
    #[test]
    fn the_cap_shortens_the_tiers_without_hiding_anything() {
        let rows: Vec<TagRow> = (1..=9).map(|i| tag(i, &format!("t{i}"), i <= 6)).collect();
        let used: Vec<Uuid> = (7..=9).map(uid).collect();
        let m = build_menu(&rows, &used, 2);
        assert_eq!(m.primary.len(), 2, "first tier capped");
        assert_eq!(m.recent.len(), 2, "second tier capped");
        assert_eq!(m.all.len(), 9, "every tag is still reachable");
        assert!(m.needs_overflow());
    }

    /// A uid left over from a tag the writer has since deleted names nothing. It must be
    /// skipped rather than surfacing an empty row, and it must not disturb the ordering
    /// of the tags that do still exist.
    #[test]
    fn a_recent_uid_for_a_deleted_tag_is_simply_skipped() {
        let rows = vec![tag(1, "character", true), tag(2, "place", true)];
        let m = build_menu(&rows, &[uid(99), uid(2)], 5);
        assert_eq!(m.primary.len(), 2);
        assert_eq!(m.primary[0].name, "place", "the live recent still floats");
    }

    /// Discoverable first in the overflow too, so one list reads in the same order the
    /// tiers above established rather than reshuffling into the alphabet.
    #[test]
    fn all_tags_lists_discoverable_before_the_rest() {
        let rows = vec![tag(1, "research", false), tag(2, "character", true)];
        let m = build_menu(&rows, &[], 5);
        assert!(m.all[0].discoverable);
        assert!(!m.all[1].discoverable);
    }
}
