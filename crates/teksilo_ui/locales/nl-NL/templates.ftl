# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

settings-page-templates = Sjablonen
settings-templates-description = Herbruikbare stukken schrijven kunt u vallen in alles wat u schrijft: een blanco tekensheet, een locatieprofiel, een beat sheet. Ze zijn opgeslagen in dit project, zodat iedereen die het opent krijgt dezelfde set.
settings-templates-filter = Filtersjablonen
settings-templates-count = { $n ->
    [0] Geen sjablonen
    [one] 1 template
   *[other] { $n } templates
}
settings-templates-empty = Dit project heeft nog geen sjablonen.
settings-templates-empty-hint = Voeg er een toe vanaf een preset, importeer een .md of .djot bestand, of schrijf iets en kies Document op te slaan als sjabloon.
settings-templates-presets = Een voorinstelling toevoegen
settings-templates-import = Importeren…
settings-templates-export = Exporteren…
settings-templates-star = Eerste tonen in het menu invoegen
settings-templates-unstar = Eerst stoppen met tonen
settings-templates-move-up = Omhoog verplaatsen
settings-templates-move-down = Omlaag verplaatsen
settings-templates-delete = Sjabloon verwijderen
settings-templates-name = Naam
settings-templates-name-placeholder = Sjabloonnaam
settings-templates-body-placeholder = De tekst die dit sjabloon invoegt
settings-templates-duplicate-name = Een ander sjabloon heet al "{ $name }"
settings-templates-words = { $n ->
    [one] 1 woord
   *[other] { $n } woorden
}

templates-imported = { $added ->
    [one] 1 sjabloon geïmporteerd
   *[other] { $added } sjablonen geïmporteerd
}
templates-imported-renamed = Hernoemd om een botsing te voorkomen: { $names }
templates-imported-skipped = Kon niet lezen: { $names }
templates-import-filter = Sjablonen (.md, .djot)
templates-exported = { $n ->
    [one] 1 sjabloon geëxporteerd
   *[other] { $n } sjablonen geëxporteerd
}
templates-preset-applied = Toegevoegd "{ $name }"

templates-delete-title = Dit sjabloon verwijderen?
templates-delete-body = "{ $name }" zal uit dit project verwijderd worden. Notes die u al gemaakt hebt worden niet beïnvloed.
templates-delete-confirm = Verwijderen

menu-document = Document (&D)
menu-insert-template = Sjabloon toevoegen (&L)
menu-insert-template-none = Geen sjablonen in dit project
menu-save-as-template = Sjabloon opslaan... (&V)

save-as-template-title = Opslaan als sjabloon
save-as-template-explain = De tekst die je aan het bewerken bent wordt een sjabloon die je overal anders kunt neerzetten.
save-as-template-name = Naam
save-as-template-placeholder = Tekenblad
save-as-template-duplicate = Een template genaamd "{ $name }" bestaat al
save-as-template-empty-editor = Deze editor is leeg. Er valt niets op te slaan.
save-as-template-confirm = Sjabloon opslaan
save-as-template-saved = Opgeslagen "{ $name }" als sjabloon
template-inserted = Ingevoegd "{ $name }"

note-template-preset-character-sheet = Tekenblad
note-template-preset-location = Locatie
note-template-preset-artifact = Object
note-template-preset-beat-sheet = Beat sheet
note-template-preset-faction = Factie
note-template-preset-research-note = Onderzoeksnota

note-template-set-essentials = Basisgegevens
note-template-set-everything = Elke template

note-template-section-identity = Identiteit
note-template-section-appearance = Uiterlijk
note-template-section-voice = Stem
note-template-section-psychology = Innerlijk leven
note-template-section-history = Historie
note-template-section-arc = Ontwikkelingsboog
note-template-section-first-impression = Eerste indruk
note-template-section-in-the-story = In het verhaal
note-template-section-the-thing-itself = Het object zelf
note-template-section-the-scene = De scène
note-template-section-the-shape = De vorm
note-template-section-what-it-is = Wat het is.
note-template-section-source = Bron
note-template-section-what-it-says = Wat er staat.

note-template-field-full-name = Volledige naam
note-template-field-known-as = Ook bekend als, getraceerd in het Alias veld van de inspecteur
note-template-field-age = Leeftijd
note-template-field-role-in-story = Rol in het verhaal
note-template-field-build-and-features = Bouwen en onderscheidende kenmerken
note-template-field-habitual-bearing = Habitatieve lagers
note-template-field-speech-patterns = Spraakpatronen en verbale tics
note-template-field-what-they-never-say = Wat ze nooit zeggen.
note-template-field-want = Wat ze willen.
note-template-field-need = Wat ze nodig hebben, maar het niet weten.
note-template-field-fear = Waar ze bang voor zijn
note-template-field-flaw = De fout die hen kost
note-template-field-formative-event = Formatieve gebeurtenis
note-template-field-relationships = Relaties
note-template-field-starts-as = Begint als
note-template-field-ends-as = Eindigt als
note-template-field-what-you-notice-first = Wat je het eerst opmerkt.
note-template-field-sound-and-smell = Geluid en geur
note-template-field-light-and-weather = Licht en weer
note-template-field-what-happened-here = Wat is hier gebeurd?
note-template-field-who-lives-or-works-here = Wie woont hier of projecteert hier?
note-template-field-scenes-set-here = Scènes zijn hier ingesteld
note-template-field-why-it-matters = Waarom het belangrijk is
note-template-field-appearance = Uiterlijk
note-template-field-age-and-origin = Leeftijd en herkomst
note-template-field-what-it-does = Wat het doet
note-template-field-who-holds-it = Wie houdt het vast?
note-template-field-who-wants-it = Wie wil het?
note-template-field-pov = Perspectief
note-template-field-time-and-place = Tijd en plaats
note-template-field-goal = Doel
note-template-field-conflict = Conflict
note-template-field-turn = De bocht
note-template-field-exit-emotion = Emotie op weg naar buiten
note-template-field-purpose = Betreft
note-template-field-who-leads-it = Wie leidt het?
note-template-field-resources = Middelen
note-template-field-allies-and-enemies = Geallieerden en vijanden
note-template-field-what-it-wants-now = Wat het nu wil.
note-template-field-where-from = Waar het vandaan kwam
note-template-field-page-or-link = Pagina of link
note-template-field-key-facts = Belangrijkste feiten
note-template-field-how-it-is-used = Hoe het wordt gebruikt in het verhaal
