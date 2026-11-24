# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Források: az iratrendező elemeihez csatolt fájlok és webes hivatkozások.

## A panel

resources-title = Források
shortcut-name-resources-show = A források megjelenítése
resources-add-files = Fájlok hozzáadása
resources-add-files-title = Fájlok hozzáadása ehhez az elemhez
resources-add-link = Webes hivatkozás hozzáadása
resources-drop-hint = Ejtsen ide fájlokat vagy webcímeket a csatolásukhoz
resources-drop-hint-unattached = Ejtsen ide fájlokat vagy webcímeket, hogy elemhez csatolás nélkül adja hozzá őket
resources-no-item = Nyisson meg egy elemet a forrásai megtekintéséhez.
resources-empty = Ehhez az elemhez még semmi sincs csatolva. Adjon hozzá fájlokat vagy webes hivatkozásokat a fenti gombokkal, vagy ejtse ide őket.
resources-dock-view-label = Megjelenítendő források
resources-dock-view-item = Ez az elem
resources-dock-view-unattached = Csatolatlan
resources-dock-view-item-hint = Annak az elemnek a fájljai és hivatkozásai, amelyen áll. Amit hozzáad, ahhoz csatolódik.
resources-dock-view-unattached-hint = Azok a fájlok és hivatkozások, amelyek egyik elemhez sincsenek csatolva, például még be nem sorolt kutatási anyag. Amit itt ad hozzá, semmihez sem csatolódik.
resources-unattached-empty = A projekt minden forrása csatolva van valamelyik elemhez. Az itt hozzáadott fájlok és hivatkozások addig nincsenek semmihez csatolva, amíg nem csatolja őket.
resources-actions = Műveletek
resources-not-found = Nem található
resources-row-linked = Hivatkozott fájl
resources-row-linked-size = Hivatkozott fájl, { $size }
resources-row-name = { $title }, { $kind }, { $detail }
resources-row-name-missing = { $title }, { $kind }, { $detail }, nem található

## Fájltípusok, ahogy a képernyőolvasó felolvassa őket

resources-kind-picture = kép
resources-kind-pdf = PDF-dokumentum
resources-kind-audio = hangfelvétel
resources-kind-video = videó
resources-kind-document = dokumentum
resources-kind-ebook = e-könyv
resources-kind-archive = archívum
resources-kind-file = fájl
resources-kind-web-link = webes hivatkozás

## Egy sor menüje

resources-open = Megnyitás
resources-show-in-folder = Megjelenítés a mappában
resources-locate = Megkeresés…
resources-insert-into-text = Beszúrás a szövegbe
resources-insert-picture = A kép beszúrása a szövegbe
resources-insert-link = Hivatkozás beszúrása a szövegbe
resources-edit = Átnevezés és leírás…
resources-detach = Leválasztás erről az elemről
resources-remove = Eltávolítás a projektből…

## A két párbeszédablak

resources-link-dialog-title = Webes hivatkozás hozzáadása
resources-link-dialog-address = Webcím
resources-link-dialog-address-placeholder = https://
resources-link-dialog-name = Cím
resources-link-dialog-name-placeholder = A webhely neve, ha üresen hagyja
resources-link-dialog-add = Hozzáadás
resources-edit-dialog-title = Átnevezés és leírás
resources-edit-dialog-name = Cím
resources-edit-dialog-description = Leírás
resources-edit-dialog-description-placeholder = Mi ez, és miért tartotta meg
resources-edit-dialog-save = Mentés
resources-dialog-cancel = Mégse

## Hozzáadás

resources-large-title = Egy nagy fájl
resources-large-text = A(z) „{ $name }” mérete { $size }. A projektben tárolva a projekt minden másolatával és minden biztonsági mentésével együtt utazik. Hivatkozva ott marad, ahol van, és a projekt csak azt jegyzi meg, hol találja.
resources-large-text-many = A(z) „{ $name }” mérete { $size }, és { $others ->
    [one] még egy másik fájl is
   *[other] még { $others } másik fájl is
} nagy. A projektben tárolva a projekt minden másolatával és minden biztonsági mentésével együtt utaznak. Hivatkozva ott maradnak, ahol vannak, és a projekt csak azt jegyzi meg, hol találja őket.
resources-large-store = Tárolás a projektben
resources-large-link = Inkább hivatkozás rá
resources-large-remember = Ne kérdezzen újra
resources-storing-title = Fájlok hozzáadása a projekthez
resources-store-cancel = Mégse
resources-store-cancelled = Egy fájl sem lett hozzáadva.
resources-store-busy = A fájlok hozzáadása még folyamatban van. Próbálja újra, amikor elkészült.
resources-store-failed = Nem sikerült hozzáadni a fájlokat: { $error }
resources-no-project = Fájlok csatolása előtt nyisson meg egy projektet.
resources-moved-away = A projekt máshová került, miközben a fájl másolása zajlott.
resources-some-failed = { $count ->
    [one] Egy fájlt nem sikerült hozzáadni: { $names }
   *[other] { $count } fájlt nem sikerült hozzáadni: { $names }
}
resources-add-failed = Nem sikerült hozzáadni a forrásokat: { $error }
resources-added = { $count ->
    [one] Egy forrás hozzáadva
   *[other] { $count } forrás hozzáadva
}
resources-dropped-added = { $count ->
    [one] Hozzáadva ennek az elemnek a forrásaként, és hivatkozva a szövegben
   *[other] { $count } fájl hozzáadva ennek az elemnek a forrásaiként, és hivatkozva a szövegben
}
resources-dropped-undo-kept = A szöveg az áthúzás óta megváltozott, így semmi sem lett visszavonva.
resources-undo = Visszavonás
resources-link-refused = A(z) „{ $url }” nem olyan webcím, amelyet a Skribisto megőrizhet. Olyan címet használjon, amely http:// vagy https:// előtaggal kezdődik, felhasználónév és jelszó nélkül.

## Értesítés egyetlen fájlban tárolt projekthez

resources-folder-title = Ez a projekt egyre nagyobb
resources-folder-text = A tárolt fájljai most { $size } helyet foglalnak. Az egyetlen fájlként tárolt projekt minden mentéskor teljes egészében újraíródik, a fájljaival együtt. Mappaként tárolva csak azt menti, ami megváltozott. Most mappaként mentheti, és onnantól a mappában dolgozhat tovább.
resources-folder-switch = Mentés mappaként…
resources-folder-not-now = Most nem
resources-folder-never = Ne kérdezzen újra ennél a projektnél

## Megnyitás

resources-not-found-toast = A(z) „{ $name }” nincs ott, ahol a projekt várja.
resources-unsafe-title = Megnyitja a(z) „{ $name }” fájlt?
resources-unsafe-text = A Skribisto nem tudja, hogy ez a fájltípus biztonságos-e. A megnyitása elindíthat egy programot a számítógépén, és egy projekt származhat valaki mástól. Először jelenítse meg a mappájában, hogy lássa, mi az.
resources-open-anyway = Megnyitás mégis
resources-open-failed = Nem sikerült megnyitni a fájlt: { $error }
resources-no-data-folder = ez a rendszer nem ad a Skribistónak adatmappát, amelyben egy másolatot megnyithatna
resources-link-gone = Ez a hivatkozás olyan forrásra mutat, amely már nincs meg a projektben.
resources-locate-title = Hol van most a(z) „{ $name }”?
resources-relinked-more = { $count ->
    [one] Ugyanabban a mappában még egy hiányzó fájl előkerült, és újra hivatkozva lett.
   *[other] Ugyanabban a mappában még { $count } hiányzó fájl előkerült, és újra hivatkozva lett.
}
resources-changed-elsewhere = A(z) „{ $name }” egy másik alkalmazásban megváltozott. A projekt megtartja az új változatot.

## Szerkesztés

resources-edit-failed = Nem sikerült módosítani a forrást: { $error }
resources-remove-title = Eltávolítja a(z) „{ $title }” forrást a projektből?
resources-remove-text = Minden elemről lekerül, amelyhez csatolva van. A projektben tárolt fájl vele együtt távozik; a hivatkozott fájl és a weboldal ott marad, ahol van. A visszavonás visszahozza.
resources-remove-confirm = Eltávolítás
resources-remove-failed = Nem sikerült eltávolítani a forrást: { $error }
resources-removed-toast = „{ $title }” eltávolítva a projektből.
resources-removed-many-toast = { $count } forrás eltávolítva a projektből.
resources-detached-toast = „{ $title }” leválasztva.
resources-attached-one-toast = „{ $title }” hozzáadva a(z) „{ $item }” forrásaihoz.
resources-attached-many-toast = { $count } forrás hozzáadva ehhez: „{ $item }”.

## A szövegbe

resources-no-text = Előbb helyezze a kurzort egy szövegbe, majd ott szúrja be a forrást.
resources-drop-insert-picture = A kép beszúrása
resources-drop-insert-link = Hivatkozás beszúrása a forrásra

## Folyamat

progress-resources-storing = A fájlok másolása
progress-resources-stored = A fájlok átmásolva

## Egy mappa Források oldala

segment-folder = Mappa
resources-folder-count = { $count ->
    [one] Egy forrás
   *[other] { $count } forrás
}
resources-view-label = Források megjelenítése
resources-view-list = Listaként
resources-view-grid = Rácsként
resources-grid-label = Források
resources-folder-drop-hint = Ejtsen ide fájlokat vagy webcímeket, hogy ehhez a mappához csatolja őket
resources-folder-empty = Ehhez a mappához és a tartalmához még semmi sincs csatolva. Adjon hozzá fájlokat vagy webes hivatkozásokat a fenti gombokkal, vagy ejtse ide őket: a mappához lesznek csatolva.
resources-attached-to = Itt:
resources-attached-separator = ,
resources-attached-more = { $count ->
    [one] és még egy
   *[other] és még { $count }
}
resources-row-name-attached = { $name }, itt: { $items }
resources-open-item = „{ $title }” megnyitása
resources-detach-from = Leválasztás erről: „{ $title }”

## Képforrások a Galériában

gallery-resource-badge = Forrás
gallery-tile-resource = { $title } képforrása: { $description }

## A projektbeállítások Források oldala

settings-page-resources = Források
settings-desc-resources = A fájlok és webes hivatkozások, amelyek már egyetlen elemhez sincsenek csatolva: csatolja őket újra, távolítsa el őket, vagy exportáljon róluk másolatot
settings-resources-description = Egy forrás semmihez sincs csatolva, ha leválasztották az utolsó eleméről, vagy ha az elem, amelyhez csatolva volt, végleg törlődött. A projektben marad, és egy tárolt fájl a projekt minden másolatával és minden biztonsági mentésével együtt utazik, amíg újra nem csatolja vagy el nem távolítja.
settings-resources-group-storage = Tárhely
settings-resources-group-unattached = Semmihez sincs csatolva
settings-resources-stored = A projektben tárolt fájlok: { $size }
settings-resources-folder-advice = Az egyetlen fájlként tárolt projekt minden mentéskor teljesen újraíródik, a tárolt fájljaival együtt. Mappaként tárolva csak azt menti, ami megváltozott, és ez gyorsabb.
settings-resources-count = { $count ->
    [one] Egy forrás
   *[other] { $count } forrás
}
settings-resources-empty = A projekt minden forrása csatolva van valamelyik elemhez.
resources-attach-to-items = Csatolás elemekhez…
resources-export-copies = Másolatok exportálása…
resources-export-copy = Másolat exportálása…
resources-remove-many-title = Eltávolítja a projektből ezt a(z) { $count } forrást?
resources-attach-title = Csatolás elemekhez
resources-attach-hint = Válassza ki az elemeket, amelyekhez csatol. Ctrl+kattintással vagy Shift+kattintással többet is választhat.
resources-attach-empty = Ebben a projektben még nincsenek elemek.
resources-attach-confirm = Csatolás
resources-attached = { $count ->
    [one] Egy forrás csatolva
   *[other] { $count } forrás csatolva
} { $items ->
    [one] egy elemhez
   *[other] { $items } elemhez
}
resources-export-title = Másolatok exportálása egy mappába
resources-exported = { $count ->
    [one] Egy forrás exportálva
   *[other] { $count } forrás exportálva
}
resources-export-missing = { $count ->
    [one] Egy fájl nincs ott, ahol a projekt várja, ezért kimaradt: { $names }
   *[other] { $count } fájl nincs ott, ahol a projekt várja, ezért kimaradtak: { $names }
}
resources-export-failed = { $count ->
    [one] Egy fájlt nem sikerült átmásolni: { $names }
   *[other] { $count } fájlt nem sikerült átmásolni: { $names }
}

## A nagy fájlok beállítása (Szerkesztő ▸ Képek és fájlok)

settings-resources-large-file = 10 MB-nál nagyobb fájlok
settings-resources-large-ask = Rákérdezés minden alkalommal
settings-resources-large-store = Mindig a projektben tárolja
settings-resources-large-link = Mindig a fájlra hivatkozzon
settings-hint-resources-large-file = Mi történik egy 10 MB-nál nagyobb fájllal, amelyet forrásként csatol. A projektben tárolva a projekt minden másolatával és minden biztonsági mentésével együtt utazik. Hivatkozva ott marad, ahol van, és a projekt csak azt jegyzi meg, hol találja.
