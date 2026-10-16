# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Italiano (Italia). & indica il tasto di accesso del menu; && indica una & letterale.
menu-work = &Progetto
menu-new-work = &Nuovo progetto
menu-open-work = &Apri progetto…
menu-new-window = Nuova &finestra
menu-import-from = &Importa da
menu-create-from = C&rea da
menu-import-plume = &Plume Creator (.plume)…
menu-import-document = &Documenti (Markdown, Word, ODT)…
menu-import-manuskript = &Manuskript (.msk)…
menu-import-scrivener = &Scrivener (.scriv)…
menu-export = &Esporta
menu-export-book = Esporta libro
menu-export-part = Esporta parte
menu-export-chapter = Esporta capitolo
menu-export-scene = Esporta scena
menu-export-note = Esporta nota
menu-export-paratext = Esporta paratesto
menu-export-folder = Esporta cartella
menu-export-choose = Scegli…
menu-export-none = Apri un documento da esportare
menu-save = &Salva
menu-save-as-file = Sa&lva come singolo file…
menu-save-as-folder = Salva come ca&rtella…
menu-backup = Esegui &backup ora
menu-close-work = C&hiudi progetto
menu-welcome = Benvenu&to…
menu-settings = Imp&ostazioni
menu-quit = Es&ci
menu-view = &Visualizza
menu-outline = &Struttura
menu-search = &Cerca nel progetto
menu-timeline = &Torna indietro nel tempo
menu-search-preview = &Anteprima della ricerca
menu-fullscreen = Schermo &intero
menu-focus-mode = Mo&dalità senza distrazioni
menu-format = F&ormato
menu-scene-break = In&serisci separatore di scena
menu-major-scene-break = Inserisci separatore di scena &maggiore
menu-go = V&ai
menu-go-next-scene = &Scena successiva
menu-go-prev-scene = Scena &precedente
menu-go-next-chapter = &Capitolo successivo
menu-go-prev-chapter = Capitolo p&recedente
menu-go-next-note = &Nota successiva
menu-go-prev-note = Nota pr&ecedente
menu-tools = S&trumenti
menu-help = Ai&uto
ctx-add = &Aggiungi
ctx-new-item = &Nuovo elemento
ctx-new-folder = Nuova &cartella
ctx-rename = &Rinomina
ctx-duplicate = &Duplica
ctx-import-here = &Importa qui…
ctx-indent = Aume&nta rientro
ctx-outdent = Riduci ri&entro
ctx-trash = Sposta nel &cestino
ctx-open-to-side = Apri a &lato
ctx-open = &Apri
ctx-reveal-in-outline = Mostra nella &struttura
ctx-move-up = Sposta in &alto
ctx-move-down = Sposta in &basso
ctx-tab-close = &Chiudi
ctx-tab-close-others = Chiudi le &altre
ctx-tab-close-all = Chiudi &tutte
ctx-tab-open-to-side = Apri a &lato
ctx-tab-open-in-main = A&pri nel riquadro principale
ctx-tab-move-to-side = &Sposta nel riquadro laterale
ctx-tab-move-to-main = &Sposta nel riquadro principale
ctx-tab-move-to-new-window = Sposta i&n una nuova finestra
ctx-tab-move-window-unsaved = Salva prima questo progetto: una seconda finestra apre un file sul disco
ctx-tab-pin = &Fissa questa scheda
ctx-tab-unpin = S&fissa questa scheda
tab-pinned-tooltip = Fissata: «Chiudi le altre» e «Chiudi tutte» la lasciano aperta
create-book = Libro
create-part = Parte
create-chapter = Capitolo
create-scene = Scena
create-note = Nota
create-note-folder = Cartella di note
create-folder = Cartella
create-paratext = Paratesto
create-paratext-folder = Cartella di paratesti
create-book-end = Fine del libro
create-story-bible-entry = Voce della bibbia narrativa…
type-book-start = Inizio del libro
type-text = Testo
new-item-book = Nuovo libro
new-item-part = Nuova parte
new-item-chapter = Nuovo capitolo
new-item-note = Nuova nota
new-item-note-folder = Nuova cartella di note
new-item-folder = Nuova cartella
new-item-paratext = Nuovo paratesto
new-item-paratext-folder = Nuova cartella di paratesti
new-item-story-bible-entry = Nuova voce della bibbia narrativa
placement-inside = all'interno
placement-after = dopo
placement-after-parent = dopo l'elemento padre
placement-top-level = livello principale
ctx-promote = Con&verti in
promote-chapter-folder = Capitolo cartella
promote-flat-chapter = Capitolo singolo
promote-lossy-title = Nessuno spazio per il testo
promote-lossy-text = Il tipo «{ $target }» non può contenere: { $kinds }. Sposta o elimina prima quel testo, poi esegui la conversione.
content-scene-text = Testo della scena
content-note-text = Testo della nota
content-book-title = Titolo del libro
content-book-subtitle = Sottotitolo del libro
content-part-title = Titolo della parte
content-chapter-title = Titolo del capitolo
content-epigraph-text = Epigrafe
content-paratext-text = Paratesto
promote-blocked-title = Capitolo non vuoto
promote-blocked-text = { $count ->
    [one] Questo capitolo contiene ancora { $count } elemento. Spostalo altrove o nel cestino prima di convertirlo in un capitolo singolo.
   *[other] Questo capitolo contiene ancora { $count } elementi. Spostali altrove o nel cestino prima di convertirlo in un capitolo singolo.
}
inspector = Proprietà
inspector-empty = Apri un elemento per esaminarne le proprietà.
inspector-promote = Converti in…
statusbar-toggle-outline = Mostra o nascondi il raccoglitore
statusbar-toggle-inspector = Mostra o nascondi le proprietà
statusbar-save-unsaved = Modifiche non salvate. Fai clic per salvare
statusbar-save-saved = Tutte le modifiche sono salvate
statusbar-save-autosave = Salvataggio automatico attivo. Le modifiche vengono salvate mentre scrivi
statusbar-saving = Salvataggio…
statusbar-word-count = { $count ->
    [one] { $count } parola
   *[other] { $count } parole
}
statusbar-word-count-tooltip = Parole nella scena che stai modificando
statusbar-word-char-count = { $words ->
    [one] { $words } parola
   *[other] { $words } parole
} · { $chars ->
    [one] { $chars } carattere
   *[other] { $chars } caratteri
}
session-toggle = Sessione di scrittura: avvia o sospendi uno sprint di concentrazione
session-configure = Imposta l'obiettivo di parole e il limite di tempo della sessione
session-configure-title = Sessione di scrittura
session-word-goal = Obiettivo di parole
session-time-limit = Limite di tempo
session-time-limit-unit = min
session-no-goal = Nessun obiettivo
session-no-limit = Nessun limite
session-reset = Azzera sessione
session-readout = { $words ->
    [one] { $words } parola
   *[other] { $words } parole
} · { $time }
session-readout-timed = { $words ->
    [one] { $words } parola
   *[other] { $words } parole
} · { $time } rimanenti
statusbar-focus-exit = Esci dalla modalità senza distrazioni
statusbar-focus-synopsis = Sinossi
statusbar-focus-comments = Commenti
statusbar-focus-go-prev = Elemento precedente (Alt+Su)
statusbar-focus-go-next = Elemento successivo (Alt+Giù)
rail-settings = Impostazioni
language = Lingua
theme = Tema
english = Inglese
french = Francese
light = Chiaro
dark = Scuro
settings-text-width = Larghezza del testo
settings-preview-width = Larghezza dell'anteprima della ricerca
settings-autosave = Salvataggio automatico su disco
settings-show-welcome = Mostra la schermata iniziale all'avvio
settings-show-welcome-tip = Se disattivato, viene riaperto l'ultimo progetto.
settings-check-for-updates = Cerca nuove versioni
settings-check-for-updates-tip = Al massimo una volta al giorno, Skribisto chiede al sito del progetto quale sia la versione attuale. La richiesta non contiene identificativi, nemmeno la versione in uso: il confronto avviene su questo computer.
settings-title = Impostazioni
settings-close = Chiudi
settings-search = Cerca nelle impostazioni
settings-reset = Ripristina valori predefiniti
settings-done = Fine
settings-cancel = Annulla
settings-apply = Applica
settings-ok = OK
settings-reset-confirm-title = Ripristinare tutte le impostazioni predefinite?
settings-reset-confirm-body = Ripristina l'aspetto dell'applicazione e le impostazioni dell'editor e della scrittura. Le impostazioni del progetto, le scorciatoie da tastiera e gli stili salvati restano invariati. L'operazione non può essere annullata.
settings-empty-title = Nessuna impostazione disponibile
settings-empty-hint = Questa sezione riceverà nuove opzioni in un prossimo aggiornamento.
settings-sec-appearance-behaviour = Aspetto e comportamento
settings-sec-editor = Editor
settings-sec-spelling = Ortografia
settings-sec-backup = Backup e sincronizzazione
settings-sec-compile = Compilazione ed esportazione
settings-page-appearance = Aspetto
settings-field-image-size-policy = Immagini grandi
settings-image-policy-ask = Chiedi ogni volta
settings-image-policy-keep = Mantieni l'originale
settings-image-policy-downscale = Ottimizza
settings-hint-image-size-policy = Come gestire un'immagine più grande del necessario per il manoscritto.
settings-page-notifications = Notifiche
settings-page-scene = Scena
settings-page-synopsis = Sinossi
settings-page-notes = Note
settings-page-editor-behavior = Comportamento dell'editor
settings-page-goals = Obiettivi e conteggio parole
settings-page-games = Giochi di scrittura
settings-page-corkboard = Bacheca
settings-page-distraction-free = Senza distrazioni
settings-page-dictionaries = Dizionari
settings-page-autosave = Salvataggio automatico
settings-page-export = Formati di esportazione
settings-page-paratext = Strutture del paratesto
settings-page-keymap = Scorciatoie da tastiera
settings-keymap-filter = Filtra scorciatoie
settings-desc-sec-appearance-behaviour = L'aspetto dell'applicazione e il suo comportamento all'avvio.
settings-desc-sec-editor = La superficie di scrittura: aspetto e comportamento durante la digitazione.
settings-desc-sec-spelling = Il controllo ortografico e i dizionari che usa.
settings-desc-sec-backup = Come il progetto viene salvato e quali copie vengono conservate.
settings-desc-sec-compile = Cosa esce da Skribisto e in quale forma.
settings-desc-sec-work = Impostazioni che appartengono al progetto e viaggiano nel suo file.
settings-desc-sec-extensions = Pagine fornite dalle estensioni installate.
settings-desc-group-typography = Una pagina per tipo di testo: carattere, dimensione, altezza delle righe e spaziatura.
settings-desc-appearance = Lingua dell'interfaccia, tema, dimensione del testo e schermata iniziale.
settings-desc-notifications = Tutti i messaggi di questa sessione e le azioni che puoi ripetere.
settings-desc-scene = Aspetto del testo delle scene: carattere, dimensione, altezza delle righe, rientri e spaziatura.
settings-desc-synopsis = Aspetto del riquadro della sinossi, indipendente dal manoscritto.
settings-desc-notes = Aspetto delle note, indipendente dal manoscritto.
settings-desc-corkboard = Dimensione e contenuto delle schede e disposizione nella bacheca.
settings-desc-distraction-free = Tipografia, larghezza della colonna e barra dei comandi per scrivere a schermo intero.
settings-desc-distraction-free-themes = La raccolta dei temi per scrivere a schermo intero: quelli integrati e i tuoi.
settings-desc-editor-behavior = Larghezza del testo, posizione della sinossi, scorrimento a macchina da scrivere ed evidenziazione intorno al cursore.
settings-desc-punctuation = Le convenzioni tipografiche iniziali dei nuovi progetti. Ogni progetto le segue, salvo impostarne di proprie.
settings-desc-goals = Obiettivi di parole e caratteri e metodi di conteggio.
settings-desc-games = Vincoli che scegli durante la stesura, come «Sempre avanti».
settings-desc-spellcheck = Un unico interruttore per attivare o disattivare ovunque il controllo ortografico.
settings-desc-dictionaries = Installa, rimuovi ed esplora i dizionari ortografici su questo computer.
settings-desc-autosave = Se le modifiche vengono scritte automaticamente sul disco.
settings-desc-backup = Quando creare copie, dove conservarle e quante tenerne. Ogni progetto segue queste regole, salvo impostarne di proprie.
settings-desc-export = Gli stili usati da ogni esportazione: quelli integrati e i tuoi.
settings-desc-paratext = Le pagine preliminari e finali con cui iniziare un nuovo progetto.
settings-desc-keymap = Tutte le scorciatoie e le loro assegnazioni.
settings-page-user = Utente
settings-desc-user = La tua identità per i commenti che scrivi.
settings-group-identity = Identità
settings-field-user-name = Il tuo nome
settings-field-user-name-placeholder = Facoltativo
settings-field-user-name-hint = Firma i commenti e le risposte che scrivi. È distinto dal nome dell'autore del progetto, che compare sul libro e viaggia nel file.
settings-field-user-initials = Le tue iniziali
settings-field-user-initials-placeholder = Dal tuo nome
settings-field-user-initials-hint = Ciò che un elaboratore di testi mostra a margine accanto al commento. Lascia vuoto per usare quelle visualizzate, ricavate dal tuo nome.
settings-field-user-hint = Entrambi sono facoltativi e valgono per tutti i progetti su questo computer. Le modifiche firmano i commenti futuri; quelli già scritti mantengono il nome originale.
settings-desc-author = Il nome che compare su questo progetto.
settings-desc-structure = Se i capitoli del progetto sono cartelle o elementi singoli.
settings-desc-language = La lingua usata per controllare l'ortografia del progetto.
settings-desc-work-backup = Le regole di backup del progetto o quelle generali.
settings-desc-personal-dictionary = Le parole considerate corrette in questo progetto.
settings-desc-tags = Le etichette colorate usate per contrassegnare gli elementi del progetto.
settings-desc-templates = I modelli di nota che puoi inserire mentre scrivi in questo progetto.
settings-desc-text-replacements = Le abbreviazioni che si espandono mentre scrivi in questo progetto.
settings-desc-work-punctuation = Virgolette, trattini e spaziatura seguiti dal testo del progetto.
settings-group-typography = Tipografia
settings-group-writing-column = Colonna di scrittura
settings-group-writing-view = Vista di scrittura
settings-group-distraction-free-strip = Barra dei comandi
settings-group-theme = Tema
settings-group-language = Lingua
settings-group-startup = Avvio
settings-group-updates = Aggiornamenti
settings-group-autosave = Salvataggio automatico
settings-field-typeface = Carattere
settings-field-size = Dimensione del testo
editor-size-changed-manuscript = Dimensione del testo del manoscritto: { $percent }
editor-size-changed-synopsis = Dimensione del testo della sinossi: { $percent }
editor-size-changed-notes = Dimensione del testo delle note: { $percent }
editor-size-changed-corkboard = Dimensione del testo delle schede: { $percent }
editor-size-changed-corkboard-expanded = Dimensione del testo dell'editor espanso: { $percent }
editor-size-changed-distraction-free = Dimensione del testo senza distrazioni: { $percent }
menu-text-size-increase = A&umenta dimensione del testo
menu-text-size-decrease = &Riduci dimensione del testo
menu-text-size-reset = Ripristina dimensione de&l testo
settings-field-line-height = Altezza delle righe
settings-field-first-line-indent = Rientro della prima riga
settings-field-paragraph-spacing-before = Spazio prima del paragrafo
settings-field-paragraph-spacing-after = Spazio dopo il paragrafo
settings-field-column-width = Larghezza della colonna
settings-distraction-free-width-hint = Vale solo nella modalità senza distrazioni. Le larghezze delle colonne di scena, sinossi e note restano invariate.
settings-distraction-free-title = Mantieni il nome dell'elemento
settings-distraction-free-word-count = Mantieni il conteggio delle parole
settings-distraction-free-session = Mantieni la sessione di scrittura
settings-distraction-free-go-to = Mantieni il pulsante «Vai a…»
settings-distraction-free-go = Mantieni i pulsanti Precedente e Successivo
settings-distraction-free-chrome-hint = Il pulsante Esci rimane sempre, qualunque sia la scelta. Permette di uscire anche quando Esc è occupato.
settings-field-app-theme = Tema
settings-theme-light = Chiaro
settings-theme-dark = Scuro
settings-field-text-scale = Dimensione del testo dell'interfaccia
settings-field-language = Lingua dell'interfaccia
settings-synopsis-placement = Posizione della sinossi
settings-synopsis-placement-none = Nessuna
settings-synopsis-placement-top = Sopra
settings-synopsis-placement-side = Accanto
settings-typewriter = Scorrimento a macchina da scrivere
settings-typewriter-tip = Mantiene la riga di scrittura a un'altezza fissa mentre il manoscritto scorre sotto. Un clic continua a posizionare il cursore dove fai clic.
settings-typewriter-position = Posizione della riga
settings-typewriter-position-top-third = Terzo superiore
settings-typewriter-position-middle = Centro
settings-typewriter-position-bottom-quarter = Quarto inferiore
settings-highlight-scope = Evidenziazione intorno al cursore
settings-highlight-scope-none = Nessuna
settings-highlight-scope-sentence = Frase
settings-highlight-scope-paragraph = Paragrafo
settings-highlight-scope-tip-none = Lascia la pagina semplice. Nulla viene evidenziato mentre scrivi.
settings-highlight-scope-tip-sentence = Evidenzia la frase che stai scrivendo per distinguerla da quelle vicine.
settings-highlight-scope-tip-paragraph = Evidenzia l'intero paragrafo che stai scrivendo, per tenere in vista il passaggio su cui lavori.
settings-group-container-views = Viste dei contenitori
settings-remember-view = Ricorda l'ultima vista per ogni tipo di elemento
settings-remember-view-tip = Apri un contenitore nella vista usata l'ultima volta per quel tipo
settings-remember-view-tip-more =
    Libri, parti e capitoli offrono più viste: la propria pagina, il manoscritto completo
    e la sinossi completa. Attiva questa opzione per riaprire ogni tipo nell'ultima vista
    scelta. Per esempio, scegliendo «Capitolo completo», anche il prossimo capitolo si
    aprirà così. Ogni tipo ricorda la propria vista.
settings-group-counting = Conteggio delle parole
settings-counting-auto = Automatico (in base alla lingua)
settings-counting-whitespace = Separa agli spazi
settings-counting-unicode-words = Parole Unicode
settings-counting-cjk-hybrid = Compatibile CJK (per carattere)
settings-counting-hint = Il conteggio automatico usa i caratteri per cinese e giapponese, le parole per le altre lingue. Cambialo solo se il risultato non è adatto alla tua lingua.
settings-group-goals-display = Visualizzazione
settings-show-characters = Mostra il conteggio dei caratteri nella barra di stato
settings-autosave-hint = Le modifiche vengono scritte automaticamente sul disco mentre scrivi.
settings-sec-work = Progetto
settings-page-structure = Struttura
settings-page-author = Autore
settings-field-author-name = Nome dell'autore
settings-field-author-placeholder = Facoltativo
settings-field-author-hint = Compare sul frontespizio generato dall'esportazione e nei metadati dei file esportati. Lascia vuoto per ometterlo. È la firma del libro; il nome che firma i commenti si trova in Impostazioni ▸ Utente.
settings-group-chapters = Capitoli
tidy-titles-title = Riordina i titoli dei capitoli
tidy-titles-none = Nessun titolo di capitolo o parte ripete soltanto il proprio numero.
tidy-titles-lead = { $count ->
        [one] Un capitolo o una parte ha come titolo soltanto il proprio numero.
       *[other] { $count } capitoli e parti hanno come titolo soltanto il proprio numero.
    }
tidy-titles-explain = Svuotando questi titoli, ogni elemento resta identificato dal numero che il libro già conosce e che compare nell'esportazione. Non cambia altro e un solo Annulla ripristina tutti i titoli.
menu-document-tidy-titles = Riordina &titoli dei capitoli…
settings-group-numbering = Numerazione
settings-number-chapters = Numera capitoli e parti
settings-number-chapters-tip = Capitoli e parti ricevono un numero dalla loro posizione nel libro, mostrato qui accanto al titolo e stampato nell'esportazione.
settings-number-chapters-tip-more = Il numero non viene mai memorizzato nel titolo, quindi resta corretto quando riordini, inserisci o elimini elementi. Disattivando questa opzione, vengono esportati soltanto i titoli, indipendentemente dallo stile. Per escludere dalla numerazione un solo capitolo, come un prologo o un interludio, usa invece Numerazione nelle Proprietà: resterà nel libro senza occupare un numero.
settings-part-resets-chapter = Ricomincia la numerazione dei capitoli a ogni parte
settings-part-resets-chapter-tip = Disattivato per impostazione predefinita: i capitoli proseguono tra le parti del libro, così la «Parte seconda» può iniziare dal Capitolo undici.
settings-part-resets-chapter-tip-more = È la prassi editoriale abituale, attesa dal lettore. Attivala se le parti devono essere lette come volumi separati, ciascuno a partire dal Capitolo uno.
settings-chapter-flat = Capitoli singoli
settings-chapter-flat-hint = Attivo: un capitolo è un elemento singolo in cui scrivere, senza scene. Disattivo: è una cartella in cui puoi scrivere e che può anche contenere scene. I nuovi capitoli seguono questa impostazione; quelli esistenti si cambiano con «Converti in».
settings-styles-builtin = Stili integrati
settings-styles-user = I miei stili
settings-styles-builtin-badge = Integrato
settings-styles-duplicate = Duplica
settings-styles-edit = Modifica
settings-styles-delete = Elimina
settings-styles-import = Importa…
settings-styles-export = Esporta…
settings-styles-copy-suffix = (copia)
settings-styles-json-filter = Stile di esportazione
settings-styles-editor-title = Modifica stile
settings-styles-editor-none = Seleziona uno stile personale da modificare o duplica uno stile integrato.
settings-styles-imported = Stile importato
settings-styles-import-failed = Impossibile importare lo stile
settings-styles-exported = Stile esportato
settings-styles-export-failed = Impossibile esportare lo stile
settings-styles-field-name = Nome
settings-styles-field-chapters = Intestazioni dei capitoli
settings-styles-field-parts = Intestazioni delle parti
settings-styles-field-scene-break = Separatore di scena
settings-styles-field-major-scene-break = Separatore di scena maggiore
settings-styles-field-spacing = Interlinea
settings-styles-field-justify = Giustifica il testo
settings-styles-field-notes = Includi note
settings-styles-field-synopses = Includi sinossi
settings-styles-field-scene-titles = Includi titoli delle scene
settings-styles-field-epigraphs = Includi epigrafi
settings-styles-field-paratexts = Includi paratesti
settings-styles-field-epigraph-placement = Posizione dell'epigrafe
settings-styles-epigraph-after = Dopo il titolo
settings-styles-epigraph-before = Prima del titolo
settings-styles-field-footnotes = Includi note a piè di pagina
settings-styles-field-footnote-numbering = Numerazione delle note a piè di pagina
settings-styles-footnote-numbering-continuous = Continua
settings-styles-footnote-numbering-per-chapter = Ricomincia a ogni capitolo
settings-styles-footnote-numbering-per-book = Ricomincia a ogni libro
settings-styles-field-images = Immagini
menu-image = &Immagine
settings-styles-images-beside = Accanto al documento
settings-styles-images-embed = All'interno del documento
settings-styles-images-omit = Escludi
settings-styles-group-round-trip = Invio a un revisore
settings-styles-field-comments = Includi commenti
settings-styles-field-round-trip-marks = Includi marcatori di reimportazione
settings-styles-round-trip-hint = Solo DOCX e ODT. I marcatori sono identificativi invisibili che permettono al file restituito di aggiornare questo progetto invece di aggiungere una seconda copia.
settings-styles-group-pages = Pagine
settings-styles-field-cover = Inizia con la copertina
settings-styles-field-word-count = Conteggio parole sul frontespizio
settings-styles-field-page-books = Nuova pagina a ogni libro
settings-styles-field-page-parts = Nuova pagina a ogni parte
settings-styles-field-page-chapters = Nuova pagina a ogni capitolo
settings-styles-field-page-paratexts = Nuova pagina a ogni paratesto
settings-styles-heading-none = Nessuna intestazione
settings-styles-heading-numbered = Solo numero
settings-styles-heading-title = Solo titolo
settings-styles-heading-both = Numero + titolo
settings-styles-break-blank = Riga vuota
settings-styles-break-none = Nessuno
settings-styles-spacing-single = Singola
settings-styles-spacing-onehalf = 1½
settings-styles-spacing-double = Doppia
welcome-title = Benvenuto in Skribisto
welcome-version = Versione { $version }
welcome-update-available = È disponibile la versione { $version }
welcome-update-tip = Apre la pagina di download nel browser.
welcome-search = Cerca progetti
welcome-open = Apri
welcome-new-work = Nuovo progetto
welcome-create-from = Crea da…
welcome-recent-works = Progetti recenti
welcome-empty-recents = Nessun progetto recente.
welcome-no-matches = Nessun progetto recente corrisponde alla ricerca.
welcome-learn-soon = Guide e suggerimenti saranno disponibili a breve.
welcome-about-blurb = Skribisto, una riscrittura dell'app di scrittura in Rust e Teksilo.
welcome-tagline = *Un luogo tranquillo per scrivere testi lunghi.*
welcome-github = GitHub
welcome-discord = Discord
nav-works = Progetti
nav-examples = Esempi
nav-learn = Impara
nav-about = Informazioni
synopsis = Sinossi
epigraph = Epigrafe
pane-manuscript = Manoscritto
corkboard = Bacheca
overview = Panoramica
corkboard-card-count = { $count ->
    [one] { $count } scheda
   *[other] { $count } schede
}
corkboard-child-count = { $count ->
    [one] { $count } elemento
   *[other] { $count } elementi
}
corkboard-view-nested = Gerarchica
corkboard-view-flat = Piatta
corkboard-layout-hint = La vista gerarchica mostra i figli diretti del contenitore. Apri la scheda di una cartella per entrarvi. La vista piatta mostra insieme tutte le scene del contenitore.
corkboard-card-size = Dimensione delle schede
corkboard-search-placeholder = Filtra schede…
corkboard-empty-title = Non c'è ancora nulla
corkboard-empty-hint = Usa «＋ Nuovo» in alto per aggiungere il primo elemento.
corkboard-new = Nuovo
corkboard-show-card-numbers = Numera le schede
corkboard-modal-size = Dimensione dell'editor espanso
corkboard-scope-hint = Vale per tutte le bacheche aperte, in ogni progetto.
corkboard-sort-manuscript = Ordine del manoscritto
corkboard-sort-title-asc = Titolo A–Z
corkboard-sort-title-desc = Titolo Z–A
corkboard-card-number = Scheda { $number }
duplicate = Duplica
duplicate-n = { $count ->
    [one] Duplica { $count } scheda
   *[other] Duplica { $count } schede
}
set-label-n = { $count ->
    [one] Imposta la dicitura su { $count } scheda
   *[other] Imposta la dicitura su { $count } schede
}
move-to-trash-n = { $count ->
    [one] Sposta { $count } scheda nel cestino
   *[other] Sposta { $count } schede nel cestino
}
reveal-in-outline = Mostra nella struttura
corkboard-move-to = Sposta in…
corkboard-move-to-n = { $count ->
    [one] Sposta { $count } scheda in…
   *[other] Sposta { $count } schede in…
}
corkboard-move-picker-title = Sposta in…
corkboard-move-picker-empty = Nessun raccoglitore. Creane prima uno.
corkboard-move-picker-cancel = Annulla
corkboard-move-here = Sposta qui
corkboard-moved-ok = { $count ->
    [one] { $count } scheda spostata
   *[other] { $count } schede spostate
}
corkboard-move-into-self = Un contenitore non può essere spostato dentro sé stesso. Scegli una destinazione esterna.
corkboard-move-failed = Impossibile spostare quelle schede in questa posizione.
overview-col-title = Titolo
overview-col-type = Tipo
overview-col-label = Dicitura
overview-col-tags = Etichette
overview-col-books = Libri
overview-col-own-words = Parole
overview-col-total-words = Totale
overview-row-count = { $count ->
    [one] { $count } elemento
   *[other] { $count } elementi
}
overview-search-placeholder = Filtra elementi…
overview-expand-all = Espandi tutto
overview-collapse-all = Comprimi tutto
overview-table-label = Contenuti
overview-empty-title = Non c'è ancora nulla
overview-gone-title = Questo contenitore non è più disponibile
overview-gone-hint = È stato spostato nel cestino. Ripristinalo o chiudi questa scheda.
overview-empty-hint = Usa «＋ Nuovo» in alto per aggiungere il primo elemento.
corkboard-grid-label = Schede della bacheca
corkboard-rename-field = Rinomina elemento
corkboard-expand-synopsis = Espandi sinossi
corkboard-synopsis-modal-title = Sinossi
corkboard-badge-scene = Scena
corkboard-badge-chapter = Capitolo
corkboard-badge-part = Parte
corkboard-badge-book = Libro
corkboard-badge-note = Nota
corkboard-badge-folder = Cartella
corkboard-badge-paratext = Paratesto
corkboard-badge-text = Testo
corkboard-badge-end = Fine
settings-group-corkboard-layout = Disposizione
settings-group-corkboard-cards = Schede
corkboard-show-word-count = Mostra il conteggio delle parole sulle schede
text-heading = Testo
no-content = Questo elemento non ha contenuti modificabili.
untitled = Senza titolo
placeholder-title = Titolo…
placeholder-subtitle = Sottotitolo…
placeholder-chapter-title = Titolo del capitolo…
split-editor = Dividi editor
close-split-view = Chiudi vista divisa
drop-open-here = Apri qui
drop-open-to-side = Apri a lato
segment-chapter = Capitolo
segment-part = Parte
segment-book = Libro
segment-pace = Ritmo
segment-notes = Note
segment-story-bible = Bibbia narrativa
segment-note-own = Nota
segment-note-details = Dettagli
segment-note-in-prose = Nel testo
pace-placeholder = Il pianificatore del ritmo appare qui.
pace-empty-title = Pianifica il ritmo di questo libro
pace-empty-body = Imposta un obiettivo di parole e una scadenza: Skribisto calcola il ritmo giornaliero necessario per raggiungerli.
pace-start-planning = Inizia a pianificare
pace-section-schedule = Calendario
pace-section-progress = Progressi
pace-advancement = Avanzamento
pace-goal = Obiettivo di parole
pace-deadline = Scadenza
pace-active = Piano attivo
pace-day-mon = Lun
pace-day-tue = Mar
pace-day-wed = Mer
pace-day-thu = Gio
pace-day-fri = Ven
pace-day-sat = Sab
pace-day-sun = Dom
pace-card-written = parole scritte
pace-card-of-goal = dell'obiettivo
pace-card-rate = parole / giorno di scrittura
pace-card-days-left = giorni di scrittura rimanenti
pace-card-streak = giorni consecutivi
pace-card-ahead = parole in anticipo sul piano
pace-card-behind = parole in ritardo sul piano
pace-charts-empty = I grafici dei progressi appariranno quando il conteggio delle parole sarà registrato con un salvataggio.
pace-chart-progression = Parole scritte e obiettivo
pace-chart-words-per-day = Parole al giorno
pace-series-actual = Effettivo
pace-series-target = Obiettivo
pace-series-words-per-day = Parole/giorno
pace-daily-target-line = Ritmo costante: { $count ->
    [one] { $count } parola/giorno
   *[other] { $count } parole/giorno
}
pace-section-holidays = Giorni di pausa
pace-section-milestones = Traguardi
pace-holidays-none = Nessuna pausa. Ogni giorno pianificato conta.
pace-holiday-label = Nome della pausa
pace-add-holiday = Aggiungi
pace-remove = Rimuovi
pace-milestones-none = Nessun traguardo. Impostane uno su una parte o un capitolo nelle Proprietà.
full-chapter = Capitolo completo
full-part = Parte completa
full-book = Libro completo
full-synopsis = Sinossi completa
rename = Rinomina
set-label = Imposta dicitura
insert-scene = Inserisci scena
insert-chapter = Inserisci capitolo
split-scene = Dividi scena
move-up = Sposta in alto
move-down = Sposta in basso
merge-with-previous = Unisci al precedente
move-to-trash = Sposta nel cestino
rename-chapter = Rinomina capitolo
add-scene = Aggiungi nuova scena
add-chapter = Aggiungi nuovo capitolo
new-scene-title = Nuova scena
placeholder-scene-name = Nome della scena…
menu-cut = Taglia
menu-copy = Copia
menu-paste = Incolla
menu-paste-unformatted = Incolla senza formattazione
menu-select-all = Seleziona tutto
format-bold = Grassetto
format-italic = Corsivo
format-underline = Sottolineato
format-strikethrough = Barrato
binder = Raccoglitore
no-work = Nessun progetto caricato
no-recent-works = Nessun progetto recente
switcher-open-section = Attualmente aperti
switcher-recent-section = Recenti
switcher-this-window = questa finestra
open-project-title = Apri progetto
open-project-question = Come vuoi aprire «{ $title }»?
open-in-new-window = Apri in una nuova finestra
open-here = Apri qui
binder-all = Tutti i raccoglitori
binder-show-all = Mostra tutti i raccoglitori
binder-new = Nuovo raccoglitore…
binder-default-name = Nuovo raccoglitore
binder-item-count = { $count ->
    [one] { $count } elemento
   *[other] { $count } elementi
}
binder-search-placeholder = Filtra la struttura…
binder-search-scope = Cerca in tutti i raccoglitori
binder-trash-confirm-title = Spostare il raccoglitore nel cestino?
binder-trash-confirm-text = «{ $name }» e tutti i suoi elementi verranno spostati nel cestino.
dialog-rename = Rinomina
dialog-new-binder = Nuovo raccoglitore
dialog-set-label = Imposta dicitura
dialog-new-scene = Nuova scena
close-work-question = Salvare le modifiche prima di chiudere il progetto?
quit-question = Salvare le modifiche prima di uscire?
quit-save-work-question = Salvare le modifiche a { $title } prima di uscire?
unsaved-changes = Questo progetto contiene modifiche non salvate.
new-work-unsaved-question = Salvare le modifiche prima di creare un nuovo progetto?
open-work-unsaved-question = Salvare le modifiche prima di aprire un altro progetto?
switch-backup-discard-title = Scartare le modifiche a questo backup?
switch-backup-discard-text = Le modifiche a un backup non possono essere salvate al suo interno. Usa Salva con nome o Ripristina per conservarle, oppure scartale e apri l'altro progetto.
switch-save-failed = Impossibile salvare il progetto, quindi non è stato sostituito: { $error }
switch-save-not-started = Impossibile salvare il progetto, quindi non è stato sostituito.
close-save-failed = Impossibile salvare il progetto, quindi non è stato chiuso: { $error }
close-save-not-started = Impossibile salvare il progetto, quindi non è stato chiuso.
save-not-started = Impossibile salvare il progetto.
could-not-open-work = Impossibile aprire «{ $file }»
could-not-open-work-too-new = «{ $file }» richiede una versione più recente di Skribisto
could-not-open-work-too-new-detail = Salvato nel formato Skribisto { $written_by }; per aprirlo serve il formato { $requires } o successivo, mentre questa versione supporta fino al formato { $supported }. Aggiorna Skribisto per aprirlo.
could-not-open-work-importing = «{ $file }» è in corso di scrittura da parte di un'importazione
could-not-open-work-importing-detail = Un'importazione sta sovrascrivendo questo progetto e non è ancora terminata. Aprilo al termine: aprendolo ora vedresti il progetto che sta per essere sostituito e il successivo salvataggio lo riscriverebbe sopra l'importazione.
could-not-open-example = Impossibile aprire l'esempio: { $error }
could-not-open-link = Impossibile aprire { $url }: { $error }
could-not-create-work = Impossibile creare il progetto: { $error }
saving-as-file = Salvataggio come { $target }…
saving-as-folder = Salvataggio come { $target }/…
saved-as = Salvato in { $target }
save-error = Impossibile salvare: { $error }
save-flush-failed = Il salvataggio è riuscito, ma questi documenti non hanno potuto esservi inclusi e restano non salvati: { $titles } ({ $error })
backup-error = Impossibile creare il backup: { $error }
backing-up = Backup in corso…
backup-nothing-open = Nessun progetto aperto di cui fare il backup.
backup-already-running = Un backup è già in corso.
backup-complete = Backup completato ({ $ok ->
    [one] { $ok } salvato
   *[other] { $ok } salvati
}, { $skipped ->
    [one] { $skipped } già aggiornato
   *[other] { $skipped } già aggiornati
})
backup-partial = Backup terminato: { $ok ->
    [one] { $ok } salvato
   *[other] { $ok } salvati
}, { $failed ->
    [one] { $failed } destinazione non riuscita
   *[other] { $failed } destinazioni non riuscite
}
backup-no-destination-title = Nessuna posizione di backup disponibile
backup-no-destination-text = Nessuna destinazione di backup configurata è raggiungibile, per esempio perché un disco esterno è scollegato. Collegalo e riprova, oppure esci senza backup.
backup-failed-close-title = Impossibile salvare il backup
backup-failed-close-text = Non è stato possibile scrivere una copia prima della chiusura. Tutte le destinazioni hanno fallito: il disco potrebbe essere stato rimosso, essere pieno o protetto da scrittura. Risolvi il problema e riprova, oppure esci senza backup.
backup-failed-close-error-text = Non è stato possibile scrivere una copia prima della chiusura: { $error }. Risolvi il problema e riprova, oppure esci senza backup.
backup-no-result = Il backup è terminato senza indicare se ha creato una copia. Controlla l'elenco dei backup per vedere se ce n'è uno nuovo.
backup-choice-title = File di backup
backup-choice-heading = Hai aperto una copia di backup
backup-choice-subtitle = È una copia del progetto in un determinato momento.
backup-choice-subtitle-dated = Backup del { $date }.
backup-choice-body = Puoi aprirla e modificarla liberamente, ma conservare le modifiche solo con Salva con nome. Il file originale del progetto non viene toccato. Oppure puoi ripristinare il progetto esattamente a questo backup.
backup-choice-open = Apri il backup
backup-choice-restore = Ripristina il progetto a questo punto…
backup-choice-not-a-backup = No, aprilo normalmente
backup-banner-title = Copia di backup: le modifiche non possono essere salvate qui
backup-banner-description = Usa Salva con nome per conservare le modifiche in un nuovo file, oppure Ripristina per sostituire il progetto originale con questo backup.
backup-banner-restore = Ripristina…
backup-banner-save-as = Salva con nome…
backup-restore-original-missing = Impossibile trovare il progetto originale da sostituire. Usa Salva con nome per conservare il backup come nuovo progetto.
backup-restore-close-elsewhere-title = Progetto aperto in un'altra finestra
backup-restore-close-elsewhere-text = Il progetto da sostituire è aperto in un'altra finestra. Chiudilo prima lì, poi riprova.
backup-restore-focus-window = Vai a quella finestra
backup-restore-confirm-title = Ripristinare questo backup?
backup-restore-confirm-text = La versione attuale del progetto verrà copiata come backup di sicurezza prima di essere sostituita da questa.
backup-restore-confirm-ok = Ripristina
backup-restore-error = Impossibile ripristinare: { $error }
backup-restored-ok = Progetto ripristinato.
backup-restored-with-safety = Progetto ripristinato. La versione precedente è stata salvata in { $path }.
close-backup-discard-title = Scartare le modifiche a questo backup?
close-backup-discard-text = Le modifiche a un backup non possono essere salvate al suo interno. Usa Salva con nome per conservarle, oppure scartale e chiudi.
quit-backup-discard-title = Scartare le modifiche e uscire?
quit-backup-discard-work-question = Scartare le modifiche a { $title } e uscire?
quit-backup-discard-text = Le modifiche a un backup non possono essere salvate al suo interno. Usa Salva con nome per conservarle, oppure scartale ed esci.
backup-nudge-text = Non sono configurati backup per questo progetto.
backup-nudge-action = Configura backup…
menu-backups-list = Bac&kup…
backups-title = Backup
backups-loading = Ricerca dei backup…
backups-empty = Nessun backup trovato per questo progetto.
backups-open = Apri
backups-reveal = Mostra
backups-delete = Elimina questo backup
backups-delete-confirm-title = Eliminare questo backup?
backups-delete-confirm-text = «{ $name }» verrà eliminato definitivamente. L'operazione non può essere annullata.
backups-delete-error = Impossibile eliminare il backup: { $error }
backups-refresh = Aggiorna
backups-close = Chiudi
settings-page-backup = Impostazioni predefinite dei backup
settings-page-work-backup = Backup
settings-backup-general-title = Impostazioni predefinite dei backup
settings-backup-work-title = Backup di questo progetto
settings-backup-inherit = Usa le impostazioni generali dei backup
settings-backup-inheriting = Questo progetto usa le impostazioni generali dei backup.
settings-backup-none-hint = Nessun backup automatico configurato: tutti gli eventi di attivazione sono disabilitati.
settings-backup-last = Ultimo backup: { $date }
settings-backup-last-never = Ancora nessun backup.
settings-backup-open-list = Apri elenco dei backup…
settings-backup-triggers = Quando eseguire il backup
settings-backup-on-close = Alla chiusura del progetto
settings-backup-on-open = All'apertura del progetto
settings-backup-interval = Periodicamente, ogni
settings-backup-destinations = Destinazioni dei backup
settings-backup-default-location = Posizione predefinita
settings-backup-usage-since = { $count ->
    [one] { $count } backup · { $size } · il più vecchio: { $oldest }
   *[other] { $count } backup · { $size } · il più vecchio: { $oldest }
}
settings-backup-usage = { $count ->
    [one] { $count } backup · { $size }
   *[other] { $count } backup · { $size }
}
settings-backup-usage-empty = Non c'è ancora nulla qui
settings-backup-usage-measuring = Calcolo…
settings-backup-reveal-root = Mostra cartella
settings-backup-dest-none = Nessuna destinazione. I backup vengono salvati accanto al progetto.
settings-backup-dest-remove = Rimuovi
settings-backup-dest-add = Aggiungi cartella…
settings-backup-dest-refresh = Aggiorna
settings-backup-retention = Quanti conservarne
settings-backup-retention-tiered = A livelli
settings-backup-retention-keep-n = Conserva gli ultimi N
settings-backup-retention-tip = Come eliminare i backup più vecchi.
settings-backup-retention-tip-more = «A livelli» conserva un backup all'ora per un giorno, uno al giorno per una settimana, uno alla settimana per un mese e poi uno al mese: la cronologia recente resta fitta, quella più vecchia si dirada. «Conserva gli ultimi N» mantiene semplicemente le N copie più recenti. In entrambi i casi vengono sempre conservate le copie più recenti nella quantità minima indicata sotto.
settings-backup-gfs-hourly = Orari (ultime 24 ore)
settings-backup-gfs-daily = Giornalieri (ultima settimana)
settings-backup-gfs-weekly = Settimanali (ultimo mese)
settings-backup-gfs-monthly = Mensili
settings-backup-keep-n = Numero da conservare
settings-backup-min-keep = Conserva sempre almeno
settings-backup-dedup = Salta il backup se non è cambiato nulla
new-work-title = Nuovo progetto
new-work-close = Chiudi
new-work-name = Nome del progetto
new-work-name-placeholder = Senza titolo
new-work-author = Autore
new-work-author-placeholder = Facoltativo
new-work-format = Formato
new-work-single-file = File singolo
new-work-bundle = Cartella
new-work-convert-later = Potrai convertire il formato in seguito.
new-work-location = Posizione
new-work-will-create = Verrà creato
new-work-language = Lingua predefinita
new-work-language-hint = Vale per i nuovi testi e il controllo ortografico. Ogni testo può usare un'altra lingua.
new-work-template = Modello
new-work-template-none = Nessuno
new-work-template-empty-novel = Romanzo vuoto
new-work-template-light-novel = Light novel
new-work-template-novel = Romanzo
new-work-template-novel-in-parts = Romanzo in parti
new-work-template-notebook = Taccuino
new-work-cancel = Annulla
new-work-create = Crea progetto
new-work-step-details = Dettagli
new-work-step-language = Lingua e struttura
new-work-step-template = Modello
new-work-back = Indietro
new-work-next = Avanti
new-work-documents-title = Nuovo progetto da documenti
new-work-step-import = Importazione
new-work-create-and-import = Crea e importa…
new-work-documents-next-title = Ora tocca ai tuoi documenti
new-work-documents-next-body = Creando il progetto si apre la procedura di importazione, dove scegli i documenti, controlli la struttura riconosciuta da Skribisto e scegli la destinazione. Nulla viene scritto nel progetto fino alla conferma in quella procedura.
new-work-documents-no-template = Questo progetto inizia volutamente vuoto: nessun modello, così conterrà soltanto i documenti importati.
new-work-documents-chapter-scene-hint = Vale per i capitoli creati dall'importazione. Puoi cambiarlo in seguito nelle impostazioni del progetto.
new-work-single-file-desc = Un archivio .skrib (ZIP). Portatile e facile da salvare come backup.
new-work-bundle-desc = Una cartella con tutti i testi e le risorse. Più adatta al controllo di versione.
new-work-template-none-count = raccoglitore vuoto
new-work-template-empty-novel-count = 1 capitolo
new-work-template-light-novel-count = 15 capitoli
new-work-template-novel-count = 20 capitoli
new-work-template-novel-in-parts-count = 3 parti, 24 capitoli
new-work-template-notebook-count = note libere
new-work-chapter-scene = Capitoli singoli
new-work-chapter-scene-tip = Ogni capitolo è un elemento singolo in cui scrivi direttamente, senza scene sottostanti. Disattiva per la struttura classica, in cui un capitolo è una cartella: puoi scriverci direttamente, ma può anche contenere scene.
new-work-chapter-scene-tip-more = Scrivi direttamente nel capitolo in entrambi i casi. Cambia solo la possibilità di *contenere* scene. L'albero del raccoglitore serve soltanto a organizzare: entrambe le strutture producono lo stesso libro, si possono mescolare liberamente e «Converti in» passa dall'una all'altra senza perdere una parola.
new-work-name-required = Inserisci un nome per il progetto
new-work-name-invalid = Questo nome non contiene caratteri utilizzabili
new-work-location-required = Scegli una posizione
new-work-location-missing = Questa cartella non esiste
new-work-location-not-folder = Questo percorso non è una cartella
new-work-location-readonly = Questa cartella non è scrivibile
new-work-target-exists = In questa cartella esiste già un progetto con questo nome. Verrà chiesta conferma prima di sostituirlo
new-work-target-is-folder = In questa posizione esiste già una cartella con questo nome. Scegli un altro nome
new-work-overwrite-title = Sostituire il progetto esistente?
new-work-overwrite-text = «{ $name }» esiste già in questa cartella. Sostituirlo con il nuovo progetto?
new-work-target-open-title = Questo progetto è aperto
new-work-target-open-text = «{ $name }» è aperto in Skribisto. Creare un nuovo progetto al suo posto sostituirebbe quello su cui stai lavorando, e il prossimo salvataggio riscriverebbe il vecchio sopra il nuovo. Scegli un altro nome o un'altra cartella, oppure chiudi prima il progetto.
target-importing-title = Un'importazione sta scrivendo questo progetto
target-importing-text = Un'importazione sta ancora scrivendo «{ $name }». Attendi che termini o scegli un altro nome.
backup-restore-importing-text = Un'importazione sta ancora scrivendo «{ $name }», il progetto che questo backup sostituirebbe. Ripristina il backup quando l'importazione sarà terminata.
new-work-manuscript = Manoscritto
new-work-notes = Note
new-work-research = Ricerca
new-work-notebook = Taccuino
new-work-scene = Scena
new-work-note = Nota
new-work-front-matter = Pagine preliminari
new-work-back-matter = Pagine finali
new-work-paratext = Struttura del libro
new-work-paratext-none = Nessuna struttura
new-work-paratext-hint = Le pagine preliminari e finali con cui una tradizione editoriale apre e chiude il libro. Potrai spostarle, rinominarle o eliminarle in seguito.
new-work-tags = Etichette
new-work-tags-none = Nessuna etichetta
new-work-tags-hint = Una raccolta iniziale per etichettare personaggi, luoghi e altro. Facoltativa: ogni etichetta potrà essere rinominata, ricolorata o eliminata.
new-work-note-templates = Modelli di nota
new-work-note-templates-none = Nessun modello
new-work-note-templates-hint = La forma iniziale delle note della bibbia narrativa. Facoltativa: ogni modello potrà essere modificato o eliminato.
new-work-characters = Personaggi
new-work-places = Luoghi
import-plume-title = Importa progetto Plume Creator
import-plume-source = Progetto Plume
import-plume-source-hint = Scegli un file .plume o .plume_backup di qualsiasi versione di Plume Creator.
import-plume-trash-warning = ⚠ Gli elementi nel cestino o eliminati non vengono trasferiti.
import-plume-source-required = Scegli un file di progetto Plume
import-plume-source-missing = Questo file non esiste
import-plume-source-not-file = Questo percorso non è un file
import-plume-manuscript-binder = Manoscritto
import-plume-story-bible-binder = Bibbia narrativa
import-plume-notes-of = { $name } (note)
import-plume-content-of = { $name } (testo)
import-plume-progress-title = Importazione del progetto Plume…
import-plume-done = { $imported ->
    [one] { $imported } elemento importato.
   *[other] { $imported } elementi importati.
} { $skipped ->
    [one] { $skipped } elemento nel cestino non è stato trasferito.
   *[other] { $skipped } elementi nel cestino non sono stati trasferiti.
}
import-plume-nested-too-deep = Il progetto non è stato importato: la parte «{ $part }» supera { $limit } livelli di annidamento. Nessun progetto Plume Creator si avvicina a tale profondità e leggerla potrebbe far arrestare Skribisto. Nulla è stato scritto.
import-plume-declares-entities = Il progetto non è stato importato: la parte «{ $part }» dichiara entità XML, cosa che nessun progetto Plume Creator fa. Espanderle potrebbe esaurire la memoria e far arrestare Skribisto. Nulla è stato scritto.
import-plume-no-outline = Questo file è un archivio compresso, ma non un progetto Plume Creator: non ha una struttura. Nulla è stato scritto.
import-plume-empty-outline = Il file della struttura di questo progetto Plume Creator è vuoto, quindi non c'è nulla da importare. Nulla è stato scritto.
import-plume-wrong-document = Questo non è un progetto Plume Creator: la sua parte «{ $part }» è un documento «{ $tag }». Nulla è stato scritto.
import-plume-too-new = La parte «{ $part }» di questo progetto è stata scritta da un Plume Creator più recente (versione { $version }) di quanto Skribisto sappia leggere (fino alla { $supported }). Nulla è stato scritto.
import-manuskript-title = Importa progetto Manuskript
import-manuskript-source = Progetto Manuskript
import-manuskript-source-hint = Scegli il file .msk o la cartella del progetto di qualsiasi versione di Manuskript.
import-manuskript-source-file = Scegli file…
import-manuskript-source-folder = Scegli cartella…
import-manuskript-source-required = Scegli un progetto Manuskript
import-manuskript-source-missing = Questo file o questa cartella non esiste
import-manuskript-manuscript-binder = Manoscritto
import-manuskript-story-bible-binder = Bibbia narrativa
import-manuskript-characters-group = Personaggi
import-manuskript-world-group = Mondo
import-manuskript-plots-group = Trame
import-manuskript-project-info-note = Informazioni sul progetto
import-manuskript-summary-note = Riassunto
import-manuskript-importance-minor = Minore
import-manuskript-importance-secondary = Secondario
import-manuskript-importance-main = Principale
import-manuskript-notes-of = { $title } (note)
import-manuskript-progress-title = Importazione del progetto Manuskript…
import-manuskript-done = { $imported ->
    [one] { $imported } elemento importato.
   *[other] { $imported } elementi importati.
} { $revisions ->
    [0] { "" }
    [one] È stata trasferita anche { $revisions } versione precedente.
   *[other] Sono state trasferite anche { $revisions } versioni precedenti.
}
import-manuskript-nested-too-deep = Il progetto non è stato importato: il file «{ $part }» supera { $limit } livelli di annidamento. Nessun progetto Manuskript si avvicina a tale profondità e leggerlo potrebbe far arrestare Skribisto. Nulla è stato scritto.
import-manuskript-folders-too-deep = Il progetto non è stato importato: uno dei suoi file si trova a più di { $limit } cartelle di profondità. Nessun progetto Manuskript si avvicina a tale profondità e leggerlo potrebbe far arrestare Skribisto. Nulla è stato scritto.
import-manuskript-declares-entities = Il progetto non è stato importato: il file «{ $part }» dichiara entità XML, cosa che nessun progetto Manuskript fa. Espanderle potrebbe esaurire la memoria e far arrestare Skribisto. Nulla è stato scritto.
import-manuskript-not-a-project-file = «{ $file }» non è né un archivio di progetto Manuskript né il piccolo file .msk che rimanda alla cartella di un progetto. Nulla è stato scritto.
import-manuskript-folder-missing = «{ $stub }» rimanda a una cartella di progetto chiamata «{ $folder }» accanto a sé, ma quella cartella non c'è. Un progetto Manuskript in modalità cartella è fatto di entrambi, e devono viaggiare insieme. Nulla è stato scritto.
import-manuskript-not-a-project = Questo non sembra un progetto Manuskript: non ha né il contrassegno di Manuskript né una struttura. Nulla è stato scritto.
import-manuskript-format-too-new = Questo progetto è nel formato Manuskript { $version }, più recente di quanto Skribisto sappia leggere (fino al { $supported }). Nulla è stato scritto.
import-manuskript-too-many-files = Il progetto non è stato importato: contiene più di { $limit } file, molti più di qualsiasi progetto Manuskript. Nulla è stato scritto.
import-manuskript-too-large = Il progetto non è stato importato: supera i { $limit } MB che Skribisto legge da un progetto Manuskript. Nulla è stato scritto.
project-import-close = Chiudi
project-import-location = Cartella di destinazione
project-import-location-title = Scegli una cartella di destinazione
project-import-name = Nome del file
project-import-name-placeholder = Nome del progetto
project-import-will-create = Verrà creato
project-import-cancel = Annulla
project-import-import = Importa
project-import-location-required = Scegli una cartella di destinazione
project-import-location-missing = Questa cartella non esiste
project-import-location-not-folder = Questo percorso non è una cartella
project-import-location-readonly = Questa cartella non è scrivibile
project-import-name-required = Inserisci un nome per il file
project-import-name-exists = Qui esiste già un file con questo nome. L'importazione chiederà conferma per sovrascriverlo
project-import-name-is-folder = Qui c'è già una cartella con questo nome, forse un progetto salvato come cartella. Un'importazione non può sostituirla: scegli un altro nome
project-import-overwrite-title = Sostituire il file esistente?
project-import-overwrite-text = «{ $name }» esiste già. Sostituirlo con il progetto importato?
project-import-busy-title = È in corso l'importazione di un altro progetto
project-import-busy-text = I progetti vengono importati uno alla volta. Attendi che l'importazione in corso termini, oppure annullala, poi avvia di nuovo questa.
project-import-progress-announcement = { $title } { $phase }
project-import-stopping = Interruzione dell'importazione…
project-import-cancelled = Importazione annullata
project-import-open-now = Apri ora
project-import-details = Dettagli
project-import-report = { $count ->
    [one] 1 cosa da sapere su questa importazione
   *[other] { $count } cose da sapere su questa importazione
}
project-import-report-title = Informazioni sull'importazione
project-import-error-title = Impossibile importare il progetto
project-import-failed-cancelled = L'importazione è stata annullata. Nulla è stato scritto.
project-import-failed-target-exists = «{ $file }» esiste già e l'importazione non era autorizzata a sostituirlo. Nulla è stato scritto.
project-import-failed-target-folder = «{ $file }» è una cartella, forse un progetto salvato come cartella, e un'importazione non può sostituirla. Scegli un altro nome. Nulla è stato scritto.
project-import-failed-not-written = Non è stato possibile scrivere «{ $file }» in «{ $folder }» ({ $reason }). Nulla è stato scritto.
project-import-failed-disk-full = In «{ $folder }» non c'è abbastanza spazio libero per scrivere «{ $file }». Libera un po' di spazio, poi importa di nuovo. Nulla è stato scritto.
project-import-failed-source-missing = «{ $file }» non c'è più. Nulla è stato scritto.
project-import-failed-unreadable = Skribisto non è riuscito a leggere «{ $file }» ({ $reason }). Nulla è stato scritto.
project-import-failed-archive-damaged = «{ $file }» è un file compresso che non è stato possibile leggere, probabilmente perché danneggiato ({ $reason }). Nulla è stato scritto.
project-import-failed-archive-entries = Il progetto non è stato importato: il suo archivio elenca { $entries } file, più dei { $limit } che un progetto può contenere, cosa che fa soltanto un file costruito apposta per esaurire la memoria. Nulla è stato scritto.
project-import-failed-archive-member = Il progetto non è stato importato: la sua parte «{ $part }» si decomprime in più di { $limit } MB, cosa che fa soltanto un file costruito apposta per esaurire la memoria. Nulla è stato scritto.
project-import-failed-archive-size = Il progetto non è stato importato: si decomprime in più di { $limit } MB, cosa che fa soltanto un file costruito apposta per esaurire la memoria. Nulla è stato scritto.
project-import-failed-nested = Il progetto non è stato importato: la sua parte «{ $part }» supera { $limit } livelli di annidamento, e leggerla potrebbe far arrestare Skribisto. Nulla è stato scritto.
project-import-failed-entities = Il progetto non è stato importato: la sua parte «{ $part }» dichiara entità XML. Espanderle potrebbe esaurire la memoria e far arrestare Skribisto. Nulla è stato scritto.
project-import-failed-folders = Il progetto non è stato importato: uno dei suoi file si trova a più di { $limit } cartelle di profondità, e leggerlo potrebbe far arrestare Skribisto. Nulla è stato scritto.
project-import-failed-malformed = Il progetto non è stato importato: la sua parte «{ $part }» è danneggiata e non può essere letta ({ $reason }). Nulla è stato scritto.
project-import-failed-scrivener-refused = Skribisto non può importare questo progetto Scrivener. Nulla è stato scritto.
project-import-failed-scrivener-question = Il progetto Scrivener pone una domanda a cui la verifica non ha risposto. Avvia di nuovo la sua importazione e rispondi alla domanda, poi importa. Nulla è stato scritto.
project-import-failed-scrivener-not-accepted = Prima di poter essere importato, il progetto Scrivener richiede che tu spunti una casella al passaggio Progetto, e non è stata spuntata. Avvia di nuovo la sua importazione e spunta la casella, poi importa. Nulla è stato scritto.
project-import-failed-scrivener-changed = Il progetto Scrivener è cambiato dopo la tua verifica. Avvia di nuovo la sua importazione e verificalo ancora, poi importa. Nulla è stato scritto.
project-import-failed-internal = L'importazione si è interrotta per un errore che Skribisto non si aspettava: { $detail }
import-target-open-title = Questo progetto è aperto
import-target-open-text = «{ $name }» è aperto in Skribisto. Importare al suo posto sostituirebbe il progetto su cui lavori e il prossimo salvataggio lo riscriverebbe sopra l'importazione. Scegli un altro nome di file o chiudi prima il progetto.
import-warnings-announcement = { $result } { $notice }.
quit-import-running-title = Un'importazione è ancora in corso
quit-import-running-text = Skribisto sta ancora importando «{ $name }». Uscire ora interrompe l'importazione: non viene scritto nulla e il progetto da cui stai importando resta com'era.
quit-import-stop = Interrompi l'importazione ed esci

## Import a Scrivener project: the wizard
import-scrivener-title = Importa un progetto Scrivener
import-scrivener-step-project = Progetto
import-scrivener-step-review = Verifica
import-scrivener-step-save = Salvataggio
import-scrivener-back = Indietro
import-scrivener-next = Avanti
import-scrivener-import = Importa
import-scrivener-cancel = Annulla
import-scrivener-help = Aiuto

## Project step: choosing the project
import-scrivener-drop-title = Trascina qui un progetto Scrivener
import-scrivener-drop-hint = Un progetto .scriv, il file .scrivx che contiene, un backup .zip o un .scrivtemplate
import-scrivener-source = Progetto Scrivener
import-scrivener-source-placeholder = Percorso del progetto
import-scrivener-choose-file = Scegli un file…
import-scrivener-choose-folder = Scegli una cartella…
import-scrivener-source-hint = Scegli la cartella .scriv del progetto, oppure aprila e scegli il file .scrivx che contiene. Va bene anche un backup .zip.
import-scrivener-source-hint-mac = Se il progetto appare come un singolo file, usa «Scegli un file…». Se appare come una cartella, usa «Scegli una cartella…». Va bene anche un backup .zip.
import-scrivener-dialog-file-title = Scegli un progetto Scrivener
import-scrivener-dialog-folder-title = Scegli la cartella di un progetto Scrivener
import-scrivener-dialog-filter = Progetto Scrivener
import-scrivener-source-missing = A questo percorso non c'è nulla
import-scrivener-source-unreadable-name = Questo nome non è leggibile. Rinomina il progetto nel gestore dei file, poi sceglilo di nuovo.
import-scrivener-empty-hint = Scegli un progetto per vedere cosa vi trova Skribisto.
import-scrivener-reading = Lettura del progetto…
import-scrivener-stop-reading = Interrompi la lettura
import-scrivener-read-failed = Non è stato possibile leggere il progetto
import-scrivener-read-failed-body = Qualcosa è andato storto durante la lettura. Scegli «Rileggi il progetto» per riprovare, oppure «Dettagli» per vedere cosa è successo.
import-scrivener-details = Dettagli
import-scrivener-check-again = Rileggi il progetto
import-scrivener-refusal-unknown-title = Questo progetto non può essere importato
import-scrivener-refusal-unknown-body = Skribisto non sa come trasferirlo. I dettagli ne indicano il motivo.

## Project step: the project card
import-scrivener-made-with-mac = Salvato l'ultima volta da Scrivener { $version } per macOS
import-scrivener-made-with-windows = Salvato l'ultima volta da Scrivener { $version } per Windows
import-scrivener-made-with-linux = Salvato l'ultima volta da Scrivener { $version } per Linux
import-scrivener-made-with-ios = Salvato l'ultima volta da Scrivener { $version } per iPhone e iPad
import-scrivener-made-with-version = Salvato l'ultima volta da Scrivener { $version }
import-scrivener-made-with-unknown = Progetto Scrivener, formato { $format }
import-scrivener-saved-on = Salvato il { $date } su «{ $device }»
import-scrivener-saved-on-no-device = Salvato il { $date }
import-scrivener-card-counts = { $documents ->
    [one] 1 documento
   *[other] { $documents } documenti
}, { $manuscript } nel manoscritto

## Review step
import-scrivener-review-hint = Ecco come arriverà il tuo raccoglitore. Nulla viene scritto finché non importi, e molti elementi possono cambiare tipo in seguito nel raccoglitore, con «Converti in».
import-scrivener-choices-structure = Livelli
import-scrivener-top-level = Cartelle di primo livello
import-scrivener-choices-text = Testo
import-scrivener-choice-containers = Cartelle al livello { $depth }
import-scrivener-choice-texts = Documenti al livello { $depth }
import-scrivener-choice-count = { $count ->
    [one] 1 elemento
   *[other] { $count } elementi
}
import-scrivener-choice-count-example = { $count ->
    [1] 1 elemento: «{ $example }»
   *[other] { $count } elementi, come «{ $example }»
}
import-scrivener-rung-part = Parti
import-scrivener-rung-chapter = Capitoli
import-scrivener-rung-plain = Cartelle semplici
import-scrivener-texts-scenes = Scene
import-scrivener-texts-chapters = Capitoli
import-scrivener-choice-books = Libri nel manoscritto
import-scrivener-books-one = Un solo libro
import-scrivener-books-several = Un libro per cartella
import-scrivener-choice-split = { $count ->
    [1] Dividi «{ $title }» alla sua intestazione
   *[other] Dividi «{ $title }» alle sue { $count } intestazioni
}
import-scrivener-root-items = { $count ->
    [one] 1 elemento
   *[other] { $count } elementi
}
import-scrivener-root-items-manuscript = { $count ->
    [one] 1 elemento. Questa cartella sembra un manoscritto.
   *[other] { $count } elementi. Questa cartella sembra un manoscritto.
}
import-scrivener-dest-notes = Note
import-scrivener-dest-story-bible = Bibbia narrativa
import-scrivener-dest-manuscript = Un altro libro
import-scrivener-dest-leave-out = Escludi
import-scrivener-choice-trash = { $count ->
    [one] Trasferisci il cestino (1 elemento)
   *[other] Trasferisci il cestino ({ $count } elementi)
}
import-scrivener-trash-detail = { $count ->
    [0] Nessuno di essi contiene testo scritto.
    [one] 1 di essi contiene testo scritto.
   *[other] { $count } di essi contengono testo scritto.
}
import-scrivener-choice-blank-lines = Le righe vuote diventano separatori di scena
import-scrivener-choice-ornaments = Le righe di ornamenti diventano separatori di scena
import-scrivener-preview-label = Struttura del nuovo progetto
import-scrivener-col-title = Titolo
import-scrivener-col-becomes = Diventa
import-scrivener-col-exports = Nelle esportazioni
import-scrivener-in-exports-yes = Sì
import-scrivener-in-exports-no = No
import-scrivener-becomes-item = Elemento
import-scrivener-becomes-trashed = Nel cestino
import-scrivener-before-title = Prima di importare
import-scrivener-review-error-unreadable = Non è stato possibile preparare la verifica. Scegli di nuovo il progetto.
import-scrivener-review-error-choice = Questa scelta non è adatta al progetto. Scegli di nuovo il progetto.
import-scrivener-review-error-unmappable = Con questa scelta non è stato possibile trasferire il raccoglitore senza perdere qualcosa. Scegli diversamente o segnala il problema.

## Save step
import-scrivener-save-hint = L'importazione crea un nuovo progetto Skribisto. Il tuo progetto Scrivener viene solo letto, mai modificato.
import-scrivener-title-field = Titolo
import-scrivener-author = Autore
import-scrivener-language = Lingua
import-scrivener-location-inside-project = Scegli una cartella fuori dal progetto Scrivener. Scrivener vi tiene soltanto i propri file.
import-scrivener-copy-research-label = Copia i file di ricerca accanto al nuovo progetto
import-scrivener-copy-research = { $count ->
    [one] 1 file di ricerca che Skribisto non sa aprire, come un PDF o un suono, va in una cartella accanto al progetto. Se l'opzione non è selezionata, la sua nota indica dove si trova nel tuo progetto Scrivener.
   *[other] { $count } file di ricerca che Skribisto non sa aprire, come PDF o suoni, vanno in una cartella accanto al progetto. Se l'opzione non è selezionata, le loro note indicano dove si trovano nel tuo progetto Scrivener.
}
import-scrivener-default-name = Progetto Scrivener

## Progress and result
import-scrivener-progress-title = Importazione del progetto Scrivener…
import-scrivener-done-title = Progetto Scrivener importato
import-scrivener-done = { $items ->
    [one] Il nuovo progetto contiene 1 elemento.
   *[other] Il nuovo progetto contiene { $items } elementi.
}
import-scrivener-done-check = { $items ->
    [one] Il nuovo progetto contiene 1 elemento.
   *[other] Il nuovo progetto contiene { $items } elementi.
} { $findings ->
    [one] 1 cosa merita un controllo.
   *[other] { $findings } cose meritano un controllo.
}
import-scrivener-review-again = Rivedi l'importazione

## The import's report (the dialog See report opens)
import-scrivener-see-report = Vedi rapporto
import-scrivener-report-from = Letto da { $path }
import-scrivener-report-to = Scritto in { $path }
import-scrivener-report-came-across = Cosa è stato trasferito
import-scrivener-report-nothing-counted = Non è stato contato nulla.
import-scrivener-report-items = { $count ->
    [one] 1 elemento
   *[other] { $count } elementi
}
import-scrivener-report-words = { $count ->
    [one] 1 parola
   *[other] { $count } parole
}
import-scrivener-report-synopses = { $count ->
    [one] 1 sinossi
   *[other] { $count } sinossi
}
import-scrivener-report-document-notes = { $count ->
    [one] le note di 1 documento
   *[other] le note di { $count } documenti
}
import-scrivener-report-comments = { $count ->
    [one] 1 commento
   *[other] { $count } commenti
}
import-scrivener-report-footnotes = { $count ->
    [one] 1 nota a piè di pagina
   *[other] { $count } note a piè di pagina
}
import-scrivener-report-pictures = { $count ->
    [one] 1 immagine
   *[other] { $count } immagini
}
import-scrivener-report-templates = { $count ->
    [one] 1 modello di nota
   *[other] { $count } modelli di nota
}
import-scrivener-report-tags = { $count ->
    [one] 1 etichetta
   *[other] { $count } etichette
}
import-scrivener-report-statuses = { $count ->
    [one] 1 stato
   *[other] { $count } stati
}
import-scrivener-report-trashed = { $count ->
    [one] 1 elemento nel cestino
   *[other] { $count } elementi nel cestino
}
import-scrivener-report-research-files = { $count ->
    [one] 1 file di ricerca
   *[other] { $count } file di ricerca
}
import-scrivener-report-choices = Cosa hai scelto
import-scrivener-report-accepted-mobile = Hai importato il progetto senza le modifiche fatte in Scrivener per iPhone o iPad che non vi erano ancora arrivate.
import-scrivener-report-accepted-no-text = Hai importato questa copia con i documenti vuoti, perché non conteneva nulla del loro testo.
import-scrivener-report-choice = { $subject }: { $answer }
import-scrivener-report-yes = sì
import-scrivener-report-no = no
import-scrivener-report-research-copied = I file di ricerca che Skribisto non sa aprire sono stati copiati in «{ $folder }», accanto al nuovo progetto.
import-scrivener-report-research-not-copied = I file di ricerca che Skribisto non sa aprire non sono stati copiati. Le loro note indicano dove si trovano nel tuo progetto Scrivener.
import-scrivener-report-research-copied-except = { $count ->
    [one] I file di ricerca che Skribisto non sa aprire sono stati copiati in «{ $folder }», accanto al nuovo progetto, tranne 1 che resta nel tuo progetto Scrivener. La sua nota indica dove si trova.
   *[other] I file di ricerca che Skribisto non sa aprire sono stati copiati in «{ $folder }», accanto al nuovo progetto, tranne { $count } che restano nel tuo progetto Scrivener. Le loro note indicano dove si trovano.
}
import-scrivener-report-research-none-copied = { $count ->
    [1] Hai chiesto di copiare i file di ricerca, ma non è stato possibile copiare l'unico file interessato. Resta nel tuo progetto Scrivener e la sua nota indica dove si trova.
   *[other] Hai chiesto di copiare i file di ricerca, ma non è stato possibile copiare nessuno dei { $count } file. Restano nel tuo progetto Scrivener e le loro note indicano dove si trovano.
}
import-scrivener-report-chose-project = Hai scelto di importare «{ $project }», uno dei vari progetti contenuti nella cartella o nell'archivio che hai selezionato.
import-scrivener-report-chose-manifest = Hai scelto il file del raccoglitore «{ $file }».
import-scrivener-report-chose-enclosing = Hai selezionato qualcosa all'interno di «{ $project }», quindi è stato importato l'intero progetto.
import-scrivener-report-chose-upgraded = Hai importato «{ $current }», il progetto in cui hai continuato a scrivere, anziché la copia che Scrivener ha conservato prima di un aggiornamento.
import-scrivener-report-chose-backup = Hai importato «{ $backup }», la copia più vecchia che Scrivener ha conservato prima di un aggiornamento.
import-scrivener-report-title-placeholder-rejected = Il titolo nelle impostazioni del progetto era un segnaposto di un modello, quindi Skribisto ha proposto al suo posto il nome del progetto.
import-scrivener-report-author-placeholder-rejected = Il nome dell'autore nelle impostazioni del progetto era un segnaposto di un modello, quindi Skribisto non ne ha proposto nessuno.
import-scrivener-report-title-from-compile-settings = Skribisto ha proposto il titolo indicato nelle impostazioni di compilazione del progetto: il nome che il progetto aveva quando è stato creato, a meno che non vi sia stato impostato un titolo in Scrivener.
import-scrivener-report-title-from-template = Skribisto ha proposto il titolo del modello stesso.
import-scrivener-report-title-from-folder-name = Skribisto ha proposto come titolo il nome del progetto.
import-scrivener-report-title-from-project-properties = Skribisto ha proposto il titolo indicato nelle proprietà del progetto.
import-scrivener-report-author-from-project = Skribisto ha proposto il nome dell'autore indicato nelle impostazioni del progetto. Lì può essere rimasto l'autore di un modello, che verrebbe stampato sul frontespizio.
import-scrivener-report-language-not-stated = Il progetto non indicava la sua lingua, quindi Skribisto ha proposto quella della propria interfaccia. La lingua determina come si contano le parole e come si scrivono le virgolette.
import-scrivener-report-language-from-script = Skribisto ha proposto la lingua in base alla scrittura del manoscritto ({ $tag }).
import-scrivener-report-you-changed-it = { $hint } Hai sostituito questa proposta.
import-scrivener-report-content-salvaged-text = Il testo di «{ $item }» mancava dal progetto. Le sue parole provengono dall'indice di ricerca di Scrivener, come testo semplice e forse da una versione precedente: controllale.
import-scrivener-report-content-salvaged-notes = Le note di «{ $item }» mancavano dal progetto. Le loro parole provengono dall'indice di ricerca di Scrivener, come testo semplice e forse da una versione precedente: controllale.
import-scrivener-report-content-salvaged-synopsis = La sinossi di «{ $item }» mancava dal progetto. Le sue parole provengono dall'indice di ricerca di Scrivener, come testo semplice e forse da una versione precedente: controllale.
import-scrivener-report-document-others = { $others ->
    [one] { $sentence } Lo stesso accade in 1 altro punto, elencato sotto.
   *[other] { $sentence } Lo stesso accade in altri { $others } punti, elencati sotto.
}
import-scrivener-report-item-notes = { $path } (note del documento)
import-scrivener-report-item-synopsis = { $path } (sinossi)
import-scrivener-report-item-project-note = { $path } (nota del progetto)
import-scrivener-report-item-recovered = { $path } (testo recuperato)
import-scrivener-report-untitled = Senza titolo
import-scrivener-report-no-findings = L'importazione non ha altro da segnalare.
import-scrivener-report-group-unread = Non leggibile
import-scrivener-report-group-check = Da controllare
import-scrivener-report-group-changed = Trasferito diversamente
import-scrivener-report-group-left = Rimasto indietro
import-scrivener-report-group-count = { $group } ({ $count })
import-scrivener-report-items-toggle = { $count ->
    [one] 1 elemento
   *[other] { $count } elementi
}
import-scrivener-report-items-toggle-named = { $count ->
    [one] 1 elemento: { $line }
   *[other] { $count } elementi: { $line }
}
import-scrivener-report-names-toggle = { $count ->
    [one] 1 nome
   *[other] { $count } nomi
}
import-scrivener-report-names-toggle-named = { $count ->
    [one] 1 nome: { $line }
   *[other] { $count } nomi: { $line }
}
import-scrivener-report-more = { $count ->
    [one] E 1 altro.
   *[other] E altri { $count }.
}
import-scrivener-report-open-item = Apri nell'editor
import-scrivener-report-links-hint = Apri il nuovo progetto, poi mostra di nuovo questo rapporto dalla tavolozza dei comandi: ogni elemento qui elencato si aprirà con un clic.
import-scrivener-copy-report = Copia rapporto
import-scrivener-report-copied = Rapporto copiato
import-scrivener-report-copy-failed = Non è stato possibile copiare il rapporto.
import-scrivener-report-close = Chiudi
import-scrivener-report-none-title = Nessun rapporto di importazione
import-scrivener-report-none = Nessuna importazione da Scrivener è terminata dall'avvio di Skribisto. Skribisto conserva i rapporti delle ultime importazioni fino alla chiusura.
import-scrivener-report-gone = { $count ->
    [one] Il rapporto di questa importazione non è più conservato. Skribisto conserva il rapporto dell'ultima importazione fino alla chiusura.
   *[other] Il rapporto di questa importazione non è più conservato. Skribisto conserva i rapporti delle ultime { $count } importazioni fino alla chiusura.
}

## Words the import writes into the new project, in the language Skribisto shows itself in
import-scrivener-manuscript-binder = Manoscritto
import-scrivener-notes-binder = Note
import-scrivener-notes-of = { $title } (note)
import-scrivener-project-notes = Note del progetto
import-scrivener-recovered = Testi recuperati
import-scrivener-untitled-template = Modello senza titolo
import-scrivener-research-folder = File di ricerca di { $name }
import-scrivener-list-separator = {", "}
import-scrivener-label-details = Dal campo «{ $field }» in Scrivener
import-scrivener-status-details = Dal campo «{ $field }» in Scrivener
import-scrivener-unnamed-label = Dicitura { $number }
import-scrivener-unnamed-status = Stato { $number }
import-scrivener-label-tag = { $name } (dicitura)
import-scrivener-untitled-tag = Senza titolo
import-scrivener-keyword-details = La parola chiave «{ $path }» in Scrivener
import-scrivener-unnamed-keyword = Parola chiave { $number }
import-scrivener-keyword-tag = { $name } (parola chiave)
import-scrivener-collection-details = Una raccolta in Scrivener
import-scrivener-collection-tag = { $name } (raccolta)
import-scrivener-field-details = Dal campo «{ $field }» in Scrivener
import-scrivener-field-value = { $field }: { $option }
import-scrivener-field-tag = { $name } (campo)
import-scrivener-deadline-day = Scadenza
import-scrivener-from-scrivener = Da Scrivener
import-scrivener-note-project = Progetto
import-scrivener-note-project-bookmarks = Segnalibri del progetto
import-scrivener-note-replacements = Sostituzioni di compilazione
import-scrivener-note-auto-complete = Voci di completamento automatico
import-scrivener-note-saved-searches = Ricerche salvate
import-scrivener-note-custom-metadata = Metadati personalizzati
import-scrivener-note-targets = Obiettivi
import-scrivener-note-bookmarks = Segnalibri
import-scrivener-abbreviated-title = Titolo abbreviato
import-scrivener-subjects = Argomenti
import-scrivener-company = Società
import-scrivener-session-target = Obiettivo della sessione
import-scrivener-session-target-calculated = Obiettivo della sessione, calcolato da Scrivener in base alla scadenza
import-scrivener-writing-days = Giorni di scrittura
import-scrivener-weekday-monday = lunedì
import-scrivener-weekday-tuesday = martedì
import-scrivener-weekday-wednesday = mercoledì
import-scrivener-weekday-thursday = giovedì
import-scrivener-weekday-friday = venerdì
import-scrivener-weekday-saturday = sabato
import-scrivener-weekday-sunday = domenica
import-scrivener-draft-target = Obiettivo della bozza
import-scrivener-deadline = Scadenza
import-scrivener-unit-words = parole
import-scrivener-unit-characters = caratteri
import-scrivener-unit-pages = pagine
import-scrivener-search-query = Testo cercato
import-scrivener-search-in = Cercato in
import-scrivener-search-operator = Corrispondenza
import-scrivener-search-in-all = tutto
import-scrivener-search-in-title = titoli
import-scrivener-search-in-text = testo
import-scrivener-search-in-notes = note
import-scrivener-search-in-synopsis = sinossi
import-scrivener-search-in-keywords = parole chiave
import-scrivener-search-in-label = diciture
import-scrivener-search-in-status = stati
import-scrivener-search-in-section-type = tipi di sezione
import-scrivener-search-in-custom-metadata = metadati personalizzati
import-scrivener-search-any = una qualsiasi delle parole
import-scrivener-search-all = tutte le parole
import-scrivener-search-exact = frase esatta
import-scrivener-label-value = { $label }: { $value }
import-scrivener-replacement-regex = espressione regolare
import-scrivener-replacement-off = disattivata
import-scrivener-in-own-note = Nella nota nata da questo documento.


## Import a Scrivener project: what the import says about the project, in the writer's words.
## Every key the converter can give has a line here (see `import_scrivener::messages`).

import-scrivener-paragraph = { $title }. { $body }

## When the project cannot be imported
## A body that ends “then read the project again” is shown in the wizard beside the button
## import-scrivener-check-again, so it uses the same words. A failed import's notice shows
## the same body beside import-scrivener-review-again, which reads the project again too:
## the body quotes no button.
import-scrivener-refusal-missing-title = A questo percorso non c'è nulla
import-scrivener-refusal-missing-body = «{ $file }» non c'è più. Scegli di nuovo il progetto.
import-scrivener-refusal-unreadable-title = Questo progetto non può essere letto
import-scrivener-refusal-unreadable-body = Skribisto non è riuscito a leggere «{ $file }» ({ $reason }). Verifica di poterlo aprire, poi riprova.
import-scrivener-refusal-not-a-scrivener-project-title = Qui non c'è alcun progetto Scrivener
import-scrivener-refusal-not-a-scrivener-project-body = Scegli la cartella .scriv del progetto, il file .scrivx che contiene, un backup .zip o un .scrivtemplate.
import-scrivener-refusal-manifest-without-files-title = I documenti non sono in questa copia
import-scrivener-refusal-manifest-without-files-body = «{ $manifest }» è qui, ma non la cartella Files che contiene i documenti che elenca. Forse un servizio di sincronizzazione non ha finito di copiare il progetto, oppure è stato copiato soltanto il file .scrivx. Se hai scelto il file .scrivx, scegli invece la cartella del progetto con «Scegli una cartella…».
import-scrivener-refusal-search-too-wide-title = Questa cartella contiene troppo materiale da esaminare
import-scrivener-refusal-search-too-wide-body = Ha più di { $limit } cartelle da esaminare. Scegli direttamente il progetto Scrivener.
import-scrivener-refusal-scrivener-one-for-mac-title = Questo progetto è stato creato con Scrivener 1 per Mac
import-scrivener-refusal-scrivener-one-for-mac-body = Skribisto legge i progetti di Scrivener 2 e versioni successive. Aprilo una volta in Scrivener 2 o 3 per aggiornarlo, oppure compilalo in un documento Word e importa quello.
import-scrivener-refusal-ios-stub-without-mobile-data-title = Questo è solo un file sostitutivo di Scrivener per iPhone o iPad
import-scrivener-refusal-ios-stub-without-mobile-data-body = Il progetto vero e proprio non è qui. Sincronizzalo dal tuo iPhone o iPad, oppure condividilo da lì come .zip, e importa quello.
import-scrivener-refusal-newer-title = Questo progetto è stato creato con uno Scrivener più recente
import-scrivener-refusal-unknown-format-version-body = Il suo formato, { $version }, non è tra quelli che Skribisto conosce. Controlla se c'è un aggiornamento di Skribisto. Nel frattempo, compilalo in un documento Word in Scrivener e importa quello.
import-scrivener-refusal-newer-format-stamp-body = I suoi file sono contrassegnati come versione { $stamp }, e Skribisto legge fino alla versione { $newest }. Controlla se c'è un aggiornamento di Skribisto. Nel frattempo, compilalo in un documento Word in Scrivener e importa quello.
import-scrivener-refusal-version-stamp-unreadable-title = Non è possibile leggere la versione del progetto
import-scrivener-refusal-version-stamp-unreadable-body = Il suo file Files/version.txt non contiene un numero di versione, quindi Skribisto non può sapere se è in grado di leggere questo progetto. Apri il progetto una volta in Scrivener, poi importalo di nuovo.
import-scrivener-refusal-damaged-manifest-title = Non è possibile leggere il file del raccoglitore
import-scrivener-refusal-damaged-manifest-body = «{ $manifest }» è danneggiato. { $spare_copies ->
    [0] Apri il progetto in Scrivener, che può ripararlo, poi importalo di nuovo.
    [1] Scrivener ne conserva una copia di riserva all'interno del progetto, che Skribisto non legge. Apri il progetto in Scrivener, che può ripararlo, poi importalo di nuovo.
   *[other] Scrivener ne conserva { $spare_copies } copie di riserva all'interno del progetto, che Skribisto non legge. Apri il progetto in Scrivener, che può ripararlo, poi importalo di nuovo.
}
import-scrivener-refusal-damaged-manifest-new-project-placeholder-body = «{ $manifest }» è il file vuoto che Scrivener scrive per un progetto non ancora salvato. Apri il progetto in Scrivener e lascia che lo salvi, poi importalo di nuovo.
import-scrivener-refusal-manifest-too-large-title = Questo file del raccoglitore è troppo grande
import-scrivener-refusal-manifest-too-large-body = «{ $manifest }» supera i { $limit } MB che Skribisto legge. Nessun progetto reale si avvicina a tale dimensione, quindi il file potrebbe essere danneggiato.
import-scrivener-refusal-no-item-identifiers-title = Il raccoglitore non elenca alcun documento che Skribisto possa trovare
import-scrivener-refusal-no-item-identifiers-body = Nessuno dei suoi elementi ha un identificatore, quindi non è possibile trovarne i file. Apri il progetto una volta in Scrivener, poi importalo di nuovo.
import-scrivener-refusal-incomplete-copy-title = I documenti mancano da questa copia
import-scrivener-refusal-incomplete-copy-body = La cartella Files è qui, ma i documenti al suo interno no. Forse un servizio di sincronizzazione non ha finito di copiare il progetto. Attendi che finisca, oppure copia di nuovo l'intera cartella .scriv, poi rileggi il progetto.
import-scrivener-refusal-archive-title = Questo archivio non può essere aperto
import-scrivener-refusal-archive-unreadable-body = Non è stato possibile leggere «{ $file }» ({ $reason }). Potrebbe essere danneggiato. Estrailo tu stesso, poi scegli la cartella .scriv che contiene.
import-scrivener-refusal-archive-unsafe-name-title = Questo archivio non può essere aperto in sicurezza
import-scrivener-refusal-archive-unsafe-name-body = «{ $file }» contiene un file chiamato «{ $member }», che finirebbe fuori dall'archivio, quindi Skribisto non lo apre.
import-scrivener-refusal-archive-unsafe-name-empty-body = «{ $file }» contiene un file senza nome, quindi Skribisto non lo apre.
import-scrivener-refusal-archive-unsafe-name-control-character-body = «{ $file }» contiene un file il cui nome include un carattere di controllo, quindi Skribisto non lo apre.
import-scrivener-refusal-archive-too-large-title = Questo archivio è troppo grande per essere aperto in sicurezza
import-scrivener-refusal-archive-too-many-members-body = Contiene più di { $limit } file. Estrailo tu stesso, poi scegli la cartella .scriv che contiene.
import-scrivener-refusal-archive-member-too-large-body = «{ $member }» si decomprime in più di { $limit } MB. Estrai tu stesso l'archivio, poi scegli la cartella .scriv che contiene.
import-scrivener-refusal-archive-too-large-body = Si decomprime in più di { $limit } MB. Estrailo tu stesso, poi scegli la cartella .scriv che contiene.
import-scrivener-refusal-archive-encrypted-title = Questo archivio è protetto da una password
import-scrivener-refusal-archive-encrypted-body = Estrai tu stesso «{ $file }», poi scegli la cartella .scriv che contiene.
import-scrivener-refusal-archive-unsupported-compression-title = Questo archivio è compresso in un modo che Skribisto non sa leggere
import-scrivener-refusal-archive-unsupported-compression-body = «{ $member }» è compresso con { $method }. Estrai tu stesso l'archivio, poi scegli la cartella .scriv che contiene.
import-scrivener-refusal-duplicate-archive-members-title = Questo archivio elenca alcuni file due volte
import-scrivener-refusal-duplicate-archive-members-body = «{ $file }» contiene più di un file con lo stesso nome, quindi Skribisto non può sapere quale appartiene al progetto. Estrailo tu stesso, poi scegli la cartella .scriv che contiene.
import-scrivener-refusal-template-undecodable-title = Questo file di modello è incompleto
import-scrivener-refusal-template-undecodable-too-large-title = Questo file di modello è troppo grande per essere aperto
import-scrivener-refusal-template-undecodable-not-a-template-title = Questo file non è un modello Scrivener
import-scrivener-refusal-template-undecodable-unknown-encoding-title = Questo file di modello è salvato in una forma che Skribisto non conosce
import-scrivener-refusal-template-undecodable-body = Non è possibile decodificare «{ $file }»: potrebbe terminare prima del progetto che contiene. Scaricalo di nuovo, oppure aprilo in Scrivener e crea un progetto a partire da esso.
import-scrivener-refusal-template-undecodable-too-large-body = «{ $file }» supera i { $limit } MB. Aprilo in Scrivener, crea un progetto a partire da esso, poi importa quel progetto.
import-scrivener-refusal-template-undecodable-not-a-template-body = «{ $file }» non contiene un progetto Scrivener. Scegli il file di modello salvato da Scrivener, oppure un progetto creato a partire da esso.
import-scrivener-refusal-template-undecodable-unknown-encoding-body = «{ $file }» salva il suo progetto in un modo che Skribisto non sa decodificare. Aprilo in Scrivener, crea un progetto a partire da esso, poi importa quel progetto.
import-scrivener-refusal-cloud-placeholders-title = { $count ->
    [one] 1 file di questo progetto è ancora nel cloud
   *[other] { $count } file di questo progetto sono ancora nel cloud
}
import-scrivener-refusal-cloud-placeholders-body = { $count ->
    [1] Non è stato scaricato su questo computer: { $examples }. Rendi il progetto disponibile offline nell'app di sincronizzazione, poi rileggi il progetto.
   *[other] Non sono stati scaricati su questo computer, per esempio { $examples }. Rendi il progetto disponibile offline nell'app di sincronizzazione, poi rileggi il progetto.
}
import-scrivener-refusal-unreadable-members-title = { $count ->
    [one] 1 file di questo progetto non può essere letto
   *[other] { $count } file di questo progetto non possono essere letti
}
import-scrivener-refusal-unreadable-members-body = Per esempio { $examples }. Verifica di avere il permesso di leggere l'intero progetto e che il disco sia in buono stato, poi rileggi il progetto.
import-scrivener-refusal-needed-member-is-link-title = Questo progetto rimanda fuori dalla propria cartella
import-scrivener-refusal-needed-member-is-link-body = «{ $member }» è un collegamento a un'altra posizione di questo computer, e Skribisto non segue i collegamenti. Copia il progetto in modo che ogni file si trovi al suo interno, poi riprova.
import-scrivener-refusal-project-changed-during-read-title = Il progetto è cambiato durante la lettura
import-scrivener-refusal-project-changed-during-read-body = Scrivener o un servizio di sincronizzazione ci sta ancora scrivendo. Chiudilo in Scrivener, attendi la fine della sincronizzazione, poi rileggi il progetto.
import-scrivener-refusal-too-many-members-title = Questo progetto contiene troppi file
import-scrivener-refusal-too-many-members-body = Ha più di { $limit } file, più di qualsiasi progetto reale. Potrebbe non essere un progetto Scrivener, oppure potrebbe essere danneggiato.
import-scrivener-refusal-too-large-title = Questo progetto è troppo grande
import-scrivener-refusal-too-large-body = Il suo testo supera i { $limit } MB, più di quanto Skribisto possa importare in sicurezza.
import-scrivener-refusal-structure-unmappable-title = Skribisto non riesce a trasporre questo raccoglitore
import-scrivener-refusal-structure-unmappable-body = Non è stato possibile trasferire il raccoglitore senza perdere o collocare male qualcosa. Si tratta di un difetto di Skribisto, non del tuo progetto. Segnalalo indicando ciò che mostra «Dettagli».

## When there is a choice to make
import-scrivener-roots-title = { $count ->
    [one] Qui c'è { $count } progetto. Quale scegli?
   *[other] Qui ci sono { $count } progetti. Quale scegli?
}
import-scrivener-roots-hint = Questa cartella o questo archivio contiene più di un progetto Scrivener. Scegli quello da importare.
import-scrivener-manifests-title = Questo progetto ha { $count } file del raccoglitore
import-scrivener-manifests-hint = Scrivener conserva il raccoglitore, l'elenco di tutti i documenti e le cartelle, in un file .scrivx. Quelli in più sono stati lasciati molto probabilmente da un servizio di sincronizzazione. Scegli quello in cui hai lavorato per ultimo.
import-scrivener-manifests-hint-preselected = Scrivener conserva il raccoglitore, l'elenco di tutti i documenti e le cartelle, in un file .scrivx. Quelli in più sono stati lasciati molto probabilmente da un servizio di sincronizzazione. Quello selezionato è con ogni probabilità quello in cui hai lavorato per ultimo: controllalo prima di proseguire.
import-scrivener-manifest-row = { $file } · salvato il { $date } · { $documents ->
    [one] 1 elemento
   *[other] { $documents } elementi
}
import-scrivener-manifest-row-undated = { $file } · { $documents ->
    [one] 1 elemento
   *[other] { $documents } elementi
}
import-scrivener-manifest-row-unread = { $file } · non leggibile
import-scrivener-enclosing-title = Fa parte del progetto Scrivener «{ $project }»
import-scrivener-enclosing-body = Hai scelto un file o una cartella all'interno di un progetto. Skribisto importa l'intero progetto.
import-scrivener-enclosing-use = Importa «{ $project }»
import-scrivener-upgrade-title = Questa è la copia che Scrivener ha conservato prima di un aggiornamento
import-scrivener-upgrade-body = Quando Scrivener aggiorna un progetto a una versione più recente, conserva quello vecchio come «{ $backup }». Il progetto in cui hai continuato a scrivere è «{ $current }», lì accanto.
import-scrivener-upgrade-body-unclear = Quando Scrivener aggiorna un progetto a una versione più recente, conserva quello vecchio come «{ $backup }». Il progetto lì accanto, «{ $current }», non è stato letto correttamente, quindi scegli quale importare.
import-scrivener-upgrade-use-current = Importa «{ $current }», il progetto in cui hai continuato a scrivere
import-scrivener-upgrade-keep-backup = Importa questa copia più vecchia, «{ $backup }»

## What must be accepted before going on
import-scrivener-ios-pending-title = Le modifiche fatte in Scrivener per iPhone o iPad non sono ancora in questa copia
import-scrivener-ios-pending-body = Scrivener su un computer integra quelle modifiche la prossima volta che apre il progetto. Apri il progetto una volta in Scrivener sul tuo computer, poi importalo di nuovo.
import-scrivener-ios-pending-accept = Importalo senza quelle modifiche
import-scrivener-no-text-title = { $count ->
    [one] 1 documento, e nemmeno una parola di testo in questa copia
   *[other] { $count } documenti, e nemmeno una parola di testo in questa copia
}
import-scrivener-no-text-body = Forse un servizio di sincronizzazione sta ancora copiando il progetto, oppure la copia ha perso i suoi documenti. Importandolo ora, ogni documento arriva vuoto.
import-scrivener-no-text-accept = Importa i documenti vuoti

## Things to know, nothing to decide
import-scrivener-notice-project-found-below-pick-title = Il progetto è stato trovato all'interno della cartella che hai scelto
import-scrivener-notice-project-found-below-pick-body = Si trova in «{ $path }».
import-scrivener-notice-source-is-template-title = Questo è un modello Scrivener
import-scrivener-notice-source-is-template-named = Questo è il modello Scrivener «{ $title }»
import-scrivener-notice-source-is-template-body = Importarlo crea un nuovo progetto da ciò che il modello contiene, istruzioni comprese.
import-scrivener-notice-source-is-backup-title = Sembra un backup creato da Scrivener
import-scrivener-notice-source-is-backup-body = Contiene il progetto com'era al momento del backup. Se hai continuato a scrivere dopo, scegli il progetto stesso.
import-scrivener-notice-importing-pre-upgrade-copy-title = Stai importando la copia che Scrivener ha conservato prima di un aggiornamento
import-scrivener-notice-importing-pre-upgrade-copy-body = Il progetto in cui hai continuato a scrivere è lì accanto e contiene tutto ciò che hai scritto da allora.
import-scrivener-notice-imported-from-ios-stub-title = Questo progetto è stato creato in Scrivener per iPhone o iPad
import-scrivener-notice-imported-from-ios-stub-body = Non è mai stato aperto su un computer, quindi viene letto dalla copia conservata dall'iPhone o dall'iPad.
import-scrivener-notice-stale-mobile-binder-ignored-title = Un vecchio raccoglitore di un iPhone o iPad viene messo da parte
import-scrivener-notice-stale-mobile-binder-ignored-body = Scrivener su un computer ne ha già integrato le modifiche.
import-scrivener-notice-identical-manifest-copies-title = { $count ->
    [one] Questo progetto ha { $count } file del raccoglitore identico
   *[other] Questo progetto ha { $count } file del raccoglitore identici
}
import-scrivener-notice-identical-manifest-copies-body = Contengono la stessa cosa, quindi una qualsiasi di esse dà lo stesso progetto.
import-scrivener-notice-manifest-copies-ignored-title = Le copie in più del file del raccoglitore vengono messe da parte
import-scrivener-notice-manifest-copies-ignored-body = «{ $chosen }» è quello in cui Scrivener ha lavorato per ultimo. { $count ->
    [one] 1 copia lasciata da un servizio di sincronizzazione viene messa da parte.
   *[other] { $count } copie lasciate da un servizio di sincronizzazione vengono messe da parte.
}
import-scrivener-notice-pinned-manifest-looks-like-a-copy-title = Il file del raccoglitore che hai scelto sembra una copia di sincronizzazione
import-scrivener-notice-pinned-manifest-looks-like-a-copy-body = «{ $name }» ha il nome di una copia di «{ $original }», che si trova lì accanto. Se l'importazione sembra datata, scegli invece «{ $original }».
import-scrivener-notice-pinned-manifest-has-siblings-title = { $count ->
    [one] Accanto a quello che hai scelto c'è 1 altro file del raccoglitore
   *[other] Accanto a quello che hai scelto ci sono altri { $count } file del raccoglitore
}
import-scrivener-notice-pinned-manifest-has-siblings-body = Viene letto soltanto quello che hai scelto.
import-scrivener-notice-legacy-v1-files-beside-title = I file di uno Scrivener più vecchio vengono messi da parte
import-scrivener-notice-legacy-v1-files-beside-body = Sono rimasti indietro quando il progetto è stato aggiornato. Vengono letti soltanto i file della versione attuale.
import-scrivener-notice-mixed-payload-trees-title = In questo progetto ci sono file di due versioni di Scrivener
import-scrivener-notice-mixed-payload-trees-body = I documenti vengono letti dove il file del raccoglitore indica che si trovano.
import-scrivener-notice-version-disagreement-title = I file di questo progetto non concordano tutti sulla sua versione
import-scrivener-notice-id-scheme-mismatch-body = Il file del raccoglitore indica la versione { $version }, ma dà ai documenti i nomi usati da un'altra versione. Vengono letti dove indica. Se l'importazione sembra errata, apri il progetto una volta in Scrivener e importalo di nuovo.
import-scrivener-notice-version-stamp-mismatch-body = Viene letto come lo descrive il suo file del raccoglitore. Se l'importazione sembra errata, apri il progetto una volta in Scrivener e importalo di nuovo.
import-scrivener-notice-unrecognised-writer-title = Questo progetto è stato salvato da un programma che Skribisto non riconosce
import-scrivener-notice-unrecognised-writer-named = Si presenta come «{ $creator }». Viene letto come un progetto Scrivener.
import-scrivener-notice-unrecognised-writer-body = Non indica il proprio nome. Viene letto come un progetto Scrivener.
import-scrivener-notice-invalid-item-identifier-title = { $count ->
    [one] 1 elemento del raccoglitore ha un identificatore non valido
   *[other] { $count } elementi del raccoglitore hanno un identificatore non valido
}
import-scrivener-notice-invalid-item-identifier-body = Un identificatore di questo tipo viene messo da parte, e l'elemento che lo porta viene conservato.
import-scrivener-notice-items-without-identifier-title = { $count ->
    [one] 1 elemento del raccoglitore non ha un identificatore
   *[other] { $count } elementi del raccoglitore non hanno un identificatore
}
import-scrivener-notice-items-without-identifier-body = Senza un identificatore, non è possibile trovare i file di un elemento, quindi l'elemento arriva vuoto.
import-scrivener-notice-duplicate-item-key-title = Due elementi del raccoglitore condividono lo stesso identificatore: «{ $item }»
import-scrivener-notice-duplicate-item-key-body = Il primo conserva i documenti. Il secondo conserva il suo posto e arriva vuoto.
import-scrivener-notice-lock-here-title = Scrivener ha questo progetto aperto su questo computer
import-scrivener-notice-lock-here-body = Chiudilo prima in Scrivener, così ogni modifica viene salvata.
import-scrivener-notice-lock-title = Scrivener aveva questo progetto aperto su «{ $host }»
import-scrivener-notice-lock-elsewhere-title = Scrivener aveva questo progetto aperto su un altro computer
import-scrivener-notice-lock-body = Se è ancora aperto lì, chiudilo prima: i file potrebbero non contenere ancora le ultime modifiche. Se Scrivener è stato chiuso da tempo, puoi importarlo così com'è.
import-scrivener-notice-archived-while-open-title = Questo archivio è stato creato mentre Scrivener aveva il progetto aperto
import-scrivener-notice-archived-while-open-body = Le modifiche fatte dopo quel momento potrebbero mancare.
import-scrivener-notice-content-missing-notes = Mancano le note di «{ $item }»
import-scrivener-notice-content-missing-synopsis = Manca la sinossi di «{ $item }»
import-scrivener-notice-content-missing-text = Manca il testo di «{ $item }»
import-scrivener-notice-content-missing-body = { $place ->
    [notes] Il loro file non è in questa copia, quindi arrivano vuote. Forse un servizio di sincronizzazione sta ancora copiando il progetto.
    [synopsis] Il suo file non è in questa copia, quindi arriva vuota. Forse un servizio di sincronizzazione sta ancora copiando il progetto.
   *[text] Il suo file non è in questa copia, quindi arriva vuoto. Forse un servizio di sincronizzazione sta ancora copiando il progetto.
}
import-scrivener-notice-content-missing-salvaged = { $place ->
    [notes] Il loro file non è in questa copia, ma l'indice di ricerca di Scrivener ne conservava ancora le parole. Vengono conservate come testo semplice, forse da una versione precedente.
    [synopsis] Il suo file non è in questa copia, ma l'indice di ricerca di Scrivener ne conservava ancora le parole. Vengono conservate come testo semplice, forse da una versione precedente.
   *[text] Il suo file non è in questa copia, ma l'indice di ricerca di Scrivener ne conservava ancora le parole. Vengono conservate come testo semplice, forse da una versione precedente.
}
import-scrivener-notice-content-truncated-notes = Le note di «{ $item }» sono vuote sul disco
import-scrivener-notice-content-truncated-synopsis = La sinossi di «{ $item }» è vuota sul disco
import-scrivener-notice-content-truncated-text = Il testo di «{ $item }» è vuoto sul disco
import-scrivener-notice-content-truncated-body = { $place ->
    [notes] Scrivener le ha registrate con del testo, ma il loro file non ne contiene: forse una sincronizzazione si è fermata a metà. Arrivano vuote.
    [synopsis] Scrivener l'ha registrata con del testo, ma il suo file non ne contiene: forse una sincronizzazione si è fermata a metà. Arriva vuota.
   *[text] Scrivener lo ha registrato con del testo, ma il suo file non ne contiene: forse una sincronizzazione si è fermata a metà. Arriva vuoto.
}
import-scrivener-notice-missing-media-title = Manca il file di «{ $item }»
import-scrivener-notice-missing-media-body = L'immagine o il file di ricerca che contiene non è in questa copia.
import-scrivener-notice-sync-conflict-copies-title = { $count ->
    [one] È stata trovata 1 copia di sincronizzazione di un documento
   *[other] Sono state trovate { $count } copie di sincronizzazione di documenti
}
import-scrivener-notice-sync-conflict-copies-body = { $missing ->
    [0] Le copie di sincronizzazione vengono trasferite come testi recuperati, in una cartella a parte, così puoi confrontarle con i documenti.
    [one] Le copie di sincronizzazione vengono trasferite come testi recuperati, in una cartella a parte. Una di esse è la copia di un documento che non è più nel progetto.
   *[other] Le copie di sincronizzazione vengono trasferite come testi recuperati, in una cartella a parte. { $missing } di esse sono copie di documenti che non sono più nel progetto.
}
import-scrivener-notice-recovered-unreferenced-text-title = { $count ->
    [one] È stato trovato 1 documento che nessun elemento del raccoglitore elenca
   *[other] Sono stati trovati { $count } documenti che nessun elemento del raccoglitore elenca
}
import-scrivener-notice-recovered-unreferenced-text-body = I documenti di questo tipo vengono trasferiti come testi recuperati, in una cartella a parte.
import-scrivener-notice-sync-debris-ignored-title = { $count ->
    [one] 1 copia in più lasciata da un servizio di sincronizzazione viene messa da parte
   *[other] { $count } copie in più lasciate da un servizio di sincronizzazione vengono messe da parte
}
import-scrivener-notice-sync-debris-ignored-body = Nemmeno Scrivener legge copie di questo tipo. Vengono letti soltanto i file propri del progetto.
import-scrivener-notice-unrecognised-files-title = { $count ->
    [one] 1 file che non fa parte di un progetto Scrivener viene messo da parte
   *[other] { $count } file che non fanno parte di un progetto Scrivener vengono messi da parte
}
import-scrivener-notice-examples = Per esempio { $examples }.
import-scrivener-notice-symlink-not-followed-title = { $count ->
    [one] 1 collegamento all'interno del progetto non viene seguito
   *[other] { $count } collegamenti all'interno del progetto non vengono seguiti
}
import-scrivener-notice-unreadable-files-skipped-title = { $count ->
    [one] Non è stato possibile leggere 1 file di cui l'importazione non ha bisogno
   *[other] Non è stato possibile leggere { $count } file di cui l'importazione non ha bisogno
}
import-scrivener-notice-cloud-placeholders-skipped-title = { $count ->
    [one] 1 file ancora nel cloud viene saltato
   *[other] { $count } file ancora nel cloud vengono saltati
}
import-scrivener-notice-cloud-placeholders-skipped-body = Niente di ciò che serve all'importazione è ancora nel cloud.
import-scrivener-notice-no-text-found-title = { $count ->
    [1] 1 documento, e non contiene testo
   *[other] { $count } documenti, e nessuno contiene testo
}
import-scrivener-notice-no-text-found-body = Questo progetto sembra vuoto. Viene importato così com'è.
import-scrivener-notice-metadata-unreadable-title = Non è stato possibile leggere «{ $member }»
import-scrivener-notice-metadata-unreadable-body = Ciò che Scrivener vi conservava non viene trasferito.
import-scrivener-notice-dangling-keyword-title = { $count ->
    [one] 1 riferimento rimanda a una parola chiave che il progetto non definisce
   *[other] { $count } riferimenti rimandano a parole chiave che il progetto non definisce
}
import-scrivener-notice-dangling-label-title = { $count ->
    [one] 1 riferimento rimanda a una dicitura che il progetto non definisce
   *[other] { $count } riferimenti rimandano a diciture che il progetto non definisce
}
import-scrivener-notice-dangling-status-title = { $count ->
    [one] 1 riferimento rimanda a uno stato che il progetto non definisce
   *[other] { $count } riferimenti rimandano a stati che il progetto non definisce
}
import-scrivener-notice-dangling-section-type-title = { $count ->
    [one] 1 riferimento rimanda a un tipo di sezione che il progetto non definisce
   *[other] { $count } riferimenti rimandano a tipi di sezione che il progetto non definisce
}
import-scrivener-notice-dangling-custom-field-title = { $count ->
    [one] 1 riferimento rimanda a un campo di metadati personalizzati che il progetto non definisce
   *[other] { $count } riferimenti rimandano a campi di metadati personalizzati che il progetto non definisce
}
import-scrivener-notice-dangling-item-title = { $count ->
    [one] 1 riferimento rimanda a un documento che il progetto non definisce
   *[other] { $count } riferimenti rimandano a documenti che il progetto non definisce
}
import-scrivener-notice-dangling-references-body = Un riferimento a qualcosa che il progetto non definisce viene escluso.
import-scrivener-notice-snapshots-not-imported-line = { $count ->
    [one] 1 istantanea resta nel tuo progetto Scrivener: Skribisto non trasferisce le istantanee.
   *[other] { $count } istantanee restano nel tuo progetto Scrivener: Skribisto non trasferisce le istantanee.
}
import-scrivener-notice-writing-history-not-imported-line = { $days ->
    [one] Il registro di Scrivener delle parole scritte ogni giorno, 1 giorno, non viene trasferito. Skribisto avvia il proprio conteggio dal primo salvataggio.
   *[other] Il registro di Scrivener delle parole scritte ogni giorno, { $days } giorni, non viene trasferito. Skribisto avvia il proprio conteggio dal primo salvataggio.
}
import-scrivener-notice-title-placeholder-rejected-hint = Il titolo nelle impostazioni del progetto è un segnaposto di un modello, quindi al suo posto viene proposto il nome del progetto.
import-scrivener-notice-author-placeholder-rejected-hint = Il nome dell'autore nelle impostazioni del progetto è un segnaposto di un modello, quindi non ne viene proposto nessuno.
import-scrivener-notice-title-from-project-hint-compile-settings = Questo titolo è quello indicato nelle impostazioni di compilazione del progetto. Finché non vi si imposta un titolo, Scrivener conserva il nome che il progetto aveva quando è stato creato.
import-scrivener-notice-title-from-project-hint-template = Questo titolo è quello del modello stesso.
import-scrivener-notice-title-from-project-hint-folder-name = Questo titolo è il nome del progetto.
import-scrivener-notice-title-from-project-hint-project-properties = Questo titolo è quello indicato nelle proprietà del progetto.
import-scrivener-notice-author-from-project-hint = Questo nome è quello indicato nelle impostazioni del progetto. Lì può essere rimasto l'autore di un modello, che verrebbe stampato sul frontespizio.
import-scrivener-notice-language-not-stated-hint = Il progetto non indica la sua lingua, quindi viene proposta quella dell'interfaccia di Skribisto. La lingua determina come si contano le parole e come si scrivono le virgolette.
import-scrivener-notice-language-from-script-hint = Proposta in base alla scrittura del manoscritto ({ $tag }).

## Before you import: what the import does with the binder, or leaves behind
import-scrivener-structure-script-mode-draft-line = { $script ->
    [1] { $documents ->
        [1] L'unico documento del manoscritto è scritto in modalità sceneggiatura. Skribisto ne conserva le parole, non l'impaginazione da sceneggiatura.
       *[other] { $script } dei { $documents } documenti del manoscritto è scritto in modalità sceneggiatura. Skribisto ne conserva le parole, non l'impaginazione da sceneggiatura.
    }
   *[other] { $script } dei { $documents } documenti del manoscritto sono scritti in modalità sceneggiatura. Skribisto ne conserva le parole, non l'impaginazione da sceneggiatura.
}
import-scrivener-structure-one-document-manuscript-line = { $count ->
    [1] Un documento del manoscritto contiene i suoi capitoli come intestazioni.
   *[other] { $count } documenti del manoscritto contengono i loro capitoli come intestazioni.
}
import-scrivener-structure-title-repeats-number-line = { $count ->
    [one] 1 titolo di capitolo o di parte contiene anche il proprio numero, come «Capitolo II» o «Parte prima», quindi un'esportazione potrebbe stampare il numero due volte.
   *[other] { $count } titoli di capitoli o di parti contengono anche il proprio numero, come «Capitolo II» o «Parte prima», quindi un'esportazione potrebbe stampare il numero due volte.
}
import-scrivener-structure-claim-overruled-line = { $count ->
    [one] 1 elemento sembra un libro, una parte o un capitolo, ma si trova dentro un elemento già di quel livello, quindi viene trasferito a un livello inferiore.
   *[other] { $count } elementi sembrano libri, parti o capitoli, ma ognuno si trova dentro un elemento già di quel livello, quindi vengono trasferiti a un livello inferiore.
}
import-scrivener-structure-adopted-by-chapter-line = { $count ->
    [one] 1 documento che segue un capitolo diventa la parte finale di quel capitolo.
   *[other] { $count } documenti che seguono un capitolo diventano la parte finale di quel capitolo.
}
import-scrivener-structure-looks-like-manuscript-line = { $count ->
    [one] 1 cartella di primo livello conservata nelle note sembra manoscritto. Puoi inviarla al manoscritto come un altro libro.
   *[other] { $count } cartelle di primo livello conservate nelle note sembrano manoscritto. Puoi inviarle al manoscritto come altri libri.
}
import-scrivener-structure-generic-sheet-title-line = { $count ->
    [one] 1 voce della bibbia narrativa ha ancora il titolo di un modello, come «Eroe». Skribisto cerca quel titolo nel tuo testo ed elenca ogni punto in cui compare nella pagina Nel testo: rinomina la voce se quella parola è comune nel tuo libro.
   *[other] { $count } voci della bibbia narrativa hanno ancora il titolo di un modello, come «Eroe». Skribisto cerca quei titoli nel tuo testo ed elenca ogni punto in cui compaiono nella pagina Nel testo: rinomina le voci se quelle parole sono comuni nel tuo libro.
}
import-scrivener-structure-placeholder-title-line = { $count ->
    [one] 1 titolo indicava soltanto il proprio livello e viene lasciato vuoto: il raccoglitore e le esportazioni lo numerano.
   *[other] { $count } titoli indicavano soltanto il proprio livello e vengono lasciati vuoti: il raccoglitore e le esportazioni li numerano.
}
import-scrivener-structure-untitled-item-line = { $count ->
    [one] 1 elemento non aveva un titolo in Scrivener e resta senza titolo.
   *[other] { $count } elementi non avevano un titolo in Scrivener e restano senza titolo.
}
import-scrivener-structure-title-cleaned-line = { $count ->
    [one] 1 titolo conteneva interruzioni di riga o tabulazioni, che vengono rimosse.
   *[other] { $count } titoli contenevano interruzioni di riga o tabulazioni, che vengono rimosse.
}
import-scrivener-structure-own-text-moved-line = { $count ->
    [one] 1 cartella ha un testo proprio, che viene spostato in un primo documento al suo interno.
   *[other] { $count } cartelle hanno un testo proprio, che viene spostato in un primo documento al loro interno.
}
import-scrivener-structure-text-with-children-line = { $count ->
    [one] 1 documento che contiene altri documenti diventa una cartella.
   *[other] { $count } documenti che contengono altri documenti diventano cartelle.
}
import-scrivener-structure-empty-folder-plain-line = { $count ->
    [one] 1 cartella vuota resta una cartella semplice, così nessuna esportazione stampa un'intestazione che Scrivener non ha mai stampato.
   *[other] { $count } cartelle vuote restano cartelle semplici, così nessuna esportazione stampa un'intestazione che Scrivener non ha mai stampato.
}
import-scrivener-structure-book-inside-book-line = { $count ->
    [one] 1 libro dentro un libro diventa una parte.
   *[other] { $count } libri dentro un libro diventano parti.
}
import-scrivener-structure-prologue-not-numbered-line = { $count ->
    [one] 1 prologo o epilogo viene escluso dalla numerazione dei capitoli.
   *[other] { $count } prologhi o epiloghi vengono esclusi dalla numerazione dei capitoli.
}
import-scrivener-structure-matter-inside-book-line = { $count ->
    [one] 1 elemento delle pagine preliminari o finali esterno al manoscritto va all'interno del libro come paratesto, escluso dalle esportazioni finché non lo includi.
   *[other] { $count } elementi delle pagine preliminari o finali esterni al manoscritto vanno all'interno del libro come paratesto, esclusi dalle esportazioni finché non li includi.
}
import-scrivener-structure-paratext-not-counted-line = { $documents ->
    [one] 1 documento del manoscritto diventa paratesto (pagine preliminari o finali), quindi { $words ->
        [one] { $words } parola non conta
       *[other] { $words } parole non contano
    } più come manoscritto.
   *[other] { $documents } documenti del manoscritto diventano paratesto (pagine preliminari o finali), quindi { $words ->
        [one] { $words } parola non conta
       *[other] { $words } parole non contano
    } più come manoscritto.
}
import-scrivener-structure-document-notes-line = { $count ->
    [one] Le note di 1 documento diventano una nota accanto a esso.
   *[other] Le note di { $count } documenti diventano note accanto a essi.
}
import-scrivener-structure-draft-content-as-note-line = Il testo proprio della cartella del manoscritto diventa una nota in cima al manoscritto.
import-scrivener-structure-story-bible-folder-line = { $count ->
    [one] 1 cartella di personaggi o luoghi diventa un'etichetta della bibbia narrativa.
   *[other] { $count } cartelle di personaggi o luoghi diventano etichette della bibbia narrativa.
}
import-scrivener-structure-templates-created-line = { $count ->
    [one] 1 modello di documento diventa un modello di nota.
   *[other] { $count } modelli di documento diventano modelli di nota.
}
import-scrivener-structure-template-nesting-flattened-line = { $count ->
    [one] 1 cartella di modelli dentro un'altra viene appiattita: ogni testo al suo interno diventa un modello.
   *[other] { $count } cartelle di modelli dentro altre vengono appiattite: ogni testo al loro interno diventa un modello.
}
import-scrivener-structure-template-kept-as-note-line = { $count ->
    [one] 1 modello contiene ciò che un modello di nota non può contenere, e viene conservato come nota.
   *[other] { $count } modelli contengono ciò che un modello di nota non può contenere, e vengono conservati come note.
}
import-scrivener-structure-recovered-texts-line = { $count ->
    [one] 1 testo che nessun elemento del raccoglitore elencava viene trasferito in una cartella a parte.
   *[other] { $count } testi che nessun elemento del raccoglitore elencava vengono trasferiti in una cartella a parte.
}
import-scrivener-structure-project-notes-line = { $count ->
    [one] 1 nota del progetto viene trasferita in una cartella a parte.
   *[other] { $count } note del progetto vengono trasferite in una cartella a parte.
}
import-scrivener-structure-trash-brought-across-line = { $items ->
    [1] Il cestino viene trasferito: 1 elemento, che puoi ripristinare dal cestino.
   *[other] Il cestino viene trasferito: { $items } elementi, { $writing ->
        [0] nessuno dei quali con testo scritto
        [one] uno dei quali con testo scritto
       *[other] { $writing } dei quali con testo scritto
    }, che puoi ripristinare dal cestino.
}
import-scrivener-structure-not-compiled-line = { $count ->
    [one] 1 elemento che Scrivener escludeva dalla compilazione viene escluso anche dalle esportazioni.
   *[other] { $count } elementi che Scrivener escludeva dalla compilazione vengono esclusi anche dalle esportazioni.
}
import-scrivener-structure-media-in-draft-line = { $count ->
    [one] 1 immagine o file nel manoscritto diventa una nota, perché un libro non può contenerlo.
   *[other] { $count } immagini o file nel manoscritto diventano note, perché un libro non può contenerli.
}
import-scrivener-structure-left-out-line = { $count ->
    [one] 1 cartella di primo livello viene esclusa, con { $items ->
        [one] 1 elemento
       *[other] { $items } elementi
    }.
   *[other] { $count } cartelle di primo livello vengono escluse, con { $items ->
        [one] 1 elemento
       *[other] { $items } elementi
    }.
}
import-scrivener-structure-trash-left-out-line = { $items ->
    [1] Il cestino resta indietro: 1 elemento.
   *[other] Il cestino resta indietro: { $items } elementi, { $writing ->
        [0] nessuno dei quali con testo scritto
        [one] uno dei quali con testo scritto
       *[other] { $writing } dei quali con testo scritto
    }.
}
import-scrivener-structure-card-image-left-behind-line = { $count ->
    [one] 1 immagine di scheda su un elemento del manoscritto resta indietro: verrebbe stampata nel libro.
   *[other] { $count } immagini di scheda su elementi del manoscritto restano indietro: verrebbero stampate nel libro.
}
import-scrivener-structure-default-template-not-linked-line = { $count ->
    [one] 1 cartella aveva un modello predefinito per i nuovi documenti al suo interno. In Skribisto solo un'etichetta della bibbia narrativa può portare un modello, quindi quel collegamento non viene conservato.
   *[other] { $count } cartelle avevano un modello predefinito per i nuovi documenti al loro interno. In Skribisto solo un'etichetta della bibbia narrativa può portare un modello, quindi quei collegamenti non vengono conservati.
}
import-scrivener-structure-page-break-not-kept-line = { $count ->
    [one] 1 impostazione «Interruzione di pagina prima» non trova posto in Skribisto e non viene conservata.
   *[other] { $count } impostazioni «Interruzione di pagina prima» non trovano posto in Skribisto e non vengono conservate.
}
import-scrivener-structure-compile-as-is-not-kept-line = { $count ->
    [one] 1 impostazione «Compila così com'è» non ha un equivalente in Skribisto e non viene conservata.
   *[other] { $count } impostazioni «Compila così com'è» non hanno un equivalente in Skribisto e non vengono conservate.
}

## What the import did with the labels, the status list and the dates
import-scrivener-metadata-status-categories-guessed-line = { $count ->
    [one] 1 stato che Skribisto non riconosce riceve un tipo in base al suo posto nell'elenco. Controllalo in Impostazioni ▸ Progetto ▸ Stati.
   *[other] { $count } stati che Skribisto non riconosce ricevono un tipo in base al loro posto nell'elenco. Controllali in Impostazioni ▸ Progetto ▸ Stati.
}
import-scrivener-metadata-dates-unreadable-line = { $count ->
    [one] Non è stato possibile leggere le date di 1 elemento: porta la data dell'importazione.
   *[other] Non è stato possibile leggere le date di { $count } elementi: portano la data dell'importazione.
}
import-scrivener-metadata-labels-became-tags-line = { $tags ->
    [one] Le diciture diventano 1 etichetta, applicata a { $rows ->
        [one] { $rows } elemento
       *[other] { $rows } elementi
    }.
   *[other] Le diciture diventano { $tags } etichette, applicate a { $rows ->
        [one] { $rows } elemento
       *[other] { $rows } elementi
    }.
}
import-scrivener-metadata-statuses-became-ladder-line = { $rungs ->
    [one] L'elenco degli stati diventa quello del progetto: { $rungs } stato, applicato a { $rows ->
        [one] { $rows } elemento
       *[other] { $rows } elementi
    }.
   *[other] L'elenco degli stati diventa quello del progetto: { $rungs } stati, applicati a { $rows ->
        [one] { $rows } elemento
       *[other] { $rows } elementi
    }.
}
import-scrivener-metadata-ladder-seeded-line = { $count ->
    [1] Il progetto non aveva un elenco di stati, quindi inizia con lo stato predefinito di Skribisto.
   *[other] Il progetto non aveva un elenco di stati, quindi inizia con i { $count } stati predefiniti di Skribisto.
}
import-scrivener-metadata-label-renamed-line = In Scrivener il campo Dicitura si chiamava «{ $title }». Questo nome si trova nei dettagli di ogni etichetta.
import-scrivener-metadata-status-renamed-line = In Scrivener il campo Stato si chiamava «{ $title }». Questo nome si trova nei dettagli di ogni stato.
import-scrivener-metadata-none-label-collapsed-line = { $count ->
    [one] La dicitura «{ $name }», conservata accanto alla dicitura vuota di Scrivener con lo stesso nome, viene letta come nessuna dicitura su 1 elemento.
   *[other] La dicitura «{ $name }», conservata accanto alla dicitura vuota di Scrivener con lo stesso nome, viene letta come nessuna dicitura su { $count } elementi.
}
import-scrivener-metadata-none-status-collapsed-line = { $count ->
    [one] Lo stato «{ $name }», conservato accanto allo stato vuoto di Scrivener con lo stesso nome, viene letto come nessuno stato su 1 elemento.
   *[other] Lo stato «{ $name }», conservato accanto allo stato vuoto di Scrivener con lo stesso nome, viene letto come nessuno stato su { $count } elementi.
}
import-scrivener-metadata-unnamed-label-line = Una dicitura senza nome prende il nome dal suo numero, { $id }.
import-scrivener-metadata-unnamed-status-line = Uno stato senza nome prende il nome dal suo numero, { $id }.
import-scrivener-metadata-tag-name-disambiguated-line = { $count ->
    [one] 1 etichetta prende un nuovo nome, perché un'altra etichetta ha già il suo.
   *[other] { $count } etichette prendono un nuovo nome, perché altre etichette hanno già il loro.
}
import-scrivener-metadata-rung-names-disambiguated-line = { $count ->
    [one] 1 stato ha lo stesso nome di un altro, quindi riceve un numero dopo il nome. Controllalo in Impostazioni ▸ Progetto ▸ Stati.
   *[other] { $count } stati hanno lo stesso nome di un altro, quindi ciascuno riceve un numero dopo il nome. Controllali in Impostazioni ▸ Progetto ▸ Stati.
}
import-scrivener-metadata-unused-labels-skipped-line = { $count ->
    [one] 1 dicitura che nessun documento porta non viene importata.
   *[other] { $count } diciture che nessun documento porta non vengono importate.
}
import-scrivener-metadata-labels-without-row-line = { $count ->
    [one] La dicitura di 1 elemento che non trova posto nel nuovo progetto non viene trasferita.
   *[other] Le diciture di { $count } elementi che non trovano posto nel nuovo progetto non vengono trasferite.
}
import-scrivener-metadata-statuses-without-row-line = { $count ->
    [one] Lo stato di 1 elemento che non trova posto nel nuovo progetto non viene trasferito.
   *[other] Gli stati di { $count } elementi che non trovano posto nel nuovo progetto non vengono trasferiti.
}

import-scrivener-metadata-unnamed-keyword-line = Una parola chiave senza nome prende il nome dal suo numero, { $id }.
import-scrivener-metadata-keywords-without-row-line = { $count ->
    [one] Le parole chiave di 1 elemento che non trova posto nel nuovo progetto non vengono trasferite.
   *[other] Le parole chiave di { $count } elementi che non trovano posto nel nuovo progetto non vengono trasferite.
}
import-scrivener-metadata-custom-values-without-row-line = { $count ->
    [one] I metadati personalizzati di 1 elemento che non trova posto nel nuovo progetto non vengono trasferiti.
   *[other] I metadati personalizzati di { $count } elementi che non trovano posto nel nuovo progetto non vengono trasferiti.
}
import-scrivener-metadata-bookmarks-without-row-line = { $count ->
    [one] I segnalibri di 1 elemento che non trova posto nel nuovo progetto non vengono trasferiti.
   *[other] I segnalibri di { $count } elementi che non trovano posto nel nuovo progetto non vengono trasferiti.
}
import-scrivener-metadata-targets-without-row-line = { $count ->
    [one] L'obiettivo di 1 elemento che non trova posto nel nuovo progetto non viene trasferito.
   *[other] Gli obiettivi di { $count } elementi che non trovano posto nel nuovo progetto non vengono trasferiti.
}

## What the import does with the keywords, custom metadata, collections, bookmarks, targets,
## the deadline, the auto-complete list and the icons. Each line is said twice: on the Review
## page before the import, then in its report, so it reads the same at both times. `$note`
## and `$folder` are the titles the import gives a note and its "From Scrivener" folder;
## `$day` is a day as YYYY-MM-DD.
import-scrivener-metadata-pov-hint-line = { $count ->
    [1] La dicitura o il valore di campo { $values } corrisponde alla voce della bibbia narrativa { $matches }. Se indica il punto di vista, impostalo nelle Proprietà: l'importazione non collega nulla.
   *[other] Le diciture o i valori di campo { $values } corrispondono alle voci della bibbia narrativa { $matches }. Se indicano punti di vista, impostali nelle Proprietà: l'importazione non collega nulla.
}
import-scrivener-metadata-checkbox-default-unknown-line = Il campo casella di controllo «{ $field }» ha un valore predefinito che Skribisto non sa leggere, quindi non può sapere quali documenti sono selezionati per impostazione predefinita. Solo i documenti selezionati uno per uno portano la sua etichetta.
import-scrivener-metadata-goals-in-other-unit-line = { $count ->
    [one] 1 obiettivo è impostato nell'altra unità di conteggio. Viene conservato e appare quando cambi l'unità di conteggio in Impostazioni ▸ Progetto ▸ Struttura.
   *[other] { $count } obiettivi sono impostati nell'altra unità di conteggio. Vengono conservati e appaiono quando cambi l'unità di conteggio in Impostazioni ▸ Progetto ▸ Struttura.
}
import-scrivener-metadata-counting-differs-line = Scrivener calcolava l'obiettivo della bozza su documenti diversi da quelli che conta Skribisto. L'obiettivo del libro conta le scene incluse nelle esportazioni.
import-scrivener-metadata-writing-days-not-read-line = Non è possibile leggere i giorni di scrittura «{ $raw }», quindi il Ritmo del libro conta ogni giorno come giorno di scrittura. Modificali lì.
import-scrivener-metadata-script-mode-rows-line = { $count ->
    [one] 1 elemento era in modalità sceneggiatura in Scrivener. Skribisto ne conserva le parole come prosa, senza l'impaginazione da sceneggiatura.
   *[other] { $count } elementi erano in modalità sceneggiatura in Scrivener. Skribisto ne conserva le parole come prosa, senza l'impaginazione da sceneggiatura.
}
import-scrivener-metadata-keywords-became-tags-line = { $tags ->
    [one] 1 parola chiave diventa un'etichetta, applicata a { $rows ->
        [one] { $rows } elemento
       *[other] { $rows } elementi
    }.
   *[other] { $tags } parole chiave diventano etichette, applicate a { $rows ->
        [one] { $rows } elemento
       *[other] { $rows } elementi
    }.
}
import-scrivener-metadata-keyword-hierarchy-flattened-line = { $count ->
    [one] 1 parola chiave si trovava dentro un'altra in Scrivener. Le etichette non hanno gerarchia: i dettagli della sua etichetta indicano dove si trovava.
   *[other] { $count } parole chiave si trovavano dentro altre in Scrivener. Le etichette non hanno gerarchia: i dettagli di ogni etichetta indicano dove si trovava la sua parola chiave.
}
import-scrivener-metadata-fields-became-tags-line = { $fields ->
    [one] 1 campo elenco o casella di controllo diventa { $tags ->
        [one] { $tags } etichetta, applicata
       *[other] { $tags } etichette, applicate
    } a { $rows ->
        [one] { $rows } elemento
       *[other] { $rows } elementi
    }.
   *[other] { $fields } campi elenco e casella di controllo diventano { $tags ->
        [one] { $tags } etichetta, applicata
       *[other] { $tags } etichette, applicate
    } a { $rows ->
        [one] { $rows } elemento
       *[other] { $rows } elementi
    }.
}
import-scrivener-metadata-collections-became-tags-line = { $tags ->
    [one] 1 raccolta diventa un'etichetta, applicata a { $rows ->
        [one] { $rows } elemento
       *[other] { $rows } elementi
    }.
   *[other] { $tags } raccolte diventano etichette, applicate a { $rows ->
        [one] { $rows } elemento
       *[other] { $rows } elementi
    }.
}
import-scrivener-metadata-collection-order-lost-line = { $count ->
    [one] Un'etichetta non ha un ordine, quindi l'ordine dei documenti di 1 raccolta non viene conservato.
   *[other] Un'etichetta non ha un ordine, quindi l'ordine dei documenti di { $count } raccolte non viene conservato.
}
import-scrivener-metadata-bookmarks-became-links-line = { $count ->
    [one] 1 segnalibro diventa un collegamento tra due note, o tra una scena e una voce della bibbia narrativa: vedi Collegamenti e Personaggi presenti.
   *[other] { $count } segnalibri diventano collegamenti tra note, o tra scene e voci della bibbia narrativa: vedi Collegamenti e Personaggi presenti.
}
import-scrivener-metadata-goals-kept-line = { $count ->
    [one] L'obiettivo che Scrivener assegnava a 1 documento diventa il suo obiettivo in Skribisto.
   *[other] Gli obiettivi che Scrivener assegnava a { $count } documenti diventano i loro obiettivi in Skribisto.
}
import-scrivener-metadata-draft-target-kept-words-line = { $value ->
    [one] L'obiettivo della bozza di 1 parola diventa l'obiettivo del libro.
   *[other] L'obiettivo della bozza di { $value } parole diventa l'obiettivo del libro.
}
import-scrivener-metadata-draft-target-kept-characters-line = { $value ->
    [one] L'obiettivo della bozza di 1 carattere diventa l'obiettivo del libro.
   *[other] L'obiettivo della bozza di { $value } caratteri diventa l'obiettivo del libro.
}
import-scrivener-metadata-pace-planned-line = La scadenza, { $day }, imposta il Ritmo del libro, che pianifica le parole di ogni giorno di scrittura scelto in Scrivener, a partire dal giorno dell'importazione.
import-scrivener-metadata-pace-planned-day-off-line = La scadenza, { $day }, imposta il Ritmo del libro, che pianifica le parole di ogni giorno di scrittura scelto in Scrivener, a partire dal giorno dell'importazione. Come in Scrivener, il giorno della scadenza è un giorno di pausa.
import-scrivener-metadata-dictionary-words-line = { $count ->
    [one] 1 parola del completamento automatico entra nel dizionario personale del progetto, così non viene segnalata come errore di ortografia.
   *[other] { $count } parole del completamento automatico entrano nel dizionario personale del progetto, così non vengono segnalate come errori di ortografia.
}
import-scrivener-metadata-listed-project-line = { $count ->
    [one] 1 dettaglio del progetto è elencato nella nota «{ $note }» della cartella «{ $folder }».
   *[other] { $count } dettagli del progetto sono elencati nella nota «{ $note }» della cartella «{ $folder }».
}
import-scrivener-metadata-listed-project-bookmarks-line = { $count ->
    [one] 1 segnalibro del progetto è elencato nella nota «{ $note }» della cartella «{ $folder }».
   *[other] { $count } segnalibri del progetto sono elencati nella nota «{ $note }» della cartella «{ $folder }».
}
import-scrivener-metadata-listed-replacements-line = { $count ->
    [one] 1 sostituzione di compilazione è elencata nella nota «{ $note }» della cartella «{ $folder }». Skribisto non la applica.
   *[other] { $count } sostituzioni di compilazione sono elencate nella nota «{ $note }» della cartella «{ $folder }». Skribisto non le applica.
}
import-scrivener-metadata-listed-auto-complete-line = { $count ->
    [one] 1 voce di completamento automatico di più parole è elencata nella nota «{ $note }» della cartella «{ $folder }».
   *[other] { $count } voci di completamento automatico di più parole sono elencate nella nota «{ $note }» della cartella «{ $folder }».
}
import-scrivener-metadata-listed-saved-searches-line = { $count ->
    [one] 1 ricerca salvata è elencata nella nota «{ $note }» della cartella «{ $folder }».
   *[other] { $count } ricerche salvate sono elencate nella nota «{ $note }» della cartella «{ $folder }».
}
import-scrivener-metadata-listed-custom-metadata-line = { $count ->
    [one] 1 valore di metadati personalizzati che un'etichetta non può contenere è elencato nella nota «{ $note }» della cartella «{ $folder }».
   *[other] { $count } valori di metadati personalizzati che un'etichetta non può contenere sono elencati nella nota «{ $note }» della cartella «{ $folder }».
}
import-scrivener-metadata-listed-targets-line = { $count ->
    [one] 1 obiettivo di documento è elencato nella nota «{ $note }» della cartella «{ $folder }».
   *[other] { $count } obiettivi di documento sono elencati nella nota «{ $note }» della cartella «{ $folder }».
}
import-scrivener-metadata-listed-bookmarks-line = { $count ->
    [one] 1 segnalibro che nessun collegamento di Skribisto può mostrare è elencato nella nota «{ $note }» della cartella «{ $folder }».
   *[other] { $count } segnalibri che nessun collegamento di Skribisto può mostrare sono elencati nella nota «{ $note }» della cartella «{ $folder }».
}
import-scrivener-metadata-targets-on-notes-line = { $count ->
    [one] 1 obiettivo di documento si trova su una nota, il cui testo non è conteggiato da alcun obiettivo, quindi non diventa un obiettivo.
   *[other] { $count } obiettivi di documento si trovano su note, il cui testo non è conteggiato da alcun obiettivo, quindi non diventano obiettivi.
}
import-scrivener-metadata-targets-on-split-documents-line = { $count ->
    [one] 1 obiettivo di documento si trova su un documento diviso alle sue intestazioni, i cui capitoli non sono conteggiati insieme da un unico obiettivo, quindi non diventa un obiettivo.
   *[other] { $count } obiettivi di documento si trovano su documenti divisi alle loro intestazioni, i cui capitoli non sono conteggiati insieme da un unico obiettivo, quindi non diventano obiettivi.
}
import-scrivener-metadata-bookmark-ends-without-row-line = { $count ->
    [one] 1 segnalibro porta a un documento che non trova posto nel nuovo progetto, come un modello di documento. È elencato in base a dove si trovava il documento in Scrivener.
   *[other] { $count } segnalibri portano a documenti che non trovano posto nel nuovo progetto, come i modelli di documento. Sono elencati in base a dove si trovavano i documenti in Scrivener.
}
import-scrivener-metadata-unused-keywords-skipped-line = { $count ->
    [one] 1 parola chiave che nessun documento porta non viene importata.
   *[other] { $count } parole chiave che nessun documento porta non vengono importate.
}
import-scrivener-metadata-list-value-invalid-line = { $count ->
    [one] 1 valore del campo elenco «{ $field }» non corrisponde a nessuna delle sue opzioni e non viene importato.
   *[other] { $count } valori del campo elenco «{ $field }» non corrispondono a nessuna delle sue opzioni e non vengono importati.
}
import-scrivener-metadata-undefined-custom-field-line = { $count ->
    [one] 1 valore appartiene a un campo personalizzato che il progetto non definisce più e che nemmeno Scrivener mostra. Non viene importato.
   *[other] { $count } valori appartengono a campi personalizzati che il progetto non definisce più e che nemmeno Scrivener mostra. Non vengono importati.
}
import-scrivener-metadata-collection-members-missing-line = { $count ->
    [one] 1 documento della raccolta «{ $name }» non trova posto nel nuovo progetto, oppure non è più nel progetto Scrivener.
   *[other] { $count } documenti della raccolta «{ $name }» non trovano posto nel nuovo progetto, oppure non sono più nel progetto Scrivener.
}
import-scrivener-metadata-empty-collections-skipped-line = { $count ->
    [one] 1 raccolta che non contiene documenti non viene importata.
   *[other] { $count } raccolte che non contengono documenti non vengono importate.
}
import-scrivener-metadata-bookmarks-unresolved-line = { $count ->
    [one] 1 segnalibro porta a un documento che il progetto Scrivener non contiene più e non viene importato.
   *[other] { $count } segnalibri portano a documenti che il progetto Scrivener non contiene più e non vengono importati.
}
import-scrivener-metadata-targets-in-pages-line = { $count ->
    [one] 1 obiettivo di documento conta le pagine, che Skribisto non conta, quindi non diventa un obiettivo.
   *[other] { $count } obiettivi di documento contano le pagine, che Skribisto non conta, quindi non diventano obiettivi.
}
import-scrivener-metadata-targets-unreadable-line = { $count ->
    [one] 1 obiettivo non viene importato: il suo valore non è un numero, è inferiore a zero o supera quanto un obiettivo può contenere.
   *[other] { $count } obiettivi non vengono importati: i loro valori non sono numeri, sono inferiori a zero o superano quanto un obiettivo può contenere.
}
import-scrivener-metadata-draft-target-in-pages-line = { $value ->
    [one] L'obiettivo della bozza di 1 pagina non diventa un obiettivo: Skribisto conta parole o caratteri. È elencato nella nota «{ $note }» della cartella «{ $folder }».
   *[other] L'obiettivo della bozza di { $value } pagine non diventa un obiettivo: Skribisto conta parole o caratteri. È elencato nella nota «{ $note }» della cartella «{ $folder }».
}
import-scrivener-metadata-draft-target-no-book-words-line = { $value ->
    [one] L'obiettivo della bozza di 1 parola non diventa un obiettivo, perché il manoscritto forma più libri, o nessuno. È elencato nella nota «{ $note }» della cartella «{ $folder }».
   *[other] L'obiettivo della bozza di { $value } parole non diventa un obiettivo, perché il manoscritto forma più libri, o nessuno. È elencato nella nota «{ $note }» della cartella «{ $folder }».
}
import-scrivener-metadata-draft-target-no-book-characters-line = { $value ->
    [one] L'obiettivo della bozza di 1 carattere non diventa un obiettivo, perché il manoscritto forma più libri, o nessuno. È elencato nella nota «{ $note }» della cartella «{ $folder }».
   *[other] L'obiettivo della bozza di { $value } caratteri non diventa un obiettivo, perché il manoscritto forma più libri, o nessuno. È elencato nella nota «{ $note }» della cartella «{ $folder }».
}
import-scrivener-metadata-deadline-passed-line = La scadenza, { $day }, è passata, quindi non imposta il Ritmo del libro. È elencata nella nota «{ $note }» della cartella «{ $folder }».
import-scrivener-metadata-deadline-no-writing-day-line = Dal giorno dell'importazione alla scadenza, { $day }, non resta alcun giorno di scrittura, quindi la scadenza non imposta il Ritmo del libro. È elencata nella nota «{ $note }» della cartella «{ $folder }».
import-scrivener-metadata-deadline-template-line = La scadenza, { $day }, era salvata nel modello, quindi non imposta il Ritmo del libro. È elencata nella nota «{ $note }» della cartella «{ $folder }».
import-scrivener-metadata-deadline-no-word-goal-line = La scadenza, { $day }, non imposta il Ritmo del libro, perché la bozza non ha un obiettivo di parole verso cui pianificare. È elencata nella nota «{ $note }» della cartella «{ $folder }».
import-scrivener-metadata-deadline-character-goal-line = La scadenza, { $day }, non imposta il Ritmo del libro, perché l'obiettivo della bozza conta i caratteri e il Ritmo conta le parole. È elencata nella nota «{ $note }» della cartella «{ $folder }».
import-scrivener-metadata-deadline-no-book-line = La scadenza, { $day }, non imposta il Ritmo di alcun libro, perché il manoscritto forma più libri, o nessuno. È elencata nella nota «{ $note }» della cartella «{ $folder }».
import-scrivener-metadata-deadline-unreadable-line = Non è possibile leggere la scadenza, quindi non imposta il Ritmo del libro. È elencata così come è scritta nella nota «{ $note }» della cartella «{ $folder }».
import-scrivener-metadata-session-target-words-line = { $value ->
    [one] L'obiettivo della sessione di 1 parola non viene importato: in Skribisto, l'obiettivo della sessione di scrittura è un'unica impostazione per tutti i progetti. È elencato nella nota «{ $note }» della cartella «{ $folder }».
   *[other] L'obiettivo della sessione di { $value } parole non viene importato: in Skribisto, l'obiettivo della sessione di scrittura è un'unica impostazione per tutti i progetti. È elencato nella nota «{ $note }» della cartella «{ $folder }».
}
import-scrivener-metadata-session-target-characters-line = { $value ->
    [one] L'obiettivo della sessione di 1 carattere non viene importato: in Skribisto, l'obiettivo della sessione di scrittura è un'unica impostazione per tutti i progetti, in parole. È elencato nella nota «{ $note }» della cartella «{ $folder }».
   *[other] L'obiettivo della sessione di { $value } caratteri non viene importato: in Skribisto, l'obiettivo della sessione di scrittura è un'unica impostazione per tutti i progetti, in parole. È elencato nella nota «{ $note }» della cartella «{ $folder }».
}
import-scrivener-metadata-session-target-pages-line = { $value ->
    [one] L'obiettivo della sessione di 1 pagina non viene importato: in Skribisto, l'obiettivo della sessione di scrittura è un'unica impostazione per tutti i progetti, in parole. È elencato nella nota «{ $note }» della cartella «{ $folder }».
   *[other] L'obiettivo della sessione di { $value } pagine non viene importato: in Skribisto, l'obiettivo della sessione di scrittura è un'unica impostazione per tutti i progetti, in parole. È elencato nella nota «{ $note }» della cartella «{ $folder }».
}
import-scrivener-metadata-session-target-calculated-words-line = { $value ->
    [one] L'obiettivo della sessione di 1 parola, calcolato da Scrivener in base alla scadenza e ai giorni di scrittura, non viene importato: in Skribisto, l'obiettivo della sessione di scrittura è un'unica impostazione per tutti i progetti. È elencato nella nota «{ $note }» della cartella «{ $folder }».
   *[other] L'obiettivo della sessione di { $value } parole, calcolato da Scrivener in base alla scadenza e ai giorni di scrittura, non viene importato: in Skribisto, l'obiettivo della sessione di scrittura è un'unica impostazione per tutti i progetti. È elencato nella nota «{ $note }» della cartella «{ $folder }».
}
import-scrivener-metadata-session-target-calculated-characters-line = { $value ->
    [one] L'obiettivo della sessione di 1 carattere, calcolato da Scrivener in base alla scadenza e ai giorni di scrittura, non viene importato: in Skribisto, l'obiettivo della sessione di scrittura è un'unica impostazione per tutti i progetti, in parole. È elencato nella nota «{ $note }» della cartella «{ $folder }».
   *[other] L'obiettivo della sessione di { $value } caratteri, calcolato da Scrivener in base alla scadenza e ai giorni di scrittura, non viene importato: in Skribisto, l'obiettivo della sessione di scrittura è un'unica impostazione per tutti i progetti, in parole. È elencato nella nota «{ $note }» della cartella «{ $folder }».
}
import-scrivener-metadata-session-target-calculated-pages-line = { $value ->
    [one] L'obiettivo della sessione di 1 pagina, calcolato da Scrivener in base alla scadenza e ai giorni di scrittura, non viene importato: in Skribisto, l'obiettivo della sessione di scrittura è un'unica impostazione per tutti i progetti, in parole. È elencato nella nota «{ $note }» della cartella «{ $folder }».
   *[other] L'obiettivo della sessione di { $value } pagine, calcolato da Scrivener in base alla scadenza e ai giorni di scrittura, non viene importato: in Skribisto, l'obiettivo della sessione di scrittura è un'unica impostazione per tutti i progetti, in parole. È elencato nella nota «{ $note }» della cartella «{ $folder }».
}
import-scrivener-metadata-icons-dropped-line = { $count ->
    [one] 1 elemento aveva un'icona nel raccoglitore, che non viene trasferita: Skribisto mostra l'icona di ogni elemento in base al suo tipo.
   *[other] { $count } elementi avevano icone nel raccoglitore, che non vengono trasferite: Skribisto mostra l'icona di ogni elemento in base al suo tipo.
}

## What converting the prose did, as the import's report says it. `$count` is how many texts,
## or how many of a thing, as each line says; `$example` quotes the project.
import-scrivener-prose-unreadable-line = { $count ->
    [one] Non è stato possibile leggere 1 testo. Arriva vuoto.
   *[other] Non è stato possibile leggere { $count } testi. Arrivano vuoti.
}
import-scrivener-prose-comments-unreadable-line = { $count ->
    [one] Non è stato possibile leggere i commenti e le note a piè di pagina collegate di 1 testo. Le parole che segnavano vengono conservate.
   *[other] Non è stato possibile leggere i commenti e le note a piè di pagina collegate di { $count } testi. Le parole che segnavano vengono conservate.
}
import-scrivener-prose-read-as-plain-text-line = { $count ->
    [one] 1 testo era salvato come testo semplice anziché come testo formattato, e viene letto come tale.
   *[other] { $count } testi erano salvati come testo semplice anziché come testo formattato, e vengono letti come tali.
}
import-scrivener-prose-inline-note-unmatched-line = { $count ->
    [one] 1 segno di nota in linea non ha un segno corrispondente e viene conservato come testo.
   *[other] { $count } segni di nota in linea non hanno un segno corrispondente e vengono conservati come testo.
}
import-scrivener-prose-style-unresolved-line = { $count ->
    [one] 1 segno di stile indica uno stile che il progetto non definisce. Viene rimosso e il testo conserva la sua formattazione.
   *[other] { $count } segni di stile indicano uno stile che il progetto non definisce. Vengono rimossi e il testo conserva la sua formattazione.
}
import-scrivener-prose-styles-lost-in-export-line = { $count ->
    [one] 1 paragrafo potrebbe essere un'intestazione che ha perso il suo stile quando Scrivener 3 ha esportato il progetto nel formato precedente, e arriva come paragrafo normale. Controllalo, oppure importa il progetto da cui è stato esportato.
   *[other] { $count } paragrafi potrebbero essere intestazioni che hanno perso il loro stile quando Scrivener 3 ha esportato il progetto nel formato precedente, e arrivano come paragrafi normali. Controllali, oppure importa il progetto da cui sono stati esportati.
}
import-scrivener-prose-comment-anchor-without-entry-line = { $count ->
    [one] 1 commento o nota a piè di pagina aveva perso il suo contenuto in Scrivener. Le parole che segnava vengono conservate, senza di esso.
   *[other] { $count } commenti o note a piè di pagina avevano perso il loro contenuto in Scrivener. Le parole che segnavano vengono conservate, senza di essi.
}
import-scrivener-prose-comment-entry-without-anchor-line = { $count ->
    [one] 1 commento non aveva più un posto nel suo testo e viene agganciato al primo paragrafo.
   *[other] { $count } commenti non avevano più un posto nel loro testo e vengono agganciati al primo paragrafo.
}
import-scrivener-prose-footnote-entry-without-anchor-line = { $count ->
    [one] 1 nota a piè di pagina collegata non aveva più un posto nel suo testo, quindi viene citata alla fine di quel testo.
   *[other] { $count } note a piè di pagina collegate non avevano più un posto nei loro testi, quindi ognuna viene citata alla fine del proprio testo.
}
import-scrivener-prose-range-comment-reading-line = { $count ->
    [one] 1 commento di uno Scrivener più vecchio è stato collocato con un'altra lettura della sua posizione. Controlla che sia sulle parole giuste.
   *[other] { $count } commenti di uno Scrivener più vecchio sono stati collocati con un'altra lettura della loro posizione. Controlla che siano sulle parole giuste.
}
import-scrivener-prose-range-comment-reading-markers-counted-line = { $count ->
    [one] 1 commento di uno Scrivener più vecchio è stato collocato contando i segni delle sue note in linea. Controlla che sia sulle parole giuste.
   *[other] { $count } commenti di uno Scrivener più vecchio sono stati collocati contando i segni delle loro note in linea. Controlla che siano sulle parole giuste.
}
import-scrivener-prose-range-comment-reading-pictures-not-counted-line = { $count ->
    [one] 1 commento di uno Scrivener più vecchio è stato collocato senza contare le sue immagini. Controlla che sia sulle parole giuste.
   *[other] { $count } commenti di uno Scrivener più vecchio sono stati collocati senza contare le loro immagini. Controlla che siano sulle parole giuste.
}
import-scrivener-prose-range-comment-approximate-line = { $count ->
    [one] 1 commento di uno Scrivener più vecchio non corrisponde esattamente a nessuna lettura della sua posizione, e potrebbe essere spostato di qualche carattere.
   *[other] { $count } commenti di uno Scrivener più vecchio non corrispondono esattamente a nessuna lettura della loro posizione, e potrebbero essere spostati di qualche carattere.
}
import-scrivener-prose-document-link-unresolved-line = { $count ->
    [one] 1 collegamento rimanda a un documento che l'importazione non ha trasferito. Le sue parole restano.
   *[other] { $count } collegamenti rimandano a documenti che l'importazione non ha trasferito. Le loro parole restano.
}
import-scrivener-prose-linked-picture-not-read-line = { $count ->
    [1] 1 immagine collegata a un file sul tuo disco, «{ $example }», non è stata trasferita.
   *[other] { $count } immagini collegate a file sul tuo disco, come «{ $example }», non sono state trasferite.
}
import-scrivener-prose-heading-repeats-title-line = { $count ->
    [one] 1 testo inizia con un'intestazione che ripete il suo titolo, quindi un'esportazione potrebbe stampare il titolo due volte.
   *[other] { $count } testi iniziano con un'intestazione che ripete il loro titolo, quindi un'esportazione potrebbe stampare il titolo due volte.
}
import-scrivener-prose-markdown-as-text-line = { $count ->
    [one] 1 paragrafo scritto in Markdown viene conservato così com'è, come testo.
   *[other] { $count } paragrafi scritti in Markdown vengono conservati così come sono, come testo.
}
import-scrivener-prose-citation-keys-as-text-line = { $count ->
    [one] 1 paragrafo contiene chiavi di citazione per un programma di bibliografia, conservate come testo.
   *[other] { $count } paragrafi contengono chiavi di citazione per un programma di bibliografia, conservate come testo.
}
import-scrivener-prose-placeholders-in-exported-text-line = { $count ->
    [1] 1 segnaposto di Scrivener, { $example }, viene conservato come testo semplice in ciò che esporti. Sostituiscilo a mano.
   *[other] { $count } segnaposto di Scrivener, come { $example }, vengono conservati come testo semplice in ciò che esporti. Sostituiscili a mano.
}
import-scrivener-prose-placeholders-in-other-text-line = { $count ->
    [1] 1 segnaposto di Scrivener, { $example }, viene conservato come testo semplice in un testo che non viene esportato.
   *[other] { $count } segnaposto di Scrivener, come { $example }, vengono conservati come testo semplice in testi che non vengono esportati.
}
import-scrivener-prose-ornament-lines-kept-line = { $count ->
    [one] 1 riga centrata di ornamenti potrebbe segnare un separatore di scena e viene conservata così com'è scritta.
   *[other] { $count } righe centrate di ornamenti potrebbero segnare separatori di scena e vengono conservate così come sono scritte.
}
import-scrivener-prose-long-document-line = { $count ->
    [one] 1 testo è molto lungo e potrebbe aprirsi lentamente.
   *[other] { $count } testi sono molto lunghi e potrebbero aprirsi lentamente.
}
import-scrivener-prose-pictures-without-description-line = { $count ->
    [one] 1 immagine non ha una descrizione per i lettori che non possono vederla. Puoi aggiungerne una nell'editor.
   *[other] { $count } immagini non hanno una descrizione per i lettori che non possono vederle. Puoi aggiungerne una nell'editor.
}
import-scrivener-prose-struck-text-compiled-away-line = { $count ->
    [one] Le parti barrate di 1 testo vengono conservate. La compilazione di Scrivener le escludeva dal libro, ma un'esportazione da Skribisto le stampa.
   *[other] Le parti barrate di { $count } testi vengono conservate. La compilazione di Scrivener le escludeva dal libro, ma un'esportazione da Skribisto le stampa.
}
import-scrivener-prose-compile-deleted-style-printed-line = { $count ->
    [1] Il testo nello stile «{ $example }», che la compilazione di Scrivener eliminava, viene conservato come prosa, e un'esportazione da Skribisto lo stampa.
   *[other] Il testo in { $count } stili che la compilazione di Scrivener eliminava, come «{ $example }», viene conservato come prosa, e un'esportazione da Skribisto lo stampa.
}
import-scrivener-prose-compile-raw-style-printed-line = { $count ->
    [1] Il testo nello stile «{ $example }», che la compilazione di Scrivener trasmetteva come codice per un altro programma, viene conservato come prosa, e un'esportazione da Skribisto lo stampa.
   *[other] Il testo in { $count } stili che la compilazione di Scrivener trasmetteva come codice per un altro programma, come «{ $example }», viene conservato come prosa, e un'esportazione da Skribisto lo stampa.
}
import-scrivener-prose-left-out-after-document-line = { $count ->
    [one] 1 frammento di un file danneggiato, dopo la sua fine, viene escluso.
   *[other] { $count } frammenti di file danneggiati, dopo la loro fine, vengono esclusi.
}
import-scrivener-prose-left-out-header-or-footer-line = { $count ->
    [one] Un'intestazione o un piè di pagina viene escluso: non fa parte del testo.
   *[other] { $count } intestazioni e piè di pagina vengono esclusi: non fanno parte del testo.
}
import-scrivener-prose-left-out-list-label-picture-line = { $count ->
    [one] 1 immagine usata come punto elenco viene esclusa.
   *[other] { $count } immagini usate come punti elenco vengono escluse.
}
import-scrivener-prose-left-out-line = { $count ->
    [one] 1 frammento di file che nessun testo contiene viene escluso.
   *[other] { $count } frammenti di file che nessun testo contiene vengono esclusi.
}
import-scrivener-prose-undecodable-characters-line = { $count ->
    [one] 1 carattere non è stato decodificato e appare come segno sostitutivo (�) dove si trovava.
   *[other] { $count } caratteri non sono stati decodificati e appaiono come segni sostitutivi (�) dove si trovavano.
}
import-scrivener-prose-inline-notes-merged-line = { $count ->
    [one] 1 nota in linea che Scrivener aveva diviso dove cambiava la formattazione viene riunita.
   *[other] { $count } note in linea che Scrivener aveva diviso dove cambiava la formattazione vengono riunite.
}
import-scrivener-prose-inline-notes-joined-across-paragraphs-line = { $count ->
    [1] 1 nota in linea scritta un paragrafo alla volta viene riunita in un'unica nota.
   *[other] { $count } note in linea scritte un paragrafo alla volta vengono riunite in note di più paragrafi.
}
import-scrivener-prose-blank-notes-dropped-line = { $count ->
    [one] 1 nota in linea vuota viene esclusa.
   *[other] { $count } note in linea vuote vengono escluse.
}
import-scrivener-prose-annotations-became-comments-line = { $count ->
    [one] 1 annotazione in linea è diventata un commento.
   *[other] { $count } annotazioni in linea sono diventate commenti.
}
import-scrivener-prose-annotation-only-document-kept-line = { $count ->
    [one] 1 documento conteneva soltanto un'annotazione. Le sue parole vengono conservate come prosa.
   *[other] { $count } documenti contenevano soltanto un'annotazione. Le loro parole vengono conservate come prosa.
}
import-scrivener-prose-preserve-formatting-passages-line = { $count ->
    [one] 1 passaggio contrassegnato per mantenere la sua formattazione arriva come testo normale.
   *[other] { $count } passaggi contrassegnati per mantenere la loro formattazione arrivano come testo normale.
}
import-scrivener-prose-nested-note-unwrapped-line = { $count ->
    [one] 1 nota all'interno di un commento o di una nota a piè di pagina viene conservata come testo, dove si trovava.
   *[other] { $count } note all'interno di commenti o di note a piè di pagina vengono conservate come testo, dove si trovavano.
}
import-scrivener-prose-style-not-carried-line = { $count ->
    [1] Lo stile «{ $example }» non ha un equivalente in Skribisto. Il suo testo conserva le parole.
   *[other] { $count } stili, come «{ $example }», non hanno un equivalente in Skribisto. Il loro testo conserva le parole.
}
import-scrivener-prose-document-links-dropped-line = { $count ->
    [one] 1 collegamento a un altro documento non trova posto in Skribisto. Le sue parole restano.
   *[other] { $count } collegamenti ad altri documenti non trovano posto in Skribisto. Le loro parole restano.
}
import-scrivener-prose-document-links-kept-line = { $count ->
    [one] 1 collegamento a un altro documento viene conservato come riferimento.
   *[other] { $count } collegamenti ad altri documenti vengono conservati come riferimenti.
}
import-scrivener-prose-other-links-kept-as-text-line = { $count ->
    [one] 1 collegamento a un altro progetto o a un file viene conservato come testo.
   *[other] { $count } collegamenti ad altri progetti o a file vengono conservati come testo.
}
import-scrivener-prose-soft-breaks-split-line = { $count ->
    [one] 1 interruzione di riga all'interno di un paragrafo ora inizia un paragrafo a sé.
   *[other] { $count } interruzioni di riga all'interno di paragrafi ora iniziano paragrafi a sé.
}
import-scrivener-prose-typed-indents-removed-line = { $count ->
    [one] 1 rientro digitato all'inizio di un paragrafo viene rimosso.
   *[other] { $count } rientri digitati all'inizio dei paragrafi vengono rimossi.
}
import-scrivener-prose-left-indents-not-kept-line = { $count ->
    [one] 1 paragrafo rientrato rispetto al margine perde il suo rientro.
   *[other] { $count } paragrafi rientrati rispetto al margine perdono il loro rientro.
}
import-scrivener-prose-blank-lines-removed-line = { $count ->
    [one] 1 paragrafo vuoto tra altri due viene rimosso.
   *[other] { $count } paragrafi vuoti tra altri paragrafi vengono rimossi.
}
import-scrivener-prose-verse-stanza-gaps-lost-line = { $count ->
    [one] 1 riga vuota tra le strofe di una poesia viene rimossa, quindi le strofe si susseguono senza stacco.
   *[other] { $count } righe vuote tra le strofe di una poesia vengono rimosse, quindi le strofe si susseguono senza stacco.
}
import-scrivener-prose-blank-lines-became-scene-breaks-line = { $count ->
    [one] 1 riga vuota tra paragrafi è diventata un separatore di scena.
   *[other] { $count } righe vuote tra paragrafi sono diventate separatori di scena.
}
import-scrivener-prose-ornaments-became-scene-breaks-line = { $count ->
    [one] 1 riga di ornamenti è diventata un separatore di scena.
   *[other] { $count } righe di ornamenti sono diventate separatori di scena.
}
import-scrivener-prose-table-first-row-header-line = { $count ->
    [one] 1 tabella ora inizia con una riga di intestazione.
   *[other] { $count } tabelle ora iniziano con una riga di intestazione.
}
import-scrivener-prose-table-cells-joined-line = { $count ->
    [one] 1 cella di tabella conteneva più paragrafi, ora riuniti in uno solo.
   *[other] { $count } celle di tabella contenevano più paragrafi, ora riuniti in uno solo.
}
import-scrivener-prose-formatting-lost-colour-line = { $count ->
    [one] Il colore del testo viene rimosso in 1 testo.
   *[other] Il colore del testo viene rimosso in { $count } testi.
}
import-scrivener-prose-formatting-lost-highlight-line = { $count ->
    [one] L'evidenziazione viene rimossa in 1 testo.
   *[other] L'evidenziazione viene rimossa in { $count } testi.
}
import-scrivener-prose-formatting-lost-small-caps-line = { $count ->
    [one] Il maiuscoletto appare come lettere normali in 1 testo.
   *[other] Il maiuscoletto appare come lettere normali in { $count } testi.
}
import-scrivener-prose-formatting-lost-all-caps-line = { $count ->
    [one] Il testo mostrato in maiuscolo appare come era stato digitato in 1 testo.
   *[other] Il testo mostrato in maiuscolo appare come era stato digitato in { $count } testi.
}
import-scrivener-prose-formatting-lost-hidden-line = { $count ->
    [one] Il testo nascosto viene escluso in 1 testo.
   *[other] Il testo nascosto viene escluso in { $count } testi.
}
import-scrivener-prose-formatting-lost-line = { $count ->
    [one] La formattazione per cui Skribisto non ha posto viene rimossa in 1 testo.
   *[other] La formattazione per cui Skribisto non ha posto viene rimossa in { $count } testi.
}
import-scrivener-prose-justified-text-not-kept-line = { $count ->
    [one] 1 paragrafo giustificato segue invece lo stile dell'esportazione.
   *[other] { $count } paragrafi giustificati seguono invece lo stile dell'esportazione.
}
import-scrivener-prose-comment-colours-lost-line = { $count ->
    [one] 1 commento perde il suo colore.
   *[other] { $count } commenti perdono il loro colore.
}
import-scrivener-prose-control-characters-removed-line = { $count ->
    [one] 1 carattere di controllo invisibile viene rimosso.
   *[other] { $count } caratteri di controllo invisibili vengono rimossi.
}
import-scrivener-prose-note-on-heading-lost-line = { $count ->
    [one] 1 nota a piè di pagina o commento sull'intestazione di un capitolo non poteva restare lì: un commento si sposta sul primo paragrafo del capitolo, una nota a piè di pagina viene esclusa.
   *[other] { $count } note a piè di pagina o commenti sulle intestazioni dei capitoli non potevano restare lì: un commento si sposta sul primo paragrafo del capitolo, una nota a piè di pagina viene esclusa.
}
import-scrivener-prose-media-file-copied-line = { $count ->
    [one] 1 file di ricerca che Skribisto non sa aprire è stato copiato accanto al nuovo progetto. La sua nota indica dove.
   *[other] { $count } file di ricerca che Skribisto non sa aprire sono stati copiati accanto al nuovo progetto. Le loro note indicano dove.
}
import-scrivener-prose-media-file-not-carried-line = { $count ->
    [one] 1 file di ricerca che Skribisto non sa aprire resta nel tuo progetto Scrivener. La sua nota ne riporta il nome.
   *[other] { $count } file di ricerca che Skribisto non sa aprire restano nel tuo progetto Scrivener. Le loro note ne riportano i nomi.
}

## When the import fails
## Shown in the notice of a failed import, above the button import-scrivener-review-again,
## which these sentences quote.
import-scrivener-error-refused = { $refusal } Nulla è stato scritto.
import-scrivener-error-question = Il progetto ora pone una domanda a cui la verifica non ha risposto. Scegli «Rivedi l'importazione», poi importa. Nulla è stato scritto.
import-scrivener-error-not-accepted = Prima di poter essere importato, il progetto richiede che tu spunti una casella al passaggio Progetto, e non è stata spuntata. Scegli «Rivedi l'importazione», spuntala, poi importa. Nulla è stato scritto.
import-scrivener-error-changed = Il progetto è cambiato dopo la tua verifica. Scegli «Rivedi l'importazione», poi importa. Nulla è stato scritto.
import-scrivener-error-choice = La struttura scelta nella verifica non è più adatta a questo progetto. Scegli «Rivedi l'importazione», poi importa. Nulla è stato scritto.

export-title = Esporta
export-close = Chiudi
export-scope-label = Contenuto
export-format-label = Formato
export-style-label = Stile
export-path-label = File
export-browse = Sfoglia…
export-show-non-exportable = Mostra gli elementi esclusi dall'esportazione
export-choose-empty = Nessun progetto da cui scegliere.
export-cancel = Annulla
export-export = Esporta
export-save-dialog-title = Esporta in un file
export-section-what = Cosa esportare
export-section-style = Stile
export-section-destination = Destinazione
export-custom-selection = Selezione personalizzata
export-selected-count = { $count ->
    [one] { $count } selezionato
   *[other] { $count } selezionati
}
export-chip-chapters-none = Capitoli: nessuna intestazione
export-chip-chapters-numbered = Capitoli: numerati
export-chip-chapters-title = Capitoli: solo titolo
export-chip-chapters-both = Capitoli: numero + titolo
export-chip-scene-break-glyph = Separatore di scena: { $glyph }
export-chip-scene-break-blank = Separatore di scena: riga vuota
export-chip-scene-break-none = Separatore di scena: nessuno
export-chip-major-break-glyph = Separatore maggiore: { $glyph }
export-chip-major-break-blank = Separatore maggiore: riga vuota
export-chip-major-break-none = Separatore maggiore: nessuno
export-chip-major-break-same = Entrambi i livelli di separazione uguali
export-chip-spacing-single = Interlinea: singola
export-chip-spacing-onehalf = Interlinea: 1½
export-chip-spacing-double = Interlinea: doppia
export-chip-notes-included = Note incluse
export-chip-notes-excluded = Note escluse
export-format-docx = Word
export-format-odt = LibreOffice
export-format-html = HTML
export-format-markdown = Markdown
export-format-djot = Djot
export-format-text = Testo
export-format-latex = LaTeX
export-format-epub = EPUB
export-format-pdf = PDF
export-overwrite-title = Sostituire il file esistente?
export-overwrite-text = «{ $name }» esiste già. Sostituirlo?
export-progress-title = Esportazione…
export-cancelled = Esportazione annullata
export-done = { $count ->
    [one] { $count } elemento esportato
   *[other] { $count } elementi esportati
}
export-open-file = Apri
export-show-in-folder = Mostra nella cartella
export-error-title = Impossibile esportare
export-error-details = Dettagli
backup-progress-start = Avvio…
backup-progress-retention = Pulizia dei vecchi backup…
backup-progress-done = Completato
backup-progress-destination = Destinazione { $i } di { $n }
backup-details = Dettagli
backup-issues-title = Problemi con il backup
backup-failed-title = Backup non riuscito
backup-complete-prune-warning = Backup completato ({ $ok ->
    [one] { $ok } salvato
   *[other] { $ok } salvati
}, { $skipped ->
    [one] { $skipped } già aggiornato
   *[other] { $skipped } già aggiornati
}). Alcuni vecchi backup non sono stati rimossi
search = Cerca
search-query-placeholder = Cerca…
search-replace-placeholder = Sostituisci con…
search-replace-toggle = Mostra o nascondi la sostituzione
search-replace-all = Sostituisci tutto
search-opt-case = Distingui maiuscole e minuscole
search-opt-whole-word = Parola intera
search-opt-diacritics = Distingui accenti
search-scope-body = Testo
search-scope-title = Titolo
search-scope-synopsis = Sinossi
search-scope-label = Dicitura
search-facet-book = Libri
search-facet-part = Parti
search-facet-chapter = Capitoli
search-facet-scene = Scene
search-facet-note = Note
search-facet-folder = Cartelle
search-tip-case = Distingui maiuscole e minuscole: tratta «Elena» ed «elena» come corrispondenze diverse.
search-tip-whole-word = Parola intera: trova soltanto parole complete, così «cane» non viene trovato in «canestro».
search-tip-diacritics = Distingui accenti: considera diverse le lettere accentate, così «cafe» non corrisponde a «café».
search-tip-body = Testo: cerca nel testo delle scene e delle note.
search-tip-title = Titolo: cerca nei titoli degli elementi del raccoglitore.
search-tip-synopsis = Sinossi: cerca nel riassunto di ogni elemento di scrittura.
search-tip-label = Dicitura: cerca nella breve annotazione sotto il titolo di un elemento.
search-tip-comment = Commenti: cerca nelle discussioni e nelle risposte. Un commento riguarda il manoscritto ma non ne fa parte, quindi ha un interruttore proprio; nella sostituzione i commenti restano deselezionati finché non li selezioni.
search-tip-book = Libri: il contenitore del libro e i suoi marcatori di inizio e fine.
search-tip-part = Parti: le divisioni a livello di parte.
search-tip-chapter = Capitoli: in qualsiasi forma siano memorizzati nel progetto.
search-tip-scene = Scene: gli elementi che contengono il tuo testo narrativo.
search-tip-note = Note: annotazioni libere.
search-tip-folder = Cartelle: semplici contenitori organizzativi e separatori.
search-tip-paratext = Un testo che appartiene al libro ma non alla storia: prefazione, dedica, postfazione. Non viene mai contato nel manoscritto.
search-tip-preserve-case = Mantieni maiuscole e minuscole: la sostituzione riprende la forma trovata, così «ELENA» diventa «MARTA» ed «Elena» diventa «Marta».
search-tip-replace = Sostituisci: mostra il campo di sostituzione e Sostituisci tutto.
search-error = Ricerca non riuscita: { $message }
search-no-matches = Nessuna corrispondenza
search-count =
    { $matches ->
        [one] { $matches } corrispondenza
       *[other] { $matches } corrispondenze
    } in { $items ->
        [one] { $items } documento
       *[other] { $items } documenti
    }
search-count-truncated =
    { $matches ->
        [one] { $matches } corrispondenza
       *[other] { $matches } corrispondenze
    } in { $items ->
        [one] { $items } documento
       *[other] { $items } documenti
    } (solo i primi risultati)
search-occurrences = { $count }
search-field-body = Testo
search-field-title = Titolo
search-field-synopsis = Sinossi
search-field-label = Dicitura
search-field-epigraph = Epigrafe
search-field-comment = Commento
search-field-comment-reply = Risposta
search-field-footnote = Nota a piè di pagina
search-include-in-replace = Includi in Sostituisci tutto
search-collapse-all = Comprimi tutti i risultati
search-replace-here = Sostituisci questa corrispondenza
search-dismiss = Rimuovi dai risultati
search-undo-dismiss = Ripristina l'ultimo risultato rimosso
search-replace-nothing = (nulla)
search-replace-confirm-title = Sostituire tutte le corrispondenze?
search-replace-confirm-text =
    Sostituire { $occurrences ->
        [one] { $occurrences } occorrenza
       *[other] { $occurrences } occorrenze
    } di «{ $query }» con «{ $replacement }» in { $items ->
        [one] { $items } documento
       *[other] { $items } documenti
    }? Potrai annullare l'operazione dalla notifica successiva.
search-replace-done-title = Sostituzione completata
search-replace-done =
    { $occurrences ->
        [one] { $occurrences } occorrenza sostituita
       *[other] { $occurrences } occorrenze sostituite
    } in { $items ->
        [one] { $items } documento
       *[other] { $items } documenti
    }.
search-replace-done-skipped =
    { $occurrences ->
        [one] { $occurrences } occorrenza sostituita
       *[other] { $occurrences } occorrenze sostituite
    } in { $items ->
        [one] { $items } documento
       *[other] { $items } documenti
    }. { $skipped ->
        [one] { $skipped } campo saltato
       *[other] { $skipped } campi saltati
    } perché modificato dopo la ricerca.
search-replace-undo = Annulla
search-replace-failed-title = Sostituzione non riuscita
search-replace-skipped-title = Nulla sostituito
search-replace-skipped-body =
    { $fields ->
        [one] Questo testo è cambiato
       *[other] { $fields } di questi testi sono cambiati
    } dopo la ricerca, quindi { $fields ->
        [one] è rimasto invariato
       *[other] sono rimasti invariati
    }. Cerca di nuovo per vedere dove sono ora le parole.
search-replace-undo-failed-title = Impossibile annullare
search-preview = Anteprima
search-preview-empty = Seleziona un risultato per vederne l'anteprima qui
search-preview-no-prose = Questa corrispondenza non ha testo modificabile
search-preview-prompt = Esegui una ricerca per vedere un'anteprima qui.
search-preview-open-search = Cerca nel progetto
search-preview-footnote-prompt = Questa corrispondenza è nel testo di una nota a piè di pagina. Aprila nel relativo pannello per leggerla e modificarla.
search-preview-open-footnotes = Apri note a piè di pagina
find-placeholder = Cerca in questo documento…
find-count = { $current } di { $total }
find-no-results = Nessun risultato
find-close = Chiudi ricerca
find-previous = Corrispondenza precedente (Maiusc+Invio)
find-next = Corrispondenza successiva (Invio)
find-opt-case = Distingui maiuscole e minuscole
find-opt-whole-word = Parola intera
find-replace-toggle = Mostra o nascondi la sostituzione (Ctrl+R)
find-replace-placeholder = Sostituisci con…
find-replace = Sostituisci
find-replace-all = Sostituisci tutto
find-preserve-case = Mantieni maiuscole e minuscole
dict-download-title = Download di { $name }…
dict-download-done = { $name } installato
dict-download-failed = Impossibile scaricare { $name }: { $error }
dict-removed = { $name } rimosso
dict-accept-first = Accetta la licenza prima di scaricare { $name }
settings-dict-tab-installed = Installati
settings-dict-tab-get-more = Scarica altri
settings-dict-tab-personal = Parole personali
dict-installed-empty = Nessun dizionario trovato su questo computer.
dict-get-more-search = Cerca lingue
dict-system-badge = sul tuo sistema
dict-unusable-badge = inutilizzabile
dict-download-button = Scarica
dict-downloading = Download…
dict-installed-label = Installato
dict-remove = Rimuovi
dict-view-license = Visualizza licenza
dict-approx-size = ~{ $size }
dict-personal-empty = Nessuna parola personale in questo progetto.
dict-personal-add = Aggiungi
dict-personal-placeholder = Aggiungi una parola…
dict-missing-toast = { $count ->
    [one] Questo progetto usa { $count } dizionario non installato
   *[other] Questo progetto usa { $count } dizionari non installati
}
dict-missing-action = Scarica dizionari
dict-license-title = Licenza di { $name }
dict-license-accept = Accetta e scarica
dict-license-cancel = Annulla
dict-license-close = Chiudi
dict-add-button = Aggiungi dizionario…
dict-add-title = Aggiungi un dizionario
dict-add-close = Chiudi
dict-add-name = Nome
dict-add-name-placeholder = Ad esempio: Il mio dizionario latino
dict-add-code = Codice della lingua
dict-add-code-placeholder = Ad esempio: la oppure it-IT-x-custom
dict-add-code-hint = Un codice breve a tua scelta, che selezionerai come lingua del documento.
dict-add-aff = File degli affissi (.aff)
dict-add-dic = Elenco di parole (.dic)
dict-add-submit = Aggiungi
dict-add-cancel = Annulla
dict-add-name-required = Dai un nome al dizionario.
dict-add-code-required = Inserisci un codice della lingua.
dict-add-code-invalid = Usa soltanto lettere, cifre e - _ .
dict-add-code-reserved = È il codice di un dizionario integrato. Scegline un altro.
dict-add-file-required = Scegli un file.
dict-add-file-missing = Questo file non esiste.
dict-add-code-taken = È già installato un dizionario con questo codice.
dict-add-done = { $name } aggiunto
dict-add-unusable = Questi file non costituiscono un dizionario utilizzabile: { $error }
dict-add-failed = Impossibile aggiungere il dizionario: { $error }
inspector-export = Esportazione
inspector-exportable = Includi nelle esportazioni
ctx-number = &Numera questo capitolo
ctx-unnumber = &Non numerare questo capitolo
inspector-numbering = Numerazione
inspector-numbered = Numerato
inspector-numbered-tip = Questo capitolo occupa il suo posto nella numerazione del libro. Disattiva per un prologo, un epilogo o un interludio.
inspector-numbered-tip-more = Un capitolo non numerato resta nel libro esattamente com'era: intestazione, testo e conteggio delle parole restano invariati. Semplicemente non stampa né occupa un numero: il capitolo dopo un prologo è il primo, non il secondo. Escluderlo dall'esportazione è un'opzione diversa, sopra, che rimuove invece l'intero capitolo dal libro.
inspector-apply-to-children = Applica agli elementi contenuti
inspector-milestone = Data del traguardo
inspector-milestone-none = Nessuna data
inspector-milestone-clear = Cancella
inspector-tags = Etichette
inspector-aliases = Altri nomi
inspector-dict-language = Lingua
inspector-apply-language-to-children = Applica la lingua agli elementi contenuti
settings-page-language = Lingua
settings-field-dict-language = Lingue
dict-tradeoff-hint = Ogni lingua aggiuntiva accetta più parole, quindi rileva meno errori.
lang-inherit-hint = Ereditata: questa scena usa le lingue del libro o del progetto.
lang-pill-list = Lingue
lang-pill-add = Aggiungi una lingua
lang-pill-remove = Rimuovi { $name }
lang-pill-mute = Disattiva il controllo ortografico per { $name }
lang-pill-unmute = Attiva il controllo ortografico per { $name }
settings-page-personal-dictionary = Dizionario personale
settings-user-dict-desc = Le parole aggiunte qui sono considerate corrette in tutta l'applicazione e non verranno più segnalate.
settings-user-dict-add-placeholder = Aggiungi una parola…
settings-user-dict-add = Aggiungi parola
settings-user-dict-search = Filtra parole
settings-user-dict-count = { $count ->
    [one] { $count } parola
   *[other] { $count } parole
}
settings-user-dict-rename = Rinomina
settings-user-dict-remove = Rimuovi
settings-user-dict-empty = Nessuna parola. Aggiungi quelle che il correttore deve ignorare o importa un elenco.
settings-user-dict-import = Importa…
settings-user-dict-export = Esporta…
settings-user-dict-import-tip = Aggiungi parole da un file di testo semplice, una per riga. Le parole esistenti vengono conservate.
settings-user-dict-export-tip = Salva l'intero elenco in un file di testo semplice, una parola per riga.
settings-user-dict-txt-filter = File di testo
settings-user-dict-duplicate = Già nel dizionario
settings-user-dict-imported = { $count ->
    [one] { $count } parola importata
   *[other] { $count } parole importate
} ({ $duplicates } già presenti).
settings-user-dict-import-failed = Impossibile leggere l'elenco di parole: { $error }
settings-user-dict-exported = { $count ->
    [one] { $count } parola salvata.
   *[other] { $count } parole salvate.
}
settings-user-dict-export-failed = Impossibile salvare l'elenco di parole: { $error }
settings-page-text-replacements = Sostituzioni di testo
settings-text-repl-desc = Sostituisci le abbreviazioni con il testo completo mentre scrivi: «cmq» diventa «comunque» appena digiti uno spazio o un segno di punteggiatura.
settings-text-repl-enable = Usa sostituzioni di testo in questo progetto
settings-text-repl-disabled-hint = Attiva per definire abbreviazioni che si espandono mentre scrivi.
settings-text-repl-add = Aggiungi regola
settings-text-repl-trigger-placeholder = Abbreviazione
settings-text-repl-replacement-placeholder = Testo sostitutivo
settings-text-repl-added = «{ $trigger }» aggiunta
settings-text-repl-duplicate = «{ $trigger }» ha già una regola
settings-text-repl-filter = Filtra regole
settings-text-repl-count = { $n ->
    [one] 1 regola
   *[other] { $n } regole
}
settings-text-repl-row-enabled = Usa questa regola
settings-text-repl-delete = Elimina la regola per { $trigger }
settings-text-repl-deleted = Regola per «{ $trigger }» eliminata
settings-text-repl-empty = Nessuna regola.
settings-text-repl-csv-filter = File CSV
settings-text-repl-import = Importa…
settings-text-repl-export = Esporta…
settings-text-repl-imported = { $added } importate, { $skipped } saltate
settings-text-repl-exported = { $n ->
    [one] 1 regola esportata
   *[other] { $n } regole esportate
}
editor-menu-no-suggestions = Nessun suggerimento
editor-menu-add-to-dictionary = Aggiungi «{ $word }» al dizionario
editor-menu-add-words-to-dictionary = Aggiungi le parole selezionate al dizionario
editor-dict-added = «{ $word }» aggiunta al dizionario.
editor-dict-added-multi = { $count ->
    [one] { $count } parola aggiunta al dizionario.
   *[other] { $count } parole aggiunte al dizionario.
}
toast-undo = Annulla
undo-superseded-title = Quel passaggio non è più l'ultimo
undo-superseded-body = Nel frattempo è cambiato qualcos'altro nel progetto. Annulla avrebbe rimosso quella modifica, quindi non è stato eseguito. Nulla di questo passaggio è stato annullato. Usa Modifica ▸ Annulla per ripercorrere la cronologia.
undo-failed = Impossibile annullare: { $error }
menu-edit = &Modifica
menu-edit-undo = &Annulla
menu-edit-redo = &Ripeti
menu-edit-undo-target = &Annulla: { $target }
menu-edit-redo-target = &Ripeti: { $target }
shortcut-name-edit-undo = Annulla
shortcut-name-edit-redo = Ripeti
undo-target-typing = digitazione
undo-target-project = ultima modifica al progetto
prose-history-reset-title = Cronologia di digitazione azzerata
prose-history-reset-body = { $count ->
    [one] Una scena aperta è stata ripristinata a uno stato precedente; la sua cronologia di digitazione non è più valida.
   *[other] { $count } scene aperte sono state ripristinate a uno stato precedente; le loro cronologie di digitazione non sono più valide.
}
undo-target-trash = spostamento nel cestino
undo-target-restore = ripristino dal cestino
undo-target-delete-forever = eliminazione definitiva
undo-target-replace-all = sostituzione in tutto il progetto
undo-target-import = importazione dei documenti
undo-target-duplicate = duplicazione
undo-target-move = spostamento
undo-target-merge = unione di due scene
undo-target-split = divisione di una scena
undo-target-promote = cambio del tipo
undo-target-tidy-titles = riordino dei titoli dei capitoli
undo-target-import-tags = importazione delle etichette
undo-target-import-templates = importazione dei modelli di nota
undo-target-create = creazione
undo-target-remove = eliminazione
undo-target-rename = ridenominazione
undo-target-edit = quella modifica
menu-edit-find = C&erca…
menu-edit-find-next = Cerca &successivo
menu-edit-find-prev = Cerca prece&dente
menu-edit-replace = Cerca e s&ostituisci…
menu-edit-find-in-project = Cerca nel &progetto…
menu-edit-replace-in-project = Sostituisci nel pro&getto…
undo-frozen = Annulla («Sempre avanti» è attivo)
redo-frozen = Ripeti («Sempre avanti» è attivo)
menu-edit-cut = &Taglia
menu-edit-copy = &Copia
menu-edit-paste = &Incolla
menu-edit-paste-plain = Incolla senza &formattazione
menu-edit-select-all = Seleziona t&utto
shortcut-name-edit-cut = Taglia
shortcut-name-edit-copy = Copia
shortcut-name-edit-paste = Incolla
shortcut-name-edit-paste-plain = Incolla senza formattazione
shortcut-name-edit-select-all = Seleziona tutto
titlebar-spellcheck-on = Controllo ortografico attivo. Fai clic per disattivarlo (F7)
titlebar-spellcheck-off = Controllo ortografico disattivato. Fai clic per riattivarlo (F7)
titlebar-distraction-free = Entra nella modalità senza distrazioni (F11)
menu-spellcheck = C&ontrolla ortografia
menu-comments = Co&mmenti
settings-page-spellcheck = Controllo ortografico
settings-group-spellcheck = Controllo ortografico
settings-spellcheck-enabled = Controlla l'ortografia mentre scrivo
settings-spellcheck-hint = Sottolinea le parole sconosciute a tutti i dizionari installati. Disattivando l'opzione si interrompe ogni controllo in tutti i progetti, fino alla riattivazione. Per escludere una sola lingua, togli la sua spunta nel campo Lingua del progetto o dell'elemento.
menu-trash = C&estino
trash-title = Cestino
trash-empty-state = Il cestino è vuoto.
trash-empty-button = Svuota cestino…
trash-restore = &Ripristina
trash-restore-to = Ripristina &in…
trash-delete-forever = &Elimina definitivamente
trash-restored-ok = { $count ->
    [one] { $count } elemento ripristinato.
   *[other] { $count } elementi ripristinati.
}
trash-restore-error = Impossibile ripristinare: { $error }
trash-restore-orphaned = La posizione originale di questo elemento non esiste più. Scegli dove ripristinarlo.
trash-restore-no-project = Nessun progetto aperto, quindi nulla da ripristinare.
trash-delete-no-project = Nessun progetto aperto, quindi nulla da eliminare.
trash-empty-confirm-title = Svuotare il cestino?
trash-empty-confirm-text = { $count ->
    [one] { $count } elemento nel cestino verrà eliminato definitivamente.
   *[other] { $count } elementi nel cestino verranno eliminati definitivamente.
} Non resteranno nel cestino. Annulla può ancora recuperarli finché non esegui un'altra azione.
trash-emptied-title = Cestino svuotato
trash-emptied-body = Tutto il contenuto del cestino è stato eliminato definitivamente.
trash-empty-no-project = Nessun progetto aperto, quindi nessun cestino da svuotare.
trash-delete-forever-confirm-title = Eliminare definitivamente?
trash-delete-forever-confirm-text = { $count ->
    [one] { $count } elemento verrà eliminato definitivamente.
   *[other] { $count } elementi verranno eliminati definitivamente.
} Non resteranno nel cestino. Annulla può ancora recuperarli finché non esegui un'altra azione.
trash-deleted-title = Eliminati definitivamente
trash-deleted-body = { $count ->
    [one] { $count } elemento eliminato definitivamente.
   *[other] { $count } elementi eliminati definitivamente.
}
trash-undo = Annulla
trash-restore-picker-title = Ripristina in…
trash-restore-to-confirm-title = Ripristinare qui?
trash-restore-to-confirm-text = Ripristinare «{ $item }» in «{ $destination }»?
trash-restore-picker-restore-here = Ripristina qui
trash-restore-picker-cancel = Annulla
trash-restore-picker-empty = Nessun raccoglitore. Creane prima uno.
trash-banner-title = Questo elemento è nel cestino
trash-banner-description = Non apparirà nella struttura o nelle esportazioni finché non lo ripristini.
trash-banner-restore = Ripristina…
trash-tab-tooltip = Nel cestino
settings-styles-sheet-yes = Sì
settings-styles-sheet-no = No
settings-styles-sheet-font = Carattere
settings-styles-sheet-indent = Rientro della prima riga
settings-styles-sheet-para-spacing = Spaziatura dei paragrafi
settings-styles-sheet-page = Formato della pagina
settings-styles-sheet-margins = Margini (alto/destra/basso/sinistra)
settings-styles-sheet-title-page = Frontespizio del libro
settings-styles-sheet-heading-language = Lingua delle intestazioni
settings-styles-sheet-auto = Segue il testo
settings-styles-sheet-digits = Cifre
settings-styles-sheet-direction = Direzione del testo
settings-styles-sheet-formats = Formati
settings-styles-sheet-all-formats = Tutti
settings-styles-editor-group = Editor dello stile
settings-styles-editor-missing = Quello stile non è più disponibile.
settings-styles-page-letter = Letter
settings-styles-digits-western = Occidentali (0–9)
settings-styles-digits-eastern-arabic = Arabo-orientali (٠–٩)
settings-styles-direction-ltr = Da sinistra a destra
settings-styles-direction-rtl = Da destra a sinistra
format-dock-title = Formato
format-panel-empty = Posiziona il cursore in una scena, una nota o una sinossi per vedere le opzioni di formattazione.
format-group-marks = Testo
format-group-block = Paragrafo
format-group-lists = Elenchi
format-group-tables = Tabella
format-group-breaks = Separatori di scena
format-superscript = Apice
format-subscript = Pedice
format-link = Collegamento…
link-dialog-insert-title = Inserisci collegamento
link-dialog-edit-title = Modifica collegamento
link-dialog-text-label = Testo
link-dialog-text-placeholder = le parole che vede il lettore
link-dialog-url-label = Indirizzo
link-dialog-url-placeholder = esempio.it
link-dialog-insert = Inserisci
link-dialog-apply = Applica
link-dialog-cancel = Annulla
link-dialog-remove = Rimuovi collegamento
link-scheme-refused = Si possono aprire soltanto collegamenti web e indirizzi email. { $url } non è stato aperto.
menu-format-link = Co&llegamento…
format-clear = Rimuovi formattazione
format-heading = Livello dell'intestazione
format-heading-normal = Testo normale
format-heading-1 = Intestazione 1
format-heading-2 = Intestazione 2
format-heading-3 = Intestazione 3
format-heading-4 = Intestazione 4
format-heading-5 = Intestazione 5
format-heading-6 = Intestazione 6
format-align-left = Allinea a sinistra
format-align-center = Centra
format-direction-rtl = Paragrafo da destra a sinistra
format-blockquote = Citazione in blocco
format-list-bullet = Elenco puntato
format-list-numbered = Elenco numerato
format-indent = Aumenta rientro
format-outdent = Riduci rientro
format-table-insert = Inserisci tabella
format-table-row-above = Inserisci riga sopra
format-table-row-below = Inserisci riga sotto
format-table-col-before = Inserisci colonna prima
format-table-col-after = Inserisci colonna dopo
format-table-row-delete = Elimina riga
format-table-col-delete = Elimina colonna
format-table-remove = Rimuovi tabella
menu-format-marks-bold = &Grassetto
menu-format-marks-italic = &Corsivo
menu-format-marks-underline = S&ottolineato
menu-format-marks-strike = &Barrato
menu-format-marks-superscript = &Apice
menu-format-marks-subscript = &Pedice
menu-format-marks-clear = Rimuovi &formattazione
menu-format-heading = &Intestazione
menu-format-heading-normal = Testo &normale
menu-format-heading-1 = Intestazione &1
menu-format-heading-2 = Intestazione &2
menu-format-heading-3 = Intestazione &3
menu-format-heading-4 = Intestazione &4
menu-format-heading-5 = Intestazione &5
menu-format-heading-6 = Intestazione &6
menu-format-alignment = Allin&eamento
menu-format-align-left = Allinea a &sinistra
menu-format-align-center = &Centra
menu-format-direction = Di&rezione
menu-format-direction-auto = &Automatica
menu-format-direction-ltr = Da &sinistra a destra
menu-format-direction-rtl = Da &destra a sinistra
menu-format-blockquote = Cita&zione in blocco
menu-format-lists = Ele&nchi
menu-format-list-bullet = Elenco &puntato
menu-format-list-numbered = Ele&nco numerato
menu-format-indent = &Aumenta rientro
menu-format-outdent = &Riduci rientro
menu-format-table = &Tabella
menu-format-table-insert = &Inserisci tabella
menu-format-table-2x2 = &2 x 2
menu-format-table-3x3 = &3 x 3
menu-format-table-4x4 = &4 x 4
menu-format-table-row-above = In&serisci riga sopra
menu-format-table-row-below = Inserisci riga s&otto
menu-format-table-col-before = Inserisci colonna &prima
menu-format-table-col-after = Inserisci colonna &dopo
menu-format-table-row-delete = &Elimina riga
menu-format-table-col-delete = Elimina &colonna
menu-format-table-remove = &Rimuovi tabella
menu-format-typo-marks = Segno &di formattazione
menu-format-typo-nbsp = &Spazio non separabile
menu-format-typo-nbhyphen = &Trattino non separabile
menu-format-typo-soft-hyphen = Trattino &facoltativo
menu-format-typo-nnbsp = Spazio non separabile st&retto
menu-format-typo-zwsp = Spazio a larghezza &zero
menu-format-typo-word-joiner = &Unione di parole
menu-about = &Informazioni su Skribisto…
about-title = Informazioni su Skribisto
about-version = Versione { $version }
about-update-available = È disponibile la versione { $version }
about-update-see-notes = Scopri le novità
about-update-flathub = Flathub mantiene aggiornata questa copia.
about-update-distro = La tua distribuzione mantiene aggiornata questa copia.
about-tagline = Un'app per scrivere romanzi e narrativa lunga, realizzata in Rust con il toolkit Teksilo.
about-license = Distribuito con licenza GNU General Public License, versione 3.
about-copyright = © 2026 Cyril Jacquet
about-close = Chiudi
native-menu-about = Informazioni su { $app }
native-menu-hide = Nascondi { $app }
native-menu-quit = Esci da { $app }
native-menu-settings = Impostazioni…
native-menu-window = Finestra
native-menu-minimize = Riduci a icona
native-menu-zoom = Ingrandisci
window-title = { $title } - { $app }
window-title-numbered = { $title } - { $app } (Finestra { $n })
window-title-empty = { $app }
first-run-window-title = Benvenuto
first-run-title = Configura { $app }
first-run-body = { $app } conserva le impostazioni separatamente da Skribisto, quindi parte da zero. Puoi copiare ora preferenze, progetti recenti, dizionari e disposizione delle finestre.
first-run-from = Copia da
first-run-to = Copia in
first-run-copy-note = Nulla viene spostato o rimosso. Skribisto conserva tutte le proprie impostazioni e continua a funzionare come prima.
first-run-import = Importa impostazioni
first-run-start-fresh = Inizia da zero
first-run-import-failed = Alcune impostazioni non sono state importate: { $error }
settings-page-punctuation = Punteggiatura predefinita
settings-group-punctuation = Punteggiatura intelligente
settings-page-work-punctuation = Punteggiatura
settings-punctuation-override = Usa regole di punteggiatura proprie per questo progetto
settings-punctuation-override-hint = Disattivo: il progetto segue le preferenze dell'applicazione. Queste regole viaggiano nel file .skrib, così un coautore scrive con la stessa tipografia.
settings-punctuation-dashes = Trasforma -- in una lineetta media e --- in una lineetta lunga
settings-punctuation-ellipsis = Trasforma ... in puntini di sospensione
settings-punctuation-quotes = Usa virgolette e apostrofi tipografici
settings-quote-style = Virgolette
settings-quote-style-locale = Predefinite per la lingua
settings-quote-style-curly = “Doppie”
settings-quote-style-curly-single = ‘Singole’
settings-quote-style-guillemets = «Caporali»
settings-quote-style-low-high = „Basse-alte“
settings-punctuation-spacing = Spazio prima di ; : ! ?
settings-punctuation-spacing-hint = La tipografia francese inserisce uno spazio non separabile stretto prima di ; ! ? e uno intero prima di :. Vale solo per il testo in francese. Gli spazi dentro le caporali « » accompagnano le virgolette stesse.
settings-punctuation-sample = La tua lingua produce
settings-punctuation-app-hint = Regole seguite da ogni progetto, salvo impostarne di proprie in Progetto ▸ Punteggiatura.
settings-punctuation-dialogue = Inizia con una lineetta di dialogo i paragrafi digitati come «- »
settings-punctuation-dialogue-hint = Per lingue che segnano il dialogo con una lineetta anziché con virgolette, come francese, spagnolo e russo. Funziona solo all'inizio del paragrafo.
settings-punctuation-auto-lists = Inizia un elenco da un paragrafo digitato come "- ", "* " o "1. "
settings-punctuation-auto-lists-hint = Il paragrafo diventa un elenco puntato o numerato. Dove la lineetta di dialogo è attiva e la lingua ne ha una, "- " apre invece una battuta di dialogo. Premi Backspace subito dopo per riavere ciò che hai digitato.
statusbar-go-to = Vai a…
go-to-placeholder = Cerca nel raccoglitore
go-to-no-matches = Nessun elemento corrisponde alla ricerca.
menu-go-to = &Vai a…
settings-page-distraction-free-themes = Temi senza distrazioni
settings-themes-builtin = Temi integrati
settings-themes-builtin-badge = Integrato
settings-themes-user = I miei temi
settings-themes-editor-group = Modifica tema
settings-themes-editor-empty = Scegli un tema in «I miei temi» per modificarlo.
settings-themes-use = Usa
settings-themes-duplicate = Duplica
settings-themes-edit = Modifica
settings-themes-delete = Elimina
settings-themes-export = Esporta…
settings-themes-import = Importa…
settings-themes-copy-suffix = copia
settings-themes-json-filter = Tema (JSON)
settings-themes-imported = Tema importato
settings-themes-import-failed = Impossibile importare il tema
settings-themes-exported = Tema esportato
settings-themes-export-failed = Impossibile esportare il tema
settings-themes-low-contrast = contrasto basso
settings-themes-low-contrast-band = l'evidenziazione nasconde il testo
settings-themes-field-name = Nome
settings-themes-field-paper = Pagina
settings-themes-field-ink = Testo
settings-themes-field-general = Sfondo
settings-themes-field-widget-text = Testo della barra dei comandi
settings-themes-field-caret-band = Evidenziazione intorno al cursore
statusbar-focus-settings = Impostazioni senza distrazioni
statusbar-focus-manage-themes = Gestisci temi…
comments-title = Commenti
comments-document-title = Questo documento
comments-empty-project = Nessun commento in questo progetto.
comments-empty-document = Nessun commento su questo documento.
comments-filter-all = Tutti
comments-filter-open = Aperti
comments-filter-resolved = Risolti
comments-filter-orphaned = Orfani
comments-status-open = Aperto
comments-status-resolved = Risolto
comments-status-orphaned = Testo perso
comments-orphan-snippet = (il testo commentato non c'è più)
comments-status-unplaced = Nessuna posizione nel testo
comments-unplaced-snippet = (non ancorato a un testo)
comments-reply-count = { $count ->
    [one] { $count } risposta
   *[other] { $count } risposte
}
comments-menu-resolve = Risolvi
comments-menu-reopen = Riapri
comments-menu-delete = Elimina
comments-sort-document = Ordine del documento
comments-sort-newest = Più recenti prima
comments-sort-reader = Raggruppati per lettore
comments-unsigned-author = Senza firma
comments-agreement = { $count ->
    [one] { $count } lettore
   *[other] { $count } lettori
}
comments-menu-add = Aggiungi commento
comments-menu-add-paragraph = Commenta questo paragrafo
overview-col-comments = Commenti
overview-col-total-comments = Commenti totali
comments-card-placeholder = Scrivi un commento…
comments-card-unknown-author = Autore sconosciuto
comments-unsigned-toast = I tuoi commenti non sono firmati: non è impostato un nome per te su questo computer.
comments-unsigned-action = Imposta il tuo nome
comments-card-reply = Rispondi
comments-card-reply-placeholder = Rispondi…
comments-card-actions = Azioni sul commento
comments-reply-actions = Azioni sulla risposta
comments-menu-delete-reply = Elimina risposta
comments-menu-delete-all = Elimina tutti i commenti qui
comments-deleted-toast = Commento eliminato
comments-reply-deleted-toast = Risposta eliminata
comments-deleted-all-toast = { $count ->
    [one] { $count } commento eliminato
   *[other] { $count } commenti eliminati
}
comments-undo = Annulla
analysis-segment = Analisi
analysis-scope-book = Analisi di questo libro
analysis-run = Esegui analisi
analysis-stale = Modificato dopo l'analisi
analysis-not-run = Non ancora analizzato.
analysis-running = Lettura del manoscritto…
analysis-failed = Impossibile completare l'analisi.
analysis-no-scenes = Nessuna scena in questo libro.
analysis-shape = Forma
analysis-arrivals = Provenienza del testo
analysis-arrivals-explainer = Come il testo è entrato nel progetto mentre era aperto. Indica il percorso dei caratteri, non chi li ha scritti: chi prepara una bozza altrove e la incolla ha incollato, chi detta ha dettato. Non c'è un valore da raggiungere.
analysis-arrivals-scope = Tutto il progetto, non solo questo libro.
analysis-arrivals-session = Dall'apertura. Chiudendo il progetto il conteggio riparte.
analysis-arrivals-nothing = Nessun testo inserito in questa sessione.
analysis-arrivals-typed = Digitato
analysis-arrivals-pasted = Incollato
analysis-arrivals-dictated = Dettato
analysis-arrivals-imported = Importato
analysis-arrivals-programmatic = Inserito automaticamente
analysis-arrivals-count = { $count ->
    [one] { $count } carattere
   *[other] { $count } caratteri
}
analysis-arrivals-none = nessuno
analysis-words-per-scene = Parole per scena
analysis-median-words = La scena mediana di questo libro contiene { $count ->
    [one] { $count } parola.
   *[other] { $count } parole.
}
analysis-median-line = Mediana: { $count ->
    [one] { $count } parola
   *[other] { $count } parole
}
analysis-dialogue = Dialogo
analysis-dialogue-unsupported = Il dialogo non viene ancora misurato per questa lingua.
analysis-footnote-words = Parole nelle note a piè di pagina
analysis-footnote-words-count = { $count ->
    [one] { $count } parola nelle note a piè di pagina di questo libro, separata dal totale del manoscritto.
   *[other] { $count } parole nelle note a piè di pagina di questo libro, separate dal totale del manoscritto.
}
analysis-footnote-words-pending = Conteggio delle note a piè di pagina…
analysis-ignore-empty = Ignora i testi ancora vuoti
analysis-empty-hidden = { $count } { $count ->
        [one] testo vuoto nascosto.
       *[other] testi vuoti nascosti.
    }
analysis-all-texts-empty = Tutti i testi di questo libro sono ancora vuoti.
binder-filter-count = { $shown } di { $total } mostrati
binder-filter-clear = Cancella
binder-filter-none = Nessuna corrispondenza per «{ $query }».
settings-paratext-intro = Le pagine preliminari e finali con cui iniziare un progetto. Ogni struttura appartiene a una tradizione editoriale e i titoli delle pagine sono nella sua lingua. Puoi rinominarli liberamente dopo aver creato il progetto.
settings-paratext-structures = Strutture
settings-paratext-broken = Impossibile leggere
settings-paratext-edit = Modifica
settings-paratext-duplicate = Duplica
settings-paratext-delete = Elimina
settings-paratext-new = Nuova struttura
settings-paratext-save = Salva
settings-paratext-editor-hint = Un nome, le pagine da creare prima del manoscritto e quelle da creare dopo. Decidi tu dove metterle: una volta creato il progetto, sono elementi normali.
image-insert = Inserisci i&mmagine…
image-no-project = Apri un progetto prima di inserire un'immagine.
image-choose-title = Scegli un'immagine
image-filter-label = Immagini
image-large-title = Questa immagine è grande
image-large-text =
    { $name } ha { $megapixels } megapixel ({ $width }×{ $height }).
    Mantenendola così, il file originale viene conservato nel progetto e accompagna
    ogni backup ed esportazione. Ottimizzandola si salva invece una copia più piccola,
    fino a 2560 pixel sul lato più lungo.
image-large-keep = Mantieni l'originale
image-large-downscale = Ottimizza
image-large-remember = Usa questa scelta d'ora in poi, senza chiedere
image-not-recorded = L'immagine è stata salvata ma non è stato possibile registrarla nel progetto.
image-describe-title = Descrivi l'immagine
image-describe-explain = Ciò che mostra l'immagine, per chi non può vederla. Non fa parte del manoscritto: non viene contato, cercato o esportato come testo narrativo.
image-describe-placeholder = un faro contro un cielo grigio
image-resize-title = Ridimensiona l'immagine
image-resize-explain = Una percentuale della dimensione visualizzata ora. 100 la lascia invariata.
image-resize-invalid = Inserisci un numero tra 1 e 1000.
image-menu-describe = &Descrivi l'immagine…
image-menu-resize = &Ridimensiona l'immagine…
image-menu-reset-size = Dimensione &originale
cover-heading = Copertina
cover-pick = Scegli…
cover-change = Cambia…
cover-remove = Rimuovi
cover-none = Ancora nessuna copertina
cover-missing = Il file dell'immagine manca nel progetto
cover-choose-title = Scegli una copertina
cover-set = Copertina impostata.
cover-cleared = Copertina rimossa. L'immagine è ancora nel progetto.
card-label = Dicitura
card-exportable = Esportato
card-numbered = Numerato
card-created = Creato
card-modified = Modificato
card-yes = sì
card-no = no
card-type = Tipo
card-position = Posizione
card-children = Elementi contenuti
card-goal = Obiettivo
card-words = Parole
card-synopsis = Sinossi
card-point-of-view = Punto di vista
card-aliases = Altri nomi
export-orphan-footnotes-title = Note a piè di pagina senza riferimento
export-comments-dropped =
    { $count ->
        [one] Un commento non è stato collocato nel testo esportato ed è stato escluso.
       *[other] { $count } commenti non sono stati collocati nel testo esportato e sono stati esclusi.
    }
export-orphan-footnotes =
    { $count ->
        [one] Una nota a piè di pagina non ha più riferimenti nel manoscritto. Il suo testo non apparirà nel libro esportato.
       *[other] { $count } note a piè di pagina non hanno più riferimenti nel manoscritto. Il loro testo non apparirà nel libro esportato.
    }
menu-footnotes = &Note a piè di pagina
footnotes-title = Note a piè di pagina
footnotes-insert = Inserisci nota a &piè di pagina
footnotes-empty = Nessuna nota a piè di pagina. Posiziona il cursore in una scena, poi usa + in alto o Ctrl+Alt+F.
footnotes-filter-all = Tutte
footnotes-filter-document = Questo documento
footnotes-filter-orphaned = Orfane
footnotes-orphaned = Nulla rimanda più a questa nota
footnotes-untitled-home = Senza titolo
footnotes-body-placeholder = il testo della nota
footnotes-insert-tooltip = Inserisci una nota a piè di pagina al cursore (Ctrl+Alt+F)
footnotes-actions = Azioni sulla nota a piè di pagina
footnotes-delete = &Elimina la nota e il suo riferimento
footnotes-no-project = Apri un progetto prima di inserire una nota a piè di pagina.
footnotes-no-caret = Posiziona il cursore nel testo di una scena per inserirvi una nota a piè di pagina.
footnotes-not-created = Impossibile aggiungere la nota a piè di pagina al progetto.
footnotes-deleted-toast = Nota eliminata
footnotes-undo-delete = Annulla
import-document-title = Importa documenti
import-document-close = Chiudi
import-document-step-files = File
import-document-step-review = Verifica
import-document-step-destination = Destinazione
import-document-step-reconcile = Unione
import-document-reconcile-hint = Alcuni di questi elementi sono già presenti. Scegli cosa fare per ciascuno.
import-document-reconcile-all-new = Nulla in questo file corrisponde al progetto: ogni elemento verrà aggiunto come nuovo.
import-document-duplicate-returns = Questi file sono più copie restituite dello stesso manoscritto. Importali uno alla volta: il confronto allinea una sola copia al progetto, quindi importarli insieme aggiungerebbe una seconda copia del libro anziché unirli.
import-document-hunk-added = Accetta questo nuovo paragrafo
import-document-hunk-removed = Elimina questo paragrafo
import-document-hunk-changed = Accetta questa riscrittura
import-document-col-stray-prose = Il suo testo
import-document-stray-as-paratext = Conserva come paratesto
import-document-stray-discard = Scarta il testo
import-document-col-current = Nel tuo progetto
import-document-col-incoming = In questo file
import-document-col-status = Stato
import-document-col-action = Cosa fare
import-document-status-identical = Uguale
import-document-status-editor-edited = Modificato dal revisore
import-document-status-you-edited = Modificato da te
import-document-status-conflict = Modificato da entrambi
import-document-status-different = Diverso
import-document-status-new = Nuovo
import-document-status-missing = Assente da questo file
import-document-status-moved = Spostato
import-document-action-comments-only = Solo commenti
import-document-action-take-import = Usa questa versione
import-document-action-keep-current = Mantieni la mia
import-document-action-create-new = Aggiungi come nuovo
import-document-action-ignore = Salta
import-document-compare = Confronta
import-document-compare-legend = La tua versione confrontata con quella del file. Sola lettura.
import-document-compare-close = Chiudi
import-document-drop-title = Trascina qui i documenti
import-document-drop-hint = Markdown (.md), Word (.docx), OpenDocument (.odt) e testo semplice (.txt)
import-document-browse = Sfoglia…
import-document-move-up = Sposta in alto
import-document-move-down = Sposta in basso
import-document-remove-file = Rimuovi
import-document-file-count = { $count ->
    [one] 1 file
   *[other] { $count } file
}
import-document-no-files = Nessun file selezionato.
import-document-col-included = Importa
import-document-col-title = Titolo
import-document-col-type = Tipo
import-document-col-words = Parole
import-document-col-breaks = Separatori
import-document-col-comments = Commenti
import-document-col-footnotes = Note a piè di pagina
import-document-col-epigraph = Epigrafe
import-document-col-source = Origine
import-document-level-rules = Livelli delle intestazioni
import-document-level-n = Intestazione { $level }
import-document-add-top-level = Aggiungi livello principale
import-document-add-top-level-tooltip = Inserisci un libro sopra tutti gli elementi analizzati, per file di capitoli privi di intestazione del libro
import-document-destination = Destinazione
import-document-destination-hint = Scegli un raccoglitore o un elemento: i nuovi elementi vengono inseriti dentro una cartella o dopo una scena.
import-document-destination-empty = Nessun raccoglitore. Creane prima uno.
import-document-plan-empty = Ancora nulla da importare.
import-document-pictures = { $count ->
    [1] Con questi documenti arriva un'immagine.
   *[other] Con questi documenti arrivano { $count } immagini.
}
import-document-summary = { $rows ->
    [one] 1 elemento
   *[other] { $rows } elementi
} · { $breaks ->
    [one] 1 separatore di scena
   *[other] { $breaks } separatori di scena
}
import-document-back = Indietro
import-document-cancel = Annulla
import-document-analyse = Avanti
import-document-analysing = Lettura dei documenti…
import-document-step-analysing = Lettura
import-document-cancel-analysis = Interrompi lettura
import-document-analyse-failed = Impossibile leggere i documenti.
import-document-details = Dettagli
import-document-import = Importa
import-document-done = { $count ->
    [one] 1 elemento importato
   *[other] { $count } elementi importati
}
import-document-undo = Annulla
import-diagnostic-file-unreadable = Impossibile leggere «{ $path }»: { $detail }. Gli altri file possono essere importati.
import-diagnostic-nested-too-deep = «{ $path }» non è stato letto: il contenuto supera { $limit } livelli di annidamento. Nessun documento reale raggiunge tale profondità e leggerlo potrebbe far arrestare Skribisto, quindi viene escluso. Gli altri file possono essere importati.
import-diagnostic-archive-too-large = «{ $path }» non è stato letto: è un file compresso che si espanderebbe enormemente, rischiando di esaurire la memoria e far arrestare Skribisto. Viene escluso; gli altri file possono essere importati.
import-diagnostic-lossy-decode = { $count ->
    [one] Un carattere in «{ $path }» non è stato decodificato. Salva di nuovo il file in UTF-8 per conservarlo.
   *[other] { $count } caratteri in «{ $path }» non sono stati decodificati. Salva di nuovo il file in UTF-8 per conservarli.
}
import-diagnostic-decoded-from-bom = «{ $path }» è stato decodificato come { $detail }, non UTF-8.
import-diagnostic-empty-file = «{ $path }» è vuoto.
import-diagnostic-no-headings = «{ $path }» non ha intestazioni, quindi viene importato come elemento singolo.
import-diagnostic-unsupported-format = I file «.{ $detail }» non sono supportati, quindi «{ $path }» è stato saltato.
import-diagnostic-front-matter-not-flat = Metadati iniziali di «{ $path }»: «{ $detail }» non è un valore semplice ed è stato saltato.
import-diagnostic-footnotes-degraded = { $count ->
    [one] Una nota a piè di pagina in «{ $path }» arriva come testo semplice: le note non vengono lette da Markdown.
   *[other] { $count } note a piè di pagina in «{ $path }» arrivano come testo semplice: le note non vengono lette da Markdown.
}
import-diagnostic-footnote-not-carried = { $count ->
    [one] Una nota a piè di pagina in «{ $path }» non è stata trasferita. Le altre sì.
   *[other] { $count } note a piè di pagina in «{ $path }» non sono state trasferite. Le altre sì.
}
import-diagnostic-raw-html-dropped = { $count ->
    [one] Un blocco di HTML grezzo in «{ $path }» è stato escluso.
   *[other] { $count } blocchi di HTML grezzo in «{ $path }» sono stati esclusi.
}
import-diagnostic-nested-break-dropped = { $count ->
    [one] Un separatore di scena dentro una citazione o un elenco in «{ $path }» è stato escluso. Si conservano solo separatori su una riga propria.
   *[other] { $count } separatori di scena dentro citazioni o elenchi in «{ $path }» sono stati esclusi. Si conservano solo separatori su una riga propria.
}
import-diagnostic-image-not-ingested-format = In «{ $path }», l'immagine «{ $detail }» è in un formato che Skribisto non sa mostrare, quindi è stata esclusa. Le immagini PNG, JPEG, WebP, GIF, TIFF o BMP vengono importate: salvala in uno di questi formati e inseriscila a mano.
import-diagnostic-image-not-ingested-remote = In «{ $path }», l'immagine «{ $detail }» si trova sul web. Durante un'importazione le immagini non vengono mai scaricate, quindi è stata esclusa.
import-diagnostic-image-not-ingested-outside = In «{ $path }», l'immagine «{ $detail }» si trova fuori dal documento e dalla sua cartella, quindi è stata esclusa. Vengono importate solo le immagini contenute nel documento o nella cartella del documento stesso.
import-diagnostic-image-not-ingested-missing = In «{ $path }», l'immagine «{ $detail }» non è stata trovata, quindi è stata esclusa.
import-diagnostic-image-not-ingested-too-large = In «{ $path }», l'immagine «{ $detail }» è più grande di quanto un progetto accetti, quindi è stata esclusa.
import-diagnostic-image-not-ingested-unreadable = In «{ $path }», non è stato possibile leggere l'immagine «{ $detail }», quindi è stata esclusa.
import-diagnostic-image-not-ingested-heading = In «{ $path }», l'immagine «{ $detail }» si trova in un'intestazione. Ogni intestazione diventa il titolo di un elemento del raccoglitore, e un titolo non contiene immagini, quindi l'immagine è stata esclusa.
import-diagnostic-image-not-ingested-note = In «{ $path }», l'immagine «{ $detail }» si trova in una nota a piè di pagina, in una nota di chiusura o in un commento. Questi conservano le parole ma non le immagini, quindi è stata esclusa.
import-diagnostic-image-not-ingested-html = In «{ $path }», l'immagine «{ $detail }» è scritta in HTML e un'importazione esclude l'HTML, quindi è stata esclusa insieme a esso. Scrivila come immagine Markdown per importarla.
import-diagnostic-image-not-ingested-document-full = In «{ $path }», l'immagine «{ $detail }» è stata esclusa: le immagini di questo documento, tutte insieme, superano quanto un'importazione può trasferire. Importa il documento in più parti, oppure inserisci a mano le restanti.
import-diagnostic-picture-first-frame-only = In «{ $path }», l'immagine animata «{ $detail }» arriva come il suo primo fotogramma. Le immagini di un progetto non si muovono.
import-diagnostic-duplicate-title = «{ $title }» compare { $count } volte. Se hai già importato questi file, ne verranno creati dei duplicati.
import-diagnostic-heading-level-jump = «{ $title }» passa dal livello di intestazione { $from } a { $to }; viene collocato un livello sotto il proprio padre.
import-diagnostic-illegal-combination = «{ $title }» contiene testo, ma il tipo «{ $kind }» non può contenerne. L'importazione resta bloccata finché non cambi il tipo o lo escludi. Altrimenti non verrà importato nulla.
import-diagnostic-tracked-changes-flattened = { $path } era in revisione: { $count ->
    [one] { $count } modifica registrata è stata accettata
   *[other] { $count } modifiche registrate sono state accettate
} e le eliminazioni sono state applicate. Questo è il testo finale. Per mantenere invece le tue formulazioni, usa Confronta nell'ultimo passaggio.
import-diagnostic-tracked-changes-flattened-by = { $path } era in revisione: { $count ->
    [one] { $count } modifica registrata di { $names } è stata accettata
   *[other] { $count } modifiche registrate di { $names } sono state accettate
} e le eliminazioni sono state applicate. Questo è il testo finale. Per mantenere invece le tue formulazioni, usa Confronta nell'ultimo passaggio.
import-diagnostic-text-box-dropped = { $count ->
    [one] { $path } contiene { $count } casella di testo. Il suo testo è fuori dal flusso del documento e non è possibile stabilire dove collocarlo nel manoscritto. Non viene importata.
   *[other] { $path } contiene { $count } caselle di testo. Il loro testo è fuori dal flusso del documento e non è possibile stabilire dove collocarlo nel manoscritto. Non vengono importate.
}
import-diagnostic-embedded-object-dropped = { $count ->
    [one] { $path } contiene { $count } oggetto incorporato: un grafico, una formula o simili. Nel manoscritto non c'è nulla che possa contenerlo.
   *[other] { $path } contiene { $count } oggetti incorporati: grafici, formule o simili. Nel manoscritto non c'è nulla che possa contenerli.
}
import-diagnostic-field-flattened = { $count ->
    [one] { $path } contiene { $count } campo: un numero di pagina, un riferimento incrociato o una data. Mantiene l'ultimo testo visualizzato e non si aggiornerà più.
   *[other] { $path } contiene { $count } campi: numeri di pagina, riferimenti incrociati o date. Ciascuno mantiene l'ultimo testo visualizzato e non si aggiornerà più.
}
import-diagnostic-unknown-style-level = { $path } usa lo stile «{ $detail }», che sembra un'intestazione ma non indica il livello. Quei paragrafi vengono importati come testo, senza indovinarne la profondità.
import-diagnostic-comment-unanchored = Il commento «{ $detail }» in { $path } non è stato ancorato alle parole a cui si riferiva. Resta sul suo elemento, dove puoi spostarlo.
import-diagnostic-comment-not-carried = Il commento «{ $detail }» in { $path } non viene importato perché nessun contenuto del file arriva come testo a cui associarlo. Copialo dal file se ti serve.
import-diagnostic-comment-replies-flattened = { $count ->
    [one] { $count } risposta in { $path } rimandava a un commento assente dal file, quindi arriva come commento indipendente.
   *[other] { $count } risposte in { $path } rimandavano a commenti assenti dal file, quindi arrivano come commenti indipendenti.
}
import-diagnostic-prose-not-verbatim = { $count ->
    [one] { $count } paragrafo in { $path } non è stato trasferito con la formattazione esatta, quindi arriva come testo semplice. Confrontalo con l'originale.
   *[other] { $count } paragrafi in { $path } non sono stati trasferiti con la formattazione esatta, quindi arrivano come testo semplice. Confrontali con l'originale.
}
import-diagnostic-styled-spaces-not-carried = { $count ->
    [one] In { $path }, { $count } sequenza di spazi era sottolineata o barrata, per esempio una riga da compilare. Skribisto non conserva queste linee sugli spazi soli: arriva come spazi semplici oppure viene omessa a inizio o fine paragrafo.
   *[other] In { $path }, { $count } sequenze di spazi erano sottolineate o barrate, per esempio righe da compilare. Skribisto non conserva queste linee sugli spazi soli: arrivano come spazi semplici oppure vengono omesse a inizio o fine paragrafo.
}
import-diagnostic-list-nesting-flattened = { $count ->
    [one] { $count } voce di elenco in { $path } superava { $limit } livelli di annidamento. Arriva al livello { $limit }, il più profondo conservato da Skribisto, con parole e formattazione.
   *[other] { $count } voci di elenco in { $path } superavano { $limit } livelli di annidamento. Arrivano al livello { $limit }, il più profondo conservato da Skribisto, con parole e formattazione.
}
import-diagnostic-table-read-as-paragraphs = { $count ->
    [one] { $count } tabella in { $path } ha una riga molto più larga delle altre, troppo irregolare per una griglia. Le celle arrivano come paragrafi consecutivi, con parole e formattazione.
   *[other] { $count } tabelle in { $path } hanno una riga molto più larga delle altre, troppo irregolare per una griglia. Le celle arrivano come paragrafi consecutivi, con parole e formattazione.
}
import-diagnostic-spaces-shortened = { $count ->
    [one] { $count } sequenza di spazi in { $path } superava { $limit } spazi ed è stata accorciata a { $limit }. Tutte le parole circostanti sono conservate.
   *[other] { $count } sequenze di spazi in { $path } superavano { $limit } spazi e sono state accorciate a { $limit }. Tutte le parole circostanti sono conservate.
}
import-diagnostic-epigraph-not-carried = «{ $title }» è preceduto da un'epigrafe, ma il tipo «{ $kind }» non può contenerla. La citazione resta invece all'inizio del suo testo.
import-diagnostic-epigraph-placement-ambiguous = Un'epigrafe si trova tra «{ $title }» e «{ $below }» e potrebbe appartenere a entrambi. È stata assegnata a «{ $title }», la posizione abituale per un'epigrafe.
import-document-diagnostics = { $errors ->
    [0] { $warnings ->
            [one] 1 cosa da sapere
           *[other] { $warnings } cose da sapere
        }
   *[other] { $errors ->
            [one] 1 file non è stato letto
           *[other] { $errors } file non sono stati letti
        }
}
import-document-diagnostics-none = Nulla da segnalare.
versions-title = Versioni
versions-scope-synopsis = Sinossi
versions-scope-prose = Testo
versions-loading = Esame dei backup…
versions-empty = Nessuna versione precedente disponibile
versions-error = Impossibile leggere i backup. Nulla è andato perso, ma l'elenco potrebbe essere incompleto
versions-did-not-exist = Non esisteva ancora il { $date }
versions-deleted-after = Eliminato dopo il { $date }
versions-unreadable = { $count ->
    [one] { $count } backup non è stato letto
   *[other] { $count } backup non sono stati letti
}
versions-thinned = { $count ->
    [one] Le versioni più vecchie si diradano con il tempo. La cronologia del progetto ha già eliminato { $count } stato precedente di questo testo.
   *[other] Le versioni più vecchie si diradano con il tempo. La cronologia del progetto ha già eliminato { $count } stati precedenti di questo testo.
}
versions-source-backup = Da un backup
versions-source-project = Dalla cronologia del progetto
versions-list-caption = Una voce per modifica, non per backup
versions-pick-a-version = Scegli una versione per vedere cosa è cambiato
versions-earliest = La prima versione registrata. Non ce ne sono di precedenti con cui confrontarla.
versions-no-change = Nessuna modifica in questa parte dell'elemento
versions-formatting-only = Qui è cambiata solo la formattazione. Le parole sono le stesse
versions-show-unchanged = Mostra parti invariate
versions-hide-unchanged = Nascondi parti invariate
versions-next-change = Modifica successiva
versions-near = vicino a «{ $text }»
versions-words-added = { $count ->
    [one] { $count } parola aggiunta
   *[other] { $count } parole aggiunte
}
versions-words-removed = { $count ->
    [one] { $count } parola rimossa
   *[other] { $count } parole rimosse
}
versions-blocks-moved = { $count ->
    [one] { $count } paragrafo spostato
   *[other] { $count } paragrafi spostati
}
versions-pin = Fissa il backup da cui proviene questa versione. La pulizia automatica non lo eliminerà mai
versions-unpin = Sfissa il backup da cui proviene questa versione. La pulizia automatica potrà eliminarlo di nuovo
versions-pinned-only = Mostra solo versioni fissate
versions-pin-note = Si possono fissare solo versioni provenienti da un backup. Fissare protegge un file, e la cronologia del progetto non è un file.
versions-pinned-empty = Non c'è ancora nulla di fissato
versions-pinned-empty-log = Non c'è ancora nulla di fissato. Si possono fissare solo versioni provenienti da un backup. Fissare protegge un file, e la cronologia del progetto non è un file.
versions-range-filter = Mostra solo versioni tra due date
versions-filtered-empty = Nessuna versione corrisponde ai filtri impostati
versions-clear-filters = Cancella filtri
versions-last-30-days = Ultimi 30 giorni
versions-restore-button = Ripristina questa versione
versions-restore-confirm-title = Sostituire questo testo con la versione del { $date }?
versions-restore-confirm-text = Il testo attuale verrà sostituito con quello che questo elemento aveva il { $date }.
versions-restore-confirm-with-comments = Il testo attuale verrà sostituito con quello che questo elemento aveva il { $date }. { $count ->
    [one] { $count } commento è ancorato al testo attuale e potrebbe restare orfano.
   *[other] { $count } commenti sono ancorati al testo attuale e potrebbero restare orfani.
}
versions-restore-confirm-undo-note = Prima viene creato un backup. Ctrl+Z annulla l'operazione in un solo passaggio.
versions-restored-toast = Versione del { $date } ripristinata
versions-undo = Annulla
versions-restore-row-gone = Quell'elemento non è più in questo progetto
versions-restore-no-home = L'elemento ha cambiato tipo da allora e non può più contenere il vecchio testo
versions-restore-no-safety-copy = Il backup di sicurezza non è stato eseguito, quindi nulla è cambiato
versions-restore-failed = Ripristino non riuscito: { $error }
versions-restore-backup-busy = Un backup è già in corso. Riprova tra poco
versions-restore-in-backup-file = Stai guardando un file di backup; apri il progetto originale per ripristinarlo
versions-restore-no-project = Nessun progetto aperto
versions-recreate-button = Recupera questo elemento…
versions-recreate-picker-title = Dove vuoi metterlo?
versions-recreate-picker-empty = Questo progetto non ha un raccoglitore in cui inserirlo
versions-recreate-picker-confirm = Recupera qui
versions-recreate-picker-cancel = Annulla
versions-recreate-untitled = questo elemento
versions-recreate-confirm-title = Recuperare «{ $item }»?
versions-recreate-confirm-text = Verrà aggiunto a { $destination }, con il testo che aveva il { $date }.
versions-recreate-confirm-undo-note = Annulla, nel messaggio successivo, lo rimuove immediatamente.
versions-recreated-toast = «{ $item }» è di nuovo nel progetto
versions-recreated-partial-toast = { $count ->
    [one] «{ $item }» è tornato, ma uno dei suoi testi non è stato letto
   *[other] «{ $item }» è tornato, ma { $count } dei suoi testi non sono stati letti
}
versions-recreate-already-here = Quell'elemento è già nel progetto
versions-recreate-no-destination = Scegli una posizione nel raccoglitore
versions-recreate-unreadable = Impossibile leggere quel backup, quindi nulla è stato aggiunto
versions-recreate-failed = Impossibile recuperare l'elemento: { $error }
versions-changed-percent = Il { $percent }% è cambiato
versions-hidden-paragraphs = { $count ->
    [one] … { $count } paragrafo invariato …
   *[other] … { $count } paragrafi invariati …
}
timeline-title = Torna indietro nel tempo
timeline-coverage = { $count ->
    [one] { $count } versione registrata, a partire dal { $oldest }
   *[other] { $count } versioni registrate, a partire dal { $oldest }
}
timeline-loading = Esame del passato del progetto…
timeline-empty = Nessuna versione del progetto è stata ancora registrata
timeline-no-changes = Nulla è cambiato da allora
timeline-no-text-changes = Nessun testo è cambiato da allora
timeline-slider-label = Versione registrata
timeline-series-name = Dimensione del progetto
timeline-bars-caption = Ogni barra è una versione registrata, alta quanto il progetto in quel momento. Quella evidenziata è la versione che stai guardando.
timeline-bars-caption-periods = Troppe versioni per mostrarle singolarmente: ogni barra rappresenta un periodo ({ $period }) e la sua altezza indica la dimensione del progetto alla fine di quel periodo.
timeline-unit-hour = ora
timeline-unit-day = giorno
timeline-unit-week = settimana
timeline-unit-month = mese
timeline-range-filter = Mostra solo versioni tra due date
timeline-range-empty = Nessuna versione registrata in quelle date
timeline-open-period = Apri questo periodo
timeline-show-all = Mostra tutta la cronologia
timeline-changed-since = { $count ->
    [one] { $count } elemento è diverso tra il { $date } e il progetto attuale
   *[other] { $count } elementi sono diversi tra il { $date } e il progetto attuale
}
timeline-kind-added = Scritto da allora
timeline-kind-removed = Non più nel progetto
timeline-kind-changed = Modificato da allora
timeline-kind-moved = Spostato da allora
timeline-not-yet-written = Non esisteva ancora in quel momento
timeline-no-text-of-its-own = Non ha un testo proprio. È un'intestazione per ciò che contiene
timeline-reader-close = Chiudi
timeline-reader-stamp = Com'era il { $date }
timeline-reader-loading = Apertura della versione registrata…
timeline-reader-unreadable = Impossibile leggere questa registrazione. Il backup potrebbe essere stato spostato, eliminato o trovarsi su un disco scollegato.
timeline-reader-compared = Com'era il { $date }, confrontato con il testo attuale
timeline-reader-diff-legend = Il testo barrato è stato rimosso da allora; quello sottolineato è stato aggiunto.
timeline-reader-view-label = Cosa mostrare
timeline-reader-view-diff = Modifiche
timeline-reader-view-text = Testo
timeline-reader-deleted = Questo elemento non è più nel progetto. Qui puoi leggerlo e copiarne il testo.
timeline-prose-only-record = Questo punto viene dalla cronologia del progetto, che conserva soltanto i testi: non può mostrare cosa è stato eliminato o spostato da allora. Per questo, scegli un backup.
settings-sec-extensions = Estensioni
settings-group-games-forward = Sempre avanti
settings-games-forward-toggle = Gioca a «Sempre avanti»
settings-games-forward-hint =
    Durante il gioco non puoi ritirare ciò che hai scritto: Backspace, Canc, Taglia,
    trascinamento e Annulla sono disattivati nelle superfici che scegli sotto.
    Puoi ancora digitare, incollare, formattare e muoverti liberamente. La bozza può solo crescere.
settings-games-session-warning =
    È una scelta valida solo per la sessione e non viene salvata. Chiudere il progetto
    o uscire da Skribisto termina sempre il gioco, e tutto ciò che hai scritto durante
    il gioco torna subito annullabile.
settings-group-games-scope = Dove si applica
settings-games-in-prose = Testo del manoscritto
settings-games-in-synopsis = Sinossi
settings-games-scope-hint =
    Commenti, note a piè di pagina e titoli non vengono mai bloccati: sono i posti in cui
    annotare la correzione che ti sei appena vietato di fare.
settings-games-inert-warning =
    «Sempre avanti» è attivo ma non si applica a nulla. Seleziona almeno una superficie
    sopra, altrimenti non cambia nulla mentre scrivi.
games-title = Giochi di scrittura
games-forward-name = Sempre avanti
games-forward-blurb =
    Scrivi la bozza senza tornare indietro. Eliminazione, taglio e annullamento sono
    disattivati durante il gioco. Scrivi la frase successiva invece di correggere l'ultima.
games-forward-playing = Gioco attivo: eliminazione disabilitata
games-forward-idle = Nessun gioco attivo
games-session-note = Termina quando chiudi il progetto.
games-scope-prose-and-synopsis = Si applica al testo e alle sinossi.
games-scope-prose = Si applica al testo.
games-scope-synopsis = Si applica alle sinossi.
games-scope-nothing = Non si applica ancora a nulla. Scegli una superficie nelle Impostazioni.
games-settings-link = Impostazioni dei giochi di scrittura…
statusbar-games-forward = Sempre avanti
statusbar-games-forward-tooltip =
    «Sempre avanti» è attivo: eliminazione, taglio e annullamento sono disattivati
    durante la stesura. Fai clic per terminare il gioco.
goal-progress-words = { $g ->
    [one] { $count } di { $goal } parola
   *[other] { $count } di { $goal } parole
}
goal-progress-characters = { $g ->
    [one] { $count } di { $goal } carattere
   *[other] { $count } di { $goal } caratteri
}
goal-count-words = { $n ->
    [one] { $count } parola
   *[other] { $count } parole
}
goal-count-characters = { $n ->
    [one] { $count } carattere
   *[other] { $count } caratteri
}
goal-unit-words = Parole
goal-unit-characters = Caratteri
inspector-goal = Obiettivo
inspector-goal-none = Nessun obiettivo
overview-col-goal = Obiettivo
overview-excluded-from-export = Escluso dall'esportazione, quindi non contribuisce ad alcun totale
goal-subtree-total-words = { $count ->
    [one] { $items } obiettivo interno ammonta a { $words } parole
   *[other] { $items } obiettivi interni ammontano a { $words } parole
}
goal-subtree-total-characters = { $count ->
    [one] { $items } obiettivo interno ammonta a { $words } caratteri
   *[other] { $items } obiettivi interni ammontano a { $words } caratteri
}
new-work-goal-unit = Conta in
new-work-goal-unit-hint = Usato per tutti gli obiettivi di parole o caratteri del progetto. Modificabile in seguito.
settings-group-goal-unit = Obiettivi
settings-goal-unit-switch-title = Cambia unità di conteggio
settings-goal-unit-switch-text = Gli obiettivi e i traguardi già impostati per questo progetto sono stati inseriti in { $from }. Passare a { $to } non li converte, quindi ogni numero esistente verrà ora letto come { $to }.
settings-goal-unit-switch-informative = Nulla va perso. Torna indietro in qualsiasi momento per ripristinare l'interpretazione originale, poi aggiorna gli obiettivi che vuoi mantenere.
settings-goal-unit-switch-confirm = Cambia
goal-distribute-action = Distribuisci…
distribute-title = Distribuisci l'obiettivo
distribute-empty = Non ci sono elementi interni tra cui distribuire l'obiettivo.
distribute-weight = Ripartizione
distribute-weight-length = Per lunghezza
distribute-weight-rows = Per elementi
distribute-weight-even = In parti uguali
distribute-overwrite = Sostituisci gli obiettivi già impostati
distribute-col-item = Elemento
distribute-col-current = Attuale
distribute-col-proposed = Proposto
distribute-total = La somma è { $total }, rispetto a un obiettivo di { $goal }.
distribute-over-budget = Gli obiettivi interni già impostati superano quello del contenitore di { $over }. Aumenta il suo obiettivo, riduci quelli interni oppure sostituiscili.
distribute-apply = Distribuisci
distribute-cancel = Annulla
milestone-target-gone = Obiettivo eliminato
milestone-no-target = Nessun obiettivo
milestone-add = Aggiungi
milestone-add-label = Cosa raggiungere
pace-summary-title = A che punto è il libro
pace-summary-remaining = Mancano { $words }
pace-summary-open = Apri il piano
pace-summary-close = Chiudi
pace-summary-dont-show = Non mostrare all'apertura
pace-summary-menu = Piano di scrittura…
menu-help-topics = &Argomenti della guida
menu-help-shortcuts = &Scorciatoie da tastiera…
menu-help-website = Skribisto sul &web
menu-help-report = Segnala un &problema…
menu-help-check-updates = Cerca a&ggiornamenti
menu-command-palette = Tavolozza dei &comandi…
learn-help-topics = Argomenti della guida
learn-shortcuts = Scorciatoie da tastiera
learn-website = Skribisto sul web
help-window-title = Guida
help-filter-topics = Filtra argomenti
help-no-matching-topic = Nessun argomento corrispondente.
help-topic-missing = Questo argomento non è più disponibile.
help-back = Indietro
help-not-translated = Questa pagina non è ancora stata tradotta, quindi viene mostrata in inglese.
help-section-getting-started = Per iniziare
help-section-writing = Scrittura
help-section-reviewing = Revisione
help-section-exchanging = Importare ed esportare testi
help-section-keeping = Proteggere il tuo lavoro
help-section-extensions = Estensioni
help-topic-getting-started = Il tuo primo progetto
help-topic-writing-model = Come è fatto un libro
help-topic-drafts-and-old-versions = Conservare una vecchia bozza
help-topic-goals-and-pace = Obiettivi e ritmo
help-topic-comments = Commenti
help-topic-round-trip = Inviare il libro a un lettore
help-topic-export = Esportazione
help-topic-import-documents = Importare documenti
help-topic-import-projects = Trasferire un intero progetto
help-topic-import-scrivener = Passare da Scrivener
help-topic-backups-and-versions = Backup e versioni
help-shortcuts-title = Scorciatoie da tastiera
help-shortcuts-filter = Filtra scorciatoie
help-shortcuts-no-matches = Nessuna scorciatoia corrispondente.
help-shortcuts-rebind = Modifica scorciatoie…
help-shortcuts-close = Chiudi
command-palette-placeholder = Digita un comando
shortcut-name-binder-duplicate = Duplica
shortcut-name-comments-add = Aggiungi commento
shortcut-name-comments-add-paragraph = Commenta paragrafo
shortcut-name-spellcheck-toggle = Controlla ortografia
shortcut-name-editor-tab-close = Chiudi scheda
shortcut-name-editor-tab-pin = Fissa o sfissa scheda
shortcut-name-editor-save = Salva
shortcut-name-work-export = Esporta…
shortcut-name-work-new = Nuovo progetto
shortcut-name-work-open = Apri progetto
shortcut-name-window-new = Nuova finestra
shortcut-name-work-close = Chiudi progetto
shortcut-name-work-import-scrivener = Importa progetto Scrivener…
shortcut-name-work-import-scrivener-report = Mostra il rapporto di importazione da Scrivener
shortcut-name-app-settings = Impostazioni
shortcut-name-app-quit = Esci
shortcut-name-editor-insert-footnote = Inserisci nota a piè di pagina
shortcut-name-format-scene-break = Inserisci separatore di scena
shortcut-name-format-major-scene-break = Inserisci separatore di scena maggiore
shortcut-name-format-link = Inserisci collegamento
shortcut-name-format-mark-nbsp = Inserisci spazio non separabile
shortcut-name-format-mark-nbhyphen = Inserisci trattino non separabile
shortcut-name-format-mark-nnbsp = Inserisci spazio non separabile stretto
shortcut-name-format-mark-zwsp = Inserisci spazio a larghezza zero
shortcut-name-go-next = Successivo
shortcut-name-go-prev = Precedente
shortcut-name-go-to = Vai a
shortcut-name-outline-toggle = Mostra o nascondi struttura
shortcut-name-preview-toggle = Mostra o nascondi fascia di anteprima
shortcut-name-view-fullscreen = Attiva o disattiva schermo intero
shortcut-name-view-focus-mode = Attiva o disattiva modalità senza distrazioni
shortcut-name-editor-size-increase = Aumenta dimensione del testo
shortcut-name-editor-size-decrease = Riduci dimensione del testo
shortcut-name-editor-size-reset = Ripristina dimensione del testo
shortcut-name-editor-find = Cerca
shortcut-name-editor-replace = Sostituisci
shortcut-name-editor-find-next = Corrispondenza successiva
shortcut-name-editor-find-prev = Corrispondenza precedente
shortcut-name-search-show = Cerca nel progetto
shortcut-name-search-replace = Sostituisci nel progetto
shortcut-name-outline-open-to-side = Apri a lato
shortcut-name-help-topics = Argomenti della guida
shortcut-name-help-shortcuts = Scorciatoie da tastiera
shortcut-name-help-website = Skribisto sul web
shortcut-name-help-report = Segnala un problema
shortcut-name-help-check-updates = Cerca aggiornamenti
update-check-behind = È disponibile la versione { $version }.
update-check-current = Stai usando la versione più recente.
update-check-failed = Impossibile verificare la disponibilità di una nuova versione. { $error }
shortcut-name-command-palette = Tavolozza dei comandi
help-section-reference = Significato dei termini
help-concept-scene-break = Separatore di scena
help-concept-major-scene-break = Separatore di scena maggiore
help-concept-find-in-prose = Cerca nel testo
help-concept-story-bible = Bibbia narrativa
help-concept-goal-unit = Parole o caratteri
help-concept-goal-progress = Progressi
help-concept-manuscript-words = Cosa conta come manoscritto
help-concept-exportable = Escluso dall'esportazione
help-concept-distribute = Distribuire un obiettivo
help-concept-subtree-total = Somma degli obiettivi interni
help-concept-milestone = Traguardo
help-concept-pace-plan = Piano del ritmo
help-concept-tag = Etichetta
help-concept-label = Dicitura
help-concept-point-of-view = Punto di vista
help-concept-epigraph = Epigrafe
help-concept-footnote = Nota a piè di pagina
help-concept-chapter-mode = Forma del capitolo
help-concept-comment = Commento
help-concept-backup = Backup
help-concept-version = Versione
help-concept-trash = Cestino
help-concept-spellcheck = Controllo ortografico
help-concept-search-replace = Cerca e sostituisci
help-concept-note-template = Modello di nota
help-concept-story-bible-entry = Voce della bibbia narrativa
help-concept-text-replacement = Sostituzione di testo
help-concept-smart-punctuation = Punteggiatura intelligente
help-concept-export-style = Stile di esportazione
help-concept-round-trip-marks = Marcatori di reimportazione
epigraph-mark-attribution = Riga della fonte
epigraph-mark-attribution-tip = Segna la riga del cursore come fonte della citazione, affinché venga stampata come attribuzione
margin-lane-name = Segni a margine
margin-lane-provider-comments = Commenti
margin-lane-provider-story-bible = Voce della bibbia narrativa
margin-lane-provider-story-bible-hint = Nella vista «Nel testo» di una nota, i punti in cui la voce è nominata e le scene narrate dal suo punto di vista. Segna anche quei nomi nel testo stesso.
margin-lane-mark-point-of-view = Narrato da questo punto di vista
margin-lane-mark-named-here = Nominato qui
margin-lane-provider-comments-hint = Dove un commento è collegato al testo
margin-lane-provider-search = Risultati della ricerca
margin-lane-provider-search-hint = Tutte le corrispondenze dell'ultima ricerca, ovunque nel documento
margin-lane-provider-boundaries = Inizio di ogni documento
margin-lane-provider-boundaries-hint = Una linea all'inizio di ogni scena in una vista continua, per sapere dove ti trovi
margin-lane-provider-spelling = Ortografia
margin-lane-provider-spelling-hint = Dove il correttore ha segnalato una parola. Disattivato salvo tua richiesta: rappresenta il giudizio di una macchina sul testo
margin-lane-spelling = { $word }, possibile errore ortografico
margin-lane-search-hit = { $text }, corrispondenza { $index } di { $total }
margin-lane-search-current = { $text }, corrispondenza { $index } di { $total }, quella attuale
margin-lane-boundary = Inizio di { $title }
margin-lane-boundary-untitled = un documento senza titolo
settings-page-margin-lane = Segni a margine
settings-desc-margin-lane = La fascia accanto alla barra di scorrimento e ciò che mostra
settings-margin-lane-enabled = Mostra la fascia a margine
settings-margin-lane-enabled-hint = Lo stesso interruttore di Visualizza ▸ Segni a margine.
settings-margin-lane-enabled-more = Non dichiara mai che qualcosa è sbagliato: mostra dove si trovano le cose e lascia a te l'interpretazione.
settings-group-margin-lane-marks = Cosa segnare
settings-margin-lane-no-providers = Nulla segna ancora la fascia.
settings-group-margin-lane-texture = Distribuzione del dialogo
settings-margin-lane-texture = Mostra la distribuzione del dialogo
settings-margin-lane-texture-hint = Una barra per paragrafo: la sua lunghezza e la parte di discorso diretto.
settings-margin-lane-texture-more = Per le lingue senza convenzioni curate non viene misurata: la barra mostra solo la lunghezza.
settings-group-margin-lane-surfaces = Dove appare
settings-margin-lane-surface-editor = Editor di testo
settings-margin-lane-surface-stream = Viste continue
settings-margin-lane-surface-search-preview = Anteprima della ricerca
menu-margin-lane = Segni a &margine
status-preset-drafting = Stesura
status-preset-passes = Passaggi di revisione
status-preset-plume = Plume Creator
status-todo = Da fare
status-draft = Bozza
status-revised = Rivisto
status-final = Definitivo
status-outline = Scaletta
status-first-edit = 1ª revisione
status-second-edit = 2ª revisione
status-done = Completato
status-plume-draft-1 = 1ª bozza
status-plume-draft-2 = 2ª bozza
status-plume-draft-3 = 3ª bozza
status-plume-edit-1 = 1ª revisione
status-plume-edit-2 = 2ª revisione
status-plume-edit-3 = 3ª revisione
status-plume-proofread = Corretto
status-plume-finished = Terminato
new-work-statuses = Flusso di lavoro
new-work-statuses-hint = Le fasi attraversate da una scena. Puoi rinominarle, riordinarle o aggiungerne altre in qualsiasi momento.
status-none = Nessuno stato
inspector-status = Stato
overview-col-status = Stato
overview-status-mixed = gli elementi sottostanti hanno stati diversi
help-concept-status = Stato
status-completion-title = A che punto è il libro
status-completion-headline = { $done } di { $total } scene terminate
status-completion-empty = Nessuna scena. Questa vista legge il manoscritto e si riempie mentre scrivi.
status-completion-open = A che punto è il libro…
settings-page-statuses = Stati
settings-desc-statuses = Rinomina, riordina, aggiungi e rimuovi le fasi attraversate da una scena
settings-statuses-add = Aggiungi stato
settings-statuses-add-placeholder = Nome del nuovo stato
settings-statuses-desc =
    Le fasi attraversate da una scena, ordinate dalla meno completa in alto alla più
    completa in basso. Questo ordine definisce cosa significa «meno completo di»
    nel resto dell'applicazione: disponile secondo il tuo vero processo di lavoro.
settings-statuses-apply-preset = Applica una configurazione…
settings-statuses-preset-applied = { $added ->
    [one] 1 stato aggiunto
   *[other] { $added } stati aggiunti
}
settings-statuses-preset-refused = Il progetto ha già una sequenza di stati. Le configurazioni riempiono solo una sequenza vuota. Elimina prima le fasi che non vuoi.
settings-statuses-duplicate = «{ $name }» è già nella sequenza
settings-statuses-added = «{ $name }» aggiunto
settings-statuses-move-up = Sposta in alto (meno completo)
settings-statuses-move-down = Sposta in basso (più completo)
settings-statuses-details-placeholder = Significato della fase (facoltativo)
settings-statuses-delete = Elimina «{ $name }»
settings-statuses-delete-in-use = { $count ->
    [one] Elimina «{ $name }»: 1 elemento lo usa e perderà lo stato
   *[other] Elimina «{ $name }»: { $count } elementi lo usano e perderanno lo stato
}
settings-statuses-deleted = «{ $name }» eliminato
settings-statuses-deleted-in-use = { $count ->
    [one] «{ $name }» eliminato. 1 elemento non ha più uno stato.
   *[other] «{ $name }» eliminato. { $count } elementi non hanno più uno stato.
}
settings-statuses-empty-title = Nessun flusso di lavoro
settings-statuses-empty-body = Uno stato indica a che punto è una scena. Aggiungine uno sopra o parti da una configurazione.
status-category-planned = Pianificato
status-category-drafting = In stesura
status-category-needs-work = Da rivedere
status-category-revised = Rivisto
status-category-final = Definitivo

## Long-operation progress (shared::progress_text). The backend reports phase
## codes it cannot translate; these are their labels. `progress-percent` is the
## toast body on its own, `progress-percent-with-label` with the phase after it.
progress-percent = { $percent }%
progress-percent-with-label = { $percent }% · { $label }
progress-import-opening = Apertura del progetto…
progress-import-outline = Lettura della struttura…
progress-import-converting = Conversione di capitoli e scene…
progress-import-writing = Scrittura del file .skrib…
progress-import-done = Fatto
progress-import-structure = Analisi della struttura…
progress-export-start = Avvio…
progress-export-writing = Scrittura…
progress-project-reading = Lettura del progetto…
# Settings: the open project's section title, "Work: <its title>".
settings-sec-work-titled = Progetto: { $title }

## Project-import notices (shared::import_warnings::notices): one line of the
## Plume / Manuskript warnings list each. $file, $folder, $title, $name, $field
## and $error are data (file names, titles, a parser's own message) and stay as
## they are. $container is "single-file" or "folder"; $kind is "scene", "note",
## "synopsis" or "title".
import-notice-flattened-rich-texts = { $count ->
    [one] Un testo era annidato più in profondità di quanto un progetto Skribisto possa contenere; le sue parole sono state conservate come testo semplice, senza formattazione.
   *[other] { $count } testi erano annidati più in profondità di quanto un progetto Skribisto possa contenere; le loro parole sono state conservate come testo semplice, senza formattazione.
}
import-notice-free-text-badges = { $count ->
    [one] Un badge di scena sembra testo libero più che un elenco di stati, quindi è stato conservato come dicitura dell'elemento invece di diventare un'etichetta.
   *[other] { $count } badge di scena diversi sembrano testo libero più che un elenco di stati, quindi sono stati conservati come diciture degli elementi invece di diventare etichette.
}
import-notice-separator-text-dropped = Il separatore «{ $name }» conteneva del testo, che in un progetto Skribisto non trova posto; il testo non è stato importato.
import-notice-separator-without-scene = Il separatore «{ $name }» non aveva una scena prima di sé nel suo capitolo a cui agganciarsi, quindi non è stato importato.
import-notice-chapter-without-scenes = Il capitolo «{ $name }» non ha scene, quindi i suoi separatori non sono stati importati.
import-notice-rich-text-unconverted = Non è stato possibile convertire parte del testo formattato ({ $error }); non è stato importato.
import-notice-unresolved-cross-link = Non è stato possibile seguire un collegamento alla scheda personaggio n. { $number }: la scheda manca o era nel cestino.
import-notice-read-copy = È stata letta la copia { $container ->
    [single-file] a file singolo
   *[folder] in cartella
} di questo progetto.
import-notice-read-copy-dated = È stata letta la copia { $container ->
    [single-file] a file singolo
   *[folder] in cartella
} di questo progetto, modificata l'ultima volta il { $day }.
import-notice-original-format = Questo progetto è nel formato originale di Manuskript del 2016. È stato letto per intero; nulla al suo interno è stato modificato.
import-notice-newer-single-file-beside = «{ $file }» è più recente di questa cartella ({ $file_day } contro { $folder_day }). Quando un progetto passa alla modalità a file singolo, Manuskript lascia la vecchia cartella dov'è, quindi questa cartella potrebbe essere una copia abbandonata; importando invece il file .msk leggeresti quella più recente.
import-notice-pickle-skipped = «{ $file }» non è stato letto. È un file pickle di Python, e caricarne uno da un file di progetto era una falla di sicurezza dello stesso Manuskript (CVE-2021-35196); le impostazioni che contiene riguardano la visualizzazione, non la scrittura.
import-notice-file-unreadable = Non è stato possibile leggere «{ $file }» ({ $error }), quindi è stato saltato; tutto il resto è stato importato.
import-notice-not-utf8 = «{ $file }» non è UTF-8 valido; i caratteri illeggibili sono stati sostituiti invece di interrompere l'importazione.
import-notice-table-unreadable = Non è stato possibile leggere «{ $file }» ({ $error }); il suo contenuto non è stato importato.
import-notice-outline-missing = «{ $file }» manca, quindi questo progetto non ha un manoscritto da importare.
import-notice-outline-unreadable = Non è stato possibile leggere «{ $file }» ({ $error }); nessun manoscritto è stato importato.
import-notice-named-none-row = «{ $file }» inizia con una riga con un nome («{ $name }») dove Manuskript tiene la sua voce vuota «none». È stata esclusa, così le righe successive mantengono i numeri usati dal manoscritto.
import-notice-unknown-field = «{ $file }» ha un campo che questo importatore non conosce, «{ $field }». Non è stato importato.
import-notice-colorless-labels = { $count ->
    [one] «{ $file }» ha una riga senza colore. È stata importata senza colore; Manuskript interrompe il caricamento del progetto su una riga del genere.
   *[other] «{ $file }» ha { $count } righe senza colore. Sono state importate senza colore; Manuskript interrompe il caricamento del progetto su una riga del genere.
}
import-notice-settings-unreadable = Non è stato possibile leggere «{ $file }» ({ $error }); la lingua del progetto non è stata importata.
import-notice-revisions-unreadable = Non è stato possibile leggere «{ $file }» ({ $error }); la cronologia del progetto non è stata importata, ma il resto non ne ha risentito.
import-notice-foreign-files = { $count ->
    [one] Questo progetto contiene un file che non fa parte del formato Manuskript e non è stato importato: { $files }.
   *[other] Questo progetto contiene { $count } file che non fanno parte del formato Manuskript e non sono stati importati: { $files }.
}
import-notice-foreign-files-more = Questo progetto contiene { $count } file che non fanno parte del formato Manuskript e non sono stati importati: { $files } e { $more ->
    [one] un altro
   *[other] altri { $more }
}.
import-notice-not-a-character-file = «{ $file }» non è un file personaggio di Manuskript ed è stato escluso.
import-notice-not-a-text-item = «{ $file }» non è un elemento di testo di Manuskript ed è stato escluso dalla struttura.
import-notice-missing-blank-line = «{ $file }» non ha una riga vuota dopo l'intestazione. Il suo ultimo campo è stato conservato, anche se Manuskript lo perde.
import-notice-folder-without-metadata = «{ $folder }» non contiene { $file }, quindi è stata importata come semplice cartella con il nome della directory. Manuskript salta una directory del genere e tutto ciò che contiene.
import-notice-rows-given-ids = { $count ->
    [one] Un elemento della struttura non aveva un proprio ID e ne ha ricevuto uno. Manuskript scarta un elemento del genere, o interrompe il caricamento del progetto se è una cartella.
   *[other] { $count } elementi della struttura non avevano un proprio ID e ne hanno ricevuto uno. Manuskript scarta un elemento del genere, o interrompe il caricamento del progetto se è una cartella.
}
import-notice-plots-unreadable = Non è stato possibile leggere «{ $file }» ({ $error }); le trame non sono state importate.
import-notice-world-unreadable = Non è stato possibile leggere «{ $file }» ({ $error }); le voci del mondo non sono state importate.
import-notice-world-not-opml = «{ $file }» non è un documento OPML; le voci del mondo non sono state importate.
import-notice-text-flattened = Il testo di «{ $title }» è annidato più in profondità di quanto un progetto Skribisto possa contenere; le sue parole sono state conservate come testo semplice, senza formattazione.
import-notice-text-unconverted = Non è stato possibile convertire il testo di «{ $title }» ({ $error }); è stato conservato esattamente com'era scritto.
import-notice-text-without-file-name = Non è stato possibile assegnare un nome di file { $kind ->
    [scene] al testo della scena
    [note] al testo della nota
    [synopsis] alla sinossi
   *[title] al titolo
} di «{ $title }», quindi questo contenuto non è stato importato.
import-notice-unknown-label = «{ $title }» ha la dicitura { $index }, che non è nell'elenco delle diciture di questo progetto. È stato importato senza dicitura.
import-notice-unknown-status = «{ $title }» ha lo stato { $index }, che non è nell'elenco degli stati di questo progetto. È stato importato senza stato.
import-notice-unknown-point-of-view = «{ $title }» è raccontato da un personaggio che non è tra i personaggi di questo progetto. Il punto di vista non è stato impostato.
import-notice-unresolved-references = Non è stato possibile risolvere i riferimenti nel testo di «{ $title }» ({ $error }); il testo è stato conservato così come è stato convertito, segni di riferimento compresi.
import-notice-custom-icons = { $count ->
    [one] Un elemento aveva un'icona personalizzata. Skribisto dà a ogni elemento l'icona del suo tipo, quindi l'icona non è stata importata; per il resto l'elemento non è cambiato.
   *[other] { $count } elementi avevano un'icona personalizzata. Skribisto dà a ogni elemento l'icona del suo tipo, quindi le icone non sono state importate; per il resto quegli elementi non sono cambiati.
}
import-notice-character-colors = { $count ->
    [one] Un personaggio aveva un colore accanto al nome. Skribisto colora le etichette, non gli elementi, quindi il colore non è stato importato; il personaggio sì.
   *[other] { $count } personaggi avevano un colore accanto al nome. Skribisto colora le etichette, non gli elementi, quindi i colori non sono stati importati; i personaggi invece sì, tutti.
}
import-notice-versions-kept = { $found ->
    [one] Questo progetto aveva registrato una versione precedente
   *[other] Questo progetto aveva registrato { $found } versioni precedenti
}; { $kept ->
    [one] ne è stata conservata una
   *[other] ne sono state conservate { $kept }
}, diradate come Skribisto dirada la propria cronologia. Si trovano nel pannello Versioni.
import-notice-versions-unplaced = { $count ->
    [one] Una versione precedente appartiene a un elemento che non è più nel progetto e non è stato possibile collocarla.
   *[other] { $count } versioni precedenti appartengono a un elemento che non è più nel progetto e non è stato possibile collocarle.
}

## Native file dialogs: the window title, and the label of the file-type filter.
dialog-open-work-title = Apri un progetto Skribisto
dialog-save-as-file-title = Salva come singolo file .skrib
dialog-save-as-folder-title = Scegli la cartella in cui salvare il progetto
dialog-manuskript-file-title = Scegli un progetto Manuskript
dialog-manuskript-folder-title = Scegli la cartella di un progetto Manuskript
dialog-plume-file-title = Scegli un progetto Plume Creator
dialog-filter-skrib = Progetto Skribisto
dialog-filter-manuskript = Progetto Manuskript
dialog-filter-plume = Progetto Plume Creator
dialog-filter-hunspell-aff = File degli affissi Hunspell (.aff)
dialog-filter-hunspell-dic = Dizionario Hunspell (.dic)
dialog-filter-export = File esportato
# Help ▸ Keyboard shortcuts / Settings ▸ Keymap / command palette.
shortcut-name-view-margin-lane = Mostra o nascondi segni a margine

## The built-in distraction-free themes' names (distraction_free::theme).
df-theme-paper = Carta
df-theme-sepia = Seppia
df-theme-night = Notte
df-theme-high-contrast = Contrasto elevato
df-theme-midnight = Mezzanotte

## Welcome ▸ Examples: the line under a bundled example's title.
welcome-example-byline = di { $author }

## Spoken by a screen reader only.
# Welcome window: the name of the section tab bar.
welcome-sections-a11y = Sezioni della finestra di benvenuto
# A comment's anchored span in the editor: author, plain-text preview, replies.
comments-a11y-summary-author = { $author }: { $text }
comments-a11y-summary-replies = { $text } ({ $count ->
    [one] 1 risposta
   *[other] { $count } risposte
})
comments-a11y-summary-author-replies = { $author }: { $text } ({ $count ->
    [one] 1 risposta
   *[other] { $count } risposte
})
# Margin marks beside the text: one mark, several merged into one, and the
# lane's own name when nothing more specific was given.
margin-lane-a11y-mark = { $label }, al { $percent } per cento del testo
margin-lane-a11y-marks = { $label }, { $count } segni, al { $percent } per cento del testo
margin-lane-a11y-fallback = { $count ->
    [one] Segni a margine, 1 segno
   *[other] Segni a margine, { $count } segni
}
# Export ▸ Choose: the row standing for a binder that has no name.
export-choose-unnamed-binder = Raccoglitore senza nome
# The built-in export styles named in words (the national manuscript presets
# keep their own language's name).
export-style-neutral = Neutro
export-style-manuscript-shunn = Manoscritto standard (Shunn)
export-style-ebook-clean = E-book essenziale

## Dates. Put the parts in the order your language writes a short date; the
## parts arrive already zero-padded where that matters.
# Comment cards, the outline card's Created/Modified.
date-time-short = { $day }/{ $month }/{ $year } { $hour }:{ $minute }
# Pace chart: a day on the axis, a milestone's date, a holiday.
pace-day-label = { $day }/{ $month }

## File sizes (shared::byte_size). $whole and $tenth are the two sides of the
## decimal mark: write your language's mark between them and its unit symbol.
byte-size-bytes = { $value } B
byte-size-kb = { $value } KB
byte-size-mb = { $whole },{ $tenth } MB
byte-size-gb = { $whole },{ $tenth } GB
# A whole percentage (sliders, the Pace card, the text-size toast).
unit-percent = { $value }%
# A whole number of pixels (slider readouts).
unit-pixels = { $value } px
# Settings ▸ Paratext: a preset saved without a `name` line. `name` is the TOML
# key the writer types and stays as it is.
settings-paratext-error-no-name = Una struttura deve avere una riga `name`.
# Export: started with no project open (a menu raced a close).
export-error-no-project = Nessun progetto aperto da esportare.

shortcut-group-work = Progetto
shortcut-group-edit = Modifica
shortcut-group-view = Visualizza
shortcut-group-document = Documento
shortcut-group-format = Formato
shortcut-group-go = Vai
shortcut-group-tools = Strumenti
shortcut-group-help = Aiuto

## Manuskript import: the headings of the notes it writes and the title of a project with no name.
import-manuskript-section-paragraph-summary = Riassunto in un paragrafo
import-manuskript-section-full-summary = Riassunto completo
import-manuskript-section-motivation = Motivazione
import-manuskript-section-epiphany = Epifania
import-manuskript-section-notes = Note
import-manuskript-section-description = Descrizione
import-manuskript-section-passion = Passione
import-manuskript-section-result = Risultato
import-manuskript-section-summary = Riassunto
import-manuskript-section-meta = Metadati
import-manuskript-section-series = Serie
import-manuskript-section-volume = Volume
import-manuskript-section-genre = Genere
import-manuskript-section-license = Licenza
import-manuskript-section-email = Email
import-manuskript-section-situation = Situazione
import-manuskript-section-sentence = Una frase
import-manuskript-section-paragraph = Un paragrafo
import-manuskript-section-page = Una pagina
import-manuskript-section-full = Riassunto completo
import-manuskript-untitled-project = Progetto Manuskript importato

## Plume import: the story-bible groups an old project implies, and the title of a project with no name.
import-plume-group-characters = Personaggi
import-plume-group-items = Oggetti
import-plume-group-places = Luoghi
import-plume-untitled-project = Progetto Plume importato
