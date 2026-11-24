# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Bronnen: bestanden en weblinks die aan items in de ordner hangen.

## Het paneel

resources-title = Bronnen
shortcut-name-resources-show = Bronnen tonen
resources-add-files = Bestanden toevoegen
resources-add-files-title = Bestanden aan dit item toevoegen
resources-add-link = Een weblink toevoegen
resources-drop-hint = Sleep bestanden of webadressen hierheen om ze toe te voegen
resources-drop-hint-unattached = Sleep bestanden of webadressen hierheen om ze toe te voegen zonder ze ergens aan te hangen
resources-no-item = Open een item om de bronnen ervan te zien.
resources-empty = Aan dit item hangt nog niets. Voeg bestanden of weblinks toe met de knoppen hierboven, of sleep ze hierheen.
resources-dock-view-label = Te tonen bronnen
resources-dock-view-item = Dit item
resources-dock-view-unattached = Nergens aan
resources-dock-view-item-hint = De bestanden en links van het item waar u bent. Wat u toevoegt, hangt eraan.
resources-dock-view-unattached-hint = De bestanden en links die nergens aan hangen, zoals onderzoek dat u nog niet hebt ingedeeld. Wat u hier toevoegt, hangt nergens aan.
resources-unattached-empty = Elke bron van dit project hangt aan een item. Bestanden en links die u hier toevoegt, hangen nergens aan tot u ze ergens aan hangt.
resources-actions = Acties
resources-not-found = Niet gevonden
resources-row-linked = Gekoppeld bestand
resources-row-linked-size = Gekoppeld bestand, { $size }
resources-row-name = { $title }, { $kind }, { $detail }
resources-row-name-missing = { $title }, { $kind }, { $detail }, niet gevonden

## Bestandssoorten, zoals een schermlezer ze uitspreekt

resources-kind-picture = afbeelding
resources-kind-pdf = pdf-document
resources-kind-audio = geluidsopname
resources-kind-video = video
resources-kind-document = document
resources-kind-ebook = e-book
resources-kind-archive = archief
resources-kind-file = bestand
resources-kind-web-link = weblink

## Het menu van een rij

resources-open = Openen
resources-show-in-folder = Tonen in map
resources-locate = Terugvinden…
resources-insert-into-text = In de tekst invoegen
resources-insert-picture = De afbeelding in de tekst invoegen
resources-insert-link = Een koppeling in de tekst invoegen
resources-edit = Naam wijzigen en beschrijven…
resources-detach = Losmaken van dit item
resources-remove = Uit het project verwijderen…

## De twee dialoogvensters

resources-link-dialog-title = Een weblink toevoegen
resources-link-dialog-address = Adres
resources-link-dialog-address-placeholder = https://
resources-link-dialog-name = Titel
resources-link-dialog-name-placeholder = De naam van de site, als u dit leeg laat
resources-link-dialog-add = Toevoegen
resources-edit-dialog-title = Naam wijzigen en beschrijven
resources-edit-dialog-name = Titel
resources-edit-dialog-description = Beschrijving
resources-edit-dialog-description-placeholder = Wat dit is, en waarom u het hebt bewaard
resources-edit-dialog-save = Opslaan
resources-dialog-cancel = Annuleren

## Toevoegen

resources-large-title = Een groot bestand
resources-large-text = “{ $name }” is { $size }. Opgeslagen in het project reist het mee met elke kopie en elke back-up ervan. Gekoppeld blijft het waar het is, en onthoudt het project alleen waar het te vinden is.
resources-large-text-many = “{ $name }” is { $size }, en { $others ->
    [one] nog één ander bestand is
   *[other] nog { $others } andere bestanden zijn
} ook groot. Opgeslagen in het project reizen ze mee met elke kopie en elke back-up ervan. Gekoppeld blijven ze waar ze zijn, en onthoudt het project alleen waar ze te vinden zijn.
resources-large-store = In het project opslaan
resources-large-link = Liever koppelen
resources-large-remember = Niet meer vragen
resources-storing-title = Bestanden aan het project toevoegen
resources-store-cancel = Annuleren
resources-store-cancelled = Er zijn geen bestanden toegevoegd.
resources-store-busy = Er worden nog bestanden toegevoegd. Probeer het opnieuw als ze binnen zijn.
resources-store-failed = De bestanden konden niet worden toegevoegd: { $error }
resources-no-project = Open een project voordat u bestanden toevoegt.
resources-moved-away = Het project is verplaatst terwijl het bestand werd gekopieerd.
resources-some-failed = { $count ->
    [one] Eén bestand kon niet worden toegevoegd: { $names }
   *[other] { $count } bestanden konden niet worden toegevoegd: { $names }
}
resources-add-failed = De bronnen konden niet worden toegevoegd: { $error }
resources-added = { $count ->
    [one] Eén bron toegevoegd
   *[other] { $count } bronnen toegevoegd
}
resources-dropped-added = { $count ->
    [one] Toegevoegd als bron van dit item, en gekoppeld in de tekst
   *[other] { $count } bestanden toegevoegd als bronnen van dit item, en gekoppeld in de tekst
}
resources-dropped-undo-kept = De tekst is sinds het neerzetten gewijzigd, dus er is niets ongedaan gemaakt.
resources-undo = Ongedaan maken
resources-link-refused = “{ $url }” is geen webadres dat Skribisto kan bewaren. Gebruik een adres dat begint met http:// of https://, zonder gebruikersnaam of wachtwoord.

## De melding voor een project in één bestand

resources-folder-title = Dit project wordt groot
resources-folder-text = De opgeslagen bestanden nemen nu { $size } in beslag. Een project dat als één bestand wordt bewaard, wordt bij elke opslag helemaal opnieuw geschreven, de bestanden ook. Als map bewaard slaat het alleen op wat er is veranderd. U kunt het nu als map opslaan, en vanaf dan in de map verder werken.
resources-folder-switch = Als map opslaan…
resources-folder-not-now = Niet nu
resources-folder-never = Niet meer vragen voor dit project

## Openen

resources-not-found-toast = “{ $name }” staat niet waar het project het verwacht.
resources-unsafe-title = “{ $name }” openen?
resources-unsafe-text = Skribisto weet niet of dit soort bestand veilig is. Het openen kan een programma op uw computer starten, en een project kan van iemand anders komen. Toon het eerst in de map om te zien wat het is.
resources-open-anyway = Toch openen
resources-open-failed = Het bestand kon niet worden geopend: { $error }
resources-no-data-folder = dit systeem geeft Skribisto geen gegevensmap om een kopie in te openen
resources-link-gone = Deze koppeling verwijst naar een bron die het project niet meer heeft.
resources-locate-title = Waar is “{ $name }” nu?
resources-relinked-more = { $count ->
    [one] Nog één ontbrekend bestand is in dezelfde map gevonden, en opnieuw gekoppeld.
   *[other] Nog { $count } ontbrekende bestanden zijn in dezelfde map gevonden, en opnieuw gekoppeld.
}
resources-changed-elsewhere = “{ $name }” is in een andere app gewijzigd. Het project houdt de nieuwe versie.

## Bewerken

resources-edit-failed = De bron kon niet worden gewijzigd: { $error }
resources-remove-title = “{ $title }” uit het project verwijderen?
resources-remove-text = De bron wordt losgemaakt van elk item waaraan ze hangt. Een bestand dat in het project is opgeslagen, verdwijnt mee; een gekoppeld bestand en een webpagina blijven waar ze zijn. Ongedaan maken brengt de bron terug.
resources-remove-confirm = Verwijderen
resources-remove-failed = De bron kon niet worden verwijderd: { $error }
resources-removed-toast = ‘{ $title }’ is uit het project verwijderd.
resources-removed-many-toast = { $count } bronnen zijn uit het project verwijderd.
resources-detached-toast = ‘{ $title }’ is losgekoppeld.
resources-attached-one-toast = ‘{ $title }’ toegevoegd aan de bronnen van ‘{ $item }’.
resources-attached-many-toast = { $count } bronnen toegevoegd aan ‘{ $item }’.

## In de tekst

resources-no-text = Plaats eerst de cursor in een tekst, en voeg de bron daar dan in.
resources-drop-insert-picture = De afbeelding invoegen
resources-drop-insert-link = Een koppeling naar de bron invoegen

## Voortgang

progress-resources-storing = De bestanden kopiëren
progress-resources-stored = Bestanden gekopieerd

## De pagina Bronnen van een map

segment-folder = Map
resources-folder-count = { $count ->
    [one] Eén bron
   *[other] { $count } bronnen
}
resources-view-label = Bronnen tonen als
resources-view-list = Lijst
resources-view-grid = Raster
resources-grid-label = Bronnen
resources-folder-drop-hint = Sleep bestanden of webadressen hierheen om ze aan deze map te hangen
resources-folder-empty = Aan deze map en aan wat erin zit hangt nog niets. Voeg bestanden of weblinks toe met de knoppen hierboven, of sleep ze hierheen: ze worden aan de map gehangen.
resources-attached-to = Aan
resources-attached-separator = ,
resources-attached-more = { $count ->
    [one] en nog één
   *[other] en nog { $count }
}
resources-row-name-attached = { $name }, aan { $items }
resources-open-item = “{ $title }” openen
resources-detach-from = Losmaken van “{ $title }”

## Afbeeldingen die bronnen zijn, in de Galerij

gallery-resource-badge = Bron
gallery-tile-resource = Afbeelding die een bron is van { $title }: { $description }

## De pagina Bronnen van de projectinstellingen

settings-page-resources = Bronnen
settings-desc-resources = De bestanden en weblinks die aan geen enkel item meer hangen: hang ze weer ergens aan, verwijder ze, of exporteer kopieën
settings-resources-description = Een bron hangt nergens meer aan zodra ze van haar laatste item is losgemaakt, of zodra het item waar ze aan hing definitief is verwijderd. Ze blijft in het project, en een opgeslagen bestand reist mee met elke kopie en elke back-up ervan, tot u haar weer ergens aan hangt of verwijdert.
settings-resources-group-storage = Opslag
settings-resources-group-unattached = Nergens aan gehangen
settings-resources-stored = Bestanden opgeslagen in dit project: { $size }
settings-resources-folder-advice = Een project dat als één bestand wordt bewaard, wordt bij elke opslag helemaal opnieuw geschreven, de opgeslagen bestanden ook. Als map bewaard slaat het alleen op wat er is veranderd, en dat gaat sneller.
settings-resources-count = { $count ->
    [one] Eén bron
   *[other] { $count } bronnen
}
settings-resources-empty = Elke bron van dit project hangt aan een item.
resources-attach-to-items = Aan items hangen…
resources-export-copies = Kopieën exporteren…
resources-export-copy = Een kopie exporteren…
resources-remove-many-title = { $count } bronnen uit het project verwijderen?
resources-attach-title = Aan items hangen
resources-attach-hint = Kies de items om aan te hangen. Ctrl+klik of Shift+klik kiest er meerdere.
resources-attach-empty = Dit project heeft nog geen items.
resources-attach-confirm = Aanhangen
resources-attached = { $count ->
    [one] Eén bron
   *[other] { $count } bronnen
} gehangen aan { $items ->
    [one] één item
   *[other] { $items } items
}
resources-export-title = Kopieën naar een map exporteren
resources-exported = { $count ->
    [one] Eén bron geëxporteerd
   *[other] { $count } bronnen geëxporteerd
}
resources-export-missing = { $count ->
    [one] Eén bestand staat niet waar het project het verwacht, en is weggelaten: { $names }
   *[other] { $count } bestanden staan niet waar het project ze verwacht, en zijn weggelaten: { $names }
}
resources-export-failed = { $count ->
    [one] Eén bestand kon niet worden gekopieerd: { $names }
   *[other] { $count } bestanden konden niet worden gekopieerd: { $names }
}

## De voorkeur voor grote bestanden (Editor ▸ Afbeeldingen en bestanden)

settings-resources-large-file = Bestanden boven 10 MB
settings-resources-large-ask = Elke keer vragen
settings-resources-large-store = Altijd in het project opslaan
settings-resources-large-link = Altijd aan het bestand koppelen
settings-hint-resources-large-file = Wat er gebeurt met een bestand boven 10 MB dat u als bron toevoegt. In het project opgeslagen reist het mee met elke kopie en elke back-up ervan. Gekoppeld blijft het waar het is, en onthoudt het project alleen waar het te vinden is.
