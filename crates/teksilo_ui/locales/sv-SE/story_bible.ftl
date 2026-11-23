# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet
story-bible-modal-configure-title = Konfigurera den nya posten
story-bible-modal-name-label = Namn
story-bible-modal-name-placeholder = En person, en plats, vad som helst som är värt att arkivera…
story-bible-modal-no-binders = Projektet har ännu ingen pärm att arkivera den i
story-bible-modal-template-label = Utgå från en mall (valfritt)
story-bible-modal-template-placeholder = Ingen mall
story-bible-modal-body-label = Brödtext
story-bible-modal-cancel = Avbryt
story-bible-modal-create = Skapa
story-bible-modal-create-and-open = Skapa och öppna
story-bible-modal-failed = Det gick inte att skapa posten
ctx-add-as-note = Lägg till som &anteckning…
ctx-add-as-note-all-tags = Alla taggar…
ctx-add-as-note-untagged = Utan tagg
story-bible-grid-label = Berättelsebibel
story-bible-grid-untagged = Ännu inte taggade
story-bible-grid-empty-title = Inget arkiverat här ännu
story-bible-grid-empty-hint = Tagga en anteckning för att hitta den i prosan, eller lägg till en från en markering, så visas den här.
story-bible-grid-alias-count = { $count ->
       [0] Inga alias ännu
       [one] { $count } alias
      *[other] { $count } alias
   }
story-bible-grid-mention-count = { $count ->
       [0] Förekommer inte ännu
       [one] Förekommer i { $count } scen i hela projektet
      *[other] Förekommer i { $count } scener i hela projektet
   }
story-bible-books-filter-all = Alla böcker
story-bible-grid-not-searched = Inte sökt ännu: kräver en tagg för berättelsebibeln
note-in-prose-pov = Berättarperspektiv
note-in-prose-cast = Persongalleri
note-in-prose-pov-and-cast = Berättarperspektiv · Persongalleri
note-in-prose-no-books = Projektet har ingen bok ännu
note-in-prose-empty-book = Ingen scen i den här boken har posten i sitt persongalleri eller som berättarperspektiv ännu.
note-in-prose-empty-viewpoint = Ingen scen i den här boken berättas ur postens perspektiv ännu.
note-in-prose-scope = Vyn I prosan visar bara de scener som du har bekräftat. En scen som nämner posten förblir ett förslag tills du lägger till posten i dess persongalleri, under Detaljer eller i inspektören.
note-in-prose-viewpoint-only = Bara scener ur dess perspektiv
note-in-prose-viewpoint-only-tooltip = Visa bara de scener som berättas ur postens perspektiv, och dölj dem där den bara förekommer
story-bible-capture-toast = ”{ $name }” har lagts till i berättelsebibeln
story-bible-capture-open = Öppna
story-bible-capture-undo = Ångra
story-bible-capture-where-title = Var ska de här anteckningarna placeras?
story-bible-capture-where-prompt = Välj mappen där nya anteckningar ska arkiveras.
story-bible-capture-where-confirm = Arkivera här
story-bible-capture-where-needs-folder = Välj en mapp som anteckningarna ska placeras i.
story-bible-capture-where-tag = Frågas bara en gång. Anteckningar taggade ”{ $tag }” hamnar här från och med nu, och du kan ändra det senare under Inställningar, Projekt, Taggar.
story-bible-capture-where-untagged = Frågas bara en gång. Anteckningar utan tagg hamnar här från och med nu, och du kan ändra det senare under Inställningar, Projekt, Taggar.

# What a screen reader hears for a binder row that is a story bible entry: its icon
# says so on screen, and this says it aloud. The second form keeps the row's label.
binder-row-story-bible-entry = Post i berättelsebibeln
binder-row-story-bible-entry-with-label = Post i berättelsebibeln, { $label }
