# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Modèles de note — le catalogue du projet, son panneau de paramètres, le sous-menu
# d'insertion du menu Document, et les modèles fournis.
#
# Les NOMS des modèles fournis et les intitulés de section/champ dont leur corps est
# assemblé sont traduits à dessein : les modèles sont construits dans le code plutôt que
# livrés comme fichiers `.djot`, afin qu'en appliquer un aboutisse dans la langue de
# l'INTERFACE en cours d'utilisation — et pour qu'une amélioration ultérieure de la
# formulation touche toutes les langues au lieu d'être figée dans les projets déjà créés.
# Celle de l'interface, et non celle du projet : un modèle est un formulaire que l'auteur
# remplit puis réécrit, et une fois appliquées ses lignes sont stockées telles quelles et
# lui appartiennent.

## Paramètres ▸ Projet ▸ Modèles
settings-page-templates = Modèles
settings-templates-description = Des morceaux de texte réutilisables à insérer dans ce que vous écrivez : une fiche de personnage vierge, un profil de lieu, une trame de scène. Ils sont enregistrés dans ce projet, donc toute personne qui l’ouvre dispose des mêmes.
settings-templates-filter = Filtrer les modèles
settings-templates-count = { $n ->
    [0] Aucun modèle
    [one] 1 modèle
   *[other] { $n } modèles
}
settings-templates-empty = Ce projet ne contient encore aucun modèle.
settings-templates-empty-hint = Ajoutez-en un depuis les modèles fournis, importez un fichier .md ou .djot, ou écrivez quelque chose et choisissez Document ▸ Enregistrer comme modèle.
settings-templates-presets = Ajouter un modèle fourni
settings-templates-import = Importer…
settings-templates-export = Exporter…
settings-templates-star = Afficher en tête du menu d’insertion
settings-templates-unstar = Ne plus afficher en tête
settings-templates-move-up = Monter
settings-templates-move-down = Descendre
settings-templates-delete = Supprimer le modèle
settings-templates-name = Nom
settings-templates-name-placeholder = Nom du modèle
settings-templates-body-placeholder = Le texte que ce modèle insère
settings-templates-duplicate-name = Un autre modèle s’appelle déjà « { $name } »
settings-templates-words = { $n ->
    [one] 1 mot
   *[other] { $n } mots
}

## Retour d'import / export
templates-imported = { $added ->
    [one] 1 modèle importé
   *[other] { $added } modèles importés
}
templates-imported-renamed = Renommage pour éviter un doublon : { $names }
templates-imported-skipped = Illisibles : { $names }
templates-import-filter = Modèles (.md, .djot)
templates-exported = { $n ->
    [one] 1 modèle exporté
   *[other] { $n } modèles exportés
}
templates-preset-applied = « { $name } » ajouté

## Confirmation de suppression
templates-delete-title = Supprimer ce modèle ?
templates-delete-body = « { $name } » sera retiré de ce projet. Les notes déjà créées à partir de lui ne sont pas touchées.
templates-delete-confirm = Supprimer

## Menu Document
menu-document = &Document
menu-insert-template = Insérer un &modèle
menu-insert-template-none = Aucun modèle dans ce projet
menu-save-as-template = Enregi&strer comme modèle…

## Enregistrer comme modèle
save-as-template-title = Enregistrer comme modèle
save-as-template-explain = Le texte que vous éditez devient un modèle que vous pourrez insérer n’importe où ailleurs.
save-as-template-name = Nom
save-as-template-placeholder = Fiche de personnage
save-as-template-duplicate = Un modèle nommé « { $name } » existe déjà
save-as-template-empty-editor = Cet éditeur est vide — il n’y a rien à enregistrer.
save-as-template-confirm = Enregistrer le modèle
save-as-template-saved = « { $name } » enregistré comme modèle
template-inserted = « { $name } » inséré

## Noms des modèles fournis
note-template-preset-character-sheet = Fiche de personnage
note-template-preset-location = Lieu
note-template-preset-artifact = Objet
note-template-preset-beat-sheet = Trame de scène
note-template-preset-faction = Faction
note-template-preset-research-note = Note de documentation

## Ensembles de départ, proposés à la création d'un projet
note-template-set-essentials = L'essentiel
note-template-set-everything = Tous les modèles

## Intitulés de section
note-template-section-identity = Identité
note-template-section-appearance = Apparence
note-template-section-voice = Voix
note-template-section-psychology = Vie intérieure
note-template-section-history = Histoire
note-template-section-arc = Évolution
note-template-section-first-impression = Première impression
note-template-section-in-the-story = Dans le récit
note-template-section-the-thing-itself = L’objet lui-même
note-template-section-the-scene = La scène
note-template-section-the-shape = La structure
note-template-section-what-it-is = Ce que c’est
note-template-section-source = Source
note-template-section-what-it-says = Ce qu’elle dit

## Intitulés de champ
note-template-field-full-name = Nom complet
note-template-field-known-as = Surnom, à renseigner dans le champ Autres noms de l’inspecteur
note-template-field-age = Âge
note-template-field-role-in-story = Rôle dans le récit
note-template-field-build-and-features = Silhouette et signes particuliers
note-template-field-habitual-bearing = Maintien habituel
note-template-field-speech-patterns = Façon de parler et tics de langage
note-template-field-what-they-never-say = Ce qu’il ou elle ne dit jamais
note-template-field-want = Ce qu’il ou elle veut
note-template-field-need = Ce dont il ou elle a besoin sans le savoir
note-template-field-fear = Ce qu’il ou elle craint
note-template-field-flaw = Le défaut qui lui coûte
note-template-field-formative-event = Événement fondateur
note-template-field-relationships = Relations
note-template-field-starts-as = Au départ
note-template-field-ends-as = À l’arrivée
note-template-field-what-you-notice-first = Ce qu’on remarque en premier
note-template-field-sound-and-smell = Sons et odeurs
note-template-field-light-and-weather = Lumière et météo
note-template-field-what-happened-here = Ce qui s’est passé ici
note-template-field-who-lives-or-works-here = Qui y vit ou y travaille
note-template-field-scenes-set-here = Scènes qui s’y déroulent
note-template-field-why-it-matters = Pourquoi c’est important
note-template-field-appearance = Aspect
note-template-field-age-and-origin = Âge et origine
note-template-field-what-it-does = Ce qu’il fait
note-template-field-who-holds-it = Qui le détient
note-template-field-who-wants-it = Qui le convoite
note-template-field-pov = Point de vue
note-template-field-time-and-place = Moment et lieu
note-template-field-goal = Objectif
note-template-field-conflict = Conflit
note-template-field-turn = Le basculement
note-template-field-exit-emotion = Émotion en sortant
note-template-field-purpose = Raison d’être
note-template-field-who-leads-it = Qui la dirige
note-template-field-resources = Ressources
note-template-field-allies-and-enemies = Alliés et ennemis
note-template-field-what-it-wants-now = Ce qu’elle veut maintenant
note-template-field-where-from = D’où elle vient
note-template-field-page-or-link = Page ou lien
note-template-field-key-facts = Faits essentiels
note-template-field-how-it-is-used = Comment elle sert le récit
