# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Verlinkte Begriffserklärungen für Kurzinfos und Hilfe.
wm-book = Ein Buch in Ihrem Projekt.
wm-book-more = Ein Buch kann so einfach oder so vielschichtig sein, wie Sie möchten. Eine Kurzgeschichte besteht vielleicht nur aus wenigen [Szenen](:wm-scene). Ein längeres Werk kann [Teile](:wm-part) und [Kapitel](:wm-chapter) beliebig tief verschachteln. Sie bestimmen die Form.
wm-part = Ein Teil, der Kapitel zusammenfasst.
wm-part-more = Ein Teil fasst [Kapitel](:wm-chapter) unter einer Überschrift zusammen. Wie ein Kapitel reicht er bis zum nächsten Teil oder bis zum Ende des [Buchs](:wm-book).
wm-chapter = Ein Kapitel des Buchs.
wm-chapter-more = Ein Kapitel reicht von hier bis zum nächsten Kapitel, zum nächsten [Teil](:wm-part) oder zum [Buchende](:wm-end-of-book). Es gibt zwei Formen: einen Kapitelordner mit [Szenen](:wm-scene) oder ein Einzelkapitel mit eigenem Text. Unter Einstellungen ▸ Struktur wählen Sie die Form neuer Kapitel.
wm-scene = Eine Szene mit Erzähltext.
wm-scene-more = Eine Szene enthält Ihren Text. Jeder Schreibeintrag hat zwei Seiten: den Haupttext und eine [Zusammenfassung](:wm-synopsis) zum Planen. Beide öffnen sich gemeinsam im Editor.
wm-note = Eine Notiz.
wm-note-more = Eine Notiz enthält wie eine [Szene](:wm-scene) Text und eine [Zusammenfassung](:wm-synopsis), etwa für Recherche, Anmerkungen oder Erinnerungen. Jeder Eintrag kann Teil des zusammengestellten Buchs sein, auch eine Notiz innerhalb eines [Kapitels](:wm-chapter).
wm-note-folder = Eine Notiz, die weitere Einträge enthält.
wm-note-folder-more = Eine [Notiz](:wm-note), die zusätzlich Untereinträge enthält, um zusammengehörige Notizen zu gruppieren. Sie behält ihre eigene [Zusammenfassung](:wm-synopsis); die enthaltenen Einträge behalten ihren eigenen Text.
wm-folder = Ein Ordner zur Organisation.
wm-folder-more = Ein Sammeleintrag, um die Einträge der Projektmappe nach Ihren Wünschen anzuordnen. Er hat eine [Zusammenfassung](:wm-synopsis), aber keinen eigenen Erzähltext. Der Text liegt in seinen Untereinträgen.
wm-end-of-book = Markiert das Ende eines Buchs.
wm-end-of-book-more = Alle Bücher teilen sich eine fortlaufende Liste; das Ende eines Buchs ergibt sich daher nicht aus der Verschachtelung. Diese Markierung sagt: Das [Buch](:wm-book) endet hier. Alles danach gehört zum nächsten Buch.
wm-story-bible-entry = Eine Notiz zur Geschichte mit Schlagwörtern und Alternativnamen.
wm-story-bible-entry-more = Eine [Notiz](:wm-note) zu einer Figur, einem Ort oder etwas anderem, das Sie festhalten möchten, in einem Schritt mit Name, Schlagwörtern, Alternativnamen und Ausgangsvorlage erstellt. Danach unterscheidet sie sich auf der Seite nicht von anderen Notizen: Sie können sie wie jede andere umbenennen, neu verschlagworten oder umwandeln.
wm-synopsis = Die Zusammenfassung eines Schreibeintrags.
wm-synopsis-more = Eine Zusammenfassung, die zu jedem Schreibeintrag gehören kann: meist ein kurzer Absatz, aber beliebig lang. Sie hilft beim Planen und Navigieren, bevor der eigentliche Text existiert. Im Editor steht sie neben dem Haupttext.
scene-break-minor = Ein gewöhnlicher Szenentrenner: ein Wechsel von Zeit, Ort oder Perspektive innerhalb eines Kapitels.
scene-break-minor-more =
    Markiert eine Unterbrechung *genau dort, wo Sie sie setzen*, auch mitten in einer [Szene](:wm-scene). Den Text in zwei Einträge aufzuteilen dient der Organisation und erzeugt für sich allein keinen Trenner.

    Eingegeben als drei durch Leerzeichen getrennte Sternchen. Die Druckdarstellung bestimmt der Exportstil, nicht Ihre Eingabe: Ein Shunn-Manuskript verwendet ein einzelnes #, ein Taschenbuch etwa eine Sternchengruppe; in vielen französischen, deutschen, spanischen, russischen und italienischen Publikationen erscheint nur ein Abstand ohne Zeichen.
scene-break-major = Eine stärkere Unterteilung: ein großer Zeitsprung oder ein entscheidender Perspektivwechsel.
scene-break-major-more =
    Dasselbe Prinzip wie beim gewöhnlichen Szenentrenner, nur eine Stufe stärker. Verwenden Sie ihn, wenn ein einfacher Trenner den Sprung nicht deutlich genug macht.

    Eingegeben als # # # und vom Exportstil anders dargestellt als die gewöhnliche Stufe. Shunns Standardmanuskriptformat unterscheidet genau dafür zwischen einem einzelnen # und # # #. Wo eine Tradition keine stärkere Markierung kennt, erscheinen beide Stufen gleich.
wm-find-in-prose = Skribisto sucht in Ihrem Text nach diesem Eintrag.
wm-find-in-prose-more = Aktivieren Sie dies für Schlagwörter, die Dinge in Ihrer Geschichte benennen: Figuren, Orte, Gegenstände. Bei jedem so verschlagworteten Eintrag werden Titel und weitere Namen im Text gesucht. So führt jede [Szene](:wm-scene) automatisch auf, wer und was darin vorkommt. Lassen Sie es für beschreibende Schlagwörter wie Entwurfsstand oder Kontinuitätsprüfung aus. Alle gefundenen Einträge gehören auch zu Ihrer [Geschichtenbibel](:wm-story-bible).
wm-story-bible = Alle Figuren, Orte und weiteren verschlagworteten Einträge in einem Raster.
wm-story-bible-more = Öffnen Sie sie im Tab eines beliebigen Notizordners. Die Karten sind nach Schlagwort gruppiert und zeigen die Anzahl der Alternativnamen und der Szenen mit Auftritten. Grundlage sind alle Schlagwörter mit [Textsuche](:wm-find-in-prose) im Projekt. Der Buchfilter schränkt nur die angezeigten Karten ein; er bestimmt nicht, welche Einträge existieren.
tooltip-go-to = Zu einem beliebigen Eintrag der Projektmappe springen (Strg+G)
synopsis-collapse-tooltip = Zusammenfassungsspalte ausblenden
wm-paratext = Ein Text außerhalb der eigentlichen Erzählung.
wm-paratext-more = Vorwort, Widmung, Nachwort, Kolophon: Texte, die zum Buch gehören, aber nicht zur Erzählung. Sie werden an ihrer jeweiligen Position exportiert und nie zur Wortzahl des Manuskripts gezählt. Ihre Platzierung bestimmen Sie: Die Konventionen unterscheiden sich je nach Land und Verlag.
wm-paratext-folder = Ein Ordner für Paratexte.
wm-paratext-folder-more = Ein Ort für Vor- und Nachworte, damit die Projektmappe übersichtlich bleibt. Er dient nur der Organisation: Er hat eine [Zusammenfassung](:wm-synopsis), trägt aber nichts zum exportierten Buch bei, nicht einmal seinen eigenen Namen.
goal-target = Der geplante Umfang dieses Textstücks.
goal-target-more = Sie können ein Ziel für alles festlegen: eine [Szene](:wm-scene), ein [Kapitel](:wm-chapter), einen [Teil](:wm-part) oder das ganze [Buch](:wm-book). Jedes Ziel ist eigenständig. Ein Kapitelziel ändert nie das Buchziel, und das Buchziel ist nie die Summe seiner Kapitelziele. Addiert wird der geschriebene Text: Der [Fortschritt](:goal-progress) eines Sammeleintrags umfasst alles darunter Geschriebene. Null bedeutet: kein Ziel gesetzt. Die Einheit sind [Wörter oder Zeichen](:goal-unit), je nach Projekteinstellung.
goal-unit = Ob dieses Projekt in Wörtern oder Zeichen zählt.
goal-unit-more = Eine projektweite Wahl, denn die Längeneinheit gehört zum Manuskript und seinem Markt, nicht zu einem einzelnen Abschnitt: Deutsche und französische Verlage messen in Zeichen, japanische in 400-Zeichen-Seiten, chinesische in Tausend Zeichen, die meisten englischsprachigen in Wörtern. Beide Werte werden parallel geführt und nie ineinander umgerechnet. Ein Einheitenwechsel richtet jedes [Ziel](:goal-target) auf den anderen Wert; Zurückwechseln stellt den ursprünglichen Bezug wieder her. Vorhandene Zielzahlen werden dabei nicht umgerechnet und müssen von Hand angepasst werden.
goal-progress = Der bereits geschriebene Umfang im Verhältnis zum Ziel.
goal-progress-explained = Ein Eintrag mit eigenem Text wird anhand seiner eigenen Länge gemessen, ein Sammeleintrag anhand [aller darunter geschriebenen Texte](:goal-manuscript-words). Der Balken wechselt mit wachsendem Text von Rot über Gelb zu Grün und ändert sich erneut, wenn der geplante Umfang deutlich überschritten wird. So fällt auf, wenn ein Kapitel seine vorgesehene Länge überschreitet.
goal-manuscript-words = Was zum Manuskript zählt.
goal-manuscript-words-more = Nur tatsächlich exportierter Erzähltext: [Szenen](:wm-scene) und [Kapitel](:wm-chapter) mit eigenem Text. Eine [Notiz](:wm-note) oder ein [Paratext](:wm-paratext) gehört nicht zum Manuskript und wird nie dafür gezählt. Einträge im Papierkorb und [vom Export ausgeschlossene Einträge](:goal-exportable) zählen ebenfalls nicht mit.
goal-exportable = Dieser Eintrag ist vom Export ausgeschlossen.
goal-exportable-more = Seine eigene Länge bleibt sichtbar, denn der Text ist noch vorhanden. Er zählt aber zu keiner Gesamtsumme und erscheint in keinem exportierten Buch. Das Ausschließen eines Sammeleintrags schließt seine Untereinträge nicht aus: Jeder Eintrag entscheidet für sich. Dafür gibt es neben dem Schalter „Auf Untereinträge anwenden“.
goal-distribute = Dieses Ziel auf die enthaltenen Einträge verteilen.
goal-distribute-more = Verteilt das Ziel eines [Buchs](:wm-book) oder [Teils](:wm-part) auf seine direkten Untereinträge, sodass deren Summe genau dem Ziel entspricht. Standardmäßig werden nur Einträge ohne Ziel gefüllt; Ihre eigenen Zahlen bleiben erhalten. Es entstehen gewöhnliche [Ziele](:goal-target), die Sie anschließend frei ändern können. Dies ist eine einmalige Aktion, keine dauerhafte Kopplung.
goal-subtree-total = Die Summe der enthaltenen Ziele.
goal-subtree-total-more = Eine Information, kein Ziel. Das Projekt setzt das Ziel eines Sammeleintrags nie mit der Summe seiner Unterziele gleich; sonst würde sich die Ordnerzahl von selbst ändern, sobald eine Szene ein Ziel erhält. Diese Zeile zeigt lediglich die Summe Ihrer Zahlen, damit Sie sie selbst mit dem [Ziel](:goal-target) des Sammeleintrags vergleichen oder ein Ziel [verteilen](:goal-distribute) können.
goal-milestone = Ein Datum, bis zu dem etwas erreicht werden soll.
goal-milestone-more = Es gibt zwei Arten. Die erste legt fest, welchen Umfang das [Buch](:wm-book) bis zu einem Datum erreichen soll: ein Zwischenpunkt seiner Entwicklung. Die zweite legt fest, wann ein bestimmtes [Kapitel](:wm-chapter) oder ein [Teil](:wm-part) [sein eigenes Ziel](:goal-target) erreicht haben soll. Sie liest den Zielwert laufend, statt eine Kopie zu speichern; Zieländerungen aktualisieren daher auch den Meilenstein. Beide erscheinen im [Schreibplan](:pace-plan) des Buchs.
pace-plan = Der Zeitplan für das Buch.
pace-plan-more = Aus [Ziel](:goal-target), Abgabetermin und Ihren Schreibtagen ergibt sich das erforderliche Tagespensum. Der Plan verwendet das eigene Buchziel: Hier und im Inspektor steht dieselbe Zahl an zwei Stellen. Zwischenziele heißen [Meilensteine](:goal-milestone).
concept-tag = Ein farbiges Schlagwort, das Sie jedem Eintrag zuweisen und im ganzen Projekt wiederverwenden können.
concept-tag-more = Markieren Sie einen Eintrag mit einem Schlagwort, etwa für einen Arbeitsstand, einen Ort oder einen Handlungsstrang, den Sie verfolgen. Dasselbe Schlagwort kann beliebig viele Einträge markieren, und ein Eintrag kann mehrere haben. Aktivieren Sie [Im Text suchen](:wm-find-in-prose) für ein Schlagwort, das etwas im Buch benennt, etwa eine Figur oder einen Ort. Skribisto sucht dann danach im Text.
concept-label = Eine kurze Anmerkung unter dem Titel eines Eintrags, nur für Sie.
concept-label-more = Kein [Schlagwort](:concept-tag): Eine Kennzeichnung gehört nur zu einem Eintrag, als einfacher Text ohne Farbe oder Katalog, etwa „1. Wendepunkt“ oder „Name fehlt“. Legen Sie sie im Kontextmenü fest oder bearbeiten Sie sie direkt in der Spalte „Kennzeichnung“ der Übersicht. Sie erscheint als kleiner Untertitel sowohl in der Gliederung als auch in der fortlaufenden Ansicht.
concept-point-of-view = Durch wessen Augen eine Szene erzählt wird: eine oder mehrere beteiligte Figuren.
concept-point-of-view-more = Im Inspektor neben den Beteiligten festgelegt. Wer noch nicht zu den Beteiligten gehört, wird beim Auswählen hinzugefügt. Mehrere Perspektiven sind erlaubt, etwa für eine gemeinsame Szene oder einen Perspektivwechsel; Skribisto blockiert das nicht. Zur Auswahl stehen Einträge, die durch ein Schlagwort mit [Textsuche](:wm-find-in-prose) zur Geschichtenbibel gehören.
concept-epigraph = Ein Zitat am Anfang eines Teils oder Kapitels.
concept-epigraph-more = Ein eigenes Feld, getrennt vom Text des [Kapitels](:wm-chapter) oder [Teils](:wm-part). Eine [Szene](:wm-scene) oder [Notiz](:wm-note) hat keines. Schreiben Sie ein gewöhnliches Zitat; bei mehreren wird jedes zu einem eigenen Blockzitat und als eigenes Motto ausgegeben. Seine Wörter zählen nie zum Manuskript.
concept-footnote = Ein Verweis im Text, der als nummerierte Anmerkung ausgegeben wird.
concept-footnote-more = Der Verweis ist eine einzelne Markierung in Ihrem Text und wird mit den umliegenden Wörtern verschoben oder gelöscht. Seine Nummer wird nie gespeichert: Skribisto berechnet sie aus der Position aller Verweise im gesamten Manuskript neu. Das Einfügen einer Fußnote nummeriert daher alle folgenden um. Fußnotentext wird getrennt vom Manuskript gezählt und nie eingerechnet.
concept-chapter-mode = Ob ein neues Kapitel ein Szenenordner oder ein einzelner Eintrag ist.
concept-chapter-mode-more = Unter Einstellungen ▸ Struktur festgelegt. Sie schreiben in beiden Fällen direkt in ein [Kapitel](:wm-chapter); der einzige Unterschied ist, ob es zusätzlich Szenen enthalten kann. Beide Formen ergeben beim Export dasselbe Buch und lassen sich frei kombinieren. „Umwandeln in“ wechselt die Form vorhandener Kapitel ohne Wortverlust.
concept-comment = Eine Anmerkung zu einer bestimmten Textstelle, angezeigt am Rand.
concept-comment-more = Sie merkt sich die Bezugswörter und ihre Umgebung statt einer festen Position. So übersteht sie gewöhnliche Bearbeitungen und erneutes Laden. Antworten darunter führen den Gesprächsstrang fort. Wird das genaue Zitat nicht mehr gefunden, zeigt der Kommentar dies an, statt unbemerkt zum falschen Satz zu wandern.
concept-backup = Eine Kopie des gesamten Projekts zu einem bestimmten Zeitpunkt, separat aufbewahrt.
concept-backup-more = Wird nach Ihrem Zeitplan erstellt: beim Öffnen, Schließen, in Intervallen oder kombiniert. Alte Kopien werden nach Alter oder Anzahl bereinigt. Eine Sicherung öffnet sich in einem eigenen Fenster. Sie können sie ansehen und bearbeiten, Änderungen aber nur mit „Speichern unter“ oder „Wiederherstellen“ behalten. Anders als eine [Version](:concept-version), die die Vergangenheit eines einzelnen Eintrags abbildet.
concept-version = Die Vergangenheit eines Eintrags: jede Änderung an Text oder Zusammenfassung.
concept-version-more = Entsteht gemeinsam aus projektinternem Speicherverlauf und Sicherungen, mit einem Eintrag je Änderung statt je Datei. Wählen Sie eine Version, um Änderungen zu sehen und nur den Text dieses Eintrags wiederherzustellen. Der Rest des Projekts bleibt unberührt; zuvor wird eine Sicherheitskopie erstellt. Anders als eine [Sicherung](:concept-backup), die alles kopiert.
concept-trash = Ein Eintrag im Papierkorb ist ausgeblendet, nicht verschoben: Er bleibt genau an seinem Ort.
concept-trash-more = Das Verschieben in den Papierkorb setzt nur eine Markierung: Der Eintrag behält seinen Platz in der Projektmappe und wird als inaktiv markiert, statt entfernt zu werden. Beim Wiederherstellen erscheint er genau dort wieder, ohne dass Sie ihn neu einordnen müssen. „Papierkorb leeren“ entfernt Einträge endgültig. Jede dieser Aktionen ist wie andere Projektänderungen ein rückgängig machbarer Schritt.
concept-spellcheck = Unterstreicht Wörter, die kein installiertes Wörterbuch der Projektsprache erkennt.
concept-spellcheck-more = Jedes Projekt hat seine eigene Arbeitssprache oder mehrere Sprachen. Ein Eintrag kann sie für eine anderssprachige Szene überschreiben. Unbekannte Wörter können Sie über das Kontextmenü des Editors in ein persönliches, projektübergreifendes Wörterbuch aufnehmen. Ausschalten beendet die Rechtschreibprüfung überall, bis Sie sie wieder einschalten.
concept-search-replace = Findet Wörter oder Ausdrücke im gesamten Projekt, nicht nur im geöffneten Dokument.
concept-search-replace-more = Durchsucht alle Szenen, Notizen, Titel, Zusammenfassungen und Kennzeichnungen sowie Kommentare und Fußnoten. Groß-/Kleinschreibung, ganze Wörter und Akzente haben separate Schalter. „Alle ersetzen“ ändert alle geprüften Treffer in einem Durchgang und einem Rückgängig-Schritt: Strg+Z nimmt den gesamten Vorgang zurück, nicht nur die letzte Änderung.
concept-note-template = Ein wiederverwendbarer Textbaustein zum Einfügen in Ihre Arbeit.
concept-note-template-more = Ein leeres Figurenblatt, ein Ortsprofil, ein Szenenplan: einmal schreiben und überall einfügen, in jeden Eintrag, nicht nur in Notizen. Beginnen Sie mit einer Vorgabe, importieren Sie eine .md- oder .djot-Datei oder wählen Sie für einen eigenen Text Dokument ▸ Als Vorlage speichern. Die Vorlagen werden im Projekt gespeichert und stehen auch Mitautoren zur Verfügung.
concept-text-replacement = Eigene Kürzel, die beim Tippen automatisch erweitert werden.
concept-text-replacement-more = Legen Sie ein Kürzel und seine Ersetzung fest, etwa „mfg“ für „Mit freundlichen Grüßen“. Es wird erweitert, sobald ein Leer- oder Satzzeichen folgt. Die Großschreibung richtet sich nach der Eingabe: Ein großgeschriebenes Kürzel ergibt eine großgeschriebene Erweiterung. Direkt danach nimmt die Rücktaste nur diese Ersetzung zurück. Projektweise aktiviert und getrennt von [intelligenter Zeichensetzung](:concept-smart-punctuation).
concept-smart-punctuation = Automatische, sprachgerechte Typografie beim Tippen: Anführungszeichen, Striche, Auslassungszeichen.
concept-smart-punctuation-more = Wandelt gerade Anführungszeichen in typografische um, -- in einen Halbgeviertstrich, --- in einen Geviertstrich und ... in ein Auslassungszeichen. Auf Wunsch wird „- “ am Absatzanfang zu einem Dialogstrich. Projektweise festgelegt und in der .skrib-Datei mitgeführt, damit Mitautoren dieselben Regeln verwenden. Getrennt von [eigenen Ersetzungsregeln](:concept-text-replacement).
concept-export-style = Ein benannter Satz von Exportoptionen: Überschriften, Abstände und eingeschlossene Inhalte.
concept-export-style-more = Ein Stil wird für alle Exporte unabhängig vom Format wiederverwendet. Vorinstallierte Stile sind schreibgeschützt; duplizieren Sie einen als bearbeitbare Grundlage. Zu den Optionen gehören [Rückimport-Markierungen](:concept-round-trip-marks) für Dateien, die zum Lektorat und zurück gehen.
concept-round-trip-marks = Unsichtbare Lesezeichen, durch die bearbeitete DOCX- und ODT-Dateien beim Rückimport erkannt werden.
concept-round-trip-marks-more = Werden nur in DOCX und ODT geschrieben, die beiden für den Rückweg unterstützten Formate. Andere Exporte enthalten keine. Es sind Lesezeichen statt eigener Attribute: LibreOffice entfernt eigene Attribute beim Speichern, lässt Lesezeichen aber bestehen. Aktivierung pro [Exportstil](:concept-export-style) unter „Zum Lektorat senden“.
concept-status = Der Stand eines Eintrags in Ihrem Arbeitsablauf: jeweils eine Stufe auf dem Weg nach vorn.
concept-status-more = Kein [Schlagwort](:concept-tag): Ein Eintrag hat genau einen Status. Die Stufen sind geordnet und gehören zum Projekt, sodass Sie sie jederzeit umbenennen, umordnen oder ergänzen können. Ein Sternchen neben einem Kapitel oder Buch bedeutet, dass die Untereinträge nicht alle denselben Stand haben. Festlegen können Sie den Status im Inspektor, auf einer Karte, in einer fortlaufenden Ansicht oder hier.
