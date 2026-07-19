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
