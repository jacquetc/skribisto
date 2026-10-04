# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

wm-book = O carte din proiectul tău.
wm-book-more = O carte este la fel de simplu sau la fel de stratificat cum doriți. O poveste scurtă poate fi doar câteva [scene](:wm-scene). Un proiect mai lung poate cuib [părți](:wm-part) și [capitole](:wm-chapter) la fel de adânc cum doriți. Forma este a ta.
wm-part = O parte de grup capitole.
wm-part-more = O parte adună [capitole](:wm-chapter) într-un singur titlu. Ca un capitol, se execută până la următoarea parte sau la sfârșitul [Carte](:wm-book).
wm-chapter = Un capitol din carte.
wm-chapter-more = Un capitol merge de aici până la următorul capitol, următorul [Parte](:wm-part), sau [Sfârşitul cărţii](:wm-end-of-book). Acesta vine în două forme: un dosar capitol care deține [scene](:wm-scene), sau un capitol plat, care poartă propria proza. Alege care un nou capitol se utilizează în Setări, în cadrul Structurii.
wm-scene = O scenă de proză.
wm-scene-more = Fiecare articol de scris are două laturi: textul principal și un [Rezumat](:wm-synopsis) pentru a planifica cu. Ambele deschise împreună în editor.
wm-note = Un bilet.
wm-note-more = O notă păstrează text și un [Rezumat](:wm-synopsis), cum ar fi un [Scenă](:wm-scene), pentru cercetare, deoparte, sau memento-uri. Orice element poate face parte din cartea compilată, inclusiv o notă așezată într-un [Capitol](:wm-chapter).
wm-note-folder = O notă care conține alte elemente.
wm-note-folder-more = Un [Notă](:wm-note) care conține, de asemenea, elemente sub ea, astfel încât să puteți grupa note legate împreună. Acesta păstrează propriile sale [Rezumat](:wm-synopsis), iar elementele din interiorul păstrează propriul text.
wm-folder = Un dosar de organizare.
wm-folder-more = Un container pentru aranjarea obiectelor în liant, cu toate acestea, vă convine. Acesta poartă un [Rezumat](:wm-synopsis), dar nu proza proprie. Obiectele din interior dețin scrisul.
wm-end-of-book = Urme unde se termină o carte.
wm-end-of-book-more = Fiecare carte împărtășește o listă continuă, astfel încât sfârșitul unei cărți nu este decis prin cuibărire. Acest marker spune [Carte](:wm-book) se oprește aici. Orice după ce aparține cărții următoare.
wm-story-bible-entry = O notiţă falsă despre povestea ta.
wm-story-bible-entry-more = Un [Notă](:wm-note) despre un personaj, o locație, sau orice altceva în valoare de depunere, creat cu un nume, etichete, pseudonime și un șablon de pornire toate într-un singur pas. Nimic pe pagina o distinge după aceea, deoarece rămâne o notă obișnuită puteți redenumi, retag sau converti ca orice alt.
wm-synopsis = Un rezumat al unui articol scris.
wm-synopsis-more = Un rezumat atașat la orice element scris, de obicei, un scurt paragraf, dar atâta timp cât doriți. Utilizați-l pentru a planifica și naviga înainte de proza există. Se află lângă textul principal în editor.

scene-break-minor = O scenă obişnuită: o schimbare de timp, loc sau punct de vedere într-un capitol.
scene-break-minor-more =
    Marchează o pauză *în cazul în care l-ai pus*, inclusiv în mijlocul unui [Scenă](:wm-scene). Împarte proza în două elemente este o alegere organizațională, așa că nu creează o pauză pe cont propriu.

    Tipărit în proza ca trei asteriscuri separate de spaţii. Cum se printează este decis de stilul de export, nu de ceea ce tip: un manuscris Shunn-l stabileşte ca un singur #, un suport de hârtie comercială ca un dinkus, şi cele mai multe franceză, germană, spaniolă, rusă şi italiană publicarea ca un gol gol gol gol, fără nici un semn la toate.
scene-break-major = O diviziune mai puternică: o săritură în timp mare sau o schimbare decisivă a punctului de vedere.
scene-break-major-more =
    Aceeași idee ca o pauză de scenă obișnuită, un pas în sus. Utilizați-l atunci când o pauză simplă ar substate saltul.

    Tastat ca # # #, și imprimate diferit de nivelul obișnuit prin stilul de export. Shunn standard format manuscris stabilește un singur # împotriva # # # pentru exact această distincție. În cazul în care o tradiție nu are o marcă mai puternică, ambele niveluri print la fel.
wm-find-in-prose = Skribisto caută acest articol în proza ta.
wm-find-in-prose-more = Porniți acest lucru pe tag-uri care nume lucruri pe care le scrieți despre: caractere, locuri, obiecte. Orice element care transportă o astfel de etichetă este potrivit cu proza dvs. de titlul său și de alte nume pe care le dați, astfel încât fiecare [Scenă](:wm-scene)  liste care și ce apare în ea fără a vă conecta nimic de mână. Lăsați-l off pentru etichetele care descriu un element mai degrabă decât numele unul, cum ar fi un proiect de stare sau un memento pentru a verifica continuitatea. Fiecare element se potrivește, de asemenea, se alătură dvs. [Ghidul poveștii](:wm-story-bible).
wm-story-bible = Fiecare personaj, loc și alte tagged intrare, adunat într-o singură grilă.
wm-story-bible-more = Deschideți din tab-ul propriu al oricărui dosar de note. Carduri grup de tag-uri, și fiecare nume cât de multe pseudonime o intrare răspunde la și câte scene a apărut în, extrase din fiecare [găsi în proză](:wm-find-in-prose) tag-ul din proiect. Filtrarea de carte doar îngustează ce cărți arată; nu decide niciodată ce intrări există.

tooltip-go-to = Salt la orice element din liant (Ctrl+G)
synopsis-collapse-tooltip = Ascunde coloana sinopsis

wm-paratext = Un text care nu face parte din poveste.
wm-paratext-more = O prefaţă, o dedicaţie, un afterword, un colofon: scrisul care aparţine cărţii, dar nu corpului ei. Exportat oriunde ai pus-o, şi niciodată nu conta în numărul de cuvinte al manuscrisului. Unde merge depinde de tine: convenţiile diferă în funcţie de ţară şi editor.
wm-paratext-folder = Un dosar pentru paratexturi.
wm-paratext-folder-more = Undeva pentru a păstra prefeţe şi după cuvinte astfel încât acestea să nu dezordine liant. Organizarea numai: aceasta poartă un [Rezumat](:wm-synopsis), dar adaugă nimic la cartea exportată, nici măcar propriul nume.

goal-target = Cât timp această piesă este menit să fie.
goal-target-more = Stabilește o țintă pe orice: un [Scenă](:wm-scene), un [Capitol](:wm-chapter), un [Parte](:wm-part), întregul [Carte](:wm-book). Fiecare este propriul său număr și stă pe cont propriu, astfel încât stabilirea unei ținte pe un capitol nu schimbă niciodată cartea sa, și ținta unei cărți nu este niciodată o sumă a capitolelor din interiorul ei. Ceea ce adaugă este scris: un container de [Progres](:goal-progress) este totul scris sub ea. Lăsați o țintă la zero și pur și simplu nu este stabilit. Numărul este numărat în [cuvinte sau caractere](:goal-unit), oricare ar fi acest proiect folosește.
goal-unit = Fie că acest proiect contează în cuvinte sau în caractere.
goal-unit-more = O alegere la nivel de proiect, deoarece o unitate de lungime aparţine manuscrisului şi pieţei sale mai degrabă decât unei secţiuni: Editura germană şi franceză în semne, japoneză în foi de 400 de caractere, chineză cu mii de caractere, cea mai mare editură în limba engleză în cuvinte. Ambele numere sunt păstrate una lângă alta şi nici nu este convertită vreodată în cealaltă, aşa că schimbarea punctelor unitate la fiecare [Obiectiv](:goal-target) la cealaltă cifră şi schimbarea înapoi restaurează primul. Ceea ce nu face este convertirea nimic, astfel încât obiectivele deja introduse vor avea nevoie de actualizare de mână.
goal-progress = Cât de mult este scris, împotriva ţintei.
goal-progress-explained = O piesa cu proza proprie este masurata in functie de lungimea proprie; un container impotriva [totul scris sub ea](:goal-manuscript-words). Barul se incalzeste de la rosu la chihlimbar la verde pe masura ce scrierea apare, si se schimba din nou dupa ce piesa a trecut cu bine peste ceea ce a fost planificat, ceea ce merita observat atunci cand un capitol are o lungime de pastrat.
goal-manuscript-words = Ceea ce contează ca manuscris.
goal-manuscript-words-more = Doar proza care ar fi exportată de fapt: [scene](:wm-scene) și [capitole](:wm-chapter) care își poartă propriul text. A [Notă](:wm-note) sau un [Paratext](:wm-paratext) nu face parte din manuscris și nu este niciodată luată în considerare față de unul. Orice se află în coșul de gunoi cade, și la fel face și nimic [lăsat în afara exportului](:goal-exportable).
goal-exportable = Acest articol este exclus din export.
goal-exportable-more = Lungimea proprie încă arată, pentru că scrisul este încă acolo, dar contează spre nici un total, exact așa cum va apărea în nici o carte exportată. Lăsând un container afară nu lasă piesele în interiorul acestuia: fiecare element răspunde pentru sine, care este ceea ce "Aplică pentru copii" butonul de lângă comutator este pentru.
goal-distribute = Împărţiţi această ţintă peste ceea ce este înăuntru.
goal-distribute-more = Împarte o țintă [Cartea lui](:wm-book) sau [Part's](:wm-part) pe copiii săi imediat, astfel încât piesele adăuga până la ea exact. În mod implicit umple numai cele cu nici o țintă încă și lasă propriile numere singur. Ceea ce scrie sunt obișnuite [obiective](:goal-target), gratuit pentru a edita după și liber pentru a pluti în afară: aceasta este o acțiune, nu un aranjament în picioare.
goal-subtree-total = La ce se adaugă ţintele din interior.
goal-subtree-total-more = Un fapt, nu o țintă. Acest proiect nu tratează niciodată ținta unui container ca suma celor din interiorul acestuia, pentru că acesta este modul în care cifra unui folder ajunge să se miște pe cont propriu atunci când o scenă din interiorul ei este dat un număr. Această linie pur și simplu vă spune ce numerele proprii vin la, astfel încât să puteți compara cu containerul lui [Obiectiv](:goal-target) pe dumneavoastră sau [Împărţiţi unul.](:goal-distribute).
goal-milestone = O dată pentru a ajunge la ceva de.
goal-milestone-more = Unul spune că [Carte](:wm-book) ar fi trebuit să stea la o anumită lungime de o dată, un punct de trecere pe propria curbă. Celălalt spune că un anumit [Capitol](:wm-chapter) sau [Parte](:wm-part) ar fi trebuit să ajungă la [propria sa țintă](:goal-target) până atunci, și citește că numărul trăiesc mai degrabă decât păstrarea o copie a acesteia, astfel încât editarea țintei actualizează de asemenea etapa de referință. Ambele apar pe cartea lui [planul de ritm](:pace-plan).
pace-plan = Programul de scris al cărţii.
pace-plan-more = Un [Obiectiv](:goal-target) și un termen limită, cu zilele pe care le scrieți, s-au transformat în cât de mult are nevoie o zi. Ea citește propria țintă a cărții, astfel încât stabilirea ei aici și stabilirea în inspector sunt același număr în două locuri. Punctele de cale de-a lungul drum sunt [etape de referință](:goal-milestone).

concept-tag = O etichetă cu coduri de culoare puteți pune pe orice element, reutilizabil pe tot parcursul proiectului.
concept-tag-more = Tag-ul un element să-l marcheze: un statut, o locație, un fir pe care îl urmăriți. Aceeași etichetă poate sta pe orice număr de elemente, și un element poate transporta mai multe. Porniți [găsi în proză](:wm-find-in-prose) pentru o etichetă care numește ceva în cartea ta, un personaj sau un loc, și Skribisto începe căutarea pentru ea în proza ta.
concept-label = O scurtă notă pe care o scrii sub titlul unui articol, pentru ochii tăi.
concept-label-more = Nu este un [Etichetă](:concept-tag): o etichetă aparține unui singur element, text simplu, fără culoare sau catalog în spatele acestuia, ceva de genul "primul punct de complot" sau "necesită un nume." Setați-l din meniul contextual al elementului, sau editați-l în linie în coloana "Observation's Label"; acesta arată ca un subtitlu mic sub titlul, în contur și în flux deopotrivă.
concept-point-of-view = A căror ochi o scenă este narat prin intermediul: una sau mai multe dintre exprimate povestea ta.
concept-point-of-view-more = Alegeți pe cineva care nu este încă în distribuție le adaugă acolo. Mai mult de un punct de vedere este o alegere legală, utilă pentru o scenă comună sau o schimbare de ochi la mijlocul cărții, nu ceva Skribisto blocuri. Candidații provin de la elementele un tag [găsi în proză](:wm-find-in-prose) pune în biblia poveste.
concept-epigraph = Un citat setat în capul unei părți sau al unui capitol.
concept-epigraph-more = Câmpul său propriu, separat de proza [Capitol](:wm-chapter) sau de proza proprie a [Parte](:wm-part); un [Scenă](:wm-scene) sau un [Notă](:wm-note) nu poartă nimic. Scrieți unul ca un citat obișnuit; scrieți mai multe și fiecare devine propriul său citat de bloc, tipărit ca epigraf propriu. Cuvintele sale nu sunt niciodată considerate ca cuvinte de manuscris.
concept-footnote = O citaţie marcată în proză, care se tipăreşte ca o notă numerotată.
concept-footnote-more = Referinta este un singur semn in cuvintele tale, asa ca se misca si se sterge exact ca si cuvintele din jurul ei. Numarul ei nu este niciodata stocat: Skribisto Proiecteaza-l proaspat de unde fiecare referinta sta in tot manuscrisul, astfel incat inserarea unei renumere fiecare nota dupa el. Cuvintele proprii ale unei note de subsol sunt luate in afara manuscrisului, niciodata pliate in el.
concept-chapter-mode = Fie că un nou capitol este un dosar de scene sau un singur element plat.
concept-chapter-mode-more = Setați în Setări, sub Structură. Scrieți direct într-un [Capitol](:wm-chapter) în orice mod; singura diferență este dacă poate conține și scene sub ea. Ambele codări se compilează în aceeași carte, și le puteți amesteca liber într-un singur proiect. Promovează convertirea unui capitol existent între cele două fără a pierde un cuvânt.
concept-comment = O notă ancorată pe o porţiune exactă de proză, arătată în margine.
concept-comment-more = Îşi aminteşte cuvintele pe care le indică şi ce le înconjoară, nu o poziţie fixă, aşa că supravieţuieşte editării obişnuite şi unei reîncărcări complete. Răspunde dedesubt pentru a păstra un fir merge. Dacă citatul exact nu mai poate fi găsit, comentariul spune mai degrabă decât în derivă tăcut la propoziţia greşită.
concept-backup = O copie punctuală a întregului proiect, ţinută lângă el.
concept-backup-more = Luate pe un program pe care îl alegeți: pe deschide, pe aproape, pe un interval, sau orice mix, cu copii vechi prunc după vârstă sau după număr. Deschiderea unei sarcini în propria fereastră: puteți uita și chiar edita, dar numai Salvați As sau Restore păstrează nimic. Diferit de un [versiune](:concept-version), care urmărește propriul trecut al unui element.
concept-version = Trecutul unui element: fiecare schimbare a textului sau sintezei sale.
concept-version-more = Construit din propria istorie de salvare a proiectului și backup-urile sale împreună, o intrare pe schimbare, mai degrabă decât una pe fișier. Alegeți o intrare pentru a vedea ce sa schimbat și restaura doar textul acelui articol la el; restul proiectului este neatins, și o copie de siguranță este luată în primul rând. Diferit de un [Copie de siguranță](:concept-backup), care este o copie a tot ce.
concept-trash = Un obiect distrus este ascuns, nu mutat: rămâne exact unde a fost.
concept-trash-more = Distrugerea doar răstoarnă un singur steag: elementul păstrează locul său în liant, marcat inactiv, mai degrabă decât eliminat. Restabilește-l și reapare în acel loc exact, fără re-filare de a face. Gunoi gol elimină obiecte distruse pentru totdeauna, și fiecare dintre acestea este un pas de neatins, ca orice alt proiect de editare.
concept-spellcheck = Subliniază cuvinte pe care nici un dicţionar instalat nu le recunoaşte, per limbaj de proiect.
concept-spellcheck-more = Fiecare proiect are propriul său limbaj de lucru sau limbi; un element le poate suprascrie pentru o scenă scrisă într-o altă limbă. Adăugați un cuvânt Skribisto nu știe din meniul de context al editorului, într-un dicționar personal care trece peste fiecare proiect. Verificând vraja se oprește peste tot, până când îl porniți din nou.
concept-search-replace = Găsește un cuvânt sau o frază peste tot în proiect, nu numai documentul deschis.
concept-search-replace-more = Se uită prin fiecare scenă, notă, titlu, sinopsis și etichetă, și prin comentarii și note de subsol de asemenea. Caz, potrivire întreg cuvânt și potrivire accent sunt trei comutatoare separate. Înlocuiți Toate rescrie fiecare meci revizuit într-o singură trecere și un pas de anulare, astfel încât Ctrl+Z ia înapoi întregul lot, nu numai ultima schimbare.
concept-note-template = O bucată reutilizabilă de scris pe care o introduci în orice lucru la care lucrezi.
concept-note-template-more = O foaie de caractere oarbe, un profil de locație, o foaie de beat: scrieți-l o dată și fixați-l în oriunde aveți nevoie, în orice element, nu numai o notă. Începeți de la un built-in preset, importați un .md  sau .djot fişier, sau scrie ceva şi alege documentul Save ca şablon. Stocat în proiect, astfel încât un co-autor care deschide devine acelaşi set.
concept-text-replacement = Propria ta prescurtare, extinsă automat pe măsură ce tastezi.
concept-text-replacement-more = Setaţi un declanşator şi înlocuirea acestuia, "btw" pentru "apropo," şi se extinde momentul în care tastaţi un spaţiu sau punctuaţie după el. Cazul urmează ceea ce aţi tastat: capitalizaţi declanşatorul şi expansiunea se capitalizează de asemenea. A Backspace imediat după o expansiune anulează doar acea substituţie. Comutat pe fiecare proiect, distinct de [punctuaţie inteligentă](:concept-smart-punctuation).
concept-smart-punctuation = Tipografia locală se aplică automat pe măsură ce tastați: citate, dale, elipse.
concept-smart-punctuation-more = Curls drepte citate în cele tipografice, se transformă -- într-un en dash și -- într-o bordură em, se transformă ... într-o ellipsă, și, în cazul în care îl porniți, deschide un paragraf tastat ca "-" cu o bordură dialog. Setat pe proiect, și călătorește în interiorul .skrib astfel încât un co-autor scrie cu aceleași reguli. Distinct de la [propriile tale reguli](:concept-text-replacement), pe care le scrie.
concept-export-style = Un pachet numit de opţiuni de export: poziţii, distanţă, ceea ce se include.
concept-export-style-more = Un stil, reutilizat pe fiecare model de export pe care îl alegeți. Stilurile built-in sunt doar citite; duplicat unul pentru a obține un punct de pornire puteți schimba de fapt. Printre opțiunile sale este dacă să includă [Markeri rotunzi](:concept-round-trip-marks), pentru un fișier legat de un editor și înapoi.
concept-round-trip-marks = Semne de carte invizibile, care permit o editat DOCX sau ODT vin acasă recunoscut.
concept-round-trip-marks-more = Scris doar în cele două formate un editor poate trimite înapoi, DOCX și ODT; fiecare alt export nu poartă nici unul. Un semn de carte, nu un atribut personalizat: LibreOffice benzi atribute personalizate pe salvare, dar lasă un semn de carte intact. Turnat pe [stil de export](:concept-export-style), sub "Trimiterea la un editor."
concept-status = În cazul în care un element se află în propriul flux de lucru: o etapă la un moment dat, în mișcare înainte.
concept-status-more = Nu este un [Etichetă](:concept-tag): un articol are exact un statut, etapele sunt comandate, iar proiectul deține lista, astfel încât să puteți redenumi, reordona sau adăuga la ea ori de câte ori doriți. Un asterisc lângă un capitol sau carte înseamnă părțile de mai jos nu sunt toate de acord cu ea. Setați-l de la inspector, o carte, un element de flux, sau aici.
