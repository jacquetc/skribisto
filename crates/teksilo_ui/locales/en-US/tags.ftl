# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Tags — the per-project palette, the inspector section, and the preset catalog.
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
tags-pill-create-failed = Couldn't create "{ $name }"
tags-pill-new-discoverable = Find-in-prose tag
tags-pill-new-discoverable-hint = Items with this tag are matched against your prose to fill the roster.

## The alias pill field
tags-alias-list = Also known as
tags-alias-add = Add another name
tags-alias-remove = Remove { $name }
tags-alias-placeholder = Another name, then Enter
tags-alias-hint = Names this appears under in your prose, beside its title.
tags-alias-collision = { $name } already answers to this name.

## Settings ▸ Work ▸ Tags
settings-page-tags = Tags
settings-tags-desc = Tags label the items in your binder. A find-in-prose tag also tells Skribisto to look for that item's names in your prose.
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
settings-tags-discoverable = Find in prose
settings-tags-creates-in = New notes go to
settings-tags-creates-in-unset = Ask me the first time
settings-tags-creates-in-untitled = Untitled folder
settings-tags-creates-in-trashed = { $name } (in the trash)
settings-tags-template = Starting template
settings-tags-template-unset = Blank note
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
cast-unresolved = No longer in the story bible

## Backlinks on a story-bible item
mentions-backlinks = Appears in
# The control on a suggested backlink row: the writer says yes, this really is her,
# and the entry is written into that document's cast. Named after what it does rather
# than after the agreement, and worded to mirror `cast-pin`, which is the same write
# made from the other end. Confirm only — there is no "not her" to record; see
# `teksilo_ui::mentions::presence`.
mentions-confirm = Add to the cast of { $name }
mentions-hit-count = { $n ->
    [one] once
   *[other] { $n } times
}
# The badge on a row whose target is the owner's declared point of view (see
# `point_of_view`, distinct from `references`). Shown on both the Cast list and the
# backlinks ("Appears in") list, since a point of view can appear in either direction.
mentions-point-of-view-badge = Point of view
mentions-point-of-view-badge-tooltip = Declared as the point of view here, set by hand, not detected in the prose.

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
pov-unresolved = A point of view was pinned here, but that entry no longer exists or lost its story-bible tag.

# ── Book filing ──────────────────────────────────────────────────────────────
# Which Book or Books a note or note folder is declared to belong to. Shown
# only on story-bible material outside the manuscript flow, since a scene's
# Book is already given by where it sits in the binder, and only once the
# project holds two or more Books; a one-Book project has nothing to file
# against.
books-section = Filed under
books-empty = Not filed under a book yet
books-add = File under a book…
books-remove = Remove from { $name }
books-apply-to-children = Apply filing to children

# ── The Details segment on an `Item/Note` tab ────────────────────────────────
# Almost the same field set as the Inspector's own story-bible section (tags,
# aliases, cast, point of view, books), reused deliberately, but under its own
# `note-details-` keys: this segment is a full-width page, not a dock, so its
# copy is free to say a little more than the dock's narrower strings do. See
# `teksilo_ui::tabs::note_details`.
note-details-name-placeholder = Name…
note-details-tags = Tags
note-details-aliases = Other names
note-details-books = Filed under
note-details-books-empty = Not filed under a Book yet
note-details-links = Links
note-details-links-empty = No links yet. Add one to point at another entry in your story bible.
note-details-links-add = Link to a note…
note-details-pov = Point of view
note-details-pov-empty = No point of view set
note-details-pov-unresolved = A point of view was pinned here, but that entry no longer exists or lost its story-bible tag.
note-details-pov-multiple = This note has two points of view.
note-details-backlinks = Appears in the manuscript
note-details-backlinks-empty = Nothing yet. Once this name appears in your prose, it will show up here.
note-details-backlinks-outside = Outside the books
note-details-backlinks-confirm-all = Confirm every appearance
note-details-backlinks-confirm-all-tooltip = Add this entry to the cast of every document shown here that has not confirmed it yet
note-details-untitled-document = Untitled

## The In prose reading's own counter: how many times this entry is named across
## the rows on screen, and which of them the reader has stepped to. See
## `teksilo_ui::story_bible::highlight::SubjectWalk`.
note-in-prose-mentions = { $n ->
    [one] 1 mention
   *[other] { $n } mentions
}
note-in-prose-mention-at = { $current } of { $total }
note-in-prose-mention-previous = Previous mention
note-in-prose-mention-next = Next mention
