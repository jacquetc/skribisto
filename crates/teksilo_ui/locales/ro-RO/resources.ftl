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
resources-no-item = Deschideți un element pentru a-i vedea resursele.
resources-empty = Nimic nu este încă atașat acestui element. Adăugați fișiere sau legături web cu butoanele de mai sus, sau plasați-le aici.
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

## În text

resources-no-text = Puneți mai întâi cursorul într-un text, apoi inserați resursa acolo.
resources-drop-insert-picture = Inserează imaginea
resources-drop-insert-link = Inserează o legătură către resursă

## Progres

progress-resources-storing = Se copiază fișierele
progress-resources-stored = Fișiere copiate
