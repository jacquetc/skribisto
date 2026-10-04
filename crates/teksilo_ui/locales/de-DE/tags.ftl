# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Namen aus Vorgaben werden beim Anwenden als bearbeitbare Projektdaten gespeichert.
tags-preset-basic = Grundlagen
tags-preset-scifi = Science-Fiction
tags-preset-fantasy = Fantasy
tags-preset-mystery = Krimi
tags-preset-historical = Historisch
tags-preset-needs-research = Recherche nötig
tags-preset-continuity-check = Kontinuität prüfen
tags-preset-plot-point = Wendepunkt
tags-preset-character = Figur
tags-preset-place = Ort
tags-preset-item = Gegenstand
tags-preset-vessel = Raumschiff
tags-preset-planet = Planet
tags-preset-organization = Organisation
tags-preset-creature = Wesen
tags-preset-faction = Fraktion
tags-preset-artifact = Artefakt
tags-preset-realm = Reich
tags-preset-magic-system = Magiesystem
tags-preset-suspect = Verdächtiger
tags-preset-victim = Opfer
tags-preset-clue = Hinweis
tags-preset-red-herring = falsche Fährte
tags-preset-historical-figure = historische Person
tags-preset-source = Quelle
tags-preset-period-detail = zeitgeschichtliches Detail
tags-pill-list = Schlagwörter
tags-pill-add = Schlagwort hinzufügen
tags-none-yet = Keine Schlagwörter
tags-pick-list = Verfügbare Schlagwörter
tags-pill-remove = { $name } entfernen
tags-pill-filter-placeholder = Filtern oder neues Schlagwort benennen
tags-pill-no-match = Kein passendes Schlagwort
tags-pill-create = „{ $name }“ erstellen
tags-pill-create-failed = „{ $name }“ konnte nicht erstellt werden
tags-pill-new-discoverable = Schlagwort für Textsuche
tags-pill-new-discoverable-hint = Einträge mit diesem Schlagwort werden im Text gesucht, um die Beteiligtenliste zu füllen.
tags-alias-list = Auch bekannt als
tags-alias-add = Weiteren Namen hinzufügen
tags-alias-remove = { $name } entfernen
tags-alias-placeholder = Weiterer Name, dann Eingabe
tags-alias-hint = Namen, unter denen dieser Eintrag neben seinem Titel im Text vorkommt.
tags-alias-collision = { $name } verwendet diesen Namen bereits.
settings-page-tags = Schlagwörter
settings-tags-desc = Schlagwörter kennzeichnen Einträge in Ihrer Projektmappe. Ein Schlagwort für Textsuche veranlasst Skribisto zusätzlich, im Text nach den Namen dieses Eintrags zu suchen.
settings-tags-add = Schlagwort hinzufügen
settings-tags-add-placeholder = Neues Schlagwort benennen
settings-tags-added = „{ $name }“ hinzugefügt
settings-tags-duplicate = „{ $name }“ existiert bereits
settings-tags-filter = Schlagwörter filtern
settings-tags-count = { $n ->
    [one] 1 Schlagwort
   *[other] { $n } Schlagwörter
}
settings-tags-details-placeholder = Bedeutung dieses Schlagworts
settings-tags-discoverable = Im Text suchen
settings-tags-creates-in = Neue Notizen ablegen in
settings-tags-creates-in-unset = Beim ersten Mal fragen
settings-tags-creates-in-untitled = Unbenannter Ordner
settings-tags-creates-in-trashed = { $name } (im Papierkorb)
settings-tags-template = Ausgangsvorlage
settings-tags-template-unset = Leere Notiz
settings-tags-delete = { $name } löschen
settings-tags-deleted = „{ $name }“ gelöscht und von allen Einträgen entfernt
settings-tags-empty = Noch keine Schlagwörter.
settings-tags-apply-preset = Vorgabe anwenden…
settings-tags-preset-applied = { $added } hinzugefügt, { $skipped } bereits vorhandene übersprungen
settings-tags-csv-filter = CSV-Dateien
settings-tags-import = Importieren…
settings-tags-export = Exportieren…
settings-tags-imported = { $added } importiert, { $skipped } übersprungen
settings-tags-exported = { $n ->
    [one] 1 Schlagwort exportiert
   *[other] { $n } Schlagwörter exportiert
}
tags-chip-more = { $n ->
    [one] 1 weiteres Schlagwort
   *[other] { $n } weitere Schlagwörter
}
cast-section = Beteiligte
cast-add = Beteiligte hinzufügen…
cast-add-filter-placeholder = Geschichtenbibel filtern…
cast-add-empty = Keine Einträge aus der Geschichtenbibel zum Hinzufügen
cast-pin = { $name } zu den Beteiligten hinzufügen
cast-unpin = { $name } aus den Beteiligten entfernen
cast-empty = Noch niemand zugeordnet. Fügen Sie jemanden hinzu oder übernehmen Sie einen Vorschlag
cast-unresolved = Nicht mehr in der Geschichtenbibel
mentions-confirm = Zu den Beteiligten von { $name } hinzufügen
mentions-hit-count = { $n ->
    [one] einmal
   *[other] { $n }-mal
}
mentions-point-of-view-badge = Erzählperspektive
mentions-point-of-view-badge-tooltip = Hier von Hand als Erzählperspektive festgelegt, nicht im Text erkannt.
mentions-roster = Beteiligte
mentions-pin = { $name } zu den Beteiligten hinzufügen
pov-section = Erzählperspektive
pov-empty = Keine Erzählperspektive festgelegt
pov-add = Erzählperspektive festlegen…
pov-multiple = Diese Szene hat zwei Erzählperspektiven.
pov-remove = { $name } als Erzählperspektive entfernen
pov-unresolved = Hier war eine Erzählperspektive zugeordnet, aber ihr Eintrag existiert nicht mehr oder hat sein Schlagwort für die Geschichtenbibel verloren.
books-section = Zugeordnet zu
books-empty = Noch keinem Buch zugeordnet
books-add = Einem Buch zuordnen…
books-remove = Aus { $name } entfernen
books-apply-to-children = Zuordnung auf Untereinträge anwenden
note-details-name-placeholder = Name…
note-details-tags = Schlagwörter
note-details-aliases = Weitere Namen
note-details-books = Zugeordnet zu
note-details-books-empty = Noch keinem Buch zugeordnet
note-details-links = Verknüpfungen
note-details-links-empty = Noch keine Verknüpfungen. Fügen Sie eine hinzu, um auf einen anderen Eintrag Ihrer Geschichtenbibel zu verweisen.
note-details-links-add = Mit einer Notiz verknüpfen…
note-details-pov = Erzählperspektive
note-details-pov-empty = Keine Erzählperspektive festgelegt
note-details-pov-unresolved = Hier war eine Erzählperspektive zugeordnet, aber ihr Eintrag existiert nicht mehr oder hat sein Schlagwort für die Geschichtenbibel verloren.
note-details-pov-multiple = Diese Notiz hat zwei Erzählperspektiven.
note-details-backlinks = Kommt im Manuskript vor
note-details-backlinks-empty = Noch nichts. Sobald dieser Name in Ihrem Text vorkommt, erscheint die Stelle hier.
note-details-backlinks-outside = Außerhalb der Bücher
note-details-backlinks-confirm-all = Alle Auftritte bestätigen
note-details-backlinks-confirm-all-tooltip = Diesen Eintrag zu den Beteiligten aller hier angezeigten Dokumente hinzufügen, in denen er noch nicht bestätigt ist
note-details-untitled-document = Ohne Titel
note-in-prose-mentions = { $n ->
    [one] 1 Erwähnung
   *[other] { $n } Erwähnungen
}
note-in-prose-mention-at = { $current } von { $total }
note-in-prose-mention-previous = Vorherige Erwähnung
note-in-prose-mention-next = Nächste Erwähnung
