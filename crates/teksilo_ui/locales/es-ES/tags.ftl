# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

tags-preset-basic = Básica
tags-preset-scifi = Ciencia ficción
tags-preset-fantasy = Fantasía
tags-preset-mystery = Misterio
tags-preset-historical = Histórico


tags-preset-needs-research = falta documentarse
tags-preset-continuity-check = revisar continuidad
tags-preset-plot-point = punto de trama

tags-preset-character = personaje
tags-preset-place = lugar
tags-preset-item = objeto

tags-preset-vessel = nave
tags-preset-planet = planeta
tags-preset-organization = organización
tags-preset-creature = criatura
tags-preset-faction = facción
tags-preset-artifact = artefacto
tags-preset-realm = reino
tags-preset-magic-system = sistema mágico
tags-preset-suspect = sospechoso
tags-preset-victim = víctima
tags-preset-clue = pista
tags-preset-red-herring = pista falsa
tags-preset-historical-figure = figura histórica
tags-preset-source = fuente
tags-preset-period-detail = detalle de época

tags-pill-list = Etiquetas
tags-pill-add = Añadir una etiqueta
tags-none-yet = No hay etiquetas
tags-pick-list = Etiquetas disponibles
tags-pill-remove = Eliminar { $name }
tags-pill-filter-placeholder = Filtrar o nombrar una nueva etiqueta
tags-pill-no-match = Ninguna etiqueta coincide
tags-pill-create = Crear «{ $name }»
tags-pill-create-failed = No se pudo crear «{ $name }»
tags-pill-new-discoverable = Etiqueta de búsqueda en la prosa
tags-pill-new-discoverable-hint = Los elementos con esta etiqueta se buscan en su prosa para completar el reparto.

tags-alias-list = También conocido como
tags-alias-add = Añadir otro nombre
tags-alias-remove = Quitar { $name }
tags-alias-placeholder = Otro nombre y luego Intro
tags-alias-hint = Nombres con los que aparece en su prosa, además de su título.
tags-alias-collision = { $name } ya responde a este nombre.

settings-page-tags = Etiquetas
settings-tags-desc = Las etiquetas marcan los elementos de su archivador. Una etiqueta de búsqueda en la prosa también indica a Skribisto que busque los nombres de ese elemento en su prosa.
settings-tags-add = Añadir etiqueta
settings-tags-add-placeholder = Nombre de la nueva etiqueta
settings-tags-added = «{ $name }» añadida
settings-tags-duplicate = «{ $name }» ya existe
settings-tags-filter = Filtrar etiquetas
settings-tags-count = { $n ->
    [one] 1 etiqueta
   *[other] { $n } etiquetas
}
settings-tags-details-placeholder = Lo que significa esta etiqueta
settings-tags-discoverable = Buscar en la prosa
settings-tags-creates-in = Las notas nuevas van a
settings-tags-creates-in-unset = Preguntarme la primera vez
settings-tags-creates-in-untitled = Carpeta sin título
settings-tags-creates-in-trashed = { $name } (en la papelera)
settings-tags-template = Plantilla inicial
settings-tags-template-unset = Nota en blanco
settings-tags-delete = Eliminar { $name }
settings-tags-deleted = «{ $name }» eliminada y quitada de todos los elementos
settings-tags-empty = Todavía no hay etiquetas.
settings-tags-apply-preset = Aplicar un conjunto predefinido…
settings-tags-preset-applied = Añadidas: { $added }; omitidas por estar ya presentes: { $skipped }
settings-tags-csv-filter = Archivos CSV
settings-tags-import = Importar…
settings-tags-export = Exportar…
settings-tags-imported = Importadas: { $added }; omitidas: { $skipped }
settings-tags-exported = { $n ->
    [one] 1 etiqueta exportada
   *[other] { $n } etiquetas exportadas
}

tags-chip-more = { $n ->
    [one] 1 etiqueta más
   *[other] { $n } etiquetas más
}

cast-section = Reparto
cast-add = Añadir al reparto…
cast-add-filter-placeholder = Filtrar la biblia narrativa…
cast-add-empty = No hay entradas de la biblia narrativa que añadir
cast-pin = Añadir a { $name } al reparto
cast-unpin = Quitar a { $name } del reparto
cast-empty = Todavía no hay nadie fijado. Añada a alguien o conserve una sugerencia
cast-unresolved = Ya no está en la biblia narrativa

mentions-confirm = Añadir al reparto de { $name }
mentions-hit-count = { $n ->
    [one] una vez
   *[other] { $n } veces
}
mentions-point-of-view-badge = Punto de vista
mentions-point-of-view-badge-tooltip = Declarado aquí como punto de vista, definido a mano, no detectado en la prosa.

mentions-roster = Reparto
mentions-pin = Añadir a { $name } al reparto

pov-section = Punto de vista
pov-empty = Sin punto de vista definido
pov-add = Definir el punto de vista…
pov-multiple = Esta escena tiene dos puntos de vista.
pov-remove = Quitar a { $name } como punto de vista
pov-unresolved = Aquí se fijó un punto de vista, pero esa entrada ya no existe o ha perdido su etiqueta de la biblia narrativa.

books-section = Clasificado en
books-empty = Todavía no está clasificado en ningún libro
books-add = Clasificar en un libro…
books-remove = Quitar de { $name }
books-apply-to-children = Aplicar la clasificación a los hijos

note-details-name-placeholder = Nombre…
note-details-tags = Etiquetas
note-details-aliases = Otros nombres
note-details-books = Clasificado en
note-details-books-empty = Todavía no está clasificado en ningún libro
note-details-links = Enlaces
note-details-links-empty = Todavía no hay enlaces. Añada uno para apuntar a otra entrada de su biblia narrativa.
note-details-links-add = Enlazar con una nota…
note-details-pov = Punto de vista
note-details-pov-empty = Sin punto de vista definido
note-details-pov-unresolved = Aquí se fijó un punto de vista, pero esa entrada ya no existe o ha perdido su etiqueta de la biblia narrativa.
note-details-pov-multiple = Esta nota tiene dos puntos de vista.
note-details-backlinks = Aparece en el manuscrito
note-details-backlinks-empty = Todavía nada. En cuanto este nombre aparezca en su prosa, aparecerá aquí.
note-details-backlinks-outside = Fuera de los libros
note-details-backlinks-confirm-all = Confirmar todas las apariciones
note-details-backlinks-confirm-all-tooltip = Añadir esta entrada al reparto de todos los documentos mostrados aquí que aún no la han confirmado
note-details-untitled-document = Sin título

note-in-prose-mentions = { $n ->
    [one] 1 mención
   *[other] { $n } menciones
}
note-in-prose-mention-at = { $current } de { $total }
note-in-prose-mention-previous = Mención anterior
note-in-prose-mention-next = Siguiente mención
