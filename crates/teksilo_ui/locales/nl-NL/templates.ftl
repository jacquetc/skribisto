# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

settings-page-templates = Sjablonen
settings-templates-description = Herbruikbare stukken tekst die u kunt invoegen in alles wat u schrijft: een leeg personageblad, een profiel van een locatie, een beat sheet. Ze worden in dit project opgeslagen, zodat iedereen die het opent dezelfde set krijgt.
settings-templates-filter = Sjablonen filteren
settings-templates-count = { $n ->
        [0] Geen sjablonen
        [one] 1 sjabloon
       *[other] { $n } sjablonen
    }
settings-templates-empty = Dit project heeft nog geen sjablonen.
settings-templates-empty-hint = Voeg er een toe uit een voorinstelling, importeer een .md- of .djot-bestand, of schrijf iets en kies Document ▸ Opslaan als sjabloon.
settings-templates-presets = Een voorinstelling toevoegen
settings-templates-import = Importeren…
settings-templates-export = Exporteren…
settings-templates-star = Bovenaan tonen in het invoegmenu
settings-templates-unstar = Niet meer bovenaan tonen
settings-templates-move-up = Omhoog verplaatsen
settings-templates-move-down = Omlaag verplaatsen
settings-templates-delete = Sjabloon verwijderen
settings-templates-name = Naam
settings-templates-name-placeholder = Sjabloonnaam
settings-templates-body-placeholder = De tekst die dit sjabloon invoegt
settings-templates-duplicate-name = Er is al een ander sjabloon met de naam “{ $name }”
settings-templates-words = { $n ->
    [one] 1 woord
   *[other] { $n } woorden
}

templates-imported = { $added ->
    [one] 1 sjabloon geïmporteerd
   *[other] { $added } sjablonen geïmporteerd
}
templates-imported-renamed = Hernoemd om een botsing te voorkomen: { $names }
templates-imported-skipped = Kan niet lezen: { $names }
templates-import-filter = Sjablonen (.md, .djot)
templates-exported = { $n ->
    [one] 1 sjabloon geëxporteerd
   *[other] { $n } sjablonen geëxporteerd
}
templates-preset-applied = “{ $name }” toegevoegd

templates-delete-title = Dit sjabloon verwijderen?
templates-delete-body = “{ $name }” wordt uit dit project verwijderd. Notities die u er al mee hebt gemaakt, blijven ongewijzigd.
templates-delete-confirm = Verwijderen

menu-document = &Document
menu-insert-template = &Sjabloon invoegen
menu-insert-template-none = Geen sjablonen in dit project
menu-save-as-template = Opslaan als s&jabloon…

save-as-template-title = Opslaan als sjabloon
save-as-template-explain = De tekst die u bewerkt, wordt een sjabloon dat u overal elders kunt invoegen.
save-as-template-name = Naam
save-as-template-placeholder = Personageblad
save-as-template-duplicate = Er bestaat al een sjabloon met de naam “{ $name }”
save-as-template-empty-editor = Deze editor is leeg. Er valt niets op te slaan.
save-as-template-confirm = Sjabloon opslaan
save-as-template-saved = “{ $name }” opgeslagen als sjabloon
template-inserted = “{ $name }” ingevoegd

note-template-preset-character-sheet = Personageblad
note-template-preset-location = Locatie
note-template-preset-artifact = Object
note-template-preset-beat-sheet = Beat sheet
note-template-preset-faction = Factie
note-template-preset-research-note = Researchnotitie

note-template-set-essentials = Basisgegevens
note-template-set-everything = Alle sjablonen

note-template-section-identity = Identiteit
note-template-section-appearance = Uiterlijk
note-template-section-voice = Stem
note-template-section-psychology = Innerlijk leven
note-template-section-history = Voorgeschiedenis
note-template-section-arc = Ontwikkelingsboog
note-template-section-first-impression = Eerste indruk
note-template-section-in-the-story = In het verhaal
note-template-section-the-thing-itself = Het object zelf
note-template-section-the-scene = De scène
note-template-section-the-shape = De vorm
note-template-section-what-it-is = Wat het is
note-template-section-source = Bron
note-template-section-what-it-says = Wat erin staat

note-template-field-full-name = Volledige naam
note-template-field-known-as = Ook bekend als (bijgehouden in het veld Ook bekend als van de Inspector)
note-template-field-age = Leeftijd
note-template-field-role-in-story = Rol in het verhaal
note-template-field-build-and-features = Lichaamsbouw en opvallende kenmerken
note-template-field-habitual-bearing = Gebruikelijke houding
note-template-field-speech-patterns = Spraakpatronen en verbale tics
note-template-field-what-they-never-say = Wat dit personage nooit zegt
note-template-field-want = Wat dit personage wil
note-template-field-need = Wat dit personage nodig heeft zonder het te weten
note-template-field-fear = Waar dit personage bang voor is
note-template-field-flaw = De zwakte die het personage iets kost
note-template-field-formative-event = Vormende gebeurtenis
note-template-field-relationships = Relaties
note-template-field-starts-as = Begint als
note-template-field-ends-as = Eindigt als
note-template-field-what-you-notice-first = Wat u het eerst opvalt
note-template-field-sound-and-smell = Geluid en geur
note-template-field-light-and-weather = Licht en weer
note-template-field-what-happened-here = Wat hier gebeurd is
note-template-field-who-lives-or-works-here = Wie hier woont of werkt
note-template-field-scenes-set-here = Scènes die zich hier afspelen
note-template-field-why-it-matters = Waarom het belangrijk is
note-template-field-appearance = Uiterlijk
note-template-field-age-and-origin = Leeftijd en herkomst
note-template-field-what-it-does = Wat het doet
note-template-field-who-holds-it = Wie het in bezit heeft
note-template-field-who-wants-it = Wie het wil hebben
note-template-field-pov = Vertelperspectief
note-template-field-time-and-place = Tijd en plaats
note-template-field-goal = Doel
note-template-field-conflict = Conflict
note-template-field-turn = De wending
note-template-field-exit-emotion = Emotie bij het einde
note-template-field-purpose = Doel en bestaansreden
note-template-field-who-leads-it = Wie het leidt
note-template-field-resources = Middelen
note-template-field-allies-and-enemies = Bondgenoten en vijanden
note-template-field-what-it-wants-now = Wat het nu wil
note-template-field-where-from = Waar het vandaan kwam
note-template-field-page-or-link = Pagina of koppeling
note-template-field-key-facts = Belangrijkste feiten
note-template-field-how-it-is-used = Hoe het wordt gebruikt in het verhaal
