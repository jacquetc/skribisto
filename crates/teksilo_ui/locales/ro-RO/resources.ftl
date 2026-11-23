# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Resurse: fișiere și legături web atașate elementelor din clasor.

## Panoul

resources-title = Resurse
shortcut-name-resources-show = Afișează resursele
resources-add-files = Adaugă fișiere
resources-add-files-title = Adaugă fișiere la acest element
resources-add-link = Adaugă o legătură web
resources-drop-hint = Plasați aici fișiere sau adrese web pentru a le atașa
resources-drop-hint-unattached = Plasați aici fișiere sau adrese web pentru a le adăuga fără a le atașa unui element
resources-no-item = Deschideți un element pentru a-i vedea resursele.
resources-empty = Nimic nu este încă atașat acestui element. Adăugați fișiere sau legături web cu butoanele de mai sus, sau plasați-le aici.
resources-dock-view-label = Resurse de afișat
resources-dock-view-item = Acest element
resources-dock-view-unattached = Neatașate
resources-dock-view-item-hint = Fișierele și legăturile elementului în care vă aflați. Ce adăugați i se atașează.
resources-dock-view-unattached-hint = Fișierele și legăturile neatașate niciunui element, de exemplu documentare încă neclasată. Ce adăugați aici nu se atașează la nimic.
resources-unattached-empty = Fiecare resursă a acestui proiect este atașată unui element. Fișierele și legăturile adăugate aici rămân neatașate până când le atașați.
resources-actions = Acțiuni
resources-not-found = Negăsit
resources-row-linked = Fișier legat
resources-row-linked-size = Fișier legat, { $size }
resources-row-name = { $title }, { $kind }, { $detail }
resources-row-name-missing = { $title }, { $kind }, { $detail }, negăsit

## Tipurile de fișiere, așa cum le anunță un cititor de ecran

resources-kind-picture = imagine
resources-kind-pdf = document PDF
resources-kind-audio = înregistrare audio
resources-kind-video = video
resources-kind-document = document
resources-kind-ebook = carte electronică
resources-kind-archive = arhivă
resources-kind-file = fișier
resources-kind-web-link = legătură web

## Meniul unui rând

resources-open = Deschide
resources-show-in-folder = Arată în dosar
resources-locate = Regăsește…
resources-insert-into-text = Inserează în text
resources-insert-picture = Inserează imaginea în text
resources-insert-link = Inserează o legătură în text
resources-edit = Redenumește și descrie…
resources-detach = Detașează de acest element
resources-remove = Elimină din proiect…

## Cele două ferestre de dialog

resources-link-dialog-title = Adaugă o legătură web
resources-link-dialog-address = Adresă
resources-link-dialog-address-placeholder = https://
resources-link-dialog-name = Titlu
resources-link-dialog-name-placeholder = Numele site-ului, dacă îl lăsați gol
resources-link-dialog-add = Adaugă
resources-edit-dialog-title = Redenumește și descrie
resources-edit-dialog-name = Titlu
resources-edit-dialog-description = Descriere
resources-edit-dialog-description-placeholder = Ce este, și de ce l-ați păstrat
resources-edit-dialog-save = Salvează
resources-dialog-cancel = Renunță

## Adăugare

resources-large-title = Un fișier mare
resources-large-text = „{ $name }” are { $size }. Păstrat în proiect, călătorește cu fiecare copie și fiecare copie de siguranță a proiectului. Legat, rămâne unde este, iar proiectul ține minte doar unde îl poate găsi.
resources-large-text-many = „{ $name }” are { $size }, iar { $others ->
    [one] încă un fișier este
    [few] alte { $others } fișiere sunt
   *[other] alte { $others } de fișiere sunt
} de asemenea mari. Păstrate în proiect, călătoresc cu fiecare copie și fiecare copie de siguranță a proiectului. Legate, rămân unde sunt, iar proiectul ține minte doar unde le poate găsi.
resources-large-store = Păstrează-l în proiect
resources-large-link = Leagă-l în schimb
resources-large-remember = Nu mai întreba
resources-storing-title = Se adaugă fișiere în proiect
resources-store-cancel = Renunță
resources-store-cancelled = Nu a fost adăugat niciun fișier.
resources-store-busy = Încă se adaugă fișiere. Încercați din nou după ce au fost adăugate.
resources-store-failed = Fișierele nu au putut fi adăugate: { $error }
resources-no-project = Deschideți un proiect înainte de a atașa fișiere.
resources-moved-away = Proiectul a fost mutat în timp ce fișierul era copiat.
resources-some-failed = { $count ->
    [one] Un fișier nu a putut fi adăugat: { $names }
    [few] { $count } fișiere nu au putut fi adăugate: { $names }
   *[other] { $count } de fișiere nu au putut fi adăugate: { $names }
}
resources-add-failed = Resursele nu au putut fi adăugate: { $error }
resources-added = { $count ->
    [one] O resursă adăugată
    [few] { $count } resurse adăugate
   *[other] { $count } de resurse adăugate
}
resources-dropped-added = { $count ->
    [one] Adăugat ca resursă a acestui element, și legat în text
    [few] { $count } fișiere adăugate ca resurse ale acestui element, și legate în text
   *[other] { $count } de fișiere adăugate ca resurse ale acestui element, și legate în text
}
resources-dropped-undo-kept = Textul s-a schimbat de la plasare, așa că nu a fost anulat nimic.
resources-undo = Anulează
resources-link-refused = „{ $url }” nu este o adresă web pe care Skribisto o poate păstra. Folosiți o adresă care începe cu http:// sau https://, fără nume de utilizator sau parolă.

## Anunțul pentru un proiect într-un singur fișier

resources-folder-title = Acest proiect devine mare
resources-folder-text = Fișierele sale păstrate ocupă acum { $size }. Un proiect păstrat ca un singur fișier este rescris în întregime la fiecare salvare, cu tot cu fișierele sale. Păstrat ca dosar, salvează doar ce s-a schimbat. Îl puteți salva acum ca dosar și puteți lucra de atunci înainte în dosar.
resources-folder-switch = Salvează ca dosar…
resources-folder-not-now = Nu acum
resources-folder-never = Nu mai întreba pentru acest proiect

## Deschidere

resources-not-found-toast = „{ $name }” nu este acolo unde îl așteaptă proiectul.
resources-unsafe-title = Deschideți „{ $name }”?
resources-unsafe-text = Skribisto nu știe dacă acest tip de fișier este sigur. Deschiderea lui ar putea porni un program pe calculatorul dumneavoastră, iar un proiect poate veni de la altcineva. Arătați-l mai întâi în dosarul său pentru a vedea ce este.
resources-open-anyway = Deschide oricum
resources-open-failed = Fișierul nu a putut fi deschis: { $error }
resources-no-data-folder = acest sistem nu îi oferă lui Skribisto niciun dosar de date în care să deschidă o copie
resources-link-gone = Această legătură duce la o resursă pe care proiectul nu o mai are.
resources-locate-title = Unde se află acum „{ $name }”?
resources-relinked-more = { $count ->
    [one] Încă un fișier lipsă a fost găsit în același dosar și legat din nou.
    [few] Încă { $count } fișiere lipsă au fost găsite în același dosar și legate din nou.
   *[other] Încă { $count } de fișiere lipsă au fost găsite în același dosar și legate din nou.
}
resources-changed-elsewhere = „{ $name }” a fost modificat în altă aplicație. Proiectul păstrează versiunea nouă.

## Editare

resources-edit-failed = Resursa nu a putut fi modificată: { $error }
resources-remove-title = Eliminați „{ $title }” din proiect?
resources-remove-text = Este scoasă de la fiecare element la care este atașată. Un fișier păstrat în proiect pleacă odată cu ea; un fișier legat și o pagină web rămân unde sunt. Anularea o aduce înapoi.
resources-remove-confirm = Elimină
resources-remove-failed = Resursa nu a putut fi eliminată: { $error }
resources-removed-toast = „{ $title }” a fost eliminată din proiect.
resources-removed-many-toast = { $count } resurse au fost eliminate din proiect.
resources-detached-toast = „{ $title }” a fost detașată.
resources-attached-one-toast = „{ $title }” a fost adăugată la resursele lui „{ $item }”.
resources-attached-many-toast = { $count } resurse au fost adăugate la „{ $item }”.

## În text

resources-no-text = Puneți mai întâi cursorul într-un text, apoi inserați resursa acolo.
resources-drop-insert-picture = Inserează imaginea
resources-drop-insert-link = Inserează o legătură către resursă

## Progres

progress-resources-storing = Se copiază fișierele
progress-resources-stored = Fișiere copiate

## Pagina Resurse a unui dosar

segment-folder = Dosar
resources-folder-count = { $count ->
    [one] O resursă
    [few] { $count } resurse
   *[other] { $count } de resurse
}
resources-view-label = Afișează resursele ca
resources-view-list = Listă
resources-view-grid = Grilă
resources-grid-label = Resurse
resources-folder-drop-hint = Plasați aici fișiere sau adrese web pentru a le atașa acestui dosar
resources-folder-empty = Nimic nu este încă atașat acestui dosar sau la ceea ce conține. Adăugați fișiere sau legături web cu butoanele de mai sus, sau plasați-le aici: vor fi atașate dosarului.
resources-attached-to = La
resources-attached-separator = ,
resources-attached-more = { $count ->
    [one] și încă unul
    [few] și încă { $count }
   *[other] și încă { $count }
}
resources-row-name-attached = { $name }, la { $items }
resources-open-item = Deschide „{ $title }”
resources-detach-from = Detașează de „{ $title }”

## Resursele-imagine în Galerie

gallery-resource-badge = Resursă
gallery-tile-resource = Imagine-resursă a elementului { $title }: { $description }

## Pagina Resurse din setările proiectului

settings-page-resources = Resurse
settings-desc-resources = Fișierele și legăturile web pe care nu le mai poartă niciun element: atașați-le din nou, eliminați-le sau exportați copii
settings-resources-description = O resursă nu mai este atașată la nimic când este detașată de ultimul ei element, sau când elementul care o purta este șters definitiv. Rămâne în proiect, iar un fișier păstrat călătorește cu fiecare copie și fiecare copie de siguranță a proiectului, până când o atașați din nou sau o eliminați.
settings-resources-group-storage = Spațiu
settings-resources-group-unattached = Neatașate la nimic
settings-resources-stored = Fișiere păstrate în acest proiect: { $size }
settings-resources-folder-advice = Un proiect păstrat ca un singur fișier este rescris în întregime la fiecare salvare, cu tot cu fișierele păstrate. Păstrat ca dosar, salvează doar ce s-a schimbat, iar asta e mai rapid.
settings-resources-count = { $count ->
    [one] O resursă
    [few] { $count } resurse
   *[other] { $count } de resurse
}
settings-resources-empty = Fiecare resursă a acestui proiect este atașată unui element.
resources-attach-to-items = Atașează la elemente…
resources-export-copies = Exportă copii…
resources-export-copy = Exportă o copie…
resources-remove-many-title = Eliminați din proiect resursele ({ $count })?
resources-attach-title = Atașează la elemente
resources-attach-hint = Alegeți elementele la care atașați. Ctrl+clic sau Shift+clic alege mai multe.
resources-attach-empty = Acest proiect nu are încă elemente.
resources-attach-confirm = Atașează
resources-attached = { $count ->
    [one] O resursă atașată
    [few] { $count } resurse atașate
   *[other] { $count } de resurse atașate
} la { $items ->
    [one] un element
    [few] { $items } elemente
   *[other] { $items } de elemente
}
resources-export-title = Exportă copii într-un dosar
resources-exported = { $count ->
    [one] O resursă exportată
    [few] { $count } resurse exportate
   *[other] { $count } de resurse exportate
}
resources-export-missing = { $count ->
    [one] Un fișier nu este unde îl așteaptă proiectul, și a fost lăsat deoparte: { $names }
    [few] { $count } fișiere nu sunt unde le așteaptă proiectul, și au fost lăsate deoparte: { $names }
   *[other] { $count } de fișiere nu sunt unde le așteaptă proiectul, și au fost lăsate deoparte: { $names }
}
resources-export-failed = { $count ->
    [one] Un fișier nu a putut fi copiat: { $names }
    [few] { $count } fișiere nu au putut fi copiate: { $names }
   *[other] { $count } de fișiere nu au putut fi copiate: { $names }
}

## Preferința pentru fișierele mari (Comportamentul editorului)

settings-resources-large-file = Fișiere de peste 10 MB
settings-resources-large-ask = Întreabă de fiecare dată
settings-resources-large-store = Păstrează întotdeauna în proiect
settings-resources-large-link = Leagă întotdeauna la fișier
settings-hint-resources-large-file = Ce se întâmplă cu un fișier de peste 10 MB pe care îl atașați ca resursă. Păstrat în proiect, călătorește cu fiecare copie și fiecare copie de siguranță a proiectului. Legat, rămâne unde este, iar proiectul ține minte doar unde să-l găsească.
