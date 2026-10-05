# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

settings-page-templates = Maler
settings-templates-description = Tekster du kan bruke om igjen og sette inn i hva som helst du skriver: et tomt personskjema, en stedsprofil, en scenedisposisjon. De lagres i dette prosjektet, så alle som åpner det, får de samme malene.
settings-templates-filter = Filtrer maler
settings-templates-count = { $n ->
    [0] Ingen maler
    [one] 1 mal
   *[other] { $n } maler
}
settings-templates-empty = Dette prosjektet har ingen maler ennå.
settings-templates-empty-hint = Legg til en fra en forhåndsinnstilling, importer en .md- eller .djot-fil, eller skriv noe og velg Dokument ▸ Lagre som mal.
settings-templates-presets = Legg til en forhåndsinnstilling
settings-templates-import = Importer…
settings-templates-export = Eksporter…
settings-templates-star = Vis først i Sett inn-menyen
settings-templates-unstar = Ikke vis først lenger
settings-templates-move-up = Flytt opp
settings-templates-move-down = Flytt ned
settings-templates-delete = Slett mal
settings-templates-name = Navn
settings-templates-name-placeholder = Navn på mal
settings-templates-body-placeholder = Teksten denne malen setter inn
settings-templates-duplicate-name = En annen mal heter allerede «{ $name }»
settings-templates-words = { $n ->
    [one] 1 ord
   *[other] { $n } ord
}

templates-imported = { $added ->
    [one] 1 mal importert
   *[other] { $added } maler importert
}
templates-imported-renamed = Fikk nytt navn for å unngå navnekonflikt: { $names }
templates-imported-skipped = Kunne ikke lese: { $names }
templates-import-filter = Maler (.md, .djot)
templates-exported = { $n ->
    [one] 1 mal eksportert
   *[other] { $n } maler eksportert
}
templates-preset-applied = La til «{ $name }»

templates-delete-title = Vil du slette denne malen?
templates-delete-body = «{ $name }» fjernes fra dette prosjektet. Notater du allerede har opprettet fra den, påvirkes ikke.
templates-delete-confirm = Slett

menu-document = &Dokument
menu-insert-template = Sett inn &mal
menu-insert-template-none = Ingen maler i dette prosjektet
menu-save-as-template = &Lagre som mal…

save-as-template-title = Lagre som mal
save-as-template-explain = Teksten du redigerer, blir en mal du kan sette inn hvor som helst ellers.
save-as-template-name = Navn
save-as-template-placeholder = Personskjema
save-as-template-duplicate = Det finnes allerede en mal som heter «{ $name }»
save-as-template-empty-editor = Dette redigeringsfeltet er tomt. Det er ingenting å lagre.
save-as-template-confirm = Lagre mal
save-as-template-saved = Lagret «{ $name }» som mal
template-inserted = Satte inn «{ $name }»

note-template-preset-character-sheet = Personskjema
note-template-preset-location = Sted
note-template-preset-artifact = Gjenstand
note-template-preset-beat-sheet = Scenedisposisjon
note-template-preset-faction = Fraksjon
note-template-preset-research-note = Researchnotat

note-template-set-essentials = Det viktigste
note-template-set-everything = Alle maler

note-template-section-identity = Identitet
note-template-section-appearance = Utseende
note-template-section-voice = Stemme
note-template-section-psychology = Indre liv
note-template-section-history = Forhistorie
note-template-section-arc = Utviklingskurve
note-template-section-first-impression = Førsteinntrykk
note-template-section-in-the-story = I historien
note-template-section-the-thing-itself = Selve gjenstanden
note-template-section-the-scene = Scenen
note-template-section-the-shape = Formen
note-template-section-what-it-is = Hva det er
note-template-section-source = Kilde
note-template-section-what-it-says = Hva det sier

note-template-field-full-name = Fullt navn
note-template-field-known-as = Også kjent som, registrert i feltet Også kjent som i inspektøren
note-template-field-age = Alder
note-template-field-role-in-story = Rolle i historien
note-template-field-build-and-features = Kroppsbygning og kjennetegn
note-template-field-habitual-bearing = Vanlig holdning
note-template-field-speech-patterns = Talemåte og verbale tics
note-template-field-what-they-never-say = Det personen aldri sier
note-template-field-want = Det personen vil ha
note-template-field-need = Det personen trenger, uten å vite det
note-template-field-fear = Det personen frykter
note-template-field-flaw = Svakheten som koster personen dyrt
note-template-field-formative-event = Avgjørende hendelse
note-template-field-relationships = Relasjoner
note-template-field-starts-as = Begynner som
note-template-field-ends-as = Ender som
note-template-field-what-you-notice-first = Det du legger merke til først
note-template-field-sound-and-smell = Lyd og lukt
note-template-field-light-and-weather = Lys og vær
note-template-field-what-happened-here = Hva som skjedde her
note-template-field-who-lives-or-works-here = Hvem som bor eller arbeider her
note-template-field-scenes-set-here = Scener som foregår her
note-template-field-why-it-matters = Hvorfor det er viktig
note-template-field-appearance = Utseende
note-template-field-age-and-origin = Alder og opprinnelse
note-template-field-what-it-does = Hva den gjør
note-template-field-who-holds-it = Hvem som har den
note-template-field-who-wants-it = Hvem som vil ha den
note-template-field-pov = Synsvinkel
note-template-field-time-and-place = Tid og sted
note-template-field-goal = Mål
note-template-field-conflict = Konflikt
note-template-field-turn = Vendingen
note-template-field-exit-emotion = Følelsen ved utgangen
note-template-field-purpose = Formål
note-template-field-who-leads-it = Hvem som leder den
note-template-field-resources = Ressurser
note-template-field-allies-and-enemies = Allierte og fiender
note-template-field-what-it-wants-now = Hva den vil nå
note-template-field-where-from = Hvor det kommer fra
note-template-field-page-or-link = Side eller lenke
note-template-field-key-facts = Viktige fakta
note-template-field-how-it-is-used = Hvordan det brukes i historien
