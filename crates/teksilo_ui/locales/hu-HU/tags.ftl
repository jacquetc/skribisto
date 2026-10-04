# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

tags-preset-basic = Alap
tags-preset-scifi = Tudományos-fantasztikus
tags-preset-fantasy = Fantázia
tags-preset-mystery = Rejtély
tags-preset-historical = Történelmi


tags-preset-needs-research = kutatásokra van szükség
tags-preset-continuity-check = A folyamatosság ellenőrzése
tags-preset-plot-point = telken

tags-preset-character = karakter
tags-preset-place = hely
tags-preset-item = tétel

tags-preset-vessel = hajó
tags-preset-planet = bolygó
tags-preset-organization = szervezés
tags-preset-creature = teremtmény
tags-preset-faction = frakció
tags-preset-artifact = ereklye
tags-preset-realm = birodalom
tags-preset-magic-system = mágikus rendszer
tags-preset-suspect = gyanúsított
tags-preset-victim = áldozat
tags-preset-clue = nyom
tags-preset-red-herring = vörös hering
tags-preset-historical-figure = történeti adat
tags-preset-source = forrás
tags-preset-period-detail = időszak részletezése

tags-pill-list = Címkék
tags-pill-add = Címke hozzáadása
tags-none-yet = Nincs címke
tags-pick-list = Címkék közül választhat
tags-pill-remove = A { $name } eltávolítása
tags-pill-filter-placeholder = Új címke szűrése vagy elnevezése
tags-pill-no-match = Nincs címke egyezés.
tags-pill-create = Létrehozása { $name }
tags-pill-create-failed = Létrehozhatnánk a { $name }
tags-pill-new-discoverable = Find- in- prózacímke
tags-pill-new-discoverable-hint = Az ezzel a címkével ellátott elemek illeszkednek a prózájához, hogy kitöltsék a névsort.

tags-alias-list = Más néven:
tags-alias-add = Új név hozzáadása
tags-alias-remove = A { $name } eltávolítása
tags-alias-placeholder = Másik név, majd Enter
tags-alias-hint = Nevek jelennek meg a prózádban, a címe mellett.
tags-alias-collision = A { $name } már válaszol erre a névre.

settings-page-tags = Címkék
settings-tags-desc = Címkék címkék a tételek a mappában. A find- in-próza címke is azt mondja Skribisto, hogy keresse meg ezt a tételt a prózában.
settings-tags-add = Címke hozzáadása
settings-tags-add-placeholder = Új címke
settings-tags-added = Hozzáadva { $name }
settings-tags-duplicate = A { $name } már létezik.
settings-tags-filter = Szűrőcímkék
settings-tags-count = { $n ->
    [one] 1 címke
   *[other] { $n } címkék
}
settings-tags-details-placeholder = Mit jelent ez a címke?
settings-tags-discoverable = Keresés a prózában
settings-tags-creates-in = Új jegyzetek:
settings-tags-creates-in-unset = Először kérdezd meg.
settings-tags-creates-in-untitled = Névtelen mappa
settings-tags-creates-in-trashed = { $name } (a szemétben)
settings-tags-template = Kiindulási sablon
settings-tags-template-unset = Üres megjegyzés
settings-tags-delete = Törlés { $name }
settings-tags-deleted = Törölve: { $name }, és minden darabból eltávolítva
settings-tags-empty = Még nincs rajta címke.
settings-tags-apply-preset = Előzetes beállítás alkalmazása
settings-tags-preset-applied = Hozzáadott { $added }, kimaradt { $skipped } már jelen
settings-tags-csv-filter = CSV fájl
settings-tags-import = Importálás…
settings-tags-export = Exportálás…
settings-tags-imported = Importált { $added }, kimaradt { $skipped }
settings-tags-exported = Kivitel { $n ->
    [one] 1 címke
   *[other] { $n } címkék
}

tags-chip-more = { $n ->
    [one] 1 további címke
   *[other] { $n } további címke
}

cast-section = Szereplők
cast-add = Hozzáadás a gipszhez
cast-add-filter-placeholder = Szűrő történet bibliaName
cast-add-empty = Nincs történet-bibliai bejegyzések hozzá
cast-pin = Hozzáadás { $name } a gipsz
cast-unpin = Távolítsa el a { $name }-et a gipszet
cast-empty = Még senki sem volt beszorulva.
cast-unresolved = Már nem a történet bibliában.

mentions-confirm = Hozzáadás a { $name }-hez
mentions-hit-count = { $n ->
    [one] egyszer
   *[other] { $n }-szer
}
mentions-point-of-view-badge = Nézőpont
mentions-point-of-view-badge-tooltip = Itt a nézőpontnak minősítve, kézzel beállítva, a prózában nem észlelve.

mentions-roster = Szereplők
mentions-pin = Hozzáadás { $name } a gipsz

pov-section = Nézőpont
pov-empty = Nincs nézőpont
pov-add = Meghatározott nézőpont
pov-multiple = Ennek a jelenetnek két nézőpontja van.
pov-remove = Távolítsa el a { $name } nézőpontból
pov-unresolved = Egy nézőpont volt itt, de ez a bejegyzés már nem létezik, vagy elvesztette a történet-bibliai címkét.

books-section = Kategória:
books-empty = Még nem jelent meg egy könyv alatt.
books-add = Fájl könyv alatt
books-remove = Eltávolítás a { $name }-ből
books-apply-to-children = Kérelmezés gyermekek számára

note-details-name-placeholder = Név…
note-details-tags = Címkék
note-details-aliases = Egyéb nevek
note-details-books = Kategória:
note-details-books-empty = Még nem került be könyvbe
note-details-links = Linkek
note-details-links-empty = Még nincs kapcsolat, adjon hozzá egyet, hogy rámutasson egy másik bejegyzésre a bibliájában.
note-details-links-add = Link egy cetlihez
note-details-pov = Nézőpont
note-details-pov-empty = Nincs nézőpont
note-details-pov-unresolved = Egy nézőpont volt itt, de ez a bejegyzés már nem létezik, vagy elvesztette a történet-bibliai címkét.
note-details-pov-multiple = Ennek a megjegyzésnek két szempontja van.
note-details-backlinks = Megjelenések a kéziratban
note-details-backlinks-empty = Ha ez a név megjelenik a prózádban, itt fog megjelenni.
note-details-backlinks-outside = A könyveken kívül
note-details-backlinks-confirm-all = Minden megjelenés megerősítése
note-details-backlinks-confirm-all-tooltip = Ezt a bejegyzést adja hozzá az itt bemutatott minden olyan dokumentumhoz, amely még nem erősítette meg.
note-details-untitled-document = Név nélkül

note-in-prose-mentions = { $n ->
    [one] 1 említés
   *[other] { $n }
}
note-in-prose-mention-at = { $current }, { $total }
note-in-prose-mention-previous = Előző említés
note-in-prose-mention-next = Következő említés
