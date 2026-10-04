# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

settings-page-templates = Mallar
settings-templates-description = Återanvändbara bitar av att skriva kan du släppa in allt du skriver: ett tomt teckenblad, en platsprofil, ett slagplåt. De lagras i detta projekt, så alla som öppnar det får samma uppsättning.
settings-templates-filter = Filter mallar
settings-templates-count = { $n ->
    [0] Inga mallar
    [one] 1 mall
   *[other] { $n } mallar
}
settings-templates-empty = Detta projekt har inga mallar än.
settings-templates-empty-hint = Lägg till en från en förinställd, importera en .md eller .djot-fil eller skriv något och välj Dokument - Spara som mall.
settings-templates-presets = Lägg till ett förinställt
settings-templates-import = Importera…
settings-templates-export = Exportera…
settings-templates-star = Visa först i insatsmenyn
settings-templates-unstar = Sluta visa först
settings-templates-move-up = Flytta upp
settings-templates-move-down = Flytta ned
settings-templates-delete = Radera mall
settings-templates-name = Namn
settings-templates-name-placeholder = Malta namn
settings-templates-body-placeholder = Texten denna mall infogar
settings-templates-duplicate-name = En annan mall kallas redan "{ $name }"
settings-templates-words = { $n ->
    [one] 1 ord
   *[other] { $n } ord
}

templates-imported = { $added ->
    [one] 1 mall importerad
   *[other] { $added } mallar importerade
}
templates-imported-renamed = Omdöpt för att undvika en sammandrabbning: { $names }
templates-imported-skipped = Kunde inte läsa: { $names }
templates-import-filter = Mallar (.md, .djot)
templates-exported = { $n ->
    [one] 1 mall exporterad
   *[other] { $n } mallar exporterade
}
templates-preset-applied = Tillagd "{ $name }"

templates-delete-title = Ta bort denna mall?
templates-delete-body = "{ $name }" kommer att tas bort från det här projektet. Observera att du redan skapat av det inte påverkas.
templates-delete-confirm = Ta bort

menu-document = Dokument (&D)
menu-insert-template = Infoga mall (&L)
menu-insert-template-none = Inga mallar i detta projekt
menu-save-as-template = Spara som mall... (&V)

save-as-template-title = Spara som mall
save-as-template-explain = Texten du redigerar blir en mall du kan släppa i någon annanstans.
save-as-template-name = Namn
save-as-template-placeholder = Karaktärsblad
save-as-template-duplicate = En mall som kallas "{ $name }" finns redan.
save-as-template-empty-editor = Denna redaktör är tom. Det finns inget att spara.
save-as-template-confirm = Spara mall
save-as-template-saved = Sparad "{ $name }" som en mall
template-inserted = Infogad "{ $name }"

note-template-preset-character-sheet = Karaktärsblad
note-template-preset-location = Plats
note-template-preset-artifact = Objekt
note-template-preset-beat-sheet = Beat Sheet
note-template-preset-faction = Fraktion
note-template-preset-research-note = Forskningsnot

note-template-set-essentials = Grunduppgifter
note-template-set-everything = Varje mall

note-template-section-identity = Identitet
note-template-section-appearance = Utseende
note-template-section-voice = Röst
note-template-section-psychology = Inre liv
note-template-section-history = Historia historia
note-template-section-arc = Utvecklingskurva
note-template-section-first-impression = Första intrycket
note-template-section-in-the-story = I berättelsen
note-template-section-the-thing-itself = Objektet själv
note-template-section-the-scene = Scenen
note-template-section-the-shape = Formen
note-template-section-what-it-is = Vad det är
note-template-section-source = Källa
note-template-section-what-it-says = Vad det säger

note-template-field-full-name = Fullständigt namn
note-template-field-known-as = Även känd som, spårad i fältet Inspector's Alias
note-template-field-age = Ålder
note-template-field-role-in-story = Roll i berättelsen
note-template-field-build-and-features = Bygg och särskiljande funktioner
note-template-field-habitual-bearing = Vanligt lager
note-template-field-speech-patterns = Talmönster och verbala tics
note-template-field-what-they-never-say = Vad de aldrig säger
note-template-field-want = Vad de vill ha
note-template-field-need = Vad de behöver men vet inte det
note-template-field-fear = Vad de fruktar
note-template-field-flaw = bristen som kostar dem
note-template-field-formative-event = Formativ händelse
note-template-field-relationships = Relationer
note-template-field-starts-as = Börjar som
note-template-field-ends-as = Slutar som
note-template-field-what-you-notice-first = Vad du märker först
note-template-field-sound-and-smell = Ljud och lukt
note-template-field-light-and-weather = Ljus och väder
note-template-field-what-happened-here = Vad hände här
note-template-field-who-lives-or-works-here = Vem bor eller projekt här
note-template-field-scenes-set-here = Scener här
note-template-field-why-it-matters = Varför det spelar roll
note-template-field-appearance = Utseende
note-template-field-age-and-origin = Ålder och ursprung
note-template-field-what-it-does = Vad det gör
note-template-field-who-holds-it = Vem håller den
note-template-field-who-wants-it = Vem vill ha den
note-template-field-pov = Perspektiv
note-template-field-time-and-place = Tid och plats
note-template-field-goal = Mål
note-template-field-conflict = Konflikt
note-template-field-turn = Svängen
note-template-field-exit-emotion = Emotion på vägen ut
note-template-field-purpose = Syfte
note-template-field-who-leads-it = Vem leder den
note-template-field-resources = Resurser
note-template-field-allies-and-enemies = Allierade och fiender
note-template-field-what-it-wants-now = Vad den vill ha nu
note-template-field-where-from = Där den kom från
note-template-field-page-or-link = Sidan eller länken
note-template-field-key-facts = Viktiga fakta
note-template-field-how-it-is-used = Hur den används i berättelsen
