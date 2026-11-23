# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Ressources : fichiers et liens web attachés aux éléments du classeur.

## Le panneau

resources-title = Ressources
shortcut-name-resources-show = Afficher les ressources
resources-add-files = Ajouter des fichiers
resources-add-files-title = Ajouter des fichiers à cet élément
resources-add-link = Ajouter un lien web
resources-drop-hint = Déposez des fichiers ou des adresses web pour les attacher
resources-drop-hint-unattached = Déposez des fichiers ou des adresses web pour les ajouter sans les attacher à un élément
resources-no-item = Ouvrez un élément pour voir ses ressources.
resources-empty = Rien n’est encore attaché à cet élément. Ajoutez des fichiers ou des liens web avec les boutons ci-dessus, ou déposez-les ici.
resources-dock-view-label = Ressources à afficher
resources-dock-view-item = Cet élément
resources-dock-view-unattached = Non attachées
resources-dock-view-item-hint = Les fichiers et liens de l’élément où vous êtes. Ce que vous ajoutez lui est attaché.
resources-dock-view-unattached-hint = Les fichiers et liens attachés à aucun élément, comme une recherche pas encore classée. Ce que vous ajoutez ici n’est attaché à rien.
resources-unattached-empty = Chaque ressource de ce projet est attachée à un élément. Les fichiers et liens ajoutés ici ne sont attachés à rien tant que vous ne les attachez pas.
resources-actions = Actions
resources-not-found = Introuvable
resources-row-linked = Fichier lié
resources-row-linked-size = Fichier lié, { $size }
resources-row-name = { $title }, { $kind }, { $detail }
resources-row-name-missing = { $title }, { $kind }, { $detail }, introuvable

## Les types de fichier, tels qu’un lecteur d’écran les annonce

resources-kind-picture = image
resources-kind-pdf = document PDF
resources-kind-audio = enregistrement sonore
resources-kind-video = vidéo
resources-kind-document = document
resources-kind-ebook = livre numérique
resources-kind-archive = archive
resources-kind-file = fichier
resources-kind-web-link = lien web

## Le menu d’une ligne

resources-open = Ouvrir
resources-show-in-folder = Afficher dans le dossier
resources-locate = Retrouver…
resources-insert-into-text = Insérer dans le texte
resources-insert-picture = Insérer l’image dans le texte
resources-insert-link = Insérer un lien dans le texte
resources-edit = Renommer et décrire…
resources-detach = Détacher de cet élément
resources-remove = Retirer du projet…

## Les deux boîtes de dialogue

resources-link-dialog-title = Ajouter un lien web
resources-link-dialog-address = Adresse
resources-link-dialog-address-placeholder = https://
resources-link-dialog-name = Titre
resources-link-dialog-name-placeholder = Le nom du site, si vous le laissez vide
resources-link-dialog-add = Ajouter
resources-edit-dialog-title = Renommer et décrire
resources-edit-dialog-name = Titre
resources-edit-dialog-description = Description
resources-edit-dialog-description-placeholder = Ce que c’est, et pourquoi vous l’avez gardé
resources-edit-dialog-save = Enregistrer
resources-dialog-cancel = Annuler

## Ajouter

resources-large-title = Un gros fichier
resources-large-text = « { $name } » fait { $size }. Rangé dans le projet, il voyage avec chaque copie et chaque sauvegarde du projet. Lié, il reste où il est, et le projet retient seulement où le trouver.
resources-large-text-many = « { $name } » fait { $size }, et { $others ->
    [1] un autre fichier est
   *[other] { $others } autres fichiers sont
} gros aussi. Rangés dans le projet, ils voyagent avec chaque copie et chaque sauvegarde du projet. Liés, ils restent où ils sont, et le projet retient seulement où les trouver.
resources-large-store = Le ranger dans le projet
resources-large-link = Le lier plutôt
resources-large-remember = Ne plus demander
resources-storing-title = Ajout des fichiers au projet
resources-store-cancel = Annuler
resources-store-cancelled = Aucun fichier n’a été ajouté.
resources-store-busy = Des fichiers sont encore en cours d’ajout. Réessayez quand ils seront arrivés.
resources-store-failed = Les fichiers n’ont pas pu être ajoutés : { $error }
resources-no-project = Ouvrez un projet avant d’attacher des fichiers.
resources-moved-away = Le projet a été déplacé pendant la copie du fichier.
resources-some-failed = { $count ->
    [1] Un fichier n’a pas pu être ajouté : { $names }
   *[other] { $count } fichiers n’ont pas pu être ajoutés : { $names }
}
resources-add-failed = Les ressources n’ont pas pu être ajoutées : { $error }
resources-added = { $count ->
    [1] Une ressource ajoutée
   *[other] { $count } ressources ajoutées
}
resources-dropped-added = { $count ->
    [1] Ajouté comme ressource de cet élément, et lié dans le texte
   *[other] { $count } fichiers ajoutés comme ressources de cet élément, et liés dans le texte
}
resources-dropped-undo-kept = Le texte a changé depuis le dépôt, donc rien n’a été annulé.
resources-undo = Annuler
resources-link-refused = « { $url } » n’est pas une adresse web que Skribisto peut garder. Utilisez une adresse qui commence par http:// ou https://, sans nom d’utilisateur ni mot de passe.

## L’avis pour un projet en un seul fichier

resources-folder-title = Ce projet devient volumineux
resources-folder-text = Ses fichiers rangés occupent maintenant { $size }. Un projet gardé en un seul fichier est réécrit en entier à chaque enregistrement, ses fichiers aussi. Gardé en dossier, il n’enregistre que ce qui a changé. Vous pouvez l’enregistrer en dossier maintenant, et continuer de travailler dans le dossier ensuite.
resources-folder-switch = Enregistrer en dossier…
resources-folder-not-now = Pas maintenant
resources-folder-never = Ne plus demander pour ce projet

## Ouvrir

resources-not-found-toast = « { $name } » n’est pas là où le projet l’attend.
resources-unsafe-title = Ouvrir « { $name } » ?
resources-unsafe-text = Skribisto ne sait pas si ce genre de fichier est sûr. L’ouvrir pourrait lancer un programme sur votre ordinateur, et un projet peut venir de quelqu’un d’autre. Affichez-le d’abord dans son dossier pour voir ce que c’est.
resources-open-anyway = Ouvrir quand même
resources-open-failed = Le fichier n’a pas pu être ouvert : { $error }
resources-no-data-folder = ce système ne donne à Skribisto aucun dossier de données où ouvrir une copie
resources-link-gone = Ce lien mène à une ressource que le projet n’a plus.
resources-locate-title = Où se trouve « { $name } » maintenant ?
resources-relinked-more = { $count ->
    [1] Un autre fichier introuvable a été trouvé dans le même dossier, et lié à nouveau.
   *[other] { $count } autres fichiers introuvables ont été trouvés dans le même dossier, et liés à nouveau.
}
resources-changed-elsewhere = « { $name } » a été modifié dans une autre application. Le projet garde la nouvelle version.

## Modifier

resources-edit-failed = La ressource n’a pas pu être modifiée : { $error }
resources-remove-title = Retirer « { $title } » du projet ?
resources-remove-text = Elle est retirée de chaque élément auquel elle est attachée. Un fichier rangé dans le projet part avec elle ; un fichier lié et une page web restent où ils sont. Annuler la fait revenir.
resources-remove-confirm = Retirer
resources-remove-failed = La ressource n’a pas pu être retirée : { $error }
resources-removed-toast = « { $title } » a été retirée du projet.
resources-removed-many-toast = { $count } ressources ont été retirées du projet.
resources-detached-toast = « { $title } » a été détachée.
resources-attached-one-toast = « { $title } » ajoutée aux ressources de « { $item } ».
resources-attached-many-toast = { $count } ressources ajoutées à « { $item } ».

## Dans le texte

resources-no-text = Placez d’abord le curseur dans un texte, puis insérez-y la ressource.
resources-drop-insert-picture = Insérer l’image
resources-drop-insert-link = Insérer un lien vers la ressource

## Progression

progress-resources-storing = Copie des fichiers
progress-resources-stored = Fichiers copiés

## La page Ressources d’un dossier

segment-folder = Dossier
resources-folder-count = { $count ->
    [1] Une ressource
   *[other] { $count } ressources
}
resources-view-label = Afficher les ressources en
resources-view-list = Liste
resources-view-grid = Grille
resources-grid-label = Ressources
resources-folder-drop-hint = Déposez des fichiers ou des adresses web pour les attacher à ce dossier
resources-folder-empty = Rien n’est encore attaché à ce dossier ni à ce qu’il contient. Ajoutez des fichiers ou des liens web avec les boutons ci-dessus, ou déposez-les ici : ils sont attachés au dossier.
resources-attached-to = Sur
resources-attached-separator = ,
resources-attached-more = { $count ->
    [1] et un autre
   *[other] et { $count } autres
}
resources-row-name-attached = { $name }, sur { $items }
resources-open-item = Ouvrir « { $title } »
resources-detach-from = Détacher de « { $title } »

## Les images en ressource dans la Galerie

gallery-resource-badge = Ressource
gallery-tile-resource = Image en ressource de { $title } : { $description }

## La page Ressources des réglages du projet

settings-page-resources = Ressources
settings-desc-resources = Les fichiers et liens web qu’aucun élément ne porte plus : attachez-les de nouveau, retirez-les, ou exportez-en des copies
settings-resources-description = Une ressource n’est plus attachée à rien quand elle est détachée de son dernier élément, ou quand l’élément qui la portait est supprimé pour de bon. Elle reste dans le projet, et un fichier rangé voyage avec chaque copie et chaque sauvegarde du projet, jusqu’à ce que vous l’attachiez de nouveau ou la retiriez.
settings-resources-group-storage = Stockage
settings-resources-group-unattached = Attachées à rien
settings-resources-stored = Fichiers rangés dans ce projet : { $size }
settings-resources-folder-advice = Un projet gardé en un seul fichier est réécrit en entier à chaque enregistrement, ses fichiers rangés aussi. Gardé en dossier, il n’enregistre que ce qui a changé, ce qui va plus vite.
settings-resources-count = { $count ->
    [1] Une ressource
   *[other] { $count } ressources
}
settings-resources-empty = Chaque ressource de ce projet est attachée à un élément.
resources-attach-to-items = Attacher à des éléments…
resources-export-copies = Exporter des copies…
resources-export-copy = Exporter une copie…
resources-remove-many-title = Retirer { $count } ressources du projet ?
resources-attach-title = Attacher à des éléments
resources-attach-hint = Choisissez les éléments auxquels attacher. Ctrl+clic ou Maj+clic en choisit plusieurs.
resources-attach-empty = Ce projet n’a encore aucun élément.
resources-attach-confirm = Attacher
resources-attached = { $count ->
    [1] Une ressource attachée
   *[other] { $count } ressources attachées
} à { $items ->
    [1] un élément
   *[other] { $items } éléments
}
resources-export-title = Exporter des copies dans un dossier
resources-exported = { $count ->
    [1] Une ressource exportée
   *[other] { $count } ressources exportées
}
resources-export-missing = { $count ->
    [1] Un fichier n’est pas là où le projet l’attend, et a été laissé de côté : { $names }
   *[other] { $count } fichiers ne sont pas là où le projet les attend, et ont été laissés de côté : { $names }
}
resources-export-failed = { $count ->
    [1] Un fichier n’a pas pu être copié : { $names }
   *[other] { $count } fichiers n’ont pas pu être copiés : { $names }
}

## La préférence des gros fichiers (Comportement de l’éditeur)

settings-resources-large-file = Fichiers de plus de 10 Mo
settings-resources-large-ask = Demander à chaque fois
settings-resources-large-store = Toujours ranger dans le projet
settings-resources-large-link = Toujours lier au fichier
settings-hint-resources-large-file = Ce que devient un fichier de plus de 10 Mo que vous attachez comme ressource. Rangé dans le projet, il voyage avec chaque copie et chaque sauvegarde du projet. Lié, il reste où il est, et le projet retient seulement où le trouver.
