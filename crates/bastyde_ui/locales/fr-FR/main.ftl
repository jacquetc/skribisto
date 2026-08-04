# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Skribisto: chaînes de l'interface (français).
# Un simple « & » marque le mnémonique d'un libellé de menu ; « && » est un « & » littéral.

## Barre de menus: Œuvre
menu-work = Œ&uvre
menu-new-work = &Nouvelle œuvre
menu-open-work = &Ouvrir une œuvre…
menu-new-window = Nouvelle fenê&tre
menu-import-from = &Importer depuis
menu-import-plume = &Plume Creator (.plume)…
menu-export = E&xporter
menu-export-book = Exporter le livre
menu-export-part = Exporter la partie
menu-export-chapter = Exporter le chapitre
menu-export-scene = Exporter la scène
menu-export-note = Exporter la note
menu-export-paratext = Exporter le paratexte
menu-export-folder = Exporter le dossier
menu-export-choose = Choisir…
menu-export-none = Ouvrez un document à exporter
menu-save = &Enregistrer
menu-save-as-file = Enregistrer comme fichier &unique…
menu-save-as-folder = Enregistrer comme &dossier…
menu-backup = Créer une &copie de secours
menu-close-work = &Fermer l'œuvre
menu-welcome = &Bienvenue…
menu-settings = &Paramètres
menu-quit = &Quitter

## Barre de menus: Affichage
menu-view = &Affichage
menu-outline = &Plan
menu-search = &Rechercher dans le projet
menu-search-preview = &Aperçu de recherche
menu-fullscreen = P&lein écran
menu-focus-mode = &Mode sans distraction
menu-format = Fo&rmat
menu-scene-break = Insérer un &saut de scène
menu-major-scene-break = Insérer un saut de scène &majeur

## Barre de menus: Aller
menu-go = A&ller
menu-go-next-scene = &Scène suivante
menu-go-prev-scene = Scène p&récédente
menu-go-next-chapter = &Chapitre suivant
menu-go-prev-chapter = Chapitre précéde&nt
menu-go-next-note = No&te suivante
menu-go-prev-note = Note précédent&e

menu-tools = &Outils
menu-help = A&ide

## Menu contextuel du classeur
ctx-add = &Ajouter
ctx-new-item = &Nouvel élément
ctx-new-folder = Nouveau d&ossier
ctx-rename = &Renommer
ctx-duplicate = &Dupliquer
ctx-indent = &Indenter
ctx-outdent = Désinden&ter
ctx-trash = Mettre à la &corbeille
ctx-open-to-side = Ouvrir &sur le côté
ctx-open = &Ouvrir
ctx-reveal-in-outline = Afficher dans le &plan
ctx-move-up = Déplacer vers le &haut
ctx-move-down = Déplacer vers le &bas

## Recommandations de création: libellés de types (titre du SplitButton + Ajouter ▸)
create-book = Livre
create-part = Partie
create-chapter = Chapitre
create-scene = Scène
create-note = Note
create-note-folder = Dossier de notes
create-folder = Dossier
create-paratext = Paratexte
create-paratext-folder = Dossier de paratextes
create-book-end = Fin du livre
# Item-type names for the Overview's Type column (the rest reuse the create-* nouns).
type-book-start = Début du livre
type-text = Texte

## Création: le titre par défaut d'une nouvelle ligne.
## Donnée, et non habillage — résolu une seule fois à la création puis propriété
## de l'auteur, si bien que changer de langue ne renomme jamais l'existant.
## (Une scène réutilise « new-scene-title », partagé avec la division de scène.)
new-item-book = Nouveau livre
new-item-part = Nouvelle partie
new-item-chapter = Nouveau chapitre
new-item-note = Nouvelle note
new-item-note-folder = Nouveau dossier de notes
new-item-folder = Nouveau dossier
new-item-paratext = Nouveau paratexte
new-item-paratext-folder = Nouveau dossier de paratextes

## Recommandations de création: indication de placement en fin de ligne
placement-inside = à l’intérieur
placement-after = après
placement-after-parent = après le parent
placement-top-level = au niveau supérieur

# (Les infobulles enrichies du modèle d’écriture sont dans le tooltips.ftl de cette locale.)

## Promouvoir: convertir un élément du classeur vers son type apparié
ctx-promote = &Convertir en
promote-chapter-folder = Dossier de chapitre
promote-flat-chapter = Chapitre à plat
promote-lossy-title = Aucun endroit pour ce texte
promote-lossy-text = Un(e) { $target } n'a nulle part où conserver : { $kinds }. Déplacez ou effacez ce texte, puis convertissez.
# Noms des rôles de contenu, pour expliquer ce qu'une conversion ne peut pas reprendre.
content-scene-text = Texte de la scène
content-note-text = Texte de la note
content-book-title = Titre du livre
content-book-subtitle = Sous-titre du livre
content-part-title = Titre de la partie
content-chapter-title = Titre du chapitre
content-epigraph-text = Épigraphe
content-paratext-text = Paratexte
promote-blocked-title = Chapitre non vide
promote-blocked-text = Ce chapitre contient encore { $count } élément(s). Déplacez-les ou mettez-les à la corbeille avant de le convertir en chapitre à plat.

## Inspecteur (dock de droite) + bascules de docks dans la barre d'état
inspector = Inspecteur
inspector-empty = Ouvrez un élément pour l'inspecter.
inspector-promote = Convertir en…
statusbar-toggle-outline = Afficher/masquer le classeur
statusbar-toggle-inspector = Afficher/masquer l'inspecteur
# L'indicateur d'enregistrement (barre d'état, à côté de la bascule du classeur).
statusbar-save-unsaved = Modifications non enregistrées. Cliquez pour enregistrer
statusbar-save-saved = Toutes les modifications sont enregistrées
statusbar-save-autosave = L'enregistrement automatique est activé. Les modifications sont enregistrées au fil de l'écriture
statusbar-saving = Enregistrement…
# Le nombre de mots en direct de l'élément ciblé (barre d'état).
statusbar-word-count = { $count ->
    [one] { $count } mot
   *[other] { $count } mots
}
statusbar-word-count-tooltip = Mots dans la scène en cours d'édition
# Mots + caractères, quand « Afficher les caractères » est activé (Réglages ▸ Objectifs).
statusbar-word-char-count = { $words ->
    [one] { $words } mot
   *[other] { $words } mots
} · { $chars ->
    [one] { $chars } caractère
   *[other] { $chars } caractères
}
# La session d'écriture (minuteur de sprint + compteur de mots, barre d'état).
session-toggle = Session d'écriture : démarrer ou mettre en pause un sprint
session-configure = Définir l'objectif de mots et la limite de temps
session-configure-title = Session d'écriture
session-word-goal = Objectif de mots
session-time-limit = Limite de temps
session-no-goal = Aucun objectif
session-no-limit = Aucune limite
session-reset = Réinitialiser la session
session-readout = { $words } mots · { $time }
session-readout-timed = { $words } mots · { $time } restant
# La bande de contrôle toujours visible du mode sans distraction (étape 2).
statusbar-focus-exit = Quitter le mode sans distraction
# Les boutons Suivant/Précédent de la bande (étape 4) — déclenchent les mêmes
# actions go.next/go.prev que le raccourci et la paire générique du menu Aller à.
statusbar-focus-synopsis = Synopsis
statusbar-focus-go-prev = Élément précédent (Alt+Haut)
statusbar-focus-go-next = Élément suivant (Alt+Bas)

## Actions de la barre d'activités (commandes sans dock dans la barre d'icônes)
rail-settings = Paramètres

## Paramètres
language = Langue
theme = Thème
english = Anglais
french = Français
light = Clair
dark = Sombre
settings-text-width = Largeur du texte
settings-preview-width = Largeur de l'aperçu de recherche
settings-autosave = Enregistrement automatique sur le disque
settings-show-welcome = Afficher le lanceur au démarrage
settings-show-welcome-tip = Sinon, le dernier projet se rouvre.

## Fenêtre des paramètres: cadre
settings-title = Paramètres
settings-close = Fermer
settings-search = Rechercher un paramètre
settings-reset = Réinitialiser
settings-done = Terminé
settings-cancel = Annuler
settings-apply = Appliquer
settings-ok = OK
settings-reset-confirm-title = Réinitialiser tous les paramètres ?
settings-reset-confirm-body = Cela rétablit la valeur d'usine de chaque paramètre, sur toutes les pages. Cette action est irréversible.
settings-empty-title = Aucun paramètre ici pour l'instant
settings-empty-hint = Cette section proposera des options dans une prochaine mise à jour.

## Fenêtre des paramètres: catégories
settings-sec-appearance-behaviour = Apparence et comportement
settings-sec-editor = Éditeur
settings-sec-spelling = Orthographe
settings-sec-backup = Sauvegarde et synchronisation
settings-sec-compile = Compilation et export
settings-page-appearance = Apparence
settings-page-menus = Menus et barres d'outils
settings-page-notifications = Notifications
settings-page-scene = Scène
settings-page-synopsis = Synopsis
settings-page-notes = Notes
settings-page-editor-behavior = Comportement de l'éditeur
settings-page-goals = Objectifs et nombre de mots
settings-page-corkboard = Tableau de liège
settings-page-distraction-free = Sans distraction
settings-page-dictionaries = Dictionnaires
settings-page-autosave = Enregistrement automatique
settings-page-export = Formats d'export
settings-page-paratext = Structures de paratexte
settings-page-keymap = Raccourcis clavier
# Champ de filtre de la page Raccourcis (filtre la liste ShortcutSettings par nom / id / catégorie).
settings-keymap-filter = Filtrer les raccourcis

## Fenêtre des paramètres: champs
settings-group-typography = Typographie
settings-group-writing-column = Colonne d'écriture
settings-group-writing-view = Affichage de l'écriture
# Les éléments optionnels de la bande de contrôle du mode sans distraction
# (Quitter n'est jamais optionnel).
settings-group-distraction-free-strip = Bande de contrôle
settings-group-theme = Thème
settings-group-language = Langue
settings-group-startup = Démarrage
settings-group-autosave = Enregistrement automatique
settings-field-typeface = Police
settings-field-size = Taille du texte
settings-field-line-height = Interligne
settings-field-first-line-indent = Retrait de première ligne
settings-field-paragraph-spacing-before = Espace avant le paragraphe
settings-field-paragraph-spacing-after = Espace après le paragraphe
settings-field-column-width = Largeur de colonne
settings-distraction-free-width-hint = S'applique uniquement en mode sans distraction — les largeurs de colonne de la Scène, du Synopsis et des Notes restent inchangées.
settings-distraction-free-title = Conserver le nom de l'élément
settings-distraction-free-word-count = Conserver le compteur de mots
settings-distraction-free-session = Conserver la séance d'écriture
settings-distraction-free-go-to = Conserver le bouton Aller à…
settings-distraction-free-go = Conserver les boutons Précédent et Suivant
settings-distraction-free-chrome-hint = Le bouton Quitter reste toujours affiché, quels que soient ces choix — c'est votre porte de sortie si Échap est déjà pris.
settings-field-app-theme = Thème
settings-field-text-scale = Taille du texte de l'interface
settings-field-language = Langue de l'interface
settings-synopsis-placement = Position du synopsis
settings-synopsis-placement-none = Aucun
settings-synopsis-placement-top = Au-dessus
settings-synopsis-placement-side = À côté
settings-typewriter = Défilement machine à écrire
settings-typewriter-tip = Maintient la ligne en cours d'écriture à une hauteur fixe pendant que le manuscrit défile en dessous. Un clic place toujours le curseur là où vous cliquez.
settings-typewriter-position = Position de la ligne
settings-typewriter-position-top-third = Premier tiers
settings-typewriter-position-middle = Milieu
settings-typewriter-position-bottom-quarter = Quart inférieur
settings-highlight-scope = Mise en évidence autour du curseur
settings-highlight-scope-none = Aucune
settings-highlight-scope-sentence = Phrase
settings-highlight-scope-paragraph = Paragraphe
settings-highlight-scope-tip-none = Laisser la page unie. Rien n'est teinté pendant l'écriture.
settings-highlight-scope-tip-sentence = Teinter la phrase en cours d'écriture, pour la distinguer de celles qui l'entourent.
settings-highlight-scope-tip-paragraph = Teinter tout le paragraphe en cours d'écriture, pour garder sous les yeux le passage travaillé.
settings-group-container-views = Vues des conteneurs
settings-remember-view = Mémoriser la dernière vue pour chaque type d'élément
settings-remember-view-tip = Ouvrir un conteneur sur la vue utilisée en dernier pour ce type
settings-remember-view-tip-more =
    Un Livre, une Partie et un Chapitre offrent chacun plusieurs vues (sa propre
    page, le manuscrit complet, le synopsis complet). Activez cette option et
    chaque type se rouvre sur la vue choisie en dernier, par exemple, passez un
    Chapitre en Chapitre complet et le prochain Chapitre ouvert s'affichera aussi
    en Chapitre complet. Chaque type mémorise sa propre vue.
# Volet Objectifs et comptage des mots
settings-group-counting = Comptage des mots
settings-counting-auto = Automatique (selon la langue)
settings-counting-whitespace = Découper aux espaces
settings-counting-unicode-words = Mots Unicode
settings-counting-cjk-hybrid = Adapté au CJC (par caractère)
settings-counting-hint = Le mode automatique compte le chinois et le japonais par caractère, et toutes les autres langues par mot. Ne le changez que si le comptage semble incorrect pour votre langue.
settings-group-goals-display = Affichage
settings-show-characters = Afficher le nombre de caractères dans la barre d'état
settings-autosave-hint = Les modifications sont enregistrées automatiquement au fil de l'écriture.

## Paramètres: Œuvre (le projet ouvert)
settings-sec-work = Œuvre
settings-page-structure = Structure
settings-page-author = Auteur
settings-field-author-name = Nom de l'auteur
settings-field-author-placeholder = Facultatif
settings-field-author-hint = Apparaît sur la page de titre compilée et dans les métadonnées des fichiers exportés. Laissez vide pour l'omettre.
settings-group-chapters = Chapitres
settings-chapter-flat = Chapitres à plat
settings-chapter-flat-hint = Activé : un chapitre est une seule ligne. Vous y écrivez, et il ne contient aucune scène. Désactivé : un chapitre est un dossier. Vous y écrivez également, mais il peut en outre contenir des scènes. Les nouveaux chapitres suivent ce réglage ; les existants se convertissent via Promouvoir.

## Paramètres: Styles d'export (Compilation et export ▸ Formats d'export)
settings-styles-builtin = Styles intégrés
settings-styles-user = Mes styles
settings-styles-builtin-badge = Intégré
settings-styles-duplicate = Dupliquer
settings-styles-edit = Modifier
settings-styles-delete = Supprimer
settings-styles-import = Importer…
settings-styles-export = Exporter…
settings-styles-copy-suffix = (copie)
settings-styles-json-filter = Style d'export
settings-styles-editor-title = Modifier le style
settings-styles-editor-none = Sélectionnez un style personnalisé à modifier, ou dupliquez-en un intégré.
settings-styles-imported = Style importé
settings-styles-import-failed = Impossible d'importer le style
settings-styles-exported = Style exporté
settings-styles-export-failed = Impossible d'exporter le style
# Libellés des champs de l'éditeur
settings-styles-field-name = Nom
settings-styles-field-chapters = Titres de chapitre
settings-styles-field-parts = Titres de partie
settings-styles-field-scene-break = Saut de scène
settings-styles-field-major-scene-break = Saut de scène majeur
settings-styles-field-spacing = Interligne
settings-styles-field-justify = Justifier le texte
settings-styles-field-notes = Inclure les notes
settings-styles-field-synopses = Inclure les synopsis
settings-styles-field-scene-titles = Inclure les titres de scène
settings-styles-field-epigraphs = Inclure les épigraphes
settings-styles-field-paratexts = Inclure les paratextes
settings-styles-group-pages = Pages
settings-styles-field-word-count = Nombre de mots sur la page de titre
settings-styles-field-page-books = Nouvelle page à chaque livre
settings-styles-field-page-parts = Nouvelle page à chaque partie
settings-styles-field-page-chapters = Nouvelle page à chaque chapitre
settings-styles-field-page-paratexts = Nouvelle page à chaque paratexte
# Options de schéma de titre
settings-styles-heading-none = Aucun titre
settings-styles-heading-numbered = Numéro seul
settings-styles-heading-title = Titre seul
settings-styles-heading-both = Numéro + titre
# Options de saut de scène (les glyphes restent tels quels)
settings-styles-break-blank = Ligne vide
settings-styles-break-none = Aucun
# Options d'interligne
settings-styles-spacing-single = Simple
settings-styles-spacing-onehalf = 1½
settings-styles-spacing-double = Double

## Accueil
welcome-title = Bienvenue dans Skribisto
# $version provient de l'étiquette git, apposée à la compilation (src/version.rs).
welcome-version = Version { $version }
welcome-search = Rechercher des œuvres
welcome-open = Ouvrir
welcome-new-work = Nouvelle œuvre
welcome-recent-works = Œuvres récentes
welcome-empty-recents = Aucune œuvre récente.
# Affiché à la place de la liste des œuvres récentes lorsque la recherche n'en
# trouve aucune, à distinguer du cas où il n'y a aucune œuvre récente.
welcome-no-matches = Aucune œuvre récente ne correspond à votre recherche.
welcome-learn-soon = Guides et astuces à venir.
welcome-about-blurb = Skribisto, une réécriture en Rust + Bastyde de l'application d'écriture.
# Les *…* sont du balisage, pas de la décoration : ils mettent la ligne en
# italique (rendue dans un serif italique). Conserver les astérisques.
welcome-tagline = *Un endroit calme pour écrire de longs textes.*
# Infobulle et nom vocalisé des deux icônes de lien sous la barre de navigation.
# Noms propres : à conserver tels quels.
welcome-github = GitHub
welcome-discord = Discord
nav-works = Œuvres
nav-examples = Exemples
nav-learn = Apprendre
nav-about = À propos

## Onglets & volets d'édition
synopsis = Synopsis
epigraph = Épigraphe
pane-manuscript = Manuscrit
corkboard = Tableau d'affichage
overview = Aperçu

## Corkboard
corkboard-card-count = { $count ->
    [one] { $count } carte
   *[other] { $count } cartes
}
corkboard-child-count = { $count ->
    [one] { $count } élément
   *[other] { $count } éléments
}
corkboard-view-nested = Imbriqué
corkboard-view-flat = À plat
corkboard-layout-hint = « Imbriqué » affiche les enfants directs d'un conteneur — ouvrez une carte dossier pour y entrer. « À plat » affiche toutes les scènes du conteneur d'un coup.
corkboard-card-size = Taille des cartes
corkboard-search-placeholder = Filtrer les cartes…
corkboard-empty-title = Rien ici pour l'instant
corkboard-empty-hint = Utilisez « ＋ Nouveau » ci-dessus pour ajouter le premier élément.
corkboard-new = Nouveau
corkboard-show-card-numbers = Numéroter les cartes
corkboard-modal-size = Taille de l'éditeur agrandi
corkboard-scope-hint = S'applique à tous les tableaux ouverts, dans tous les projets.
corkboard-sort-manuscript = Ordre du manuscrit
corkboard-sort-title-asc = Titre A–Z
corkboard-sort-title-desc = Titre Z–A
corkboard-card-number = Carte { $number }

## Actions groupées du tableau (le menu d'une carte agit sur toute la sélection)
duplicate = Dupliquer
duplicate-n = { $count ->
    [one] Dupliquer { $count } carte
   *[other] Dupliquer { $count } cartes
}
set-label-n = { $count ->
    [one] Définir l'étiquette sur { $count } carte
   *[other] Définir l'étiquette sur { $count } cartes
}
move-to-trash-n = { $count ->
    [one] Mettre { $count } carte à la corbeille
   *[other] Mettre { $count } cartes à la corbeille
}
reveal-in-outline = Afficher dans le plan

## Sélecteur de destination « Déplacer vers… » du tableau
corkboard-move-to = Déplacer vers…
corkboard-move-to-n = { $count ->
    [one] Déplacer { $count } carte vers…
   *[other] Déplacer { $count } cartes vers…
}
corkboard-move-picker-title = Déplacer vers…
corkboard-move-picker-empty = Aucun classeur pour l'instant — créez-en un d'abord.
corkboard-move-picker-cancel = Annuler
corkboard-move-here = Déplacer ici
corkboard-moved-ok = { $count ->
    [one] { $count } carte déplacée
   *[other] { $count } cartes déplacées
}
corkboard-move-into-self = Un conteneur ne peut pas être déplacé dans lui-même. Choisissez une destination en dehors.
corkboard-move-failed = Ces cartes n'ont pas pu être déplacées là.

## Overview (the container's contents as a sortable table)
overview-col-title = Titre
overview-col-type = Type
overview-col-label = Étiquette
overview-col-tags = Étiquettes
overview-col-own-words = Mots
overview-col-total-words = Total
overview-row-count = { $count ->
    [one] { $count } ligne
   *[other] { $count } lignes
}
overview-search-placeholder = Filtrer les lignes…
overview-expand-all = Tout déplier
overview-collapse-all = Tout replier
overview-table-label = Contenu
overview-empty-title = Rien pour l’instant
overview-gone-title = Ce conteneur n’existe plus
overview-gone-hint = Il a été mis à la corbeille. Restaurez-le, ou fermez cet onglet.
overview-empty-hint = Utilisez « ＋ Nouveau » ci-dessus pour ajouter le premier élément.
corkboard-grid-label = Cartes du tableau
corkboard-rename-field = Renommer l’élément
corkboard-expand-synopsis = Agrandir le synopsis
corkboard-synopsis-modal-title = Synopsis
corkboard-badge-scene = Scène
corkboard-badge-chapter = Chapitre
corkboard-badge-part = Partie
corkboard-badge-book = Livre
corkboard-badge-note = Note
corkboard-badge-folder = Dossier
corkboard-badge-paratext = Paratexte
corkboard-badge-text = Texte
corkboard-badge-end = Fin
settings-group-corkboard-layout = Disposition
settings-group-corkboard-cards = Cartes
corkboard-show-word-count = Afficher le nombre de mots sur les cartes
text-heading = Texte
no-content = Cet élément n'a pas de contenu modifiable.
untitled = Sans titre
placeholder-title = Titre…
placeholder-subtitle = Sous-titre…
placeholder-chapter-title = Titre du chapitre…
split-editor = Diviser l'éditeur
close-split-view = Fermer la vue divisée
drop-open-here = Ouvrir ici
drop-open-to-side = Ouvrir sur le côté

## Flux du manuscrit (Chapitre / Partie / Livre complet + Synopsis complet)
# La page du conteneur lui-même. « Chapitre » = ce chapitre ; « Chapitre complet » =
# ce chapitre et toutes ses scènes.
segment-chapter = Chapitre
segment-part = Partie
segment-book = Livre
segment-notes = Notes
segment-pace = Rythme
pace-placeholder = Le planificateur de rythme apparaît ici.
# Planificateur de rythme
pace-empty-title = Planifiez le rythme de ce livre
pace-empty-body = Fixez un objectif de mots et une échéance : Skribisto calcule le rythme quotidien pour y parvenir.
pace-start-planning = Commencer la planification
pace-section-schedule = Calendrier
pace-section-progress = Progression
pace-advancement = Avancement
pace-goal = Objectif de mots
pace-deadline = Échéance
pace-active = Rythme actif
pace-day-mon = Lun
pace-day-tue = Mar
pace-day-wed = Mer
pace-day-thu = Jeu
pace-day-fri = Ven
pace-day-sat = Sam
pace-day-sun = Dim
pace-card-written = mots écrits
pace-card-of-goal = de l'objectif
pace-card-rate = mots / jour d'écriture
pace-card-days-left = jours d'écriture restants
pace-card-streak = jours de série
pace-card-ahead = mots d'avance
pace-card-behind = mots de retard
pace-charts-empty = Les graphiques de progression apparaissent ici une fois le nombre de mots enregistré lors d'une sauvegarde.
pace-chart-progression = Mots écrits par rapport à l'objectif
pace-chart-words-per-day = Mots par jour
pace-series-actual = Réel
pace-series-target = Objectif
pace-series-words-per-day = Mots/jour
pace-daily-target-line = Rythme régulier : { $count } mots/jour
pace-section-holidays = Congés
pace-section-milestones = Jalons
pace-holidays-none = Aucun congé. Tous les jours prévus comptent.
pace-holiday-label = Nom du congé
pace-add-holiday = Ajouter
pace-remove = Retirer
pace-milestones-none = Aucun jalon. Définissez-en un sur une partie ou un chapitre dans l'inspecteur.
full-chapter = Chapitre complet
full-part = Partie complète
full-book = Livre complet
full-synopsis = Synopsis complet
rename = Renommer
set-label = Définir l'étiquette
insert-scene = Insérer une scène
insert-chapter = Insérer un chapitre
split-scene = Scinder la scène
move-up = Monter
move-down = Descendre
merge-with-previous = Fusionner avec la précédente
move-to-trash = Mettre à la corbeille
rename-chapter = Renommer le chapitre
add-scene = Ajouter une scène
add-chapter = Ajouter un chapitre
new-scene-title = Nouvelle scène
placeholder-scene-name = Nom de la scène…
menu-cut = Couper
menu-copy = Copier
menu-paste = Coller
menu-paste-unformatted = Coller sans mise en forme
menu-select-all = Tout sélectionner
# Infobulles de la rangée de mise en forme du menu contextuel. Boutons sans
# libellé : l'infobulle est leur seul nom accessible, pas une décoration.
format-bold = Gras
format-italic = Italique
format-underline = Souligné
format-strikethrough = Barré

## Classeur / récents
binder = Classeur
no-work = Aucune œuvre chargée
no-recent-works = Aucune œuvre récente
switcher-open-section = Œuvres ouvertes
switcher-recent-section = Récentes
switcher-this-window = cette fenêtre
open-project-title = Ouvrir l’œuvre
open-project-question = Comment ouvrir « { $title } » ?
open-in-new-window = Ouvrir dans une nouvelle fenêtre
open-here = Ouvrir ici

## Sélecteur de classeur + recherche
binder-all = Tous les classeurs
binder-show-all = Afficher tous les classeurs
binder-new = Nouveau classeur…
binder-item-count = { $count } éléments
binder-search-placeholder = Filtrer le plan…
binder-search-scope = Chercher dans tous les classeurs
binder-trash-confirm-title = Mettre le classeur à la corbeille ?
binder-trash-confirm-text = « { $name } » et tous ses éléments seront mis à la corbeille.

## Boîtes de dialogue
dialog-rename = Renommer
dialog-set-label = Définir l'étiquette
dialog-new-scene = Nouvelle scène
close-work-question = Enregistrer les modifications avant de fermer l'œuvre ?
quit-question = Enregistrer les modifications avant de quitter ?
quit-save-work-question = Enregistrer les modifications de { $title } avant de quitter ?
unsaved-changes = Cette œuvre a des modifications non enregistrées.
# Affiché par Quitter quand une autre œuvre ouverte (pas celle de cette fenêtre)
# a encore des modifications non enregistrées — le dialogue unique listant
# chaque œuvre modifiée (voir l'action `app.quit` dans
# app::commands::file). Quitter est refusé tant qu'elles ne sont pas
# enregistrées ou fermées depuis leur propre fenêtre.
# Remplacement de l'œuvre ouverte dans cette fenêtre (Nouvelle œuvre, Ouvrir une
# œuvre, « Ouvrir ici », « Ouvrir maintenant » du bandeau d'import), même garde
# que la fermeture, puisque l'œuvre ouverte est fermée dans tous les cas.
new-work-unsaved-question = Enregistrer les modifications avant de créer une nouvelle œuvre ?
open-work-unsaved-question = Enregistrer les modifications avant d'ouvrir une autre œuvre ?
switch-backup-discard-title = Abandonner les modifications de cette copie de secours ?
switch-backup-discard-text = Les modifications d'une copie de secours ne peuvent pas y être enregistrées. Utilisez « Enregistrer sous » ou « Restaurer » pour les conserver, ou abandonnez-les et ouvrez l'autre œuvre.
switch-save-failed = L'œuvre n'a pas pu être enregistrée, elle n'a donc pas été remplacée : { $error }
switch-save-not-started = L'œuvre n'a pas pu être enregistrée, elle n'a donc pas été remplacée.
close-save-failed = L'œuvre n'a pas pu être enregistrée, elle n'a donc pas été fermée : { $error }
close-save-not-started = L'œuvre n'a pas pu être enregistrée, elle n'a donc pas été fermée.
save-not-started = L'œuvre n'a pas pu être enregistrée.

## Notifications
# Titre de notification. Court, car le titre tient sur une ligne et se voit tronqué ; la
# cause va dans le corps, qui est le texte d'erreur non traduit. $file est le nom du
# fichier, sans son chemin.
could-not-open-work = Impossible d'ouvrir « { $file } »
# Un projet enregistré par une version de Skribisto plus récente que celle-ci — titre et
# corps. $written_by est la version qui l'a écrit, $requires la plus ancienne version
# capable de l'ouvrir (les deux diffèrent lorsque la version récente n'a rien utilisé de
# nouveau), $supported la plus récente que cette version comprend.
could-not-open-work-too-new = « { $file } » nécessite une version plus récente de Skribisto
could-not-open-work-too-new-detail = Enregistré au format Skribisto { $written_by } ; son ouverture nécessite le format { $requires } ou plus récent, or cette version ne prend en charge que le format { $supported } au maximum. Mettez Skribisto à jour pour l'ouvrir.
could-not-open-example = Impossible d'ouvrir l'exemple : { $error }
# Le navigateur (ou ce qui traite les liens http) n'a pas pu être lancé pour l'un
# des liens de la barre latérale d'accueil. $url est affichée pour pouvoir être
# copiée malgré tout.
could-not-open-link = Impossible d'ouvrir { $url } : { $error }
could-not-create-work = Impossible de créer l'œuvre : { $error }
saving-as-file = Enregistrement sous { $target }…
saving-as-folder = Enregistrement sous { $target }/…
saved-as = Enregistré dans { $target }
save-error = Impossible d'enregistrer : { $error }
backup-error = Impossible de créer la copie de secours : { $error }
backing-up = Création de la copie de secours…
backup-nothing-open = Aucun projet ouvert à sauvegarder.
backup-already-running = Une copie de secours est déjà en cours.
backup-complete = Copie de secours terminée ({ $ok } enregistrée(s), { $skipped } déjà à jour)
backup-partial = Copie de secours terminée : { $ok } enregistrée(s), échec pour { $failed } destination(s)
backup-no-destination-title = Aucun emplacement de copie de secours disponible
backup-no-destination-text = Aucune des destinations configurées n'est accessible (par exemple, un disque externe peut être débranché). Branchez-le et réessayez, ou quittez sans créer de copie de secours.
backup-failed-close-title = La copie de secours n'a pas pu être enregistrée
backup-failed-close-text = Aucune copie de secours n'a pu être écrite avant la fermeture : toutes les destinations ont échoué (le disque a peut-être été retiré, ou il est plein ou protégé en écriture). Corrigez le problème et réessayez, ou quittez sans créer de copie de secours.

## Ouvrir une copie de secours (modale de choix + bannière permanente + restauration)
backup-choice-title = Copie de secours
backup-choice-heading = Vous avez ouvert une copie de secours
backup-choice-subtitle = Il s'agit d'une copie de secours d'un projet, à un instant donné.
backup-choice-subtitle-dated = Copie de secours du { $date }.
backup-choice-body = Vous pouvez l'ouvrir et la modifier librement, mais les changements ne peuvent être conservés qu'avec « Enregistrer sous ». Le fichier du projet d'origine n'est pas modifié. Ou restaurez ce projet exactement à cette copie de secours.
backup-choice-open = Ouvrir la copie de secours
backup-choice-restore = Restaurer le projet à ce point…
backup-choice-not-a-backup = Non, l'ouvrir normalement
backup-banner-title = Copie de secours : les changements ne peuvent pas être enregistrés ici
backup-banner-description = Utilisez « Enregistrer sous » pour conserver vos modifications dans un nouveau fichier, ou « Restaurer » pour remplacer le projet d'origine par cette copie.
backup-banner-restore = Restaurer…
backup-banner-save-as = Enregistrer sous…
backup-restore-original-missing = Impossible de trouver le projet d'origine à restaurer. Utilisez « Enregistrer sous » pour conserver cette copie comme nouveau projet.
backup-restore-close-elsewhere-title = Projet ouvert dans une autre fenêtre
backup-restore-close-elsewhere-text = Le projet que vous restaurez est ouvert dans une autre fenêtre. Fermez-le d'abord, puis réessayez.
backup-restore-focus-window = Afficher cette fenêtre
backup-restore-confirm-title = Restaurer cette copie de secours ?
backup-restore-confirm-text = La version actuelle du projet sera copiée à côté comme sauvegarde de sécurité avant d'être remplacée par celle-ci.
backup-restore-confirm-ok = Restaurer
backup-restore-error = Impossible de restaurer : { $error }
backup-restored-ok = Projet restauré.
backup-restored-with-safety = Projet restauré. Votre version précédente a été enregistrée dans { $path }.
close-backup-discard-title = Abandonner les modifications de cette copie de secours ?
close-backup-discard-text = Les modifications d'une copie de secours ne peuvent pas y être enregistrées. Utilisez « Enregistrer sous » pour les conserver, ou abandonnez et fermez.
quit-backup-discard-title = Abandonner les modifications et quitter ?
quit-backup-discard-work-question = Abandonner les modifications de { $title } et quitter ?
quit-backup-discard-text = Les modifications d'une copie de secours ne peuvent pas y être enregistrées. Utilisez « Enregistrer sous » pour les conserver, ou abandonnez et quittez.
backup-nudge-text = Aucune copie de secours n'est configurée pour ce projet.
backup-nudge-action = Configurer les copies de secours…

## Panneau de la liste des copies de secours
menu-backups-list = &Liste des copies de secours…
backups-title = Copies de secours
backups-loading = Recherche des copies de secours…
backups-empty = Aucune copie de secours trouvée pour ce projet.
backups-open = Ouvrir
backups-reveal = Afficher
backups-delete = Supprimer cette copie de secours
backups-delete-confirm-title = Supprimer cette copie de secours ?
backups-delete-confirm-text = « { $name } » sera définitivement supprimée. Cette action est irréversible.
backups-delete-error = Impossible de supprimer la copie de secours : { $error }
backups-refresh = Actualiser
backups-close = Fermer

## Panneaux de réglages des copies de secours
settings-page-backup = Copies de secours
settings-page-work-backup = Copies de secours
settings-backup-general-title = Réglages par défaut des copies de secours
settings-backup-work-title = Copies de secours de ce projet
settings-backup-inherit = Utiliser les réglages généraux
settings-backup-inheriting = Ce projet utilise les réglages généraux des copies de secours.
settings-backup-none-hint = Aucune copie de secours automatique n'est configurée (tous les déclencheurs sont désactivés).
settings-backup-last = Dernière copie de secours : { $date }
settings-backup-last-never = Aucune copie de secours pour l'instant.
settings-backup-open-list = Ouvrir la liste des copies de secours…
settings-backup-triggers = Quand créer une copie de secours
settings-backup-on-close = À la fermeture du projet
settings-backup-on-open = À l'ouverture du projet
settings-backup-interval = Régulièrement, toutes les
settings-backup-destinations = Destinations des copies de secours
settings-backup-dest-none = Aucune destination. Les copies sont enregistrées à côté du projet.
settings-backup-dest-remove = Retirer
settings-backup-dest-add = Ajouter un dossier…
settings-backup-dest-refresh = Actualiser
settings-backup-retention = Combien en conserver
settings-backup-retention-tiered = Par paliers
settings-backup-retention-keep-n = Garder les N dernières
settings-backup-retention-tip = Comment les anciennes copies sont supprimées.
settings-backup-retention-tip-more = Le mode par paliers conserve une copie par heure pendant un jour, par jour pendant une semaine, par semaine pendant un mois, et par mois au-delà : l'historique récent reste dense et l'ancien s'éclaircit. « Garder les N dernières » conserve simplement les N copies les plus récentes. Dans les deux modes, les plus récentes (le minimum ci-dessous) sont toujours conservées.
settings-backup-gfs-hourly = Par heure (dernières 24 h)
settings-backup-gfs-daily = Par jour (dernière semaine)
settings-backup-gfs-weekly = Par semaine (dernier mois)
settings-backup-gfs-monthly = Par mois
settings-backup-keep-n = Nombre à conserver
settings-backup-min-keep = Toujours conserver au moins
settings-backup-dedup = Ignorer une copie si rien n'a changé

## Boîte de dialogue Nouvelle œuvre
new-work-title = Nouvelle œuvre
new-work-close = Fermer
new-work-name = Nom de l'œuvre
new-work-name-placeholder = Sans titre
new-work-author = Auteur
new-work-author-placeholder = Facultatif
new-work-format = Format
new-work-single-file = Fichier unique
new-work-bundle = Dossier
new-work-convert-later = Vous pourrez convertir entre les formats plus tard.
new-work-location = Emplacement
new-work-will-create = Va créer
new-work-language = Langue par défaut
new-work-language-hint = Appliquée aux nouveaux textes & à la correction orthographique. Chaque texte peut être basculé vers une autre langue.
new-work-template = Modèle
new-work-template-none = Aucun
new-work-template-empty-novel = Roman vide
new-work-template-light-novel = Roman court
new-work-template-novel = Roman
new-work-template-notebook = Carnet
new-work-cancel = Annuler
new-work-create = Créer l'œuvre
# Descriptions des tuiles de format
new-work-single-file-desc = Une archive .skrib (zip). Portable, facile à sauvegarder.
new-work-bundle-desc = Un dossier contenant chaque texte & ressource. Adapté au contrôle de version.
# Décomptes des modèles
new-work-template-none-count = classeur vide
new-work-template-empty-novel-count = classeurs, sans chapitres
new-work-template-light-novel-count = 15 chapitres
new-work-template-novel-count = 20 chapitres
new-work-template-notebook-count = notes libres
# Bascule ChapterScene (modèles de roman)
new-work-chapter-scene = Chapitres à plat
new-work-chapter-scene-tip = Chaque chapitre est une seule ligne où vous écrivez directement, sans scène en dessous. Laissez désactivé pour la disposition classique : le chapitre est alors un dossier, dans lequel vous écrivez tout autant, mais qui peut aussi contenir des scènes.
new-work-chapter-scene-tip-more = Vous écrivez dans le chapitre dans les deux cas. La seule différence est sa capacité à *contenir* des scènes. L'arborescence du classeur de Skribisto est purement organisationnelle : les deux dispositions produisent le même livre, vous pouvez les mélanger librement, et Promouvoir convertit un chapitre de l'une à l'autre sans perdre un mot.
# Validation des champs
new-work-name-required = Saisissez un nom pour l'œuvre
new-work-name-invalid = Ce nom ne contient aucun caractère utilisable
new-work-location-required = Choisissez un emplacement
new-work-location-missing = Ce dossier n'existe pas
new-work-location-not-folder = Ce chemin n'est pas un dossier
new-work-location-readonly = Ce dossier n'est pas accessible en écriture

## Libellés des modèles de nouvelle œuvre (transmis au backend, qui ne fait pas d'i18n)
new-work-manuscript = Manuscrit
new-work-notes = Notes
new-work-research = Recherche
new-work-notebook = Carnet
new-work-chapter = Chapitre
new-work-scene = Scène
new-work-note = Note
new-work-front-matter = Pages liminaires
new-work-back-matter = Annexes
new-work-paratext = Structure du livre
new-work-paratext-none = Aucune structure
new-work-paratext-hint = Les pages liminaires et les annexes propres à une tradition éditoriale. Vous pourrez tout déplacer, renommer ou supprimer ensuite.

## Boîte de dialogue d'import Plume Creator
import-plume-title = Importer un projet Plume Creator
import-plume-close = Fermer
import-plume-source = Projet Plume
import-plume-source-hint = Choisissez un fichier .plume ou .plume_backup (n'importe quelle version de Plume Creator).
import-plume-location = Dossier de destination
import-plume-name = Nom du fichier
import-plume-name-placeholder = Nom du projet
import-plume-will-create = Créera
import-plume-trash-warning = ⚠ Les éléments à la corbeille / supprimés ne sont pas migrés.
import-plume-cancel = Annuler
import-plume-import = Importer
# Validation des champs
import-plume-source-required = Choisissez un fichier de projet Plume
import-plume-source-missing = Ce fichier n'existe pas
import-plume-source-not-file = Ce chemin n'est pas un fichier
import-plume-location-required = Choisissez un dossier de destination
import-plume-location-missing = Ce dossier n'existe pas
import-plume-location-not-folder = Ce chemin n'est pas un dossier
import-plume-location-readonly = Ce dossier n'est pas accessible en écriture
import-plume-name-required = Saisissez un nom de fichier
import-plume-name-exists = Un fichier de ce nom existe déjà ici. L'import demandera confirmation du remplacement
# Confirmation de remplacement
import-plume-overwrite-title = Remplacer le fichier existant ?
import-plume-overwrite-text = « { $name } » existe déjà. Le remplacer par le projet importé ?
# Noms de classeurs transmis au backend (qui ne fait pas d'i18n)
import-plume-manuscript-binder = Manuscrit
import-plume-story-bible-binder = Personnages et lieux
# Toast de progression (l'import est une opération longue)
import-plume-progress-title = Importation du projet Plume…
import-plume-cancel-import = Annuler
import-plume-cancelled = Importation annulée
# Résultat
import-plume-done = { $imported } éléments importés. { $skipped } éléments à la corbeille n'ont pas été migrés.
import-plume-open-now = Ouvrir maintenant
# Affiché quand l'importateur n'a pas pu tout reprendre à l'identique.
import-plume-warnings = { $count ->
    [one] 1 élément n'a pas pu être importé à l'identique
   *[other] { $count } éléments n'ont pas pu être importés à l'identique
}
import-plume-details = Détails
import-plume-warnings-title = Avertissements d'importation
# Toast d'erreur : motif court dans le corps, chaîne technique complète derrière « Détails »
import-plume-error-title = Impossible d'importer le projet
import-plume-error-details = Détails

## Dialogue d'exportation
export-title = Exporter
export-close = Fermer
export-scope-label = Quoi
export-format-label = Format
export-style-label = Style
export-path-label = Fichier
export-browse = Parcourir…
export-preview-label = Aperçu
export-preview-empty = Rien à prévisualiser pour cette sélection.
export-show-non-exportable = Afficher les non-exportables
export-choose-empty = Aucun projet à sélectionner.
export-cancel = Annuler
export-export = Exporter
export-save-dialog-title = Exporter vers un fichier
# En-têtes de sections du panneau + contrôle segmenté de portée
export-section-what = Quoi exporter
export-section-style = Style
export-section-destination = Destination
export-custom-selection = Sélection personnalisée
export-selected-count = { $count } sélectionné(s)
# En-tête de l'aperçu en direct
export-preview-compiled = compilé
export-preview-live = Aperçu en direct
# Étiquettes récapitulatives du style
export-chip-chapters-none = Chapitres : aucun
export-chip-chapters-numbered = Chapitres : numérotés
export-chip-chapters-title = Chapitres : titre seul
export-chip-chapters-both = Chapitres : numéro + titre
export-chip-scene-break-glyph = Saut de scène : { $glyph }
export-chip-scene-break-blank = Saut de scène : ligne vide
export-chip-scene-break-none = Saut de scène : aucun
export-chip-major-break-glyph = Saut majeur { $glyph }
export-chip-major-break-blank = Saut majeur : ligne blanche
export-chip-major-break-none = Saut majeur : aucun
export-chip-major-break-same = Les deux niveaux identiques
export-chip-spacing-single = Interligne : simple
export-chip-spacing-onehalf = Interligne : 1½
export-chip-spacing-double = Interligne : double
export-chip-notes-included = Notes incluses
export-chip-notes-excluded = Notes exclues
# Formats de sortie
export-format-docx = Word
export-format-html = HTML
export-format-markdown = Markdown
export-format-djot = Djot
export-format-text = Texte
export-format-latex = LaTeX
export-format-epub = EPUB
export-format-pdf = PDF
# Confirmation de remplacement
export-overwrite-title = Remplacer le fichier existant ?
export-overwrite-text = « { $name } » existe déjà. Le remplacer ?
# Toast de progression (l'export est une opération longue)
export-progress-title = Exportation…
export-cancelled = Exportation annulée
export-done = { $count } élément(s) exporté(s)
export-open-file = Ouvrir
export-show-in-folder = Afficher dans le dossier
# Toast d'erreur : motif court dans le corps, chaîne technique complète derrière « Détails »
export-error-title = Impossible d'exporter
export-error-details = Détails

## Planificateur de sauvegardes (toast de progression + détails d'échec/d'avertissement de purge, revue backup, T1-2/T1-7/T2-3/T2-8/T2-9)
backup-progress-start = Démarrage…
backup-progress-retention = Nettoyage des anciennes sauvegardes…
backup-progress-done = Terminé
backup-progress-destination = Destination { $i } sur { $n }
backup-details = Détails
backup-issues-title = Problèmes de sauvegarde
backup-failed-title = Échec de la sauvegarde
backup-complete-prune-warning = Sauvegarde terminée ({ $ok } enregistrée(s), { $skipped } déjà à jour). Certaines anciennes sauvegardes n'ont pas pu être supprimées

# Recherche et remplacement
search = Rechercher
search-query-placeholder = Rechercher…
search-replace-placeholder = Remplacer par…
search-replace-toggle = Afficher le remplacement
search-replace-all = Tout remplacer
search-preserve-case = Conserver la casse
search-opt-case = Respecter la casse
search-opt-whole-word = Mot entier
search-opt-diacritics = Respecter les accents
search-scope-body = Corps
search-scope-title = Titre
search-scope-synopsis = Synopsis
search-scope-label = Étiquette
search-facet-book = Livres
search-facet-part = Parties
search-facet-chapter = Chapitres
search-facet-scene = Scènes
search-facet-note = Notes
search-facet-folder = Dossiers
# Info-bulles détaillées des options
search-tip-case = Respecter la casse : les majuscules et les minuscules sont distinctes, « Elena » et « elena » sont des résultats différents.
search-tip-whole-word = Mot entier : ne trouver que les mots complets ; « chat » n’est pas trouvé dans « château ».
search-tip-diacritics = Respecter les accents : les lettres accentuées sont distinctes ; « cafe » ne trouve pas « café ».
search-tip-body = Corps : rechercher dans la prose des scènes et des notes.
search-tip-title = Titre : rechercher dans les titres des éléments du classeur.
search-tip-synopsis = Synopsis : rechercher dans le résumé de chaque ligne d’écriture.
search-tip-label = Étiquette : rechercher dans la note affichée sous le titre d’un élément.
search-tip-comment = Commentaires : rechercher dans le texte des fils de commentaires et de leurs réponses. Un commentaire porte sur le manuscrit sans en faire partie, d’où son propre bouton — et un remplacement laisse les commentaires décochés tant que vous ne les cochez pas.
search-tip-book = Livres : le conteneur du livre et ses marqueurs de début / fin.
search-tip-part = Parties : les séparateurs de partie.
search-tip-chapter = Chapitres : les chapitres, quel que soit leur stockage.
search-tip-scene = Scènes : les lignes qui contiennent votre prose.
search-tip-note = Notes : les notes libres.
search-tip-folder = Dossiers : les simples dossiers d’organisation et séparateurs.
search-tip-paratext = Un texte qui appartient au livre mais non à son récit — une préface, une dédicace, une postface. Jamais compté dans le manuscrit.
search-tip-replace = Remplacer : afficher le champ de remplacement et « Tout remplacer ».
search-error = Échec de la recherche : { $message }
search-no-matches = Aucun résultat
search-count =
    { $matches ->
        [one] { $matches } résultat
       *[other] { $matches } résultats
    } dans { $items ->
        [one] { $items } document
       *[other] { $items } documents
    }
search-count-truncated =
    { $matches ->
        [one] { $matches } résultat
       *[other] { $matches } résultats
    } dans { $items ->
        [one] { $items } document
       *[other] { $items } documents
    } (premiers résultats seulement)
search-occurrences = ×{ $count }
search-field-body = Corps
search-field-title = Titre
search-field-synopsis = Synopsis
search-field-label = Étiquette
search-field-epigraph = Épigraphe
search-field-comment = Commentaire
search-field-comment-reply = Réponse
search-include-in-replace = Inclure dans « Tout remplacer »
search-replace-nothing = (rien)
search-replace-confirm-title = Remplacer tous les résultats ?
search-replace-confirm-text =
    Remplacer { $occurrences ->
        [one] { $occurrences } occurrence
       *[other] { $occurrences } occurrences
    } de « { $query } » par « { $replacement } » dans { $items ->
        [one] { $items } document
       *[other] { $items } documents
    } ? Vous pourrez annuler depuis la notification.
search-replace-done-title = Remplacement terminé
search-replace-done =
    { $occurrences ->
        [one] { $occurrences } occurrence remplacée
       *[other] { $occurrences } occurrences remplacées
    } dans { $items ->
        [one] { $items } document
       *[other] { $items } documents
    }.
search-replace-done-skipped =
    { $occurrences ->
        [one] { $occurrences } occurrence remplacée
       *[other] { $occurrences } occurrences remplacées
    } dans { $items ->
        [one] { $items } document
       *[other] { $items } documents
    }. { $skipped ->
        [one] { $skipped } champ ignoré
       *[other] { $skipped } champs ignorés
    } (modifiés depuis la recherche).
search-replace-undo = Annuler
search-replace-failed-title = Échec du remplacement
search-replace-undo-failed-title = Échec de l'annulation
search-preview = Aperçu
search-preview-empty = Sélectionnez un résultat pour l'afficher ici
search-preview-no-prose = Ce résultat n'a pas de texte modifiable
search-preview-prompt = Pour afficher un aperçu ici, lancez une recherche.
search-preview-open-search = Rechercher dans le projet

# Barre de recherche par éditeur (Ctrl+F)
find-placeholder = Rechercher dans ce document…
find-count = { $current } sur { $total }
find-no-results = Aucun résultat
find-close = Fermer la recherche
find-previous = Résultat précédent (Maj+Entrée)
find-next = Résultat suivant (Entrée)
find-opt-case = Respecter la casse
find-opt-whole-word = Mot entier
find-replace-toggle = Afficher le remplacement (Ctrl+R)
find-replace-placeholder = Remplacer par…
find-replace = Remplacer
find-replace-all = Tout remplacer
find-preserve-case = Conserver la casse

## Dictionnaires et vérification orthographique
# Notifications de téléchargement (local, pas une opération longue du backend)
dict-download-title = Téléchargement de { $name }…
dict-download-done = { $name } installé
dict-download-failed = Impossible de télécharger { $name } : { $error }
dict-removed = { $name } supprimé
dict-accept-first = Acceptez la licence avant de télécharger { $name }
# Réglages ▸ Dictionnaires
settings-dict-tab-installed = Installés
settings-dict-tab-get-more = En obtenir plus
settings-dict-tab-personal = Mots personnels
dict-installed-empty = Aucun dictionnaire trouvé sur cet ordinateur pour l'instant.
dict-get-more-search = Rechercher une langue
dict-system-badge = sur votre système
dict-unusable-badge = inutilisable
dict-download-button = Télécharger
dict-downloading = Téléchargement…
dict-installed-label = Installé
dict-remove = Supprimer
dict-view-license = Voir la licence
dict-approx-size = ~{ $size }
dict-personal-empty = Aucun mot personnel dans ce projet pour l'instant.
dict-personal-add = Ajouter
dict-personal-placeholder = Ajouter un mot…
# Proposition d'installer les dictionnaires manquants à l'ouverture d'un projet
dict-missing-toast = Ce projet utilise { $count } dictionnaires que vous n'avez pas installés
dict-missing-action = Obtenir les dictionnaires
# Fenêtre de licence
dict-license-title = Licence de { $name }
dict-license-accept = Accepter et télécharger
dict-license-cancel = Annuler
dict-license-close = Fermer
# Ajouter un dictionnaire (fichiers .aff/.dic locaux)
dict-add-button = Ajouter un dictionnaire…
dict-add-title = Ajouter un dictionnaire
dict-add-close = Fermer
dict-add-name = Nom
dict-add-name-placeholder = ex. Mon dictionnaire latin
dict-add-code = Code de langue
dict-add-code-placeholder = ex. la ou fr-FR-x-perso
dict-add-code-hint = Une étiquette courte de votre choix. C'est ce que vous choisirez comme langue d'un document.
dict-add-aff = Fichier d'affixes (.aff)
dict-add-dic = Liste de mots (.dic)
dict-add-submit = Ajouter
dict-add-cancel = Annuler
dict-add-name-required = Donnez un nom au dictionnaire.
dict-add-code-required = Saisissez un code de langue.
dict-add-code-invalid = Utilisez uniquement des lettres, chiffres et - _ .
dict-add-code-reserved = Ce code correspond à un dictionnaire intégré. Choisissez-en un autre.
dict-add-file-required = Choisissez un fichier.
dict-add-file-missing = Ce fichier n'existe pas.
dict-add-code-taken = Un dictionnaire pour ce code est déjà installé.
dict-add-done = { $name } ajouté
dict-add-unusable = Ces fichiers ne sont pas un dictionnaire utilisable : { $error }
dict-add-failed = Impossible d'ajouter le dictionnaire : { $error }
# Les contrôles d'export par élément dans l'Inspecteur (M3)
inspector-export = Export
inspector-exportable = Inclure dans les exports
inspector-apply-to-children = Appliquer aux enfants
# La date de jalon par Partie/Chapitre de l'inspecteur (M5), montrée sur le Rythme du Livre.
inspector-milestone = Date de jalon
inspector-milestone-none = Aucune date
inspector-milestone-clear = Effacer
# Le champ de langues à pastilles (Inspecteur + Réglages)
inspector-tags = Étiquettes
inspector-aliases = Aussi appelé
inspector-dict-language = Langue
inspector-apply-language-to-children = Appliquer la langue aux enfants
settings-page-language = Langue
settings-field-dict-language = Langues
dict-tradeoff-hint = Chaque langue supplémentaire accepte plus de mots, donc moins de fautes sont détectées.
lang-inherit-hint = Hérité : cette scène utilise les langues du livre ou du projet.
lang-pill-list = Langues
lang-pill-add = Ajouter une langue
lang-pill-remove = Retirer { $name }
lang-pill-mute = Désactiver la correction pour { $name }
lang-pill-unmute = Activer la correction pour { $name }

## Réglages: Projet ▸ Dictionnaire personnel (liste de mots par projet)
settings-page-personal-dictionary = Dictionnaire personnel
settings-user-dict-desc = Les mots que vous ajoutez ici sont considérés comme correctement orthographiés partout dans l’application et ne sont plus jamais signalés.
settings-user-dict-add-placeholder = Ajouter un mot…
settings-user-dict-add = Ajouter le mot
settings-user-dict-search = Filtrer les mots
settings-user-dict-count = { $count ->
    [one] { $count } mot
   *[other] { $count } mots
}
settings-user-dict-rename = Renommer
settings-user-dict-remove = Retirer
settings-user-dict-empty = Aucun mot pour l’instant. Ajoutez des mots à ignorer, ou importez une liste.
settings-user-dict-import = Importer…
settings-user-dict-export = Exporter…
settings-user-dict-import-tip = Ajoute les mots d’un fichier texte brut, un mot par ligne. Les mots existants sont conservés.
settings-user-dict-export-tip = Enregistre toute la liste dans un fichier texte brut, un mot par ligne.
settings-user-dict-txt-filter = Fichiers texte
settings-user-dict-duplicate = Déjà dans le dictionnaire
settings-user-dict-imported = { $count } mots importés ({ $duplicates } déjà présents).
settings-user-dict-import-failed = Impossible de lire la liste de mots : { $error }
settings-user-dict-exported = { $count } mots enregistrés.
settings-user-dict-export-failed = Impossible d’enregistrer la liste de mots : { $error }

## Réglages: Projet ▸ Remplacements de texte (lexique personnalisé par projet)
settings-page-text-replacements = Remplacements de texte
settings-text-repl-desc = Remplace une abréviation par le texte complet à la frappe : « stp » devient « s’il te plaît » dès que vous tapez une espace ou une ponctuation.
settings-text-repl-enable = Utiliser les remplacements de texte dans ce projet
settings-text-repl-disabled-hint = Activez cette option pour définir des abréviations qui se développent à l’écriture.
settings-text-repl-add = Ajouter la règle
settings-text-repl-trigger-placeholder = Abréviation
settings-text-repl-replacement-placeholder = Ce qu’elle devient
settings-text-repl-added = « { $trigger } » ajouté
settings-text-repl-duplicate = « { $trigger } » a déjà une règle
settings-text-repl-filter = Filtrer les règles
# `{ $n }` rather than a literal "1" in the [one] branch: French puts zero in the
# `one` category, so a hardcoded numeral renders "1 règle" for an empty lexicon.
settings-text-repl-count = { $n ->
    [one] { $n } règle
   *[other] { $n } règles
}
settings-text-repl-row-enabled = Utiliser cette règle
settings-text-repl-delete = Supprimer la règle de { $trigger }
settings-text-repl-deleted = Règle de « { $trigger } » supprimée
settings-text-repl-empty = Aucune règle pour l’instant.
settings-text-repl-csv-filter = Fichiers CSV
settings-text-repl-import = Importer…
settings-text-repl-export = Exporter…
settings-text-repl-imported = { $added } importées, { $skipped } ignorées
settings-text-repl-exported = { $n ->
    [one] { $n } règle exportée
   *[other] { $n } règles exportées
}

## Éditeur: orthographe (menu contextuel + notification)
# Affiché à la place des corrections quand un mot signalé n’en a aucune.
editor-menu-no-suggestions = Aucune suggestion
editor-menu-add-to-dictionary = Ajouter « { $word } » au dictionnaire
editor-menu-add-words-to-dictionary = Ajouter les mots sélectionnés au dictionnaire
editor-dict-added = « { $word } » ajouté à votre dictionnaire.
editor-dict-added-multi = { $count } mots ajoutés à votre dictionnaire.
toast-undo = Annuler

## Orthographe: l'interrupteur principal (barre de titre / menu Affichage / F7 / Paramètres ▸ Orthographe)
titlebar-spellcheck-on = La vérification orthographique est active. Cliquez pour l'arrêter (F7)
titlebar-spellcheck-off = La vérification orthographique est désactivée. Cliquez pour la réactiver (F7)
menu-spellcheck = &Vérifier l'orthographe
menu-comments = &Commentaires
settings-page-spellcheck = Vérification orthographique
settings-group-spellcheck = Vérification orthographique
settings-spellcheck-enabled = Vérifier l'orthographe pendant que j'écris
settings-spellcheck-hint = Souligne les mots qu'aucun dictionnaire installé ne connaît. Désactiver cette option arrête toute vérification, dans tous les projets, jusqu'à ce que vous la réactiviez. Pour ne cesser de vérifier qu'une seule langue, décochez-la dans le champ Langue de l'œuvre ou d'un élément.

## Panneau de la corbeille
menu-trash = &Corbeille
trash-title = Corbeille
trash-empty-state = La corbeille est vide.
trash-empty-button = Vider la corbeille…
trash-restore = &Restaurer
trash-restore-to = Restaurer &vers…
trash-delete-forever = &Supprimer définitivement
trash-restored-ok = { $count } élément(s) restauré(s).
trash-restore-error = Impossible de restaurer : { $error }
trash-restore-orphaned = L'emplacement d'origine de cet élément n'existe plus — choisissez où le restaurer.
trash-restore-no-project = Aucun projet n'est ouvert, il n'y a donc rien à restaurer.
trash-delete-no-project = Aucun projet n'est ouvert, il n'y a donc rien à supprimer.
trash-empty-confirm-title = Vider la corbeille ?
trash-empty-confirm-text = Les { $count } éléments de la corbeille seront définitivement supprimés. Action irréversible (une courte période de grâce permet d'annuler juste après).
trash-emptied-title = Corbeille vidée
trash-emptied-body = Tout le contenu de la corbeille a été définitivement supprimé.
trash-empty-no-project = Aucun projet n'est ouvert, il n'y a donc aucune corbeille à vider.
trash-delete-forever-confirm-title = Supprimer définitivement ?
trash-delete-forever-confirm-text = { $count } élément(s) seront définitivement supprimés. Action irréversible (une courte période de grâce permet d'annuler juste après).
trash-deleted-title = Supprimé définitivement
trash-deleted-body = { $count } élément(s) définitivement supprimé(s).
trash-undo = Annuler
trash-restore-picker-title = Restaurer vers…
trash-restore-to-confirm-title = Restaurer ici ?
trash-restore-to-confirm-text = Restaurer « { $item } » dans « { $destination } » ?
trash-restore-picker-restore-here = Restaurer ici
trash-restore-picker-cancel = Annuler
trash-restore-picker-empty = Aucun classeur pour l'instant — créez-en un d'abord.
trash-banner-title = Cet élément est dans la corbeille
trash-banner-description = Il n'apparaîtra ni dans le plan ni dans les exports tant que vous ne l'aurez pas restauré.
trash-banner-restore = Restaurer…
trash-tab-tooltip = Dans la corbeille

# Infobulle composite : la fiche complète des paramètres d'un style d'export.
settings-styles-sheet-yes = Oui
settings-styles-sheet-no = Non
settings-styles-sheet-font = Police
settings-styles-sheet-indent = Alinéa
settings-styles-sheet-para-spacing = Espacement des paragraphes
settings-styles-sheet-page = Format de page
settings-styles-sheet-margins = Marges (H/D/B/G)
settings-styles-sheet-title-page = Page de titre
settings-styles-sheet-heading-language = Langue des titres
settings-styles-sheet-auto = Suit le texte
settings-styles-sheet-digits = Chiffres
settings-styles-sheet-direction = Sens du texte
settings-styles-sheet-formats = Formats
settings-styles-sheet-all-formats = Tous

settings-styles-editor-group = Éditeur de style
settings-styles-editor-missing = Ce style n'est plus disponible.

settings-styles-page-letter = Letter
settings-styles-digits-western = Occidentaux (0–9)
settings-styles-digits-eastern-arabic = Arabes orientaux (٠–٩)
settings-styles-direction-ltr = De gauche à droite
settings-styles-direction-rtl = De droite à gauche

## Dock de mise en forme
format-dock-title = Mise en forme
format-panel-empty = Placez le curseur dans une scène, une note ou un synopsis pour voir les options de mise en forme.
# En-têtes de groupe.
format-group-history = Historique
format-group-marks = Texte
format-group-block = Paragraphe
format-group-lists = Listes
format-group-tables = Tableau
format-group-breaks = Sauts de scène
# Infobulles des boutons. Boutons sans libellé : l'infobulle est leur seul nom
# accessible, pas une décoration.
format-undo = Annuler
format-redo = Rétablir
format-superscript = Exposant
format-subscript = Indice
format-clear = Effacer la mise en forme
format-heading = Niveau de titre
format-heading-normal = Texte normal
format-heading-1 = Titre 1
format-heading-2 = Titre 2
format-heading-3 = Titre 3
format-heading-4 = Titre 4
format-heading-5 = Titre 5
format-heading-6 = Titre 6
format-align-left = Aligner à gauche
format-align-center = Centrer
format-direction-rtl = Paragraphe de droite à gauche
format-blockquote = Citation
format-list-bullet = Liste à puces
format-list-numbered = Liste numérotée
format-indent = Augmenter le retrait
format-outdent = Diminuer le retrait
format-table-insert = Insérer un tableau
format-table-row-above = Insérer une ligne au-dessus
format-table-row-below = Insérer une ligne en dessous
format-table-col-before = Insérer une colonne avant
format-table-col-after = Insérer une colonne après
format-table-row-delete = Supprimer la ligne
format-table-col-delete = Supprimer la colonne
format-table-remove = Supprimer le tableau

## Menu : Format
menu-format-marks-bold = &Gras
menu-format-marks-italic = &Italique
menu-format-marks-underline = S&ouligné
menu-format-marks-strike = &Barré
menu-format-marks-superscript = Ex&posant
menu-format-marks-subscript = In&dice
menu-format-marks-clear = &Effacer la mise en forme
menu-format-heading = &Titre
menu-format-heading-normal = Texte &normal
menu-format-heading-1 = Titre &1
menu-format-heading-2 = Titre &2
menu-format-heading-3 = Titre &3
menu-format-heading-4 = Titre &4
menu-format-heading-5 = Titre &5
menu-format-heading-6 = Titre &6
menu-format-alignment = &Alignement
menu-format-align-left = Aligner à &gauche
menu-format-align-center = &Centrer
menu-format-direction = Sens du te&xte
menu-format-direction-auto = &Automatique
menu-format-direction-ltr = De &gauche à droite
menu-format-direction-rtl = De &droite à gauche
menu-format-blockquote = &Citation
menu-format-lists = &Listes
menu-format-list-bullet = Liste à &puces
menu-format-list-numbered = Liste &numérotée
menu-format-indent = &Augmenter le retrait
menu-format-outdent = &Diminuer le retrait
menu-format-table = Tablea&u
menu-format-table-insert = &Insérer un tableau
menu-format-table-2x2 = &2 x 2
menu-format-table-3x3 = &3 x 3
menu-format-table-4x4 = &4 x 4
menu-format-table-row-above = Insérer une ligne au-&dessus
menu-format-table-row-below = Insérer une ligne en des&sous
menu-format-table-col-before = Insérer une colonne a&vant
menu-format-table-col-after = Insérer une colonne a&près
menu-format-table-row-delete = Supprimer la &ligne
menu-format-table-col-delete = Supprimer la &colonne
menu-format-table-remove = Supprimer le &tableau
menu-format-undo = A&nnuler
menu-format-redo = &Rétablir

## Fenêtre « À propos »

menu-about = À &propos de Skribisto…
about-title = À propos de Skribisto
about-version = Version { $version }
about-tagline = Une application d'écriture de romans pour la fiction longue, écrite en Rust avec la boîte à outils Bastyde.
about-license = Distribué sous la Licence publique générale GNU, version 3.
about-copyright = © 2026 Cyril Jacquet
about-close = Fermer

# ── Work ▸ Punctuation — le style typographique du projet ───────────────────
settings-page-punctuation = Ponctuation
settings-group-punctuation = Ponctuation intelligente
settings-punctuation-override = Donner à ce projet ses propres règles de ponctuation
settings-punctuation-override-hint = Désactivé : le projet suit la préférence de l'application. Ces règles voyagent dans le .skrib : un co-auteur qui ouvre le fichier écrit avec la même typographie.
settings-punctuation-dashes = Transformer -- en tiret demi-cadratin, --- en cadratin
settings-punctuation-ellipsis = Transformer ... en points de suspension
settings-punctuation-quotes = Courber les guillemets et les apostrophes
settings-quote-style = Guillemets
settings-quote-style-locale = Défaut de la langue
settings-quote-style-curly = « Courbes » (“…”)
settings-quote-style-guillemets = «Chevrons»
settings-quote-style-low-high = „Bas-haut“
settings-punctuation-spacing = Espace avant ; : ! ?
settings-punctuation-spacing-hint = La typographie française place une espace fine insécable avant ; ! ? et une espace insécable avant :. Ne s'applique qu'au texte écrit en français. L'espace à l'intérieur des guillemets « » accompagne les guillemets eux-mêmes.
settings-punctuation-sample = Votre langue donne
settings-punctuation-app-hint = Ce que fait chaque projet, sauf s'il adopte ses propres règles dans Projet ▸ Ponctuation.
settings-punctuation-dialogue = Ouvrir un paragraphe saisi « - » par un tiret de dialogue
settings-punctuation-dialogue-hint = Pour les langues qui marquent le dialogue par un tiret plutôt que par des guillemets — français, espagnol, russe et d'autres. Ne se déclenche qu'en tout début de paragraphe.

## Aller à (rejoindre n'importe quel élément)
statusbar-go-to = Aller à…
go-to-placeholder = Rechercher dans le classeur
go-to-no-matches = Aucun élément ne correspond à cette recherche.
menu-go-to = &Aller à…

# ── Thèmes sans distraction (Réglages ▸ Éditeur ▸ Thèmes sans distraction) ──
settings-page-distraction-free-themes = Thèmes sans distraction
settings-themes-builtin = Thèmes fournis
settings-themes-builtin-badge = Fourni
settings-themes-user = Mes thèmes
settings-themes-editor-group = Modifier le thème
settings-themes-editor-empty = Choisissez un thème sous « Mes thèmes » pour le modifier.
settings-themes-use = Utiliser
settings-themes-duplicate = Dupliquer
settings-themes-edit = Modifier
settings-themes-delete = Supprimer
settings-themes-export = Exporter…
settings-themes-import = Importer…
settings-themes-copy-suffix = copie
settings-themes-json-filter = Thème (JSON)
settings-themes-imported = Thème importé
settings-themes-import-failed = Impossible d'importer ce thème
settings-themes-exported = Thème exporté
settings-themes-export-failed = Impossible d'exporter ce thème
# Affiché pour un thème dont le texte et la page sont sous le seuil WCAG AA.
settings-themes-low-contrast = contraste faible
settings-themes-field-name = Nom
settings-themes-field-paper = Page
settings-themes-field-ink = Texte
settings-themes-field-general = Arrière-plan
settings-themes-field-widget-text = Texte de la bande de contrôle

# L'engrenage de réglages rapides de la bande sans distraction, et l'accès à la
# bibliothèque complète de thèmes depuis le mode.
statusbar-focus-settings = Réglages du mode sans distraction
statusbar-focus-manage-themes = Gérer les thèmes…

## Commentaires — les deux panneaux, les fiches de fil et leurs actions.
comments-title = Commentaires
comments-document-title = Ce document
comments-empty-project = Aucun commentaire dans ce projet.
comments-empty-document = Aucun commentaire sur ce document.
comments-filter-all = Tous
comments-filter-open = Ouverts
comments-filter-resolved = Résolus
comments-filter-orphaned = Orphelins
comments-status-open = Ouvert
comments-status-resolved = Résolu
comments-status-orphaned = Texte introuvable
comments-orphan-snippet = (le texte commenté a disparu)
comments-reply-count = { $count } réponses
comments-menu-resolve = Résoudre
comments-menu-reopen = Rouvrir
comments-menu-delete = Supprimer
comments-sort-document = Dans l'ordre du document
comments-sort-newest = Les plus récents d'abord
comments-menu-add = Ajouter un commentaire
comments-menu-add-paragraph = Commenter ce paragraphe
overview-col-comments = Commentaires
overview-col-total-comments = Total commentaires
comments-card-placeholder = Écrire un commentaire…
comments-card-unknown-author = Auteur inconnu
comments-card-reply = Répondre
comments-card-reply-placeholder = Répondre…
comments-card-actions = Actions du commentaire
comments-reply-actions = Actions de la réponse
comments-menu-delete-reply = Supprimer la réponse
comments-menu-delete-all = Supprimer tous les commentaires ici
comments-deleted-toast = Commentaire supprimé
comments-reply-deleted-toast = Réponse supprimée
comments-deleted-all-toast = { $count } commentaires supprimés
comments-undo = Annuler

# ── Analyse (segment du conteneur Livre) ─────────────────────────────────────
analysis-segment = Analyse
analysis-scope-book = Analyse de ce livre
analysis-run = Lancer l'analyse
analysis-stale = Modifié depuis cette analyse
analysis-not-run = Pas encore analysé.
analysis-running = Lecture du manuscrit…
analysis-failed = L'analyse n'a pas pu aboutir.
analysis-no-scenes = Aucune scène dans ce livre pour l'instant.

analysis-shape = Forme
analysis-repetition = Répétitions
analysis-synopsis = Synopsis
analysis-voice = Voix

analysis-words-per-scene = Mots par scène
analysis-median-words = La scène médiane de ce livre compte { $count } mots.
analysis-median-line = Médiane : { $count } mots
analysis-dialogue = Dialogue
analysis-dialogue-unsupported = Le dialogue n'est pas encore mesuré pour cette langue.

analysis-echoes = Mots répétés
analysis-echoes-explainer = Les mots distinctifs employés deux fois à moins d'une page d'intervalle, la paire la plus rapprochée d'abord. Les mots courants sont écartés. Une répétition n'est pas une faute : c'est seulement là qu'un lecteur risque de l'entendre. Choisissez une scène pour l'ouvrir.
analysis-no-echoes = Aucun mot ne se répète d'assez près pour ressortir.
analysis-echo-row = « { $word } » apparaît { $count } fois, à { $gap } mots d'intervalle au plus proche.
analysis-repetition-text-tooltip =
    { $count ->
        [one] Un mot distinctif se répète de près dans ce texte. Cliquez pour l'ouvrir.
       *[other] { $count } mots distinctifs se répètent de près dans ce texte. Cliquez pour l'ouvrir.
    }
analysis-repetition-word-tooltip = { $count } emplois de « { $word } » sont assez rapprochés pour s'entendre comme une répétition — pas forcément toutes ses apparitions ici. Les deux plus proches sont à { $gap } mots d'intervalle, ce qui détermine si un lecteur la remarque.
analysis-repetition-gap-short = { $gap } m
analysis-similar-scenes = Scènes similaires
analysis-similar-explainer = Deux scènes qui partagent de longs passages de formulation identique. En général une scène copiée puis retouchée, ou coupée en deux sans jamais diverger.
analysis-no-similar-scenes = Aucune paire de scènes ne partage de longs passages.
analysis-similar-row = « { $a } » et « { $b } » partagent environ { $percent } % du texte de la plus courte.
analysis-more-rows = { $count } de plus non affichées.

analysis-ignore-empty = Ignorer les textes encore vides
analysis-empty-hidden = { $count } { $count ->
        [one] texte vide masqué
       *[other] textes vides masqués
    }.
analysis-all-texts-empty = Tous les textes de ce livre sont encore vides.

analysis-synopsis-drift = Synopsis et texte
analysis-no-synopses = Aucun synopsis écrit pour l'instant, il n'y a donc rien à comparer.
analysis-no-drift = Chaque synopsis suit sa scène d'aussi près que les autres.
analysis-drift-row = « { $title } » — son synopsis mentionne { $terms }, mais pas le texte.

analysis-vocabulary = Vocabulaire
analysis-vocabulary-explainer = À quel point la formulation varie, mesurée sur une fenêtre glissante afin qu'un livre long ne soit pas mieux noté du seul fait de sa longueur. La mesure décrit l'écriture, elle ne la juge pas : une prose sobre obtient un score plus bas qu'une prose ornée par construction, et aucune des deux ne vaut mieux que l'autre.
analysis-words-measured = { $words } mots, dont { $distinct } distincts.
analysis-mattr = Variété du vocabulaire : { $value }
analysis-mattr-scale = 0 correspondrait à un seul mot répété sans fin ; 1 à un livre ne réutilisant jamais un mot. La prose réelle se tient loin de ces extrêmes, et il n'y a aucune valeur à viser.
analysis-not-enough-text = Pas encore assez de texte pour mesurer cela.
analysis-vocabulary-caveat = Comparable au sein de ce livre uniquement.

# ── Retour du filtre du classeur ─────────────────────────────────────────────
binder-filter-count = { $shown } sur { $total } affichés
binder-filter-clear = Effacer
binder-filter-none = Rien ne correspond à « { $query } ».

## Settings: paratext structures
settings-paratext-intro = Les pages liminaires et les annexes d'un nouveau projet. Chaque structure appartient à une tradition éditoriale, et ses titres sont écrits dans la langue de cette tradition — renommez-les librement une fois le projet créé.
settings-paratext-structures = Structures
settings-paratext-broken = Lecture impossible
settings-paratext-edit = Modifier
settings-paratext-duplicate = Dupliquer
settings-paratext-delete = Supprimer
settings-paratext-new = Nouvelle structure
settings-paratext-save = Enregistrer
settings-paratext-editor-hint = Un nom, les pages à créer avant le manuscrit, et celles à créer après. Leur place vous appartient : ce sont des éléments ordinaires une fois le projet créé.
