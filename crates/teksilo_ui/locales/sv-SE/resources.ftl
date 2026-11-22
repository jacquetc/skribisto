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

## In i texten

resources-no-text = Placera först markören i en text, och infoga sedan resursen där.
resources-drop-insert-picture = Infoga bilden
resources-drop-insert-link = Infoga en länk till resursen

## Förlopp

progress-resources-storing = Kopierar filerna
progress-resources-stored = Filerna har kopierats
