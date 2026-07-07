# Skribisto — chaînes de l'interface (français).
# Un simple « & » marque le mnémonique d'un libellé de menu ; « && » est un « & » littéral.

## Barre de menus — Fichier
menu-file = &Fichier
menu-new-work = &Nouvelle œuvre
menu-open-work = &Ouvrir une œuvre…
menu-import-from = &Importer depuis
menu-import-plume = &Plume Creator (.plume)…
menu-save = &Enregistrer
menu-save-as-file = Enregistrer comme fichier &unique…
menu-save-as-folder = Enregistrer comme &dossier…
menu-backup = &Sauvegarder maintenant
menu-close-work = &Fermer l'œuvre
menu-welcome = &Bienvenue…
menu-settings = &Paramètres
menu-quit = &Quitter

## Barre de menus — Affichage
menu-view = &Affichage
menu-outline = &Plan

## Menu contextuel du classeur
ctx-new-item = &Nouvel élément
ctx-new-folder = Nouveau d&ossier
ctx-rename = &Renommer
ctx-duplicate = &Dupliquer
ctx-trash = Mettre à la &corbeille

## Paramètres
language = Langue
theme = Thème
english = Anglais
french = Français
light = Clair
dark = Sombre
settings-text-width = Largeur du texte
settings-autosave = Enregistrement automatique sur le disque
settings-show-welcome = Afficher l'écran d'accueil au démarrage

## Fenêtre des paramètres — cadre
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

## Fenêtre des paramètres — catégories
settings-sec-appearance-behaviour = Apparence et comportement
settings-sec-editor = Éditeur
settings-sec-spelling = Orthographe
settings-sec-backup = Sauvegarde et synchronisation
settings-sec-compile = Compilation et export
settings-page-appearance = Apparence
settings-page-menus = Menus et barres d'outils
settings-page-notifications = Notifications
settings-page-manuscript = Manuscrit et polices
settings-page-goals = Objectifs et nombre de mots
settings-page-corkboard = Tableau de liège
settings-page-dictionaries = Dictionnaires
settings-page-autosave = Enregistrement automatique
settings-page-export = Formats d'export
settings-page-keymap = Raccourcis clavier

## Fenêtre des paramètres — champs
settings-group-manuscript-font = Police du manuscrit
settings-group-writing-column = Colonne d'écriture
settings-group-theme = Thème
settings-group-language = Langue
settings-group-startup = Démarrage
settings-group-autosave = Enregistrement automatique
settings-field-typeface = Police
settings-field-size = Taille du texte
settings-field-line-height = Interligne
settings-field-editor-theme = Thème de l'éditeur
settings-field-language = Langue de l'interface
settings-synopsis-pane = Afficher le synopsis au-dessus du manuscrit
settings-typewriter = Défilement machine à écrire (garder la ligne du curseur centrée)
settings-highlight-sentence = Surligner la phrase courante
settings-autosave-hint = Les modifications sont enregistrées automatiquement au fil de l'écriture.

## Accueil
welcome-title = Bienvenue dans Skribisto
welcome-close = Fermer
welcome-search = Rechercher des œuvres
welcome-open = Ouvrir
welcome-new-work = Nouvelle œuvre
welcome-recent-works = Œuvres récentes
welcome-empty-recents = Aucune œuvre récente.
welcome-learn-soon = Guides et astuces à venir.
welcome-about-blurb = Skribisto — une réécriture en Rust + Bastyde de l'application d'écriture.
welcome-tagline = Un endroit calme pour écrire de longs textes.
welcome-show-at-startup = Afficher au démarrage
nav-works = Œuvres
nav-examples = Exemples
nav-learn = Apprendre
nav-about = À propos

## Onglets & volets d'édition
synopsis = Synopsis
corkboard = Tableau d'affichage
overview = Aperçu
text-heading = Texte
no-content = Cet élément n'a pas de contenu modifiable.
untitled = Sans titre
placeholder-title = Titre…
placeholder-subtitle = Sous-titre…
placeholder-chapter-title = Titre du chapitre…

## Classeur / récents
binder = Classeur
no-work = Aucune œuvre
no-recent-works = Aucune œuvre récente

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
close-work-question = Enregistrer les modifications avant de fermer l'œuvre ?
close-question = Enregistrer les modifications avant de fermer ?
unsaved-changes = Cette œuvre a des modifications non enregistrées.
tooltip-welcome = Accueil

## Notifications
could-not-open-work = Impossible d'ouvrir l'œuvre : { $error }
could-not-open-example = Impossible d'ouvrir l'exemple : { $error }
could-not-create-work = Impossible de créer l'œuvre : { $error }
saving-as-file = Enregistrement sous { $target }…
saving-as-folder = Enregistrement sous { $target }/…
save-error = Impossible d'enregistrer : { $error }
backup-error = Impossible de sauvegarder : { $error }
backing-up = Sauvegarde en cours…

## Boîte de dialogue Nouvelle œuvre
new-work-title = Nouvelle œuvre
new-work-close = Fermer
new-work-name = Nom de l'œuvre
new-work-name-placeholder = Sans titre
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
new-work-chapter-scene = Écrire directement dans les chapitres
new-work-chapter-scene-tip = Chaque chapitre devient une page unique où vous écrivez directement (un *ChapterScene*). Laissez désactivé pour la disposition classique — chaque chapitre est un dossier contenant une scène vide, préférable quand un chapitre compte plusieurs scènes.
new-work-chapter-scene-tip-more = L'arborescence du classeur de Skribisto est purement organisationnelle : les deux dispositions produisent le même livre. Un *dossier* de chapitre est défini par les scènes qu'il contient ; un *ChapterScene* est l'équivalent à plat qui ouvre le chapitre et porte son texte en une seule ligne. Vous pouvez librement mélanger les deux par la suite.
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
import-plume-name-exists = Un fichier de ce nom existe déjà ici — l'import demandera confirmation du remplacement
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
# Toast d'erreur : motif court dans le corps, chaîne technique complète derrière « Détails »
import-plume-error-title = Impossible d'importer le projet
import-plume-error-details = Détails
