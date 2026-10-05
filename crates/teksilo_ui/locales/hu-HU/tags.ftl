# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

tags-preset-basic = Alap
tags-preset-scifi = Sci-fi
tags-preset-fantasy = Fantasy
tags-preset-mystery = Krimi
tags-preset-historical = Történelmi
tags-preset-needs-research = utánajárást igényel
tags-preset-continuity-check = folytonosság ellenőrzése
tags-preset-plot-point = fordulópont
tags-preset-character = szereplő
tags-preset-place = helyszín
tags-preset-item = tárgy
tags-preset-vessel = űrhajó
tags-preset-planet = bolygó
tags-preset-organization = szervezet
tags-preset-creature = lény
tags-preset-faction = frakció
tags-preset-artifact = varázstárgy
tags-preset-realm = birodalom
tags-preset-magic-system = mágiarendszer
tags-preset-suspect = gyanúsított
tags-preset-victim = áldozat
tags-preset-clue = nyom
tags-preset-red-herring = hamis nyom
tags-preset-historical-figure = történelmi személy
tags-preset-source = forrás
tags-preset-period-detail = korabeli részlet
tags-pill-list = Címkék
tags-pill-add = Címke felvétele
tags-none-yet = Nincs címke
tags-pick-list = Választható címkék
tags-pill-remove = { $name } eltávolítása
tags-pill-filter-placeholder = Szűrés vagy új címke neve
tags-pill-no-match = Nincs egyező címke
tags-pill-create = „{ $name }” létrehozása
tags-pill-create-failed = Nem sikerült létrehozni: „{ $name }”
tags-pill-new-discoverable = Prózában keresett címke
tags-pill-new-discoverable-hint = Az ilyen címkét viselő elemeket a program megkeresi a prózában, hogy feltöltse a szereplőlistát.
tags-alias-list = Más néven
tags-alias-add = Másik név hozzáadása
tags-alias-remove = { $name } eltávolítása
tags-alias-placeholder = Másik név, majd Enter
tags-alias-hint = Nevek, amelyeken a címén kívül szerepel a prózában.
tags-alias-collision = Ezt a nevet már viseli: { $name }.
settings-page-tags = Címkék
settings-tags-desc = A címkék az iratrendező elemeit jelölik. A prózában keresett címke arra is utasítja a Skribistót, hogy keresse az adott elem neveit a prózájában.
settings-tags-add = Címke hozzáadása
settings-tags-add-placeholder = Az új címke neve
settings-tags-added = Hozzáadva: „{ $name }”
settings-tags-duplicate = A(z) „{ $name }” már létezik
settings-tags-filter = Címkék szűrése
settings-tags-count = { $n ->
    [one] 1 címke
   *[other] { $n } címke
}
settings-tags-details-placeholder = Mit jelent ez a címke
settings-tags-discoverable = Keresés a prózában
settings-tags-creates-in = Az új jegyzetek helye
settings-tags-creates-in-unset = Első alkalommal kérdezzen rá
settings-tags-creates-in-untitled = Névtelen mappa
settings-tags-creates-in-trashed = { $name } (a kukában)
settings-tags-template = Kezdősablon
settings-tags-template-unset = Üres jegyzet
settings-tags-delete = { $name } törlése
settings-tags-deleted = „{ $name }” törölve, és minden elemről eltávolítva
settings-tags-empty = Még nincsenek címkék.
settings-tags-apply-preset = Előre beállított készlet alkalmazása…
settings-tags-preset-applied = Hozzáadva: { $added }, kihagyva { $skipped } már meglévő
settings-tags-csv-filter = CSV-fájlok
settings-tags-import = Importálás…
settings-tags-export = Exportálás…
settings-tags-imported = Importálva: { $added }, kihagyva: { $skipped }
settings-tags-exported = Exportálva: { $n ->
    [one] 1 címke
   *[other] { $n } címke
}
tags-chip-more = { $n ->
    [one] még 1 címke
   *[other] még { $n } címke
}
cast-section = Szereplők
cast-add = Hozzáadás a szereplőkhöz…
cast-add-filter-placeholder = Szűrés a történetbibliában…
cast-add-empty = Nincs hozzáadható történetbiblia-bejegyzés
cast-pin = { $name } hozzáadása a szereplőkhöz
cast-unpin = { $name } eltávolítása a szereplők közül
cast-empty = Még senki sincs rögzítve. Adjon hozzá valakit, vagy tartson meg egy javaslatot
cast-unresolved = Már nincs a történetbibliában
mentions-confirm = Hozzáadás a(z) { $name } szereplőihez
mentions-hit-count = { $n ->
    [one] egyszer
   *[other] { $n } alkalommal
}
mentions-point-of-view-badge = Nézőpont
mentions-point-of-view-badge-tooltip = Itt nézőpontként megadva, kézzel beállítva, nem a prózából felismerve.
mentions-roster = Szereplők
mentions-pin = { $name } hozzáadása a szereplőkhöz
pov-section = Nézőpont
pov-empty = Nincs megadva nézőpont
pov-add = Nézőpont megadása…
pov-multiple = Ennek a jelenetnek két nézőpontja van.
pov-remove = { $name } eltávolítása a nézőpontból
pov-unresolved = Ide egy nézőpont volt rögzítve, de az a bejegyzés már nem létezik, vagy elvesztette a történetbiblia-címkéjét.
books-section = Könyvekhez rendelve
books-empty = Még nincs könyvhöz rendelve
books-add = Könyvhöz rendelés…
books-remove = Eltávolítás innen: { $name }
books-apply-to-children = A könyvhöz rendelés alkalmazása a gyermekelemekre
note-details-name-placeholder = Név…
note-details-tags = Címkék
note-details-aliases = Más nevek
note-details-books = Könyvekhez rendelve
note-details-books-empty = Még nincs könyvhöz rendelve
note-details-links = Kapcsolódó bejegyzések
note-details-links-empty = Még nincsenek kapcsolódó bejegyzések. Adjon hozzá egyet, hogy a történetbiblia egy másik bejegyzésére mutasson.
note-details-links-add = Kapcsolás egy jegyzethez…
note-details-pov = Nézőpont
note-details-pov-empty = Nincs megadva nézőpont
note-details-pov-unresolved = Ide egy nézőpont volt rögzítve, de az a bejegyzés már nem létezik, vagy elvesztette a történetbiblia-címkéjét.
note-details-pov-multiple = Ennek a jegyzetnek két nézőpontja van.
note-details-backlinks = Előfordulások a kéziratban
note-details-backlinks-empty = Még semmi. Amint ez a név megjelenik a prózájában, itt is feltűnik.
note-details-backlinks-outside = A könyveken kívül
note-details-backlinks-confirm-all = Minden előfordulás megerősítése
note-details-backlinks-confirm-all-tooltip = A bejegyzés hozzáadása minden itt látható dokumentum szereplőihez, ahol még nincs megerősítve
note-details-untitled-document = Névtelen
note-in-prose-mentions = { $n ->
    [one] 1 említés
   *[other] { $n } említés
}
note-in-prose-mention-at = { $current } / { $total }
note-in-prose-mention-previous = Előző említés
note-in-prose-mention-next = Következő említés
