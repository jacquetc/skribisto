# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Resurser: filer och webblänkar som är kopplade till objekt i pärmen.

## Panelen

resources-title = Resurser
shortcut-name-resources-show = Visa resurserna
resources-add-files = Lägg till filer
resources-add-files-title = Lägg till filer i det här objektet
resources-add-link = Lägg till en webblänk
resources-drop-hint = Släpp filer eller webbadresser här för att koppla dem
resources-no-item = Öppna ett objekt för att se dess resurser.
resources-empty = Inget är kopplat till det här objektet än. Lägg till filer eller webblänkar med knapparna ovan, eller släpp dem här.
resources-actions = Åtgärder
resources-not-found = Hittades inte
resources-row-linked = Länkad fil
resources-row-linked-size = Länkad fil, { $size }
resources-row-name = { $title }, { $kind }, { $detail }
resources-row-name-missing = { $title }, { $kind }, { $detail }, hittades inte

## Filtyper, så som en skärmläsare läser upp dem

resources-kind-picture = bild
resources-kind-pdf = PDF-dokument
resources-kind-audio = ljudinspelning
resources-kind-video = video
resources-kind-document = dokument
resources-kind-ebook = e-bok
resources-kind-archive = arkiv
resources-kind-file = fil
resources-kind-web-link = webblänk

## En rads meny

resources-open = Öppna
resources-show-in-folder = Visa i mappen
resources-locate = Hitta igen…
resources-insert-into-text = Infoga i texten
resources-insert-picture = Infoga bilden i texten
resources-insert-link = Infoga en länk i texten
resources-edit = Byt namn och beskriv…
resources-detach = Koppla bort från det här objektet
resources-remove = Ta bort från projektet…

## De två dialogrutorna

resources-link-dialog-title = Lägg till en webblänk
resources-link-dialog-address = Adress
resources-link-dialog-address-placeholder = https://
resources-link-dialog-name = Titel
resources-link-dialog-name-placeholder = Webbplatsens namn, om fältet lämnas tomt
resources-link-dialog-add = Lägg till
resources-edit-dialog-title = Byt namn och beskriv
resources-edit-dialog-name = Titel
resources-edit-dialog-description = Beskrivning
resources-edit-dialog-description-placeholder = Vad det här är, och varför du sparade det
resources-edit-dialog-save = Spara
resources-dialog-cancel = Avbryt

## Lägga till

resources-large-title = En stor fil
resources-large-text = ”{ $name }” är { $size }. Lagrad i projektet följer den med varje kopia och varje säkerhetskopia av det. Länkad stannar den där den är, och projektet kommer bara ihåg var den finns.
resources-large-text-many = ”{ $name }” är { $size }, och { $others ->
    [one] en annan fil är
   *[other] { $others } andra filer är
} också stora. Lagrade i projektet följer de med varje kopia och varje säkerhetskopia av det. Länkade stannar de där de är, och projektet kommer bara ihåg var de finns.
resources-large-store = Lagra den i projektet
resources-large-link = Länka till den i stället
resources-large-remember = Fråga inte igen
resources-storing-title = Lägger till filer i projektet
resources-store-cancel = Avbryt
resources-store-cancelled = Inga filer lades till.
resources-store-busy = Filer läggs fortfarande till. Försök igen när de har kommit in.
resources-store-failed = Filerna kunde inte läggas till: { $error }
resources-no-project = Öppna ett projekt innan du kopplar filer.
resources-moved-away = Projektet flyttades medan filen kopierades.
resources-some-failed = { $count ->
    [one] En fil kunde inte läggas till: { $names }
   *[other] { $count } filer kunde inte läggas till: { $names }
}
resources-add-failed = Resurserna kunde inte läggas till: { $error }
resources-added = { $count ->
    [one] En resurs tillagd
   *[other] { $count } resurser tillagda
}
resources-dropped-added = { $count ->
    [one] Tillagd som resurs för det här objektet, och länkad i texten
   *[other] { $count } filer tillagda som resurser för det här objektet, och länkade i texten
}
resources-undo = Ångra
resources-link-refused = ”{ $url }” är inte en webbadress som Skribisto kan spara. Använd en adress som börjar med http:// eller https://, utan användarnamn eller lösenord.

## Meddelandet för ett projekt i en enda fil

resources-folder-title = Det här projektet börjar bli stort
resources-folder-text = Dess lagrade filer tar nu { $size }. Ett projekt som sparas som en enda fil skrivs om helt vid varje sparning, även dess filer. Sparat som mapp sparar det bara det som har ändrats. Du kan spara det som mapp nu, och fortsätta arbeta i mappen från och med då.
resources-folder-switch = Spara som mapp…
resources-folder-not-now = Inte nu
resources-folder-never = Fråga inte igen för det här projektet

## Öppna

resources-not-found-toast = ”{ $name }” finns inte där projektet väntar sig.
resources-unsafe-title = Öppna ”{ $name }”?
resources-unsafe-text = Skribisto vet inte om den här sortens fil är säker. Att öppna den kan starta ett program på din dator, och ett projekt kan komma från någon annan. Visa den i dess mapp först för att se vad den är.
resources-open-anyway = Öppna ändå
resources-open-failed = Filen kunde inte öppnas: { $error }
resources-no-data-folder = det här systemet ger inte Skribisto någon datamapp att öppna en kopia i
resources-link-gone = Den här länken pekar på en resurs som projektet inte längre har.
resources-locate-title = Var är ”{ $name }” nu?
resources-relinked-more = { $count ->
    [one] Ytterligare en saknad fil hittades i samma mapp och länkades igen.
   *[other] Ytterligare { $count } saknade filer hittades i samma mapp och länkades igen.
}
resources-changed-elsewhere = ”{ $name }” ändrades i en annan app. Projektet behåller den nya versionen.

## Redigera

resources-edit-failed = Resursen kunde inte ändras: { $error }
resources-remove-title = Ta bort ”{ $title }” från projektet?
resources-remove-text = Den tas bort från varje objekt den är kopplad till. En fil som är lagrad i projektet försvinner med den; en länkad fil och en webbsida stannar där de är. Ångra tar tillbaka den.
resources-remove-confirm = Ta bort
resources-remove-failed = Resursen kunde inte tas bort: { $error }
resources-removed-toast = ”{ $title }” togs bort från projektet.
resources-removed-many-toast = { $count } resurser togs bort från projektet.
resources-detached-toast = ”{ $title }” kopplades loss.
resources-attached-one-toast = ”{ $title }” lades till bland resurserna för ”{ $item }”.
resources-attached-many-toast = { $count } resurser lades till ”{ $item }”.

## In i texten

resources-no-text = Placera först markören i en text, och infoga sedan resursen där.
resources-drop-insert-picture = Infoga bilden
resources-drop-insert-link = Infoga en länk till resursen

## Förlopp

progress-resources-storing = Kopierar filerna
progress-resources-stored = Filerna har kopierats

## En mapps sida Resurser

segment-folder = Mapp
resources-folder-count = { $count ->
    [one] En resurs
   *[other] { $count } resurser
}
resources-view-label = Visa resurser som
resources-view-list = Lista
resources-view-grid = Rutnät
resources-grid-label = Resurser
resources-folder-drop-hint = Släpp filer eller webbadresser här för att koppla dem till den här mappen
resources-folder-empty = Inget är kopplat till den här mappen eller till det den innehåller än. Lägg till filer eller webblänkar med knapparna ovan, eller släpp dem här: de kopplas till mappen.
resources-attached-to = På
resources-attached-separator = ,
resources-attached-more = { $count ->
    [one] och en till
   *[other] och { $count } till
}
resources-row-name-attached = { $name }, på { $items }
resources-open-item = Öppna ”{ $title }”
resources-detach-from = Koppla bort från ”{ $title }”

## Bildresurser i Galleriet

gallery-resource-badge = Resurs
gallery-tile-resource = Bildresurs till { $title }: { $description }

## Projektinställningarnas sida Resurser

settings-page-resources = Resurser
settings-desc-resources = De filer och webblänkar som inget objekt har längre: koppla dem igen, ta bort dem eller exportera kopior
settings-resources-description = En resurs är kopplad till ingenting när den har kopplats bort från sitt sista objekt, eller när objektet den satt på har raderats för gott. Den finns kvar i projektet, och en lagrad fil följer med varje kopia och varje säkerhetskopia av det, tills du kopplar den igen eller tar bort den.
settings-resources-group-storage = Lagring
settings-resources-group-unattached = Kopplade till ingenting
settings-resources-stored = Filer lagrade i det här projektet: { $size }
settings-resources-folder-advice = Ett projekt som sparas som en enda fil skrivs om helt vid varje sparning, även dess lagrade filer. Sparat som mapp sparar det bara det som har ändrats, och det går fortare.
settings-resources-count = { $count ->
    [one] En resurs
   *[other] { $count } resurser
}
settings-resources-empty = Varje resurs i det här projektet är kopplad till ett objekt.
resources-attach-to-items = Koppla till objekt…
resources-export-copies = Exportera kopior…
resources-export-copy = Exportera en kopia…
resources-remove-many-title = Ta bort { $count } resurser från projektet?
resources-attach-title = Koppla till objekt
resources-attach-hint = Välj de objekt som ska kopplas. Ctrl+klick eller Skift+klick väljer flera.
resources-attach-empty = Det här projektet har inga objekt än.
resources-attach-confirm = Koppla
resources-attached = { $count ->
    [one] En resurs kopplad
   *[other] { $count } resurser kopplade
} till { $items ->
    [one] ett objekt
   *[other] { $items } objekt
}
resources-export-title = Exportera kopior till en mapp
resources-exported = { $count ->
    [one] En resurs exporterad
   *[other] { $count } resurser exporterade
}
resources-export-missing = { $count ->
    [one] En fil finns inte där projektet väntar sig den, och utelämnades: { $names }
   *[other] { $count } filer finns inte där projektet väntar sig dem, och utelämnades: { $names }
}
resources-export-failed = { $count ->
    [one] En fil kunde inte kopieras: { $names }
   *[other] { $count } filer kunde inte kopieras: { $names }
}

## Inställningen för stora filer (Redigerarens beteende)

settings-resources-large-file = Filer över 10 MB
settings-resources-large-ask = Fråga varje gång
settings-resources-large-store = Lagra alltid i projektet
settings-resources-large-link = Länka alltid till filen
settings-hint-resources-large-file = Vad som händer med en fil över 10 MB som du kopplar som resurs. Lagrad i projektet följer den med varje kopia och varje säkerhetskopia av det. Länkad stannar den där den är, och projektet minns bara var den finns.
