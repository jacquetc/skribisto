# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

settings-page-templates = Maler
settings-templates-description = Reusable stykke skriving kan du slippe inn i alt du skriver: et tomt tegn ark, en plassering profil, et beat ark. De lagres i dette prosjektet, slik at alle som åpner det får det samme settet.
settings-templates-filter = Filtrer maler
settings-templates-count = { $n ->
    [0] Ingen maler
    [one] 1 mal
   *[other] { $n } maler
}
settings-templates-empty = Dette prosjektet har ingen maler ennå.
settings-templates-empty-hint = Legg til en fra en forhåndsinnstilling, importer en .md eller .djot-fil, eller skriv noe og velg Dokument - Lagre som mal.
settings-templates-presets = Legg til en forhåndsinnstilling
settings-templates-import = Importer…
settings-templates-export = Eksporter…
settings-templates-star = Vis først i innskuddsmenyen
settings-templates-unstar = Slutt å vise først
settings-templates-move-up = Flytt opp
settings-templates-move-down = Flytt ned
settings-templates-delete = Slett mal
settings-templates-name = Navn
settings-templates-name-placeholder = Malnavn
settings-templates-body-placeholder = Teksten denne malen setter inn
settings-templates-duplicate-name = En annen mal kalles allerede -{ $name }"
settings-templates-words = { $n ->
    [one] 1 ord
   *[other] { $n } ord
}

templates-imported = { $added ->
    [one] 1 mal importert
   *[other] { $added } maler importert
}
templates-imported-renamed = Omdøpt for å unngå et sammenstøt: { $names }
templates-imported-skipped = Kunne ikke lese: { $names }
templates-import-filter = Maler (.md, .djot)
templates-exported = { $n ->
    [one] 1 mal eksportert
   *[other] { $n } maler eksportert
}
templates-preset-applied = Lagt til{ $name }"

templates-delete-title = Slett denne malen?
templates-delete-body = "{ $name }- vil bli fjernet fra dette prosjektet. Noterer du allerede opprettet fra det er ikke påvirket.
templates-delete-confirm = Slett

menu-document = Dokument (&D)
menu-insert-template = Sett inn mal (&L)
menu-insert-template-none = Ingen maler i dette prosjektet
menu-save-as-template = Lagre som mal... (&V)

save-as-template-title = Lagre som mal
save-as-template-explain = Teksten du redigerer blir en mal som du kan slippe inn hvor som helst.
save-as-template-name = Navn
save-as-template-placeholder = Tegneark
save-as-template-duplicate = En mal som heter -{ $name }- allerede eksisterer
save-as-template-empty-editor = Denne redaktøren er tom. Det er ingenting å redde.
save-as-template-confirm = Lagre mal
save-as-template-saved = Reddet -{ $name } som en mal
template-inserted = Innsatt -{ $name }"

note-template-preset-character-sheet = Tegneark
note-template-preset-location = Beliggenhet
note-template-preset-artifact = Objekt
note-template-preset-beat-sheet = Beat ark
note-template-preset-faction = Fraksjon
note-template-preset-research-note = Forskningsnotat

note-template-set-essentials = Essensials
note-template-set-everything = Hver mal

note-template-section-identity = Identifikasjon
note-template-section-appearance = Utseende
note-template-section-voice = Stemme
note-template-section-psychology = Indre liv
note-template-section-history = Historien
note-template-section-arc = Utviklingskurve
note-template-section-first-impression = Første inntrykk
note-template-section-in-the-story = I historien
note-template-section-the-thing-itself = Selve objektet
note-template-section-the-scene = Scenen
note-template-section-the-shape = Formen
note-template-section-what-it-is = Hva det er
note-template-section-source = Kilde
note-template-section-what-it-says = Hva det sier

note-template-field-full-name = Fullt navn
note-template-field-known-as = Også kjent som, sporet i Inspectors Alias-felt
note-template-field-age = Alder
note-template-field-role-in-story = Rolle i historien
note-template-field-build-and-features = Bygge og skille funksjoner
note-template-field-habitual-bearing = Habituell lager
note-template-field-speech-patterns = Talemønstre og verbale tics
note-template-field-what-they-never-say = Hva de aldri sier
note-template-field-want = Hva de vil ha
note-template-field-need = Hva de trenger, men ikke vet det
note-template-field-fear = Hva de frykter
note-template-field-flaw = Feilen som koster dem
note-template-field-formative-event = Formativ hendelse
note-template-field-relationships = Relasjoner
note-template-field-starts-as = Starter som
note-template-field-ends-as = Slutter som
note-template-field-what-you-notice-first = Hva du legger merke til først
note-template-field-sound-and-smell = Lyd og lukt
note-template-field-light-and-weather = Lett og vær
note-template-field-what-happened-here = Hva som skjedde her
note-template-field-who-lives-or-works-here = Hvem bor eller prosjekter her
note-template-field-scenes-set-here = Scenes satt her
note-template-field-why-it-matters = Hvorfor det betyr noe
note-template-field-appearance = Utseende
note-template-field-age-and-origin = Alder og opprinnelse
note-template-field-what-it-does = Hva det gjør
note-template-field-who-holds-it = Hvem har den
note-template-field-who-wants-it = Hvem vil ha det
note-template-field-pov = Synsvinkel
note-template-field-time-and-place = Tid og sted
note-template-field-goal = Mål
note-template-field-conflict = konflikt
note-template-field-turn = Turen
note-template-field-exit-emotion = Følelse på veien ut
note-template-field-purpose = Formål
note-template-field-who-leads-it = Hvem leder det?
note-template-field-resources = Ressurser
note-template-field-allies-and-enemies = allierte og fiender
note-template-field-what-it-wants-now = Hva det vil ha nå
note-template-field-where-from = Hvor det kom fra
note-template-field-page-or-link = Side eller kobling
note-template-field-key-facts = Nøkkelfakta
note-template-field-how-it-is-used = Hvordan det brukes i historien
