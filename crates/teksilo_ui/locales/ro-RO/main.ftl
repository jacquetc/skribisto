# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

menu-work = Proiect (&W)
menu-new-work = Proiect nou (&N)
menu-open-work = Deschidere proiect… (&O)
menu-new-window = Fereastră nouă (&W)
menu-import-from = Importă din (&I)
menu-create-from = Creează din (&C)
menu-import-plume = Plume Creator (.plume)… (&P)
menu-import-document = Documente (Markdown, Word, ODT)... (&D)
menu-import-manuskript = Manuskript (.msk)… (&M)
menu-export = Exportare (&X)
menu-export-book = Cartea de export
menu-export-part = Partea de export
menu-export-chapter = Capitolul privind exportul
menu-export-scene = Exportă scena
menu-export-note = Notă de export
menu-export-paratext = Exportă paratext
menu-export-folder = Exportă dosarul
menu-export-choose = Alege...
menu-export-none = Deschide un document de export
menu-save = Salvare (&S)
menu-save-as-file = Salvare ca fișier unic… (&F)
menu-save-as-folder = Salvare ca dosar… (&D)
menu-backup = Copie de siguranță acum (&B)
menu-close-work = Închidere proiect (&C)
menu-welcome = Bine ai venit... (&L)
menu-settings = Setări (&E)
menu-quit = Ieșire (&Q)

menu-view = Vizualizare (&V)
menu-outline = Schiță (&O)
menu-search = Caută în proiect (&S)
menu-timeline = Du-te înapoi în timp (&G)
menu-search-preview = Previzualizare căutare (&P)
menu-fullscreen = Pe tot ecranul (&F)
menu-focus-mode = Mod fără distragere a atenţiei (&D)
menu-format = Formatare (&O)
menu-scene-break = Inserează pauza de scenă (&K)
menu-major-scene-break = Se introduce pauza de scenă majoră (&M)

menu-go = Navigare (&G)
menu-go-next-scene = Scena următoare (&S)
menu-go-prev-scene = Scena anterioară (&V)
menu-go-next-chapter = Capitolul următor (&C)
menu-go-prev-chapter = Capitolul anterior (&H)
menu-go-next-note = Nota următoare (&N)
menu-go-prev-note = Nota anterioară (&T)

menu-tools = Instrumente (&T)
menu-help = Ajutor (&H)

ctx-add = Adăugare (&A)
ctx-new-item = Post nou (&N)
ctx-new-folder = Dosar nou (&F)
ctx-rename = Redenumire (&R)
ctx-duplicate = Duplicare (&D)
ctx-import-here = Importă aici... (&I)
ctx-indent = Indentare (&I)
ctx-outdent = Ieșire (&O)
ctx-trash = Mutare în coș (&T)
ctx-open-to-side = Deschisă în lateral (&S)
ctx-open = Deschidere (&O)
ctx-reveal-in-outline = Dezvăluie în contur (&L)
ctx-move-up = Mutare în sus (&U)
ctx-move-down = Mutare în jos (&N)

ctx-tab-close = Închidere (&C)
ctx-tab-close-others = Închidere alte file (&O)
ctx-tab-close-all = Închidere toate filele (&L)
ctx-tab-open-to-side = Deschisă în lateral (&S)
ctx-tab-open-in-main = Deschide în fereastra principală (&I)
ctx-tab-move-to-side = Mută-te în lateral. (&M)
ctx-tab-move-to-main = Mută la panoul principal (&M)
ctx-tab-move-to-new-window = Mutare într-o fereastră nouă (&N)
ctx-tab-move-window-unsaved = Salvează acest proiect mai întâi: o a doua fereastră se deschide pe un fișier de pe disc
ctx-tab-pin = Pin această filă (&P)
ctx-tab-unpin = Desfă fila (&U)
tab-pinned-tooltip = Pinned: "Închide altele" și "Închide toate" lăsați-l deschis

create-book = Carte
create-part = Parte
create-chapter = Capitol
create-scene = Scenă
create-note = Notă
create-note-folder = Dosarul notelor
create-folder = Dosar
create-paratext = Paratext
create-paratext-folder = Dosar paratext
create-book-end = Sfârşitul cărţii
create-story-bible-entry = Story Bible entry...
type-book-start = Start carte
type-text = Text

new-item-book = Carte nouă
new-item-part = Parte nouă
new-item-chapter = Capitol nou
new-item-note = Notă nouă
new-item-note-folder = Dosar de notă nouă
new-item-folder = Dosar nou
new-item-paratext = Paratext nou
new-item-paratext-folder = Dosar nou paratext
new-item-story-bible-entry = Intrarea în Biblie a unei noi relatări

placement-inside = în interiorul
placement-after = după
placement-after-parent = după părinte
placement-top-level = nivel superior


ctx-promote = Conversie la (&C)
promote-chapter-folder = Dosar de capitol
promote-flat-chapter = Capitol independent
promote-lossy-title = Nicăieri să păstreze textul
promote-lossy-text = Un { $target } nu are unde să le păstreze: { $kinds }. Mutați sau ștergeți textul mai întâi, apoi convertiți.
content-scene-text = Text Scenă
content-note-text = Text notă
content-book-title = Titlul cărții
content-book-subtitle = Subtitrare de carte
content-part-title = Parte titlu
content-chapter-title = Titlul capitolului
content-epigraph-text = Motto
content-paratext-text = Paratext
promote-blocked-title = Capitolul nu este gol
promote-blocked-text = { $count ->
    [one] Acest capitol are încă { $count } produs. Mutați sau gunoi-l înainte de a converti într-un capitol plat.
   *[other] Acest capitol are încă { $count } articole. Mutați sau le gunoi înainte de a converti într-un capitol plat.
}

inspector = Proprietăți
inspector-empty = Deschide un obiect pentru a-l inspecta.
inspector-promote = Conversia la...
statusbar-toggle-outline = Comută liantul
statusbar-toggle-inspector = Comută inspectorul
statusbar-save-unsaved = Modificări nesalvate. Click pentru a salva
statusbar-save-saved = Toate modificările salvate
statusbar-save-autosave = Autosalvare este pornit. Modificările sunt salvate în timp ce scrieți
statusbar-saving = Salvarea...
statusbar-word-count = { $count ->
    [one] { $count } cuvânt
    [few] { $count } cuvinte
   *[other] { $count } de cuvinte
}
statusbar-word-count-tooltip = Cuvinte în scena pe care o editezi
statusbar-word-char-count = { $words ->
    [one] { $words } cuvânt
    [few] { $words } cuvinte
   *[other] { $words } de cuvinte
} · { $chars ->
    [one] { $chars } caracter
    [few] { $chars } caractere
   *[other] { $chars } de caractere
}
session-toggle = Sesiune de scriere: începe sau întrerupe un sprint concentrat
session-configure = Stabilește scopul și termenul sesiunii
session-configure-title = Sesiune de scriere
session-word-goal = Obiectiv de cuvinte
session-time-limit = Termen
session-time-limit-unit = min
session-no-goal = Nici un scop
session-no-limit = Fără limită
session-reset = Resetează sesiunea
session-readout = { $words } cuvinte · { $time }
session-readout-timed = { $words } cuvinte · { $time } rămase
statusbar-focus-exit = Modul fără distragere a atenției
statusbar-focus-synopsis = Rezumat
statusbar-focus-go-prev = Postul anterior (Alt+Up)
statusbar-focus-go-next = Următorul punct (Alt+jos)

rail-settings = Setări

language = Limbă
theme = Temă
english = Engleză
french = Franceză
light = Lumină
dark = Întuneric
settings-text-width = Lăţimea textului
settings-preview-width = Lăţimea previzualizarei de căutare
settings-autosave = Autosalvare pe disc
settings-show-welcome = Arată lansatorul la pornire
settings-show-welcome-tip = Când este oprit, ultimul proiect se redeschide.
settings-check-for-updates = Verificați dacă există versiuni noi
settings-check-for-updates-tip = O dată pe zi, cel mult Skribisto întreabă site-ul web al proiectului care este versiunea curentă. Cererea nu poartă niciun identificator și nici măcar versiunea pe care o executați: compararea are loc pe acest computer.

settings-title = Setări
settings-close = Închidere
settings-search = Configurări căutare
settings-reset = Reinițializează
settings-done = Gata
settings-cancel = Anulare
settings-apply = Aplicare
settings-ok = Bine.
settings-reset-confirm-title = Resetează toate setările în mod implicit?
settings-reset-confirm-body = Restaurează aspectul acestei aplicații, editorul și setările de scriere. Setările proprii ale proiectului, comenzile rapide și stilurile salvate nu sunt afectate. Nu poate fi anulată.
settings-empty-title = Încă nu există setări aici
settings-empty-hint = Această secțiune va câștiga opțiuni într-o actualizare viitoare.

settings-sec-appearance-behaviour = Comportament de aspect
settings-sec-editor = Editor
settings-sec-spelling = Tipărire
settings-sec-backup = Sincronizare de rezervă
settings-sec-compile = Export compil
settings-page-appearance = Aspect
settings-field-image-size-policy = Imagini mari
settings-image-policy-ask = Întreabă de fiecare dată
settings-image-policy-keep = Păstrează originalul
settings-image-policy-downscale = Optimizează
settings-hint-image-size-policy = Ce este de făcut atunci când o imagine inserată este mai mare decât manuscrisul are nevoie.
settings-page-notifications = Notificări
settings-page-scene = Scenă
settings-page-synopsis = Rezumat
settings-page-notes = Note
settings-page-editor-behavior = Comportament editor
settings-page-goals = Scopul Word Count
settings-page-games = Scrierea de jocuri
settings-page-corkboard = Panou de fișe
settings-page-distraction-free = Distragere liberă
settings-page-dictionaries = Dictionare
settings-page-autosave = Autosalvare
settings-page-export = Formate de export
settings-page-paratext = Structuri paratext
settings-page-keymap = KeymapComment
settings-keymap-filter = Scurtături de filtrare

settings-desc-sec-appearance-behaviour = Cum arată aplicaţia însăşi şi ce face când începe.
settings-desc-sec-editor = Suprafaţa de scris: cum arată şi ce face după cum tastezi.
settings-desc-sec-spelling = Verificarea vrăjilor şi dicţionarele din spatele ei.
settings-desc-sec-backup = Cum ajunge proiectul tău pe disc şi ce copii sunt păstrate.
settings-desc-sec-compile = Ceea ce lasă Skribisto, şi în ce formă.
settings-desc-sec-work = Configurări care aparțin acestui proiect și călătoresc în interiorul fișierului său.
settings-desc-sec-extensions = Paginile au contribuit la extensiile instalate aici.
settings-desc-group-typography = O pagină pentru fiecare tip de text: tastatură, mărime, înălțime și spațiu între linii.
settings-desc-appearance = Interfață limbă, temă, dimensiunea textului, și lansator la pornire.
settings-desc-notifications = Fiecare mesaj pe care această sesiune l-a arătat şi acţiunile pe care le puteţi reda.
settings-desc-scene = Cum se stabilește proza scenei: tastatură față, dimensiune, înălțime linie, liniuță și distanță.
settings-desc-synopsis = Modul în care panoul sinopsis este setat, independent de manuscris.
settings-desc-notes = Cum sunt stabilite notiţele, independent de manuscris.
settings-desc-corkboard = Mărimea cardului, ce arată o carte şi cum le aranjează consiliul.
settings-desc-distraction-free = Tipografie, lățime coloană și bandă de control pentru scrierea ecranului complet.
settings-desc-distraction-free-themes = Biblioteca tematică pentru scrierea pe ecran integral: cele expediate şi ale tale.
settings-desc-editor-behavior = Lăţimea textului, poziţia sintopsisului, defilarea maşinii de scris şi evidenţierea îngrijirii.
settings-desc-punctuation = Fiecare proiect îl urmează dacă nu îşi stabileşte propria casă.
settings-desc-goals = Cuvinte și obiective de caracter, și modul în care cuvintele sunt numărate.
settings-desc-games = Constrângeri pe care le setați în timp ce elaborarea, cum ar fi întotdeauna înainte.
settings-desc-spellcheck = Un singur întrerupător care porneşte şi se opreşte peste tot.
settings-desc-dictionaries = Instalați, eliminați și navigați dicționarele de ortografie pe această mașină.
settings-desc-autosave = Dacă editurile sunt scrise pe disc pe cont propriu.
settings-desc-backup = Când copiile sunt luate, unde sunt păstrate, și cât de multe. Fiecare proiect urmează acest lucru, cu excepția cazului în care își stabilește propria.
settings-desc-export = Stilurile prin care se compilează fiecare export: cele expediate şi ale tale.
settings-desc-paratext = Chestiunea din faţă şi din spate cu care poate începe un nou proiect.
settings-desc-keymap = Fiecare scurtătură, şi ceea ce este obligat să.
settings-page-user = Utilizator
settings-desc-user = Cine eşti, pentru comentariile pe care le scrii.
settings-group-identity = Identitate
settings-field-user-name = Numele tău
settings-field-user-name-placeholder = Opțional
settings-field-user-name-hint = Semnezi comentariile și răspunsurile pe care le scrii. Distinct de la numele autorului unui proiect, care este byline-ul cărții și călătorește în interiorul fișierului.
settings-field-user-initials = Iniţialele tale.
settings-field-user-initials-placeholder = Din numele tău
settings-field-user-initials-hint = Ce un procesor de cuvinte arată alături de comentariul dvs. în marja. Lăsați-l gol pentru a utiliza cele afișate, luate de la numele tău.
settings-field-user-hint = Ambele sunt opţionale şi se aplică fiecărui proiect de pe acest computer. Schimbarea acestora semnează comentariile pe care le scrieţi în continuare. Comentariile deja scrise păstrează numele sub care au fost scrise.
settings-desc-author = Numele care merge pe acest proiect.
settings-desc-structure = Fie că capitolele acestui proiect sunt dosare sau elemente unice.
settings-desc-language = Limba proza acestui proiect este verificata impotriva.
settings-desc-work-backup = Politica de rezervă a acestui proiect, sau cea generală.
settings-desc-personal-dictionary = Cuvintele pe care acest proiect le tratează corect.
settings-desc-tags = Etichetele cu coduri de culoare cu care acest proiect îşi marchează produsele.
settings-desc-templates = Șabloane notă puteți introduce în timp ce scrieți în acest proiect.
settings-desc-text-replacements = Scurtături care se extind pe măsură ce tastați în acest proiect.
settings-desc-work-punctuation = Urmează citatele, liniuţele şi spaţiul prozei acestui proiect.

settings-group-typography = Tipografie
settings-group-writing-column = Scriere coloană
settings-group-writing-view = Scriere vizualizare
settings-group-distraction-free-strip = Bandă de control
settings-group-theme = Temă
settings-group-language = Limbă
settings-group-startup = Pornire
settings-group-updates = Actualizări
settings-group-autosave = Autosalvare
settings-field-typeface = Față de tip
settings-field-size = Mărime text

editor-size-changed-manuscript = Mărime text manuscris: { $percent }
editor-size-changed-synopsis = Dimensiunea textului Synopsis: { $percent }
editor-size-changed-notes = Note de text: { $percent }
editor-size-changed-corkboard = Dimensiune card de plută text: { $percent }
editor-size-changed-corkboard-expanded = Mărime editor extins text: { $percent }
editor-size-changed-distraction-free = Dimensiune text fara distragere: { $percent }

menu-text-size-increase = Mărește dimensiunea textului (&I)
menu-text-size-decrease = Diminuează dimensiunea textului (&C)
menu-text-size-reset = Resetează dimensiunea textului (&R)
settings-field-line-height = Înălțimea liniei
settings-field-first-line-indent = Prima liniuţă
settings-field-paragraph-spacing-before = Spațiu înainte de paragraf
settings-field-paragraph-spacing-after = Spațiu după paragraf
settings-field-column-width = Lățimea coloanei
settings-distraction-free-width-hint = Se aplică doar în timp ce modul fără distragere a atenției este pornit. Lățimile coloanei Scena, Sinopsis și Note sunt neatinse.
settings-distraction-free-title = Păstrați numele elementului
settings-distraction-free-word-count = Păstrează cuvântul conte.
settings-distraction-free-session = Păstrează sesiunea de scriere
settings-distraction-free-go-to = Păstrați Go la... buton
settings-distraction-free-go = Păstrați butoanele anterioare și următoarele
settings-distraction-free-chrome-hint = Butonul Exit rămâne întotdeauna, indiferent ce alegeţi aici. Este calea de ieşire în cazul în care Escape este ocupat.
settings-field-app-theme = Temă
settings-theme-light = Lumină
settings-theme-dark = Întuneric
settings-field-text-scale = Dimensiunea textului interfeței
settings-field-language = Limbajul interfeței
settings-synopsis-placement = Poziţia sintezei
settings-synopsis-placement-none = Niciunul
settings-synopsis-placement-top = Deasupra
settings-synopsis-placement-side = În afară de
settings-typewriter = Derulare tipwriter
settings-typewriter-tip = Detine linia pe care o scrieti la inaltime fixa in timp ce manuscrisul se desfasoara sub ea. Click-ul inca pune ingrijirea unde dati click.
settings-typewriter-position = Poziția liniei
settings-typewriter-position-top-third = În al treilea rând
settings-typewriter-position-middle = Mijloc
settings-typewriter-position-bottom-quarter = Cartierul de jos
settings-highlight-scope = Evidențiază în jurul îngrijirii
settings-highlight-scope-none = Niciunul
settings-highlight-scope-sentence = Sentinţa
settings-highlight-scope-paragraph = Puncte
settings-highlight-scope-tip-none = Nimic nu e umbrit cum scrii.
settings-highlight-scope-tip-sentence = Shade fraza pe care o scrii, așa că iese în evidență de cele din jurul ei.
settings-highlight-scope-tip-paragraph = Shade întregul paragraf pe care îl scrieți, pentru a păstra pasajul la care lucrați în vedere.
settings-group-container-views = Vedere container
settings-remember-view = Amintiți-vă ultima vizualizare pentru fiecare tip de element
settings-remember-view-tip = Deschideţi un container pe vizualizarea pe care aţi folosit-o ultima dată pentru acest tip de produs
settings-remember-view-tip-more =
    O carte, o parte și un capitol fiecare oferă mai multe vizualizări (o pagină proprie, întreaga
    Porneşte asta şi fiecare tip se redeschide pe
    vedeți că ați ales ultima dată pentru ea, de exemplu treceți un capitol la capitolul complet și la capitolul complet
    Următorul capitol pe care îl deschideți arată complet Capitolul prea. Fiecare tip își amintește propria vedere.
settings-group-counting = Numărare cuvânt
settings-counting-auto = Automat (pe limbă)
settings-counting-whitespace = Split pe spații
settings-counting-unicode-words = Cuvinte unicode
settings-counting-cjk-hybrid = CJK-aware (per caracter)
settings-counting-hint = Automat contează chineză și japoneză de caractere și orice altă limbă după cuvânt. Schimbă-l numai în cazul în care numărul arată greșit pentru limba ta.
settings-group-goals-display = Afișează
settings-show-characters = Arată numărul de caractere în bara de stare
settings-autosave-hint = Modificările sunt scrise pe disc automat în timp ce scrieți.

settings-sec-work = Proiect
settings-page-structure = Structură
settings-page-author = Autor
settings-field-author-name = Numele autorului
settings-field-author-placeholder = Opțional
settings-field-author-hint = Apare pe pagina de titlu compilată și în metadatele de fișiere exportate. Lăsați-l gol pentru a omite. Acesta este byline cartea. Numele comentariilor sunt semnate cu este în Setări utilizator.
settings-group-chapters = Capitole
tidy-titles-title = Titluri de capitol
tidy-titles-none = Niciun capitol sau titlu nu-şi repetă doar propriul număr.
tidy-titles-lead = { $count ->
        [one] Un capitol sau o parte este intitulat cu nimic altceva decât propriul său număr.
       *[other] { $count } capitole și piese sunt intitulate cu nimic altceva decât numărul lor propriu.
    }
tidy-titles-explain = Curățarea acestor titluri lasă fiecare nume după numărul pe care cartea îl știe deja, același număr de amprentele de export. Nimic altceva nu se schimbă, și o desface le pune pe toate înapoi.
menu-document-tidy-titles = Titluri de capitol... (&Y)
settings-group-numbering = Numerotare
settings-number-chapters = Capitolele și părțile numerelor
settings-number-chapters-tip = Capitolele și părțile poartă un număr elaborat de unde stau în carte, afișat lângă titlul lor aici, și tipărit de export.
settings-number-chapters-tip-more = Numarul nu este niciodata stocat in titlu, asa ca ramane chiar atunci cand reordonati, introduceti sau stergeti. Inchideti acest lucru si printurile de export numai titlurile, indiferent de stilul de export cerut. Sa lasati un capitol afara (un prolog, un interludiu), folositi in schimb comutatorul de Numarare al Inspectorului: care il pastreaza in carte dar il opreste sa ia un numar.
settings-part-resets-chapter = Reporniți numerele capitolelor la fiecare parte
settings-part-resets-chapter-tip = Off în mod implicit: capitolele rula direct pe părți ale unei cărți, astfel încât "Partea a doua" se deschide pe capitolul 11.
settings-part-resets-chapter-tip-more = Aceasta este practica comercială obișnuită, și ceea ce se așteaptă un cititor. Porniți-o pentru o carte ale cărei părți sunt menite să citească ca volume separate, în cazul în care fiecare dintre ele se deschide pe Capitolul 1.
settings-chapter-flat = Capitole plate
settings-chapter-flat-hint = Pe: un capitol este un singur element. Scrieți în el, și nu deține scene. Off: un capitol este un dosar. Încă scrieți în el, dar poate deține scene, de asemenea. Noi capitole urmează acest cadru; cele existente converti prin Promovare.

settings-styles-builtin = Stiluri construite
settings-styles-user = Stilurile mele.
settings-styles-builtin-badge = Construit
settings-styles-duplicate = Duplicare
settings-styles-edit = Editare
settings-styles-delete = Ștergere
settings-styles-import = Importare…
settings-styles-export = Exportare…
settings-styles-copy-suffix = (copie)
settings-styles-json-filter = Stilul de export
settings-styles-editor-title = Editează stilul
settings-styles-editor-none = Selectaţi un stil personalizat pentru a-l edita, sau duplicat un built-in unul.
settings-styles-imported = Stil importat
settings-styles-import-failed = Nu se poate importa stilul
settings-styles-exported = Stil exportat
settings-styles-export-failed = Nu s-a putut exporta stilul
settings-styles-field-name = Nume
settings-styles-field-chapters = Capitole
settings-styles-field-parts = Capturi din părți
settings-styles-field-scene-break = Spargerea scenei
settings-styles-field-major-scene-break = Pauză de scenă majoră
settings-styles-field-spacing = Spațierea liniei
settings-styles-field-justify = Justifică textul
settings-styles-field-notes = Include note
settings-styles-field-synopses = Include sinopsele
settings-styles-field-scene-titles = Include titlurile scenei
settings-styles-field-epigraphs = Include epigrafe
settings-styles-field-paratexts = Include paratexte
settings-styles-field-epigraph-placement = Poziția epigrafului
settings-styles-epigraph-after = După titlu
settings-styles-epigraph-before = Înainte de titlu
settings-styles-field-footnotes = Include notele de subsol
settings-styles-field-footnote-numbering = Numerotarea notelor de subsol
settings-styles-footnote-numbering-continuous = Continuu
settings-styles-footnote-numbering-per-chapter = Reporniți fiecare capitol
settings-styles-footnote-numbering-per-book = Reporneşte fiecare carte
settings-styles-field-images = Imagini
menu-image = Imagine (&I)
settings-styles-images-beside = Lângă document
settings-styles-images-embed = În interiorul documentului
settings-styles-images-omit = Las-o baltă.
settings-styles-group-round-trip = Trimitere la un editor
settings-styles-field-comments = Include observații
settings-styles-field-round-trip-marks = Include markeri rotunzi
settings-styles-round-trip-hint = DOCX  şi ODT  Numai. Markerele sunt identificatori invizibili care permit unui fişier returnat să actualizeze acest proiect în loc să aterizeze lângă el ca o a doua copie.
settings-styles-group-pages = Pagini
settings-styles-field-cover = Deschide cu capacul
settings-styles-field-word-count = Word conta pe pagina de titlu
settings-styles-field-page-books = Pagină nouă la fiecare carte
settings-styles-field-page-parts = Pagină nouă la fiecare parte
settings-styles-field-page-chapters = Pagina nouă la fiecare capitol
settings-styles-field-page-paratexts = Pagina nouă la fiecare paratext
settings-styles-heading-none = Fără titlu
settings-styles-heading-numbered = Numai număr
settings-styles-heading-title = Numai titlu
settings-styles-heading-both = Număr + titlu
settings-styles-break-blank = Linie albă
settings-styles-break-none = Niciunul
settings-styles-spacing-single = Simplă
settings-styles-spacing-onehalf = 1½
settings-styles-spacing-double = Dublu

welcome-title = Bine ati venit la Skribisto
welcome-version = Versiunea { $version }
welcome-update-available = Versiunea { $version } este disponibilă
welcome-update-tip = Deschide pagina de descărcare în browser.
welcome-search = Caută proiecte
welcome-open = Deschidere
welcome-new-work = Proiect nou
welcome-create-from = Creează din...
welcome-recent-works = Proiecte recente
welcome-empty-recents = Nu există proiecte recente încă.
welcome-no-matches = Niciun proiect recent nu se potriveşte cu căutările tale.
welcome-learn-soon = În curând vin ghizi şi bacşişuri.
welcome-about-blurb = Skribisto, a Rust + Teksilo rescrie aplicația de scriere.
welcome-tagline = *Un loc liniştit pentru a scrie lucruri lungi.*
welcome-github = GitHub
welcome-discord = Discord
nav-works = Proiecte
nav-examples = Exemple
nav-learn = Învaţă
nav-about = Despre

synopsis = Rezumat
epigraph = Motto
pane-manuscript = Manuscris
corkboard = Panou de fișe
overview = Privire de ansamblu

corkboard-card-count = { $count ->
    [one] { $count } card
   *[other] { $count } cărți
}
corkboard-child-count = { $count ->
    [one] Postul { $count }
   *[other] { $count } articole
}
corkboard-view-nested = Cuișoare
corkboard-view-flat = Plată
corkboard-layout-hint = Cuiburile arată copii direcţi ai unui container. Deschideţi un dosar pentru a intra în el. Flat arată fiecare scenă din tot containerul în acelaşi timp.
corkboard-card-size = Mărimea cardului
corkboard-search-placeholder = File cards...
corkboard-empty-title = Încă nu e nimic aici.
corkboard-empty-hint = Utilizați "+ Nou" de mai sus pentru a adăuga primul.
corkboard-new = Nou
corkboard-show-card-numbers = Numărați cărțile
corkboard-modal-size = Dimensiune editor extinsă
corkboard-scope-hint = Se aplică la fiecare plută deschisă, în fiecare proiect.
corkboard-sort-manuscript = Ordinea manuscrisului
corkboard-sort-title-asc = Titlul A? Z
corkboard-sort-title-desc = Titlul Z? A
corkboard-card-number = Carte de vizită { $number }

duplicate = Duplicare
duplicate-n = { $count ->
    [one] Duplicate { $count } card
   *[other] Duplicate { $count } carduri
}
set-label-n = { $count ->
    [one] Setează eticheta pe card { $count }
   *[other] Set etichetă pe { $count } carduri
}
move-to-trash-n = { $count ->
    [one] Mută cartea { $count } la gunoi
   *[other] Mutați { $count } carduri la gunoi
}
reveal-in-outline = Dezvăluie în contur

corkboard-move-to = Mută-te la...
corkboard-move-to-n = { $count ->
    [one] Mutați { $count } card la...
   *[other] Mutați { $count } cărți pentru a...
}
corkboard-move-picker-title = Mută-te la...
corkboard-move-picker-empty = Nu există lianți încă. Creați unul mai întâi.
corkboard-move-picker-cancel = Anulare
corkboard-move-here = Mută aici
corkboard-moved-ok = { $count ->
    [one] Cardul { $count } mutat
   *[other] { $count } carduri mutate
}
corkboard-move-into-self = Un container nu poate fi mutat în interiorul său. Alegeţi o destinaţie în afara acestuia.
corkboard-move-failed = Aceste carduri nu au putut fi mutate acolo.

overview-col-title = Titlu
overview-col-type = Tip
overview-col-label = Descriere scurtă
overview-col-tags = Etichete
overview-col-books = Cărți
overview-col-own-words = Cuvinte
overview-col-total-words = Total
overview-row-count = { $count ->
    [one] Postul { $count }
   *[other] { $count } articole
}
overview-search-placeholder = Filtrare elemente...
overview-expand-all = Extinde tot
overview-collapse-all = Se prăbuşeşte toate
overview-table-label = Cuprins
overview-empty-title = Încă nu e nimic aici.
overview-gone-title = Acest container este plecat
overview-gone-hint = A fost mutată la gunoi.
overview-empty-hint = Utilizați "+ Nou" de mai sus pentru a adăuga primul.
corkboard-grid-label = Cărți de plută
corkboard-rename-field = Redenumește elementul
corkboard-expand-synopsis = Extinde sinopsisul
corkboard-synopsis-modal-title = Rezumat
corkboard-badge-scene = Scenă
corkboard-badge-chapter = Capitol
corkboard-badge-part = Parte
corkboard-badge-book = Carte
corkboard-badge-note = Notă
corkboard-badge-folder = Dosar
corkboard-badge-paratext = Paratext
corkboard-badge-text = Text
corkboard-badge-end = Sfârşit
settings-group-corkboard-layout = Aranjament
settings-group-corkboard-cards = Cărți
corkboard-show-word-count = Afișează numărul de cuvinte pe cărți
text-heading = Text
no-content = Acest produs nu are conținut editabil.
untitled = Fără titlu
placeholder-title = Titlu…
placeholder-subtitle = Subtitrare...
placeholder-chapter-title = Titlul capitolului...
split-editor = Editor Split
close-split-view = Închide vizualizarea divizată
drop-open-here = Deschide aici.
drop-open-to-side = Deschideţi spre lateral.

segment-chapter = Capitol
segment-part = Parte
segment-book = Carte
segment-pace = Ritm de scriere
segment-notes = Note
segment-story-bible = Ghidul poveștii
segment-note-own = Notă
segment-note-details = Detalii
segment-note-in-prose = În proză
pace-placeholder = Planificatorul Pace apare aici.
pace-empty-title = Planifică ritmul acestei cărţi
pace-empty-body = Stabilește un cuvânt gol și un termen limită, și Skribisto Proiecte în ritmul zilnic pentru a vă ajunge acolo.
pace-start-planning = Începe planificarea
pace-section-schedule = Program
pace-section-progress = Progres
pace-advancement = Avansare
pace-goal = Obiectiv de cuvinte
pace-deadline = Termen-limită
pace-active = Pace activ
pace-day-mon = Luni
pace-day-tue = Mar
pace-day-wed = Mireasă
pace-day-thu = Thu.
pace-day-fri = Vine
pace-day-sat = Sâm
pace-day-sun = Soare
pace-card-written = cuvinte scrise
pace-card-of-goal = al obiectivului
pace-card-rate = cuvinte / zi de scriere
pace-card-days-left = zile de scriere rămase
pace-card-streak = dungi de zi
pace-card-ahead = cuvinte înaintea ritmului
pace-card-behind = cuvinte în spatele ritmului
pace-charts-empty = Graficele de progres apar aici odată ce numărul dvs. de cuvinte este înregistrat pe o salvare.
pace-chart-progression = Cuvinte scrise împotriva ţintei
pace-chart-words-per-day = Cuvintele pe zi
pace-series-actual = Realizat
pace-series-target = Obiectiv
pace-series-words-per-day = Cuvinte/zi
pace-daily-target-line = Chiar şi ritmul: { $count ->
    [one] { $count } cuvânt/zi
   *[other] { $count } cuvinte/zi
}
pace-section-holidays = Vacanţe
pace-section-milestones = Etape
pace-holidays-none = Fiecare zi programată contează.
pace-holiday-label = Numele de vacanță
pace-add-holiday = Adăugare
pace-remove = Eliminare
pace-milestones-none = Nu sunt încă etape de etapă. Setați una pe o parte sau capitol în inspector.
full-chapter = Capitolul complet
full-part = Parte completă
full-book = Carte completă
full-synopsis = Sinopsis complet
rename = Redenumire
set-label = Setează eticheta
insert-scene = Inserează scenă
insert-chapter = Se introduce capitolul
split-scene = Scena despărţirii
move-up = Mutare în sus
move-down = Mutare în jos
merge-with-previous = Combină cu precedentul
move-to-trash = Mutare în coș
rename-chapter = Redenumește capitolul
add-scene = Adaugă scenă nouă
add-chapter = Adaugă un nou capitol
new-scene-title = Scenă nouă
placeholder-scene-name = Numele scenei...
menu-cut = Decupare
menu-copy = Copiere
menu-paste = Lipire
menu-paste-unformatted = Lipire fără formatare
menu-select-all = Selectare totală
format-bold = Îndrăzneţ
format-italic = Italic
format-underline = Subliniat
format-strikethrough = StrikeThrough

binder = Arbore de documente
no-work = Niciun proiect încărcat
no-recent-works = Nu există proiecte recente
switcher-open-section = În prezent deschis
switcher-recent-section = Recent
switcher-this-window = Această fereastră
open-project-title = Deschide proiectul
open-project-question = Cum vrei să deschizi "{ $title }"?
open-in-new-window = Deschide în fereastră nouă
open-here = Deschide aici.

binder-all = Toate legaturile
binder-show-all = Arată toți lianții
binder-new = Liant nou...
binder-default-name = Liant nou
binder-item-count = { $count ->
    [one] Postul { $count }
   *[other] { $count } articole
}
binder-search-placeholder = Filtraţi conturul...
binder-search-scope = Căutați toți lianții
binder-trash-confirm-title = Mutați liant la gunoi?
binder-trash-confirm-text = "{ $name }" și toate elementele sale vor fi mutate la gunoi.

dialog-rename = Redenumire
dialog-new-binder = Liant nou
dialog-set-label = Setează eticheta
dialog-new-scene = Scenă nouă
close-work-question = Salvați modificările înainte de închiderea proiectului?
quit-question = Salvați modificările înainte de ieșire?
quit-save-work-question = Salvați modificările la { $title } înainte de a demisiona?
unsaved-changes = Acest proiect are modificări nesalvate.
new-work-unsaved-question = Salvează modificările înainte de a crea un nou proiect?
open-work-unsaved-question = Salvați modificările înainte de a deschide un alt proiect?
switch-backup-discard-title = Arunca modificările la acest backup?
switch-backup-discard-text = Modificările aduse unui backup nu pot fi salvate în el. Utilizați Salvează ca sau restaurează pentru a le păstra, sau aruncați-le și deschideți celălalt proiect.
switch-save-failed = Proiectul nu a putut fi salvat, deci nu a fost înlocuit: { $error }
switch-save-not-started = Proiectul nu a putut fi salvat, aşa că nu a fost înlocuit.
close-save-failed = Proiectul nu a putut fi salvat, deci nu a fost închis: { $error }
close-save-not-started = Proiectul nu a putut fi salvat, aşa că nu a fost închis.
save-not-started = Proiectul nu a putut fi salvat.

could-not-open-work = Nu s-a putut deschide "{ $file }"
could-not-open-work-too-new = "{ $file }" are nevoie de un nou Skribisto
could-not-open-work-too-new-detail = Salvat de Skribisto format { $written_by }; deschiderea are nevoie de format { $requires } sau mai nou, iar acest build suportă până la formatul { $supported }. Actualizează Skribisto pentru a-l deschide.
could-not-open-work-importing = "{ $file }" este scris de un import
could-not-open-work-importing-detail = Un import este scris peste acest proiect și nu a terminat. Deschideți-l odată ce importul se face: deschis acum, ar arăta proiectul de import se înlocuiește, și următorul său salva ar scrie acel proiect înapoi peste import.
could-not-open-example = Nu am putut deschide exemplul: { $error }
could-not-open-link = Nu s-a putut deschide { $url }: { $error }
could-not-create-work = Nu s-a putut crea Proiectul: { $error }
saving-as-file = Economisesc { $target }...
saving-as-folder = Salvarea ca { $target }/...
saved-as = Salvat la { $target }
save-error = Nu s-a putut salva: { $error }
save-flush-failed = Salvarea a trecut prin, dar aceste documente nu au putut fi predate și sunt încă nesalvate: { $titles } ({ $error })
backup-error = Nu s-a putut susține: { $error }
backing-up = Înapoi...
backup-nothing-open = Nici un proiect nu este deschis pentru a da înapoi.
backup-already-running = O copie de rezervă este deja în curs.
backup-complete = Backup complet ({ $ok } salvat, { $skipped } deja curent)
backup-partial = Backup terminat: { $ok } salvat, { $failed ->
    [one] Destinația { $failed } a eșuat
   *[other] Destinațiile { $failed } au eșuat
}
backup-no-destination-title = Nu există locaţie de rezervă disponibilă
backup-no-destination-text = Niciuna dintre destinațiile de rezervă configurate nu poate fi atinsă (de exemplu, o unitate externă poate fi deconectată). Conectați-l și rejudecați-l sau ieșiți fără a da înapoi.
backup-failed-close-title = Întăririle nu au putut fi salvate.
backup-failed-close-text = Nici o copie de rezervă nu poate fi scrisă înainte de închidere. Orice destinație a eșuat (se poate să fi fost îndepărtată, sau poate fi complet sau protejat de scriere). Fixați-l și rejudecați, sau ieșiți fără a da înapoi.

backup-choice-title = Fișier de rezervă
backup-choice-heading = Ai deschis o copie de rezervă.
backup-choice-subtitle = Aceasta este o copie de rezervă punct-in-time a unui proiect.
backup-choice-subtitle-dated = Backup luat { $date }.
backup-choice-body = Puteți deschide și edita gratuit, dar modificările pot fi păstrate doar cu Save As. Fișierul original proiect nu este atins. Sau restabili acest proiect la exact acest backup.
backup-choice-open = Deschideţi rezerva.
backup-choice-restore = Restaurarea acestui proiect în acest punct...
backup-choice-not-a-backup = Nu, deschide-l normal.
backup-banner-title = Copie de rezervă: modificările nu pot fi salvate aici
backup-banner-description = Utilizați Save As pentru a păstra editarea într-un fișier nou, sau Restore pentru a înlocui proiectul original cu acest backup.
backup-banner-restore = Restaurare…
backup-banner-save-as = Salvează ca...
backup-restore-original-missing = Nu se poate găsi proiectul original pentru a restabili peste. Utilizați Salvați Ca pentru a păstra acest backup ca un nou proiect în schimb.
backup-restore-close-elsewhere-title = Proiect deschis într-o altă fereastră
backup-restore-close-elsewhere-text = Proiectul pe care îl restauraţi este deschis într-o altă fereastră. Vă rugăm să-l închideţi acolo mai întâi, apoi să încercaţi din nou.
backup-restore-focus-window = Concentrează-te pe fereastră.
backup-restore-confirm-title = Restaura acest backup?
backup-restore-confirm-text = Versiunea actuală a proiectului va fi copiată ca rezervă de siguranță înainte de a fi înlocuită cu aceasta.
backup-restore-confirm-ok = Restaurare
backup-restore-error = Nu s-a putut restabili: { $error }
backup-restored-ok = Proiectul a fost restaurat.
backup-restored-with-safety = Proiectul a fost restaurat. Versiunea anterioară a fost salvată la { $path }.
close-backup-discard-title = Arunca modificările la acest backup?
close-backup-discard-text = Modificările la o rezervă nu poate fi salvat să-l. Utilizați Salvați Ca pentru a le păstra, sau aruncați și aproape.
quit-backup-discard-title = Să renunţ la schimbări şi să renunţ?
quit-backup-discard-work-question = Eliminaţi modificările la { $title } şi renunţaţi?
quit-backup-discard-text = Modificările la o copie de rezervă nu poate fi salvat să-l. Utilizați Salvați Ca să le păstrați, sau aruncați și să renunțe.
backup-nudge-text = Nu sunt stabilite backup-uri pentru acest proiect.
backup-nudge-action = Aranjează copiile de rezervă...

menu-backups-list = Copii de siguranță… (&K)
backups-title = Copii de siguranță
backups-loading = Scanez după întăriri...
backups-empty = Încă nu s-au găsit întăriri pentru acest proiect.
backups-open = Deschidere
backups-reveal = Dezvăluie
backups-delete = Șterge această copie de rezervă
backups-delete-confirm-title = Să şterg întăririle astea?
backups-delete-confirm-text = „{ $name }” va fi șters definitiv. Acțiunea nu poate fi anulată.
backups-delete-error = Nu s-a putut șterge rezerva: { $error }
backups-refresh = Reîmprospătează
backups-close = Închidere

settings-page-backup = Implicite de rezervă
settings-page-work-backup = Copii de siguranță
settings-backup-general-title = Configurări de rezervă implicite
settings-backup-work-title = Întăriri pentru acest proiect
settings-backup-inherit = Folosește setările generale de rezervă
settings-backup-inheriting = Acest proiect utilizează setările generale de rezervă.
settings-backup-none-hint = Nu sunt instalate backup-uri automate (fiecare declanșator este oprit).
settings-backup-last = Ultima rezervă: { $date }
settings-backup-last-never = Nu există întăriri încă.
settings-backup-open-list = Deschide lista de rezervă...
settings-backup-triggers = Când să se retragă
settings-backup-on-close = Înapoi la închiderea proiectului
settings-backup-on-open = Înapoi la deschiderea proiectului
settings-backup-interval = Înapoi periodic, fiecare
settings-backup-destinations = Destinații de rezervă
settings-backup-default-location = Locație implicită
settings-backup-usage-since = { $count ->
    [one] { $count } backup · { $size } · cel mai vechi { $oldest }
   *[other] { $count } copii de rezervă · { $size } · cel mai vechi { $oldest }
}
settings-backup-usage = { $count ->
    [one] Întăriri { $count } · { $size }
   *[other] { $count } copii de rezervă · { $size }
}
settings-backup-usage-empty = Nimic încă nu e ţinut aici.
settings-backup-usage-measuring = Măsurând...
settings-backup-reveal-root = Arată dosarul
settings-backup-dest-none = Nu există destinaţii. Backup-urile sunt salvate lângă proiect.
settings-backup-dest-remove = Eliminare
settings-backup-dest-add = Adaugă dosar...
settings-backup-dest-refresh = Reîmprospătează
settings-backup-retention = Cât de multe pentru a păstra
settings-backup-retention-tiered = Cu un conținut de grăsimi de minimum 85% din greutate
settings-backup-retention-keep-n = Păstrează ultimul N
settings-backup-retention-tip = Cât de vechi sunt prunele de rezervă.
settings-backup-retention-tip-more = Tiered păstrează un backup pe oră pentru o zi, pe zi timp de o săptămână, pe săptămână timp de o lună, și pe lună dincolo, astfel încât istoria recentă rămâne densă și istorie veche se subțiază afară. "Păstrați ultima N" pur și simplu păstrează N cele mai recente copii. În ambele moduri cele mai noi copii (cel mai mic de mai jos) sunt întotdeauna păstrate.
settings-backup-gfs-hourly = Ore (ultimele 24 de ore)
settings-backup-gfs-daily = Zilnic (săptămâna trecută)
settings-backup-gfs-weekly = Săptămânal (ultima lună)
settings-backup-gfs-monthly = Lunar
settings-backup-keep-n = Numărul de păstrat
settings-backup-min-keep = Păstrează întotdeauna cel puţin
settings-backup-dedup = Sari peste o rezervă când nimic nu s-a schimbat.

new-work-title = Proiect nou
new-work-close = Închidere
new-work-name = Denumirea proiectului
new-work-name-placeholder = Fără titlu
new-work-author = Autor
new-work-author-placeholder = Opțional
new-work-format = Formatare
new-work-single-file = Fișier unic
new-work-bundle = Fișier unic
new-work-convert-later = Puteți converti între formate mai târziu.
new-work-location = Locație
new-work-will-create = Va crea
new-work-language = Limba implicită
new-work-language-hint = Aplicat la texte noi vrajacheck. Fiecare text poate fi trecut la o altă limbă.
new-work-template = Șablon
new-work-template-none = Niciunul
new-work-template-empty-novel = Roman gol
new-work-template-light-novel = Roman deschis
new-work-template-novel = Roman
new-work-template-novel-in-parts = Roman în părți
new-work-template-notebook = Caiet de notițe
new-work-cancel = Anulare
new-work-create = Creează proiect
new-work-step-details = Detalii
new-work-step-language = Structura lingvistică
new-work-step-template = Șablon
new-work-back = Înapoi.
new-work-next = Următorul
new-work-documents-title = Proiect nou din documente
new-work-step-import = Importare
new-work-create-and-import = Creează import...
new-work-documents-next-title = Urmează documentele tale.
new-work-documents-next-body = Crearea proiectului deschide expertul de import peste el, în cazul în care alegeți documentele, revizuiți structura Skribisto citește de la ei, și alege unde aterizează. Nimic nu este scris în proiect până când nu-l confirmați acolo.
new-work-documents-no-template = Acest proiect începe gol în scop: nici un șablon, astfel încât documentele importate sunt singurul lucru în ea.
new-work-documents-chapter-scene-hint = Se aplică capitolelor create de import. O puteți modifica mai târziu în setările proiectului.
new-work-single-file-desc = O arhiva .skrib (zip). Portabila, usor de sustinut.
new-work-bundle-desc = Un dosar care deţine fiecare activ de text. Mai prietenos pentru controlul versiunii.
new-work-template-none-count = liant gol
new-work-template-empty-novel-count = 1 capitol
new-work-template-light-novel-count = 15 capitole
new-work-template-novel-count = 20 capitole
new-work-template-novel-in-parts-count = 3 părți, 24 capitole
new-work-template-notebook-count = note cu formă liberă
new-work-chapter-scene = Capitole plate
new-work-chapter-scene-tip = Fiecare capitol este un singur articol vă scrie direct în, fără scene sub ea. Lăsați acest off pentru aspectul clasic, în cazul în care un capitol este un dosar: vă scrie direct în care prea, dar poate deține scene, de asemenea.
new-work-chapter-scene-tip-more = Puteți scrie într-un capitol, fie un fel. Singura diferență este dacă se poate *conține* scene. Skribisto liant copac este organizațional numai, astfel încât ambele layouts compila în aceeași carte, le puteți amesteca liber, și Promovează converti un capitol între cele două fără a pierde un cuvânt.
new-work-name-required = Introduceți un nume pentru proiect
new-work-name-invalid = Acest nume nu are caractere utilizabile
new-work-location-required = Alegeți o locație
new-work-location-missing = Acest dosar nu există
new-work-location-not-folder = Această cale nu este un dosar
new-work-location-readonly = Acest dosar nu este scris
new-work-target-exists = Un proiect cu acest nume este deja în acest dosar. Crearea proiectului va cere înainte de a-l înlocui
new-work-target-is-folder = Un dosar cu acest nume este deja în această locație. Alege un alt nume
new-work-overwrite-title = Înlocuirea proiectului existent?
new-work-overwrite-text = "{ $name }" există deja în acest dosar. Înlocuiți-l cu noul proiect?
new-work-target-open-title = Acest proiect este deschis.
new-work-target-open-text = "{ $name }" este deschis în Skribisto. Crearea unui nou proiect în locul său ar înlocui proiectul la care lucrați, iar următorul dvs. de salvare ar scrie acel proiect înapoi peste cel nou. Alegeți un alt nume sau dosar, sau închideți proiectul mai întâi.
target-importing-title = Un import scrie acest proiect
target-importing-text = Un import este încă scris "{ $name }." Așteptați până când a terminat, sau alegeți un alt nume.
import-plume-busy-title = Un alt proiect Plume Creator este importat
import-plume-busy-text = Plume Creator proiecte sunt importate una la un moment dat. Așteptați până când importul în curs de desfășurare a terminat, sau anulați-l, apoi începeți aceasta din nou.
import-manuskript-busy-title = Un alt proiect Manuskript este importat
import-manuskript-busy-text = Manuskript proiecte sunt importate una la un moment dat. Așteptați până când importul în curs de desfășurare a terminat, sau anulați-l, apoi începeți aceasta din nou.
backup-restore-importing-text = Un import este încă scris "{ $name }," proiectul acest backup ar fi restaurat peste. Restaurare backup odată ce importul a terminat.

new-work-manuscript = Manuscris
new-work-notes = Note
new-work-research = Cercetare
new-work-notebook = Caiet de notițe
new-work-scene = Scenă
new-work-note = Notă
new-work-front-matter = Materia din față
new-work-back-matter = Materia din spate
new-work-paratext = Structura cărții
new-work-paratext-none = Nicio structură
new-work-paratext-hint = Materia din față și din spate o tradiție deschide și închide o carte cu. Puteți muta, redenumi sau șterge orice din ea după aceea.
new-work-tags = Etichete
new-work-tags-none = Fără etichete
new-work-tags-hint = O paletă de început pentru etichetarea caracterelor, a locurilor și a restului. Opțional, și fiecare etichetă poate fi redenumită, recolorată sau ștersă după aceea.
new-work-note-templates = Șabloane pentru note
new-work-note-templates-none = Fără șabloane
new-work-note-templates-hint = Forma o nota story-bible începe în. Opțional, și fiecare șablon poate fi editat sau șters după aceea.
new-work-characters = Personaje
new-work-places = Locuri

import-plume-title = Importă proiectul Plume Creator
import-plume-close = Închidere
import-plume-source = Proiect Plume
import-plume-source-hint = Alegeți un fișier .plume sau .plume_backup (orice versiune Plume Creator).
import-plume-location = Dosarul de destinație
import-plume-name = Nume fișier
import-plume-name-placeholder = Denumirea proiectului
import-plume-will-create = Va crea
import-plume-trash-warning = Nu se migrează prin efracţie.
import-plume-cancel = Anulare
import-plume-import = Importare
import-plume-source-required = Alegeți un fișier de proiect Plume
import-plume-source-missing = Acest fișier nu există
import-plume-source-not-file = Această cale nu este un fișier
import-plume-location-required = Alegeți un dosar de destinație
import-plume-location-missing = Acest dosar nu există
import-plume-location-not-folder = Această cale nu este un dosar
import-plume-location-readonly = Acest dosar nu este scris
import-plume-name-required = Introduceți un nume de fișier
import-plume-name-exists = Un fișier cu acest nume există deja aici. Import va confirma suprascrierea
import-plume-overwrite-title = Înlocuiți fișierul existent?
import-plume-overwrite-text = "{ $name }" există deja. Înlocuiți-l cu proiectul importat?
import-plume-manuscript-binder = Manuscris
import-plume-story-bible-binder = Ghidul poveștii
import-plume-progress-title = Import proiect Plume...
import-plume-cancel-import = Anulare
import-plume-cancelled = Importă anulată
import-plume-done = Importat { $imported ->
    [one] { $imported } element.
   *[other] { $imported } articole.
 } { $skipped ->
    [one] { $skipped } produs distrus nu a fost migrat.
   *[other] { $skipped } nu au fost migrate.
}
import-plume-open-now = Deschide acum.
import-plume-warnings = { $count ->
    [one] 1 lucru nu a putut fi importat exact
   *[other] { $count } lucruri nu pot fi importate exact
}
import-plume-details = Detalii
import-plume-warnings-title = Avertizare importare
import-plume-error-title = Nu am putut importa proiectul
import-plume-nested-too-deep = Proiectul nu a fost importat: partea sa "{ $part }" este cuibărită cu mai mult de { $limit } niveluri adânci. nr. Plume Creator proiect se apropie de acest lucru, iar citirea sa ar putea prăbuși Skribisto. Nimic nu a fost scris.
import-plume-declares-entities = Proiectul nu a fost importat: partea sa "{ $part }" declară entități XML, care nu face nici un proiect Plume Creator. Extinderea acestora ar putea lua mai mult memorie decât computerul are și accident Skribisto. Nimic nu a fost scris.
import-plume-error-details = Detalii

import-manuskript-title = Importă proiectul Manuskript
import-manuskript-close = Închidere
import-manuskript-source = Proiect Manuskript
import-manuskript-source-hint = Alegeți fișierul .msk sau dosarul proiectului (orice versiune Manuskript).
import-manuskript-source-file = Alegeți fișierul...
import-manuskript-source-folder = Alegeți dosarul...
import-manuskript-location = Dosarul de destinație
import-manuskript-name = Nume fișier
import-manuskript-name-placeholder = Denumirea proiectului
import-manuskript-will-create = Va crea
import-manuskript-cancel = Anulare
import-manuskript-import = Importare
import-manuskript-source-required = Alege un proiect Manuskript
import-manuskript-source-missing = Acest fișier sau dosar nu există
import-manuskript-location-required = Alegeți un dosar de destinație
import-manuskript-location-missing = Acest dosar nu există
import-manuskript-location-not-folder = Această cale nu este un dosar
import-manuskript-location-readonly = Acest dosar nu este scris
import-manuskript-name-required = Introduceți un nume de fișier
import-manuskript-name-exists = Un fișier cu acest nume există deja aici. Import va confirma suprascrierea
import-manuskript-overwrite-title = Înlocuiți fișierul existent?
import-manuskript-overwrite-text = "{ $name }" există deja. Înlocuiți-l cu proiectul importat?
import-manuskript-manuscript-binder = Manuscris
import-manuskript-story-bible-binder = Ghidul poveștii
import-manuskript-characters-group = Personaje
import-manuskript-world-group = Lumea
import-manuskript-plots-group = Ploturi
import-manuskript-project-info-note = Informații privind proiectul
import-manuskript-summary-note = Rezumat
import-manuskript-importance-minor = Minore
import-manuskript-importance-secondary = Secundar
import-manuskript-importance-main = Principal
import-manuskript-progress-title = Importare proiect Manuskript...
import-manuskript-cancel-import = Anulare
import-manuskript-cancelled = Importă anulată
import-manuskript-done = { $imported ->
    [one] { $imported } produs importat.
   *[other] { $imported } produse importate.
 } { $revisions ->
    [0] { "" }
    [one] Versiunea anterioară { $revisions } a venit cu ea.
   *[other] { $revisions } versiuni anterioare au venit cu ele.
}
import-manuskript-open-now = Deschide acum.
import-manuskript-warnings = { $count ->
    [one] 1 lucru de știut despre acest import
   *[other] { $count } lucruri de ştiut despre acest import
}
import-manuskript-details = Detalii
import-manuskript-warnings-title = Despre acest import
import-manuskript-error-title = Nu am putut importa proiectul
import-manuskript-nested-too-deep = Proiectul nu a fost importat: fişierul său "{ $part }" este cuibărit mai mult de { $limit } niveluri adânci. Nu Manuskript proiect se apropie de faptul că, şi lectură ar putea accident Skribisto. Nimic nu a fost scris.
import-manuskript-folders-too-deep = Proiectul nu a fost importat: unul dintre fişierele sale stă mai mult de { $limit } foldere adânci. Proiectul nr. Manuskript se apropie de acest lucru, şi citind că ar putea accidenta Skribisto. Nimic nu a fost scris.
import-manuskript-declares-entities = Proiectul nu a fost importat: fişierul său "{ $part }" declară entităţi XML, care nu face nici un proiect Manuskript. Extinderea lor ar putea lua mai mult memorie decât computerul are şi accident Skribisto. Nimic nu a fost scris.
import-manuskript-error-details = Detalii

import-target-open-title = Acest proiect este deschis.
import-target-open-text = "{ $name }" este deschis în Skribisto. Importarea peste el ar înlocui proiectul la care lucrați, iar următoarea salvare ar scrie acel proiect înapoi peste import. Alegeți un alt nume de fișier, sau închideți proiectul mai întâi.
import-warnings-announcement = { $result } { $notice }.

export-title = Exportare
export-close = Închidere
export-scope-label = Ce?
export-format-label = Formatare
export-style-label = Stil
export-path-label = Fișier
export-browse = Răsfoieşte...
export-show-non-exportable = Afișează neexportabil
export-choose-empty = Nici un proiect de a alege de la.
export-cancel = Anulare
export-export = Exportare
export-save-dialog-title = Exportă în fișier
export-section-what = Ce să exporte
export-section-style = Stil
export-section-destination = Destinația
export-custom-selection = Selecţie personalizată
export-selected-count = { $count } selectate
export-chip-chapters-none = Capitole: niciunul
export-chip-chapters-numbered = Capitole: numerotate
export-chip-chapters-title = Capitole: numai titlu
export-chip-chapters-both = Capitole: număr + titlu
export-chip-scene-break-glyph = Pauza de scena: { $glyph }
export-chip-scene-break-blank = Spargerea scenei: linie albă
export-chip-scene-break-none = Pauza de scena: nici unul
export-chip-major-break-glyph = Pauză majoră { $glyph }
export-chip-major-break-blank = Pauză majoră: linie liberă
export-chip-major-break-none = Pauză majoră: niciuna
export-chip-major-break-same = Ambele niveluri rupe la fel
export-chip-spacing-single = Spațiere: unică
export-chip-spacing-onehalf = Spațiere: 11⁄2
export-chip-spacing-double = Spațiere: dublu
export-chip-notes-included = Note incluse
export-chip-notes-excluded = Note excluse
export-format-docx = Word
export-format-odt = LibreOffice
export-format-html = HTML
export-format-markdown = Markdown
export-format-djot = Djot
export-format-text = Text
export-format-latex = LaTeX
export-format-epub = EPUB
export-format-pdf = PDF
export-overwrite-title = Înlocuiți fișierul existent?
export-overwrite-text = "{ $name }" există deja.
export-progress-title = Export...
export-cancelled = Exportă anulat
export-done = Exportat { $count ->
    [one] Postul { $count }
   *[other] { $count } articole
}
export-open-file = Deschidere
export-show-in-folder = Arată în dosar
export-error-title = Nu a putut exporta
export-error-details = Detalii

backup-progress-start = Începând...
backup-progress-retention = Curățarea backup-uri vechi ...
backup-progress-done = Gata
backup-progress-destination = Destinația { $i } din { $n }
backup-details = Detalii
backup-issues-title = Aspecte de rezervă
backup-failed-title = Întăriri eșuate
backup-complete-prune-warning = Backup complet ({ $ok } salvat, { $skipped } deja curent). Unele backup-uri vechi nu au putut fi eliminate

search = Căutare
search-query-placeholder = Căutare…
search-replace-placeholder = Înlocuiți cu...
search-replace-toggle = Comută înlocuirea
search-replace-all = Înlocuiește toate
search-opt-case = Caz de potrivire
search-opt-whole-word = Întreg cuvântul
search-opt-diacritics = Se potrivesc accentele
search-scope-body = Organism
search-scope-title = Titlu
search-scope-synopsis = Rezumat
search-scope-label = Descriere scurtă
search-facet-book = Cărți
search-facet-part = Părți
search-facet-chapter = Capitole
search-facet-scene = Scene
search-facet-note = Note
search-facet-folder = Dosare
search-tip-case = Caz meci: tratați Uppercase și minuscule ca diferite, astfel "Elena" și "elena" sunt meciuri separate.
search-tip-whole-word = Întregul cuvânt: se potriveşte doar cu cuvintele complete, aşa că "pisică" nu se găseşte în "categoria."
search-tip-diacritics = Accente meci: trata litere accentuate ca distincte, astfel încât "cafe" nu se potrivește cu "café."
search-tip-body = Body: căutați proza de scene și note.
search-tip-title = Titlu: căutați titlurile de articole liant.
search-tip-synopsis = Synopsis: căutați rezumatul fiecărui element scris.
search-tip-label = Etichetă: căutați nota scurtă afișată sub titlul unui articol de liant.
search-tip-comment = Comentarii: căutați textul de fire de comentariu și răspunsurile lor. Un comentariu este despre manuscris mai degrabă decât o parte din ea, astfel încât are propriul comutator, și un comentariu de înlocuire se potrivește până când le bifați.
search-tip-book = Cărți: recipientul de carte și markerii săi de început / sfârșit.
search-tip-part = Părți: separatoare de nivel parțial.
search-tip-chapter = Capitole: capitole, în orice mod le stochează proiectul.
search-tip-scene = Scene: elementele care dețin proza ta.
search-tip-note = Note: note de formă liberă.
search-tip-folder = Dosare: organizare simplă a folderelor și separatoarelor.
search-tip-paratext = Un text care aparţine cărţii, dar nu şi poveştii sale: o prefaţă, o dedicaţie, un cuvânt de după. Niciodată nu s-a numărat în manuscris.
search-tip-preserve-case = Caz de conservare: un înlocuitor ia cazul găsit, astfel încât "ELENA" devine "MARTA" și "Elena" devine "Marta."
search-tip-replace = Înlocuiește: arată câmpul de înlocuire și Înlocuiește toate.
search-error = Căutare eșuată: { $message }
search-no-matches = Nicio potrivire.
search-count =
    { $matches ->
        [one] { $matches } meci
       *[other] { $matches } meciuri
    } in { $items ->
        [one] Document { $items }
       *[other] { $items } documente
    }
search-count-truncated =
    { $matches ->
        [one] { $matches } meci
       *[other] { $matches } meciuri
    } in { $items ->
        [one] Document { $items }
       *[other] { $items } documente
    } (numai primele rezultate)
search-occurrences = { $count }
search-field-body = Organism
search-field-title = Titlu
search-field-synopsis = Rezumat
search-field-label = Descriere scurtă
search-field-epigraph = Motto
search-field-comment = Comentariu
search-field-comment-reply = Răspunde
search-field-footnote = Notă de subsol
search-include-in-replace = Include în înlocuire toate
search-collapse-all = Distrugerea tuturor rezultatelor
search-replace-here = Înlocuiește acest
search-dismiss = Respingeţi rezultatele.
search-undo-dismiss = Adu înapoi ultimul rezultat respins
search-replace-nothing = (nimic)
search-replace-confirm-title = Să înlocuim toate chibriturile?
search-replace-confirm-text =
    Înlocuire { $occurrences ->
        [one] { $occurrences } apariţie
       *[other] { $occurrences } apariţii
    } of "{ $query }" with "{ $replacement }" across { $items ->
        [one] Document { $items }
       *[other] { $items } documente
    } Puteți anula acest lucru de la notificarea după aceea.
search-replace-done-title = Înlocuiește complet
search-replace-done =
    { $occurrences ->
        [one] { $occurrences } apariţie
       *[other] { $occurrences } apariţii
    } replaced in { $items ->
        [one] Document { $items }
       *[other] { $items } documente
    } .
search-replace-done-skipped =
    { $occurrences ->
        [one] { $occurrences } apariţie
       *[other] { $occurrences } apariţii
    } replaced in { $items ->
        [one] Document { $items }
       *[other] { $items } documente
    }. { $skipped ->
        [one] Câmp { $skipped }
       *[other] { $skipped } câmpurile
    } a sărit (schimbat de la căutare).
search-replace-undo = Anulare acțiune
search-replace-failed-title = Înlocuire eșuată
search-replace-skipped-title = Nimic nu a fost înlocuit.
search-replace-skipped-body = Numărul de texte schimbate de la căutare: { $fields }. Aceste texte au rămas nemodificate. Căutați din nou pentru a vedea unde se află acum cuvintele.
search-replace-undo-failed-title = Anulare eșuată
search-preview = Previzualizare
search-preview-empty = Selectaţi un rezultat pentru a-l previzualizare aici
search-preview-no-prose = Acest meci nu are text editabil
search-preview-prompt = Pentru a vedea o previzualizare aici, executați o căutare.
search-preview-open-search = Caută în proiect
search-preview-footnote-prompt = Acest meci este în propriul text al unei note de subsol. Deschide-l în doc Notes pentru a vedea și edita-l.
search-preview-open-footnotes = Deschide note de subsol

find-placeholder = Caută în acest document...
find-count = { $current } din { $total }
find-no-results = Niciun rezultat
find-close = Închide căutarea
find-previous = Meciul anterior (Shift+Enter)
find-next = Următorul meci (Introducere)
find-opt-case = Caz de potrivire
find-opt-whole-word = Întreg cuvântul
find-replace-toggle = Comută înlocuirea (Ctrl+R)
find-replace-placeholder = Înlocuiți cu...
find-replace = Înlocuire
find-replace-all = Înlocuiește toate
find-preserve-case = Conserve case

dict-download-title = Descarc { $name }...
dict-download-done = Instalat { $name }
dict-download-failed = Nu s-a putut descărca { $name }: { $error }
dict-removed = Eliminate { $name }
dict-accept-first = Acceptă licență înainte de a descărca { $name }
settings-dict-tab-installed = Instalat
settings-dict-tab-get-more = Ia mai mult.
settings-dict-tab-personal = Cuvintele personale
dict-installed-empty = Încă nu s-au găsit dicţionare pe acest computer.
dict-get-more-search = Limbi de căutare
dict-system-badge = pe sistemul dumneavoastră
dict-unusable-badge = inutilizabil
dict-download-button = Descărcare
dict-downloading = Descărcare...
dict-installed-label = Instalat
dict-remove = Eliminare
dict-view-license = Licență de vizualizare
dict-approx-size = ~{ $size }
dict-personal-empty = Nu există cuvinte personale în acest proiect încă.
dict-personal-add = Adăugare
dict-personal-placeholder = Adaugă un cuvânt...
dict-missing-toast = { $count ->
    [one] Acest proiect utilizează dicţionar { $count } pe care nu îl aveţi instalat
   *[other] Acest proiect utilizează { $count } dicționare pe care nu le-ați instalat
}
dict-missing-action = Obține dicționare
dict-license-title = { $name } licenţă
dict-license-accept = Acceptă descărcarea
dict-license-cancel = Anulare
dict-license-close = Închidere
dict-add-button = Adaugă dicționar...
dict-add-title = Adaugă un dicționar
dict-add-close = Închidere
dict-add-name = Nume
dict-add-name-placeholder = De exemplu dicționarul meu latin
dict-add-code = Codul lingvistic
dict-add-code-placeholder = de exemplu la sau f-FR-x-custom
dict-add-code-hint = Aceasta este ceea ce alegi ca limbă a unui document.
dict-add-aff = Fișier Affix (.aff)
dict-add-dic = Listă de cuvinte (.dic)
dict-add-submit = Adăugare
dict-add-cancel = Anulare
dict-add-name-required = Dă-i dicţionarului un nume.
dict-add-code-required = Introduceți un cod lingvistic.
dict-add-code-invalid = Utilizați numai litere, cifre, și - _ .
dict-add-code-reserved = Ăsta e un cod de dicţionar încorporat.
dict-add-file-required = Alege un fişier.
dict-add-file-missing = Acest fişier nu există.
dict-add-code-taken = Un dicţionar pentru acel cod este deja instalat.
dict-add-done = Adăugat { $name }
dict-add-unusable = Aceste fișiere nu sunt un dicționar utilizabil: { $error }
dict-add-failed = Nu a putut adăuga dicţionarul: { $error }
inspector-export = Exportare
inspector-exportable = Include în exporturi
ctx-number = Numără acest capitol (&N)
ctx-unnumber = Nu număra acest capitol (&N)
inspector-numbering = Numerotare
inspector-numbered = Numerotat
inspector-numbered-tip = Acest capitol îşi are locul în numerotarea cărţii. Opriţi-l pentru un prolog, un epilog sau un interludiu.
inspector-numbered-tip-more = Un capitol nenumerotat rămâne în carte exact așa cum a fost: poziția sa, proza sa și numărul său de cuvinte sunt neatinse. Pur și simplu printează nici un număr, și nu folosește unul în sus: capitolul după un prolog este capitolul unu, nu capitolul doi. Lăsând-o din export, în schimb, este un comutator diferit, de mai sus, și că unul elimină capitolul din carte cu totul.
inspector-apply-to-children = Aplicați la copii
inspector-milestone = Data de referință
inspector-milestone-none = Nicio dată
inspector-milestone-clear = Golire
inspector-tags = Etichete
inspector-aliases = Cunoscut şi sub numele de
inspector-dict-language = Limbă
inspector-apply-language-to-children = Aplică limba la copii
settings-page-language = Limbă
settings-field-dict-language = Limbi
dict-tradeoff-hint = Fiecare limbă suplimentară acceptă mai multe cuvinte, astfel încât mai puține greșeli sunt prinse.
lang-inherit-hint = Moștenit: această scenă folosește limbile cărții sau ale proiectului.
lang-pill-list = Limbi
lang-pill-add = Adaugă o limbă
lang-pill-remove = Elimină { $name }
lang-pill-mute = Opreşte verificarea vrăjilor pentru { $name }
lang-pill-unmute = Verific vraja pentru { $name }.

settings-page-personal-dictionary = Dicționar personal
settings-user-dict-desc = Cuvintele pe care le adăugați aici sunt tratate ca corect scris peste tot în aplicație, și niciodată marcat din nou.
settings-user-dict-add-placeholder = Adaugă un cuvânt...
settings-user-dict-add = Adaugă cuvânt
settings-user-dict-search = Cuvinte de filtrare
settings-user-dict-count = { $count ->
    [one] { $count } cuvânt
    [few] { $count } cuvinte
   *[other] { $count } de cuvinte
}
settings-user-dict-rename = Redenumire
settings-user-dict-remove = Eliminare
settings-user-dict-empty = Adăugaţi cuvinte pe care trebuie să le ignore sau să importe o listă.
settings-user-dict-import = Importare…
settings-user-dict-export = Exportare…
settings-user-dict-import-tip = Adăugați cuvinte dintr-un fișier text simplu, un cuvânt pe linie. Cuvintele existente sunt păstrate.
settings-user-dict-export-tip = Salvați întreaga listă într-un fișier text simplu, un cuvânt pe linie.
settings-user-dict-txt-filter = Fișiere text
settings-user-dict-duplicate = Deja în dicţionar
settings-user-dict-imported = Importat { $count ->
    [one] { $count } cuvânt
   *[other] { $count } cuvinte
} ({ $duplicates } este deja prezentă).
settings-user-dict-import-failed = Nu am putut citi lista de cuvinte: { $error }
settings-user-dict-exported = Salvat { $count ->
    [one] { $count } cuvânt.
   *[other] { $count } cuvinte.
}
settings-user-dict-export-failed = Nu s-a putut salva lista de cuvinte: { $error }

settings-page-text-replacements = Înlocuiri text
settings-text-repl-desc = Înlocuiți prescurtarea cu textul complet pe măsură ce tastați: "btw" devine "apropo" momentul în care tastați un spațiu sau punctuație.
settings-text-repl-enable = Utilizați înlocuitori de text în acest proiect
settings-text-repl-disabled-hint = Porneşte asta pentru a defini prescurtarea care se extinde pe măsură ce scrii.
settings-text-repl-add = Adaugă regulă
settings-text-repl-trigger-placeholder = Scurtătură
settings-text-repl-replacement-placeholder = Ceea ce devine
settings-text-repl-added = Adăugat "{ $trigger }"
settings-text-repl-duplicate = "{ $trigger }" are deja o regulă
settings-text-repl-filter = Reguli de filtrare
settings-text-repl-count = { $n ->
    [one] 1 regulă
   *[other] { $n } reguli
}
settings-text-repl-row-enabled = Foloseşte această regulă.
settings-text-repl-delete = Se elimină regula pentru { $trigger }
settings-text-repl-deleted = Eliminat regula pentru "{ $trigger }"
settings-text-repl-empty = Nu există reguli încă.
settings-text-repl-csv-filter = Fișiere CSV
settings-text-repl-import = Importare…
settings-text-repl-export = Exportare…
settings-text-repl-imported = Importat { $added }, sărit { $skipped }
settings-text-repl-exported = Exportat { $n ->
    [one] 1 regulă
   *[other] { $n } reguli
}

editor-menu-no-suggestions = Fără sugestii.
editor-menu-add-to-dictionary = Adaugă "{ $word }" în dicţionar
editor-menu-add-words-to-dictionary = Adaugă cuvinte selectate în dicționar
editor-dict-added = Adăugat "{ $word }" în dicţionar.
editor-dict-added-multi = { $count ->
    [one] A adăugat { $count } cuvânt în dicţionar.
   *[other] Am adăugat { $count } de cuvinte în dicţionarul tău.
}
toast-undo = Anulare acțiune

undo-superseded-title = Acest pas nu mai este ultimul
undo-superseded-body = Altceva sa schimbat în acest proiect de atunci. Undo ar lua înapoi, în schimb, așa că nu a făcut. Nimic acest pas a fost inversat. Utilizați Editare & Undo pentru a pas înapoi prin istoria ta.
undo-failed = Anulare eșuat: { $error }

menu-edit = Editare (&E)
menu-edit-undo = Anulare acțiune (&U)
menu-edit-redo = Refacere (&R)
menu-edit-undo-target = Anulează acțiunea { $target } (&U)
menu-edit-redo-target = Redo { $target } (&R)
shortcut-name-edit-undo = Anulare acțiune
shortcut-name-edit-redo = Refacere
undo-target-typing = dactilografiere
undo-target-project = ultima modificare a acestui proiect
prose-history-reset-title = Resetarea istoricului de tastarea
prose-history-reset-body = { $count ->
    [one] O scenă deschisă a fost restaurată dintr-un stat anterior, astfel încât istoria sa de dactilografiere nu se mai aplică.
   *[other] { $count } scene deschise au fost restaurate dintr-o stare anterioară, astfel încât istoria lor de dactilografiere nu se mai aplică.
}
undo-target-trash = Mutarea la gunoi
undo-target-restore = restaurarea din gunoi
undo-target-delete-forever = Ștergere pentru totdeauna
undo-target-replace-all = înlocuirea întregului proiect
undo-target-import = importul documentului
undo-target-duplicate = Duplicarea
undo-target-move = în mișcare
undo-target-merge = fuzionarea a două scene
undo-target-split = Împărţim o scenă
undo-target-promote = modificarea tipului
undo-target-tidy-titles = Curățarea titlurilor capitolului
undo-target-import-tags = importul de etichete
undo-target-import-templates = Șabloane pentru nota de import
undo-target-create = crearea
undo-target-remove = Ștergere
undo-target-rename = redenumită
undo-target-edit = care editează
menu-edit-find = Căutare… (&F)
menu-edit-find-next = Găsește următorul (&N)
menu-edit-find-prev = Caută anterior (&V)
menu-edit-replace = Găsiţi şi înlocuiţi... (&L)
menu-edit-find-in-project = Caută în proiect... (&J)
menu-edit-replace-in-project = Înlocuiește în proiect... (&I)
undo-frozen = Anulează ("Întotdeauna înainte" este pe)
redo-frozen = Redo ("Întotdeauna înainte" este pe)
menu-edit-cut = Decupare (&T)
menu-edit-copy = Copiere (&C)
menu-edit-paste = Lipire (&P)
menu-edit-paste-plain = Lipire fără formatare (&W)
menu-edit-select-all = Selectare totală (&A)
shortcut-name-edit-cut = Decupare
shortcut-name-edit-copy = Copiere
shortcut-name-edit-paste = Lipire
shortcut-name-edit-paste-plain = Lipire fără formatare
shortcut-name-edit-select-all = Selectare totală

titlebar-spellcheck-on = Verificare vrajă este pornit. Click pentru a opri verificarea (F7)
titlebar-spellcheck-off = Verificarea pe litere este oprit. Faceți clic pentru a verifica din nou (F7)
menu-spellcheck = Verificați ortografia (&C)
menu-comments = Comentarii (&M)
settings-page-spellcheck = Verificarea vrăjilor
settings-group-spellcheck = Verificarea vrăjilor
settings-spellcheck-enabled = Verifică ortografia în timp ce scriu.
settings-spellcheck-hint = Subliniază cuvinte pe care nici un dicţionar instalat nu le ştie. Oprirea acestui lucru opreşte toate verificările, în fiecare proiect, până când îl porniţi. Pentru a nu mai verifica o singură limbă, ştergeţi-i semnul de verificare pe câmpul lingvistic al proiectului sau al unui element.

menu-trash = Coș de gunoi (&T)
trash-title = Coș de gunoi
trash-empty-state = Gunoiul e gol.
trash-empty-button = Gunoi gol...
trash-restore = Restaurare (&R)
trash-restore-to = Restaurare la... (&T)
trash-delete-forever = Șterge pentru totdeauna (&D)
trash-restored-ok = { $count } restaurat.
trash-restore-error = Nu s-a putut restabili: { $error }
trash-restore-orphaned = Locul original al acestui articol este plecat. Alegeți unde să-l restaureze.
trash-restore-no-project = Niciun proiect nu este deschis, deci nu este nimic de restaurat.
trash-delete-no-project = Niciun proiect nu este deschis, deci nu este nimic de şters.
trash-empty-confirm-title = Să golesc gunoiul?
trash-empty-confirm-text = { $count ->
    [one] { $count } intrare aruncată va fi eliminată permanent.
   *[other] { $count } intrări distruse vor fi șterse permanent.
} Ei lasă gunoiul pentru totdeauna.
trash-emptied-title = Gunoi golit
trash-emptied-body = Totul din gunoi a fost şters permanent.
trash-empty-no-project = Nici un proiect nu este deschis, deci nu există nici un gunoi pentru a goli.
trash-delete-forever-confirm-title = Şterg pentru totdeauna?
trash-delete-forever-confirm-text = { $count ->
    [one] Punctul { $count } va fi eliminat permanent.
   *[other] Articolele { $count } vor fi șterse permanent.
} Ei lasă gunoiul pentru totdeauna.
trash-deleted-title = Şters pentru totdeauna.
trash-deleted-body = { $count } eliminate permanent.
trash-undo = Anulare acțiune
trash-restore-picker-title = Restaurare la...
trash-restore-to-confirm-title = Restaurare aici?
trash-restore-to-confirm-text = Restaurare "{ $item }" în "{ $destination }"?
trash-restore-picker-restore-here = Restaurare aici
trash-restore-picker-cancel = Anulare
trash-restore-picker-empty = Nu există lianți încă. Creați unul mai întâi.
trash-banner-title = Acest articol este în gunoi
trash-banner-description = Nu va apărea în contur sau exporturi până când nu-l restauraţi.
trash-banner-restore = Restaurare…
trash-tab-tooltip = În gunoi.

settings-styles-sheet-yes = Da
settings-styles-sheet-no = Nu
settings-styles-sheet-font = Font
settings-styles-sheet-indent = Prima liniuţă
settings-styles-sheet-para-spacing = Ecartamentul dintre puncte
settings-styles-sheet-page = Dimensiunea paginii
settings-styles-sheet-margins = Margini (T/R/B/L)
settings-styles-sheet-title-page = Pagina titlului de carte
settings-styles-sheet-heading-language = Limba de direcţie
settings-styles-sheet-auto = Urmează textul
settings-styles-sheet-digits = Numerale
settings-styles-sheet-direction = Direcţie text
settings-styles-sheet-formats = Formate
settings-styles-sheet-all-formats = Toate

settings-styles-editor-group = Editor de stil
settings-styles-editor-missing = Acest stil nu mai este disponibil.

settings-styles-page-letter = Scrisoare
settings-styles-digits-western = Vest (0
settings-styles-digits-eastern-arabic = Arabă de Est ( bază)
settings-styles-direction-ltr = Stânga la dreapta
settings-styles-direction-rtl = Dreapta la stânga

format-dock-title = Formatare
format-panel-empty = Pune cursorul într-o scenă, notă, sau sinopsis pentru a vedea opțiunile de formatare.
format-group-marks = Text
format-group-block = Puncte
format-group-lists = Liste
format-group-tables = Tabel
format-group-breaks = Scena pauze
format-superscript = Superscripție
format-subscript = Subscripție
format-link = Link...
link-dialog-insert-title = Inserează link
link-dialog-edit-title = Editează linkul
link-dialog-text-label = Text
link-dialog-text-placeholder = cuvintele pe care le vede cititorul.
link-dialog-url-label = Legături către
link-dialog-url-placeholder = exemplu.com
link-dialog-insert = Inserează
link-dialog-apply = Aplicare
link-dialog-cancel = Anulare
link-dialog-remove = Elimină linkul
link-scheme-refused = Numai link-uri web și e-mail pot fi deschise. { $url } a fost lăsat în pace.
menu-format-link = Link... (&N)
format-clear = Curăță formatarea
format-heading = Nivelul rubricii
format-heading-normal = Text normal
format-heading-1 = Rubrica 1
format-heading-2 = Rubrica 2
format-heading-3 = Rubrica 3
format-heading-4 = Rubrica 4
format-heading-5 = Rubrica 5
format-heading-6 = Rubrica 6
format-align-left = Aliniază stânga
format-align-center = Centru
format-direction-rtl = Paragraful de la dreapta la stânga
format-blockquote = Citat bloc
format-list-bullet = Lista cu gloanțe
format-list-numbered = Lista numerotată
format-indent = Indentare
format-outdent = Ieșire
format-table-insert = Inserează tabel
format-table-row-above = Se introduce rândul de mai sus
format-table-row-below = Se introduce rândul de mai jos
format-table-col-before = Inserează coloană înainte
format-table-col-after = Se introduce coloana după
format-table-row-delete = Șterge rândul
format-table-col-delete = Șterge coloana
format-table-remove = Elimină tabel

menu-format-marks-bold = Îndrăzneţ (&D)
menu-format-marks-italic = Italic (&I)
menu-format-marks-underline = Subliniat (&U)
menu-format-marks-strike = StrikeThrough (&S)
menu-format-marks-superscript = Superscripție (&P)
menu-format-marks-subscript = Subscripție (&B)
menu-format-marks-clear = Curăță formatarea (&C)
menu-format-heading = Rubrica (&H)
menu-format-heading-normal = Text normal (&N)
menu-format-heading-1 = Rubrica 1 (&1)
menu-format-heading-2 = Rubrica 2 (&2)
menu-format-heading-3 = Rubrica 3 (&3)
menu-format-heading-4 = Rubrica 4 (&4)
menu-format-heading-5 = Rubrica 5 (&5)
menu-format-heading-6 = Rubrica 6 (&6)
menu-format-alignment = Aliniere (&A)
menu-format-align-left = Aliniere stânga (&L)
menu-format-align-center = Centru (&C)
menu-format-direction = Direcţie (&E)
menu-format-direction-auto = Automat (&A)
menu-format-direction-ltr = Stânga la dreapta (&L)
menu-format-direction-rtl = Dreapta la stânga (&R)
menu-format-blockquote = Citat bloc (&Q)
menu-format-lists = Liste (&L)
menu-format-list-bullet = Lista cu gloanţe (&B)
menu-format-list-numbered = Listă numerotată (&N)
menu-format-indent = Indentare (&I)
menu-format-outdent = Ieșire (&O)
menu-format-table = Tabel (&T)
menu-format-table-insert = Inserează tabel (&I)
menu-format-table-2x2 = 2 x 2 (&2)
menu-format-table-3x3 = 3 x 3 (&3)
menu-format-table-4x4 = 4 x 4 (&4)
menu-format-table-row-above = Inserează rând de mai sus (&V)
menu-format-table-row-below = @ info: whatsthis (&W)
menu-format-table-col-before = Inserează coloană înainte (&L)
menu-format-table-col-after = Inserează coloană după (&T)
menu-format-table-row-delete = Șterge rândul (&D)
menu-format-table-col-delete = Șterge coloana (&C)
menu-format-table-remove = Elimină tabel (&R)

menu-format-typo-marks = Formatarea mărcii (&F)
menu-format-typo-nbsp = Spațiu fără pauză (&N)
menu-format-typo-nbhyphen = Hyphen fără ruptură (&H)
menu-format-typo-soft-hyphen = Hifen moale (&S)
menu-format-typo-nnbsp = Spaţiu îngust fără ruptură (&R)
menu-format-typo-zwsp = Spațiu cu lățime zero (&Z)
menu-format-typo-word-joiner = Word Joiner (&W)


menu-about = Cam Skribisto... (&A)
about-title = Despre Skribisto
about-version = Versiunea { $version }
about-update-available = Versiunea { $version } este disponibilă
about-update-see-notes = Vezi ce s-a schimbat.
about-update-flathub = Flathub ţine această copie la zi.
about-update-distro = Distribuţia ta ţine această copie la zi.
about-tagline = O aplicație de scriere de romane pentru ficțiune de lungă formă, scrisă în Rust cu trusa de instrumente Teksilo.
about-license = Eliberat sub licenţa publică generală GNU, versiunea 3.
about-copyright = © 2026 Cyril Jacquet
about-close = Închidere

native-menu-about = Despre { $app }
native-menu-hide = A se ascunde { $app }
native-menu-quit = Renunță la { $app }
native-menu-settings = Setări…
native-menu-window = Fereastră
native-menu-minimize = Minimizează
native-menu-zoom = Măreşte

window-title = { $title } - { $app }
window-title-numbered = { $title } - { $app } (Vânt { $n })
window-title-empty = { $app }

first-run-window-title = Bine ai venit.
first-run-title = Configurați { $app }
first-run-body = { $app } păstrează setările sale separat de Skribisto, astfel încât începe gol. Preferințele dumneavoastră, proiecte recente, dicționare și dispunerea ferestrei pot fi copiate peste acum.
first-run-from = Copiază de la
first-run-to = Copiază în
first-run-copy-note = Nimic nu este mutat sau eliminat. Skribisto păstrează fiecare dintre setările sale proprii, și merge mai departe de lucru exact ca înainte.
first-run-import = Importă configurări
first-run-start-fresh = Începeți din nou
first-run-import-failed = Unele setări nu au putut fi importate: { $error }

settings-page-punctuation = Implicite punctuație
settings-group-punctuation = Punctuație inteligentă
settings-page-work-punctuation = Punctuație
settings-punctuation-override = Oferă acestui proiect propriile reguli de punctuaţie
settings-punctuation-override-hint = Off: proiectul urmează preferințele de aplicare. Aceste reguli călătoresc în interiorul .skrib, astfel încât un co-autor care deschide fișierul scrie cu aceeași tipografie.
settings-punctuation-dashes = Transformă-te într-o liniuţă.
settings-punctuation-ellipsis = Transformă-te... într-o elipsă.
settings-punctuation-quotes = Mărcile curl și apostrofe
settings-quote-style = Mărcile de cotaţie
settings-quote-style-locale = Limba implicită
settings-quote-style-curly = "Dublu"
settings-quote-style-curly-single = bază"
settings-quote-style-guillemets = Guillemets
settings-quote-style-low-high = "Low-high"
settings-punctuation-spacing = Spaţiu înainte; : ?
settings-punctuation-spacing-hint = Tipografia franceză stabilește un spațiu îngust fără pauză înainte de ; ? ? și unul plin înainte de :. Se aplică numai la textul scris în limba franceză. Spațiul din interiorul guillemets
settings-punctuation-sample = Limba ta da
settings-punctuation-app-hint = Ceea ce face fiecare proiect dacă nu are propriile reguli în Proiectul de Punctuație.
settings-punctuation-dialogue = Deschideți un paragraf tastat ca "-" cu o liniuță de dialog
settings-punctuation-dialogue-hint = Pentru limbile care marchează vorbirea cu o liniuță mai degrabă decât marcajele de cotare: franceză, spaniolă, rusă și altele. Se trage doar la începutul unui paragraf.

statusbar-go-to = Du-te la...
go-to-placeholder = Caută în liant
go-to-no-matches = Niciun obiect nu se potriveşte cu acea căutare.
menu-go-to = Du-te la... (&G)

settings-page-distraction-free-themes = Teme lipsite de distragere
settings-themes-builtin = Teme construite
settings-themes-builtin-badge = Construit
settings-themes-user = Temele mele
settings-themes-editor-group = Editează tematica
settings-themes-editor-empty = Alege o temă sub "Temele mele" pentru a o edita.
settings-themes-use = Utilizare
settings-themes-duplicate = Duplicare
settings-themes-edit = Editare
settings-themes-delete = Ștergere
settings-themes-export = Exportare…
settings-themes-import = Importare…
settings-themes-copy-suffix = Copiere
settings-themes-json-filter = Tematică (JSON)
settings-themes-imported = Tematică importată
settings-themes-import-failed = Nu am putut importa această temă
settings-themes-exported = Tematică exportată
settings-themes-export-failed = Nu s-a putut exporta această temă
settings-themes-low-contrast = contrast scăzut
settings-themes-low-contrast-band = evidenţiază ascunde textul
settings-themes-field-name = Nume
settings-themes-field-paper = Pagina
settings-themes-field-ink = Text
settings-themes-field-general = Context
settings-themes-field-widget-text = Controlează textul benzilor
settings-themes-field-caret-band = Evidențiază în jurul îngrijirii

statusbar-focus-settings = Setări fără distragere
statusbar-focus-manage-themes = Gestionează temele...

comments-title = Comentarii
comments-document-title = Prezentul document
comments-empty-project = Nu există comentarii în acest proiect încă.
comments-empty-document = Nu există observații cu privire la acest document.
comments-filter-all = Toate
comments-filter-open = Deschidere
comments-filter-resolved = Rezolvat
comments-filter-orphaned = Orfan
comments-status-open = Deschidere
comments-status-resolved = Rezolvat
comments-status-orphaned = Şi-a pierdut textul.
comments-orphan-snippet = (textul comentat a dispărut)
comments-status-unplaced = Nicio poziție text
comments-unplaced-snippet = (neancorate la nici un text)
comments-reply-count = { $count ->
    [one] Răspuns: { $count }
   *[other] { $count } răspunsuri
}
comments-menu-resolve = Rezolvare
comments-menu-reopen = Redeschide
comments-menu-delete = Ștergere
comments-sort-document = În ordinea documentelor
comments-sort-newest = Cel mai nou primul
comments-sort-reader = Grupat de cititor
comments-unsigned-author = Nesemnată
comments-agreement = { $count ->
    [one] Cititor { $count }
   *[other] { $count } cititori
}
comments-menu-add = Adaugă comentariu
comments-menu-add-paragraph = Comentariu cu privire la acest paragraf
overview-col-comments = Comentarii
overview-col-total-comments = Total observații
comments-card-placeholder = Scrie un comentariu...
comments-card-unknown-author = Autor necunoscut
comments-unsigned-toast = Comentariile dvs. sunt nesemnate: nici un nume nu este setat pentru tine pe acest calculator.
comments-unsigned-action = Setează-ţi numele.
comments-card-reply = Răspunde
comments-card-reply-placeholder = Răspunde...
comments-card-actions = Comentează acțiunile
comments-reply-actions = Răspunde la acțiuni
comments-menu-delete-reply = Șterge răspunsul
comments-menu-delete-all = Șterge toate comentariile aici
comments-deleted-toast = Comentariu șters
comments-reply-deleted-toast = Răspunsul sters
comments-deleted-all-toast = { $count ->
    [one] Comentariu { $count } eliminat
   *[other] Observații eliminate { $count }
}
comments-undo = Anulare acțiune

analysis-segment = Analiză
analysis-scope-book = Analizând această carte
analysis-run = Analiza rulării
analysis-stale = S-a schimbat de când a fugit asta.
analysis-not-run = Încă nu s-a analizat.
analysis-running = Citind manuscrisul...
analysis-failed = Analiza nu a putut termina.
analysis-no-scenes = Nu există scene în această carte încă.

analysis-shape = Forma

analysis-arrivals = Sosiri
analysis-arrivals-explainer = Cum textul a ajuns la acest proiect în timp ce a fost deschis. Se spune care traseu personajele au venit în jos, și nimic despre care le-a scris: un scriitor care proiectează în altă parte și paste a lipit, și unul care dictează a dictat. Nu există nici un număr pentru a viza aici.
analysis-arrivals-scope = Întregul proiect, nu doar această carte.
analysis-arrivals-session = De când l-ai deschis, închiderea proiectului începe din nou numărătoarea.
analysis-arrivals-nothing = Nici un text nu a sosit încă în această sesiune.
analysis-arrivals-typed = Tipărit
analysis-arrivals-pasted = Pastate
analysis-arrivals-dictated = Dictate
analysis-arrivals-imported = Importat
analysis-arrivals-programmatic = Introduceți pentru dvs.
analysis-arrivals-count = { $count ->
    [one] { $count } caracter
    [few] { $count } caractere
   *[other] { $count } de caractere
}
analysis-arrivals-none = Niciunul

analysis-words-per-scene = Cuvinte pe scenă
analysis-median-words = Scena mediană a acestei cărţi se desfăşoară { $count ->
    [one] { $count } cuvânt.
   *[other] { $count } cuvinte.
}
analysis-median-line = Mediană: { $count ->
    [one] { $count } cuvânt
   *[other] { $count } cuvinte
}
analysis-dialogue = Dialog
analysis-dialogue-unsupported = Dialogul nu este încă măsurat pentru această limbă.

analysis-footnote-words = Cuvinte de subsol
analysis-footnote-words-count = { $count ->
    [one] { $count } se află în notele de subsol ale acestei cărţi, ţinute departe de manuscrisul total.
   *[other] În notele de subsol ale acestei cărţi sunt menţionate { $count } de cuvinte, ţinute departe de manuscrisul total.
}
analysis-footnote-words-pending = Numărând notele de subsol...

analysis-ignore-empty = Ignoră textele cu nimic scris încă
analysis-empty-hidden = { $count } gol { $count ->
        [one] Text
       *[other] texte
    } Ascuns.
analysis-all-texts-empty = Fiecare text din această carte este încă gol.

binder-filter-count = { $shown } din { $total }
binder-filter-clear = Golire
binder-filter-none = Nimic nu se potriveşte cu "{ $query }."

settings-paratext-intro = Fiecare structură aparţine unei tradiţii de publicare, iar titlurile sale de pagină sunt scrise în limba proprie a tradiţiei. Redenumeşte-le liber odată ce un proiect este creat.
settings-paratext-structures = Structuri
settings-paratext-broken = Nu se poate citi
settings-paratext-edit = Editare
settings-paratext-duplicate = Duplicare
settings-paratext-delete = Ștergere
settings-paratext-new = Structura nouă
settings-paratext-save = Salvare
settings-paratext-editor-hint = Un nume, paginile pentru a crea înainte de manuscris, și paginile pentru a crea după el. În cazul în care acestea ajung este a ta: acestea sunt elemente obișnuite odată ce proiectul există.


image-insert = Inserează imagine... (&M)
image-no-project = Deschide un proiect înainte de inserarea unei imagini.
image-choose-title = Alege o imagine
image-filter-label = Imagini
image-large-title = Aceasta este o imagine mare
image-large-text =
    { $name } este { $megapixels } megapixeli ({ $width }×{ $height }).
    Păstrarea acestuia așa cum este stochează fișierul original în proiect: călătorește
    cu fiecare rezervă și fiecare export. Optimizarea stochează o copie mai mică
    în schimb, până la 2560 pixeli pe partea cea mai lungă.
image-large-keep = Păstrează originalul
image-large-downscale = Optimizează
image-large-remember = Fă asta de acum încolo, nu întreba din nou.
image-not-recorded = Imaginea a fost salvată, dar nu a putut fi înregistrată în proiect.
image-describe-title = Descrie imaginea
image-describe-explain = Ceea ce arată imaginea, pentru un cititor care nu o poate vedea. Nu face parte din manuscris: nu este niciodată numărată, căutată sau exportată ca proza.
image-describe-placeholder = un far împotriva unui cer gri
image-resize-title = Redimensionează imaginea
image-resize-explain = Un procent din mărimea pe care o are acum. 100 îl lasă aşa cum este.
image-resize-invalid = Introduceți un număr între 1 și 1000.
image-menu-describe = Descrie imaginea... (&D)
image-menu-resize = Redimensionează imaginea... (&R)
image-menu-reset-size = Dimensiunea originală (&Z)

cover-choose = Coperta de carte... (&V)
cover-clear = Îndepărtează capacul (&R)
cover-choose-title = Alege o copertă
cover-set = Coperta e aranjată.
cover-cleared = Coperta a fost scoasă. Imaginea este încă în proiect.
card-label = Descriere scurtă
card-exportable = Exportat
card-numbered = Numerotat
card-created = Creat
card-modified = Modificat
card-yes = Da
card-no = Nu
card-type = Tip
card-position = Poziţie
card-children = Copii
card-goal = Obiectiv
card-words = Cuvinte
card-synopsis = Rezumat
card-point-of-view = Punct de vedere
card-aliases = Cunoscut şi sub numele de

export-orphan-footnotes-title = Note de subsol fără referință
export-comments-dropped =
    { $count ->
        [one] Un comentariu nu a putut fi introdus în textul exportat și a fost omis.
       *[other] Observațiile { $count } nu au putut fi introduse în textul exportat și au fost omise.
    }
export-orphan-footnotes =
    { $count ->
        [one] O notă de subsol nu mai este menționată nicăieri în manuscris. Textul său nu va mai apărea în cartea exportată.
       *[other] { $count } note de subsol nu mai sunt menționate nicăieri în manuscris. Textul lor nu va mai apărea în cartea exportată.
    }


menu-footnotes = Note de subsol (&N)
footnotes-title = Note de subsol
footnotes-insert = Inserează notă de subsol (&F)
footnotes-empty = Nu există note de subsol încă. Pune cursorul într-o scenă, apoi utilizați + de mai sus, sau Ctrl+Alt+F.
footnotes-filter-all = Toate
footnotes-filter-document = Prezentul document
footnotes-filter-orphaned = Orfan
footnotes-orphaned = Nimic nu mai indică această notă.
footnotes-untitled-home = Fără titlu
footnotes-body-placeholder = nota în sine
footnotes-insert-tooltip = Se introduce o notă de subsol la cursor (Ctrl+Alt+F)
footnotes-actions = Acţiuni de subsol
footnotes-delete = Se elimină nota și trimiterea acesteia (&D)
footnotes-no-project = Deschide un proiect înainte de a introduce o notă de subsol.
footnotes-no-caret = Pune cursorul în textul unei scene pentru a introduce o notă de subsol acolo.
footnotes-not-created = Nota de subsol nu a putut fi adăugată la proiect.
footnotes-deleted-toast = Notă eliminată
footnotes-undo-delete = Anulare acțiune

import-document-title = Documente de import
import-document-close = Închidere
import-document-step-files = Fișiere
import-document-step-review = Revizuire
import-document-step-destination = Destinația
import-document-step-reconcile = Combină
import-document-reconcile-hint = Unele dintre aceste elemente sunt cele pe care le au deja. Spune ce ar trebui să se întâmple cu fiecare.
import-document-reconcile-all-new = Nimic din acest fișier se potrivește proiectului dvs.: fiecare element va fi adăugat ca nou.
import-document-duplicate-returns = Aceste fișiere sunt mai mult de o copie a aceluiași manuscris vine înapoi. Importați-le unul la un moment dat: potrivirea doar o singură copie de întoarcere împotriva proiectului, astfel încât importul mai multe împreună ar adăuga o a doua copie a cărții în loc de fuzionarea acestuia.
import-document-hunk-added = Ia acest nou paragraf
import-document-hunk-removed = Renunță la acest paragraf
import-document-hunk-changed = Ia această rescriere
import-document-col-stray-prose = Textul său
import-document-stray-as-paratext = Păstrați ca paratext
import-document-stray-discard = Aruncă textul
import-document-col-current = În proiectul dumneavoastră
import-document-col-incoming = În acest fișier
import-document-col-status = Stare
import-document-col-action = Ce e de făcut?
import-document-status-identical = La fel.
import-document-status-editor-edited = L-au editat.
import-document-status-you-edited = L-ai editat.
import-document-status-conflict = Amândoi l-au editat.
import-document-status-different = Diferite
import-document-status-new = Nou
import-document-status-missing = Nu în acest fișier
import-document-status-moved = Mutat
import-document-action-comments-only = Numai comentarii
import-document-action-take-import = Ia această versiune
import-document-action-keep-current = Păstrează-l pe al meu.
import-document-action-create-new = Adaugă ca nou
import-document-action-ignore = Sari
import-document-compare = Compară
import-document-compare-legend = Versiunea ta în comparaţie cu a acestui fişier.
import-document-compare-close = Închidere
import-document-drop-title = Drop documente aici
import-document-drop-hint = Markdown (.md), Word (.docx), OpenDocument (.odt) și text simplu (.txt)
import-document-browse = Răsfoieşte...
import-document-move-up = Mutare în sus
import-document-move-down = Mutare în jos
import-document-remove-file = Eliminare
import-document-file-count = { $count ->
    [one] 1 fișier
   *[other] Fișiere { $count }
}
import-document-no-files = Nu există fișiere alese încă.
import-document-col-included = Importare
import-document-col-title = Titlu
import-document-col-type = Tip
import-document-col-words = Cuvinte
import-document-col-breaks = Pauzele
import-document-col-comments = Comentarii
import-document-col-footnotes = Note de subsol
import-document-col-epigraph = Motto
import-document-col-source = Sursă
import-document-level-rules = Niveluri de rubrici
import-document-level-n = Poziția { $level }
import-document-add-top-level = Adaugă nivel superior
import-document-add-top-level-tooltip = Introduceți o carte de mai sus fiecare element analizat, pentru fișiere de capitol care nu au o poziție de carte
import-document-destination = Destinația
import-document-destination-hint = Alegeți un liant sau un element: elemente noi aterizează în interiorul unui dosar, sau după o scenă.
import-document-destination-empty = Nu există lianți încă. Creați unul mai întâi.
import-document-plan-empty = Nimic de import încă.
import-document-summary = { $rows ->
    [one] 1 element
   *[other] { $rows } articole
} · { $breaks ->
    [one] 1 pauza scena
   *[other] { $breaks } pauzele scenei
}
import-document-back = Înapoi.
import-document-cancel = Anulare
import-document-analyse = Următorul
import-document-analysing = Citesc documente...
import-document-step-analysing = Citire
import-document-cancel-analysis = Nu mai citi.
import-document-analyse-failed = Documentele nu au putut fi citite.
import-document-details = Detalii
import-document-import = Importare
import-document-done = { $count ->
    [one] 1 produs importat
   *[other] { $count } articole importate
}
import-document-undo = Anulare acțiune
import-diagnostic-file-unreadable = "{ $path }" nu a putut fi citit: { $detail }. Celelalte fişiere încă mai importă.
import-diagnostic-nested-too-deep = "{ $path }" nu a fost citit: conţinutul său este cuibărit mai mult de { $limit } niveluri adânci. Nici un document real se apropie de faptul că, şi lectură ar putea accident Skribisto, aşa că este lăsat afară. Celelalte fişiere încă importă.
import-diagnostic-archive-too-large = "{ $path }" nu a fost citit: este un fişier comprimat construit pentru a despacheta mult mai mare decât este, care ar putea epuiza memoria şi accident Skribisto, aşa că este lăsat afară. Celelalte fişiere încă importă.
import-diagnostic-lossy-decode = { $count ->
    [one] Un personaj din "{ $path }" nu a decodat. Re-salvați fișierul ca UTF-8 pentru a-l păstra.
   *[other] { $count } caractere în "{ $path }" nu a decodat. Re-salvare fișierul ca UTF-8 pentru a le păstra.
}
import-diagnostic-decoded-from-bom = "{ $path }" a fost decodat ca { $detail }, nu UTF-8.
import-diagnostic-empty-file = "{ $path }" este gol.
import-diagnostic-no-headings = "{ $path }" nu are rubrici, așa că ajunge ca un singur element.
import-diagnostic-unsupported-format = Nimic nu citeste ".{ $detail }" fisiere, asa ca "{ $path }" a fost sarit.
import-diagnostic-front-matter-not-flat = Materia frontală din "{ $path }": "{ $detail }" nu este o valoare simplă și a fost omisă.
import-diagnostic-footnotes-degraded = { $count ->
    [one] O notă de subsol din "{ $path }" ajunge ca text simplu: notele de subsol nu se citesc din Markdown.
   *[other] { $count } note de subsol în documentul "{ $path }": notele de subsol nu se citesc din documentul Markdown.
}
import-diagnostic-footnote-not-carried = { $count ->
    [one] O notă de subsol din "{ $path }" nu a putut fi preluată.
   *[other] { $count } note de subsol în "{ $path }" nu a putut fi adus peste. Celelalte au fost.
}
import-diagnostic-raw-html-dropped = { $count ->
    [one] Un bloc de HTML brute în "{ $path }" a fost scăzut.
   *[other] Au fost retrase { $count } blocuri de HTML brute în "{ $path }."
}
import-diagnostic-nested-break-dropped = { $count ->
    [one] O spargere în interiorul unui citat sau listă în "{ $path }" a fost scăzut. Doar o pauză pe propria linie este păstrat.
   *[other] Scena { $count } sparge în interiorul ghilimele sau liste în "{ $path }" au fost retrase. Doar o pauză pe propria linie este păstrat.
}
import-diagnostic-image-not-ingested = "{ $path }" se referă la imaginea "{ $detail }." Referinţa ajunge ca text; imaginea în sine nu este copiată.
import-diagnostic-duplicate-title = "{ $title }" apare { $count }  Dacă ați importat aceste fișiere înainte, acest lucru le va duplica.
import-diagnostic-heading-level-jump = "{ $title }" sare de la nivelul poziției { $from } la { $to }; este plasat la un nivel sub părintele său.
import-diagnostic-illegal-combination = "{ $title }" poartă proza, dar un { $kind } nu poate deţine nici una. Importul este deţinut până când nu se schimbă tipul său sau să-l lase afară. Nimic nu este importat, la toate celelalte.
import-diagnostic-tracked-changes-flattened = { $path } era la jumătatea vizionării: { $count ->
    [one] { $count } modificare urmărită a fost acceptată
   *[other] { $count } modificări urmărite au fost acceptate
} Pentru a păstra oricare dintre propriile formulări în schimb, utilizați Comparați în ultimul pas.
import-diagnostic-tracked-changes-flattened-by = { $path } era la jumătatea vizionării: { $count ->
    [one] { $count } modificare urmărită de { $names } a fost acceptată
   *[other] { $count } modificări urmărite de { $names } au fost acceptate
} Pentru a păstra oricare dintre propriile formulări în schimb, utilizați Comparați în ultimul pas.
import-diagnostic-text-box-dropped = { $count ->
    [one] { $path } deţine cutia de text { $count }. Textul său se află în afara fluxului documentului, astfel încât locul său într-un manuscris nu poate fi răspuns. Nu este importat.
   *[other] { $path } deține { $count } cutii text. Textul lor stă în afara fluxului documentului, astfel încât în cazul în care acesta aparține într-un manuscris nu poate fi răspuns.
}
import-diagnostic-embedded-object-dropped = { $count ->
    [one] { $path } deține { $count } obiect încorporat: o diagramă, o ecuație sau similar. Nu există nimic într-un manuscris care ar putea ține.
   *[other] { $path } deține { $count } obiecte încorporate: o diagramă, o ecuație sau similare. Nu există nimic într-un manuscris care le-ar putea deține.
}
import-diagnostic-field-flattened = { $count ->
    [one] { $path } deţine { $count } teren: un număr de pagină, o referinţă încrucişată, o dată. Păstrează textul pe care l-a prezentat ultima dată şi nu se va actualiza din nou.
   *[other] { $path } deţine { $count } câmpuri: un număr de pagină, o referinţă încrucişată, o dată. Fiecare păstrează textul pe care l-a prezentat ultima dată şi nu se va actualiza din nou.
}
import-diagnostic-unknown-style-level = { $path } foloseste stilul "{ $detail }," care arata ca un titlu, dar nume nici un nivel. Aceste paragrafe sunt importate ca proza, mai degrabă decât ghicit la o adâncime.
import-diagnostic-comment-unanchored = Comentariul "{ $detail }" din { $path } nu a putut fi atașat la cuvintele despre care a fost vorba. Acesta este păstrat pe elementul său, în cazul în care îl puteți muta.
import-diagnostic-comment-not-carried = Comentariu "{ $detail }" în { $path } nu este importat, pentru că nimic din fișier nu ajunge ca text ar putea fi păstrat pe. Copiați-l din fișier, dacă aveți nevoie.
import-diagnostic-comment-replies-flattened = { $count ->
    [one] { $count } răspuns în { $path } numit un comentariu care nu este în dosar, astfel încât acesta ajunge ca un comentariu al său.
   *[other] { $count } răspunsuri în { $path } au numit un comentariu care nu figurează în dosar, astfel încât acestea sosesc ca comentarii proprii.
}
import-diagnostic-prose-not-verbatim = { $count ->
    [one] { $count } punctul { $path } nu a putut fi adus cu formatul exact așa cum este scris, astfel încât ajunge ca text simplu. Comparați-l cu originalul.
   *[other] { $count } paragrafele din { $path } nu au putut fi aduse cu formatarea lor exact așa cum este scris, astfel încât acestea ajung ca text simplu. Comparați-le cu originalul.
}
import-diagnostic-styled-spaces-not-carried = { $count ->
    [one] În { $path }, s-a subliniat sau s-a lovit o porţiune de spaţiu gol de { $count }, o linie rămasă de completat, de exemplu. Skribisto nu menţine nici o linie numai pe spaţii, astfel încât ajunge ca spaţii simple, sau deloc în cazul în care începe sau se termină un paragraf.
   *[other] În { $path }, { $count } întinderi de spaţiu gol au fost subliniate sau lovite prin, liniile rămase pentru a completa, de exemplu. Skribisto nu păstrează nici o linie pe spaţii singur, astfel încât acestea ajung ca spaţii simple, sau deloc în cazul în care acestea încep sau se încheie un paragraf.
}
import-diagnostic-list-nesting-flattened = { $count ->
    [one] { $count } list item în { $path } a fost cuibărit mai mult { $limit } niveluri adânci. Acesta ajunge la nivelul { $limit }, cel mai profund Skribisto păstrează, cu cuvintele sale și formatarea.
   *[other] { $count } lista de elemente în { $path } au fost cuibărite mai mult de { $limit } niveluri adânci. Acestea ajung la nivelul { $limit }, cele mai profunde Skribisto păstrează, cu cuvintele lor și formatarea.
}
import-diagnostic-table-read-as-paragraphs = { $count ->
    [one] Tabelul { $count } din { $path } are un rând mult mai larg decât restul, prea dezechilibrat pentru a stabili ca o grilă. Celulele sale ajung ca paragrafe, unul după altul, cu cuvintele și formatarea lor.
   *[other] { $count } tabele în { $path } au un rând mult mai larg decât restul, prea slăbit pentru a stabili ca o grilă. Celulele lor ajung ca paragrafe, unul după altul, cu cuvintele și formatarea lor.
}
import-diagnostic-spaces-shortened = { $count ->
    [one] { $count } rulează spaţii în { $path } a fost mai mult de { $limit } spaţii, şi ajunge redus la { $limit }. Cuvintele din jurul ei sunt toate acolo.
   *[other] { $count } ruleaza spatii in { $path } au fost mai lungi de { $limit } spatii, si ajung la { $limit }. Cuvintele din jurul lor sunt toate acolo.
}
import-diagnostic-epigraph-not-carried = "{ $title }" este condus de un epigraf, dar un { $kind } nu poate deține unul. Citatul este păstrat în partea de sus a textului său în schimb.
import-diagnostic-epigraph-placement-ambiguous = Un epigraf stă între "{ $title }" și "{ $below }" și ar putea cap fie. Acesta a fost dat la "{ $title }," care este în cazul în care un epigraf merge, de obicei.
import-document-diagnostics = { $errors ->
    [0] { $warnings ->
            [one] Un lucru de ştiut.
           *[other] { $warnings } lucruri de ştiut
        }
   *[other] { $errors ->
            [one] 1 fișier nu a putut fi citit
           *[other] Fișierele { $errors } nu au putut fi citite
        }
}
import-document-diagnostics-none = Nimic de raportat.

versions-title = Versiuni
versions-scope-synopsis = Rezumat
versions-scope-prose = Text
versions-loading = Privind prin backup-uri...
versions-empty = Nici o versiune mai devreme a acestui încă
versions-error = Nu s-a pierdut nimic, dar lista asta poate fi incompletă.
versions-did-not-exist = Nu a existat încă pe { $date }
versions-deleted-after = Șters la un moment dat după { $date }
versions-unreadable = { $count ->
    [one] Întăririle { $count } nu au putut fi citite.
   *[other] Întăririle { $count } nu au putut fi citite.
}
versions-thinned = { $count ->
    [one] Versiuni mai vechi subţiază pe măsură ce îmbătrânesc. Istoria proiectului a scăzut deja { $count } starea anterioară a acestui text.
   *[other] Versiuni mai vechi subţiază pe măsură ce îmbătrânesc. Istoria proiectului a scăzut deja { $count } stări anterioare ale acestui text.
}
versions-source-backup = De la o copie de rezervă
versions-source-project = Din istoria proiectului
versions-list-caption = O intrare per schimbare, nu per rezervă
versions-pick-a-version = Alege o versiune pentru a vedea ce sa schimbat
versions-earliest = Nu e nimic mai vechi cu care s-o compari.
versions-no-change = Nimic nu sa schimbat în această parte a elementului
versions-formatting-only = Doar formatarea s-a schimbat aici.
versions-show-unchanged = Afișează neschimbat
versions-hide-unchanged = Ascunde neschimbat
versions-next-change = Următoarea schimbare
versions-near = în apropiere de "{ $text }"
versions-words-added = { $count ->
    [one] { $count } cuvânt adăugat
   *[other] { $count } cuvinte adăugate
}
versions-words-removed = { $count ->
    [one] { $count } cuvânt eliminat
   *[other] { $count } cuvinte eliminate
}
versions-blocks-moved = { $count ->
    [one] Punctul { $count } mutat
   *[other] Punctele { $count } mutate
}
versions-pin = Pin backup această versiune a venit de la. Curățare automată nu va șterge niciodată
versions-unpin = Unpin backup această versiune a venit de la. Curățare automată poate șterge din nou
versions-pinned-only = Afișează numai versiunile fixate
versions-pin-note = Doar versiunile dintr-o copie de rezervă pot fi blocate. Un ac păstrează un fișier, iar propria istorie a proiectului nu este una.
versions-pinned-empty = Nimic de aici nu e încă prins.
versions-pinned-empty-log = Nimic aici nu este fixat încă. Doar versiunile dintr-o copie de rezervă pot fi fixate. Un pin păstrează un fișier, iar propria istorie a proiectului nu este una.
versions-range-filter = Arată doar versiunile între două date
versions-filtered-empty = Nicio versiune nu se potrivește filtrelor pe care le-ați setat
versions-clear-filters = Curățați filtrele
versions-last-30-days = Ultimele 30 de zile
versions-restore-button = Restaurează această versiune
versions-restore-confirm-title = Înlocuiți acest text cu versiunea de la { $date }?
versions-restore-confirm-text = Ceea ce aveți acum va fi înlocuit cu textul pe care acest articol l-a avut la { $date }.
versions-restore-confirm-with-comments = Ceea ce aveți acum va fi înlocuit cu textul pe care acest articol l-a avut la { $date }. { $count ->
    [one] { $count } comentariu este ancorat în textul actual și poate fi lăsat orfan.
   *[other] Observațiile { $count } sunt ancorate în textul actual și pot fi lăsate orfane.
}
versions-restore-confirm-undo-note = O copie de rezervă se face în primul rând, și Ctrl+Z anulează acest lucru într-un singur pas.
versions-restored-toast = Restaurat versiunea de la { $date }
versions-undo = Anulare acțiune
versions-restore-row-gone = Acest element nu mai este în acest proiect
versions-restore-no-home = Acest articol a schimbat tipul de atunci, și textul vechi nu are unde să meargă în ea
versions-restore-no-safety-copy = Întăririle tale nu au funcţionat, aşa că nu s-a schimbat nimic.
versions-restore-failed = Restaurarea a eșuat: { $error }
versions-restore-backup-busy = O copie de rezervă este deja difuzate. Încearcă din nou într-un moment
versions-restore-in-backup-file = Te uiți la un fișier de rezervă; deschide proiectul în sine pentru a restabili în ea
versions-restore-no-project = Niciun proiect nu este deschis
versions-recreate-button = Adu asta înapoi...
versions-recreate-picker-title = Unde ar trebui să se ducă?
versions-recreate-picker-empty = Acest proiect nu are nici un liant să-l pună în
versions-recreate-picker-confirm = Adu-l înapoi aici.
versions-recreate-picker-cancel = Anulare
versions-recreate-untitled = Acest punct
versions-recreate-confirm-title = Aduce "{ $item }" înapoi?
versions-recreate-confirm-text = Acesta va fi adăugat la { $destination }, cu textul a avut pe { $date }.
versions-recreate-confirm-undo-note = Anulează, pe mesajul care urmează, ia-l înapoi.
versions-recreated-toast = "{ $item }" este din nou în proiectul dumneavoastră
versions-recreated-partial-toast = { $count ->
    [one] "{ $item }" este din nou, dar unul dintre textele sale nu a putut fi citit
   *[other] "{ $item }" s-a întors, dar { $count } din textele sale nu au putut fi citite
}
versions-recreate-already-here = Acest element este deja în proiectul dumneavoastră
versions-recreate-no-destination = Alege undeva în liant să-l pună
versions-recreate-unreadable = Întăririle n-au putut fi citite, aşa că nu s-a adăugat nimic.
versions-recreate-failed = Nu a putut aduce înapoi: { $error }
versions-changed-percent = { $percent }% din această modificare
versions-hidden-paragraphs = { $count ->
    [one] ... { $count } paragraf neschimbat ...
   *[other] ... { $count } paragrafe neschimbate ...
}

timeline-title = Du-te înapoi în timp
timeline-coverage = { $count ->
    [one] Versiunea { $count } înregistrată, revenind la { $oldest }
   *[other] Versiuni { $count } înregistrate, revenind la { $oldest }
}
timeline-loading = Privind prin trecutul proiectului tău...
timeline-empty = Nici o versiune a acestui proiect nu a fost înregistrată încă
timeline-no-changes = Nimic nu s-a schimbat de atunci.
timeline-no-text-changes = Niciun text nu s-a schimbat de atunci.
timeline-slider-label = Versiunea înregistrată
timeline-series-name = Dimensiunea proiectului
timeline-bars-caption = Fiecare bar este o versiune înregistrată, la fel de înaltă ca proiectul a fost atunci. Cel subliniat este ceea ce se uită la.
timeline-bars-caption-periods = Prea multe versiuni pentru a arăta unul câte unul, astfel încât fiecare bar este un { $period }, la fel de înalt ca proiectul a fost până la sfârșitul acestuia.
timeline-unit-hour = oră
timeline-unit-day = zi
timeline-unit-week = săptămână
timeline-unit-month = lună
timeline-range-filter = Arată doar versiunile între două date
timeline-range-empty = Nicio versiune nu a fost înregistrată în aceste date
timeline-open-period = Deschide această perioadă
timeline-show-all = Arată întreaga istorie
timeline-changed-since = { $count ->
    [one] { $count } element diferă între { $date } și proiectul dumneavoastră acum
   *[other] { $count } elemente diferite între { $date } și proiectul dvs. acum
}
timeline-kind-added = Scris de atunci
timeline-kind-removed = Nu mai sunt în proiect.
timeline-kind-changed = Editat de atunci
timeline-kind-moved = Mutat de când
timeline-not-yet-written = Acest lucru nu a existat încă în acel moment
timeline-no-text-of-its-own = Acest lucru nu are nici un text propriu. Este o direcție pentru ceea ce este în interiorul ei
timeline-reader-close = Închidere
timeline-reader-stamp = Așa cum a fost pe { $date }
timeline-reader-loading = Deschiderea versiunii înregistrate...
timeline-reader-unreadable = Această înregistrare nu a putut fi citită. Întăririle pot fi mutate, șterse sau pe o unitate care nu este conectată.
timeline-reader-compared = Așa cum a fost pe { $date }, setat împotriva a ceea ce spune acum
timeline-reader-diff-legend = Lovit prin a trecut de atunci; subliniat a fost adăugat.
timeline-reader-view-label = Ce să arate
timeline-reader-view-diff = Modificări
timeline-reader-view-text = Text
timeline-reader-deleted = Asta nu mai e în proiectul tău, îl poţi citi şi copia aici.
timeline-prose-only-record = Acest punct vine din propria istorie a proiectului, care păstrează textul și nimic altceva, așa că ceea ce a fost șters sau mutat, deoarece nu poate fi afișat. Alege o copie de rezervă pentru asta.

settings-sec-extensions = Extensii

settings-group-games-forward = Întotdeauna înainte
settings-games-forward-toggle = Joacă "Întotdeauna înainte"
settings-games-forward-hint =
    În timp ce jucați, nimic din ce ați scris nu poate fi luat înapoi: Backspace, Ștergeți,
    Tăiați, trageți-și-drop și Anulați sunt toate dezactivate în suprafețele pe care le alegeți mai jos.
    Încă mai poţi tasta, lipi, formata şi să te mişti liber.
settings-games-session-warning =
    Aceasta este o alegere per-sesiune: nu este niciodată salvat. Închiderea proiectului, sau demisia
    Skribisto, întotdeauna se termină jocul, și tot ce ai scris în timp ce joc devine
    Se poate anula din nou în momentul în care o face.
settings-group-games-scope = În cazul în care se aplică
settings-games-in-prose = Proza manuscrisă
settings-games-in-synopsis = Sinoduri
settings-games-scope-hint =
    Comentarii, note de subsol și titluri nu sunt niciodată înghețate: acestea sunt în cazul în care notați fix
    Tocmai ţi-ai interzis să faci asta.
settings-games-inert-warning =
    "Întotdeauna înainte" este pe, dar se aplică la nimic. Tic cel puțin o suprafață de mai sus,
    sau nu schimbă nimic în timp ce scrii.

games-title = Scrierea de jocuri
games-forward-name = Întotdeauna înainte
games-forward-blurb =
    Proiectare fără a lua nimic înapoi. Ștergerea, tăierea și desfacerea sunt dezactivate
    Scrie următoarea propoziţie în loc să o repari pe ultima.
games-forward-playing = Redare: Ștergerea este dezactivată
games-forward-idle = Nu joc
games-session-note = Se termină când închizi proiectul.
games-scope-prose-and-synopsis = Se aplică prozei şi sintopezelor tale.
games-scope-prose = Se aplică prozei tale.
games-scope-synopsis = Se aplică sintopselor tale.
games-scope-nothing = Se aplică la nimic încă. Alege o suprafață în Setări.
games-settings-link = Scrierea setărilor de joc...
statusbar-games-forward = Întotdeauna înainte
statusbar-games-forward-tooltip =
    "Întotdeauna înainte" este pe: ștergerea, tăierea și desfacerea sunt dezactivate în timp ce
    Faceți clic pentru a opri jocul.
goal-progress-words = { $g ->
   *[other] Cuvinte: { $count } / { $goal }
}
goal-progress-characters = { $g ->
   *[other] Caractere: { $count } / { $goal }
}
goal-count-words = { $n ->
    [one] { $count } cuvânt
    [few] { $count } cuvinte
   *[other] { $count } de cuvinte
}
goal-count-characters = { $n ->
    [one] { $count } caracter
    [few] { $count } caractere
   *[other] { $count } de caractere
}
goal-unit-words = Cuvinte
goal-unit-characters = Caractere

inspector-goal = Obiectiv
inspector-goal-none = Nicio ţintă

overview-col-goal = Obiectiv
overview-excluded-from-export = Lăsat în afara exportului, astfel încât să conteze pentru nici un total

goal-subtree-total-words = { $count ->
    [one] { $items } țintă în interiorul adaugă până la { $words } cuvinte
   *[other] { $items }  țintele din interior se adaugă până la { $words }  cuvinte
}
goal-subtree-total-characters = { $count ->
    [one] Ţinta din interior { $items } se ridică la { $words } caractere
   *[other] { $items } ţinte din interior adaugă până la { $words } caractere
}

new-work-goal-unit = Contează în
new-work-goal-unit-hint = Utilizat pentru fiecare cuvânt sau țintă de caracter în acest proiect. Schimbabil mai târziu.

settings-group-goal-unit = Obiective
settings-goal-unit-switch-title = Comută unitatea de numărare
settings-goal-unit-switch-text = Obiectivele și etapele de referință deja stabilite pentru acest proiect au fost introduse în { $from }. Trecerea la { $to } nu le convertește, astfel încât fiecare număr existent va fi acum citit ca { $to }.
settings-goal-unit-switch-informative = Nimic nu este pierdut. Comutați înapoi în orice moment pentru a restabili citirea originală, apoi actualizați țintele pe care doriți să le păstrați.
settings-goal-unit-switch-confirm = Comută

goal-distribute-action = Distribuie...
distribute-title = Distribuiți ținta
distribute-empty = Nu este nimic în interiorul acestui lucru pentru a partaja ținta la.
distribute-weight = Partajează
distribute-weight-length = Pe lungime
distribute-weight-rows = Prin bucăți
distribute-weight-even = Echilibrat
distribute-overwrite = Înlocuiește obiectivele care sunt deja stabilite
distribute-col-item = Postul
distribute-col-current = Acum.
distribute-col-proposed = După
distribute-total = Acestea se adaugă la { $total }, împotriva unei ținte de { $goal }.
distribute-over-budget = Țintele deja stabilite în interiorul vin la { $over } mai mult decât propriul container. Ridicați-și ținta, mai mici lor, sau le înlocuiți.
distribute-apply = Distribuie
distribute-cancel = Anulare

milestone-target-gone = Tinta eliminata
milestone-no-target = Nicio ţintă
milestone-add = Adăugare
milestone-add-label = La ce trebuie să se ajungă?

pace-summary-title = Unde stă cartea
pace-summary-remaining = { $words } pentru a merge
pace-summary-open = Deschide planul.
pace-summary-close = Închidere
pace-summary-dont-show = Nu afișa acest lucru la deschidere
pace-summary-menu = Plan de scris...

menu-help-topics = Ajutor la subiecte (&T)
menu-help-shortcuts = Scurtături tastatură... (&K)
menu-help-website = Skribisto pe web (&W)
menu-help-report = Raportaţi o problemă... (&R)
menu-help-check-updates = Verificați actualizările (&U)
menu-command-palette = Comanda Palette... (&C)

learn-help-topics = Subiecte de ajutor
learn-shortcuts = Scurtături de tastatură
learn-website = Skribisto pe web

help-window-title = Ajutor
help-filter-topics = Subiecte de filtrare
help-no-matching-topic = Nici un subiect nu se potriveşte.
help-topic-missing = Acest subiect nu mai este disponibil.
help-back = Înapoi.
help-not-translated = Acest articol nu este încă disponibil în română. Este afișat în engleză.

help-section-getting-started = Începem
help-section-writing = Scriere
help-section-reviewing = Revizuire
help-section-exchanging = Noțiuni de bază cuvinte și în afară
help-section-keeping = Păstrarea în siguranță a proiectului
help-section-extensions = Extensii

help-topic-getting-started = Primul tău proiect
help-topic-writing-model = Cum se pune o carte împreună
help-topic-drafts-and-old-versions = Păstrarea unui proiect mai vechi
help-topic-goals-and-pace = Obiective și ritm
help-topic-comments = Comentarii
help-topic-round-trip = Trimiterea cărții la un cititor
help-topic-export = Exportare
help-topic-import-documents = Documente de import
help-topic-import-projects = Aducerea unui proiect întreg peste
help-topic-backups-and-versions = Backup-uri și versiuni

help-shortcuts-title = Scurtături de tastatură
help-shortcuts-filter = Scurtături de filtrare
help-shortcuts-no-matches = Nicio scurtătură nu se potriveşte.
help-shortcuts-rebind = Schimbă comenzile rapide...
help-shortcuts-close = Închidere

command-palette-placeholder = Tastați comanda

shortcut-name-binder-duplicate = Duplicare
shortcut-name-comments-add = Adaugă comentariu
shortcut-name-comments-add-paragraph = Comentariu asupra paragrafului
shortcut-name-spellcheck-toggle = Verificați ortografia
shortcut-name-editor-tab-close = Închide fila
shortcut-name-editor-tab-pin = Pin sau fila Unpin
shortcut-name-editor-save = Salvare
shortcut-name-work-export = Exportare…
shortcut-name-work-new = Proiect nou
shortcut-name-work-open = Deschide proiectul
shortcut-name-window-new = Fereastră nouă
shortcut-name-work-close = Închidere proiect
shortcut-name-app-settings = Setări
shortcut-name-app-quit = Ieșire
shortcut-name-editor-insert-footnote = Inserează notă de subsol
shortcut-name-format-scene-break = Inserează pauză Scenă
shortcut-name-format-major-scene-break = Inserează pauza de scenă majoră
shortcut-name-format-link = Inserează legătură
shortcut-name-format-mark-nbsp = @ info: whatsthis
shortcut-name-format-mark-nbhyphen = Inserează Hyphen fără întrerupere
shortcut-name-format-mark-nnbsp = Inserează spațiu îngust fără rupere
shortcut-name-format-mark-zwsp = Inserează spațiu cu lățimea zero
shortcut-name-go-next = Următorul
shortcut-name-go-prev = Anteriorul
shortcut-name-go-to = Du-te la
shortcut-name-outline-toggle = Comută outline
shortcut-name-preview-toggle = Comută banda de previzualizare
shortcut-name-view-fullscreen = Comută tot ecranul
shortcut-name-view-focus-mode = Comută modul fără distragere a atenției
shortcut-name-editor-size-increase = Mărește dimensiunea textului
shortcut-name-editor-size-decrease = Scade dimensiunea textului
shortcut-name-editor-size-reset = Resetează dimensiunea textului
shortcut-name-editor-find = Căutare
shortcut-name-editor-replace = Înlocuire
shortcut-name-editor-find-next = Meciul următor
shortcut-name-editor-find-prev = Potrivire anterioară
shortcut-name-search-show = Caută în proiect
shortcut-name-search-replace = Înlocuiește în proiect
shortcut-name-outline-open-to-side = Deschisă în lateral
shortcut-name-help-topics = Ajutor la subiecte
shortcut-name-help-shortcuts = Scurtături tastatură
shortcut-name-help-website = Skribisto pe web
shortcut-name-help-report = Raportaţi o problemă
shortcut-name-help-check-updates = Verificați actualizările
update-check-behind = Versiunea { $version } este disponibilă.
update-check-current = Tu conduci cea mai nouă versiune.
update-check-failed = Nu am putut verifica pentru o versiune mai nouă. { $error }
shortcut-name-command-palette = Paletă de comandă
help-section-reference = Ce lucruri sunt

help-concept-scene-break = Spargerea scenei
help-concept-major-scene-break = Pauză de scenă majoră
help-concept-find-in-prose = Caută în proză
help-concept-story-bible = Ghidul poveștii
help-concept-goal-unit = Cuvinte sau caractere
help-concept-goal-progress = Progres
help-concept-manuscript-words = Ceea ce contează ca manuscris
help-concept-exportable = Lăsat în afara exportului
help-concept-distribute = Împărţim o ţintă
help-concept-subtree-total = Ceea ce obiectivele din interior adaugă până la
help-concept-milestone = Etapă
help-concept-pace-plan = Planul de pace

help-concept-tag = Etichetă
help-concept-label = Descriere scurtă
help-concept-point-of-view = Punct de vedere
help-concept-epigraph = Motto
help-concept-footnote = Notă de subsol
help-concept-chapter-mode = Forma capitolului
help-concept-comment = Comentariu
help-concept-backup = Copie de siguranță
help-concept-version = Versiune
help-concept-trash = Coș de gunoi
help-concept-spellcheck = Verificare ortografie
help-concept-search-replace = Căutare și înlocuire
help-concept-note-template = Șablon notă
help-concept-story-bible-entry = Intrarea în Biblie
help-concept-text-replacement = Înlocuirea textului
help-concept-smart-punctuation = Punctuație inteligentă
help-concept-export-style = Stilul de export
help-concept-round-trip-marks = Markeri rotunzi

epigraph-mark-attribution = Linia sursă
epigraph-mark-attribution-tip = Marcaţi linia în care se află îngrijirea ca sursă a citatului, aşa că printează ca o atribuire.
margin-lane-name = Marcaje de marjă
margin-lane-provider-comments = Comentarii
margin-lane-provider-story-bible = Intrarea în Biblie
margin-lane-provider-story-bible-hint = Pe o notă în proză lectură, în cazul în care această intrare este numit, și scenele spuse prin ochii săi. De asemenea, marchează aceste nume în proza însăși.
margin-lane-mark-point-of-view = I-am spus de aici
margin-lane-mark-named-here = Numit aici.
margin-lane-provider-comments-hint = În cazul în care o notă este atașată la text
margin-lane-provider-search = Caută hit-uri
margin-lane-provider-search-hint = Fiecare lovitură pentru ceea ce ai căutat ultima dată, oriunde ar fi în document.
margin-lane-provider-boundaries = În cazul în care începe fiecare document
margin-lane-provider-boundaries-hint = O regulă în partea de sus a fiecărei scene într-un flux, astfel încât să puteți vedea unde vă aflați
margin-lane-provider-spelling = Tipărire
margin-lane-provider-spelling-hint = În cazul în care verificatorul de vrăji a marcat un cuvânt. Off dacă nu-l cere: marchează opinia unei mașini de proza ta
margin-lane-spelling = { $word }, posibil mispelling
margin-lane-search-hit = { $text }, cu { $index } din { $total }
margin-lane-search-current = { $text }, meci { $index } din { $total }, cel pe care sunteți pe
margin-lane-boundary = Începând de la { $title }
margin-lane-boundary-untitled = un document fără titlu
settings-page-margin-lane = Marcaje de marjă
settings-desc-margin-lane = Fâșia de lângă bara de pergament, și ceea ce arată
settings-margin-lane-enabled = Arată banda de margine
settings-margin-lane-enabled-hint = Acelaşi întrerupător ca şi vizualizarea marcilor.
settings-margin-lane-enabled-more = Nu spune niciodată nimic greşit: îţi arată unde sunt lucrurile şi îţi lasă ţie ce să faci cu ele.
settings-group-margin-lane-marks = Ceea ce marchează
settings-margin-lane-no-providers = Nimic nu marchează banda încă.
settings-group-margin-lane-texture = Textura dialogului
settings-margin-lane-texture = Arată textura dialogului
settings-margin-lane-texture-hint = O bară pe paragraf: cât de lungă este, și cât de mult este vorbită.
settings-margin-lane-texture-more = Nu sunt măsurate pentru limbi fără convenţie curatorie, unde barul arată doar lungime.
settings-group-margin-lane-surfaces = Unde apare
settings-margin-lane-surface-editor = Editor de text
settings-margin-lane-surface-stream = Fluxuri de text
settings-margin-lane-surface-search-preview = Previzualizare căutare
menu-margin-lane = Marcaje de marjă (&M)

status-preset-drafting = Redactarea ciornei
status-preset-passes = Permise de revizuire
status-preset-plume = Plume Creator
status-todo = Pentru a face
status-draft = Ciornă
status-revised = Revizuit
status-final = Final
status-outline = Schiță
status-first-edit = Editare 1
status-second-edit = A doua editare
status-done = Gata
status-plume-draft-1 = Primul proiect
status-plume-draft-2 = Al doilea proiect
status-plume-draft-3 = Al treilea proiect
status-plume-edit-1 = Editare 1
status-plume-edit-2 = A doua editare
status-plume-edit-3 = Editare 3
status-plume-proofread = Corectat
status-plume-finished = Finalizat
new-work-statuses = Fluxul de lucru
new-work-statuses-hint = Etapele pe care le mutați o scenă prin. Puteți redenumi, reordona sau adăugați la acestea în orice moment.
status-none = Nicio stare
inspector-status = Stare
overview-col-status = Stare
overview-status-mixed = părțile de mai jos nu sunt de acord
help-concept-status = Stare
status-completion-title = Unde stă cartea
status-completion-headline = { $done } din { $total } scene terminate
status-completion-empty = Asta citeşte manuscrisul, aşa că se completează în timp ce scrii.
status-completion-open = În cazul în care cartea stă ...

settings-page-statuses = Stare
settings-desc-statuses = Redenumeşte, reordonează, adaugă şi elimină etapele prin care treci o scenă
settings-statuses-add = Adaugă stare
settings-statuses-add-placeholder = Nume nou de statut
settings-statuses-desc =
    Etapele unei scene trec prin, în ordine, de la cel mai puțin terminat de la partea de sus la cele mai multe
    Acest ordin este ceea ce înseamnă "mai puţin terminat decât" peste tot.
    în aplicație, așa că pune-le modul în care procesul funcționează de fapt.
settings-statuses-apply-preset = Aplică un preset...
settings-statuses-preset-applied = { $added ->
    [one] 1 statut adăugat
   *[other] { $added } statute adăugate
}
settings-statuses-preset-refused = Acest proiect are deja o scară. Presets umple doar unul gol. Ștergeți treptele pe care nu doriți mai întâi.
settings-statuses-duplicate = "{ $name }" este deja pe această scară
settings-statuses-added = Adăugat "{ $name }"
settings-statuses-move-up = Mutare în sus (mai puțin terminat)
settings-statuses-move-down = Mutare în jos (mai finisat)
settings-statuses-details-placeholder = Ce înseamnă această etapă (opțional)
settings-statuses-delete = Se elimină "{ $name }"
settings-statuses-delete-in-use = { $count ->
    [one] Șterge "{ $name }": 1 element se află pe el și își va pierde statutul
   *[other] Șterge "{ $name }": { $count } articole sunt pe el și își vor pierde statutul
}
settings-statuses-deleted = Eliminat "{ $name }"
settings-statuses-deleted-in-use = { $count ->
    [one] Eliminat "{ $name }." 1 element nu mai are un statut.
   *[other] Șterge "{ $name }." { $count } articole nu mai au un statut.
}
settings-statuses-empty-title = Nici un flux de lucru încă
settings-statuses-empty-body = Un statut spune cât de departe de-a lungul unei scene este. Adăugați una de mai sus, sau începe de la un preset.
status-category-planned = Planificate
status-category-drafting = Redactarea ciornei
status-category-needs-work = Necesită lucru
status-category-revised = Revizuit
status-category-final = Final
