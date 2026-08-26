# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Étiquettes — la palette du projet, la section de l'inspecteur et les préréglages.
# Les NOMS des étiquettes des préréglages sont traduits volontairement : les préréglages
# sont générés dans le code plutôt que livrés comme données, précisément pour qu'un projet
# français obtienne des noms français.

## Menu des préréglages
tags-preset-basic = Basique
tags-preset-scifi = Science-fiction
tags-preset-fantasy = Fantasy
tags-preset-mystery = Policier
tags-preset-historical = Historique

## Préréglage basique — le cycle de travail. Le préfixe « statut/ » est une convention de
## nommage : le tri alphabétique suffit à regrouper ces étiquettes dans toutes les listes.
tags-preset-status-outline = statut/plan
tags-preset-status-draft = statut/brouillon
tags-preset-status-to-review = statut/à relire
tags-preset-status-finished = statut/terminé

## Préréglage basique — les marqueurs. Volontairement sans préfixe : une scène peut être un
## brouillon ET demander des recherches, ils ne font donc pas partie du cycle ci-dessus.
tags-preset-needs-research = recherches à faire
tags-preset-continuity-check = vérifier la continuité
tags-preset-plot-point = point d’intrigue

## Préréglage basique — la taxinomie que l'index des mentions cherche dans le texte.
tags-preset-character = personnage
tags-preset-place = lieu
tags-preset-item = objet

## Ajouts par genre
tags-preset-vessel = vaisseau
tags-preset-planet = planète
tags-preset-organization = organisation
tags-preset-creature = créature
tags-preset-faction = faction
tags-preset-artifact = artefact
tags-preset-realm = royaume
tags-preset-magic-system = système de magie
tags-preset-suspect = suspect
tags-preset-victim = victime
tags-preset-clue = indice
tags-preset-red-herring = fausse piste
tags-preset-historical-figure = personnage historique
tags-preset-source = source
tags-preset-period-detail = détail d’époque

## Le champ d'étiquettes (inspecteur) et son sélecteur « + »
tags-pill-list = Étiquettes
tags-pill-add = Ajouter une étiquette
tags-pill-remove = Retirer { $name }
tags-pill-filter-placeholder = Filtrer ou nommer une étiquette
tags-pill-no-match = Aucune étiquette ne correspond
tags-pill-create = Créer « { $name } »
tags-pill-create-failed = Impossible de créer « { $name } »
tags-pill-new-discoverable = Étiquette repérable dans le texte
tags-pill-new-discoverable-hint = Les éléments portant cette étiquette sont recherchés dans votre texte pour remplir la distribution.

## Le champ des autres noms
tags-alias-list = Autres noms
tags-alias-add = Ajouter un autre nom
tags-alias-remove = Retirer { $name }
tags-alias-placeholder = Un autre nom, puis Entrée
tags-alias-hint = Les noms sous lesquels cet élément apparaît dans votre texte, en plus de son titre.
tags-alias-collision = { $name } répond déjà à ce nom.

## Paramètres ▸ Projet ▸ Étiquettes
settings-page-tags = Étiquettes
settings-tags-desc = Les étiquettes servent à qualifier les éléments du classeur. Une étiquette repérable dans le texte indique en plus à Skribisto de chercher les noms de cet élément dans votre texte.
settings-tags-add = Ajouter
settings-tags-add-placeholder = Nommer une nouvelle étiquette
settings-tags-added = « { $name } » ajoutée
settings-tags-duplicate = « { $name } » existe déjà
settings-tags-filter = Filtrer les étiquettes
settings-tags-count = { $n ->
    [one] 1 étiquette
   *[other] { $n } étiquettes
}
settings-tags-details-placeholder = Ce que signifie cette étiquette
settings-tags-discoverable = Repérer dans le texte
settings-tags-delete = Supprimer { $name }
settings-tags-deleted = « { $name } » supprimée et retirée de tous les éléments
settings-tags-empty = Aucune étiquette pour le moment.
settings-tags-apply-preset = Appliquer un préréglage…
settings-tags-preset-applied = { $added ->
    [one] { $added } ajoutée
   *[other] { $added } ajoutées
}, { $skipped ->
    [one] { $skipped } déjà présente ignorée
   *[other] { $skipped } déjà présentes ignorées
}
settings-tags-csv-filter = Fichiers CSV
settings-tags-import = Importer…
settings-tags-export = Exporter…
settings-tags-imported = { $added ->
    [one] { $added } importée
   *[other] { $added } importées
}, { $skipped ->
    [one] { $skipped } ignorée
   *[other] { $skipped } ignorées
}
settings-tags-exported = { $n ->
    [one] 1 étiquette exportée
   *[other] { $n } étiquettes exportées
}

## La rangée de pastilles affichée dans le flux, le tableau et le sous-titre de l’éditeur
tags-chip-more = { $n ->
    [one] 1 étiquette de plus
   *[other] { $n } étiquettes de plus
}

## Présence de la scène (épingles bible narrative + suggestions)
cast-section = Présence
cast-add = Ajouter à la présence…
cast-add-filter-placeholder = Filtrer la bible narrative…
cast-add-empty = Aucune entrée de bible narrative à ajouter
cast-pin = Ajouter { $name } à la présence
cast-unpin = Retirer { $name } de la présence
cast-empty = Personne n’est encore épinglé — ajoutez ou conservez une suggestion
cast-unresolved = Ne fait plus partie de la bible narrative

## Rétroliens sur une entrée de bible narrative
mentions-backlinks = Apparaît dans
mentions-hit-count = { $n ->
    [one] une fois
   *[other] { $n } fois
}
# Le badge sur une ligne dont la cible est le point de vue déclaré du propriétaire (voir
# `point_of_view`, distinct de `references`). Affiché aussi bien dans la liste de présence
# que dans les rétroliens (« Apparaît dans »), car un point de vue peut apparaître dans les
# deux sens.
mentions-point-of-view-badge = Point de vue
mentions-point-of-view-badge-tooltip = Déclaré comme point de vue ici, défini à la main, pas détecté dans le texte.

## Anciennes clés conservées pour les scripts/tests qui y font encore référence
mentions-roster = Présence
mentions-pin = Ajouter { $name } à la présence

# ── Point de vue ─────────────────────────────────────────────────────────────
pov-section = Point de vue
pov-empty = Aucun point de vue défini
pov-add = Définir le point de vue…
pov-multiple = Cette scène a deux points de vue.
pov-remove = Retirer { $name } comme point de vue
pov-unresolved = Un point de vue a été épinglé ici, mais cette entrée n’existe plus ou a perdu son étiquette de bible narrative.

# ── Classement par livre ─────────────────────────────────────────────────────
# Le ou les livres auxquels une note ou un dossier de notes est rattaché.
# Affiché uniquement pour la bible narrative, en dehors du fil du manuscrit,
# car le livre d'une scène se déduit déjà de sa position dans le classeur, et
# seulement à partir de deux livres dans le projet ; un projet à un seul livre
# n'a rien à classer.
books-section = Classé sous
books-empty = Pas encore classé sous un livre
books-add = Classer sous un livre…
books-remove = Retirer de { $name }
books-apply-to-children = Appliquer le classement aux enfants

# ── Le segment Détails de l'onglet d'un `Item/Note` ──────────────────────────
# Presque le même ensemble de champs que la section bible narrative de
# l'inspecteur (étiquettes, autres noms, présence, point de vue, livres),
# repris volontairement, mais sous ses propres clés `note-details-` : ce
# segment est une page pleine largeur, pas un panneau, et son texte peut donc
# se permettre d'en dire un peu plus que les chaînes plus étroites du
# panneau. Voir `teksilo_ui::tabs::note_details`.
note-details-name-placeholder = Nom…
note-details-tags = Étiquettes
note-details-aliases = Autres noms
note-details-books = Classé sous
note-details-books-empty = Pas encore classé sous un livre
note-details-cast = Présence
note-details-cast-empty = Personne pour l’instant. Ajoutez quelqu’un, ou attendez qu’une analyse en suggère un.
note-details-pov = Point de vue
note-details-pov-empty = Aucun point de vue défini
note-details-pov-unresolved = Un point de vue a été épinglé ici, mais cette entrée n’existe plus ou a perdu son étiquette de bible narrative.
note-details-pov-multiple = Cette note a deux points de vue.
note-details-backlinks = Apparaît dans le manuscrit
note-details-backlinks-empty = Rien pour l’instant. Dès que ce nom apparaîtra dans votre texte, il s’affichera ici.
note-details-untitled-document = Sans titre
