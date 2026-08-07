// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Built-in note templates, **assembled in code rather than shipped as data**.
//!
//! Same reasoning as [`crate::tags::presets`], and it matters more here: a template body is
//! a whole document, so the obvious alternative — one `.djot` asset per locale, loaded by
//! `include_str!` — would bake a language into the project at the moment the writer applied
//! the preset, and nothing would ever retrofit it if the English master were later improved.
//! Building the body from `tr!`'d **field labels** instead keeps every string in
//! `templates.ftl`, where it is compile-validated against `en-US.ftl` like every other key
//! in the app, and means the preset is always resolved in the locale the writer is using
//! *now*.
//!
//! Fluent could not hold the bodies whole in any case: a multiline FTL value ends at the
//! first blank line, and every one of these has blank lines between its sections.
//!
//! A new project starts with **no** templates; the writer applies a preset if they want
//! one. Applied rows are ordinary rows thereafter — renamable, starrable, deletable — and
//! re-applying is safe: `import_note_templates` suffixes a colliding name rather than
//! duplicating or skipping it.
//!
//! **Djot, not Markdown.** Bodies here use Djot's own inline syntax, which differs from GFM
//! where it counts: strong is `*x*` (not `**x**`), emphasis is `_x_`. Headings and `-`
//! bullets are shared. `every_preset_body_round_trips_through_djot` pins that these parse.

use teksilo::i18n::LocalizedString;
use teksilo::prelude::*;

use crate::models::TemplateRow;

/// Which built-in the writer picked.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Preset {
    CharacterSheet,
    Location,
    Artifact,
    BeatSheet,
    Faction,
    ResearchNote,
}

impl Preset {
    /// Every preset, in menu order: the two a novelist reaches for constantly, then the
    /// worldbuilding trio, then research.
    pub const ALL: [Preset; 6] = [
        Preset::CharacterSheet,
        Preset::Location,
        Preset::Artifact,
        Preset::BeatSheet,
        Preset::Faction,
        Preset::ResearchNote,
    ];

    /// The menu label, and the name the created template gets.
    pub fn label(self) -> LocalizedString {
        match self {
            Preset::CharacterSheet => tr!(note_template_preset_character_sheet()),
            Preset::Location => tr!(note_template_preset_location()),
            Preset::Artifact => tr!(note_template_preset_artifact()),
            Preset::BeatSheet => tr!(note_template_preset_beat_sheet()),
            Preset::Faction => tr!(note_template_preset_faction()),
            Preset::ResearchNote => tr!(note_template_preset_research_note()),
        }
    }

    /// The row to hand to `import_note_templates`, resolved in the active locale.
    ///
    /// The id is zero — the import assigns a real one. Only the character sheet is starred:
    /// it is the one a novelist reaches for most, and starring everything would make the
    /// star mean nothing in the insert menu.
    pub fn rows(self) -> Vec<TemplateRow> {
        vec![TemplateRow {
            id: 0,
            name: self.label().resolve_now(),
            body: self.body(),
            starred: self == Preset::CharacterSheet,
        }]
    }

    /// The Djot body.
    ///
    /// Deliberately lean. Every comparable tool that shipped a deep fixed schema backed off
    /// it — bibisco's 138-question interview is *optional* prompts, Plottr and Manuskript
    /// both ended up letting the user author the field list. These are a starting point the
    /// writer edits, not a form to complete, so each one is a page of prompts at most.
    fn body(self) -> String {
        match self {
            Preset::CharacterSheet => doc(
                self.label(),
                &[
                    (
                        tr!(note_template_section_identity()),
                        &[
                            tr!(note_template_field_full_name()),
                            tr!(note_template_field_known_as()),
                            tr!(note_template_field_age()),
                            tr!(note_template_field_role_in_story()),
                        ],
                    ),
                    (
                        tr!(note_template_section_appearance()),
                        &[
                            tr!(note_template_field_build_and_features()),
                            tr!(note_template_field_habitual_bearing()),
                        ],
                    ),
                    (
                        tr!(note_template_section_voice()),
                        &[
                            tr!(note_template_field_speech_patterns()),
                            tr!(note_template_field_what_they_never_say()),
                        ],
                    ),
                    (
                        tr!(note_template_section_psychology()),
                        &[
                            tr!(note_template_field_want()),
                            tr!(note_template_field_need()),
                            tr!(note_template_field_fear()),
                            tr!(note_template_field_flaw()),
                        ],
                    ),
                    (
                        tr!(note_template_section_history()),
                        &[
                            tr!(note_template_field_formative_event()),
                            tr!(note_template_field_relationships()),
                        ],
                    ),
                    (
                        tr!(note_template_section_arc()),
                        &[
                            tr!(note_template_field_starts_as()),
                            tr!(note_template_field_ends_as()),
                        ],
                    ),
                ],
            ),
            Preset::Location => doc(
                self.label(),
                &[
                    (
                        tr!(note_template_section_first_impression()),
                        &[
                            tr!(note_template_field_what_you_notice_first()),
                            tr!(note_template_field_sound_and_smell()),
                            tr!(note_template_field_light_and_weather()),
                        ],
                    ),
                    (
                        tr!(note_template_section_history()),
                        &[
                            tr!(note_template_field_what_happened_here()),
                            tr!(note_template_field_who_lives_or_works_here()),
                        ],
                    ),
                    (
                        tr!(note_template_section_in_the_story()),
                        &[
                            tr!(note_template_field_scenes_set_here()),
                            tr!(note_template_field_why_it_matters()),
                        ],
                    ),
                ],
            ),
            Preset::Artifact => doc(
                self.label(),
                &[
                    (
                        tr!(note_template_section_the_thing_itself()),
                        &[
                            tr!(note_template_field_appearance()),
                            tr!(note_template_field_age_and_origin()),
                            tr!(note_template_field_what_it_does()),
                        ],
                    ),
                    (
                        tr!(note_template_section_in_the_story()),
                        &[
                            tr!(note_template_field_who_holds_it()),
                            tr!(note_template_field_who_wants_it()),
                            tr!(note_template_field_why_it_matters()),
                        ],
                    ),
                ],
            ),
            Preset::BeatSheet => doc(
                self.label(),
                &[
                    (
                        tr!(note_template_section_the_scene()),
                        &[
                            tr!(note_template_field_pov()),
                            tr!(note_template_field_time_and_place()),
                        ],
                    ),
                    (
                        tr!(note_template_section_the_shape()),
                        &[
                            tr!(note_template_field_goal()),
                            tr!(note_template_field_conflict()),
                            tr!(note_template_field_turn()),
                            tr!(note_template_field_exit_emotion()),
                        ],
                    ),
                ],
            ),
            Preset::Faction => doc(
                self.label(),
                &[
                    (
                        tr!(note_template_section_what_it_is()),
                        &[
                            tr!(note_template_field_purpose()),
                            tr!(note_template_field_who_leads_it()),
                            tr!(note_template_field_resources()),
                        ],
                    ),
                    (
                        tr!(note_template_section_in_the_story()),
                        &[
                            tr!(note_template_field_allies_and_enemies()),
                            tr!(note_template_field_what_it_wants_now()),
                        ],
                    ),
                ],
            ),
            Preset::ResearchNote => doc(
                self.label(),
                &[
                    (
                        tr!(note_template_section_source()),
                        &[
                            tr!(note_template_field_where_from()),
                            tr!(note_template_field_page_or_link()),
                        ],
                    ),
                    (
                        tr!(note_template_section_what_it_says()),
                        &[tr!(note_template_field_key_facts())],
                    ),
                    (
                        tr!(note_template_section_in_the_story()),
                        &[tr!(note_template_field_how_it_is_used())],
                    ),
                ],
            ),
        }
    }
}

/// Assemble one Djot document: a level-1 title, then a level-2 heading per section with its
/// prompts as a bullet list.
///
/// Each prompt ends in a colon and nothing else — the writer types after it. A prompt with
/// placeholder text would have to be deleted before it could be replaced, which is worse
/// than an empty line.
fn doc(title: LocalizedString, sections: &[(LocalizedString, &[LocalizedString])]) -> String {
    let mut out = format!("# {}\n", title.resolve_now());
    for (heading, fields) in sections {
        out.push_str(&format!("\n## {}\n\n", heading.resolve_now()));
        for f in *fields {
            out.push_str(&format!("- {}:\n", f.resolve_now()));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every preset must produce a non-empty, uniquely named row.
    #[test]
    fn every_preset_yields_one_named_row() {
        let mut names = Vec::new();
        for p in Preset::ALL {
            let rows = p.rows();
            assert_eq!(rows.len(), 1, "{p:?} must yield exactly one template");
            let r = &rows[0];
            assert!(!r.name.trim().is_empty(), "{p:?} has a blank name");
            assert!(!r.body.trim().is_empty(), "{p:?} has a blank body");
            assert_eq!(r.id, 0, "{p:?} must leave id assignment to the import");
            names.push(r.name.clone());
        }
        let mut sorted = names.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), names.len(), "preset names must be distinct");
    }

    /// Exactly one preset is starred. Starring several would make the star meaningless in
    /// the insert menu, which is the only thing it drives.
    #[test]
    fn exactly_one_preset_is_starred() {
        let starred: Vec<_> = Preset::ALL.iter().filter(|p| p.rows()[0].starred).collect();
        assert_eq!(starred.len(), 1, "got {starred:?}");
    }

    /// The bodies must be **Djot**, and must survive the exact trip they take when
    /// inserted: parsed into a document, then written back out. This is what would catch a
    /// body hand-authored with GFM's `**bold**`, which Djot does not read the same way.
    #[test]
    fn every_preset_body_round_trips_through_djot() {
        for p in Preset::ALL {
            let body = p.rows()[0].body.clone();
            let doc = teksilo::text_document::TextDocument::new();
            doc.set_djot(&body)
                .and_then(|op| op.wait())
                .unwrap_or_else(|e| panic!("{p:?} body is not valid Djot: {e}"));
            let out = doc.to_djot().expect("export djot");
            assert!(
                !out.trim().is_empty(),
                "{p:?} body vanished through the document model"
            );
            // The title survives, so the writer sees the heading they expect.
            let title = p.label().resolve_now();
            assert!(
                out.contains(&title),
                "{p:?} lost its title '{title}' in the round trip:\n{out}"
            );
        }
    }

    /// No preset may use GFM's double-asterisk strong — Djot reads `*x*` as strong, so a
    /// copy-pasted Markdown body would render with stray asterisks.
    #[test]
    fn no_preset_uses_markdown_strong() {
        for p in Preset::ALL {
            let body = p.rows()[0].body.clone();
            assert!(
                !body.contains("**"),
                "{p:?} uses GFM `**strong**`; Djot's strong is a single `*`"
            );
        }
    }

    /// The shape the `doc` builder promises: a level-1 title, level-2 sections, `- ` prompts.
    #[test]
    fn a_body_has_a_title_sections_and_prompts() {
        let body = Preset::CharacterSheet.rows()[0].body.clone();
        assert!(body.starts_with("# "), "starts with the title: {body}");
        assert!(body.contains("\n## "), "has sections: {body}");
        assert!(body.contains("\n- "), "has prompts: {body}");
        assert!(
            body.lines()
                .filter(|l| l.starts_with("- "))
                .all(|l| l.ends_with(':')),
            "every prompt ends in a colon and nothing else"
        );
    }
}
