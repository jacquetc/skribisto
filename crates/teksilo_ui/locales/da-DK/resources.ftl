# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Ressourcer: filer og weblinks, der er knyttet til elementer i ringbindet.

## Panelet

resources-title = Ressourcer
shortcut-name-resources-show = Vis ressourcerne
resources-add-files = Tilføj filer
resources-add-files-title = Tilføj filer til dette element
resources-add-link = Tilføj et weblink
resources-drop-hint = Slip filer eller webadresser her for at knytte dem til
resources-drop-hint-unattached = Slip filer eller webadresser her for at tilføje dem uden at knytte dem til et element
resources-no-item = Åbn et element for at se dets ressourcer.
resources-empty = Der er endnu ikke knyttet noget til dette element. Tilføj filer eller weblinks med knapperne ovenfor, eller slip dem her.
resources-dock-view-label = Ressourcer at vise
resources-dock-view-item = Dette element
resources-dock-view-unattached = Ikke knyttet
resources-dock-view-item-hint = Filerne og linkene for det element, du er på. Det, du tilføjer, knyttes til det.
resources-dock-view-unattached-hint = Filerne og linkene, der ikke er knyttet til noget element, fx research, du endnu ikke har sorteret. Det, du tilføjer her, knyttes ikke til noget.
resources-unattached-empty = Hver ressource i dette projekt er knyttet til et element. Filer og links, der tilføjes her, er ikke knyttet til noget, før du knytter dem.
resources-actions = Handlinger
resources-not-found = Ikke fundet
resources-row-linked = Linket fil
resources-row-linked-size = Linket fil, { $size }
resources-row-name = { $title }, { $kind }, { $detail }
resources-row-name-missing = { $title }, { $kind }, { $detail }, ikke fundet

## Filtyper, som en skærmlæser læser dem op

resources-kind-picture = billede
resources-kind-pdf = PDF-dokument
resources-kind-audio = lydoptagelse
resources-kind-video = video
resources-kind-document = dokument
resources-kind-ebook = e-bog
resources-kind-archive = arkiv
resources-kind-file = fil
resources-kind-web-link = weblink

## Menuen for en række

resources-open = Åbn
resources-show-in-folder = Vis i mappe
resources-locate = Find igen…
resources-insert-into-text = Indsæt i teksten
resources-insert-picture = Indsæt billedet i teksten
resources-insert-link = Indsæt et link i teksten
resources-edit = Omdøb og beskriv…
resources-detach = Løsn fra dette element
resources-remove = Fjern fra projektet…

## De to dialoger

resources-link-dialog-title = Tilføj et weblink
resources-link-dialog-address = Adresse
resources-link-dialog-address-placeholder = https://
resources-link-dialog-name = Titel
resources-link-dialog-name-placeholder = Navnet på webstedet, hvis feltet er tomt
resources-link-dialog-add = Tilføj
resources-edit-dialog-title = Omdøb og beskriv
resources-edit-dialog-name = Titel
resources-edit-dialog-description = Beskrivelse
resources-edit-dialog-description-placeholder = Hvad det er, og hvorfor du har gemt det
resources-edit-dialog-save = Gem
resources-dialog-cancel = Annullér

## Tilføjelse

resources-large-title = En stor fil
resources-large-text = »{ $name }« fylder { $size }. Gemt i projektet følger den med hver kopi og hver sikkerhedskopi af det. Linket bliver den, hvor den er, og projektet husker kun, hvor den kan findes.
resources-large-text-many = »{ $name }« fylder { $size }, og { $others ->
    [one] en anden fil er
   *[other] { $others } andre filer er
} også store. Gemt i projektet følger de med hver kopi og hver sikkerhedskopi af det. Linket bliver de, hvor de er, og projektet husker kun, hvor de kan findes.
resources-large-store = Gem den i projektet
resources-large-link = Link til den i stedet
resources-large-remember = Spørg ikke igen
resources-storing-title = Tilføjer filer til projektet
resources-store-cancel = Annullér
resources-store-cancelled = Ingen filer blev tilføjet.
resources-store-busy = Der bliver stadig tilføjet filer. Prøv igen, når de er kommet ind.
resources-store-failed = Filerne kunne ikke tilføjes: { $error }
resources-no-project = Åbn et projekt, før du knytter filer til.
resources-moved-away = Projektet blev flyttet, mens filen blev kopieret.
resources-some-failed = { $count ->
    [one] En fil kunne ikke tilføjes: { $names }
   *[other] { $count } filer kunne ikke tilføjes: { $names }
}
resources-add-failed = Ressourcerne kunne ikke tilføjes: { $error }
resources-added = { $count ->
    [one] En ressource tilføjet
   *[other] { $count } ressourcer tilføjet
}
resources-dropped-added = { $count ->
    [one] Tilføjet som ressource for dette element og linket i teksten
   *[other] { $count } filer tilføjet som ressourcer for dette element og linket i teksten
}
resources-dropped-undo-kept = Teksten er ændret siden slippet, så intet blev fortrudt.
resources-undo = Fortryd
resources-link-refused = »{ $url }« er ikke en webadresse, som Skribisto kan gemme. Brug en adresse, der begynder med http:// eller https://, uden brugernavn eller adgangskode.

## Meddelelsen for et projekt i én fil

resources-folder-title = Dette projekt er ved at blive stort
resources-folder-text = Dets gemte filer fylder nu { $size }. Et projekt, der gemmes som én fil, skrives helt om ved hver gemning, også dets filer. Gemt som mappe gemmer det kun det, der er ændret. Du kan gemme det som mappe nu og arbejde videre i mappen fra da af.
resources-folder-switch = Gem som mappe…
resources-folder-not-now = Ikke nu
resources-folder-never = Spørg ikke igen for dette projekt

## Åbning

resources-not-found-toast = »{ $name }« er ikke der, hvor projektet forventer den.
resources-unsafe-title = Åbn »{ $name }«?
resources-unsafe-text = Skribisto ved ikke, om denne type fil er sikker. Hvis du åbner den, kan den starte et program på din computer, og et projekt kan komme fra en anden. Vis den først i dens mappe for at se, hvad den er.
resources-open-anyway = Åbn alligevel
resources-open-failed = Filen kunne ikke åbnes: { $error }
resources-no-data-folder = dette system giver ikke Skribisto nogen datamappe at åbne en kopi i
resources-link-gone = Dette link peger på en ressource, som projektet ikke længere har.
resources-locate-title = Hvor er »{ $name }« nu?
resources-relinked-more = { $count ->
    [one] Endnu en manglende fil blev fundet i samme mappe og linket igen.
   *[other] { $count } manglende filer mere blev fundet i samme mappe og linket igen.
}
resources-changed-elsewhere = »{ $name }« blev ændret i en anden app. Projektet beholder den nye version.

## Redigering

resources-edit-failed = Ressourcen kunne ikke ændres: { $error }
resources-remove-title = Fjern »{ $title }« fra projektet?
resources-remove-text = Den fjernes fra hvert element, den er knyttet til. En fil, der er gemt i projektet, forsvinder med den; en linket fil og en webside bliver, hvor de er. Fortryd bringer den tilbage.
resources-remove-confirm = Fjern
resources-remove-failed = Ressourcen kunne ikke fjernes: { $error }
resources-removed-toast = “{ $title }” blev fjernet fra projektet.
resources-removed-many-toast = { $count } ressourcer blev fjernet fra projektet.
resources-detached-toast = “{ $title }” blev frakoblet.
resources-attached-one-toast = “{ $title }” føjet til ressourcerne for “{ $item }”.
resources-attached-many-toast = { $count } ressourcer føjet til “{ $item }”.

## Ind i teksten

resources-no-text = Placér først markøren i en tekst, og indsæt så ressourcen der.
resources-drop-insert-picture = Indsæt billedet
resources-drop-insert-link = Indsæt et link til ressourcen

## Fremskridt

progress-resources-storing = Kopierer filerne
progress-resources-stored = Filer kopieret

## En mappes side Ressourcer

segment-folder = Mappe
resources-folder-count = { $count ->
    [one] Én ressource
   *[other] { $count } ressourcer
}
resources-view-label = Vis ressourcer som
resources-view-list = Liste
resources-view-grid = Gitter
resources-grid-label = Ressourcer
resources-folder-drop-hint = Slip filer eller webadresser her for at knytte dem til denne mappe
resources-folder-empty = Der er endnu ikke knyttet noget til denne mappe eller til det, den indeholder. Tilføj filer eller weblinks med knapperne ovenfor, eller slip dem her: de knyttes til mappen.
resources-attached-to = På
resources-attached-separator = ,
resources-attached-more = { $count ->
    [one] og én mere
   *[other] og { $count } mere
}
resources-row-name-attached = { $name }, på { $items }
resources-open-item = Åbn »{ $title }«
resources-detach-from = Løsn fra »{ $title }«

## Billedressourcer i Galleriet

gallery-resource-badge = Ressource
gallery-tile-resource = Billedressource til { $title }: { $description }

## Projektindstillingernes side Ressourcer

settings-page-resources = Ressourcer
settings-desc-resources = De filer og weblinks, som intet element længere har: knyt dem til igen, fjern dem, eller eksportér kopier
settings-resources-description = En ressource er knyttet til ingenting, når den er løsnet fra sit sidste element, eller når det element, den var knyttet til, er slettet for altid. Den bliver i projektet, og en gemt fil følger med hver kopi og hver sikkerhedskopi af det, indtil du knytter den til igen eller fjerner den.
settings-resources-group-storage = Lager
settings-resources-group-unattached = Knyttet til ingenting
settings-resources-stored = Filer gemt i dette projekt: { $size }
settings-resources-folder-advice = Et projekt, der gemmes som én fil, skrives helt om ved hver gemning, også dets gemte filer. Gemt som mappe gemmer det kun det, der er ændret, og det går hurtigere.
settings-resources-count = { $count ->
    [one] Én ressource
   *[other] { $count } ressourcer
}
settings-resources-empty = Hver ressource i dette projekt er knyttet til et element.
resources-attach-to-items = Knyt til elementer…
resources-export-copies = Eksportér kopier…
resources-export-copy = Eksportér en kopi…
resources-remove-many-title = Fjern { $count } ressourcer fra projektet?
resources-attach-title = Knyt til elementer
resources-attach-hint = Vælg de elementer, der skal knyttes til. Ctrl+klik eller Skift+klik vælger flere.
resources-attach-empty = Dette projekt har endnu ingen elementer.
resources-attach-confirm = Knyt til
resources-attached = { $count ->
    [one] Én ressource
   *[other] { $count } ressourcer
} knyttet til { $items ->
    [one] ét element
   *[other] { $items } elementer
}
resources-export-title = Eksportér kopier til en mappe
resources-exported = { $count ->
    [one] Én ressource eksporteret
   *[other] { $count } ressourcer eksporteret
}
resources-export-missing = { $count ->
    [one] Én fil er ikke der, hvor projektet forventer den, og blev udeladt: { $names }
   *[other] { $count } filer er ikke der, hvor projektet forventer dem, og blev udeladt: { $names }
}
resources-export-failed = { $count ->
    [one] Én fil kunne ikke kopieres: { $names }
   *[other] { $count } filer kunne ikke kopieres: { $names }
}

## Indstillingen for store filer (Editorens adfærd)

settings-resources-large-file = Filer over 10 MB
settings-resources-large-ask = Spørg hver gang
settings-resources-large-store = Gem altid i projektet
settings-resources-large-link = Link altid til filen
settings-hint-resources-large-file = Hvad der sker med en fil over 10 MB, som du knytter til som ressource. Gemt i projektet følger den med hver kopi og hver sikkerhedskopi af det. Linket bliver den, hvor den er, og projektet husker kun, hvor den kan findes.
