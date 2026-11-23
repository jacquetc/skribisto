# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Ressurser: filer og nettlenker som er knyttet til elementer i permen.

## Panelet

resources-title = Ressurser
shortcut-name-resources-show = Vis ressursene
resources-add-files = Legg til filer
resources-add-files-title = Legg til filer i dette elementet
resources-add-link = Legg til en nettlenke
resources-drop-hint = Slipp filer eller nettadresser her for å knytte dem til
resources-drop-hint-unattached = Slipp filer eller nettadresser her for å legge dem til uten å knytte dem til et element
resources-no-item = Åpne et element for å se ressursene.
resources-empty = Ingenting er knyttet til dette elementet ennå. Legg til filer eller nettlenker med knappene over, eller slipp dem her.
resources-dock-view-label = Ressurser som vises
resources-dock-view-item = Dette elementet
resources-dock-view-unattached = Ikke knyttet
resources-dock-view-item-hint = Filene og lenkene til elementet du er på. Det du legger til, knyttes til det.
resources-dock-view-unattached-hint = Filene og lenkene som ikke er knyttet til noe element, for eksempel research du ikke har sortert ennå. Det du legger til her, knyttes ikke til noe.
resources-unattached-empty = Hver ressurs i dette prosjektet er knyttet til et element. Filer og lenker som legges til her, er ikke knyttet til noe før du knytter dem.
resources-actions = Handlinger
resources-not-found = Ikke funnet
resources-row-linked = Lenket fil
resources-row-linked-size = Lenket fil, { $size }
resources-row-name = { $title }, { $kind }, { $detail }
resources-row-name-missing = { $title }, { $kind }, { $detail }, ikke funnet

## Filtyper, slik en skjermleser leser dem opp

resources-kind-picture = bilde
resources-kind-pdf = PDF-dokument
resources-kind-audio = lydopptak
resources-kind-video = video
resources-kind-document = dokument
resources-kind-ebook = e-bok
resources-kind-archive = arkiv
resources-kind-file = fil
resources-kind-web-link = nettlenke

## Menyen for en rad

resources-open = Åpne
resources-show-in-folder = Vis i mappe
resources-locate = Finn igjen…
resources-insert-into-text = Sett inn i teksten
resources-insert-picture = Sett inn bildet i teksten
resources-insert-link = Sett inn en lenke i teksten
resources-edit = Gi nytt navn og beskriv…
resources-detach = Løsne fra dette elementet
resources-remove = Fjern fra prosjektet…

## De to dialogene

resources-link-dialog-title = Legg til en nettlenke
resources-link-dialog-address = Adresse
resources-link-dialog-address-placeholder = https://
resources-link-dialog-name = Tittel
resources-link-dialog-name-placeholder = Navnet på nettstedet, hvis feltet står tomt
resources-link-dialog-add = Legg til
resources-edit-dialog-title = Gi nytt navn og beskriv
resources-edit-dialog-name = Tittel
resources-edit-dialog-description = Beskrivelse
resources-edit-dialog-description-placeholder = Hva dette er, og hvorfor du tok vare på det
resources-edit-dialog-save = Lagre
resources-dialog-cancel = Avbryt

## Legge til

resources-large-title = En stor fil
resources-large-text = «{ $name }» er på { $size }. Lagret i prosjektet følger den med hver kopi og hver sikkerhetskopi av det. Lenket blir den der den er, og prosjektet husker bare hvor den finnes.
resources-large-text-many = «{ $name }» er på { $size }, og { $others ->
    [one] én annen fil er
   *[other] { $others } andre filer er
} også store. Lagret i prosjektet følger de med hver kopi og hver sikkerhetskopi av det. Lenket blir de der de er, og prosjektet husker bare hvor de finnes.
resources-large-store = Lagre den i prosjektet
resources-large-link = Lenk til den i stedet
resources-large-remember = Ikke spør igjen
resources-storing-title = Legger til filer i prosjektet
resources-store-cancel = Avbryt
resources-store-cancelled = Ingen filer ble lagt til.
resources-store-busy = Filer blir fortsatt lagt til. Prøv igjen når de er kommet inn.
resources-store-failed = Filene kunne ikke legges til: { $error }
resources-no-project = Åpne et prosjekt før du knytter til filer.
resources-moved-away = Prosjektet ble flyttet mens filen ble kopiert.
resources-some-failed = { $count ->
    [one] Én fil kunne ikke legges til: { $names }
   *[other] { $count } filer kunne ikke legges til: { $names }
}
resources-add-failed = Ressursene kunne ikke legges til: { $error }
resources-added = { $count ->
    [one] Én ressurs lagt til
   *[other] { $count } ressurser lagt til
}
resources-dropped-added = { $count ->
    [one] Lagt til som ressurs for dette elementet, og lenket i teksten
   *[other] { $count } filer lagt til som ressurser for dette elementet, og lenket i teksten
}
resources-dropped-undo-kept = Teksten er endret siden slippet, så ingenting ble angret.
resources-undo = Angre
resources-link-refused = «{ $url }» er ikke en nettadresse Skribisto kan ta vare på. Bruk en adresse som begynner med http:// eller https://, uten brukernavn eller passord.

## Meldingen for et prosjekt i én fil

resources-folder-title = Dette prosjektet begynner å bli stort
resources-folder-text = De lagrede filene tar nå { $size }. Et prosjekt som holdes i én fil, skrives helt på nytt ved hver lagring, filene også. Holdt som mappe lagrer det bare det som er endret. Du kan lagre det som mappe nå, og jobbe videre i mappen fra da av.
resources-folder-switch = Lagre som mappe…
resources-folder-not-now = Ikke nå
resources-folder-never = Ikke spør igjen for dette prosjektet

## Åpne

resources-not-found-toast = «{ $name }» er ikke der prosjektet venter å finne den.
resources-unsafe-title = Åpne «{ $name }»?
resources-unsafe-text = Skribisto vet ikke om denne typen fil er trygg. Å åpne den kan starte et program på datamaskinen din, og et prosjekt kan komme fra noen andre. Vis den i mappen først for å se hva den er.
resources-open-anyway = Åpne likevel
resources-open-failed = Filen kunne ikke åpnes: { $error }
resources-no-data-folder = dette systemet gir ikke Skribisto noen datamappe å åpne en kopi i
resources-link-gone = Denne lenken peker til en ressurs prosjektet ikke lenger har.
resources-locate-title = Hvor er «{ $name }» nå?
resources-relinked-more = { $count ->
    [one] Én fil til som manglet, ble funnet i samme mappe og lenket på nytt.
   *[other] { $count } filer til som manglet, ble funnet i samme mappe og lenket på nytt.
}
resources-changed-elsewhere = «{ $name }» ble endret i en annen app. Prosjektet beholder den nye versjonen.

## Redigere

resources-edit-failed = Ressursen kunne ikke endres: { $error }
resources-remove-title = Fjerne «{ $title }» fra prosjektet?
resources-remove-text = Den tas bort fra hvert element den er knyttet til. En fil som er lagret i prosjektet, forsvinner med den; en lenket fil og en nettside blir der de er. Angre henter den tilbake.
resources-remove-confirm = Fjern
resources-remove-failed = Ressursen kunne ikke fjernes: { $error }
resources-removed-toast = «{ $title }» ble fjernet fra prosjektet.
resources-removed-many-toast = { $count } ressurser ble fjernet fra prosjektet.
resources-detached-toast = «{ $title }» ble koblet fra.
resources-attached-one-toast = «{ $title }» lagt til ressursene for «{ $item }».
resources-attached-many-toast = { $count } ressurser lagt til «{ $item }».

## Inn i teksten

resources-no-text = Plasser først markøren i en tekst, og sett så inn ressursen der.
resources-drop-insert-picture = Sett inn bildet
resources-drop-insert-link = Sett inn en lenke til ressursen

## Fremdrift

progress-resources-storing = Kopierer filene
progress-resources-stored = Filene er kopiert

## En mappes side Ressurser

segment-folder = Mappe
resources-folder-count = { $count ->
    [one] Én ressurs
   *[other] { $count } ressurser
}
resources-view-label = Vis ressurser som
resources-view-list = Liste
resources-view-grid = Rutenett
resources-grid-label = Ressurser
resources-folder-drop-hint = Slipp filer eller nettadresser her for å knytte dem til denne mappen
resources-folder-empty = Ingenting er knyttet til denne mappen eller til det den inneholder ennå. Legg til filer eller nettlenker med knappene over, eller slipp dem her: de knyttes til mappen.
resources-attached-to = På
resources-attached-separator = ,
resources-attached-more = { $count ->
    [one] og én til
   *[other] og { $count } til
}
resources-row-name-attached = { $name }, på { $items }
resources-open-item = Åpne «{ $title }»
resources-detach-from = Løsne fra «{ $title }»

## Bilderessurser i Galleriet

gallery-resource-badge = Ressurs
gallery-tile-resource = Bilderessurs for { $title }: { $description }

## Prosjektinnstillingenes side Ressurser

settings-page-resources = Ressurser
settings-desc-resources = Filene og nettlenkene som ikke noe element har lenger: knytt dem til igjen, fjern dem, eller eksporter kopier
settings-resources-description = En ressurs er knyttet til ingenting når den er løsnet fra sitt siste element, eller når elementet den var knyttet til er slettet for godt. Den blir i prosjektet, og en lagret fil følger med hver kopi og hver sikkerhetskopi av det, til du knytter den til igjen eller fjerner den.
settings-resources-group-storage = Lagring
settings-resources-group-unattached = Knyttet til ingenting
settings-resources-stored = Filer lagret i dette prosjektet: { $size }
settings-resources-folder-advice = Et prosjekt som holdes i én fil, skrives helt på nytt ved hver lagring, de lagrede filene også. Holdt som mappe lagrer det bare det som er endret, og det går raskere.
settings-resources-count = { $count ->
    [one] Én ressurs
   *[other] { $count } ressurser
}
settings-resources-empty = Hver ressurs i dette prosjektet er knyttet til et element.
resources-attach-to-items = Knytt til elementer…
resources-export-copies = Eksporter kopier…
resources-export-copy = Eksporter en kopi…
resources-remove-many-title = Fjerne { $count } ressurser fra prosjektet?
resources-attach-title = Knytt til elementer
resources-attach-hint = Velg elementene det skal knyttes til. Ctrl+klikk eller Skift+klikk velger flere.
resources-attach-empty = Dette prosjektet har ingen elementer ennå.
resources-attach-confirm = Knytt til
resources-attached = { $count ->
    [one] Én ressurs
   *[other] { $count } ressurser
} knyttet til { $items ->
    [one] ett element
   *[other] { $items } elementer
}
resources-export-title = Eksporter kopier til en mappe
resources-exported = { $count ->
    [one] Én ressurs eksportert
   *[other] { $count } ressurser eksportert
}
resources-export-missing = { $count ->
    [one] Én fil er ikke der prosjektet venter den, og ble utelatt: { $names }
   *[other] { $count } filer er ikke der prosjektet venter dem, og ble utelatt: { $names }
}
resources-export-failed = { $count ->
    [one] Én fil kunne ikke kopieres: { $names }
   *[other] { $count } filer kunne ikke kopieres: { $names }
}

## Innstillingen for store filer (Redigeringsatferd)

settings-resources-large-file = Filer over 10 MB
settings-resources-large-ask = Spør hver gang
settings-resources-large-store = Lagre alltid i prosjektet
settings-resources-large-link = Lenk alltid til filen
settings-hint-resources-large-file = Hva som skjer med en fil over 10 MB som du knytter til som ressurs. Lagret i prosjektet følger den med hver kopi og hver sikkerhetskopi av det. Lenket blir den der den er, og prosjektet husker bare hvor den finnes.
