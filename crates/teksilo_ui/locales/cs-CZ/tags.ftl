# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

tags-preset-basic = Základní
tags-preset-scifi = Science fiction
tags-preset-fantasy = Fantasy
tags-preset-mystery = Detektivka
tags-preset-historical = Historický román


tags-preset-needs-research = vyžaduje rešerši
tags-preset-continuity-check = kontrola návaznosti
tags-preset-plot-point = zlom v ději

tags-preset-character = postava
tags-preset-place = místo
tags-preset-item = předmět

tags-preset-vessel = loď
tags-preset-planet = planeta
tags-preset-organization = organizace
tags-preset-creature = tvor
tags-preset-faction = frakce
tags-preset-artifact = artefakt
tags-preset-realm = říše
tags-preset-magic-system = magický systém
tags-preset-suspect = podezřelý
tags-preset-victim = oběť
tags-preset-clue = stopa
tags-preset-red-herring = falešná stopa
tags-preset-historical-figure = historická postava
tags-preset-source = pramen
tags-preset-period-detail = dobový detail

tags-pill-list = Štítky
tags-pill-add = Přidat štítek
tags-none-yet = Žádné štítky
tags-pick-list = Štítky k výběru
tags-pill-remove = Odebrat { $name }
tags-pill-filter-placeholder = Filtrovat nebo pojmenovat nový štítek
tags-pill-no-match = Žádný štítek neodpovídá
tags-pill-create = Vytvořit „{ $name }“
tags-pill-create-failed = „{ $name }“ nelze vytvořit
tags-pill-new-discoverable = Štítek s hledáním v próze
tags-pill-new-discoverable-hint = Položky s tímto štítkem se hledají ve vaší próze a doplňují seznam postav.

tags-alias-list = Také známo jako
tags-alias-add = Přidat další jméno
tags-alias-remove = Odebrat { $name }
tags-alias-placeholder = Další jméno, potom Enter
tags-alias-hint = Jména, pod kterými se kromě názvu objevuje ve vaší próze.
tags-alias-collision = Na toto jméno už slyší { $name }.

settings-page-tags = Štítky
settings-tags-desc = Štítky označují položky ve vašem pořadači. Štítek s hledáním v próze navíc aplikaci Skribisto říká, aby jména dané položky hledala ve vaší próze.
settings-tags-add = Přidat štítek
settings-tags-add-placeholder = Název nového štítku
settings-tags-added = Přidáno „{ $name }“
settings-tags-duplicate = „{ $name }“ už existuje
settings-tags-filter = Filtrovat štítky
settings-tags-count = { $n ->
    [one] 1 štítek
    [few] { $n } štítky
   *[other] { $n } štítků
}
settings-tags-details-placeholder = Co tento štítek znamená
settings-tags-discoverable = Hledat v próze
settings-tags-creates-in = Nové poznámky ukládat do
settings-tags-creates-in-unset = Zeptat se poprvé
settings-tags-creates-in-untitled = Složka bez názvu
settings-tags-creates-in-trashed = { $name } (v koši)
settings-tags-template = Výchozí šablona
settings-tags-template-unset = Prázdná poznámka
settings-tags-delete = Smazat { $name }
settings-tags-deleted = Štítek „{ $name }“ byl smazán a odebrán ze všech položek
settings-tags-empty = Zatím žádné štítky.
settings-tags-apply-preset = Použít předvolbu…
settings-tags-preset-applied = Přidáno: { $added }, přeskočeno (už existuje): { $skipped }
settings-tags-csv-filter = Soubory CSV
settings-tags-import = Importovat…
settings-tags-export = Exportovat…
settings-tags-imported = Importováno: { $added }, přeskočeno: { $skipped }
settings-tags-exported = { $n ->
    [one] Exportován 1 štítek
    [few] Exportovány { $n } štítky
   *[other] Exportováno { $n } štítků
}

tags-chip-more = { $n ->
    [one] 1 další štítek
    [few] { $n } další štítky
   *[other] { $n } dalších štítků
}

cast-section = Postavy
cast-add = Přidat mezi postavy…
cast-add-filter-placeholder = Filtrovat bibli příběhu…
cast-add-empty = Žádné záznamy bible příběhu k přidání
cast-pin = Přidat { $name } mezi postavy
cast-unpin = Odebrat { $name } z postav
cast-empty = Zatím nikdo není připnut. Přidejte nebo ponechte některý návrh
cast-unresolved = Už není v bibli příběhu

mentions-confirm = Přidat mezi postavy dokumentu { $name }
mentions-hit-count = { $n ->
    [one] jednou
    [few] { $n }krát
   *[other] { $n }krát
}
mentions-point-of-view-badge = Úhel pohledu
mentions-point-of-view-badge-tooltip = Zde ručně určený úhel pohledu, nezjištěný z prózy.

mentions-roster = Postavy
mentions-pin = Přidat { $name } mezi postavy

pov-section = Úhel pohledu
pov-empty = Úhel pohledu není nastaven
pov-add = Nastavit úhel pohledu…
pov-multiple = Tato scéna má dva úhly pohledu.
pov-remove = Odebrat { $name } jako úhel pohledu
pov-unresolved = Byl zde připnut úhel pohledu, ale daný záznam už neexistuje nebo přišel o svůj štítek bible příběhu.

books-section = Zařazeno pod
books-empty = Zatím není zařazeno pod žádnou knihu
books-add = Zařadit pod knihu…
books-remove = Odebrat z { $name }
books-apply-to-children = Použít zařazení na podřízené položky

note-details-name-placeholder = Jméno…
note-details-tags = Štítky
note-details-aliases = Další jména
note-details-books = Zařazeno pod
note-details-books-empty = Zatím není zařazeno pod žádnou knihu
note-details-links = Odkazy
note-details-links-empty = Zatím žádné odkazy. Přidejte odkaz na jiný záznam ve vaší bibli příběhu.
note-details-links-add = Odkázat na poznámku…
note-details-pov = Úhel pohledu
note-details-pov-empty = Úhel pohledu není nastaven
note-details-pov-unresolved = Byl zde připnut úhel pohledu, ale daný záznam už neexistuje nebo přišel o svůj štítek bible příběhu.
note-details-pov-multiple = Tato poznámka má dva úhly pohledu.
note-details-backlinks = Výskyty v rukopisu
note-details-backlinks-empty = Zatím nic. Jakmile se toto jméno objeví ve vaší próze, zobrazí se zde.
note-details-backlinks-outside = Mimo knihy
note-details-backlinks-confirm-all = Potvrdit všechny výskyty
note-details-backlinks-confirm-all-tooltip = Přidá tento záznam mezi postavy každého zde zobrazeného dokumentu, který jej ještě nepotvrdil
note-details-untitled-document = Bez názvu

note-in-prose-mentions = { $n ->
    [one] 1 zmínka
    [few] { $n } zmínky
   *[other] { $n } zmínek
}
note-in-prose-mention-at = { $current } z { $total }
note-in-prose-mention-previous = Předchozí zmínka
note-in-prose-mention-next = Další zmínka
