# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

## La fenêtre d'une entrée de bible narrative : l'étape de configuration ouverte par
## « Entrée de bible narrative… » du vocabulaire ＋ Créer sur une ligne qu'il vient de
## créer. Voir `teksilo_ui::story_bible::modal`.
##
## « Ajouter comme note » n'y mène plus : cette porte range la note aussitôt, sous la
## seule étiquette choisie. Ses propres chaînes sont plus bas, sous « la capture ».

story-bible-modal-configure-title = Configurer la nouvelle entrée
story-bible-modal-name-label = Nom
story-bible-modal-name-placeholder = Un personnage, un lieu, tout ce qui mérite d’être répertorié…
story-bible-modal-no-binders = Ce projet n’a pas encore de classeur où la ranger
story-bible-modal-template-label = Partir d’un modèle (facultatif)
story-bible-modal-template-placeholder = Aucun modèle
story-bible-modal-body-label = Contenu
story-bible-modal-cancel = Annuler
story-bible-modal-create = Créer
story-bible-modal-create-and-open = Créer et ouvrir
story-bible-modal-failed = Impossible de créer l’entrée

## « Ajouter comme note » : la ligne du menu contextuel de l'éditeur. Un sous-menu des
## étiquettes du projet, en trois niveaux, « Sans étiquette » toujours en dernier et
## toujours présent. En choisir une range aussitôt la sélection comme note, sans boîte
## de dialogue : l'étiquette dit où elle va et de quoi elle part.
ctx-add-as-note = Ajouter comme &note…
ctx-add-as-note-all-tags = Toutes les étiquettes…
ctx-add-as-note-untagged = Sans étiquette

## La bible narrative (C2) : une grille de fiches sur chaque dossier de notes,
## groupée par étiquette repérable. Voir `teksilo_ui::tabs::story_bible_place`.
story-bible-grid-label = Bible narrative
story-bible-grid-untagged = Pas encore étiquetée
story-bible-grid-empty-title = Rien n’est répertorié ici pour l’instant
story-bible-grid-empty-hint = Rendez une note repérable, ou ajoutez-en une depuis une sélection, et elle apparaîtra ici.
story-bible-grid-alias-count = { $count ->
    [0] Aucun alias pour l’instant
    [one] { $count } alias
   *[other] { $count } alias
}
# À l’échelle du projet, jamais un chiffre propre à un seul livre : une
# occurrence relevée dans n’importe quel livre du projet compte, quel que
# soit celui actuellement ouvert. Uniquement les scènes : une note d’univers
# qui cite cette entrée ne compte pas comme « une scène ».
# Compte une présence déclarée, pas seulement une occurrence textuelle : une
# scène épinglée à la main, ou déclarée comme son point de vue, compte même
# si le nom de l’entrée n’y est jamais réellement écrit. Le texte dit
# « apparaît », pas « mentionnée » : il ne doit pas prétendre que le nom
# figure dans le texte quand seule une relation a été déclarée.
story-bible-grid-mention-count = { $count ->
    [0] Aucune apparition pour l’instant
    [one] Apparaît dans { $count } scène de l’ensemble du projet
   *[other] Apparaît dans { $count } scènes de l’ensemble du projet
}
story-bible-books-filter-all = Tous les livres
# Une entrée sans étiquette découvrable n'a jamais figuré dans la table du scan :
# rien n'a donc été cherché. Afficher « Aucune apparition » pour elle affirmerait une
# absence jamais mesurée.
story-bible-grid-not-searched = Pas encore analysé — il manque une étiquette de bible narrative

## Le segment « Dans le texte » de l'onglet d'un `Item/Note` (C3) : un flux
## modifiable du texte du manuscrit où cette note a été déclarée présente, un
## livre à la fois. Voir `teksilo_ui::tabs::note_in_prose`.
note-in-prose-pov = Point de vue
note-in-prose-cast = Présence
note-in-prose-pov-and-cast = Point de vue · Présence
note-in-prose-no-books = Ce projet n’a pas encore de livre
note-in-prose-empty-book = Pas encore déclarée dans ce livre

# The capture flow: one click from a selection to a filed note.
story-bible-capture-toast = « { $name } » ajouté à la bible narrative
story-bible-capture-open = Ouvrir
story-bible-capture-undo = Annuler
story-bible-capture-where-title = Où ranger ces notes ?
story-bible-capture-where-prompt = Choisissez le dossier où classer les nouvelles notes.
story-bible-capture-where-confirm = Classer ici
story-bible-capture-where-needs-folder = Choisissez un dossier dans lequel ranger ces notes.
story-bible-capture-where-tag = Question posée une seule fois. Les notes étiquetées « { $tag } » iront désormais ici, et vous pourrez le changer dans Paramètres, Œuvre, Étiquettes.
story-bible-capture-where-untagged = Question posée une seule fois. Les notes sans étiquette iront désormais ici, et vous pourrez le changer dans Paramètres, Œuvre, Étiquettes.
