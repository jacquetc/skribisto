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
resources-no-item = Open een item om de bronnen ervan te zien.
resources-empty = Aan dit item hangt nog niets. Voeg bestanden of weblinks toe met de knoppen hierboven, of sleep ze hierheen.
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

## In de tekst

resources-no-text = Plaats eerst de cursor in een tekst, en voeg de bron daar dan in.
resources-drop-insert-picture = De afbeelding invoegen
resources-drop-insert-link = Een koppeling naar de bron invoegen

## Voortgang

progress-resources-storing = De bestanden kopiëren
progress-resources-stored = Bestanden gekopieerd
