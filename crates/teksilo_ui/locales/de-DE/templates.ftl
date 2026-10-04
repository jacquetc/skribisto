# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Vorlagen werden beim Anwenden zu frei bearbeitbaren Projektdaten.
settings-page-templates = Vorlagen
settings-templates-description = Wiederverwendbare Textbausteine, die Sie überall beim Schreiben einfügen können: ein leeres Figurenblatt, ein Ortsprofil oder ein Szenenplan. Sie werden im Projekt gespeichert und stehen allen zur Verfügung, die es öffnen.
settings-templates-filter = Vorlagen filtern
settings-templates-count = { $n ->
    [0] Keine Vorlagen
    [one] 1 Vorlage
   *[other] { $n } Vorlagen
}
settings-templates-empty = Dieses Projekt hat noch keine Vorlagen.
settings-templates-empty-hint = Fügen Sie eine Vorgabe hinzu, importieren Sie eine .md- oder .djot-Datei oder schreiben Sie einen Text und wählen Sie Dokument ▸ Als Vorlage speichern.
settings-templates-presets = Vorgabe hinzufügen
settings-templates-import = Importieren…
settings-templates-export = Exportieren…
settings-templates-star = Im Einfügemenü zuerst anzeigen
settings-templates-unstar = Nicht mehr zuerst anzeigen
settings-templates-move-up = Nach oben verschieben
settings-templates-move-down = Nach unten verschieben
settings-templates-delete = Vorlage löschen
settings-templates-name = Name
settings-templates-name-placeholder = Vorlagenname
settings-templates-body-placeholder = Der von dieser Vorlage eingefügte Text
settings-templates-duplicate-name = Eine andere Vorlage heißt bereits „{ $name }“
settings-templates-words = { $n ->
    [one] 1 Wort
   *[other] { $n } Wörter
}
templates-imported = { $added ->
    [one] 1 Vorlage importiert
   *[other] { $added } Vorlagen importiert
}
templates-imported-renamed = Zur Vermeidung eines Namenskonflikts umbenannt: { $names }
templates-imported-skipped = Konnte nicht gelesen werden: { $names }
templates-import-filter = Vorlagen (.md, .djot)
templates-exported = { $n ->
    [one] 1 Vorlage exportiert
   *[other] { $n } Vorlagen exportiert
}
templates-preset-applied = „{ $name }“ hinzugefügt
templates-delete-title = Diese Vorlage löschen?
templates-delete-body = „{ $name }“ wird aus diesem Projekt entfernt. Bereits daraus erstellte Notizen sind nicht betroffen.
templates-delete-confirm = Löschen
menu-document = &Dokument
menu-insert-template = Vor&lage einfügen
menu-insert-template-none = Keine Vorlagen in diesem Projekt
menu-save-as-template = Als &Vorlage speichern…
save-as-template-title = Als Vorlage speichern
save-as-template-explain = Der gerade bearbeitete Text wird zu einer Vorlage, die Sie überall einfügen können.
save-as-template-name = Name
save-as-template-placeholder = Figurenblatt
save-as-template-duplicate = Eine Vorlage namens „{ $name }“ existiert bereits
save-as-template-empty-editor = Dieser Editor ist leer. Es gibt nichts zu speichern.
save-as-template-confirm = Vorlage speichern
save-as-template-saved = „{ $name }“ als Vorlage gespeichert
template-inserted = „{ $name }“ eingefügt
note-template-preset-character-sheet = Figurenblatt
note-template-preset-location = Ort
note-template-preset-artifact = Gegenstand
note-template-preset-beat-sheet = Szenenplan
note-template-preset-faction = Fraktion
note-template-preset-research-note = Recherchenotiz
note-template-set-essentials = Grundlagen
note-template-set-everything = Alle Vorlagen
note-template-section-identity = Identität
note-template-section-appearance = Aussehen
note-template-section-voice = Stimme
note-template-section-psychology = Innenleben
note-template-section-history = Vorgeschichte
note-template-section-arc = Entwicklung
note-template-section-first-impression = Erster Eindruck
note-template-section-in-the-story = In der Geschichte
note-template-section-the-thing-itself = Der Gegenstand selbst
note-template-section-the-scene = Die Szene
note-template-section-the-shape = Der Aufbau
note-template-section-what-it-is = Was es ist
note-template-section-source = Quelle
note-template-section-what-it-says = Inhalt
note-template-field-full-name = Vollständiger Name
note-template-field-known-as = Weitere Namen, erfasst im Feld „Auch bekannt als“ des Inspektors
note-template-field-age = Alter
note-template-field-role-in-story = Rolle in der Geschichte
note-template-field-build-and-features = Körperbau und besondere Merkmale
note-template-field-habitual-bearing = Typische Haltung
note-template-field-speech-patterns = Sprechweise und sprachliche Eigenheiten
note-template-field-what-they-never-say = Was die Figur niemals sagt
note-template-field-want = Was die Figur will
note-template-field-need = Was die Figur braucht, ohne es zu wissen
note-template-field-fear = Wovor die Figur Angst hat
note-template-field-flaw = Die Schwäche, die ihren Preis fordert
note-template-field-formative-event = Prägendes Ereignis
note-template-field-relationships = Beziehungen
note-template-field-starts-as = Am Anfang
note-template-field-ends-as = Am Ende
note-template-field-what-you-notice-first = Was zuerst auffällt
note-template-field-sound-and-smell = Geräusche und Gerüche
note-template-field-light-and-weather = Licht und Wetter
note-template-field-what-happened-here = Was hier geschehen ist
note-template-field-who-lives-or-works-here = Wer hier lebt oder arbeitet
note-template-field-scenes-set-here = Szenen an diesem Ort
note-template-field-why-it-matters = Warum es wichtig ist
note-template-field-appearance = Aussehen
note-template-field-age-and-origin = Alter und Herkunft
note-template-field-what-it-does = Seine Funktion
note-template-field-who-holds-it = Wer es besitzt
note-template-field-who-wants-it = Wer es haben will
note-template-field-pov = Erzählperspektive
note-template-field-time-and-place = Zeit und Ort
note-template-field-goal = Ziel
note-template-field-conflict = Konflikt
note-template-field-turn = Die Wendung
note-template-field-exit-emotion = Gefühl am Ende
note-template-field-purpose = Zweck
note-template-field-who-leads-it = Wer die Führung hat
note-template-field-resources = Ressourcen
note-template-field-allies-and-enemies = Verbündete und Feinde
note-template-field-what-it-wants-now = Aktuelle Absichten
note-template-field-where-from = Herkunft
note-template-field-page-or-link = Seite oder Link
note-template-field-key-facts = Wichtige Fakten
note-template-field-how-it-is-used = Verwendung in der Geschichte
