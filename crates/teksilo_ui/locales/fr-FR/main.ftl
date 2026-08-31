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
menu-create-from = &Créer depuis
menu-import-plume = &Plume Creator (.plume)…
menu-import-document = &Documents (Markdown, Word, ODT)…
menu-import-manuskript = &Manuskript (.msk)…
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
menu-backup = Créer une copie de &secours
menu-close-work = &Fermer l’œuvre
menu-welcome = &Bienvenue…
menu-settings = &Paramètres
menu-quit = &Quitter

## Barre de menus: Affichage
menu-view = &Affichage
menu-outline = &Plan
menu-search = &Rechercher dans le projet
menu-timeline = R&emonter le temps
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
ctx-import-here = &Importer ici…
ctx-indent = &Indenter
ctx-outdent = Désinden&ter
ctx-trash = Mettre à la &corbeille
ctx-open-to-side = Ouvrir &sur le côté
ctx-open = &Ouvrir
ctx-reveal-in-outline = Afficher dans le &plan
ctx-move-up = Déplacer vers le &haut
ctx-move-down = Déplacer vers le &bas

## Menu contextuel des onglets de l'éditeur
# Des clés `ctx-tab-*` neuves, jamais une réutilisation des lignes `ctx-*`
# ci-dessus : celles-ci servent aussi de lignes de la barre de menus, si bien que
# retoucher l'un de leurs mnémoniques pour ce menu casserait l'unicité de la barre.
# Trois lignes sont exclusives deux à deux — la division, le déplacement et
# l'épinglage n'en construisent qu'une chacun — d'où des lettres qui peuvent se
# répéter d'une paire à l'autre, mais jamais dans un même menu construit.
ctx-tab-close = &Fermer
ctx-tab-close-others = Fermer les &autres
ctx-tab-close-all = Fermer &tout
# Les lignes de division se lisent depuis le volet où se trouve déjà l'onglet, et
# elles dupliquent : l'élément finit ouvert dans les deux volets sur un seul et
# même document partagé.
ctx-tab-open-to-side = Ouvrir &sur le côté
ctx-tab-open-in-main = Ouvrir dans le volet &principal
# Les lignes de déplacement se lisent pareil, mais l'onglet quitte son volet.
ctx-tab-move-to-side = &Déplacer sur le côté
ctx-tab-move-to-main = &Déplacer dans le volet principal
ctx-tab-move-to-new-window = Déplacer dans une &nouvelle fenêtre
# Pourquoi cette dernière ligne est indisponible. La seconde fenêtre s'ouvre sur
# le fichier du projet : un projet jamais enregistré n'a donc rien à lui ouvrir.
# Infobulle de la ligne désactivée, pas un libellé de menu, donc aucun mnémonique.
ctx-tab-move-window-unsaved = Enregistrez d’abord ce projet — une seconde fenêtre s’ouvre sur un fichier du disque
ctx-tab-pin = Épin&gler cet onglet
ctx-tab-unpin = Désépin&gler cet onglet
# Infobulle de l'onglet, pas une ligne de menu : aucun mnémonique.
tab-pinned-tooltip = Épinglé — « Fermer les autres » et « Fermer tout » le laissent ouvert

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
create-story-bible-entry = Entrée de bible narrative…
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
new-item-story-bible-entry = Nouvelle entrée de bible narrative

## Recommandations de création: indication de placement en fin de ligne
placement-inside = à l’intérieur
placement-after = après
placement-after-parent = après le parent
placement-top-level = au niveau supérieur

# (Les infobulles enrichies du modèle d’écriture sont dans le tooltips.ftl de cette locale.)

## Promouvoir: convertir un élément du classeur vers son type apparié
ctx-promote = Con&vertir en
promote-chapter-folder = Dossier de chapitre
promote-flat-chapter = Chapitre à plat
promote-lossy-title = Aucun endroit pour ce texte
promote-lossy-text = Le type « { $target } » n’a nulle part où conserver ceci : { $kinds }. Déplacez ou effacez ce texte, puis convertissez.
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
promote-blocked-text = { $count ->
    [one] Ce chapitre contient encore { $count } élément. Déplacez-le ou mettez-le à la corbeille avant de le convertir en chapitre à plat.
   *[other] Ce chapitre contient encore { $count } éléments. Déplacez-les ou mettez-les à la corbeille avant de le convertir en chapitre à plat.
}

## Inspecteur (dock de droite) + bascules de docks dans la barre d'état
inspector = Inspecteur
inspector-empty = Ouvrez un élément pour l’inspecter.
inspector-promote = Convertir en…
statusbar-toggle-outline = Afficher/masquer le classeur
statusbar-toggle-inspector = Afficher/masquer l’inspecteur
# L'indicateur d'enregistrement (barre d'état, à côté de la bascule du classeur).
statusbar-save-unsaved = Modifications non enregistrées. Cliquez pour enregistrer
statusbar-save-saved = Toutes les modifications sont enregistrées
statusbar-save-autosave = L’enregistrement automatique est activé. Les modifications sont enregistrées au fil de l’écriture
statusbar-saving = Enregistrement…
# Le nombre de mots en direct de l'élément ciblé (barre d'état).
statusbar-word-count = { $count ->
    [one] { $count } mot
   *[other] { $count } mots
}
statusbar-word-count-tooltip = Mots dans la scène en cours d’édition
# Mots + caractères, quand « Afficher les caractères » est activé (Paramètres ▸ Objectifs).
statusbar-word-char-count = { $words ->
    [one] { $words } mot
   *[other] { $words } mots
} · { $chars ->
    [one] { $chars } caractère
   *[other] { $chars } caractères
}
# La session d'écriture (minuteur de sprint + compteur de mots, barre d'état).
session-toggle = Session d’écriture : démarrer ou mettre en pause un sprint
session-configure = Définir l’objectif de mots et la limite de temps
session-configure-title = Session d’écriture
session-word-goal = Objectif de mots
session-time-limit = Limite de temps
session-time-limit-unit = min
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
settings-preview-width = Largeur de l’aperçu de recherche
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
settings-reset-confirm-title = Réinitialiser tous les paramètres ?
settings-reset-confirm-body = Rétablit l’apparence et les réglages d’écriture de l’application. Les paramètres propres à votre projet, les raccourcis clavier et les styles enregistrés ne sont pas touchés. Cette action est irréversible.
settings-empty-title = Aucun paramètre ici pour l’instant
settings-empty-hint = Cette section proposera des options dans une prochaine mise à jour.

## Fenêtre des paramètres: catégories
settings-sec-appearance-behaviour = Apparence et comportement
settings-sec-editor = Éditeur
settings-sec-spelling = Orthographe
settings-sec-backup = Sauvegardes et synchro
settings-sec-compile = Compilation et export
settings-page-appearance = Apparence
settings-field-image-size-policy = Grandes images
settings-image-policy-ask = Demander à chaque fois
settings-image-policy-keep = Conserver l’original
settings-image-policy-downscale = Optimiser
settings-hint-image-size-policy = Que faire quand une image insérée est plus grande que nécessaire.
settings-page-notifications = Notifications
settings-page-scene = Scène
settings-page-synopsis = Synopsis
settings-page-notes = Notes
settings-page-editor-behavior = Comportement de l’éditeur
settings-page-goals = Objectifs et nombre de mots
settings-page-games = Jeux d’écriture
settings-page-corkboard = Tableau de liège
settings-page-distraction-free = Sans distraction
settings-page-dictionaries = Dictionnaires
settings-page-autosave = Enregistrement automatique
settings-page-export = Formats d’export
settings-page-paratext = Structures de paratexte
settings-page-keymap = Raccourcis clavier
# Champ de filtre de la page Raccourcis (filtre la liste ShortcutSettings par nom / id / catégorie).
settings-keymap-filter = Filtrer les raccourcis

## Fenêtre des paramètres: à quoi sert chaque page
# Une ligne par page, affichée sous son lien dans la page de son parent — et,
# pour un parent, sous son propre titre. Une seule ligne : elles disent ce que
# la page contient, pas comment s'en servir.
settings-desc-sec-appearance-behaviour = L’aspect de l’application elle-même, et ce qu’elle fait au démarrage.
settings-desc-sec-editor = La surface d’écriture : son aspect, et ce qu’elle fait pendant que vous tapez.
settings-desc-sec-spelling = La vérification orthographique et les dictionnaires qui la nourrissent.
settings-desc-sec-backup = Comment votre travail atteint le disque, et quelles copies sont conservées.
settings-desc-sec-compile = Ce qui sort de Skribisto, et sous quelle forme.
settings-desc-sec-work = Les paramètres propres à ce projet, qui voyagent dans son fichier.
settings-desc-sec-extensions = Les pages ajoutées par les extensions installées ici.
settings-desc-group-typography = Une page par type de texte — police, taille, interligne et espacements.
settings-desc-appearance = Langue de l’interface, thème, taille du texte et écran d’accueil au démarrage.
settings-desc-notifications = Tous les messages affichés pendant cette session, et les actions à rejouer.
settings-desc-scene = La mise en page des scènes : police, taille, interligne, retraits et espacements.
settings-desc-synopsis = La mise en page du volet synopsis, indépendante du manuscrit.
settings-desc-notes = La mise en page des notes, indépendante du manuscrit.
settings-desc-corkboard = La taille des fiches, ce qu’elles montrent, et la disposition du tableau.
settings-desc-distraction-free = Typographie, largeur de colonne et bande de contrôle du plein écran.
settings-desc-distraction-free-themes = La bibliothèque de thèmes du plein écran — ceux fournis et les vôtres.
settings-desc-editor-behavior = Largeur du texte, place du synopsis, défilement machine à écrire et surlignage du curseur.
settings-desc-punctuation = Le style typographique dont héritent les nouveaux projets. Chaque projet suit ces réglages, sauf s’il définit les siens.
settings-desc-goals = Les objectifs en mots et en caractères, et la façon de compter les mots.
settings-desc-games = Les contraintes que vous vous imposez en écrivant, comme Toujours en avant.
settings-desc-spellcheck = L’interrupteur unique qui active ou coupe la vérification orthographique partout.
settings-desc-dictionaries = Installer, retirer et parcourir les dictionnaires présents sur cette machine.
settings-desc-autosave = Si vos modifications sont écrites sur le disque toutes seules.
settings-desc-backup = Quand les copies sont prises, où elles sont rangées, et combien sont gardées. Chaque projet suit ces réglages, sauf s’il définit les siens.
settings-desc-export = Les styles par lesquels passe chaque export — ceux fournis et les vôtres.
settings-desc-paratext = Les pages liminaires et finales dont peut partir un nouveau projet.
settings-desc-keymap = Tous les raccourcis, et ce à quoi ils sont attachés.
settings-page-user = Utilisateur
settings-desc-user = Qui vous êtes, pour les commentaires que vous écrivez.
settings-group-identity = Identité
settings-field-user-name = Votre nom
settings-field-user-name-placeholder = Facultatif
settings-field-user-name-hint = Signe les commentaires et les réponses que vous écrivez. À distinguer du nom d’auteur d’un projet, qui est la signature du livre et voyage dans le fichier.
settings-field-user-initials = Vos initiales
settings-field-user-initials-placeholder = D’après votre nom
settings-field-user-initials-hint = Ce qu’un traitement de texte affiche en marge à côté de votre commentaire. Laissez vide pour utiliser celles affichées, déduites de votre nom.
settings-field-user-hint = Les deux sont facultatifs et valent pour tous les projets de cet ordinateur. Les modifier signe les prochains commentaires que vous écrirez — ceux déjà écrits gardent le nom sous lequel ils l’ont été.
settings-desc-author = Le nom qui figure sur ce projet.
settings-desc-structure = Si les chapitres de ce projet sont des dossiers ou des éléments simples.
settings-desc-language = La langue dans laquelle la prose de ce projet est vérifiée.
settings-desc-work-backup = La politique de copies propre à ce projet, ou celle par défaut.
settings-desc-personal-dictionary = Les mots que ce projet considère comme bien orthographiés.
settings-desc-tags = Les étiquettes colorées dont ce projet marque ses éléments.
settings-desc-templates = Les modèles de notes que vous pouvez insérer en écrivant dans ce projet.
settings-desc-text-replacements = Les abréviations qui se déplient à la frappe dans ce projet.
settings-desc-work-punctuation = Les guillemets, tirets et espacements que suit la prose de ce projet.

## Fenêtre des paramètres: champs
settings-group-typography = Typographie
settings-group-writing-column = Colonne d’écriture
settings-group-writing-view = Affichage de l’écriture
# Les éléments optionnels de la bande de contrôle du mode sans distraction
# (Quitter n'est jamais optionnel).
settings-group-distraction-free-strip = Bande de contrôle
settings-group-theme = Thème
settings-group-language = Langue
settings-group-startup = Démarrage
settings-group-autosave = Enregistrement automatique
settings-field-typeface = Police
settings-field-size = Taille du texte

# Le message affiché pendant que Ctrl+Molette / Ctrl+= / Ctrl+- / Ctrl+0
# modifient la taille du texte d'un éditeur. Un message complet par surface
# plutôt qu'un gabarit partagé : le nom de la surface ne se substitue pas de
# la même façon dans toutes les langues. $percent arrive déjà formaté.
editor-size-changed-manuscript = Taille du texte du manuscrit : { $percent }
editor-size-changed-synopsis = Taille du texte du synopsis : { $percent }
editor-size-changed-notes = Taille du texte des notes : { $percent }
editor-size-changed-corkboard = Taille du texte des fiches : { $percent }
editor-size-changed-corkboard-expanded = Taille du texte de l’éditeur agrandi : { $percent }
editor-size-changed-distraction-free = Taille du texte sans distraction : { $percent }

menu-text-size-increase = Agrandir le &texte
menu-text-size-decrease = Réd&uire le texte
menu-text-size-reset = Réinitialiser la taille du te&xte
settings-field-line-height = Interligne
settings-field-first-line-indent = Retrait de première ligne
settings-field-paragraph-spacing-before = Espace avant le paragraphe
settings-field-paragraph-spacing-after = Espace après le paragraphe
settings-field-column-width = Largeur de colonne
settings-distraction-free-width-hint = S’applique uniquement en mode sans distraction — les largeurs de colonne de la Scène, du Synopsis et des Notes restent inchangées.
settings-distraction-free-title = Conserver le nom de l’élément
settings-distraction-free-word-count = Conserver le compteur de mots
settings-distraction-free-session = Conserver la session d’écriture
settings-distraction-free-go-to = Conserver le bouton Aller à…
settings-distraction-free-go = Conserver les boutons Précédent et Suivant
settings-distraction-free-chrome-hint = Le bouton Quitter reste toujours affiché, quels que soient ces choix — c’est votre porte de sortie si Échap est déjà pris.
settings-field-app-theme = Thème
# Les deux entrées du sélecteur de thème (les apparences claire / sombre de Fluent).
settings-theme-light = Clair
settings-theme-dark = Sombre
settings-field-text-scale = Taille du texte de l’interface
settings-field-language = Langue de l’interface
settings-synopsis-placement = Position du synopsis
settings-synopsis-placement-none = Aucun
settings-synopsis-placement-top = Au-dessus
settings-synopsis-placement-side = À côté
settings-typewriter = Défilement machine à écrire
settings-typewriter-tip = Maintient la ligne en cours d’écriture à une hauteur fixe pendant que le manuscrit défile en dessous. Un clic place toujours le curseur là où vous cliquez.
settings-typewriter-position = Position de la ligne
settings-typewriter-position-top-third = Premier tiers
settings-typewriter-position-middle = Milieu
settings-typewriter-position-bottom-quarter = Quart inférieur
settings-highlight-scope = Mise en évidence autour du curseur
settings-highlight-scope-none = Aucune
settings-highlight-scope-sentence = Phrase
settings-highlight-scope-paragraph = Paragraphe
settings-highlight-scope-tip-none = Laisser la page unie. Rien n’est teinté pendant l’écriture.
settings-highlight-scope-tip-sentence = Teinter la phrase en cours d’écriture, pour la distinguer de celles qui l’entourent.
settings-highlight-scope-tip-paragraph = Teinter tout le paragraphe en cours d’écriture, pour garder sous les yeux le passage travaillé.
settings-group-container-views = Vues des conteneurs
settings-remember-view = Mémoriser la dernière vue pour chaque type d’élément
settings-remember-view-tip = Ouvrir un conteneur sur la vue utilisée en dernier pour ce type
settings-remember-view-tip-more =
    Un Livre, une Partie et un Chapitre offrent chacun plusieurs vues (sa propre
    page, le manuscrit complet, le synopsis complet). Activez cette option et
    chaque type se rouvre sur la vue choisie en dernier, par exemple, passez un
    Chapitre en Chapitre complet et le prochain Chapitre ouvert s’affichera aussi
    en Chapitre complet. Chaque type mémorise sa propre vue.
# Volet Objectifs et comptage des mots
settings-group-counting = Comptage des mots
settings-counting-auto = Automatique (selon la langue)
settings-counting-whitespace = Découper aux espaces
settings-counting-unicode-words = Mots Unicode
settings-counting-cjk-hybrid = Adapté au CJC (par caractère)
settings-counting-hint = Le mode automatique compte le chinois et le japonais par caractère, et toutes les autres langues par mot. Ne le changez que si le comptage semble incorrect pour votre langue.
settings-group-goals-display = Affichage
settings-show-characters = Afficher le nombre de caractères dans la barre d’état
settings-autosave-hint = Les modifications sont enregistrées automatiquement au fil de l’écriture.

## Paramètres: Œuvre (le projet ouvert)
settings-sec-work = Œuvre
settings-page-structure = Structure
settings-page-author = Auteur
settings-field-author-name = Nom de l’auteur
settings-field-author-placeholder = Facultatif
settings-field-author-hint = Apparaît sur la page de titre compilée et dans les métadonnées des fichiers exportés. Laissez vide pour l’omettre. C’est la signature du livre — le nom qui signe vos commentaires se trouve dans Paramètres ▸ Utilisateur.
settings-group-chapters = Chapitres
tidy-titles-title = Nettoyer les titres de chapitre
tidy-titles-none = Aucun titre de chapitre ou de partie ne se contente de répéter son propre numéro.
tidy-titles-lead = { $count ->
        [one] Un chapitre ou une partie n’a pour titre que son propre numéro.
       *[other] { $count } chapitres et parties n’ont pour titre que leur propre numéro.
    }
tidy-titles-explain = Effacer ces titres laisse chacun désigné par le numéro que le livre connaît déjà — celui-là même qu’imprime l’export. Rien d’autre ne change, et une seule annulation les rétablit tous.
menu-document-tidy-titles = &Nettoyer les titres de chapitre…
settings-group-numbering = Numérotation
settings-number-chapters = Numéroter les chapitres et les parties
settings-number-chapters-tip = Les chapitres et les parties portent un numéro déduit de leur place dans le livre — affiché ici à côté de leur titre, et imprimé par l’export.
settings-number-chapters-tip-more = Le numéro n’est jamais enregistré dans le titre : il reste juste quand vous réorganisez, insérez ou supprimez. Désactivez ceci et l’export n’imprime que les titres, quel que soit le style d’export. Pour n’exclure qu’un seul chapitre — un prologue, un interlude — utilisez plutôt l’interrupteur Numérotation de l’inspecteur : il le garde dans le livre mais l’empêche de prendre un numéro.
settings-part-resets-chapter = Recommencer la numérotation à chaque partie
settings-part-resets-chapter-tip = Désactivé par défaut : les chapitres se suivent d’une partie à l’autre, si bien que la « Deuxième partie » s’ouvre sur le chapitre onze.
settings-part-resets-chapter-tip-more = C’est l’usage de l’édition, et ce qu’attend un lecteur. Activez-le pour un livre dont les parties se lisent comme des volumes distincts, chacune s’ouvrant sur le chapitre un.
settings-chapter-flat = Chapitres à plat
settings-chapter-flat-hint = Activé : un chapitre est une seule ligne. Vous y écrivez, et il ne contient aucune scène. Désactivé : un chapitre est un dossier. Vous y écrivez également, mais il peut en outre contenir des scènes. Les nouveaux chapitres suivent ce paramètre ; les existants se convertissent via Promouvoir.

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
settings-styles-json-filter = Style d’export
settings-styles-editor-title = Modifier le style
settings-styles-editor-none = Sélectionnez un style personnalisé à modifier, ou dupliquez-en un intégré.
settings-styles-imported = Style importé
settings-styles-import-failed = Impossible d’importer le style
settings-styles-exported = Style exporté
settings-styles-export-failed = Impossible d’exporter le style
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
settings-styles-field-epigraph-placement = Position de l’épigraphe
settings-styles-epigraph-after = Après le titre
settings-styles-epigraph-before = Avant le titre
settings-styles-field-footnotes = Inclure les notes de bas de page
settings-styles-field-footnote-numbering = Numérotation des notes
settings-styles-footnote-numbering-continuous = Continue
settings-styles-footnote-numbering-per-chapter = Redémarre à chaque chapitre
settings-styles-footnote-numbering-per-book = Redémarre à chaque livre
settings-styles-field-images = Images
menu-image = &Image
settings-styles-images-beside = À côté du document
settings-styles-images-embed = Dans le document
settings-styles-images-omit = Ne pas inclure
settings-styles-group-round-trip = Envoi à un éditeur
settings-styles-field-comments = Inclure les commentaires
settings-styles-field-round-trip-marks = Inclure les marqueurs d’aller-retour
settings-styles-round-trip-hint = DOCX et ODT uniquement. Les marqueurs sont des identifiants invisibles qui permettent à un fichier de retour de mettre à jour ce projet au lieu de s’y ajouter en double.
settings-styles-group-pages = Pages
settings-styles-field-cover = Ouvrir sur la couverture
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
welcome-create-from = Créer depuis…
welcome-recent-works = Œuvres récentes
welcome-empty-recents = Aucune œuvre récente.
# Affiché à la place de la liste des œuvres récentes lorsque la recherche n'en
# trouve aucune, à distinguer du cas où il n'y a aucune œuvre récente.
welcome-no-matches = Aucune œuvre récente ne correspond à votre recherche.
welcome-learn-soon = Guides et astuces à venir.
welcome-about-blurb = Skribisto, une réécriture en Rust + Teksilo de l’application d’écriture.
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
corkboard = Tableau de liège
overview = Vue d’ensemble

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
corkboard-layout-hint = « Imbriqué » affiche les enfants directs d’un conteneur — ouvrez une carte dossier pour y entrer. « À plat » affiche toutes les scènes du conteneur d’un coup.
corkboard-card-size = Taille des cartes
corkboard-search-placeholder = Filtrer les cartes…
corkboard-empty-title = Rien ici pour l’instant
corkboard-empty-hint = Utilisez « ＋ Nouveau » ci-dessus pour ajouter le premier élément.
corkboard-new = Nouveau
corkboard-show-card-numbers = Numéroter les cartes
corkboard-modal-size = Taille de l’éditeur agrandi
corkboard-scope-hint = S’applique à tous les tableaux ouverts, dans tous les projets.
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
    [one] Définir le libellé sur { $count } carte
   *[other] Définir le libellé sur { $count } cartes
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
corkboard-move-picker-empty = Aucun classeur pour l’instant — créez-en un d’abord.
corkboard-move-picker-cancel = Annuler
corkboard-move-here = Déplacer ici
corkboard-moved-ok = { $count ->
    [one] { $count } carte déplacée
   *[other] { $count } cartes déplacées
}
corkboard-move-into-self = Un conteneur ne peut pas être déplacé dans lui-même. Choisissez une destination en dehors.
corkboard-move-failed = Ces cartes n’ont pas pu être déplacées là.

## Overview (the container's contents as a sortable table)
overview-col-title = Titre
overview-col-type = Type
overview-col-label = Libellé
overview-col-tags = Étiquettes
# Construite uniquement quand le Work compte deux Livres ou plus : voir la
# condition d'`overview_columns`, la même que partage chaque surface Livres de
# cette édition.
overview-col-books = Livres
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
overview-empty-hint = Utilisez « ＋ Nouveau » ci-dessus pour ajouter le premier élément.
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
no-content = Cet élément n’a pas de contenu modifiable.
untitled = Sans titre
placeholder-title = Titre…
placeholder-subtitle = Sous-titre…
placeholder-chapter-title = Titre du chapitre…
split-editor = Diviser l’éditeur
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
segment-story-bible = Bible narrative
segment-pace = Rythme
# Les trois segments de l'onglet d'un `Item/Note` : son propre texte, ses champs de
# bible narrative, et (seulement si elle porte une étiquette repérable) le texte du
# manuscrit où elle a été déclarée présente. Voir `teksilo_ui::tabs::item_note`.
segment-note-own = Note
segment-note-details = Détails
segment-note-in-prose = Dans le texte
pace-placeholder = Le planificateur de rythme apparaît ici.
# Planificateur de rythme
pace-empty-title = Planifiez le rythme de ce livre
pace-empty-body = Fixez un objectif de mots et une échéance : Skribisto calcule le rythme quotidien pour y parvenir.
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
pace-card-of-goal = de l’objectif
pace-card-rate = mots / jour d’écriture
pace-card-days-left = jours d’écriture restants
pace-card-streak = jours d’affilée
pace-card-ahead = mots d’avance
pace-card-behind = mots de retard
pace-charts-empty = Les graphiques de progression apparaissent ici dès que le nombre de mots est relevé, à l’enregistrement du projet.
pace-chart-progression = Mots écrits par rapport à l’objectif
pace-chart-words-per-day = Mots par jour
pace-series-actual = Réel
pace-series-target = Objectif
pace-series-words-per-day = Mots/jour
pace-daily-target-line = Rythme régulier : { $count } mots/jour
pace-section-holidays = Congés
pace-section-milestones = Jalons
pace-holidays-none = Aucun congé. Tous les jours prévus comptent.
pace-holiday-label = Nom du congé
pace-add-holiday = Ajouter
pace-remove = Retirer
pace-milestones-none = Aucun jalon. Définissez-en un sur une partie ou un chapitre dans l’inspecteur.
full-chapter = Chapitre complet
full-part = Partie complète
full-book = Livre complet
full-synopsis = Synopsis complet
rename = Renommer
set-label = Définir le libellé
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
open-project-question = Comment ouvrir « { $title } » ?
open-in-new-window = Ouvrir dans une nouvelle fenêtre
open-here = Ouvrir ici

## Sélecteur de classeur + recherche
binder-all = Tous les classeurs
binder-show-all = Afficher tous les classeurs
binder-new = Nouveau classeur…
binder-item-count = { $count ->
    [one] { $count } élément
   *[other] { $count } éléments
}
binder-search-placeholder = Filtrer le plan…
binder-search-scope = Chercher dans tous les classeurs
binder-trash-confirm-title = Mettre le classeur à la corbeille ?
binder-trash-confirm-text = « { $name } » et tous ses éléments seront mis à la corbeille.

## Boîtes de dialogue
dialog-rename = Renommer
dialog-set-label = Définir le libellé
dialog-new-scene = Nouvelle scène
close-work-question = Enregistrer les modifications avant de fermer l’œuvre ?
quit-question = Enregistrer les modifications avant de quitter ?
quit-save-work-question = Enregistrer les modifications de { $title } avant de quitter ?
unsaved-changes = Cette œuvre a des modifications non enregistrées.
# Affiché par Quitter quand une autre œuvre ouverte (pas celle de cette fenêtre)
# a encore des modifications non enregistrées — le dialogue unique listant
# chaque œuvre modifiée (voir l'action `app.quit` dans
# app::commands::file). Quitter est refusé tant qu'elles ne sont pas
# enregistrées ou fermées depuis leur propre fenêtre.
# Remplacement de l'œuvre ouverte dans cette fenêtre (Nouvelle œuvre, Ouvrir une
# œuvre, « Ouvrir ici », « Ouvrir maintenant » du bandeau d'import), même garde
# que la fermeture, puisque l'œuvre ouverte est fermée dans tous les cas.
new-work-unsaved-question = Enregistrer les modifications avant de créer une nouvelle œuvre ?
open-work-unsaved-question = Enregistrer les modifications avant d’ouvrir une autre œuvre ?
switch-backup-discard-title = Abandonner les modifications de cette copie de secours ?
switch-backup-discard-text = Les modifications d’une copie de secours ne peuvent pas y être enregistrées. Utilisez « Enregistrer sous » ou « Restaurer » pour les conserver, ou abandonnez-les et ouvrez l’autre œuvre.
switch-save-failed = L’œuvre n’a pas pu être enregistrée, elle n’a donc pas été remplacée : { $error }
switch-save-not-started = L’œuvre n’a pas pu être enregistrée, elle n’a donc pas été remplacée.
close-save-failed = L’œuvre n’a pas pu être enregistrée, elle n’a donc pas été fermée : { $error }
close-save-not-started = L’œuvre n’a pas pu être enregistrée, elle n’a donc pas été fermée.
save-not-started = L’œuvre n’a pas pu être enregistrée.

## Notifications
# Titre de notification. Court, car le titre tient sur une ligne et se voit tronqué ; la
# cause va dans le corps, qui est le texte d'erreur non traduit. $file est le nom du
# fichier, sans son chemin.
could-not-open-work = Impossible d’ouvrir « { $file } »
# Un projet enregistré par une version de Skribisto plus récente que celle-ci — titre et
# corps. $written_by est la version qui l'a écrit, $requires la plus ancienne version
# capable de l'ouvrir (les deux diffèrent lorsque la version récente n'a rien utilisé de
# nouveau), $supported la plus récente que cette version comprend.
could-not-open-work-too-new = « { $file } » nécessite une version plus récente de Skribisto
could-not-open-work-too-new-detail = Enregistré au format Skribisto { $written_by } ; son ouverture nécessite le format { $requires } ou plus récent, or cette version ne prend en charge que le format { $supported } au maximum. Mettez Skribisto à jour pour l’ouvrir.
could-not-open-example = Impossible d’ouvrir l’exemple : { $error }
# Le navigateur (ou ce qui traite les liens http) n'a pas pu être lancé pour l'un
# des liens de la barre latérale d'accueil. $url est affichée pour pouvoir être
# copiée malgré tout.
could-not-open-link = Impossible d’ouvrir { $url } : { $error }
could-not-create-work = Impossible de créer l’œuvre : { $error }
saving-as-file = Enregistrement sous { $target }…
saving-as-folder = Enregistrement sous { $target }/…
saved-as = Enregistré dans { $target }
save-error = Impossible d’enregistrer : { $error }
backup-error = Impossible de créer la copie de secours : { $error }
backing-up = Création de la copie de secours…
backup-nothing-open = Aucun projet n’est ouvert, il n’y a donc aucune copie de secours à créer.
backup-already-running = Une copie de secours est déjà en cours.
backup-complete = Copie de secours terminée ({ $ok ->
    [one] { $ok } enregistrée
   *[other] { $ok } enregistrées
}, { $skipped } déjà à jour)
backup-partial = Copie de secours terminée : { $ok ->
    [one] { $ok } enregistrée
   *[other] { $ok } enregistrées
}, échec pour { $failed ->
    [one] { $failed } destination
   *[other] { $failed } destinations
}
backup-no-destination-title = Aucun emplacement de copie de secours disponible
backup-no-destination-text = Aucune des destinations configurées n’est accessible (par exemple, un disque externe peut être débranché). Branchez-le et réessayez, ou quittez sans créer de copie de secours.
backup-failed-close-title = La copie de secours n’a pas pu être enregistrée
backup-failed-close-text = Aucune copie de secours n’a pu être écrite avant la fermeture : toutes les destinations ont échoué (le disque a peut-être été retiré, ou il est plein ou protégé en écriture). Corrigez le problème et réessayez, ou quittez sans créer de copie de secours.

## Ouvrir une copie de secours (modale de choix + bannière permanente + restauration)
backup-choice-title = Copie de secours
backup-choice-heading = Vous avez ouvert une copie de secours
backup-choice-subtitle = Il s’agit d’une copie de secours d’un projet, à un instant donné.
backup-choice-subtitle-dated = Copie de secours du { $date }.
backup-choice-body = Vous pouvez l’ouvrir et la modifier librement, mais les changements ne peuvent être conservés qu’avec « Enregistrer sous ». Le fichier du projet d’origine n’est pas modifié. Ou restaurez ce projet exactement à cette copie de secours.
backup-choice-open = Ouvrir la copie de secours
backup-choice-restore = Restaurer le projet à ce point…
backup-choice-not-a-backup = Non, l’ouvrir normalement
backup-banner-title = Copie de secours : les changements ne peuvent pas être enregistrés ici
backup-banner-description = Utilisez « Enregistrer sous » pour conserver vos modifications dans un nouveau fichier, ou « Restaurer » pour remplacer le projet d’origine par cette copie.
backup-banner-restore = Restaurer…
backup-banner-save-as = Enregistrer sous…
backup-restore-original-missing = Impossible de trouver le projet d’origine à restaurer. Utilisez « Enregistrer sous » pour conserver cette copie comme nouveau projet.
backup-restore-close-elsewhere-title = Projet ouvert dans une autre fenêtre
backup-restore-close-elsewhere-text = Le projet que vous restaurez est ouvert dans une autre fenêtre. Fermez-le d’abord, puis réessayez.
backup-restore-focus-window = Afficher cette fenêtre
backup-restore-confirm-title = Restaurer cette copie de secours ?
backup-restore-confirm-text = La version actuelle du projet sera copiée à côté comme copie de sécurité avant d’être remplacée par celle-ci.
backup-restore-confirm-ok = Restaurer
backup-restore-error = Impossible de restaurer : { $error }
backup-restored-ok = Projet restauré.
backup-restored-with-safety = Projet restauré. Votre version précédente a été enregistrée dans { $path }.
close-backup-discard-title = Abandonner les modifications de cette copie de secours ?
close-backup-discard-text = Les modifications d’une copie de secours ne peuvent pas y être enregistrées. Utilisez « Enregistrer sous » pour les conserver, ou abandonnez et fermez.
quit-backup-discard-title = Abandonner les modifications et quitter ?
quit-backup-discard-work-question = Abandonner les modifications de { $title } et quitter ?
quit-backup-discard-text = Les modifications d’une copie de secours ne peuvent pas y être enregistrées. Utilisez « Enregistrer sous » pour les conserver, ou abandonnez et quittez.
backup-nudge-text = Aucune copie de secours n’est configurée pour ce projet.
backup-nudge-action = Configurer les copies de secours…

## Panneau de la liste des copies de secours
menu-backups-list = &Liste des copies de secours…
backups-title = Copies de secours
backups-loading = Recherche des copies de secours…
backups-empty = Aucune copie de secours trouvée pour ce projet.
backups-open = Ouvrir
backups-reveal = Afficher
backups-delete = Supprimer cette copie de secours
backups-delete-confirm-title = Supprimer cette copie de secours ?
backups-delete-confirm-text = « { $name } » sera définitivement supprimée. Cette action est irréversible.
backups-delete-error = Impossible de supprimer la copie de secours : { $error }
backups-refresh = Actualiser
backups-close = Fermer

## Panneaux de paramètres des copies de secours
settings-page-backup = Réglages de sauvegarde
settings-page-work-backup = Copies de secours
settings-backup-general-title = Paramètres par défaut des copies de secours
settings-backup-work-title = Copies de secours de ce projet
settings-backup-inherit = Utiliser les paramètres généraux
settings-backup-inheriting = Ce projet utilise les paramètres généraux des copies de secours.
settings-backup-none-hint = Aucune copie de secours automatique n’est configurée (tous les déclencheurs sont désactivés).
settings-backup-last = Dernière copie de secours : { $date }
settings-backup-last-never = Aucune copie de secours pour l’instant.
settings-backup-open-list = Ouvrir la liste des copies de secours…
settings-backup-triggers = Quand créer une copie de secours
settings-backup-on-close = À la fermeture du projet
settings-backup-on-open = À l’ouverture du projet
settings-backup-interval = Régulièrement, toutes les
settings-backup-destinations = Destinations des copies de secours
settings-backup-default-location = Emplacement par défaut
settings-backup-usage-since = { $count ->
    [one] { $count } copie de secours · { $size } · la plus ancienne { $oldest }
   *[other] { $count } copies de secours · { $size } · la plus ancienne { $oldest }
}
settings-backup-usage = { $count ->
    [one] { $count } copie · { $size }
   *[other] { $count } copies · { $size }
}
settings-backup-usage-empty = Aucune copie conservée ici pour l’instant
settings-backup-usage-measuring = Calcul en cours…
settings-backup-reveal-root = Afficher le dossier
settings-backup-dest-none = Aucune destination. Les copies sont enregistrées à côté du projet.
settings-backup-dest-remove = Retirer
settings-backup-dest-add = Ajouter un dossier…
settings-backup-dest-refresh = Actualiser
settings-backup-retention = Combien en conserver
settings-backup-retention-tiered = Par paliers
settings-backup-retention-keep-n = Garder les N dernières
settings-backup-retention-tip = Comment les anciennes copies sont supprimées.
settings-backup-retention-tip-more = Le mode par paliers conserve une copie par heure pendant un jour, par jour pendant une semaine, par semaine pendant un mois, et par mois au-delà : l’historique récent reste dense et l’ancien s’éclaircit. « Garder les N dernières » conserve simplement les N copies les plus récentes. Dans les deux modes, les plus récentes (le minimum ci-dessous) sont toujours conservées.
settings-backup-gfs-hourly = Par heure (dernières 24 h)
settings-backup-gfs-daily = Par jour (dernière semaine)
settings-backup-gfs-weekly = Par semaine (dernier mois)
settings-backup-gfs-monthly = Par mois
settings-backup-keep-n = Nombre à conserver
settings-backup-min-keep = Toujours conserver au moins
settings-backup-dedup = Ignorer une copie si rien n’a changé

## Boîte de dialogue Nouvelle œuvre
new-work-title = Nouvelle œuvre
new-work-close = Fermer
new-work-name = Nom de l’œuvre
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
new-work-template-novel-in-parts = Roman en parties
new-work-template-notebook = Carnet
new-work-cancel = Annuler
new-work-create = Créer l’œuvre
# Étapes de l'assistant et navigation
new-work-step-details = Détails
new-work-step-language = Langue et structure
new-work-step-template = Modèle
new-work-back = Précédent
new-work-next = Suivant
# Le même assistant, ouvert depuis « Depuis des documents… » du lanceur
new-work-documents-title = Nouvelle œuvre depuis des documents
new-work-step-import = Importation
new-work-create-and-import = Créer et importer…
new-work-documents-next-title = Vos documents arrivent ensuite
new-work-documents-next-body = La création du projet ouvre l’assistant d’importation par-dessus : vous y choisissez les documents, vérifiez la structure que Skribisto y lit, et indiquez où elle doit atterrir. Rien n’est écrit dans le projet tant que vous ne l’avez pas confirmé là-bas.
new-work-documents-no-template = Ce projet démarre volontairement vide : aucun modèle, afin que les documents importés en soient le seul contenu.
new-work-documents-chapter-scene-hint = S’applique aux chapitres créés par l’importation. Vous pourrez le changer plus tard dans les paramètres du projet.
# Descriptions des tuiles de format
new-work-single-file-desc = Une archive .skrib (zip). Portable, facile à sauvegarder.
new-work-bundle-desc = Un dossier contenant chaque texte & ressource. Adapté au contrôle de version.
# Décomptes des modèles
new-work-template-none-count = classeur vide
new-work-template-empty-novel-count = 1 chapitre
new-work-template-light-novel-count = 15 chapitres
new-work-template-novel-count = 20 chapitres
new-work-template-novel-in-parts-count = 3 parties, 24 chapitres
new-work-template-notebook-count = notes libres
# Bascule ChapterScene (modèles de roman)
new-work-chapter-scene = Chapitres à plat
new-work-chapter-scene-tip = Chaque chapitre est une seule ligne où vous écrivez directement, sans scène en dessous. Laissez désactivé pour la disposition classique : le chapitre est alors un dossier, dans lequel vous écrivez tout autant, mais qui peut aussi contenir des scènes.
new-work-chapter-scene-tip-more = Vous écrivez dans le chapitre dans les deux cas. La seule différence est sa capacité à *contenir* des scènes. L’arborescence du classeur de Skribisto est purement organisationnelle : les deux dispositions produisent le même livre, vous pouvez les mélanger librement, et Promouvoir convertit un chapitre de l’une à l’autre sans perdre un mot.
# Validation des champs
new-work-name-required = Saisissez un nom pour l’œuvre
new-work-name-invalid = Ce nom ne contient aucun caractère utilisable
new-work-location-required = Choisissez un emplacement
new-work-location-missing = Ce dossier n’existe pas
new-work-location-not-folder = Ce chemin n’est pas un dossier
new-work-location-readonly = Ce dossier n’est pas accessible en écriture

## Libellés des modèles de nouvelle œuvre (transmis au backend, qui ne fait pas d'i18n)
new-work-manuscript = Manuscrit
new-work-notes = Notes
new-work-research = Recherche
new-work-notebook = Carnet
new-work-scene = Scène
new-work-note = Note
new-work-front-matter = Pages liminaires
new-work-back-matter = Annexes
new-work-paratext = Structure du livre
new-work-paratext-none = Aucune structure
new-work-paratext-hint = Les pages liminaires et les annexes propres à une tradition éditoriale. Vous pourrez tout déplacer, renommer ou supprimer ensuite.
new-work-tags = Étiquettes
new-work-tags-none = Aucune étiquette
new-work-tags-hint = Une palette de départ pour étiqueter les personnages, les lieux et le reste. Facultative, et chaque étiquette peut ensuite être renommée, recolorée ou supprimée.
new-work-note-templates = Modèles de note
new-work-note-templates-none = Aucun modèle
new-work-note-templates-hint = La forme que prend une fiche de la bible narrative au départ. Facultatifs, et chaque modèle peut ensuite être modifié ou supprimé.
new-work-characters = Personnages
new-work-places = Lieux

## Boîte de dialogue d'import Plume Creator
import-plume-title = Importer un projet Plume Creator
import-plume-close = Fermer
import-plume-source = Projet Plume
import-plume-source-hint = Choisissez un fichier .plume ou .plume_backup (n’importe quelle version de Plume Creator).
import-plume-location = Dossier de destination
import-plume-name = Nom du fichier
import-plume-name-placeholder = Nom du projet
import-plume-will-create = Créera
import-plume-trash-warning = ⚠ Les éléments à la corbeille / supprimés ne sont pas migrés.
import-plume-cancel = Annuler
import-plume-import = Importer
# Validation des champs
import-plume-source-required = Choisissez un fichier de projet Plume
import-plume-source-missing = Ce fichier n’existe pas
import-plume-source-not-file = Ce chemin n’est pas un fichier
import-plume-location-required = Choisissez un dossier de destination
import-plume-location-missing = Ce dossier n’existe pas
import-plume-location-not-folder = Ce chemin n’est pas un dossier
import-plume-location-readonly = Ce dossier n’est pas accessible en écriture
import-plume-name-required = Saisissez un nom de fichier
import-plume-name-exists = Un fichier de ce nom existe déjà ici. L’import demandera confirmation du remplacement
# Confirmation de remplacement
import-plume-overwrite-title = Remplacer le fichier existant ?
import-plume-overwrite-text = « { $name } » existe déjà. Le remplacer par le projet importé ?
# Noms de classeurs transmis au backend (qui ne fait pas d'i18n)
import-plume-manuscript-binder = Manuscrit
import-plume-story-bible-binder = Personnages et lieux
# Toast de progression (l'import est une opération longue)
import-plume-progress-title = Importation du projet Plume…
import-plume-cancel-import = Annuler
import-plume-cancelled = Importation annulée
# Résultat
import-plume-done = { $imported ->
    [one] { $imported } élément importé.
   *[other] { $imported } éléments importés.
} { $skipped ->
    [one] { $skipped } élément à la corbeille n’a pas été migré.
   *[other] { $skipped } éléments à la corbeille n’ont pas été migrés.
}
import-plume-open-now = Ouvrir maintenant
# Affiché quand l'importateur n'a pas pu tout reprendre à l'identique.
import-plume-warnings = { $count ->
    [one] 1 élément n’a pas pu être importé à l’identique
   *[other] { $count } éléments n’ont pas pu être importés à l’identique
}
import-plume-details = Détails
import-plume-warnings-title = Avertissements d’importation
# Toast d'erreur : motif court dans le corps, chaîne technique complète derrière « Détails »
import-plume-error-title = Impossible d’importer le projet
import-plume-error-details = Détails

## Boîte de dialogue d'import de projet Manuskript
import-manuskript-title = Importer un projet Manuskript
import-manuskript-close = Fermer
import-manuskript-source = Projet Manuskript
# Un projet Manuskript est un fichier .msk plus, dans son mode habituel, un
# dossier du même nom à côté. L'un ou l'autre convient, le dossier aussi.
import-manuskript-source-hint = Choisissez le fichier .msk ou le dossier du projet (n’importe quelle version de Manuskript).
import-manuskript-source-file = Choisir un fichier…
import-manuskript-source-folder = Choisir un dossier…
import-manuskript-location = Dossier de destination
import-manuskript-name = Nom du fichier
import-manuskript-name-placeholder = Nom du projet
import-manuskript-will-create = Créera
import-manuskript-cancel = Annuler
import-manuskript-import = Importer
# Validation des champs
import-manuskript-source-required = Choisissez un projet Manuskript
import-manuskript-source-missing = Ce fichier ou ce dossier n’existe pas
import-manuskript-location-required = Choisissez un dossier de destination
import-manuskript-location-missing = Ce dossier n’existe pas
import-manuskript-location-not-folder = Ce chemin n’est pas un dossier
import-manuskript-location-readonly = Ce dossier n’est pas accessible en écriture
import-manuskript-name-required = Saisissez un nom de fichier
import-manuskript-name-exists = Un fichier de ce nom existe déjà ici. L’import demandera confirmation du remplacement
# Confirmation de remplacement
import-manuskript-overwrite-title = Remplacer le fichier existant ?
import-manuskript-overwrite-text = « { $name } » existe déjà. Le remplacer par le projet importé ?
# Noms transmis au backend (qui ne fait pas d'i18n). Manuskript n'en stocke
# aucun : il n'a ni classeurs ni groupes de bible, et son échelle d'importance
# est faite de trois nombres dont les noms vivent dans son interface.
import-manuskript-manuscript-binder = Manuscrit
import-manuskript-story-bible-binder = Bible de l’histoire
import-manuskript-characters-group = Personnages
import-manuskript-world-group = Univers
import-manuskript-plots-group = Intrigues
import-manuskript-project-info-note = Informations du projet
import-manuskript-summary-note = Résumé
import-manuskript-importance-minor = Mineur
import-manuskript-importance-secondary = Secondaire
import-manuskript-importance-main = Principal
# Toast de progression (l'import est une opération longue)
import-manuskript-progress-title = Importation du projet Manuskript…
import-manuskript-cancel-import = Annuler
import-manuskript-cancelled = Importation annulée
# Résultat
import-manuskript-done = { $imported ->
    [one] { $imported } élément importé.
   *[other] { $imported } éléments importés.
} { $revisions ->
    [0] { "" }
    [one] { $revisions } version antérieure a suivi.
   *[other] { $revisions } versions antérieures ont suivi.
}
import-manuskript-open-now = Ouvrir maintenant
# Affiché quand l'importateur n'a pas pu tout reprendre à l'identique.
import-manuskript-warnings = { $count ->
    [one] 1 chose à savoir sur cet import
   *[other] { $count } choses à savoir sur cet import
}
import-manuskript-details = Détails
import-manuskript-warnings-title = À propos de cet import
# Toast d'erreur : motif court dans le corps, chaîne technique complète derrière « Détails »
import-manuskript-error-title = Impossible d’importer le projet
import-manuskript-error-details = Détails

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
export-selected-count = { $count ->
    [one] { $count } sélectionné
   *[other] { $count } sélectionnés
}
# En-tête de l'aperçu en direct
export-preview-compiled = compilé
export-preview-live = Aperçu en direct
# Étiquettes récapitulatives du style
export-chip-chapters-none = Chapitres : aucun
export-chip-chapters-numbered = Chapitres : numérotés
export-chip-chapters-title = Chapitres : titre seul
export-chip-chapters-both = Chapitres : numéro + titre
export-chip-scene-break-glyph = Saut de scène : { $glyph }
export-chip-scene-break-blank = Saut de scène : ligne vide
export-chip-scene-break-none = Saut de scène : aucun
export-chip-major-break-glyph = Saut majeur { $glyph }
export-chip-major-break-blank = Saut majeur : ligne vide
export-chip-major-break-none = Saut majeur : aucun
export-chip-major-break-same = Les deux niveaux identiques
export-chip-spacing-single = Interligne : simple
export-chip-spacing-onehalf = Interligne : 1½
export-chip-spacing-double = Interligne : double
export-chip-notes-included = Notes incluses
export-chip-notes-excluded = Notes exclues
# Formats de sortie
export-format-docx = Word
export-format-odt = LibreOffice
export-format-html = HTML
export-format-markdown = Markdown
export-format-djot = Djot
export-format-text = Texte
export-format-latex = LaTeX
export-format-epub = EPUB
export-format-pdf = PDF
# Confirmation de remplacement
export-overwrite-title = Remplacer le fichier existant ?
export-overwrite-text = « { $name } » existe déjà. Le remplacer ?
# Toast de progression (l'export est une opération longue)
export-progress-title = Exportation…
export-cancelled = Exportation annulée
export-done = { $count ->
    [one] { $count } élément exporté
   *[other] { $count } éléments exportés
}
export-open-file = Ouvrir
export-show-in-folder = Afficher dans le dossier
# Toast d'erreur : motif court dans le corps, chaîne technique complète derrière « Détails »
export-error-title = Impossible d’exporter
export-error-details = Détails

## Planificateur de sauvegardes (toast de progression + détails d'échec/d'avertissement de purge, revue backup, T1-2/T1-7/T2-3/T2-8/T2-9)
backup-progress-start = Démarrage…
backup-progress-retention = Nettoyage des anciennes copies de secours…
backup-progress-done = Terminé
backup-progress-destination = Destination { $i } sur { $n }
backup-details = Détails
backup-issues-title = Problèmes de copie de secours
backup-failed-title = Échec de la copie de secours
backup-complete-prune-warning = Copie de secours terminée ({ $ok ->
    [one] { $ok } enregistrée
   *[other] { $ok } enregistrées
}, { $skipped } déjà à jour). Certaines anciennes copies n’ont pas pu être supprimées

# Recherche et remplacement
search = Rechercher
search-query-placeholder = Rechercher…
search-replace-placeholder = Remplacer par…
search-replace-toggle = Afficher le remplacement
search-replace-all = Tout remplacer
search-opt-case = Respecter la casse
search-opt-whole-word = Mot entier
search-opt-diacritics = Respecter les accents
search-scope-body = Corps
search-scope-title = Titre
search-scope-synopsis = Synopsis
search-scope-label = Libellé
search-facet-book = Livres
search-facet-part = Parties
search-facet-chapter = Chapitres
search-facet-scene = Scènes
search-facet-note = Notes
search-facet-folder = Dossiers
# Info-bulles détaillées des options
search-tip-case = Respecter la casse : les majuscules et les minuscules sont distinctes, « Elena » et « elena » sont des résultats différents.
search-tip-whole-word = Mot entier : ne trouver que les mots complets ; « chat » n’est pas trouvé dans « château ».
search-tip-diacritics = Respecter les accents : les lettres accentuées sont distinctes ; « cafe » ne trouve pas « café ».
search-tip-body = Corps : rechercher dans la prose des scènes et des notes.
search-tip-title = Titre : rechercher dans les titres des éléments du classeur.
search-tip-synopsis = Synopsis : rechercher dans le résumé de chaque ligne d’écriture.
search-tip-label = Libellé : rechercher dans la note affichée sous le titre d’un élément.
search-tip-comment = Commentaires : rechercher dans le texte des fils de commentaires et de leurs réponses. Un commentaire porte sur le manuscrit sans en faire partie, d’où son propre bouton — et un remplacement laisse les commentaires décochés tant que vous ne les cochez pas.
search-tip-book = Livres : le conteneur du livre et ses marqueurs de début / fin.
search-tip-part = Parties : les séparateurs de partie.
search-tip-chapter = Chapitres : les chapitres, quel que soit leur stockage.
search-tip-scene = Scènes : les lignes qui contiennent votre prose.
search-tip-note = Notes : les notes libres.
search-tip-folder = Dossiers : les simples dossiers d’organisation et séparateurs.
search-tip-paratext = Un texte qui appartient au livre mais non à son récit — une préface, une dédicace, une postface. Jamais compté dans le manuscrit.
search-tip-preserve-case = Conserver la casse : le remplacement reprend la casse trouvée, « ELENA » devient « MARTA » et « Elena » devient « Marta ».
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
search-occurrences = { $count }
search-field-body = Corps
search-field-title = Titre
search-field-synopsis = Synopsis
search-field-label = Libellé
search-field-epigraph = Épigraphe
search-field-comment = Commentaire
search-field-comment-reply = Réponse
search-field-footnote = Note de bas de page
search-include-in-replace = Inclure dans « Tout remplacer »
search-collapse-all = Replier tous les résultats
search-replace-here = Remplacer ceci
search-dismiss = Retirer des résultats
search-undo-dismiss = Rétablir le dernier résultat retiré
search-replace-nothing = (rien)
search-replace-confirm-title = Remplacer tous les résultats ?
search-replace-confirm-text =
    Remplacer { $occurrences ->
        [one] { $occurrences } occurrence
       *[other] { $occurrences } occurrences
    } de « { $query } » par « { $replacement } » dans { $items ->
        [one] { $items } document
       *[other] { $items } documents
    } ? Vous pourrez annuler depuis la notification.
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
search-replace-skipped-title = Rien n’a été remplacé
search-replace-skipped-body =
    { $fields ->
        [one] Ce texte a changé
       *[other] { $fields } de ces textes ont changé
    } depuis la recherche, { $fields ->
        [one] il n’a donc pas été modifié
       *[other] ils n’ont donc pas été modifiés
    }. Relancez la recherche pour voir où sont les mots.
search-replace-undo-failed-title = Échec de l’annulation
search-preview = Aperçu
search-preview-empty = Sélectionnez un résultat pour l’afficher ici
search-preview-no-prose = Ce résultat n’a pas de texte modifiable
search-preview-prompt = Pour afficher un aperçu ici, lancez une recherche.
search-preview-open-search = Rechercher dans le projet
search-preview-footnote-prompt = Ce résultat se trouve dans le texte d’une note de bas de page — ouvrez-la dans le panneau Notes de bas de page pour la consulter et la modifier.
search-preview-open-footnotes = Ouvrir les notes de bas de page

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
dict-download-failed = Impossible de télécharger { $name } : { $error }
dict-removed = { $name } supprimé
dict-accept-first = Acceptez la licence avant de télécharger { $name }
# Paramètres ▸ Dictionnaires
settings-dict-tab-installed = Installés
settings-dict-tab-get-more = En obtenir plus
settings-dict-tab-personal = Mots personnels
dict-installed-empty = Aucun dictionnaire trouvé sur cet ordinateur pour l’instant.
dict-get-more-search = Rechercher une langue
dict-system-badge = sur votre système
dict-unusable-badge = inutilisable
dict-download-button = Télécharger
dict-downloading = Téléchargement…
dict-installed-label = Installé
dict-remove = Supprimer
dict-view-license = Voir la licence
dict-approx-size = ~{ $size }
dict-personal-empty = Aucun mot personnel dans ce projet pour l’instant.
dict-personal-add = Ajouter
dict-personal-placeholder = Ajouter un mot…
# Proposition d'installer les dictionnaires manquants à l'ouverture d'un projet
dict-missing-toast = { $count ->
    [one] Ce projet utilise { $count } dictionnaire que vous n’avez pas installé
   *[other] Ce projet utilise { $count } dictionnaires que vous n’avez pas installés
}
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
dict-add-code-hint = Une étiquette courte de votre choix. C’est ce que vous choisirez comme langue d’un document.
dict-add-aff = Fichier d’affixes (.aff)
dict-add-dic = Liste de mots (.dic)
dict-add-submit = Ajouter
dict-add-cancel = Annuler
dict-add-name-required = Donnez un nom au dictionnaire.
dict-add-code-required = Saisissez un code de langue.
dict-add-code-invalid = Utilisez uniquement des lettres, chiffres et - _ .
dict-add-code-reserved = Ce code correspond à un dictionnaire intégré. Choisissez-en un autre.
dict-add-file-required = Choisissez un fichier.
dict-add-file-missing = Ce fichier n’existe pas.
dict-add-code-taken = Un dictionnaire pour ce code est déjà installé.
dict-add-done = { $name } ajouté
dict-add-unusable = Ces fichiers ne sont pas un dictionnaire utilisable : { $error }
dict-add-failed = Impossible d’ajouter le dictionnaire : { $error }
# Les contrôles d'export par élément dans l'Inspecteur (M3)
inspector-export = Export
inspector-exportable = Inclure dans les exports
ctx-number = &Numéroter ce chapitre
ctx-unnumber = Ne pas &numéroter ce chapitre
inspector-numbering = Numérotation
inspector-numbered = Numéroté
inspector-numbered-tip = Ce chapitre prend sa place dans la numérotation du livre. Désactivez-le pour un prologue, un épilogue ou un interlude.
inspector-numbered-tip-more = Un chapitre non numéroté reste dans le livre tel quel : son titre, son texte et son compte de mots sont intacts. Il n’imprime simplement aucun numéro, et n’en consomme pas : le chapitre qui suit un prologue est le chapitre un, pas le chapitre deux. L’exclure de l’export est un autre interrupteur, au-dessus, et celui-là retire le chapitre du livre.
inspector-apply-to-children = Appliquer aux enfants
# La date de jalon par Partie/Chapitre de l'inspecteur (M5), montrée sur le Rythme du Livre.
inspector-milestone = Date de jalon
inspector-milestone-none = Aucune date
inspector-milestone-clear = Effacer
# Le champ de langues à pastilles (Inspecteur + Paramètres)
inspector-tags = Étiquettes
inspector-aliases = Autres noms
inspector-dict-language = Langue
inspector-apply-language-to-children = Appliquer la langue aux enfants
settings-page-language = Langue
settings-field-dict-language = Langues
dict-tradeoff-hint = Chaque langue supplémentaire accepte plus de mots, donc moins de fautes sont détectées.
lang-inherit-hint = Hérité : cette scène utilise les langues du livre ou du projet.
lang-pill-list = Langues
lang-pill-add = Ajouter une langue
lang-pill-remove = Retirer { $name }
lang-pill-mute = Désactiver la correction pour { $name }
lang-pill-unmute = Activer la correction pour { $name }

## Paramètres: Projet ▸ Dictionnaire personnel (liste de mots par projet)
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
settings-user-dict-imported = { $count ->
    [one] { $count } mot importé
   *[other] { $count } mots importés
} ({ $duplicates ->
    [one] { $duplicates } déjà présent
   *[other] { $duplicates } déjà présents
}).
settings-user-dict-import-failed = Impossible de lire la liste de mots : { $error }
settings-user-dict-exported = { $count ->
    [one] { $count } mot enregistré.
   *[other] { $count } mots enregistrés.
}
settings-user-dict-export-failed = Impossible d’enregistrer la liste de mots : { $error }

## Paramètres: Projet ▸ Remplacements de texte (lexique personnalisé par projet)
settings-page-text-replacements = Remplacements de texte
settings-text-repl-desc = Remplace une abréviation par le texte complet à la frappe : « stp » devient « s’il te plaît » dès que vous tapez une espace ou une ponctuation.
settings-text-repl-enable = Utiliser les remplacements de texte dans ce projet
settings-text-repl-disabled-hint = Activez cette option pour définir des abréviations qui se développent à l’écriture.
settings-text-repl-add = Ajouter la règle
settings-text-repl-trigger-placeholder = Abréviation
settings-text-repl-replacement-placeholder = Ce qu’elle devient
settings-text-repl-added = « { $trigger } » ajouté
settings-text-repl-duplicate = « { $trigger } » a déjà une règle
settings-text-repl-filter = Filtrer les règles
# `{ $n }` rather than a literal "1" in the [one] branch: French puts zero in the
# `one` category, so a hardcoded numeral renders "1 règle" for an empty lexicon.
settings-text-repl-count = { $n ->
    [one] { $n } règle
   *[other] { $n } règles
}
settings-text-repl-row-enabled = Utiliser cette règle
settings-text-repl-delete = Supprimer la règle de { $trigger }
settings-text-repl-deleted = Règle de « { $trigger } » supprimée
settings-text-repl-empty = Aucune règle pour l’instant.
settings-text-repl-csv-filter = Fichiers CSV
settings-text-repl-import = Importer…
settings-text-repl-export = Exporter…
settings-text-repl-imported = { $added ->
    [one] { $added } importée
   *[other] { $added } importées
}, { $skipped ->
    [one] { $skipped } ignorée
   *[other] { $skipped } ignorées
}
settings-text-repl-exported = { $n ->
    [one] { $n } règle exportée
   *[other] { $n } règles exportées
}

## Éditeur: orthographe (menu contextuel + notification)
# Affiché à la place des corrections quand un mot signalé n’en a aucune.
editor-menu-no-suggestions = Aucune suggestion
editor-menu-add-to-dictionary = Ajouter « { $word } » au dictionnaire
editor-menu-add-words-to-dictionary = Ajouter les mots sélectionnés au dictionnaire
editor-dict-added = « { $word } » ajouté à votre dictionnaire.
editor-dict-added-multi = { $count ->
    [one] { $count } mot ajouté à votre dictionnaire.
   *[other] { $count } mots ajoutés à votre dictionnaire.
}
toast-undo = Annuler

## Revenir sur une opération précise
##
## Le bouton Annuler d’une notification porte sur une opération précise. Si
## autre chose s’est produit depuis, il le dit plutôt que de défaire ce qui se
## trouve désormais en dernier.
undo-superseded-title = Cette étape n’est plus la dernière
undo-superseded-body = Autre chose a changé dans ce projet depuis. Annuler reviendrait sur cette autre modification : rien n’a donc été fait. Cette étape reste en place — utilisez Édition ▸ Annuler pour remonter l’historique vous-même.
undo-failed = Échec de l’annulation : { $error }

## Le menu Édition
##
## La ligne Annuler nomme ce qu’elle va reprendre : sur un historique où la prose
## et la structure se côtoient, un simple « Annuler » laisse deviner si la
## prochaine pression va retaper un mot ou ressusciter un chapitre.
menu-edit = Édi&tion
menu-edit-undo = A&nnuler
menu-edit-redo = &Rétablir
menu-edit-undo-target = A&nnuler { $target }
menu-edit-redo-target = &Rétablir { $target }
shortcut-name-edit-undo = Annuler
shortcut-name-edit-redo = Rétablir
undo-target-typing = la saisie
undo-target-project = la dernière modification du projet
prose-history-reset-title = Historique de saisie réinitialisé
prose-history-reset-body = { $count ->
    [one] Une scène ouverte a été restaurée depuis un état antérieur : son historique de saisie ne s’applique plus.
   *[other] { $count } scènes ouvertes ont été restaurées depuis un état antérieur : leur historique de saisie ne s’applique plus.
}
undo-target-trash = la mise à la corbeille
undo-target-restore = la restauration depuis la corbeille
undo-target-delete-forever = la suppression définitive
undo-target-replace-all = le remplacement dans tout le projet
undo-target-import = l’import de document
undo-target-duplicate = la duplication
undo-target-move = le déplacement
undo-target-merge = la fusion de deux scènes
undo-target-split = la division d’une scène
undo-target-promote = le changement de type
undo-target-tidy-titles = le nettoyage des titres de chapitre
undo-target-import-tags = l’import d’étiquettes
undo-target-import-templates = l’import de modèles de note
undo-target-create = la création
undo-target-remove = la suppression
undo-target-rename = le renommage
undo-target-edit = cette modification
menu-edit-find = Rec&hercher…
menu-edit-find-next = Occurrence s&uivante
menu-edit-find-prev = Occurrence précéden&te
menu-edit-replace = Rechercher et remp&lacer…
menu-edit-find-in-project = Rechercher &dans le projet…
menu-edit-replace-in-project = Remplacer dans le pro&jet…
undo-frozen = Annuler (« Toujours en avant » est actif)
redo-frozen = Rétablir (« Toujours en avant » est actif)
menu-edit-cut = Cou&per
menu-edit-copy = &Copier
menu-edit-paste = C&oller
menu-edit-paste-plain = Coller sans &mise en forme
menu-edit-select-all = Tout &sélectionner
shortcut-name-edit-cut = Couper
shortcut-name-edit-copy = Copier
shortcut-name-edit-paste = Coller
shortcut-name-edit-paste-plain = Coller sans mise en forme
shortcut-name-edit-select-all = Tout sélectionner

## Orthographe: l'interrupteur principal (barre de titre / menu Affichage / F7 / Paramètres ▸ Orthographe)
titlebar-spellcheck-on = La vérification orthographique est active. Cliquez pour l’arrêter (F7)
titlebar-spellcheck-off = La vérification orthographique est désactivée. Cliquez pour la réactiver (F7)
menu-spellcheck = &Vérifier l’orthographe
menu-comments = &Commentaires
settings-page-spellcheck = Vérification orthographique
settings-group-spellcheck = Vérification orthographique
settings-spellcheck-enabled = Vérifier l’orthographe pendant que j’écris
settings-spellcheck-hint = Souligne les mots qu’aucun dictionnaire installé ne connaît. Désactiver cette option arrête toute vérification, dans tous les projets, jusqu’à ce que vous la réactiviez. Pour ne cesser de vérifier qu’une seule langue, décochez-la dans le champ Langue de l’œuvre ou d’un élément.

## Panneau de la corbeille
menu-trash = &Corbeille
trash-title = Corbeille
trash-empty-state = La corbeille est vide.
trash-empty-button = Vider la corbeille…
trash-restore = &Restaurer
trash-restore-to = Restaurer &vers…
trash-delete-forever = &Supprimer définitivement
trash-restored-ok = { $count ->
    [one] { $count } élément restauré.
   *[other] { $count } éléments restaurés.
}
trash-restore-error = Impossible de restaurer : { $error }
trash-restore-orphaned = L’emplacement d’origine de cet élément n’existe plus — choisissez où le restaurer.
trash-restore-no-project = Aucun projet n’est ouvert, il n’y a donc rien à restaurer.
trash-delete-no-project = Aucun projet n’est ouvert, il n’y a donc rien à supprimer.
trash-empty-confirm-title = Vider la corbeille ?
trash-empty-confirm-text = { $count ->
    [one] L’élément de la corbeille sera définitivement supprimé.
   *[other] Les { $count } éléments de la corbeille seront définitivement supprimés.
} Ils quittent définitivement la corbeille. Annuler peut encore les rétablir, tant que vous n’avez rien fait d’autre.
trash-emptied-title = Corbeille vidée
trash-emptied-body = Tout le contenu de la corbeille a été définitivement supprimé.
trash-empty-no-project = Aucun projet n’est ouvert, il n’y a donc aucune corbeille à vider.
trash-delete-forever-confirm-title = Supprimer définitivement ?
trash-delete-forever-confirm-text = { $count ->
    [one] { $count } élément sera définitivement supprimé.
   *[other] { $count } éléments seront définitivement supprimés.
} Ils quittent définitivement la corbeille. Annuler peut encore les rétablir, tant que vous n’avez rien fait d’autre.
trash-deleted-title = Supprimé définitivement
trash-deleted-body = { $count ->
    [one] { $count } élément définitivement supprimé.
   *[other] { $count } éléments définitivement supprimés.
}
trash-undo = Annuler
trash-restore-picker-title = Restaurer vers…
trash-restore-to-confirm-title = Restaurer ici ?
trash-restore-to-confirm-text = Restaurer « { $item } » dans « { $destination } » ?
trash-restore-picker-restore-here = Restaurer ici
trash-restore-picker-cancel = Annuler
trash-restore-picker-empty = Aucun classeur pour l’instant — créez-en un d’abord.
trash-banner-title = Cet élément est dans la corbeille
trash-banner-description = Il n’apparaîtra ni dans le plan ni dans les exports tant que vous ne l’aurez pas restauré.
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
settings-styles-editor-missing = Ce style n’est plus disponible.

settings-styles-page-letter = Letter
settings-styles-digits-western = Occidentaux (0–9)
settings-styles-digits-eastern-arabic = Arabes orientaux (٠–٩)
settings-styles-direction-ltr = De gauche à droite
settings-styles-direction-rtl = De droite à gauche

## Dock de mise en forme
format-dock-title = Mise en forme
format-panel-empty = Placez le curseur dans une scène, une note ou un synopsis pour voir les options de mise en forme.
# En-têtes de groupe.
format-group-marks = Texte
format-group-block = Paragraphe
format-group-lists = Listes
format-group-tables = Tableau
format-group-breaks = Sauts de scène
# Infobulles des boutons. Boutons sans libellé : l'infobulle est leur seul nom
# accessible, pas une décoration.
format-superscript = Exposant
format-subscript = Indice
format-link = Lien…
# La commande Lien : un hyperlien dans le texte. Une seule commande, trois
# portes (le dock Format, le menu Format et Ctrl+K), d'où des chaînes partagées.
link-dialog-insert-title = Insérer un lien
link-dialog-edit-title = Modifier le lien
link-dialog-text-label = Texte
link-dialog-text-placeholder = les mots que le lecteur voit
link-dialog-url-label = Pointe vers
link-dialog-url-placeholder = exemple.com
link-dialog-insert = Insérer
link-dialog-apply = Appliquer
link-dialog-cancel = Annuler
link-dialog-remove = Supprimer le lien
# Refusé, sans quoi un document pourrait faire lancer un programme par un clic.
link-scheme-refused = Seuls les liens web et e-mail peuvent être ouverts. { $url } n’a pas été ouvert.
menu-format-link = Li&en…
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
menu-format-marks-clear = Effacer la mise en &forme
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

## Fenêtre « À propos »

menu-about = À &propos de Skribisto…
about-title = À propos de Skribisto
about-version = Version { $version }
about-tagline = Une application d’écriture de romans pour la fiction longue, écrite en Rust avec la boîte à outils Teksilo.
about-license = Distribué sous la Licence publique générale GNU, version 3.
about-copyright = © 2026 Cyril Jacquet
about-close = Fermer

## Barre de menus native (macOS)
# Libellés des menus standard « Application » et « Fenêtre » recopiés dans la
# barre de menus globale de macOS. Pas de mnémonique `&` ici : macOS n'en a pas,
# et le pont natif ne les retire pas — une esperluette s'afficherait telle
# quelle. Le nom de l'application est une donnée (le nom de l'édition en cours
# d'exécution) : il arrive en argument, comme pour les titres de fenêtre.
native-menu-about = À propos de { $app }
native-menu-hide = Masquer { $app }
native-menu-quit = Quitter { $app }
native-menu-settings = Paramètres…
native-menu-window = Fenêtre
native-menu-minimize = Réduire
native-menu-zoom = Zoom

# Titres de fenêtre. Le nom de l'application est une donnée (le nom de l'édition
# en cours d'exécution) : il arrive en argument plutôt que d'être écrit ici.
window-title = { $title } — { $app }
window-title-numbered = { $title } — { $app } (Fenêtre { $n })
window-title-empty = { $app }

# Import des paramètres au premier lancement (une édition disposant de son propre
# dossier de configuration, qui trouve à côté ceux de l'installation communautaire).
first-run-window-title = Bienvenue
first-run-title = Configurer { $app }
first-run-body = { $app } conserve ses paramètres séparément de Skribisto : il démarre donc vierge. Vos préférences, projets récents, dictionnaires et disposition de fenêtre peuvent être copiés dès maintenant.
first-run-from = Copier depuis
first-run-to = Copier vers
first-run-copy-note = Rien n’est déplacé ni supprimé. Skribisto conserve tous ses paramètres et continue de fonctionner exactement comme avant.
first-run-import = Importer les paramètres
first-run-start-fresh = Repartir de zéro
first-run-import-failed = Certains paramètres n’ont pas pu être importés : { $error }

# ── Work ▸ Punctuation — le style typographique du projet ───────────────────
settings-page-punctuation = Réglages de ponctuation
settings-group-punctuation = Ponctuation intelligente
settings-page-work-punctuation = Ponctuation
settings-punctuation-override = Donner à ce projet ses propres règles de ponctuation
settings-punctuation-override-hint = Désactivé : le projet suit la préférence de l’application. Ces règles voyagent dans le .skrib : un co-auteur qui ouvre le fichier écrit avec la même typographie.
settings-punctuation-dashes = Transformer -- en tiret demi-cadratin, --- en cadratin
settings-punctuation-ellipsis = Transformer ... en points de suspension
settings-punctuation-quotes = Courber les guillemets et les apostrophes
settings-quote-style = Guillemets
settings-quote-style-locale = Défaut de la langue
settings-quote-style-curly = “Doubles”
settings-quote-style-curly-single = ‘Simples’
settings-quote-style-guillemets = «Chevrons»
settings-quote-style-low-high = „Bas-haut“
settings-punctuation-spacing = Espace avant ; : ! ?
settings-punctuation-spacing-hint = La typographie française place une espace fine insécable avant le point-virgule, le point d’exclamation et le point d’interrogation, et une espace insécable avant les deux-points. Ne s’applique qu’au texte écrit en français. L’espace à l’intérieur des guillemets « » accompagne les guillemets eux-mêmes.
settings-punctuation-sample = Votre langue donne
settings-punctuation-app-hint = Ce que fait chaque projet, sauf s’il adopte ses propres règles dans Projet ▸ Ponctuation.
settings-punctuation-dialogue = Ouvrir un paragraphe saisi « - » par un tiret de dialogue
settings-punctuation-dialogue-hint = Pour les langues qui marquent le dialogue par un tiret plutôt que par des guillemets — français, espagnol, russe et d’autres. Ne se déclenche qu’en tout début de paragraphe.

## Aller à (rejoindre n'importe quel élément)
statusbar-go-to = Aller à…
go-to-placeholder = Rechercher dans le classeur
go-to-no-matches = Aucun élément ne correspond à cette recherche.
menu-go-to = &Aller à…

# ── Thèmes sans distraction (Paramètres ▸ Éditeur ▸ Thèmes sans distraction) ──
settings-page-distraction-free-themes = Thèmes sans distraction
settings-themes-builtin = Thèmes fournis
settings-themes-builtin-badge = Fourni
settings-themes-user = Mes thèmes
settings-themes-editor-group = Modifier le thème
settings-themes-editor-empty = Choisissez un thème sous « Mes thèmes » pour le modifier.
settings-themes-use = Utiliser
settings-themes-duplicate = Dupliquer
settings-themes-edit = Modifier
settings-themes-delete = Supprimer
settings-themes-export = Exporter…
settings-themes-import = Importer…
settings-themes-copy-suffix = copie
settings-themes-json-filter = Thème (JSON)
settings-themes-imported = Thème importé
settings-themes-import-failed = Impossible d’importer ce thème
settings-themes-exported = Thème exporté
settings-themes-export-failed = Impossible d’exporter ce thème
# Affiché pour un thème dont le texte et la page sont sous le seuil WCAG AA.
settings-themes-low-contrast = contraste faible
# Affiché lorsque la page et le texte vont bien, mais que c'est la mise en
# évidence qui masque la prose — un autre défaut, invisible sur les pastilles
# de la ligne.
settings-themes-low-contrast-band = la mise en évidence masque le texte
settings-themes-field-name = Nom
settings-themes-field-paper = Page
settings-themes-field-ink = Texte
settings-themes-field-general = Arrière-plan
settings-themes-field-widget-text = Texte de la bande de contrôle
# La teinte dessinée autour du curseur — la phrase ou le paragraphe en cours
# d'écriture, selon Éditeur ▸ « Mise en évidence autour du curseur ». Ce champ
# en donne la couleur en mode sans distraction.
settings-themes-field-caret-band = Mise en évidence autour du curseur

# L'engrenage de paramètres rapides de la bande sans distraction, et l'accès à la
# bibliothèque complète de thèmes depuis le mode.
statusbar-focus-settings = Paramètres du mode sans distraction
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
# Un commentaire dont l'ancrage a été résolu avec succès mais qui n'a aucune
# position réelle à indiquer — importé depuis un format sans notion de position
# dans le texte pour ce cas (un titre, un paragraphe vide, un tableau). À
# distinguer de « Texte introuvable » : rien n'a disparu, il n'y a simplement
# jamais eu de position.
comments-status-unplaced = Aucune position dans le texte
comments-unplaced-snippet = (non rattaché à un texte)
comments-reply-count = { $count ->
    [one] { $count } réponse
   *[other] { $count } réponses
}
comments-menu-resolve = Résoudre
comments-menu-reopen = Rouvrir
comments-menu-delete = Supprimer
comments-sort-document = Dans l’ordre du document
comments-sort-newest = Les plus récents d’abord
comments-menu-add = Ajouter un commentaire
comments-menu-add-paragraph = Commenter ce paragraphe
overview-col-comments = Commentaires
overview-col-total-comments = Total commentaires
comments-card-placeholder = Écrire un commentaire…
comments-card-unknown-author = Auteur inconnu
comments-unsigned-toast = Vos commentaires ne sont pas signés — aucun nom n’est défini pour vous sur cet ordinateur.
comments-unsigned-action = Définir votre nom
comments-card-reply = Répondre
comments-card-reply-placeholder = Répondre…
comments-card-actions = Actions du commentaire
comments-reply-actions = Actions de la réponse
comments-menu-delete-reply = Supprimer la réponse
comments-menu-delete-all = Supprimer tous les commentaires ici
comments-deleted-toast = Commentaire supprimé
comments-reply-deleted-toast = Réponse supprimée
comments-deleted-all-toast = { $count ->
    [one] { $count } commentaire supprimé
   *[other] { $count } commentaires supprimés
}
comments-undo = Annuler

# ── Analyse (segment du conteneur Livre) ─────────────────────────────────────
analysis-segment = Analyse
analysis-scope-book = Analyse de ce livre
analysis-run = Lancer l’analyse
analysis-stale = Modifié depuis cette analyse
analysis-not-run = Pas encore analysé.
analysis-running = Lecture du manuscrit…
analysis-failed = L’analyse n’a pas pu aboutir.
analysis-no-scenes = Aucune scène dans ce livre pour l’instant.

analysis-shape = Forme

# ── Provenance du texte ───────────────────────────────────────────────────────
analysis-arrivals = Arrivées
analysis-arrivals-explainer = Par quelle voie le texte est arrivé dans ce projet depuis son ouverture. La mesure indique le canal emprunté par les caractères, et absolument rien sur qui les a écrits : qui rédige ailleurs puis colle a collé, qui dicte a dicté. Il n’y a ici aucune valeur à viser.
analysis-arrivals-scope = L’ensemble du projet, et non ce seul livre.
analysis-arrivals-session = Depuis son ouverture. Fermer le projet remet le compte à zéro.
analysis-arrivals-nothing = Aucun texte n’est encore arrivé au cours de cette session.
analysis-arrivals-typed = Saisi au clavier
analysis-arrivals-pasted = Collé
analysis-arrivals-dictated = Dicté
analysis-arrivals-imported = Importé
analysis-arrivals-programmatic = Inséré pour vous
analysis-arrivals-count = { $count ->
    [one] { $count } caractère
   *[other] { $count } caractères
}
analysis-arrivals-none = aucun

analysis-words-per-scene = Mots par scène
analysis-median-words = La scène médiane de ce livre compte { $count } mots.
analysis-median-line = Médiane : { $count } mots
analysis-dialogue = Dialogue
analysis-dialogue-unsupported = Le dialogue n’est pas encore mesuré pour cette langue.

analysis-footnote-words = Mots en notes de bas de page
analysis-footnote-words-count = { $count } mots se trouvent dans les notes de bas de page de ce livre, comptés à part du total du manuscrit.
analysis-footnote-words-pending = Comptage des notes de bas de page…

analysis-ignore-empty = Ignorer les textes encore vides
analysis-empty-hidden = { $count } { $count ->
        [one] texte vide masqué
       *[other] textes vides masqués
    }.
analysis-all-texts-empty = Tous les textes de ce livre sont encore vides.

# ── Retour du filtre du classeur ─────────────────────────────────────────────
binder-filter-count = { $shown ->
    [one] { $shown } sur { $total } affiché
   *[other] { $shown } sur { $total } affichés
}
binder-filter-clear = Effacer
binder-filter-none = Rien ne correspond à « { $query } ».

## Settings: paratext structures
settings-paratext-intro = Les pages liminaires et les annexes d’un nouveau projet. Chaque structure appartient à une tradition éditoriale, et ses titres sont écrits dans la langue de cette tradition — renommez-les librement une fois le projet créé.
settings-paratext-structures = Structures
settings-paratext-broken = Lecture impossible
settings-paratext-edit = Modifier
settings-paratext-duplicate = Dupliquer
settings-paratext-delete = Supprimer
settings-paratext-new = Nouvelle structure
settings-paratext-save = Enregistrer
settings-paratext-editor-hint = Un nom, les pages à créer avant le manuscrit, et celles à créer après. Leur place vous appartient : ce sont des éléments ordinaires une fois le projet créé.

## Images

image-insert = Insérer une ima&ge…
image-no-project = Ouvrez un projet avant d’insérer une image.
image-choose-title = Choisir une image
image-filter-label = Images
image-large-title = Cette image est volumineuse
image-large-text =
    { $name } fait { $megapixels } mégapixels ({ $width }×{ $height }).
    La conserver telle quelle enregistre votre fichier d’origine dans le projet :
    il accompagne chaque copie de secours et chaque export. L’optimiser enregistre à la
    place une copie réduite, jusqu’à 2560 pixels sur son plus grand côté.
image-large-keep = Conserver l’original
image-large-downscale = Optimiser
image-large-remember = Faire ainsi désormais, ne plus demander
image-not-recorded = L’image a été enregistrée mais n’a pas pu être inscrite au projet.
image-describe-title = Décrire l’image
image-describe-explain = Ce que montre l’image, pour qui ne peut pas la voir. Cette description ne fait pas partie du manuscrit : elle n’est jamais comptée, recherchée ni exportée comme du texte.
image-describe-placeholder = un phare sur un ciel gris
image-resize-title = Redimensionner l’image
image-resize-explain = Un pourcentage de la taille actuellement affichée. 100 la laisse inchangée.
image-resize-invalid = Saisissez un nombre entre 1 et 1000.
image-menu-describe = &Décrire l’image…
image-menu-resize = &Redimensionner l’image…
image-menu-reset-size = Taille d’&origine

# La couverture du livre — choisie depuis le livre, non insérée dans une scène.
cover-choose = &Couverture du livre…
cover-clear = &Retirer la couverture
cover-choose-title = Choisir une couverture
cover-set = La couverture est définie.
cover-cleared = La couverture a été retirée. L’image est toujours dans le projet.
# ── Fiche de ligne du plan ──────────────────────────────────────────────────
card-label = Libellé
card-exportable = Exporté
card-numbered = Numéroté
card-created = Créé
card-modified = Modifié
card-yes = oui
card-no = non
card-type = Type
card-position = Position
card-children = Enfants
card-goal = Objectif
card-words = Mots
card-synopsis = Synopsis
card-point-of-view = Point de vue
card-aliases = Autres noms

export-orphan-footnotes-title = Notes sans appel
export-comments-dropped =
    { $count ->
        [one] Un commentaire n’a pas pu être placé dans le texte exporté et a été omis.
       *[other] { $count } commentaires n’ont pas pu être placés dans le texte exporté et ont été omis.
    }
export-orphan-footnotes =
    { $count ->
        [one] Une note de bas de page n’est plus appelée nulle part dans le manuscrit. Son texte n’apparaîtra pas dans le livre exporté.
       *[other] { $count } notes de bas de page ne sont plus appelées dans le manuscrit. Leur texte n’apparaîtra pas dans le livre exporté.
    }

## Notes de bas de page

menu-footnotes = Notes de bas de pa&ge
footnotes-title = Notes de bas de page
footnotes-insert = Insérer une note de bas de p&age
footnotes-empty = Aucune note pour l’instant. Placez le curseur dans une scène, puis utilisez + ci-dessus — ou Ctrl+Alt+F.
footnotes-filter-all = Toutes
footnotes-filter-document = Ce document
footnotes-filter-orphaned = Orphelines
footnotes-orphaned = Plus rien ne renvoie à cette note
footnotes-untitled-home = Sans titre
footnotes-body-placeholder = la note elle-même
footnotes-insert-tooltip = Insérer une note de bas de page au curseur (Ctrl+Alt+F)
footnotes-actions = Actions sur la note
footnotes-delete = &Supprimer la note et son appel
footnotes-no-project = Ouvrez un projet avant d’insérer une note de bas de page.
footnotes-no-caret = Placez le curseur dans le texte d’une scène pour y insérer une note.
footnotes-not-created = La note n’a pas pu être ajoutée au projet.
footnotes-deleted-toast = Note supprimée
footnotes-undo-delete = Annuler

# ── Importer des documents (Markdown / Word / ODT / texte brut) ───────────────
import-document-title = Importer des documents
import-document-close = Fermer
import-document-step-files = Fichiers
import-document-step-review = Vérifier
import-document-step-destination = Destination
import-document-step-reconcile = Fusion
import-document-reconcile-hint = Certaines de ces lignes sont des lignes que vous avez déjà. Indiquez ce qu’il faut faire de chacune.
import-document-reconcile-all-new = Rien dans ce fichier ne correspond à votre projet — chaque ligne sera ajoutée comme nouvelle.
import-document-duplicate-returns = Ces fichiers sont plusieurs exemplaires du même manuscrit qui reviennent. Importez-les un par un : la mise en correspondance ne confronte qu’un seul exemplaire à votre projet, si bien qu’en importer plusieurs ensemble ajouterait une seconde copie du livre au lieu de la fusionner.
import-document-hunk-added = Prendre ce nouveau paragraphe
import-document-hunk-removed = Supprimer ce paragraphe
import-document-hunk-changed = Prendre cette réécriture
import-document-col-stray-prose = Son texte
import-document-stray-as-paratext = Garder comme paratexte
import-document-stray-discard = Supprimer le texte
import-document-col-current = Dans votre projet
import-document-col-incoming = Dans ce fichier
import-document-col-status = État
import-document-col-action = Que faire
import-document-status-identical = Identique
import-document-status-editor-edited = Modifié par l’éditeur
import-document-status-you-edited = Modifié par vous
import-document-status-conflict = Modifié des deux côtés
import-document-status-different = Différent
import-document-status-new = Nouveau
import-document-status-missing = Absent de ce fichier
import-document-status-moved = Déplacé
import-document-action-comments-only = Commentaires seulement
import-document-action-take-import = Prendre cette version
import-document-action-keep-current = Garder la mienne
import-document-action-create-new = Ajouter comme nouveau
import-document-action-ignore = Ignorer
import-document-compare = Comparer
import-document-compare-legend = Votre version comparée à celle de ce fichier. Lecture seule.
import-document-compare-close = Fermer
import-document-drop-title = Déposez les documents ici
import-document-drop-hint = Markdown (.md), Word (.docx), OpenDocument (.odt) et texte brut (.txt)
import-document-browse = Parcourir…
import-document-move-up = Monter
import-document-move-down = Descendre
import-document-remove-file = Retirer
import-document-file-count = { $count ->
    [one] 1 fichier
   *[other] { $count } fichiers
}
import-document-no-files = Aucun fichier choisi pour l’instant.
import-document-col-included = Importer
import-document-col-title = Titre
import-document-col-type = Type
import-document-col-words = Mots
import-document-col-breaks = Coupures
import-document-col-comments = Commentaires
import-document-col-footnotes = Notes de bas de page
import-document-col-epigraph = Épigraphe
import-document-col-source = Source
import-document-level-rules = Niveaux de titre
import-document-level-n = Titre { $level }
import-document-add-top-level = Ajouter un niveau supérieur
import-document-add-top-level-tooltip = Insère un Livre au-dessus de toutes les lignes analysées — pour des chapitres sans titre de livre
import-document-destination = Destination
import-document-destination-hint = Choisissez un classeur ou un élément — les nouvelles lignes arrivent dans un dossier, ou après une scène.
import-document-destination-empty = Aucun classeur pour l’instant — créez-en un d’abord.
import-document-plan-empty = Rien à importer pour l’instant.
import-document-summary = { $rows ->
    [one] 1 ligne
   *[other] { $rows } lignes
} · { $breaks ->
    [one] 1 coupure de scène
   *[other] { $breaks } coupures de scène
}
import-document-back = Précédent
import-document-cancel = Annuler
import-document-analyse = Suivant
import-document-analysing = Lecture des documents…
import-document-step-analysing = Lecture
import-document-cancel-analysis = Arrêter la lecture
import-document-analyse-failed = Les documents n’ont pas pu être lus.
import-document-details = Détails
import-document-import = Importer
import-document-done = { $count ->
    [one] 1 élément importé
   *[other] { $count } éléments importés
}
import-document-undo = Annuler l’import
# ── Diagnostics d'import ──────────────────────────────────────────────────────
import-diagnostic-file-unreadable = « { $path } » n’a pas pu être lu : { $detail }. Les autres fichiers sont importés quand même.
import-diagnostic-lossy-decode = { $count ->
    [one] Un caractère de « { $path } » n’a pas pu être décodé. Enregistrez le fichier en UTF-8 pour le conserver.
   *[other] { $count } caractères de « { $path } » n’ont pas pu être décodés. Enregistrez le fichier en UTF-8 pour les conserver.
}
import-diagnostic-decoded-from-bom = « { $path } » a été décodé en { $detail }, pas en UTF-8.
import-diagnostic-empty-file = « { $path } » est vide.
import-diagnostic-no-headings = « { $path } » ne contient aucun titre : il arrive en un seul élément.
import-diagnostic-unsupported-format = Aucun lecteur ne prend en charge les fichiers « .{ $detail } » : « { $path } » a été ignoré.
import-diagnostic-front-matter-not-flat = En-tête de « { $path } » : « { $detail } » n’est pas une valeur simple et a été ignoré.
import-diagnostic-footnotes-degraded = { $count ->
    [one] Une note de bas de page de « { $path } » arrive en texte brut — les notes ne sont pas lues depuis Markdown.
   *[other] { $count } notes de bas de page de « { $path } » arrivent en texte brut — les notes ne sont pas lues depuis Markdown.
}
import-diagnostic-footnote-not-carried = { $count ->
    [one] Une note de bas de page de « { $path } » n’a pas pu être reprise — les autres l’ont été.
   *[other] { $count } notes de bas de page de « { $path } » n’ont pas pu être reprises — les autres l’ont été.
}
import-diagnostic-raw-html-dropped = { $count ->
    [one] Un bloc HTML brut de « { $path } » a été supprimé.
   *[other] { $count } blocs HTML bruts de « { $path } » ont été supprimés.
}
import-diagnostic-nested-break-dropped = { $count ->
    [one] Une séparation de scène située dans une citation ou une liste de « { $path } » a été supprimée. Seule une séparation seule sur sa ligne est conservée.
   *[other] { $count } séparations de scène situées dans des citations ou des listes de « { $path } » ont été supprimées. Seule une séparation seule sur sa ligne est conservée.
}
import-diagnostic-image-not-ingested = « { $path } » fait référence à l’image « { $detail } ». La référence arrive en texte ; l’image elle-même n’est pas copiée.
import-diagnostic-duplicate-title = « { $title } » apparaît { $count } fois. Si vous avez déjà importé ces fichiers, cela les dupliquera.
import-diagnostic-heading-level-jump = « { $title } » passe du niveau de titre { $from } au niveau { $to } ; il est placé un niveau sous son parent.
import-diagnostic-illegal-combination = « { $title } » contient du texte, mais un élément de type « { $kind } » ne peut pas en contenir. L’importation est suspendue tant que vous n’avez pas changé son type ou décoché la ligne — sinon rien du tout ne serait importé.
import-diagnostic-tracked-changes-flattened = { $path } était en cours de révision : { $count ->
    [one] { $count } modification suivie a été acceptée
   *[other] { $count } modifications suivies ont été acceptées
} et les suppressions écartées. C’est le texte final — pour garder votre propre formulation, utilisez Comparer à la dernière étape.
import-diagnostic-tracked-changes-flattened-by = { $path } était en cours de révision : { $count ->
    [one] { $count } modification suivie de { $names } a été acceptée
   *[other] { $count } modifications suivies de { $names } ont été acceptées
} et les suppressions écartées. C’est le texte final — pour garder votre propre formulation, utilisez Comparer à la dernière étape.
import-diagnostic-text-box-dropped = { $count ->
    [one] { $path } contient { $count } zone de texte. Son contenu est hors du fil du document : impossible de dire où il se place dans un manuscrit, elle n’est donc pas importée.
   *[other] { $path } contient { $count } zones de texte. Leur contenu est hors du fil du document : impossible de dire où il se place dans un manuscrit, elles ne sont donc pas importées.
}
import-diagnostic-embedded-object-dropped = { $count ->
    [one] { $path } contient { $count } objet incorporé — graphique, équation ou similaire. Rien dans un manuscrit ne peut l’accueillir.
   *[other] { $path } contient { $count } objets incorporés — graphique, équation ou similaire. Rien dans un manuscrit ne peut les accueillir.
}
import-diagnostic-field-flattened = { $count ->
    [one] { $path } contient { $count } champ — numéro de page, renvoi, date. Il conserve le texte affiché en dernier et ne se mettra plus à jour.
   *[other] { $path } contient { $count } champs — numéro de page, renvoi, date. Chacun conserve le texte affiché en dernier et ne se mettra plus à jour.
}
import-diagnostic-unknown-style-level = { $path } utilise le style « { $detail } », qui ressemble à un titre mais n’indique aucun niveau. Ces paragraphes sont importés comme texte plutôt que devinés.
import-diagnostic-comment-unanchored = Le commentaire « { $detail } » dans { $path } n’a pas pu être rattaché aux mots qu’il visait. Il est conservé sur son élément, où vous pouvez le déplacer.
import-diagnostic-comment-replies-flattened = { $count ->
    [one] { $count } réponse dans { $path } désigne un commentaire absent du fichier : elle arrive donc comme un commentaire à part entière.
   *[other] { $count } réponses dans { $path } désignent un commentaire absent du fichier : elles arrivent donc comme des commentaires à part entière.
}
import-diagnostic-epigraph-not-carried = « { $title } » est précédé d’une épigraphe, mais un élément de type « { $kind } » ne peut pas en porter. La citation est conservée en tête de son texte.
import-diagnostic-epigraph-placement-ambiguous = Une épigraphe se trouve entre « { $title } » et « { $below } » et pourrait précéder l’un ou l’autre. Elle a été attribuée à « { $title } », où se place habituellement une épigraphe.
import-document-diagnostics = { $errors ->
    [0] { $warnings ->
            [one] 1 point à connaître
           *[other] { $warnings } points à connaître
        }
   *[other] { $errors ->
            [one] 1 fichier illisible
           *[other] { $errors } fichiers illisibles
        }
}
import-document-diagnostics-none = Rien à signaler.

# ── Dock Versions ──
versions-title = Versions
versions-scope-synopsis = Synopsis
versions-scope-prose = Texte
versions-loading = Recherche dans vos copies de secours…
versions-empty = Aucune version antérieure pour l’instant
versions-error = Impossible de lire vos copies de secours — rien n’est perdu, mais cette liste peut être incomplète
versions-did-not-exist = N’existait pas encore le { $date }
versions-deleted-after = Supprimé après le { $date }
versions-unreadable = { $count ->
    [one] { $count } copie de secours illisible
   *[other] { $count } copies de secours illisibles
}
versions-thinned = { $count ->
    [one] Les anciennes versions s’espacent avec le temps — l’historique interne du projet a déjà supprimé { $count } état antérieur de ce texte.
   *[other] Les anciennes versions s’espacent avec le temps — l’historique interne du projet a déjà supprimé { $count } états antérieurs de ce texte.
}
versions-source-backup = Depuis une copie de secours
versions-source-project = Depuis l’historique du projet
versions-list-caption = Une entrée par changement, pas par copie de secours
versions-pick-a-version = Choisissez une version pour voir ce qui a changé
versions-earliest = La plus ancienne version enregistrée. Il n’y a rien de plus ancien à quoi la comparer.
versions-no-change = Rien n’a changé dans cette partie
versions-formatting-only = Seule la mise en forme a changé — les mots sont les mêmes
versions-show-unchanged = Afficher l’inchangé
versions-hide-unchanged = Masquer l’inchangé
versions-next-change = Changement suivant
versions-near = près de « { $text } »
versions-words-added = { $count ->
    [one] { $count } mot ajouté
   *[other] { $count } mots ajoutés
}
versions-words-removed = { $count ->
    [one] { $count } mot supprimé
   *[other] { $count } mots supprimés
}
versions-blocks-moved = { $count ->
    [one] { $count } paragraphe déplacé
   *[other] { $count } paragraphes déplacés
}
versions-pin = Épingler la copie de secours d’où vient cette version — le nettoyage automatique ne la supprimera jamais
versions-unpin = Désépingler la copie de secours d’où vient cette version — le nettoyage automatique pourra de nouveau la supprimer
versions-pinned-only = N’afficher que les versions épinglées
versions-pin-note = Seules les versions issues d’une copie de secours peuvent être épinglées — une épingle conserve un fichier, et l’historique interne du projet n’en est pas un.
versions-pinned-empty = Rien n’est encore épinglé ici
versions-pinned-empty-log = Rien n’est encore épinglé ici. Seules les versions issues d’une copie de secours peuvent être épinglées — une épingle conserve un fichier, et l’historique interne du projet n’en est pas un.
versions-range-filter = N’afficher que les versions comprises entre deux dates
versions-filtered-empty = Aucune version ne correspond aux filtres définis
versions-clear-filters = Effacer les filtres
versions-last-30-days = 30 derniers jours
versions-restore-button = Restaurer cette version
versions-restore-confirm-title = Remplacer ce texte par la version du { $date } ?
versions-restore-confirm-text = Ce que vous avez maintenant sera remplacé par le texte de cette ligne au { $date }.
versions-restore-confirm-with-comments = Ce que vous avez maintenant sera remplacé par le texte de cette ligne au { $date }. { $count ->
    [one] { $count } commentaire est ancré dans le texte actuel et risque de devenir orphelin.
   *[other] { $count } commentaires sont ancrés dans le texte actuel et risquent de devenir orphelins.
}
versions-restore-confirm-undo-note = Une copie de sécurité est faite d’abord, et Ctrl+Z annule tout en une fois.
versions-restored-toast = Version du { $date } restaurée
versions-undo = Annuler
versions-restore-row-gone = Cette ligne n’est plus dans ce projet
versions-restore-no-home = Cette ligne a changé de type depuis, et l’ancien texte n’y a plus sa place
versions-restore-no-safety-copy = Votre copie de sécurité n’a pas été faite, rien n’a été modifié
versions-restore-failed = La restauration a échoué : { $error }
versions-restore-backup-busy = Une copie de secours est déjà en cours — réessayez dans un instant
versions-restore-in-backup-file = Vous consultez une copie de secours ; ouvrez le projet lui-même pour y restaurer
versions-restore-no-project = Aucun projet ouvert
versions-recreate-button = Rétablir cet élément…
versions-recreate-picker-title = Où le placer ?
versions-recreate-picker-empty = Ce projet n’a aucun classeur où le placer
versions-recreate-picker-confirm = Le rétablir ici
versions-recreate-picker-cancel = Annuler
versions-recreate-untitled = cet élément
versions-recreate-confirm-title = Rétablir « { $item } » ?
versions-recreate-confirm-text = Il sera ajouté dans { $destination }, avec le texte qu’il avait le { $date }.
versions-recreate-confirm-undo-note = « Annuler », sur le message qui suit, le retire aussitôt.
versions-recreated-toast = « { $item } » est de retour dans votre projet
versions-recreated-partial-toast = { $count ->
    [one] « { $item } » est de retour, mais l’un de ses textes n’a pas pu être lu
   *[other] « { $item } » est de retour, mais { $count } de ses textes n’ont pas pu être lus
}
versions-recreate-already-here = Cet élément est déjà dans votre projet
versions-recreate-no-destination = Choisissez un emplacement dans le classeur
versions-recreate-unreadable = Cette copie de secours n’a pas pu être lue, rien n’a été ajouté
versions-recreate-failed = Impossible de le rétablir : { $error }
versions-changed-percent = { $percent } % de ce texte a changé
versions-hidden-paragraphs = { $count ->
    [one] … { $count } paragraphe inchangé …
   *[other] … { $count } paragraphes inchangés …
}

# ── Bandeau Chronologie ──
timeline-title = Remonter le temps
timeline-coverage = { $count ->
    [one] { $count } version enregistrée, remontant au { $oldest }
   *[other] { $count } versions enregistrées, remontant au { $oldest }
}
timeline-loading = Recherche dans le passé du projet…
timeline-empty = Aucune version de ce projet n’a encore été enregistrée
timeline-no-changes = Rien n’a changé depuis
timeline-no-text-changes = Aucun texte n’a changé depuis
timeline-slider-label = Version enregistrée
timeline-series-name = Taille du projet
timeline-bars-caption = Chaque barre est une version enregistrée, aussi haute que l’était le projet alors. Celle qui est mise en évidence est celle que vous consultez.
timeline-bars-caption-periods = Trop de versions pour les afficher une à une : chaque barre représente { $period }, aussi haute que l’était le projet à la fin de cette période.
timeline-unit-hour = une heure
timeline-unit-day = une journée
timeline-unit-week = une semaine
timeline-unit-month = un mois
timeline-range-filter = N’afficher que les versions comprises entre deux dates
timeline-range-empty = Aucune version n’a été enregistrée à ces dates
timeline-open-period = Ouvrir cette période
timeline-show-all = Afficher tout l’historique
timeline-changed-since = { $count ->
    [one] { $count } élément diffère entre le { $date } et votre projet actuel
   *[other] { $count } éléments diffèrent entre le { $date } et votre projet actuel
}
timeline-kind-added = Écrit depuis
timeline-kind-removed = N’est plus dans le projet
timeline-kind-changed = Modifié depuis
timeline-kind-moved = Déplacé depuis
timeline-not-yet-written = Cela n’existait pas encore à ce moment-là
timeline-no-text-of-its-own = Ceci n’a pas de texte propre — c’est un intitulé pour ce qu’il contient
timeline-reader-close = Fermer
timeline-reader-stamp = Tel quel le { $date }
timeline-reader-loading = Ouverture de la version enregistrée…
timeline-reader-unreadable = Cet enregistrement n’a pas pu être lu : la copie de secours a peut-être été déplacée ou supprimée, ou se trouve sur un disque non connecté.
timeline-reader-compared = Tel quel le { $date }, comparé à ce qu’il dit aujourd’hui
timeline-reader-diff-legend = Le texte barré a disparu depuis ; le texte souligné a été ajouté.
timeline-reader-view-label = Ce qui est affiché
timeline-reader-view-diff = Modifications
timeline-reader-view-text = Texte
timeline-reader-deleted = Cela n’est plus dans votre projet. Vous pouvez le lire et le copier ici.
timeline-prose-only-record = Ce point provient de l’historique interne du projet, qui ne conserve que le texte : ce qui a été supprimé ou déplacé depuis ne peut donc pas être affiché. Choisissez une copie de secours pour cela.

# The Settings tree's section for pages an extension contributed.
settings-sec-extensions = Extensions

# Volet Jeux d'écriture — contraintes d'écriture volontaires
settings-group-games-forward = Droit devant
settings-games-forward-toggle = Jouer à « Droit devant »
settings-games-forward-hint =
    Tant que vous jouez, rien de ce que vous avez écrit ne peut être repris :
    Retour arrière, Suppr, Couper, le glisser-déposer et Annuler sont désactivés dans
    les surfaces choisies ci-dessous. Vous pouvez toujours écrire, coller, mettre en
    forme et vous déplacer librement — le brouillon ne fait que grandir.
settings-games-session-warning =
    Ce choix ne vaut que pour la session en cours : il n’est jamais enregistré. Fermer
    le projet ou quitter Skribisto met toujours fin à la partie — et tout ce que vous
    avez écrit en jouant redevient annulable à cet instant.
settings-group-games-scope = Où cela s’applique
settings-games-in-prose = Texte du manuscrit
settings-games-in-synopsis = Synopsis
settings-games-scope-hint =
    Les commentaires, les notes de bas de page et les titres ne sont jamais figés :
    c’est là que vous notez la correction que vous venez de vous interdire.
settings-games-inert-warning =
    « Droit devant » est activé mais ne s’applique à rien — cochez au moins une surface
    ci-dessus, sinon cela ne change rien à votre écriture.

# Panneau Jeux d'écriture (rail de gauche) + l'avertissement de la barre d'état
games-title = Jeux d’écriture
games-forward-name = Droit devant
games-forward-blurb =
    Écrivez sans jamais revenir en arrière. Supprimer, couper et annuler sont
    désactivés pendant la partie — écrivez la phrase suivante plutôt que de corriger
    la précédente.
games-forward-playing = Partie en cours — la suppression est désactivée
games-forward-idle = Aucune partie en cours
games-session-note = La partie prend fin à la fermeture du projet.
games-scope-prose-and-synopsis = S’applique à votre texte et à vos synopsis.
games-scope-prose = S’applique à votre texte.
games-scope-synopsis = S’applique à vos synopsis.
games-scope-nothing = Ne s’applique à rien pour l’instant — choisissez une surface dans les paramètres.
games-settings-link = Paramètres des jeux d’écriture…
statusbar-games-forward = Droit devant
statusbar-games-forward-tooltip =
    « Droit devant » est activé : supprimer, couper et annuler sont désactivés pendant
    que vous écrivez. Cliquez pour arrêter la partie.
# ── Objectifs de mots / de caractères ─────────────────────────────────────────
# Le vocabulaire partagé par toutes les surfaces qui montrent un objectif : l'Inspecteur,
# la colonne de la Vue d'ensemble, la barre d'état, la page d'un conteneur et l'aperçu de
# la répartition. Les pluriels se décident sur le nombre brut (`$g` / `$n`) tandis que le
# chiffre affiché est la chaîne groupée : un compte écrit avec des espaces fines n'est plus
# un nombre sur lequel Fluent puisse choisir.
goal-progress-words = { $g ->
    [one] { $count } sur { $goal } mot
   *[other] { $count } sur { $goal } mots
}
goal-progress-characters = { $g ->
    [one] { $count } sur { $goal } caractère
   *[other] { $count } sur { $goal } caractères
}
goal-count-words = { $n ->
    [one] { $count } mot
   *[other] { $count } mots
}
goal-count-characters = { $n ->
    [one] { $count } caractère
   *[other] { $count } caractères
}
goal-unit-words = Mots
goal-unit-characters = Caractères

# L'objectif par élément de l'Inspecteur.
inspector-goal = Objectif
inspector-goal-none = Aucun objectif

# La colonne d'objectif de la Vue d'ensemble, et la mention portée par une ligne que
# l'export laisse de côté.
overview-col-goal = Objectif
overview-excluded-from-export = Hors export : cette ligne ne compte dans aucun total

# La page propre d'un conteneur : ce à quoi s'ajoutent les objectifs définis à l'intérieur.
# Ce n'est délibérément pas un objectif, et la formulation ne doit pas laisser croire que
# c'en est un.
goal-subtree-total-words = { $count ->
    [one] { $items } objectif à l’intérieur totalise { $words } mots
   *[other] { $items } objectifs à l’intérieur totalisent { $words } mots
}
goal-subtree-total-characters = { $count ->
    [one] { $items } objectif à l’intérieur totalise { $words } caractères
   *[other] { $items } objectifs à l’intérieur totalisent { $words } caractères
}

# Le sélecteur d'unité de comptage du panneau Nouveau projet, à côté de la langue dont il
# tire sa valeur par défaut.
new-work-goal-unit = Compter en
new-work-goal-unit-hint = Utilisé pour tous les objectifs de ce projet. Modifiable ensuite.

# Paramètres ▸ Projet ▸ Structure : l'unité de comptage du projet, et l'avertissement affiché
# avant d'en changer. Aucune conversion n'a lieu, dans un sens comme dans l'autre.
settings-group-goal-unit = Objectifs
settings-goal-unit-switch-title = Changer d’unité de comptage
settings-goal-unit-switch-text = Les objectifs et jalons déjà définis pour ce projet ont été saisis en { $from }. Passer à { $to } ne les convertit pas : chaque nombre existant sera désormais lu comme des { $to }.
settings-goal-unit-switch-informative = Rien n’est perdu. Revenez en arrière à tout moment pour retrouver la lecture d’origine, puis mettez à jour les objectifs à conserver.
settings-goal-unit-switch-confirm = Changer

# Répartition : partager l'objectif d'un conteneur entre les éléments qu'il contient.
goal-distribute-action = Répartir…
distribute-title = Répartir l’objectif
distribute-empty = Cet élément ne contient rien entre quoi répartir l’objectif.
distribute-weight = Répartir
distribute-weight-length = Selon la longueur
distribute-weight-rows = Selon le nombre d’éléments
distribute-weight-even = À parts égales
distribute-overwrite = Remplacer les objectifs déjà définis
distribute-col-item = Élément
distribute-col-current = Actuel
distribute-col-proposed = Après
distribute-total = Total : { $total }, pour un objectif de { $goal }.
distribute-over-budget = Les objectifs déjà définis à l’intérieur dépassent de { $over } celui de ce conteneur. Augmentez le sien, réduisez les leurs, ou remplacez-les.
distribute-apply = Répartir
distribute-cancel = Annuler

# Les jalons du rythme du livre : un point de passage daté, de l'une des deux sortes.
milestone-target-gone = Cible supprimée
milestone-no-target = Aucun objectif
milestone-add = Ajouter
milestone-add-label = Ce qui doit être atteint

# Le résumé du plan d'écriture, affiché une fois à l'ouverture d'un projet dont le plan est
# actif.
pace-summary-title = Où en est le livre
pace-summary-remaining = Il reste { $words }
pace-summary-open = Ouvrir le plan
pace-summary-close = Fermer
pace-summary-dont-show = Ne plus afficher à l’ouverture
pace-summary-menu = Plan d’écriture…

# ── Aide ─────────────────────────────────────────────────────────────────────
# Voir en-US/main.ftl pour le contexte. Les corps des rubriques ne sont pas ici :
# ce sont des documents Djot dans crates/teksilo_ui/help/<locale>/.
menu-help-topics = &Rubriques d’aide
menu-help-shortcuts = Raccourcis c&lavier…
menu-help-website = Skribisto sur le &Web
menu-help-report = &Signaler un problème…
menu-command-palette = Palette de &commandes…

# Les entrées du panneau Apprendre. Clés distinctes de celles du menu Aide : une
# étiquette de menu porte un mnémonique `&` qu'un bouton afficherait tel quel.
learn-help-topics = Rubriques d’aide
learn-shortcuts = Raccourcis clavier
learn-website = Skribisto sur le Web

help-window-title = Aide
help-filter-topics = Filtrer les rubriques
help-no-matching-topic = Aucune rubrique ne correspond.
help-topic-missing = Cette rubrique n’est plus disponible.
help-back = Retour
help-not-translated = Cette page n’est pas encore traduite : elle est affichée en anglais.

help-section-getting-started = Premiers pas
help-section-writing = Écriture
help-section-reviewing = Relecture
help-section-exchanging = Importer et exporter
help-section-keeping = Protéger votre travail
help-section-extensions = Extensions

help-topic-getting-started = Votre premier projet
help-topic-writing-model = Comment un livre est structuré
help-topic-drafts-and-old-versions = Conserver une version précédente
help-topic-goals-and-pace = Objectifs et rythme
help-topic-comments = Commentaires
help-topic-round-trip = Envoyer votre livre à un lecteur
help-topic-export = Exporter
help-topic-import-documents = Importer des documents
help-topic-import-projects = Faire venir un projet entier
help-topic-backups-and-versions = Copies de secours et versions

help-shortcuts-title = Raccourcis clavier
help-shortcuts-filter = Filtrer les raccourcis
help-shortcuts-no-matches = Aucun raccourci ne correspond.
help-shortcuts-rebind = Modifier les raccourcis…
help-shortcuts-close = Fermer

command-palette-placeholder = Saisissez une commande

# ── Shortcut names ───────────────────────────────────────────────────────────
# Voir en-US/main.ftl pour le contexte. Quand la même commande porte déjà un
# libellé de menu ci-dessus, ce libellé est repris à l'identique pour que le
# menu et la liste des raccourcis se répondent.
shortcut-name-binder-duplicate = Dupliquer
shortcut-name-comments-add = Ajouter un commentaire
shortcut-name-comments-add-paragraph = Commenter ce paragraphe
shortcut-name-spellcheck-toggle = Vérifier l’orthographe
shortcut-name-editor-tab-close = Fermer l’onglet
shortcut-name-editor-tab-pin = Épingler ou désépingler l’onglet
shortcut-name-editor-save = Enregistrer
shortcut-name-work-export = Exporter…
shortcut-name-work-new = Nouvelle œuvre
shortcut-name-work-open = Ouvrir une œuvre
shortcut-name-window-new = Nouvelle fenêtre
shortcut-name-work-close = Fermer l’œuvre
shortcut-name-app-settings = Paramètres
shortcut-name-app-quit = Quitter
shortcut-name-editor-insert-footnote = Insérer une note de bas de page
shortcut-name-format-scene-break = Insérer un saut de scène
shortcut-name-format-major-scene-break = Insérer un saut de scène majeur
shortcut-name-format-link = Lien…
shortcut-name-go-next = Suivant
shortcut-name-go-prev = Précédent
shortcut-name-go-to = Aller à
shortcut-name-outline-toggle = Plan
shortcut-name-preview-toggle = Aperçu de recherche
shortcut-name-view-fullscreen = Plein écran
shortcut-name-view-focus-mode = Mode sans distraction
shortcut-name-editor-size-increase = Agrandir le texte
shortcut-name-editor-size-decrease = Réduire le texte
shortcut-name-editor-size-reset = Réinitialiser la taille du texte
shortcut-name-editor-find = Rechercher
shortcut-name-editor-replace = Remplacer
shortcut-name-editor-find-next = Résultat suivant
shortcut-name-editor-find-prev = Résultat précédent
shortcut-name-search-show = Rechercher dans le projet
shortcut-name-search-replace = Remplacer dans le projet
shortcut-name-outline-open-to-side = Ouvrir sur le côté
shortcut-name-help-topics = Rubriques d’aide
shortcut-name-help-shortcuts = Raccourcis clavier
shortcut-name-help-website = Skribisto sur le Web
shortcut-name-help-report = Signaler un problème
shortcut-name-command-palette = Palette de commandes
help-section-reference = Ce que sont les choses

# Titres des entrées du glossaire dont le concept ne correspond à aucun élément
# créable, faute d'étiquette « ＋ Créer » à réutiliser. Voir help.rs::concept_topics.
help-concept-scene-break = Saut de scène
help-concept-major-scene-break = Saut de scène majeur
help-concept-find-in-prose = Repérer dans le texte
help-concept-story-bible = Bible narrative
help-concept-goal-unit = Mots ou caractères
help-concept-goal-progress = Progression
help-concept-manuscript-words = Ce qui compte comme manuscrit
help-concept-exportable = Exclu de l’export
help-concept-distribute = Répartir un objectif
help-concept-subtree-total = Total des objectifs internes
help-concept-milestone = Jalon
help-concept-pace-plan = Plan de rythme

# Titres du glossaire pour les concepts de fonctionnalités. Voir help.rs::concept_topics.
help-concept-tag = Étiquette
help-concept-label = Libellé
help-concept-point-of-view = Point de vue
help-concept-epigraph = Épigraphe
help-concept-footnote = Note de bas de page
help-concept-chapter-mode = Forme des chapitres
help-concept-comment = Commentaire
help-concept-backup = Copie de secours
help-concept-version = Version
help-concept-trash = Corbeille
help-concept-spellcheck = Vérification orthographique
help-concept-search-replace = Rechercher et remplacer
help-concept-note-template = Modèle de note
help-concept-story-bible-entry = Entrée de bible narrative
help-concept-text-replacement = Remplacement de texte
help-concept-smart-punctuation = Ponctuation intelligente
help-concept-export-style = Style d’export
help-concept-round-trip-marks = Marqueurs d’aller-retour

# Le contrôle d'attribution de l'épigraphe. Voir tabs/shared/panes.rs::attribution_control.
epigraph-mark-attribution = Ligne de source
epigraph-mark-attribution-tip = Marque la ligne où se trouve le curseur comme la source de la citation, pour qu’elle s’imprime en attribution

## Margin lane — the strip beside the scrollbar that maps a document
margin-lane-name = Repères de marge
margin-lane-provider-comments = Commentaires
margin-lane-provider-story-bible = Entrée de bible narrative
margin-lane-provider-story-bible-hint = Dans la lecture « Dans le texte » d'une note, les endroits où l'entrée est nommée, et les scènes racontées de son point de vue. Surligne aussi ces noms dans la prose elle-même.
margin-lane-mark-point-of-view = Raconté d'ici
margin-lane-mark-named-here = Nommée ici
margin-lane-provider-comments-hint = Où une note est attachée au texte
margin-lane-provider-search = Occurrences trouvées
margin-lane-provider-search-hint = Toutes les occurrences de votre dernière recherche, où qu'elles soient dans le document
margin-lane-provider-boundaries = Début de chaque document
margin-lane-provider-boundaries-hint = Un filet en tête de chaque scène d'un flux, pour savoir où vous êtes
margin-lane-provider-spelling = Orthographe
margin-lane-provider-spelling-hint = Les mots signalés par le correcteur. Désactivé sauf demande : c’est l’avis d’une machine sur votre prose
margin-lane-spelling = { $word }, faute possible
margin-lane-search-hit = { $text }, occurrence { $index } sur { $total }
margin-lane-search-current = { $text }, occurrence { $index } sur { $total }, celle où vous êtes
margin-lane-boundary = Début de { $title }
margin-lane-boundary-untitled = un document sans titre
settings-page-margin-lane = Repères de marge
settings-desc-margin-lane = La bande le long de la barre de défilement, et ce qu'elle montre
settings-margin-lane-enabled = Afficher la marge de repères
settings-margin-lane-enabled-hint = Le même interrupteur que Affichage ▸ Repères de marge.
settings-margin-lane-enabled-more = Elle ne dit jamais que quelque chose ne va pas : elle montre où sont les choses, et vous laisse en juger.
settings-group-margin-lane-marks = Ce qu'elle signale
settings-margin-lane-no-providers = Rien ne marque encore la marge.
settings-group-margin-lane-texture = Texture des dialogues
settings-margin-lane-texture = Afficher la texture des dialogues
settings-margin-lane-texture-hint = Une barre par paragraphe : sa longueur, et la part qui est parlée.
settings-margin-lane-texture-more = Non mesurée pour les langues sans convention établie, où la barre n'indique que la longueur.
settings-group-margin-lane-surfaces = Où elle apparaît
settings-margin-lane-surface-editor = Éditeur de texte
settings-margin-lane-surface-stream = Flux
settings-margin-lane-surface-search-preview = Aperçu de recherche
menu-margin-lane = Repères de &marge

## Statuts — l'échelle d'avancement du projet.
## Le nom d'un échelon est une DONNÉE du projet : ces chaînes sont résolues une seule fois,
## à la création de l'échelle, puis stockées telles quelles. Elles ne sont pas retraduites
## ensuite, et l'auteur peut renommer n'importe quel échelon.
## Les huit derniers reprennent le vocabulaire français de Plume Creator lui-même, pour
## qu'un projet importé retrouve les mots qu'il avait.
status-preset-drafting = Écriture
status-preset-passes = Passes de révision
status-preset-plume = Plume Creator
status-todo = À écrire
status-draft = Brouillon
status-revised = Révisé
status-final = Finalisé
status-outline = Plan
status-first-edit = 1re révision
status-second-edit = 2e révision
status-done = Terminé
status-plume-draft-1 = 1er brouillon
status-plume-draft-2 = 2nd brouillon
status-plume-draft-3 = 3ème brouillon
status-plume-edit-1 = 1ère édition
status-plume-edit-2 = 2nde édition
status-plume-edit-3 = 3ème édition
status-plume-proofread = Vérifié
status-plume-finished = Fini
new-work-statuses = Avancement
new-work-statuses-hint = Les étapes par lesquelles passe une scène. Vous pourrez les renommer, les réordonner ou en ajouter à tout moment.
status-none = Aucun statut
inspector-status = Statut
overview-col-status = Statut
overview-status-mixed = les parties en dessous divergent
help-concept-status = Statut
status-completion-title = Où en est le livre
status-completion-headline = { $done } scènes terminées sur { $total }
status-completion-empty = Aucune scène pour l'instant — cette vue lit le manuscrit, elle se remplira à mesure que vous écrirez.
status-completion-open = Où en est le livre…

## Réglages ▸ Projet ▸ Statuts — l'éditeur d'échelle.
## L'auteur possède le NOM d'un échelon et l'ORDRE de l'échelle ; l'application possède sa
## catégorie, et la catégorie possède le symbole et la couleur. Seuls les noms de catégorie
## ci-dessous sont traduits — le nom d'un échelon est le texte de l'auteur et reste tel quel.
settings-page-statuses = Statuts
settings-desc-statuses = Renommer, réordonner, ajouter et supprimer les étapes par lesquelles passe une scène
settings-statuses-add = Ajouter un statut
settings-statuses-add-placeholder = Nom du nouveau statut
settings-statuses-desc =
    Les étapes par lesquelles passe une scène, dans l'ordre — de la moins avancée en haut à
    la plus avancée en bas. Cet ordre définit ce que « moins avancé que » signifie partout
    ailleurs dans l'application : placez-les selon votre processus réel.
settings-statuses-apply-preset = Appliquer un préréglage…
settings-statuses-preset-applied = { $added ->
    [one] 1 statut ajouté
   *[other] { $added } statuts ajoutés
}
settings-statuses-preset-refused = Ce projet a déjà une échelle. Les préréglages ne remplissent qu'une échelle vide — supprimez d'abord les échelons dont vous ne voulez pas.
settings-statuses-duplicate = « { $name } » figure déjà sur cette échelle
settings-statuses-added = « { $name } » ajouté
settings-statuses-move-up = Monter (moins avancé)
settings-statuses-move-down = Descendre (plus avancé)
settings-statuses-details-placeholder = Ce que signifie cette étape (facultatif)
settings-statuses-delete = Supprimer « { $name } »
settings-statuses-delete-in-use = { $count ->
    [one] Supprimer « { $name } » — { $count } élément le porte et perdra son statut
   *[other] Supprimer « { $name } » — { $count } éléments le portent et perdront leur statut
}
settings-statuses-deleted = « { $name } » supprimé
settings-statuses-deleted-in-use = { $count ->
    [one] « { $name } » supprimé. { $count } élément n'a plus de statut.
   *[other] « { $name } » supprimé. { $count } éléments n'ont plus de statut.
}
settings-statuses-empty-title = Aucun processus défini
settings-statuses-empty-body = Un statut indique où en est une scène. Ajoutez-en un ci-dessus, ou partez d'un préréglage.
status-category-planned = Prévu
status-category-drafting = Brouillon
status-category-needs-work = À reprendre
status-category-revised = Révisé
status-category-final = Terminé
