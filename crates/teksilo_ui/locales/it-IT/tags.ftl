# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

tags-preset-basic = Base
tags-preset-scifi = Fantascienza
tags-preset-fantasy = Fantasy
tags-preset-mystery = Giallo
tags-preset-historical = Storico
tags-preset-needs-research = richiede ricerche
tags-preset-continuity-check = verifica della continuità
tags-preset-plot-point = snodo narrativo
tags-preset-character = personaggio
tags-preset-place = luogo
tags-preset-item = oggetto
tags-preset-vessel = nave
tags-preset-planet = pianeta
tags-preset-organization = organizzazione
tags-preset-creature = creatura
tags-preset-faction = fazione
tags-preset-artifact = manufatto
tags-preset-realm = regno
tags-preset-magic-system = sistema magico
tags-preset-suspect = sospettato
tags-preset-victim = vittima
tags-preset-clue = indizio
tags-preset-red-herring = falsa pista
tags-preset-historical-figure = figura storica
tags-preset-source = fonte
tags-preset-period-detail = dettaglio d'epoca
tags-pill-list = Etichette
tags-pill-add = Aggiungi un'etichetta
tags-none-yet = Nessuna etichetta
tags-pick-list = Etichette disponibili
tags-pill-remove = Rimuovi { $name }
tags-pill-filter-placeholder = Filtra o assegna un nome a una nuova etichetta
tags-pill-no-match = Nessuna etichetta corrispondente
tags-pill-create = Crea «{ $name }»
tags-pill-create-failed = Impossibile creare «{ $name }»
tags-pill-new-discoverable = Etichetta da cercare nel testo
tags-pill-new-discoverable-hint = I nomi degli elementi con questa etichetta vengono cercati nel testo per compilare l'elenco dei personaggi presenti.
tags-alias-list = Altri nomi
tags-alias-add = Aggiungi un altro nome
tags-alias-remove = Rimuovi { $name }
tags-alias-placeholder = Un altro nome, poi Invio
tags-alias-hint = Nomi con cui compare nel testo, oltre al titolo.
tags-alias-collision = { $name } è già conosciuto con questo nome.
settings-page-tags = Etichette
settings-tags-desc = Le etichette contrassegnano gli elementi del raccoglitore. Un'etichetta da cercare nel testo indica anche a Skribisto di cercare i nomi di quell'elemento nel testo.
settings-tags-add = Aggiungi etichetta
settings-tags-add-placeholder = Nome della nuova etichetta
settings-tags-added = Aggiunta «{ $name }»
settings-tags-duplicate = «{ $name }» esiste già
settings-tags-filter = Filtra le etichette
settings-tags-count = { $n ->
    [one] 1 etichetta
   *[other] { $n } etichette
}
settings-tags-details-placeholder = Significato dell'etichetta
settings-tags-discoverable = Cerca nel testo
settings-tags-creates-in = Destinazione delle nuove note
settings-tags-creates-in-unset = Chiedimelo la prima volta
settings-tags-creates-in-untitled = Cartella senza titolo
settings-tags-creates-in-trashed = { $name } (nel cestino)
settings-tags-template = Modello iniziale
settings-tags-template-unset = Nota vuota
settings-tags-delete = Elimina { $name }
settings-tags-deleted = «{ $name }» eliminata e rimossa da tutti gli elementi
settings-tags-empty = Ancora nessuna etichetta.
settings-tags-apply-preset = Applica un insieme predefinito…
settings-tags-preset-applied = Aggiunte: { $added }; già presenti e ignorate: { $skipped }
settings-tags-csv-filter = File CSV
settings-tags-import = Importa…
settings-tags-export = Esporta…
settings-tags-imported = Importate: { $added }; ignorate: { $skipped }
settings-tags-exported = { $n ->
    [one] 1 etichetta esportata
   *[other] { $n } etichette esportate
}
tags-chip-more = { $n ->
    [one] 1 altra etichetta
   *[other] { $n } altre etichette
}
cast-section = Personaggi presenti
cast-add = Aggiungi ai personaggi presenti…
cast-add-filter-placeholder = Filtra la bibbia narrativa…
cast-add-empty = Nessuna voce della bibbia narrativa da aggiungere
cast-pin = Aggiungi { $name } ai personaggi presenti
cast-unpin = Rimuovi { $name } dai personaggi presenti
cast-empty = Ancora nessuna presenza confermata. Aggiungine una o accetta un suggerimento
cast-unresolved = Non più nella bibbia narrativa
mentions-confirm = Aggiungi ai personaggi presenti di { $name }
mentions-hit-count = { $n ->
    [one] una volta
   *[other] { $n } volte
}
mentions-point-of-view-badge = Punto di vista
mentions-point-of-view-badge-tooltip = Punto di vista dichiarato qui manualmente, non rilevato nel testo.
mentions-roster = Personaggi presenti
mentions-pin = Aggiungi { $name } ai personaggi presenti
pov-section = Punto di vista
pov-empty = Nessun punto di vista impostato
pov-add = Imposta punto di vista…
pov-multiple = Questa scena ha due punti di vista.
pov-remove = Rimuovi { $name } come punto di vista
pov-unresolved = Qui era stato fissato un punto di vista, ma la voce non esiste più o ha perso l'etichetta della bibbia narrativa.
books-section = Associato a
books-empty = Non ancora associato a un libro
books-add = Associa a un libro…
books-remove = Rimuovi da { $name }
books-apply-to-children = Applica l'associazione agli elementi contenuti
note-details-name-placeholder = Nome…
note-details-tags = Etichette
note-details-aliases = Altri nomi
note-details-books = Associato a
note-details-books-empty = Non ancora associato a un libro
note-details-links = Collegamenti
note-details-links-empty = Ancora nessun collegamento. Aggiungine uno per rimandare a un'altra voce della bibbia narrativa.
note-details-links-add = Collega a una nota…
note-details-pov = Punto di vista
note-details-pov-empty = Nessun punto di vista impostato
note-details-pov-unresolved = Qui era stato fissato un punto di vista, ma la voce non esiste più o ha perso l'etichetta della bibbia narrativa.
note-details-pov-multiple = Questa nota ha due punti di vista.
note-details-backlinks = Compare nel manoscritto
note-details-backlinks-empty = Ancora nulla. Quando questo nome comparirà nel testo, verrà elencato qui.
note-details-backlinks-outside = Fuori dai libri
note-details-backlinks-confirm-all = Conferma tutte le presenze
note-details-backlinks-confirm-all-tooltip = Aggiungi questa voce ai personaggi presenti di ogni documento qui elencato che non l'ha ancora confermata
note-details-untitled-document = Senza titolo
note-in-prose-mentions = { $n ->
    [one] 1 menzione
   *[other] { $n } menzioni
}
note-in-prose-mention-at = { $current } di { $total }
note-in-prose-mention-previous = Menzione precedente
note-in-prose-mention-next = Menzione successiva
