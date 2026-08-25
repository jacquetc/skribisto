# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Skribisto: infobulles enrichies du modèle d’écriture (français).
# Enregistrées dans tooltip_registry.rs et attachées par clé depuis les menus
# « ＋ Créer » / « Convertir en ». Les corps wm-*-more se renvoient l’un à
# l’autre via des liens [label](:key) ; ce fichier fait ainsi office d’aide en
# place.

wm-book = Un livre de votre projet.
wm-book-more = Un livre est aussi simple ou aussi structuré que vous le souhaitez. Une nouvelle peut se limiter à quelques [scènes](:wm-scene). Une œuvre plus longue peut imbriquer des [parties](:wm-part) et des [chapitres](:wm-chapter) aussi profondément que vous voulez. La forme vous appartient.
wm-part = Une partie regroupant des chapitres.
wm-part-more = Une partie rassemble des [chapitres](:wm-chapter) sous un même titre. Comme un chapitre, elle court jusqu’à la partie suivante ou la fin du [livre](:wm-book).
wm-chapter = Un chapitre du livre.
wm-chapter-more = Un chapitre court d’ici jusqu’au chapitre suivant, à la [partie](:wm-part) suivante, ou à la [fin du livre](:wm-end-of-book). Il existe sous deux formes : un dossier de chapitre qui contient des [scènes](:wm-scene), ou un chapitre à plat qui porte son propre texte. Choisissez la forme des nouveaux chapitres dans les Paramètres, sous Structure.
wm-scene = Une scène de texte.
wm-scene-more = Une scène contient votre texte. Chaque élément d’écriture a deux faces : le texte principal et un [synopsis](:wm-synopsis) pour préparer. Les deux s’ouvrent ensemble dans l’éditeur.
wm-note = Une note.
wm-note-more = Une note conserve un texte et un [synopsis](:wm-synopsis), comme une [scène](:wm-scene), pour la documentation, les idées ou les rappels. N’importe quel élément peut faire partie du livre compilé, y compris une note placée dans un [chapitre](:wm-chapter).
wm-note-folder = Une note qui contient d’autres éléments.
wm-note-folder-more = Une [note](:wm-note) qui contient aussi des éléments en dessous, pour regrouper des notes liées. Elle garde son propre [synopsis](:wm-synopsis), et les éléments à l’intérieur gardent leur propre texte.
wm-folder = Un dossier d’organisation.
wm-folder-more = Un conteneur pour ranger les éléments du classeur à votre convenance. Il porte un [synopsis](:wm-synopsis) mais pas de texte propre. Les éléments à l’intérieur portent le texte.
wm-end-of-book = Marque la fin d’un livre.
wm-end-of-book-more = Chaque livre partage une seule liste continue ; sa fin n’est donc pas déterminée par l’imbrication. Ce repère indique que le [livre](:wm-book) s’arrête ici. Tout ce qui suit appartient au livre suivant.
wm-story-bible-entry = Une note sur votre histoire, étiquetée et pourvue d’alias.
wm-story-bible-entry-more = Une [note](:wm-note) sur un personnage, un lieu ou tout autre élément à répertorier, créée avec un nom, des étiquettes, des alias et un modèle de départ en une seule étape. Rien sur la page ne la distingue ensuite : elle reste une note ordinaire, que vous pouvez renommer, réétiqueter ou convertir comme n’importe quelle autre.
wm-synopsis = Un résumé d’un élément d’écriture.
wm-synopsis-more = Un résumé attaché à tout élément d’écriture, en général un court paragraphe mais aussi long que vous voulez. Utilisez-le pour préparer et naviguer avant que le texte n’existe. Il se place à côté du texte principal dans l’éditeur.

scene-break-minor = Un saut de scène ordinaire : un changement de temps, de lieu ou de point de vue au sein d’un chapitre.
scene-break-minor-more =
    Marque une rupture *là où vous la placez*, y compris au milieu d’une [scène](:wm-scene). Découper le texte en deux éléments est un choix d’organisation, cela ne crée jamais de rupture en soi.

    Se saisit dans le texte sous la forme de trois astérisques séparés par des espaces. Son rendu dépend du style d’export, pas de ce que vous tapez : un manuscrit Shunn l’imprime avec un seul #, un livre de poche en astérisques, et l’édition française, allemande, espagnole, russe ou italienne le plus souvent par un simple blanc, sans aucun signe.
scene-break-major = Une rupture plus forte : une grande ellipse, ou un changement de point de vue décisif.
scene-break-major-more =
    La même idée qu’un saut de scène ordinaire, d’un cran au-dessus. À utiliser quand une rupture simple sous-estimerait le saut.

    Se saisit sous la forme # # #, et le style d’export l’imprime différemment du niveau ordinaire. La norme Shunn oppose précisément un seul # à # # #. Là où une tradition ne connaît pas de marque plus forte, les deux niveaux s’impriment de la même façon.
wm-find-in-prose = Skribisto recherche cet élément dans votre texte.
wm-find-in-prose-more = Activez cette option pour les étiquettes qui nomment ce dont vous parlez : personnages, lieux, objets. Tout élément portant une telle étiquette est recherché dans votre texte par son titre et par les autres noms que vous lui donnez, si bien que chaque [scène](:wm-scene) énumère qui et quoi y apparaît sans que vous ayez à créer le moindre lien. Laissez-la désactivée pour les étiquettes qui décrivent un élément au lieu de le nommer, comme un état d’avancement ou un rappel de vérifier la continuité. Chaque élément ainsi repéré rejoint aussi votre [bible narrative](:wm-story-bible).
wm-story-bible = Chaque personnage, lieu et autre entrée étiquetée, réunis dans une même grille.
wm-story-bible-more = S’ouvre depuis l’onglet propre à chaque dossier de notes. Les cartes sont groupées par étiquette, et chacune indique le nombre d’alias de l’entrée et le nombre de scènes où elle est apparue, à partir de chaque étiquette [repérable dans le texte](:wm-find-in-prose) du projet. Filtrer par livre ne fait que réduire les cartes affichées ; cela ne décide jamais quelles entrées existent.

tooltip-go-to = Rejoindre n’importe quel élément du classeur (Ctrl+G)
synopsis-collapse-tooltip = Masquer la colonne du synopsis

wm-paratext = Un texte qui ne fait pas partie du récit.
wm-paratext-more = Une préface, une dédicace, une postface, un achevé d’imprimer : un écrit qui appartient au livre mais non à son corps. Exporté là où vous le placez, et jamais compté dans le nombre de mots du manuscrit. Sa place vous appartient : les usages varient selon les pays et les éditeurs.
wm-paratext-folder = Un dossier de paratextes.
wm-paratext-folder-more = De quoi ranger préfaces et postfaces sans encombrer le classeur. Purement organisationnel : il porte un [synopsis](:wm-synopsis) mais n’ajoute rien au livre exporté, pas même son nom.

# ── Objectifs de mots / de caractères ─────────────────────────────────────────
# Un second réseau d'infobulles à côté de celui du modèle d'écriture. Il tient lieu de la
# seule explication du comptage dans l'application, faute d'une aide ailleurs.
goal-target = La longueur visée pour cet élément.
goal-target-more = Vous pouvez fixer un objectif sur n’importe quoi : une [scène](:wm-scene), un [chapitre](:wm-chapter), une [partie](:wm-part), le [livre](:wm-book) entier. Chacun est son propre nombre et vaut pour lui-même : fixer l’objectif d’un chapitre ne change jamais celui de son livre, et l’objectif d’un livre n’est jamais la somme des chapitres qu’il contient. Ce qui s’additionne, c’est l’écriture : la [progression](:goal-progress) d’un conteneur, c’est tout ce qui est écrit en dessous. Un objectif laissé à zéro n’est simplement pas défini. Le nombre se compte en [mots ou en caractères](:goal-unit), selon ce qu’utilise ce projet.
goal-unit = Si ce projet compte en mots ou en caractères.
goal-unit-more = Un choix propre au projet, car une unité de longueur appartient au manuscrit et à son marché plutôt qu’à une section : l’édition allemande et française mesure en signes, la japonaise en feuillets de 400 caractères, la chinoise au millier de caractères, l’édition anglophone en mots. Les deux nombres sont conservés côte à côte et jamais convertis l’un dans l’autre : changer d’unité pointe chaque [objectif](:goal-target) vers l’autre chiffre, et revenir en arrière retrouve le premier. Ce que cela ne fait pas, c’est convertir : les objectifs déjà saisis seront à reprendre à la main.
goal-progress = Ce qui est écrit, rapporté à l’objectif.
goal-progress-explained = Un élément qui porte son propre texte se mesure à sa propre longueur ; un conteneur, à [tout ce qui est écrit en dessous](:goal-manuscript-words). La barre se réchauffe du rouge à l’ambre puis au vert à mesure que le texte arrive, et change de nouveau lorsque l’élément dépasse nettement le prévu, ce qui mérite d’être vu quand un chapitre a une longueur à tenir.
goal-manuscript-words = Ce qui compte comme manuscrit.
goal-manuscript-words-more = Uniquement le texte qui serait réellement exporté : les [scènes](:wm-scene) et les [chapitres](:wm-chapter) qui portent leur propre prose. Une [note](:wm-note) ou un [paratexte](:wm-paratext) ne fait pas partie du manuscrit et n’est jamais compté dedans. Ce qui est à la corbeille en sort, et ce qui est [exclu de l’export](:goal-exportable) aussi.
goal-exportable = Cette ligne est hors export.
goal-exportable-more = Sa longueur propre reste affichée, car le texte est toujours là, mais elle ne compte dans aucun total, exactement comme elle n’apparaîtra dans aucun livre exporté. Exclure un conteneur n’exclut pas ce qu’il contient : chaque ligne répond pour elle-même, et c’est à cela que sert le bouton « Appliquer aux enfants » à côté de l’interrupteur.
goal-distribute = Répartir cet objectif sur ce qu’il contient.
goal-distribute-more = Partage l’objectif d’un [livre](:wm-book) ou d’une [partie](:wm-part) entre ses enfants immédiats, de sorte que les morceaux y retombent exactement. Par défaut, seuls ceux qui n’ont pas encore d’objectif sont remplis : vos propres nombres ne sont pas touchés. Ce qui est écrit, ce sont des [objectifs](:goal-target) ordinaires, modifiables ensuite et libres de diverger : c’est une action, pas un lien permanent.
goal-subtree-total = Ce à quoi s’ajoutent les objectifs intérieurs.
goal-subtree-total-more = Un constat, pas un objectif. Ce projet ne traite jamais l’objectif d’un conteneur comme la somme de ceux qu’il contient : c’est ainsi que le chiffre d’un dossier finit par bouger tout seul dès qu’une scène à l’intérieur reçoit un nombre. Cette ligne vous dit simplement ce que donnent vos propres nombres, à comparer vous-même à l’[objectif](:goal-target) du conteneur, ou à [répartir](:goal-distribute).
goal-milestone = Une date à laquelle atteindre quelque chose.
goal-milestone-more = Deux sortes. L’une dit que le [livre](:wm-book) devrait faire telle longueur à telle date, un point de passage sur sa propre courbe. L’autre dit qu’un [chapitre](:wm-chapter) ou une [partie](:wm-part) devrait avoir atteint [son propre objectif](:goal-target) d’ici là, et lit ce nombre en direct plutôt que d’en garder une copie : modifier l’objectif met donc le jalon à jour. Les deux figurent sur le [plan de rythme](:pace-plan) du livre.
pace-plan = Le calendrier d’écriture du livre.
pace-plan-more = Un [objectif](:goal-target) et une échéance, avec les jours où vous écrivez, traduits en ce qu’il faut écrire par jour. Il lit l’objectif propre du livre : le fixer ici ou le fixer dans l’Inspecteur, c’est le même nombre à deux endroits. Les points de passage en chemin sont les [jalons](:goal-milestone).

# ── Concepts de fonctionnalités ─────────────────────────────────────────────────
# Un troisième réseau d'infobulles, à côté de celui du modèle d'écriture et de celui
# des objectifs de mots/caractères ci-dessus. Chaque entrée explique ici une
# fonctionnalité du projet qui n'a sa page nulle part ailleurs : étiquettes, note de
# ligne, point de vue, épigraphes, notes de bas de page, mode des chapitres,
# commentaires, copies de secours, versions, corbeille, orthographe, recherche et
# remplacement, modèles de note, remplacement de texte, ponctuation intelligente,
# styles d'export et marqueurs d'aller-retour. Enregistrées comme les deux réseaux
# ci-dessus, et reliées à eux chaque fois qu'un concept en cite réellement un.
concept-tag = Une étiquette colorée que vous posez sur n’importe quel élément, réutilisable dans tout le projet.
concept-tag-more = Étiquetez un élément pour le repérer : un statut, un lieu, un fil que vous suivez. La même étiquette peut se poser sur n’importe quel nombre d’éléments, et un élément peut en porter plusieurs. Activez [repérer dans le texte](:wm-find-in-prose) pour une étiquette qui nomme quelque chose dans votre livre, un personnage ou un lieu : Skribisto se met alors à le chercher dans votre texte.
concept-label = Une courte note que vous écrivez sous le titre d’un élément, pour vous seul.
concept-label-more = Ce n’est pas une [étiquette](:concept-tag) : elle n’appartient qu’à cet élément, un texte libre sans couleur ni catalogue derrière lui, quelque chose comme « 1er rebondissement » ou « à nommer ». Définissez-la depuis le menu contextuel de la ligne, ou modifiez-la directement dans la colonne Libellé de la vue d’ensemble ; elle apparaît en petit sous le titre, dans l’arborescence comme dans le flux.
concept-point-of-view = À travers les yeux de qui une scène est racontée : un ou plusieurs membres de la présence.
concept-point-of-view-more = Se règle dans l’Inspecteur, à côté de la présence. Choisir quelqu’un qui n’y figure pas encore l’y ajoute du même geste. Porter plusieurs points de vue est un choix légitime, utile pour une scène partagée ou un changement de regard en cours de livre, que Skribisto ne bloque pas. Les candidats sont les éléments qu’une étiquette [repérer dans le texte](:wm-find-in-prose) place dans votre bible narrative.
concept-epigraph = Une citation placée en tête d’une partie ou d’un chapitre.
concept-epigraph-more = Un champ à part, distinct du texte propre du [chapitre](:wm-chapter) ou de la [partie](:wm-part) ; une [scène](:wm-scene) ou une [note](:wm-note) n’en porte aucune. Écrivez-en une comme une citation ordinaire ; écrivez-en plusieurs et chacune devient sa propre citation en bloc, imprimée comme sa propre épigraphe. Ses mots ne sont jamais comptés dans le manuscrit.
concept-footnote = Un appel de note marqué dans le texte, qui s’imprime comme une note numérotée.
concept-footnote-more = L’appel n’est qu’un caractère au milieu de vos mots : il se déplace et s’efface exactement comme eux. Son numéro n’est jamais enregistré : Skribisto le recalcule à chaque fois, d’après la place de chaque appel dans le manuscrit entier, si bien qu’en insérer un renumérote tous les suivants. Les mots d’une note se comptent à part du manuscrit, jamais dedans.
concept-chapter-mode = Si un nouveau chapitre est un dossier de scènes ou une seule ligne à plat.
concept-chapter-mode-more = Se règle dans les Paramètres, sous Structure. Vous écrivez dans le [chapitre](:wm-chapter) dans les deux cas ; la seule différence est sa capacité à *contenir* des scènes. Les deux formes produisent le même livre, vous pouvez les mélanger librement, et Promouvoir convertit un chapitre de l’une à l’autre sans perdre un mot.
concept-comment = Une remarque ancrée sur un passage précis du texte, affichée dans la marge.
concept-comment-more = Elle retient les mots visés et ce qui les entoure, non une position, ce qui lui permet de survivre à une modification ordinaire et à un rechargement complet. Répondez en dessous pour poursuivre le fil. Si la citation exacte ne se retrouve plus, le commentaire le signale au lieu de glisser en silence vers la mauvaise phrase.
concept-backup = Une copie de secours de tout le projet, à un instant donné, conservée à côté de lui.
concept-backup-more = Prise selon le calendrier de votre choix : à l’ouverture, à la fermeture, à intervalles réguliers, ou toute combinaison, les anciennes copies étant élaguées par âge ou par nombre. En ouvrir une l’affiche dans sa propre fenêtre : vous pouvez la consulter et même la modifier, mais seuls « Enregistrer sous » ou « Restaurer » gardent quoi que ce soit. Différente d’une [version](:concept-version), qui suit le passé d’une seule ligne.
concept-version = Le passé d’une seule ligne : chaque changement de son texte ou de son synopsis.
concept-version-more = Construite à la fois depuis l’historique propre du projet et ses copies de secours, une entrée par changement plutôt qu’une par fichier. Choisissez une entrée pour voir ce qui a changé et ne restaurer que le texte de cette ligne ; le reste du projet reste intact, et une copie de sécurité est prise d’abord. Différente d’une [copie de secours](:concept-backup), qui couvre tout le projet.
concept-trash = Un élément mis à la corbeille est caché, non déplacé : il reste exactement où il était.
concept-trash-more = Mettre à la corbeille se limite à basculer un indicateur : la ligne garde sa place dans le classeur, seulement désactivée. La restaurer la fait réapparaître à cet endroit précis, sans aucun rangement à refaire. Vider la corbeille supprime les éléments pour de bon, et chacune de ces actions est un geste annulable, comme toute autre modification du projet.
concept-spellcheck = Souligne les mots qu’aucun dictionnaire installé ne reconnaît, selon la langue du projet.
concept-spellcheck-more = Chaque projet a sa ou ses langues de travail ; un élément peut les remplacer pour une scène écrite dans une autre langue. Ajoutez un mot que Skribisto ne connaît pas depuis le menu contextuel de l’éditeur, dans un dictionnaire personnel qui vaut ensuite pour tous vos projets. Désactiver la vérification l’arrête partout, jusqu’à ce que vous la réactiviez.
concept-search-replace = Trouve un mot ou une expression dans tout le projet, pas seulement le document ouvert.
concept-search-replace-more = Parcourt chaque scène, note, titre, synopsis et étiquette, ainsi que les commentaires et les notes de bas de page. Respecter la casse, le mot entier et les accents sont trois interrupteurs distincts. Tout remplacer réécrit chaque résultat vérifié en une seule fois et un seul geste d’annulation : Ctrl+Z reprend tout le lot, jamais seulement le dernier changement.
concept-note-template = Un morceau de texte réutilisable que vous insérez dans ce que vous écrivez.
concept-note-template-more = Une fiche de personnage vierge, un profil de lieu, une trame de scène : écrivez-le une fois et déposez-le partout où vous en avez besoin, dans n’importe quel élément, pas seulement une note. Partez d’un modèle fourni, importez un fichier .md ou .djot, ou écrivez quelque chose et choisissez Document ▸ Enregistrer comme modèle. Enregistrés dans le projet, si bien qu’un co-auteur qui l’ouvre dispose du même ensemble.
concept-text-replacement = Vos propres raccourcis, développés automatiquement pendant que vous écrivez.
concept-text-replacement-more = Définissez un déclencheur et son remplacement, « stp » pour « s’il te plaît », et il se déploie dès que vous tapez une espace ou une ponctuation après lui. La casse suit ce que vous avez tapé : mettez une majuscule au déclencheur, et le remplacement en prend une aussi. Un Retour arrière saisi aussitôt après un remplacement annule seulement celui-ci. Activé par projet, distinct de la [ponctuation intelligente](:concept-smart-punctuation).
concept-smart-punctuation = Typographie locale appliquée automatiquement pendant que vous écrivez : guillemets, tirets, points de suspension.
concept-smart-punctuation-more = Courbe les guillemets droits en guillemets typographiques, transforme -- en tiret demi-cadratin et --- en cadratin, transforme ... en points de suspension, et, si vous l’activez, ouvre un paragraphe saisi « - » par un tiret de dialogue. Se règle par projet, et voyage dans le .skrib pour qu’un co-auteur écrive avec les mêmes règles. Distincte de [vos propres règles](:concept-text-replacement), que vous écrivez vous-même.
concept-export-style = Un ensemble nommé de choix d’export : titres, espacement, ce qui est inclus.
concept-export-style-more = Un seul style, réutilisé par tous les exports quel que soit le format choisi. Les styles fournis sont en lecture seule ; dupliquez-en un pour obtenir un point de départ que vous pouvez réellement modifier. Parmi ses paramètres, celui d’inclure les [marqueurs d’aller-retour](:concept-round-trip-marks), pour un fichier envoyé à un éditeur puis renvoyé.
concept-round-trip-marks = Des signets invisibles qui permettent à un DOCX ou un ODT modifié de revenir reconnu.
concept-round-trip-marks-more = Écrits seulement dans les deux formats qu’un éditeur peut renvoyer, DOCX et ODT ; aucun autre export n’en porte. Un signet, non un attribut personnalisé : LibreOffice supprime les attributs personnalisés à l’enregistrement mais laisse un signet intact. S’active par [style d’export](:concept-export-style), sous « Envoi à un éditeur ».
