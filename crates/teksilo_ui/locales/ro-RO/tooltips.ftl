# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Skribisto: writing-model rich tooltips (source locale).
# Registered in tooltip_registry.rs and attached by key from the "＋ Create" /
# "Convert to" menus. The wm-*-more bodies cascade to one another via
# [label](:key) links, so this file doubles as an in-place Help doc.

wm-book = O carte din proiectul dumneavoastră.
wm-book-more = O carte poate fi oricât de simplă sau de stratificată doriți. O povestire scurtă poate avea doar câteva [scene](:wm-scene). O lucrare mai lungă poate imbrica [părți](:wm-part) și [capitole](:wm-chapter) pe oricâte niveluri. Forma vă aparține.
wm-part = O parte care grupează capitole.
wm-part-more = O parte reunește [capitole](:wm-chapter) sub un singur titlu. Ca un capitol, ține până la partea următoare sau până la sfârșitul [cărții](:wm-book).
wm-chapter = Un capitol al cărții.
wm-chapter-more = Un capitol ține de aici până la capitolul următor, până la [partea](:wm-part) următoare sau până la [sfârșitul cărții](:wm-end-of-book). Are două forme: un capitol-dosar care conține [scene](:wm-scene) sau un capitol simplu care are propria proză. Alegeți forma folosită de capitolele noi în Setări, la Alcătuire.
wm-scene = O scenă de proză.
wm-scene-more = O scenă conține textul dumneavoastră. Fiecare element de scris are două fețe: textul principal și un [sinopsis](:wm-synopsis) pentru planificare. Amândouă se deschid împreună în editor.
wm-note = O notă.
wm-note-more = O notă păstrează text și un [sinopsis](:wm-synopsis), ca o [scenă](:wm-scene), pentru documentare, digresiuni sau mementouri. Orice element poate face parte din cartea compilată, inclusiv o notă aflată într-un [capitol](:wm-chapter).
wm-note-folder = O notă care conține alte elemente.
wm-note-folder-more = O [notă](:wm-note) care conține și elemente sub ea, ca să puteți grupa notele înrudite. Își păstrează propriul [sinopsis](:wm-synopsis), iar elementele din interior își păstrează propriul text.
wm-folder = Un dosar de organizare.
wm-folder-more = Un container pentru a aranja elementele în clasor cum vă convine. Are un [sinopsis](:wm-synopsis), dar nu are proză proprie. Elementele din interior conțin textul.
wm-end-of-book = Marchează unde se termină o carte.
wm-end-of-book-more = Toate cărțile împart o singură listă continuă, așa că sfârșitul unei cărți nu este stabilit prin imbricare. Acest marcaj spune că [cartea](:wm-book) se oprește aici. Tot ce urmează după el aparține cărții următoare.
wm-story-bible-entry = O notă cu etichete și nume alternative despre povestea dumneavoastră.
wm-story-bible-entry-more = O [notă](:wm-note) despre un personaj, un loc sau orice altceva merită consemnat, creată într-un singur pas cu nume, etichete, nume alternative și un șablon de pornire. Ulterior, nimic de pe pagină nu o mai deosebește: rămâne o notă obișnuită, pe care o puteți redenumi, reeticheta sau converti ca pe oricare alta.
wm-synopsis = Un rezumat al unui element de scris.
wm-synopsis-more = Un rezumat atașat oricărui element de scris, de obicei un paragraf scurt, dar oricât de lung doriți. Folosiți-l pentru a planifica și a vă orienta înainte să existe proza. Stă lângă textul principal în editor.

scene-break-minor = Un separator de scenă obișnuit: o schimbare de timp, de loc sau de punct de vedere în cadrul unui capitol.
scene-break-minor-more =
    Marchează o separare *acolo unde o puneți*, inclusiv în mijlocul unei [scene](:wm-scene). Împărțirea prozei în două elemente este o alegere de organizare, așa că nu creează niciodată singură o separare.

    Se tastează în proză ca trei asteriscuri despărțite de spații. Felul în care este tipărit îl decide stilul de export, nu ce tastați: un manuscris Shunn îl redă ca un singur #, o ediție comercială ca un dinkus (trei steluțe), iar majoritatea editurilor franceze, germane, spaniole, rusești și italiene ca un simplu spațiu liber, fără niciun semn.
scene-break-major = O diviziune mai puternică: un salt mare în timp sau o schimbare decisivă de punct de vedere.
scene-break-major-more =
    Aceeași idee ca separatorul de scenă obișnuit, cu o treaptă mai sus. Folosiți-l când o separare simplă ar minimaliza saltul.

    Se tastează ca # # # și este tipărit de stilul de export altfel decât nivelul obișnuit. Formatul standard de manuscris Shunn pune un singur # față de # # # exact pentru această distincție. Acolo unde o tradiție nu are un semn mai puternic, ambele niveluri se tipăresc la fel.
wm-find-in-prose = Skribisto caută acest element în proza dumneavoastră.
wm-find-in-prose-more = Activați opțiunea pentru etichetele care numesc lucruri despre care scrieți: personaje, locuri, obiecte. Orice element care poartă o asemenea etichetă este căutat în proză după titlul lui și după celelalte nume pe care i le dați, astfel încât fiecare [scenă](:wm-scene) arată cine și ce apare în ea fără să legați nimic de mână. Lăsați opțiunea dezactivată pentru etichetele care descriu un element în loc să-l numească, cum ar fi o etapă de lucru sau un memento de verificat continuitatea. Fiecare element găsit intră și în [biblia poveștii](:wm-story-bible).
wm-story-bible = Toate personajele, locurile și celelalte intrări etichetate, adunate într-o singură grilă.
wm-story-bible-more = Se deschide din fila proprie a oricărui dosar de note. Fișele sunt grupate după etichetă și fiecare arată la câte nume alternative răspunde intrarea și în câte scene a apărut, pe baza tuturor etichetelor cu [căutare în proză](:wm-find-in-prose) din proiect. Filtrarea după carte doar restrânge fișele afișate; nu decide niciodată ce intrări există.

tooltip-go-to = Salt la orice element din clasor (Ctrl+G)
synopsis-collapse-tooltip = Ascunde coloana sinopsisului

wm-paratext = Un text care nu face parte din poveste.
wm-paratext-more = O prefață, o dedicație, o postfață, un colofon: texte care aparțin cărții, dar nu corpului ei. Exportat oriunde îl puneți și niciodată numărat în numărul de cuvinte al manuscrisului. Unde îl puneți depinde de dumneavoastră: convențiile diferă de la o țară și de la o editură la alta.
wm-paratext-folder = Un dosar pentru paratexte.
wm-paratext-folder-more = Un loc unde să păstrați prefețele și postfețele, ca să nu aglomereze clasorul. Doar de organizare: are un [sinopsis](:wm-synopsis), dar nu adaugă nimic cărții exportate, nici măcar propriul nume.

# ── Word / character targets ──────────────────────────────────────────────────
# A second cascade web beside the writing-model one. It doubles as the app's only
# explanation of how counting works, since there is no Help surface anywhere else.
goal-target = Cât de lung ar trebui să fie acest fragment.
goal-target-more = Stabiliți o țintă pentru orice: o [scenă](:wm-scene), un [capitol](:wm-chapter), o [parte](:wm-part), întreaga [carte](:wm-book). Fiecare este un număr de sine stătător, așa că ținta unui capitol nu o schimbă niciodată pe cea a cărții, iar ținta unei cărți nu este niciodată suma capitolelor din ea. Ce se adună este textul scris: [progresul](:goal-progress) unui container este tot ce s-a scris în el. O țintă lăsată la zero înseamnă pur și simplu că nu este stabilită. Numărul se socotește în [cuvinte sau caractere](:goal-unit), după cum folosește acest proiect.
goal-unit = Dacă acest proiect numără în cuvinte sau în caractere.
goal-unit-more = O alegere la nivelul întregului proiect, pentru că unitatea de lungime ține de manuscris și de piața lui, nu de o secțiune: editurile germane și franceze măsoară în semne, cele japoneze în coli de 400 de caractere, cele chineze în mii de caractere, iar majoritatea editurilor de limbă engleză în cuvinte. Ambele numere sunt păstrate unul lângă altul și niciunul nu este convertit vreodată în celălalt, așa că schimbarea unității îndreaptă fiecare [țintă](:goal-target) spre cealaltă cifră, iar revenirea o restabilește pe prima. Ce nu face este să convertească ceva, așa că țintele deja introduse vor trebui actualizate de mână.
goal-progress = Cât s-a scris, față de țintă.
goal-progress-explained = Un fragment cu proză proprie este măsurat după propria lungime; un container, după [tot ce s-a scris în el](:goal-manuscript-words). Bara trece de la roșu la chihlimbariu și apoi la verde pe măsură ce textul se adună și se schimbă din nou când fragmentul a depășit cu mult ce s-a planificat, lucru bun de observat când un capitol are o lungime de respectat.
goal-manuscript-words = Ce intră în manuscris.
goal-manuscript-words-more = Doar proza care ar fi efectiv exportată: [scenele](:wm-scene) și [capitolele](:wm-chapter) care au text propriu. O [notă](:wm-note) sau un [paratext](:wm-paratext) nu fac parte din manuscris și nu sunt numărate niciodată în el. Tot ce este în coșul de gunoi iese din calcul, la fel ca tot ce este [exclus din export](:goal-exportable).
goal-exportable = Acest element este exclus din export.
goal-exportable-more = Lungimea lui proprie este în continuare afișată, pentru că textul există, dar nu contează la niciun total, exact cum nu va apărea în nicio carte exportată. Excluderea unui container nu le exclude și pe elementele din el: fiecare element răspunde pentru sine, iar pentru asta există butonul „Aplică și elementelor conținute” de lângă comutator.
goal-distribute = Împarte această țintă între elementele din interior.
goal-distribute-more = Împarte ținta unei [cărți](:wm-book) sau a unei [părți](:wm-part) între copiii ei direcți, astfel încât fragmentele să însumeze exact ținta. Implicit, completează doar elementele care nu au încă țintă și nu vă atinge propriile numere. Ce scrie sunt [ținte](:goal-target) obișnuite, pe care le puteți edita ulterior și care se pot îndepărta unele de altele: este o acțiune punctuală, nu o regulă permanentă.
goal-subtree-total = Cât însumează țintele din interior.
goal-subtree-total-more = Un fapt, nu o țintă. Acest proiect nu tratează niciodată ținta unui container ca sumă a celor din interior, pentru că așa ajunge cifra unui dosar să se schimbe singură când o scenă din el primește un număr. Rândul acesta vă spune doar cât însumează propriile numere, ca să-l puteți compara singur(ă) cu [ținta](:goal-target) containerului sau să [împărțiți una](:goal-distribute).
goal-milestone = O dată până la care trebuie atins ceva.
goal-milestone-more = Există două feluri. Primul spune că [cartea](:wm-book) ar trebui să ajungă la o anumită lungime până la o dată, un reper pe propria curbă. Al doilea spune că un anumit [capitol](:wm-chapter) sau o anumită [parte](:wm-part) ar trebui să-și fi atins [propria țintă](:goal-target) până atunci și citește acel număr direct, fără să păstreze o copie, așa că modificarea țintei actualizează și jalonul. Ambele apar în [planul de ritm](:pace-plan) al cărții.
pace-plan = Programul de scriere al cărții.
pace-plan-more = O [țintă](:goal-target) și un termen-limită, împreună cu zilele în care scrieți, transformate în cât trebuie scris pe zi. Citește ținta proprie a cărții, așa că stabilirea ei aici și în inspector înseamnă același număr în două locuri. Reperele de pe parcurs sunt [jaloanele](:goal-milestone).

# ── Feature concepts ───────────────────────────────────────────────────────────
# A third cascade web, beside the writing-model one and the word/character-target
# one above it. Each entry here teaches one project feature that otherwise has no
# page of its own: tags, the per-row label, point of view, epigraphs, footnotes,
# chapter mode, comments, backups, versions, trash, spell-check, search and
# replace, note templates, text replacement, smart punctuation, export styles and
# the round-trip markers an export can carry. Registered the same way as the two
# webs above, and cross-linking into them wherever a concept genuinely cites one.
concept-tag = O etichetă colorată pe care o puteți pune pe orice element, refolosibilă în tot proiectul.
concept-tag-more = Etichetați un element ca să-l marcați: o etapă, un loc, un fir narativ pe care îl urmăriți. Aceeași etichetă poate sta pe oricâte elemente, iar un element poate purta mai multe. Activați [căutarea în proză](:wm-find-in-prose) pentru o etichetă care numește ceva din carte, un personaj sau un loc, și Skribisto începe să-l caute în proza dumneavoastră.
concept-label = O notă scurtă pe care o scrieți sub titlul unui element, pentru uzul dumneavoastră.
concept-label-more = Nu este o [etichetă](:concept-tag): o legendă aparține unui singur element, text simplu fără culoare sau catalog în spate, ceva de felul „primul punct de cotitură” sau „îi trebuie un nume”. O setați din meniul contextual al elementului sau o editați direct în coloana Legendă din Vedere de ansamblu; apare ca un mic subtitlu sub titlu, atât în structură, cât și în flux.
concept-point-of-view = Prin ochii cui este povestită o scenă: unul sau mai mulți membri ai distribuției poveștii.
concept-point-of-view-more = Se setează în inspector, lângă distribuție. Dacă alegeți pe cineva care nu este încă în distribuție, este adăugat și acolo. Mai multe puncte de vedere sunt o alegere permisă, utilă pentru o scenă comună sau o schimbare de perspectivă la mijlocul cărții, nu ceva ce Skribisto blochează. Candidații provin din elementele pe care o etichetă cu [căutare în proză](:wm-find-in-prose) le pune în biblia poveștii.
concept-epigraph = Un citat pus în fruntea unei părți sau a unui capitol.
concept-epigraph-more = Are un câmp propriu, separat de proza [capitolului](:wm-chapter) sau a [părții](:wm-part); o [scenă](:wm-scene) sau o [notă](:wm-note) nu au motto. Scrieți unul ca pe un citat obișnuit; scrieți mai multe și fiecare devine un bloc de citat separat, tipărit ca motto separat. Cuvintele lui nu sunt niciodată numărate drept cuvinte ale manuscrisului.
concept-footnote = O trimitere marcată în proză, tipărită ca notă numerotată.
concept-footnote-more = Trimiterea este un singur semn în textul dumneavoastră, așa că se mută și se șterge exact ca și cuvintele din jur. Numărul ei nu este stocat niciodată: Skribisto îl calculează din nou după locul fiecărei trimiteri în tot manuscrisul, așa că inserarea uneia renumerotează toate notele de după ea. Cuvintele unei note de subsol sunt numărate separat de manuscris, niciodată incluse în el.
concept-chapter-mode = Dacă un capitol nou este un dosar de scene sau un singur element simplu.
concept-chapter-mode-more = Se setează în Setări, la Alcătuire. Scrieți direct într-un [capitol](:wm-chapter) în ambele cazuri; singura diferență este dacă poate conține și scene sub el. Ambele forme se compilează în aceeași carte și le puteți combina liber într-un proiect. „Convertește în” trece un capitol existent dintr-o formă în alta fără să piardă niciun cuvânt.
concept-comment = O notă ancorată într-un fragment exact de proză, afișată în margine.
concept-comment-more = Ține minte cuvintele la care se referă și contextul lor, nu o poziție fixă, așa că rezistă editărilor obișnuite și unei reîncărcări complete. Răspundeți dedesubt pentru a continua discuția. Dacă citatul exact nu mai poate fi găsit, comentariul o spune, în loc să alunece pe tăcute spre o propoziție greșită.
concept-backup = O copie a întregului proiect la un anumit moment, păstrată lângă el.
concept-backup-more = Se face după un program ales de dumneavoastră: la deschidere, la închidere, la interval sau orice combinație, iar copiile vechi sunt eliminate după vechime sau după număr. Deschiderea uneia o încarcă într-o fereastră proprie: puteți să vă uitați și chiar să editați, dar doar „Salvează ca” sau „Restaurează” păstrează ceva. Este diferită de o [versiune](:concept-version), care urmărește trecutul unui singur element.
concept-version = Trecutul unui element: fiecare modificare a textului sau a sinopsisului său.
concept-version-more = Construită din istoricul de salvări al proiectului și din copiile de siguranță, cu o intrare pentru fiecare modificare, nu pentru fiecare fișier. Alegeți o intrare pentru a vedea ce s-a schimbat și readuceți doar textul acelui element la ea; restul proiectului nu este atins, iar înainte se face o copie de precauție. Este diferită de o [copie de siguranță](:concept-backup), care copiază totul.
concept-trash = Un element aruncat la coșul de gunoi este ascuns, nu mutat: rămâne exact unde era.
concept-trash-more = Aruncarea la coșul de gunoi doar schimbă un indicator: elementul își păstrează locul în clasor, marcat ca inactiv, nu eliminat. Restaurați-l și reapare exact în acel loc, fără să trebuiască reorganizat ceva. „Golește coșul de gunoi” elimină definitiv elementele aruncate, iar fiecare dintre aceste operații este un pas care poate fi anulat, ca orice altă modificare a proiectului.
concept-spellcheck = Subliniază cuvintele pe care niciun dicționar instalat nu le recunoaște, după limba proiectului.
concept-spellcheck-more = Fiecare proiect are propria limbă sau propriile limbi de lucru; un element le poate înlocui pentru o scenă scrisă în altă limbă. Adăugați un cuvânt pe care Skribisto nu îl cunoaște din meniul contextual al editorului, într-un dicționar personal care se aplică tuturor proiectelor. Dezactivarea verificării ortografice o oprește peste tot, până o reactivați.
concept-search-replace = Găsește un cuvânt sau o expresie oriunde în proiect, nu doar în documentul deschis.
concept-search-replace-more = Caută în toate scenele, notele, titlurile, sinopsisurile și legendele, precum și în comentarii și note de subsol. Majusculele, cuvântul întreg și diacriticele sunt trei comutatoare separate. „Înlocuiește tot” rescrie fiecare potrivire verificată dintr-o singură trecere și un singur pas de anulare, așa că Ctrl+Z anulează tot lotul, nu doar ultima modificare.
concept-note-template = Un text refolosibil pe care îl inserați în orice lucrați.
concept-note-template-more = O fișă de personaj goală, un profil de loc, o schemă de ritm narativ: scrieți-o o dată și inserați-o oriunde aveți nevoie, în orice element, nu doar într-o notă. Porniți de la un model inclus, importați un fișier .md sau .djot ori scrieți ceva și alegeți Document ▸ Salvează ca șablon. Este stocat în proiect, așa că un coautor care îl deschide are același set.
concept-text-replacement = Propriile prescurtări, extinse automat în timp ce tastați.
concept-text-replacement-more = Stabiliți o prescurtare și înlocuirea ei, „aprx” pentru „aproximativ”, iar ea se extinde în clipa în care tastați după ea un spațiu sau un semn de punctuație. Majusculele urmează ce ați tastat: scrieți prescurtarea cu majusculă și extinderea începe și ea cu majusculă. Un Backspace imediat după extindere anulează doar acea înlocuire. Se activează pentru fiecare proiect și este diferită de [punctuația inteligentă](:concept-smart-punctuation).
concept-smart-punctuation = Tipografie adaptată limbii, aplicată automat în timp ce tastați: ghilimele, linii, puncte de suspensie.
concept-smart-punctuation-more = Transformă ghilimelele drepte în ghilimele tipografice, -- în linie scurtă și --- în linie de pauză, ... în puncte de suspensie și, dacă o activați, începe cu o linie de dialog un paragraf tastat ca „- ”. Un paragraf tastat ca "- ", "* " sau "1. " devine o listă. Se setează pentru fiecare proiect și este păstrată în fișierul .skrib, așa că un coautor scrie cu aceleași reguli. Este diferită de [propriile reguli](:concept-text-replacement), pe care le scrieți singur(ă).
concept-export-style = Un set de opțiuni de export, cu nume: titluri, spațiere, ce se include.
concept-export-style-more = Un singur stil, refolosit la orice export, indiferent de formatul ales. Stilurile incluse nu pot fi modificate; duplicați unul pentru a avea un punct de pornire pe care îl puteți schimba. Printre opțiunile lui se află includerea [marcajelor de dus-întors](:concept-round-trip-marks), pentru un fișier care pleacă la un redactor și se întoarce.
concept-round-trip-marks = Semne de carte invizibile, datorită cărora un DOCX sau ODT editat este recunoscut la întoarcere.
concept-round-trip-marks-more = Sunt scrise doar în cele două formate pe care un redactor le poate trimite înapoi, DOCX și ODT; niciun alt export nu le conține. Sunt semne de carte, nu atribute personalizate: LibreOffice elimină atributele personalizate la salvare, dar lasă semnele de carte intacte. Se activează pentru fiecare [stil de export](:concept-export-style), la „Trimitere către un redactor”.
concept-status = Unde se află un element în propriul flux de lucru: câte o etapă, mereu înainte.
concept-status-more = Nu este o [etichetă](:concept-tag): un element are exact o stare, etapele sunt ordonate, iar lista aparține proiectului, așa că o puteți redenumi, reordona sau completa oricând. Un asterisc lângă un capitol sau o carte înseamnă că părțile de dedesubt nu au toate aceeași stare. O setați din inspector, de pe o fișă, dintr-un element al fluxului sau de aici.
