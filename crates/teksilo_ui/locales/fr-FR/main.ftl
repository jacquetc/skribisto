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
menu-import-scrivener = &Scrivener (.scriv)…
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
menu-focus-mode = Mode sans &distraction
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
# Trois lignes sont exclusives deux à deux : la division, le déplacement et
# l'épinglage n'en construisent qu'une chacun, d'où des lettres qui peuvent se
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
ctx-tab-move-window-unsaved = Enregistrez d’abord ce projet : une seconde fenêtre s’ouvre sur un fichier du disque
ctx-tab-pin = Épin&gler cet onglet
ctx-tab-unpin = Désépin&gler cet onglet
# Infobulle de l'onglet, pas une ligne de menu : aucun mnémonique.
tab-pinned-tooltip = Épinglé : « Fermer les autres » et « Fermer tout » le laissent ouvert

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
## Donnée, et non habillage : résolu une seule fois à la création puis propriété
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
session-readout = { $words ->
    [one] { $words } mot
   *[other] { $words } mots
} · { $time }
session-readout-timed = { $words ->
    [one] { $words } mot
   *[other] { $words } mots
} · { $time } restant
# La bande de contrôle toujours visible du mode sans distraction (étape 2).
statusbar-focus-exit = Quitter le mode sans distraction
# Les boutons Suivant/Précédent de la bande (étape 4) déclenchent les mêmes
# actions go.next/go.prev que le raccourci et la paire générique du menu Aller à.
statusbar-focus-synopsis = Synopsis
statusbar-focus-comments = Commentaires
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
settings-check-for-updates = Rechercher les nouvelles versions
settings-check-for-updates-tip = Une fois par jour au maximum, Skribisto demande au site du projet quelle est la version actuelle. La requête ne transporte aucun identifiant, ni même la version que vous utilisez : la comparaison a lieu sur cet ordinateur.

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
settings-sec-backup = Copies de secours et synchro
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
# Une ligne par page, affichée sous son lien dans la page de son parent et,
# pour un parent, sous son propre titre. Une seule ligne : elles disent ce que
# la page contient, pas comment s'en servir.
settings-desc-sec-appearance-behaviour = L’aspect de l’application elle-même, et ce qu’elle fait au démarrage.
settings-desc-sec-editor = La surface d’écriture : son aspect, et ce qu’elle fait pendant que vous tapez.
settings-desc-sec-spelling = La vérification orthographique et les dictionnaires qui la nourrissent.
settings-desc-sec-backup = Comment votre travail atteint le disque, et quelles copies sont conservées.
settings-desc-sec-compile = Ce qui sort de Skribisto, et sous quelle forme.
settings-desc-sec-work = Les paramètres propres à ce projet, qui voyagent dans son fichier.
settings-desc-sec-extensions = Les pages ajoutées par les extensions installées ici.
settings-desc-group-typography = Une page par type de texte : police, taille, interligne et espacements.
settings-desc-appearance = Langue de l’interface, thème, taille du texte et lanceur au démarrage.
settings-desc-notifications = Tous les messages affichés pendant cette session, et les actions à rejouer.
settings-desc-scene = La mise en page des scènes : police, taille, interligne, retraits et espacements.
settings-desc-synopsis = La mise en page du volet synopsis, indépendante du manuscrit.
settings-desc-notes = La mise en page des notes, indépendante du manuscrit.
settings-desc-corkboard = La taille des cartes, ce qu’elles montrent, et la disposition du tableau.
settings-desc-distraction-free = Typographie, largeur de colonne et bande de contrôle du plein écran.
settings-desc-distraction-free-themes = La bibliothèque de thèmes du plein écran : ceux fournis et les vôtres.
settings-desc-editor-behavior = Largeur du texte, place du synopsis, défilement machine à écrire et surlignage du curseur.
settings-desc-punctuation = Le style typographique dont héritent les nouveaux projets. Chaque projet suit ces réglages, sauf s’il définit les siens.
settings-desc-goals = Les objectifs en mots et en caractères, et la façon de compter les mots.
settings-desc-games = Les contraintes que vous vous imposez en écrivant, comme « Droit devant ».
settings-desc-spellcheck = L’interrupteur unique qui active ou coupe la vérification orthographique partout.
settings-desc-dictionaries = Installer, retirer et parcourir les dictionnaires présents sur cette machine.
settings-desc-autosave = Si vos modifications sont écrites sur le disque toutes seules.
settings-desc-backup = Quand les copies sont prises, où elles sont rangées, et combien sont gardées. Chaque projet suit ces réglages, sauf s’il définit les siens.
settings-desc-export = Les styles par lesquels passe chaque export : ceux fournis et les vôtres.
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
settings-field-user-hint = Les deux sont facultatifs et valent pour tous les projets de cet ordinateur. Les modifier signe les prochains commentaires que vous écrirez. Ceux déjà écrits gardent le nom sous lequel ils l’ont été.
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
settings-group-updates = Mises à jour
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
editor-size-changed-corkboard = Taille du texte des cartes : { $percent }
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
settings-distraction-free-width-hint = S’applique uniquement en mode sans distraction : les largeurs de colonne de la Scène, du Synopsis et des Notes restent inchangées.
settings-distraction-free-title = Conserver le nom de l’élément
settings-distraction-free-word-count = Conserver le compteur de mots
settings-distraction-free-session = Conserver la session d’écriture
settings-distraction-free-go-to = Conserver le bouton Aller à…
settings-distraction-free-go = Conserver les boutons Précédent et Suivant
settings-distraction-free-chrome-hint = Le bouton Quitter reste toujours affiché, quels que soient ces choix : c’est votre porte de sortie si Échap est déjà pris.
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
settings-field-author-hint = Apparaît sur la page de titre compilée et dans les métadonnées des fichiers exportés. Laissez vide pour l’omettre. C’est la signature du livre. Le nom qui signe vos commentaires se trouve dans Paramètres ▸ Utilisateur.
settings-group-chapters = Chapitres
tidy-titles-title = Nettoyer les titres de chapitre
tidy-titles-none = Aucun titre de chapitre ou de partie ne se contente de répéter son propre numéro.
tidy-titles-lead = { $count ->
        [1] Un chapitre ou une partie n’a pour titre que son propre numéro.
       *[other] { $count } chapitres et parties n’ont pour titre que leur propre numéro.
    }
tidy-titles-explain = Effacer ces titres laisse chacun désigné par le numéro que le livre connaît déjà, celui-là même qu’imprime l’export. Rien d’autre ne change, et une seule annulation les rétablit tous.
menu-document-tidy-titles = &Nettoyer les titres de chapitre…
settings-group-numbering = Numérotation
settings-number-chapters = Numéroter les chapitres et les parties
settings-number-chapters-tip = Les chapitres et les parties portent un numéro déduit de leur place dans le livre, affiché ici à côté de leur titre, et imprimé par l’export.
settings-number-chapters-tip-more = Le numéro n’est jamais enregistré dans le titre : il reste juste quand vous réorganisez, insérez ou supprimez. Désactivez ceci et l’export n’imprime que les titres, quel que soit le style d’export. Pour n’exclure qu’un seul chapitre (un prologue, un interlude), utilisez plutôt l’interrupteur Numérotation de l’inspecteur : il le garde dans le livre mais l’empêche de prendre un numéro.
settings-part-resets-chapter = Recommencer la numérotation à chaque partie
settings-part-resets-chapter-tip = Désactivé par défaut : les chapitres se suivent d’une partie à l’autre, si bien que la « Deuxième partie » s’ouvre sur le chapitre onze.
settings-part-resets-chapter-tip-more = C’est l’usage de l’édition, et ce qu’attend un lecteur. Activez-le pour un livre dont les parties se lisent comme des volumes distincts, chacune s’ouvrant sur le chapitre un.
settings-chapter-flat = Chapitres à plat
settings-chapter-flat-hint = Activé : un chapitre est un seul élément. Vous y écrivez, et il ne contient aucune scène. Désactivé : un chapitre est un dossier. Vous y écrivez également, mais il peut en outre contenir des scènes. Les nouveaux chapitres suivent ce paramètre ; les existants se convertissent via Promouvoir.

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
menu-image = I&mage
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
# Affiché sous la version dans la barre latérale du lanceur, seulement si une version plus récente existe.
welcome-update-available = La version { $version } est disponible
welcome-update-tip = Ouvre la page de téléchargement dans votre navigateur.
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
corkboard-layout-hint = « Imbriqué » affiche les enfants directs d’un conteneur : ouvrez une carte dossier pour y entrer. « À plat » affiche toutes les scènes du conteneur d’un coup.
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
corkboard-move-picker-empty = Aucun classeur pour l’instant. Créez-en un d’abord.
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
    [one] { $count } élément
   *[other] { $count } éléments
}
overview-search-placeholder = Filtrer les éléments…
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
pace-daily-target-line = Rythme régulier : { $count ->
    [one] { $count } mot/jour
   *[other] { $count } mots/jour
}
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
# Pré-rempli dans la boîte « Nouveau classeur », pour n'avoir qu'à valider.
# Stocké littéralement ensuite : un nom de classeur est une donnée, pas un libellé.
binder-default-name = Nouveau classeur
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
dialog-new-binder = Nouveau classeur
dialog-set-label = Définir le libellé
dialog-new-scene = Nouvelle scène
close-work-question = Enregistrer les modifications avant de fermer l’œuvre ?
quit-question = Enregistrer les modifications avant de quitter ?
quit-save-work-question = Enregistrer les modifications de { $title } avant de quitter ?
unsaved-changes = Cette œuvre a des modifications non enregistrées.
# Affiché par Quitter quand une autre œuvre ouverte (pas celle de cette fenêtre)
# a encore des modifications non enregistrées. Le dialogue unique listant
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
# Un projet enregistré par une version de Skribisto plus récente que celle-ci : titre et
# corps. $written_by est la version qui l'a écrit, $requires la plus ancienne version
# capable de l'ouvrir (les deux diffèrent lorsque la version récente n'a rien utilisé de
# nouveau), $supported la plus récente que cette version comprend.
could-not-open-work-too-new = « { $file } » nécessite une version plus récente de Skribisto
could-not-open-work-too-new-detail = Enregistré au format Skribisto { $written_by } ; son ouverture nécessite le format { $requires } ou plus récent, or cette version ne prend en charge que le format { $supported } au maximum. Mettez Skribisto à jour pour l’ouvrir.
could-not-open-work-importing = « { $file } » est en cours d’écriture par un import
could-not-open-work-importing-detail = Un import réécrit ce projet et n’est pas terminé. Ouvrez-le une fois l’import achevé : ouvert maintenant, il montrerait le projet que l’import remplace, et son prochain enregistrement réécrirait ce projet par-dessus l’import.
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
save-flush-failed = L’enregistrement a eu lieu, mais ces documents n’ont pas pu lui être transmis et restent non enregistrés : { $titles } ({ $error })
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
backup-failed-close-error-text = Aucune copie de secours n’a pu être écrite avant la fermeture : { $error }. Corrigez le problème et réessayez, ou quittez sans créer de copie de secours.
backup-no-result = La copie de secours est terminée sans indiquer si elle a été créée. Consultez la liste des copies de secours pour vérifier si une nouvelle s’y trouve.

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
settings-page-backup = Réglages des copies de secours
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
new-work-language-hint = Appliquée aux nouveaux textes et à la correction orthographique. Chaque texte peut être basculé vers une autre langue.
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
new-work-bundle-desc = Un dossier contenant chaque texte et chaque ressource. Adapté au contrôle de version.
# Décomptes des modèles
new-work-template-none-count = classeur vide
new-work-template-empty-novel-count = 1 chapitre
new-work-template-light-novel-count = 15 chapitres
new-work-template-novel-count = 20 chapitres
new-work-template-novel-in-parts-count = 3 parties, 24 chapitres
new-work-template-notebook-count = notes libres
# Bascule ChapterScene (modèles de roman)
new-work-chapter-scene = Chapitres à plat
new-work-chapter-scene-tip = Chaque chapitre est un seul élément où vous écrivez directement, sans scène en dessous. Laissez désactivé pour la disposition classique : le chapitre est alors un dossier, dans lequel vous écrivez tout autant, mais qui peut aussi contenir des scènes.
new-work-chapter-scene-tip-more = Vous écrivez dans le chapitre dans les deux cas. La seule différence est sa capacité à *contenir* des scènes. L’arborescence du classeur de Skribisto est purement organisationnelle : les deux dispositions produisent le même livre, vous pouvez les mélanger librement, et Promouvoir convertit un chapitre de l’une à l’autre sans perdre un mot.
# Validation des champs
new-work-name-required = Saisissez un nom pour l’œuvre
new-work-name-invalid = Ce nom ne contient aucun caractère utilisable
new-work-location-required = Choisissez un emplacement
new-work-location-missing = Ce dossier n’existe pas
new-work-location-not-folder = Ce chemin n’est pas un dossier
new-work-location-readonly = Ce dossier n’est pas accessible en écriture
new-work-target-exists = Un projet de ce nom se trouve déjà dans ce dossier. La création de l’œuvre demandera confirmation avant de le remplacer
new-work-target-is-folder = Un dossier de ce nom se trouve déjà à cet emplacement. Choisissez un autre nom
new-work-overwrite-title = Remplacer le projet existant ?
new-work-overwrite-text = « { $name } » existe déjà dans ce dossier. Le remplacer par le nouveau projet ?
new-work-target-open-title = Ce projet est ouvert
new-work-target-open-text = « { $name } » est ouvert dans Skribisto. Créer un nouveau projet à sa place remplacerait le projet sur lequel vous travaillez, et votre prochain enregistrement réécrirait ce projet par-dessus le nouveau. Choisissez un autre nom ou un autre dossier, ou fermez d’abord le projet.
target-importing-title = Un import écrit ce projet
target-importing-text = Un import écrit encore « { $name } ». Attendez qu’il soit terminé, ou choisissez un autre nom.
backup-restore-importing-text = Un import écrit encore « { $name } », le projet sur lequel cette copie de secours serait restaurée. Restaurez-la une fois l’import terminé.

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
new-work-note-templates-hint = La forme de départ d’une note de la bible narrative. Facultatifs, et chaque modèle peut ensuite être modifié ou supprimé.
new-work-characters = Personnages
new-work-places = Lieux

## Boîte de dialogue d'import Plume Creator
import-plume-title = Importer un projet Plume Creator
import-plume-source = Projet Plume
import-plume-source-hint = Choisissez un fichier .plume ou .plume_backup (n’importe quelle version de Plume Creator).
import-plume-trash-warning = ⚠ Les éléments à la corbeille / supprimés ne sont pas migrés.
# Validation des champs
import-plume-source-required = Choisissez un fichier de projet Plume
import-plume-source-missing = Ce fichier n’existe pas
import-plume-source-not-file = Ce chemin n’est pas un fichier
# Noms de classeurs transmis au backend (qui ne fait pas d'i18n)
import-plume-manuscript-binder = Manuscrit
import-plume-story-bible-binder = Bible narrative
import-plume-notes-of = { $name } (notes)
# Le titre de la scène qui reçoit le texte propre d’un dossier Plume Creator, après le nom
# du dossier. Enregistré tel quel dans le nouveau projet.
import-plume-content-of = { $name } (texte)
# Toast de progression (l'import est une opération longue)
import-plume-progress-title = Importation du projet Plume…
# Résultat
import-plume-done = { $imported ->
    [one] { $imported } élément importé.
   *[other] { $imported } éléments importés.
} { $skipped ->
    [one] { $skipped } élément à la corbeille n’a pas été migré.
   *[other] { $skipped } éléments à la corbeille n’ont pas été migrés.
}
# Pourquoi un projet Plume Creator n’a pas été importé. $part est le nom de la
# partie dans le projet (tree, attendance, info) ; $limit est le plafond, un nombre.
import-plume-nested-too-deep = Le projet n’a pas été importé : sa partie « { $part } » est imbriquée sur plus de { $limit } niveaux. Aucun projet Plume Creator n’en approche, et la lire pourrait provoquer l’arrêt brutal de Skribisto. Rien n’a été écrit.
# Projet refusé sans être lu : l’une de ses parties déclare des entités XML,
# qui pourraient occuper plus de mémoire que n’en a l’ordinateur une fois
# développées. $part est le nom de la partie (tree, attendance, info).
import-plume-declares-entities = Le projet n’a pas été importé : sa partie « { $part } » déclare des entités XML, ce qu’aucun projet Plume Creator ne fait. Les développer pourrait demander plus de mémoire que n’en a l’ordinateur et provoquer l’arrêt brutal de Skribisto. Rien n’a été écrit.
import-plume-no-outline = Ce fichier est une archive compressée, mais pas un projet Plume Creator : il n’a pas de plan. Rien n’a été écrit.
import-plume-empty-outline = Le fichier de plan de ce projet Plume Creator est vide : il n’y a rien à importer. Rien n’a été écrit.
# $tag est le nom du document que la partie s’est révélée être.
import-plume-wrong-document = Ce n’est pas un projet Plume Creator : sa partie « { $part } » est un document « { $tag } ». Rien n’a été écrit.
# $version est la version de la partie, $supported la plus récente que l’import lit.
import-plume-too-new = La partie « { $part } » de ce projet a été écrite par un Plume Creator plus récent (version { $version }) que ce que Skribisto sait lire (jusqu’à { $supported }). Rien n’a été écrit.

## Boîte de dialogue d'import de projet Manuskript
import-manuskript-title = Importer un projet Manuskript
import-manuskript-source = Projet Manuskript
# Un projet Manuskript est un fichier .msk plus, dans son mode habituel, un
# dossier du même nom à côté. L'un ou l'autre convient, le dossier aussi.
import-manuskript-source-hint = Choisissez le fichier .msk ou le dossier du projet (n’importe quelle version de Manuskript).
import-manuskript-source-file = Choisir un fichier…
import-manuskript-source-folder = Choisir un dossier…
# Validation des champs
import-manuskript-source-required = Choisissez un projet Manuskript
import-manuskript-source-missing = Ce fichier ou ce dossier n’existe pas
# Noms transmis au backend (qui ne fait pas d'i18n). Manuskript n'en stocke
# aucun : il n'a ni classeurs ni groupes de bible, et son échelle d'importance
# est faite de trois nombres dont les noms vivent dans son interface.
import-manuskript-manuscript-binder = Manuscrit
import-manuskript-story-bible-binder = Bible narrative
import-manuskript-characters-group = Personnages
import-manuskript-world-group = Univers
import-manuskript-plots-group = Intrigues
import-manuskript-project-info-note = Informations du projet
import-manuskript-summary-note = Résumé
import-manuskript-importance-minor = Mineur
import-manuskript-importance-secondary = Secondaire
import-manuskript-importance-main = Principal
import-manuskript-notes-of = { $title } (notes)
# Toast de progression (l'import est une opération longue)
import-manuskript-progress-title = Importation du projet Manuskript…
# Résultat
import-manuskript-done = { $imported ->
    [one] { $imported } élément importé.
   *[other] { $imported } éléments importés.
} { $revisions ->
    [0] { "" }
    [one] { $revisions } version antérieure a suivi.
   *[other] { $revisions } versions antérieures ont suivi.
}
# Pourquoi un projet Manuskript n’a pas été importé. $part est le nom de ce
# fichier dans le projet (world.opml, plots.xml…) ; $limit est le plafond, un nombre.
import-manuskript-nested-too-deep = Le projet n’a pas été importé : son fichier « { $part } » est imbriqué sur plus de { $limit } niveaux. Aucun projet Manuskript n’en approche, et le lire pourrait provoquer l’arrêt brutal de Skribisto. Rien n’a été écrit.
# L’un de ses fichiers est rangé dans des dossiers imbriqués au-delà de ce que
# l’application accepte (Manuskript range le plan d’un projet en dossiers, un par niveau).
import-manuskript-folders-too-deep = Le projet n’a pas été importé : l’un de ses fichiers se trouve à plus de { $limit } niveaux de dossiers de profondeur. Aucun projet Manuskript n’en approche, et le lire pourrait provoquer l’arrêt brutal de Skribisto. Rien n’a été écrit.
# Projet refusé sans être lu : l’un de ses fichiers déclare des entités XML,
# qui pourraient occuper plus de mémoire que n’en a l’ordinateur une fois
# développées. $part est le nom de ce fichier dans le projet (world.opml,
# plots.xml…).
import-manuskript-declares-entities = Le projet n’a pas été importé : son fichier « { $part } » déclare des entités XML, ce qu’aucun projet Manuskript ne fait. Les développer pourrait demander plus de mémoire que n’en a l’ordinateur et provoquer l’arrêt brutal de Skribisto. Rien n’a été écrit.
import-manuskript-not-a-project-file = « { $file } » n’est ni l’archive d’un projet Manuskript ni le petit fichier .msk qui désigne le dossier d’un projet. Rien n’a été écrit.
import-manuskript-folder-missing = « { $stub } » désigne un dossier de projet nommé « { $folder } » à côté de lui, et ce dossier n’y est pas. Un projet Manuskript en mode dossier est fait des deux, qui doivent voyager ensemble. Rien n’a été écrit.
import-manuskript-not-a-project = Ceci ne ressemble pas à un projet Manuskript : il n’a ni marqueur Manuskript ni plan. Rien n’a été écrit.
import-manuskript-format-too-new = Ce projet est au format Manuskript { $version }, plus récent que ce que Skribisto sait lire (jusqu’à { $supported }). Rien n’a été écrit.
import-manuskript-too-many-files = Le projet n’a pas été importé : il contient plus de { $limit } fichiers, bien plus que n’importe quel projet Manuskript. Rien n’a été écrit.
import-manuskript-too-large = Le projet n’a pas été importé : il dépasse les { $limit } Mo que Skribisto lit d’un projet Manuskript. Rien n’a été écrit.
project-import-close = Fermer
project-import-location = Dossier de destination
# Le titre de la boîte de choix de dossier, comme les clés dialog-*-title : un titre de
# fenêtre (infinitif), pas la consigne de project-import-location-required.
project-import-location-title = Choisir un dossier de destination
project-import-name = Nom du fichier
project-import-name-placeholder = Nom du projet
project-import-will-create = Créera
project-import-cancel = Annuler
project-import-import = Importer
# Validation des champs
project-import-location-required = Choisissez un dossier de destination
project-import-location-missing = Ce dossier n’existe pas
project-import-location-not-folder = Ce chemin n’est pas un dossier
project-import-location-readonly = Ce dossier n’est pas accessible en écriture
project-import-name-required = Saisissez un nom de fichier
project-import-name-exists = Un fichier de ce nom existe déjà ici. L’import demandera confirmation du remplacement
project-import-name-is-folder = Un dossier de ce nom est déjà ici, peut-être un projet gardé en dossier. Un import ne peut pas le remplacer : choisissez un autre nom
# Confirmation de remplacement
project-import-overwrite-title = Remplacer le fichier existant ?
project-import-overwrite-text = « { $name } » existe déjà. Le remplacer par le projet importé ?
# Les projets s’importent un à la fois, quel que soit l’import qui le demande.
project-import-busy-title = Un autre projet est en cours d’import
project-import-busy-text = Les projets s’importent un à la fois. Attendez que l’import en cours soit terminé, ou annulez-le, puis relancez celui-ci.
# Lu par un lecteur d’écran une fois par étape, jamais affiché : le titre du
# toast, puis le nom de l’étape.
project-import-progress-announcement = { $title } { $phase }
# Le toast une fois « Annuler » pressé, jusqu’à l’arrêt effectif de l’import : il
# termine peut-être un fichier qu’il ne peut pas laisser à moitié écrit.
project-import-stopping = Arrêt de l’import…
project-import-cancelled = Importation annulée
project-import-open-now = Ouvrir maintenant
project-import-details = Détails
# Le compte rendu de l’import : ce que l’auteur doit savoir de ce qui est arrivé.
project-import-report = { $count ->
    [one] { $count } chose à savoir sur cet import
   *[other] { $count } choses à savoir sur cet import
}
project-import-report-title = À propos de cet import
# Toast d'erreur : motif court dans le corps, chaîne technique complète derrière « Détails »
project-import-error-title = Impossible d’importer le projet
# Pourquoi un projet n’a pas été importé, quel que soit l’import qui l’a lu. $file
# est un nom de fichier, $folder un chemin complet (c’est le dossier sur lequel
# l’auteur doit agir), $reason ce qu’a dit le système ou un analyseur, $limit un nombre.
project-import-failed-cancelled = L’import a été annulé. Rien n’a été écrit.
project-import-failed-target-exists = « { $file } » existe déjà, et l’import n’avait pas le droit de le remplacer. Rien n’a été écrit.
project-import-failed-target-folder = « { $file } » est un dossier, peut-être un projet gardé en dossier, et un import ne peut pas le remplacer. Choisissez un autre nom. Rien n’a été écrit.
project-import-failed-not-written = « { $file } » n’a pas pu être écrit dans « { $folder } » ({ $reason }). Rien n’a été écrit.
project-import-failed-disk-full = Il ne reste pas assez de place dans « { $folder } » pour écrire « { $file } ». Libérez de la place à cet endroit, puis relancez l’import. Rien n’a été écrit.
project-import-failed-source-missing = « { $file } » n’est plus là. Rien n’a été écrit.
project-import-failed-unreadable = Skribisto n’a pas pu lire « { $file } » ({ $reason }). Rien n’a été écrit.
project-import-failed-archive-damaged = « { $file } » est un fichier compressé qui n’a pas pu être lu, sans doute parce qu’il est endommagé ({ $reason }). Rien n’a été écrit.
# $entries et $limit sont des nombres de fichiers ; $entries dépasse toujours $limit.
project-import-failed-archive-entries = Le projet n’a pas été importé : son archive liste { $entries ->
    [one] { $entries } fichier
   *[other] { $entries } fichiers
}, plus que les { $limit } qu’un projet peut contenir, ce que seul un fichier conçu pour épuiser la mémoire fait. Rien n’a été écrit.
project-import-failed-archive-member = Le projet n’a pas été importé : sa partie « { $part } » se décompresse en plus de { $limit } Mo, ce que seul un fichier conçu pour épuiser la mémoire fait. Rien n’a été écrit.
project-import-failed-archive-size = Le projet n’a pas été importé : il se décompresse en plus de { $limit } Mo, ce que seul un fichier conçu pour épuiser la mémoire fait. Rien n’a été écrit.
project-import-failed-nested = Le projet n’a pas été importé : sa partie « { $part } » est imbriquée sur plus de { $limit } niveaux, et la lire pourrait provoquer l’arrêt brutal de Skribisto. Rien n’a été écrit.
project-import-failed-entities = Le projet n’a pas été importé : sa partie « { $part } » déclare des entités XML. Les développer pourrait demander plus de mémoire que n’en a l’ordinateur et provoquer l’arrêt brutal de Skribisto. Rien n’a été écrit.
project-import-failed-folders = Le projet n’a pas été importé : l’un de ses fichiers se trouve à plus de { $limit } niveaux de dossiers de profondeur, et le lire pourrait provoquer l’arrêt brutal de Skribisto. Rien n’a été écrit.
project-import-failed-malformed = Le projet n’a pas été importé : sa partie « { $part } » est endommagée et ne peut pas être lue ({ $reason }). Rien n’a été écrit.
# Un projet Scrivener que l’import ne peut pas reprendre (un projet de Scrivener 1 pour Mac,
# un fichier de classeur endommagé). Dire pourquoi revient à l’import Scrivener ; les
# détails en donnent la clé.
project-import-failed-scrivener-refused = Skribisto ne peut pas importer ce projet Scrivener. Rien n’a été écrit.
# Le projet Scrivener pose une question que sa vérification n’a pas tranchée : un projet
# ou un fichier de classeur y est apparu depuis. Affiché sans bouton.
project-import-failed-scrivener-question = Le projet Scrivener pose une question à laquelle sa vérification n’a pas répondu. Relancez son import et répondez-y, puis importez. Rien n’a été écrit.
# Le projet Scrivener demande à l’auteur de cocher une case avant l’import (l’importer sans
# les modifications faites sur iPhone ou iPad, ou avec ses documents vides), et l’import a
# été lancé sans elle. Affiché sans bouton.
project-import-failed-scrivener-not-accepted = Avant de pouvoir être importé, le projet Scrivener vous demande de cocher une case à l’étape Projet, et elle n’a pas été cochée. Relancez son import et cochez-la, puis importez. Rien n’a été écrit.
# Le projet Scrivener a changé après que l’auteur l’a vérifié. Affiché sans bouton.
project-import-failed-scrivener-changed = Le projet Scrivener a changé depuis que vous l’avez vérifié. Relancez son import et vérifiez-le de nouveau, puis importez. Rien n’a été écrit.
# Une erreur que rien ci-dessus ne prévoit ; $detail en est la description technique.
project-import-failed-internal = L’import s’est arrêté sur une erreur que Skribisto n’attendait pas : { $detail }

## Import de projet, commun aux boîtes de dialogue Plume Creator et Manuskript
# La destination choisie est un projet ouvert dans une fenêtre, ici ou dans une autre
# instance de Skribisto. L’import est refusé plutôt qu’écrit par-dessus.
import-target-open-title = Ce projet est ouvert
import-target-open-text = « { $name } » est ouvert dans Skribisto. Un import à cet emplacement remplacerait le projet sur lequel vous travaillez, et votre prochain enregistrement réécrirait ce projet par-dessus l’import. Choisissez un autre nom de fichier, ou fermez d’abord le projet.
# Lu par un lecteur d’écran quand le compte rendu d’un import apparaît, jamais affiché.
# $result est la ligne de résultat de l’import (« 12 éléments importés. »), $notice
# le titre de l’avis (« 2 choses à savoir sur cet import »).
# L’avis est lu par-dessus le résultat : il dit donc les deux.
import-warnings-announcement = { $result } { $notice }.
# Quitter pendant un import. $name est le fichier que l’import écrit.
quit-import-running-title = Un import est encore en cours
quit-import-running-text = Skribisto importe encore « { $name } ». Quitter maintenant arrête l’import : rien n’est écrit, et le projet que vous importez reste tel qu’il était.
quit-import-stop = Arrêter l’import et quitter

## Importer un projet Scrivener : l’assistant
import-scrivener-title = Importer un projet Scrivener
import-scrivener-step-project = Projet
import-scrivener-step-review = Vérifier
import-scrivener-step-save = Enregistrer
import-scrivener-back = Précédent
import-scrivener-next = Suivant
import-scrivener-import = Importer
import-scrivener-cancel = Annuler
import-scrivener-help = Aide

## Étape Projet : choisir le projet
import-scrivener-drop-title = Déposez un projet Scrivener ici
import-scrivener-drop-hint = Un projet .scriv, le fichier .scrivx qu’il contient, une copie de secours .zip ou un .scrivtemplate
import-scrivener-source = Projet Scrivener
import-scrivener-source-placeholder = Chemin du projet
import-scrivener-choose-file = Choisir un fichier…
import-scrivener-choose-folder = Choisir un dossier…
# Sous Windows et Linux, un projet .scriv est un dossier ordinaire.
import-scrivener-source-hint = Choisissez le dossier .scriv du projet, ou ouvrez-le et choisissez le fichier .scrivx qu’il contient. Une copie de secours .zip convient aussi.
# Sous macOS, un projet .scriv apparaît comme un seul fichier quand Scrivener est installé, comme un dossier sinon.
import-scrivener-source-hint-mac = Si le projet apparaît comme un seul fichier, utilisez « Choisir un fichier… ». S’il apparaît comme un dossier, utilisez « Choisir un dossier… ». Une copie de secours .zip convient aussi.
import-scrivener-dialog-file-title = Choisir un projet Scrivener
import-scrivener-dialog-folder-title = Choisir le dossier d’un projet Scrivener
import-scrivener-dialog-filter = Projet Scrivener
import-scrivener-source-missing = Il n’y a rien à cet emplacement
import-scrivener-source-unreadable-name = Ce nom est illisible. Renommez le projet dans votre gestionnaire de fichiers, puis choisissez-le de nouveau.
import-scrivener-empty-hint = Choisissez un projet pour voir ce que Skribisto y trouve.
import-scrivener-reading = Lecture du projet…
import-scrivener-stop-reading = Arrêter la lecture
import-scrivener-read-failed = Le projet n’a pas pu être lu
# Affiché au-dessus des deux boutons qu’il nomme : import-scrivener-check-again et
# import-scrivener-details.
import-scrivener-read-failed-body = Un problème est survenu pendant la lecture. Choisissez « Relire le projet » pour réessayer, ou « Détails » pour voir ce qui s’est passé.
import-scrivener-details = Détails
# Le bouton de l’assistant qui relit le projet Scrivener sur le disque, tel qu’il est
# maintenant. Pas l’action d’import-scrivener-review-again : les deux libellés doivent
# rester bien distincts.
import-scrivener-check-again = Relire le projet
import-scrivener-refusal-unknown-title = Ce projet ne peut pas être importé
import-scrivener-refusal-unknown-body = Skribisto ne sait pas le reprendre. Les détails donnent la raison.

## Étape Projet : la fiche du projet
import-scrivener-made-with-mac = Enregistré en dernier par Scrivener { $version } pour macOS
import-scrivener-made-with-windows = Enregistré en dernier par Scrivener { $version } pour Windows
import-scrivener-made-with-linux = Enregistré en dernier par Scrivener { $version } pour Linux
import-scrivener-made-with-ios = Enregistré en dernier par Scrivener { $version } pour iPhone et iPad
import-scrivener-made-with-version = Enregistré en dernier par Scrivener { $version }
import-scrivener-made-with-unknown = Projet Scrivener, format { $format }
import-scrivener-saved-on = Enregistré le { $date } sur « { $device } »
import-scrivener-saved-on-no-device = Enregistré le { $date }
# $documents est le nombre de documents (textes, pas dossiers) du projet ; $manuscript
# combien d’entre eux sont dans le manuscrit (le dossier Draft de Scrivener).
import-scrivener-card-counts = { $documents ->
    [one] { $documents } document
   *[other] { $documents } documents
}, dont { $manuscript } dans le manuscrit

## Étape Vérifier
import-scrivener-review-hint = Voici comment votre classeur va arriver. Rien n’est écrit avant l’import, et bien des éléments peuvent ensuite changer de type dans le classeur, avec Convertir en.
import-scrivener-choices-structure = Niveaux
import-scrivener-top-level = Dossiers de premier niveau
import-scrivener-choices-text = Texte
import-scrivener-choice-containers = Dossiers au niveau { $depth }
import-scrivener-choice-texts = Documents au niveau { $depth }
import-scrivener-choice-count = { $count ->
    [one] { $count } élément
   *[other] { $count } éléments
}
import-scrivener-choice-count-example = { $count ->
    [1] { $count } élément : « { $example } »
   *[other] { $count } éléments, comme « { $example } »
}
import-scrivener-rung-part = Parties
import-scrivener-rung-chapter = Chapitres
import-scrivener-rung-plain = Dossiers simples
import-scrivener-texts-scenes = Scènes
import-scrivener-texts-chapters = Chapitres
import-scrivener-choice-books = Livres du manuscrit
import-scrivener-books-one = Un seul livre
import-scrivener-books-several = Un livre par dossier
import-scrivener-choice-split = { $count ->
    [1] Découper « { $title } » à son titre
   *[other] Découper « { $title } » à ses { $count } titres
}
import-scrivener-root-items = { $count ->
    [one] { $count } élément
   *[other] { $count } éléments
}
# Sous le nom d’un dossier de premier niveau, page Vérifier : combien d’éléments il
# contient, et qu’il ressemble à un manuscrit.
import-scrivener-root-items-manuscript = { $count ->
    [one] { $count } élément. Ce dossier ressemble à un manuscrit.
   *[other] { $count } éléments. Ce dossier ressemble à un manuscrit.
}
import-scrivener-dest-notes = Notes
import-scrivener-dest-story-bible = Bible narrative
import-scrivener-dest-manuscript = Un autre livre
import-scrivener-dest-leave-out = Laisser de côté
import-scrivener-choice-trash = { $count ->
    [one] Reprendre la corbeille ({ $count } élément)
   *[other] Reprendre la corbeille ({ $count } éléments)
}
import-scrivener-trash-detail = { $count ->
    [0] Aucun ne contient de texte.
    [one] { $count } d’entre eux contient du texte.
   *[other] { $count } d’entre eux contiennent du texte.
}
import-scrivener-choice-blank-lines = Les lignes vides deviennent des séparateurs de scènes
import-scrivener-choice-ornaments = Les lignes d’ornements deviennent des séparateurs de scènes
import-scrivener-preview-label = Structure du nouveau projet
import-scrivener-col-title = Titre
import-scrivener-col-becomes = Devient
import-scrivener-col-exports = Exporté
import-scrivener-in-exports-yes = Oui
import-scrivener-in-exports-no = Non
import-scrivener-becomes-item = Élément
import-scrivener-becomes-trashed = Dans la corbeille
import-scrivener-before-title = Avant d’importer
import-scrivener-review-error-unreadable = La vérification n’a pas pu être préparée. Choisissez de nouveau le projet.
import-scrivener-review-error-choice = Ce choix ne correspond pas au projet. Choisissez de nouveau le projet.
import-scrivener-review-error-unmappable = Avec ce choix, le classeur ne peut pas être repris sans rien perdre. Faites un autre choix, ou signalez le problème.

## Étape Enregistrer
import-scrivener-save-hint = L’import crée un nouveau projet Skribisto. Votre projet Scrivener est seulement lu, jamais modifié.
import-scrivener-title-field = Titre
import-scrivener-author = Auteur
import-scrivener-language = Langue
import-scrivener-location-inside-project = Choisissez un dossier en dehors du projet Scrivener. Scrivener n’y range que ses propres fichiers.
import-scrivener-copy-research-label = Copier les fichiers de recherche à côté du nouveau projet
import-scrivener-copy-research = { $count ->
    [one] { $count } fichier de recherche que Skribisto ne sait pas ouvrir, comme un PDF ou un son, va dans un dossier à côté du projet. Si la case est décochée, sa note indique où il se trouve dans votre projet Scrivener.
   *[other] { $count } fichiers de recherche que Skribisto ne sait pas ouvrir, comme des PDF ou des sons, vont dans un dossier à côté du projet. Si la case est décochée, leurs notes indiquent où ils se trouvent dans votre projet Scrivener.
}
import-scrivener-default-name = Projet Scrivener

## Progression et résultat
import-scrivener-progress-title = Importation du projet Scrivener…
import-scrivener-done-title = Projet Scrivener importé
import-scrivener-done = { $items ->
    [one] Le nouveau projet contient { $items } élément.
   *[other] Le nouveau projet contient { $items } éléments.
}
import-scrivener-done-check = { $items ->
    [one] Le nouveau projet contient { $items } élément.
   *[other] Le nouveau projet contient { $items } éléments.
} { $findings ->
    [one] { $findings } point demande un coup d’œil.
   *[other] { $findings } points demandent un coup d’œil.
}
# Le bouton de l’avis d’un import qui a échoué : il rouvre l’assistant sur le même projet
# Scrivener, avec les choix faits pour cet import. Pas l’action
# d’import-scrivener-check-again. Court : il partage une ligne de l’avis avec « Détails ».
import-scrivener-review-again = Revoir l’import
## Le compte rendu de l’import (la fenêtre qu’ouvre Voir le compte rendu)
import-scrivener-see-report = Voir le compte rendu
import-scrivener-report-from = Lu dans { $path }
import-scrivener-report-to = Écrit dans { $path }
import-scrivener-report-came-across = Ce qui est arrivé
import-scrivener-report-nothing-counted = Rien n’a été compté.
import-scrivener-report-items = { $count ->
    [one] { $count } élément
   *[other] { $count } éléments
}
import-scrivener-report-words = { $count ->
    [one] { $count } mot
   *[other] { $count } mots
}
import-scrivener-report-synopses = { $count ->
    [one] { $count } synopsis
   *[other] { $count } synopsis
}
import-scrivener-report-document-notes = { $count ->
    [one] les notes de { $count } document
   *[other] les notes de { $count } documents
}
import-scrivener-report-comments = { $count ->
    [one] { $count } commentaire
   *[other] { $count } commentaires
}
import-scrivener-report-footnotes = { $count ->
    [one] { $count } note de bas de page
   *[other] { $count } notes de bas de page
}
import-scrivener-report-pictures = { $count ->
    [one] { $count } image
   *[other] { $count } images
}
import-scrivener-report-templates = { $count ->
    [one] { $count } modèle de note
   *[other] { $count } modèles de notes
}
import-scrivener-report-tags = { $count ->
    [one] { $count } étiquette
   *[other] { $count } étiquettes
}
import-scrivener-report-statuses = { $count ->
    [one] { $count } statut
   *[other] { $count } statuts
}
import-scrivener-report-trashed = { $count ->
    [one] { $count } élément dans la corbeille
   *[other] { $count } éléments dans la corbeille
}
import-scrivener-report-research-files = { $count ->
    [one] { $count } fichier de recherche
   *[other] { $count } fichiers de recherche
}
import-scrivener-report-choices = Ce que vous avez choisi
import-scrivener-report-accepted-mobile = Vous avez importé le projet sans les modifications faites dans Scrivener pour iPhone ou iPad qui n’y étaient pas encore.
import-scrivener-report-accepted-no-text = Vous avez importé cette copie avec ses documents vides, car aucun de leurs textes ne s’y trouvait.
import-scrivener-report-choice = { $subject } : { $answer }
import-scrivener-report-yes = oui
import-scrivener-report-no = non
import-scrivener-report-research-copied = Les fichiers de recherche que Skribisto ne sait pas ouvrir ont été copiés dans « { $folder } », à côté du nouveau projet.
import-scrivener-report-research-not-copied = Les fichiers de recherche que Skribisto ne sait pas ouvrir n’ont pas été copiés. Leurs notes indiquent où ils se trouvent dans votre projet Scrivener.
import-scrivener-report-research-copied-except = { $count ->
    [one] Les fichiers de recherche que Skribisto ne sait pas ouvrir ont été copiés dans « { $folder } », à côté du nouveau projet, sauf { $count } qui reste dans votre projet Scrivener. Sa note indique où il se trouve.
   *[other] Les fichiers de recherche que Skribisto ne sait pas ouvrir ont été copiés dans « { $folder } », à côté du nouveau projet, sauf { $count } qui restent dans votre projet Scrivener. Leurs notes indiquent où ils se trouvent.
}
# L’auteur a demandé la copie des fichiers de recherche et aucun n’a été copié : $count est
# le nombre de fichiers restés dans le projet Scrivener, c’est-à-dire tous.
import-scrivener-report-research-none-copied = { $count ->
    [1] Vous avez demandé que les fichiers de recherche soient copiés, mais le seul fichier concerné n’a pas pu l’être. Il reste dans votre projet Scrivener, et sa note indique où il se trouve.
   *[other] Vous avez demandé que les fichiers de recherche soient copiés, mais aucun des { $count } n’a pu l’être. Ils restent dans votre projet Scrivener, et leurs notes indiquent où ils se trouvent.
}
import-scrivener-report-chose-project = Vous avez choisi d’importer « { $project } », l’un des projets que contenait le dossier ou l’archive choisi.
import-scrivener-report-chose-manifest = Vous avez choisi le fichier de classeur « { $file } ».
import-scrivener-report-chose-enclosing = Vous aviez sélectionné un élément de « { $project } » : le projet entier a été importé.
import-scrivener-report-chose-upgraded = Vous avez importé « { $current } », le projet dans lequel vous avez continué d’écrire, plutôt que la copie que Scrivener a gardée avant une mise à jour.
import-scrivener-report-chose-backup = Vous avez importé « { $backup } », la copie plus ancienne que Scrivener a gardée avant une mise à jour.
import-scrivener-report-title-placeholder-rejected = Le titre indiqué dans les réglages du projet était une balise de modèle : Skribisto a proposé le nom du projet à la place.
import-scrivener-report-author-placeholder-rejected = Le nom d’auteur indiqué dans les réglages du projet était une balise de modèle : Skribisto n’en a proposé aucun.
import-scrivener-report-title-from-compile-settings = Skribisto a proposé le titre qu’indiquent les réglages de compilation du projet : le nom que le projet avait à sa création, sauf si un titre y a été indiqué dans Scrivener.
import-scrivener-report-title-from-template = Skribisto a proposé le titre du modèle.
import-scrivener-report-title-from-folder-name = Skribisto a proposé le nom du projet comme titre.
import-scrivener-report-title-from-project-properties = Skribisto a proposé le titre qu’indiquent les propriétés du projet.
import-scrivener-report-author-from-project = Skribisto a proposé le nom d’auteur qu’indiquent les réglages du projet. Le nom d’auteur d’un modèle peut y être resté, et il s’imprimerait sur une page de titre.
import-scrivener-report-language-not-stated = Le projet n’indiquait pas sa langue : Skribisto a proposé celle dans laquelle il s’affiche. Elle décide de la façon de compter les mots et d’écrire les guillemets.
import-scrivener-report-language-from-script = Skribisto a proposé la langue d’après l’écriture du manuscrit ({ $tag }).
# Une proposition de la page Enregistrer que la personne a remplacée par la sienne. $hint
# est la phrase qui dit ce que Skribisto proposait : un titre, un nom d’auteur ou une langue.
import-scrivener-report-you-changed-it = { $hint } Vous avez remplacé cette proposition.
import-scrivener-report-content-salvaged-text = Le texte de « { $item } » manquait dans le projet. Ses mots viennent de l’index de recherche de Scrivener, en texte brut et peut-être d’une version plus ancienne : vérifiez-les.
import-scrivener-report-content-salvaged-notes = Les notes de « { $item } » manquaient dans le projet. Leurs mots viennent de l’index de recherche de Scrivener, en texte brut et peut-être d’une version plus ancienne : vérifiez-les.
import-scrivener-report-content-salvaged-synopsis = Le synopsis de « { $item } » manquait dans le projet. Ses mots viennent de l’index de recherche de Scrivener, en texte brut et peut-être d’une version plus ancienne : vérifiez-les.
# Une remarque de l’import de documents trouvée à plusieurs endroits : sa phrase sur le
# premier, puis combien d’autres la liste en dessous en nomme.
import-scrivener-report-document-others = { $others ->
    [one] { $sentence } Il en va de même à { $others } autre endroit, cité ci-dessous.
   *[other] { $sentence } Il en va de même à { $others } autres endroits, cités ci-dessous.
}
# Un élément du compte rendu, par son chemin dans le classeur de Scrivener, avec celui de
# ses textes dont parle la ligne quand ce n’est pas le texte.
import-scrivener-report-item-notes = { $path } (notes du document)
import-scrivener-report-item-synopsis = { $path } (synopsis)
import-scrivener-report-item-project-note = { $path } (note du projet)
import-scrivener-report-item-recovered = { $path } (texte récupéré)
# Un élément sans nom dans Scrivener.
import-scrivener-report-untitled = Sans titre
import-scrivener-report-no-findings = L’import n’a rien d’autre à signaler.
import-scrivener-report-group-unread = Illisible
import-scrivener-report-group-check = À vérifier
import-scrivener-report-group-changed = Repris autrement
import-scrivener-report-group-left = Laissé de côté
import-scrivener-report-group-count = { $group } ({ $count })
import-scrivener-report-items-toggle = { $count ->
    [one] { $count } élément
   *[other] { $count } éléments
}
# La même chose, telle qu’un lecteur d’écran la nomme : avec la ligne dont elle liste les éléments.
import-scrivener-report-items-toggle-named = { $count ->
    [one] { $count } élément : { $line }
   *[other] { $count } éléments : { $line }
}
# Un bouton qui montre les noms dont parle une ligne du compte rendu, tels que le projet les
# écrit : libellés, mots-clés, collections, statuts, étiquettes ou champs.
import-scrivener-report-names-toggle = { $count ->
    [one] { $count } nom
   *[other] { $count } noms
}
import-scrivener-report-names-toggle-named = { $count ->
    [one] { $count } nom : { $line }
   *[other] { $count } noms : { $line }
}
# Sous une liste d’éléments ou de noms qui n’en montre que les premiers : combien d’autres
# elle contient, non affichés.
import-scrivener-report-more = { $count ->
    [one] Et { $count } autre.
   *[other] Et { $count } autres.
}
import-scrivener-report-open-item = Ouvrir dans l’éditeur
import-scrivener-report-links-hint = Ouvrez le nouveau projet, puis affichez de nouveau ce compte rendu depuis la palette de commandes : chaque élément cité ici s’ouvre alors d’un clic.
import-scrivener-copy-report = Copier le compte rendu
import-scrivener-report-copied = Compte rendu copié
import-scrivener-report-copy-failed = Le compte rendu n’a pas pu être copié.
import-scrivener-report-close = Fermer
import-scrivener-report-none-title = Aucun compte rendu d’import
import-scrivener-report-none = Aucun import Scrivener ne s’est terminé depuis le lancement de Skribisto. Skribisto garde les comptes rendus des derniers imports jusqu’à sa fermeture.
# $count est le nombre de comptes rendus d’import que Skribisto garde (16 actuellement).
import-scrivener-report-gone = { $count ->
    [1] Le compte rendu de cet import n’est plus gardé. Skribisto garde le compte rendu du dernier import jusqu’à sa fermeture.
   *[other] Le compte rendu de cet import n’est plus gardé. Skribisto garde les comptes rendus des { $count } derniers imports jusqu’à sa fermeture.
}

## Mots que l’import écrit dans le nouveau projet, dans la langue de l’interface
import-scrivener-manuscript-binder = Manuscrit
import-scrivener-notes-binder = Notes
import-scrivener-notes-of = { $title } (notes)
import-scrivener-project-notes = Notes du projet
import-scrivener-recovered = Textes récupérés
import-scrivener-untitled-template = Modèle sans titre
# Le dossier, à côté du nouveau projet, où ses fichiers de recherche sont copiés. Un nom de
# dossier : sans caractère interdit dans un nom de fichier.
import-scrivener-research-folder = { $name } (fichiers de recherche)
import-scrivener-list-separator = {", "}
# Ici et plus bas, le « libellé » est le Label de Scrivener (une étiquette de couleur),
# que l’import change en étiquette ; pas le Libellé de Skribisto.
import-scrivener-label-details = Du champ « { $field } » de Scrivener
import-scrivener-status-details = Du champ « { $field } » de Scrivener
import-scrivener-unnamed-label = Libellé { $number }
import-scrivener-unnamed-status = Statut { $number }
import-scrivener-label-tag = { $name } (libellé)
import-scrivener-untitled-tag = Sans titre
import-scrivener-keyword-details = Le mot-clé « { $path } » de Scrivener
import-scrivener-unnamed-keyword = Mot-clé { $number }
import-scrivener-keyword-tag = { $name } (mot-clé)
import-scrivener-collection-details = Une collection de Scrivener
import-scrivener-collection-tag = { $name } (collection)
import-scrivener-field-details = Du champ « { $field } » de Scrivener
import-scrivener-field-value = { $field } : { $option }
import-scrivener-field-tag = { $name } (champ)
import-scrivener-deadline-day = Échéance
import-scrivener-from-scrivener = Venu de Scrivener
import-scrivener-note-project = Projet
import-scrivener-note-project-bookmarks = Signets du projet
import-scrivener-note-replacements = Remplacements à la compilation
import-scrivener-note-auto-complete = Entrées d’autocomplétion
import-scrivener-note-saved-searches = Recherches enregistrées
import-scrivener-note-custom-metadata = Métadonnées personnalisées
import-scrivener-note-targets = Objectifs
import-scrivener-note-bookmarks = Signets
import-scrivener-abbreviated-title = Titre abrégé
import-scrivener-subjects = Sujets
import-scrivener-company = Société
import-scrivener-session-target = Objectif de session
import-scrivener-session-target-calculated = Objectif de session, calculé par Scrivener d’après l’échéance
import-scrivener-writing-days = Jours d’écriture
import-scrivener-weekday-monday = lundi
import-scrivener-weekday-tuesday = mardi
import-scrivener-weekday-wednesday = mercredi
import-scrivener-weekday-thursday = jeudi
import-scrivener-weekday-friday = vendredi
import-scrivener-weekday-saturday = samedi
import-scrivener-weekday-sunday = dimanche
import-scrivener-draft-target = Objectif du manuscrit
import-scrivener-deadline = Échéance
# L’unité d’un objectif, entre parenthèses après son nombre : « 80000 (mots) ». Une
# étiquette d’unité, sans accord avec le nombre.
import-scrivener-unit-words = mots
import-scrivener-unit-characters = caractères
import-scrivener-unit-pages = pages
# Les étiquettes d’une recherche enregistrée, dans la note qui les liste, suivies chacune
# de deux-points et de leur valeur.
import-scrivener-search-query = Recherche
import-scrivener-search-in = Dans
import-scrivener-search-operator = Correspondance
# Ce qu’une recherche enregistrée parcourt, selon le type que Scrivener nomme.
import-scrivener-search-in-all = tout
import-scrivener-search-in-title = titres
import-scrivener-search-in-text = texte
import-scrivener-search-in-notes = notes
import-scrivener-search-in-synopsis = synopsis
import-scrivener-search-in-keywords = mots-clés
import-scrivener-search-in-label = libellés
import-scrivener-search-in-status = statuts
import-scrivener-search-in-section-type = types de section
import-scrivener-search-in-custom-metadata = métadonnées personnalisées
# Comment les mots d’une recherche enregistrée se combinent.
import-scrivener-search-any = n’importe quel mot
import-scrivener-search-all = tous les mots
import-scrivener-search-exact = expression exacte
# Une ligne d’une liste : la place d’un signet et sa note, ou les mots d’un lien et son adresse.
import-scrivener-label-value = { $label } : { $value }
# Dit d’un remplacement à la compilation, entre parenthèses après lui : « `teh` → `the`
# (expression régulière, désactivé) ».
import-scrivener-replacement-regex = expression régulière
import-scrivener-replacement-off = désactivé
# Dans les notes qui listent les métadonnées personnalisées et les signets, dit après le
# chemin d’un document devenu une note : ses valeurs ou ses signets sont dans le texte de
# cette note.
import-scrivener-in-own-note = Dans la note issue de ce document.


## Importer un projet Scrivener : ce que l’import dit du projet, dans les mots de l’auteur.
## Chaque clé que le convertisseur peut donner a une ligne ici (voir `import_scrivener::messages`).

# Un refus en une ligne : son titre, puis quoi faire.
import-scrivener-paragraph = { $title }. { $body }

## Quand le projet ne peut pas être importé
## Un corps qui finit par « puis relisez le projet » s’affiche dans l’assistant à côté du
## bouton import-scrivener-check-again ; l’avis d’un import qui a échoué l’affiche à côté
## d’import-scrivener-review-again : il ne cite donc aucun bouton.
import-scrivener-refusal-missing-title = Il n’y a rien à cet emplacement
import-scrivener-refusal-missing-body = « { $file } » n’est plus là. Choisissez de nouveau le projet.
import-scrivener-refusal-unreadable-title = Ce projet ne peut pas être lu
import-scrivener-refusal-unreadable-body = Skribisto n’a pas pu lire « { $file } » ({ $reason }). Vérifiez que vous pouvez l’ouvrir, puis réessayez.
import-scrivener-refusal-not-a-scrivener-project-title = Aucun projet Scrivener ici
import-scrivener-refusal-not-a-scrivener-project-body = Choisissez le dossier .scriv du projet, le fichier .scrivx qu’il contient, une copie de secours .zip ou un .scrivtemplate.
import-scrivener-refusal-manifest-without-files-title = Les documents ne sont pas dans cette copie
import-scrivener-refusal-manifest-without-files-body = « { $manifest } » est là, mais pas le dossier Files qui contient les documents qu’il répertorie. Un service de synchronisation n’a peut-être pas fini de copier le projet, ou seul le fichier .scrivx a été copié. Si vous avez choisi le fichier .scrivx, choisissez plutôt le dossier du projet avec « Choisir un dossier… ».
import-scrivener-refusal-search-too-wide-title = Ce dossier contient trop de choses pour y chercher
import-scrivener-refusal-search-too-wide-body = Il contient plus de { $limit } dossiers à parcourir. Choisissez le projet Scrivener lui-même.
import-scrivener-refusal-scrivener-one-for-mac-title = Ce projet a été créé par Scrivener 1 pour Mac
import-scrivener-refusal-scrivener-one-for-mac-body = Skribisto lit les projets de Scrivener 2 et des versions suivantes. Ouvrez-le une fois dans Scrivener 2 ou 3 pour le mettre à jour, ou compilez-le en document Word et importez celui-ci.
# Un petit fichier que Scrivener pour iPhone ou iPad laisse à la place d’un projet qu’il
# garde : pas une balise de modèle.
import-scrivener-refusal-ios-stub-without-mobile-data-title = Ceci n’est qu’un fichier d’attente de Scrivener pour iPhone ou iPad
import-scrivener-refusal-ios-stub-without-mobile-data-body = Le projet lui-même n’est pas là. Synchronisez-le depuis votre iPhone ou votre iPad, ou partagez-le depuis l’appareil sous forme de .zip, et importez celui-ci.
import-scrivener-refusal-newer-title = Ce projet a été créé par un Scrivener plus récent
import-scrivener-refusal-unknown-format-version-body = Son format, { $version }, n’est pas un format que Skribisto connaît. Vérifiez s’il existe une mise à jour de Skribisto. En attendant, compilez-le en document Word dans Scrivener et importez celui-ci.
import-scrivener-refusal-newer-format-stamp-body = Ses fichiers sont marqués comme version { $stamp }, et Skribisto lit jusqu’à la version { $newest }. Vérifiez s’il existe une mise à jour de Skribisto. En attendant, compilez-le en document Word dans Scrivener et importez celui-ci.
import-scrivener-refusal-version-stamp-unreadable-title = La version du projet ne peut pas être lue
import-scrivener-refusal-version-stamp-unreadable-body = Son fichier Files/version.txt ne contient aucun numéro de version, si bien que Skribisto ne peut pas savoir s’il sait lire ce projet. Ouvrez une fois le projet dans Scrivener, puis importez-le de nouveau.
import-scrivener-refusal-damaged-manifest-title = Le fichier du classeur est illisible
import-scrivener-refusal-damaged-manifest-body = « { $manifest } » est endommagé. { $spare_copies ->
    [0] Ouvrez le projet dans Scrivener, qui sait le réparer, puis importez-le de nouveau.
    [1] Scrivener en garde une copie de réserve dans le projet, que Skribisto ne lit pas. Ouvrez le projet dans Scrivener, qui sait le réparer, puis importez-le de nouveau.
   *[other] Scrivener en garde { $spare_copies } copies de réserve dans le projet, que Skribisto ne lit pas. Ouvrez le projet dans Scrivener, qui sait le réparer, puis importez-le de nouveau.
}
import-scrivener-refusal-damaged-manifest-new-project-placeholder-body = « { $manifest } » est le fichier vide que Scrivener écrit pour un projet qu’il n’a pas encore enregistré. Ouvrez le projet dans Scrivener et laissez-le l’enregistrer, puis importez-le de nouveau.
import-scrivener-refusal-manifest-too-large-title = Ce fichier de classeur est trop volumineux
import-scrivener-refusal-manifest-too-large-body = « { $manifest } » dépasse les { $limit } Mo que lit Skribisto. Aucun projet réel n’en approche : le fichier est peut-être endommagé.
import-scrivener-refusal-no-item-identifiers-title = Le classeur ne répertorie aucun document que Skribisto puisse trouver
import-scrivener-refusal-no-item-identifiers-body = Aucun de ses éléments ne porte d’identifiant, si bien que leurs fichiers sont introuvables. Ouvrez une fois le projet dans Scrivener, puis importez-le de nouveau.
import-scrivener-refusal-incomplete-copy-title = Les documents manquent dans cette copie
import-scrivener-refusal-incomplete-copy-body = Le dossier Files est là, mais pas les documents qu’il devrait contenir. Un service de synchronisation n’a peut-être pas fini de copier le projet. Attendez qu’il ait fini, ou copiez de nouveau tout le dossier .scriv, puis relisez le projet.
import-scrivener-refusal-archive-title = Cette archive ne peut pas être ouverte
import-scrivener-refusal-archive-unreadable-body = « { $file } » n’a pas pu être lu ({ $reason }). Il est peut-être endommagé. Décompressez-le vous-même, puis choisissez le dossier .scriv qu’il contient.
import-scrivener-refusal-archive-unsafe-name-title = Cette archive ne peut pas être ouverte sans risque
import-scrivener-refusal-archive-unsafe-name-body = « { $file } » contient un fichier nommé « { $member } », qui serait écrit en dehors de l’archive : Skribisto ne l’ouvre donc pas.
import-scrivener-refusal-archive-unsafe-name-empty-body = « { $file } » contient un fichier sans nom : Skribisto ne l’ouvre donc pas.
import-scrivener-refusal-archive-unsafe-name-control-character-body = « { $file } » contient un fichier dont le nom comporte un caractère de contrôle : Skribisto ne l’ouvre donc pas.
import-scrivener-refusal-archive-too-large-title = Cette archive est trop volumineuse pour être ouverte sans risque
import-scrivener-refusal-archive-too-many-members-body = Elle contient plus de { $limit } fichiers. Décompressez-la vous-même, puis choisissez le dossier .scriv qu’elle contient.
import-scrivener-refusal-archive-member-too-large-body = « { $member } » dépasse { $limit } Mo une fois décompressé. Décompressez l’archive vous-même, puis choisissez le dossier .scriv qu’elle contient.
import-scrivener-refusal-archive-too-large-body = Elle dépasse { $limit } Mo une fois décompressée. Décompressez-la vous-même, puis choisissez le dossier .scriv qu’elle contient.
import-scrivener-refusal-archive-encrypted-title = Cette archive est protégée par un mot de passe
import-scrivener-refusal-archive-encrypted-body = Décompressez « { $file } » vous-même, puis choisissez le dossier .scriv qu’il contient.
import-scrivener-refusal-archive-unsupported-compression-title = Cette archive est compressée d’une façon que Skribisto ne sait pas lire
import-scrivener-refusal-archive-unsupported-compression-body = « { $member } » est compressé avec { $method }. Décompressez l’archive vous-même, puis choisissez le dossier .scriv qu’elle contient.
import-scrivener-refusal-duplicate-archive-members-title = Cette archive répertorie certains fichiers deux fois
import-scrivener-refusal-duplicate-archive-members-body = « { $file } » contient plusieurs fichiers sous le même nom, si bien que Skribisto ne peut pas savoir lequel appartient au projet. Décompressez-le vous-même, puis choisissez le dossier .scriv qu’il contient.
import-scrivener-refusal-template-undecodable-title = Ce fichier de modèle est incomplet
import-scrivener-refusal-template-undecodable-too-large-title = Ce fichier de modèle est trop volumineux pour être ouvert
import-scrivener-refusal-template-undecodable-not-a-template-title = Ce fichier n’est pas un modèle Scrivener
import-scrivener-refusal-template-undecodable-unknown-encoding-title = Ce fichier de modèle est enregistré sous une forme que Skribisto ne connaît pas
import-scrivener-refusal-template-undecodable-body = « { $file } » ne peut pas être décodé : il s’arrête peut-être avant la fin du projet qu’il contient. Téléchargez-le de nouveau, ou ouvrez-le dans Scrivener et créez un projet à partir de lui.
import-scrivener-refusal-template-undecodable-too-large-body = « { $file } » dépasse { $limit } Mo. Ouvrez-le dans Scrivener, créez un projet à partir de lui, puis importez ce projet.
import-scrivener-refusal-template-undecodable-not-a-template-body = « { $file } » ne contient pas de projet Scrivener. Choisissez le fichier de modèle enregistré par Scrivener, ou un projet créé à partir de lui.
import-scrivener-refusal-template-undecodable-unknown-encoding-body = « { $file } » enregistre son projet sous une forme que Skribisto ne sait pas décoder. Ouvrez-le dans Scrivener, créez un projet à partir de lui, puis importez ce projet.
import-scrivener-refusal-cloud-placeholders-title = { $count ->
    [one] { $count } fichier de ce projet est encore dans le nuage
   *[other] { $count } fichiers de ce projet sont encore dans le nuage
}
# Dit sous import-scrivener-refusal-cloud-placeholders-title. $count est le nombre de
# fichiers de ce titre ; $examples en nomme quelques-uns, ou le seul.
import-scrivener-refusal-cloud-placeholders-body = { $count ->
    [1] Il n’a pas été téléchargé sur cet ordinateur : { $examples }. Rendez le projet disponible hors ligne dans votre application de synchronisation, puis relisez le projet.
   *[other] Ils n’ont pas été téléchargés sur cet ordinateur, par exemple { $examples }. Rendez le projet disponible hors ligne dans votre application de synchronisation, puis relisez le projet.
}
import-scrivener-refusal-unreadable-members-title = { $count ->
    [one] { $count } fichier de ce projet ne peut pas être lu
   *[other] { $count } fichiers de ce projet ne peuvent pas être lus
}
import-scrivener-refusal-unreadable-members-body = Par exemple { $examples }. Vérifiez que vous avez le droit de lire tout le projet et que le disque est en bon état, puis relisez le projet.
import-scrivener-refusal-needed-member-is-link-title = Ce projet renvoie hors de son propre dossier
import-scrivener-refusal-needed-member-is-link-body = « { $member } » est un lien vers un autre endroit de cet ordinateur, et Skribisto ne suit pas les liens. Copiez le projet de sorte que tous ses fichiers soient à l’intérieur, puis réessayez.
import-scrivener-refusal-project-changed-during-read-title = Le projet a changé pendant sa lecture
import-scrivener-refusal-project-changed-during-read-body = Scrivener ou un service de synchronisation y écrit encore. Fermez-le dans Scrivener, attendez la fin de la synchronisation, puis relisez le projet.
import-scrivener-refusal-too-many-members-title = Ce projet contient trop de fichiers
import-scrivener-refusal-too-many-members-body = Il en contient plus de { $limit }, plus qu’aucun projet réel. Ce n’est peut-être pas un projet Scrivener, ou il est peut-être endommagé.
import-scrivener-refusal-too-large-title = Ce projet est trop volumineux
import-scrivener-refusal-too-large-body = Son texte dépasse { $limit } Mo, plus que ce que Skribisto peut importer sans risque.
import-scrivener-refusal-structure-unmappable-title = Skribisto ne sait pas reprendre ce classeur
import-scrivener-refusal-structure-unmappable-body = Le classeur ne peut pas être repris sans perdre ou mal placer quelque chose. C’est un défaut de Skribisto, pas de votre projet. Merci de le signaler avec ce qu’affiche « Détails ».

## Quand il faut choisir
# $count vaut 2 ou plus : le nombre de projets Scrivener du dossier ou de l’archive choisi.
import-scrivener-roots-title = { $count ->
    [one] Il y a ici { $count } projet. Lequel importer ?
   *[other] Il y a ici { $count } projets. Lequel importer ?
}
import-scrivener-roots-hint = Ce dossier ou cette archive contient plusieurs projets Scrivener. Choisissez celui à importer.
# $count vaut 2 ou plus : le nombre de fichiers de classeur (.scrivx) du projet.
import-scrivener-manifests-title = { $count ->
    [one] Ce projet a { $count } fichier de classeur
   *[other] Ce projet a { $count } fichiers de classeur
}
import-scrivener-manifests-hint = Scrivener garde le classeur, la liste de tous les documents et dossiers, dans un fichier .scrivx. Les fichiers en trop ont sans doute été laissés par un service de synchronisation. Choisissez celui dans lequel vous avez travaillé en dernier.
import-scrivener-manifests-hint-preselected = Scrivener garde le classeur, la liste de tous les documents et dossiers, dans un fichier .scrivx. Les fichiers en trop ont sans doute été laissés par un service de synchronisation. Celui qui est sélectionné est très probablement celui dans lequel vous avez travaillé en dernier : vérifiez-le avant de continuer.
# Un fichier de classeur à choisir, sur une ligne : son nom, sa date d’enregistrement, et
# le nombre d’éléments de son classeur. $documents compte tous les éléments, dossiers compris.
import-scrivener-manifest-row = { $file } · enregistré le { $date } · { $documents ->
    [one] { $documents } élément
   *[other] { $documents } éléments
}
# La même chose, quand le fichier ne dit pas quand il a été enregistré.
import-scrivener-manifest-row-undated = { $file } · { $documents ->
    [one] { $documents } élément
   *[other] { $documents } éléments
}
import-scrivener-manifest-row-unread = { $file } · illisible
import-scrivener-enclosing-title = Ceci fait partie du projet Scrivener « { $project } »
import-scrivener-enclosing-body = Vous avez choisi un fichier ou un dossier à l’intérieur d’un projet. Skribisto importe le projet entier.
import-scrivener-enclosing-use = Importer « { $project } »
import-scrivener-upgrade-title = C’est la copie que Scrivener a gardée avant une mise à jour
import-scrivener-upgrade-body = Quand Scrivener met un projet à jour vers une version plus récente, il garde l’ancien sous le nom « { $backup } ». Le projet dans lequel vous avez continué d’écrire est « { $current } », juste à côté.
import-scrivener-upgrade-body-unclear = Quand Scrivener met un projet à jour vers une version plus récente, il garde l’ancien sous le nom « { $backup } ». Le projet d’à côté, « { $current } », n’a pas pu être lu proprement : choisissez lequel importer.
import-scrivener-upgrade-use-current = Importer « { $current } », le projet dans lequel vous avez continué d’écrire
import-scrivener-upgrade-keep-backup = Importer cette copie plus ancienne, « { $backup } »

## Ce qu’il faut accepter pour continuer
import-scrivener-ios-pending-title = Des modifications faites dans Scrivener pour iPhone ou iPad ne sont pas encore dans cette copie
import-scrivener-ios-pending-body = Scrivener sur ordinateur les intègre la prochaine fois qu’il ouvre le projet. Ouvrez une fois le projet dans Scrivener sur votre ordinateur, puis importez-le de nouveau.
import-scrivener-ios-pending-accept = L’importer sans ces modifications
import-scrivener-no-text-title = { $count ->
    [one] { $count } document, et pas un mot de texte dans cette copie
   *[other] { $count } documents, et pas un mot de texte dans cette copie
}
import-scrivener-no-text-body = Un service de synchronisation est peut-être encore en train de copier le projet, ou la copie a perdu ses documents. Importés maintenant, tous les documents arrivent vides.
import-scrivener-no-text-accept = Importer les documents vides

## À savoir, rien à décider
import-scrivener-notice-project-found-below-pick-title = Le projet a été trouvé dans le dossier choisi
import-scrivener-notice-project-found-below-pick-body = Il se trouve à « { $path } ».
import-scrivener-notice-source-is-template-title = Ceci est un modèle Scrivener
import-scrivener-notice-source-is-template-named = Ceci est le modèle Scrivener « { $title } »
import-scrivener-notice-source-is-template-body = L’importer crée un nouveau projet à partir de ce que contient le modèle, instructions comprises.
import-scrivener-notice-source-is-backup-title = Ceci ressemble à une copie de secours faite par Scrivener
import-scrivener-notice-source-is-backup-body = Elle contient le projet tel qu’il était au moment de la copie. Si vous avez continué d’écrire depuis, choisissez le projet lui-même.
import-scrivener-notice-importing-pre-upgrade-copy-title = Vous importez la copie que Scrivener a gardée avant une mise à jour
import-scrivener-notice-importing-pre-upgrade-copy-body = Le projet dans lequel vous avez continué d’écrire est juste à côté, avec tout ce qui a été écrit depuis.
import-scrivener-notice-imported-from-ios-stub-title = Ce projet a été créé dans Scrivener pour iPhone ou iPad
import-scrivener-notice-imported-from-ios-stub-body = Il n’a jamais été ouvert sur un ordinateur : il est donc lu depuis la copie que garde l’iPhone ou l’iPad.
import-scrivener-notice-stale-mobile-binder-ignored-title = Un ancien classeur venu d’un iPhone ou d’un iPad est mis de côté
import-scrivener-notice-stale-mobile-binder-ignored-body = Scrivener sur ordinateur en a déjà intégré les modifications.
# $count vaut 2 ou plus : tous les fichiers de classeur (.scrivx) du projet sont identiques.
import-scrivener-notice-identical-manifest-copies-title = { $count ->
    [one] Ce projet a { $count } fichier de classeur identique
   *[other] Ce projet a { $count } fichiers de classeur identiques
}
import-scrivener-notice-identical-manifest-copies-body = Elles contiennent la même chose : chacune donne le même projet.
import-scrivener-notice-manifest-copies-ignored-title = Des copies en trop du fichier du classeur sont mises de côté
import-scrivener-notice-manifest-copies-ignored-body = « { $chosen } » est celui dans lequel Scrivener a travaillé en dernier. { $count ->
    [one] { $count } copie laissée par un service de synchronisation est mise de côté.
   *[other] { $count } copies laissées par un service de synchronisation sont mises de côté.
}
import-scrivener-notice-pinned-manifest-looks-like-a-copy-title = Le fichier de classeur choisi ressemble à une copie de synchronisation
import-scrivener-notice-pinned-manifest-looks-like-a-copy-body = « { $name } » porte le nom d’une copie de « { $original } », qui se trouve à côté. Si l’import semble ancien, choisissez plutôt « { $original } ».
import-scrivener-notice-pinned-manifest-has-siblings-title = { $count ->
    [one] { $count } autre fichier de classeur se trouve à côté de celui choisi
   *[other] { $count } autres fichiers de classeur se trouvent à côté de celui choisi
}
import-scrivener-notice-pinned-manifest-has-siblings-body = Seul celui que vous avez choisi est lu.
import-scrivener-notice-legacy-v1-files-beside-title = Des fichiers d’une ancienne version de Scrivener sont mis de côté
import-scrivener-notice-legacy-v1-files-beside-body = Ils sont restés lors de la mise à jour du projet. Seuls les fichiers de la version actuelle sont lus.
import-scrivener-notice-mixed-payload-trees-title = Des fichiers de deux versions de Scrivener sont dans ce projet
import-scrivener-notice-mixed-payload-trees-body = Les documents sont lus là où le fichier du classeur les situe.
import-scrivener-notice-version-disagreement-title = Les fichiers de ce projet ne s’accordent pas tous sur sa version
import-scrivener-notice-id-scheme-mismatch-body = Le fichier du classeur indique la version { $version }, mais nomme ses documents à la manière d’une autre version. Ils sont lus là où il l’indique. Si le résultat semble faux, ouvrez une fois le projet dans Scrivener, puis importez-le de nouveau.
import-scrivener-notice-version-stamp-mismatch-body = Il est lu comme le décrit son fichier de classeur. Si le résultat semble faux, ouvrez une fois le projet dans Scrivener, puis importez-le de nouveau.
import-scrivener-notice-unrecognised-writer-title = Ce projet a été enregistré par un programme que Skribisto ne reconnaît pas
import-scrivener-notice-unrecognised-writer-named = Il se nomme « { $creator } ». Il est lu comme un projet Scrivener.
import-scrivener-notice-unrecognised-writer-body = Il ne se nomme pas. Il est lu comme un projet Scrivener.
import-scrivener-notice-invalid-item-identifier-title = { $count ->
    [one] { $count } élément du classeur porte un identifiant qui n’en est pas un
   *[other] { $count } éléments du classeur portent un identifiant qui n’en est pas un
}
import-scrivener-notice-invalid-item-identifier-body = Un tel identifiant est mis de côté, et l’élément qui le porte est gardé.
import-scrivener-notice-items-without-identifier-title = { $count ->
    [one] { $count } élément du classeur n’a pas d’identifiant
   *[other] { $count } éléments du classeur n’ont pas d’identifiant
}
import-scrivener-notice-items-without-identifier-body = Sans identifiant, les fichiers d’un élément sont introuvables : il arrive donc vide.
import-scrivener-notice-duplicate-item-key-title = Deux éléments du classeur partagent un identifiant : « { $item } »
import-scrivener-notice-duplicate-item-key-body = Le premier garde les documents. Le second garde sa place et arrive vide.
import-scrivener-notice-lock-here-title = Scrivener a ce projet ouvert sur cet ordinateur
import-scrivener-notice-lock-here-body = Fermez-le d’abord dans Scrivener, pour que toutes les modifications soient enregistrées.
import-scrivener-notice-lock-title = Scrivener avait ce projet ouvert sur « { $host } »
import-scrivener-notice-lock-elsewhere-title = Scrivener avait ce projet ouvert sur un autre ordinateur
import-scrivener-notice-lock-body = S’il y est encore ouvert, fermez-le d’abord : les fichiers ne contiennent peut-être pas encore les dernières modifications. Si Scrivener a été fermé il y a longtemps, vous pouvez l’importer tel quel.
import-scrivener-notice-archived-while-open-title = Cette archive a été faite pendant que Scrivener avait le projet ouvert
import-scrivener-notice-archived-while-open-body = Des modifications faites après ce moment y manquent peut-être.
import-scrivener-notice-content-missing-notes = Les notes de « { $item } » manquent
import-scrivener-notice-content-missing-synopsis = Le synopsis de « { $item } » manque
import-scrivener-notice-content-missing-text = Le texte de « { $item } » manque
# Dit sous l’un des trois titres ci-dessus : $place est ce qui manque, « notes » (les notes
# du document), « synopsis » ou « text » (le texte).
import-scrivener-notice-content-missing-body = { $place ->
    [notes] Leur fichier n’est pas dans cette copie : elles arrivent donc vides. Un service de synchronisation est peut-être encore en train de copier le projet.
    [synopsis] Son fichier n’est pas dans cette copie : il arrive donc vide. Un service de synchronisation est peut-être encore en train de copier le projet.
   *[text] Son fichier n’est pas dans cette copie : il arrive donc vide. Un service de synchronisation est peut-être encore en train de copier le projet.
}
# La même chose, quand l’index de recherche de Scrivener en avait encore les mots.
import-scrivener-notice-content-missing-salvaged = { $place ->
    [notes] Leur fichier n’est pas dans cette copie, mais l’index de recherche de Scrivener en contenait encore les mots. Ils sont gardés en texte brut, peut-être d’une version plus ancienne.
    [synopsis] Son fichier n’est pas dans cette copie, mais l’index de recherche de Scrivener en contenait encore les mots. Ils sont gardés en texte brut, peut-être d’une version plus ancienne.
   *[text] Son fichier n’est pas dans cette copie, mais l’index de recherche de Scrivener en contenait encore les mots. Ils sont gardés en texte brut, peut-être d’une version plus ancienne.
}
import-scrivener-notice-content-truncated-notes = Les notes de « { $item } » sont vides sur le disque
import-scrivener-notice-content-truncated-synopsis = Le synopsis de « { $item } » est vide sur le disque
import-scrivener-notice-content-truncated-text = Le texte de « { $item } » est vide sur le disque
# Dit sous l’un des trois titres ci-dessus : $place est ce qui est vide, « notes »,
# « synopsis » ou « text ».
import-scrivener-notice-content-truncated-body = { $place ->
    [notes] Scrivener les a enregistrées avec du texte, mais leur fichier n’en contient aucun : une synchronisation s’est peut-être arrêtée en cours de route. Elles arrivent vides.
    [synopsis] Scrivener l’a enregistré avec du texte, mais son fichier n’en contient aucun : une synchronisation s’est peut-être arrêtée en cours de route. Il arrive vide.
   *[text] Scrivener l’a enregistré avec du texte, mais son fichier n’en contient aucun : une synchronisation s’est peut-être arrêtée en cours de route. Il arrive vide.
}
import-scrivener-notice-missing-media-title = Le fichier de « { $item } » manque
import-scrivener-notice-missing-media-body = L’image ou le fichier de recherche qu’il contient n’est pas dans cette copie.
import-scrivener-notice-sync-conflict-copies-title = { $count ->
    [one] { $count } copie de synchronisation d’un document a été trouvée
   *[other] { $count } copies de synchronisation de documents ont été trouvées
}
# $missing est le nombre de copies dont le document n’est plus dans le projet.
import-scrivener-notice-sync-conflict-copies-body = { $missing ->
    [0] Les copies de synchronisation arrivent comme textes récupérés, dans un dossier à part, pour que vous puissiez les comparer aux documents.
    [one] Les copies de synchronisation arrivent comme textes récupérés, dans un dossier à part. { $missing } d’entre elles est la copie d’un document qui n’est plus dans le projet.
   *[other] Les copies de synchronisation arrivent comme textes récupérés, dans un dossier à part. { $missing } d’entre elles sont des copies de documents qui ne sont plus dans le projet.
}
import-scrivener-notice-recovered-unreferenced-text-title = { $count ->
    [one] { $count } document qu’aucun élément du classeur ne répertorie a été trouvé
   *[other] { $count } documents qu’aucun élément du classeur ne répertorie ont été trouvés
}
import-scrivener-notice-recovered-unreferenced-text-body = Les documents de ce genre arrivent comme textes récupérés, dans un dossier à part.
import-scrivener-notice-sync-debris-ignored-title = { $count ->
    [one] { $count } copie en trop laissée par un service de synchronisation est mise de côté
   *[other] { $count } copies en trop laissées par un service de synchronisation sont mises de côté
}
import-scrivener-notice-sync-debris-ignored-body = Scrivener ne lit pas non plus ce genre de copie. Seuls les fichiers propres au projet sont lus.
import-scrivener-notice-unrecognised-files-title = { $count ->
    [one] { $count } fichier qui ne fait pas partie d’un projet Scrivener est mis de côté
   *[other] { $count } fichiers qui ne font pas partie d’un projet Scrivener sont mis de côté
}
import-scrivener-notice-examples = Par exemple { $examples }.
import-scrivener-notice-symlink-not-followed-title = { $count ->
    [one] { $count } lien à l’intérieur du projet n’est pas suivi
   *[other] { $count } liens à l’intérieur du projet ne sont pas suivis
}
import-scrivener-notice-unreadable-files-skipped-title = { $count ->
    [one] { $count } fichier dont l’import n’a pas besoin n’a pas pu être lu
   *[other] { $count } fichiers dont l’import n’a pas besoin n’ont pas pu être lus
}
import-scrivener-notice-cloud-placeholders-skipped-title = { $count ->
    [one] { $count } fichier encore dans le nuage est ignoré
   *[other] { $count } fichiers encore dans le nuage sont ignorés
}
import-scrivener-notice-cloud-placeholders-skipped-body = Rien de ce dont l’import a besoin n’est encore dans le nuage.
import-scrivener-notice-no-text-found-title = { $count ->
    [1] { $count } document, qui ne contient aucun texte
   *[other] { $count } documents, et aucun ne contient de texte
}
import-scrivener-notice-no-text-found-body = Ce projet semble vide. Il est importé tel quel.
import-scrivener-notice-metadata-unreadable-title = « { $member } » n’a pas pu être lu
import-scrivener-notice-metadata-unreadable-body = Ce que Scrivener y gardait n’est pas repris.
import-scrivener-notice-dangling-keyword-title = { $count ->
    [one] { $count } référence renvoie à un mot-clé que le projet ne définit pas
   *[other] { $count } références renvoient à des mots-clés que le projet ne définit pas
}
import-scrivener-notice-dangling-label-title = { $count ->
    [one] { $count } référence renvoie à un libellé que le projet ne définit pas
   *[other] { $count } références renvoient à des libellés que le projet ne définit pas
}
import-scrivener-notice-dangling-status-title = { $count ->
    [one] { $count } référence renvoie à un état que le projet ne définit pas
   *[other] { $count } références renvoient à des états que le projet ne définit pas
}
import-scrivener-notice-dangling-section-type-title = { $count ->
    [one] { $count } référence renvoie à un type de section que le projet ne définit pas
   *[other] { $count } références renvoient à des types de section que le projet ne définit pas
}
import-scrivener-notice-dangling-custom-field-title = { $count ->
    [one] { $count } référence renvoie à un champ de métadonnées personnalisé que le projet ne définit pas
   *[other] { $count } références renvoient à des champs de métadonnées personnalisés que le projet ne définit pas
}
import-scrivener-notice-dangling-item-title = { $count ->
    [one] { $count } référence renvoie à un document que le projet ne définit pas
   *[other] { $count } références renvoient à des documents que le projet ne définit pas
}
import-scrivener-notice-dangling-references-body = Une référence à quelque chose que le projet ne définit pas est laissée de côté.
import-scrivener-notice-snapshots-not-imported-line = { $count ->
    [one] { $count } instantané reste dans votre projet Scrivener : Skribisto ne reprend pas les instantanés.
   *[other] { $count } instantanés restent dans votre projet Scrivener : Skribisto ne reprend pas les instantanés.
}
import-scrivener-notice-writing-history-not-imported-line = { $days ->
    [one] Le relevé des mots écrits chaque jour dans Scrivener, sur { $days } jour, n’est pas repris. Skribisto commence son propre décompte au premier enregistrement.
   *[other] Le relevé des mots écrits chaque jour dans Scrivener, sur { $days } jours, n’est pas repris. Skribisto commence son propre décompte au premier enregistrement.
}
# Une balise de modèle est l’un des codes de Scrivener, comme <$projecttitle>.
import-scrivener-notice-title-placeholder-rejected-hint = Le titre indiqué dans les réglages du projet est une balise de modèle : le nom du projet est proposé à la place.
import-scrivener-notice-author-placeholder-rejected-hint = Le nom d’auteur indiqué dans les réglages du projet est une balise de modèle : aucun n’est proposé.
import-scrivener-notice-title-from-project-hint-compile-settings = Ce titre est celui qu’indiquent les réglages de compilation du projet. Scrivener y met le nom que le projet avait à sa création, tant qu’aucun titre n’y est indiqué.
import-scrivener-notice-title-from-project-hint-template = Ce titre est celui du modèle.
import-scrivener-notice-title-from-project-hint-folder-name = Ce titre est le nom du projet.
import-scrivener-notice-title-from-project-hint-project-properties = Ce titre est celui qu’indiquent les propriétés du projet.
import-scrivener-notice-author-from-project-hint = Ce nom est celui qu’indiquent les réglages du projet. Le nom d’auteur d’un modèle peut y être resté, et il s’imprimerait sur une page de titre.
import-scrivener-notice-language-not-stated-hint = Le projet n’indique pas sa langue : la langue dans laquelle Skribisto s’affiche est proposée. Elle décide de la façon de compter les mots et d’écrire les guillemets.
import-scrivener-notice-language-from-script-hint = Proposée d’après l’écriture du manuscrit ({ $tag }).

## Avant d’importer : ce que l’import fait du classeur, ou laisse de côté
import-scrivener-structure-script-mode-draft-line = { $script ->
    [1] { $documents ->
        [1] Le seul document du manuscrit est écrit en mode script. Skribisto garde ses mots, pas sa mise en page de scénario.
       *[other] { $script } document du manuscrit sur { $documents } est écrit en mode script. Skribisto garde ses mots, pas sa mise en page de scénario.
    }
   *[other] { $script } documents du manuscrit sur { $documents } sont écrits en mode script. Skribisto garde leurs mots, pas leur mise en page de scénario.
}
import-scrivener-structure-one-document-manuscript-line = { $count ->
    [1] Un document du manuscrit contient ses chapitres sous forme de titres.
   *[other] { $count } documents du manuscrit contiennent leurs chapitres sous forme de titres.
}
# Le numéro en chiffres, en chiffres romains ou en lettres.
import-scrivener-structure-title-repeats-number-line = { $count ->
    [one] { $count } titre de chapitre ou de partie contient aussi son numéro, comme « Chapitre II » ou « Première partie » : un export risque d’imprimer le numéro deux fois.
   *[other] { $count } titres de chapitre ou de partie contiennent aussi leur numéro, comme « Chapitre II » ou « Première partie » : un export risque d’imprimer le numéro deux fois.
}
# Un élément dont le titre ou le type de section dit qu’il ouvre un livre, une partie ou un
# chapitre, à l’intérieur d’un élément qui en ouvre déjà un de même niveau.
import-scrivener-structure-claim-overruled-line = { $count ->
    [one] { $count } élément ressemble à un livre, une partie ou un chapitre, mais il se trouve dans un élément déjà de ce niveau : il arrive donc à un niveau inférieur.
   *[other] { $count } éléments ressemblent à des livres, des parties ou des chapitres, mais chacun se trouve dans un élément déjà de ce niveau : ils arrivent donc à un niveau inférieur.
}
import-scrivener-structure-adopted-by-chapter-line = { $count ->
    [one] { $count } document qui suit un chapitre en devient la fin.
   *[other] { $count } documents qui suivent un chapitre en deviennent la fin.
}
import-scrivener-structure-looks-like-manuscript-line = { $count ->
    [one] { $count } dossier de premier niveau gardé dans les notes ressemble à un manuscrit. Vous pouvez l’envoyer dans le manuscrit comme un autre livre.
   *[other] { $count } dossiers de premier niveau gardés dans les notes ressemblent à un manuscrit. Vous pouvez les envoyer dans le manuscrit comme d’autres livres.
}
# « Dans le texte » est la page d’une fiche qui liste où le texte la nomme
# (segment-note-in-prose).
import-scrivener-structure-generic-sheet-title-line = { $count ->
    [one] { $count } fiche de la bible narrative porte encore le titre d’un modèle, comme « Héros ». Skribisto cherche ce titre dans votre texte et liste chaque endroit où il apparaît, sous Dans le texte : renommez la fiche si ce mot est courant dans votre livre.
   *[other] { $count } fiches de la bible narrative portent encore le titre d’un modèle, comme « Héros ». Skribisto cherche ces titres dans votre texte et liste chaque endroit où ils apparaissent, sous Dans le texte : renommez ces fiches si ces mots sont courants dans votre livre.
}
import-scrivener-structure-placeholder-title-line = { $count ->
    [one] { $count } titre ne faisait que nommer son niveau et est laissé vide : le classeur et les exports le numérotent.
   *[other] { $count } titres ne faisaient que nommer leur niveau et sont laissés vides : le classeur et les exports les numérotent.
}
import-scrivener-structure-untitled-item-line = { $count ->
    [one] { $count } élément n’avait pas de titre dans Scrivener et reste sans titre.
   *[other] { $count } éléments n’avaient pas de titre dans Scrivener et restent sans titre.
}
import-scrivener-structure-title-cleaned-line = { $count ->
    [one] { $count } titre contenait des retours à la ligne ou des tabulations, qui sont retirés.
   *[other] { $count } titres contenaient des retours à la ligne ou des tabulations, qui sont retirés.
}
import-scrivener-structure-own-text-moved-line = { $count ->
    [one] { $count } dossier a son propre texte, qui passe dans un premier document à l’intérieur.
   *[other] { $count } dossiers ont leur propre texte, qui passe dans un premier document à l’intérieur.
}
import-scrivener-structure-text-with-children-line = { $count ->
    [one] { $count } document contenant d’autres documents devient un dossier.
   *[other] { $count } documents contenant d’autres documents deviennent des dossiers.
}
import-scrivener-structure-empty-folder-plain-line = { $count ->
    [one] { $count } dossier vide reste un dossier simple : aucun export n’imprime un titre que Scrivener n’imprimait pas.
   *[other] { $count } dossiers vides restent des dossiers simples : aucun export n’imprime un titre que Scrivener n’imprimait pas.
}
import-scrivener-structure-book-inside-book-line = { $count ->
    [one] { $count } livre à l’intérieur d’un livre devient une partie.
   *[other] { $count } livres à l’intérieur d’un livre deviennent des parties.
}
import-scrivener-structure-prologue-not-numbered-line = { $count ->
    [one] { $count } prologue ou épilogue est laissé hors de la numérotation des chapitres.
   *[other] { $count } prologues ou épilogues sont laissés hors de la numérotation des chapitres.
}
import-scrivener-structure-matter-inside-book-line = { $count ->
    [one] { $count } élément de paratexte pris en dehors du manuscrit va dans le livre, exclu des exports jusqu’à ce que vous l’incluiez.
   *[other] { $count } éléments de paratexte pris en dehors du manuscrit vont dans le livre, exclus des exports jusqu’à ce que vous les incluiez.
}
import-scrivener-structure-paratext-not-counted-line = { $documents ->
    [one] { $documents } document du manuscrit devient du paratexte : { $words ->
        [one] { $words } mot ne compte
       *[other] { $words } mots ne comptent
    } donc plus dans le manuscrit.
   *[other] { $documents } documents du manuscrit deviennent du paratexte : { $words ->
        [one] { $words } mot ne compte
       *[other] { $words } mots ne comptent
    } donc plus dans le manuscrit.
}
import-scrivener-structure-document-notes-line = { $count ->
    [one] Les notes de { $count } document deviennent une note à côté de lui.
   *[other] Les notes de { $count } documents deviennent des notes à côté d’eux.
}
import-scrivener-structure-draft-content-as-note-line = Le texte propre du dossier du manuscrit devient une note en tête du manuscrit.
import-scrivener-structure-story-bible-folder-line = { $count ->
    [one] { $count } dossier de personnages ou de lieux devient une étiquette de la bible narrative.
   *[other] { $count } dossiers de personnages ou de lieux deviennent des étiquettes de la bible narrative.
}
import-scrivener-structure-templates-created-line = { $count ->
    [one] { $count } modèle de document devient un modèle de note.
   *[other] { $count } modèles de document deviennent des modèles de note.
}
import-scrivener-structure-template-nesting-flattened-line = { $count ->
    [one] { $count } dossier de modèles imbriqué est aplati : chaque texte qu’il contient devient un modèle.
   *[other] { $count } dossiers de modèles imbriqués sont aplatis : chaque texte qu’ils contiennent devient un modèle.
}
import-scrivener-structure-template-kept-as-note-line = { $count ->
    [one] { $count } modèle contient ce qu’un modèle de note ne peut pas contenir, et est gardé comme note.
   *[other] { $count } modèles contiennent ce qu’un modèle de note ne peut pas contenir, et sont gardés comme notes.
}
import-scrivener-structure-recovered-texts-line = { $count ->
    [one] { $count } texte qu’aucun élément du classeur ne répertoriait arrive dans un dossier à part.
   *[other] { $count } textes qu’aucun élément du classeur ne répertoriait arrivent dans un dossier à part.
}
import-scrivener-structure-project-notes-line = { $count ->
    [one] { $count } note de projet arrive dans un dossier à part.
   *[other] { $count } notes de projet arrivent dans un dossier à part.
}
# $items est le nombre d’éléments de la corbeille, $writing combien d’entre eux contiennent
# du texte.
import-scrivener-structure-trash-brought-across-line = { $items ->
    [1] La corbeille est reprise : { $items } élément, que vous pouvez restaurer depuis la corbeille.
   *[other] La corbeille est reprise : { $items } éléments, { $writing ->
        [0] dont aucun avec du texte
       *[other] dont { $writing } avec du texte
    }, que vous pouvez restaurer depuis la corbeille.
}
import-scrivener-structure-not-compiled-line = { $count ->
    [one] { $count } élément que Scrivener excluait de sa compilation est aussi exclu des exports.
   *[other] { $count } éléments que Scrivener excluait de sa compilation sont aussi exclus des exports.
}
import-scrivener-structure-media-in-draft-line = { $count ->
    [one] { $count } image ou fichier dans le manuscrit devient une note, puisqu’un livre ne peut pas le contenir.
   *[other] { $count } images ou fichiers dans le manuscrit deviennent des notes, puisqu’un livre ne peut pas les contenir.
}
import-scrivener-structure-left-out-line = { $count ->
    [one] { $count } dossier de premier niveau est laissé de côté, avec { $items ->
        [one] { $items } élément
       *[other] { $items } éléments
    }.
   *[other] { $count } dossiers de premier niveau sont laissés de côté, avec { $items ->
        [one] { $items } élément
       *[other] { $items } éléments
    }.
}
# $items est le nombre d’éléments de la corbeille, $writing combien d’entre eux contiennent
# du texte.
import-scrivener-structure-trash-left-out-line = { $items ->
    [1] La corbeille est laissée de côté : { $items } élément.
   *[other] La corbeille est laissée de côté : { $items } éléments, { $writing ->
        [0] dont aucun avec du texte
       *[other] dont { $writing } avec du texte
    }.
}
import-scrivener-structure-card-image-left-behind-line = { $count ->
    [one] { $count } image de fiche sur un élément du manuscrit est laissée de côté : elle s’imprimerait dans le livre.
   *[other] { $count } images de fiche sur des éléments du manuscrit sont laissées de côté : elles s’imprimeraient dans le livre.
}
import-scrivener-structure-default-template-not-linked-line = { $count ->
    [one] { $count } dossier avait un modèle par défaut pour les nouveaux documents qu’il contient. Dans Skribisto, seule une étiquette de la bible narrative peut porter un modèle : ce lien n’est donc pas gardé.
   *[other] { $count } dossiers avaient un modèle par défaut pour les nouveaux documents qu’ils contiennent. Dans Skribisto, seule une étiquette de la bible narrative peut porter un modèle : ces liens ne sont donc pas gardés.
}
# « Saut de page avant » est le libellé de ce réglage dans Scrivener.
import-scrivener-structure-page-break-not-kept-line = { $count ->
    [one] { $count } réglage « Saut de page avant » n’a pas de place dans Skribisto et n’est pas gardé.
   *[other] { $count } réglages « Saut de page avant » n’ont pas de place dans Skribisto et ne sont pas gardés.
}
# « Compiler tel quel » est le libellé de ce réglage dans Scrivener.
import-scrivener-structure-compile-as-is-not-kept-line = { $count ->
    [one] { $count } réglage « Compiler tel quel » n’a pas d’équivalent dans Skribisto et n’est pas gardé.
   *[other] { $count } réglages « Compiler tel quel » n’ont pas d’équivalent dans Skribisto et ne sont pas gardés.
}

## Ce que l’import a fait des libellés, de la liste d’états et des dates
import-scrivener-metadata-status-categories-guessed-line = { $count ->
    [one] { $count } statut que Skribisto ne reconnaît pas reçoit un type d’après sa place dans la liste. Vérifiez-le dans Paramètres ▸ Œuvre ▸ Statuts.
   *[other] { $count } statuts que Skribisto ne reconnaît pas reçoivent un type d’après leur place dans la liste. Vérifiez-les dans Paramètres ▸ Œuvre ▸ Statuts.
}
import-scrivener-metadata-dates-unreadable-line = { $count ->
    [one] Les dates de { $count } élément n’ont pas pu être lues : il porte la date de l’import.
   *[other] Les dates de { $count } éléments n’ont pas pu être lues : ils portent la date de l’import.
}
# Les libellés de Scrivener. $rows est le nombre d’éléments qui portent les nouvelles
# étiquettes.
import-scrivener-metadata-labels-became-tags-line = { $tags ->
    [one] Les libellés deviennent { $tags } étiquette, sur { $rows ->
        [one] { $rows } élément
       *[other] { $rows } éléments
    }.
   *[other] Les libellés deviennent { $tags } étiquettes, sur { $rows ->
        [one] { $rows } élément
       *[other] { $rows } éléments
    }.
}
# $rungs est le nombre d’états de la liste de Scrivener ; $rows combien d’éléments en portent un.
import-scrivener-metadata-statuses-became-ladder-line = { $rungs ->
    [one] La liste d’états devient la liste de statuts du projet : { $rungs } statut, sur { $rows ->
        [one] { $rows } élément
       *[other] { $rows } éléments
    }.
   *[other] La liste d’états devient la liste de statuts du projet : { $rungs } statuts, sur { $rows ->
        [one] { $rows } élément
       *[other] { $rows } éléments
    }.
}
import-scrivener-metadata-ladder-seeded-line = { $count ->
    [1] Le projet n’avait pas de liste d’états : il commence avec le statut par défaut de Skribisto.
   *[other] Le projet n’avait pas de liste d’états : il commence avec les { $count } statuts par défaut de Skribisto.
}
import-scrivener-metadata-label-renamed-line = Le champ Libellé s’appelait « { $title } » dans Scrivener. Ce nom figure dans les détails de chaque étiquette.
import-scrivener-metadata-status-renamed-line = Le champ État s’appelait « { $title } » dans Scrivener. Ce nom figure dans les détails de chaque statut.
import-scrivener-metadata-none-label-collapsed-line = { $count ->
    [one] Le libellé « { $name } », gardé à côté du libellé vide de Scrivener sous le même nom, est lu comme aucun libellé sur { $count } élément.
   *[other] Le libellé « { $name } », gardé à côté du libellé vide de Scrivener sous le même nom, est lu comme aucun libellé sur { $count } éléments.
}
import-scrivener-metadata-none-status-collapsed-line = { $count ->
    [one] L’état « { $name } », gardé à côté de l’état vide de Scrivener sous le même nom, est lu comme aucun état sur { $count } élément.
   *[other] L’état « { $name } », gardé à côté de l’état vide de Scrivener sous le même nom, est lu comme aucun état sur { $count } éléments.
}
import-scrivener-metadata-unnamed-label-line = Un libellé sans nom est nommé d’après son numéro, { $id }.
import-scrivener-metadata-unnamed-status-line = Un état sans nom est nommé d’après son numéro, { $id }.
import-scrivener-metadata-tag-name-disambiguated-line = { $count ->
    [one] { $count } étiquette prend un nouveau nom, car une autre étiquette porte déjà le sien.
   *[other] { $count } étiquettes prennent un nouveau nom, car d’autres étiquettes portent déjà le leur.
}
import-scrivener-metadata-rung-names-disambiguated-line = { $count ->
    [one] { $count } statut porte le même nom qu’un autre : il prend un numéro après son nom. Vérifiez-le dans Paramètres ▸ Œuvre ▸ Statuts.
   *[other] { $count } statuts portent le même nom qu’un autre : chacun prend un numéro après son nom. Vérifiez-les dans Paramètres ▸ Œuvre ▸ Statuts.
}
import-scrivener-metadata-unused-labels-skipped-line = { $count ->
    [one] { $count } libellé qu’aucun document ne porte n’est pas importé.
   *[other] { $count } libellés qu’aucun document ne porte ne sont pas importés.
}
import-scrivener-metadata-labels-without-row-line = { $count ->
    [one] Le libellé de { $count } élément sans place dans le nouveau projet n’est pas repris.
   *[other] Les libellés de { $count } éléments sans place dans le nouveau projet ne sont pas repris.
}
import-scrivener-metadata-statuses-without-row-line = { $count ->
    [one] L’état de { $count } élément sans place dans le nouveau projet n’est pas repris.
   *[other] Les états de { $count } éléments sans place dans le nouveau projet ne sont pas repris.
}

import-scrivener-metadata-unnamed-keyword-line = Un mot-clé sans nom est nommé d’après son numéro, { $id }.
import-scrivener-metadata-keywords-without-row-line = { $count ->
    [one] Les mots-clés de { $count } élément sans place dans le nouveau projet ne sont pas repris.
   *[other] Les mots-clés de { $count } éléments sans place dans le nouveau projet ne sont pas repris.
}
import-scrivener-metadata-custom-values-without-row-line = { $count ->
    [one] Les métadonnées personnalisées de { $count } élément sans place dans le nouveau projet ne sont pas reprises.
   *[other] Les métadonnées personnalisées de { $count } éléments sans place dans le nouveau projet ne sont pas reprises.
}
import-scrivener-metadata-bookmarks-without-row-line = { $count ->
    [one] Les signets de { $count } élément sans place dans le nouveau projet ne sont pas repris.
   *[other] Les signets de { $count } éléments sans place dans le nouveau projet ne sont pas repris.
}
import-scrivener-metadata-targets-without-row-line = { $count ->
    [one] L’objectif de { $count } élément sans place dans le nouveau projet n’est pas repris.
   *[other] Les objectifs de { $count } éléments sans place dans le nouveau projet ne sont pas repris.
}

## Ce que l’import fait des mots-clés, des métadonnées personnalisées, des collections, des
## signets, des objectifs, de l’échéance, de la liste d’autocomplétion et des icônes. Chaque
## ligne est dite deux fois : sur la page Vérifier avant l’import, puis dans son compte
## rendu ; elle se lit donc de la même façon aux deux moments. `$note` et `$folder` sont les
## titres que l’import donne à une note et à son dossier « Venu de Scrivener » ; `$day` est un
## jour au format AAAA-MM-JJ.
import-scrivener-metadata-pov-hint-line = { $count ->
    [1] Le libellé ou la valeur de champ { $values } correspond à la fiche de la bible narrative { $matches }. S’il désigne le point de vue, indiquez-le dans l’Inspecteur : l’import ne relie rien.
   *[other] Les libellés ou valeurs de champ { $values } correspondent aux fiches de la bible narrative { $matches }. S’ils désignent des points de vue, indiquez-les dans l’Inspecteur : l’import ne relie rien.
}
import-scrivener-metadata-checkbox-default-unknown-line = Le champ case à cocher « { $field } » a une valeur par défaut que Skribisto ne sait pas lire : il ne peut pas savoir quels documents sont cochés par défaut. Seuls les documents cochés un par un portent son étiquette.
import-scrivener-metadata-goals-in-other-unit-line = { $count ->
    [one] { $count } objectif est fixé dans l’autre unité de comptage. Il est gardé, et apparaît dès que vous changez d’unité dans Paramètres ▸ Œuvre ▸ Structure.
   *[other] { $count } objectifs sont fixés dans l’autre unité de comptage. Ils sont gardés, et apparaissent dès que vous changez d’unité dans Paramètres ▸ Œuvre ▸ Structure.
}
import-scrivener-metadata-counting-differs-line = Scrivener comptait l’objectif du manuscrit sur d’autres documents que Skribisto. L’objectif du livre compte les scènes incluses dans les exports.
import-scrivener-metadata-writing-days-not-read-line = Les jours d’écriture « { $raw } » ne peuvent pas être lus : le Rythme du livre compte donc chaque jour comme un jour d’écriture. Modifiez-les là.
import-scrivener-metadata-script-mode-rows-line = { $count ->
    [one] { $count } élément était en mode script dans Scrivener. Skribisto garde ses mots en prose, sans la mise en page du scénario.
   *[other] { $count } éléments étaient en mode script dans Scrivener. Skribisto garde leurs mots en prose, sans la mise en page du scénario.
}
import-scrivener-metadata-keywords-became-tags-line = { $tags ->
    [one] { $tags } mot-clé devient une étiquette, sur { $rows ->
        [one] { $rows } élément
       *[other] { $rows } éléments
    }.
   *[other] { $tags } mots-clés deviennent des étiquettes, sur { $rows ->
        [one] { $rows } élément
       *[other] { $rows } éléments
    }.
}
import-scrivener-metadata-keyword-hierarchy-flattened-line = { $count ->
    [one] { $count } mot-clé était rangé dans un autre dans Scrivener. Les étiquettes n’ont pas de niveaux : les détails de son étiquette disent où il était.
   *[other] { $count } mots-clés étaient rangés dans d’autres dans Scrivener. Les étiquettes n’ont pas de niveaux : les détails de chaque étiquette disent où était son mot-clé.
}
# $fields champs deviennent $tags étiquettes ; $rows est le nombre d’éléments qui les portent.
import-scrivener-metadata-fields-became-tags-line = { $fields ->
    [one] { $fields } champ liste ou case à cocher devient { $tags ->
        [one] { $tags } étiquette
       *[other] { $tags } étiquettes
    }, sur { $rows ->
        [one] { $rows } élément
       *[other] { $rows } éléments
    }.
   *[other] { $fields } champs liste et case à cocher deviennent { $tags ->
        [one] { $tags } étiquette
       *[other] { $tags } étiquettes
    }, sur { $rows ->
        [one] { $rows } élément
       *[other] { $rows } éléments
    }.
}
import-scrivener-metadata-collections-became-tags-line = { $tags ->
    [one] { $tags } collection devient une étiquette, sur { $rows ->
        [one] { $rows } élément
       *[other] { $rows } éléments
    }.
   *[other] { $tags } collections deviennent des étiquettes, sur { $rows ->
        [one] { $rows } élément
       *[other] { $rows } éléments
    }.
}
import-scrivener-metadata-collection-order-lost-line = { $count ->
    [one] Une étiquette n’a pas d’ordre : l’ordre des documents de { $count } collection n’est pas gardé.
   *[other] Une étiquette n’a pas d’ordre : l’ordre des documents de { $count } collections n’est pas gardé.
}
import-scrivener-metadata-bookmarks-became-links-line = { $count ->
    [one] { $count } signet devient un lien entre deux notes, ou entre une scène et une fiche de la bible narrative : voir Liens et Présence.
   *[other] { $count } signets deviennent des liens entre notes, ou entre scènes et fiches de la bible narrative : voir Liens et Présence.
}
# Un objectif Scrivener (target) devient un objectif Skribisto (goal).
import-scrivener-metadata-goals-kept-line = { $count ->
    [one] L’objectif que Scrivener fixait à { $count } document devient son objectif dans Skribisto.
   *[other] Les objectifs que Scrivener fixait à { $count } documents deviennent leurs objectifs dans Skribisto.
}
import-scrivener-metadata-draft-target-kept-words-line = { $value ->
    [one] L’objectif du manuscrit, { $value } mot, devient l’objectif du livre.
   *[other] L’objectif du manuscrit, { $value } mots, devient l’objectif du livre.
}
import-scrivener-metadata-draft-target-kept-characters-line = { $value ->
    [one] L’objectif du manuscrit, { $value } caractère, devient l’objectif du livre.
   *[other] L’objectif du manuscrit, { $value } caractères, devient l’objectif du livre.
}
# « Rythme » est le nom de la page Rythme d’un livre (segment-pace).
import-scrivener-metadata-pace-planned-line = L’échéance, { $day }, met en place le Rythme du livre, qui planifie les mots de chaque jour d’écriture choisi dans Scrivener, à partir du jour de l’import.
import-scrivener-metadata-pace-planned-day-off-line = L’échéance, { $day }, met en place le Rythme du livre, qui planifie les mots de chaque jour d’écriture choisi dans Scrivener, à partir du jour de l’import. Comme dans Scrivener, le jour de l’échéance est un congé.
import-scrivener-metadata-dictionary-words-line = { $count ->
    [one] { $count } mot d’autocomplétion entre dans le dictionnaire personnel du projet : il n’est pas signalé comme faute.
   *[other] { $count } mots d’autocomplétion entrent dans le dictionnaire personnel du projet : ils ne sont pas signalés comme fautes.
}
import-scrivener-metadata-listed-project-line = { $count ->
    [one] { $count } renseignement sur le projet figure dans la note « { $note } » du dossier « { $folder } ».
   *[other] { $count } renseignements sur le projet figurent dans la note « { $note } » du dossier « { $folder } ».
}
import-scrivener-metadata-listed-project-bookmarks-line = { $count ->
    [one] { $count } signet du projet figure dans la note « { $note } » du dossier « { $folder } ».
   *[other] { $count } signets du projet figurent dans la note « { $note } » du dossier « { $folder } ».
}
import-scrivener-metadata-listed-replacements-line = { $count ->
    [one] { $count } remplacement à la compilation figure dans la note « { $note } » du dossier « { $folder } ». Skribisto ne le fait pas.
   *[other] { $count } remplacements à la compilation figurent dans la note « { $note } » du dossier « { $folder } ». Skribisto ne les fait pas.
}
import-scrivener-metadata-listed-auto-complete-line = { $count ->
    [one] { $count } entrée d’autocomplétion de plusieurs mots figure dans la note « { $note } » du dossier « { $folder } ».
   *[other] { $count } entrées d’autocomplétion de plusieurs mots figurent dans la note « { $note } » du dossier « { $folder } ».
}
import-scrivener-metadata-listed-saved-searches-line = { $count ->
    [one] { $count } recherche enregistrée figure dans la note « { $note } » du dossier « { $folder } ».
   *[other] { $count } recherches enregistrées figurent dans la note « { $note } » du dossier « { $folder } ».
}
import-scrivener-metadata-listed-custom-metadata-line = { $count ->
    [one] { $count } valeur de métadonnée personnalisée qu’une étiquette ne peut pas porter figure dans la note « { $note } » du dossier « { $folder } ».
   *[other] { $count } valeurs de métadonnées personnalisées qu’une étiquette ne peut pas porter figurent dans la note « { $note } » du dossier « { $folder } ».
}
import-scrivener-metadata-listed-targets-line = { $count ->
    [one] { $count } objectif de document figure dans la note « { $note } » du dossier « { $folder } ».
   *[other] { $count } objectifs de document figurent dans la note « { $note } » du dossier « { $folder } ».
}
import-scrivener-metadata-listed-bookmarks-line = { $count ->
    [one] { $count } signet qu’aucun lien de Skribisto ne montre figure dans la note « { $note } » du dossier « { $folder } ».
   *[other] { $count } signets qu’aucun lien de Skribisto ne montre figurent dans la note « { $note } » du dossier « { $folder } ».
}
import-scrivener-metadata-targets-on-notes-line = { $count ->
    [one] { $count } objectif de document porte sur une note, dont aucun objectif ne compte l’écriture : il ne devient pas un objectif.
   *[other] { $count } objectifs de document portent sur des notes, dont aucun objectif ne compte l’écriture : ils ne deviennent pas des objectifs.
}
import-scrivener-metadata-targets-on-split-documents-line = { $count ->
    [one] { $count } objectif de document porte sur un document découpé à ses titres, dont aucun objectif ne compte ensemble les chapitres : il ne devient pas un objectif.
   *[other] { $count } objectifs de document portent sur des documents découpés à leurs titres, dont aucun objectif ne compte ensemble les chapitres : ils ne deviennent pas des objectifs.
}
import-scrivener-metadata-bookmark-ends-without-row-line = { $count ->
    [one] { $count } signet mène à un document sans place dans le nouveau projet, comme un modèle de document. Il est indiqué par la place du document dans Scrivener.
   *[other] { $count } signets mènent à des documents sans place dans le nouveau projet, comme des modèles de document. Ils sont indiqués par la place des documents dans Scrivener.
}
import-scrivener-metadata-unused-keywords-skipped-line = { $count ->
    [one] { $count } mot-clé qu’aucun document ne porte n’est pas importé.
   *[other] { $count } mots-clés qu’aucun document ne porte ne sont pas importés.
}
import-scrivener-metadata-list-value-invalid-line = { $count ->
    [one] { $count } valeur du champ liste « { $field } » ne désigne aucune de ses options : elle n’est pas importée.
   *[other] { $count } valeurs du champ liste « { $field } » ne désignent aucune de ses options : elles ne sont pas importées.
}
import-scrivener-metadata-undefined-custom-field-line = { $count ->
    [one] { $count } valeur appartient à un champ personnalisé que le projet ne définit plus, et que Scrivener n’affiche pas non plus. Elle n’est pas importée.
   *[other] { $count } valeurs appartiennent à des champs personnalisés que le projet ne définit plus, et que Scrivener n’affiche pas non plus. Elles ne sont pas importées.
}
import-scrivener-metadata-collection-members-missing-line = { $count ->
    [one] { $count } document de la collection « { $name } » n’a pas de place dans le nouveau projet, ou a disparu du projet Scrivener.
   *[other] { $count } documents de la collection « { $name } » n’ont pas de place dans le nouveau projet, ou ont disparu du projet Scrivener.
}
import-scrivener-metadata-empty-collections-skipped-line = { $count ->
    [one] { $count } collection qui ne contient aucun document n’est pas importée.
   *[other] { $count } collections qui ne contiennent aucun document ne sont pas importées.
}
import-scrivener-metadata-bookmarks-unresolved-line = { $count ->
    [one] { $count } signet mène à un document que le projet Scrivener ne contient plus : il n’est pas importé.
   *[other] { $count } signets mènent à des documents que le projet Scrivener ne contient plus : ils ne sont pas importés.
}
import-scrivener-metadata-targets-in-pages-line = { $count ->
    [one] { $count } objectif de document compte des pages, ce que Skribisto ne compte pas : il ne devient pas un objectif.
   *[other] { $count } objectifs de document comptent des pages, ce que Skribisto ne compte pas : ils ne deviennent pas des objectifs.
}
import-scrivener-metadata-targets-unreadable-line = { $count ->
    [one] { $count } objectif n’est pas importé : son nombre n’en est pas un, est négatif, ou dépasse ce qu’un objectif peut contenir.
   *[other] { $count } objectifs ne sont pas importés : leur nombre n’en est pas un, est négatif, ou dépasse ce qu’un objectif peut contenir.
}
import-scrivener-metadata-draft-target-in-pages-line = { $value ->
    [one] L’objectif du manuscrit, { $value } page, ne devient pas un objectif : Skribisto compte des mots ou des caractères. Il figure dans la note « { $note } » du dossier « { $folder } ».
   *[other] L’objectif du manuscrit, { $value } pages, ne devient pas un objectif : Skribisto compte des mots ou des caractères. Il figure dans la note « { $note } » du dossier « { $folder } ».
}
import-scrivener-metadata-draft-target-no-book-words-line = { $value ->
    [one] L’objectif du manuscrit, { $value } mot, ne devient pas un objectif, car le manuscrit donne plusieurs livres, ou aucun. Il figure dans la note « { $note } » du dossier « { $folder } ».
   *[other] L’objectif du manuscrit, { $value } mots, ne devient pas un objectif, car le manuscrit donne plusieurs livres, ou aucun. Il figure dans la note « { $note } » du dossier « { $folder } ».
}
import-scrivener-metadata-draft-target-no-book-characters-line = { $value ->
    [one] L’objectif du manuscrit, { $value } caractère, ne devient pas un objectif, car le manuscrit donne plusieurs livres, ou aucun. Il figure dans la note « { $note } » du dossier « { $folder } ».
   *[other] L’objectif du manuscrit, { $value } caractères, ne devient pas un objectif, car le manuscrit donne plusieurs livres, ou aucun. Il figure dans la note « { $note } » du dossier « { $folder } ».
}
# « Rythme » est le nom de la page Rythme d’un livre (segment-pace).
import-scrivener-metadata-deadline-passed-line = L’échéance, { $day }, est passée : elle ne met donc pas en place le Rythme du livre. Elle figure dans la note « { $note } » du dossier « { $folder } ».
import-scrivener-metadata-deadline-no-writing-day-line = Il ne reste aucun jour d’écriture entre le jour de l’import et l’échéance, { $day } : elle ne met donc pas en place le Rythme du livre. Elle figure dans la note « { $note } » du dossier « { $folder } ».
import-scrivener-metadata-deadline-template-line = L’échéance, { $day }, est celle enregistrée dans le modèle : elle ne met donc pas en place le Rythme du livre. Elle figure dans la note « { $note } » du dossier « { $folder } ».
import-scrivener-metadata-deadline-no-word-goal-line = L’échéance, { $day }, ne met pas en place le Rythme du livre, car le manuscrit n’a pas d’objectif de mots vers lequel avancer. Elle figure dans la note « { $note } » du dossier « { $folder } ».
import-scrivener-metadata-deadline-character-goal-line = L’échéance, { $day }, ne met pas en place le Rythme du livre, car l’objectif du manuscrit compte des caractères et le Rythme compte des mots. Elle figure dans la note « { $note } » du dossier « { $folder } ».
import-scrivener-metadata-deadline-no-book-line = L’échéance, { $day }, ne met en place le Rythme d’aucun livre, car le manuscrit donne plusieurs livres, ou aucun. Elle figure dans la note « { $note } » du dossier « { $folder } ».
import-scrivener-metadata-deadline-unreadable-line = L’échéance ne peut pas être lue : elle ne met donc pas en place le Rythme du livre. Elle figure telle qu’écrite dans la note « { $note } » du dossier « { $folder } ».
import-scrivener-metadata-session-target-words-line = { $value ->
    [one] L’objectif de session, { $value } mot, n’est pas importé : dans Skribisto, l’objectif de la session d’écriture est un seul réglage pour tous les projets. Il figure dans la note « { $note } » du dossier « { $folder } ».
   *[other] L’objectif de session, { $value } mots, n’est pas importé : dans Skribisto, l’objectif de la session d’écriture est un seul réglage pour tous les projets. Il figure dans la note « { $note } » du dossier « { $folder } ».
}
import-scrivener-metadata-session-target-characters-line = { $value ->
    [one] L’objectif de session, { $value } caractère, n’est pas importé : dans Skribisto, l’objectif de la session d’écriture est un seul réglage pour tous les projets, en mots. Il figure dans la note « { $note } » du dossier « { $folder } ».
   *[other] L’objectif de session, { $value } caractères, n’est pas importé : dans Skribisto, l’objectif de la session d’écriture est un seul réglage pour tous les projets, en mots. Il figure dans la note « { $note } » du dossier « { $folder } ».
}
import-scrivener-metadata-session-target-pages-line = { $value ->
    [one] L’objectif de session, { $value } page, n’est pas importé : dans Skribisto, l’objectif de la session d’écriture est un seul réglage pour tous les projets, en mots. Il figure dans la note « { $note } » du dossier « { $folder } ».
   *[other] L’objectif de session, { $value } pages, n’est pas importé : dans Skribisto, l’objectif de la session d’écriture est un seul réglage pour tous les projets, en mots. Il figure dans la note « { $note } » du dossier « { $folder } ».
}
import-scrivener-metadata-session-target-calculated-words-line = { $value ->
    [one] L’objectif de session, { $value } mot, calculé par Scrivener d’après l’échéance et les jours d’écriture, n’est pas importé : dans Skribisto, l’objectif de la session d’écriture est un seul réglage pour tous les projets. Il figure dans la note « { $note } » du dossier « { $folder } ».
   *[other] L’objectif de session, { $value } mots, calculé par Scrivener d’après l’échéance et les jours d’écriture, n’est pas importé : dans Skribisto, l’objectif de la session d’écriture est un seul réglage pour tous les projets. Il figure dans la note « { $note } » du dossier « { $folder } ».
}
import-scrivener-metadata-session-target-calculated-characters-line = { $value ->
    [one] L’objectif de session, { $value } caractère, calculé par Scrivener d’après l’échéance et les jours d’écriture, n’est pas importé : dans Skribisto, l’objectif de la session d’écriture est un seul réglage pour tous les projets, en mots. Il figure dans la note « { $note } » du dossier « { $folder } ».
   *[other] L’objectif de session, { $value } caractères, calculé par Scrivener d’après l’échéance et les jours d’écriture, n’est pas importé : dans Skribisto, l’objectif de la session d’écriture est un seul réglage pour tous les projets, en mots. Il figure dans la note « { $note } » du dossier « { $folder } ».
}
import-scrivener-metadata-session-target-calculated-pages-line = { $value ->
    [one] L’objectif de session, { $value } page, calculé par Scrivener d’après l’échéance et les jours d’écriture, n’est pas importé : dans Skribisto, l’objectif de la session d’écriture est un seul réglage pour tous les projets, en mots. Il figure dans la note « { $note } » du dossier « { $folder } ».
   *[other] L’objectif de session, { $value } pages, calculé par Scrivener d’après l’échéance et les jours d’écriture, n’est pas importé : dans Skribisto, l’objectif de la session d’écriture est un seul réglage pour tous les projets, en mots. Il figure dans la note « { $note } » du dossier « { $folder } ».
}
import-scrivener-metadata-icons-dropped-line = { $count ->
    [one] { $count } élément avait une icône dans le classeur, qui n’est pas reprise : Skribisto montre l’icône de chaque élément d’après son type.
   *[other] { $count } éléments avaient des icônes dans le classeur, qui ne sont pas reprises : Skribisto montre l’icône de chaque élément d’après son type.
}

## Ce que la conversion de la prose a fait, comme le dit le compte rendu de l’import. `$count` est
## un nombre de textes ou de choses, selon la ligne ; `$example` cite le projet.
import-scrivener-prose-unreadable-line = { $count ->
    [one] { $count } texte n’a pas pu être lu. Il arrive vide.
   *[other] { $count } textes n’ont pas pu être lus. Ils arrivent vides.
}
import-scrivener-prose-comments-unreadable-line = { $count ->
    [one] Les commentaires et les notes de bas de page liées de { $count } texte n’ont pas pu être lus. Les mots qu’ils marquaient sont gardés.
   *[other] Les commentaires et les notes de bas de page liées de { $count } textes n’ont pas pu être lus. Les mots qu’ils marquaient sont gardés.
}
import-scrivener-prose-read-as-plain-text-line = { $count ->
    [one] { $count } texte était enregistré en texte brut plutôt qu’en texte mis en forme, et il est lu comme tel.
   *[other] { $count } textes étaient enregistrés en texte brut plutôt qu’en texte mis en forme, et ils sont lus comme tels.
}
import-scrivener-prose-inline-note-unmatched-line = { $count ->
    [one] { $count } marque de note dans le texte ne forme pas de paire et est gardée comme texte.
   *[other] { $count } marques de notes dans le texte ne forment pas de paire et sont gardées comme texte.
}
import-scrivener-prose-style-unresolved-line = { $count ->
    [one] { $count } marque de style nomme un style que le projet ne définit pas. Elle est retirée, et le texte garde sa mise en forme.
   *[other] { $count } marques de style nomment un style que le projet ne définit pas. Elles sont retirées, et le texte garde sa mise en forme.
}
import-scrivener-prose-styles-lost-in-export-line = { $count ->
    [one] { $count } paragraphe est peut-être un titre qui a perdu son style quand Scrivener 3 a exporté le projet vers son ancien format, et arrive comme un paragraphe ordinaire. Vérifiez-le, ou importez le projet d’origine de l’export.
   *[other] { $count } paragraphes sont peut-être des titres qui ont perdu leur style quand Scrivener 3 a exporté le projet vers son ancien format, et arrivent comme des paragraphes ordinaires. Vérifiez-les, ou importez le projet d’origine de l’export.
}
import-scrivener-prose-comment-anchor-without-entry-line = { $count ->
    [one] { $count } commentaire ou note de bas de page avait perdu son contenu dans Scrivener. Les mots qu’il marquait sont gardés, sans lui.
   *[other] { $count } commentaires ou notes de bas de page avaient perdu leur contenu dans Scrivener. Les mots qu’ils marquaient sont gardés, sans eux.
}
import-scrivener-prose-comment-entry-without-anchor-line = { $count ->
    [one] { $count } commentaire n’avait plus de place dans son texte et est rattaché au premier paragraphe.
   *[other] { $count } commentaires n’avaient plus de place dans leur texte et sont rattachés au premier paragraphe.
}
import-scrivener-prose-footnote-entry-without-anchor-line = { $count ->
    [one] { $count } note de bas de page liée n’avait plus de place dans son texte : elle est citée à la fin de ce texte.
   *[other] { $count } notes de bas de page liées n’avaient plus de place dans leur texte : chacune est citée à la fin de son propre texte.
}
import-scrivener-prose-range-comment-reading-line = { $count ->
    [one] { $count } commentaire d’un Scrivener plus ancien a été placé par une autre lecture de sa position. Vérifiez qu’il porte sur les bons mots.
   *[other] { $count } commentaires d’un Scrivener plus ancien ont été placés par une autre lecture de leur position. Vérifiez qu’ils portent sur les bons mots.
}
import-scrivener-prose-range-comment-reading-markers-counted-line = { $count ->
    [one] { $count } commentaire d’un Scrivener plus ancien a été placé en comptant les marques de ses notes dans le texte. Vérifiez qu’il porte sur les bons mots.
   *[other] { $count } commentaires d’un Scrivener plus ancien ont été placés en comptant les marques de leurs notes dans le texte. Vérifiez qu’ils portent sur les bons mots.
}
import-scrivener-prose-range-comment-reading-pictures-not-counted-line = { $count ->
    [one] { $count } commentaire d’un Scrivener plus ancien a été placé en laissant ses images de côté. Vérifiez qu’il porte sur les bons mots.
   *[other] { $count } commentaires d’un Scrivener plus ancien ont été placés en laissant leurs images de côté. Vérifiez qu’ils portent sur les bons mots.
}
import-scrivener-prose-range-comment-approximate-line = { $count ->
    [one] { $count } commentaire d’un Scrivener plus ancien ne correspond exactement à aucune lecture de sa position, et peut être décalé de quelques caractères.
   *[other] { $count } commentaires d’un Scrivener plus ancien ne correspondent exactement à aucune lecture de leur position, et peuvent être décalés de quelques caractères.
}
import-scrivener-prose-document-link-unresolved-line = { $count ->
    [one] { $count } lien mène à un document que l’import n’a pas repris. Ses mots restent.
   *[other] { $count } liens mènent à des documents que l’import n’a pas repris. Leurs mots restent.
}
import-scrivener-prose-linked-picture-not-read-line = { $count ->
    [1] { $count } image liée à un fichier de votre disque, « { $example } », n’a pas été reprise.
   *[other] { $count } images liées à des fichiers de votre disque, comme « { $example } », n’ont pas été reprises.
}
import-scrivener-prose-heading-repeats-title-line = { $count ->
    [one] { $count } texte commence par un titre qui répète son nom, si bien qu’un export peut imprimer le titre deux fois.
   *[other] { $count } textes commencent par un titre qui répète leur nom, si bien qu’un export peut imprimer le titre deux fois.
}
import-scrivener-prose-markdown-as-text-line = { $count ->
    [one] { $count } paragraphe écrit en Markdown est gardé tel quel.
   *[other] { $count } paragraphes écrits en Markdown sont gardés tels quels.
}
import-scrivener-prose-citation-keys-as-text-line = { $count ->
    [one] { $count } paragraphe contient des clés de citation pour un logiciel de bibliographie, gardées comme texte.
   *[other] { $count } paragraphes contiennent des clés de citation pour un logiciel de bibliographie, gardées comme texte.
}
# Une balise est ici l’un des codes de Scrivener, comme <$n> ou <$projecttitle>, que
# Scrivener remplace par du texte à la compilation.
import-scrivener-prose-placeholders-in-exported-text-line = { $count ->
    [1] { $count } balise Scrivener, { $example }, est gardée telle quelle dans ce que vous exportez. Remplacez-la à la main.
   *[other] { $count } balises Scrivener, comme { $example }, sont gardées telles quelles dans ce que vous exportez. Remplacez-les à la main.
}
import-scrivener-prose-placeholders-in-other-text-line = { $count ->
    [1] { $count } balise Scrivener, { $example }, est gardée telle quelle dans un texte qui n’est pas exporté.
   *[other] { $count } balises Scrivener, comme { $example }, sont gardées telles quelles dans des textes qui ne sont pas exportés.
}
import-scrivener-prose-ornament-lines-kept-line = { $count ->
    [one] { $count } ligne d’ornements centrée peut marquer un séparateur de scènes et est gardée telle quelle.
   *[other] { $count } lignes d’ornements centrées peuvent marquer des séparateurs de scènes et sont gardées telles quelles.
}
import-scrivener-prose-long-document-line = { $count ->
    [one] { $count } texte est très long, et peut être lent à ouvrir.
   *[other] { $count } textes sont très longs, et peuvent être lents à ouvrir.
}
import-scrivener-prose-pictures-without-description-line = { $count ->
    [one] { $count } image n’a pas de description pour les lecteurs qui ne la voient pas. Vous pouvez en ajouter une dans l’éditeur.
   *[other] { $count } images n’ont pas de description pour les lecteurs qui ne les voient pas. Vous pouvez en ajouter une dans l’éditeur.
}
import-scrivener-prose-struck-text-compiled-away-line = { $count ->
    [one] Le texte barré de { $count } texte est gardé. La compilation de Scrivener le retirait du livre, mais un export de Skribisto l’imprime.
   *[other] Le texte barré de { $count } textes est gardé. La compilation de Scrivener le retirait du livre, mais un export de Skribisto l’imprime.
}
import-scrivener-prose-compile-deleted-style-printed-line = { $count ->
    [1] Le texte au style « { $example } », que la compilation de Scrivener supprimait, est gardé comme prose, et un export de Skribisto l’imprime.
   *[other] Le texte de { $count } styles que la compilation de Scrivener supprimait, comme « { $example } », est gardé comme prose, et un export de Skribisto l’imprime.
}
import-scrivener-prose-compile-raw-style-printed-line = { $count ->
    [1] Le texte au style « { $example } », que la compilation de Scrivener transmettait comme du code pour un autre logiciel, est gardé comme prose, et un export de Skribisto l’imprime.
   *[other] Le texte de { $count } styles que la compilation de Scrivener transmettait comme du code pour un autre logiciel, comme « { $example } », est gardé comme prose, et un export de Skribisto l’imprime.
}
import-scrivener-prose-left-out-after-document-line = { $count ->
    [one] { $count } morceau d’un fichier abîmé, placé après sa fin, est laissé de côté.
   *[other] { $count } morceaux de fichiers abîmés, placés après leur fin, sont laissés de côté.
}
import-scrivener-prose-left-out-header-or-footer-line = { $count ->
    [one] { $count } en-tête ou pied de page est laissé de côté : il ne fait pas partie du texte.
   *[other] { $count } en-têtes et pieds de page sont laissés de côté : ils ne font pas partie du texte.
}
import-scrivener-prose-left-out-list-label-picture-line = { $count ->
    [one] { $count } image servant de puce de liste est laissée de côté.
   *[other] { $count } images servant de puces de liste sont laissées de côté.
}
import-scrivener-prose-left-out-line = { $count ->
    [one] { $count } morceau d’un fichier qu’aucun texte ne contient est laissé de côté.
   *[other] { $count } morceaux de fichiers qu’aucun texte ne contient sont laissés de côté.
}
import-scrivener-prose-undecodable-characters-line = { $count ->
    [one] { $count } caractère n’a pas pu être décodé et apparaît comme un signe de remplacement (�) là où il se trouvait.
   *[other] { $count } caractères n’ont pas pu être décodés et apparaissent comme des signes de remplacement (�) là où ils se trouvaient.
}
import-scrivener-prose-inline-notes-merged-line = { $count ->
    [one] { $count } note dans le texte que Scrivener avait coupée là où sa mise en forme changeait est recollée.
   *[other] { $count } notes dans le texte que Scrivener avait coupées là où leur mise en forme changeait sont recollées.
}
import-scrivener-prose-inline-notes-joined-across-paragraphs-line = { $count ->
    [1] { $count } note dans le texte écrite un paragraphe à la fois est réunie en une seule note.
   *[other] { $count } notes dans le texte écrites un paragraphe à la fois sont réunies en notes de plusieurs paragraphes.
}
import-scrivener-prose-blank-notes-dropped-line = { $count ->
    [one] { $count } note vide dans le texte est laissée de côté.
   *[other] { $count } notes vides dans le texte sont laissées de côté.
}
import-scrivener-prose-annotations-became-comments-line = { $count ->
    [one] { $count } annotation dans le texte est devenue un commentaire.
   *[other] { $count } annotations dans le texte sont devenues des commentaires.
}
import-scrivener-prose-annotation-only-document-kept-line = { $count ->
    [one] { $count } document ne contenait qu’une annotation. Ses mots sont gardés comme prose.
   *[other] { $count } documents ne contenaient qu’une annotation. Leurs mots sont gardés comme prose.
}
# « Preserve Formatting » est le nom de ce format dans Scrivener.
import-scrivener-prose-preserve-formatting-passages-line = { $count ->
    [one] { $count } passage marqué pour conserver sa mise en forme arrive comme du texte ordinaire.
   *[other] { $count } passages marqués pour conserver leur mise en forme arrivent comme du texte ordinaire.
}
import-scrivener-prose-nested-note-unwrapped-line = { $count ->
    [one] { $count } note placée dans un commentaire ou une note de bas de page est gardée comme texte, là où elle se trouvait.
   *[other] { $count } notes placées dans des commentaires ou des notes de bas de page sont gardées comme texte, là où elles se trouvaient.
}
import-scrivener-prose-style-not-carried-line = { $count ->
    [1] Le style « { $example } » n’a pas d’équivalent dans Skribisto. Son texte garde ses mots.
   *[other] { $count } styles, comme « { $example } », n’ont pas d’équivalent dans Skribisto. Leur texte garde ses mots.
}
import-scrivener-prose-document-links-dropped-line = { $count ->
    [one] { $count } lien vers un autre document n’a pas de place dans Skribisto. Ses mots restent.
   *[other] { $count } liens vers d’autres documents n’ont pas de place dans Skribisto. Leurs mots restent.
}
import-scrivener-prose-document-links-kept-line = { $count ->
    [one] { $count } lien vers un autre document est gardé comme référence.
   *[other] { $count } liens vers d’autres documents sont gardés comme références.
}
import-scrivener-prose-other-links-kept-as-text-line = { $count ->
    [one] { $count } lien vers un autre projet ou vers un fichier est gardé comme texte.
   *[other] { $count } liens vers d’autres projets ou vers des fichiers sont gardés comme texte.
}
import-scrivener-prose-soft-breaks-split-line = { $count ->
    [one] { $count } retour à la ligne dans un paragraphe commence désormais un paragraphe à part.
   *[other] { $count } retours à la ligne dans des paragraphes commencent désormais des paragraphes à part.
}
import-scrivener-prose-typed-indents-removed-line = { $count ->
    [one] { $count } retrait tapé au début d’un paragraphe est retiré.
   *[other] { $count } retraits tapés au début des paragraphes sont retirés.
}
import-scrivener-prose-left-indents-not-kept-line = { $count ->
    [one] { $count } paragraphe en retrait de la marge perd son retrait.
   *[other] { $count } paragraphes en retrait de la marge perdent leur retrait.
}
import-scrivener-prose-blank-lines-removed-line = { $count ->
    [one] { $count } paragraphe vide entre deux autres est retiré.
   *[other] { $count } paragraphes vides entre d’autres sont retirés.
}
import-scrivener-prose-verse-stanza-gaps-lost-line = { $count ->
    [one] { $count } ligne vide entre les strophes d’un poème est retirée, si bien que les strophes se suivent.
   *[other] { $count } lignes vides entre les strophes d’un poème sont retirées, si bien que les strophes se suivent.
}
import-scrivener-prose-blank-lines-became-scene-breaks-line = { $count ->
    [one] { $count } ligne vide entre des paragraphes est devenue un séparateur de scènes.
   *[other] { $count } lignes vides entre des paragraphes sont devenues des séparateurs de scènes.
}
import-scrivener-prose-ornaments-became-scene-breaks-line = { $count ->
    [one] { $count } ligne d’ornements est devenue un séparateur de scènes.
   *[other] { $count } lignes d’ornements sont devenues des séparateurs de scènes.
}
import-scrivener-prose-table-first-row-header-line = { $count ->
    [one] { $count } tableau commence désormais par une ligne d’en-tête.
   *[other] { $count } tableaux commencent désormais par une ligne d’en-tête.
}
import-scrivener-prose-table-cells-joined-line = { $count ->
    [one] { $count } cellule de tableau contenait plusieurs paragraphes, désormais réunis en un seul.
   *[other] { $count } cellules de tableau contenaient plusieurs paragraphes, désormais réunis en un seul.
}
import-scrivener-prose-formatting-lost-colour-line = { $count ->
    [one] La couleur du texte est retirée dans { $count } texte.
   *[other] La couleur du texte est retirée dans { $count } textes.
}
import-scrivener-prose-formatting-lost-highlight-line = { $count ->
    [one] Le surlignage est retiré dans { $count } texte.
   *[other] Le surlignage est retiré dans { $count } textes.
}
import-scrivener-prose-formatting-lost-small-caps-line = { $count ->
    [one] Les petites capitales s’affichent en lettres ordinaires dans { $count } texte.
   *[other] Les petites capitales s’affichent en lettres ordinaires dans { $count } textes.
}
import-scrivener-prose-formatting-lost-all-caps-line = { $count ->
    [one] Le texte affiché en capitales s’affiche tel qu’il a été tapé dans { $count } texte.
   *[other] Le texte affiché en capitales s’affiche tel qu’il a été tapé dans { $count } textes.
}
import-scrivener-prose-formatting-lost-hidden-line = { $count ->
    [one] Le texte masqué est laissé de côté dans { $count } texte.
   *[other] Le texte masqué est laissé de côté dans { $count } textes.
}
import-scrivener-prose-formatting-lost-line = { $count ->
    [one] Une mise en forme sans place dans Skribisto est retirée dans { $count } texte.
   *[other] Une mise en forme sans place dans Skribisto est retirée dans { $count } textes.
}
import-scrivener-prose-justified-text-not-kept-line = { $count ->
    [one] { $count } paragraphe justifié suit désormais le style de l’export.
   *[other] { $count } paragraphes justifiés suivent désormais le style de l’export.
}
import-scrivener-prose-comment-colours-lost-line = { $count ->
    [one] { $count } commentaire perd sa couleur.
   *[other] { $count } commentaires perdent leur couleur.
}
import-scrivener-prose-control-characters-removed-line = { $count ->
    [one] { $count } caractère de contrôle invisible est retiré.
   *[other] { $count } caractères de contrôle invisibles sont retirés.
}
import-scrivener-prose-note-on-heading-lost-line = { $count ->
    [one] { $count } note de bas de page ou commentaire sur un titre de chapitre ne pouvait pas y rester : un commentaire passe au premier paragraphe du chapitre, une note de bas de page est laissée de côté.
   *[other] { $count } notes de bas de page ou commentaires sur des titres de chapitre ne pouvaient pas y rester : un commentaire passe au premier paragraphe du chapitre, une note de bas de page est laissée de côté.
}
import-scrivener-prose-media-file-copied-line = { $count ->
    [one] { $count } fichier de recherche que Skribisto ne sait pas ouvrir a été copié à côté du nouveau projet. Sa note dit où.
   *[other] { $count } fichiers de recherche que Skribisto ne sait pas ouvrir ont été copiés à côté du nouveau projet. Leurs notes disent où.
}
import-scrivener-prose-media-file-not-carried-line = { $count ->
    [one] { $count } fichier de recherche que Skribisto ne sait pas ouvrir reste dans votre projet Scrivener. Sa note le nomme.
   *[other] { $count } fichiers de recherche que Skribisto ne sait pas ouvrir restent dans votre projet Scrivener. Leurs notes les nomment.
}

## Quand l’import échoue
## Affiché dans l’avis d’un import qui a échoué, au-dessus du bouton
## import-scrivener-review-again, que ces phrases citent.
import-scrivener-error-refused = { $refusal } Rien n’a été écrit.
import-scrivener-error-question = Le projet pose maintenant une question à laquelle sa vérification n’a pas répondu. Choisissez « Revoir l’import », puis importez. Rien n’a été écrit.
# « Projet » est la première étape de l’assistant (import-scrivener-step-project) ; la case
# est import-scrivener-ios-pending-accept ou import-scrivener-no-text-accept.
import-scrivener-error-not-accepted = Avant de pouvoir être importé, le projet vous demande de cocher une case à l’étape Projet, et elle n’a pas été cochée. Choisissez « Revoir l’import », cochez-la, puis importez. Rien n’a été écrit.
import-scrivener-error-changed = Le projet a changé depuis que vous l’avez vérifié. Choisissez « Revoir l’import », puis importez. Rien n’a été écrit.
import-scrivener-error-choice = La structure choisie lors de la vérification ne correspond plus à ce projet. Choisissez « Revoir l’import », puis importez. Rien n’a été écrit.

## Dialogue d'exportation
export-title = Exporter
export-close = Fermer
export-scope-label = Quoi
export-format-label = Format
export-style-label = Style
export-path-label = Fichier
export-browse = Parcourir…
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
# Étiquettes récapitulatives du style
export-chip-chapters-none = Chapitres : aucun
export-chip-chapters-numbered = Chapitres : numérotés
export-chip-chapters-title = Chapitres : titre seul
export-chip-chapters-both = Chapitres : numéro + titre
export-chip-scene-break-glyph = Saut de scène : { $glyph }
export-chip-scene-break-blank = Saut de scène : ligne vide
export-chip-scene-break-none = Saut de scène : aucun
export-chip-major-break-glyph = Saut majeur : { $glyph }
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
search-tip-synopsis = Synopsis : rechercher dans le résumé de chaque élément d’écriture.
search-tip-label = Libellé : rechercher dans le court texte affiché sous le titre d’un élément.
search-tip-comment = Commentaires : rechercher dans le texte des fils de commentaires et de leurs réponses. Un commentaire porte sur le manuscrit sans en faire partie, d’où son propre bouton, et un remplacement laisse les commentaires décochés tant que vous ne les cochez pas.
search-tip-book = Livres : le conteneur du livre et ses marqueurs de début / fin.
search-tip-part = Parties : les séparateurs de partie.
search-tip-chapter = Chapitres : les chapitres, quel que soit leur stockage.
search-tip-scene = Scènes : les éléments qui contiennent votre prose.
search-tip-note = Notes : les notes libres.
search-tip-folder = Dossiers : les simples dossiers d’organisation et séparateurs.
search-tip-paratext = Un texte qui appartient au livre mais non à son récit : une préface, une dédicace, une postface. Jamais compté dans le manuscrit.
search-tip-preserve-case = Conserver la casse : le remplacement reprend la casse trouvée, « ELENA » devient « MARTA » et « Elena » devient « Marta ».
search-tip-replace = Remplacer : afficher le champ de remplacement et « Tout remplacer ».
search-error = Échec de la recherche : { $message }
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
search-include-in-replace = Inclure dans « Tout remplacer »
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
        [1] Ce texte a changé
       *[other] { $fields } de ces textes ont changé
    } depuis la recherche, { $fields ->
        [1] il n’a donc pas été modifié
       *[other] ils n’ont donc pas été modifiés
    }. Relancez la recherche pour voir où sont les mots.
search-replace-undo-failed-title = Échec de l’annulation
search-preview = Aperçu
search-preview-empty = Sélectionnez un résultat pour l’afficher ici
search-preview-no-prose = Ce résultat n’a pas de texte modifiable
search-preview-prompt = Pour afficher un aperçu ici, lancez une recherche.
search-preview-open-search = Rechercher dans le projet
search-preview-footnote-prompt = Ce résultat se trouve dans le texte d’une note de bas de page : ouvrez-la dans le panneau Notes de bas de page pour la consulter et la modifier.
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
undo-superseded-body = Autre chose a changé dans ce projet depuis. Annuler reviendrait sur cette autre modification : rien n’a donc été fait. Cette étape reste en place. Utilisez Édition ▸ Annuler pour remonter l’historique vous-même.
undo-failed = Échec de l’annulation : { $error }

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
    [1] Une scène ouverte a été restaurée depuis un état antérieur : son historique de saisie ne s’applique plus.
   *[other] { $count } scènes ouvertes ont été restaurées depuis un état antérieur : leur historique de saisie ne s’applique plus.
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
undo-frozen = Annuler (« Droit devant » est actif)
redo-frozen = Rétablir (« Droit devant » est actif)
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
titlebar-distraction-free = Passer en mode sans distraction (F11)
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
trash-restore-orphaned = L’emplacement d’origine de cet élément n’existe plus. Choisissez où le restaurer.
trash-restore-no-project = Aucun projet n’est ouvert, il n’y a donc rien à restaurer.
trash-delete-no-project = Aucun projet n’est ouvert, il n’y a donc rien à supprimer.
trash-empty-confirm-title = Vider la corbeille ?
trash-empty-confirm-text = { $count ->
    [1] L’élément de la corbeille sera définitivement supprimé.
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
trash-restore-picker-empty = Aucun classeur pour l’instant. Créez-en un d’abord.
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

## Marques de formatage : les caractères invisibles que l'auteur pose à la main.
## Les mnémoniques sont uniques dans le sous-menu (E, T, c, n, u, J).
menu-format-typo-marks = Ma&rque de formatage
menu-format-typo-nbsp = &Espace insécable
menu-format-typo-nbhyphen = Trait d’union &insécable
menu-format-typo-soft-hyphen = Trait d’union &conditionnel
menu-format-typo-nnbsp = Espace fi&ne insécable
menu-format-typo-zwsp = Espace de largeur n&ulle
menu-format-typo-word-joiner = &Gluon de mots

## Fenêtre « À propos »

menu-about = À &propos de Skribisto…
about-title = À propos de Skribisto
about-version = Version { $version }
about-update-available = La version { $version } est disponible
about-update-see-notes = Voir les nouveautés
about-update-flathub = Flathub tient cette copie à jour.
about-update-distro = Votre distribution tient cette copie à jour.
about-tagline = Une application d’écriture de romans pour la fiction longue, écrite en Rust avec la boîte à outils Teksilo.
about-license = Distribué sous la Licence publique générale GNU, version 3.
about-copyright = © 2026 Cyril Jacquet
about-close = Fermer

## Barre de menus native (macOS)
# Libellés des menus standard « Application » et « Fenêtre » recopiés dans la
# barre de menus globale de macOS. Pas de mnémonique `&` ici : macOS n'en a pas,
# et le pont natif ne les retire pas : une esperluette s'afficherait telle
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
window-title = { $title } - { $app }
window-title-numbered = { $title } - { $app } (Fenêtre { $n })
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

# ── Work ▸ Punctuation : le style typographique du projet ───────────────────
settings-page-punctuation = Réglages de ponctuation
settings-group-punctuation = Ponctuation intelligente
settings-page-work-punctuation = Ponctuation
settings-punctuation-override = Donner à ce projet ses propres règles de ponctuation
settings-punctuation-override-hint = Désactivé : le projet suit la préférence de l’application. Ces règles voyagent dans le .skrib : un co-auteur qui ouvre le fichier écrit avec la même typographie.
settings-punctuation-dashes = Transformer -- en tiret demi-cadratin, --- en cadratin
settings-punctuation-ellipsis = Transformer ... en points de suspension
settings-punctuation-quotes = Courber les guillemets et les apostrophes
settings-quote-style = Guillemets
settings-quote-style-locale = Selon la langue
settings-quote-style-curly = “Doubles”
settings-quote-style-curly-single = ‘Simples’
settings-quote-style-guillemets = «Chevrons»
settings-quote-style-low-high = „Bas-haut“
settings-punctuation-spacing = Espace avant ; : ! ?
settings-punctuation-spacing-hint = La typographie française place une espace fine insécable avant le point-virgule, le point d’exclamation et le point d’interrogation, et une espace insécable avant les deux-points. Ne s’applique qu’au texte écrit en français. L’espace à l’intérieur des guillemets « » accompagne les guillemets eux-mêmes.
settings-punctuation-sample = Votre langue donne
settings-punctuation-app-hint = Ce que fait chaque projet, sauf s’il adopte ses propres règles dans Projet ▸ Ponctuation.
settings-punctuation-dialogue = Ouvrir un paragraphe saisi "- " par un tiret de dialogue
settings-punctuation-dialogue-hint = Pour les langues qui marquent le dialogue par un tiret plutôt que par des guillemets : français, espagnol, russe et d’autres. Ne se déclenche qu’en tout début de paragraphe.
settings-punctuation-auto-lists = Commencer une liste par un paragraphe saisi "- ", "* " ou "1. "
settings-punctuation-auto-lists-hint = Le paragraphe devient une liste à puces ou numérotée. Si le tiret de dialogue est activé et que la langue en a un, "- " ouvre plutôt une réplique. Un retour arrière juste après rend ce que vous avez saisi.

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
# évidence qui masque la prose : un autre défaut, invisible sur les pastilles
# de la ligne.
settings-themes-low-contrast-band = la mise en évidence masque le texte
settings-themes-field-name = Nom
settings-themes-field-paper = Page
settings-themes-field-ink = Texte
settings-themes-field-general = Arrière-plan
settings-themes-field-widget-text = Texte de la bande de contrôle
# La teinte dessinée autour du curseur : la phrase ou le paragraphe en cours
# d'écriture, selon Éditeur ▸ « Mise en évidence autour du curseur ». Ce champ
# en donne la couleur en mode sans distraction.
settings-themes-field-caret-band = Mise en évidence autour du curseur

# L'engrenage de paramètres rapides de la bande sans distraction, et l'accès à la
# bibliothèque complète de thèmes depuis le mode.
statusbar-focus-settings = Paramètres du mode sans distraction
statusbar-focus-manage-themes = Gérer les thèmes…

## Commentaires : les deux panneaux, les fiches de fil et leurs actions.
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
# position réelle à indiquer : importé depuis un format sans notion de position
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
comments-sort-reader = Groupés par lecteur
comments-unsigned-author = Non signé
comments-agreement = { $count ->
    [one] { $count } lecteur
   *[other] { $count } lecteurs
}
comments-menu-add = Ajouter un commentaire
comments-menu-add-paragraph = Commenter ce paragraphe
overview-col-comments = Commentaires
overview-col-total-comments = Total commentaires
comments-card-placeholder = Écrire un commentaire…
comments-card-unknown-author = Auteur inconnu
comments-unsigned-toast = Vos commentaires ne sont pas signés : aucun nom n’est défini pour vous sur cet ordinateur.
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
analysis-median-words = La scène médiane de ce livre compte { $count ->
    [one] { $count } mot.
   *[other] { $count } mots.
}
analysis-median-line = Médiane : { $count ->
    [one] { $count } mot
   *[other] { $count } mots
}
analysis-dialogue = Dialogue
analysis-dialogue-unsupported = Le dialogue n’est pas encore mesuré pour cette langue.

analysis-footnote-words = Mots en notes de bas de page
analysis-footnote-words-count = { $count ->
    [one] { $count } mot se trouve dans les notes de bas de page de ce livre, compté à part du total du manuscrit.
   *[other] { $count } mots se trouvent dans les notes de bas de page de ce livre, comptés à part du total du manuscrit.
}
analysis-footnote-words-pending = Comptage des notes de bas de page…

analysis-ignore-empty = Ignorer les textes encore vides
analysis-empty-hidden = { $count } { $count ->
        [1] texte vide masqué
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
settings-paratext-intro = Les pages liminaires et les annexes d’un nouveau projet. Chaque structure appartient à une tradition éditoriale, et ses titres sont écrits dans la langue de cette tradition. Renommez-les librement une fois le projet créé.
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

# La couverture du livre, choisie depuis le livre, non insérée dans une scène.
cover-heading = Couverture
cover-pick = Choisir…
cover-change = Changer…
cover-remove = Retirer
cover-none = Pas encore de couverture
cover-missing = Le fichier de l’image manque dans le projet
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
        [1] Un commentaire n’a pas pu être placé dans le texte exporté et a été omis.
       *[other] { $count } commentaires n’ont pas pu être placés dans le texte exporté et ont été omis.
    }
export-orphan-footnotes =
    { $count ->
        [1] Une note de bas de page n’est plus appelée nulle part dans le manuscrit. Son texte n’apparaîtra pas dans le livre exporté.
       *[other] { $count } notes de bas de page ne sont plus appelées dans le manuscrit. Leur texte n’apparaîtra pas dans le livre exporté.
    }

## Notes de bas de page

menu-footnotes = Notes de bas de pa&ge
footnotes-title = Notes de bas de page
footnotes-insert = Insérer une note de bas de p&age
footnotes-empty = Aucune note de bas de page pour l’instant. Placez le curseur dans une scène, puis utilisez + ci-dessus, ou Ctrl+Alt+F.
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
footnotes-no-caret = Placez le curseur dans le texte d’une scène pour y insérer une note de bas de page.
footnotes-not-created = La note n’a pas pu être ajoutée au projet.
footnotes-deleted-toast = Note de bas de page supprimée
footnotes-undo-delete = Annuler

# ── Importer des documents (Markdown / Word / ODT / texte brut) ───────────────
import-document-title = Importer des documents
import-document-close = Fermer
import-document-step-files = Fichiers
import-document-step-review = Vérifier
import-document-step-destination = Destination
import-document-step-reconcile = Fusion
import-document-reconcile-hint = Certains de ces éléments existent déjà dans votre projet. Indiquez ce qu’il faut faire de chacun.
import-document-reconcile-all-new = Rien dans ce fichier ne correspond à votre projet : chaque élément sera ajouté comme nouveau.
import-document-duplicate-returns = Ces fichiers sont plusieurs exemplaires du même manuscrit qui reviennent. Importez-les un par un : la mise en correspondance ne confronte qu’un seul exemplaire à votre projet, si bien qu’en importer plusieurs ensemble ajouterait une seconde copie du livre au lieu de la fusionner.
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
    [one] { $count } fichier
   *[other] { $count } fichiers
}
import-document-no-files = Aucun fichier choisi pour l’instant.
import-document-col-included = Importer
import-document-col-title = Titre
import-document-col-type = Type
import-document-col-words = Mots
import-document-col-breaks = Sauts
import-document-col-comments = Commentaires
import-document-col-footnotes = Notes de bas de page
import-document-col-epigraph = Épigraphe
import-document-col-source = Source
import-document-level-rules = Niveaux de titre
import-document-level-n = Titre { $level }
import-document-add-top-level = Ajouter un niveau supérieur
import-document-add-top-level-tooltip = Insère un Livre au-dessus de tous les éléments analysés, pour des chapitres sans titre de livre
import-document-destination = Destination
import-document-destination-hint = Choisissez un classeur ou un élément : les nouveaux éléments arrivent dans un dossier, ou après une scène.
import-document-destination-empty = Aucun classeur pour l’instant. Créez-en un d’abord.
import-document-plan-empty = Rien à importer pour l’instant.
import-document-pictures = { $count ->
    [1] Une image accompagne ces documents.
   *[other] { $count } images accompagnent ces documents.
}
import-document-summary = { $rows ->
    [one] { $rows } élément
   *[other] { $rows } éléments
} · { $breaks ->
    [one] { $breaks } saut de scène
   *[other] { $breaks } sauts de scène
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
    [one] { $count } élément importé
   *[other] { $count } éléments importés
}
import-document-undo = Annuler l’import
# ── Diagnostics d'import ──────────────────────────────────────────────────────
import-diagnostic-file-unreadable = « { $path } » n’a pas pu être lu : { $detail }. Les autres fichiers sont importés quand même.
# Fichier refusé sans être lu : son XML est imbriqué au-delà de ce que
# l’application accepte. $limit est le plafond, un nombre.
import-diagnostic-nested-too-deep = « { $path } » n’a pas été lu : son contenu est imbriqué sur plus de { $limit } niveaux. Aucun document réel n’en approche, et le lire pourrait provoquer l’arrêt brutal de Skribisto ; il est donc écarté. Les autres fichiers sont importés quand même.
import-diagnostic-archive-too-large = « { $path } » n’a pas été lu : c’est un fichier compressé conçu pour se décompresser bien au-delà de sa taille, ce qui pourrait saturer la mémoire et provoquer l’arrêt brutal de Skribisto ; il est donc écarté. Les autres fichiers sont importés quand même.
import-diagnostic-lossy-decode = { $count ->
    [1] Un caractère de « { $path } » n’a pas pu être décodé. Enregistrez le fichier en UTF-8 pour le conserver.
   *[other] { $count } caractères de « { $path } » n’ont pas pu être décodés. Enregistrez le fichier en UTF-8 pour les conserver.
}
import-diagnostic-decoded-from-bom = « { $path } » a été décodé en { $detail }, pas en UTF-8.
import-diagnostic-empty-file = « { $path } » est vide.
import-diagnostic-no-headings = « { $path } » ne contient aucun titre : il arrive en un seul élément.
import-diagnostic-unsupported-format = Aucun lecteur ne prend en charge les fichiers « .{ $detail } » : « { $path } » a été ignoré.
import-diagnostic-front-matter-not-flat = En-tête de « { $path } » : « { $detail } » n’est pas une valeur simple et a été ignoré.
import-diagnostic-footnotes-degraded = { $count ->
    [1] Une note de bas de page de « { $path } » arrive en texte brut : les notes ne sont pas lues depuis Markdown.
   *[other] { $count } notes de bas de page de « { $path } » arrivent en texte brut : les notes ne sont pas lues depuis Markdown.
}
import-diagnostic-footnote-not-carried = { $count ->
    [1] Une note de bas de page de « { $path } » n’a pas pu être reprise. Les autres l’ont été.
   *[other] { $count } notes de bas de page de « { $path } » n’ont pas pu être reprises. Les autres l’ont été.
}
import-diagnostic-raw-html-dropped = { $count ->
    [1] Un bloc HTML brut de « { $path } » a été supprimé.
   *[other] { $count } blocs HTML bruts de « { $path } » ont été supprimés.
}
import-diagnostic-nested-break-dropped = { $count ->
    [1] Un saut de scène situé dans une citation ou une liste de « { $path } » a été supprimé. Seul un saut placé seul sur sa ligne est conservé.
   *[other] { $count } sauts de scène situés dans des citations ou des listes de « { $path } » ont été supprimés. Seul un saut placé seul sur sa ligne est conservé.
}
import-diagnostic-image-not-ingested-format = Dans « { $path } », l’image « { $detail } » est dans un format que Skribisto ne sait pas afficher : elle n’a pas été importée. Les images PNG, JPEG, WebP, GIF, TIFF et BMP sont importées ; enregistrez-la dans l’un de ces formats et insérez-la à la main.
import-diagnostic-image-not-ingested-remote = Dans « { $path } », l’image « { $detail } » se trouve sur le Web. Une importation ne télécharge jamais d’image : elle n’a pas été importée.
import-diagnostic-image-not-ingested-outside = Dans « { $path } », l’image « { $detail } » se trouve hors du document et de son dossier : elle n’a pas été importée. Seules les images enregistrées dans le document, ou dans le dossier du document, sont importées.
import-diagnostic-image-not-ingested-missing = Dans « { $path } », l’image « { $detail } » est introuvable : elle n’a pas été importée.
import-diagnostic-image-not-ingested-too-large = Dans « { $path } », l’image « { $detail } » dépasse la taille qu’un projet accepte : elle n’a pas été importée.
import-diagnostic-image-not-ingested-unreadable = Dans « { $path } », l’image « { $detail } » ne peut pas être lue comme une image : elle n’a pas été importée.
import-diagnostic-image-not-ingested-heading = Dans « { $path } », l’image « { $detail } » se trouve dans un titre. Chaque titre devient l’intitulé d’un élément du classeur, et un intitulé ne contient pas d’image : l’image n’a pas été importée.
import-diagnostic-image-not-ingested-note = Dans « { $path } », l’image « { $detail } » se trouve dans une note de bas de page, une note de fin ou un commentaire. Ceux-ci gardent leurs mots, mais pas d’image : elle n’a pas été importée.
import-diagnostic-image-not-ingested-html = Dans « { $path } », l’image « { $detail } » est écrite en HTML, qu’une importation laisse de côté : elle n’a pas été importée. Écrivez-la comme une image Markdown pour l’importer.
import-diagnostic-image-not-ingested-document-full = Dans « { $path } », l’image « { $detail } » n’a pas été importée : les images de ce document dépassent ensemble ce qu’un import fait entrer. Importez le document en plusieurs fois, ou insérez les autres images à la main.
import-diagnostic-picture-first-frame-only = Dans « { $path } », l’image animée « { $detail } » arrive sous la forme de sa première image. Les images d’un projet ne sont pas animées.
import-diagnostic-duplicate-title = « { $title } » apparaît { $count } fois. Si vous avez déjà importé ces fichiers, cela les dupliquera.
import-diagnostic-heading-level-jump = « { $title } » passe du niveau de titre { $from } au niveau { $to } ; il est placé un niveau sous son parent.
import-diagnostic-illegal-combination = « { $title } » contient du texte, mais un élément de type « { $kind } » ne peut pas en contenir. L’importation est suspendue tant que vous n’avez pas changé son type ou décoché l’élément : sinon rien du tout ne serait importé.
import-diagnostic-tracked-changes-flattened = { $path } était en cours de révision : { $count ->
    [one] { $count } modification suivie a été acceptée
   *[other] { $count } modifications suivies ont été acceptées
} et les suppressions écartées. C’est le texte final. Pour garder votre propre formulation, utilisez Comparer à la dernière étape.
import-diagnostic-tracked-changes-flattened-by = { $path } était en cours de révision : { $count ->
    [one] { $count } modification suivie de { $names } a été acceptée
   *[other] { $count } modifications suivies de { $names } ont été acceptées
} et les suppressions écartées. C’est le texte final. Pour garder votre propre formulation, utilisez Comparer à la dernière étape.
import-diagnostic-text-box-dropped = { $count ->
    [one] { $path } contient { $count } zone de texte. Son contenu est hors du fil du document : impossible de dire où il se place dans un manuscrit, elle n’est donc pas importée.
   *[other] { $path } contient { $count } zones de texte. Leur contenu est hors du fil du document : impossible de dire où il se place dans un manuscrit, elles ne sont donc pas importées.
}
import-diagnostic-embedded-object-dropped = { $count ->
    [one] { $path } contient { $count } objet incorporé : graphique, équation ou similaire. Rien dans un manuscrit ne peut l’accueillir.
   *[other] { $path } contient { $count } objets incorporés : graphique, équation ou similaire. Rien dans un manuscrit ne peut les accueillir.
}
import-diagnostic-field-flattened = { $count ->
    [one] { $path } contient { $count } champ : numéro de page, renvoi, date. Il conserve le texte affiché en dernier et ne se mettra plus à jour.
   *[other] { $path } contient { $count } champs : numéro de page, renvoi, date. Chacun conserve le texte affiché en dernier et ne se mettra plus à jour.
}
import-diagnostic-unknown-style-level = { $path } utilise le style « { $detail } », qui ressemble à un titre mais n’indique aucun niveau. Ces paragraphes sont importés comme texte plutôt que devinés.
import-diagnostic-comment-unanchored = Le commentaire « { $detail } » dans { $path } n’a pas pu être rattaché aux mots qu’il visait. Il est conservé sur son élément, où vous pouvez le déplacer.
import-diagnostic-comment-not-carried = Le commentaire « { $detail } » dans { $path } n’est pas importé, car rien dans le fichier n’arrive sous forme de texte où le conserver. Recopiez-le depuis le fichier si vous en avez besoin.
import-diagnostic-comment-replies-flattened = { $count ->
    [one] { $count } réponse dans { $path } désigne un commentaire absent du fichier : elle arrive donc comme un commentaire à part entière.
   *[other] { $count } réponses dans { $path } désignent un commentaire absent du fichier : elles arrivent donc comme des commentaires à part entière.
}
import-diagnostic-prose-not-verbatim = { $count ->
    [one] { $count } paragraphe de { $path } n’a pas pu être repris tel quel avec sa mise en forme : il arrive donc en texte brut. Comparez-le avec l’original.
   *[other] { $count } paragraphes de { $path } n’ont pas pu être repris tels quels avec leur mise en forme : ils arrivent donc en texte brut. Comparez-les avec l’original.
}
import-diagnostic-styled-spaces-not-carried = { $count ->
    [one] Dans { $path }, { $count } suite d’espaces était soulignée ou barrée, par exemple une ligne à remplir à la main. Skribisto ne garde aucun de ces traits sur des espaces seules : elle arrive donc sans trait, ou pas du tout si elle ouvre ou termine un paragraphe.
   *[other] Dans { $path }, { $count } suites d’espaces étaient soulignées ou barrées, par exemple des lignes à remplir à la main. Skribisto ne garde aucun de ces traits sur des espaces seules : elles arrivent donc sans trait, ou pas du tout quand elles ouvrent ou terminent un paragraphe.
}
# Éléments de liste imbriqués plus profondément que ce que l’importation conserve, placés au
# niveau le plus profond conservé. $count est leur nombre ; $limit ce niveau, un nombre.
import-diagnostic-list-nesting-flattened = { $count ->
    [one] { $count } élément de liste de { $path } était imbriqué sur plus de { $limit } niveaux. Il arrive au niveau { $limit }, le plus profond que Skribisto conserve, avec son texte et sa mise en forme.
   *[other] { $count } éléments de liste de { $path } étaient imbriqués sur plus de { $limit } niveaux. Ils arrivent au niveau { $limit }, le plus profond que Skribisto conserve, avec leur texte et leur mise en forme.
}
# Tableaux trop déséquilibrés pour être complétés en grille, une ligne bien plus large que
# de nombreuses lignes courtes, qui arrivent comme les paragraphes de leurs cellules.
# $count est le nombre de tableaux.
import-diagnostic-table-read-as-paragraphs = { $count ->
    [one] { $count } tableau de { $path } a une ligne bien plus large que les autres, trop déséquilibré pour être disposé en grille. Ses cellules arrivent comme des paragraphes, l’une après l’autre, avec leur texte et leur mise en forme.
   *[other] { $count } tableaux de { $path } ont une ligne bien plus large que les autres, trop déséquilibrés pour être disposés en grille. Leurs cellules arrivent comme des paragraphes, l’une après l’autre, avec leur texte et leur mise en forme.
}
# Suites d’espaces plus longues que ce que l’importation conserve, raccourcies à cette
# longueur. $count est leur nombre ; $limit le nombre d’espaces conservées au plus, un nombre.
import-diagnostic-spaces-shortened = { $count ->
    [one] { $count } suite d’espaces de { $path } comptait plus de { $limit } espaces : elle arrive raccourcie à { $limit }. Le texte qui l’entoure est intact.
   *[other] { $count } suites d’espaces de { $path } comptaient plus de { $limit } espaces : elles arrivent raccourcies à { $limit }. Le texte qui les entoure est intact.
}
import-diagnostic-epigraph-not-carried = « { $title } » est précédé d’une épigraphe, mais un élément de type « { $kind } » ne peut pas en porter. La citation est conservée en tête de son texte.
import-diagnostic-epigraph-placement-ambiguous = Une épigraphe se trouve entre « { $title } » et « { $below } » et pourrait précéder l’un ou l’autre. Elle a été attribuée à « { $title } », où se place habituellement une épigraphe.
import-document-diagnostics = { $errors ->
    [0] { $warnings ->
            [one] { $warnings } point à connaître
           *[other] { $warnings } points à connaître
        }
   *[other] { $errors ->
            [one] { $errors } fichier illisible
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
versions-error = Impossible de lire vos copies de secours. Rien n’est perdu, mais cette liste peut être incomplète
versions-did-not-exist = N’existait pas encore le { $date }
versions-deleted-after = Supprimé après le { $date }
versions-unreadable = { $count ->
    [one] { $count } copie de secours illisible
   *[other] { $count } copies de secours illisibles
}
versions-thinned = { $count ->
    [one] Les anciennes versions s’espacent avec le temps. L’historique interne du projet a déjà supprimé { $count } état antérieur de ce texte.
   *[other] Les anciennes versions s’espacent avec le temps. L’historique interne du projet a déjà supprimé { $count } états antérieurs de ce texte.
}
versions-source-backup = Depuis une copie de secours
versions-source-project = Depuis l’historique du projet
versions-list-caption = Une entrée par changement, pas par copie de secours
versions-pick-a-version = Choisissez une version pour voir ce qui a changé
versions-earliest = La plus ancienne version enregistrée. Il n’y a rien de plus ancien à quoi la comparer.
versions-no-change = Rien n’a changé dans cette partie
versions-formatting-only = Seule la mise en forme a changé : les mots sont les mêmes
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
versions-pin = Épingler la copie de secours d’où vient cette version. Le nettoyage automatique ne la supprimera jamais
versions-unpin = Désépingler la copie de secours d’où vient cette version. Le nettoyage automatique pourra de nouveau la supprimer
versions-pinned-only = N’afficher que les versions épinglées
versions-pin-note = Seules les versions issues d’une copie de secours peuvent être épinglées. Une épingle conserve un fichier, et l’historique interne du projet n’en est pas un.
versions-pinned-empty = Rien n’est encore épinglé ici
versions-pinned-empty-log = Rien n’est encore épinglé ici. Seules les versions issues d’une copie de secours peuvent être épinglées. Une épingle conserve un fichier, et l’historique interne du projet n’en est pas un.
versions-range-filter = N’afficher que les versions comprises entre deux dates
versions-filtered-empty = Aucune version ne correspond aux filtres définis
versions-clear-filters = Effacer les filtres
versions-last-30-days = 30 derniers jours
versions-restore-button = Restaurer cette version
versions-restore-confirm-title = Remplacer ce texte par la version du { $date } ?
versions-restore-confirm-text = Ce que vous avez maintenant sera remplacé par le texte de cet élément au { $date }.
versions-restore-confirm-with-comments = Ce que vous avez maintenant sera remplacé par le texte de cet élément au { $date }. { $count ->
    [one] { $count } commentaire est ancré dans le texte actuel et risque de devenir orphelin.
   *[other] { $count } commentaires sont ancrés dans le texte actuel et risquent de devenir orphelins.
}
versions-restore-confirm-undo-note = Une copie de sécurité est faite d’abord, et Ctrl+Z annule tout en une fois.
versions-restored-toast = Version du { $date } restaurée
versions-undo = Annuler
versions-restore-row-gone = Cet élément n’est plus dans ce projet
versions-restore-no-home = Cet élément a changé de type depuis, et l’ancien texte n’y a plus sa place
versions-restore-no-safety-copy = Votre copie de sécurité n’a pas été faite, rien n’a été modifié
versions-restore-failed = La restauration a échoué : { $error }
versions-restore-backup-busy = Une copie de secours est déjà en cours. Réessayez dans un instant
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
    [1] « { $item } » est de retour, mais l’un de ses textes n’a pas pu être lu
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
timeline-no-text-of-its-own = Ceci n’a pas de texte propre : c’est un intitulé pour ce qu’il contient
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

# Volet Jeux d'écriture : contraintes d'écriture volontaires
settings-group-games-forward = Droit devant
settings-games-forward-toggle = Jouer à « Droit devant »
settings-games-forward-hint =
    Tant que vous jouez, rien de ce que vous avez écrit ne peut être repris :
    Retour arrière, Suppr, Couper, le glisser-déposer et Annuler sont désactivés dans
    les surfaces choisies ci-dessous. Vous pouvez toujours écrire, coller, mettre en
    forme et vous déplacer librement. Le brouillon ne fait que grandir.
settings-games-session-warning =
    Ce choix ne vaut que pour la session en cours : il n’est jamais enregistré. Fermer
    le projet ou quitter Skribisto met toujours fin à la partie, et tout ce que vous
    avez écrit en jouant redevient annulable à cet instant.
settings-group-games-scope = Où cela s’applique
settings-games-in-prose = Texte du manuscrit
settings-games-in-synopsis = Synopsis
settings-games-scope-hint =
    Les commentaires, les notes de bas de page et les titres ne sont jamais figés :
    c’est là que vous notez la correction que vous venez de vous interdire.
settings-games-inert-warning =
    « Droit devant » est activé mais ne s’applique à rien : cochez au moins une surface
    ci-dessus, sinon cela ne change rien à votre écriture.

# Panneau Jeux d'écriture (rail de gauche) + l'avertissement de la barre d'état
games-title = Jeux d’écriture
games-forward-name = Droit devant
games-forward-blurb =
    Écrivez sans jamais revenir en arrière. Supprimer, couper et annuler sont
    désactivés pendant la partie : écrivez la phrase suivante plutôt que de corriger
    la précédente.
games-forward-playing = Partie en cours : la suppression est désactivée
games-forward-idle = Aucune partie en cours
games-session-note = La partie prend fin à la fermeture du projet.
games-scope-prose-and-synopsis = S’applique à votre texte et à vos synopsis.
games-scope-prose = S’applique à votre texte.
games-scope-synopsis = S’applique à vos synopsis.
games-scope-nothing = Ne s’applique à rien pour l’instant. Choisissez une surface dans les paramètres.
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
overview-excluded-from-export = Hors export : cet élément ne compte dans aucun total

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
pace-summary-open = Ouvrir le plan de rythme
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
menu-help-check-updates = Rechercher les &mises à jour
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
help-topic-drafts-and-old-versions = Conserver un ancien brouillon
help-topic-goals-and-pace = Objectifs et rythme
help-topic-comments = Commentaires
help-topic-round-trip = Envoyer votre livre à un lecteur
help-topic-export = Exporter
help-topic-import-documents = Importer des documents
help-topic-import-projects = Importer un projet entier
help-topic-import-scrivener = Venir de Scrivener
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
shortcut-name-work-import-scrivener = Importer un projet Scrivener…
shortcut-name-work-import-scrivener-report = Afficher le compte rendu de l’import Scrivener
shortcut-name-app-settings = Paramètres
shortcut-name-app-quit = Quitter
shortcut-name-editor-insert-footnote = Insérer une note de bas de page
shortcut-name-format-scene-break = Insérer un saut de scène
shortcut-name-format-major-scene-break = Insérer un saut de scène majeur
shortcut-name-format-link = Insérer un lien
shortcut-name-format-mark-nbsp = Insérer une espace insécable
shortcut-name-format-mark-nbhyphen = Insérer un trait d’union insécable
shortcut-name-format-mark-nnbsp = Insérer une espace fine insécable
shortcut-name-format-mark-zwsp = Insérer une espace de largeur nulle
shortcut-name-go-next = Suivant
shortcut-name-go-prev = Précédent
shortcut-name-go-to = Aller à
shortcut-name-outline-toggle = Afficher/masquer le plan
shortcut-name-preview-toggle = Afficher/masquer l’aperçu de recherche
shortcut-name-view-fullscreen = Basculer en plein écran
shortcut-name-view-focus-mode = Basculer en mode sans distraction
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
shortcut-name-help-check-updates = Rechercher les mises à jour
# Affiché uniquement à la demande, via la commande du menu Aide.
update-check-behind = La version { $version } est disponible.
update-check-current = Vous utilisez la version la plus récente.
update-check-failed = Impossible de vérifier s’il existe une version plus récente. { $error }
shortcut-name-command-palette = Palette de commandes
help-section-reference = Glossaire

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

## Margin lane: the strip beside the scrollbar that maps a document
margin-lane-name = Repères de marge
margin-lane-provider-comments = Commentaires
margin-lane-provider-story-bible = Entrée de bible narrative
margin-lane-provider-story-bible-hint = Dans la lecture « Dans le texte » d’une note, les endroits où l’entrée est nommée, et les scènes racontées de son point de vue. Surligne aussi ces noms dans la prose elle-même.
margin-lane-mark-point-of-view = Raconté d’ici
margin-lane-mark-named-here = Nommée ici
margin-lane-provider-comments-hint = Là où un commentaire est attaché au texte
margin-lane-provider-search = Occurrences trouvées
margin-lane-provider-search-hint = Toutes les occurrences de votre dernière recherche, où qu’elles soient dans le document
margin-lane-provider-boundaries = Début de chaque document
margin-lane-provider-boundaries-hint = Un filet en tête de chaque scène d’un flux, pour savoir où vous êtes
margin-lane-provider-spelling = Orthographe
margin-lane-provider-spelling-hint = Les mots signalés par le correcteur. Désactivé sauf demande : c’est l’avis d’une machine sur votre prose
margin-lane-spelling = { $word }, faute possible
margin-lane-search-hit = { $text }, occurrence { $index } sur { $total }
margin-lane-search-current = { $text }, occurrence { $index } sur { $total }, celle où vous êtes
margin-lane-boundary = Début de { $title }
margin-lane-boundary-untitled = un document sans titre
settings-page-margin-lane = Repères de marge
settings-desc-margin-lane = La bande le long de la barre de défilement, et ce qu’elle montre
settings-margin-lane-enabled = Afficher la marge de repères
settings-margin-lane-enabled-hint = Le même interrupteur que Affichage ▸ Repères de marge.
settings-margin-lane-enabled-more = Elle ne dit jamais que quelque chose ne va pas : elle montre où sont les choses, et vous laisse en juger.
settings-group-margin-lane-marks = Ce qu’elle signale
settings-margin-lane-no-providers = Rien ne marque encore la marge.
settings-group-margin-lane-texture = Texture des dialogues
settings-margin-lane-texture = Afficher la texture des dialogues
settings-margin-lane-texture-hint = Une barre par paragraphe : sa longueur, et la part qui est parlée.
settings-margin-lane-texture-more = Non mesurée pour les langues sans convention établie, où la barre n’indique que la longueur.
settings-group-margin-lane-surfaces = Où elle apparaît
settings-margin-lane-surface-editor = Éditeur de texte
settings-margin-lane-surface-stream = Flux
settings-margin-lane-surface-search-preview = Aperçu de recherche
menu-margin-lane = Repères de &marge

## Statuts : l'échelle d'avancement du projet.
## Le nom d'un échelon est une DONNÉE du projet : ces chaînes sont résolues une seule fois,
## à la création de l'échelle, puis stockées telles quelles. Elles ne sont pas retraduites
## ensuite, et l'auteur peut renommer n'importe quel échelon.
## Les huit derniers reprennent le vocabulaire français de Plume Creator lui-même, pour
## qu'un projet importé retrouve les mots qu'il avait.
status-preset-drafting = Rédaction
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
status-plume-draft-2 = 2e brouillon
status-plume-draft-3 = 3e brouillon
status-plume-edit-1 = 1re révision
status-plume-edit-2 = 2e révision
status-plume-edit-3 = 3e révision
status-plume-proofread = Relu
status-plume-finished = Fini
new-work-statuses = Étapes de travail
new-work-statuses-hint = Les étapes par lesquelles passe une scène. Vous pourrez les renommer, les réordonner ou en ajouter à tout moment.
status-none = Aucun statut
inspector-status = Statut
overview-col-status = Statut
overview-status-mixed = les parties en dessous divergent
help-concept-status = Statut
status-completion-title = Où en est le livre
status-completion-headline = { $done ->
    [one] { $done } scène terminée sur { $total }
   *[other] { $done } scènes terminées sur { $total }
}
status-completion-empty = Aucune scène pour l’instant. Cette vue lit le manuscrit, elle se remplira à mesure que vous écrirez.
status-completion-open = Où en est le livre…

## Réglages ▸ Projet ▸ Statuts : l'éditeur d'échelle.
## L'auteur possède le NOM d'un échelon et l'ORDRE de l'échelle ; l'application possède sa
## catégorie, et la catégorie possède le symbole et la couleur. Seuls les noms de catégorie
## ci-dessous sont traduits : le nom d'un échelon est le texte de l'auteur et reste tel quel.
settings-page-statuses = Statuts
settings-desc-statuses = Renommer, réordonner, ajouter et supprimer les étapes par lesquelles passe une scène
settings-statuses-add = Ajouter un statut
settings-statuses-add-placeholder = Nom du nouveau statut
settings-statuses-desc =
    Les étapes par lesquelles passe une scène, dans l’ordre, de la moins avancée en haut à
    la plus avancée en bas. Cet ordre définit ce que « moins avancé que » signifie partout
    ailleurs dans l’application : placez-les selon votre processus réel.
settings-statuses-apply-preset = Appliquer un préréglage…
settings-statuses-preset-applied = { $added ->
    [one] { $added } statut ajouté
   *[other] { $added } statuts ajoutés
}
settings-statuses-preset-refused = Ce projet a déjà une échelle. Les préréglages ne remplissent qu’une échelle vide. Supprimez d’abord les échelons dont vous ne voulez pas.
settings-statuses-duplicate = « { $name } » figure déjà sur cette échelle
settings-statuses-added = « { $name } » ajouté
settings-statuses-move-up = Monter (moins avancé)
settings-statuses-move-down = Descendre (plus avancé)
settings-statuses-details-placeholder = Ce que signifie cette étape (facultatif)
settings-statuses-delete = Supprimer « { $name } »
settings-statuses-delete-in-use = { $count ->
    [one] Supprimer « { $name } » : { $count } élément le porte et perdra son statut
   *[other] Supprimer « { $name } » : { $count } éléments le portent et perdront leur statut
}
settings-statuses-deleted = « { $name } » supprimé
settings-statuses-deleted-in-use = { $count ->
    [one] « { $name } » supprimé. { $count } élément n’a plus de statut.
   *[other] « { $name } » supprimé. { $count } éléments n’ont plus de statut.
}
settings-statuses-empty-title = Aucun processus défini
settings-statuses-empty-body = Un statut indique où en est une scène. Ajoutez-en un ci-dessus, ou partez d’un préréglage.
status-category-planned = Prévu
status-category-drafting = Rédaction
status-category-needs-work = À reprendre
status-category-revised = Révisé
status-category-final = Finalisé

## Long-operation progress (shared::progress_text). The backend reports phase
## codes it cannot translate; these are their labels. `progress-percent` is the
## toast body on its own, `progress-percent-with-label` with the phase after it.
progress-percent = { $percent } %
progress-percent-with-label = { $percent } % · { $label }
progress-import-opening = Ouverture du projet…
progress-import-outline = Lecture du plan…
progress-import-converting = Conversion des chapitres et des scènes…
progress-import-writing = Écriture du fichier .skrib…
progress-import-done = Terminé
progress-import-structure = Analyse de la structure…
progress-export-start = Démarrage…
progress-export-writing = Écriture…
progress-project-reading = Lecture du projet…
# Settings: the open project's section title, "Work: <its title>".
settings-sec-work-titled = Œuvre : { $title }

## Project-import notices (shared::import_warnings::notices): one line of the
## Plume / Manuskript warnings list each. $file, $folder, $title, $name, $field
## and $error are data (file names, titles, a parser's own message) and stay as
## they are. $container is "single-file" or "folder"; $kind is "scene", "note",
## "synopsis" or "title".
import-notice-flattened-rich-texts = { $count ->
    [1] Un texte était imbriqué plus profondément qu’un projet Skribisto ne le permet ; ses mots ont été conservés en texte brut, sans leur mise en forme.
   *[other] { $count } textes étaient imbriqués plus profondément qu’un projet Skribisto ne le permet ; leurs mots ont été conservés en texte brut, sans leur mise en forme.
}
import-notice-free-text-badges = { $count ->
    [1] Une pastille de scène ressemble à du texte libre plutôt qu’à une liste de statuts ; elle a donc été conservée comme libellé de l’élément au lieu de devenir une étiquette.
   *[other] { $count } pastilles de scène différentes ressemblent à du texte libre plutôt qu’à une liste de statuts ; elles ont donc été conservées comme libellés des éléments au lieu de devenir des étiquettes.
}
import-notice-separator-text-dropped = Le séparateur « { $name } » contenait du texte, qui n’a pas sa place dans un projet Skribisto ; ce texte n’a pas été importé.
import-notice-separator-without-scene = Le séparateur « { $name } » n’avait aucune scène avant lui dans son chapitre à laquelle se rattacher ; il n’a donc pas été importé.
import-notice-chapter-without-scenes = Le chapitre « { $name } » ne contient aucune scène ; ses séparateurs n’ont donc pas été importés.
import-notice-rich-text-unconverted = Du texte mis en forme n’a pas pu être converti ({ $error }) ; il n’a pas été importé.
import-notice-unresolved-cross-link = Un lien vers la fiche de personnage nº { $number } n’a pas pu être suivi : la fiche est absente ou était à la corbeille.
import-notice-read-copy = C’est la copie { $container ->
    [single-file] en fichier unique
   *[folder] en dossier
} de ce projet qui a été lue.
import-notice-read-copy-dated = C’est la copie { $container ->
    [single-file] en fichier unique
   *[folder] en dossier
} de ce projet qui a été lue, modifiée pour la dernière fois le { $day }.
import-notice-original-format = Ce projet est au format d’origine de Manuskript, celui de 2016. Il a été lu en entier ; rien n’y a été modifié.
import-notice-newer-single-file-beside = « { $file } » est plus récent que ce dossier ({ $file_day } contre { $folder_day }). Manuskript laisse l’ancien dossier en place quand un projet passe en mode fichier unique : ce dossier est donc peut-être une copie abandonnée. Importer plutôt le fichier .msk permettrait de lire la version la plus récente.
import-notice-pickle-skipped = « { $file } » n’a pas été lu. C’est un fichier pickle Python, et en charger un depuis un fichier de projet était une faille de sécurité de Manuskript lui-même (CVE-2021-35196) ; les réglages qu’il contient sont des préférences d’affichage, pas de l’écriture.
import-notice-file-unreadable = « { $file } » n’a pas pu être lu ({ $error }) et a été ignoré ; tout le reste a été importé.
import-notice-not-utf8 = « { $file } » n’est pas en UTF-8 valide ; les caractères illisibles ont été remplacés au lieu d’interrompre l’import.
import-notice-table-unreadable = « { $file } » n’a pas pu être lu ({ $error }) ; son contenu n’a pas été importé.
import-notice-outline-missing = « { $file } » est absent ; ce projet n’a donc aucun manuscrit à importer.
import-notice-outline-unreadable = « { $file } » n’a pas pu être lu ({ $error }) ; aucun manuscrit n’a été importé.
import-notice-named-none-row = « { $file } » commence par une ligne nommée (« { $name } ») là où Manuskript place son entrée vide « none ». Elle a été laissée de côté, pour que les lignes suivantes gardent les numéros qu’utilise le manuscrit.
import-notice-unknown-field = « { $file } » contient un champ que cet importateur ne connaît pas, « { $field } ». Il n’a pas été importé.
import-notice-colorless-labels = { $count ->
    [1] « { $file } » contient une ligne sans couleur. Elle a été importée sans couleur ; Manuskript, lui, cesse de charger le projet sur une telle ligne.
   *[other] « { $file } » contient { $count } lignes sans couleur. Elles ont été importées sans couleur ; Manuskript, lui, cesse de charger le projet sur une telle ligne.
}
import-notice-settings-unreadable = « { $file } » n’a pas pu être lu ({ $error }) ; la langue du projet n’a pas été importée.
import-notice-revisions-unreadable = « { $file } » n’a pas pu être lu ({ $error }) ; l’historique du projet n’a pas été importé, mais rien d’autre n’a été touché.
import-notice-foreign-files = { $count ->
    [1] Ce projet contient un fichier qui ne fait pas partie du format Manuskript et n’a pas été importé : { $files }.
   *[other] Ce projet contient { $count } fichiers qui ne font pas partie du format Manuskript et n’ont pas été importés : { $files }.
}
import-notice-foreign-files-more = Ce projet contient { $count } fichiers qui ne font pas partie du format Manuskript et n’ont pas été importés : { $files }, et { $more ->
    [1] un autre
   *[other] { $more } autres
}.
import-notice-not-a-character-file = « { $file } » n’est pas un fichier de personnage Manuskript et a été laissé de côté.
import-notice-not-a-text-item = « { $file } » n’est pas un élément de texte Manuskript et a été laissé hors du plan.
import-notice-missing-blank-line = « { $file } » n’a pas de ligne vide après son en-tête. Son dernier champ a été conservé, alors que Manuskript le perd.
import-notice-folder-without-metadata = « { $folder } » n’a pas de { $file } ; il a donc été importé comme simple dossier portant le nom du répertoire. Manuskript ignore un tel répertoire et tout ce qu’il contient.
import-notice-rows-given-ids = { $count ->
    [1] Un élément du plan n’avait pas d’identifiant propre et en a reçu un. Manuskript abandonne un tel élément, ou cesse de charger le projet si c’est un dossier.
   *[other] { $count } éléments du plan n’avaient pas d’identifiant propre et en ont reçu un. Manuskript abandonne un tel élément, ou cesse de charger le projet si c’est un dossier.
}
import-notice-plots-unreadable = « { $file } » n’a pas pu être lu ({ $error }) ; les intrigues n’ont pas été importées.
import-notice-world-unreadable = « { $file } » n’a pas pu être lu ({ $error }) ; les entrées de l’univers n’ont pas été importées.
import-notice-world-not-opml = « { $file } » n’est pas un document OPML ; les entrées de l’univers n’ont pas été importées.
import-notice-text-flattened = Le texte de « { $title } » est imbriqué plus profondément qu’un projet Skribisto ne le permet ; ses mots ont été conservés en texte brut, sans leur mise en forme.
import-notice-text-unconverted = Le texte de « { $title } » n’a pas pu être converti ({ $error }) ; il a été conservé exactement tel qu’il était écrit.
import-notice-text-without-file-name = { $kind ->
    [scene] Le texte de scène
    [note] Le texte de note
    [synopsis] Le synopsis
   *[title] Le titre
} de « { $title } » n’a pas pu recevoir de nom de fichier ; il n’a donc pas été importé.
import-notice-unknown-label = « { $title } » porte le libellé { $index }, absent de la liste des libellés de ce projet. Il a été importé sans libellé.
import-notice-unknown-status = « { $title } » porte le statut { $index }, absent de la liste des statuts de ce projet. Il a été importé sans statut.
import-notice-unknown-point-of-view = « { $title } » est raconté par un personnage qui ne figure pas parmi les personnages de ce projet. Son point de vue n’a pas été défini.
import-notice-unresolved-references = Les références du texte de « { $title } » n’ont pas pu être résolues ({ $error }) ; le texte a été conservé tel qu’il a été converti, marqueurs de référence compris.
import-notice-custom-icons = { $count ->
    [1] Un élément avait une icône personnalisée. Skribisto donne à chaque élément l’icône de son type ; l’icône n’a donc pas été importée, et rien d’autre n’a changé pour cet élément.
   *[other] { $count } éléments avaient une icône personnalisée. Skribisto donne à chaque élément l’icône de son type ; ces icônes n’ont donc pas été importées, et rien d’autre n’a changé pour ces éléments.
}
import-notice-character-colors = { $count ->
    [1] Un personnage avait une couleur à côté de son nom. Skribisto colore les étiquettes plutôt que les éléments ; la couleur n’a donc pas été importée, mais le personnage, lui, l’a été.
   *[other] { $count } personnages avaient une couleur à côté de leur nom. Skribisto colore les étiquettes plutôt que les éléments ; les couleurs n’ont donc pas été importées, mais les personnages, eux, l’ont tous été.
}
import-notice-versions-kept = { $found ->
    [1] Ce projet avait enregistré une version antérieure
   *[other] Ce projet avait enregistré { $found } versions antérieures
} ; { $kept ->
    [one] { $kept } a été conservée
   *[other] { $kept } ont été conservées
}, espacées comme Skribisto espace son propre historique. Voir le panneau Versions.
import-notice-versions-unplaced = { $count ->
    [1] Une version antérieure appartient à un élément qui ne figure plus dans le projet et n’a pas pu être placée.
   *[other] { $count } versions antérieures appartiennent à un élément qui ne figure plus dans le projet et n’ont pas pu être placées.
}

## Native file dialogs: the window title, and the label of the file-type filter.
dialog-open-work-title = Ouvrir une œuvre Skribisto
dialog-save-as-file-title = Enregistrer comme fichier .skrib unique
dialog-save-as-folder-title = Choisir le dossier parent de l’œuvre
dialog-manuskript-file-title = Choisir un projet Manuskript
dialog-manuskript-folder-title = Choisir le dossier d’un projet Manuskript
dialog-plume-file-title = Choisir un projet Plume Creator
dialog-filter-skrib = Œuvre Skribisto
dialog-filter-manuskript = Projet Manuskript
dialog-filter-plume = Projet Plume Creator
dialog-filter-hunspell-aff = Fichier d’affixes Hunspell (.aff)
dialog-filter-hunspell-dic = Dictionnaire Hunspell (.dic)
dialog-filter-export = Fichier exporté
# Help ▸ Keyboard shortcuts / Settings ▸ Keymap / command palette.
shortcut-name-view-margin-lane = Afficher/masquer les repères de marge

## The built-in distraction-free themes' names (distraction_free::theme).
df-theme-paper = Papier
df-theme-sepia = Sépia
df-theme-night = Nuit
df-theme-high-contrast = Contraste élevé
df-theme-midnight = Minuit

## Welcome ▸ Examples: the line under a bundled example's title.
welcome-example-byline = par { $author }

## Spoken by a screen reader only.
# Welcome window: the name of the section tab bar.
welcome-sections-a11y = Sections de l’accueil
# A comment's anchored span in the editor: author, plain-text preview, replies.
comments-a11y-summary-author = { $author } : { $text }
comments-a11y-summary-replies = { $text } ({ $count ->
    [one] { $count } réponse
   *[other] { $count } réponses
})
comments-a11y-summary-author-replies = { $author } : { $text } ({ $count ->
    [one] { $count } réponse
   *[other] { $count } réponses
})
# Margin marks beside the text: one mark, several merged into one, and the
# lane's own name when nothing more specific was given.
margin-lane-a11y-mark = { $label }, à { $percent } pour cent du texte
margin-lane-a11y-marks = { $label }, { $count } repères, à { $percent } pour cent du texte
margin-lane-a11y-fallback = { $count ->
    [one] Repères de marge, { $count } repère
   *[other] Repères de marge, { $count } repères
}
# Export ▸ Choose: the row standing for a binder that has no name.
export-choose-unnamed-binder = Classeur sans nom
# The built-in export styles named in words (the national manuscript presets
# keep their own language's name).
export-style-neutral = Neutre
export-style-manuscript-shunn = Manuscrit standard (Shunn)
export-style-ebook-clean = Livre numérique épuré

## Dates. Put the parts in the order your language writes a short date; the
## parts arrive already zero-padded where that matters.
# Comment cards, the outline card's Created/Modified.
date-time-short = { $day }/{ $month }/{ $year } { $hour }:{ $minute }
# Pace chart: a day on the axis, a milestone's date, a holiday.
pace-day-label = { $day }/{ $month }

## File sizes (shared::byte_size). $whole and $tenth are the two sides of the
## decimal mark: write your language's mark between them and its unit symbol.
byte-size-bytes = { $value } o
byte-size-kb = { $value } Ko
byte-size-mb = { $whole },{ $tenth } Mo
byte-size-gb = { $whole },{ $tenth } Go
# A whole percentage (sliders, the Pace card, the text-size toast).
unit-percent = { $value } %
# A whole number of pixels (slider readouts).
unit-pixels = { $value } px
# Settings ▸ Paratext: a preset saved without a `name` line. `name` is the TOML
# key the writer types and stays as it is.
settings-paratext-error-no-name = Une structure doit avoir une ligne `name`.
# Export: started with no project open (a menu raced a close).
export-error-no-project = Aucun projet n’est ouvert pour l’export.

shortcut-group-work = Œuvre
shortcut-group-edit = Édition
shortcut-group-view = Affichage
shortcut-group-document = Document
shortcut-group-format = Format
shortcut-group-go = Aller
shortcut-group-tools = Outils
shortcut-group-help = Aide

## Manuskript import: the headings of the notes it writes and the title of a project with no name.
import-manuskript-section-paragraph-summary = Résumé en un paragraphe
import-manuskript-section-full-summary = Résumé complet
import-manuskript-section-motivation = Motivation
import-manuskript-section-epiphany = Épiphanie
import-manuskript-section-notes = Notes
import-manuskript-section-description = Description
import-manuskript-section-passion = Passion
import-manuskript-section-result = Résultat
import-manuskript-section-summary = Résumé
import-manuskript-section-meta = Métadonnées
import-manuskript-section-series = Série
import-manuskript-section-volume = Tome
import-manuskript-section-genre = Genre
import-manuskript-section-license = Licence
import-manuskript-section-email = E-mail
import-manuskript-section-situation = Situation
import-manuskript-section-sentence = Une phrase
import-manuskript-section-paragraph = Un paragraphe
import-manuskript-section-page = Une page
import-manuskript-section-full = Résumé complet
import-manuskript-untitled-project = Projet Manuskript importé

## Plume import: the story-bible groups an old project implies, and the title of a project with no name.
import-plume-group-characters = Personnages
import-plume-group-items = Objets
import-plume-group-places = Lieux
import-plume-untitled-project = Projet Plume importé
