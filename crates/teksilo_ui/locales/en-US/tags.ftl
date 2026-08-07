# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Tags — the per-project palette, the inspector section, and the preset catalogue.
# Preset tag NAMES are translated deliberately: presets are generated in code rather than
# shipped as data precisely so a French project gets French tag names.

## Preset menu
tags-preset-basic = Basic
tags-preset-scifi = Science fiction
tags-preset-fantasy = Fantasy
tags-preset-mystery = Mystery
tags-preset-historical = Historical

## Basic preset — the workflow ladder. The `status/` prefix is a naming convention:
## alphabetical sorting alone makes these cluster together in every list.
tags-preset-status-outline = status/outline
tags-preset-status-draft = status/draft
tags-preset-status-to-review = status/to review
tags-preset-status-finished = status/finished

## Basic preset — flags. Deliberately unprefixed: a scene can be a draft AND need
## research at the same time, so these are not part of the ladder above.
tags-preset-needs-research = needs research
tags-preset-continuity-check = continuity check
tags-preset-plot-point = plot point

## Basic preset — the taxonomy the mention index scans prose for.
tags-preset-character = character
tags-preset-place = place
tags-preset-item = item

## Genre additions
tags-preset-vessel = vessel
tags-preset-planet = planet
tags-preset-organization = organization
tags-preset-creature = creature
tags-preset-faction = faction
tags-preset-artifact = artifact
tags-preset-realm = realm
tags-preset-magic-system = magic system
tags-preset-suspect = suspect
tags-preset-victim = victim
tags-preset-clue = clue
tags-preset-red-herring = red herring
tags-preset-historical-figure = historical figure
tags-preset-source = source
tags-preset-period-detail = period detail

## The tag pill field (Inspector) and its "+" picker
tags-pill-list = Tags
tags-pill-add = Add a tag
tags-pill-remove = Remove { $name }
tags-pill-filter-placeholder = Filter or name a new tag
tags-pill-no-match = No tag matches
tags-pill-create = Create "{ $name }"
tags-pill-new-discoverable = Story-bible tag
tags-pill-new-discoverable-hint = Items with this tag are matched against your prose to fill the roster.

## The alias pill field
tags-alias-list = Also known as
tags-alias-add = Add another name
tags-alias-remove = Remove { $name }
tags-alias-placeholder = Another name, then Enter
tags-alias-hint = Names this appears under in your prose, beside its title.

## Settings ▸ Work ▸ Tags
settings-page-tags = Tags
settings-tags-desc = Tags label the items in your binder. A story-bible tag also tells Skribisto to look for that item's names in your prose.
settings-tags-add = Add tag
settings-tags-add-placeholder = Name a new tag
settings-tags-added = Added "{ $name }"
settings-tags-duplicate = "{ $name }" already exists
settings-tags-filter = Filter tags
settings-tags-count = { $n ->
    [one] 1 tag
   *[other] { $n } tags
}
settings-tags-details-placeholder = What this tag means
settings-tags-discoverable = Story bible
settings-tags-delete = Delete { $name }
settings-tags-deleted = Deleted "{ $name }" and removed it from every item
settings-tags-empty = No tags yet.
settings-tags-apply-preset = Apply a preset…
settings-tags-preset-applied = Added { $added }, skipped { $skipped } already present
settings-tags-csv-filter = CSV files
settings-tags-import = Import…
settings-tags-export = Export…
settings-tags-imported = Imported { $added }, skipped { $skipped }
settings-tags-exported = Exported { $n ->
    [one] 1 tag
   *[other] { $n } tags
}

## The dot row shown on the stream, corkboard and editor subtitle
tags-chip-more = { $n ->
    [one] 1 more tag
   *[other] { $n } more tags
}

## Cast of the scene (references-first story-bible pins + suggestions)
cast-section = Cast
cast-add = Add to cast…
cast-add-filter-placeholder = Filter story bible…
cast-add-empty = No story-bible entries to add
cast-pin = Add { $name } to cast
cast-unpin = Remove { $name } from cast
cast-empty = No one pinned yet — add or keep a suggestion

## Backlinks on a story-bible item
mentions-backlinks = Appears in
mentions-hit-count = { $n ->
    [one] once
   *[other] { $n } times
}

## Legacy keys kept so older scripts/tests that still reference them compile
mentions-roster = Cast
mentions-pin = Add { $name } to cast

# ── Point of view ────────────────────────────────────────────────────────────
# Whose eyes a scene is told through. Distinct from the cast above: the cast is
# who appears, the point of view is who holds the camera. A scene may have none
# (unassigned) or, deliberately, more than one — which is what head-hopping is.
pov-section = Point of view
pov-empty = No point of view set
pov-add = Set point of view…
pov-multiple = This scene has two points of view.
pov-remove = Remove { $name } as point of view
