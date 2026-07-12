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
menu-backup = Créer une &copie de secours
menu-close-work = &Fermer l'œuvre
menu-welcome = &Bienvenue…
menu-settings = &Paramètres
menu-quit = &Quitter

## Barre de menus — Affichage
menu-view = &Affichage
menu-outline = &Plan

## Menu contextuel du classeur
ctx-add = &Ajouter
ctx-new-item = &Nouvel élément
ctx-new-folder = Nouveau d&ossier
ctx-rename = &Renommer
ctx-duplicate = &Dupliquer
ctx-trash = Mettre à la &corbeille
ctx-open-to-side = Ouvrir &sur le côté

## Recommandations de création — libellés de types (titre du SplitButton + Ajouter ▸)
create-book = Livre
create-part = Partie
create-chapter = Chapitre
create-scene = Scène
create-note = Note
create-note-folder = Dossier de notes
create-folder = Dossier
create-book-end = Fin du livre

## Recommandations de création — infobulles enrichies ({ $kind } = type, { $target } = titre de l’ancre)
create-tooltip-child = Ajouter un(e) { $kind } dans « { $target } ».
create-tooltip-sibling = Ajouter un(e) { $kind } après « { $target } ».
create-tooltip-parent-sibling = Ajouter un(e) { $kind } après le conteneur « { $target } ».
create-tooltip-top = Ajouter un(e) { $kind } au niveau supérieur.

## Promouvoir — convertir un élément du classeur vers son type apparié
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
promote-blocked-title = Chapitre non vide
promote-blocked-text = Ce chapitre contient encore { $count } élément(s). Déplacez-les ou mettez-les à la corbeille avant de le convertir en chapitre à plat.

## Inspecteur (dock de droite) + bascules de docks dans la barre d'état
inspector = Inspecteur
inspector-empty = Ouvrez un élément pour l'inspecter.
inspector-promote = Convertir en…
statusbar-toggle-outline = Afficher/masquer le classeur
statusbar-toggle-inspector = Afficher/masquer l'inspecteur

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
settings-page-scene = Scène
settings-page-synopsis = Synopsis
settings-page-notes = Notes
settings-page-editor-behavior = Comportement de l'éditeur
settings-page-goals = Objectifs et nombre de mots
settings-page-corkboard = Tableau de liège
settings-page-dictionaries = Dictionnaires
settings-page-autosave = Enregistrement automatique
settings-page-export = Formats d'export
settings-page-keymap = Raccourcis clavier

## Fenêtre des paramètres — champs
settings-group-typography = Typographie
settings-group-writing-column = Colonne d'écriture
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
settings-field-app-theme = Thème
settings-field-text-scale = Taille du texte de l'interface
settings-field-language = Langue de l'interface
settings-synopsis-pane = Afficher le synopsis au-dessus du manuscrit
settings-typewriter = Défilement machine à écrire (garder la ligne du curseur centrée)
settings-highlight-sentence = Surligner la phrase courante
settings-autosave-hint = Les modifications sont enregistrées automatiquement au fil de l'écriture.

## Paramètres — Œuvre (le projet ouvert)
settings-sec-work = Œuvre
settings-page-structure = Structure
settings-group-chapters = Chapitres
settings-chapter-flat = Chapitres à plat
settings-chapter-flat-hint = Activé : un chapitre est une seule ligne. Vous y écrivez, et il ne contient aucune scène. Désactivé : un chapitre est un dossier. Vous y écrivez également, mais il peut en outre contenir des scènes. Les nouveaux chapitres suivent ce réglage ; les existants se convertissent via Promouvoir.

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
close-question = Enregistrer les modifications avant de fermer ?
unsaved-changes = Cette œuvre a des modifications non enregistrées.
tooltip-welcome = Accueil

## Notifications
could-not-open-work = Impossible d'ouvrir l'œuvre : { $error }
could-not-open-example = Impossible d'ouvrir l'exemple : { $error }
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
backup-choice-body = Vous pouvez l'ouvrir et la modifier librement, mais les changements ne peuvent être conservés qu'avec « Enregistrer sous » — le fichier du projet d'origine n'est pas modifié. Ou restaurez ce projet exactement à cette copie de secours.
backup-choice-open = Ouvrir la copie de secours
backup-choice-restore = Restaurer le projet à ce point…
backup-choice-not-a-backup = Non, l'ouvrir normalement
backup-banner-title = Copie de secours — les changements ne peuvent pas être enregistrés ici
backup-banner-description = Utilisez « Enregistrer sous » pour conserver vos modifications dans un nouveau fichier, ou « Restaurer » pour remplacer le projet d'origine par cette copie.
backup-banner-restore = Restaurer…
backup-banner-save-as = Enregistrer sous…
restore-original-missing = Impossible de trouver le projet d'origine à restaurer. Utilisez « Enregistrer sous » pour conserver cette copie comme nouveau projet.
restore-close-elsewhere-title = Projet ouvert dans une autre fenêtre
restore-close-elsewhere-text = Le projet que vous restaurez est ouvert dans une autre fenêtre. Fermez-le d'abord, puis réessayez.
restore-focus-window = Afficher cette fenêtre
restore-confirm-title = Restaurer cette copie de secours ?
restore-confirm-text = La version actuelle du projet sera copiée à côté comme sauvegarde de sécurité avant d'être remplacée par celle-ci.
restore-confirm-ok = Restaurer
restore-error = Impossible de restaurer : { $error }
restored-ok = Projet restauré.
restored-with-safety = Projet restauré. Votre version précédente a été enregistrée dans { $path }.
close-backup-discard-title = Abandonner les modifications de cette copie de secours ?
close-backup-discard-text = Les modifications d'une copie de secours ne peuvent pas y être enregistrées. Utilisez « Enregistrer sous » pour les conserver, ou abandonnez et fermez.
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
settings-backup-dest-none = Aucune destination — les copies sont enregistrées à côté du projet.
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

## Planificateur de sauvegardes (toast de progression + détails d'échec/d'avertissement de purge — revue backup, T1-2/T1-7/T2-3/T2-8/T2-9)
backup-progress-start = Démarrage…
backup-progress-retention = Nettoyage des anciennes sauvegardes…
backup-progress-done = Terminé
backup-progress-destination = Destination { $i } sur { $n }
backup-details = Détails
backup-issues-title = Problèmes de sauvegarde
backup-failed-title = Échec de la sauvegarde
backup-complete-prune-warning = Sauvegarde terminée ({ $ok } enregistrée(s), { $skipped } déjà à jour) — certaines anciennes sauvegardes n'ont pas pu être supprimées
