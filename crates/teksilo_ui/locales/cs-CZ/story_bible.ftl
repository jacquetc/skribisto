# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

story-bible-modal-configure-title = Nastavení nového záznamu
story-bible-modal-name-label = Jméno
story-bible-modal-name-placeholder = Postava, místo, cokoli, co stojí za zaznamenání…
story-bible-modal-no-binders = Tento projekt zatím nemá žádný pořadač, kam by šlo záznam zařadit
story-bible-modal-template-label = Začít ze šablony (nepovinné)
story-bible-modal-template-placeholder = Bez šablony
story-bible-modal-body-label = Text
story-bible-modal-cancel = Zrušit
story-bible-modal-create = Vytvořit
story-bible-modal-create-and-open = Vytvořit a otevřít
story-bible-modal-failed = Záznam nelze vytvořit

ctx-add-as-note = Přidat jako &poznámku…
ctx-add-as-note-all-tags = Všechny štítky…
ctx-add-as-note-untagged = Bez štítku

story-bible-grid-label = Bible příběhu
story-bible-grid-untagged = Zatím bez štítku
story-bible-grid-empty-title = Zatím tu nic není zařazeno
story-bible-grid-empty-hint = Označte poznámku štítkem s hledáním v próze, nebo ji přidejte z výběru, a objeví se zde.
story-bible-grid-alias-count = { $count ->
    [0] Zatím žádná další jména
    [one] { $count } další jméno
    [few] { $count } další jména
   *[other] { $count } dalších jmen
}
story-bible-grid-mention-count = { $count ->
    [0] Zatím bez výskytu
    [one] Vyskytuje se v { $count } scéně v celém projektu
   *[other] Vyskytuje se v { $count } scénách v celém projektu
}
story-bible-books-filter-all = Všechny knihy
story-bible-grid-not-searched = Zatím nehledáno: potřebuje štítek bible příběhu

note-in-prose-pov = Úhel pohledu
note-in-prose-cast = Postavy
note-in-prose-pov-and-cast = Úhel pohledu · Postavy
note-in-prose-no-books = Tento projekt zatím nemá žádnou knihu
# The page lists only scenes where the writer placed the entry in the cast or the point of
# view, never the ones the scan merely found its name in, so an empty page says so: a bare
# "nothing here" read as a search that had failed.
note-in-prose-empty-book = Žádná scéna v této knize zatím nemá tento záznam mezi postavami ani jako úhel pohledu.
note-in-prose-empty-viewpoint = Žádná scéna v této knize zatím není vyprávěna z úhlu pohledu tohoto záznamu.
note-in-prose-scope = Zobrazení V próze ukazuje jen scény, které jste potvrdili. Scéna, která tento záznam zmiňuje, zůstává návrhem, dokud záznam nepřidáte mezi její postavy, v zobrazení Podrobnosti nebo v inspektoru.
# A switch beside the Book bar: keep only the scenes this entry narrates.
note-in-prose-viewpoint-only = Jen scény z jeho úhlu pohledu
note-in-prose-viewpoint-only-tooltip = Zobrazí jen scény vyprávěné z úhlu pohledu tohoto záznamu a skryje ty, ve kterých se jen objevuje

story-bible-capture-toast = „{ $name }“ bylo přidáno do bible příběhu
story-bible-capture-open = Otevřít
story-bible-capture-undo = Vrátit zpět
story-bible-capture-where-title = Kam tyto poznámky ukládat?
story-bible-capture-where-prompt = Zvolte složku, do které se mají nové poznámky zařazovat.
story-bible-capture-where-confirm = Zařadit sem
story-bible-capture-where-needs-folder = Zvolte složku, do které se tyto poznámky mají ukládat.
story-bible-capture-where-tag = Ptáme se jen jednou. Poznámky se štítkem „{ $tag }“ budou odteď ukládány sem a později to můžete změnit v Nastavení, Projekt, Štítky.
story-bible-capture-where-untagged = Ptáme se jen jednou. Poznámky bez štítku budou odteď ukládány sem a později to můžete změnit v Nastavení, Projekt, Štítky.

# What a screen reader hears for a binder row that is a story bible entry: its icon
# says so on screen, and this says it aloud. The second form keeps the row's label.
binder-row-story-bible-entry = Záznam bible příběhu
binder-row-story-bible-entry-with-label = Záznam bible příběhu, { $label }
