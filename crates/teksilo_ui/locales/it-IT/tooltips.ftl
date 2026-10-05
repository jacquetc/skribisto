# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

wm-book = Un libro nel progetto.
wm-book-more = Un libro può essere semplice o articolato quanto desideri. Un racconto può consistere in poche [scene](:wm-scene). Un'opera più lunga può contenere [parti](:wm-part) e [capitoli](:wm-chapter) annidati a piacere. La struttura la scegli tu.
wm-part = Una parte che raggruppa capitoli.
wm-part-more = Una parte raccoglie [capitoli](:wm-chapter) sotto un unico titolo. Come un capitolo, si estende fino alla parte successiva o alla fine del [libro](:wm-book).
wm-chapter = Un capitolo del libro.
wm-chapter-more = Un capitolo si estende da qui al capitolo successivo, alla [parte](:wm-part) successiva o alla [fine del libro](:wm-end-of-book). Può essere un capitolo cartella che contiene [scene](:wm-scene) oppure un capitolo singolo con il proprio testo. Scegli il tipo dei nuovi capitoli in Impostazioni, Struttura.
wm-scene = Una scena narrativa.
wm-scene-more = Una scena contiene ciò che scrivi. Ogni elemento di scrittura ha due lati: il testo principale e una [sinossi](:wm-synopsis) per pianificarlo. Entrambi si aprono insieme nell'editor.
wm-note = Una nota.
wm-note-more = Una nota conserva testo e [sinossi](:wm-synopsis), come una [scena](:wm-scene), per ricerche, digressioni o promemoria. Qualsiasi elemento può far parte del libro esportato, anche una nota dentro un [capitolo](:wm-chapter).
wm-note-folder = Una nota che contiene altri elementi.
wm-note-folder-more = Una [nota](:wm-note) che contiene anche elementi subordinati, per raggruppare note correlate. Conserva la propria [sinossi](:wm-synopsis), mentre gli elementi contenuti mantengono il proprio testo.
wm-folder = Una cartella organizzativa.
wm-folder-more = Un contenitore per disporre gli elementi nel raccoglitore come preferisci. Ha una [sinossi](:wm-synopsis), ma nessun testo proprio. La scrittura appartiene agli elementi contenuti.
wm-end-of-book = Indica dove termina un libro.
wm-end-of-book-more = Tutti i libri condividono un unico elenco continuo, quindi la fine di un libro non dipende dalla nidificazione. Questo indicatore stabilisce che il [libro](:wm-book) termina qui. Tutto ciò che segue appartiene al libro successivo.
wm-story-bible-entry = Una nota sulla storia, con etichette e altri nomi.
wm-story-bible-entry-more = Una [nota](:wm-note) su un personaggio, un luogo o qualsiasi cosa da annotare, creata in un solo passaggio con nome, etichette, altri nomi e modello iniziale. In seguito rimane una normale nota: puoi rinominarla, cambiarne le etichette o convertirla come qualsiasi altra.
wm-synopsis = Il riassunto di un elemento di scrittura.
wm-synopsis-more = Un riassunto associato a qualsiasi elemento di scrittura, di solito un breve paragrafo, ma lungo quanto vuoi. Usalo per pianificare e orientarti prima di scrivere il testo. Nell'editor appare accanto al testo principale.
scene-break-minor = Uno stacco di scena ordinario: un cambio di tempo, luogo o punto di vista all'interno di un capitolo.
scene-break-minor-more =
    Segna uno stacco *dove lo inserisci*, anche nel mezzo di una [scena](:wm-scene). Dividere il testo in due elementi è una scelta organizzativa e non crea da sola uno stacco.

    Si digita come tre asterischi separati da spazi. L'aspetto stampato dipende dallo stile di esportazione: un manoscritto Shunn usa un solo #, un'edizione tascabile un ornamento tipografico, mentre molte edizioni francesi, tedesche, spagnole, russe e italiane usano soltanto uno spazio vuoto.
scene-break-major = Una divisione più forte: un grande salto temporale o un cambio decisivo di punto di vista.
scene-break-major-more =
    Lo stesso principio di uno stacco ordinario, ma più marcato. Usalo quando uno stacco semplice non renderebbe l'entità del salto.

    Si digita come # # # e lo stile di esportazione lo distingue dallo stacco ordinario. Il formato standard Shunn contrappone un solo # a # # # proprio per questo. Dove non esiste un segno più forte, i due livelli vengono stampati allo stesso modo.
wm-find-in-prose = Skribisto cerca questo elemento nel testo.
wm-find-in-prose-more = Attiva questa opzione per le etichette che indicano ciò di cui scrivi: personaggi, luoghi, oggetti. Il titolo e gli altri nomi di ogni elemento con tale etichetta vengono cercati nel testo, così ogni [scena](:wm-scene) elenca chi e cosa vi compare senza collegamenti manuali. Lasciala disattivata per etichette descrittive, come lo stato di una bozza o un controllo di continuità. Ogni elemento individuato entra anche nella [bibbia narrativa](:wm-story-bible).
wm-story-bible = Tutti i personaggi, i luoghi e le altre voci etichettate, raccolti in una griglia.
wm-story-bible-more = Si apre da qualsiasi cartella di note, nella sua scheda dell'editor. Le voci sono raggruppate per etichetta e ognuna indica quanti altri nomi ha e in quante scene compare, in base a tutte le etichette [Cerca nel testo](:wm-find-in-prose) del progetto. Il filtro per libro limita soltanto le voci visibili, non decide quali voci esistono.
tooltip-go-to = Vai a qualsiasi elemento del raccoglitore (Ctrl+G)
synopsis-collapse-tooltip = Nascondi la colonna della sinossi
wm-paratext = Un testo che non fa parte della storia.
wm-paratext-more = Una prefazione, una dedica, una postfazione, un colophon: testi che appartengono al libro ma non al racconto. Vengono esportati dove li inserisci e non entrano mai nel conteggio delle parole del manoscritto. La posizione la decidi tu: le convenzioni cambiano secondo il paese e l'editore.
wm-paratext-folder = Una cartella per i paratesti.
wm-paratext-folder-more = Un posto per prefazioni e postfazioni, per non affollare il raccoglitore. Serve solo a organizzare: ha una [sinossi](:wm-synopsis), ma non aggiunge nulla al libro esportato, nemmeno il proprio nome.
goal-target = La lunghezza prevista per questo testo.
goal-target-more = Imposta un obiettivo su una [scena](:wm-scene), un [capitolo](:wm-chapter), una [parte](:wm-part) o l'intero [libro](:wm-book). Ogni valore è indipendente: cambiare quello di un capitolo non cambia quello del libro e l'obiettivo del libro non è la somma dei capitoli. Si somma invece il testo: l'[avanzamento](:goal-progress) di un contenitore comprende tutto ciò che è scritto al suo interno. Zero significa nessun obiettivo. Il numero è espresso in [parole o caratteri](:goal-unit), secondo l'unità del progetto.
goal-unit = Se questo progetto conta parole o caratteri.
goal-unit-more = Una scelta per l'intero progetto: l'unità dipende dal manoscritto e dal suo mercato, non da una sezione. L'editoria tedesca e francese misura in battute, quella giapponese in fogli da 400 caratteri, quella cinese in migliaia di caratteri e gran parte di quella inglese in parole. I due conteggi restano separati e non vengono convertiti. Cambiare unità fa riferimento all'altro conteggio per ogni [obiettivo](:goal-target); tornando indietro si ripristina il primo. Gli obiettivi già inseriti vanno quindi aggiornati a mano.
goal-progress = Quanto è stato scritto rispetto all'obiettivo.
goal-progress-explained = Un elemento con testo proprio viene misurato sulla propria lunghezza; un contenitore su [tutto ciò che contiene](:goal-manuscript-words). La barra passa dal rosso all'ambra al verde man mano che scrivi e cambia ancora quando superi di molto il previsto: utile quando un capitolo deve rispettare una lunghezza.
goal-manuscript-words = Cosa viene conteggiato nel manoscritto.
goal-manuscript-words-more = Solo il testo effettivamente esportabile: [scene](:wm-scene) e [capitoli](:wm-chapter) con testo proprio. Una [nota](:wm-note) o un [paratesto](:wm-paratext) non fanno parte del manoscritto e non vengono conteggiati. Sono esclusi anche gli elementi nel cestino o [esclusi dall'esportazione](:goal-exportable).
goal-exportable = Questo elemento è escluso dall'esportazione.
goal-exportable-more = La sua lunghezza rimane visibile perché il testo esiste ancora, ma non contribuisce ad alcun totale, così come non apparirà nel libro esportato. Escludere un contenitore non esclude i suoi elementi: ogni elemento è indipendente. Il pulsante «Applica agli elementi contenuti» accanto all'interruttore serve proprio a questo.
goal-distribute = Ripartisci questo obiettivo tra gli elementi contenuti.
goal-distribute-more = Divide l'obiettivo di un [libro](:wm-book) o di una [parte](:wm-part) tra i figli diretti, in modo che la loro somma corrisponda esattamente. Per impostazione predefinita compila solo quelli senza obiettivo, lasciando intatti i tuoi valori. Sono normali [obiettivi](:goal-target), modificabili in seguito: è un'azione singola, non un vincolo permanente.
goal-subtree-total = La somma degli obiettivi contenuti.
goal-subtree-total-more = Un dato, non un obiettivo. L'obiettivo di un contenitore non è mai trattato come la somma di quelli al suo interno, altrimenti cambierebbe da solo assegnando un valore a una scena. Questa riga mostra semplicemente la somma dei tuoi numeri, da confrontare con l'[obiettivo](:goal-target) del contenitore o da usare per [ripartirne uno](:goal-distribute).
goal-milestone = Una data entro cui raggiungere un risultato.
goal-milestone-more = Ne esistono due tipi. Il primo fissa una lunghezza del [libro](:wm-book) entro una data, come tappa del suo percorso. Il secondo richiede che un [capitolo](:wm-chapter) o una [parte](:wm-part) raggiunga il [proprio obiettivo](:goal-target) entro quella data, leggendone il valore attuale: modificarlo aggiorna anche il traguardo. Entrambi appaiono nel [piano del ritmo](:pace-plan) del libro.
pace-plan = Il calendario di scrittura del libro.
pace-plan-more = Un [obiettivo](:goal-target), una scadenza e i giorni in cui scrivi determinano quanto serve scrivere al giorno. Usa l'obiettivo del libro: impostarlo qui o nelle Proprietà cambia lo stesso valore. Le tappe intermedie sono i [traguardi](:goal-milestone).
concept-tag = Un'etichetta colorata da assegnare a qualsiasi elemento, riutilizzabile in tutto il progetto.
concept-tag-more = Contrassegna un elemento con uno stato, un luogo o un filo narrativo da seguire. La stessa etichetta può appartenere a più elementi e un elemento può averne diverse. Attiva [Cerca nel testo](:wm-find-in-prose) per un'etichetta che indica un personaggio o un luogo del libro: Skribisto inizierà a cercarlo nel testo.
concept-label = Una breve annotazione sotto il titolo di un elemento, solo per te.
concept-label-more = Diversamente da un'[etichetta](:concept-tag), una dicitura appartiene a un solo elemento: semplice testo senza colore o catalogo, come «primo snodo narrativo» o «serve un nome». Impostala dal menu contestuale o nella colonna Dicitura della panoramica; appare come sottotitolo sotto il titolo, sia nella struttura sia nelle viste continue.
concept-point-of-view = Attraverso gli occhi di chi viene narrata una scena: uno o più personaggi della storia.
concept-point-of-view-more = Si imposta nelle Proprietà, accanto ai personaggi presenti. Scegliere qualcuno non ancora presente lo aggiunge anche a quell'elenco. Sono ammessi più punti di vista, utili per una scena condivisa o un cambio di prospettiva. I candidati provengono dalle voci inserite nella bibbia narrativa da un'etichetta [Cerca nel testo](:wm-find-in-prose).
concept-epigraph = Una citazione all'inizio di una parte o di un capitolo.
concept-epigraph-more = Un campo separato dal testo del [capitolo](:wm-chapter) o della [parte](:wm-part); una [scena](:wm-scene) o una [nota](:wm-note) non ne hanno. Scrivila come una normale citazione; se ne scrivi diverse, ciascuna diventa un blocco e un'epigrafe distinta. Le sue parole non entrano nel conteggio del manoscritto.
concept-footnote = Un riferimento nel testo, stampato come nota numerata.
concept-footnote-more = Il riferimento è un singolo segno nel testo e si sposta o si elimina come le parole circostanti. Il numero non viene memorizzato: Skribisto lo ricalcola dalla posizione dei riferimenti nell'intero manoscritto, quindi inserire una nota rinumera quelle successive. Le parole delle note sono conteggiate separatamente dal manoscritto.
concept-chapter-mode = Se un nuovo capitolo è una cartella di scene o un singolo elemento.
concept-chapter-mode-more = Si imposta in Impostazioni, Struttura. In entrambi i casi scrivi direttamente nel [capitolo](:wm-chapter); cambia soltanto la possibilità di contenere scene. Entrambe le forme producono lo stesso libro e possono convivere nel progetto. «Converti in» passa dall'una all'altra senza perdere una parola.
concept-comment = Una nota ancorata a un passaggio preciso, mostrata nel margine.
concept-comment-more = Ricorda le parole citate e il contesto, non una posizione fissa: resiste alle normali modifiche e al ricaricamento. Rispondi sotto per continuare la discussione. Se la citazione non si trova più, il commento lo segnala invece di spostarsi silenziosamente sulla frase sbagliata.
concept-backup = Una copia dell'intero progetto in un dato momento, conservata accanto al progetto.
concept-backup-more = Creata secondo il programma che scegli: all'apertura, alla chiusura, a intervalli o con una combinazione, eliminando le vecchie copie per età o numero. Aprirla carica una finestra separata: puoi consultarla e modificarla, ma soltanto Salva con nome o Ripristina conserva i cambiamenti. Diversa da una [versione](:concept-version), che segue la storia di un singolo elemento.
concept-version = Il passato di un elemento: ogni modifica al testo o alla sinossi.
concept-version-more = Ricostruito dalla cronologia dei salvataggi e dai backup del progetto, con una voce per ogni modifica anziché per file. Scegli una voce per vedere i cambiamenti e ripristinare solo il testo di quell'elemento; il resto del progetto rimane intatto e viene prima creata una copia di sicurezza. Diversa da un [backup](:concept-backup), che copia tutto.
concept-trash = Un elemento nel cestino è nascosto, non spostato: rimane esattamente dov'era.
concept-trash-more = Il cestino cambia soltanto un indicatore: l'elemento conserva il posto nel raccoglitore, contrassegnato come inattivo. Ripristinandolo riappare nello stesso punto. Svuotare il cestino rimuove gli elementi definitivamente; ciascuna di queste azioni è annullabile come le altre modifiche al progetto.
concept-spellcheck = Sottolinea le parole non riconosciute dai dizionari installati, secondo la lingua del progetto.
concept-spellcheck-more = Ogni progetto ha una o più lingue di lavoro; un elemento può sostituirle per una scena in un'altra lingua. Dal menu contestuale dell'editor puoi aggiungere parole sconosciute al dizionario personale, condiviso fra i progetti. Disattivare il controllo ortografico lo interrompe ovunque finché non lo riattivi.
concept-search-replace = Cerca una parola o una frase in tutto il progetto, non solo nel documento aperto.
concept-search-replace-more = Cerca in scene, note, titoli, sinossi, diciture, commenti e note a piè di pagina. Maiuscole, parole intere e accenti sono tre opzioni separate. Sostituisci tutto modifica tutte le corrispondenze esaminate in un solo passaggio annullabile: Ctrl+Z annulla l'intero gruppo.
concept-note-template = Un testo riutilizzabile da inserire in ciò che stai scrivendo.
concept-note-template-more = Una scheda personaggio vuota, la descrizione di un luogo, una scaletta narrativa: scrivila una volta e inseriscila dove serve, in qualsiasi elemento. Parti da un modello integrato, importa un file .md o .djot oppure scrivi un testo e scegli Documento ▸ Salva come modello. I modelli sono nel progetto, così anche un coautore dispone degli stessi.
concept-text-replacement = Abbreviazioni personali espanse automaticamente mentre scrivi.
concept-text-replacement-more = Imposta un'abbreviazione e la sostituzione, per esempio «cmq» per «comunque»: si espande appena digiti uno spazio o un segno di punteggiatura. Le maiuscole seguono ciò che hai digitato. Backspace subito dopo annulla soltanto quella sostituzione. Si attiva per progetto ed è distinta dalla [punteggiatura intelligente](:concept-smart-punctuation).
concept-smart-punctuation = Tipografia adatta alla lingua, applicata automaticamente: virgolette, trattini, puntini di sospensione.
concept-smart-punctuation-more = Trasforma le virgolette dritte in tipografiche, -- in una lineetta media, --- in una lineetta lunga e ... in puntini di sospensione. Se attivato, un paragrafo iniziato con «- » diventa un dialogo con lineetta. Si imposta per progetto e viaggia nel file .skrib, così un coautore usa le stesse regole. Distinta dalle [regole personali](:concept-text-replacement), scritte da te.
concept-export-style = Un insieme di opzioni di esportazione con un nome: titoli, spaziatura, contenuti inclusi.
concept-export-style-more = Uno stile riutilizzato in ogni esportazione, qualunque sia il formato. Gli stili integrati sono di sola lettura: duplicane uno per modificarlo. Fra le opzioni ci sono i [marcatori di reimportazione](:concept-round-trip-marks), per i file da inviare a un revisore e poi reimportare.
concept-round-trip-marks = Segnalibri invisibili che permettono di riconoscere un DOCX o ODT dopo la revisione.
concept-round-trip-marks-more = Sono scritti solo nei due formati che un revisore può restituire, DOCX e ODT; nessun'altra esportazione li contiene. Sono segnalibri, non attributi personalizzati: LibreOffice elimina questi ultimi al salvataggio ma conserva i segnalibri. Si attivano nello [stile di esportazione](:concept-export-style), sotto «Invio a un revisore».
concept-status = La fase di un elemento nel tuo flusso di lavoro: uno stato alla volta, procedendo in avanti.
concept-status-more = Diversamente da un'[etichetta](:concept-tag), ogni elemento ha un solo stato. Le fasi sono ordinate e l'elenco appartiene al progetto: puoi rinominarle, riordinarle o aggiungerne. Un asterisco accanto a un capitolo o libro indica che gli elementi contenuti non hanno tutti lo stesso stato. Impostalo dalle Proprietà, da una scheda della bacheca, da un elemento di una vista continua o da qui.
