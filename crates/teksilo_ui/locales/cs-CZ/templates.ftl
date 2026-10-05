# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

settings-page-templates = Šablony
settings-templates-description = Opakovaně použitelné kusy textu, které můžete vložit do čehokoli, co píšete: prázdný list postavy, profil místa, osnova scén. Ukládají se v tomto projektu, takže kdokoli jej otevře, dostane stejnou sadu.
settings-templates-filter = Filtrovat šablony
settings-templates-count = { $n ->
    [0] Žádné šablony
    [one] 1 šablona
    [few] { $n } šablony
   *[other] { $n } šablon
}
settings-templates-empty = Tento projekt zatím nemá žádné šablony.
settings-templates-empty-hint = Přidejte šablonu z předvolby, importujte soubor .md nebo .djot, nebo něco napište a zvolte Dokument ▸ Uložit jako šablonu.
settings-templates-presets = Přidat předvolbu
settings-templates-import = Importovat…
settings-templates-export = Exportovat…
settings-templates-star = Zobrazovat v nabídce vložení jako první
settings-templates-unstar = Přestat zobrazovat jako první
settings-templates-move-up = Posunout nahoru
settings-templates-move-down = Posunout dolů
settings-templates-delete = Smazat šablonu
settings-templates-name = Název
settings-templates-name-placeholder = Název šablony
settings-templates-body-placeholder = Text, který tato šablona vloží
settings-templates-duplicate-name = Jiná šablona už se jmenuje „{ $name }“
settings-templates-words = { $n ->
    [one] 1 slovo
    [few] { $n } slova
   *[other] { $n } slov
}

templates-imported = { $added ->
    [one] Importována 1 šablona
    [few] Importovány { $added } šablony
   *[other] Importováno { $added } šablon
}
templates-imported-renamed = Přejmenováno kvůli shodě názvů: { $names }
templates-imported-skipped = Nelze přečíst: { $names }
templates-import-filter = Šablony (.md, .djot)
templates-exported = { $n ->
    [one] Exportována 1 šablona
    [few] Exportovány { $n } šablony
   *[other] Exportováno { $n } šablon
}
templates-preset-applied = Přidáno „{ $name }“

templates-delete-title = Smazat tuto šablonu?
templates-delete-body = „{ $name }“ bude z tohoto projektu odebrána. Poznámek, které jste z ní už vytvořili, se to nedotkne.
templates-delete-confirm = Smazat

menu-document = &Dokument
menu-insert-template = &Vložit šablonu
menu-insert-template-none = V tomto projektu nejsou žádné šablony
menu-save-as-template = Uložit j&ako šablonu…

save-as-template-title = Uložit jako šablonu
save-as-template-explain = Text, který upravujete, se stane šablonou, kterou můžete vložit kamkoli jinam.
save-as-template-name = Název
save-as-template-placeholder = List postavy
save-as-template-duplicate = Šablona s názvem „{ $name }“ už existuje
save-as-template-empty-editor = Tento editor je prázdný. Není co uložit.
save-as-template-confirm = Uložit šablonu
save-as-template-saved = „{ $name }“ byl uložen jako šablona
template-inserted = Vloženo „{ $name }“

note-template-preset-character-sheet = List postavy
note-template-preset-location = Místo
note-template-preset-artifact = Předmět
note-template-preset-beat-sheet = Osnova scén
note-template-preset-faction = Frakce
note-template-preset-research-note = Poznámka z rešerše

note-template-set-essentials = To podstatné
note-template-set-everything = Všechny šablony

note-template-section-identity = Identita
note-template-section-appearance = Vzhled
note-template-section-voice = Hlas
note-template-section-psychology = Vnitřní život
note-template-section-history = Minulost
note-template-section-arc = Vývoj
note-template-section-first-impression = První dojem
note-template-section-in-the-story = V příběhu
note-template-section-the-thing-itself = Předmět sám
note-template-section-the-scene = O scéně
note-template-section-the-shape = Stavba
note-template-section-what-it-is = Co to je
note-template-section-source = Pramen
note-template-section-what-it-says = Co říká

note-template-field-full-name = Celé jméno
note-template-field-known-as = Také známo jako (vede se v poli Alternativní jména v inspektoru)
note-template-field-age = Věk
note-template-field-role-in-story = Role v příběhu
note-template-field-build-and-features = Postava a poznávací znamení
note-template-field-habitual-bearing = Obvyklé držení těla
note-template-field-speech-patterns = Způsob řeči a slovní tiky
note-template-field-what-they-never-say = Co nikdy neřekne
note-template-field-want = Co chce
note-template-field-need = Co potřebuje, aniž by to věděl(a)
note-template-field-fear = Čeho se bojí
note-template-field-flaw = Slabina, která jej/ji něco stojí
note-template-field-formative-event = Formující událost
note-template-field-relationships = Vztahy
note-template-field-starts-as = Na začátku
note-template-field-ends-as = Na konci
note-template-field-what-you-notice-first = Čeho si všimnete nejdřív
note-template-field-sound-and-smell = Zvuky a vůně
note-template-field-light-and-weather = Světlo a počasí
note-template-field-what-happened-here = Co se tu stalo
note-template-field-who-lives-or-works-here = Kdo tu žije nebo pracuje
note-template-field-scenes-set-here = Scény, které se tu odehrávají
note-template-field-why-it-matters = Proč na tom záleží
note-template-field-appearance = Vzhled
note-template-field-age-and-origin = Stáří a původ
note-template-field-what-it-does = K čemu slouží
note-template-field-who-holds-it = Kdo jej má
note-template-field-who-wants-it = Kdo jej chce
note-template-field-pov = Úhel pohledu
note-template-field-time-and-place = Čas a místo
note-template-field-goal = Cíl
note-template-field-conflict = Konflikt
note-template-field-turn = Zvrat
note-template-field-exit-emotion = Emoce na odchodu
note-template-field-purpose = Účel
note-template-field-who-leads-it = Kdo ji vede
note-template-field-resources = Prostředky
note-template-field-allies-and-enemies = Spojenci a nepřátelé
note-template-field-what-it-wants-now = Co teď chce
note-template-field-where-from = Odkud pochází
note-template-field-page-or-link = Stránka nebo odkaz
note-template-field-key-facts = Klíčová fakta
note-template-field-how-it-is-used = Jak se využívá v příběhu
