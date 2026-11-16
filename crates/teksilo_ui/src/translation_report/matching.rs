// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Finding the string a writer saw, from the words they type.
//!
//! The matcher is `text_document`'s, the one Search & Replace and the mention scan
//! use: case and diacritic folding (a French writer on a US keyboard finds
//! "éléments" by typing "elements") and Turkish dotted i, per language. What this
//! module adds is what a *catalogue* needs on top of it:
//!
//! - **Ranking** ([`Rank`]): the whole value first, then every word in order, then
//!   every word, then a partial match.
//! - **Variables.** `{ $count } éléments` is found by "3 éléments": a variable accepts
//!   whatever the writer typed in its position.
//! - **Composition.** The Edit menu shows "Annuler la mise à la corbeille", which is
//!   `menu-edit-undo-target = A&nnuler { $target }` filled with
//!   `undo-target-trash = la mise à la corbeille`. When the text a variable accepted is
//!   itself another message, the hit names both ([`Hit::composed`]): an agreement error
//!   across two messages can be fixed in either.
//! - **Grouping** ([`group`]): fr-FR has 23 keys reading "Annuler". They are one row,
//!   with the English sources they translate, so the writer can report one place or
//!   all of them.
//! - **English as a second key** ([`english_hits`]): a writer may type the English
//!   original; the result then says what the translation reads, or that there is none.
//!
//! **Cost.** One locale is about 3,900 strings across the five catalogues. Building
//! its index (parsing and folding all of them) takes about 0.1 s in a debug build, and
//! one search about 13 ms, measured on fr-FR on 2026-11-16. Both run on the UI thread:
//! the index once per language the window is pointed at, the search per keystroke.

use std::collections::HashMap;

use teksilo::text_document::matching::{FoldLocale, FoldSpec, FoldedText};

use super::catalogue::{Area, Catalogue, CatalogueEntry, SOURCE_LOCALE, entries_for};
use super::display::{WILD, normalize_for_search};

/// How closely a hit matches the query, best first.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Rank {
    /// The query is the whole value (a variable accepting anything in its place).
    Whole,
    /// Every word of the query, in the query's order.
    InOrder,
    /// Every word of the query, in any order.
    AllWords,
    /// Every word of the query, at least as part of a word.
    Partial,
}

/// One message (or help paragraph) the query matched.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Hit {
    /// Index into [`LocaleIndex::entries`].
    pub entry: usize,
    pub rank: Rank,
    /// The message whose value filled one of this message's variables, when the query
    /// is the composition of the two.
    pub composed: Option<usize>,
}

/// Every string one locale ships, folded once for searching.
pub(crate) struct LocaleIndex {
    pub locale: &'static str,
    pub entries: Vec<CatalogueEntry>,
    /// One per form of each entry, in the same order.
    folded: Vec<Vec<FoldedText>>,
    /// `(family, id)` → entry, for finding a message's counterpart in another locale.
    by_key: HashMap<(Family, String), usize>,
    spec: FoldSpec,
}

/// Which catalogue an entry's English counterpart lives in. A framework string and
/// its override translate the same English message.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Family {
    App,
    Framework,
    Export,
}

fn family(catalogue: Catalogue) -> Option<Family> {
    match catalogue {
        Catalogue::App => Some(Family::App),
        Catalogue::Framework | Catalogue::WidgetOverride => Some(Family::Framework),
        Catalogue::Export => Some(Family::Export),
        // Paragraphs of two languages' pages are not aligned one to one.
        Catalogue::Help => None,
    }
}

/// How many hits a search returns at most, before grouping. A query that matches
/// more than this is not narrowing anything down yet.
pub(crate) const MAX_HITS: usize = 300;

impl LocaleIndex {
    /// The index of `locale`, or `None` for a locale the app does not ship.
    pub(crate) fn build(locale: &str) -> Option<Self> {
        let entries = entries_for(locale);
        let first = entries.first()?;
        let locale = first.locale;
        let spec = FoldSpec {
            case_sensitive: false,
            diacritic_sensitive: false,
            locale: FoldLocale::from_tag(locale),
        };
        let folded = entries
            .iter()
            .map(|e| {
                e.forms
                    .iter()
                    .map(|f| FoldedText::new(&f.search, &spec))
                    .collect()
            })
            .collect();
        let by_key = entries
            .iter()
            .enumerate()
            .filter_map(|(i, e)| Some(((family(e.catalogue)?, e.id.clone()), i)))
            .collect();
        Some(Self {
            locale,
            entries,
            folded,
            by_key,
            spec,
        })
    }

    /// The entry translating the same message as `other`, an entry of another
    /// locale's index.
    pub(crate) fn counterpart(&self, other: &CatalogueEntry) -> Option<usize> {
        let family = family(other.catalogue)?;
        self.by_key.get(&(family, other.id.clone())).copied()
    }

    /// The entry with this catalogue and id. A framework id also finds its override.
    #[cfg(test)]
    pub(crate) fn find(&self, catalogue: Catalogue, id: &str) -> Option<usize> {
        let family = family(catalogue)?;
        self.by_key.get(&(family, id.to_string())).copied()
    }

    /// Every entry `query` matches, best first, at most [`MAX_HITS`].
    pub(crate) fn search(&self, query: &str) -> Vec<Hit> {
        let Some(query) = Query::new(query, &self.spec) else {
            return Vec::new();
        };
        let mut hits: Vec<Hit> = (0..self.entries.len())
            .filter_map(|i| self.rank_entry(i, &query))
            .collect();
        hits.sort_by(|a, b| {
            let (ea, eb) = (&self.entries[a.entry], &self.entries[b.entry]);
            a.rank
                .cmp(&b.rank)
                .then(area_order(ea.area).cmp(&area_order(eb.area)))
                .then(ea.shown().chars().count().cmp(&eb.shown().chars().count()))
                .then(a.entry.cmp(&b.entry))
        });
        hits.truncate(MAX_HITS);
        hits
    }

    /// How entry `i` matches `query`: its best form.
    fn rank_entry(&self, i: usize, query: &Query) -> Option<Hit> {
        let mut best: Option<Hit> = None;
        for (f, folded) in self.folded[i].iter().enumerate() {
            let search = &self.entries[i].forms[f].search;
            let hit = if let Some(gaps) = whole_match(search, &query.folded, query.len) {
                Some(Hit {
                    entry: i,
                    rank: Rank::Whole,
                    composed: self.composition(i, &query.text, &gaps),
                })
            } else if query.whole_only {
                None
            } else {
                word_rank(folded, &query.words).map(|rank| Hit {
                    entry: i,
                    rank,
                    composed: None,
                })
            };
            if let Some(hit) = hit
                && best.is_none_or(|b| hit.rank < b.rank)
            {
                best = Some(hit);
                if hit.rank == Rank::Whole {
                    break;
                }
            }
        }
        best
    }

    /// The message whose value is exactly the text a variable of entry `i` accepted,
    /// if any: same catalogue, no variable of its own.
    fn composition(&self, i: usize, query: &str, gaps: &[(usize, usize)]) -> Option<usize> {
        let catalogue = self.entries[i].catalogue;
        let chars: Vec<char> = query.chars().collect();
        for &(start, end) in gaps {
            let gap: String = chars[start..end]
                .iter()
                .collect::<String>()
                .trim()
                .to_string();
            let gap_len = gap.chars().count();
            if gap_len == 0 {
                continue;
            }
            let gap_folded = FoldedText::new(&gap, &self.spec);
            let found = self.entries.iter().enumerate().find(|(j, e)| {
                *j != i
                    && e.catalogue == catalogue
                    && e.forms.iter().any(|f| {
                        !f.has_wild()
                            && (gap_len / 2..=gap_len * 2).contains(&f.search.chars().count())
                            && gap_folded
                                .find_all(&f.search, false)
                                .iter()
                                .any(|m| m.char_start == 0 && m.char_len == gap_len)
                    })
            });
            if let Some((j, _)) = found {
                return Some(j);
            }
        }
        None
    }
}

/// Interface strings first: they are what most reports are about.
fn area_order(area: Area) -> u8 {
    match area {
        Area::Interface => 0,
        Area::DefaultContent => 1,
        Area::Help => 2,
        Area::Export => 3,
    }
}

/// A query, prepared once per search.
struct Query {
    /// Normalised as the index's search strings are.
    text: String,
    folded: FoldedText,
    /// In chars.
    len: usize,
    words: Vec<String>,
    /// A single Latin letter is in nearly every string: only an exact match is worth
    /// showing until the writer types more.
    whole_only: bool,
}

impl Query {
    fn new(raw: &str, spec: &FoldSpec) -> Option<Self> {
        let text = normalize_for_search(raw);
        if text.is_empty() {
            return None;
        }
        let len = text.chars().count();
        Some(Self {
            folded: FoldedText::new(&text, spec),
            words: query_words(&text),
            whole_only: len < 2 && text.is_ascii(),
            len,
            text,
        })
    }
}

/// The query's words, with the punctuation at their edges removed.
fn query_words(query: &str) -> Vec<String> {
    query
        .split(' ')
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric() && c != WILD))
        .filter(|w| !w.is_empty())
        .map(str::to_string)
        .collect()
}

/// Whether the folded query `q` (of `q_len` chars) is the whole of `search`, each
/// [`WILD`] in it accepting at least one character. Returns the char ranges of the
/// query the wildcards accepted.
fn whole_match(search: &str, q: &FoldedText, q_len: usize) -> Option<Vec<(usize, usize)>> {
    let segments: Vec<&str> = search.split(WILD).collect();
    // A value with no word of its own (`{ $name }`, `{ $a }: { $b }`) would be the
    // whole of every query. It cannot be found by its text, so it is never a hit.
    if !segments
        .iter()
        .any(|s| s.chars().any(char::is_alphanumeric))
    {
        return None;
    }
    // Folding changes a length by a bounded factor (ß → ss, a ligature to two
    // letters), never by more than double: a value whose literal text is much longer
    // or, without a variable, much shorter than the query cannot be the whole of it.
    let literal: usize = segments.iter().map(|s| s.chars().count()).sum();
    if literal > q_len * 2 || (segments.len() == 1 && literal * 2 < q_len) {
        return None;
    }
    let last = segments.len() - 1;
    let mut gaps = Vec::new();

    if last == 0 {
        let full = q
            .find_all(search, false)
            .into_iter()
            .any(|m| m.char_start == 0 && m.char_len == q_len);
        return full.then_some(gaps);
    }

    // Where the text matched so far ends, and where the open wildcard began.
    let mut at = 0usize;
    for (i, segment) in segments.iter().enumerate() {
        if i == 0 {
            if !segment.is_empty() {
                let m = q
                    .find_all(segment, false)
                    .into_iter()
                    .find(|m| m.char_start == 0)?;
                at = m.char_len;
            }
            continue;
        }
        if segment.is_empty() {
            if i == last {
                // A trailing wildcard takes the rest, which must not be empty.
                if at >= q_len {
                    return None;
                }
                gaps.push((at, q_len));
                at = q_len;
            }
            // Two wildcards in a row are one.
            continue;
        }
        let m = if i == last {
            q.find_all(segment, false)
                .into_iter()
                .find(|m| m.char_start > at && m.char_start + m.char_len == q_len)?
        } else {
            q.find_all(segment, false)
                .into_iter()
                .find(|m| m.char_start > at)?
        };
        gaps.push((at, m.char_start));
        at = m.char_start + m.char_len;
    }
    (at == q_len).then_some(gaps)
}

/// How well `words` match a form's folded text, short of the whole value.
fn word_rank(folded: &FoldedText, words: &[String]) -> Option<Rank> {
    if words.is_empty() {
        return None;
    }
    let mut in_order = true;
    let mut all = true;
    let mut after = 0usize;
    for word in words {
        let found = folded.find_all(word, true);
        if found.is_empty() {
            all = false;
            break;
        }
        match found.iter().find(|m| m.char_start >= after) {
            Some(m) => after = m.char_start + m.char_len,
            None => in_order = false,
        }
    }
    if all {
        return Some(if in_order {
            Rank::InOrder
        } else {
            Rank::AllWords
        });
    }
    words
        .iter()
        .all(|w| !folded.find_all(w, false).is_empty())
        .then_some(Rank::Partial)
}

/// Hits reading the same text, shown as one row.
#[derive(Clone, Debug)]
pub(crate) struct ResultGroup {
    /// The text the row prints.
    pub shown: String,
    /// Best first.
    pub hits: Vec<Hit>,
}

/// `hits` with identical values gathered into one group each, in the order of their
/// best member. A composed hit stays alone: it is a pair, not a value.
pub(crate) fn group(index: &LocaleIndex, hits: &[Hit]) -> Vec<ResultGroup> {
    let mut groups: Vec<ResultGroup> = Vec::new();
    let mut slot: HashMap<String, usize> = HashMap::new();
    for hit in hits {
        let shown = index.entries[hit.entry].shown().to_string();
        if hit.composed.is_none()
            && let Some(&g) = slot.get(&shown)
        {
            groups[g].hits.push(*hit);
            continue;
        }
        if hit.composed.is_none() {
            slot.insert(shown.clone(), groups.len());
        }
        groups.push(ResultGroup {
            shown,
            hits: vec![*hit],
        });
    }
    groups
}

/// The English sources a group translates, each with how many of its places read it:
/// "Cancel ×14 · Undo ×9". Most frequent first.
pub(crate) fn english_breakdown(
    index: &LocaleIndex,
    english: &LocaleIndex,
    group: &ResultGroup,
) -> Vec<(String, usize)> {
    let mut counts: Vec<(String, usize)> = Vec::new();
    for hit in &group.hits {
        let Some(source) = english
            .counterpart(&index.entries[hit.entry])
            .map(|e| english.entries[e].shown().to_string())
        else {
            continue;
        };
        match counts.iter_mut().find(|(s, _)| *s == source) {
            Some((_, n)) => *n += 1,
            None => counts.push((source, 1)),
        }
    }
    counts.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    counts
}

/// A query that matched the English source rather than the translation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct EnglishHit {
    /// Index into the English index.
    pub english: usize,
    pub rank: Rank,
    /// The translation of the same message in the target index, if the target ships
    /// one.
    pub translation: Option<usize>,
    /// Whether the translation reads exactly as the English: "not translated".
    pub untranslated: bool,
}

/// Hits of `query` in the English index, each with its translation in `index`.
/// Empty when `index` is English itself. Only the closer ranks: a partial match on
/// the English side is noise beside the target-language results.
pub(crate) fn english_hits(
    index: &LocaleIndex,
    english: &LocaleIndex,
    query: &str,
) -> Vec<EnglishHit> {
    if index.locale == SOURCE_LOCALE {
        return Vec::new();
    }
    english
        .search(query)
        .into_iter()
        .filter(|h| h.rank <= Rank::InOrder)
        .map(|h| {
            let translation = index.counterpart(&english.entries[h.entry]);
            let untranslated = translation
                .is_some_and(|t| index.entries[t].shown() == english.entries[h.entry].shown());
            EnglishHit {
                english: h.entry,
                rank: h.rank,
                translation,
                untranslated,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::startup::SUPPORTED_LOCALES;
    use std::sync::LazyLock;

    static FRENCH: LazyLock<Option<LocaleIndex>> = LazyLock::new(|| LocaleIndex::build("fr-FR"));
    static ENGLISH: LazyLock<Option<LocaleIndex>> =
        LazyLock::new(|| LocaleIndex::build(SOURCE_LOCALE));

    fn french() -> &'static LocaleIndex {
        FRENCH.as_ref().expect("fr-FR is shipped")
    }
    fn english() -> &'static LocaleIndex {
        ENGLISH.as_ref().expect("en-US is shipped")
    }

    #[test]
    fn an_unshipped_locale_has_no_index() {
        assert!(LocaleIndex::build("xx-YY").is_none());
    }

    #[test]
    fn a_whole_value_is_found_without_its_mnemonic_or_its_accents() {
        let fr = french();
        let hits = fr.search("annuler");
        let whole: Vec<&str> = hits
            .iter()
            .filter(|h| h.rank == Rank::Whole)
            .map(|h| fr.entries[h.entry].shown())
            .collect();
        assert!(!whole.is_empty());
        assert!(
            whole.iter().all(|s| s.eq_ignore_ascii_case("annuler")),
            "{whole:?}"
        );
        let mnemonic = hits
            .iter()
            .find(|h| fr.entries[h.entry].source.contains('&'))
            .expect("a menu label with a mnemonic reads Annuler");
        assert_eq!(mnemonic.rank, Rank::Whole);
        // Folding: no accent typed, accent found.
        let elements = fr.search("elements");
        assert!(!elements.is_empty());
        assert!(elements.iter().any(|h| {
            fr.entries[h.entry]
                .shown()
                .to_lowercase()
                .contains("éléments")
        }),);
    }

    #[test]
    fn a_variable_accepts_what_the_writer_saw_in_its_place() {
        let fr = french();
        let undo = fr
            .find(Catalogue::App, "menu-edit-undo-target")
            .expect("indexed");
        let hits = fr.search("Annuler n'importe quoi");
        assert!(
            hits.iter()
                .any(|h| h.entry == undo && h.rank == Rank::Whole),
            "{:?}",
            hits.iter()
                .take(5)
                .map(|h| &fr.entries[h.entry].id)
                .collect::<Vec<_>>()
        );
    }

    /// The Edit menu's composed label returns the pair, naming both messages.
    #[test]
    fn a_composed_label_returns_the_pair() {
        let fr = french();
        let undo = fr
            .find(Catalogue::App, "menu-edit-undo-target")
            .expect("indexed");
        let trash = fr
            .find(Catalogue::App, "undo-target-trash")
            .expect("indexed");
        let shown = fr.entries[trash].shown().to_string();
        let query = format!("Annuler {shown}");
        let hits = fr.search(&query);
        let pair = hits
            .iter()
            .find(|h| h.entry == undo)
            .unwrap_or_else(|| panic!("{query:?} did not find the undo label"));
        assert_eq!(pair.rank, Rank::Whole);
        assert_eq!(pair.composed, Some(trash), "{query:?}");
    }

    #[test]
    fn words_in_order_rank_above_words_in_any_order() {
        assert!(Rank::Whole < Rank::InOrder);
        assert!(Rank::InOrder < Rank::AllWords);
        assert!(Rank::AllWords < Rank::Partial);
        let folded = FoldedText::new(
            "Annuler la mise à la corbeille",
            &FoldSpec {
                case_sensitive: false,
                diacritic_sensitive: false,
                locale: FoldLocale::Root,
            },
        );
        let w = |s: &str| query_words(s);
        assert_eq!(
            word_rank(&folded, &w("mise corbeille")),
            Some(Rank::InOrder)
        );
        assert_eq!(
            word_rank(&folded, &w("corbeille mise")),
            Some(Rank::AllWords)
        );
        assert_eq!(word_rank(&folded, &w("corb mis")), Some(Rank::Partial));
        assert_eq!(word_rank(&folded, &w("fenêtre")), None);
    }

    #[test]
    fn whole_match_needs_every_wildcard_filled() {
        let spec = FoldSpec {
            case_sensitive: false,
            diacritic_sensitive: false,
            locale: FoldLocale::Root,
        };
        let pattern = format!("{WILD} éléments sélectionnés");
        let q = FoldedText::new("3 elements selectionnes", &spec);
        assert_eq!(whole_match(&pattern, &q, 23), Some(vec![(0, 1)]));
        let q = FoldedText::new(" éléments sélectionnés", &spec);
        assert_eq!(
            whole_match(&pattern, &q, 22),
            None,
            "the wildcard took nothing"
        );
        let middle = format!("De {WILD} à {WILD}");
        let q = FoldedText::new("De Paris à Rome", &spec);
        assert_eq!(whole_match(&middle, &q, 15), Some(vec![(3, 8), (11, 15)]));
        let q = FoldedText::new("Annuler", &spec);
        assert_eq!(whole_match("Annuler", &q, 7), Some(vec![]));
        assert_eq!(whole_match("Annuler tout", &q, 7), None);
    }

    /// Arabic writes its numbers with its own digits, and a variable takes them.
    #[test]
    fn a_number_in_its_own_script_fills_a_variable() {
        let ar = LocaleIndex::build("ar-SA").expect("ar-SA is shipped");
        let with_count = ar
            .entries
            .iter()
            .enumerate()
            .find(|(_, e)| {
                e.catalogue == Catalogue::App
                    && e.forms.iter().any(|f| {
                        f.search.starts_with(WILD)
                            && f.search.chars().filter(|c| *c == WILD).count() == 1
                    })
            })
            .map(|(i, e)| {
                (
                    i,
                    e.forms
                        .iter()
                        .find(|f| f.search.starts_with(WILD))
                        .map(|f| f.search.clone())
                        .unwrap_or_default(),
                )
            })
            .expect("an Arabic message starts with a number");
        let query = with_count.1.replacen(WILD, "٣", 1);
        let hits = ar.search(&query);
        assert!(
            hits.iter()
                .any(|h| h.entry == with_count.0 && h.rank == Rank::Whole),
            "{query:?}"
        );
    }

    #[test]
    fn turkish_dotted_i_folds_as_turkish() {
        let tr = LocaleIndex::build("tr-TR").expect("tr-TR is shipped");
        let entry = tr
            .entries
            .iter()
            .position(|e| e.catalogue == Catalogue::App && e.shown().contains('İ'))
            .expect("a Turkish string holds a capital dotted I");
        let lowered: String = tr.entries[entry]
            .shown()
            .chars()
            .map(|c| if c == 'İ' { 'i' } else { c })
            .collect();
        let hits = tr.search(&lowered);
        assert!(hits.iter().any(|h| h.entry == entry), "{lowered:?}");
    }

    #[test]
    fn identical_values_gather_into_one_group_with_their_english_sources() {
        let fr = french();
        let hits = fr.search("Annuler");
        let groups = group(fr, &hits);
        let annuler = groups
            .iter()
            .find(|g| g.shown == "Annuler")
            .expect("a group reads Annuler");
        assert!(
            annuler.hits.len() > 1,
            "fr-FR uses Annuler for several messages"
        );
        let breakdown = english_breakdown(fr, english(), annuler);
        assert!(
            breakdown.len() > 1,
            "Cancel and Undo both read Annuler: {breakdown:?}"
        );
        let total: usize = breakdown.iter().map(|(_, n)| n).sum();
        assert!(total <= annuler.hits.len());
    }

    #[test]
    fn an_english_query_names_the_translation() {
        let fr = french();
        let en = english();
        let hits = english_hits(fr, en, "Report a Problem…");
        let report = hits
            .iter()
            .find(|h| en.entries[h.english].id == "menu-help-report")
            .expect("the English label is found");
        let t = report.translation.expect("French translates it");
        assert_eq!(fr.entries[t].id, "menu-help-report");
        assert!(!report.untranslated);
        assert!(english_hits(en, en, "Report").is_empty());
    }

    #[test]
    fn a_single_letter_finds_only_whole_values() {
        let fr = french();
        assert!(fr.search("a").iter().all(|h| h.rank == Rank::Whole));
        assert!(fr.search("   ").is_empty());
    }

    /// **The guarantee the window rests on**: whatever a catalogue line shows, typing
    /// it is a whole match for that line, in every locale and catalogue. The query is
    /// the *shown* text with each variable filled in, as a writer would read it on
    /// screen, not the index's search string: building both sides from the same
    /// normalisation would pass trivially. Exhaustive through `rank_entry`, the
    /// per-entry step of `search`; the ranking and grouping around it are checked on a
    /// sample by [`a_sample_of_shown_strings_lands_in_the_first_groups`].
    #[test]
    fn every_shown_string_finds_its_own_message() {
        for locale in SUPPORTED_LOCALES {
            let index = LocaleIndex::build(locale).expect("shipped");
            for (i, entry) in index.entries.iter().enumerate() {
                if entry.catalogue == Catalogue::Help {
                    continue;
                }
                let Some(form) = entry.forms.first() else {
                    continue;
                };
                // A value with no word of its own is not findable by its text; see
                // `whole_match`.
                if !form
                    .search
                    .split(WILD)
                    .any(|s| s.chars().any(char::is_alphanumeric))
                {
                    continue;
                }
                // A value that shows braces of its own (a string literal such as
                // `{"{ $count }"}`) cannot be told from a variable in the shown
                // text; `a_literal_brace_is_found_as_typed` covers that case.
                if form.shown.matches("{ $").count() != form.search.matches(WILD).count() {
                    continue;
                }
                let query = fill_variables(&form.shown);
                if query.trim().is_empty() {
                    continue;
                }
                let prepared = Query::new(&query, &index.spec);
                let rank = prepared
                    .and_then(|q| index.rank_entry(i, &q))
                    .map(|h| h.rank);
                assert_eq!(
                    rank,
                    Some(Rank::Whole),
                    "{locale} {} {}: typing {query:?} is not a whole match",
                    entry.catalogue.slug(),
                    entry.id
                );
            }
        }
    }

    /// The full search on a sample: a shown string lands in one of the whole-match
    /// groups, which the window lists first.
    #[test]
    fn a_sample_of_shown_strings_lands_in_the_first_groups() {
        for locale in ["fr-FR", "de-DE", "ja-JP", "ar-SA"] {
            let index = LocaleIndex::build(locale).expect("shipped");
            for (i, entry) in index.entries.iter().enumerate().step_by(97) {
                if entry.catalogue == Catalogue::Help {
                    continue;
                }
                let Some(form) = entry.forms.first() else {
                    continue;
                };
                if !form
                    .search
                    .split(WILD)
                    .any(|s| s.chars().any(char::is_alphanumeric))
                {
                    continue;
                }
                let query = fill_variables(&form.shown);
                let groups = group(&index, &index.search(&query));
                let found = groups
                    .iter()
                    .take_while(|g| g.hits.first().is_some_and(|h| h.rank == Rank::Whole))
                    .any(|g| g.hits.iter().any(|h| h.entry == i));
                assert!(
                    found,
                    "{locale} {}: {query:?} is not in the first groups",
                    entry.id
                );
            }
        }
    }

    /// A value showing literal braces is found by typing them, braces and all.
    #[test]
    fn a_literal_brace_is_found_as_typed() {
        let en = english();
        let hint = en
            .find(Catalogue::App, "translation-report-hint-placeholder")
            .expect("indexed");
        let shown = en.entries[hint].shown().to_string();
        assert!(shown.contains("{ $count }"), "{shown}");
        let hits = en.search(&shown);
        assert!(
            hits.iter()
                .any(|h| h.entry == hint && h.rank == Rank::Whole)
        );
    }

    /// `{ $name }` replaced by a plausible value, as the screen shows it.
    fn fill_variables(shown: &str) -> String {
        let mut out = String::new();
        let mut rest = shown;
        while let Some(open) = rest.find("{ $") {
            out.push_str(&rest[..open]);
            match rest[open..].find(" }") {
                Some(close) => {
                    out.push('7');
                    rest = &rest[open + close + 2..];
                }
                None => {
                    rest = &rest[open..];
                    break;
                }
            }
        }
        out.push_str(rest);
        out
    }

    /// Help paragraphs are long; a sentence from one still finds its page.
    #[test]
    fn a_sentence_from_a_help_page_finds_its_paragraph() {
        let fr = french();
        let (i, entry) = fr
            .entries
            .iter()
            .enumerate()
            .find(|(_, e)| e.catalogue == Catalogue::Help && e.shown().split(' ').count() > 12)
            .expect("a French help paragraph");
        let words: Vec<&str> = entry.shown().split(' ').skip(3).take(6).collect();
        let hits = fr.search(&words.join(" "));
        assert!(hits.iter().any(|h| h.entry == i), "{words:?}");
    }
}
