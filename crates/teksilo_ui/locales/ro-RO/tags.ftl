# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

tags-preset-basic = De bază
tags-preset-scifi = Științifico-fantastic
tags-preset-fantasy = Fantezie
tags-preset-mystery = Mister
tags-preset-historical = Istoric


tags-preset-needs-research = necesită cercetare
tags-preset-continuity-check = verificarea continuităţii
tags-preset-plot-point = punct de complot

tags-preset-character = caracter
tags-preset-place = loc
tags-preset-item = element

tags-preset-vessel = navă
tags-preset-planet = planeta
tags-preset-organization = organizare
tags-preset-creature = creatură
tags-preset-faction = Facţiune
tags-preset-artifact = artefact
tags-preset-realm = tărâm
tags-preset-magic-system = sistem magic
tags-preset-suspect = suspect
tags-preset-victim = victimă
tags-preset-clue = indiciu
tags-preset-red-herring = hering roșu
tags-preset-historical-figure = figura istorică
tags-preset-source = sursă
tags-preset-period-detail = Detalii privind perioada

tags-pill-list = Etichete
tags-pill-add = Adaugă o etichetă
tags-none-yet = Fără etichete
tags-pick-list = Etichete de ales de la
tags-pill-remove = Elimină { $name }
tags-pill-filter-placeholder = Filtru sau nume o etichetă nouă
tags-pill-no-match = Nicio etichetă nu se potriveşte.
tags-pill-create = Creează "{ $name }"
tags-pill-create-failed = Nu am putut crea "{ $name }"
tags-pill-new-discoverable = Etichetă de căutare în Prose
tags-pill-new-discoverable-hint = Obiectele cu această etichetă sunt potrivite împotriva prozei tale pentru a umple lista.

tags-alias-list = Cunoscut şi sub numele de
tags-alias-add = Adaugă un alt nume
tags-alias-remove = Elimină { $name }
tags-alias-placeholder = Un alt nume, apoi Enter
tags-alias-hint = Numele acestea apar în proza ta, pe lângă titlul ei.
tags-alias-collision = { $name } răspunde deja la acest nume.

settings-page-tags = Etichete
settings-tags-desc = Etichetele etichetează elementele din liant. O etichetă găsi-in-prose, de asemenea, spune Skribisto pentru a căuta numele acelui produs în proza ta.
settings-tags-add = Adaugă etichetă
settings-tags-add-placeholder = Nume o etichetă nouă
settings-tags-added = Adăugat "{ $name }"
settings-tags-duplicate = "{ $name }" există deja
settings-tags-filter = Etichete filtru
settings-tags-count = { $n ->
    [one] 1 etichetă
   *[other] { $n } tags
}
settings-tags-details-placeholder = Ce înseamnă această etichetă
settings-tags-discoverable = Caută în proză
settings-tags-creates-in = Notițe noi merge la
settings-tags-creates-in-unset = Întreabă-mă de prima dată.
settings-tags-creates-in-untitled = Dosar fără titlu
settings-tags-creates-in-trashed = { $name } (la gunoi)
settings-tags-template = Șablon de pornire
settings-tags-template-unset = Notă albă
settings-tags-delete = Se elimină punctul { $name }.
settings-tags-deleted = Se elimină "{ $name }" și se elimină din fiecare element
settings-tags-empty = Nici o etichetă încă.
settings-tags-apply-preset = Aplică un preset...
settings-tags-preset-applied = Adăugat { $added }, sărit { $skipped } deja prezente
settings-tags-csv-filter = Fișiere CSV
settings-tags-import = Importare…
settings-tags-export = Exportare…
settings-tags-imported = Importat { $added }, sărit { $skipped }
settings-tags-exported = Exportat { $n ->
    [one] 1 etichetă
   *[other] { $n } tags
}

tags-chip-more = { $n ->
    [one] 1 mai mult etichetă
   *[other] { $n } mai multe etichete
}

cast-section = Personaje
cast-add = Adaugă în distribuţie...
cast-add-filter-placeholder = Filtrare Biblie poveste...
cast-add-empty = Nu există intrări-biblie povești de adăugat
cast-pin = Se adaugă { $name } la distribuţie
cast-unpin = Elimină { $name } din ghips
cast-empty = Nimeni nu a fost încă prins.
cast-unresolved = Nu mai e în Biblie.

mentions-confirm = Adauga la distributia de { $name }
mentions-hit-count = { $n ->
    [one] o dată
   *[other] { $n } ori
}
mentions-point-of-view-badge = Punct de vedere
mentions-point-of-view-badge-tooltip = Declarat ca punct de vedere aici, stabilit manual, nedetectat în proza.

mentions-roster = Personaje
mentions-pin = Se adaugă { $name } la distribuţie

pov-section = Punct de vedere
pov-empty = Niciun punct de vedere stabilit
pov-add = Setează punctul de vedere...
pov-multiple = Această scenă are două puncte de vedere.
pov-remove = Elimină ca punct de vedere { $name }
pov-unresolved = Un punct de vedere a fost fixat aici, dar acea intrare nu mai există sau şi-a pierdut eticheta biblică.

books-section = Filate sub
books-empty = Nu a fost depusă sub o carte încă
books-add = Fișier sub o carte...
books-remove = Elimină din { $name }
books-apply-to-children = Aplică depunerea la copii

note-details-name-placeholder = Nume…
note-details-tags = Etichete
note-details-aliases = Alte denumiri
note-details-books = Filate sub
note-details-books-empty = Încă nu s-a depus sub o carte
note-details-links = Legături
note-details-links-empty = Adăugaţi unul la punctul de la o altă intrare în Biblia ta poveste.
note-details-links-add = Link către un bilet...
note-details-pov = Punct de vedere
note-details-pov-empty = Niciun punct de vedere stabilit
note-details-pov-unresolved = Un punct de vedere a fost fixat aici, dar acea intrare nu mai există sau şi-a pierdut eticheta biblică.
note-details-pov-multiple = Această notă are două puncte de vedere.
note-details-backlinks = Apare în manuscris
note-details-backlinks-empty = Odată ce acest nume apare în proza ta, va apărea aici.
note-details-backlinks-outside = În afara cărților
note-details-backlinks-confirm-all = Confirmă fiecare aspect
note-details-backlinks-confirm-all-tooltip = Adăugați această rubrică în distribuția fiecărui document indicat aici care nu a confirmat-o încă
note-details-untitled-document = Fără titlu

note-in-prose-mentions = { $n ->
    [one] 1 mențiune
   *[other] { $n } menţionează
}
note-in-prose-mention-at = { $current } din { $total }
note-in-prose-mention-previous = Mențiune anterioară
note-in-prose-mention-next = Următoarea mențiune
