# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Risorse: file e collegamenti web allegati agli elementi del raccoglitore.

## Il pannello

resources-title = Risorse
shortcut-name-resources-show = Mostra le risorse
resources-add-files = Aggiungi file
resources-add-files-title = Aggiungi file a questo elemento
resources-add-link = Aggiungi un collegamento web
resources-drop-hint = Trascina qui file o indirizzi web per allegarli
resources-no-item = Apri un elemento per vederne le risorse.
resources-empty = A questo elemento non è ancora allegato nulla. Aggiungi file o collegamenti web con i pulsanti qui sopra, oppure trascinali qui.
resources-actions = Azioni
resources-not-found = Non trovato
resources-row-linked = File collegato
resources-row-linked-size = File collegato, { $size }
resources-row-name = { $title }, { $kind }, { $detail }
resources-row-name-missing = { $title }, { $kind }, { $detail }, non trovato

## I tipi di file, come li annuncia un lettore di schermo

resources-kind-picture = immagine
resources-kind-pdf = documento PDF
resources-kind-audio = registrazione audio
resources-kind-video = video
resources-kind-document = documento
resources-kind-ebook = e-book
resources-kind-archive = archivio
resources-kind-file = file
resources-kind-web-link = collegamento web

## Il menu di una riga

resources-open = Apri
resources-show-in-folder = Mostra nella cartella
resources-locate = Ritrova…
resources-insert-into-text = Inserisci nel testo
resources-insert-picture = Inserisci l'immagine nel testo
resources-insert-link = Inserisci un collegamento nel testo
resources-edit = Rinomina e descrivi…
resources-detach = Stacca da questo elemento
resources-remove = Rimuovi dal progetto…

## Le due finestre di dialogo

resources-link-dialog-title = Aggiungi un collegamento web
resources-link-dialog-address = Indirizzo
resources-link-dialog-address-placeholder = https://
resources-link-dialog-name = Titolo
resources-link-dialog-name-placeholder = Il nome del sito, se lo lasci vuoto
resources-link-dialog-add = Aggiungi
resources-edit-dialog-title = Rinomina e descrivi
resources-edit-dialog-name = Titolo
resources-edit-dialog-description = Descrizione
resources-edit-dialog-description-placeholder = Che cos'è, e perché l'hai tenuto
resources-edit-dialog-save = Salva
resources-dialog-cancel = Annulla

## Aggiungere

resources-large-title = Un file grande
resources-large-text = «{ $name }» pesa { $size }. Conservato nel progetto, viaggia con ogni copia e ogni backup del progetto. Collegato, resta dov'è, e il progetto ricorda solo dove trovarlo.
resources-large-text-many = «{ $name }» pesa { $size }, e { $others ->
    [one] anche un altro file è grande
   *[other] anche altri { $others } file sono grandi
}. Conservati nel progetto, viaggiano con ogni copia e ogni backup del progetto. Collegati, restano dove sono, e il progetto ricorda solo dove trovarli.
resources-large-store = Conservalo nel progetto
resources-large-link = Collegalo invece
resources-large-remember = Non chiedere più
resources-storing-title = Aggiunta dei file al progetto
resources-store-cancel = Annulla
resources-store-cancelled = Nessun file è stato aggiunto.
resources-store-busy = Alcuni file sono ancora in fase di aggiunta. Riprova quando saranno arrivati.
resources-store-failed = Non è stato possibile aggiungere i file: { $error }
resources-no-project = Apri un progetto prima di allegare file.
resources-moved-away = Il progetto è stato spostato mentre il file veniva copiato.
resources-some-failed = { $count ->
    [one] Non è stato possibile aggiungere un file: { $names }
   *[other] Non è stato possibile aggiungere { $count } file: { $names }
}
resources-add-failed = Non è stato possibile aggiungere le risorse: { $error }
resources-added = { $count ->
    [one] Una risorsa aggiunta
   *[other] { $count } risorse aggiunte
}
resources-dropped-added = { $count ->
    [one] Aggiunto come risorsa di questo elemento, e collegato nel testo
   *[other] { $count } file aggiunti come risorse di questo elemento, e collegati nel testo
}
resources-undo = Annulla
resources-link-refused = «{ $url }» non è un indirizzo web che Skribisto possa tenere. Usa un indirizzo che inizi con http:// o https://, senza nome utente né password.

## L'avviso per un progetto in un unico file

resources-folder-title = Questo progetto sta diventando grande
resources-folder-text = I suoi file conservati occupano ora { $size }. Un progetto tenuto in un unico file viene riscritto per intero a ogni salvataggio, file compresi. Tenuto come cartella, salva solo ciò che è cambiato. Puoi salvarlo come cartella ora, e da quel momento continuare a lavorare nella cartella.
resources-folder-switch = Salva come cartella…
resources-folder-not-now = Non ora
resources-folder-never = Non chiedere più per questo progetto

## Aprire

resources-not-found-toast = «{ $name }» non si trova dove il progetto se lo aspetta.
resources-unsafe-title = Aprire «{ $name }»?
resources-unsafe-text = Skribisto non sa se questo tipo di file sia sicuro. Aprirlo potrebbe avviare un programma sul tuo computer, e un progetto può provenire da qualcun altro. Mostralo prima nella sua cartella per vedere che cos'è.
resources-open-anyway = Apri comunque
resources-open-failed = Non è stato possibile aprire il file: { $error }
resources-no-data-folder = questo sistema non fornisce a Skribisto alcuna cartella di dati in cui aprire una copia
resources-link-gone = Questo collegamento porta a una risorsa che il progetto non ha più.
resources-locate-title = Dove si trova ora «{ $name }»?
resources-relinked-more = { $count ->
    [one] Un altro file mancante è stato trovato nella stessa cartella, e collegato di nuovo.
   *[other] Altri { $count } file mancanti sono stati trovati nella stessa cartella, e collegati di nuovo.
}
resources-changed-elsewhere = «{ $name }» è stato modificato in un'altra app. Il progetto tiene la nuova versione.

## Modificare

resources-edit-failed = Non è stato possibile modificare la risorsa: { $error }
resources-remove-title = Rimuovere «{ $title }» dal progetto?
resources-remove-text = Viene tolta da ogni elemento a cui è allegata. Un file conservato nel progetto se ne va con essa; un file collegato e una pagina web restano dove sono. Annulla la riporta indietro.
resources-remove-confirm = Rimuovi
resources-remove-failed = Non è stato possibile rimuovere la risorsa: { $error }

## Nel testo

resources-no-text = Posiziona prima il cursore in un testo, poi inserisci lì la risorsa.
resources-drop-insert-picture = Inserisci l'immagine
resources-drop-insert-link = Inserisci un collegamento alla risorsa

## Avanzamento

progress-resources-storing = Copia dei file
progress-resources-stored = File copiati
