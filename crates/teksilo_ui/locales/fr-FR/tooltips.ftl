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
wm-chapter-more = Un chapitre court d’ici jusqu’au chapitre suivant, à la [partie](:wm-part) suivante, ou à la [fin du livre](:wm-end-of-book). Il existe sous deux formes : un dossier de chapitre qui contient des [scènes](:wm-scene), ou un chapitre à plat qui porte son propre texte. Choisissez la forme des nouveaux chapitres dans les Réglages, sous Structure.
wm-scene = Une scène de texte.
wm-scene-more = Une scène contient votre texte. Chaque élément d’écriture a deux faces : le texte principal et un [synopsis](:wm-synopsis) pour préparer. Les deux s’ouvrent ensemble dans l’éditeur.
wm-note = Une note.
wm-note-more = Une note conserve un texte et un [synopsis](:wm-synopsis), comme une [scène](:wm-scene), pour la documentation, les idées ou les rappels. N’importe quel élément peut faire partie du livre compilé, y compris une note placée dans un [chapitre](:wm-chapter).
wm-note-folder = Une note qui contient d’autres éléments.
wm-note-folder-more = Une [note](:wm-note) qui contient aussi des éléments en dessous, pour regrouper des notes liées. Elle garde son propre [synopsis](:wm-synopsis), et les éléments à l’intérieur gardent leur propre texte.
wm-folder = Un dossier d’organisation.
wm-folder-more = Un conteneur pour ranger les éléments du classeur à votre convenance. Il porte un [synopsis](:wm-synopsis) mais pas de texte propre. Les éléments à l’intérieur portent le texte.
wm-end-of-book = Marque la fin d’un livre.
wm-end-of-book-more = Chaque livre partage une seule liste continue ; sa fin n’est donc pas déterminée par l’imbrication. Ce repère indique que le [livre](:wm-book) s’arrête ici. Tout ce qui suit appartient au livre suivant.
wm-synopsis = Un résumé d’un élément d’écriture.
wm-synopsis-more = Un résumé attaché à tout élément d’écriture, en général un court paragraphe mais aussi long que vous voulez. Utilisez-le pour préparer et naviguer avant que le texte n’existe. Il se place à côté du texte principal dans l’éditeur.

scene-break-minor = Un saut de scène ordinaire — un changement de temps, de lieu ou de point de vue au sein d'un chapitre.
scene-break-minor-more =
    Marque une rupture *là où vous la placez*, y compris au milieu d'une [scène](:wm-scene) — découper le texte en deux éléments est un choix d'organisation, cela ne crée jamais de rupture en soi.

    Se saisit dans le texte sous la forme `* * *`. Son rendu dépend du style d'export, pas de ce que vous tapez : un manuscrit Shunn l'imprime en `#`, un livre de poche en astérisques, et l'édition française, allemande, espagnole, russe ou italienne le plus souvent par un simple blanc, sans aucun signe.
scene-break-major = Une rupture plus forte — une grande ellipse, ou un changement de point de vue décisif.
scene-break-major-more =
    La même idée qu'un saut de scène ordinaire, d'un cran au-dessus. À utiliser quand une rupture simple sous-estimerait le saut.

    Se saisit sous la forme `# # #`, et le style d'export l'imprime différemment du niveau ordinaire — la norme Shunn oppose précisément `#` à `# # #`. Là où une tradition ne connaît pas de marque plus forte, les deux niveaux s'impriment de la même façon.
wm-story-bible = Skribisto recherche cet élément dans votre texte.
wm-story-bible-more = Activez cette option pour les étiquettes qui nomment ce dont vous parlez : personnages, lieux, objets. Tout élément portant une telle étiquette est recherché dans votre texte par son titre et par les autres noms que vous lui donnez, si bien que chaque [scène](:wm-scene) énumère qui et quoi y apparaît sans que vous ayez à créer le moindre lien. Laissez-la désactivée pour les étiquettes qui décrivent un élément au lieu de le nommer, comme un état d’avancement ou un rappel de vérifier la continuité.

tooltip-go-to = Rejoindre n'importe quel élément du classeur (Ctrl+G)
synopsis-collapse-tooltip = Masquer la colonne du synopsis

wm-paratext = Un texte qui ne fait pas partie du récit.
wm-paratext-more = Une préface, une dédicace, une postface, un achevé d'imprimer — un écrit qui appartient au livre mais non à son corps. Exporté là où vous le placez, et jamais compté dans le nombre de mots du manuscrit. Sa place vous appartient : les usages varient selon les pays et les éditeurs.
wm-paratext-folder = Un dossier de paratextes.
wm-paratext-folder-more = De quoi ranger préfaces et postfaces sans encombrer le classeur. Purement organisationnel — il porte un [synopsis](:wm-synopsis) mais n'ajoute rien au livre exporté, pas même son nom.

# ── Objectifs de mots / de caractères ─────────────────────────────────────────
# Un second réseau d'infobulles à côté de celui du modèle d'écriture. Il tient lieu de la
# seule explication du comptage dans l'application, faute d'une aide ailleurs.
goal-target = La longueur visée pour cet élément.
goal-target-more = Vous pouvez fixer un objectif sur n'importe quoi : une [scène](:wm-scene), un [chapitre](:wm-chapter), une [partie](:wm-part), le [livre](:wm-book) entier. Chacun est son propre nombre et vaut pour lui-même : fixer l'objectif d'un chapitre ne change jamais celui de son livre, et l'objectif d'un livre n'est jamais la somme des chapitres qu'il contient. Ce qui s'additionne, c'est l'écriture : la [progression](:goal-progress) d'un conteneur, c'est tout ce qui est écrit en dessous. Un objectif laissé à zéro n'est simplement pas défini. Le nombre se compte en [mots ou en caractères](:goal-unit), selon ce qu'utilise ce projet.
goal-unit = Si ce projet compte en mots ou en caractères.
goal-unit-more = Un choix propre au projet, car une unité de longueur appartient au manuscrit et à son marché plutôt qu'à une section : l'édition allemande et française mesure en signes, la japonaise en feuillets de 400 caractères, la chinoise au millier de caractères, l'édition anglophone en mots. Les deux nombres sont conservés côte à côte et jamais convertis l'un dans l'autre : changer d'unité pointe chaque [objectif](:goal-target) vers l'autre chiffre, et revenir en arrière retrouve le premier. Ce que cela ne fait pas, c'est convertir : les objectifs déjà saisis seront à reprendre à la main.
goal-progress = Ce qui est écrit, rapporté à l'objectif.
goal-progress-explained = Un élément qui porte son propre texte se mesure à sa propre longueur ; un conteneur, à [tout ce qui est écrit en dessous](:goal-manuscript-words). La barre se réchauffe du rouge à l'ambre puis au vert à mesure que le texte arrive, et change encore une fois l'élément nettement au-delà du prévu, ce qui mérite d'être vu quand un chapitre a une longueur à tenir.
goal-manuscript-words = Ce qui compte comme manuscrit.
goal-manuscript-words-more = Uniquement le texte qui serait réellement exporté : les [scènes](:wm-scene) et les [chapitres](:wm-chapter) qui portent leur propre prose. Une [note](:wm-note) ou un [paratexte](:wm-paratext) ne fait pas partie du manuscrit et n'est jamais compté dedans. Ce qui est à la corbeille en sort, et ce qui est [exclu de l'export](:goal-exportable) aussi.
goal-exportable = Cette ligne est hors export.
goal-exportable-more = Sa longueur propre reste affichée, car le texte est toujours là, mais elle ne compte dans aucun total, exactement comme elle n'apparaîtra dans aucun livre exporté. Exclure un conteneur n'exclut pas ce qu'il contient : chaque ligne répond pour elle-même, et c'est à cela que sert le bouton « Appliquer aux enfants » à côté de l'interrupteur.
goal-distribute = Répartir cet objectif sur ce qu'il contient.
goal-distribute-more = Partage l'objectif d'un [livre](:wm-book) ou d'une [partie](:wm-part) entre ses enfants immédiats, de sorte que les morceaux y retombent exactement. Par défaut, seuls ceux qui n'ont pas encore d'objectif sont remplis : vos propres nombres ne sont pas touchés. Ce qui est écrit, ce sont des [objectifs](:goal-target) ordinaires, modifiables ensuite et libres de diverger : c'est une action, pas un lien permanent.
goal-subtree-total = Ce à quoi s'ajoutent les objectifs intérieurs.
goal-subtree-total-more = Un constat, pas un objectif. Ce projet ne traite jamais l'objectif d'un conteneur comme la somme de ceux qu'il contient : c'est ainsi que le chiffre d'un dossier finit par bouger tout seul dès qu'une scène à l'intérieur reçoit un nombre. Cette ligne vous dit simplement ce que donnent vos propres nombres, à comparer vous-même à l'[objectif](:goal-target) du conteneur, ou à [répartir](:goal-distribute).
goal-milestone = Une date à laquelle atteindre quelque chose.
goal-milestone-more = Deux sortes. L'une dit que le [livre](:wm-book) devrait faire telle longueur à telle date, un point de passage sur sa propre courbe. L'autre dit qu'un [chapitre](:wm-chapter) ou une [partie](:wm-part) devrait avoir atteint [son propre objectif](:goal-target) d'ici là, et lit ce nombre en direct plutôt que d'en garder une copie : modifier l'objectif met donc le jalon à jour. Les deux figurent sur le [plan de rythme](:pace-plan) du livre.
pace-plan = Le calendrier d'écriture du livre.
pace-plan-more = Un [objectif](:goal-target) et une échéance, avec les jours où vous écrivez, traduits en ce qu'il faut écrire par jour. Il lit l'objectif propre du livre : le fixer ici ou le fixer dans l'Inspecteur, c'est le même nombre à deux endroits. Les points de passage en chemin sont les [jalons](:goal-milestone).
