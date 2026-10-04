# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

settings-page-templates = Skabeloner
settings-templates-description = Genbrugelige stykker af skrive kan du falde ind i noget, du skriver: en blank karakter ark, en placering profil, et beat ark. De er gemt i dette projekt, så alle, der åbner det får det samme sæt.
settings-templates-filter = Filterskabeloner
settings-templates-count = { $n ->
    [0] Ingen skabeloner
    [one] 1 skabelon
   *[other] { $n } skabeloner
}
settings-templates-empty = Dette projekt har ingen skabeloner endnu.
settings-templates-empty-hint = Tilføj en fra et forudindstillet, importere en .md eller .djot fil, eller skrive noget og vælg Document Note Gem som skabelon.
settings-templates-presets = Tilføj et forudindstillet
settings-templates-import = Importér…
settings-templates-export = Eksportér…
settings-templates-star = Vis først i menuen Indsæt
settings-templates-unstar = Stop med at vise først
settings-templates-move-up = Flyt op
settings-templates-move-down = Flyt ned
settings-templates-delete = Slet skabelon
settings-templates-name = Navn
settings-templates-name-placeholder = Skabelonnavn
settings-templates-body-placeholder = Teksten denne skabelon indsætter
settings-templates-duplicate-name = En anden skabelon kaldes allerede { $name }
settings-templates-words = { $n ->
    [one] 1 ord
   *[other] { $n } ord
}

templates-imported = { $added ->
    [one] 1 skabelon importeret
   *[other] { $added } importerede skabeloner
}
templates-imported-renamed = Omdøbt for at undgå sammenstød: { $names }
templates-imported-skipped = Kunne ikke læse: { $names }
templates-import-filter = Skabeloner (.md, .djot)
templates-exported = { $n ->
    [one] 1 Exported skabelon
   *[other] { $n } eksporterede skabeloner
}
templates-preset-applied = Tilføjet: { $name }

templates-delete-title = Slet denne skabelon?
templates-delete-body = "{ $name }Bemærk, at du allerede er skabt fra det er ikke påvirket.
templates-delete-confirm = Slet

menu-document = Dokument (&D)
menu-insert-template = Indsæt skabelon (&L)
menu-insert-template-none = Ingen skabeloner i dette projekt
menu-save-as-template = Gem som skabelon (&V)

save-as-template-title = Gem som skabelon
save-as-template-explain = Den tekst du redigerer bliver en skabelon du kan falde i hvor som helst andet.
save-as-template-name = Navn
save-as-template-placeholder = Tegneark
save-as-template-duplicate = Der findes allerede en skabelon ved navn MD{ $name }
save-as-template-empty-editor = Denne editor er tom. Der er intet at gemme.
save-as-template-confirm = Gem skabelon
save-as-template-saved = Gemt i { $name } som skabelon
template-inserted = Opsat i { $name }

note-template-preset-character-sheet = Tegneark
note-template-preset-location = Sted
note-template-preset-artifact = Objekt
note-template-preset-beat-sheet = Beat-ark
note-template-preset-faction = Funktion
note-template-preset-research-note = Forskningsnotat

note-template-set-essentials = Grundoplysninger
note-template-set-everything = Hver skabelon

note-template-section-identity = Identitet
note-template-section-appearance = Udseende
note-template-section-voice = Stemme
note-template-section-psychology = Indre liv
note-template-section-history = Historie
note-template-section-arc = Udviklingskurve
note-template-section-first-impression = Første indtryk
note-template-section-in-the-story = I historien
note-template-section-the-thing-itself = Objektet selv
note-template-section-the-scene = Scenen
note-template-section-the-shape = Formen
note-template-section-what-it-is = Hvad er det?
note-template-section-source = Kilde
note-template-section-what-it-says = Hvad står der?

note-template-field-full-name = Fuldt navn
note-template-field-known-as = Også kendt som, sporet i Inspector 's Alias felt
note-template-field-age = Alder
note-template-field-role-in-story = Rolle i historien
note-template-field-build-and-features = Bygge og skelne funktioner
note-template-field-habitual-bearing = Habitatleje
note-template-field-speech-patterns = Talemønstre og verbalteknik
note-template-field-what-they-never-say = Hvad de aldrig siger
note-template-field-want = Hvad de vil have
note-template-field-need = Hvad de har brug for, men ikke ved det
note-template-field-fear = Hvad de frygter
note-template-field-flaw = Fejlen, der koster dem
note-template-field-formative-event = Formelle begivenhed
note-template-field-relationships = Forhold
note-template-field-starts-as = Starter som
note-template-field-ends-as = Ender som
note-template-field-what-you-notice-first = Hvad du bemærker først
note-template-field-sound-and-smell = Lyd og lugt
note-template-field-light-and-weather = Lys og vejr
note-template-field-what-happened-here = Hvad er der sket her?
note-template-field-who-lives-or-works-here = Hvem bor eller projekter her
note-template-field-scenes-set-here = Scener sat her
note-template-field-why-it-matters = Hvorfor det betyder noget
note-template-field-appearance = Udseende
note-template-field-age-and-origin = Alder og oprindelse
note-template-field-what-it-does = Hvad det gør
note-template-field-who-holds-it = Hvem har den?
note-template-field-who-wants-it = Hvem vil have det?
note-template-field-pov = Synsvinkel
note-template-field-time-and-place = Tid og sted
note-template-field-goal = Mål
note-template-field-conflict = Konflikt
note-template-field-turn = Drej
note-template-field-exit-emotion = Følelse på vej ud
note-template-field-purpose = Formål
note-template-field-who-leads-it = Hvem leder det?
note-template-field-resources = Ressourcer
note-template-field-allies-and-enemies = Allierede og fjender
note-template-field-what-it-wants-now = Hvad vil den nu?
note-template-field-where-from = Hvor det kom fra
note-template-field-page-or-link = Side eller link
note-template-field-key-facts = Vigtigste kendsgerninger
note-template-field-how-it-is-used = Hvordan det bruges i historien
