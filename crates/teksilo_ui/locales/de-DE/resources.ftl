# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Ressourcen: Dateien und Weblinks, die an Einträge der Projektmappe angehängt sind.

## Das Seitenfenster

resources-title = Ressourcen
shortcut-name-resources-show = Ressourcen anzeigen
resources-add-files = Dateien hinzufügen
resources-add-files-title = Diesem Eintrag Dateien hinzufügen
resources-add-link = Weblink hinzufügen
resources-drop-hint = Dateien oder Webadressen hier ablegen, um sie anzuhängen
resources-drop-hint-unattached = Dateien oder Webadressen hier ablegen, um sie ohne Eintrag hinzuzufügen
resources-no-item = Öffnen Sie einen Eintrag, um seine Ressourcen zu sehen.
resources-empty = An diesen Eintrag ist noch nichts angehängt. Fügen Sie mit den Schaltflächen oben Dateien oder Weblinks hinzu, oder legen Sie sie hier ab.
resources-dock-view-label = Anzuzeigende Ressourcen
resources-dock-view-item = Dieser Eintrag
resources-dock-view-unattached = Nicht angehängt
resources-dock-view-item-hint = Die Dateien und Links des Eintrags, an dem Sie sind. Was Sie hinzufügen, wird an ihn angehängt.
resources-dock-view-unattached-hint = Die Dateien und Links, die an keinem Eintrag hängen, etwa Recherche, die Sie noch nicht eingeordnet haben. Was Sie hier hinzufügen, hängt an nichts.
resources-unattached-empty = Jede Ressource dieses Projekts hängt an einem Eintrag. Hier hinzugefügte Dateien und Links hängen an nichts, bis Sie sie anhängen.
resources-actions = Aktionen
resources-not-found = Nicht gefunden
resources-row-linked = Verknüpfte Datei
resources-row-linked-size = Verknüpfte Datei, { $size }
resources-row-name = { $title }, { $kind }, { $detail }
resources-row-name-missing = { $title }, { $kind }, { $detail }, nicht gefunden

## Dateiarten, wie ein Bildschirmleser sie vorliest

resources-kind-picture = Bild
resources-kind-pdf = PDF-Dokument
resources-kind-audio = Tonaufnahme
resources-kind-video = Video
resources-kind-document = Dokument
resources-kind-ebook = E-Book
resources-kind-archive = Archiv
resources-kind-file = Datei
resources-kind-web-link = Weblink

## Das Menü einer Zeile

resources-open = Öffnen
resources-show-in-folder = Im Ordner anzeigen
resources-locate = Wiederfinden…
resources-insert-into-text = In den Text einfügen
resources-insert-picture = Das Bild in den Text einfügen
resources-insert-link = Einen Link in den Text einfügen
resources-edit = Umbenennen und beschreiben…
resources-detach = Von diesem Eintrag lösen
resources-remove = Aus dem Projekt entfernen…

## Die beiden Dialoge

resources-link-dialog-title = Weblink hinzufügen
resources-link-dialog-address = Adresse
resources-link-dialog-address-placeholder = https://
resources-link-dialog-name = Titel
resources-link-dialog-name-placeholder = Der Name der Website, wenn leer gelassen
resources-link-dialog-add = Hinzufügen
resources-edit-dialog-title = Umbenennen und beschreiben
resources-edit-dialog-name = Titel
resources-edit-dialog-description = Beschreibung
resources-edit-dialog-description-placeholder = Was das ist und warum Sie es aufbewahren
resources-edit-dialog-save = Speichern
resources-dialog-cancel = Abbrechen

## Hinzufügen

resources-large-title = Eine große Datei
resources-large-text = „{ $name }“ ist { $size } groß. Im Projekt abgelegt, reist sie mit jeder Kopie und jeder Sicherung des Projekts mit. Verknüpft, bleibt sie, wo sie ist, und das Projekt merkt sich nur, wo sie zu finden ist.
resources-large-text-many = „{ $name }“ ist { $size } groß, und { $others ->
    [one] eine weitere Datei ist
   *[other] { $others } weitere Dateien sind
} ebenfalls groß. Im Projekt abgelegt, reisen sie mit jeder Kopie und jeder Sicherung des Projekts mit. Verknüpft, bleiben sie, wo sie sind, und das Projekt merkt sich nur, wo sie zu finden sind.
resources-large-store = Im Projekt ablegen
resources-large-link = Stattdessen verknüpfen
resources-large-remember = Nicht mehr fragen
resources-storing-title = Dateien werden dem Projekt hinzugefügt
resources-store-cancel = Abbrechen
resources-store-cancelled = Es wurden keine Dateien hinzugefügt.
resources-store-busy = Es werden noch Dateien hinzugefügt. Versuchen Sie es erneut, wenn sie da sind.
resources-store-failed = Die Dateien konnten nicht hinzugefügt werden: { $error }
resources-no-project = Öffnen Sie ein Projekt, bevor Sie Dateien anhängen.
resources-moved-away = Das Projekt wurde verschoben, während die Datei kopiert wurde.
resources-some-failed = { $count ->
    [one] Eine Datei konnte nicht hinzugefügt werden: { $names }
   *[other] { $count } Dateien konnten nicht hinzugefügt werden: { $names }
}
resources-add-failed = Die Ressourcen konnten nicht hinzugefügt werden: { $error }
resources-added = { $count ->
    [one] Eine Ressource hinzugefügt
   *[other] { $count } Ressourcen hinzugefügt
}
resources-dropped-added = { $count ->
    [one] Als Ressource dieses Eintrags hinzugefügt und im Text verlinkt
   *[other] { $count } Dateien als Ressourcen dieses Eintrags hinzugefügt und im Text verlinkt
}
resources-dropped-undo-kept = Der Text hat sich seit dem Ablegen geändert, daher wurde nichts rückgängig gemacht.
resources-undo = Rückgängig
resources-link-refused = „{ $url }“ ist keine Webadresse, die Skribisto aufbewahren kann. Verwenden Sie eine Adresse, die mit http:// oder https:// beginnt, ohne Benutzernamen oder Passwort.

## Der Hinweis für ein Projekt in einer einzigen Datei

resources-folder-title = Dieses Projekt wird groß
resources-folder-text = Seine abgelegten Dateien belegen jetzt { $size }. Ein Projekt, das als einzelne Datei gespeichert ist, wird bei jedem Speichern vollständig neu geschrieben, mitsamt seinen Dateien. Als Ordner gespeichert, schreibt es nur, was sich geändert hat. Sie können es jetzt als Ordner speichern und von da an im Ordner weiterarbeiten.
resources-folder-switch = Als Ordner speichern…
resources-folder-not-now = Nicht jetzt
resources-folder-never = Für dieses Projekt nicht mehr fragen

## Öffnen

resources-not-found-toast = „{ $name }“ ist nicht dort, wo das Projekt die Datei erwartet.
resources-unsafe-title = „{ $name }“ öffnen?
resources-unsafe-text = Skribisto weiß nicht, ob diese Art von Datei sicher ist. Sie zu öffnen könnte ein Programm auf Ihrem Computer starten, und ein Projekt kann von jemand anderem stammen. Zeigen Sie sie zuerst in ihrem Ordner an, um zu sehen, was sie ist.
resources-open-anyway = Trotzdem öffnen
resources-open-failed = Die Datei konnte nicht geöffnet werden: { $error }
resources-no-data-folder = dieses System gibt Skribisto keinen Datenordner, in dem eine Kopie geöffnet werden könnte
resources-link-gone = Dieser Link verweist auf eine Ressource, die das Projekt nicht mehr hat.
resources-locate-title = Wo ist „{ $name }“ jetzt?
resources-relinked-more = { $count ->
    [one] Eine weitere fehlende Datei wurde im selben Ordner gefunden und wieder verknüpft.
   *[other] { $count } weitere fehlende Dateien wurden im selben Ordner gefunden und wieder verknüpft.
}
resources-changed-elsewhere = „{ $name }“ wurde in einer anderen App geändert. Das Projekt behält die neue Version.

## Bearbeiten

resources-edit-failed = Die Ressource konnte nicht geändert werden: { $error }
resources-remove-title = „{ $title }“ aus dem Projekt entfernen?
resources-remove-text = Sie wird von jedem Eintrag gelöst, an den sie angehängt ist. Eine im Projekt abgelegte Datei verschwindet mit ihr; eine verknüpfte Datei und eine Webseite bleiben, wo sie sind. Rückgängig holt sie zurück.
resources-remove-confirm = Entfernen
resources-remove-failed = Die Ressource konnte nicht entfernt werden: { $error }
resources-removed-toast = „{ $title }“ wurde aus dem Projekt entfernt.
resources-removed-many-toast = { $count } Ressourcen wurden aus dem Projekt entfernt.
resources-detached-toast = „{ $title }“ wurde gelöst.
resources-attached-one-toast = „{ $title }“ zu den Ressourcen von „{ $item }“ hinzugefügt.
resources-attached-many-toast = { $count } Ressourcen zu „{ $item }“ hinzugefügt.

## In den Text

resources-no-text = Setzen Sie zuerst die Schreibmarke in einen Text und fügen Sie die Ressource dann dort ein.
resources-drop-insert-picture = Das Bild einfügen
resources-drop-insert-link = Einen Link zur Ressource einfügen

## Fortschritt

progress-resources-storing = Dateien werden kopiert
progress-resources-stored = Dateien kopiert

## Die Ressourcen-Seite eines Ordners

segment-folder = Ordner
resources-folder-count = { $count ->
    [one] Eine Ressource
   *[other] { $count } Ressourcen
}
resources-view-label = Ressourcen anzeigen als
resources-view-list = Liste
resources-view-grid = Raster
resources-grid-label = Ressourcen
resources-folder-drop-hint = Dateien oder Webadressen hier ablegen, um sie an diesen Ordner anzuhängen
resources-folder-empty = An diesen Ordner und an seinen Inhalt ist noch nichts angehängt. Fügen Sie mit den Schaltflächen oben Dateien oder Weblinks hinzu, oder legen Sie sie hier ab: Sie werden an den Ordner angehängt.
resources-attached-to = An
resources-attached-separator = ,
resources-attached-more = { $count ->
    [one] und einem weiteren
   *[other] und { $count } weiteren
}
resources-row-name-attached = { $name }, an { $items }
resources-open-item = „{ $title }“ öffnen
resources-detach-from = Von „{ $title }“ lösen

## Bild-Ressourcen in der Galerie

gallery-resource-badge = Ressource
gallery-tile-resource = Bild-Ressource von { $title }: { $description }

## Die Ressourcen-Seite der Projekteinstellungen

settings-page-resources = Ressourcen
settings-desc-resources = Die Dateien und Weblinks, die an keinem Eintrag mehr hängen: wieder anhängen, entfernen oder Kopien exportieren
settings-resources-description = Eine Ressource hängt an nichts mehr, sobald sie von ihrem letzten Eintrag gelöst wird oder der Eintrag, an dem sie hing, endgültig gelöscht wird. Sie bleibt im Projekt, und eine abgelegte Datei reist mit jeder Kopie und jeder Sicherung des Projekts mit, bis Sie sie wieder anhängen oder entfernen.
settings-resources-group-storage = Speicherplatz
settings-resources-group-unattached = An nichts angehängt
settings-resources-stored = Im Projekt abgelegte Dateien: { $size }
settings-resources-folder-advice = Ein Projekt, das als einzelne Datei gespeichert ist, wird bei jedem Speichern vollständig neu geschrieben, mitsamt seinen abgelegten Dateien. Als Ordner gespeichert, schreibt es nur, was sich geändert hat, und das geht schneller.
settings-resources-count = { $count ->
    [one] Eine Ressource
   *[other] { $count } Ressourcen
}
settings-resources-empty = Jede Ressource dieses Projekts hängt an einem Eintrag.
resources-attach-to-items = An Einträge anhängen…
resources-export-copies = Kopien exportieren…
resources-export-copy = Eine Kopie exportieren…
resources-remove-many-title = { $count } Ressourcen aus dem Projekt entfernen?
resources-attach-title = An Einträge anhängen
resources-attach-hint = Wählen Sie die Einträge, an die angehängt wird. Mit Strg+Klick oder Umschalt+Klick wählen Sie mehrere.
resources-attach-empty = Dieses Projekt hat noch keine Einträge.
resources-attach-confirm = Anhängen
resources-attached = { $count ->
    [one] Eine Ressource
   *[other] { $count } Ressourcen
} an { $items ->
    [one] einen Eintrag
   *[other] { $items } Einträge
} angehängt
resources-export-title = Kopien in einen Ordner exportieren
resources-exported = { $count ->
    [one] Eine Ressource exportiert
   *[other] { $count } Ressourcen exportiert
}
resources-export-missing = { $count ->
    [one] Eine Datei ist nicht dort, wo das Projekt sie erwartet, und wurde ausgelassen: { $names }
   *[other] { $count } Dateien sind nicht dort, wo das Projekt sie erwartet, und wurden ausgelassen: { $names }
}
resources-export-failed = { $count ->
    [one] Eine Datei konnte nicht kopiert werden: { $names }
   *[other] { $count } Dateien konnten nicht kopiert werden: { $names }
}

## Die Einstellung für große Dateien (Editor ▸ Bilder und Dateien)

settings-resources-large-file = Dateien über 10 MB
settings-resources-large-ask = Jedes Mal fragen
settings-resources-large-store = Immer im Projekt ablegen
settings-resources-large-link = Immer mit der Datei verknüpfen
settings-hint-resources-large-file = Was mit einer Datei über 10 MB geschieht, die Sie als Ressource anhängen. Im Projekt abgelegt, reist sie mit jeder Kopie und jeder Sicherung des Projekts mit. Verknüpft, bleibt sie, wo sie ist, und das Projekt merkt sich nur, wo sie zu finden ist.
