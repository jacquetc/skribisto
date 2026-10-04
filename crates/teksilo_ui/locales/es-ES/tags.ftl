# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

tags-preset-basic = Básica
tags-preset-scifi = Ciencia ficción
tags-preset-fantasy = Fantasía
tags-preset-mystery = Misterio
tags-preset-historical = Histórico


tags-preset-needs-research = Necesidades de investigación
tags-preset-continuity-check = Control de continuidad
tags-preset-plot-point = trama punto

tags-preset-character = carácter
tags-preset-place = lugar
tags-preset-item = Tema

tags-preset-vessel = buque
tags-preset-planet = planetario
tags-preset-organization = organización
tags-preset-creature = criatura
tags-preset-faction = facción
tags-preset-artifact = artefacto
tags-preset-realm = reino
tags-preset-magic-system = sistema mágico
tags-preset-suspect = sospechoso
tags-preset-victim = víctima
tags-preset-clue = pista
tags-preset-red-herring = arenque rojo
tags-preset-historical-figure = figura histórica
tags-preset-source = fuente
tags-preset-period-detail = período de sesiones

tags-pill-list = Etiquetas
tags-pill-add = Añadir una etiqueta
tags-none-yet = No hay etiquetas
tags-pick-list = Etiquetas disponibles
tags-pill-remove = Eliminar { $name }
tags-pill-filter-placeholder = Filtrar o nombrar una nueva etiqueta
tags-pill-no-match = No hay coincidencias con la etiqueta
tags-pill-create = Crear "{ $name }"
tags-pill-create-failed = No podía crear "{ $name }"
tags-pill-new-discoverable = Buscar-en-prose tag
tags-pill-new-discoverable-hint = Los artículos con esta etiqueta son compatibles con su prose para llenar la lista.

tags-alias-list = Otros nombres
tags-alias-add = Añadir otro nombre
tags-alias-remove = Eliminar { $name }
tags-alias-placeholder = Otro nombre, luego entrar
tags-alias-hint = Nombres que esto aparece en su prosa, al lado de su título.
tags-alias-collision = { $name } ya responde a este nombre.

settings-page-tags = Etiquetas
settings-tags-desc = Etiquetas etiquetar los artículos en su carpeta. Una etiqueta find-in-prose también dice Skribisto para buscar los nombres de ese artículo en su prosa.
settings-tags-add = Añadir etiqueta
settings-tags-add-placeholder = Nombre una nueva etiqueta
settings-tags-added = Añadido "{ $name }"
settings-tags-duplicate = "{ $name }" ya existe
settings-tags-filter = Etiquetas de filtro
settings-tags-count = { $n ->
    [one] 1 etiqueta
   *[other] { $n } etiquetas
}
settings-tags-details-placeholder = Lo que significa esta etiqueta
settings-tags-discoverable = Buscar en el texto
settings-tags-creates-in = Nuevas notas van a
settings-tags-creates-in-unset = Pregúntame la primera vez
settings-tags-creates-in-untitled = Carpeta sin título
settings-tags-creates-in-trashed = { $name } (en la basura)
settings-tags-template = Plantilla de inicio
settings-tags-template-unset = Nota de Blank
settings-tags-delete = Suprímase { $name }
settings-tags-deleted = Eliminado "{ $name }" y removido de cada artículo
settings-tags-empty = Todavía no hay etiquetas.
settings-tags-apply-preset = Aplique un preset...
settings-tags-preset-applied = Añadido { $added }, saltado { $skipped } ya presente
settings-tags-csv-filter = CSV archivos
settings-tags-import = Importar...
settings-tags-export = Exportar...
settings-tags-imported = Importado { $added }, saltado { $skipped }
settings-tags-exported = Exportado { $n ->
    [one] 1 etiqueta
   *[other] { $n } etiquetas
}

tags-chip-more = { $n ->
    [one] 1 etiqueta más
   *[other] { $n } más etiquetas
}

cast-section = Personajes presentes
cast-add = Añadir al yeso...
cast-add-filter-placeholder = Filtro de la Biblia...
cast-add-empty = No hay entradas de historia-bible para añadir
cast-pin = Añadir { $name } para fundir
cast-unpin = Eliminar { $name } del yeso
cast-empty = No hay nadie inmovilizado todavía.
cast-unresolved = Ya no en la Biblia de la historia

mentions-confirm = Añadir al yeso de { $name }
mentions-hit-count = { $n ->
    [one] una vez
   *[other] { $n } veces
}
mentions-point-of-view-badge = Punto de vista
mentions-point-of-view-badge-tooltip = Declarado como el punto de vista aquí, fijado a mano, no detectado en la prosa.

mentions-roster = Personajes presentes
mentions-pin = Añadir { $name } para fundir

pov-section = Punto de vista
pov-empty = No hay punto de vista
pov-add = Es el punto de vista...
pov-multiple = Esta escena tiene dos puntos de vista.
pov-remove = Eliminar { $name } como punto de vista
pov-unresolved = Un punto de vista fue clavado aquí, pero esa entrada ya no existe o perdió su etiqueta de la historia-bible.

books-section = Asignado a
books-empty = No presentado en virtud de un libro todavía
books-add = Fichero bajo un libro...
books-remove = Eliminar de { $name }
books-apply-to-children = Aplicar la presentación a los elementos secundarios

note-details-name-placeholder = Nombre…
note-details-tags = Etiquetas
note-details-aliases = Otros nombres
note-details-books = Asignado a
note-details-books-empty = No presentado bajo un libro todavía
note-details-links = Enlaces
note-details-links-empty = Aún no hay enlaces. Agregue uno a punto en otra entrada en su Biblia de historia.
note-details-links-add = Enlace a una nota...
note-details-pov = Punto de vista
note-details-pov-empty = No hay punto de vista
note-details-pov-unresolved = Un punto de vista fue clavado aquí, pero esa entrada ya no existe o perdió su etiqueta de la historia-bible.
note-details-pov-multiple = Esta nota tiene dos puntos de vista.
note-details-backlinks = Aparece en el manuscrito
note-details-backlinks-empty = Una vez que este nombre aparezca en tu prosa, aparecerá aquí.
note-details-backlinks-outside = Fuera de los libros
note-details-backlinks-confirm-all = Confirme cada apariencia
note-details-backlinks-confirm-all-tooltip = Añada esta entrada al reparto de todos los documentos que se muestran aquí que aún no lo han confirmado
note-details-untitled-document = Sin título

note-in-prose-mentions = { $n ->
    [one] 1 mención
   *[other] { $n } menciones
}
note-in-prose-mention-at = { $current } de { $total }
note-in-prose-mention-previous = Mención anterior
note-in-prose-mention-next = Siguiente mención
