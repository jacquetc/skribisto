# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

## La fenêtre de création d'une entrée de bible narrative (C1), atteinte
## depuis « Ajouter comme note » sur une sélection de texte, et comme étape
## de configuration ouverte par « Entrée de bible narrative… » du vocabulaire
## ＋ Créer sur une ligne qu'il vient de créer. Voir
## `teksilo_ui::story_bible::modal`.

story-bible-modal-create-title = Nouvelle entrée de bible narrative
story-bible-modal-configure-title = Configurer la nouvelle entrée
story-bible-modal-name-label = Nom
story-bible-modal-name-placeholder = Un personnage, un lieu, tout ce qui mérite d’être répertorié…
story-bible-modal-location-label = Où
story-bible-modal-no-binders = Ce projet n’a pas encore de classeur où la ranger
story-bible-modal-template-label = Partir d’un modèle (facultatif)
story-bible-modal-template-placeholder = Aucun modèle
story-bible-modal-body-label = Contenu
story-bible-modal-cancel = Annuler
story-bible-modal-create = Créer
story-bible-modal-create-and-open = Créer et ouvrir
story-bible-modal-choose-location = Choisissez d’abord où ranger cette entrée
story-bible-modal-failed = Impossible de créer l’entrée

## « Ajouter comme note » : la ligne du menu contextuel de l'éditeur qui ouvre
## la fenêtre ci-dessus, préremplie à partir de la sélection en cours.
ctx-add-as-note = Ajouter comme &note…

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
story-bible-grid-mention-count = { $count ->
    [0] Pas encore mentionnée
    [one] Mentionnée dans { $count } scène de l’ensemble du projet
   *[other] Mentionnée dans { $count } scènes de l’ensemble du projet
}
story-bible-books-filter-all = Tous les livres
