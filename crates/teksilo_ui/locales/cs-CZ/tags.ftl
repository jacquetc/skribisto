# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

tags-preset-basic = Základní
tags-preset-scifi = Science fiction
tags-preset-fantasy = Fantasy
tags-preset-mystery = Záhada
tags-preset-historical = Historické


tags-preset-needs-research = potřeby výzkumu
tags-preset-continuity-check = kontrola kontinuity
tags-preset-plot-point = bod

tags-preset-character = znak
tags-preset-place = místo
tags-preset-item = položka

tags-preset-vessel = plavidlo
tags-preset-planet = planeta
tags-preset-organization = organizace
tags-preset-creature = stvoření
tags-preset-faction = frakce
tags-preset-artifact = artefakt
tags-preset-realm = říše
tags-preset-magic-system = magický systém
tags-preset-suspect = podezřelý
tags-preset-victim = oběť
tags-preset-clue = nápověda
tags-preset-red-herring = sledě červeného
tags-preset-historical-figure = historický údaj
tags-preset-source = zdroj
tags-preset-period-detail = Údaj o období

tags-pill-list = Štítky
tags-pill-add = Přidat značku
tags-none-yet = Žádné štítky
tags-pick-list = Značky vybrat z
tags-pill-remove = Odstranit { $name }
tags-pill-filter-placeholder = Filtrovat nebo pojmenovat novou značku
tags-pill-no-match = Žádný štítek neodpovídá.
tags-pill-create = Vytvořit CLAS{ $name }
tags-pill-create-failed = Mohl jsem vytvořit { $name }
tags-pill-new-discoverable = Značka Find- in- próza
tags-pill-new-discoverable-hint = Položky s touto značkou jsou porovnány s prózou vyplnit soupis.

tags-alias-list = Také známý jako
tags-alias-add = Přidat jiné jméno
tags-alias-remove = Odstranit { $name }
tags-alias-placeholder = Další jméno, pak Enter
tags-alias-hint = Jména, která se objeví ve vaší próze, vedle jejího názvu.
tags-alias-collision = { $name } již odpovídá na toto jméno.

settings-page-tags = Štítky
settings-tags-desc = Značky označují položky ve Vašem pořadači. Find-in-prose tag také říká Skribisto hledat tuto položku jména ve Vašem próze.
settings-tags-add = Přidat značku
settings-tags-add-placeholder = Jmenuj novou značku
settings-tags-added = Přidáno { $name }
settings-tags-duplicate = CLAS{ $name } již existuje
settings-tags-filter = Filtrační značky
settings-tags-count = { $n ->
    [one] 1 tag
   *[other] { $n } tagy
}
settings-tags-details-placeholder = Co ta značka znamená?
settings-tags-discoverable = Najít v próza
settings-tags-creates-in = Nové poznámky jdou na
settings-tags-creates-in-unset = Zeptej se mě poprvé.
settings-tags-creates-in-untitled = Nepojmenovaná složka
settings-tags-creates-in-trashed = { $name } (v koši)
settings-tags-template = Počáteční šablona
settings-tags-template-unset = Prázdná poznámka
settings-tags-delete = Smazat { $name }
settings-tags-deleted = Odstraněno { $name } a odstraněno z každé položky
settings-tags-empty = Zatím žádné známky.
settings-tags-apply-preset = Použít přednastavené nastavení
settings-tags-preset-applied = Přidáno { $added }, přeskočeno { $skipped } již přítomno
settings-tags-csv-filter = CSV souborů
settings-tags-import = Importovat…
settings-tags-export = Exportovat…
settings-tags-imported = Dovezené { $added }, přeskočené { $skipped }
settings-tags-exported = Vývoz { $n ->
    [one] 1 tag
   *[other] { $n } tagy
}

tags-chip-more = { $n ->
    [one] 1 další tag
   *[other] { $n } další tagy
}

cast-section = Postavy
cast-add = Přidat do obsazení
cast-add-filter-placeholder = Filtr příběh bible
cast-add-empty = Žádné příběhy-bible položky přidat
cast-pin = Přidat { $name } k obsazení
cast-unpin = Odstranit { $name } z odlitku
cast-empty = Ještě nikdo neklepal, přidejte nebo nechte návrh.
cast-unresolved = Už ne v bibli příběhu

mentions-confirm = Přidat do obsazení { $name }
mentions-hit-count = { $n ->
    [one] jednou
   *[other] { $n } krát
}
mentions-point-of-view-badge = Úhel pohledu
mentions-point-of-view-badge-tooltip = Vyhlášen jako úhel pohledu zde, nastavený ručně, není detekován v próze.

mentions-roster = Postavy
mentions-pin = Přidat { $name } k obsazení

pov-section = Úhel pohledu
pov-empty = Žádný úhel pohledu
pov-add = Nastavit úhel pohledu
pov-multiple = Tahle scéna má dva úhly pohledu.
pov-remove = Odstranit { $name } jako úhel pohledu
pov-unresolved = Byl zde přišpendlen úhel pohledu, ale tento záznam už neexistuje nebo ztratil svou pohádkovou biblickou značku.

books-section = Soubor pod
books-empty = Ještě není vyplněno pod knihou.
books-add = Soubor pod knihou
books-remove = Odstranit z { $name }
books-apply-to-children = Aplikovat podání na děti

note-details-name-placeholder = Název…
note-details-tags = Štítky
note-details-aliases = Ostatní názvy
note-details-books = Soubor pod
note-details-books-empty = Ještě není vyplněno pod knihou
note-details-links = Odkazy
note-details-links-empty = Zatím žádné odkazy. Přidejte jeden k bodu na jiný záznam ve vašem příběhu bible.
note-details-links-add = Odkaz na poznámku
note-details-pov = Úhel pohledu
note-details-pov-empty = Žádný úhel pohledu
note-details-pov-unresolved = Byl zde přišpendlen úhel pohledu, ale tento záznam už neexistuje nebo ztratil svou pohádkovou biblickou značku.
note-details-pov-multiple = Tato poznámka má dva názory.
note-details-backlinks = Objevuje se v rukopisu
note-details-backlinks-empty = Jakmile se to jméno objeví ve vaší próze, objeví se tady.
note-details-backlinks-outside = Mimo knihy.
note-details-backlinks-confirm-all = Potvrdit každý vzhled
note-details-backlinks-confirm-all-tooltip = Přidat tuto položku do obsazení každého dokumentu zde zobrazeného, který ještě nebyl potvrzen
note-details-untitled-document = Bez názvu

note-in-prose-mentions = { $n ->
    [one] 1
   *[other] { $n } zmiňuje
}
note-in-prose-mention-at = { $current } z { $total }
note-in-prose-mention-previous = Předchozí zmínka
note-in-prose-mention-next = Další poznámka
