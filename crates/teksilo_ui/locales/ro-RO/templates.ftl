# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Note templates: the per-project catalog, its settings pane, the Document menu's
# insert submenu, and the built-in presets.
#
# Preset NAMES and the section/field labels their bodies are built from are translated
# deliberately: presets are assembled in code rather than shipped as `.djot` assets so that
# applying one lands in the language of the INTERFACE in use at that moment, and so
# improving the English wording later reaches every locale instead of being frozen into
# whatever projects had already applied it. The interface's locale and not the project's:
# a template is a form the writer fills in and then overwrites, and once applied its rows
# are stored literally and are theirs to edit.

## Settings ▸ Work ▸ Templates
settings-page-templates = Șabloane
settings-templates-description = Texte refolosibile pe care le puteți insera în orice scrieți: o fișă de personaj goală, un profil de loc, o schemă de ritm narativ. Sunt stocate în acest proiect, așa că oricine îl deschide are același set.
settings-templates-filter = Filtrează șabloanele
settings-templates-count = { $n ->
       [0] Niciun șablon
       [one] Un șablon
       [few] { $n } șabloane
      *[other] { $n } de șabloane
   }
settings-templates-empty = Acest proiect nu are încă șabloane.
settings-templates-empty-hint = Adăugați unul dintr-un model inclus, importați un fișier .md sau .djot ori scrieți ceva și alegeți Document ▸ Salvează ca șablon.
settings-templates-presets = Adaugă un model inclus
settings-templates-import = Importă…
settings-templates-export = Exportă…
settings-templates-star = Afișează primul în meniul de inserare
settings-templates-unstar = Nu îl mai afișa primul
settings-templates-move-up = Mută în sus
settings-templates-move-down = Mută în jos
settings-templates-delete = Șterge șablonul
settings-templates-name = Nume
settings-templates-name-placeholder = Numele șablonului
settings-templates-body-placeholder = Textul pe care îl inserează acest șablon
settings-templates-duplicate-name = Există deja un alt șablon numit „{ $name }”
settings-templates-words = { $n ->
       [one] Un cuvânt
       [few] { $n } cuvinte
      *[other] { $n } de cuvinte
   }

## Import / export feedback
templates-imported = { $added ->
       [one] Un șablon importat
       [few] { $added } șabloane importate
      *[other] { $added } de șabloane importate
   }
templates-imported-renamed = Redenumite pentru a evita un conflict: { $names }
templates-imported-skipped = Nu au putut fi citite: { $names }
templates-import-filter = Șabloane (.md, .djot)
templates-exported = { $n ->
       [one] Un șablon exportat
       [few] { $n } șabloane exportate
      *[other] { $n } de șabloane exportate
   }
templates-preset-applied = A fost adăugat „{ $name }”

## Delete confirmation
templates-delete-title = Ștergeți acest șablon?
templates-delete-body = „{ $name }” va fi eliminat din acest proiect. Notele pe care le-ați creat deja din el nu sunt afectate.
templates-delete-confirm = Șterge

## Document menu
menu-document = &Document
menu-insert-template = Inserează ș&ablon
menu-insert-template-none = Niciun șablon în acest proiect
menu-save-as-template = Salvează ca șa&blon…

## Save as template
save-as-template-title = Salvare ca șablon
save-as-template-explain = Textul pe care îl editați devine un șablon pe care îl puteți insera oriunde altundeva.
save-as-template-name = Nume
save-as-template-placeholder = Fișă de personaj
save-as-template-duplicate = Există deja un șablon numit „{ $name }”
save-as-template-empty-editor = Acest editor este gol. Nu există nimic de salvat.
save-as-template-confirm = Salvează șablonul
save-as-template-saved = „{ $name }” a fost salvat ca șablon
template-inserted = A fost inserat „{ $name }”

## Preset names
note-template-preset-character-sheet = Fișă de personaj
note-template-preset-location = Loc
note-template-preset-artifact = Obiect
note-template-preset-beat-sheet = Schemă de ritm narativ
note-template-preset-faction = Facțiune
note-template-preset-research-note = Notă de documentare

## Starter sets, offered when a project is created
note-template-set-essentials = Esențiale
note-template-set-everything = Toate șabloanele

## Preset section headings
note-template-section-identity = Identitate
note-template-section-appearance = Înfățișare
note-template-section-voice = Voce
note-template-section-psychology = Viață interioară
note-template-section-history = Trecut
note-template-section-arc = Arc narativ
note-template-section-first-impression = Prima impresie
note-template-section-in-the-story = În poveste
note-template-section-the-thing-itself = Obiectul în sine
note-template-section-the-scene = Scena
note-template-section-the-shape = Structura
note-template-section-what-it-is = Ce este
note-template-section-source = Sursa
note-template-section-what-it-says = Ce spune

## Preset field prompts
note-template-field-full-name = Numele complet
note-template-field-known-as = Cunoscut(ă) și ca (urmărit în câmpul de nume alternative din inspector)
note-template-field-age = Vârsta
note-template-field-role-in-story = Rolul în poveste
note-template-field-build-and-features = Constituție și trăsături distinctive
note-template-field-habitual-bearing = Ținuta obișnuită
note-template-field-speech-patterns = Felul de a vorbi și ticuri verbale
note-template-field-what-they-never-say = Ce nu spune niciodată
note-template-field-want = Ce vrea
note-template-field-need = De ce are nevoie fără să știe
note-template-field-fear = De ce se teme
note-template-field-flaw = Defectul care îi aduce pierderi
note-template-field-formative-event = Evenimentul formator
note-template-field-relationships = Relații
note-template-field-starts-as = La început
note-template-field-ends-as = La sfârșit
note-template-field-what-you-notice-first = Ce observi mai întâi
note-template-field-sound-and-smell = Sunete și mirosuri
note-template-field-light-and-weather = Lumină și vreme
note-template-field-what-happened-here = Ce s-a întâmplat aici
note-template-field-who-lives-or-works-here = Cine locuiește sau lucrează aici
note-template-field-scenes-set-here = Scene care se petrec aici
note-template-field-why-it-matters = De ce contează
note-template-field-appearance = Înfățișare
note-template-field-age-and-origin = Vechime și origine
note-template-field-what-it-does = Ce face
note-template-field-who-holds-it = Cine îl deține
note-template-field-who-wants-it = Cine îl vrea
note-template-field-pov = Punct de vedere
note-template-field-time-and-place = Timp și loc
note-template-field-goal = Obiectiv
note-template-field-conflict = Conflict
note-template-field-turn = Întorsătura
note-template-field-exit-emotion = Emoția de la final
note-template-field-purpose = Scop
note-template-field-who-leads-it = Cine o conduce
note-template-field-resources = Resurse
note-template-field-allies-and-enemies = Aliați și dușmani
note-template-field-what-it-wants-now = Ce vrea acum
note-template-field-where-from = De unde provine
note-template-field-page-or-link = Pagină sau legătură
note-template-field-key-facts = Fapte esențiale
note-template-field-how-it-is-used = Cum este folosit în poveste
