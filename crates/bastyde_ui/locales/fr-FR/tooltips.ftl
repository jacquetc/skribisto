# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Skribisto: infobulles enrichies du modèle d’écriture (français).
# Enregistrées dans tooltip_registry.rs et attachées par clé depuis les menus
# « ＋ Créer » / « Convertir en ». Les corps wm-*-more se renvoient l’un à
# l’autre via des liens [label](:key) ; ce fichier fait ainsi office d’aide en
# place.

wm-book = Un livre de votre projet.
wm-book-more = Un livre est aussi simple ou aussi structuré que vous le souhaitez. Une nouvelle peut se limiter à quelques [scènes](:wm-scene). Une œuvre plus longue peut imbriquer des [parties](:wm-part) et des [chapitres](:wm-chapter) aussi profondément que vous voulez. La forme vous appartient.
wm-part = Une partie regroupant des chapitres.
wm-part-more = Une partie rassemble des [chapitres](:wm-chapter) sous un même titre. Comme un chapitre, elle court jusqu’à la partie suivante ou la fin du [livre](:wm-book).
wm-chapter = Un chapitre du livre.
wm-chapter-more = Un chapitre court d’ici jusqu’au chapitre suivant, à la [partie](:wm-part) suivante, ou à la [fin du livre](:wm-end-of-book). Il existe sous deux formes : un dossier de chapitre qui contient des [scènes](:wm-scene), ou un chapitre à plat qui porte son propre texte. Choisissez la forme des nouveaux chapitres dans les Réglages, sous Structure.
wm-scene = Une scène de texte.
wm-scene-more = Une scène contient votre texte. Chaque élément d’écriture a deux faces : le texte principal et un [synopsis](:wm-synopsis) pour préparer. Les deux s’ouvrent ensemble dans l’éditeur.
wm-note = Une note.
wm-note-more = Une note conserve un texte et un [synopsis](:wm-synopsis), comme une [scène](:wm-scene), pour la documentation, les idées ou les rappels. N’importe quel élément peut faire partie du livre compilé, y compris une note placée dans un [chapitre](:wm-chapter).
wm-note-folder = Une note qui contient d’autres éléments.
wm-note-folder-more = Une [note](:wm-note) qui contient aussi des éléments en dessous, pour regrouper des notes liées. Elle garde son propre [synopsis](:wm-synopsis), et les éléments à l’intérieur gardent leur propre texte.
wm-folder = Un dossier d’organisation.
wm-folder-more = Un conteneur pour ranger les éléments du classeur à votre convenance. Il porte un [synopsis](:wm-synopsis) mais pas de texte propre. Les éléments à l’intérieur portent le texte.
wm-end-of-book = Marque la fin d’un livre.
wm-end-of-book-more = Chaque livre partage une seule liste continue ; sa fin n’est donc pas déterminée par l’imbrication. Ce repère indique que le [livre](:wm-book) s’arrête ici. Tout ce qui suit appartient au livre suivant.
wm-synopsis = Un résumé d’un élément d’écriture.
wm-synopsis-more = Un résumé attaché à tout élément d’écriture, en général un court paragraphe mais aussi long que vous voulez. Utilisez-le pour préparer et naviguer avant que le texte n’existe. Il se place à côté du texte principal dans l’éditeur.
