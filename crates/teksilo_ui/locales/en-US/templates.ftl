# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Note templates — the per-project catalog, its settings pane, the Document menu's
# insert submenu, and the built-in presets.
#
# Preset NAMES and the section/field labels their bodies are built from are translated
# deliberately: presets are assembled in code rather than shipped as `.djot` assets
# precisely so a French project gets a French character sheet — and so improving the
# English wording later reaches every locale instead of being frozen into whatever
# projects had already applied it.

## Settings ▸ Work ▸ Templates
settings-page-templates = Templates
settings-templates-description = Reusable pieces of writing you can drop into anything you are writing — a blank character sheet, a location profile, a beat sheet. They are stored in this project, so anyone who opens it gets the same set.
settings-templates-filter = Filter templates
settings-templates-count = { $n ->
    [0] No templates
    [one] 1 template
   *[other] { $n } templates
}
settings-templates-empty = This project has no templates yet.
settings-templates-empty-hint = Add one from a preset, import a .md or .djot file, or write something and choose Document ▸ Save as template.
settings-templates-presets = Add a preset
settings-templates-import = Import…
settings-templates-export = Export…
settings-templates-star = Show first in the insert menu
settings-templates-unstar = Stop showing first
settings-templates-move-up = Move up
settings-templates-move-down = Move down
settings-templates-delete = Delete template
settings-templates-name = Name
settings-templates-name-placeholder = Template name
settings-templates-body-placeholder = The text this template inserts
settings-templates-duplicate-name = Another template is already called "{ $name }"
settings-templates-words = { $n ->
    [one] 1 word
   *[other] { $n } words
}

## Import / export feedback
templates-imported = { $added ->
    [one] 1 template imported
   *[other] { $added } templates imported
}
templates-imported-renamed = Renamed to avoid a clash: { $names }
templates-imported-skipped = Could not read: { $names }
templates-import-filter = Templates (.md, .djot)
templates-exported = { $n ->
    [one] 1 template exported
   *[other] { $n } templates exported
}
templates-preset-applied = Added "{ $name }"

## Delete confirmation
templates-delete-title = Delete this template?
templates-delete-body = "{ $name }" will be removed from this project. Notes you already created from it are not affected.
templates-delete-confirm = Delete

## Document menu
menu-document = &Document
menu-insert-template = Insert temp&late
menu-insert-template-none = No templates in this project
menu-save-as-template = Sa&ve as template…

## Save as template
save-as-template-title = Save as template
save-as-template-explain = The text you are editing becomes a template you can drop in anywhere else.
save-as-template-name = Name
save-as-template-placeholder = Character sheet
save-as-template-duplicate = A template called "{ $name }" already exists
save-as-template-empty-editor = This editor is empty — there is nothing to save.
save-as-template-confirm = Save template
save-as-template-saved = Saved "{ $name }" as a template
template-inserted = Inserted "{ $name }"

## Preset names
note-template-preset-character-sheet = Character sheet
note-template-preset-location = Location
note-template-preset-artifact = Object
note-template-preset-beat-sheet = Beat sheet
note-template-preset-faction = Faction
note-template-preset-research-note = Research note

## Starter sets, offered when a project is created
note-template-set-essentials = Essentials
note-template-set-everything = Every template

## Preset section headings
note-template-section-identity = Identity
note-template-section-appearance = Appearance
note-template-section-voice = Voice
note-template-section-psychology = Inner life
note-template-section-history = History
note-template-section-arc = Arc
note-template-section-first-impression = First impression
note-template-section-in-the-story = In the story
note-template-section-the-thing-itself = The object itself
note-template-section-the-scene = The scene
note-template-section-the-shape = The shape
note-template-section-what-it-is = What it is
note-template-section-source = Source
note-template-section-what-it-says = What it says

## Preset field prompts
note-template-field-full-name = Full name
note-template-field-known-as = Also known as, tracked in the Inspector's Alias field
note-template-field-age = Age
note-template-field-role-in-story = Role in the story
note-template-field-build-and-features = Build and distinguishing features
note-template-field-habitual-bearing = Habitual bearing
note-template-field-speech-patterns = Speech patterns and verbal tics
note-template-field-what-they-never-say = What they never say
note-template-field-want = What they want
note-template-field-need = What they need but do not know it
note-template-field-fear = What they fear
note-template-field-flaw = The flaw that costs them
note-template-field-formative-event = Formative event
note-template-field-relationships = Relationships
note-template-field-starts-as = Starts as
note-template-field-ends-as = Ends as
note-template-field-what-you-notice-first = What you notice first
note-template-field-sound-and-smell = Sound and smell
note-template-field-light-and-weather = Light and weather
note-template-field-what-happened-here = What happened here
note-template-field-who-lives-or-works-here = Who lives or works here
note-template-field-scenes-set-here = Scenes set here
note-template-field-why-it-matters = Why it matters
note-template-field-appearance = Appearance
note-template-field-age-and-origin = Age and origin
note-template-field-what-it-does = What it does
note-template-field-who-holds-it = Who holds it
note-template-field-who-wants-it = Who wants it
note-template-field-pov = Point of view
note-template-field-time-and-place = Time and place
note-template-field-goal = Goal
note-template-field-conflict = Conflict
note-template-field-turn = The turn
note-template-field-exit-emotion = Emotion on the way out
note-template-field-purpose = Purpose
note-template-field-who-leads-it = Who leads it
note-template-field-resources = Resources
note-template-field-allies-and-enemies = Allies and enemies
note-template-field-what-it-wants-now = What it wants now
note-template-field-where-from = Where it came from
note-template-field-page-or-link = Page or link
note-template-field-key-facts = Key facts
note-template-field-how-it-is-used = How it is used in the story
