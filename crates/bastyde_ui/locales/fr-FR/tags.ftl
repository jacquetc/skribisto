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
tags-preset-plot-point = point d'intrigue

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
tags-preset-period-detail = détail d'époque

## Le champ d'étiquettes (inspecteur) et son sélecteur « + »
tags-pill-list = Étiquettes
tags-pill-add = Ajouter une étiquette
tags-pill-remove = Retirer { $name }
tags-pill-filter-placeholder = Filtrer ou nommer une étiquette
tags-pill-no-match = Aucune étiquette ne correspond
tags-pill-create = Créer « { $name } »
tags-pill-new-discoverable = Étiquette de bible narrative
tags-pill-new-discoverable-hint = Les éléments portant cette étiquette sont recherchés dans votre texte pour remplir la distribution.

## Le champ des autres noms
tags-alias-list = Aussi appelé
tags-alias-add = Ajouter un autre nom
tags-alias-remove = Retirer { $name }
tags-alias-placeholder = Un autre nom, puis Entrée
tags-alias-hint = Les noms sous lesquels cet élément apparaît dans votre texte, en plus de son titre.

## Réglages ▸ Projet ▸ Étiquettes
settings-page-tags = Étiquettes
settings-tags-desc = Les étiquettes servent à qualifier les éléments du classeur. Une étiquette de bible narrative indique en plus à Skribisto de chercher les noms de cet élément dans votre texte.
settings-tags-add = Ajouter
settings-tags-add-placeholder = Nommer une nouvelle étiquette
settings-tags-added = « { $name } » ajoutée
settings-tags-duplicate = « { $name } » existe déjà
settings-tags-filter = Filtrer les étiquettes
settings-tags-count = { $n ->
    [one] 1 étiquette
   *[other] { $n } étiquettes
}
settings-tags-details-placeholder = Ce que signifie cette étiquette
settings-tags-discoverable = Bible narrative
settings-tags-delete = Supprimer { $name }
settings-tags-deleted = « { $name } » supprimée et retirée de tous les éléments
settings-tags-empty = Aucune étiquette pour le moment.
settings-tags-apply-preset = Appliquer un préréglage…
settings-tags-preset-applied = { $added } ajoutée(s), { $skipped } déjà présente(s) ignorée(s)
settings-tags-csv-filter = Fichiers CSV
settings-tags-import = Importer…
settings-tags-export = Exporter…
settings-tags-imported = { $added } importée(s), { $skipped } ignorée(s)
settings-tags-exported = { $n ->
    [one] 1 étiquette exportée
   *[other] { $n } étiquettes exportées
}
