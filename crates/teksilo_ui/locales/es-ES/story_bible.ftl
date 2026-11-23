# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

story-bible-modal-configure-title = Configurar la nueva entrada
story-bible-modal-name-label = Nombre
story-bible-modal-name-placeholder = Un personaje, un lugar, cualquier cosa que merezca un registro…
story-bible-modal-no-binders = Este proyecto todavía no tiene archivadores en los que guardarlo
story-bible-modal-template-label = Empezar con una plantilla (opcional)
story-bible-modal-template-placeholder = Sin plantilla
story-bible-modal-body-label = Cuerpo
story-bible-modal-cancel = Cancelar
story-bible-modal-create = Crear
story-bible-modal-create-and-open = Crear y abrir
story-bible-modal-failed = No se pudo crear la entrada

ctx-add-as-note = Añadir como &nota…
ctx-add-as-note-all-tags = Todas las etiquetas…
ctx-add-as-note-untagged = Sin etiqueta

story-bible-grid-label = Biblia narrativa
story-bible-grid-untagged = Todavía sin etiquetar
story-bible-grid-empty-title = Todavía no hay nada registrado aquí
story-bible-grid-empty-hint = Etiquete una nota para buscarla en la prosa, o añada una a partir de una selección, y aparecerá aquí.
story-bible-grid-alias-count = { $count ->
    [0] Todavía sin otros nombres
    [one] { $count } otro nombre
   *[other] { $count } otros nombres
}
story-bible-grid-mention-count = { $count ->
    [0] Todavía sin apariciones
    [one] Aparece en { $count } escena de todo el proyecto
   *[other] Aparece en { $count } escenas de todo el proyecto
}
story-bible-books-filter-all = Todos los libros
story-bible-grid-not-searched = Todavía no se busca: necesita una etiqueta de la biblia narrativa

note-in-prose-pov = Punto de vista
note-in-prose-cast = Reparto
note-in-prose-pov-and-cast = Punto de vista · Reparto
note-in-prose-no-books = Este proyecto aún no tiene ningún libro
note-in-prose-empty-book = Ninguna escena de este libro tiene todavía esta entrada en su reparto ni en su punto de vista.
note-in-prose-empty-viewpoint = Ninguna escena de este libro está narrada todavía desde el punto de vista de esta entrada.
note-in-prose-scope = La página En la prosa muestra solo las escenas que usted ha confirmado. Una escena que nombra esta entrada sigue siendo una sugerencia hasta que añada la entrada a su reparto, en Detalles o en el Inspector.
note-in-prose-viewpoint-only = Solo escenas de su punto de vista
note-in-prose-viewpoint-only-tooltip = Mostrar solo las escenas narradas desde el punto de vista de esta entrada y ocultar aquellas en las que simplemente aparece

story-bible-capture-toast = «{ $name }» añadido a la biblia narrativa
story-bible-capture-open = Abrir
story-bible-capture-undo = Deshacer
story-bible-capture-where-title = ¿Dónde van estas notas?
story-bible-capture-where-prompt = Elija la carpeta en la que se deben guardar las notas nuevas.
story-bible-capture-where-confirm = Guardar aquí
story-bible-capture-where-needs-folder = Elija una carpeta en la que guardar estas notas.
story-bible-capture-where-tag = Solo se pregunta una vez. Las notas con la etiqueta «{ $tag }» irán aquí a partir de ahora, y puede cambiarlo más adelante en Configuración, Proyecto, Etiquetas.
story-bible-capture-where-untagged = Solo se pregunta una vez. Las notas sin etiqueta irán aquí a partir de ahora, y puede cambiarlo más adelante en Configuración, Proyecto, Etiquetas.

# What a screen reader hears for a binder row that is a story bible entry: its icon
# says so on screen, and this says it aloud. The second form keeps the row's label.
binder-row-story-bible-entry = Entrada de la biblia narrativa
binder-row-story-bible-entry-with-label = Entrada de la biblia narrativa, { $label }
