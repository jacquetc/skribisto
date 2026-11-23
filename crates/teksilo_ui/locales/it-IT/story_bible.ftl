# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

story-bible-modal-configure-title = Configura la nuova voce
story-bible-modal-name-label = Nome
story-bible-modal-name-placeholder = Un personaggio, un luogo, qualcosa da annotare…
story-bible-modal-no-binders = Questo progetto non ha ancora raccoglitori in cui inserirla
story-bible-modal-template-label = Inizia da un modello (facoltativo)
story-bible-modal-template-placeholder = Nessun modello
story-bible-modal-body-label = Testo
story-bible-modal-cancel = Annulla
story-bible-modal-create = Crea
story-bible-modal-create-and-open = Crea e apri
story-bible-modal-failed = Impossibile creare la voce
ctx-add-as-note = Aggiungi come &nota…
ctx-add-as-note-all-tags = Tutte le etichette…
ctx-add-as-note-untagged = Senza etichetta
story-bible-grid-label = Bibbia narrativa
story-bible-grid-untagged = Non ancora etichettato
story-bible-grid-empty-title = Ancora nessuna voce qui
story-bible-grid-empty-hint = Assegna un'etichetta a una nota per trovarla nel testo, oppure aggiungine una da una selezione: apparirà qui.
story-bible-grid-alias-count = { $count ->
    [0] Ancora nessun altro nome
    [one] { $count } altro nome
   *[other] { $count } altri nomi
}
story-bible-grid-mention-count = { $count ->
    [0] Ancora nessuna presenza
    [one] Compare in { $count } scena nell'intero progetto
   *[other] Compare in { $count } scene nell'intero progetto
}
story-bible-books-filter-all = Tutti i libri
story-bible-grid-not-searched = Non ancora cercato: serve un'etichetta della bibbia narrativa
note-in-prose-pov = Punto di vista
note-in-prose-cast = Personaggi presenti
note-in-prose-pov-and-cast = Punto di vista · Personaggi presenti
note-in-prose-no-books = Questo progetto non ha ancora libri
note-in-prose-empty-book = Nessuna scena di questo libro ha ancora questa voce tra i personaggi presenti o come punto di vista.
note-in-prose-empty-viewpoint = Nessuna scena di questo libro è ancora narrata dal punto di vista di questa voce.
note-in-prose-scope = La pagina Nel testo mostra solo le scene che hai confermato. Una scena che nomina questa voce resta un suggerimento finché non aggiungi la voce ai suoi personaggi presenti, in Dettagli o nelle Proprietà.
note-in-prose-viewpoint-only = Solo scene dal suo punto di vista
note-in-prose-viewpoint-only-tooltip = Mostra solo le scene narrate dal punto di vista di questa voce e nascondi quelle in cui compare soltanto
story-bible-capture-toast = «{ $name }» aggiunto alla bibbia narrativa
story-bible-capture-open = Apri
story-bible-capture-undo = Annulla
story-bible-capture-where-title = Dove inserire queste note?
story-bible-capture-where-prompt = Scegli la cartella in cui inserire le nuove note.
story-bible-capture-where-confirm = Inserisci qui
story-bible-capture-where-needs-folder = Scegli una cartella che contenga queste note.
story-bible-capture-where-tag = Questa domanda viene posta una sola volta. D'ora in poi le note con l'etichetta «{ $tag }» verranno inserite qui. Puoi cambiare la destinazione in Impostazioni, Progetto, Etichette.
story-bible-capture-where-untagged = Questa domanda viene posta una sola volta. D'ora in poi le note senza etichetta verranno inserite qui. Puoi cambiare la destinazione in Impostazioni, Progetto, Etichette.

# What a screen reader hears for a binder row that is a story bible entry: its icon
# says so on screen, and this says it aloud. The second form keeps the row's label.
binder-row-story-bible-entry = Voce della bibbia narrativa
binder-row-story-bible-entry-with-label = Voce della bibbia narrativa, { $label }
