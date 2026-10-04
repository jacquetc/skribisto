# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

settings-page-templates = Šablony
settings-templates-description = Opakovatelné části psaní můžete spadnout do všeho, co píšete: prázdný znak list, profil umístění, beat list. Jsou uloženy v tomto projektu, takže každý, kdo otevře to dostane stejnou sadu.
settings-templates-filter = Filtrační šablony
settings-templates-count = { $n ->
    [0] Bez šablon
    [one] 1 šablona
   *[other] Šablony { $n }
}
settings-templates-empty = Tento projekt zatím nemá žádné šablony.
settings-templates-empty-hint = Přidejte jeden z přednastaveného souboru, importujte soubor .md nebo .djot, nebo něco napište a vyberte Document ΆUložit jako šablonu.
settings-templates-presets = Přidat přednastavený
settings-templates-import = Importovat…
settings-templates-export = Exportovat…
settings-templates-star = Zobrazit jako první v menu vkladu
settings-templates-unstar = Přestaň se předvádět.
settings-templates-move-up = Posunout nahoru
settings-templates-move-down = Posunout dolů
settings-templates-delete = Smazat šablonu
settings-templates-name = Název
settings-templates-name-placeholder = Název šablony
settings-templates-body-placeholder = Text vložený do této šablony
settings-templates-duplicate-name = Další šablona se již nazývá currency { $name } currency
settings-templates-words = { $n ->
    [one] 1 slovo
   *[other] { $n } slov
}

templates-imported = { $added ->
    [one] 1 dovezená šablona
   *[other] { $added } importované šablony
}
templates-imported-renamed = Přejmenováno, aby se zabránilo střetu: { $names }
templates-imported-skipped = Nelze číst: { $names }
templates-import-filter = Šablony (.md, .djot)
templates-exported = { $n ->
    [one] 1 vyvezená šablona
   *[other] { $n } vyvezené šablony
}
templates-preset-applied = Přidáno { $name }

templates-delete-title = Smazat tuto šablonu?
templates-delete-body = Z tohoto projektu bude odstraněno { $name }. Poznámky, které jste z něj již vytvořili, nejsou ovlivněny.
templates-delete-confirm = Smazat

menu-document = Dokument (&D)
menu-insert-template = Vložit šablonu (&L)
menu-insert-template-none = Žádné šablony v tomto projektu
menu-save-as-template = Uložit jako šablonu (&V)

save-as-template-title = Uložit jako šablonu
save-as-template-explain = Text, který editujete, se stává šablonou, kterou můžete použít kdekoliv jinde.
save-as-template-name = Název
save-as-template-placeholder = Znakový list
save-as-template-duplicate = Šablona nazvaná CLAS{ $name } již existuje
save-as-template-empty-editor = Ten editor je prázdný, není co zachraňovat.
save-as-template-confirm = Uložit šablonu
save-as-template-saved = Uloženo{ $name }Šablona jako šablona
template-inserted = Vloženo { $name }

note-template-preset-character-sheet = Znakový list
note-template-preset-location = Umístění
note-template-preset-artifact = Předmět
note-template-preset-beat-sheet = Beat list
note-template-preset-faction = Frakce
note-template-preset-research-note = Výzkumná poznámka

note-template-set-essentials = Základní údaje
note-template-set-everything = Každá šablona

note-template-section-identity = Identita
note-template-section-appearance = Vzhled
note-template-section-voice = Hlas
note-template-section-psychology = Vnitřní život
note-template-section-history = Historie
note-template-section-arc = Vývoj postavy
note-template-section-first-impression = První dojem
note-template-section-in-the-story = V příběhu
note-template-section-the-thing-itself = Objekt sám
note-template-section-the-scene = Scéna
note-template-section-the-shape = Tvar
note-template-section-what-it-is = Co to je?
note-template-section-source = Zdroj
note-template-section-what-it-says = Co to říká?

note-template-field-full-name = Celé jméno
note-template-field-known-as = Známý také jako, sledovaný v poli Alias Inspector Acads.
note-template-field-age = Věk
note-template-field-role-in-story = Role v příběhu
note-template-field-build-and-features = Vybudovat a rozlišovat prvky
note-template-field-habitual-bearing = Habituální ložisko
note-template-field-speech-patterns = Vzor řeči a slovní činnost
note-template-field-what-they-never-say = Co nikdy neřeknou
note-template-field-want = Co chtějí
note-template-field-need = Co potřebují, ale nevědí to.
note-template-field-fear = Čeho se bojí?
note-template-field-flaw = Chyba, která je stojí
note-template-field-formative-event = Formativní událost
note-template-field-relationships = Vztahy
note-template-field-starts-as = Začíná jako
note-template-field-ends-as = Konce jako
note-template-field-what-you-notice-first = Co si všimneš jako první
note-template-field-sound-and-smell = Zvuk a vůně
note-template-field-light-and-weather = Světlo a počasí
note-template-field-what-happened-here = Co se tu stalo?
note-template-field-who-lives-or-works-here = Kdo tady žije nebo žije?
note-template-field-scenes-set-here = Scény nastaven zde
note-template-field-why-it-matters = Proč na tom záleží?
note-template-field-appearance = Vzhled
note-template-field-age-and-origin = Věk a původ
note-template-field-what-it-does = Co to dělá?
note-template-field-who-holds-it = Kdo to drží?
note-template-field-who-wants-it = Kdo to chce?
note-template-field-pov = Úhel pohledu
note-template-field-time-and-place = Čas a místo
note-template-field-goal = Cíl
note-template-field-conflict = Konflikt
note-template-field-turn = Zvrat
note-template-field-exit-emotion = Emoce na cestě ven
note-template-field-purpose = Účel
note-template-field-who-leads-it = Kdo to vede?
note-template-field-resources = Zdroje
note-template-field-allies-and-enemies = Spojenci a nepřátelé
note-template-field-what-it-wants-now = Co chce teď.
note-template-field-where-from = Where it come from
note-template-field-page-or-link = Strana nebo odkaz
note-template-field-key-facts = Klíčové skutečnosti
note-template-field-how-it-is-used = Jak se používá v příběhu
