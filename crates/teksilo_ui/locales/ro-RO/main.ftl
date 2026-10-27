# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Skribisto: UI strings (source locale).
# A single `&` marks the keyboard mnemonic of a menu label; `&&` is a literal `&`.

## Menu bar: Work
menu-work = &Proiect
menu-new-work = Proiect &nou
menu-open-work = &Deschide proiect…
menu-new-window = &Fereastră nouă
menu-import-from = &Importă din
# The Launcher's counterpart to `menu-import-from`, over the same two rows: these
# produce a brand-new project, where the project window's importers land content
# *in* the one already open.
menu-create-from = &Creează din
menu-import-plume = &Plume Creator (.plume)…
menu-import-document = &Documente (Markdown, Word, ODT)…
menu-import-manuskript = &Manuskript (.msk)…
menu-import-scrivener = &Scrivener (.scriv)…
menu-export = E&xportă
menu-export-book = Exportă cartea
menu-export-part = Exportă partea
menu-export-chapter = Exportă capitolul
menu-export-scene = Exportă scena
menu-export-note = Exportă nota
menu-export-paratext = Exportă paratextul
menu-export-folder = Exportă dosarul
menu-export-choose = Alege…
menu-export-none = Deschideți un document pentru a-l exporta
menu-save = &Salvează
menu-save-as-file = Salvează ca fișier &unic…
menu-save-as-folder = Salvează ca d&osar…
menu-backup = Fă o &copie de siguranță acum
menu-close-work = Înc&hide proiectul
menu-welcome = &Bun venit…
menu-settings = Se&tări
menu-quit = Ieși&re

## Menu bar: View
menu-view = &Vizualizare
menu-outline = &Structură
menu-search = &Caută în proiect
menu-timeline = Înapoi în &timp
menu-search-preview = &Previzualizare căutare
menu-fullscreen = &Ecran complet
menu-focus-mode = Mod fără &distrageri
menu-format = &Formatare
menu-scene-break = Inserează se&parator de scenă
menu-major-scene-break = Inserează separator &major de scenă
menu-line-break = Inserea&ză întrerupere de rând

## Menu bar: Go
menu-go = &Navigare
menu-go-next-scene = &Scena următoare
menu-go-prev-scene = Scena &precedentă
menu-go-next-chapter = &Capitolul următor
menu-go-prev-chapter = C&apitolul precedent
menu-go-next-note = &Nota următoare
menu-go-prev-note = N&ota precedentă

menu-tools = &Instrumente
menu-help = &Ajutor

## Binder context menu
ctx-add = &Adaugă
ctx-new-item = Element &nou
ctx-new-folder = &Dosar nou
ctx-rename = &Redenumește
ctx-duplicate = D&uplică
ctx-import-here = &Importă aici…
ctx-indent = &Indentează
ctx-outdent = &Scade indentarea
ctx-trash = Mută la coșul de &gunoi
ctx-open-to-side = Deschide în &lateral
ctx-open = &Deschide
ctx-reveal-in-outline = Arată în &structură
ctx-move-up = Mută în &sus
ctx-move-down = Mută în &jos

## Editor tab context menu
# Fresh `ctx-tab-*` keys, never a reuse of the `ctx-*` rows above: those double as
# menu-BAR rows (shell/project_menus/document.rs), so retuning one of their
# mnemonics to fit this menu would break the bar's own per-locale uniqueness.
# Three rows are either/or: the split, the move and the pin each build exactly
# one of their pair, so a letter may repeat across a pair but never within one
# built menu, which is a debug_assert! panic in MenuList::build, not a warning.
ctx-tab-close = În&chide
ctx-tab-close-others = Închide c&elelalte
ctx-tab-close-all = Închide &toate
# The split rows read directionally, from the pane the tab is already in, so the
# writer never has to work out which half "the side" means. They duplicate: the
# item ends up open in both panes over one shared document.
ctx-tab-open-to-side = Deschide în &lateral
ctx-tab-open-in-main = Deschide în panoul &principal
# The move rows read the same way, but the tab leaves the pane it came from.
ctx-tab-move-to-side = &Mută în lateral
ctx-tab-move-to-main = &Mută în panoul principal
ctx-tab-move-to-new-window = Mută într-o fereastră &nouă
# Why that last row is unavailable. A second window is opened onto the project's
# file, so a project that has never been saved has nothing for it to open. A
# tooltip on the disabled row, not a menu label, hence no mnemonic.
ctx-tab-move-window-unsaved = Salvați mai întâi acest proiect: a doua fereastră se deschide pe un fișier de pe disc
ctx-tab-pin = &Fixează această filă
ctx-tab-unpin = &Desprinde această filă
# Tab tooltip, not a menu row: no mnemonic. It names the two commands a pin
# actually protects against, because the tab itself shows only a glyph and a
# missing close button, neither of which says what the pin is for.
tab-pinned-tooltip = Fixată: „Închide celelalte” și „Închide toate” o lasă deschisă

## Create recommendations: logical type labels (SplitButton title + Add ▸ rows)
create-book = Carte
create-part = Parte
create-chapter = Capitol
create-scene = Scenă
create-note = Notă
create-note-folder = Dosar de note
create-folder = Dosar
create-paratext = Paratext
create-paratext-folder = Dosar de paratext
create-book-end = Sfârșitul cărții
create-story-bible-entry = Intrare în biblia poveștii…
# Item-type names for the Overview's Type column (the rest reuse the create-* nouns).
type-book-start = Începutul cărții
type-text = Text

## Create: the default title a new row is given.
## Data, not chrome. Resolved once at creation and then owned by the writer,
## so switching language never retitles anything already created.
## (A scene reuses `new-scene-title`, shared with split-scene.)
new-item-book = Carte nouă
new-item-part = Parte nouă
new-item-chapter = Capitol nou
new-item-note = Notă nouă
new-item-note-folder = Dosar de note nou
new-item-folder = Dosar nou
new-item-paratext = Paratext nou
new-item-paratext-folder = Dosar de paratext nou
new-item-story-bible-entry = Intrare nouă în biblia poveștii

## Create recommendations: trailing placement hint on each row
placement-inside = în interior
placement-after = după
placement-after-parent = după părinte
placement-top-level = nivelul de sus

# (Writing-model rich tooltips live in this locale's tooltips.ftl.)

## Promote: convert a binder item to its paired type
ctx-promote = &Convertește în
promote-chapter-folder = Capitol-dosar
promote-flat-chapter = Capitol simplu
promote-lossy-title = Textul nu are unde să fie păstrat
promote-lossy-text = Tipul „{ $target }” nu are unde să păstreze: { $kinds }. Mutați sau ștergeți mai întâi acel text, apoi convertiți.
# Content-role names, for explaining what a conversion cannot carry over.
content-scene-text = Textul scenei
content-note-text = Textul notei
content-book-title = Titlul cărții
content-book-subtitle = Subtitlul cărții
content-part-title = Titlul părții
content-chapter-title = Titlul capitolului
content-epigraph-text = Motto
content-paratext-text = Paratext
promote-blocked-title = Capitolul nu este gol
promote-blocked-text = { $count ->
       [one] Acest capitol mai conține { $count } element. Mutați-l sau aruncați-l la coșul de gunoi înainte de a-l converti în capitol simplu.
       [few] Acest capitol mai conține { $count } elemente. Mutați-le sau aruncați-le la coșul de gunoi înainte de a-l converti în capitol simplu.
      *[other] Acest capitol mai conține { $count } de elemente. Mutați-le sau aruncați-le la coșul de gunoi înainte de a-l converti în capitol simplu.
   }

## Inspector (trailing dock) + status-bar dock toggles
inspector = Inspector
inspector-empty = Deschideți un element pentru a-l inspecta.
inspector-promote = Convertește în…
statusbar-toggle-outline = Afișează sau ascunde clasorul
statusbar-toggle-inspector = Afișează sau ascunde inspectorul
# The save indicator (status bar, next to the binder toggle).
statusbar-save-unsaved = Modificări nesalvate. Clic pentru a salva
statusbar-save-saved = Toate modificările sunt salvate
statusbar-save-autosave = Salvarea automată este activă. Modificările se salvează pe măsură ce scrieți
statusbar-saving = Se salvează…
# The focused item's live word count (status bar).
statusbar-word-count = { $count ->
       [one] { $count } cuvânt
       [few] { $count } cuvinte
      *[other] { $count } de cuvinte
   }
statusbar-word-count-tooltip = Cuvintele din scena pe care o editați
# Words + characters, shown when "Show character count" is on (Settings ▸ Goals).
statusbar-word-char-count = { $words ->
       [one] { $words } cuvânt
       [few] { $words } cuvinte
      *[other] { $words } de cuvinte
   } · { $chars ->
       [one] { $chars } caracter
       [few] { $chars } caractere
      *[other] { $chars } de caractere
   }
# The writing session (status-bar sprint timer + word tracker).
session-toggle = Sesiune de scriere: începeți sau întrerupeți o rundă de scris concentrat
session-configure = Stabiliți obiectivul de cuvinte și limita de timp ale sesiunii
session-configure-title = Sesiune de scriere
session-word-goal = Obiectiv de cuvinte
session-time-limit = Limită de timp
# The spin box's non-editable trailing unit. An abbreviation, not a word that
# agrees with the number: a `SpinBox` suffix is static text and cannot carry a
# plural selector. The separating space is prepended in code (Qt's `" min"`
# convention), so give the unit alone. Never shown at 0, which reads
# `session-no-limit` instead.
session-time-limit-unit = min
session-no-goal = Fără obiectiv
session-no-limit = Fără limită
session-reset = Resetează sesiunea
session-readout = { $words ->
    [one] { $words } cuvânt
    [few] { $words } cuvinte
   *[other] { $words } de cuvinte
} · { $time }
session-readout-timed = { $words ->
    [one] { $words } cuvânt
    [few] { $words } cuvinte
   *[other] { $words } de cuvinte
} · mai sunt { $time }
# Distraction-free mode's always-visible control strip (Increment 2).
statusbar-focus-exit = Ieși din modul fără distrageri
# The strip's Next/Previous buttons (Increment 4) fire the same go.next/go.prev
# actions as the shortcut and the Go menu's generic pair.
statusbar-focus-synopsis = Sinopsis
statusbar-focus-comments = Comentarii
statusbar-focus-go-prev = Elementul precedent (Alt+Sus)
statusbar-focus-go-next = Elementul următor (Alt+Jos)

## Activity-rail actions (dockless commands in the icon rail)
rail-settings = Setări

## Settings
language = Limbă
theme = Temă
english = Engleză
french = Franceză
light = Luminoasă
dark = Întunecată
settings-text-width = Lățimea textului
settings-preview-width = Lățimea previzualizării căutării
settings-autosave = Salvare automată pe disc
settings-show-welcome = Afișează lansatorul la pornire
settings-show-welcome-tip = Dacă este dezactivat, se redeschide ultimul proiect.
settings-check-for-updates = Caută versiuni noi
settings-check-for-updates-tip = Cel mult o dată pe zi, Skribisto întreabă site-ul proiectului care este versiunea curentă. Cererea nu conține niciun identificator și nici măcar versiunea pe care o folosiți: comparația se face pe acest calculator.

## Settings window: chrome
settings-title = Setări
settings-close = Închide
settings-search = Caută în setări
settings-reset = Revino la valorile implicite
settings-done = Gata
settings-cancel = Renunță
settings-apply = Aplică
settings-ok = OK
settings-reset-confirm-title = Readuceți toate setările la valorile implicite?
settings-reset-confirm-body = Readuce la valorile implicite aspectul aplicației, setările editorului și pe cele de scriere. Setările proprii ale proiectului, comenzile rapide de la tastatură și stilurile salvate nu sunt afectate. Operația nu poate fi anulată.
settings-empty-title = Încă nu există setări aici
settings-empty-hint = Această secțiune va primi opțiuni într-o actualizare viitoare.

## Settings window: categories
settings-sec-appearance-behaviour = Aspect și comportament
settings-sec-editor = Editor
settings-sec-spelling = Ortografie
settings-sec-backup = Copii de siguranță și sincronizare
settings-sec-compile = Compilare și export
settings-page-appearance = Aspect
settings-field-image-size-policy = Imagini mari
settings-image-policy-ask = Întreabă de fiecare dată
settings-image-policy-keep = Păstrează originalul
settings-image-policy-downscale = Optimizează
settings-hint-image-size-policy = Ce se întâmplă când o imagine inserată este mai mare decât are nevoie manuscrisul.
settings-page-notifications = Notificări
settings-page-scene = Scenă
settings-page-synopsis = Sinopsis
settings-page-notes = Note
settings-page-editor-behavior = Comportamentul editorului
settings-page-goals = Obiective și număr de cuvinte
settings-page-games = Jocuri de scriere
settings-page-corkboard = Avizier
settings-page-distraction-free = Fără distrageri
settings-page-dictionaries = Dicționare
settings-page-autosave = Salvare automată
settings-page-export = Formate de export
settings-page-paratext = Scheme de paratext
settings-page-keymap = Comenzi rapide
# Filter box on the Keymap page (filters the ShortcutSettings list by name / id / category).
settings-keymap-filter = Filtrează comenzile rapide

## Settings window: what each page is for
# One line per page, shown under that page's link on its parent's page and,
# for a parent, under its own title. Keep them to one line: they describe what
# the page holds, not how to use it.
settings-desc-sec-appearance-behaviour = Cum arată aplicația și ce face la pornire.
settings-desc-sec-editor = Suprafața de scris: cum arată și ce face în timp ce tastați.
settings-desc-sec-spelling = Verificarea ortografică și dicționarele pe care se bazează.
settings-desc-sec-backup = Cum ajunge munca dumneavoastră pe disc și ce copii se păstrează.
settings-desc-sec-compile = Ce iese din Skribisto și în ce formă.
settings-desc-sec-work = Setările care aparțin acestui proiect și sunt păstrate în fișierul lui.
settings-desc-sec-extensions = Pagini adăugate de extensiile instalate aici.
settings-desc-group-typography = O pagină pentru fiecare tip de text: font, mărime, înălțimea rândului și spațiere.
settings-desc-appearance = Limba interfeței, tema, mărimea textului și lansatorul la pornire.
settings-desc-notifications = Toate mesajele afișate în această sesiune și acțiunile pe care le puteți relua.
settings-desc-scene = Cum este așezată proza scenelor: font, mărime, înălțimea rândului, indentări și spațiere.
settings-desc-synopsis = Cum este așezat panoul de sinopsis, independent de manuscris.
settings-desc-notes = Cum sunt așezate notele, independent de manuscris.
settings-desc-corkboard = Mărimea fișelor, ce afișează o fișă și cum le aranjează avizierul.
settings-desc-distraction-free = Tipografia, lățimea coloanei și bara de control pentru scrisul pe tot ecranul.
settings-desc-distraction-free-themes = Biblioteca de teme pentru scrisul pe tot ecranul: cele incluse și ale dumneavoastră.
settings-desc-editor-behavior = Lățimea textului, poziția sinopsisului, derularea de tip mașină de scris și evidențierea din jurul cursorului.
settings-desc-punctuation = Stilul tipografic de la care pornesc proiectele noi. Fiecare proiect îl urmează, dacă nu își stabilește propriul stil.
settings-desc-goals = Obiective de cuvinte și de caractere și felul în care sunt numărate cuvintele.
settings-desc-games = Constrângeri pe care vi le impuneți singur(ă) în timp ce scrieți ciorna, cum ar fi „Mereu înainte”.
settings-desc-spellcheck = Comutatorul unic care pornește și oprește peste tot verificarea ortografică.
settings-desc-dictionaries = Instalați, eliminați și răsfoiți dicționarele ortografice de pe acest calculator.
settings-desc-autosave = Dacă modificările se scriu singure pe disc.
settings-desc-backup = Când se fac copii, unde sunt păstrate și câte. Fiecare proiect urmează aceste reguli, dacă nu își stabilește propriile reguli.
settings-desc-export = Stilurile prin care trece fiecare export: cele incluse și ale dumneavoastră.
settings-desc-paratext = Paginile de început și de sfârșit cu care poate porni un proiect nou.
settings-desc-keymap = Toate comenzile rapide și acțiunile cărora le sunt atribuite.
settings-page-user = Utilizator
settings-desc-user = Cine sunteți, pentru comentariile pe care le scrieți.
settings-group-identity = Identitate
settings-field-user-name = Numele dumneavoastră
settings-field-user-name-placeholder = Opțional
settings-field-user-name-hint = Semnează comentariile și răspunsurile pe care le scrieți. Este diferit de numele de autor al unui proiect, care apare pe carte și este păstrat în fișierul acestuia.
settings-field-user-initials = Inițialele dumneavoastră
settings-field-user-initials-placeholder = Din numele dumneavoastră
settings-field-user-initials-hint = Ce afișează un procesor de text lângă comentariul dumneavoastră, în margine. Lăsați câmpul gol pentru a le folosi pe cele afișate, deduse din numele dumneavoastră.
settings-field-user-hint = Ambele sunt opționale și se aplică tuturor proiectelor de pe acest calculator. Dacă le modificați, se schimbă semnătura comentariilor pe care le scrieți de acum înainte. Comentariile deja scrise își păstrează numele cu care au fost scrise.
settings-desc-author = Numele care apare pe acest proiect.
settings-desc-structure = Dacă acest proiect are capitole-dosar sau capitole simple.
settings-desc-language = Limba în raport cu care este verificată proza acestui proiect.
settings-desc-work-backup = Regulile proprii de copii de siguranță ale acestui proiect sau cele generale.
settings-desc-personal-dictionary = Cuvintele pe care acest proiect le consideră scrise corect.
settings-desc-tags = Etichetele colorate cu care acest proiect își marchează elementele.
settings-desc-templates = Șabloanele de note pe care le puteți insera când scrieți în acest proiect.
settings-desc-text-replacements = Prescurtări care se extind în timp ce tastați în acest proiect.
settings-desc-work-punctuation = Ghilimelele, liniile de dialog și spațierea pe care le urmează proza acestui proiect.

## Settings window: fields
settings-group-typography = Tipografie
settings-group-writing-column = Coloana de scris
settings-group-writing-view = Vizualizarea de scris
# The distraction-free control strip's optional items (Exit is never optional).
settings-group-distraction-free-strip = Bara de control
settings-group-theme = Temă
settings-group-language = Limbă
settings-group-startup = Pornire
settings-group-updates = Actualizări
settings-group-autosave = Salvare automată
settings-field-typeface = Font
settings-field-size = Mărimea textului

# The readout shown while Ctrl+Wheel / Ctrl+= / Ctrl+- / Ctrl+0 change an
# editor's text size. One whole message per surface rather than a shared
# "{ $surface }: { $percent }" template: the surface name is not a plain
# substitution in every language, and a translator should get complete
# sentences. $percent arrives already formatted ("110%").
editor-size-changed-manuscript = Mărimea textului manuscrisului: { $percent }
editor-size-changed-synopsis = Mărimea textului sinopsisului: { $percent }
editor-size-changed-notes = Mărimea textului notelor: { $percent }
editor-size-changed-corkboard = Mărimea textului fișelor de pe avizier: { $percent }
editor-size-changed-corkboard-expanded = Mărimea textului în editorul extins: { $percent }
editor-size-changed-distraction-free = Mărimea textului în modul fără distrageri: { $percent }

menu-text-size-increase = &Mărește textul
menu-text-size-decrease = M&icșorează textul
menu-text-size-reset = &Resetează mărimea textului
settings-field-line-height = Înălțimea rândului
settings-field-first-line-indent = Indentarea primului rând
settings-field-paragraph-spacing-before = Spațiu înainte de paragraf
settings-field-paragraph-spacing-after = Spațiu după paragraf
settings-field-column-width = Lățimea coloanei
settings-distraction-free-width-hint = Se aplică doar cât timp modul fără distrageri este activ. Lățimile coloanelor de scenă, sinopsis și note rămân neschimbate.
settings-distraction-free-title = Păstrează numele elementului
settings-distraction-free-word-count = Păstrează numărul de cuvinte
settings-distraction-free-session = Păstrează sesiunea de scriere
settings-distraction-free-go-to = Păstrează butonul „Salt la…”
settings-distraction-free-go = Păstrează butoanele „Precedentul” și „Următorul”
settings-distraction-free-chrome-hint = Butonul de ieșire rămâne mereu, orice ați alege aici. Este calea de ieșire dacă tasta Escape este ocupată.
settings-field-app-theme = Temă
# The two entries of the Theme picker (the Fluent light / dark appearances).
settings-theme-light = Luminoasă
settings-theme-dark = Întunecată
settings-field-text-scale = Mărimea textului interfeței
settings-field-language = Limba interfeței
settings-synopsis-placement = Poziția sinopsisului
settings-synopsis-placement-none = Ascuns
settings-synopsis-placement-top = Deasupra
settings-synopsis-placement-side = Alături
settings-typewriter = Derulare de tip mașină de scris
settings-typewriter-tip = Ține rândul pe care scrieți la o înălțime fixă, în timp ce manuscrisul derulează sub el. Un clic pune în continuare cursorul acolo unde dați clic.
settings-typewriter-position = Poziția rândului
settings-typewriter-position-top-third = Treimea de sus
settings-typewriter-position-middle = La mijloc
settings-typewriter-position-bottom-quarter = Sfertul de jos
settings-highlight-scope = Evidențiere în jurul cursorului
settings-highlight-scope-none = Niciuna
settings-highlight-scope-sentence = Propoziția
settings-highlight-scope-paragraph = Paragraful
settings-highlight-scope-tip-none = Pagina rămâne simplă. Nimic nu este umbrit în timp ce scrieți.
settings-highlight-scope-tip-sentence = Umbrește propoziția pe care o scrieți, ca să iasă în evidență față de cele din jur.
settings-highlight-scope-tip-paragraph = Umbrește tot paragraful pe care îl scrieți, ca pasajul la care lucrați să rămână la vedere.
settings-group-container-views = Vizualizările containerelor
settings-remember-view = Ține minte ultima vizualizare pentru fiecare tip de element
settings-remember-view-tip = Deschide un container în vizualizarea folosită ultima dată pentru acel tip
settings-remember-view-tip-more =
       O carte, o parte și un capitol oferă fiecare mai multe vizualizări (pagina proprie,
       manuscrisul complet, sinopsisul complet). Dacă activați opțiunea, fiecare tip se
       redeschide în vizualizarea aleasă ultima dată: de exemplu, treceți un capitol pe
       „Capitolul complet” și următorul capitol pe care îl deschideți arată tot „Capitolul complet”.
       Fiecare tip își ține minte propria vizualizare.
# Goals & Word Count pane
settings-group-counting = Numărarea cuvintelor
settings-counting-auto = Automat (după limbă)
settings-counting-whitespace = Separare după spații
settings-counting-unicode-words = Cuvinte Unicode
settings-counting-cjk-hybrid = Pentru CJK (pe caracter)
settings-counting-hint = Modul automat numără chineza și japoneza pe caractere și toate celelalte limbi pe cuvinte. Schimbați-l doar dacă numărătoarea pare greșită pentru limba dumneavoastră.
settings-group-goals-display = Afișare
settings-show-characters = Afișează numărul de caractere în bara de stare
settings-autosave-hint = Modificările se scriu automat pe disc în timp ce scrieți.

## Settings: Work (the open project)
settings-sec-work = Proiect
settings-page-structure = Alcătuire
settings-page-author = Autor
settings-field-author-name = Numele autorului
settings-field-author-placeholder = Opțional
settings-field-author-hint = Apare pe pagina de titlu compilată și în metadatele fișierelor exportate. Lăsați câmpul gol pentru a-l omite. Acesta este numele de autor de pe carte. Numele cu care vă semnați comentariile se află în Setări ▸ Utilizator.
settings-group-chapters = Capitole
tidy-titles-title = Curăță titlurile capitolelor
tidy-titles-none = Niciun titlu de capitol sau de parte nu se reduce la propriul număr.
tidy-titles-lead = { $count ->
           [one] Un capitol sau o parte are drept titlu doar propriul număr.
           [few] { $count } capitole și părți au drept titlu doar propriul număr.
          *[other] { $count } de capitole și părți au drept titlu doar propriul număr.
       }
tidy-titles-explain = Golirea acestor titluri lasă fiecare element numit după numărul pe care cartea îl știe deja, același număr pe care îl tipărește exportul. Nimic altceva nu se schimbă, iar o singură anulare le readuce pe toate.
menu-document-tidy-titles = Curăță &titlurile capitolelor…
settings-group-numbering = Numerotare
settings-number-chapters = Numerotează capitolele și părțile
settings-number-chapters-tip = Capitolele și părțile primesc un număr calculat după locul lor în carte, afișat aici lângă titlu și tipărit la export.
settings-number-chapters-tip-more = Numărul nu este niciodată stocat în titlu, așa că rămâne corect când reordonați, inserați sau ștergeți. Dacă dezactivați opțiunea, exportul tipărește doar titlurile, orice ar cere stilul de export. Pentru a lăsa un singur capitol fără număr (un prolog, un interludiu), folosiți în schimb comutatorul Numerotare din inspector: capitolul rămâne în carte, dar nu mai primește număr.
settings-part-resets-chapter = Reia numerotarea capitolelor la fiecare parte
settings-part-resets-chapter-tip = Dezactivat implicit: capitolele se numerotează în continuare de-a lungul părților unei cărți, așa că „Partea a doua” începe cu capitolul unsprezece.
settings-part-resets-chapter-tip-more = Aceasta este practica obișnuită în edituri și ceea ce așteaptă un cititor. Activați opțiunea pentru o carte ale cărei părți sunt gândite ca volume separate, fiecare începând cu capitolul unu.
settings-chapter-flat = Capitole simple
settings-chapter-flat-hint = Activat: un capitol este un singur element. Scrieți în el și nu conține scene. Dezactivat: un capitol este un dosar. Scrieți în continuare în el, dar poate conține și scene. Capitolele noi urmează această setare; cele existente se convertesc cu „Convertește în”.

## Settings: Export styles (Compile & Export ▸ Export Formats)
settings-styles-builtin = Stiluri incluse
settings-styles-user = Stilurile mele
settings-styles-builtin-badge = Inclus
settings-styles-duplicate = Duplică
settings-styles-edit = Editează
settings-styles-delete = Șterge
settings-styles-import = Importă…
settings-styles-export = Exportă…
settings-styles-copy-suffix = (copie)
settings-styles-json-filter = Stil de export
settings-styles-editor-title = Editare stil
settings-styles-editor-none = Selectați un stil personalizat pentru a-l edita sau duplicați unul inclus.
settings-styles-imported = Stil importat
settings-styles-import-failed = Stilul nu a putut fi importat
settings-styles-exported = Stil exportat
settings-styles-export-failed = Stilul nu a putut fi exportat
# Editor field labels
settings-styles-field-name = Nume
settings-styles-field-chapters = Titlurile capitolelor
settings-styles-field-parts = Titlurile părților
settings-styles-field-scene-break = Separator de scenă
settings-styles-field-major-scene-break = Separator major de scenă
settings-styles-field-spacing = Spațiere între rânduri
settings-styles-field-justify = Aliniază textul la ambele margini
settings-styles-field-notes = Include notele
settings-styles-field-synopses = Include sinopsisurile
settings-styles-field-scene-titles = Include titlurile scenelor
settings-styles-field-epigraphs = Include mottourile
settings-styles-field-paratexts = Include paratextele
settings-styles-field-epigraph-placement = Poziția mottoului
settings-styles-epigraph-after = După titlu
settings-styles-epigraph-before = Înainte de titlu
settings-styles-field-footnotes = Include notele de subsol
settings-styles-field-footnote-numbering = Numerotarea notelor de subsol
settings-styles-footnote-numbering-continuous = Continuă
settings-styles-footnote-numbering-per-chapter = Reluată la fiecare capitol
settings-styles-footnote-numbering-per-book = Reluată la fiecare carte
settings-styles-field-images = Imagini
menu-image = I&magine
settings-styles-images-beside = Lângă document
settings-styles-images-embed = În document
settings-styles-images-omit = Omise
settings-styles-group-round-trip = Trimitere către un redactor
settings-styles-field-comments = Include comentariile
settings-styles-field-round-trip-marks = Include marcajele de dus-întors
settings-styles-round-trip-hint = Doar DOCX și ODT. Marcajele sunt identificatori invizibili care permit unui fișier întors să actualizeze acest proiect, în loc să ajungă alături de el ca a doua copie.
settings-styles-group-pages = Pagini
settings-styles-field-cover = Începe cu coperta
settings-styles-field-word-count = Numărul de cuvinte pe pagina de titlu
settings-styles-field-page-books = Pagină nouă la fiecare carte
settings-styles-field-page-parts = Pagină nouă la fiecare parte
settings-styles-field-page-chapters = Pagină nouă la fiecare capitol
settings-styles-field-page-paratexts = Pagină nouă la fiecare paratext
# Heading-scheme options
settings-styles-heading-none = Fără titlu de secțiune
settings-styles-heading-numbered = Doar numărul
settings-styles-heading-title = Doar titlul
settings-styles-heading-both = Număr + titlu
# Scene-break options (glyph values stay as typed)
settings-styles-break-blank = Rând gol
settings-styles-break-none = Niciunul
# Line-spacing options
settings-styles-spacing-single = Simplă
settings-styles-spacing-onehalf = 1½
settings-styles-spacing-double = Dublă

## Welcome
welcome-title = Bun venit în Skribisto
# $version is stamped from the git tag at build time (see src/version.rs).
welcome-version = Versiunea { $version }
# Shown under the version in the launcher sidebar, only when a newer release exists.
welcome-update-available = Versiunea { $version } este disponibilă
welcome-update-tip = Deschide pagina de descărcare în navigator.
welcome-search = Caută proiecte
welcome-open = Deschide
welcome-new-work = Proiect nou
welcome-create-from = Creează din…
welcome-recent-works = Proiecte recente
welcome-empty-recents = Încă nu există proiecte recente.
# Shown in place of the recents list when the search matched none of them,
# distinct from having no recent works at all.
welcome-no-matches = Niciun proiect recent nu corespunde căutării.
welcome-learn-soon = Ghidurile și sfaturile vor fi disponibile în curând.
welcome-about-blurb = Skribisto, aplicația de scris rescrisă în Rust + Teksilo.
# The *…* is inline markup, not decoration: it italicizes the line (the widget
# renders it in a serif italic). Keep the asterisks when translating.
welcome-tagline = *Un loc liniștit pentru a scrie lucruri lungi.*
# Tooltip and screen-reader name of the two icon links under the sidebar nav.
# Product names: keep them as they are.
welcome-github = GitHub
welcome-discord = Discord
nav-works = Proiecte
nav-examples = Exemple
nav-learn = Învățare
nav-about = Despre

## Editor tabs & panes
synopsis = Sinopsis
epigraph = Motto
pane-manuscript = Manuscris
corkboard = Avizier
overview = Vedere de ansamblu

## Corkboard
corkboard-card-count = { $count ->
       [one] { $count } fișă
       [few] { $count } fișe
      *[other] { $count } de fișe
   }
corkboard-child-count = { $count ->
       [one] { $count } element
       [few] { $count } elemente
      *[other] { $count } de elemente
   }
corkboard-view-nested = Imbricat
corkboard-view-flat = Plat
corkboard-layout-hint = „Imbricat” arată copiii direcți ai unui container. Deschideți fișa unui dosar pentru a intra în el. „Plat” arată dintr-odată toate scenele din întregul container.
corkboard-card-size = Mărimea fișelor
corkboard-search-placeholder = Filtrează fișele…
corkboard-empty-title = Încă nu este nimic aici
corkboard-empty-hint = Folosiți „＋ Nou” de mai sus pentru a adăuga primul element.
corkboard-new = Nou
corkboard-show-card-numbers = Numerotează fișele
corkboard-modal-size = Mărimea editorului extins
corkboard-scope-hint = Se aplică tuturor avizierelor deschise, în toate proiectele.
corkboard-sort-manuscript = Ordinea din manuscris
corkboard-sort-title-asc = Titlu A–Z
corkboard-sort-title-desc = Titlu Z–A
corkboard-card-number = Fișa { $number }

## Corkboard batch actions (the card kebab acts on the whole selection)
duplicate = Duplică
duplicate-n = { $count ->
       [one] Duplică { $count } fișă
       [few] Duplică { $count } fișe
      *[other] Duplică { $count } de fișe
   }
set-label-n = { $count ->
       [one] Setează legenda pe { $count } fișă
       [few] Setează legenda pe { $count } fișe
      *[other] Setează legenda pe { $count } de fișe
   }
move-to-trash-n = { $count ->
       [one] Mută { $count } fișă la coșul de gunoi
       [few] Mută { $count } fișe la coșul de gunoi
      *[other] Mută { $count } de fișe la coșul de gunoi
   }
reveal-in-outline = Arată în structură

## Corkboard “Move to…” destination picker
corkboard-move-to = Mută la…
corkboard-move-to-n = { $count ->
       [one] Mută { $count } fișă la…
       [few] Mută { $count } fișe la…
      *[other] Mută { $count } de fișe la…
   }
corkboard-move-picker-title = Mută la…
corkboard-move-picker-empty = Încă nu există clasoare. Creați mai întâi unul.
corkboard-move-picker-cancel = Renunță
corkboard-move-here = Mută aici
corkboard-moved-ok = { $count ->
       [one] { $count } fișă mutată
       [few] { $count } fișe mutate
      *[other] { $count } de fișe mutate
   }
corkboard-move-into-self = Un container nu poate fi mutat în interiorul lui însuși. Alegeți o destinație în afara lui.
corkboard-move-failed = Aceste fișe nu au putut fi mutate acolo.

## Overview (the container's contents as a sortable table)
overview-col-title = Titlu
overview-col-type = Tip
overview-col-label = Legendă
overview-col-tags = Etichete
# Only ever built when the Work has two or more Books: see `overview_columns`'s
# own gate, the same one every Books surface in this edition shares.
overview-col-books = Cărți
overview-col-own-words = Cuvinte
overview-col-total-words = Total
overview-row-count = { $count ->
       [one] { $count } element
       [few] { $count } elemente
      *[other] { $count } de elemente
   }
overview-search-placeholder = Filtrează elementele…
overview-expand-all = Extinde tot
overview-collapse-all = Restrânge tot
overview-table-label = Cuprins
overview-empty-title = Încă nu este nimic aici
overview-gone-title = Acest container nu mai există
overview-gone-hint = A fost mutat la coșul de gunoi. Restaurați-l sau închideți această filă.
overview-empty-hint = Folosiți „＋ Nou” de mai sus pentru a adăuga primul element.
corkboard-grid-label = Fișele de pe avizier
corkboard-rename-field = Redenumește elementul
corkboard-expand-synopsis = Extinde sinopsisul
corkboard-synopsis-modal-title = Sinopsis
corkboard-badge-scene = Scenă
corkboard-badge-chapter = Capitol
corkboard-badge-part = Parte
corkboard-badge-book = Carte
corkboard-badge-note = Notă
corkboard-badge-folder = Dosar
corkboard-badge-paratext = Paratext
corkboard-badge-text = Text
corkboard-badge-end = Sfârșit
settings-group-corkboard-layout = Aranjare
settings-group-corkboard-cards = Fișe
corkboard-show-word-count = Afișează numărul de cuvinte pe fișe
text-heading = Text
no-content = Acest element nu are conținut editabil.
untitled = Fără titlu
placeholder-title = Titlu…
placeholder-subtitle = Subtitlu…
placeholder-chapter-title = Titlul capitolului…
split-editor = Împarte editorul
close-split-view = Închide vizualizarea împărțită
drop-open-here = Deschide aici
drop-open-to-side = Deschide în lateral

## Manuscript streams (Full Chapter / Part / Book + Full Synopsis)
# The container's own page. "Chapter" is this chapter; "Full Chapter" is this chapter
# and every scene in it.
segment-chapter = Capitol
segment-part = Parte
segment-book = Carte
segment-pace = Ritm
segment-notes = Note
segment-story-bible = Biblia poveștii
# The three segments on an `Item/Note` tab: its own prose, its story-bible fields, and
# (only when it carries a discoverable tag) the manuscript prose it has been declared
# present in. See `teksilo_ui::tabs::item_note`.
segment-note-own = Notă
segment-note-details = Detalii
segment-note-in-prose = În proză
pace-placeholder = Planificatorul de ritm apare aici.
# Pace planner
pace-empty-title = Planificați ritmul acestei cărți
pace-empty-body = Stabiliți un obiectiv de cuvinte și un termen-limită, iar Skribisto calculează ritmul zilnic care vă duce acolo.
pace-start-planning = Începe planificarea
pace-section-schedule = Program
pace-section-progress = Progres
pace-advancement = Avansare
pace-goal = Obiectiv de cuvinte
pace-deadline = Termen-limită
pace-active = Ritm activ
pace-day-mon = Lun
pace-day-tue = Mar
pace-day-wed = Mie
pace-day-thu = Joi
pace-day-fri = Vin
pace-day-sat = Sâm
pace-day-sun = Dum
pace-card-written = cuvinte scrise
pace-card-of-goal = din obiectiv
pace-card-rate = cuvinte / zi de scris
pace-card-days-left = zile de scris rămase
pace-card-streak = zile la rând
pace-card-ahead = cuvinte peste ritm
pace-card-behind = cuvinte sub ritm
pace-charts-empty = Graficele de progres apar aici după ce numărul de cuvinte este înregistrat la o salvare.
pace-chart-progression = Cuvinte scrise față de țintă
pace-chart-words-per-day = Cuvinte pe zi
pace-series-actual = Realizat
pace-series-target = Țintă
pace-series-words-per-day = Cuvinte/zi
pace-daily-target-line = Ritm constant: { $count ->
       [one] { $count } cuvânt/zi
       [few] { $count } cuvinte/zi
      *[other] { $count } de cuvinte/zi
   }
pace-section-holidays = Zile libere
pace-section-milestones = Jaloane
pace-holidays-none = Nicio zi liberă. Fiecare zi programată contează.
pace-holiday-label = Numele zilei libere
pace-add-holiday = Adaugă
pace-remove = Elimină
pace-milestones-none = Încă nu există jaloane. Stabiliți unul pe o parte sau un capitol, din inspector.
full-chapter = Capitolul complet
full-part = Partea completă
full-book = Cartea completă
full-synopsis = Sinopsisul complet
rename = Redenumește
set-label = Setează legenda
insert-scene = Inserează o scenă
insert-chapter = Inserează un capitol
split-scene = Împarte scena
move-up = Mută în sus
move-down = Mută în jos
merge-with-previous = Unește cu precedenta
move-to-trash = Mută la coșul de gunoi
rename-chapter = Redenumește capitolul
add-scene = Adaugă o scenă nouă
add-chapter = Adaugă un capitol nou
new-scene-title = Scenă nouă
placeholder-scene-name = Numele scenei…
menu-cut = Decupează
menu-copy = Copiază
menu-paste = Lipește
menu-paste-unformatted = Lipește fără formatare
menu-select-all = Selectează tot
# Tooltips on the editor context menu's formatting row. Icon-only buttons, so
# the tooltip is their only accessible name, not decoration.
format-bold = Aldin
format-italic = Cursiv
format-underline = Subliniat
format-strikethrough = Tăiat

## Binder / recents
binder = Clasor
no-work = Niciun proiect încărcat
no-recent-works = Niciun proiect recent
switcher-open-section = Deschise acum
switcher-recent-section = Recente
switcher-this-window = această fereastră
open-project-title = Deschidere proiect
open-project-question = Cum doriți să deschideți „{ $title }”?
open-in-new-window = Deschide într-o fereastră nouă
open-here = Deschide aici

## Binder switcher + search
binder-all = Toate clasoarele
binder-show-all = Arată toate clasoarele
binder-new = Clasor nou…
# Pre-filled in the "New binder" dialog, so the writer can just press Enter.
# Stored literally from then on: a binder name is data, not a label.
binder-default-name = Clasor nou
binder-item-count = { $count ->
       [one] { $count } element
       [few] { $count } elemente
      *[other] { $count } de elemente
   }
binder-search-placeholder = Filtrează structura…
binder-search-scope = Caută în toate clasoarele
binder-trash-confirm-title = Mutați clasorul la coșul de gunoi?
binder-trash-confirm-text = „{ $name }” și toate elementele lui vor fi mutate la coșul de gunoi.

## Dialogs
dialog-rename = Redenumire
dialog-new-binder = Clasor nou
dialog-set-label = Setează legenda
dialog-new-scene = Scenă nouă
close-work-question = Salvați modificările înainte de a închide proiectul?
quit-question = Salvați modificările înainte de a ieși?
quit-save-work-question = Salvați modificările din { $title } înainte de a ieși?
unsaved-changes = Acest proiect are modificări nesalvate.
# Shown by Quit when another open Work (not this window's own) still has
# unsaved edits. The design's "one dialog listing every dirty Work" (see
# app::commands::file's `app.quit` action). Quit is refused until those are
# saved or closed from their own window.
# Replacing the open work in this window (New Work, Open Work, "Open here", the
# import toast's "Open now"), the same guard as closing, since the open work is
# closed either way.
new-work-unsaved-question = Salvați modificările înainte de a crea un proiect nou?
open-work-unsaved-question = Salvați modificările înainte de a deschide alt proiect?
switch-backup-discard-title = Renunțați la modificările din această copie de siguranță?
switch-backup-discard-text = Modificările dintr-o copie de siguranță nu pot fi salvate în ea. Folosiți „Salvează ca” sau „Restaurează” pentru a le păstra, ori renunțați la ele și deschideți celălalt proiect.
switch-save-failed = Proiectul nu a putut fi salvat, așa că nu a fost înlocuit: { $error }
switch-save-not-started = Proiectul nu a putut fi salvat, așa că nu a fost înlocuit.
close-save-failed = Proiectul nu a putut fi salvat, așa că nu a fost închis: { $error }
close-save-not-started = Proiectul nu a putut fi salvat, așa că nu a fost închis.
save-not-started = Proiectul nu a putut fi salvat.

## Toasts
# Toast title. Kept short because a toast title is one line and truncates; the cause goes
# in the body, which is the untranslated error text itself. $file is the file's base name.
could-not-open-work = „{ $file }” nu a putut fi deschis
# A project saved by a newer Skribisto than this one: title and body.
# $written_by is the version that wrote it, $requires the lowest version that can open it
# (the two differ when the newer build happened to use nothing new), $supported the newest
# this build understands.
could-not-open-work-too-new = „{ $file }” necesită o versiune mai nouă de Skribisto
could-not-open-work-too-new-detail = Salvat cu formatul Skribisto { $written_by }; deschiderea lui necesită formatul { $requires } sau mai nou, iar această versiune acceptă cel mult formatul { $supported }. Actualizați Skribisto pentru a-l deschide.
could-not-open-work-importing = „{ $file }” este în curs de scriere de către un import
could-not-open-work-importing-detail = Un import scrie peste acest proiect și nu s-a terminat. Deschideți-l după ce importul se încheie: deschis acum, ar arăta proiectul pe care importul îl înlocuiește, iar următoarea salvare ar scrie acel proiect înapoi peste import.
could-not-open-example = Exemplul nu a putut fi deschis: { $error }
# The browser (or whatever handles http links) could not be started for one of
# the Welcome sidebar's links. $url is shown so the address can still be copied.
could-not-open-link = { $url } nu a putut fi deschis: { $error }
could-not-create-work = Proiectul nu a putut fi creat: { $error }
saving-as-file = Se salvează ca { $target }…
saving-as-folder = Se salvează ca { $target }/…
saved-as = Salvat în { $target }
save-error = Salvarea nu a reușit: { $error }
save-flush-failed = Salvarea s-a făcut, dar aceste documente nu i-au putut fi transmise și sunt încă nesalvate: { $titles } ({ $error })
backup-error = Copia de siguranță nu a putut fi făcută: { $error }
backing-up = Se face copia de siguranță…
backup-nothing-open = Niciun proiect deschis pentru care să se facă o copie de siguranță.
backup-already-running = O copie de siguranță este deja în curs.
backup-complete = Copie de siguranță încheiată (salvate: { $ok }, deja la zi: { $skipped })
backup-partial = Copie de siguranță încheiată: salvate: { $ok }, { $failed ->
       [one] { $failed } destinație a eșuat
       [few] { $failed } destinații au eșuat
      *[other] { $failed } de destinații au eșuat
   }
backup-no-destination-title = Nicio locație disponibilă pentru copii de siguranță
backup-no-destination-text = Niciuna dintre destinațiile configurate pentru copii de siguranță nu poate fi accesată (de exemplu, o unitate externă poate fi deconectată). Conectați-o și reîncercați sau ieșiți fără copie de siguranță.
backup-failed-close-title = Copia de siguranță nu a putut fi salvată
backup-failed-close-text = Nicio copie de siguranță nu a putut fi scrisă înainte de închidere. Toate destinațiile au eșuat (unitatea poate fi scoasă, plină sau protejată la scriere). Remediați problema și reîncercați sau ieșiți fără copie de siguranță.
backup-failed-close-error-text = Nicio copie de siguranță nu a putut fi scrisă înainte de închidere: { $error }. Remediați problema și reîncercați sau ieșiți fără copie de siguranță.
backup-no-result = Copia de siguranță s-a încheiat fără să indice dacă a fost făcută o copie. Verificați lista copiilor de siguranță ca să vedeți dacă a apărut una nouă.

## Open-a-backup (choice modal + permanent banner + restore)
backup-choice-title = Fișier de copie de siguranță
backup-choice-heading = Ați deschis o copie de siguranță
backup-choice-subtitle = Aceasta este o copie de siguranță a unui proiect, la un anumit moment.
backup-choice-subtitle-dated = Copie de siguranță făcută la { $date }.
backup-choice-body = O puteți deschide și edita liber, dar modificările pot fi păstrate doar cu „Salvează ca”. Fișierul proiectului original nu este atins. Sau puteți readuce acest proiect exact la această copie de siguranță.
backup-choice-open = Deschide copia de siguranță
backup-choice-restore = Readu proiectul la acest moment…
backup-choice-not-a-backup = Nu, deschide-l normal
backup-banner-title = Copie de siguranță: modificările nu pot fi salvate aici
backup-banner-description = Folosiți „Salvează ca” pentru a păstra modificările într-un fișier nou sau „Restaurează” pentru a înlocui proiectul original cu această copie de siguranță.
backup-banner-restore = Restaurează…
backup-banner-save-as = Salvează ca…
backup-restore-original-missing = Proiectul original peste care să se facă restaurarea nu a fost găsit. Folosiți în schimb „Salvează ca” pentru a păstra această copie de siguranță ca proiect nou.
backup-restore-close-elsewhere-title = Proiect deschis în altă fereastră
backup-restore-close-elsewhere-text = Proiectul peste care restaurați este deschis în altă fereastră. Închideți-l mai întâi acolo, apoi reîncercați.
backup-restore-focus-window = Mergi la acea fereastră
backup-restore-confirm-title = Restaurați această copie de siguranță?
backup-restore-confirm-text = Versiunea actuală a proiectului va fi pusă deoparte ca o copie de siguranță de precauție înainte de a fi înlocuită cu aceasta.
backup-restore-confirm-ok = Restaurează
backup-restore-error = Restaurarea nu a reușit: { $error }
backup-restored-ok = Proiect restaurat.
backup-restored-with-safety = Proiect restaurat. Versiunea anterioară a fost salvată în { $path }.
close-backup-discard-title = Renunțați la modificările din această copie de siguranță?
close-backup-discard-text = Modificările dintr-o copie de siguranță nu pot fi salvate în ea. Folosiți „Salvează ca” pentru a le păstra sau renunțați la ele și închideți.
quit-backup-discard-title = Renunțați la modificări și ieșiți?
quit-backup-discard-work-question = Renunțați la modificările din { $title } și ieșiți?
quit-backup-discard-text = Modificările dintr-o copie de siguranță nu pot fi salvate în ea. Folosiți „Salvează ca” pentru a le păstra sau renunțați la ele și ieșiți.
backup-nudge-text = Nu sunt configurate copii de siguranță pentru acest proiect.
backup-nudge-action = Configurează copiile de siguranță…

## Backups list panel
menu-backups-list = Co&pii de siguranță…
backups-title = Copii de siguranță
backups-loading = Se caută copiile de siguranță…
backups-empty = Încă nu a fost găsită nicio copie de siguranță pentru acest proiect.
backups-open = Deschide
backups-reveal = Arată în dosar
backups-delete = Șterge această copie de siguranță
backups-delete-confirm-title = Ștergeți această copie de siguranță?
backups-delete-confirm-text = „{ $name }” va fi șters definitiv. Operația nu poate fi anulată.
backups-delete-error = Copia de siguranță nu a putut fi ștearsă: { $error }
backups-refresh = Reîmprospătează
backups-close = Închide

## Backup settings panes
settings-page-backup = Copii de siguranță implicite
settings-page-work-backup = Copii de siguranță
settings-backup-general-title = Setări implicite pentru copiile de siguranță
settings-backup-work-title = Copiile de siguranță ale acestui proiect
settings-backup-inherit = Folosește setările generale pentru copii de siguranță
settings-backup-inheriting = Acest proiect folosește setările generale pentru copii de siguranță.
settings-backup-none-hint = Nu sunt configurate copii de siguranță automate (toți declanșatorii sunt dezactivați).
settings-backup-last = Ultima copie de siguranță: { $date }
settings-backup-last-never = Încă nicio copie de siguranță.
settings-backup-open-list = Deschide lista copiilor de siguranță…
settings-backup-triggers = Când se fac copii de siguranță
settings-backup-on-close = La închiderea proiectului
settings-backup-on-open = La deschiderea proiectului
settings-backup-interval = Periodic, la fiecare
settings-backup-destinations = Destinațiile copiilor de siguranță
settings-backup-default-location = Locația implicită
settings-backup-usage-since = { $count ->
       [one] { $count } copie de siguranță · { $size } · cea mai veche: { $oldest }
       [few] { $count } copii de siguranță · { $size } · cea mai veche: { $oldest }
      *[other] { $count } de copii de siguranță · { $size } · cea mai veche: { $oldest }
   }
settings-backup-usage = { $count ->
       [one] { $count } copie de siguranță · { $size }
       [few] { $count } copii de siguranță · { $size }
      *[other] { $count } de copii de siguranță · { $size }
   }
settings-backup-usage-empty = Încă nu este păstrat nimic aici
settings-backup-usage-measuring = Se măsoară…
settings-backup-reveal-root = Arată dosarul
settings-backup-dest-none = Nicio destinație. Copiile de siguranță sunt salvate lângă proiect.
settings-backup-dest-remove = Elimină
settings-backup-dest-add = Adaugă un dosar…
settings-backup-dest-refresh = Reîmprospătează
settings-backup-retention = Câte se păstrează
settings-backup-retention-tiered = Eșalonat
settings-backup-retention-keep-n = Păstrează ultimele N
settings-backup-retention-tip = Cum sunt eliminate copiile de siguranță vechi.
settings-backup-retention-tip-more = Modul eșalonat păstrează o copie de siguranță pe oră timp de o zi, una pe zi timp de o săptămână, una pe săptămână timp de o lună și una pe lună după aceea, astfel încât istoricul recent rămâne dens, iar cel vechi se rărește. „Păstrează ultimele N” păstrează pur și simplu cele mai recente N copii. În ambele moduri, cele mai noi copii (minimul de mai jos) sunt păstrate întotdeauna.
settings-backup-gfs-hourly = Orare (ultimele 24 h)
settings-backup-gfs-daily = Zilnice (ultima săptămână)
settings-backup-gfs-weekly = Săptămânale (ultima lună)
settings-backup-gfs-monthly = Lunare
settings-backup-keep-n = Numărul de copii păstrate
settings-backup-min-keep = Păstrează întotdeauna cel puțin
settings-backup-dedup = Sari peste copia de siguranță dacă nu s-a schimbat nimic

## New-work dialog
new-work-title = Proiect nou
new-work-close = Închide
new-work-name = Numele proiectului
new-work-name-placeholder = Fără titlu
new-work-author = Autor
new-work-author-placeholder = Opțional
new-work-format = Format
new-work-single-file = Fișier unic
new-work-bundle = Dosar
new-work-convert-later = Puteți trece de la un format la altul și mai târziu.
new-work-location = Locație
new-work-will-create = Se va crea
new-work-language = Limba implicită
new-work-language-hint = Se aplică textelor noi și verificării ortografice. Fiecare text poate fi trecut pe altă limbă.
new-work-template = Șablon
new-work-template-none = Niciunul
new-work-template-empty-novel = Roman gol
new-work-template-light-novel = Light novel
new-work-template-novel = Roman
new-work-template-novel-in-parts = Roman în părți
new-work-template-notebook = Carnet de notițe
new-work-cancel = Renunță
new-work-create = Creează proiectul
# Wizard steps & navigation
new-work-step-details = Detalii
new-work-step-language = Limbă și structură
new-work-step-template = Șablon
new-work-back = Înapoi
new-work-next = Înainte
# The same wizard, opened from the Launcher's "From documents…"
new-work-documents-title = Proiect nou din documente
new-work-step-import = Import
new-work-create-and-import = Creează și importă…
new-work-documents-next-title = Urmează documentele dumneavoastră
new-work-documents-next-body = Crearea proiectului deschide peste el asistentul de import, unde alegeți documentele, verificați structura pe care Skribisto o citește din ele și alegeți unde ajung. Nimic nu este scris în proiect până nu confirmați acolo.
new-work-documents-no-template = Acest proiect pornește gol intenționat: fără șablon, astfel încât documentele importate să fie singurul lucru din el.
new-work-documents-chapter-scene-hint = Se aplică capitolelor create de import. O puteți schimba mai târziu din setările proiectului.
# Format tile descriptions
new-work-single-file-desc = O singură arhivă .skrib (zip). Portabilă, ușor de copiat ca siguranță.
new-work-bundle-desc = Un dosar care conține toate textele și resursele. Mai potrivit pentru controlul versiunilor.
# Template row trailing counts
new-work-template-none-count = clasor gol
new-work-template-empty-novel-count = 1 capitol
new-work-template-light-novel-count = 15 capitole
new-work-template-novel-count = 20 de capitole
new-work-template-novel-in-parts-count = 3 părți, 24 de capitole
new-work-template-notebook-count = note libere
# ChapterScene toggle (novel templates)
new-work-chapter-scene = Capitole simple
new-work-chapter-scene-tip = Fiecare capitol este un singur element în care scrieți direct, fără scene sub el. Lăsați opțiunea dezactivată pentru aranjarea clasică, în care un capitol este un dosar: scrieți direct și în el, dar poate conține și scene.
new-work-chapter-scene-tip-more = Scrieți într-un capitol în ambele cazuri. Singura diferență este dacă poate *conține* scene. Arborele clasorului din Skribisto este doar organizatoric, așa că ambele aranjări se compilează în aceeași carte, le puteți combina liber, iar „Convertește în” trece un capitol dintr-o formă în alta fără să piardă niciun cuvânt.
# Field validation
new-work-name-required = Introduceți un nume pentru proiect
new-work-name-invalid = Acest nume nu conține caractere utilizabile
new-work-location-required = Alegeți o locație
new-work-location-missing = Acest dosar nu există
new-work-location-not-folder = Această cale nu este un dosar
new-work-location-readonly = Nu se poate scrie în acest dosar
new-work-target-exists = Un proiect cu acest nume există deja în acest dosar. La creare, vi se va cere confirmarea înainte de a-l înlocui
new-work-target-is-folder = Un dosar cu acest nume există deja în această locație. Alegeți alt nume
new-work-overwrite-title = Înlocuiți proiectul existent?
new-work-overwrite-text = „{ $name }” există deja în acest dosar. Îl înlocuiți cu proiectul nou?
new-work-target-open-title = Acest proiect este deschis
new-work-target-open-text = „{ $name }” este deschis în Skribisto. Crearea unui proiect nou în locul lui ar înlocui proiectul la care lucrați, iar următoarea salvare ar scrie acel proiect înapoi peste cel nou. Alegeți alt nume sau alt dosar, ori închideți mai întâi proiectul.
target-importing-title = Un import scrie acest proiect
target-importing-text = Un import încă scrie „{ $name }”. Așteptați să se termine sau alegeți alt nume.
backup-restore-importing-text = Un import încă scrie „{ $name }”, proiectul peste care ar fi restaurată această copie de siguranță. Restaurați copia după ce importul se încheie.

## New-work template labels (passed to the backend, which can't do i18n)
new-work-manuscript = Manuscris
new-work-notes = Note
new-work-research = Documentare
new-work-notebook = Carnet de notițe
new-work-scene = Scenă
new-work-note = Notă
new-work-front-matter = Pagini de început
new-work-back-matter = Pagini de sfârșit
new-work-paratext = Schema cărții
new-work-paratext-none = Fără schemă
new-work-paratext-hint = Paginile de început și de sfârșit cu care o tradiție deschide și încheie o carte. Le puteți muta, redenumi sau șterge oricând ulterior.
new-work-tags = Etichete
new-work-tags-none = Fără etichete
new-work-tags-hint = O paletă de pornire pentru etichetarea personajelor, locurilor și a restului. Opțională; fiecare etichetă poate fi ulterior redenumită, recolorată sau ștearsă.
new-work-note-templates = Șabloane de note
new-work-note-templates-none = Fără șabloane
new-work-note-templates-hint = Forma de la care pornește o notă din biblia poveștii. Opțional; fiecare șablon poate fi ulterior editat sau șters.
new-work-characters = Personaje
new-work-places = Locuri

## Import Plume Creator dialog
import-plume-title = Importă un proiect Plume Creator
import-plume-source = Proiect Plume
import-plume-source-hint = Alegeți un fișier .plume sau .plume_backup (orice versiune Plume Creator).
import-plume-trash-warning = ⚠ Elementele aruncate la coșul de gunoi sau șterse nu sunt preluate.
# Field validation
import-plume-source-required = Alegeți un fișier de proiect Plume
import-plume-source-missing = Acest fișier nu există
import-plume-source-not-file = Această cale nu este un fișier
# Binder names passed to the backend (which can't do i18n)
import-plume-manuscript-binder = Manuscris
import-plume-story-bible-binder = Biblia poveștii
import-plume-notes-of = { $name } (note)
import-plume-content-of = { $name } (text)
# Progress toast (the import runs as a long operation)
import-plume-progress-title = Se importă proiectul Plume…
# Result
import-plume-done = { $imported ->
       [one] A fost importat { $imported } element.
       [few] Au fost importate { $imported } elemente.
      *[other] Au fost importate { $imported } de elemente.
   } { $skipped ->
       [one] { $skipped } element aruncat la coșul de gunoi nu a fost preluat.
       [few] { $skipped } elemente aruncate la coșul de gunoi nu au fost preluate.
      *[other] { $skipped } de elemente aruncate la coșul de gunoi nu au fost preluate.
   }
# The project was refused unread because one of its parts nests its XML deeper
# than the app will read. $part is the part's own name (tree, attendance, info);
# $limit is the ceiling, a number.
import-plume-nested-too-deep = Proiectul nu a fost importat: partea sa „{ $part }” este imbricată pe mai mult de { $limit } niveluri. Niciun proiect Plume Creator nu se apropie de așa ceva, iar citirea lui ar putea bloca Skribisto. Nu s-a scris nimic.
# The project was refused unread because one of its parts declares XML
# entities, which could expand into more memory than the computer has. $part
# is the part's own name (tree, attendance, info).
import-plume-declares-entities = Proiectul nu a fost importat: partea sa „{ $part }” declară entități XML, lucru pe care niciun proiect Plume Creator nu îl face. Extinderea lor ar putea cere mai multă memorie decât are calculatorul și ar putea bloca Skribisto. Nu s-a scris nimic.
import-plume-no-outline = Acest fișier este o arhivă comprimată, dar nu un proiect Plume Creator: nu are structură. Nu s-a scris nimic.
import-plume-empty-outline = Fișierul de structură al acestui proiect Plume Creator este gol, așa că nu este nimic de importat. Nu s-a scris nimic.
# $tag is the name of the document the part turned out to be.
import-plume-wrong-document = Acesta nu este un proiect Plume Creator: partea sa „{ $part }” este un document „{ $tag }”. Nu s-a scris nimic.
# $version is the part's own version, $supported the newest this importer reads.
import-plume-too-new = Partea „{ $part }” a acestui proiect a fost scrisă de un Plume Creator mai nou (versiunea { $version }) decât poate citi Skribisto (până la { $supported }). Nu s-a scris nimic.

## Import Manuskript project dialog
import-manuskript-title = Importă un proiect Manuskript
import-manuskript-source = Proiect Manuskript
# A Manuskript project is a .msk file plus, in its usual mode, a folder of the
# same name beside it. Either one gets you there, and so does the folder itself.
import-manuskript-source-hint = Alegeți fișierul .msk sau dosarul proiectului (orice versiune Manuskript).
import-manuskript-source-file = Alege fișierul…
import-manuskript-source-folder = Alege dosarul…
# Field validation
import-manuskript-source-required = Alegeți un proiect Manuskript
import-manuskript-source-missing = Acest fișier sau dosar nu există
# Names passed to the backend (which can't do i18n). Manuskript stores none of
# these: it has no binders, no story-bible groups, and its importance scale is
# three numbers whose names live in its own interface.
import-manuskript-manuscript-binder = Manuscris
import-manuskript-story-bible-binder = Biblia poveștii
import-manuskript-characters-group = Personaje
import-manuskript-world-group = Lume
import-manuskript-plots-group = Intrigi
import-manuskript-project-info-note = Informații despre proiect
import-manuskript-summary-note = Rezumat
import-manuskript-importance-minor = Minor
import-manuskript-importance-secondary = Secundar
import-manuskript-importance-main = Principal
import-manuskript-notes-of = { $title } (note)
# Progress toast (the import is a long operation)
import-manuskript-progress-title = Se importă proiectul Manuskript…
# Result
import-manuskript-done = { $imported ->
       [one] A fost importat { $imported } element.
       [few] Au fost importate { $imported } elemente.
      *[other] Au fost importate { $imported } de elemente.
   } { $revisions ->
       [0] { "" }
       [one] A fost preluată și { $revisions } versiune anterioară.
       [few] Au fost preluate și { $revisions } versiuni anterioare.
      *[other] Au fost preluate și { $revisions } de versiuni anterioare.
   }
# The project was refused unread because one of its files nests its XML deeper
# than the app will read. $part is that file's name inside the project
# (world.opml, plots.xml…); $limit is the ceiling, a number.
import-manuskript-nested-too-deep = Proiectul nu a fost importat: fișierul său „{ $part }” este imbricat pe mai mult de { $limit } niveluri. Niciun proiect Manuskript nu se apropie de așa ceva, iar citirea lui ar putea bloca Skribisto. Nu s-a scris nimic.
# The project was refused unread because one of its files sits in folders
# nested deeper than the app will read (Manuskript keeps a project's outline
# as folders, one per level). $limit is the ceiling, a number.
import-manuskript-folders-too-deep = Proiectul nu a fost importat: unul dintre fișierele sale se află la o adâncime de peste { $limit } dosare. Niciun proiect Manuskript nu se apropie de așa ceva, iar citirea lui ar putea bloca Skribisto. Nu s-a scris nimic.
# The project was refused unread because one of its files declares XML
# entities, which could expand into more memory than the computer has. $part
# is that file's name inside the project (world.opml, plots.xml…).
import-manuskript-declares-entities = Proiectul nu a fost importat: fișierul său „{ $part }” declară entități XML, lucru pe care niciun proiect Manuskript nu îl face. Extinderea lor ar putea cere mai multă memorie decât are calculatorul și ar putea bloca Skribisto. Nu s-a scris nimic.
import-manuskript-not-a-project-file = „{ $file }” nu este nici o arhivă de proiect Manuskript, nici micul fișier .msk care indică dosarul unui proiect. Nu s-a scris nimic.
import-manuskript-folder-missing = „{ $stub }” indică un dosar de proiect numit „{ $folder }”, aflat lângă el, dar acel dosar nu este acolo. Un proiect Manuskript în modul dosar este alcătuit din amândouă, iar ele trebuie mutate și copiate întotdeauna împreună. Nu s-a scris nimic.
import-manuskript-not-a-project = Acesta nu pare a fi un proiect Manuskript: nu are niciun marcaj Manuskript și nicio structură. Nu s-a scris nimic.
import-manuskript-format-too-new = Acest proiect este în formatul Manuskript { $version }, mai nou decât poate citi Skribisto (până la { $supported }). Nu s-a scris nimic.
import-manuskript-too-many-files = Proiectul nu a fost importat: numărul fișierelor sale depășește { $limit }, mult peste cel al oricărui proiect Manuskript. Nu s-a scris nimic.
import-manuskript-too-large = Proiectul nu a fost importat: depășește cei { $limit } MB pe care Skribisto îi citește dintr-un proiect Manuskript. Nu s-a scris nimic.
project-import-close = Închide
project-import-location = Dosarul de destinație
# The folder dialog's title.
project-import-location-title = Alegeți un dosar de destinație
project-import-name = Numele fișierului
project-import-name-placeholder = Numele proiectului
project-import-will-create = Se va crea
project-import-cancel = Renunță
project-import-import = Importă
# Field validation
project-import-location-required = Alegeți un dosar de destinație
project-import-location-missing = Acest dosar nu există
project-import-location-not-folder = Această cale nu este un dosar
project-import-location-readonly = Nu se poate scrie în acest dosar
project-import-name-required = Introduceți un nume de fișier
project-import-name-exists = Aici există deja un fișier cu acest nume. Importul va cere confirmarea înlocuirii
project-import-name-is-folder = Aici există deja un dosar cu acest nume, poate un proiect păstrat ca dosar. Un import nu îl poate înlocui: alegeți alt nume
# Overwrite confirmation
project-import-overwrite-title = Înlocuiți fișierul existent?
project-import-overwrite-text = „{ $name }” există deja. Îl înlocuiți cu proiectul importat?
# Projects are imported one at a time, whichever importer asks.
project-import-busy-title = Se importă deja un alt proiect
project-import-busy-text = Proiectele se importă pe rând. Așteptați să se termine importul în curs sau anulați-l, apoi porniți-l din nou pe acesta.
# Spoken by a screen reader once per phase, never shown: the toast's title, then
# the phase's name.
project-import-progress-announcement = { $title } { $phase }
# The notice once Cancel is pressed, until the converter has stopped: it may be
# finishing a file it cannot leave half written.
project-import-stopping = Se oprește importul…
project-import-cancelled = Import anulat
project-import-open-now = Deschide acum
project-import-details = Detalii
# The import's report: what the writer should know about what came across.
project-import-report = { $count ->
    [one] Un lucru de știut despre acest import
    [few] { $count } lucruri de știut despre acest import
   *[other] { $count } de lucruri de știut despre acest import
}
project-import-report-title = Despre acest import
# Error toast: a short reason in the body, the full technical chain behind Details.
project-import-error-title = Proiectul nu a putut fi importat
# Why a project was not imported, whichever importer read it. A $file is a file
# name, a $folder a full path (it is the folder the writer has to act on), a
# $reason what the system or a parser said, a $limit a number.
project-import-failed-cancelled = Importul a fost anulat. Nu s-a scris nimic.
project-import-failed-target-exists = „{ $file }” există deja, iar importul nu avea voie să îl înlocuiască. Nu s-a scris nimic.
project-import-failed-target-folder = „{ $file }” este un dosar, poate un proiect păstrat ca dosar, iar un import nu îl poate înlocui. Alegeți alt nume. Nu s-a scris nimic.
project-import-failed-not-written = „{ $file }” nu a putut fi scris în „{ $folder }” ({ $reason }). Nu s-a scris nimic.
project-import-failed-disk-full = Nu mai este destul spațiu în „{ $folder }” pentru a scrie „{ $file }”. Eliberați spațiu acolo, apoi importați din nou. Nu s-a scris nimic.
project-import-failed-source-missing = „{ $file }” nu mai este acolo. Nu s-a scris nimic.
project-import-failed-unreadable = Skribisto nu a putut citi „{ $file }” ({ $reason }). Nu s-a scris nimic.
project-import-failed-archive-damaged = „{ $file }” este un fișier comprimat care nu a putut fi citit, probabil pentru că este deteriorat ({ $reason }). Nu s-a scris nimic.
project-import-failed-archive-entries = Proiectul nu a fost importat: arhiva lui enumeră { $entries ->
    [one] un fișier
    [few] { $entries } fișiere
   *[other] { $entries } de fișiere
}, mai multe decât cele { $limit } pe care le poate conține un proiect, lucru pe care îl face doar un fișier construit ca să epuizeze memoria. Nu s-a scris nimic.
project-import-failed-archive-member = Proiectul nu a fost importat: partea sa „{ $part }” se dezarhivează la peste { $limit } MB, lucru pe care îl face doar un fișier construit ca să epuizeze memoria. Nu s-a scris nimic.
project-import-failed-archive-size = Proiectul nu a fost importat: se dezarhivează la peste { $limit } MB, lucru pe care îl face doar un fișier construit ca să epuizeze memoria. Nu s-a scris nimic.
project-import-failed-nested = Proiectul nu a fost importat: partea sa „{ $part }” este imbricată pe mai mult de { $limit } niveluri, iar citirea ei ar putea bloca Skribisto. Nu s-a scris nimic.
project-import-failed-entities = Proiectul nu a fost importat: partea sa „{ $part }” declară entități XML. Extinderea lor ar putea cere mai multă memorie decât are calculatorul și ar putea bloca Skribisto. Nu s-a scris nimic.
project-import-failed-folders = Proiectul nu a fost importat: unul dintre fișierele sale se află la o adâncime de peste { $limit } dosare, iar citirea lui ar putea bloca Skribisto. Nu s-a scris nimic.
project-import-failed-malformed = Proiectul nu a fost importat: partea sa „{ $part }” este deteriorată și nu poate fi citită ({ $reason }). Nu s-a scris nimic.
# A Scrivener project the importer cannot bring across (a Scrivener 1 for Mac project, a
# damaged binder file). Why is the Scrivener importer's own to say; Details holds its key.
project-import-failed-scrivener-refused = Skribisto nu poate importa acest proiect Scrivener. Nu s-a scris nimic.
# The Scrivener project asks something its review did not answer: a project or a binder
# file appeared in it since.
project-import-failed-scrivener-question = Proiectul Scrivener pune o întrebare la care verificarea lui nu a răspuns. Reporniți importul lui și răspundeți la întrebare, apoi importați. Nu s-a scris nimic.
# The Scrivener project raises a question the writer has to accept before it imports.
project-import-failed-scrivener-not-accepted = Înainte de a putea fi importat, proiectul Scrivener vă cere să bifați o casetă la pasul Proiect, iar caseta nu a fost bifată. Reporniți importul lui și bifați-o, apoi importați. Nu s-a scris nimic.
# The Scrivener project changed after the writer reviewed it.
project-import-failed-scrivener-changed = Proiectul Scrivener s-a modificat după ce l-ați verificat. Reporniți importul lui și verificați-l din nou, apoi importați. Nu s-a scris nimic.
# An error nothing above accounts for; $detail is its technical description.
project-import-failed-internal = Importul s-a oprit la o eroare pe care Skribisto nu o aștepta: { $detail }

## Project import, shared by the Plume Creator and Manuskript dialogs
# The chosen destination is a project open in a window, here or in another
# running copy of Skribisto. The import is refused rather than written over it.
import-target-open-title = Acest proiect este deschis
import-target-open-text = „{ $name }” este deschis în Skribisto. Importul peste el ar înlocui proiectul la care lucrați, iar următoarea salvare ar scrie acel proiect înapoi peste import. Alegeți alt nume de fișier sau închideți mai întâi proiectul.
# Spoken by a screen reader when an import's warnings notice appears, never shown.
# $result is the import's own result line ("Imported 12 items."), $notice the
# notice's title ("2 things could not be imported exactly"). The notice is read out
# over the result, so it says both.
import-warnings-announcement = { $result } { $notice }.
# Quitting while an import runs. $name is the file the import is writing.
quit-import-running-title = Un import este încă în curs
quit-import-running-text = Skribisto încă importă „{ $name }”. Dacă ieșiți acum, importul se oprește: nu se scrie nimic, iar proiectul din care importați rămâne așa cum era.
quit-import-stop = Oprește importul și ieși

## Import a Scrivener project: the wizard
import-scrivener-title = Importă un proiect Scrivener
import-scrivener-step-project = Proiect
import-scrivener-step-review = Verificare
import-scrivener-step-save = Salvare
import-scrivener-back = Înapoi
import-scrivener-next = Înainte
import-scrivener-import = Importă
import-scrivener-cancel = Renunță
import-scrivener-help = Ajutor

## Project step: choosing the project
import-scrivener-drop-title = Plasați aici un proiect Scrivener
import-scrivener-drop-hint = Un proiect .scriv, fișierul .scrivx din interiorul lui, o copie de siguranță .zip sau un .scrivtemplate
import-scrivener-source = Proiect Scrivener
import-scrivener-source-placeholder = Calea către proiect
import-scrivener-choose-file = Alege un fișier…
import-scrivener-choose-folder = Alege un dosar…
# Windows and Linux show a .scriv project as an ordinary folder.
import-scrivener-source-hint = Alegeți dosarul .scriv al proiectului sau deschideți-l și alegeți fișierul .scrivx din interior. Merge și o copie de siguranță .zip.
# macOS shows a .scriv project as a single file when Scrivener is installed, as a folder when it is not.
import-scrivener-source-hint-mac = Dacă proiectul apare ca un singur fișier, folosiți „Alege un fișier…”. Dacă apare ca dosar, folosiți „Alege un dosar…”. Merge și o copie de siguranță .zip.
# File-dialog titles and filter label, resolved when the dialog opens.
import-scrivener-dialog-file-title = Alegeți un proiect Scrivener
import-scrivener-dialog-folder-title = Alegeți dosarul unui proiect Scrivener
import-scrivener-dialog-filter = Proiect Scrivener
import-scrivener-source-missing = Nu există nimic la această cale
# A path whose name is not Unicode: the field shows it with its unreadable letters replaced.
import-scrivener-source-unreadable-name = Acest nume nu poate fi citit. Redenumiți proiectul în managerul de fișiere, apoi alegeți-l din nou.
import-scrivener-empty-hint = Alegeți un proiect ca să vedeți ce găsește Skribisto în el.
import-scrivener-reading = Se citește proiectul…
import-scrivener-stop-reading = Oprește citirea
import-scrivener-read-failed = Proiectul nu a putut fi citit
import-scrivener-read-failed-body = Ceva nu a mers bine la citirea lui. Alegeți „Recitește proiectul” ca să încercați din nou sau „Detalii” ca să vedeți ce nu a mers.
import-scrivener-details = Detalii
import-scrivener-check-again = Recitește proiectul
# A refusal this version does not know how to word.
import-scrivener-refusal-unknown-title = Acest proiect nu poate fi importat
import-scrivener-refusal-unknown-body = Skribisto nu știe cum să îl preia. Detaliile indică motivul.

## Project step: the project card
import-scrivener-made-with-mac = Salvat ultima dată de Scrivener { $version } pentru macOS
import-scrivener-made-with-windows = Salvat ultima dată de Scrivener { $version } pentru Windows
import-scrivener-made-with-linux = Salvat ultima dată de Scrivener { $version } pentru Linux
import-scrivener-made-with-ios = Salvat ultima dată de Scrivener { $version } pentru iPhone și iPad
import-scrivener-made-with-version = Salvat ultima dată de Scrivener { $version }
import-scrivener-made-with-unknown = Proiect Scrivener, formatul { $format }
import-scrivener-saved-on = Salvat la { $date } pe „{ $device }”
import-scrivener-saved-on-no-device = Salvat la { $date }
import-scrivener-card-counts = { $documents ->
    [one] Un document
    [few] { $documents } documente
   *[other] { $documents } de documente
}, dintre care { $manuscript } în manuscris

## Review step
import-scrivener-review-hint = Așa va fi preluat clasorul dumneavoastră. Nu se scrie nimic până nu importați, iar ulterior multe elemente își pot schimba tipul în clasor, cu „Convertește în”.
import-scrivener-choices-structure = Niveluri
import-scrivener-top-level = Dosarele de pe primul nivel
import-scrivener-choices-text = Text
import-scrivener-choice-containers = Dosarele de pe nivelul { $depth }
import-scrivener-choice-texts = Documentele de pe nivelul { $depth }
import-scrivener-choice-count = { $count ->
    [one] Un element
    [few] { $count } elemente
   *[other] { $count } de elemente
}
import-scrivener-choice-count-example = { $count ->
    [1] Un element: „{ $example }”
    [few] { $count } elemente, de exemplu „{ $example }”
   *[other] { $count } de elemente, de exemplu „{ $example }”
}
import-scrivener-rung-part = Părți
import-scrivener-rung-chapter = Capitole
import-scrivener-rung-plain = Dosare simple
import-scrivener-texts-scenes = Scene
import-scrivener-texts-chapters = Capitole
import-scrivener-choice-books = Cărțile din manuscris
import-scrivener-books-one = O singură carte
import-scrivener-books-several = O carte pentru fiecare dosar
import-scrivener-choice-split = { $count ->
    [1] Împarte „{ $title }” la titlul său de secțiune
    [few] Împarte „{ $title }” la cele { $count } titluri de secțiune ale sale
   *[other] Împarte „{ $title }” la cele { $count } de titluri de secțiune ale sale
}
import-scrivener-root-items = { $count ->
    [one] Un element
    [few] { $count } elemente
   *[other] { $count } de elemente
}
import-scrivener-root-items-manuscript = { $count ->
    [one] Un element. Acest dosar pare să fie manuscris.
    [few] { $count } elemente. Acest dosar pare să fie manuscris.
   *[other] { $count } de elemente. Acest dosar pare să fie manuscris.
}
import-scrivener-dest-notes = Note
import-scrivener-dest-story-bible = Biblia poveștii
import-scrivener-dest-manuscript = O altă carte
import-scrivener-dest-leave-out = Exclude
import-scrivener-choice-trash = { $count ->
    [one] Preia coșul de gunoi (un element)
    [few] Preia coșul de gunoi ({ $count } elemente)
   *[other] Preia coșul de gunoi ({ $count } de elemente)
}
import-scrivener-trash-detail = { $count ->
    [0] Niciunul dintre ele nu conține text.
    [one] Unul dintre ele conține text.
    [few] { $count } dintre ele conțin text.
   *[other] { $count } dintre ele conțin text.
}
import-scrivener-choice-blank-lines = Rândurile goale devin separatoare de scenă
import-scrivener-choice-ornaments = Rândurile de ornamente devin separatoare de scenă
import-scrivener-preview-label = Structura noului proiect
import-scrivener-col-title = Titlu
import-scrivener-col-becomes = Devine
import-scrivener-col-exports = În exporturi
import-scrivener-in-exports-yes = Da
import-scrivener-in-exports-no = Nu
# What a row the binder has no creation command for becomes.
import-scrivener-becomes-item = Element
import-scrivener-becomes-trashed = În coșul de gunoi
import-scrivener-before-title = Înainte de import
import-scrivener-review-error-unreadable = Verificarea nu a putut fi pregătită. Alegeți din nou proiectul.
import-scrivener-review-error-choice = Această alegere nu se potrivește proiectului. Alegeți din nou proiectul.
import-scrivener-review-error-unmappable = Cu această alegere, clasorul nu ar putea fi preluat fără să se piardă ceva. Alegeți altfel sau raportați problema.

## Save step
import-scrivener-save-hint = Importul creează un proiect Skribisto nou. Proiectul dumneavoastră Scrivener este doar citit, niciodată modificat.
import-scrivener-title-field = Titlu
import-scrivener-author = Autor
import-scrivener-language = Limbă
import-scrivener-location-inside-project = Alegeți un dosar din afara proiectului Scrivener. Scrivener păstrează acolo doar propriile fișiere.
import-scrivener-copy-research-label = Copiază fișierele de documentare lângă noul proiect
import-scrivener-copy-research = { $count ->
    [one] Un fișier de documentare pe care Skribisto nu îl poate deschide, de exemplu un PDF sau un sunet, este pus într-un dosar lângă proiect. Dacă opțiunea nu este bifată, nota lui spune unde se află în proiectul dumneavoastră Scrivener.
    [few] { $count } fișiere de documentare pe care Skribisto nu le poate deschide, de exemplu PDF-uri sau sunete, sunt puse într-un dosar lângă proiect. Dacă opțiunea nu este bifată, notele lor spun unde se află în proiectul dumneavoastră Scrivener.
   *[other] { $count } de fișiere de documentare pe care Skribisto nu le poate deschide, de exemplu PDF-uri sau sunete, sunt puse într-un dosar lângă proiect. Dacă opțiunea nu este bifată, notele lor spun unde se află în proiectul dumneavoastră Scrivener.
}
# The file name proposed when the project's name holds nothing a file name can.
import-scrivener-default-name = Proiect Scrivener

## Progress and result
import-scrivener-progress-title = Se importă proiectul Scrivener…
import-scrivener-done-title = Proiectul Scrivener a fost importat
# How many rows the new project holds: the documents and folders that came across, and
# the rows the import made for them (a document's notes, the project notes).
import-scrivener-done = { $items ->
    [one] Noul proiect conține un element.
    [few] Noul proiect conține { $items } elemente.
   *[other] Noul proiect conține { $items } de elemente.
}
# The same, with how many kinds of thing could not be read or need checking.
import-scrivener-done-check = { $items ->
    [one] Noul proiect conține un element.
    [few] Noul proiect conține { $items } elemente.
   *[other] Noul proiect conține { $items } de elemente.
} { $findings ->
    [one] Un lucru necesită atenție.
    [few] { $findings } lucruri necesită atenție.
   *[other] { $findings } de lucruri necesită atenție.
}
import-scrivener-review-again = Reverifică importul

## The import's report (the dialog See report opens)
import-scrivener-see-report = Vezi raportul
import-scrivener-report-from = Citit din { $path }
import-scrivener-report-to = Scris în { $path }
import-scrivener-report-came-across = Ce a fost preluat
import-scrivener-report-nothing-counted = Nu a fost numărat nimic.
import-scrivener-report-items = { $count ->
    [one] Un element
    [few] { $count } elemente
   *[other] { $count } de elemente
}
import-scrivener-report-words = { $count ->
    [one] Un cuvânt
    [few] { $count } cuvinte
   *[other] { $count } de cuvinte
}
import-scrivener-report-synopses = { $count ->
    [one] Un sinopsis
    [few] { $count } sinopsisuri
   *[other] { $count } de sinopsisuri
}
import-scrivener-report-document-notes = { $count ->
    [one] Notele unui document
    [few] Notele a { $count } documente
   *[other] Notele a { $count } de documente
}
import-scrivener-report-comments = { $count ->
    [one] Un comentariu
    [few] { $count } comentarii
   *[other] { $count } de comentarii
}
import-scrivener-report-footnotes = { $count ->
    [one] O notă de subsol
    [few] { $count } note de subsol
   *[other] { $count } de note de subsol
}
import-scrivener-report-pictures = { $count ->
    [one] O imagine
    [few] { $count } imagini
   *[other] { $count } de imagini
}
import-scrivener-report-templates = { $count ->
    [one] Un șablon de notă
    [few] { $count } șabloane de notă
   *[other] { $count } de șabloane de notă
}
import-scrivener-report-tags = { $count ->
    [one] O etichetă
    [few] { $count } etichete
   *[other] { $count } de etichete
}
import-scrivener-report-statuses = { $count ->
    [one] O stare
    [few] { $count } stări
   *[other] { $count } de stări
}
import-scrivener-report-trashed = { $count ->
    [one] Un element în coșul de gunoi
    [few] { $count } elemente în coșul de gunoi
   *[other] { $count } de elemente în coșul de gunoi
}
import-scrivener-report-research-files = { $count ->
    [one] Un fișier de documentare
    [few] { $count } fișiere de documentare
   *[other] { $count } de fișiere de documentare
}
import-scrivener-report-choices = Ce ați ales
import-scrivener-report-accepted-mobile = Ați importat proiectul fără modificările făcute în Scrivener pentru iPhone sau iPad care nu ajunseseră încă în el.
import-scrivener-report-accepted-no-text = Ați importat această copie cu documentele goale, deoarece textul lor nu se afla în ea.
import-scrivener-report-choice = { $subject }: { $answer }
import-scrivener-report-yes = da
import-scrivener-report-no = nu
import-scrivener-report-research-copied = Fișierele de documentare pe care Skribisto nu le poate deschide au fost copiate în „{ $folder }”, lângă noul proiect.
import-scrivener-report-research-not-copied = Fișierele de documentare pe care Skribisto nu le poate deschide nu au fost copiate. Notele lor spun unde se află în proiectul dumneavoastră Scrivener.
import-scrivener-report-research-copied-except = { $count ->
    [one] Fișierele de documentare pe care Skribisto nu le poate deschide au fost copiate în „{ $folder }”, lângă noul proiect, cu excepția unuia, care rămâne în proiectul dumneavoastră Scrivener. Nota lui spune unde se află.
    [few] Fișierele de documentare pe care Skribisto nu le poate deschide au fost copiate în „{ $folder }”, lângă noul proiect, cu excepția a { $count } dintre ele, care rămân în proiectul dumneavoastră Scrivener. Notele lor spun unde se află.
   *[other] Fișierele de documentare pe care Skribisto nu le poate deschide au fost copiate în „{ $folder }”, lângă noul proiect, cu excepția a { $count } dintre ele, care rămân în proiectul dumneavoastră Scrivener. Notele lor spun unde se află.
}
import-scrivener-report-research-none-copied = { $count ->
    [1] Ați cerut copierea fișierelor de documentare, dar singurul fișier de documentare nu a putut fi copiat. Rămâne în proiectul dumneavoastră Scrivener, iar nota lui spune unde se află.
   *[other] Ați cerut copierea fișierelor de documentare, dar niciunul dintre cele { $count } nu a putut fi copiat. Rămân în proiectul dumneavoastră Scrivener, iar notele lor spun unde se află.
}
import-scrivener-report-chose-project = Ați ales să importați „{ $project }”, unul dintre mai multe proiecte aflate în dosarul sau în arhiva pe care ați selectat-o.
import-scrivener-report-chose-manifest = Ați ales fișierul de clasor „{ $file }”.
import-scrivener-report-chose-enclosing = Ați ales ceva din interiorul proiectului „{ $project }”, așa că a fost importat întregul proiect.
import-scrivener-report-chose-upgraded = Ați importat „{ $current }”, proiectul în care ați continuat să scrieți, nu copia păstrată de Scrivener înainte de o actualizare.
import-scrivener-report-chose-backup = Ați importat „{ $backup }”, copia mai veche păstrată de Scrivener înainte de o actualizare.
import-scrivener-report-title-placeholder-rejected = Titlul din setările proiectului era un text provizoriu dintr-un șablon, așa că Skribisto a propus în schimb numele proiectului.
import-scrivener-report-author-placeholder-rejected = Numele autorului din setările proiectului era un text provizoriu dintr-un șablon, așa că Skribisto nu a propus niciun nume.
import-scrivener-report-title-from-compile-settings = Skribisto a propus titlul indicat în setările de compilare ale proiectului: numele pe care îl avea proiectul la creare, dacă nu a fost stabilit acolo un alt titlu în Scrivener.
import-scrivener-report-title-from-template = Skribisto a propus titlul propriu al șablonului.
import-scrivener-report-title-from-folder-name = Skribisto a propus numele proiectului drept titlu.
import-scrivener-report-title-from-project-properties = Skribisto a propus titlul indicat în proprietățile proiectului.
import-scrivener-report-author-from-project = Skribisto a propus numele autorului indicat în setările proiectului. Acolo poate rămâne autorul unui șablon, iar numele lui ar apărea pe o pagină de titlu.
import-scrivener-report-language-not-stated = Proiectul nu își indica limba, așa că Skribisto a propus limba propriei interfețe. Limba decide cum sunt numărate cuvintele și cum sunt scrise ghilimelele.
import-scrivener-report-language-from-script = Skribisto a propus limba după sistemul de scriere al manuscrisului ({ $tag }).
# A proposal of the Save page the writer replaced with their own.
import-scrivener-report-you-changed-it = { $hint } Ați înlocuit propunerea.
import-scrivener-report-content-salvaged-text = Textul elementului „{ $item }” lipsea din proiect. Cuvintele lui provin din indexul de căutare al lui Scrivener, ca text simplu și poate dintr-o versiune mai veche: verificați-le.
import-scrivener-report-content-salvaged-notes = Notele elementului „{ $item }” lipseau din proiect. Cuvintele lor provin din indexul de căutare al lui Scrivener, ca text simplu și poate dintr-o versiune mai veche: verificați-le.
import-scrivener-report-content-salvaged-synopsis = Sinopsisul elementului „{ $item }” lipsea din proiect. Cuvintele lui provin din indexul de căutare al lui Scrivener, ca text simplu și poate dintr-o versiune mai veche: verificați-le.
# A finding of the document importer found in several places: its sentence about the
# first, then how many more the list under it names.
import-scrivener-report-document-others = { $others ->
    [one] { $sentence } Același lucru se întâmplă încă într-un loc, enumerat mai jos.
    [few] { $sentence } Același lucru se întâmplă în încă { $others } locuri, enumerate mai jos.
   *[other] { $sentence } Același lucru se întâmplă în încă { $others } de locuri, enumerate mai jos.
}
# An item of the report, by its path in Scrivener's binder, with which of its writings
# the line is about when it is not the text.
import-scrivener-report-item-notes = { $path } (notele documentului)
import-scrivener-report-item-synopsis = { $path } (sinopsis)
import-scrivener-report-item-project-note = { $path } (notă de proiect)
import-scrivener-report-item-recovered = { $path } (text recuperat)
# An item with no name in Scrivener.
import-scrivener-report-untitled = Fără titlu
import-scrivener-report-no-findings = Importul nu mai are nimic de raportat.
import-scrivener-report-group-unread = Nu au putut fi citite
import-scrivener-report-group-check = De verificat
import-scrivener-report-group-changed = Preluate altfel
import-scrivener-report-group-left = Nepreluate
import-scrivener-report-group-count = { $group } ({ $count })
import-scrivener-report-items-toggle = { $count ->
    [one] Un element
    [few] { $count } elemente
   *[other] { $count } de elemente
}
# The same, as a screen reader names it: with the line whose items it lists.
import-scrivener-report-items-toggle-named = { $count ->
    [one] Un element: { $line }
    [few] { $count } elemente: { $line }
   *[other] { $count } de elemente: { $line }
}
import-scrivener-report-names-toggle = { $count ->
    [one] Un nume
    [few] { $count } nume
   *[other] { $count } de nume
}
# The same, as a screen reader hears it, naming the line it belongs to.
import-scrivener-report-names-toggle-named = { $count ->
    [one] Un nume: { $line }
    [few] { $count } nume: { $line }
   *[other] { $count } de nume: { $line }
}
import-scrivener-report-more = { $count ->
    [one] Și încă unul.
    [few] Și încă { $count }.
   *[other] Și încă { $count }.
}
import-scrivener-report-open-item = Deschide în editor
import-scrivener-report-links-hint = Deschideți noul proiect, apoi afișați din nou acest raport din paleta de comenzi: atunci fiecare element enumerat aici se deschide cu un clic.
import-scrivener-copy-report = Copiază raportul
import-scrivener-report-copied = Raport copiat
import-scrivener-report-copy-failed = Raportul nu a putut fi copiat.
import-scrivener-report-close = Închide
import-scrivener-report-none-title = Niciun raport de import
import-scrivener-report-none = Niciun import Scrivener nu s-a încheiat de când a pornit Skribisto. Skribisto păstrează rapoartele celor mai recente importuri până la închidere.
import-scrivener-report-gone = Raportul acestui import nu mai este păstrat. Skribisto păstrează până la închidere { $count ->
    [one] raportul ultimului import.
    [few] rapoartele ultimelor { $count } importuri.
   *[other] rapoartele ultimelor { $count } de importuri.
}

## Words the import writes into the new project, in the language Skribisto shows itself in
import-scrivener-manuscript-binder = Manuscris
import-scrivener-notes-binder = Note
import-scrivener-notes-of = { $title } (note)
import-scrivener-project-notes = Notele proiectului
import-scrivener-recovered = Texte recuperate
import-scrivener-untitled-template = Șablon fără titlu
# The folder beside the new project its research files are copied into.
import-scrivener-research-folder = { $name } (fișiere de documentare)
# Between two authors' names.
import-scrivener-list-separator = {", "}
import-scrivener-label-details = Din câmpul „{ $field }” din Scrivener
import-scrivener-status-details = Din câmpul „{ $field }” din Scrivener
import-scrivener-unnamed-label = Label { $number }
import-scrivener-unnamed-status = Stare { $number }
import-scrivener-label-tag = { $name } (Label)
import-scrivener-untitled-tag = Fără titlu
# The tags, the pace plan and the "From Scrivener" folder the import makes from what a
# Scrivener project holds beyond its labels and status.
import-scrivener-keyword-details = Cuvântul-cheie „{ $path }” din Scrivener
import-scrivener-unnamed-keyword = Cuvânt-cheie { $number }
import-scrivener-keyword-tag = { $name } (cuvânt-cheie)
import-scrivener-collection-details = O colecție din Scrivener
import-scrivener-collection-tag = { $name } (colecție)
import-scrivener-field-details = Din câmpul „{ $field }” din Scrivener
# A tag made from one option of a list field.
import-scrivener-field-value = { $field }: { $option }
import-scrivener-field-tag = { $name } (câmp)
# The day off a pace plan takes on its deadline.
import-scrivener-deadline-day = Termen-limită
import-scrivener-from-scrivener = Din Scrivener
import-scrivener-note-project = Proiect
import-scrivener-note-project-bookmarks = Semnele de carte ale proiectului
import-scrivener-note-replacements = Înlocuiri la compilare
import-scrivener-note-auto-complete = Intrări de completare automată
import-scrivener-note-saved-searches = Căutări salvate
import-scrivener-note-custom-metadata = Metadate personalizate
import-scrivener-note-targets = Ținte
import-scrivener-note-bookmarks = Semne de carte
import-scrivener-abbreviated-title = Titlu prescurtat
import-scrivener-subjects = Subiecte
import-scrivener-company = Companie
import-scrivener-session-target = Ținta sesiunii
import-scrivener-session-target-calculated = Ținta sesiunii, calculată de Scrivener pe baza termenului-limită
import-scrivener-writing-days = Zile de scris
import-scrivener-weekday-monday = Luni
import-scrivener-weekday-tuesday = Marți
import-scrivener-weekday-wednesday = Miercuri
import-scrivener-weekday-thursday = Joi
import-scrivener-weekday-friday = Vineri
import-scrivener-weekday-saturday = Sâmbătă
import-scrivener-weekday-sunday = Duminică
import-scrivener-draft-target = Ținta manuscrisului
import-scrivener-deadline = Termen-limită
# A unit after a count, in parentheses: "1000 (words)".
import-scrivener-unit-words = cuvinte
import-scrivener-unit-characters = caractere
import-scrivener-unit-pages = pagini
import-scrivener-search-query = Termeni căutați
import-scrivener-search-in = Căutat în
import-scrivener-search-operator = Potrivire
# What a saved search looks in, by the kind Scrivener names.
import-scrivener-search-in-all = tot
import-scrivener-search-in-title = titluri
import-scrivener-search-in-text = text
import-scrivener-search-in-notes = note
import-scrivener-search-in-synopsis = sinopsisuri
import-scrivener-search-in-keywords = cuvinte-cheie
import-scrivener-search-in-label = Label
import-scrivener-search-in-status = stări
import-scrivener-search-in-section-type = tipuri de secțiune
import-scrivener-search-in-custom-metadata = metadate personalizate
# How a saved search's words combine.
import-scrivener-search-any = oricare dintre cuvinte
import-scrivener-search-all = toate cuvintele
import-scrivener-search-exact = expresia exactă
# One line of a listing: a bookmark's place and its note, or a link's words and its address.
import-scrivener-label-value = { $label }: { $value }
import-scrivener-replacement-regex = expresie regulată
import-scrivener-replacement-off = dezactivată
# Said of a note whose own text lists its custom metadata or bookmarks.
import-scrivener-in-own-note = Enumerate în nota în care s-a transformat acest document.
# A refusal on one line: its title, then what to do.
import-scrivener-paragraph = { $title }. { $body }

## When the project cannot be imported
import-scrivener-refusal-missing-title = Nu există nimic la această cale
import-scrivener-refusal-missing-body = „{ $file }” nu mai este acolo. Alegeți din nou proiectul.
import-scrivener-refusal-unreadable-title = Acest proiect nu poate fi citit
import-scrivener-refusal-unreadable-body = Skribisto nu a putut citi „{ $file }” ({ $reason }). Verificați că îl puteți deschide, apoi încercați din nou.
import-scrivener-refusal-not-a-scrivener-project-title = Aici nu este niciun proiect Scrivener
import-scrivener-refusal-not-a-scrivener-project-body = Alegeți dosarul .scriv al proiectului, fișierul .scrivx din interiorul lui, o copie de siguranță .zip sau un .scrivtemplate.
import-scrivener-refusal-manifest-without-files-title = Documentele nu se află în această copie
import-scrivener-refusal-manifest-without-files-body = „{ $manifest }” este aici, dar dosarul Files, care conține documentele enumerate de el, nu este. Poate că un serviciu de sincronizare nu a terminat de copiat proiectul sau a fost copiat doar fișierul .scrivx. Dacă ați ales fișierul .scrivx, alegeți în schimb dosarul proiectului, cu „Alege un dosar…”.
import-scrivener-refusal-search-too-wide-title = În acest dosar sunt prea multe de căutat
import-scrivener-refusal-search-too-wide-body = Numărul dosarelor de parcurs depășește { $limit }. Alegeți chiar proiectul Scrivener.
import-scrivener-refusal-scrivener-one-for-mac-title = Acest proiect a fost creat cu Scrivener 1 pentru Mac
import-scrivener-refusal-scrivener-one-for-mac-body = Skribisto citește proiectele create cu Scrivener 2 și cu versiunile ulterioare. Deschideți-l o dată în Scrivener 2 sau 3 pentru a-l actualiza ori compilați-l într-un document Word și importați acel document.
import-scrivener-refusal-ios-stub-without-mobile-data-title = Acesta este doar un fișier provizoriu de la Scrivener pentru iPhone sau iPad
import-scrivener-refusal-ios-stub-without-mobile-data-body = Proiectul propriu-zis nu este aici. Sincronizați-l de pe iPhone sau iPad ori partajați-l de acolo ca .zip și importați acel fișier.
import-scrivener-refusal-newer-title = Acest proiect a fost creat cu un Scrivener mai nou
import-scrivener-refusal-unknown-format-version-body = Formatul lui, { $version }, nu este cunoscut de Skribisto. Verificați dacă există o actualizare Skribisto. Până atunci, compilați-l în Scrivener într-un document Word și importați acel document.
import-scrivener-refusal-newer-format-stamp-body = Fișierele lui sunt marcate cu versiunea { $stamp }, iar Skribisto citește până la versiunea { $newest }. Verificați dacă există o actualizare Skribisto. Până atunci, compilați-l în Scrivener într-un document Word și importați acel document.
import-scrivener-refusal-version-stamp-unreadable-title = Versiunea proiectului nu poate fi citită
import-scrivener-refusal-version-stamp-unreadable-body = Fișierul său Files/version.txt nu conține niciun număr de versiune, așa că Skribisto nu poate ști dacă poate citi acest proiect. Deschideți o dată proiectul în Scrivener, apoi importați-l din nou.
import-scrivener-refusal-damaged-manifest-title = Fișierul clasorului nu poate fi citit
import-scrivener-refusal-damaged-manifest-body = „{ $manifest }” este deteriorat. { $spare_copies ->
    [0] Deschideți proiectul în Scrivener, care îl poate repara, apoi importați-l din nou.
    [1] Scrivener păstrează o copie de rezervă a lui în interiorul proiectului, pe care Skribisto nu o citește. Deschideți proiectul în Scrivener, care îl poate repara, apoi importați-l din nou.
    [few] Scrivener păstrează { $spare_copies } copii de rezervă ale lui în interiorul proiectului, pe care Skribisto nu le citește. Deschideți proiectul în Scrivener, care îl poate repara, apoi importați-l din nou.
   *[other] Scrivener păstrează { $spare_copies } de copii de rezervă ale lui în interiorul proiectului, pe care Skribisto nu le citește. Deschideți proiectul în Scrivener, care îl poate repara, apoi importați-l din nou.
}
# The empty file Scrivener writes for a new project before its first save: not damage.
import-scrivener-refusal-damaged-manifest-new-project-placeholder-body = „{ $manifest }” este fișierul gol pe care Scrivener îl scrie pentru un proiect pe care nu l-a salvat încă. Deschideți proiectul în Scrivener și lăsați-l să îl salveze, apoi importați-l din nou.
import-scrivener-refusal-manifest-too-large-title = Acest fișier de clasor este prea mare
import-scrivener-refusal-manifest-too-large-body = „{ $manifest }” depășește cei { $limit } MB pe care îi citește Skribisto. Niciun proiect real nu se apropie de așa ceva, așa că fișierul poate fi deteriorat.
import-scrivener-refusal-no-item-identifiers-title = Clasorul nu enumeră niciun document pe care Skribisto să îl poată găsi
import-scrivener-refusal-no-item-identifiers-body = Niciunul dintre elementele lui nu are un identificator, așa că fișierele lor nu pot fi găsite. Deschideți o dată proiectul în Scrivener, apoi importați-l din nou.
import-scrivener-refusal-incomplete-copy-title = Documentele lipsesc din această copie
import-scrivener-refusal-incomplete-copy-body = Dosarul Files este aici, dar documentele din el nu sunt. Poate că un serviciu de sincronizare nu a terminat de copiat proiectul. Așteptați să termine sau copiați din nou întregul dosar .scriv, apoi recitiți proiectul.
import-scrivener-refusal-archive-title = Această arhivă nu poate fi deschisă
import-scrivener-refusal-archive-unreadable-body = „{ $file }” nu a putut fi citit ({ $reason }). Poate fi deteriorat. Dezarhivați manual arhiva, apoi alegeți dosarul .scriv din interior.
import-scrivener-refusal-archive-unsafe-name-title = Această arhivă nu poate fi deschisă în siguranță
# A name with `..` in it, or an absolute one.
import-scrivener-refusal-archive-unsafe-name-body = „{ $file }” conține un fișier numit „{ $member }”, care ar ajunge în afara arhivei, așa că Skribisto nu deschide arhiva.
import-scrivener-refusal-archive-unsafe-name-empty-body = „{ $file }” conține un fișier fără nume, așa că Skribisto nu deschide arhiva.
import-scrivener-refusal-archive-unsafe-name-control-character-body = „{ $file }” conține un fișier al cărui nume include un caracter de control, așa că Skribisto nu deschide arhiva.
import-scrivener-refusal-archive-too-large-title = Această arhivă este prea mare pentru a fi deschisă în siguranță
import-scrivener-refusal-archive-too-many-members-body = Numărul fișierelor din ea depășește { $limit }. Dezarhivați manual arhiva, apoi alegeți dosarul .scriv din interior.
import-scrivener-refusal-archive-member-too-large-body = „{ $member }” se dezarhivează la peste { $limit } MB. Dezarhivați manual arhiva, apoi alegeți dosarul .scriv din interior.
import-scrivener-refusal-archive-too-large-body = Se dezarhivează la peste { $limit } MB. Dezarhivați manual arhiva, apoi alegeți dosarul .scriv din interior.
import-scrivener-refusal-archive-encrypted-title = Această arhivă este protejată cu parolă
import-scrivener-refusal-archive-encrypted-body = Dezarhivați manual „{ $file }”, apoi alegeți dosarul .scriv din interior.
import-scrivener-refusal-archive-unsupported-compression-title = Această arhivă este comprimată într-un mod pe care Skribisto nu îl poate citi
import-scrivener-refusal-archive-unsupported-compression-body = „{ $member }” este comprimat cu { $method }. Dezarhivați manual arhiva, apoi alegeți dosarul .scriv din interior.
import-scrivener-refusal-duplicate-archive-members-title = Această arhivă enumeră unele fișiere de două ori
import-scrivener-refusal-duplicate-archive-members-body = „{ $file }” conține mai multe fișiere cu același nume, așa că Skribisto nu poate ști care dintre ele aparține proiectului. Dezarhivați manual arhiva, apoi alegeți dosarul .scriv din interior.
# What a file cut short looks like; a file too large, one that is not a template and one
# encoded another way have their own words.
import-scrivener-refusal-template-undecodable-title = Acest fișier de șablon este incomplet
import-scrivener-refusal-template-undecodable-too-large-title = Acest fișier de șablon este prea mare pentru a fi deschis
import-scrivener-refusal-template-undecodable-not-a-template-title = Acest fișier nu este un șablon Scrivener
import-scrivener-refusal-template-undecodable-unknown-encoding-title = Acest fișier de șablon este stocat într-o formă pe care Skribisto nu o cunoaște
import-scrivener-refusal-template-undecodable-body = „{ $file }” nu poate fi decodat: este posibil să se termine înainte de sfârșitul proiectului din el. Descărcați-l din nou sau deschideți-l în Scrivener și creați un proiect din el.
import-scrivener-refusal-template-undecodable-too-large-body = „{ $file }” depășește { $limit } MB. Deschideți-l în Scrivener, creați un proiect din el, apoi importați acel proiect.
import-scrivener-refusal-template-undecodable-not-a-template-body = „{ $file }” nu conține un proiect Scrivener. Alegeți fișierul de șablon salvat de Scrivener sau un proiect creat din el.
import-scrivener-refusal-template-undecodable-unknown-encoding-body = „{ $file }” își stochează proiectul într-un mod pe care Skribisto nu îl poate decoda. Deschideți-l în Scrivener, creați un proiect din el, apoi importați acel proiect.
import-scrivener-refusal-cloud-placeholders-title = { $count ->
    [one] Un fișier al acestui proiect este încă în cloud
    [few] { $count } fișiere ale acestui proiect sunt încă în cloud
   *[other] { $count } de fișiere ale acestui proiect sunt încă în cloud
}
import-scrivener-refusal-cloud-placeholders-body = { $count ->
    [1] Nu a fost descărcat pe acest calculator: { $examples }. Faceți proiectul disponibil offline în aplicația de sincronizare, apoi recitiți proiectul.
   *[other] Nu au fost descărcate pe acest calculator, de exemplu { $examples }. Faceți proiectul disponibil offline în aplicația de sincronizare, apoi recitiți proiectul.
}
import-scrivener-refusal-unreadable-members-title = { $count ->
    [one] Un fișier al acestui proiect nu poate fi citit
    [few] { $count } fișiere ale acestui proiect nu pot fi citite
   *[other] { $count } de fișiere ale acestui proiect nu pot fi citite
}
import-scrivener-refusal-unreadable-members-body = De exemplu { $examples }. Verificați că aveți dreptul să citiți întregul proiect și că discul este în stare bună, apoi recitiți proiectul.
import-scrivener-refusal-needed-member-is-link-title = Acest proiect trimite în afara propriului dosar
import-scrivener-refusal-needed-member-is-link-body = „{ $member }” este o legătură către alt loc de pe acest calculator, iar Skribisto nu urmează legăturile. Copiați proiectul astfel încât fiecare fișier să fie în interiorul lui, apoi încercați din nou.
import-scrivener-refusal-project-changed-during-read-title = Proiectul s-a modificat în timp ce era citit
import-scrivener-refusal-project-changed-during-read-body = Scrivener sau un serviciu de sincronizare încă scrie în el. Închideți-l în Scrivener, așteptați să se termine sincronizarea, apoi recitiți proiectul.
import-scrivener-refusal-too-many-members-title = Acest proiect conține prea multe fișiere
import-scrivener-refusal-too-many-members-body = Numărul fișierelor lui depășește { $limit }, mai mult decât are orice proiect real. Poate că nu este un proiect Scrivener sau poate că este deteriorat.
import-scrivener-refusal-too-large-title = Acest proiect este prea mare
import-scrivener-refusal-too-large-body = Textele lui depășesc { $limit } MB, mai mult decât poate importa Skribisto în siguranță.
import-scrivener-refusal-structure-unmappable-title = Skribisto nu poate transpune acest clasor
import-scrivener-refusal-structure-unmappable-body = Clasorul nu a putut fi preluat fără să se piardă sau să se plaseze greșit ceva. Este o eroare în Skribisto, nu în proiectul dumneavoastră. Vă rugăm să o raportați, împreună cu ce arată „Detalii”.

## When there is a choice to make
import-scrivener-roots-title = { $count ->
    [one] Aici este un proiect. Îl importați?
    [few] Aici sunt { $count } proiecte. Pe care îl importați?
   *[other] Aici sunt { $count } de proiecte. Pe care îl importați?
}
import-scrivener-roots-hint = Acest dosar sau această arhivă conține mai multe proiecte Scrivener. Alegeți-l pe cel de importat.
import-scrivener-manifests-title = { $count ->
    [one] Acest proiect are un fișier de clasor
    [few] Acest proiect are { $count } fișiere de clasor
   *[other] Acest proiect are { $count } de fișiere de clasor
}
import-scrivener-manifests-hint = Scrivener păstrează clasorul, lista tuturor documentelor și dosarelor, într-un fișier .scrivx. Cele în plus au fost cel mai probabil lăsate de un serviciu de sincronizare. Alegeți-l pe cel în care ați lucrat ultima dată.
import-scrivener-manifests-hint-preselected = Scrivener păstrează clasorul, lista tuturor documentelor și dosarelor, într-un fișier .scrivx. Cele în plus au fost cel mai probabil lăsate de un serviciu de sincronizare. Cel selectat este cel în care ați lucrat cel mai probabil ultima dată: verificați-l înainte de a continua.
import-scrivener-manifest-row = { $file } · salvat la { $date } · { $documents ->
    [one] un element
    [few] { $documents } elemente
   *[other] { $documents } de elemente
}
import-scrivener-manifest-row-undated = { $file } · { $documents ->
    [one] un element
    [few] { $documents } elemente
   *[other] { $documents } de elemente
}
import-scrivener-manifest-row-unread = { $file } · nu poate fi citit
import-scrivener-enclosing-title = Aceasta face parte din proiectul Scrivener „{ $project }”
import-scrivener-enclosing-body = Ați ales un fișier sau un dosar din interiorul unui proiect. Skribisto importă întregul proiect.
import-scrivener-enclosing-use = Importă „{ $project }”
import-scrivener-upgrade-title = Aceasta este copia păstrată de Scrivener înainte de o actualizare
import-scrivener-upgrade-body = Când Scrivener actualizează un proiect la o versiune mai nouă, îl păstrează pe cel vechi sub numele „{ $backup }”. Proiectul în care ați continuat să scrieți este „{ $current }”, alături.
import-scrivener-upgrade-body-unclear = Când Scrivener actualizează un proiect la o versiune mai nouă, îl păstrează pe cel vechi sub numele „{ $backup }”. Proiectul de alături, „{ $current }”, nu a putut fi citit corect, așa că alegeți pe care îl importați.
import-scrivener-upgrade-use-current = Importă „{ $current }”, proiectul în care ați continuat să scrieți
import-scrivener-upgrade-keep-backup = Importă această copie mai veche, „{ $backup }”

## What must be accepted before going on
import-scrivener-ios-pending-title = Modificările făcute în Scrivener pentru iPhone sau iPad nu sunt încă în această copie
import-scrivener-ios-pending-body = Scrivener de pe calculator preia aceste modificări data viitoare când deschide proiectul. Deschideți o dată proiectul în Scrivener pe calculator, apoi importați-l din nou.
import-scrivener-ios-pending-accept = Importă-l fără aceste modificări
import-scrivener-no-text-title = { $count ->
    [one] Un document și niciun cuvânt de text în această copie
    [few] { $count } documente și niciun cuvânt de text în această copie
   *[other] { $count } de documente și niciun cuvânt de text în această copie
}
import-scrivener-no-text-body = Poate că un serviciu de sincronizare încă copiază proiectul sau copia și-a pierdut documentele. Dacă îl importați acum, fiecare document ajunge gol.
import-scrivener-no-text-accept = Importă documentele goale

## Things to know, nothing to decide
import-scrivener-notice-project-found-below-pick-title = Proiectul a fost găsit în dosarul ales
import-scrivener-notice-project-found-below-pick-body = Se află la „{ $path }”.
import-scrivener-notice-source-is-template-title = Acesta este un șablon Scrivener
import-scrivener-notice-source-is-template-named = Acesta este șablonul Scrivener „{ $title }”
import-scrivener-notice-source-is-template-body = Importul lui creează un proiect nou din ce conține șablonul, inclusiv instrucțiunile lui.
import-scrivener-notice-source-is-backup-title = Aceasta pare o copie de siguranță făcută de Scrivener
import-scrivener-notice-source-is-backup-body = Conține proiectul așa cum era în momentul copiei. Dacă ați continuat să scrieți după aceea, alegeți chiar proiectul.
import-scrivener-notice-importing-pre-upgrade-copy-title = Importați copia păstrată de Scrivener înainte de o actualizare
import-scrivener-notice-importing-pre-upgrade-copy-body = Proiectul în care ați continuat să scrieți se află alături și conține tot ce s-a scris de atunci.
import-scrivener-notice-imported-from-ios-stub-title = Acest proiect a fost creat în Scrivener pentru iPhone sau iPad
import-scrivener-notice-imported-from-ios-stub-body = Nu a fost deschis niciodată pe un calculator, așa că este citit din copia păstrată de iPhone sau iPad.
import-scrivener-notice-stale-mobile-binder-ignored-title = Un clasor vechi de pe un iPhone sau iPad este lăsat deoparte
import-scrivener-notice-stale-mobile-binder-ignored-body = Scrivener de pe calculator i-a preluat deja modificările.
import-scrivener-notice-identical-manifest-copies-title = { $count ->
    [one] Acest proiect are un fișier de clasor identic
    [few] Acest proiect are { $count } fișiere de clasor identice
   *[other] Acest proiect are { $count } de fișiere de clasor identice
}
import-scrivener-notice-identical-manifest-copies-body = Conțin același lucru, așa că oricare dintre ele dă același proiect.
import-scrivener-notice-manifest-copies-ignored-title = Copiile în plus ale fișierului clasorului sunt lăsate deoparte
import-scrivener-notice-manifest-copies-ignored-body = „{ $chosen }” este cel în care a lucrat Scrivener ultima dată. { $count ->
    [one] O copie rămasă de la un serviciu de sincronizare este lăsată deoparte.
    [few] { $count } copii rămase de la un serviciu de sincronizare sunt lăsate deoparte.
   *[other] { $count } de copii rămase de la un serviciu de sincronizare sunt lăsate deoparte.
}
import-scrivener-notice-pinned-manifest-looks-like-a-copy-title = Fișierul de clasor ales pare o copie de sincronizare
import-scrivener-notice-pinned-manifest-looks-like-a-copy-body = „{ $name }” are numele unei copii a lui „{ $original }”, care se află alături. Dacă importul pare vechi, alegeți în schimb „{ $original }”.
import-scrivener-notice-pinned-manifest-has-siblings-title = { $count ->
    [one] Lângă cel ales se află încă un fișier de clasor
    [few] Lângă cel ales se află alte { $count } fișiere de clasor
   *[other] Lângă cel ales se află alte { $count } de fișiere de clasor
}
import-scrivener-notice-pinned-manifest-has-siblings-body = Este citit doar cel ales de dumneavoastră.
import-scrivener-notice-legacy-v1-files-beside-title = Fișierele unui Scrivener mai vechi sunt lăsate deoparte
import-scrivener-notice-legacy-v1-files-beside-body = Au rămas în urmă când proiectul a fost actualizat. Sunt citite doar fișierele versiunii actuale.
import-scrivener-notice-mixed-payload-trees-title = În acest proiect sunt fișiere din două versiuni Scrivener
import-scrivener-notice-mixed-payload-trees-body = Documentele sunt citite de acolo de unde indică fișierul clasorului.
import-scrivener-notice-version-disagreement-title = Fișierele acestui proiect nu indică toate aceeași versiune
import-scrivener-notice-id-scheme-mismatch-body = Fișierul clasorului indică versiunea { $version }, dar își numește documentele ca o altă versiune. Ele sunt citite de unde indică el. Dacă importul pare greșit, deschideți o dată proiectul în Scrivener și importați-l din nou.
import-scrivener-notice-version-stamp-mismatch-body = Este citit așa cum îl descrie fișierul clasorului. Dacă importul pare greșit, deschideți o dată proiectul în Scrivener și importați-l din nou.
import-scrivener-notice-unrecognised-writer-title = Acest proiect a fost salvat de un program pe care Skribisto nu îl recunoaște
import-scrivener-notice-unrecognised-writer-named = Se prezintă drept „{ $creator }”. Este citit ca proiect Scrivener.
import-scrivener-notice-unrecognised-writer-body = Nu își indică numele. Este citit ca proiect Scrivener.
import-scrivener-notice-invalid-item-identifier-title = { $count ->
    [one] Un element din clasor are un identificator nevalid
    [few] { $count } elemente din clasor au un identificator nevalid
   *[other] { $count } de elemente din clasor au un identificator nevalid
}
import-scrivener-notice-invalid-item-identifier-body = Un astfel de identificator este lăsat deoparte, iar elementul care îl poartă este păstrat.
import-scrivener-notice-items-without-identifier-title = { $count ->
    [one] Un element din clasor nu are identificator
    [few] { $count } elemente din clasor nu au identificator
   *[other] { $count } de elemente din clasor nu au identificator
}
import-scrivener-notice-items-without-identifier-body = Fără identificator, fișierele unui element nu pot fi găsite, așa că elementul ajunge gol.
import-scrivener-notice-duplicate-item-key-title = Două elemente din clasor au același identificator: „{ $item }”
import-scrivener-notice-duplicate-item-key-body = Primul păstrează documentele. Al doilea își păstrează locul și ajunge gol.
import-scrivener-notice-lock-here-title = Scrivener are acest proiect deschis pe acest calculator
import-scrivener-notice-lock-here-body = Închideți-l mai întâi în Scrivener, ca fiecare modificare să fie salvată.
import-scrivener-notice-lock-title = Scrivener avea acest proiect deschis pe „{ $host }”
import-scrivener-notice-lock-elsewhere-title = Scrivener avea acest proiect deschis pe alt calculator
import-scrivener-notice-lock-body = Dacă este încă deschis acolo, închideți-l mai întâi: este posibil ca fișierele să nu conțină încă cele mai recente modificări. Dacă Scrivener a fost închis de mult, îl puteți importa așa cum este.
import-scrivener-notice-archived-while-open-title = Această arhivă a fost creată în timp ce Scrivener avea proiectul deschis
import-scrivener-notice-archived-while-open-body = Este posibil ca modificările făcute după acel moment să lipsească din ea.
import-scrivener-notice-content-missing-notes = Notele elementului „{ $item }” lipsesc
import-scrivener-notice-content-missing-synopsis = Sinopsisul elementului „{ $item }” lipsește
import-scrivener-notice-content-missing-text = Textul elementului „{ $item }” lipsește
import-scrivener-notice-content-missing-body = { $place ->
    [notes] Fișierul corespunzător nu se află în această copie, așa că notele ajung goale. Poate că un serviciu de sincronizare încă copiază proiectul.
    [synopsis] Fișierul corespunzător nu se află în această copie, așa că sinopsisul ajunge gol. Poate că un serviciu de sincronizare încă copiază proiectul.
   *[text] Fișierul corespunzător nu se află în această copie, așa că textul ajunge gol. Poate că un serviciu de sincronizare încă copiază proiectul.
}
import-scrivener-notice-content-missing-salvaged = { $place ->
    [notes] Fișierul corespunzător nu se află în această copie, dar indexul de căutare al lui Scrivener le mai păstra cuvintele. Acestea sunt păstrate ca text simplu, poate dintr-o versiune mai veche.
    [synopsis] Fișierul corespunzător nu se află în această copie, dar indexul de căutare al lui Scrivener îi mai păstra cuvintele. Acestea sunt păstrate ca text simplu, poate dintr-o versiune mai veche.
   *[text] Fișierul corespunzător nu se află în această copie, dar indexul de căutare al lui Scrivener îi mai păstra cuvintele. Acestea sunt păstrate ca text simplu, poate dintr-o versiune mai veche.
}
import-scrivener-notice-content-truncated-notes = Notele elementului „{ $item }” sunt goale pe disc
import-scrivener-notice-content-truncated-synopsis = Sinopsisul elementului „{ $item }” este gol pe disc
import-scrivener-notice-content-truncated-text = Textul elementului „{ $item }” este gol pe disc
import-scrivener-notice-content-truncated-body = { $place ->
    [notes] Scrivener le-a înregistrat ca având text, dar fișierul lor nu conține nimic: poate că o sincronizare s-a oprit la jumătate. Notele ajung goale.
    [synopsis] Scrivener l-a înregistrat ca având text, dar fișierul lui nu conține nimic: poate că o sincronizare s-a oprit la jumătate. Sinopsisul ajunge gol.
   *[text] Scrivener l-a înregistrat ca având text, dar fișierul lui nu conține nimic: poate că o sincronizare s-a oprit la jumătate. Textul ajunge gol.
}
import-scrivener-notice-missing-media-title = Fișierul elementului „{ $item }” lipsește
import-scrivener-notice-missing-media-body = Imaginea sau fișierul de documentare pe care îl conține nu se află în această copie.
import-scrivener-notice-sync-conflict-copies-title = { $count ->
    [one] A fost găsită o copie de sincronizare a unui document
    [few] Au fost găsite { $count } copii de sincronizare ale unor documente
   *[other] Au fost găsite { $count } de copii de sincronizare ale unor documente
}
import-scrivener-notice-sync-conflict-copies-body = { $missing ->
    [0] Copiile de sincronizare sunt preluate ca texte recuperate, într-un dosar separat, ca să le puteți compara cu documentele.
    [one] Copiile de sincronizare sunt preluate ca texte recuperate, într-un dosar separat. Una dintre ele este copia unui document care nu mai este în proiect.
   *[other] Copiile de sincronizare sunt preluate ca texte recuperate, într-un dosar separat. { $missing } dintre ele sunt copii ale unor documente care nu mai sunt în proiect.
}
import-scrivener-notice-recovered-unreferenced-text-title = { $count ->
    [one] A fost găsit un document pe care nu îl enumeră niciun element din clasor
    [few] Au fost găsite { $count } documente pe care nu le enumeră niciun element din clasor
   *[other] Au fost găsite { $count } de documente pe care nu le enumeră niciun element din clasor
}
import-scrivener-notice-recovered-unreferenced-text-body = Documentele de acest fel sunt preluate ca texte recuperate, într-un dosar separat.
import-scrivener-notice-sync-debris-ignored-title = { $count ->
    [one] O copie în plus, rămasă de la un serviciu de sincronizare, este lăsată deoparte
    [few] { $count } copii în plus, rămase de la un serviciu de sincronizare, sunt lăsate deoparte
   *[other] { $count } de copii în plus, rămase de la un serviciu de sincronizare, sunt lăsate deoparte
}
import-scrivener-notice-sync-debris-ignored-body = Nici Scrivener nu citește astfel de copii. Sunt citite doar fișierele proprii ale proiectului.
import-scrivener-notice-unrecognised-files-title = { $count ->
    [one] Un fișier care nu face parte dintr-un proiect Scrivener este lăsat deoparte
    [few] { $count } fișiere care nu fac parte dintr-un proiect Scrivener sunt lăsate deoparte
   *[other] { $count } de fișiere care nu fac parte dintr-un proiect Scrivener sunt lăsate deoparte
}
import-scrivener-notice-examples = De exemplu { $examples }.
import-scrivener-notice-symlink-not-followed-title = { $count ->
    [one] O legătură din interiorul proiectului nu este urmată
    [few] { $count } legături din interiorul proiectului nu sunt urmate
   *[other] { $count } de legături din interiorul proiectului nu sunt urmate
}
import-scrivener-notice-unreadable-files-skipped-title = { $count ->
    [one] Un fișier de care importul nu are nevoie nu a putut fi citit
    [few] { $count } fișiere de care importul nu are nevoie nu au putut fi citite
   *[other] { $count } de fișiere de care importul nu are nevoie nu au putut fi citite
}
import-scrivener-notice-cloud-placeholders-skipped-title = { $count ->
    [one] Un fișier aflat încă în cloud este omis
    [few] { $count } fișiere aflate încă în cloud sunt omise
   *[other] { $count } de fișiere aflate încă în cloud sunt omise
}
import-scrivener-notice-cloud-placeholders-skipped-body = Nimic din ce îi trebuie importului nu mai este în cloud.
import-scrivener-notice-no-text-found-title = { $count ->
    [1] Un singur document, fără niciun text
    [few] { $count } documente, niciunul cu text
   *[other] { $count } de documente, niciunul cu text
}
import-scrivener-notice-no-text-found-body = Acest proiect pare gol. Este importat așa cum este.
import-scrivener-notice-metadata-unreadable-title = „{ $member }” nu a putut fi citit
import-scrivener-notice-metadata-unreadable-body = Ce a păstrat Scrivener în el nu este preluat.
import-scrivener-notice-dangling-keyword-title = { $count ->
    [one] O trimitere indică un cuvânt-cheie pe care proiectul nu îl definește
    [few] { $count } trimiteri indică cuvinte-cheie pe care proiectul nu le definește
   *[other] { $count } de trimiteri indică cuvinte-cheie pe care proiectul nu le definește
}
import-scrivener-notice-dangling-label-title = { $count ->
    [one] O trimitere indică o valoare Label pe care proiectul nu o definește
    [few] { $count } trimiteri indică valori Label pe care proiectul nu le definește
   *[other] { $count } de trimiteri indică valori Label pe care proiectul nu le definește
}
import-scrivener-notice-dangling-status-title = { $count ->
    [one] O trimitere indică o stare pe care proiectul nu o definește
    [few] { $count } trimiteri indică stări pe care proiectul nu le definește
   *[other] { $count } de trimiteri indică stări pe care proiectul nu le definește
}
import-scrivener-notice-dangling-section-type-title = { $count ->
    [one] O trimitere indică un tip de secțiune pe care proiectul nu îl definește
    [few] { $count } trimiteri indică tipuri de secțiune pe care proiectul nu le definește
   *[other] { $count } de trimiteri indică tipuri de secțiune pe care proiectul nu le definește
}
import-scrivener-notice-dangling-custom-field-title = { $count ->
    [one] O trimitere indică un câmp de metadate personalizate pe care proiectul nu îl definește
    [few] { $count } trimiteri indică câmpuri de metadate personalizate pe care proiectul nu le definește
   *[other] { $count } de trimiteri indică câmpuri de metadate personalizate pe care proiectul nu le definește
}
import-scrivener-notice-dangling-item-title = { $count ->
    [one] O trimitere indică un document pe care proiectul nu îl definește
    [few] { $count } trimiteri indică documente pe care proiectul nu le definește
   *[other] { $count } de trimiteri indică documente pe care proiectul nu le definește
}
import-scrivener-notice-dangling-references-body = O trimitere la ceva ce proiectul nu definește este omisă.
import-scrivener-notice-snapshots-not-imported-line = { $count ->
    [one] Un instantaneu (snapshot) rămâne în proiectul dumneavoastră Scrivener: Skribisto nu preia instantaneele.
    [few] { $count } instantanee (snapshots) rămân în proiectul dumneavoastră Scrivener: Skribisto nu preia instantaneele.
   *[other] { $count } de instantanee (snapshots) rămân în proiectul dumneavoastră Scrivener: Skribisto nu preia instantaneele.
}
import-scrivener-notice-writing-history-not-imported-line = { $days ->
    [one] Evidența ținută de Scrivener a cuvintelor scrise în fiecare zi, pentru o zi, nu este preluată. Skribisto își începe propria numărătoare de la prima salvare.
    [few] Evidența ținută de Scrivener a cuvintelor scrise în fiecare zi, pentru { $days } zile, nu este preluată. Skribisto își începe propria numărătoare de la prima salvare.
   *[other] Evidența ținută de Scrivener a cuvintelor scrise în fiecare zi, pentru { $days } de zile, nu este preluată. Skribisto își începe propria numărătoare de la prima salvare.
}
import-scrivener-notice-title-placeholder-rejected-hint = Titlul din setările proiectului este un text provizoriu dintr-un șablon, așa că este propus în schimb numele proiectului.
import-scrivener-notice-author-placeholder-rejected-hint = Numele autorului din setările proiectului este un text provizoriu dintr-un șablon, așa că nu este propus niciun nume.
import-scrivener-notice-title-from-project-hint-compile-settings = Acesta este titlul indicat în setările de compilare ale proiectului. Până când este stabilit acolo un titlu, Scrivener păstrează numele pe care îl avea proiectul la creare.
import-scrivener-notice-title-from-project-hint-template = Acesta este titlul propriu al șablonului.
import-scrivener-notice-title-from-project-hint-folder-name = Acesta este numele proiectului.
import-scrivener-notice-title-from-project-hint-project-properties = Acesta este titlul indicat în proprietățile proiectului.
import-scrivener-notice-author-from-project-hint = Acesta este numele indicat în setările proiectului. Acolo poate rămâne autorul unui șablon, iar numele lui ar apărea pe o pagină de titlu.
import-scrivener-notice-language-not-stated-hint = Proiectul nu își indică limba, așa că este propusă limba interfeței Skribisto. Limba decide cum sunt numărate cuvintele și cum sunt scrise ghilimelele.
import-scrivener-notice-language-from-script-hint = Propusă după sistemul de scriere al manuscrisului ({ $tag }).

## Before you import: what the import does with the binder, or leaves behind
import-scrivener-structure-script-mode-draft-line = { $script ->
    [1] { $documents ->
        [1] Singurul document al manuscrisului este scris în modul scenariu. Skribisto îi păstrează cuvintele, nu și aspectul de scenariu.
       *[other] Unul dintre documentele manuscrisului (din { $documents } în total) este scris în modul scenariu. Skribisto îi păstrează cuvintele, nu și aspectul de scenariu.
    }
   *[other] { $script } dintre documentele manuscrisului (din { $documents } în total) sunt scrise în modul scenariu. Skribisto le păstrează cuvintele, nu și aspectul de scenariu.
}
import-scrivener-structure-one-document-manuscript-line = { $count ->
    [1] Un document al manuscrisului își conține capitolele sub formă de titluri de secțiune.
    [few] { $count } documente ale manuscrisului își conțin capitolele sub formă de titluri de secțiune.
   *[other] { $count } de documente ale manuscrisului își conțin capitolele sub formă de titluri de secțiune.
}
import-scrivener-structure-title-repeats-number-line = { $count ->
    [one] Un titlu de capitol sau de parte își conține și numărul, de exemplu „Capitolul II” sau „Partea întâi”, așa că un export ar putea tipări numărul de două ori.
    [few] { $count } titluri de capitol sau de parte își conțin și numărul, de exemplu „Capitolul II” sau „Partea întâi”, așa că un export ar putea tipări numărul de două ori.
   *[other] { $count } de titluri de capitol sau de parte își conțin și numărul, de exemplu „Capitolul II” sau „Partea întâi”, așa că un export ar putea tipări numărul de două ori.
}
import-scrivener-structure-claim-overruled-line = { $count ->
    [one] Un element pare a fi o carte, o parte sau un capitol, dar se află într-un element care este deja la acel nivel, așa că este preluat la un nivel inferior.
    [few] { $count } elemente par a fi cărți, părți sau capitole, dar fiecare se află într-un element care este deja la acel nivel, așa că sunt preluate la un nivel inferior.
   *[other] { $count } de elemente par a fi cărți, părți sau capitole, dar fiecare se află într-un element care este deja la acel nivel, așa că sunt preluate la un nivel inferior.
}
import-scrivener-structure-adopted-by-chapter-line = { $count ->
    [one] Un document care urmează după un capitol devine sfârșitul acelui capitol.
    [few] { $count } documente care urmează după un capitol devin sfârșitul acelui capitol.
   *[other] { $count } de documente care urmează după un capitol devin sfârșitul acelui capitol.
}
import-scrivener-structure-looks-like-manuscript-line = { $count ->
    [one] Un dosar de pe primul nivel, păstrat la note, pare să fie manuscris. Îl puteți trimite în manuscris, ca o altă carte.
    [few] { $count } dosare de pe primul nivel, păstrate la note, par să fie manuscris. Le puteți trimite în manuscris, ca alte cărți.
   *[other] { $count } de dosare de pe primul nivel, păstrate la note, par să fie manuscris. Le puteți trimite în manuscris, ca alte cărți.
}
import-scrivener-structure-generic-sheet-title-line = { $count ->
    [one] O intrare din biblia poveștii are încă titlul unui șablon, de exemplu „Erou”. Skribisto caută acest titlu în text și enumeră pe pagina „În proză” fiecare loc în care apare: redenumiți intrarea dacă acest cuvânt este frecvent în cartea dumneavoastră.
    [few] { $count } intrări din biblia poveștii au încă titlul unui șablon, de exemplu „Erou”. Skribisto caută aceste titluri în text și enumeră pe pagina „În proză” fiecare loc în care apar: redenumiți intrările dacă aceste cuvinte sunt frecvente în cartea dumneavoastră.
   *[other] { $count } de intrări din biblia poveștii au încă titlul unui șablon, de exemplu „Erou”. Skribisto caută aceste titluri în text și enumeră pe pagina „În proză” fiecare loc în care apar: redenumiți intrările dacă aceste cuvinte sunt frecvente în cartea dumneavoastră.
}
import-scrivener-structure-placeholder-title-line = { $count ->
    [one] Un titlu care doar își numea nivelul este lăsat gol: clasorul și exporturile îl numerotează.
    [few] { $count } titluri care doar își numeau nivelul sunt lăsate goale: clasorul și exporturile le numerotează.
   *[other] { $count } de titluri care doar își numeau nivelul sunt lăsate goale: clasorul și exporturile le numerotează.
}
import-scrivener-structure-untitled-item-line = { $count ->
    [one] Un element nu avea titlu în Scrivener și rămâne fără titlu.
    [few] { $count } elemente nu aveau titlu în Scrivener și rămân fără titlu.
   *[other] { $count } de elemente nu aveau titlu în Scrivener și rămân fără titlu.
}
import-scrivener-structure-title-cleaned-line = { $count ->
    [one] Un titlu conținea salturi de rând sau tabulatoare, care sunt eliminate.
    [few] { $count } titluri conțineau salturi de rând sau tabulatoare, care sunt eliminate.
   *[other] { $count } de titluri conțineau salturi de rând sau tabulatoare, care sunt eliminate.
}
import-scrivener-structure-own-text-moved-line = { $count ->
    [one] Un dosar are text propriu, care este mutat într-un prim document din interiorul lui.
    [few] { $count } dosare au text propriu, care este mutat într-un prim document din interiorul lor.
   *[other] { $count } de dosare au text propriu, care este mutat într-un prim document din interiorul lor.
}
import-scrivener-structure-text-with-children-line = { $count ->
    [one] Un document care conține alte documente devine dosar.
    [few] { $count } documente care conțin alte documente devin dosare.
   *[other] { $count } de documente care conțin alte documente devin dosare.
}
import-scrivener-structure-empty-folder-plain-line = { $count ->
    [one] Un dosar gol rămâne un dosar simplu, ca niciun export să nu tipărească un titlu pe care Scrivener nu l-a tipărit niciodată.
    [few] { $count } dosare goale rămân dosare simple, ca niciun export să nu tipărească un titlu pe care Scrivener nu l-a tipărit niciodată.
   *[other] { $count } de dosare goale rămân dosare simple, ca niciun export să nu tipărească un titlu pe care Scrivener nu l-a tipărit niciodată.
}
import-scrivener-structure-book-inside-book-line = { $count ->
    [one] O carte aflată în altă carte devine parte.
    [few] { $count } cărți aflate în altă carte devin părți.
   *[other] { $count } de cărți aflate în altă carte devin părți.
}
import-scrivener-structure-prologue-not-numbered-line = { $count ->
    [one] Un prolog sau un epilog este exclus din numerotarea capitolelor.
    [few] { $count } prologuri sau epiloguri sunt excluse din numerotarea capitolelor.
   *[other] { $count } de prologuri sau epiloguri sunt excluse din numerotarea capitolelor.
}
import-scrivener-structure-matter-inside-book-line = { $count ->
    [one] Un element de tip pagini de început sau de sfârșit, aflat în afara manuscrisului, intră în carte ca paratext, exclus din exporturi până când îl includeți.
    [few] { $count } elemente de tip pagini de început sau de sfârșit, aflate în afara manuscrisului, intră în carte ca paratext, excluse din exporturi până când le includeți.
   *[other] { $count } de elemente de tip pagini de început sau de sfârșit, aflate în afara manuscrisului, intră în carte ca paratext, excluse din exporturi până când le includeți.
}
import-scrivener-structure-paratext-not-counted-line = { $documents ->
    [one] Un document al manuscrisului devine paratext (pagini de început sau de sfârșit), așa că { $words ->
        [one] un cuvânt nu mai este numărat
        [few] { $words } cuvinte nu mai sunt numărate
       *[other] { $words } de cuvinte nu mai sunt numărate
    } în manuscris.
    [few] { $documents } documente ale manuscrisului devin paratext (pagini de început sau de sfârșit), așa că { $words ->
        [one] un cuvânt nu mai este numărat
        [few] { $words } cuvinte nu mai sunt numărate
       *[other] { $words } de cuvinte nu mai sunt numărate
    } în manuscris.
   *[other] { $documents } de documente ale manuscrisului devin paratext (pagini de început sau de sfârșit), așa că { $words ->
        [one] un cuvânt nu mai este numărat
        [few] { $words } cuvinte nu mai sunt numărate
       *[other] { $words } de cuvinte nu mai sunt numărate
    } în manuscris.
}
import-scrivener-structure-document-notes-line = { $count ->
    [one] Notele unui document devin o notă alături de el.
    [few] Notele a { $count } documente devin note alături de ele.
   *[other] Notele a { $count } de documente devin note alături de ele.
}
import-scrivener-structure-draft-content-as-note-line = Textul propriu al dosarului manuscrisului devine o notă la începutul manuscrisului.
import-scrivener-structure-story-bible-folder-line = { $count ->
    [one] Un dosar de personaje sau de locuri devine o etichetă a bibliei poveștii.
    [few] { $count } dosare de personaje sau de locuri devin etichete ale bibliei poveștii.
   *[other] { $count } de dosare de personaje sau de locuri devin etichete ale bibliei poveștii.
}
import-scrivener-structure-templates-created-line = { $count ->
    [one] Un șablon de document devine un șablon de notă.
    [few] { $count } șabloane de document devin șabloane de notă.
   *[other] { $count } de șabloane de document devin șabloane de notă.
}
import-scrivener-structure-template-nesting-flattened-line = { $count ->
    [one] Un dosar de șabloane aflat în altul este aplatizat: fiecare text din el devine un șablon.
    [few] { $count } dosare de șabloane aflate în altele sunt aplatizate: fiecare text din ele devine un șablon.
   *[other] { $count } de dosare de șabloane aflate în altele sunt aplatizate: fiecare text din ele devine un șablon.
}
import-scrivener-structure-template-kept-as-note-line = { $count ->
    [one] Un șablon conține ceva ce un șablon de notă nu poate conține și este păstrat ca notă.
    [few] { $count } șabloane conțin ceva ce un șablon de notă nu poate conține și sunt păstrate ca note.
   *[other] { $count } de șabloane conțin ceva ce un șablon de notă nu poate conține și sunt păstrate ca note.
}
import-scrivener-structure-recovered-texts-line = { $count ->
    [one] Un text pe care nu îl enumera niciun element din clasor este preluat într-un dosar separat.
    [few] { $count } texte pe care nu le enumera niciun element din clasor sunt preluate într-un dosar separat.
   *[other] { $count } de texte pe care nu le enumera niciun element din clasor sunt preluate într-un dosar separat.
}
import-scrivener-structure-project-notes-line = { $count ->
    [one] O notă de proiect este preluată într-un dosar separat.
    [few] { $count } note de proiect sunt preluate într-un dosar separat.
   *[other] { $count } de note de proiect sunt preluate într-un dosar separat.
}
import-scrivener-structure-trash-brought-across-line = { $items ->
    [1] Coșul de gunoi este preluat: un element, pe care îl puteți restaura din coșul de gunoi.
    [few] Coșul de gunoi este preluat: { $items } elemente, { $writing ->
        [0] niciunul cu text
        [one] dintre care unul cu text
       *[other] dintre care { $writing } cu text
    }, pe care le puteți restaura din coșul de gunoi.
   *[other] Coșul de gunoi este preluat: { $items } de elemente, { $writing ->
        [0] niciunul cu text
        [one] dintre care unul cu text
       *[other] dintre care { $writing } cu text
    }, pe care le puteți restaura din coșul de gunoi.
}
import-scrivener-structure-not-compiled-line = { $count ->
    [one] Un element pe care Scrivener l-a exclus din compilare este exclus și din exporturi.
    [few] { $count } elemente pe care Scrivener le-a exclus din compilare sunt excluse și din exporturi.
   *[other] { $count } de elemente pe care Scrivener le-a exclus din compilare sunt excluse și din exporturi.
}
import-scrivener-structure-media-in-draft-line = { $count ->
    [one] O imagine sau un fișier din manuscris devine notă, pentru că nu își are locul într-o carte.
    [few] { $count } imagini sau fișiere din manuscris devin note, pentru că nu își au locul într-o carte.
   *[other] { $count } de imagini sau fișiere din manuscris devin note, pentru că nu își au locul într-o carte.
}
import-scrivener-structure-left-out-line = { $count ->
    [one] Un dosar de pe primul nivel este exclus, cu { $items ->
        [one] un element
        [few] { $items } elemente
       *[other] { $items } de elemente
    }.
    [few] { $count } dosare de pe primul nivel sunt excluse, cu { $items ->
        [one] un element
        [few] { $items } elemente
       *[other] { $items } de elemente
    }.
   *[other] { $count } de dosare de pe primul nivel sunt excluse, cu { $items ->
        [one] un element
        [few] { $items } elemente
       *[other] { $items } de elemente
    }.
}
import-scrivener-structure-trash-left-out-line = { $items ->
    [1] Coșul de gunoi nu este preluat: un element.
    [few] Coșul de gunoi nu este preluat: { $items } elemente, { $writing ->
        [0] niciunul cu text
        [one] dintre care unul cu text
       *[other] dintre care { $writing } cu text
    }.
   *[other] Coșul de gunoi nu este preluat: { $items } de elemente, { $writing ->
        [0] niciunul cu text
        [one] dintre care unul cu text
       *[other] dintre care { $writing } cu text
    }.
}
import-scrivener-structure-card-image-left-behind-line = { $count ->
    [one] Imaginea de pe fișa unui element al manuscrisului nu este preluată: ar fi tipărită în carte.
    [few] Imaginile de pe fișele a { $count } elemente ale manuscrisului nu sunt preluate: ar fi tipărite în carte.
   *[other] Imaginile de pe fișele a { $count } de elemente ale manuscrisului nu sunt preluate: ar fi tipărite în carte.
}
import-scrivener-structure-default-template-not-linked-line = { $count ->
    [one] Un dosar avea un șablon implicit pentru documentele noi din el. În Skribisto, doar o etichetă a bibliei poveștii poate purta un șablon, așa că această legătură nu este păstrată.
    [few] { $count } dosare aveau un șablon implicit pentru documentele noi din ele. În Skribisto, doar o etichetă a bibliei poveștii poate purta un șablon, așa că aceste legături nu sunt păstrate.
   *[other] { $count } de dosare aveau un șablon implicit pentru documentele noi din ele. În Skribisto, doar o etichetă a bibliei poveștii poate purta un șablon, așa că aceste legături nu sunt păstrate.
}
import-scrivener-structure-page-break-not-kept-line = { $count ->
    [one] O setare „Page Break Before” (salt de pagină înainte) nu are loc în Skribisto și nu este păstrată.
    [few] { $count } setări „Page Break Before” (salt de pagină înainte) nu au loc în Skribisto și nu sunt păstrate.
   *[other] { $count } de setări „Page Break Before” (salt de pagină înainte) nu au loc în Skribisto și nu sunt păstrate.
}
import-scrivener-structure-compile-as-is-not-kept-line = { $count ->
    [one] O setare „Compile As-Is” (compilare ca atare) nu are echivalent în Skribisto și nu este păstrată.
    [few] { $count } setări „Compile As-Is” (compilare ca atare) nu au echivalent în Skribisto și nu sunt păstrate.
   *[other] { $count } de setări „Compile As-Is” (compilare ca atare) nu au echivalent în Skribisto și nu sunt păstrate.
}

## What the import did with the labels, the status list and the dates
import-scrivener-metadata-status-categories-guessed-line = { $count ->
    [one] O stare pe care Skribisto nu o recunoaște primește o categorie după locul ei în listă. Verificați-o în Setări ▸ Proiect ▸ Stări.
    [few] { $count } stări pe care Skribisto nu le recunoaște primesc o categorie după locul lor în listă. Verificați-le în Setări ▸ Proiect ▸ Stări.
   *[other] { $count } de stări pe care Skribisto nu le recunoaște primesc o categorie după locul lor în listă. Verificați-le în Setări ▸ Proiect ▸ Stări.
}
import-scrivener-metadata-dates-unreadable-line = { $count ->
    [one] Datele unui element nu au putut fi citite: elementul poartă data importului.
    [few] Datele a { $count } elemente nu au putut fi citite: elementele poartă data importului.
   *[other] Datele a { $count } de elemente nu au putut fi citite: elementele poartă data importului.
}
import-scrivener-metadata-labels-became-tags-line = { $tags ->
    [one] Valorile Label devin o etichetă, pusă pe { $rows ->
        [one] un element
        [few] { $rows } elemente
       *[other] { $rows } de elemente
    }.
    [few] Valorile Label devin { $tags } etichete, puse pe { $rows ->
        [one] un element
        [few] { $rows } elemente
       *[other] { $rows } de elemente
    }.
   *[other] Valorile Label devin { $tags } de etichete, puse pe { $rows ->
        [one] un element
        [few] { $rows } elemente
       *[other] { $rows } de elemente
    }.
}
import-scrivener-metadata-statuses-became-ladder-line = { $rungs ->
    [one] Lista de stări este preluată ca listă de stări a proiectului: o stare, pusă pe { $rows ->
        [one] un element
        [few] { $rows } elemente
       *[other] { $rows } de elemente
    }.
    [few] Lista de stări este preluată ca listă de stări a proiectului: { $rungs } stări, puse pe { $rows ->
        [one] un element
        [few] { $rows } elemente
       *[other] { $rows } de elemente
    }.
   *[other] Lista de stări este preluată ca listă de stări a proiectului: { $rungs } de stări, puse pe { $rows ->
        [one] un element
        [few] { $rows } elemente
       *[other] { $rows } de elemente
    }.
}
import-scrivener-metadata-ladder-seeded-line = { $count ->
    [1] Proiectul nu avea o listă de stări, așa că pornește cu starea implicită a lui Skribisto.
    [few] Proiectul nu avea o listă de stări, așa că pornește cu cele { $count } stări implicite ale lui Skribisto.
   *[other] Proiectul nu avea o listă de stări, așa că pornește cu cele { $count } de stări implicite ale lui Skribisto.
}
import-scrivener-metadata-label-renamed-line = Câmpul Label se numea „{ $title }” în Scrivener. Acest nume apare în detaliile fiecărei etichete.
import-scrivener-metadata-status-renamed-line = Câmpul Status se numea „{ $title }” în Scrivener. Acest nume apare în detaliile fiecărei stări.
import-scrivener-metadata-none-label-collapsed-line = { $count ->
    [one] Valoarea Label „{ $name }”, care are același nume ca valoarea goală proprie lui Scrivener, este citită ca absența unei valori Label pe un element.
    [few] Valoarea Label „{ $name }”, care are același nume ca valoarea goală proprie lui Scrivener, este citită ca absența unei valori Label pe { $count } elemente.
   *[other] Valoarea Label „{ $name }”, care are același nume ca valoarea goală proprie lui Scrivener, este citită ca absența unei valori Label pe { $count } de elemente.
}
import-scrivener-metadata-none-status-collapsed-line = { $count ->
    [one] Starea „{ $name }”, care are același nume ca starea goală proprie lui Scrivener, este citită ca absența unei stări pe un element.
    [few] Starea „{ $name }”, care are același nume ca starea goală proprie lui Scrivener, este citită ca absența unei stări pe { $count } elemente.
   *[other] Starea „{ $name }”, care are același nume ca starea goală proprie lui Scrivener, este citită ca absența unei stări pe { $count } de elemente.
}
import-scrivener-metadata-unnamed-label-line = O valoare Label fără nume primește drept nume numărul ei, { $id }.
import-scrivener-metadata-unnamed-status-line = O stare fără nume primește drept nume numărul ei, { $id }.
import-scrivener-metadata-tag-name-disambiguated-line = { $count ->
    [one] O etichetă primește un nume nou, pentru că o altă etichetă are deja numele ei.
    [few] { $count } etichete primesc un nume nou, pentru că alte etichete au deja numele lor.
   *[other] { $count } de etichete primesc un nume nou, pentru că alte etichete au deja numele lor.
}
import-scrivener-metadata-rung-names-disambiguated-line = { $count ->
    [one] O stare are același nume ca alta, așa că primește un număr după nume. Verificați-o în Setări ▸ Proiect ▸ Stări.
    [few] { $count } stări au același nume ca altele, așa că fiecare primește un număr după nume. Verificați-le în Setări ▸ Proiect ▸ Stări.
   *[other] { $count } de stări au același nume ca altele, așa că fiecare primește un număr după nume. Verificați-le în Setări ▸ Proiect ▸ Stări.
}
import-scrivener-metadata-unused-labels-skipped-line = { $count ->
    [one] O valoare Label pe care nu o poartă niciun document nu este importată.
    [few] { $count } valori Label pe care nu le poartă niciun document nu sunt importate.
   *[other] { $count } de valori Label pe care nu le poartă niciun document nu sunt importate.
}
import-scrivener-metadata-labels-without-row-line = { $count ->
    [one] Valoarea Label a unui element care nu are loc în noul proiect nu este preluată.
    [few] Valorile Label a { $count } elemente care nu au loc în noul proiect nu sunt preluate.
   *[other] Valorile Label a { $count } de elemente care nu au loc în noul proiect nu sunt preluate.
}
import-scrivener-metadata-statuses-without-row-line = { $count ->
    [one] Starea unui element care nu are loc în noul proiect nu este preluată.
    [few] Stările a { $count } elemente care nu au loc în noul proiect nu sunt preluate.
   *[other] Stările a { $count } de elemente care nu au loc în noul proiect nu sunt preluate.
}
import-scrivener-metadata-unnamed-keyword-line = Un cuvânt-cheie fără nume primește drept nume numărul lui, { $id }.
import-scrivener-metadata-keywords-without-row-line = { $count ->
    [one] Cuvintele-cheie ale unui element care nu are loc în noul proiect nu sunt preluate.
    [few] Cuvintele-cheie a { $count } elemente care nu au loc în noul proiect nu sunt preluate.
   *[other] Cuvintele-cheie a { $count } de elemente care nu au loc în noul proiect nu sunt preluate.
}
import-scrivener-metadata-custom-values-without-row-line = { $count ->
    [one] Metadatele personalizate ale unui element care nu are loc în noul proiect nu sunt preluate.
    [few] Metadatele personalizate a { $count } elemente care nu au loc în noul proiect nu sunt preluate.
   *[other] Metadatele personalizate a { $count } de elemente care nu au loc în noul proiect nu sunt preluate.
}
import-scrivener-metadata-bookmarks-without-row-line = { $count ->
    [one] Semnele de carte ale unui element care nu are loc în noul proiect nu sunt preluate.
    [few] Semnele de carte a { $count } elemente care nu au loc în noul proiect nu sunt preluate.
   *[other] Semnele de carte a { $count } de elemente care nu au loc în noul proiect nu sunt preluate.
}
import-scrivener-metadata-targets-without-row-line = { $count ->
    [one] Ținta unui element care nu are loc în noul proiect nu este preluată.
    [few] Țintele a { $count } elemente care nu au loc în noul proiect nu sunt preluate.
   *[other] Țintele a { $count } de elemente care nu au loc în noul proiect nu sunt preluate.
}

## What the import does with the keywords, custom metadata, collections, bookmarks, targets,
## the deadline, the auto-complete list and the icons. Each line is said twice: on the Review
## page before the import, then in its report, so it reads the same at both times. `$note`
## and `$folder` are the titles the import gives a note and its "From Scrivener" folder;
## `$day` is a day as YYYY-MM-DD.
import-scrivener-metadata-pov-hint-line = { $count ->
    [1] Valoarea Label sau valoarea de câmp { $values } corespunde intrării din biblia poveștii { $matches }. Dacă numește punctul de vedere, setați-l în inspector: importul nu leagă nimic.
   *[other] Valorile Label sau valorile de câmp { $values } corespund intrărilor din biblia poveștii { $matches }. Dacă numesc puncte de vedere, setați-le în inspector: importul nu leagă nimic.
}
import-scrivener-metadata-checkbox-default-unknown-line = Câmpul de tip casetă de bifare „{ $field }” are o valoare implicită pe care Skribisto nu o poate citi, așa că nu poate ști ce documente sunt bifate implicit. Doar documentele bifate unul câte unul poartă eticheta lui.
import-scrivener-metadata-goals-in-other-unit-line = { $count ->
    [one] Un obiectiv este stabilit în cealaltă unitate de numărare. Este păstrat și apare după ce schimbați unitatea de numărare în Setări ▸ Proiect ▸ Alcătuire.
    [few] { $count } obiective sunt stabilite în cealaltă unitate de numărare. Sunt păstrate și apar după ce schimbați unitatea de numărare în Setări ▸ Proiect ▸ Alcătuire.
   *[other] { $count } de obiective sunt stabilite în cealaltă unitate de numărare. Sunt păstrate și apar după ce schimbați unitatea de numărare în Setări ▸ Proiect ▸ Alcătuire.
}
import-scrivener-metadata-counting-differs-line = Scrivener calcula ținta manuscrisului pe alte documente decât Skribisto. Obiectivul cărții numără scenele incluse în exporturi.
import-scrivener-metadata-writing-days-not-read-line = Zilele de scris „{ $raw }” nu pot fi citite, așa că Ritmul cărții consideră fiecare zi o zi de scris. Modificați-le acolo.
import-scrivener-metadata-script-mode-rows-line = { $count ->
    [one] Un element era în modul scenariu în Scrivener. Skribisto îi păstrează cuvintele ca proză, fără aspectul de scenariu.
    [few] { $count } elemente erau în modul scenariu în Scrivener. Skribisto le păstrează cuvintele ca proză, fără aspectul de scenariu.
   *[other] { $count } de elemente erau în modul scenariu în Scrivener. Skribisto le păstrează cuvintele ca proză, fără aspectul de scenariu.
}
import-scrivener-metadata-keywords-became-tags-line = { $tags ->
    [one] Un cuvânt-cheie devine etichetă, pusă pe { $rows ->
        [one] un element
        [few] { $rows } elemente
       *[other] { $rows } de elemente
    }.
    [few] { $tags } cuvinte-cheie devin etichete, puse pe { $rows ->
        [one] un element
        [few] { $rows } elemente
       *[other] { $rows } de elemente
    }.
   *[other] { $tags } de cuvinte-cheie devin etichete, puse pe { $rows ->
        [one] un element
        [few] { $rows } elemente
       *[other] { $rows } de elemente
    }.
}
import-scrivener-metadata-keyword-hierarchy-flattened-line = { $count ->
    [one] Un cuvânt-cheie se afla în interiorul altuia în Scrivener. Etichetele nu au ierarhie: detaliile etichetei lui spun unde se afla.
    [few] { $count } cuvinte-cheie se aflau în interiorul altora în Scrivener. Etichetele nu au ierarhie: detaliile fiecărei etichete spun unde se afla cuvântul-cheie corespunzător.
   *[other] { $count } de cuvinte-cheie se aflau în interiorul altora în Scrivener. Etichetele nu au ierarhie: detaliile fiecărei etichete spun unde se afla cuvântul-cheie corespunzător.
}
import-scrivener-metadata-fields-became-tags-line = { $fields ->
    [one] Un câmp de tip listă sau casetă de bifare devine { $tags ->
        [one] o etichetă, pusă
        [few] { $tags } etichete, puse
       *[other] { $tags } de etichete, puse
    } pe { $rows ->
        [one] un element
        [few] { $rows } elemente
       *[other] { $rows } de elemente
    }.
    [few] { $fields } câmpuri de tip listă și casetă de bifare devin { $tags ->
        [one] o etichetă, pusă
        [few] { $tags } etichete, puse
       *[other] { $tags } de etichete, puse
    } pe { $rows ->
        [one] un element
        [few] { $rows } elemente
       *[other] { $rows } de elemente
    }.
   *[other] { $fields } de câmpuri de tip listă și casetă de bifare devin { $tags ->
        [one] o etichetă, pusă
        [few] { $tags } etichete, puse
       *[other] { $tags } de etichete, puse
    } pe { $rows ->
        [one] un element
        [few] { $rows } elemente
       *[other] { $rows } de elemente
    }.
}
import-scrivener-metadata-collections-became-tags-line = { $tags ->
    [one] O colecție devine etichetă, pusă pe { $rows ->
        [one] un element
        [few] { $rows } elemente
       *[other] { $rows } de elemente
    }.
    [few] { $tags } colecții devin etichete, puse pe { $rows ->
        [one] un element
        [few] { $rows } elemente
       *[other] { $rows } de elemente
    }.
   *[other] { $tags } de colecții devin etichete, puse pe { $rows ->
        [one] un element
        [few] { $rows } elemente
       *[other] { $rows } de elemente
    }.
}
import-scrivener-metadata-collection-order-lost-line = { $count ->
    [one] O etichetă nu are ordine, așa că ordinea documentelor dintr-o colecție nu este păstrată.
    [few] O etichetă nu are ordine, așa că ordinea documentelor din { $count } colecții nu este păstrată.
   *[other] O etichetă nu are ordine, așa că ordinea documentelor din { $count } de colecții nu este păstrată.
}
import-scrivener-metadata-bookmarks-became-links-line = { $count ->
    [one] Un semn de carte devine o legătură între două note sau între o scenă și o intrare din biblia poveștii: vedeți Legături și Distribuție.
    [few] { $count } semne de carte devin legături între note sau între scene și intrări din biblia poveștii: vedeți Legături și Distribuție.
   *[other] { $count } de semne de carte devin legături între note sau între scene și intrări din biblia poveștii: vedeți Legături și Distribuție.
}
import-scrivener-metadata-goals-kept-line = { $count ->
    [one] Ținta pe care Scrivener o stabilea pentru un document devine obiectivul lui în Skribisto.
    [few] Țintele pe care Scrivener le stabilea pentru { $count } documente devin obiectivele lor în Skribisto.
   *[other] Țintele pe care Scrivener le stabilea pentru { $count } de documente devin obiectivele lor în Skribisto.
}
import-scrivener-metadata-draft-target-kept-words-line = { $value ->
    [one] Ținta manuscrisului, de un cuvânt, devine obiectivul cărții.
    [few] Ținta manuscrisului, de { $value } cuvinte, devine obiectivul cărții.
   *[other] Ținta manuscrisului, de { $value } de cuvinte, devine obiectivul cărții.
}
import-scrivener-metadata-draft-target-kept-characters-line = { $value ->
    [one] Ținta manuscrisului, de un caracter, devine obiectivul cărții.
    [few] Ținta manuscrisului, de { $value } caractere, devine obiectivul cărții.
   *[other] Ținta manuscrisului, de { $value } de caractere, devine obiectivul cărții.
}
import-scrivener-metadata-pace-planned-line = Termenul-limită, { $day }, configurează Ritmul cărții, care planifică numărul de cuvinte pentru fiecare zi de scris aleasă în Scrivener, începând din ziua importului.
import-scrivener-metadata-pace-planned-day-off-line = Termenul-limită, { $day }, configurează Ritmul cărții, care planifică numărul de cuvinte pentru fiecare zi de scris aleasă în Scrivener, începând din ziua importului. Ca în Scrivener, ziua termenului-limită este o zi liberă.
import-scrivener-metadata-dictionary-words-line = { $count ->
    [one] Un cuvânt din completarea automată intră în dicționarul personal al proiectului, ca să nu fie marcat ca greșit.
    [few] { $count } cuvinte din completarea automată intră în dicționarul personal al proiectului, ca să nu fie marcate ca greșite.
   *[other] { $count } de cuvinte din completarea automată intră în dicționarul personal al proiectului, ca să nu fie marcate ca greșite.
}
import-scrivener-metadata-listed-project-line = { $count ->
    [one] Un detaliu al proiectului este enumerat în nota „{ $note }” din dosarul „{ $folder }”.
    [few] { $count } detalii ale proiectului sunt enumerate în nota „{ $note }” din dosarul „{ $folder }”.
   *[other] { $count } de detalii ale proiectului sunt enumerate în nota „{ $note }” din dosarul „{ $folder }”.
}
import-scrivener-metadata-listed-project-bookmarks-line = { $count ->
    [one] Un semn de carte al proiectului este enumerat în nota „{ $note }” din dosarul „{ $folder }”.
    [few] { $count } semne de carte ale proiectului sunt enumerate în nota „{ $note }” din dosarul „{ $folder }”.
   *[other] { $count } de semne de carte ale proiectului sunt enumerate în nota „{ $note }” din dosarul „{ $folder }”.
}
import-scrivener-metadata-listed-replacements-line = { $count ->
    [one] O înlocuire la compilare este enumerată în nota „{ $note }” din dosarul „{ $folder }”. Skribisto nu o aplică.
    [few] { $count } înlocuiri la compilare sunt enumerate în nota „{ $note }” din dosarul „{ $folder }”. Skribisto nu le aplică.
   *[other] { $count } de înlocuiri la compilare sunt enumerate în nota „{ $note }” din dosarul „{ $folder }”. Skribisto nu le aplică.
}
import-scrivener-metadata-listed-auto-complete-line = { $count ->
    [one] O intrare de completare automată formată din mai multe cuvinte este enumerată în nota „{ $note }” din dosarul „{ $folder }”.
    [few] { $count } intrări de completare automată formate din mai multe cuvinte sunt enumerate în nota „{ $note }” din dosarul „{ $folder }”.
   *[other] { $count } de intrări de completare automată formate din mai multe cuvinte sunt enumerate în nota „{ $note }” din dosarul „{ $folder }”.
}
import-scrivener-metadata-listed-saved-searches-line = { $count ->
    [one] O căutare salvată este enumerată în nota „{ $note }” din dosarul „{ $folder }”.
    [few] { $count } căutări salvate sunt enumerate în nota „{ $note }” din dosarul „{ $folder }”.
   *[other] { $count } de căutări salvate sunt enumerate în nota „{ $note }” din dosarul „{ $folder }”.
}
import-scrivener-metadata-listed-custom-metadata-line = { $count ->
    [one] O valoare de metadate personalizate pe care o etichetă nu o poate conține este enumerată în nota „{ $note }” din dosarul „{ $folder }”.
    [few] { $count } valori de metadate personalizate pe care o etichetă nu le poate conține sunt enumerate în nota „{ $note }” din dosarul „{ $folder }”.
   *[other] { $count } de valori de metadate personalizate pe care o etichetă nu le poate conține sunt enumerate în nota „{ $note }” din dosarul „{ $folder }”.
}
import-scrivener-metadata-listed-targets-line = { $count ->
    [one] O țintă de document este enumerată în nota „{ $note }” din dosarul „{ $folder }”.
    [few] { $count } ținte de document sunt enumerate în nota „{ $note }” din dosarul „{ $folder }”.
   *[other] { $count } de ținte de document sunt enumerate în nota „{ $note }” din dosarul „{ $folder }”.
}
import-scrivener-metadata-listed-bookmarks-line = { $count ->
    [one] Un semn de carte pe care nicio legătură din Skribisto nu îl poate afișa este enumerat în nota „{ $note }” din dosarul „{ $folder }”.
    [few] { $count } semne de carte pe care nicio legătură din Skribisto nu le poate afișa sunt enumerate în nota „{ $note }” din dosarul „{ $folder }”.
   *[other] { $count } de semne de carte pe care nicio legătură din Skribisto nu le poate afișa sunt enumerate în nota „{ $note }” din dosarul „{ $folder }”.
}
import-scrivener-metadata-targets-on-notes-line = { $count ->
    [one] O țintă de document se află pe o notă, al cărei text nu este numărat de niciun obiectiv, așa că nu devine obiectiv.
    [few] { $count } ținte de document se află pe note, al căror text nu este numărat de niciun obiectiv, așa că nu devin obiective.
   *[other] { $count } de ținte de document se află pe note, al căror text nu este numărat de niciun obiectiv, așa că nu devin obiective.
}
import-scrivener-metadata-targets-on-split-documents-line = { $count ->
    [one] O țintă de document se află pe un document împărțit la titlurile sale de secțiune, ale cărui capitole nu sunt numărate împreună de niciun obiectiv, așa că nu devine obiectiv.
    [few] { $count } ținte de document se află pe documente împărțite la titlurile lor de secțiune, ale căror capitole nu sunt numărate împreună de niciun obiectiv, așa că nu devin obiective.
   *[other] { $count } de ținte de document se află pe documente împărțite la titlurile lor de secțiune, ale căror capitole nu sunt numărate împreună de niciun obiectiv, așa că nu devin obiective.
}
import-scrivener-metadata-bookmark-ends-without-row-line = { $count ->
    [one] Un semn de carte duce la un document care nu are loc în noul proiect, de exemplu un șablon de document. Este enumerat după locul în care se afla documentul în Scrivener.
    [few] { $count } semne de carte duc la documente care nu au loc în noul proiect, de exemplu șabloane de document. Sunt enumerate după locul în care se aflau documentele în Scrivener.
   *[other] { $count } de semne de carte duc la documente care nu au loc în noul proiect, de exemplu șabloane de document. Sunt enumerate după locul în care se aflau documentele în Scrivener.
}
import-scrivener-metadata-unused-keywords-skipped-line = { $count ->
    [one] Un cuvânt-cheie pe care nu îl poartă niciun document nu este importat.
    [few] { $count } cuvinte-cheie pe care nu le poartă niciun document nu sunt importate.
   *[other] { $count } de cuvinte-cheie pe care nu le poartă niciun document nu sunt importate.
}
import-scrivener-metadata-list-value-invalid-line = { $count ->
    [one] O valoare a câmpului de tip listă „{ $field }” nu numește niciuna dintre opțiunile lui și nu este importată.
    [few] { $count } valori ale câmpului de tip listă „{ $field }” nu numesc niciuna dintre opțiunile lui și nu sunt importate.
   *[other] { $count } de valori ale câmpului de tip listă „{ $field }” nu numesc niciuna dintre opțiunile lui și nu sunt importate.
}
import-scrivener-metadata-undefined-custom-field-line = { $count ->
    [one] O valoare aparține unui câmp personalizat pe care proiectul nu îl mai definește și pe care nici Scrivener nu îl afișează. Nu este importată.
    [few] { $count } valori aparțin unor câmpuri personalizate pe care proiectul nu le mai definește și pe care nici Scrivener nu le afișează. Nu sunt importate.
   *[other] { $count } de valori aparțin unor câmpuri personalizate pe care proiectul nu le mai definește și pe care nici Scrivener nu le afișează. Nu sunt importate.
}
import-scrivener-metadata-collection-members-missing-line = { $count ->
    [one] Un document al colecției „{ $name }” nu are loc în noul proiect sau nu mai există în proiectul Scrivener.
    [few] { $count } documente ale colecției „{ $name }” nu au loc în noul proiect sau nu mai există în proiectul Scrivener.
   *[other] { $count } de documente ale colecției „{ $name }” nu au loc în noul proiect sau nu mai există în proiectul Scrivener.
}
import-scrivener-metadata-empty-collections-skipped-line = { $count ->
    [one] O colecție care nu conține niciun document nu este importată.
    [few] { $count } colecții care nu conțin niciun document nu sunt importate.
   *[other] { $count } de colecții care nu conțin niciun document nu sunt importate.
}
import-scrivener-metadata-bookmarks-unresolved-line = { $count ->
    [one] Un semn de carte duce la un document pe care proiectul Scrivener nu îl mai conține și nu este importat.
    [few] { $count } semne de carte duc la documente pe care proiectul Scrivener nu le mai conține și nu sunt importate.
   *[other] { $count } de semne de carte duc la documente pe care proiectul Scrivener nu le mai conține și nu sunt importate.
}
import-scrivener-metadata-targets-in-pages-line = { $count ->
    [one] O țintă de document numără pagini, pe care Skribisto nu le numără, așa că nu devine obiectiv.
    [few] { $count } ținte de document numără pagini, pe care Skribisto nu le numără, așa că nu devin obiective.
   *[other] { $count } de ținte de document numără pagini, pe care Skribisto nu le numără, așa că nu devin obiective.
}
import-scrivener-metadata-targets-unreadable-line = { $count ->
    [one] O țintă nu este importată: valoarea ei nu este un număr, este negativă sau depășește ce poate conține un obiectiv.
    [few] { $count } ținte nu sunt importate: valorile lor nu sunt numere, sunt negative sau depășesc ce poate conține un obiectiv.
   *[other] { $count } de ținte nu sunt importate: valorile lor nu sunt numere, sunt negative sau depășesc ce poate conține un obiectiv.
}
import-scrivener-metadata-draft-target-in-pages-line = { $value ->
    [one] Ținta manuscrisului, de o pagină, nu devine obiectiv: Skribisto numără cuvinte sau caractere. Apare în nota „{ $note }” din dosarul „{ $folder }”.
    [few] Ținta manuscrisului, de { $value } pagini, nu devine obiectiv: Skribisto numără cuvinte sau caractere. Apare în nota „{ $note }” din dosarul „{ $folder }”.
   *[other] Ținta manuscrisului, de { $value } de pagini, nu devine obiectiv: Skribisto numără cuvinte sau caractere. Apare în nota „{ $note }” din dosarul „{ $folder }”.
}
import-scrivener-metadata-draft-target-no-book-words-line = { $value ->
    [one] Ținta manuscrisului, de un cuvânt, nu devine obiectiv, pentru că manuscrisul formează mai multe cărți sau niciuna. Apare în nota „{ $note }” din dosarul „{ $folder }”.
    [few] Ținta manuscrisului, de { $value } cuvinte, nu devine obiectiv, pentru că manuscrisul formează mai multe cărți sau niciuna. Apare în nota „{ $note }” din dosarul „{ $folder }”.
   *[other] Ținta manuscrisului, de { $value } de cuvinte, nu devine obiectiv, pentru că manuscrisul formează mai multe cărți sau niciuna. Apare în nota „{ $note }” din dosarul „{ $folder }”.
}
import-scrivener-metadata-draft-target-no-book-characters-line = { $value ->
    [one] Ținta manuscrisului, de un caracter, nu devine obiectiv, pentru că manuscrisul formează mai multe cărți sau niciuna. Apare în nota „{ $note }” din dosarul „{ $folder }”.
    [few] Ținta manuscrisului, de { $value } caractere, nu devine obiectiv, pentru că manuscrisul formează mai multe cărți sau niciuna. Apare în nota „{ $note }” din dosarul „{ $folder }”.
   *[other] Ținta manuscrisului, de { $value } de caractere, nu devine obiectiv, pentru că manuscrisul formează mai multe cărți sau niciuna. Apare în nota „{ $note }” din dosarul „{ $folder }”.
}
import-scrivener-metadata-deadline-passed-line = Termenul-limită, { $day }, a trecut, așa că nu configurează Ritmul cărții. Apare în nota „{ $note }” din dosarul „{ $folder }”.
import-scrivener-metadata-deadline-no-writing-day-line = Între ziua importului și termenul-limită, { $day }, nu mai rămâne nicio zi de scris, așa că termenul nu configurează Ritmul cărții. Apare în nota „{ $note }” din dosarul „{ $folder }”.
import-scrivener-metadata-deadline-template-line = Termenul-limită, { $day }, era salvat în șablon, așa că nu configurează Ritmul cărții. Apare în nota „{ $note }” din dosarul „{ $folder }”.
import-scrivener-metadata-deadline-no-word-goal-line = Termenul-limită, { $day }, nu configurează Ritmul cărții, pentru că manuscrisul nu are o țintă de cuvinte după care să se planifice. Apare în nota „{ $note }” din dosarul „{ $folder }”.
import-scrivener-metadata-deadline-character-goal-line = Termenul-limită, { $day }, nu configurează Ritmul cărții, pentru că ținta manuscrisului numără caractere, iar Ritmul numără cuvinte. Apare în nota „{ $note }” din dosarul „{ $folder }”.
import-scrivener-metadata-deadline-no-book-line = Termenul-limită, { $day }, nu configurează Ritmul niciunei cărți, pentru că manuscrisul formează mai multe cărți sau niciuna. Apare în nota „{ $note }” din dosarul „{ $folder }”.
import-scrivener-metadata-deadline-unreadable-line = Termenul-limită nu poate fi citit, așa că nu configurează Ritmul cărții. Apare, așa cum a fost scris, în nota „{ $note }” din dosarul „{ $folder }”.
import-scrivener-metadata-session-target-words-line = { $value ->
    [one] Ținta sesiunii, de un cuvânt, nu este importată: în Skribisto, obiectivul sesiunii de scriere este o singură setare pentru toate proiectele. Apare în nota „{ $note }” din dosarul „{ $folder }”.
    [few] Ținta sesiunii, de { $value } cuvinte, nu este importată: în Skribisto, obiectivul sesiunii de scriere este o singură setare pentru toate proiectele. Apare în nota „{ $note }” din dosarul „{ $folder }”.
   *[other] Ținta sesiunii, de { $value } de cuvinte, nu este importată: în Skribisto, obiectivul sesiunii de scriere este o singură setare pentru toate proiectele. Apare în nota „{ $note }” din dosarul „{ $folder }”.
}
import-scrivener-metadata-session-target-characters-line = { $value ->
    [one] Ținta sesiunii, de un caracter, nu este importată: în Skribisto, obiectivul sesiunii de scriere este o singură setare pentru toate proiectele, exprimată în cuvinte. Apare în nota „{ $note }” din dosarul „{ $folder }”.
    [few] Ținta sesiunii, de { $value } caractere, nu este importată: în Skribisto, obiectivul sesiunii de scriere este o singură setare pentru toate proiectele, exprimată în cuvinte. Apare în nota „{ $note }” din dosarul „{ $folder }”.
   *[other] Ținta sesiunii, de { $value } de caractere, nu este importată: în Skribisto, obiectivul sesiunii de scriere este o singură setare pentru toate proiectele, exprimată în cuvinte. Apare în nota „{ $note }” din dosarul „{ $folder }”.
}
import-scrivener-metadata-session-target-pages-line = { $value ->
    [one] Ținta sesiunii, de o pagină, nu este importată: în Skribisto, obiectivul sesiunii de scriere este o singură setare pentru toate proiectele, exprimată în cuvinte. Apare în nota „{ $note }” din dosarul „{ $folder }”.
    [few] Ținta sesiunii, de { $value } pagini, nu este importată: în Skribisto, obiectivul sesiunii de scriere este o singură setare pentru toate proiectele, exprimată în cuvinte. Apare în nota „{ $note }” din dosarul „{ $folder }”.
   *[other] Ținta sesiunii, de { $value } de pagini, nu este importată: în Skribisto, obiectivul sesiunii de scriere este o singură setare pentru toate proiectele, exprimată în cuvinte. Apare în nota „{ $note }” din dosarul „{ $folder }”.
}
import-scrivener-metadata-session-target-calculated-words-line = { $value ->
    [one] Ținta sesiunii, de un cuvânt, calculată de Scrivener pe baza termenului-limită și a zilelor de scris, nu este importată: în Skribisto, obiectivul sesiunii de scriere este o singură setare pentru toate proiectele. Apare în nota „{ $note }” din dosarul „{ $folder }”.
    [few] Ținta sesiunii, de { $value } cuvinte, calculată de Scrivener pe baza termenului-limită și a zilelor de scris, nu este importată: în Skribisto, obiectivul sesiunii de scriere este o singură setare pentru toate proiectele. Apare în nota „{ $note }” din dosarul „{ $folder }”.
   *[other] Ținta sesiunii, de { $value } de cuvinte, calculată de Scrivener pe baza termenului-limită și a zilelor de scris, nu este importată: în Skribisto, obiectivul sesiunii de scriere este o singură setare pentru toate proiectele. Apare în nota „{ $note }” din dosarul „{ $folder }”.
}
import-scrivener-metadata-session-target-calculated-characters-line = { $value ->
    [one] Ținta sesiunii, de un caracter, calculată de Scrivener pe baza termenului-limită și a zilelor de scris, nu este importată: în Skribisto, obiectivul sesiunii de scriere este o singură setare pentru toate proiectele, exprimată în cuvinte. Apare în nota „{ $note }” din dosarul „{ $folder }”.
    [few] Ținta sesiunii, de { $value } caractere, calculată de Scrivener pe baza termenului-limită și a zilelor de scris, nu este importată: în Skribisto, obiectivul sesiunii de scriere este o singură setare pentru toate proiectele, exprimată în cuvinte. Apare în nota „{ $note }” din dosarul „{ $folder }”.
   *[other] Ținta sesiunii, de { $value } de caractere, calculată de Scrivener pe baza termenului-limită și a zilelor de scris, nu este importată: în Skribisto, obiectivul sesiunii de scriere este o singură setare pentru toate proiectele, exprimată în cuvinte. Apare în nota „{ $note }” din dosarul „{ $folder }”.
}
import-scrivener-metadata-session-target-calculated-pages-line = { $value ->
    [one] Ținta sesiunii, de o pagină, calculată de Scrivener pe baza termenului-limită și a zilelor de scris, nu este importată: în Skribisto, obiectivul sesiunii de scriere este o singură setare pentru toate proiectele, exprimată în cuvinte. Apare în nota „{ $note }” din dosarul „{ $folder }”.
    [few] Ținta sesiunii, de { $value } pagini, calculată de Scrivener pe baza termenului-limită și a zilelor de scris, nu este importată: în Skribisto, obiectivul sesiunii de scriere este o singură setare pentru toate proiectele, exprimată în cuvinte. Apare în nota „{ $note }” din dosarul „{ $folder }”.
   *[other] Ținta sesiunii, de { $value } de pagini, calculată de Scrivener pe baza termenului-limită și a zilelor de scris, nu este importată: în Skribisto, obiectivul sesiunii de scriere este o singură setare pentru toate proiectele, exprimată în cuvinte. Apare în nota „{ $note }” din dosarul „{ $folder }”.
}
import-scrivener-metadata-icons-dropped-line = { $count ->
    [one] Un element avea o pictogramă în clasor, care nu este preluată: Skribisto afișează pictograma fiecărui element după tipul lui.
    [few] { $count } elemente aveau pictograme în clasor, care nu sunt preluate: Skribisto afișează pictograma fiecărui element după tipul lui.
   *[other] { $count } de elemente aveau pictograme în clasor, care nu sunt preluate: Skribisto afișează pictograma fiecărui element după tipul lui.
}

## What converting the prose did, as the import's report says it. `$count` is how many texts,
## or how many of a thing, as each line says; `$example` quotes the project.
import-scrivener-prose-unreadable-line = { $count ->
    [one] Un text nu a putut fi citit. Ajunge gol.
    [few] { $count } texte nu au putut fi citite. Ajung goale.
   *[other] { $count } de texte nu au putut fi citite. Ajung goale.
}
import-scrivener-prose-comments-unreadable-line = { $count ->
    [one] Comentariile și notele de subsol legate ale unui text nu au putut fi citite. Cuvintele pe care le marcau sunt păstrate.
    [few] Comentariile și notele de subsol legate a { $count } texte nu au putut fi citite. Cuvintele pe care le marcau sunt păstrate.
   *[other] Comentariile și notele de subsol legate a { $count } de texte nu au putut fi citite. Cuvintele pe care le marcau sunt păstrate.
}
import-scrivener-prose-read-as-plain-text-line = { $count ->
    [one] Un text era stocat ca text simplu, nu ca text formatat, și este citit ca atare.
    [few] { $count } texte erau stocate ca text simplu, nu ca text formatat, și sunt citite ca atare.
   *[other] { $count } de texte erau stocate ca text simplu, nu ca text formatat, și sunt citite ca atare.
}
import-scrivener-prose-inline-note-unmatched-line = { $count ->
    [one] Un marcaj de notă în text nu are pereche și este păstrat ca text.
    [few] { $count } marcaje de notă în text nu au pereche și sunt păstrate ca text.
   *[other] { $count } de marcaje de notă în text nu au pereche și sunt păstrate ca text.
}
import-scrivener-prose-style-unresolved-line = { $count ->
    [one] Un marcaj de stil numește un stil pe care proiectul nu îl definește. Este eliminat, iar textul își păstrează formatarea.
    [few] { $count } marcaje de stil numesc un stil pe care proiectul nu îl definește. Sunt eliminate, iar textul își păstrează formatarea.
   *[other] { $count } de marcaje de stil numesc un stil pe care proiectul nu îl definește. Sunt eliminate, iar textul își păstrează formatarea.
}
import-scrivener-prose-styles-lost-in-export-line = { $count ->
    [one] Un paragraf ar putea fi un titlu de secțiune care și-a pierdut stilul când Scrivener 3 a exportat proiectul în formatul lui mai vechi și este preluat ca paragraf obișnuit. Verificați-l sau importați proiectul original, din care a fost făcut exportul.
    [few] { $count } paragrafe ar putea fi titluri de secțiune care și-au pierdut stilul când Scrivener 3 a exportat proiectul în formatul lui mai vechi și sunt preluate ca paragrafe obișnuite. Verificați-le sau importați proiectul original, din care a fost făcut exportul.
   *[other] { $count } de paragrafe ar putea fi titluri de secțiune care și-au pierdut stilul când Scrivener 3 a exportat proiectul în formatul lui mai vechi și sunt preluate ca paragrafe obișnuite. Verificați-le sau importați proiectul original, din care a fost făcut exportul.
}
import-scrivener-prose-comment-anchor-without-entry-line = { $count ->
    [one] Un comentariu sau o notă de subsol își pierduse conținutul în Scrivener. Cuvintele pe care le marca sunt păstrate, dar fără comentariu sau notă.
    [few] { $count } comentarii sau note de subsol își pierduseră conținutul în Scrivener. Cuvintele pe care le marcau sunt păstrate, dar fără comentarii sau note.
   *[other] { $count } de comentarii sau note de subsol își pierduseră conținutul în Scrivener. Cuvintele pe care le marcau sunt păstrate, dar fără comentarii sau note.
}
import-scrivener-prose-comment-entry-without-anchor-line = { $count ->
    [one] Un comentariu nu mai avea un loc în textul lui și este atașat primului paragraf.
    [few] { $count } comentarii nu mai aveau un loc în textul lor și sunt atașate primului paragraf.
   *[other] { $count } de comentarii nu mai aveau un loc în textul lor și sunt atașate primului paragraf.
}
import-scrivener-prose-footnote-entry-without-anchor-line = { $count ->
    [one] O notă de subsol legată nu mai avea un loc în textul ei, așa că trimiterea ei este pusă la sfârșitul acelui text.
    [few] { $count } note de subsol legate nu mai aveau un loc în textele lor, așa că trimiterea fiecăreia este pusă la sfârșitul textului din care face parte.
   *[other] { $count } de note de subsol legate nu mai aveau un loc în textele lor, așa că trimiterea fiecăreia este pusă la sfârșitul textului din care face parte.
}
import-scrivener-prose-range-comment-reading-line = { $count ->
    [one] Un comentariu dintr-un Scrivener mai vechi a fost plasat după o altă interpretare a poziției lui. Verificați că se află pe cuvintele potrivite.
    [few] { $count } comentarii dintr-un Scrivener mai vechi au fost plasate după o altă interpretare a poziției lor. Verificați că se află pe cuvintele potrivite.
   *[other] { $count } de comentarii dintr-un Scrivener mai vechi au fost plasate după o altă interpretare a poziției lor. Verificați că se află pe cuvintele potrivite.
}
import-scrivener-prose-range-comment-reading-markers-counted-line = { $count ->
    [one] Un comentariu dintr-un Scrivener mai vechi a fost plasat numărând și marcajele notelor în text. Verificați că se află pe cuvintele potrivite.
    [few] { $count } comentarii dintr-un Scrivener mai vechi au fost plasate numărând și marcajele notelor în text. Verificați că se află pe cuvintele potrivite.
   *[other] { $count } de comentarii dintr-un Scrivener mai vechi au fost plasate numărând și marcajele notelor în text. Verificați că se află pe cuvintele potrivite.
}
import-scrivener-prose-range-comment-reading-pictures-not-counted-line = { $count ->
    [one] Un comentariu dintr-un Scrivener mai vechi a fost plasat fără să se țină seama de imagini. Verificați că se află pe cuvintele potrivite.
    [few] { $count } comentarii dintr-un Scrivener mai vechi au fost plasate fără să se țină seama de imagini. Verificați că se află pe cuvintele potrivite.
   *[other] { $count } de comentarii dintr-un Scrivener mai vechi au fost plasate fără să se țină seama de imagini. Verificați că se află pe cuvintele potrivite.
}
import-scrivener-prose-range-comment-approximate-line = { $count ->
    [one] Un comentariu dintr-un Scrivener mai vechi nu corespunde exact niciunei interpretări a poziției lui și poate fi decalat cu câteva caractere.
    [few] { $count } comentarii dintr-un Scrivener mai vechi nu corespund exact niciunei interpretări a poziției lor și pot fi decalate cu câteva caractere.
   *[other] { $count } de comentarii dintr-un Scrivener mai vechi nu corespund exact niciunei interpretări a poziției lor și pot fi decalate cu câteva caractere.
}
import-scrivener-prose-document-link-unresolved-line = { $count ->
    [one] O legătură indică un document pe care importul nu l-a preluat. Cuvintele ei rămân.
    [few] { $count } legături indică documente pe care importul nu le-a preluat. Cuvintele lor rămân.
   *[other] { $count } de legături indică documente pe care importul nu le-a preluat. Cuvintele lor rămân.
}
import-scrivener-prose-linked-picture-not-read-line = { $count ->
    [1] O imagine legată de un fișier de pe disc, „{ $example }”, nu a fost preluată.
    [few] { $count } imagini legate de fișiere de pe disc, de exemplu „{ $example }”, nu au fost preluate.
   *[other] { $count } de imagini legate de fișiere de pe disc, de exemplu „{ $example }”, nu au fost preluate.
}
import-scrivener-prose-heading-repeats-title-line = { $count ->
    [one] Un text începe cu un titlu de secțiune care îi repetă titlul, așa că un export ar putea tipări titlul de două ori.
    [few] { $count } texte încep cu un titlu de secțiune care le repetă titlul, așa că un export ar putea tipări titlul de două ori.
   *[other] { $count } de texte încep cu un titlu de secțiune care le repetă titlul, așa că un export ar putea tipări titlul de două ori.
}
import-scrivener-prose-markdown-as-text-line = { $count ->
    [one] Un paragraf scris în Markdown este păstrat ca text, așa cum este.
    [few] { $count } paragrafe scrise în Markdown sunt păstrate ca text, așa cum sunt.
   *[other] { $count } de paragrafe scrise în Markdown sunt păstrate ca text, așa cum sunt.
}
import-scrivener-prose-citation-keys-as-text-line = { $count ->
    [one] Un paragraf conține chei de citare pentru un program de bibliografie, păstrate ca text.
    [few] { $count } paragrafe conțin chei de citare pentru un program de bibliografie, păstrate ca text.
   *[other] { $count } de paragrafe conțin chei de citare pentru un program de bibliografie, păstrate ca text.
}
import-scrivener-prose-placeholders-in-exported-text-line = { $count ->
    [1] Un marcaj de înlocuire Scrivener, { $example }, este păstrat ca text simplu în ce exportați. Înlocuiți-l manual.
    [few] { $count } marcaje de înlocuire Scrivener, de exemplu { $example }, sunt păstrate ca text simplu în ce exportați. Înlocuiți-le manual.
   *[other] { $count } de marcaje de înlocuire Scrivener, de exemplu { $example }, sunt păstrate ca text simplu în ce exportați. Înlocuiți-le manual.
}
import-scrivener-prose-placeholders-in-other-text-line = { $count ->
    [1] Un marcaj de înlocuire Scrivener, { $example }, este păstrat ca text simplu într-un text care nu este exportat.
    [few] { $count } marcaje de înlocuire Scrivener, de exemplu { $example }, sunt păstrate ca text simplu în texte care nu sunt exportate.
   *[other] { $count } de marcaje de înlocuire Scrivener, de exemplu { $example }, sunt păstrate ca text simplu în texte care nu sunt exportate.
}
import-scrivener-prose-ornament-lines-kept-line = { $count ->
    [one] Un rând centrat de ornamente poate marca un separator de scenă și este păstrat așa cum a fost scris.
    [few] { $count } rânduri centrate de ornamente pot marca separatoare de scenă și sunt păstrate așa cum au fost scrise.
   *[other] { $count } de rânduri centrate de ornamente pot marca separatoare de scenă și sunt păstrate așa cum au fost scrise.
}
import-scrivener-prose-long-document-line = { $count ->
    [one] Un text este foarte lung și se poate deschide lent.
    [few] { $count } texte sunt foarte lungi și se pot deschide lent.
   *[other] { $count } de texte sunt foarte lungi și se pot deschide lent.
}
import-scrivener-prose-pictures-without-description-line = { $count ->
    [one] O imagine nu are o descriere pentru cititorii care nu o pot vedea. Puteți adăuga una în editor.
    [few] { $count } imagini nu au o descriere pentru cititorii care nu le pot vedea. Puteți adăuga câte una în editor.
   *[other] { $count } de imagini nu au o descriere pentru cititorii care nu le pot vedea. Puteți adăuga câte una în editor.
}
import-scrivener-prose-struck-text-compiled-away-line = { $count ->
    [one] Pasajele tăiate dintr-un text sunt păstrate. Compilarea Scrivener le excludea din carte, dar un export din Skribisto le tipărește.
    [few] Pasajele tăiate din { $count } texte sunt păstrate. Compilarea Scrivener le excludea din carte, dar un export din Skribisto le tipărește.
   *[other] Pasajele tăiate din { $count } de texte sunt păstrate. Compilarea Scrivener le excludea din carte, dar un export din Skribisto le tipărește.
}
import-scrivener-prose-compile-deleted-style-printed-line = { $count ->
    [1] Textul cu stilul „{ $example }”, pe care compilarea Scrivener îl ștergea, este păstrat ca proză, iar un export din Skribisto îl tipărește.
    [few] Textul formatat cu unul dintre cele { $count } stiluri pe care compilarea Scrivener le ștergea, de exemplu „{ $example }”, este păstrat ca proză, iar un export din Skribisto îl tipărește.
   *[other] Textul formatat cu unul dintre cele { $count } de stiluri pe care compilarea Scrivener le ștergea, de exemplu „{ $example }”, este păstrat ca proză, iar un export din Skribisto îl tipărește.
}
import-scrivener-prose-compile-raw-style-printed-line = { $count ->
    [1] Textul cu stilul „{ $example }”, pe care compilarea Scrivener îl transmitea drept cod pentru un alt program, este păstrat ca proză, iar un export din Skribisto îl tipărește.
    [few] Textul formatat cu unul dintre cele { $count } stiluri pe care compilarea Scrivener le transmitea drept cod pentru un alt program, de exemplu „{ $example }”, este păstrat ca proză, iar un export din Skribisto îl tipărește.
   *[other] Textul formatat cu unul dintre cele { $count } de stiluri pe care compilarea Scrivener le transmitea drept cod pentru un alt program, de exemplu „{ $example }”, este păstrat ca proză, iar un export din Skribisto îl tipărește.
}
import-scrivener-prose-left-out-after-document-line = { $count ->
    [one] Un fragment dintr-un fișier deteriorat, aflat după sfârșitul acestuia, este omis.
    [few] { $count } fragmente din fișiere deteriorate, aflate după sfârșitul lor, sunt omise.
   *[other] { $count } de fragmente din fișiere deteriorate, aflate după sfârșitul lor, sunt omise.
}
import-scrivener-prose-left-out-header-or-footer-line = { $count ->
    [one] Un antet sau un subsol este omis: nu face parte din text.
    [few] { $count } antete și subsoluri sunt omise: nu fac parte din text.
   *[other] { $count } de antete și subsoluri sunt omise: nu fac parte din text.
}
import-scrivener-prose-left-out-list-label-picture-line = { $count ->
    [one] O imagine folosită ca marcator de listă este omisă.
    [few] { $count } imagini folosite ca marcatori de listă sunt omise.
   *[other] { $count } de imagini folosite ca marcatori de listă sunt omise.
}
import-scrivener-prose-left-out-line = { $count ->
    [one] Un fragment de fișier care nu aparține niciunui text este omis.
    [few] { $count } fragmente de fișiere care nu aparțin niciunui text sunt omise.
   *[other] { $count } de fragmente de fișiere care nu aparțin niciunui text sunt omise.
}
import-scrivener-prose-undecodable-characters-line = { $count ->
    [one] Un caracter nu a putut fi decodat și apare ca semn de înlocuire (�) acolo unde se afla.
    [few] { $count } caractere nu au putut fi decodate și apar ca semne de înlocuire (�) acolo unde se aflau.
   *[other] { $count } de caractere nu au putut fi decodate și apar ca semne de înlocuire (�) acolo unde se aflau.
}
import-scrivener-prose-inline-notes-merged-line = { $count ->
    [one] O notă în text pe care Scrivener o împărțise acolo unde i se schimba formatarea este reunită.
    [few] { $count } note în text pe care Scrivener le împărțise acolo unde li se schimba formatarea sunt reunite.
   *[other] { $count } de note în text pe care Scrivener le împărțise acolo unde li se schimba formatarea sunt reunite.
}
import-scrivener-prose-inline-notes-joined-across-paragraphs-line = { $count ->
    [1] O notă în text scrisă paragraf cu paragraf este reunită într-o singură notă.
    [few] { $count } note în text scrise paragraf cu paragraf sunt reunite în note de mai multe paragrafe.
   *[other] { $count } de note în text scrise paragraf cu paragraf sunt reunite în note de mai multe paragrafe.
}
import-scrivener-prose-blank-notes-dropped-line = { $count ->
    [one] O notă goală în text este omisă.
    [few] { $count } note goale în text sunt omise.
   *[other] { $count } de note goale în text sunt omise.
}
import-scrivener-prose-annotations-became-comments-line = { $count ->
    [one] O adnotare în text a devenit comentariu.
    [few] { $count } adnotări în text au devenit comentarii.
   *[other] { $count } de adnotări în text au devenit comentarii.
}
import-scrivener-prose-annotation-only-document-kept-line = { $count ->
    [one] Un document nu conținea decât o adnotare. Cuvintele ei sunt păstrate ca proză.
    [few] { $count } documente nu conțineau decât o adnotare. Cuvintele lor sunt păstrate ca proză.
   *[other] { $count } de documente nu conțineau decât o adnotare. Cuvintele lor sunt păstrate ca proză.
}
import-scrivener-prose-preserve-formatting-passages-line = { $count ->
    [one] Un pasaj marcat cu „Preserve Formatting” (păstrează formatarea) ajunge ca text obișnuit.
    [few] { $count } pasaje marcate cu „Preserve Formatting” (păstrează formatarea) ajung ca text obișnuit.
   *[other] { $count } de pasaje marcate cu „Preserve Formatting” (păstrează formatarea) ajung ca text obișnuit.
}
import-scrivener-prose-nested-note-unwrapped-line = { $count ->
    [one] O notă aflată într-un comentariu sau într-o notă de subsol este păstrată ca text, acolo unde se afla.
    [few] { $count } note aflate în comentarii sau în note de subsol sunt păstrate ca text, acolo unde se aflau.
   *[other] { $count } de note aflate în comentarii sau în note de subsol sunt păstrate ca text, acolo unde se aflau.
}
import-scrivener-prose-style-not-carried-line = { $count ->
    [1] Stilul „{ $example }” nu are echivalent în Skribisto. Textul lui își păstrează cuvintele.
    [few] { $count } stiluri, de exemplu „{ $example }”, nu au echivalent în Skribisto. Textul lor își păstrează cuvintele.
   *[other] { $count } de stiluri, de exemplu „{ $example }”, nu au echivalent în Skribisto. Textul lor își păstrează cuvintele.
}
import-scrivener-prose-document-links-dropped-line = { $count ->
    [one] O legătură către alt document nu are loc în Skribisto. Cuvintele ei rămân.
    [few] { $count } legături către alte documente nu au loc în Skribisto. Cuvintele lor rămân.
   *[other] { $count } de legături către alte documente nu au loc în Skribisto. Cuvintele lor rămân.
}
import-scrivener-prose-document-links-kept-line = { $count ->
    [one] O legătură către alt document este păstrată ca trimitere.
    [few] { $count } legături către alte documente sunt păstrate ca trimiteri.
   *[other] { $count } de legături către alte documente sunt păstrate ca trimiteri.
}
import-scrivener-prose-other-links-kept-as-text-line = { $count ->
    [one] O legătură către alt proiect sau către un fișier este păstrată ca text.
    [few] { $count } legături către alte proiecte sau către fișiere sunt păstrate ca text.
   *[other] { $count } de legături către alte proiecte sau către fișiere sunt păstrate ca text.
}
import-scrivener-prose-typed-indents-removed-line = { $count ->
    [one] O indentare tastată la începutul unui paragraf este eliminată.
    [few] { $count } indentări tastate la începutul paragrafelor sunt eliminate.
   *[other] { $count } de indentări tastate la începutul paragrafelor sunt eliminate.
}
import-scrivener-prose-left-indents-not-kept-line = { $count ->
    [one] Un paragraf indentat față de margine își pierde indentarea.
    [few] { $count } paragrafe indentate față de margine își pierd indentarea.
   *[other] { $count } de paragrafe indentate față de margine își pierd indentarea.
}
import-scrivener-prose-blank-lines-removed-line = { $count ->
    [one] Un paragraf gol aflat între alte două este eliminat.
    [few] { $count } paragrafe goale aflate între altele sunt eliminate.
   *[other] { $count } de paragrafe goale aflate între altele sunt eliminate.
}
import-scrivener-prose-blank-lines-became-scene-breaks-line = { $count ->
    [one] Un rând gol dintre paragrafe a devenit separator de scenă.
    [few] { $count } rânduri goale dintre paragrafe au devenit separatoare de scenă.
   *[other] { $count } de rânduri goale dintre paragrafe au devenit separatoare de scenă.
}
import-scrivener-prose-ornaments-became-scene-breaks-line = { $count ->
    [one] Un rând de ornamente a devenit separator de scenă.
    [few] { $count } rânduri de ornamente au devenit separatoare de scenă.
   *[other] { $count } de rânduri de ornamente au devenit separatoare de scenă.
}
import-scrivener-prose-table-first-row-header-line = { $count ->
    [one] Un tabel începe acum cu un rând de antet.
    [few] { $count } tabele încep acum cu un rând de antet.
   *[other] { $count } de tabele încep acum cu un rând de antet.
}
import-scrivener-prose-table-cells-joined-line = { $count ->
    [one] O celulă de tabel conținea mai multe paragrafe, acum reunite într-unul singur.
    [few] { $count } celule de tabel conțineau mai multe paragrafe, acum reunite într-unul singur.
   *[other] { $count } de celule de tabel conțineau mai multe paragrafe, acum reunite într-unul singur.
}
import-scrivener-prose-formatting-lost-colour-line = { $count ->
    [one] Culoarea textului este eliminată într-un text.
    [few] Culoarea textului este eliminată în { $count } texte.
   *[other] Culoarea textului este eliminată în { $count } de texte.
}
import-scrivener-prose-formatting-lost-highlight-line = { $count ->
    [one] Evidențierea este eliminată într-un text.
    [few] Evidențierea este eliminată în { $count } texte.
   *[other] Evidențierea este eliminată în { $count } de texte.
}
import-scrivener-prose-formatting-lost-small-caps-line = { $count ->
    [one] Majusculele mici apar ca litere obișnuite într-un text.
    [few] Majusculele mici apar ca litere obișnuite în { $count } texte.
   *[other] Majusculele mici apar ca litere obișnuite în { $count } de texte.
}
import-scrivener-prose-formatting-lost-all-caps-line = { $count ->
    [one] Textul afișat cu majuscule apare așa cum a fost tastat într-un text.
    [few] Textul afișat cu majuscule apare așa cum a fost tastat în { $count } texte.
   *[other] Textul afișat cu majuscule apare așa cum a fost tastat în { $count } de texte.
}
import-scrivener-prose-formatting-lost-hidden-line = { $count ->
    [one] Textul ascuns este omis într-un text.
    [few] Textul ascuns este omis în { $count } texte.
   *[other] Textul ascuns este omis în { $count } de texte.
}
import-scrivener-prose-formatting-lost-line = { $count ->
    [one] Formatarea pentru care Skribisto nu are loc este eliminată într-un text.
    [few] Formatarea pentru care Skribisto nu are loc este eliminată în { $count } texte.
   *[other] Formatarea pentru care Skribisto nu are loc este eliminată în { $count } de texte.
}
import-scrivener-prose-justified-text-not-kept-line = { $count ->
    [one] Un paragraf aliniat stânga-dreapta urmează în schimb stilul exportului.
    [few] { $count } paragrafe aliniate stânga-dreapta urmează în schimb stilul exportului.
   *[other] { $count } de paragrafe aliniate stânga-dreapta urmează în schimb stilul exportului.
}
import-scrivener-prose-comment-colours-lost-line = { $count ->
    [one] Un comentariu își pierde culoarea.
    [few] { $count } comentarii își pierd culoarea.
   *[other] { $count } de comentarii își pierd culoarea.
}
import-scrivener-prose-control-characters-removed-line = { $count ->
    [one] Un caracter de control invizibil este eliminat.
    [few] { $count } caractere de control invizibile sunt eliminate.
   *[other] { $count } de caractere de control invizibile sunt eliminate.
}
import-scrivener-prose-note-on-heading-lost-line = { $count ->
    [one] O notă de subsol sau un comentariu de pe titlul unui capitol nu a putut rămâne acolo: un comentariu este mutat la primul paragraf al capitolului, o notă de subsol este omisă.
    [few] { $count } note de subsol sau comentarii de pe titlurile capitolelor nu au putut rămâne acolo: un comentariu este mutat la primul paragraf al capitolului, o notă de subsol este omisă.
   *[other] { $count } de note de subsol sau comentarii de pe titlurile capitolelor nu au putut rămâne acolo: un comentariu este mutat la primul paragraf al capitolului, o notă de subsol este omisă.
}
import-scrivener-prose-media-file-copied-line = { $count ->
    [one] Un fișier de documentare pe care Skribisto nu îl poate deschide a fost copiat lângă noul proiect. Nota lui spune unde.
    [few] { $count } fișiere de documentare pe care Skribisto nu le poate deschide au fost copiate lângă noul proiect. Notele lor spun unde.
   *[other] { $count } de fișiere de documentare pe care Skribisto nu le poate deschide au fost copiate lângă noul proiect. Notele lor spun unde.
}
import-scrivener-prose-media-file-not-carried-line = { $count ->
    [one] Un fișier de documentare pe care Skribisto nu îl poate deschide rămâne în proiectul dumneavoastră Scrivener. Nota lui îl numește.
    [few] { $count } fișiere de documentare pe care Skribisto nu le poate deschide rămân în proiectul dumneavoastră Scrivener. Notele lor le numesc.
   *[other] { $count } de fișiere de documentare pe care Skribisto nu le poate deschide rămân în proiectul dumneavoastră Scrivener. Notele lor le numesc.
}

## When the import fails
import-scrivener-error-refused = { $refusal } Nu s-a scris nimic.
import-scrivener-error-question = Proiectul pune acum o întrebare la care verificarea nu a răspuns. Alegeți „Reverifică importul”, apoi importați. Nu s-a scris nimic.
import-scrivener-error-not-accepted = Înainte de a putea fi importat, proiectul vă cere să bifați o casetă la pasul Proiect, iar caseta nu a fost bifată. Alegeți „Reverifică importul”, bifați-o, apoi importați. Nu s-a scris nimic.
import-scrivener-error-changed = Proiectul s-a modificat după ce l-ați verificat. Alegeți „Reverifică importul”, apoi importați. Nu s-a scris nimic.
import-scrivener-error-choice = Structura aleasă la verificare nu se mai potrivește acestui proiect. Alegeți „Reverifică importul”, apoi importați. Nu s-a scris nimic.

## Export dialog
export-title = Export
export-close = Închide
export-scope-label = Ce
export-format-label = Format
export-style-label = Stil
export-path-label = Fișier
export-browse = Răsfoiește…
export-show-non-exportable = Afișează și elementele neexportabile
export-choose-empty = Niciun proiect din care să alegeți.
export-cancel = Renunță
export-export = Exportă
export-save-dialog-title = Exportă într-un fișier
# Left-pane section headers (normal case) + scope segmented control
export-section-what = Ce se exportă
export-section-style = Stil
export-section-destination = Destinație
export-custom-selection = Selecție personalizată
export-selected-count = Selectate: { $count }
# Style-preset summary chips
export-chip-chapters-none = Capitole: fără titlu
export-chip-chapters-numbered = Capitole: numerotate
export-chip-chapters-title = Capitole: doar titlul
export-chip-chapters-both = Capitole: număr + titlu
export-chip-scene-break-glyph = Separator de scenă: { $glyph }
export-chip-scene-break-blank = Separator de scenă: rând gol
export-chip-scene-break-none = Separator de scenă: niciunul
export-chip-major-break-glyph = Separator major: { $glyph }
export-chip-major-break-blank = Separator major: rând gol
export-chip-major-break-none = Separator major: niciunul
export-chip-major-break-same = Ambele niveluri de separator identice
export-chip-spacing-single = Spațiere: simplă
export-chip-spacing-onehalf = Spațiere: 1½
export-chip-spacing-double = Spațiere: dublă
export-chip-notes-included = Note incluse
export-chip-notes-excluded = Note excluse
# Output formats
export-format-docx = Word
export-format-odt = LibreOffice
export-format-html = HTML
export-format-markdown = Markdown
export-format-djot = Djot
export-format-text = Text
export-format-latex = LaTeX
export-format-epub = EPUB
export-format-pdf = PDF
# Overwrite confirmation
export-overwrite-title = Înlocuiți fișierul existent?
export-overwrite-text = „{ $name }” există deja. Îl înlocuiți?
# Progress toast (the export runs as a long operation)
export-progress-title = Se exportă…
export-cancelled = Export anulat
export-done = { $count ->
       [one] A fost exportat { $count } element
       [few] Au fost exportate { $count } elemente
      *[other] Au fost exportate { $count } de elemente
   }
export-open-file = Deschide
export-show-in-folder = Arată în dosar
# Error toast: a short reason in the body, the full technical chain behind Details
export-error-title = Exportul nu a reușit
export-error-details = Detalii

## Backup scheduler (progress toast + failure/prune-warning details, backup review, T1-2/T1-7/T2-3/T2-8/T2-9)
backup-progress-start = Se pornește…
backup-progress-retention = Se elimină copiile de siguranță vechi…
backup-progress-done = Gata
backup-progress-destination = Destinația { $i } din { $n }
backup-details = Detalii
backup-issues-title = Probleme cu copiile de siguranță
backup-failed-title = Copia de siguranță a eșuat
backup-complete-prune-warning = Copie de siguranță încheiată (salvate: { $ok }, deja la zi: { $skipped }). Unele copii de siguranță vechi nu au putut fi eliminate

# Search & Replace
search = Căutare
search-query-placeholder = Caută…
search-replace-placeholder = Înlocuiește cu…
search-replace-toggle = Afișează sau ascunde înlocuirea
search-replace-all = Înlocuiește tot
search-opt-case = Potrivește majusculele
search-opt-whole-word = Cuvânt întreg
search-opt-diacritics = Potrivește diacriticele
search-scope-body = Text
search-scope-title = Titlu
search-scope-synopsis = Sinopsis
search-scope-label = Legendă
search-facet-book = Cărți
search-facet-part = Părți
search-facet-chapter = Capitole
search-facet-scene = Scene
search-facet-note = Note
search-facet-folder = Dosare
# Rich tooltips for the option toggles
search-tip-case = Potrivește majusculele: majusculele și minusculele sunt tratate diferit, așa că „Elena” și „elena” sunt potriviri separate.
search-tip-whole-word = Cuvânt întreg: se potrivesc doar cuvintele complete, așa că „cap” nu este găsit în „capitol”.
search-tip-diacritics = Potrivește diacriticele: literele cu diacritice sunt tratate ca distincte, așa că „sa” nu se potrivește cu „să”.
search-tip-body = Text: caută în proza scenelor și a notelor.
search-tip-title = Titlu: caută în titlurile elementelor din clasor.
search-tip-synopsis = Sinopsis: caută în rezumatul fiecărui element de scris.
search-tip-label = Legendă: caută în nota scurtă afișată sub titlul unui element din clasor.
search-tip-comment = Comentarii: caută în textul firelor de comentarii și al răspunsurilor. Un comentariu este despre manuscris, nu parte din el, așa că are propriul comutator, iar la înlocuire potrivirile din comentarii rămân nebifate până când le bifați.
search-tip-book = Cărți: containerul cărții și marcajele sale de început și de sfârșit.
search-tip-part = Părți: separatoarele la nivel de parte.
search-tip-chapter = Capitole: capitolele, în orice formă le stochează proiectul.
search-tip-scene = Scene: elementele care conțin proza dumneavoastră.
search-tip-note = Note: note libere.
search-tip-folder = Dosare: dosare simple de organizare și separatoare.
search-tip-paratext = Un text care aparține cărții, dar nu poveștii ei: o prefață, o dedicație, o postfață. Nu este numărat niciodată în manuscris.
search-tip-preserve-case = Păstrează majusculele: înlocuirea preia forma literelor găsite, așa că „ELENA” devine „MARTA”, iar „Elena” devine „Marta”.
search-tip-replace = Înlocuire: afișează câmpul de înlocuire și „Înlocuiește tot”.
search-error = Căutarea a eșuat: { $message }
search-no-matches = Nicio potrivire
search-count =
       { $matches ->
           [one] { $matches } potrivire
           [few] { $matches } potriviri
          *[other] { $matches } de potriviri
       } în { $items ->
           [one] { $items } document
           [few] { $items } documente
          *[other] { $items } de documente
       }
search-count-truncated =
       { $matches ->
           [one] { $matches } potrivire
           [few] { $matches } potriviri
          *[other] { $matches } de potriviri
       } în { $items ->
           [one] { $items } document
           [few] { $items } documente
          *[other] { $items } de documente
       } (doar primele rezultate)
search-occurrences = { $count }
search-field-body = Text
search-field-title = Titlu
search-field-synopsis = Sinopsis
search-field-label = Legendă
search-field-epigraph = Motto
search-field-comment = Comentariu
search-field-comment-reply = Răspuns
search-field-footnote = Notă de subsol
search-include-in-replace = Include în „Înlocuiește tot”
search-collapse-all = Restrânge toate rezultatele
search-replace-here = Înlocuiește aici
search-dismiss = Scoate din rezultate
search-undo-dismiss = Readu ultimul rezultat scos
search-replace-nothing = (nimic)
search-replace-confirm-title = Înlocuiți toate potrivirile?
search-replace-confirm-text =
       Înlocuiți { $occurrences ->
           [one] { $occurrences } apariție
           [few] { $occurrences } apariții
          *[other] { $occurrences } de apariții
       } ale lui „{ $query }” cu „{ $replacement }” în { $items ->
           [one] { $items } document
           [few] { $items } documente
          *[other] { $items } de documente
       }? Puteți anula ulterior din notificare.
search-replace-done-title = Înlocuire încheiată
search-replace-done =
       { $occurrences ->
           [one] { $occurrences } apariție înlocuită
           [few] { $occurrences } apariții înlocuite
          *[other] { $occurrences } de apariții înlocuite
       } în { $items ->
           [one] { $items } document
           [few] { $items } documente
          *[other] { $items } de documente
       }.
search-replace-done-skipped =
       { $occurrences ->
           [one] { $occurrences } apariție înlocuită
           [few] { $occurrences } apariții înlocuite
          *[other] { $occurrences } de apariții înlocuite
       } în { $items ->
           [one] { $items } document
           [few] { $items } documente
          *[other] { $items } de documente
       }. { $skipped ->
           [one] { $skipped } câmp omis
           [few] { $skipped } câmpuri omise
          *[other] { $skipped } de câmpuri omise
       } (modificate de la căutare).
search-replace-undo = Anulează
search-replace-failed-title = Înlocuirea a eșuat
search-replace-skipped-title = Nu s-a înlocuit nimic
search-replace-skipped-body =
       { $fields ->
           [one] Acest text s-a modificat de la căutare, așa că a fost lăsat neatins.
           [few] { $fields } dintre aceste texte s-au modificat de la căutare, așa că au fost lăsate neatinse.
          *[other] { $fields } dintre aceste texte s-au modificat de la căutare, așa că au fost lăsate neatinse.
       } Căutați din nou pentru a vedea unde se află acum cuvintele.
search-replace-undo-failed-title = Anularea a eșuat
search-preview = Previzualizare
search-preview-empty = Selectați un rezultat pentru a-l previzualiza aici
search-preview-no-prose = Această potrivire nu are text editabil
search-preview-prompt = Pentru a vedea aici o previzualizare, faceți o căutare.
search-preview-open-search = Caută în proiect
search-preview-footnote-prompt = Această potrivire se află în textul unei note de subsol. Deschideți-o în panoul Note de subsol pentru a o vedea și edita.
search-preview-open-footnotes = Deschide notele de subsol

# Per-editor find banner (Ctrl+F)
find-placeholder = Caută în acest document…
find-count = { $current } din { $total }
find-no-results = Niciun rezultat
find-close = Închide căutarea
find-previous = Potrivirea precedentă (Shift+Enter)
find-next = Potrivirea următoare (Enter)
find-opt-case = Potrivește majusculele
find-opt-whole-word = Cuvânt întreg
find-replace-toggle = Afișează sau ascunde înlocuirea (Ctrl+R)
find-replace-placeholder = Înlocuiește cu…
find-replace = Înlocuiește
find-replace-all = Înlocuiește tot
find-preserve-case = Păstrează majusculele

## Dictionaries & spell-checking
# Download toasts (app-local download, not a backend long operation)
dict-download-title = Se descarcă { $name }…
dict-download-done = { $name } a fost instalat
dict-download-failed = { $name } nu a putut fi descărcat: { $error }
dict-removed = { $name } a fost eliminat
dict-accept-first = Acceptați licența înainte de a descărca { $name }
# Settings ▸ Dictionaries pane
settings-dict-tab-installed = Instalate
settings-dict-tab-get-more = Obține mai multe
settings-dict-tab-personal = Cuvinte personale
dict-installed-empty = Încă nu s-au găsit dicționare pe acest calculator.
dict-get-more-search = Caută limbi
dict-system-badge = în sistem
dict-unusable-badge = inutilizabil
dict-download-button = Descarcă
dict-downloading = Se descarcă…
dict-installed-label = Instalat
dict-remove = Elimină
dict-view-license = Vezi licența
dict-approx-size = ~{ $size }
dict-personal-empty = Încă nu există cuvinte personale în acest proiect.
dict-personal-add = Adaugă
dict-personal-placeholder = Adaugă un cuvânt…
# Missing-dictionary prompt after opening a project
dict-missing-toast = { $count ->
       [one] Acest proiect folosește { $count } dicționar pe care nu îl aveți instalat
       [few] Acest proiect folosește { $count } dicționare pe care nu le aveți instalate
      *[other] Acest proiect folosește { $count } de dicționare pe care nu le aveți instalate
   }
dict-missing-action = Obține dicționarele
# License modal
dict-license-title = Licența { $name }
dict-license-accept = Acceptă și descarcă
dict-license-cancel = Renunță
dict-license-close = Închide
# Add a dictionary (sideload local .aff/.dic files)
dict-add-button = Adaugă un dicționar…
dict-add-title = Adăugare dicționar
dict-add-close = Închide
dict-add-name = Nume
dict-add-name-placeholder = de ex. Dicționarul meu de latină
dict-add-code = Codul limbii
dict-add-code-placeholder = de ex. la sau fr-FR-x-custom
dict-add-code-hint = O etichetă scurtă, aleasă de dumneavoastră. Pe aceasta o veți alege ca limbă a unui document.
dict-add-aff = Fișier de afixe (.aff)
dict-add-dic = Listă de cuvinte (.dic)
dict-add-submit = Adaugă
dict-add-cancel = Renunță
dict-add-name-required = Dați un nume dicționarului.
dict-add-code-required = Introduceți un cod de limbă.
dict-add-code-invalid = Folosiți doar litere, cifre și - _ .
dict-add-code-reserved = Acesta este codul unui dicționar inclus. Alegeți altul.
dict-add-file-required = Alegeți un fișier.
dict-add-file-missing = Acest fișier nu există.
dict-add-code-taken = Un dicționar cu acest cod este deja instalat.
dict-add-done = { $name } a fost adăugat
dict-add-unusable = Aceste fișiere nu formează un dicționar utilizabil: { $error }
dict-add-failed = Dicționarul nu a putut fi adăugat: { $error }
# The Inspector's per-item export controls (M3)
inspector-export = Export
inspector-exportable = Include în exporturi
ctx-number = &Numerotează acest capitol
ctx-unnumber = &Nu numerota acest capitol
inspector-numbering = Numerotare
inspector-numbered = Numerotat
inspector-numbered-tip = Acest capitol își ocupă locul în numerotarea cărții. Dezactivați opțiunea pentru un prolog, un epilog sau un interludiu.
inspector-numbered-tip-more = Un capitol nenumerotat rămâne în carte exact cum era: titlul, proza și numărul lui de cuvinte nu sunt atinse. Pur și simplu nu tipărește niciun număr și nici nu consumă unul: capitolul de după un prolog este capitolul unu, nu capitolul doi. Excluderea lui din export este un alt comutator, mai sus, iar acela scoate cu totul capitolul din carte.
inspector-apply-to-children = Aplică și elementelor conținute
# The Inspector's per-Part/Chapter milestone date (M5), shown on the Book's Pace.
inspector-milestone = Data jalonului
inspector-milestone-none = Nicio dată
inspector-milestone-clear = Golește
# The language pill field (Inspector + Settings)
inspector-tags = Etichete
inspector-aliases = Cunoscut(ă) și ca
inspector-dict-language = Limbă
inspector-apply-language-to-children = Aplică limba și elementelor conținute
settings-page-language = Limbă
settings-field-dict-language = Limbi
dict-tradeoff-hint = Fiecare limbă în plus acceptă mai multe cuvinte, deci sunt prinse mai puține greșeli.
lang-inherit-hint = Moștenit: această scenă folosește limbile cărții sau ale proiectului.
lang-pill-list = Limbi
lang-pill-add = Adaugă o limbă
lang-pill-remove = Elimină { $name }
lang-pill-mute = Oprește verificarea ortografică pentru { $name }
lang-pill-unmute = Pornește verificarea ortografică pentru { $name }

## Settings: Work ▸ Personal dictionary (per-project word list)
settings-page-personal-dictionary = Dicționar personal
settings-user-dict-desc = Cuvintele pe care le adăugați aici sunt considerate corecte peste tot în aplicație și nu mai sunt semnalate niciodată.
settings-user-dict-add-placeholder = Adaugă un cuvânt…
settings-user-dict-add = Adaugă cuvântul
settings-user-dict-search = Filtrează cuvintele
settings-user-dict-count = { $count ->
       [one] { $count } cuvânt
       [few] { $count } cuvinte
      *[other] { $count } de cuvinte
   }
settings-user-dict-rename = Redenumește
settings-user-dict-remove = Elimină
settings-user-dict-empty = Încă nu există cuvinte. Adăugați cuvintele pe care verificarea ortografică trebuie să le ignore sau importați o listă.
settings-user-dict-import = Importă…
settings-user-dict-export = Exportă…
settings-user-dict-import-tip = Adaugă cuvinte dintr-un fișier text simplu, câte un cuvânt pe rând. Cuvintele existente sunt păstrate.
settings-user-dict-export-tip = Salvează toată lista într-un fișier text simplu, câte un cuvânt pe rând.
settings-user-dict-txt-filter = Fișiere text
settings-user-dict-duplicate = Există deja în dicționar
settings-user-dict-imported = { $count ->
       [one] A fost importat { $count } cuvânt
       [few] Au fost importate { $count } cuvinte
      *[other] Au fost importate { $count } de cuvinte
   } (existau deja: { $duplicates }).
settings-user-dict-import-failed = Lista de cuvinte nu a putut fi citită: { $error }
settings-user-dict-exported = { $count ->
       [one] A fost salvat { $count } cuvânt.
       [few] Au fost salvate { $count } cuvinte.
      *[other] Au fost salvate { $count } de cuvinte.
   }
settings-user-dict-export-failed = Lista de cuvinte nu a putut fi salvată: { $error }

## Settings: Work ▸ Text replacements (per-project custom lexicon)
settings-page-text-replacements = Înlocuiri de text
settings-text-repl-desc = Înlocuiește o prescurtare cu textul complet în timp ce tastați: „aprx” devine „aproximativ” în clipa în care tastați un spațiu sau un semn de punctuație.
settings-text-repl-enable = Folosește înlocuirile de text în acest proiect
settings-text-repl-disabled-hint = Activați opțiunea pentru a defini prescurtări care se extind în timp ce scrieți.
settings-text-repl-add = Adaugă o regulă
settings-text-repl-trigger-placeholder = Prescurtare
settings-text-repl-replacement-placeholder = Ce devine
settings-text-repl-added = A fost adăugată „{ $trigger }”
settings-text-repl-duplicate = „{ $trigger }” are deja o regulă
settings-text-repl-filter = Filtrează regulile
settings-text-repl-count = { $n ->
       [one] O regulă
       [few] { $n } reguli
      *[other] { $n } de reguli
   }
settings-text-repl-row-enabled = Folosește această regulă
settings-text-repl-delete = Șterge regula pentru { $trigger }
settings-text-repl-deleted = Regula pentru „{ $trigger }” a fost ștearsă
settings-text-repl-empty = Încă nu există reguli.
settings-text-repl-csv-filter = Fișiere CSV
settings-text-repl-import = Importă…
settings-text-repl-export = Exportă…
settings-text-repl-imported = Importate: { $added }, omise: { $skipped }
settings-text-repl-exported = { $n ->
       [one] A fost exportată o regulă
       [few] Au fost exportate { $n } reguli
      *[other] Au fost exportate { $n } de reguli
   }

## Editor: spelling (context menu + toast)
# Shown in place of the corrections when a flagged word has none to offer.
editor-menu-no-suggestions = Nicio sugestie
editor-menu-add-to-dictionary = Adaugă „{ $word }” în dicționar
editor-menu-add-words-to-dictionary = Adaugă cuvintele selectate în dicționar
editor-dict-added = „{ $word }” a fost adăugat în dicționarul dumneavoastră.
editor-dict-added-multi = { $count ->
       [one] A fost adăugat { $count } cuvânt în dicționarul dumneavoastră.
       [few] Au fost adăugate { $count } cuvinte în dicționarul dumneavoastră.
      *[other] Au fost adăugate { $count } de cuvinte în dicționarul dumneavoastră.
   }
toast-undo = Anulează

## Taking one operation back
##
## The Undo on a notification is offered for one specific thing. If anything
## else has happened since, it says so rather than taking back whatever now
## happens to be last.
undo-superseded-title = Acel pas nu mai este ultimul
undo-superseded-body = Între timp s-a schimbat altceva în acest proiect. Anularea ar fi anulat acel lucru în locul pasului, așa că nu s-a făcut. Nimic din ce a făcut pasul nu a fost inversat. Folosiți Editare ▸ Anulează pentru a merge singur(ă) înapoi prin istoric.
undo-failed = Anularea a eșuat: { $error }

## The Edit menu
##
## The Undo row names what it will take back, because on a stack where prose and
## structure both appear a bare "Undo" leaves the writer guessing whether the
## next press retypes a word or resurrects a chapter.
menu-edit = &Editare
menu-edit-undo = A&nulează
menu-edit-redo = &Refă
menu-edit-undo-target = A&nulează { $target }
menu-edit-redo-target = &Refă { $target }
shortcut-name-edit-undo = Anulează
shortcut-name-edit-redo = Refă
undo-target-typing = tastarea
undo-target-project = ultima modificare din acest proiect
prose-history-reset-title = Istoricul tastării a fost resetat
prose-history-reset-body = { $count ->
       [one] O scenă deschisă a fost readusă la o stare anterioară, așa că istoricul ei de tastare nu se mai aplică.
       [few] { $count } scene deschise au fost readuse la o stare anterioară, așa că istoricul lor de tastare nu se mai aplică.
      *[other] { $count } de scene deschise au fost readuse la o stare anterioară, așa că istoricul lor de tastare nu se mai aplică.
   }
undo-target-trash = mutarea la coșul de gunoi
undo-target-restore = restaurarea din coșul de gunoi
undo-target-delete-forever = ștergerea definitivă
undo-target-replace-all = înlocuirea în tot proiectul
undo-target-import = importul de documente
undo-target-duplicate = duplicarea
undo-target-move = mutarea
undo-target-merge = unirea a două scene
undo-target-split = împărțirea unei scene
undo-target-promote = schimbarea tipului
undo-target-tidy-titles = curățarea titlurilor capitolelor
undo-target-import-tags = importul etichetelor
undo-target-import-templates = importul șabloanelor de note
undo-target-create = crearea
undo-target-remove = ștergerea
undo-target-rename = redenumirea
undo-target-edit = acea modificare
menu-edit-find = C&aută…
menu-edit-find-next = Caută &următorul
menu-edit-find-prev = Caută &precedentul
menu-edit-replace = Caută și înl&ocuiește…
menu-edit-find-in-project = Caută în pro&iect…
menu-edit-replace-in-project = Înlocuiește în proi&ect…
undo-frozen = Anulează („Mereu înainte” este activ)
redo-frozen = Refă („Mereu înainte” este activ)
menu-edit-cut = &Decupează
menu-edit-copy = &Copiază
menu-edit-paste = &Lipește
menu-edit-paste-plain = Lipește fără &formatare
menu-edit-select-all = Selectează &tot
shortcut-name-edit-cut = Decupează
shortcut-name-edit-copy = Copiază
shortcut-name-edit-paste = Lipește
shortcut-name-edit-paste-plain = Lipește fără formatare
shortcut-name-edit-select-all = Selectează tot

## Spell-check: the master switch (title bar / View menu / F7 / Settings ▸ Spelling)
titlebar-spellcheck-on = Verificarea ortografică este activă. Clic pentru a o opri (F7)
titlebar-spellcheck-off = Verificarea ortografică este oprită. Clic pentru a o reporni (F7)
titlebar-distraction-free = Intră în modul fără distrageri (F11)
menu-spellcheck = Verificare &ortografică
menu-comments = Co&mentarii
settings-page-spellcheck = Verificare ortografică
settings-group-spellcheck = Verificare ortografică
settings-spellcheck-enabled = Verifică ortografia în timp ce scriu
settings-spellcheck-hint = Subliniază cuvintele pe care niciun dicționar instalat nu le cunoaște. Dezactivarea opțiunii oprește orice verificare, în toate proiectele, până o reactivați. Pentru a opri verificarea unei singure limbi, debifați-o în câmpul Limbă al proiectului sau al unui element.

## Trash panel
menu-trash = Coș de &gunoi
trash-title = Coș de gunoi
trash-empty-state = Coșul de gunoi este gol.
trash-empty-button = Golește coșul de gunoi…
trash-restore = &Restaurează
trash-restore-to = Restaurează î&n…
trash-delete-forever = Șterge &definitiv
trash-restored-ok = Restaurate: { $count }.
trash-restore-error = Restaurarea nu a reușit: { $error }
trash-restore-orphaned = Locul original al acestui element nu mai există. Alegeți unde să fie restaurat.
trash-restore-no-project = Niciun proiect nu este deschis, deci nu există nimic de restaurat.
trash-delete-no-project = Niciun proiect nu este deschis, deci nu există nimic de șters.
trash-empty-confirm-title = Goliți coșul de gunoi?
trash-empty-confirm-text = { $count ->
       [one] { $count } intrare din coșul de gunoi va fi ștearsă definitiv.
       [few] { $count } intrări din coșul de gunoi vor fi șterse definitiv.
      *[other] { $count } de intrări din coșul de gunoi vor fi șterse definitiv.
   } Ies definitiv din coșul de gunoi. Anularea le mai poate aduce înapoi până când faceți altceva.
trash-emptied-title = Coșul de gunoi a fost golit
trash-emptied-body = Tot ce era în coșul de gunoi a fost șters definitiv.
trash-empty-no-project = Niciun proiect nu este deschis, deci nu există niciun coș de gunoi de golit.
trash-delete-forever-confirm-title = Ștergeți definitiv?
trash-delete-forever-confirm-text = { $count ->
       [one] { $count } element va fi șters definitiv.
       [few] { $count } elemente vor fi șterse definitiv.
      *[other] { $count } de elemente vor fi șterse definitiv.
   } Ies definitiv din coșul de gunoi. Anularea le mai poate aduce înapoi până când faceți altceva.
trash-deleted-title = Șters definitiv
trash-deleted-body = Șterse definitiv: { $count }.
trash-undo = Anulează
trash-restore-picker-title = Restaurează în…
trash-restore-to-confirm-title = Restaurați aici?
trash-restore-to-confirm-text = Restaurați „{ $item }” în „{ $destination }”?
trash-restore-picker-restore-here = Restaurează aici
trash-restore-picker-cancel = Renunță
trash-restore-picker-empty = Încă nu există clasoare. Creați mai întâi unul.
trash-banner-title = Acest element se află în coșul de gunoi
trash-banner-description = Nu va apărea în structură sau în exporturi până nu îl restaurați.
trash-banner-restore = Restaurează…
trash-tab-tooltip = În coșul de gunoi

# Composite tooltip: the full parameter sheet shown on an export-style row.
settings-styles-sheet-yes = Da
settings-styles-sheet-no = Nu
settings-styles-sheet-font = Font
settings-styles-sheet-indent = Indentarea primului rând
settings-styles-sheet-para-spacing = Spațiere între paragrafe
settings-styles-sheet-page = Dimensiunea paginii
settings-styles-sheet-margins = Margini (sus/dreapta/jos/stânga)
settings-styles-sheet-title-page = Pagina de titlu a cărții
settings-styles-sheet-heading-language = Limba titlurilor
settings-styles-sheet-auto = Urmează textul
settings-styles-sheet-digits = Cifre
settings-styles-sheet-direction = Direcția textului
settings-styles-sheet-formats = Formate
settings-styles-sheet-all-formats = Toate

settings-styles-editor-group = Editor de stil
settings-styles-editor-missing = Acest stil nu mai este disponibil.

settings-styles-page-letter = Letter
settings-styles-digits-western = Occidentale (0–9)
settings-styles-digits-eastern-arabic = Arabe orientale (٠–٩)
settings-styles-direction-ltr = De la stânga la dreapta
settings-styles-direction-rtl = De la dreapta la stânga

## Format dock
format-dock-title = Format
format-panel-empty = Puneți cursorul într-o scenă, o notă sau un sinopsis pentru a vedea opțiunile de formatare.
# Group headers.
format-group-marks = Text
format-group-block = Paragraf
format-group-lists = Liste
format-group-tables = Tabel
format-group-breaks = Separatoare de scenă
# Button tooltips. Icon-only buttons, so each tooltip is that button's only
# accessible name, not decoration.
format-superscript = Exponent
format-subscript = Indice
format-link = Legătură…
# The Link command: a hyperlink in the prose. One command, three doors (the
# Format dock, the Format menu and Ctrl+K), so these strings are shared.
link-dialog-insert-title = Inserare legătură
link-dialog-edit-title = Editare legătură
link-dialog-text-label = Text
link-dialog-text-placeholder = cuvintele pe care le vede cititorul
link-dialog-url-label = Duce la
link-dialog-url-placeholder = exemplu.ro
link-dialog-insert = Inserează
link-dialog-apply = Aplică
link-dialog-cancel = Renunță
link-dialog-remove = Elimină legătura
# Refused because a document could otherwise make a click launch a program.
link-scheme-refused = Pot fi deschise doar legături web și de e-mail. { $url } a fost lăsat neatins.
menu-format-link = Le&gătură…
format-clear = Elimină formatarea
format-heading = Nivelul titlului
format-heading-normal = Text normal
format-heading-1 = Titlu 1
format-heading-2 = Titlu 2
format-heading-3 = Titlu 3
format-heading-4 = Titlu 4
format-heading-5 = Titlu 5
format-heading-6 = Titlu 6
format-align-left = Aliniere la stânga
format-align-center = Centrat
format-direction-rtl = Paragraf de la dreapta la stânga
format-blockquote = Bloc de citat
format-list-bullet = Listă cu marcatori
format-list-numbered = Listă numerotată
format-indent = Mărește indentarea
format-outdent = Micșorează indentarea
format-table-insert = Inserează un tabel
format-table-row-above = Inserează un rând deasupra
format-table-row-below = Inserează un rând dedesubt
format-table-col-before = Inserează o coloană înainte
format-table-col-after = Inserează o coloană după
format-table-row-delete = Șterge rândul
format-table-col-delete = Șterge coloana
format-table-remove = Elimină tabelul

## Menu: Format
menu-format-marks-bold = &Aldin
menu-format-marks-italic = &Cursiv
menu-format-marks-underline = &Subliniat
menu-format-marks-strike = Tă&iat
menu-format-marks-superscript = E&xponent
menu-format-marks-subscript = In&dice
menu-format-marks-clear = &Elimină formatarea
menu-format-heading = &Titlu de secțiune
menu-format-heading-normal = Text &normal
menu-format-heading-1 = Titlu &1
menu-format-heading-2 = Titlu &2
menu-format-heading-3 = Titlu &3
menu-format-heading-4 = Titlu &4
menu-format-heading-5 = Titlu &5
menu-format-heading-6 = Titlu &6
menu-format-alignment = Ali&niere
menu-format-align-left = Aliniere la &stânga
menu-format-align-center = &Centrat
menu-format-direction = Di&recție
menu-format-direction-auto = &Automată
menu-format-direction-ltr = De la &stânga la dreapta
menu-format-direction-rtl = De la &dreapta la stânga
menu-format-blockquote = Bl&oc de citat
menu-format-lists = &Liste
menu-format-list-bullet = Listă cu &marcatori
menu-format-list-numbered = Listă &numerotată
menu-format-indent = Mărește &indentarea
menu-format-outdent = Micșorează in&dentarea
menu-format-table = Ta&bel
menu-format-table-insert = &Inserează un tabel
menu-format-table-2x2 = &2 x 2
menu-format-table-3x3 = &3 x 3
menu-format-table-4x4 = &4 x 4
menu-format-table-row-above = Inserează un rând &deasupra
menu-format-table-row-below = Inserează un rând dede&subt
menu-format-table-col-before = Inserează o coloană î&nainte
menu-format-table-col-after = Inserează o coloană du&pă
menu-format-table-row-delete = Șterge &rândul
menu-format-table-col-delete = Șterge &coloana
menu-format-table-remove = &Elimină tabelul

## Formatting marks: the invisible characters a writer places by hand.
## Mnemonics are unique within the submenu (N, H, S, r, Z, W).
menu-format-typo-marks = Caracter de &formatare
menu-format-typo-nbsp = Spațiu &nedespărțitor
menu-format-typo-nbhyphen = &Cratimă nedespărțitoare
menu-format-typo-soft-hyphen = Cratimă &opțională
menu-format-typo-nnbsp = Spațiu în&gust nedespărțitor
menu-format-typo-zwsp = Spațiu de lățime &zero
menu-format-typo-word-joiner = &Unitor de cuvinte

## About panel

menu-about = &Despre Skribisto…
about-title = Despre Skribisto
about-version = Versiunea { $version }
about-update-available = Versiunea { $version } este disponibilă
about-update-see-notes = Vedeți ce s-a schimbat
about-update-flathub = Flathub ține această copie la zi.
about-update-distro = Distribuția dumneavoastră ține această copie la zi.
about-tagline = O aplicație pentru scrierea romanelor și a ficțiunii de lungă durată, scrisă în Rust cu trusa de instrumente Teksilo.
about-license = Publicat sub Licența Publică Generală GNU, versiunea 3.
about-copyright = © 2026 Cyril Jacquet
about-close = Închide

## Native (macOS) menu bar
# Labels for the platform-standard App and Window menus mirrored into the macOS
# global menu bar. No `&` mnemonics here: macOS has none, and the native bridge
# resolves these without stripping one, so an ampersand would print literally.
# The application name is data (the running edition's own name), so it arrives
# as an argument: same rule as the window titles below.
native-menu-about = Despre { $app }
native-menu-hide = Ascunde { $app }
native-menu-hide-others = Ascunde celelalte
native-menu-show-all = Afișează toate
native-menu-services = Servicii
native-menu-quit = Ieși din { $app }
native-menu-settings = Setări…
native-menu-window = Fereastră
native-menu-minimize = Minimizează
native-menu-zoom = Zoom

# Window titles. The application name is data (the running edition's own name),
# so it arrives as an argument rather than being written into the value here.
window-title = { $title } - { $app }
window-title-numbered = { $title } - { $app } (fereastra { $n })
window-title-empty = { $app }

# First-run settings import (an edition with its own config directory, finding
# the community installation's settings beside it).
first-run-window-title = Bun venit
first-run-title = Configurați { $app }
first-run-body = { $app } își păstrează setările separat de Skribisto, așa că pornește fără ele. Preferințele, proiectele recente, dicționarele și aranjarea ferestrelor pot fi copiate acum.
first-run-from = Copiază din
first-run-to = Copiază în
first-run-copy-note = Nimic nu este mutat sau șters. Skribisto își păstrează toate setările și funcționează în continuare exact ca înainte.
first-run-import = Importă setările
first-run-start-fresh = Pornește de la zero
first-run-import-failed = Unele setări nu au putut fi importate: { $error }

# ── Work ▸ Punctuation: the project's typographic house style ──────────────
settings-page-punctuation = Punctuație implicită
settings-group-punctuation = Punctuație inteligentă
settings-page-work-punctuation = Punctuație
settings-punctuation-override = Folosește reguli de punctuație proprii pentru acest proiect
settings-punctuation-override-hint = Dezactivat: proiectul urmează preferința aplicației. Aceste reguli sunt păstrate în fișierul .skrib, așa că un coautor care deschide fișierul scrie cu aceeași tipografie.
settings-punctuation-dashes = Transformă -- în linie scurtă (–) și --- în linie de pauză (—)
settings-punctuation-ellipsis = Transformă ... în puncte de suspensie (…)
settings-punctuation-quotes = Ghilimele și apostrofuri tipografice
settings-quote-style = Ghilimele
settings-quote-style-locale = După limbă
settings-quote-style-curly = “Duble”
settings-quote-style-curly-single = ‘Simple’
settings-quote-style-guillemets = «Unghiulare»
settings-quote-style-low-high = „Jos-sus“
settings-punctuation-spacing = Spațiu înainte de ; : ! ?
settings-punctuation-spacing-hint = Tipografia franceză pune un spațiu îngust nedespărțitor înainte de ; ! ? și unul întreg înainte de :. Se aplică doar textului scris în franceză. Spațiul din interiorul ghilimelelor unghiulare « » vine odată cu acestea.
settings-punctuation-sample = Limba dumneavoastră dă
settings-punctuation-app-hint = Ce face fiecare proiect, dacă nu are reguli proprii în Proiect ▸ Punctuație.
settings-punctuation-dialogue = Începe cu o linie de dialog un paragraf tastat ca „- ”
settings-punctuation-dialogue-hint = Pentru limbile care marchează vorbirea cu o linie în loc de ghilimele: franceza, spaniola, rusa și altele. Se aplică doar chiar la începutul unui paragraf.
settings-punctuation-auto-lists = Începe o listă dintr-un paragraf tastat ca "- ", "* " sau "1. "
settings-punctuation-auto-lists-hint = Paragraful devine o listă cu marcatori sau numerotată. Acolo unde linia de dialog este activată și limba are una, "- " deschide în schimb o replică. Apăsați Backspace imediat după pentru a recupera ce ați tastat.

## Go to (jump to any item)
statusbar-go-to = Salt la…
go-to-placeholder = Caută în clasor
go-to-no-matches = Niciun element nu corespunde acestei căutări.
menu-go-to = &Salt la…

# ── Distraction-free themes (Settings ▸ Editor ▸ Distraction-free themes) ──
settings-page-distraction-free-themes = Teme pentru modul fără distrageri
settings-themes-builtin = Teme incluse
settings-themes-builtin-badge = Inclusă
settings-themes-user = Temele mele
settings-themes-editor-group = Editare temă
settings-themes-editor-empty = Alegeți o temă din „Temele mele” pentru a o edita.
settings-themes-use = Folosește
settings-themes-duplicate = Duplică
settings-themes-edit = Editează
settings-themes-delete = Șterge
settings-themes-export = Exportă…
settings-themes-import = Importă…
settings-themes-copy-suffix = copie
settings-themes-json-filter = Temă (JSON)
settings-themes-imported = Temă importată
settings-themes-import-failed = Tema nu a putut fi importată
settings-themes-exported = Temă exportată
settings-themes-export-failed = Tema nu a putut fi exportată
# Shown on a theme whose text and page are below the WCAG AA contrast floor.
settings-themes-low-contrast = contrast scăzut
# Shown when the page and the text are fine but the caret band is what hides
# the prose: a different fault, and one the row's own swatches do not show.
settings-themes-low-contrast-band = evidențierea ascunde textul
settings-themes-field-name = Nume
settings-themes-field-paper = Pagină
settings-themes-field-ink = Text
settings-themes-field-general = Fundal
settings-themes-field-widget-text = Textul barei de control
# The shading drawn around the caret: the sentence or paragraph being written,
# per Editor ▸ "Highlight around the caret". This field is the color it uses in
# distraction-free mode.
settings-themes-field-caret-band = Evidențiere în jurul cursorului

# The distraction-free strip's quick-settings gear, and the way through to the
# full theme library from inside it.
statusbar-focus-settings = Setările modului fără distrageri
statusbar-focus-manage-themes = Gestionează temele…

## Comments: the two docks, the thread cards, and their actions.
comments-title = Comentarii
comments-document-title = Acest document
comments-empty-project = Încă nu există comentarii în acest proiect.
comments-empty-document = Niciun comentariu la acest document.
comments-filter-all = Toate
comments-filter-open = Deschise
comments-filter-resolved = Rezolvate
# Always shown, including at zero: an orphan nobody looks for is an orphan nobody fixes.
comments-filter-orphaned = Orfane
comments-status-open = Deschis
comments-status-resolved = Rezolvat
comments-status-orphaned = Și-a pierdut textul
comments-orphan-snippet = (textul comentat nu mai există)
# A comment that resolved successfully yet has no live range to point at: an
# import from a format with no text-position concept for it (a heading, a blank
# paragraph, a table). Distinct from "Lost its text": nothing went missing, it
# never had a position to begin with.
comments-status-unplaced = Fără poziție în text
comments-unplaced-snippet = (neancorat în niciun text)
comments-reply-count = { $count ->
       [one] { $count } răspuns
       [few] { $count } răspunsuri
      *[other] { $count } de răspunsuri
   }
comments-menu-resolve = Marchează ca rezolvat
comments-menu-reopen = Redeschide
comments-menu-delete = Șterge
comments-sort-document = În ordinea din document
comments-sort-newest = Cele mai noi întâi
comments-sort-reader = Grupate după cititor
comments-unsigned-author = Nesemnat
# Shown on a thread that shares its passage with another reader's. Never at one:
# a lone remark needs no number, and "1 reader" beside every card would drown the
# two that say 3.
comments-agreement = { $count ->
       [one] { $count } cititor
       [few] { $count } cititori
      *[other] { $count } de cititori
   }
comments-menu-add = Adaugă un comentariu
comments-menu-add-paragraph = Comentează acest paragraf
overview-col-comments = Comentarii
overview-col-total-comments = Total comentarii
comments-card-placeholder = Scrieți un comentariu…
comments-card-unknown-author = Autor necunoscut
comments-unsigned-toast = Comentariile dumneavoastră sunt nesemnate: pe acest calculator nu este setat niciun nume.
comments-unsigned-action = Setați-vă numele
comments-card-reply = Răspunde
comments-card-reply-placeholder = Răspundeți…
comments-card-actions = Acțiuni pentru comentariu
comments-reply-actions = Acțiuni pentru răspuns
comments-menu-delete-reply = Șterge răspunsul
comments-menu-delete-all = Șterge toate comentariile de aici
comments-deleted-toast = Comentariu șters
comments-reply-deleted-toast = Răspuns șters
comments-deleted-all-toast = { $count ->
       [one] { $count } comentariu șters
       [few] { $count } comentarii șterse
      *[other] { $count } de comentarii șterse
   }
comments-undo = Anulează

# ── Analysis (Book container segment) ────────────────────────────────────────
# House rule for every string here: describe, never judge. No "too many", no
# "weak", no "should". Every comparison is against the book's own median or its
# own distribution, never a genre norm, and never a target value.
analysis-segment = Analiză
analysis-scope-book = Se analizează această carte
analysis-run = Pornește analiza
analysis-stale = S-a modificat de la ultima analiză
analysis-not-run = Încă neanalizată.
analysis-running = Se citește manuscrisul…
analysis-failed = Analiza nu s-a putut încheia.
analysis-no-scenes = Încă nu există scene în această carte.

analysis-shape = Formă

# ── How the text arrived ──────────────────────────────────────────────────────
# A fact about input, never about authorship. Nothing here weighs one route
# against another, and there is no total to reach and nothing to score.
analysis-arrivals = Proveniență
analysis-arrivals-explainer = Cum a ajuns textul în acest proiect cât timp a fost deschis. Arată pe ce cale au venit caracterele și nimic despre cine le-a scris: cine scrie ciorna în altă parte și o lipește a lipit, iar cine dictează a dictat. Nu există aici niciun număr de atins.
# The two scope caveats, said plainly rather than left to be assumed. Everything
# else on this bar is about one book and the whole of its text; this is about the
# whole project and only since it was opened.
analysis-arrivals-scope = Tot proiectul, nu doar această carte.
analysis-arrivals-session = De când l-ați deschis. Închiderea proiectului repornește numărătoarea.
analysis-arrivals-nothing = Încă nu a sosit niciun text în această sesiune.
analysis-arrivals-typed = Tastat
analysis-arrivals-pasted = Lipit
analysis-arrivals-dictated = Dictat
analysis-arrivals-imported = Importat
analysis-arrivals-programmatic = Inserat de aplicație
analysis-arrivals-count = { $count ->
       [one] { $count } caracter
       [few] { $count } caractere
      *[other] { $count } de caractere
   }
# Said where a route contributed nothing, so a reader is not left wondering
# whether it was measured at all.
analysis-arrivals-none = nimic

analysis-words-per-scene = Cuvinte pe scenă
analysis-median-words = Scena mediană a acestei cărți are { $count ->
       [one] { $count } cuvânt.
       [few] { $count } cuvinte.
      *[other] { $count } de cuvinte.
   }
analysis-median-line = Mediană: { $count ->
       [one] { $count } cuvânt
       [few] { $count } cuvinte
      *[other] { $count } de cuvinte
   }
analysis-dialogue = Dialog
# Said instead of showing 0%, which would read as "there is no dialogue here".
analysis-dialogue-unsupported = Dialogul nu este încă măsurat pentru această limbă.

analysis-footnote-words = Cuvinte în notele de subsol
# Kept apart from the manuscript total on purpose. See the module doc on
# AnalysisViewModel::run for why a footnote is authored prose but must not be folded
# into how far along the story reads as being.
analysis-footnote-words-count = { $count ->
       [one] { $count } cuvânt se află în notele de subsol ale acestei cărți, ținut separat de totalul manuscrisului.
       [few] { $count } cuvinte se află în notele de subsol ale acestei cărți, ținute separat de totalul manuscrisului.
      *[other] { $count } de cuvinte se află în notele de subsol ale acestei cărți, ținute separat de totalul manuscrisului.
   }
# The figure comes from its own operation and can still be catching up even once the
# rest of this report is ready, said plainly rather than shown as a misleading 0.
analysis-footnote-words-pending = Se numără notele de subsol…

# Shape's charts draw one bar per text, so an outlined-but-unwritten book is mostly gaps.
analysis-ignore-empty = Ignoră textele în care încă nu s-a scris nimic
analysis-empty-hidden = { $count ->
           [one] { $count } text gol ascuns.
           [few] { $count } texte goale ascunse.
          *[other] { $count } de texte goale ascunse.
       }
analysis-all-texts-empty = Toate textele din această carte sunt încă goale.

# ── Binder filter feedback ───────────────────────────────────────────────────
# The binder's search field lives in a popover, so once it is dismissed nothing
# on screen says a filter is still narrowing the tree. These strings are what
# say it. Without them a filtered-to-nothing binder is indistinguishable from
# an empty project.
binder-filter-count = Afișate: { $shown } din { $total }
binder-filter-clear = Golește
binder-filter-none = Nimic nu corespunde cu „{ $query }”.

## Settings: paratext structures
settings-paratext-intro = Paginile de început și de sfârșit cu care poate porni un proiect nou. Fiecare schemă aparține unei tradiții editoriale, iar titlurile paginilor ei sunt scrise în limba acelei tradiții. După crearea proiectului le puteți redenumi liber.
settings-paratext-structures = Scheme
settings-paratext-broken = Nu a putut fi citită
settings-paratext-edit = Editează
settings-paratext-duplicate = Duplică
settings-paratext-delete = Șterge
settings-paratext-new = Schemă nouă
settings-paratext-save = Salvează
settings-paratext-editor-hint = Un nume, paginile de creat înainte de manuscris și paginile de creat după el. Unde ajung depinde de dumneavoastră: odată creat proiectul, sunt elemente obișnuite.

## Images

image-insert = Inserează i&magine…
image-no-project = Deschideți un proiect înainte de a insera o imagine.
image-choose-title = Alegeți o imagine
image-filter-label = Imagini
image-large-title = Aceasta este o imagine mare
image-large-text =
       { $name } are { $megapixels } megapixeli ({ $width }×{ $height }).
       Dacă o păstrați așa cum este, fișierul original este stocat în proiect: merge
       cu fiecare copie de siguranță și cu fiecare export. Optimizarea stochează în schimb
       o copie mai mică, de cel mult 2560 de pixeli pe latura cea mai lungă.
image-large-keep = Păstrează originalul
image-large-downscale = Optimizează
image-large-remember = Fă așa de acum înainte, nu mai întreba
image-not-recorded = Imaginea a fost salvată, dar nu a putut fi înregistrată în proiect.
image-describe-title = Descrierea imaginii
image-describe-explain = Ce arată imaginea, pentru un cititor care nu o poate vedea. Nu face parte din manuscris: nu este niciodată numărată, căutată sau exportată ca proză.
image-describe-placeholder = un far pe fundalul unui cer cenușiu
image-resize-title = Redimensionarea imaginii
image-resize-explain = Un procent din mărimea la care este afișată acum. 100 o lasă așa cum este.
image-resize-invalid = Introduceți un număr între 1 și 1000.
image-menu-describe = &Descrie imaginea…
image-menu-resize = &Redimensionează imaginea…
image-menu-reset-size = &Mărimea originală

# The book's cover, chosen from the book, not typed into a scene.
cover-heading = Copertă
cover-pick = Alege…
cover-change = Schimbă…
cover-remove = Elimină
cover-none = Încă nicio copertă
cover-missing = Fișierul imaginii lipsește din proiect
cover-choose-title = Alegeți o copertă
cover-set = Coperta a fost setată.
cover-cleared = Coperta a fost eliminată. Imaginea se află în continuare în proiect.
# ── Outline row card ────────────────────────────────────────────────────────
# The hover card on an outline row: what a row is, without opening it.
card-label = Legendă
card-exportable = Exportat
card-numbered = Numerotat
card-created = Creat
card-modified = Modificat
card-yes = da
card-no = nu
card-type = Tip
card-position = Poziție
card-children = Elemente conținute
card-goal = Obiectiv
card-words = Cuvinte
card-synopsis = Sinopsis
card-point-of-view = Punct de vedere
card-aliases = Cunoscut(ă) și ca

export-orphan-footnotes-title = Note de subsol fără trimitere
export-comments-dropped =
       { $count ->
           [one] Un comentariu nu a putut fi plasat în textul exportat și a fost omis.
           [few] { $count } comentarii nu au putut fi plasate în textul exportat și au fost omise.
          *[other] { $count } de comentarii nu au putut fi plasate în textul exportat și au fost omise.
       }
export-orphan-footnotes =
       { $count ->
           [one] O notă de subsol nu mai are nicio trimitere în manuscris. Textul ei nu va apărea în cartea exportată.
           [few] { $count } note de subsol nu mai au nicio trimitere în manuscris. Textul lor nu va apărea în cartea exportată.
          *[other] { $count } de note de subsol nu mai au nicio trimitere în manuscris. Textul lor nu va apărea în cartea exportată.
       }

## Footnotes

menu-footnotes = &Note de subsol
footnotes-title = Note de subsol
footnotes-insert = Inserează &notă de subsol
footnotes-empty = Încă nu există note de subsol. Puneți cursorul într-o scenă, apoi folosiți + de mai sus sau Ctrl+Alt+F.
footnotes-filter-all = Toate
footnotes-filter-document = Acest document
footnotes-filter-orphaned = Orfane
footnotes-orphaned = Nimic nu mai trimite la această notă
footnotes-untitled-home = Fără titlu
footnotes-body-placeholder = nota propriu-zisă
footnotes-insert-tooltip = Inserează o notă de subsol la cursor (Ctrl+Alt+F)
footnotes-actions = Acțiuni pentru nota de subsol
footnotes-delete = Șterge nota și &trimiterea ei
footnotes-no-project = Deschideți un proiect înainte de a insera o notă de subsol.
footnotes-no-caret = Puneți cursorul în textul unei scene pentru a insera acolo o notă de subsol.
footnotes-not-created = Nota de subsol nu a putut fi adăugată în proiect.
footnotes-deleted-toast = Notă ștearsă
footnotes-undo-delete = Anulează

# ── Import documents (Markdown / Word / ODT / plain text) ─────────────────────
import-document-title = Importare documente
import-document-close = Închide
import-document-step-files = Fișiere
import-document-step-review = Verificare
import-document-step-destination = Destinație
import-document-step-reconcile = Îmbinare
import-document-reconcile-hint = Unele dintre aceste elemente le aveți deja. Spuneți ce trebuie să se întâmple cu fiecare.
import-document-reconcile-all-new = Nimic din acest fișier nu corespunde proiectului dumneavoastră: fiecare element va fi adăugat ca nou.
import-document-duplicate-returns = Aceste fișiere sunt mai multe copii ale aceluiași manuscris care se întorc. Importați-le pe rând: potrivirea aliniază o singură copie întoarsă cu proiectul dumneavoastră, așa că importul lor împreună ar adăuga o a doua copie a cărții în loc să o îmbine.
import-document-hunk-added = Preia acest paragraf nou
import-document-hunk-removed = Renunță la acest paragraf
import-document-hunk-changed = Preia această rescriere
import-document-col-stray-prose = Textul lui
import-document-stray-as-paratext = Păstrează ca paratext
import-document-stray-discard = Renunță la text
import-document-col-current = În proiectul dumneavoastră
import-document-col-incoming = În acest fișier
import-document-col-status = Situație
import-document-col-action = Ce se face
import-document-status-identical = Identic
import-document-status-editor-edited = Modificat de ei
import-document-status-you-edited = Modificat de dumneavoastră
import-document-status-conflict = Modificat de ambele părți
import-document-status-different = Diferit
import-document-status-new = Nou
import-document-status-missing = Nu este în acest fișier
import-document-status-moved = Mutat
import-document-action-comments-only = Doar comentariile
import-document-action-take-import = Preia această versiune
import-document-action-keep-current = Păstrează versiunea mea
import-document-action-create-new = Adaugă ca nou
import-document-action-ignore = Omite
import-document-compare = Compară
import-document-compare-legend = Versiunea dumneavoastră comparată cu cea din acest fișier. Doar pentru citire.
import-document-compare-close = Închide
import-document-drop-title = Plasați documentele aici
import-document-drop-hint = Markdown (.md), Word (.docx), OpenDocument (.odt) și text simplu (.txt)
import-document-browse = Răsfoiește…
import-document-move-up = Mută în sus
import-document-move-down = Mută în jos
import-document-remove-file = Elimină
import-document-file-count = { $count ->
       [one] Un fișier
       [few] { $count } fișiere
      *[other] { $count } de fișiere
   }
import-document-no-files = Încă nu a fost ales niciun fișier.
import-document-col-included = Import
import-document-col-title = Titlu
import-document-col-type = Tip
import-document-col-words = Cuvinte
import-document-col-breaks = Separatoare
import-document-col-comments = Comentarii
import-document-col-footnotes = Note de subsol
import-document-col-epigraph = Motto
import-document-col-source = Sursă
import-document-level-rules = Niveluri de titlu
import-document-level-n = Titlu { $level }
import-document-add-top-level = Adaugă un nivel superior
import-document-add-top-level-tooltip = Inserează o carte deasupra tuturor elementelor analizate, pentru fișierele de capitol care nu au un titlu de carte
import-document-destination = Destinație
import-document-destination-hint = Alegeți un clasor sau un element: elementele noi ajung în interiorul unui dosar sau după o scenă.
import-document-destination-empty = Încă nu există clasoare. Creați mai întâi unul.
import-document-plan-empty = Încă nu este nimic de importat.
# How many pictures the items being imported bring into the project, under the
# review tree. Shown only when there is at least one.
import-document-pictures = { $count ->
    [1] Cu aceste documente vine o imagine.
    [few] Cu aceste documente vin { $count } imagini.
   *[other] Cu aceste documente vin { $count } de imagini.
}
import-document-summary = { $rows ->
       [one] Un element
       [few] { $rows } elemente
      *[other] { $rows } de elemente
   } · { $breaks ->
       [one] un separator de scenă
       [few] { $breaks } separatoare de scenă
      *[other] { $breaks } de separatoare de scenă
   }
import-document-back = Înapoi
import-document-cancel = Renunță
import-document-analyse = Înainte
import-document-analysing = Se citesc documentele…
import-document-step-analysing = Citire
import-document-cancel-analysis = Oprește citirea
import-document-analyse-failed = Documentele nu au putut fi citite.
import-document-details = Detalii
import-document-import = Importă
import-document-done = { $count ->
       [one] Un element importat
       [few] { $count } elemente importate
      *[other] { $count } de elemente importate
   }
import-document-undo = Anulează
# ── Import diagnostics ────────────────────────────────────────────────────────
# One per `document_ingest::ImportDiagnostic::key()`. The variant's data arrives
# as arguments; the sentence is assembled here, per locale. A row-scoped
# diagnostic gets its { $title } and { $kind } from the row it names, never from
# the wire.
import-diagnostic-file-unreadable = „{ $path }” nu a putut fi citit: { $detail }. Celelalte fișiere se importă în continuare.
# A file refused unread because its XML nests deeper than the app will read.
# $limit is the ceiling, a number.
import-diagnostic-nested-too-deep = „{ $path }” nu a fost citit: conținutul lui este imbricat pe mai mult de { $limit } niveluri. Niciun document real nu se apropie de așa ceva, iar citirea lui ar putea bloca Skribisto, așa că este omis. Celelalte fișiere se importă în continuare.
import-diagnostic-archive-too-large = „{ $path }” nu a fost citit: este un fișier comprimat făcut să se dezarhiveze mult peste mărimea lui, ceea ce ar putea epuiza memoria și bloca Skribisto, așa că este omis. Celelalte fișiere se importă în continuare.
import-diagnostic-lossy-decode = { $count ->
       [one] Un caracter din „{ $path }” nu a putut fi decodat. Salvați din nou fișierul ca UTF-8 pentru a-l păstra.
       [few] { $count } caractere din „{ $path }” nu au putut fi decodate. Salvați din nou fișierul ca UTF-8 pentru a le păstra.
      *[other] { $count } de caractere din „{ $path }” nu au putut fi decodate. Salvați din nou fișierul ca UTF-8 pentru a le păstra.
   }
import-diagnostic-decoded-from-bom = „{ $path }” a fost decodat ca { $detail }, nu ca UTF-8.
import-diagnostic-empty-file = „{ $path }” este gol.
import-diagnostic-no-headings = „{ $path }” nu are titluri, așa că ajunge ca un singur element.
import-diagnostic-unsupported-format = Fișierele „.{ $detail }” nu pot fi citite, așa că „{ $path }” a fost omis.
import-diagnostic-front-matter-not-flat = Metadatele de la începutul lui „{ $path }”: „{ $detail }” nu este o valoare simplă și a fost omis.
import-diagnostic-footnotes-degraded = { $count ->
       [one] O notă de subsol din „{ $path }” ajunge ca text simplu: notele de subsol nu sunt citite din Markdown.
       [few] { $count } note de subsol din „{ $path }” ajung ca text simplu: notele de subsol nu sunt citite din Markdown.
      *[other] { $count } de note de subsol din „{ $path }” ajung ca text simplu: notele de subsol nu sunt citite din Markdown.
   }
# The sibling of the line above, and deliberately a different sentence. That one
# says the reader cannot carry footnotes at all (Markdown); this one says it
# carried the rest and names what it could not: a note with no text behind it, or
# one whose reference sits where no prose block covers it, a chapter title being
# the ordinary case.
import-diagnostic-footnote-not-carried = { $count ->
       [one] O notă de subsol din „{ $path }” nu a putut fi preluată. Celelalte au fost preluate.
       [few] { $count } note de subsol din „{ $path }” nu au putut fi preluate. Celelalte au fost preluate.
      *[other] { $count } de note de subsol din „{ $path }” nu au putut fi preluate. Celelalte au fost preluate.
   }
import-diagnostic-raw-html-dropped = { $count ->
       [one] Un bloc de HTML brut din „{ $path }” a fost eliminat.
       [few] { $count } blocuri de HTML brut din „{ $path }” au fost eliminate.
      *[other] { $count } de blocuri de HTML brut din „{ $path }” au fost eliminate.
   }
import-diagnostic-nested-break-dropped = { $count ->
       [one] Un separator de scenă dintr-un citat sau dintr-o listă din „{ $path }” a fost eliminat. Se păstrează doar un separator aflat pe un rând separat.
       [few] { $count } separatoare de scenă din citate sau liste din „{ $path }” au fost eliminate. Se păstrează doar un separator aflat pe un rând separat.
      *[other] { $count } de separatoare de scenă din citate sau liste din „{ $path }” au fost eliminate. Se păstrează doar un separator aflat pe un rând separat.
   }
# A picture the document shows that does not come with it, one sentence per reason.
# $detail is the picture as the document names it: a file name, a path inside the
# document, or a web address. Nothing is left in the text where it was.
import-diagnostic-image-not-ingested-format = În „{ $path }”, imaginea „{ $detail }” este într-un format pe care Skribisto nu îl poate afișa, așa că a fost omisă. Sunt preluate imaginile PNG, JPEG, WebP, GIF, TIFF sau BMP; salvați-o într-unul dintre aceste formate și inserați-o manual.
import-diagnostic-image-not-ingested-remote = În „{ $path }”, imaginea „{ $detail }” se află pe web. Imaginile nu sunt descărcate niciodată în timpul unui import, așa că a fost omisă.
import-diagnostic-image-not-ingested-outside = În „{ $path }”, imaginea „{ $detail }” se află în afara documentului și a dosarului lui, așa că a fost omisă. Sunt preluate doar imaginile stocate în document sau în dosarul propriu al documentului.
import-diagnostic-image-not-ingested-missing = În „{ $path }”, imaginea „{ $detail }” nu a putut fi găsită, așa că a fost omisă.
import-diagnostic-image-not-ingested-too-large = În „{ $path }”, imaginea „{ $detail }” este mai mare decât acceptă un proiect, așa că a fost omisă.
import-diagnostic-image-not-ingested-unreadable = În „{ $path }”, imaginea „{ $detail }” nu a putut fi citită ca imagine, așa că a fost omisă.
import-diagnostic-image-not-ingested-heading = În „{ $path }”, imaginea „{ $detail }” se află într-un titlu de secțiune. Fiecare titlu de secțiune devine titlul unui element din clasor, iar un titlu nu conține imagini, așa că imaginea a fost omisă.
import-diagnostic-image-not-ingested-note = În „{ $path }”, imaginea „{ $detail }” se află într-o notă de subsol, într-o notă de final sau într-un comentariu. Acestea își păstrează cuvintele, dar nu și imaginile, așa că a fost omisă.
import-diagnostic-image-not-ingested-html = În „{ $path }”, imaginea „{ $detail }” este scrisă în HTML, iar un import omite HTML-ul, așa că a fost omisă odată cu el. Scrieți-o ca imagine Markdown pentru a o prelua.
import-diagnostic-image-not-ingested-document-full = În „{ $path }”, imaginea „{ $detail }” a fost omisă: imaginile acestui document depășesc împreună cât poate prelua un singur import. Importați documentul pe bucăți sau inserați restul manual.
import-diagnostic-picture-first-frame-only = În „{ $path }”, imaginea animată „{ $detail }” ajunge doar cu primul ei cadru. Imaginile dintr-un proiect nu se mișcă.
import-diagnostic-duplicate-title = „{ $title }” apare { $count ->
       [one] o dată
       [few] de { $count } ori
      *[other] de { $count } de ori
   }. Dacă ați mai importat aceste fișiere, ele vor fi duplicate.
import-diagnostic-heading-level-jump = „{ $title }” sare de la nivelul de titlu { $from } la { $to }; este plasat cu un nivel sub părintele lui.
import-diagnostic-illegal-combination = „{ $title }” conține proză, dar tipul „{ $kind }” nu poate conține proză. Importul este blocat până când îi schimbați tipul sau îl excludeți. Altfel nu se importă absolut nimic.
# Two sentences, because the file may or may not say who. `$names` is a
# ready-formatted list the app never splits; empty when the source records no
# author, which is what an anonymised .docx looks like.
import-diagnostic-tracked-changes-flattened = { $path } era în curs de revizuire: { $count ->
       [one] { $count } modificare urmărită a fost acceptată
       [few] { $count } modificări urmărite au fost acceptate
      *[other] { $count } de modificări urmărite au fost acceptate
   }, iar ștergerile au fost aplicate. Acesta este textul final. Pentru a păstra în schimb o parte din formulările dumneavoastră, folosiți „Compară” la ultimul pas.
import-diagnostic-tracked-changes-flattened-by = { $path } era în curs de revizuire: { $count ->
       [one] { $count } modificare urmărită făcută de { $names } a fost acceptată
       [few] { $count } modificări urmărite făcute de { $names } au fost acceptate
      *[other] { $count } de modificări urmărite făcute de { $names } au fost acceptate
   }, iar ștergerile au fost aplicate. Acesta este textul final. Pentru a păstra în schimb o parte din formulările dumneavoastră, folosiți „Compară” la ultimul pas.
import-diagnostic-text-box-dropped = { $count ->
       [one] { $path } conține { $count } casetă de text. Textul ei se află în afara fluxului documentului, așa că nu se poate stabili unde îi este locul într-un manuscris. Nu este importată.
       [few] { $path } conține { $count } casete de text. Textul lor se află în afara fluxului documentului, așa că nu se poate stabili unde le este locul într-un manuscris. Nu sunt importate.
      *[other] { $path } conține { $count } de casete de text. Textul lor se află în afara fluxului documentului, așa că nu se poate stabili unde le este locul într-un manuscris. Nu sunt importate.
   }
import-diagnostic-embedded-object-dropped = { $count ->
       [one] { $path } conține { $count } obiect încorporat: o diagramă, o ecuație sau ceva asemănător. Nimic dintr-un manuscris nu l-ar putea conține.
       [few] { $path } conține { $count } obiecte încorporate: o diagramă, o ecuație sau ceva asemănător. Nimic dintr-un manuscris nu le-ar putea conține.
      *[other] { $path } conține { $count } de obiecte încorporate: o diagramă, o ecuație sau ceva asemănător. Nimic dintr-un manuscris nu le-ar putea conține.
   }
import-diagnostic-field-flattened = { $count ->
       [one] { $path } conține { $count } câmp: un număr de pagină, o trimitere, o dată. Își păstrează textul afișat ultima dată și nu se va mai actualiza.
       [few] { $path } conține { $count } câmpuri: un număr de pagină, o trimitere, o dată. Fiecare își păstrează textul afișat ultima dată și nu se va mai actualiza.
      *[other] { $path } conține { $count } de câmpuri: un număr de pagină, o trimitere, o dată. Fiecare își păstrează textul afișat ultima dată și nu se va mai actualiza.
   }
import-diagnostic-unknown-style-level = { $path } folosește stilul „{ $detail }”, care pare un stil de titlu, dar nu indică niciun nivel. Acele paragrafe sunt importate ca proză, în loc să li se ghicească un nivel.
import-diagnostic-comment-unanchored = Comentariul „{ $detail }” din { $path } nu a putut fi atașat cuvintelor la care se referea. Este păstrat pe elementul său, de unde îl puteți muta.
# The sibling of the line above, for the one case where there is no item to keep the
# comment on: the file brought no text at all, or only headings.
import-diagnostic-comment-not-carried = Comentariul „{ $detail }” din { $path } nu este importat, pentru că nimic din fișier nu ajunge ca text pe care să fie păstrat. Copiați-l din fișier dacă aveți nevoie de el.
import-diagnostic-comment-replies-flattened = { $count ->
       [one] { $count } răspuns din { $path } se referea la un comentariu care nu este în fișier, așa că ajunge ca un comentariu de sine stătător.
       [few] { $count } răspunsuri din { $path } se refereau la un comentariu care nu este în fișier, așa că ajung ca niște comentarii de sine stătătoare.
      *[other] { $count } de răspunsuri din { $path } se refereau la un comentariu care nu este în fișier, așa că ajung ca niște comentarii de sine stătătoare.
   }
import-diagnostic-prose-not-verbatim = { $count ->
       [one] { $count } paragraf din { $path } nu a putut fi preluat cu formatarea exact cum a fost scrisă, așa că ajunge ca text simplu. Comparați-l cu originalul.
       [few] { $count } paragrafe din { $path } nu au putut fi preluate cu formatarea exact cum a fost scrisă, așa că ajung ca text simplu. Comparați-le cu originalul.
      *[other] { $count } de paragrafe din { $path } nu au putut fi preluate cu formatarea exact cum a fost scrisă, așa că ajung ca text simplu. Comparați-le cu originalul.
   }
import-diagnostic-styled-spaces-not-carried = { $count ->
       [one] În { $path }, { $count } porțiune de spațiu gol era subliniată sau tăiată, de exemplu un rând lăsat de completat. Skribisto nu păstrează nicio linie trasă doar peste spații, așa că ajunge ca spații simple sau deloc, dacă se află la începutul sau la sfârșitul unui paragraf.
       [few] În { $path }, { $count } porțiuni de spațiu gol erau subliniate sau tăiate, de exemplu rânduri lăsate de completat. Skribisto nu păstrează nicio linie trasă doar peste spații, așa că ajung ca spații simple sau deloc, dacă se află la începutul sau la sfârșitul unui paragraf.
      *[other] În { $path }, { $count } de porțiuni de spațiu gol erau subliniate sau tăiate, de exemplu rânduri lăsate de completat. Skribisto nu păstrează nicio linie trasă doar peste spații, așa că ajung ca spații simple sau deloc, dacă se află la începutul sau la sfârșitul unui paragraf.
   }
# List items nested deeper than the importer keeps, placed at the deepest level it keeps.
# $count is how many items; $limit is that deepest level, a number.
import-diagnostic-list-nesting-flattened = { $count ->
       [one] { $count } element de listă din { $path } era imbricat pe mai mult de { $limit } niveluri. Ajunge la nivelul { $limit }, cel mai adânc pe care îl păstrează Skribisto, cu tot cu cuvinte și formatare.
       [few] { $count } elemente de listă din { $path } erau imbricate pe mai mult de { $limit } niveluri. Ajung la nivelul { $limit }, cel mai adânc pe care îl păstrează Skribisto, cu tot cu cuvinte și formatare.
      *[other] { $count } de elemente de listă din { $path } erau imbricate pe mai mult de { $limit } niveluri. Ajung la nivelul { $limit }, cel mai adânc pe care îl păstrează Skribisto, cu tot cu cuvinte și formatare.
   }
# Tables too lopsided to complete into a grid, one row far wider than many short ones,
# which arrive as the paragraphs of their cells. $count is how many tables.
import-diagnostic-table-read-as-paragraphs = { $count ->
       [one] { $count } tabel din { $path } are un rând mult mai lat decât celelalte, prea dezechilibrat pentru a fi așezat ca grilă. Celulele lui ajung ca paragrafe, unul după altul, cu tot cu cuvinte și formatare.
       [few] { $count } tabele din { $path } au un rând mult mai lat decât celelalte, prea dezechilibrat pentru a fi așezate ca grilă. Celulele lor ajung ca paragrafe, unul după altul, cu tot cu cuvinte și formatare.
      *[other] { $count } de tabele din { $path } au un rând mult mai lat decât celelalte, prea dezechilibrat pentru a fi așezate ca grilă. Celulele lor ajung ca paragrafe, unul după altul, cu tot cu cuvinte și formatare.
   }
# Runs of spaces longer than the importer keeps, cut to that length. $count is how many
# runs; $limit is the most spaces a run keeps, a number.
import-diagnostic-spaces-shortened = { $count ->
       [one] { $count } șir de spații din { $path } avea peste { $limit } spații și ajunge scurtat la { $limit }. Cuvintele din jur sunt toate acolo.
       [few] { $count } șiruri de spații din { $path } aveau peste { $limit } spații și ajung scurtate la { $limit }. Cuvintele din jur sunt toate acolo.
      *[other] { $count } de șiruri de spații din { $path } aveau peste { $limit } spații și ajung scurtate la { $limit }. Cuvintele din jur sunt toate acolo.
   }
import-diagnostic-epigraph-not-carried = „{ $title }” începe cu un motto, dar tipul „{ $kind }” nu poate avea motto. Citatul este păstrat în schimb la începutul textului său.
import-diagnostic-epigraph-placement-ambiguous = Un motto se află între „{ $title }” și „{ $below }” și ar putea aparține oricăruia. A fost atribuit lui „{ $title }”, acolo unde stă de obicei un motto.
import-document-diagnostics = { $errors ->
       [0] { $warnings ->
               [one] Un lucru de știut
               [few] { $warnings } lucruri de știut
              *[other] { $warnings } de lucruri de știut
           }
      *[other] { $errors ->
               [one] Un fișier nu a putut fi citit
               [few] { $errors } fișiere nu au putut fi citite
              *[other] { $errors } de fișiere nu au putut fi citite
           }
   }
import-document-diagnostics-none = Nimic de raportat.

# ── Versions dock ──
versions-title = Versiuni
versions-scope-synopsis = Sinopsis
versions-scope-prose = Text
versions-loading = Se caută prin copiile de siguranță…
versions-empty = Încă nu există o versiune anterioară
versions-error = Copiile de siguranță nu au putut fi citite. Nu s-a pierdut nimic, dar această listă poate fi incompletă
versions-did-not-exist = Nu exista încă la { $date }
versions-deleted-after = Șters la un moment dat după { $date }
versions-unreadable = { $count ->
       [one] { $count } copie de siguranță nu a putut fi citită
       [few] { $count } copii de siguranță nu au putut fi citite
      *[other] { $count } de copii de siguranță nu au putut fi citite
   }
# Counts what the project's own history dropped, which is why it names it: a
# backup may still hold one of those states, and then it is in the list above.
versions-thinned = { $count ->
       [one] Versiunile vechi se răresc pe măsură ce îmbătrânesc. Istoricul propriu al proiectului a renunțat deja la { $count } stare anterioară a acestui text.
       [few] Versiunile vechi se răresc pe măsură ce îmbătrânesc. Istoricul propriu al proiectului a renunțat deja la { $count } stări anterioare ale acestui text.
      *[other] Versiunile vechi se răresc pe măsură ce îmbătrânesc. Istoricul propriu al proiectului a renunțat deja la { $count } de stări anterioare ale acestui text.
   }
versions-source-backup = Dintr-o copie de siguranță
versions-source-project = Din istoricul propriu al proiectului
versions-list-caption = O intrare pentru fiecare modificare, nu pentru fiecare copie de siguranță
versions-pick-a-version = Alegeți o versiune pentru a vedea ce s-a schimbat
versions-earliest = Cea mai veche versiune înregistrată. Nu există nimic mai vechi cu care să fie comparată.
versions-no-change = Nimic nu s-a schimbat în această parte a elementului
versions-formatting-only = Aici s-a schimbat doar formatarea. Cuvintele sunt aceleași
versions-show-unchanged = Arată ce nu s-a schimbat
versions-hide-unchanged = Ascunde ce nu s-a schimbat
versions-next-change = Modificarea următoare
versions-near = lângă „{ $text }”
versions-words-added = { $count ->
       [one] { $count } cuvânt adăugat
       [few] { $count } cuvinte adăugate
      *[other] { $count } de cuvinte adăugate
   }
versions-words-removed = { $count ->
       [one] { $count } cuvânt eliminat
       [few] { $count } cuvinte eliminate
      *[other] { $count } de cuvinte eliminate
   }
versions-blocks-moved = { $count ->
       [one] { $count } paragraf mutat
       [few] { $count } paragrafe mutate
      *[other] { $count } de paragrafe mutate
   }
# Names the backup, not the version: a pin is held against a file path, so it
# keeps the whole snapshot this row's text was read out of. See versions-pin-note.
versions-pin = Fixează copia de siguranță din care provine această versiune. Curățarea automată nu o va șterge niciodată
versions-unpin = Anulează fixarea copiei de siguranță din care provine această versiune. Curățarea automată o poate șterge din nou
versions-pinned-only = Arată doar versiunile fixate
versions-pin-note = Doar versiunile provenite dintr-o copie de siguranță pot fi fixate. Fixarea păstrează un fișier, iar istoricul propriu al proiectului nu este un fișier.
versions-pinned-empty = Încă nu este nimic fixat aici
# The same sentence with the reason, for a list that actually holds a version no
# pin can reach. On an all-backup list the reason is true and irrelevant, and
# reads as an explanation for an emptiness it did not cause.
versions-pinned-empty-log = Încă nu este nimic fixat aici. Doar versiunile provenite dintr-o copie de siguranță pot fi fixate. Fixarea păstrează un fișier, iar istoricul propriu al proiectului nu este un fișier.
versions-range-filter = Arată doar versiunile dintre două date
versions-filtered-empty = Nicio versiune nu corespunde filtrelor setate
versions-clear-filters = Golește filtrele
# Also used by the Timeline band's filter row, like versions-error above it.
versions-last-30-days = Ultimele 30 de zile
versions-restore-button = Restaurează această versiune
versions-restore-confirm-title = Înlocuiți acest text cu versiunea din { $date }?
versions-restore-confirm-text = Ce aveți acum va fi înlocuit cu textul pe care acest element îl avea la { $date }.
versions-restore-confirm-with-comments = Ce aveți acum va fi înlocuit cu textul pe care acest element îl avea la { $date }. { $count ->
       [one] { $count } comentariu este ancorat în textul actual și poate rămâne orfan.
       [few] { $count } comentarii sunt ancorate în textul actual și pot rămâne orfane.
      *[other] { $count } de comentarii sunt ancorate în textul actual și pot rămâne orfane.
   }
versions-restore-confirm-undo-note = Mai întâi se face o copie de siguranță, iar Ctrl+Z anulează operația dintr-un singur pas.
versions-restored-toast = A fost restaurată versiunea din { $date }
versions-undo = Anulează
versions-restore-row-gone = Acest element nu mai există în proiect
versions-restore-no-home = Între timp, acest element și-a schimbat tipul, iar textul vechi nu mai are unde să fie pus în el
versions-restore-no-safety-copy = Copia de siguranță de precauție nu s-a făcut, așa că nu s-a schimbat nimic
versions-restore-failed = Restaurarea a eșuat: { $error }
versions-restore-backup-busy = O copie de siguranță este deja în curs. Reîncercați peste câteva momente
versions-restore-in-backup-file = Vă uitați la un fișier de copie de siguranță; deschideți proiectul propriu-zis pentru a restaura în el
versions-restore-no-project = Niciun proiect nu este deschis
# ── Bringing a deleted row back ──
versions-recreate-button = Readu înapoi…
versions-recreate-picker-title = Unde să fie pus?
versions-recreate-picker-empty = Acest proiect nu are niciun clasor în care să fie pus
versions-recreate-picker-confirm = Readu-l aici
versions-recreate-picker-cancel = Renunță
versions-recreate-untitled = acest element
versions-recreate-confirm-title = Readuceți „{ $item }”?
versions-recreate-confirm-text = Va fi adăugat în { $destination }, cu textul pe care îl avea la { $date }.
# NOT "Ctrl+Z": that undoes the focused editor's *document*, and this creates a
# binder row. The way back is the Undo on the toast, as it is for trash and
# comments. See versions-recreated-toast.
versions-recreate-confirm-undo-note = „Anulează”, din mesajul care urmează, îl scoate imediat la loc.
versions-recreated-toast = „{ $item }” este din nou în proiectul dumneavoastră
versions-recreated-partial-toast = { $count ->
       [one] „{ $item }” a revenit, dar unul dintre textele lui nu a putut fi citit
       [few] „{ $item }” a revenit, dar { $count } dintre textele lui nu au putut fi citite
      *[other] „{ $item }” a revenit, dar { $count } dintre textele lui nu au putut fi citite
   }
versions-recreate-already-here = Acest element se află deja în proiectul dumneavoastră
versions-recreate-no-destination = Alegeți un loc din clasor unde să fie pus
versions-recreate-unreadable = Acea copie de siguranță nu a putut fi citită, așa că nu s-a adăugat nimic
versions-recreate-failed = Nu a putut fi readus: { $error }
versions-changed-percent = S-a schimbat { $percent }% din acest text
versions-hidden-paragraphs = { $count ->
       [one] … { $count } paragraf neschimbat …
       [few] … { $count } paragrafe neschimbate …
      *[other] … { $count } de paragrafe neschimbate …
   }

# ── Timeline band ──
timeline-title = Înapoi în timp
timeline-coverage = { $count ->
       [one] { $count } versiune înregistrată, începând din { $oldest }
       [few] { $count } versiuni înregistrate, începând din { $oldest }
      *[other] { $count } de versiuni înregistrate, începând din { $oldest }
   }
timeline-loading = Se caută prin trecutul proiectului…
timeline-empty = Încă nu a fost înregistrată nicio versiune a acestui proiect
timeline-no-changes = Nimic nu s-a schimbat de atunci
timeline-no-text-changes = Niciun text nu s-a schimbat de atunci
timeline-slider-label = Versiune înregistrată
timeline-series-name = Mărimea proiectului
timeline-bars-caption = Fiecare bară este o versiune înregistrată, înaltă cât era proiectul atunci. Cea evidențiată este cea la care vă uitați.
timeline-bars-caption-periods = Sunt prea multe versiuni pentru a fi arătate una câte una, așa că fiecare bară reprezintă o { $period }, înaltă cât era proiectul la sfârșitul ei.
timeline-unit-hour = oră
timeline-unit-day = zi
timeline-unit-week = săptămână
timeline-unit-month = lună
timeline-range-filter = Arată doar versiunile dintre două date
timeline-range-empty = Nu a fost înregistrată nicio versiune între aceste date
timeline-open-period = Deschide această perioadă
timeline-show-all = Arată tot istoricul
timeline-changed-since = { $count ->
       [one] { $count } element diferă între { $date } și proiectul de acum
       [few] { $count } elemente diferă între { $date } și proiectul de acum
      *[other] { $count } de elemente diferă între { $date } și proiectul de acum
   }
timeline-kind-added = Scris de atunci
timeline-kind-removed = Nu mai este în proiect
timeline-kind-changed = Modificat de atunci
timeline-kind-moved = Mutat de atunci
timeline-not-yet-written = Acesta nu exista încă în acel moment
timeline-no-text-of-its-own = Acesta nu are text propriu. Este un titlu pentru ce conține
timeline-reader-close = Închide
timeline-reader-stamp = Așa cum era la { $date }
timeline-reader-loading = Se deschide versiunea înregistrată…
timeline-reader-unreadable = Această înregistrare nu a putut fi citită. Copia de siguranță poate fi mutată, ștearsă sau aflată pe o unitate care nu este conectată.
timeline-reader-compared = Așa cum era la { $date }, pus față în față cu ce spune acum
timeline-reader-diff-legend = Textul tăiat a dispărut de atunci; cel subliniat a fost adăugat.
timeline-reader-view-label = Ce se afișează
timeline-reader-view-diff = Modificări
timeline-reader-view-text = Text
timeline-reader-deleted = Acesta nu mai este în proiectul dumneavoastră. Îl puteți citi și copia de aici.
timeline-prose-only-record = Acest punct provine din istoricul propriu al proiectului, care păstrează doar textul, așa că nu se poate arăta ce a fost șters sau mutat de atunci. Alegeți pentru asta o copie de siguranță.

# The Settings tree's section for pages an extension contributed.
settings-sec-extensions = Extensii

# Writing games pane: self-imposed drafting constraints
settings-group-games-forward = Mereu înainte
settings-games-forward-toggle = Joacă „Mereu înainte”
settings-games-forward-hint =
       Cât timp jucați, nimic din ce ați scris nu mai poate fi retras: Backspace, Delete,
       Decupează, tragerea cu mausul și Anulează sunt dezactivate în zonele alese mai jos.
       Puteți în continuare să tastați, să lipiți, să formatați și să vă deplasați liber. Ciorna doar crește.
settings-games-session-warning =
       Este o alegere valabilă doar pentru sesiunea curentă: nu este salvată niciodată. Închiderea
       proiectului sau ieșirea din Skribisto încheie mereu jocul, iar tot ce ați scris jucând
       poate fi din nou anulat din acel moment.
settings-group-games-scope = Unde se aplică
settings-games-in-prose = Proza manuscrisului
settings-games-in-synopsis = Sinopsisuri
settings-games-scope-hint =
       Comentariile, notele de subsol și titlurile nu sunt înghețate niciodată: acolo notați
       corectura pe care tocmai v-ați interzis să o faceți.
settings-games-inert-warning =
       „Mereu înainte” este activ, dar nu se aplică nicăieri. Bifați mai sus cel puțin o zonă,
       altfel nu schimbă nimic în timp ce scrieți.

# Writing games dock (leading rail) + the status-bar warning while a game is on
games-title = Jocuri de scriere
games-forward-name = Mereu înainte
games-forward-blurb =
       Scrieți ciorna fără să retrageți nimic. Ștergerea, decuparea și anularea sunt dezactivate
       cât timp jucați. Scrieți propoziția următoare în loc să o corectați pe ultima.
games-forward-playing = În joc: ștergerea este dezactivată
games-forward-idle = Nu este activ
games-session-note = Se încheie când închideți proiectul.
games-scope-prose-and-synopsis = Se aplică prozei și sinopsisurilor.
games-scope-prose = Se aplică prozei.
games-scope-synopsis = Se aplică sinopsisurilor.
games-scope-nothing = Încă nu se aplică nicăieri. Alegeți o zonă în Setări.
games-settings-link = Setările jocurilor de scriere…
statusbar-games-forward = Mereu înainte
statusbar-games-forward-tooltip =
       „Mereu înainte” este activ: ștergerea, decuparea și anularea sunt dezactivate cât timp
       scrieți. Clic pentru a opri jocul.
# ── Word / character targets ──────────────────────────────────────────────────
# The shared vocabulary every surface that shows a target uses: the Inspector, the
# Overview column, the status bar, a container page and the Distribute preview. Plurals
# select on the bare number (`$g` / `$n`) while the visible figure is the grouped string,
# because a count printed with thin spaces is no longer a number Fluent can select on.
goal-progress-words = { $g ->
       [one] { $count } din { $goal } cuvânt
       [few] { $count } din { $goal } cuvinte
      *[other] { $count } din { $goal } de cuvinte
   }
goal-progress-characters = { $g ->
       [one] { $count } din { $goal } caracter
       [few] { $count } din { $goal } caractere
      *[other] { $count } din { $goal } de caractere
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

# The Inspector's per-item target.
inspector-goal = Țintă
inspector-goal-none = Fără țintă

# The Overview's target column, and the note on a row the export leaves out.
overview-col-goal = Țintă
overview-excluded-from-export = Exclus din export, deci nu contează la niciun total

# A container's own page: what the targets set inside it add up to. Deliberately not a
# target itself, and worded so it cannot be read as one.
goal-subtree-total-words = { $count ->
       [one] { $items } țintă în interior; total cuvinte: { $words }
       [few] { $items } ținte în interior; total cuvinte: { $words }
      *[other] { $items } de ținte în interior; total cuvinte: { $words }
   }
goal-subtree-total-characters = { $count ->
       [one] { $items } țintă în interior; total caractere: { $words }
       [few] { $items } ținte în interior; total caractere: { $words }
      *[other] { $items } de ținte în interior; total caractere: { $words }
   }

# The New Work panel's counting-unit picker, beside the language it is seeded from.
new-work-goal-unit = Numără în
new-work-goal-unit-hint = Folosit pentru toate țintele de cuvinte sau de caractere din acest proiect. Poate fi schimbat mai târziu.

# Settings ▸ Work ▸ Structure: the project's counting unit, and the warning shown before
# switching it. No conversion happens, in either direction.
settings-group-goal-unit = Ținte
settings-goal-unit-switch-title = Schimbarea unității de numărare
settings-goal-unit-switch-text = Țintele și jaloanele deja stabilite pentru acest proiect au fost introduse în { $from }. Trecerea la { $to } nu le convertește, așa că fiecare număr existent va fi citit de acum în { $to }.
settings-goal-unit-switch-informative = Nu se pierde nimic. Puteți reveni oricând pentru a recupera citirea inițială, apoi actualizați țintele pe care vreți să le păstrați.
settings-goal-unit-switch-confirm = Schimbă

# Distribute: sharing a container's target across the pieces inside it.
goal-distribute-action = Distribuie…
distribute-title = Distribuirea țintei
distribute-empty = Nu există nimic în interior între care să se împartă ținta.
distribute-weight = Împarte
distribute-weight-length = După lungime
distribute-weight-rows = După numărul de elemente
distribute-weight-even = În mod egal
distribute-overwrite = Înlocuiește țintele deja stabilite
distribute-col-item = Element
distribute-col-current = Acum
distribute-col-proposed = După
distribute-total = Acestea însumează { $total }, față de o țintă de { $goal }.
distribute-over-budget = Țintele deja stabilite în interior depășesc cu { $over } ținta acestui container. Măriți ținta lui, micșorați-le pe ale lor sau înlocuiți-le.
distribute-apply = Distribuie
distribute-cancel = Renunță

# Milestones on the Book's pace: a dated waypoint, of one of two kinds.
milestone-target-gone = Țintă ștearsă
milestone-no-target = Fără țintă
milestone-add = Adaugă
milestone-add-label = Ce trebuie atins

# The writing-plan summary shown once when a project with an active plan opens.
pace-summary-title = Unde a ajuns cartea
pace-summary-remaining = Mai sunt { $words }
pace-summary-open = Deschide planul
pace-summary-close = Închide
pace-summary-dont-show = Nu afișa la deschidere
# Work menu row that opens the same card on demand, greyed out when the project has
# no active writing plan.
pace-summary-menu = Planul de scriere…

# ── Help ─────────────────────────────────────────────────────────────────────
# The Help window (crates/teksilo_ui/src/help/), its table of contents and the
# keyboard-shortcuts sheet. Topic *bodies* are not here: they are Djot documents
# under crates/teksilo_ui/help/<locale>/, one file per topic per locale.
menu-help-topics = &Subiecte de ajutor
menu-help-shortcuts = Comenzi &rapide de la tastatură…
menu-help-website = Skribisto pe &web
menu-help-report = Raportează o pro&blemă…
menu-help-check-updates = Caută &actualizări
menu-command-palette = &Paleta de comenzi…

# The Learn pane's rows in the Launcher. Separate keys from the Help menu's rows
# on purpose: a menu label carries an `&` mnemonic, and a Button renders that
# ampersand literally ("Help &Topics"), which is what shipped for one build.
learn-help-topics = Subiecte de ajutor
learn-shortcuts = Comenzi rapide de la tastatură
learn-website = Skribisto pe web

help-window-title = Ajutor
help-filter-topics = Filtrează subiectele
help-no-matching-topic = Niciun subiect nu corespunde.
help-topic-missing = Acest subiect nu mai este disponibil.
help-back = Înapoi
help-not-translated = Această pagină nu a fost încă tradusă, așa că este afișată în engleză.

help-section-getting-started = Primii pași
help-section-writing = Scrierea
help-section-reviewing = Revizuirea
help-section-exchanging = Cum intră și ies textele
help-section-keeping = Siguranța muncii dumneavoastră
help-section-extensions = Extensii

help-topic-getting-started = Primul dumneavoastră proiect
help-topic-writing-model = Cum este alcătuită o carte
help-topic-drafts-and-old-versions = Păstrarea unei ciorne mai vechi
help-topic-goals-and-pace = Ținte și ritm
help-topic-comments = Comentarii
help-topic-round-trip = Trimiterea cărții către un cititor
help-topic-export = Exportul
help-topic-import-documents = Importul de documente
help-topic-import-projects = Preluarea unui proiect întreg
help-topic-import-scrivener = Trecerea de la Scrivener
help-topic-backups-and-versions = Copii de siguranță și versiuni

help-shortcuts-title = Comenzi rapide de la tastatură
help-shortcuts-filter = Filtrează comenzile rapide
help-shortcuts-no-matches = Nicio comandă rapidă nu corespunde.
help-shortcuts-rebind = Modifică comenzile rapide…
help-shortcuts-close = Închide

command-palette-placeholder = Tastați o comandă

# ── Shortcut names ───────────────────────────────────────────────────────────
# The user-visible name of every rebindable `Shortcut` (`Shortcut::name`,
# teksilo-core), reactive to a locale switch via `LocalizedString`/
# `Prop<String>`. Read by Settings ▸ Keymap, the Help ▸ Keyboard shortcuts
# sheet and the command palette. Kept in lockstep with the `Shortcut::new(...)`
# calls in `app/commands/*.rs`, `binder/dock.rs` and `welcome/panel.rs`. Where
# the same command already carries a menu label above, that exact wording is
# reused here so the menu and the shortcut list agree.
shortcut-name-binder-duplicate = Duplică
shortcut-name-comments-add = Adaugă un comentariu
shortcut-name-comments-add-paragraph = Comentează paragraful
shortcut-name-spellcheck-toggle = Verificare ortografică
shortcut-name-editor-tab-close = Închide fila
shortcut-name-editor-tab-pin = Fixează sau desprinde fila
shortcut-name-editor-save = Salvează
shortcut-name-work-export = Exportă…
shortcut-name-work-new = Proiect nou
shortcut-name-work-open = Deschide un proiect
shortcut-name-window-new = Fereastră nouă
shortcut-name-work-close = Închide proiectul
shortcut-name-work-import-scrivener = Importă un proiect Scrivener…
shortcut-name-work-import-scrivener-report = Arată raportul importului Scrivener
shortcut-name-app-settings = Setări
shortcut-name-settings-close = Închide setările
shortcut-name-app-quit = Ieșire
shortcut-name-editor-insert-footnote = Inserează o notă de subsol
shortcut-name-format-scene-break = Inserează un separator de scenă
shortcut-name-format-major-scene-break = Inserează un separator major de scenă
shortcut-name-format-link = Inserează o legătură
shortcut-name-format-mark-nbsp = Inserează un spațiu nedespărțitor
shortcut-name-format-mark-nbhyphen = Inserează o cratimă nedespărțitoare
shortcut-name-format-mark-nnbsp = Inserează un spațiu îngust nedespărțitor
shortcut-name-format-mark-zwsp = Inserează un spațiu de lățime zero
shortcut-name-go-next = Următorul
shortcut-name-go-prev = Precedentul
shortcut-name-go-to = Salt la
shortcut-name-outline-toggle = Afișează sau ascunde structura
shortcut-name-preview-toggle = Afișează sau ascunde banda de previzualizare
shortcut-name-view-fullscreen = Comută ecranul complet
shortcut-name-view-focus-mode = Comută modul fără distrageri
shortcut-name-editor-size-increase = Mărește textul
shortcut-name-editor-size-decrease = Micșorează textul
shortcut-name-editor-size-reset = Resetează mărimea textului
shortcut-name-editor-find = Caută
shortcut-name-editor-replace = Înlocuiește
shortcut-name-editor-find-next = Potrivirea următoare
shortcut-name-editor-find-prev = Potrivirea precedentă
shortcut-name-search-show = Caută în proiect
shortcut-name-search-replace = Înlocuiește în proiect
shortcut-name-outline-open-to-side = Deschide în lateral
shortcut-name-help-topics = Subiecte de ajutor
shortcut-name-help-close = Închide ajutorul
shortcut-name-help-shortcuts = Comenzi rapide de la tastatură
shortcut-name-help-website = Skribisto pe web
shortcut-name-help-report = Raportează o problemă
shortcut-name-help-check-updates = Caută actualizări
# Raised only by the Help menu command, which the reader asked for.
update-check-behind = Versiunea { $version } este disponibilă.
update-check-current = Folosiți cea mai nouă versiune.
update-check-failed = Nu s-a putut verifica dacă există o versiune mai nouă. { $error }
shortcut-name-command-palette = Paleta de comenzi
help-section-reference = Ce înseamnă fiecare lucru

# Titles for the glossary entries whose concept is not something you can create, so
# there is no "＋ Create" label to reuse. See help.rs::concept_topics.
help-concept-scene-break = Separator de scenă
help-concept-major-scene-break = Separator major de scenă
help-concept-find-in-prose = Căutare în proză
help-concept-story-bible = Biblia poveștii
help-concept-goal-unit = Cuvinte sau caractere
help-concept-goal-progress = Progres
help-concept-manuscript-words = Ce intră în manuscris
help-concept-exportable = Exclus din export
help-concept-distribute = Împărțirea unei ținte
help-concept-subtree-total = Cât însumează țintele din interior
help-concept-milestone = Jalon
help-concept-pace-plan = Planul de ritm

# Glossary titles for the feature-concept web. See help.rs::concept_topics.
help-concept-tag = Etichetă
help-concept-label = Legendă
help-concept-point-of-view = Punct de vedere
help-concept-epigraph = Motto
help-concept-footnote = Notă de subsol
help-concept-chapter-mode = Forma capitolelor
help-concept-comment = Comentariu
help-concept-backup = Copie de siguranță
help-concept-version = Versiune
help-concept-trash = Coș de gunoi
help-concept-spellcheck = Verificare ortografică
help-concept-search-replace = Căutare și înlocuire
help-concept-note-template = Șablon de notă
help-concept-story-bible-entry = Intrare în biblia poveștii
help-concept-text-replacement = Înlocuire de text
help-concept-smart-punctuation = Punctuație inteligentă
help-concept-export-style = Stil de export
help-concept-round-trip-marks = Marcaje de dus-întors

# The epigraph's attribution control. See tabs/shared/panes.rs::attribution_control.
epigraph-mark-attribution = Rândul sursei
epigraph-mark-attribution-tip = Marchează rândul în care se află cursorul drept sursa citatului, ca să fie tipărit ca atribuire
## Margin lane: the strip beside the scrollbar that maps a document
margin-lane-name = Bandă de margine
margin-lane-provider-comments = Comentarii
margin-lane-provider-story-bible = Intrare în biblia poveștii
margin-lane-provider-story-bible-hint = Pe pagina „În proză” a unei note: unde este numită acea intrare și scenele povestite prin ochii ei. Marchează și numele respective în proza propriu-zisă.
margin-lane-mark-point-of-view = Povestit de aici
margin-lane-mark-named-here = Numit aici
margin-lane-provider-comments-hint = Unde este atașat un comentariu la text
margin-lane-provider-search = Rezultatele căutării
margin-lane-provider-search-hint = Toate potrivirile pentru ultima căutare, oriunde s-ar afla în document
margin-lane-provider-boundaries = Unde începe fiecare document
margin-lane-provider-boundaries-hint = O linie la începutul fiecărei scene dintr-un flux, ca să vedeți unde vă aflați
margin-lane-provider-spelling = Ortografie
margin-lane-provider-spelling-hint = Unde verificarea ortografică a semnalat un cuvânt. Dezactivat dacă nu cereți: marchează părerea unei mașini despre proza dumneavoastră
margin-lane-spelling = { $word }, posibilă greșeală de ortografie
margin-lane-search-hit = { $text }, potrivirea { $index } din { $total }
margin-lane-search-current = { $text }, potrivirea { $index } din { $total }, cea pe care vă aflați
margin-lane-boundary = Începutul lui „{ $title }”
margin-lane-boundary-untitled = un document fără titlu
settings-page-margin-lane = Bandă de margine
settings-desc-margin-lane = Banda de lângă bara de derulare și ce afișează ea
settings-margin-lane-enabled = Afișează banda de margine
settings-margin-lane-enabled-hint = Același comutator ca Vizualizare ▸ Bandă de margine.
settings-margin-lane-enabled-more = Nu spune niciodată că ceva este greșit: vă arată unde se află lucrurile și vă lasă pe dumneavoastră să trageți concluziile.
settings-group-margin-lane-marks = Ce marchează
settings-margin-lane-no-providers = Încă nu marchează nimic banda.
settings-group-margin-lane-texture = Textura dialogului
settings-margin-lane-texture = Afișează textura dialogului
settings-margin-lane-texture-hint = O bară pentru fiecare paragraf: cât de lung este și cât din el este vorbire.
settings-margin-lane-texture-more = Nu se măsoară pentru limbile fără o convenție stabilită, unde bara arată doar lungimea.
settings-group-margin-lane-surfaces = Unde apare
settings-margin-lane-surface-editor = Editorul de text
settings-margin-lane-surface-stream = Fluxurile
settings-margin-lane-surface-search-preview = Previzualizarea căutării
menu-margin-lane = &Bandă de margine

## Statuses: the project's workflow ladder.
## A rung's name is project DATA: these are resolved once, when a ladder is seeded, and
## stored literally from then on. They are not re-translated afterwards, and the writer is
## free to rename any of them.
status-preset-drafting = Scrierea ciornei
status-preset-passes = Etape de revizuire
status-preset-plume = Plume Creator
status-todo = De făcut
status-draft = Ciornă
status-revised = Revizuit
status-final = Final
status-outline = Schiță
status-first-edit = Prima revizuire
status-second-edit = A doua revizuire
status-done = Gata
status-plume-draft-1 = Prima ciornă
status-plume-draft-2 = A doua ciornă
status-plume-draft-3 = A treia ciornă
status-plume-edit-1 = Prima revizuire
status-plume-edit-2 = A doua revizuire
status-plume-edit-3 = A treia revizuire
status-plume-proofread = Corectură făcută
status-plume-finished = Terminat
new-work-statuses = Etape de lucru
new-work-statuses-hint = Etapele prin care trece o scenă. Le puteți redenumi, reordona sau completa oricând.
status-none = Fără stare
inspector-status = Stare
overview-col-status = Stare
overview-status-mixed = părțile de dedesubt diferă
help-concept-status = Stare
status-completion-title = Unde a ajuns cartea
status-completion-headline = { $done } din { $total } scene terminate
status-completion-empty = Încă nu există scene. Aceasta citește manuscrisul, deci se completează pe măsură ce scrieți.
status-completion-open = Unde a ajuns cartea…

## Settings ▸ Work ▸ Statuses: the ladder editor.
## The writer owns a rung's NAME and the ladder's ORDER; the app owns its category, and
## the category owns the glyph and the colour. Only the category names below are
## translated: a rung's name is the writer's own text and stays as they typed it.
settings-page-statuses = Stări
settings-desc-statuses = Redenumiți, reordonați, adăugați și eliminați etapele prin care trece o scenă
settings-statuses-add = Adaugă o stare
settings-statuses-add-placeholder = Numele noii stări
settings-statuses-desc =
       Etapele prin care trece o scenă, în ordine, de la cea mai puțin avansată, sus, la cea mai
       avansată, jos. Această ordine este ceea ce înseamnă „mai puțin avansat decât” peste tot
       în aplicație, așa că așezați-le în ordinea în care se desfășoară de fapt procesul dumneavoastră.
settings-statuses-apply-preset = Aplică un set predefinit…
settings-statuses-preset-applied = { $added ->
       [one] O stare adăugată
       [few] { $added } stări adăugate
      *[other] { $added } de stări adăugate
   }
settings-statuses-preset-refused = Acest proiect are deja o scară de stări. Seturile predefinite completează doar una goală. Ștergeți mai întâi treptele pe care nu le vreți.
settings-statuses-duplicate = „{ $name }” există deja pe această scară
settings-statuses-added = A fost adăugată „{ $name }”
settings-statuses-move-up = Mută în sus (mai puțin avansat)
settings-statuses-move-down = Mută în jos (mai avansat)
settings-statuses-details-placeholder = Ce înseamnă această etapă (opțional)
settings-statuses-delete = Șterge „{ $name }”
settings-statuses-delete-in-use = { $count ->
       [one] Șterge „{ $name }”: un element o are și își va pierde starea
       [few] Șterge „{ $name }”: { $count } elemente o au și își vor pierde starea
      *[other] Șterge „{ $name }”: { $count } de elemente o au și își vor pierde starea
   }
settings-statuses-deleted = A fost ștearsă „{ $name }”
settings-statuses-deleted-in-use = { $count ->
       [one] A fost ștearsă „{ $name }”. Un element nu mai are nicio stare.
       [few] A fost ștearsă „{ $name }”. { $count } elemente nu mai au nicio stare.
      *[other] A fost ștearsă „{ $name }”. { $count } de elemente nu mai au nicio stare.
   }
settings-statuses-empty-title = Încă nu există etape de lucru
settings-statuses-empty-body = O stare arată cât de avansată este o scenă. Adăugați una mai sus sau porniți de la un set predefinit.
status-category-planned = Planificat
status-category-drafting = Ciornă în lucru
status-category-needs-work = De lucrat
status-category-revised = Revizuit
status-category-final = Final

## Long-operation progress (shared::progress_text). The backend reports phase
## codes it cannot translate; these are their labels. `progress-percent` is the
## toast body on its own, `progress-percent-with-label` with the phase after it.
progress-percent = { $percent }%
progress-percent-with-label = { $percent }% · { $label }
progress-import-opening = Se deschide proiectul…
progress-import-outline = Se citește structura…
progress-import-converting = Se convertesc capitolele și scenele…
progress-import-writing = Se scrie fișierul .skrib…
progress-import-done = Gata
progress-import-structure = Se stabilește structura…
progress-export-start = Se pornește…
progress-export-writing = Se scrie…
progress-project-reading = Se citește proiectul…
# Settings: the open project's section title, "Work: <its title>".
settings-sec-work-titled = Proiect: { $title }

## Project-import notices (shared::import_warnings::notices): one line of the
## Plume / Manuskript warnings list each. $file, $folder, $title, $name, $field
## and $error are data (file names, titles, a parser's own message) and stay as
## they are. $container is "single-file" or "folder"; $kind is "scene", "note",
## "synopsis" or "title".
import-notice-flattened-rich-texts = { $count ->
    [one] Un text era imbricat mai adânc decât poate conține un proiect Skribisto; cuvintele lui au fost păstrate ca text simplu, fără formatare.
    [few] { $count } texte erau imbricate mai adânc decât poate conține un proiect Skribisto; cuvintele lor au fost păstrate ca text simplu, fără formatare.
   *[other] { $count } de texte erau imbricate mai adânc decât poate conține un proiect Skribisto; cuvintele lor au fost păstrate ca text simplu, fără formatare.
}
import-notice-free-text-badges = { $count ->
    [one] O insignă de scenă pare text liber, nu o listă de stări, așa că a fost păstrată ca legendă a elementului în loc să devină etichetă.
    [few] { $count } insigne de scenă diferite par text liber, nu o listă de stări, așa că au fost păstrate ca legende ale elementelor în loc să devină etichete.
   *[other] { $count } de insigne de scenă diferite par text liber, nu o listă de stări, așa că au fost păstrate ca legende ale elementelor în loc să devină etichete.
}
import-notice-separator-text-dropped = Separatorul „{ $name }” conținea text, care nu are loc într-un proiect Skribisto; textul nu a fost importat.
import-notice-separator-without-scene = Separatorul „{ $name }” nu avea înaintea lui, în capitolul său, nicio scenă de care să fie atașat, așa că nu a fost importat.
import-notice-chapter-without-scenes = Capitolul „{ $name }” nu are scene, așa că separatoarele lui nu au fost importate.
import-notice-rich-text-unconverted = O parte din textul formatat nu a putut fi convertită ({ $error }); nu a fost importată.
import-notice-unresolved-cross-link = O legătură către intrarea de distribuție nr. { $number } nu a putut fi urmată: intrarea lipsește sau era în coșul de gunoi.
import-notice-read-copy = A fost citită copia { $container ->
    [single-file] cu fișier unic
   *[folder] de tip dosar
} a acestui proiect.
import-notice-read-copy-dated = A fost citită copia { $container ->
    [single-file] cu fișier unic
   *[folder] de tip dosar
} a acestui proiect, modificată ultima dată la { $day }.
import-notice-original-format = Acest proiect este în formatul original Manuskript din 2016. A fost citit integral; nimic din el nu a fost modificat.
import-notice-newer-single-file-beside = „{ $file }” este mai nou decât acest dosar ({ $file_day } față de { $folder_day }). Manuskript lasă dosarul vechi pe loc când un proiect trece la modul cu fișier unic, așa că acest dosar poate fi o copie abandonată; importul fișierului .msk ar citi în schimb versiunea mai nouă.
import-notice-pickle-skipped = „{ $file }” nu a fost citit. Este un fișier Python pickle, iar încărcarea unuia dintr-un fișier de proiect era o vulnerabilitate de securitate chiar în Manuskript (CVE-2021-35196); setările pe care le conține sunt preferințe de afișare, nu text.
import-notice-file-unreadable = „{ $file }” nu a putut fi citit ({ $error }) și a fost omis; tot restul a fost importat.
import-notice-not-utf8 = „{ $file }” nu este UTF-8 valid; caracterele care nu au putut fi citite au fost înlocuite, în loc să fie oprit importul.
import-notice-table-unreadable = „{ $file }” nu a putut fi citit ({ $error }); conținutul lui nu a fost importat.
import-notice-outline-missing = „{ $file }” lipsește, așa că acest proiect nu are niciun manuscris de importat.
import-notice-outline-unreadable = „{ $file }” nu a putut fi citit ({ $error }); nu a fost importat niciun manuscris.
import-notice-named-none-row = „{ $file }” începe cu un rând cu nume („{ $name }”) acolo unde Manuskript își ține intrarea goală „none”. A fost omis, astfel încât rândurile de după el își păstrează numerele folosite de manuscris.
import-notice-unknown-field = „{ $file }” are un câmp pe care acest importator nu îl cunoaște, „{ $field }”. Nu a fost importat.
import-notice-colorless-labels = { $count ->
    [one] „{ $file }” are un rând fără culoare. A fost importat fără culoare; Manuskript se oprește din încărcarea proiectului la un asemenea rând.
    [few] „{ $file }” are { $count } rânduri fără culoare. Au fost importate fără culoare; Manuskript se oprește din încărcarea proiectului la un asemenea rând.
   *[other] „{ $file }” are { $count } de rânduri fără culoare. Au fost importate fără culoare; Manuskript se oprește din încărcarea proiectului la un asemenea rând.
}
import-notice-settings-unreadable = „{ $file }” nu a putut fi citit ({ $error }); limba proiectului nu a fost importată.
import-notice-revisions-unreadable = „{ $file }” nu a putut fi citit ({ $error }); istoricul proiectului nu a fost importat, dar nimic altceva nu a fost afectat.
import-notice-foreign-files = { $count ->
    [one] Acest proiect conține un fișier care nu face parte din formatul Manuskript și nu a fost importat: { $files }.
    [few] Acest proiect conține { $count } fișiere care nu fac parte din formatul Manuskript și nu au fost importate: { $files }.
   *[other] Acest proiect conține { $count } de fișiere care nu fac parte din formatul Manuskript și nu au fost importate: { $files }.
}
import-notice-foreign-files-more = Acest proiect conține fișiere care nu fac parte din formatul Manuskript și nu au fost importate (în total: { $count }): { $files } și { $more ->
    [one] încă unul
    [few] încă { $more }
   *[other] încă { $more }
}.
import-notice-not-a-character-file = „{ $file }” nu este un fișier de personaj Manuskript și a fost omis.
import-notice-not-a-text-item = „{ $file }” nu este un element de text Manuskript și a fost omis din structură.
import-notice-missing-blank-line = „{ $file }” nu are un rând gol după antet. Ultimul lui câmp a fost păstrat, deși Manuskript îl pierde.
import-notice-folder-without-metadata = „{ $folder }” nu are { $file }, așa că a fost importat ca dosar simplu, numit după director. Manuskript omite un asemenea director și tot ce conține.
import-notice-rows-given-ids = { $count ->
    [one] Un rând din structură nu avea un ID propriu și a primit unul. Manuskript elimină un asemenea rând sau se oprește din încărcarea proiectului dacă rândul este un dosar.
    [few] { $count } rânduri din structură nu aveau un ID propriu și au primit câte unul. Manuskript elimină un asemenea rând sau se oprește din încărcarea proiectului dacă rândul este un dosar.
   *[other] { $count } de rânduri din structură nu aveau un ID propriu și au primit câte unul. Manuskript elimină un asemenea rând sau se oprește din încărcarea proiectului dacă rândul este un dosar.
}
import-notice-plots-unreadable = „{ $file }” nu a putut fi citit ({ $error }); intrigile nu au fost importate.
import-notice-world-unreadable = „{ $file }” nu a putut fi citit ({ $error }); intrările despre lume nu au fost importate.
import-notice-world-not-opml = „{ $file }” nu este un document OPML; intrările despre lume nu au fost importate.
import-notice-text-flattened = Textul lui „{ $title }” este imbricat mai adânc decât poate conține un proiect Skribisto; cuvintele lui au fost păstrate ca text simplu, fără formatare.
import-notice-text-unconverted = Textul lui „{ $title }” nu a putut fi convertit ({ $error }); a fost păstrat exact cum a fost scris.
import-notice-text-without-file-name = { $kind ->
    [scene] Textul de scenă
    [note] Textul de notă
    [synopsis] Sinopsisul
   *[title] Titlul
} lui „{ $title }” nu a putut primi un nume de fișier, așa că nu a fost importat.
import-notice-unknown-label = „{ $title }” are legenda { $index }, care nu se află în lista de legende a acestui proiect. A fost importat fără legendă.
import-notice-unknown-status = „{ $title }” are starea { $index }, care nu se află în lista de stări a acestui proiect. A fost importat fără stare.
import-notice-unknown-point-of-view = „{ $title }” este povestit prin ochii unui personaj care nu este în distribuția acestui proiect. Punctul lui de vedere nu a fost setat.
import-notice-unresolved-references = Trimiterile din textul lui „{ $title }” nu au putut fi rezolvate ({ $error }); textul a fost păstrat așa cum a fost convertit, inclusiv marcajele de trimitere.
import-notice-custom-icons = { $count ->
    [one] Un rând avea o pictogramă personalizată. Skribisto dă fiecărui rând pictograma tipului său, așa că pictograma nu a fost importată; nimic altceva nu s-a schimbat la acel rând.
    [few] { $count } rânduri aveau o pictogramă personalizată. Skribisto dă fiecărui rând pictograma tipului său, așa că pictogramele nu au fost importate; nimic altceva nu s-a schimbat la acele rânduri.
   *[other] { $count } de rânduri aveau o pictogramă personalizată. Skribisto dă fiecărui rând pictograma tipului său, așa că pictogramele nu au fost importate; nimic altceva nu s-a schimbat la acele rânduri.
}
import-notice-character-colors = { $count ->
    [one] Un personaj avea o culoare lângă nume. Skribisto colorează etichetele, nu rândurile, așa că culoarea nu a fost importată; personajul însă a fost importat.
    [few] { $count } personaje aveau o culoare lângă nume. Skribisto colorează etichetele, nu rândurile, așa că culorile nu au fost importate; personajele însă au fost toate importate.
   *[other] { $count } de personaje aveau o culoare lângă nume. Skribisto colorează etichetele, nu rândurile, așa că culorile nu au fost importate; personajele însă au fost toate importate.
}
import-notice-versions-kept = { $found ->
    [one] Acest proiect a înregistrat o versiune anterioară
    [few] Acest proiect a înregistrat { $found } versiuni anterioare
   *[other] Acest proiect a înregistrat { $found } de versiuni anterioare
}; { $kept ->
    [one] a fost păstrată una
    [few] au fost păstrate { $kept }
   *[other] au fost păstrate { $kept }
}, rărite așa cum își rărește Skribisto propriul istoric. Se află în panoul Versiuni.
import-notice-versions-unplaced = { $count ->
    [one] O versiune anterioară aparține unui rând care nu mai există în proiect și nu a putut fi plasată.
    [few] { $count } versiuni anterioare aparțin unui rând care nu mai există în proiect și nu au putut fi plasate.
   *[other] { $count } de versiuni anterioare aparțin unui rând care nu mai există în proiect și nu au putut fi plasate.
}

## Native file dialogs: the window title, and the label of the file-type filter.
dialog-open-work-title = Deschide un proiect Skribisto
dialog-save-as-file-title = Salvează ca fișier .skrib unic
dialog-save-as-folder-title = Alegeți dosarul părinte al proiectului
dialog-manuskript-file-title = Alegeți un proiect Manuskript
dialog-manuskript-folder-title = Alegeți dosarul unui proiect Manuskript
dialog-plume-file-title = Alegeți un proiect Plume Creator
dialog-filter-skrib = Proiect Skribisto
dialog-filter-manuskript = Proiect Manuskript
dialog-filter-plume = Proiect Plume Creator
dialog-filter-hunspell-aff = Fișier de afixe Hunspell (.aff)
dialog-filter-hunspell-dic = Dicționar Hunspell (.dic)
dialog-filter-export = Fișier exportat
# Help ▸ Keyboard shortcuts / Settings ▸ Keymap / command palette.
shortcut-name-view-margin-lane = Afișează sau ascunde banda de margine

## The built-in distraction-free themes' names (distraction_free::theme).
df-theme-paper = Hârtie
df-theme-sepia = Sepia
df-theme-night = Noapte
df-theme-high-contrast = Contrast ridicat
df-theme-midnight = Miez de noapte

## Welcome ▸ Examples: the line under a bundled example's title.
welcome-example-byline = de { $author }

## Spoken by a screen reader only.
# Welcome window: the name of the section tab bar.
welcome-sections-a11y = Secțiunile ferestrei de bun venit
# A comment's anchored span in the editor: author, plain-text preview, replies.
comments-a11y-summary-author = { $author }: { $text }
comments-a11y-summary-replies = { $text } ({ $count ->
    [one] un răspuns
    [few] { $count } răspunsuri
   *[other] { $count } de răspunsuri
})
comments-a11y-summary-author-replies = { $author }: { $text } ({ $count ->
    [one] un răspuns
    [few] { $count } răspunsuri
   *[other] { $count } de răspunsuri
})
# Margin marks beside the text: one mark, several merged into one, and the
# lane's own name when nothing more specific was given.
margin-lane-a11y-mark = { $label }, la { $percent } la sută din text
margin-lane-a11y-marks = { $label }, { $count ->
    [one] un marcaj
    [few] { $count } marcaje
   *[other] { $count } de marcaje
}, la { $percent } la sută din text
margin-lane-a11y-fallback = { $count ->
    [one] Bandă de margine, un marcaj
    [few] Bandă de margine, { $count } marcaje
   *[other] Bandă de margine, { $count } de marcaje
}
# Export ▸ Choose: the row standing for a binder that has no name.
export-choose-unnamed-binder = Clasor fără nume
# The built-in export styles named in words (the national manuscript presets
# keep their own language's name).
export-style-neutral = Neutru
export-style-manuscript-shunn = Manuscris standard (Shunn)
export-style-ebook-clean = Carte electronică simplă

## Dates. Put the parts in the order your language writes a short date; the
## parts arrive already zero-padded where that matters.
# Comment cards, the outline card's Created/Modified.
date-time-short = { $day }.{ $month }.{ $year } { $hour }:{ $minute }
# Pace chart: a day on the axis, a milestone's date, a holiday.
pace-day-label = { $day }.{ $month }

## File sizes (shared::byte_size). $whole and $tenth are the two sides of the
## decimal mark: write your language's mark between them and its unit symbol.
byte-size-bytes = { $value } B
byte-size-kb = { $value } KB
byte-size-mb = { $whole },{ $tenth } MB
byte-size-gb = { $whole },{ $tenth } GB
# A whole percentage (sliders, the Pace card, the text-size toast).
unit-percent = { $value }%
# A whole number of pixels (slider readouts).
unit-pixels = { $value } px
# Settings ▸ Paratext: a preset saved without a `name` line. `name` is the TOML
# key the writer types and stays as it is.
settings-paratext-error-no-name = O schemă are nevoie de o linie `name`.
# Export: started with no project open (a menu raced a close).
export-error-no-project = Niciun proiect deschis pentru export.

shortcut-group-work = Proiect
shortcut-group-edit = Editare
shortcut-group-view = Vizualizare
shortcut-group-document = Document
shortcut-group-format = Formatare
shortcut-group-go = Navigare
shortcut-group-tools = Instrumente
shortcut-group-help = Ajutor

## Manuskript import: the headings of the notes it writes (a character's
## sheet, a world entry, a plot and its steps, the project details, the
## snowflake summary ladder) and the title of a project with no name.
import-manuskript-section-paragraph-summary = Rezumat într-un paragraf
import-manuskript-section-full-summary = Rezumat complet
import-manuskript-section-motivation = Motivație
import-manuskript-section-epiphany = Revelație
import-manuskript-section-notes = Note
import-manuskript-section-description = Descriere
import-manuskript-section-passion = Pasiune
import-manuskript-section-result = Rezultat
import-manuskript-section-summary = Rezumat
import-manuskript-section-meta = Însemnări
import-manuskript-section-series = Serie
import-manuskript-section-volume = Volum
import-manuskript-section-genre = Gen
import-manuskript-section-license = Licență
import-manuskript-section-email = E-mail
# The snowflake ladder's five tellings, shortest first.
import-manuskript-section-situation = Situație
import-manuskript-section-sentence = O frază
import-manuskript-section-paragraph = Un paragraf
import-manuskript-section-page = O pagină
import-manuskript-section-full = Rezumat complet
import-manuskript-untitled-project = Proiect Manuskript importat

## Plume import: the story-bible groups an old project implies, and the title
## of a project with no name.
import-plume-group-characters = Personaje
import-plume-group-items = Obiecte
import-plume-group-places = Locuri
import-plume-untitled-project = Proiect Plume importat
