# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Tags: the per-project palette, the inspector section, and the preset catalog.
# Preset tag NAMES are translated deliberately: presets are generated in code rather than
# shipped as data so that applying one lands in the language of the INTERFACE the writer is
# looking at, instead of the single language a data file would have had to pick at authoring
# time. The interface's locale and not the project's `dict_language`: a tag name is a filter
# chip and an inspector row, the compiler never reads one, and it is read only by the person
# at the keyboard. Applied rows are stored literally and are theirs to rename.

## Preset menu
tags-preset-basic = De bază
tags-preset-scifi = Științifico-fantastic
tags-preset-fantasy = Fantasy
tags-preset-mystery = Mister și polițist
tags-preset-historical = Roman istoric

## Basic preset: the workflow ladder. The `status/` prefix is a naming convention:
## alphabetical sorting alone makes these cluster together in every list.

## Basic preset: flags. Deliberately unprefixed: a scene can be a draft AND need
## research at the same time, so these are not part of the ladder above.
tags-preset-needs-research = necesită documentare
tags-preset-continuity-check = de verificat continuitatea
tags-preset-plot-point = punct de cotitură

## Basic preset: the taxonomy the mention index scans prose for.
tags-preset-character = personaj
tags-preset-place = loc
tags-preset-item = obiect

## Genre additions
tags-preset-vessel = navă
tags-preset-planet = planetă
tags-preset-organization = organizație
tags-preset-creature = creatură
tags-preset-faction = facțiune
tags-preset-artifact = artefact
tags-preset-realm = tărâm
tags-preset-magic-system = sistem magic
tags-preset-suspect = suspect
tags-preset-victim = victimă
tags-preset-clue = indiciu
tags-preset-red-herring = pistă falsă
tags-preset-historical-figure = personalitate istorică
tags-preset-source = sursă
tags-preset-period-detail = detaliu de epocă

## The tag pill field (Inspector) and its "+" picker
tags-pill-list = Etichete
tags-pill-add = Adaugă o etichetă
# The Overview's Tags cell on a row with no tags yet: the cell IS the picker, so
# it stays a target rather than rendering nothing. Names the muted "+" placeholder.
tags-none-yet = Fără etichete
# The picker's own list of pickable tags, announced apart from the
# `tags-pill-list` row of tags already assigned.
tags-pick-list = Etichete disponibile
tags-pill-remove = Elimină { $name }
tags-pill-filter-placeholder = Filtrați sau numiți o etichetă nouă
tags-pill-no-match = Nicio etichetă nu corespunde
tags-pill-create = Creează „{ $name }”
tags-pill-create-failed = „{ $name }” nu a putut fi creată
tags-pill-new-discoverable = Etichetă cu căutare în proză
tags-pill-new-discoverable-hint = Elementele cu această etichetă sunt căutate în proza dumneavoastră pentru a completa distribuția.

## The alias pill field
tags-alias-list = Cunoscut(ă) și ca
tags-alias-add = Adaugă alt nume
tags-alias-remove = Elimină { $name }
tags-alias-placeholder = Alt nume, apoi Enter
tags-alias-hint = Numele sub care apare în proză, pe lângă titlu.
tags-alias-collision = { $name } răspunde deja la acest nume.

## Settings ▸ Work ▸ Tags
settings-page-tags = Etichete
settings-tags-desc = Etichetele marchează elementele din clasor. O etichetă cu căutare în proză îi spune în plus lui Skribisto să caute numele acelui element în proza dumneavoastră.
settings-tags-add = Adaugă eticheta
settings-tags-add-placeholder = Numele noii etichete
settings-tags-added = A fost adăugată „{ $name }”
settings-tags-duplicate = „{ $name }” există deja
settings-tags-filter = Filtrează etichetele
settings-tags-count = { $n ->
       [one] O etichetă
       [few] { $n } etichete
      *[other] { $n } de etichete
   }
settings-tags-details-placeholder = Ce înseamnă această etichetă
settings-tags-discoverable = Căutare în proză
settings-tags-creates-in = Notele noi ajung în
settings-tags-creates-in-unset = Întreabă-mă prima dată
settings-tags-creates-in-untitled = Dosar fără titlu
settings-tags-creates-in-trashed = { $name } (în coșul de gunoi)
settings-tags-template = Șablon de pornire
settings-tags-template-unset = Notă goală
settings-tags-delete = Șterge { $name }
settings-tags-deleted = „{ $name }” a fost ștearsă și eliminată de pe toate elementele
settings-tags-empty = Încă nu există etichete.
settings-tags-apply-preset = Aplică un set predefinit…
settings-tags-preset-applied = Adăugate: { $added }; omise, pentru că existau deja: { $skipped }
settings-tags-csv-filter = Fișiere CSV
settings-tags-import = Importă…
settings-tags-export = Exportă…
settings-tags-imported = Importate: { $added }, omise: { $skipped }
settings-tags-exported = { $n ->
       [one] A fost exportată o etichetă
       [few] Au fost exportate { $n } etichete
      *[other] Au fost exportate { $n } de etichete
   }

## The dot row shown on the stream, corkboard and editor subtitle
tags-chip-more = { $n ->
       [one] încă o etichetă
       [few] încă { $n } etichete
      *[other] încă { $n } de etichete
   }

## Cast of the scene (references-first story-bible pins + suggestions)
cast-section = Distribuție
cast-add = Adaugă în distribuție…
cast-add-filter-placeholder = Filtrează biblia poveștii…
cast-add-empty = Nicio intrare din biblia poveștii de adăugat
cast-pin = Adaugă { $name } în distribuție
cast-unpin = Elimină { $name } din distribuție
cast-empty = Încă nu a fost adăugat nimeni. Adăugați pe cineva sau păstrați o sugestie
cast-unresolved = Nu mai este în biblia poveștii

## Backlinks on a story-bible item
# There is no `mentions-backlinks` heading any more: the Inspector's "Appears in"
# list was removed, and the entry's own Details page names its list
# `note-details-backlinks`. The keys below are the row controls, which that page
# still uses.
# The control on a suggested backlink row: the writer says yes, this really is her,
# and the entry is written into that document's cast. Named after what it does rather
# than after the agreement, and worded to mirror `cast-pin`, which is the same write
# made from the other end. Confirm only: there is no "not her" to record; see
# `teksilo_ui::mentions::presence`.
mentions-confirm = Adaugă în distribuția lui „{ $name }”
mentions-hit-count = { $n ->
       [one] o dată
       [few] de { $n } ori
      *[other] de { $n } de ori
   }
# The badge on a row whose target is the owner's declared point of view (see
# `point_of_view`, distinct from `references`). Shown on both the Cast list and the
# backlinks list, since a point of view can appear in either direction.
mentions-point-of-view-badge = Punct de vedere
mentions-point-of-view-badge-tooltip = Declarat aici ca punct de vedere, setat de mână, nu detectat în proză.

## Legacy keys kept so older scripts/tests that still reference them compile
mentions-roster = Distribuție
mentions-pin = Adaugă { $name } în distribuție

# ── Point of view ────────────────────────────────────────────────────────────
# Whose eyes a scene is told through. Distinct from the cast above: the cast is
# who appears, the point of view is who holds the camera. A scene may have none
# (unassigned) or, deliberately, more than one, which is what head-hopping is.
pov-section = Punct de vedere
pov-empty = Niciun punct de vedere setat
pov-add = Setează punctul de vedere…
pov-multiple = Această scenă are două puncte de vedere.
pov-remove = Elimină { $name } ca punct de vedere
pov-unresolved = Aici a fost fixat un punct de vedere, dar acea intrare nu mai există sau și-a pierdut eticheta de biblie a poveștii.

# ── Book filing ──────────────────────────────────────────────────────────────
# Which Book or Books a note or note folder is declared to belong to. Shown
# only on story-bible material outside the manuscript flow, since a scene's
# Book is already given by where it sits in the binder, and only once the
# project holds two or more Books; a one-Book project has nothing to file
# against.
books-section = Clasat la
books-empty = Încă nu este clasat la nicio carte
books-add = Clasează la o carte…
books-remove = Elimină din { $name }
books-apply-to-children = Aplică clasarea și elementelor conținute

# ── The Details segment on an `Item/Note` tab ────────────────────────────────
# Almost the same field set as the Inspector's own story-bible section (tags,
# aliases, cast, point of view, books), reused deliberately, but under its own
# `note-details-` keys: this segment is a full-width page, not a dock, so its
# copy is free to say a little more than the dock's narrower strings do. See
# `teksilo_ui::tabs::note_details`.
note-details-name-placeholder = Nume…
note-details-tags = Etichete
note-details-aliases = Alte nume
note-details-books = Clasat la
note-details-books-empty = Încă nu este clasat la nicio carte
note-details-links = Legături
note-details-links-empty = Încă nu există legături. Adăugați una pentru a trimite la altă intrare din biblia poveștii.
note-details-links-add = Leagă de o notă…
note-details-pov = Punct de vedere
note-details-pov-empty = Niciun punct de vedere setat
note-details-pov-unresolved = Aici a fost fixat un punct de vedere, dar acea intrare nu mai există sau și-a pierdut eticheta de biblie a poveștii.
note-details-pov-multiple = Această notă are două puncte de vedere.
note-details-backlinks = Apare în manuscris
note-details-backlinks-empty = Încă nimic. Când acest nume va apărea în proza dumneavoastră, va fi afișat aici.
note-details-backlinks-outside = În afara cărților
note-details-backlinks-confirm-all = Confirmă toate aparițiile
note-details-backlinks-confirm-all-tooltip = Adaugă această intrare în distribuția fiecărui document afișat aici care nu a confirmat-o încă
note-details-untitled-document = Fără titlu

## The In prose reading's own counter: how many times this entry is named across
## the rows on screen, and which of them the reader has stepped to. See
## `teksilo_ui::story_bible::highlight::SubjectWalk`.
note-in-prose-mentions = { $n ->
       [one] O mențiune
       [few] { $n } mențiuni
      *[other] { $n } de mențiuni
   }
note-in-prose-mention-at = { $current } din { $total }
note-in-prose-mention-previous = Mențiunea precedentă
note-in-prose-mention-next = Mențiunea următoare
