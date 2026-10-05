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
settings-page-templates = Skabeloner
settings-templates-description = Genbrugelige tekststykker, du kan indsætte i alt, hvad du skriver: et tomt personark, en stedbeskrivelse, en beat sheet. De gemmes i dette projekt, så alle, der åbner det, får de samme.
settings-templates-filter = Filtrér skabeloner
settings-templates-count = { $n ->
    [0] Ingen skabeloner
    [one] 1 skabelon
   *[other] { $n } skabeloner
}
settings-templates-empty = Projektet har endnu ingen skabeloner.
settings-templates-empty-hint = Tilføj en fra en forudindstilling, importér en .md- eller .djot-fil, eller skriv noget og vælg Dokument ▸ Gem som skabelon.
settings-templates-presets = Tilføj en forudindstilling
settings-templates-import = Importér…
settings-templates-export = Eksportér…
settings-templates-star = Vis først i menuen Indsæt
settings-templates-unstar = Vis ikke længere først
settings-templates-move-up = Flyt op
settings-templates-move-down = Flyt ned
settings-templates-delete = Slet skabelon
settings-templates-name = Navn
settings-templates-name-placeholder = Skabelonens navn
settings-templates-body-placeholder = Den tekst, skabelonen indsætter
settings-templates-duplicate-name = Der findes allerede en anden skabelon med navnet »{ $name }«
settings-templates-words = { $n ->
    [one] 1 ord
   *[other] { $n } ord
}

## Import / export feedback
templates-imported = { $added ->
    [one] 1 skabelon er importeret
   *[other] { $added } skabeloner er importeret
}
templates-imported-renamed = Omdøbt for at undgå navnesammenfald: { $names }
templates-imported-skipped = Kunne ikke læses: { $names }
templates-import-filter = Skabeloner (.md, .djot)
templates-exported = { $n ->
    [one] 1 skabelon er eksporteret
   *[other] { $n } skabeloner er eksporteret
}
templates-preset-applied = »{ $name }« er tilføjet

## Delete confirmation
templates-delete-title = Slet denne skabelon?
templates-delete-body = »{ $name }« fjernes fra projektet. Noter, du allerede har oprettet ud fra den, berøres ikke.
templates-delete-confirm = Slet

## Document menu
menu-document = &Dokument
menu-insert-template = Indsæt &skabelon
menu-insert-template-none = Ingen skabeloner i dette projekt
menu-save-as-template = Gem som skabe&lon…

## Save as template
save-as-template-title = Gem som skabelon
save-as-template-explain = Den tekst, du redigerer, bliver en skabelon, du kan indsætte alle andre steder.
save-as-template-name = Navn
save-as-template-placeholder = Personark
save-as-template-duplicate = Der findes allerede en skabelon med navnet »{ $name }«
save-as-template-empty-editor = Editoren er tom. Der er intet at gemme.
save-as-template-confirm = Gem skabelon
save-as-template-saved = »{ $name }« er gemt som skabelon
template-inserted = »{ $name }« er indsat

## Preset names
note-template-preset-character-sheet = Personark
note-template-preset-location = Sted
note-template-preset-artifact = Genstand
note-template-preset-beat-sheet = Beat sheet
note-template-preset-faction = Fraktion
note-template-preset-research-note = Researchnote

## Starter sets, offered when a project is created
note-template-set-essentials = Det vigtigste
note-template-set-everything = Alle skabeloner

## Preset section headings
note-template-section-identity = Identitet
note-template-section-appearance = Udseende
note-template-section-voice = Stemme
note-template-section-psychology = Indre liv
note-template-section-history = Forhistorie
note-template-section-arc = Udvikling
note-template-section-first-impression = Første indtryk
note-template-section-in-the-story = I historien
note-template-section-the-thing-itself = Selve genstanden
note-template-section-the-scene = Scenen
note-template-section-the-shape = Formen
note-template-section-what-it-is = Hvad det er
note-template-section-source = Kilde
note-template-section-what-it-says = Hvad den siger

## Preset field prompts
note-template-field-full-name = Fulde navn
note-template-field-known-as = Også kendt som, registreret i inspektørens felt Også kendt som
note-template-field-age = Alder
note-template-field-role-in-story = Rolle i historien
note-template-field-build-and-features = Kropsbygning og særlige kendetegn
note-template-field-habitual-bearing = Sædvanlig holdning og fremtoning
note-template-field-speech-patterns = Talemåde og sproglige særheder
note-template-field-what-they-never-say = Hvad personen aldrig siger
note-template-field-want = Hvad personen vil have
note-template-field-need = Hvad personen har brug for uden at vide det
note-template-field-fear = Hvad personen frygter
note-template-field-flaw = Den brist, der koster personen dyrt
note-template-field-formative-event = Afgørende begivenhed
note-template-field-relationships = Relationer
note-template-field-starts-as = Begynder som
note-template-field-ends-as = Ender som
note-template-field-what-you-notice-first = Hvad man først lægger mærke til
note-template-field-sound-and-smell = Lyde og lugte
note-template-field-light-and-weather = Lys og vejr
note-template-field-what-happened-here = Hvad der er sket her
note-template-field-who-lives-or-works-here = Hvem der bor eller arbejder her
note-template-field-scenes-set-here = Scener, der foregår her
note-template-field-why-it-matters = Hvorfor det er vigtigt
note-template-field-appearance = Udseende
note-template-field-age-and-origin = Alder og oprindelse
note-template-field-what-it-does = Hvad den gør
note-template-field-who-holds-it = Hvem der har den
note-template-field-who-wants-it = Hvem der vil have den
note-template-field-pov = Synsvinkel
note-template-field-time-and-place = Tid og sted
note-template-field-goal = Mål
note-template-field-conflict = Konflikt
note-template-field-turn = Vendepunktet
note-template-field-exit-emotion = Stemningen ved scenens slutning
note-template-field-purpose = Formål
note-template-field-who-leads-it = Hvem der leder den
note-template-field-resources = Ressourcer
note-template-field-allies-and-enemies = Allierede og fjender
note-template-field-what-it-wants-now = Hvad den vil nu
note-template-field-where-from = Hvor den kommer fra
note-template-field-page-or-link = Side eller link
note-template-field-key-facts = Vigtigste fakta
note-template-field-how-it-is-used = Hvordan den bruges i historien
