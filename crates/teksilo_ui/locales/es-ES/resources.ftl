# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Recursos: archivos y enlaces web adjuntos a los elementos del archivador.

## El panel

resources-title = Recursos
shortcut-name-resources-show = Mostrar los recursos
resources-add-files = Añadir archivos
resources-add-files-title = Añadir archivos a este elemento
resources-add-link = Añadir un enlace web
resources-drop-hint = Suelte archivos o direcciones web para adjuntarlos
resources-no-item = Abra un elemento para ver sus recursos.
resources-empty = Todavía no hay nada adjunto a este elemento. Añada archivos o enlaces web con los botones de arriba, o suéltelos aquí.
resources-actions = Acciones
resources-not-found = No encontrado
resources-row-linked = Archivo vinculado
resources-row-linked-size = Archivo vinculado, { $size }
resources-row-name = { $title }, { $kind }, { $detail }
resources-row-name-missing = { $title }, { $kind }, { $detail }, no encontrado

## Los tipos de archivo, tal como los anuncia un lector de pantalla

resources-kind-picture = imagen
resources-kind-pdf = documento PDF
resources-kind-audio = grabación de sonido
resources-kind-video = vídeo
resources-kind-document = documento
resources-kind-ebook = libro electrónico
resources-kind-archive = archivo comprimido
resources-kind-file = archivo
resources-kind-web-link = enlace web

## El menú de una fila

resources-open = Abrir
resources-show-in-folder = Mostrar en la carpeta
resources-locate = Localizar…
resources-insert-into-text = Insertar en el texto
resources-insert-picture = Insertar la imagen en el texto
resources-insert-link = Insertar un enlace en el texto
resources-edit = Cambiar nombre y describir…
resources-detach = Separar de este elemento
resources-remove = Quitar del proyecto…

## Los dos cuadros de diálogo

resources-link-dialog-title = Añadir un enlace web
resources-link-dialog-address = Dirección
resources-link-dialog-address-placeholder = https://
resources-link-dialog-name = Título
resources-link-dialog-name-placeholder = El nombre del sitio, si lo deja vacío
resources-link-dialog-add = Añadir
resources-edit-dialog-title = Cambiar nombre y describir
resources-edit-dialog-name = Título
resources-edit-dialog-description = Descripción
resources-edit-dialog-description-placeholder = Qué es y por qué lo guardó
resources-edit-dialog-save = Guardar
resources-dialog-cancel = Cancelar

## Añadir

resources-large-title = Un archivo grande
resources-large-text = «{ $name }» ocupa { $size }. Guardado en el proyecto, viaja con cada copia y cada copia de seguridad del proyecto. Vinculado, se queda donde está, y el proyecto solo recuerda dónde encontrarlo.
resources-large-text-many = «{ $name }» ocupa { $size }, y { $others ->
    [one] otro archivo también es grande
   *[other] otros { $others } archivos también son grandes
}. Guardados en el proyecto, viajan con cada copia y cada copia de seguridad del proyecto. Vinculados, se quedan donde están, y el proyecto solo recuerda dónde encontrarlos.
resources-large-store = Guardarlo en el proyecto
resources-large-link = Vincularlo en su lugar
resources-large-remember = No volver a preguntar
resources-storing-title = Añadiendo archivos al proyecto
resources-store-cancel = Cancelar
resources-store-cancelled = No se ha añadido ningún archivo.
resources-store-busy = Todavía se están añadiendo archivos. Vuelva a intentarlo cuando hayan terminado.
resources-store-failed = No se han podido añadir los archivos: { $error }
resources-no-project = Abra un proyecto antes de adjuntar archivos.
resources-moved-away = El proyecto se movió mientras se copiaba el archivo.
resources-some-failed = { $count ->
    [one] No se ha podido añadir un archivo: { $names }
   *[other] No se han podido añadir { $count } archivos: { $names }
}
resources-add-failed = No se han podido añadir los recursos: { $error }
resources-added = { $count ->
    [one] Un recurso añadido
   *[other] { $count } recursos añadidos
}
resources-dropped-added = { $count ->
    [one] Añadido como recurso de este elemento, y enlazado en el texto
   *[other] { $count } archivos añadidos como recursos de este elemento, y enlazados en el texto
}
resources-undo = Deshacer
resources-link-refused = «{ $url }» no es una dirección web que Skribisto pueda guardar. Use una dirección que empiece por http:// o https://, sin nombre de usuario ni contraseña.

## El aviso para un proyecto en un solo archivo

resources-folder-title = Este proyecto se está haciendo grande
resources-folder-text = Sus archivos guardados ocupan ahora { $size }. Un proyecto guardado como un solo archivo se reescribe entero cada vez que se guarda, sus archivos también. Guardado como carpeta, solo guarda lo que ha cambiado. Puede guardarlo ahora como carpeta, y seguir trabajando en la carpeta a partir de entonces.
resources-folder-switch = Guardar como carpeta…
resources-folder-not-now = Ahora no
resources-folder-never = No volver a preguntar para este proyecto

## Abrir

resources-not-found-toast = «{ $name }» no está donde el proyecto lo espera.
resources-unsafe-title = ¿Abrir «{ $name }»?
resources-unsafe-text = Skribisto no sabe si este tipo de archivo es seguro. Abrirlo podría iniciar un programa en su ordenador, y un proyecto puede venir de otra persona. Muéstrelo primero en su carpeta para ver qué es.
resources-open-anyway = Abrir de todos modos
resources-open-failed = No se ha podido abrir el archivo: { $error }
resources-no-data-folder = este sistema no da a Skribisto ninguna carpeta de datos donde abrir una copia
resources-link-gone = Este enlace lleva a un recurso que el proyecto ya no tiene.
resources-locate-title = ¿Dónde está ahora «{ $name }»?
resources-relinked-more = { $count ->
    [one] Se ha encontrado otro archivo que faltaba en la misma carpeta, y se ha vuelto a vincular.
   *[other] Se han encontrado otros { $count } archivos que faltaban en la misma carpeta, y se han vuelto a vincular.
}
resources-changed-elsewhere = «{ $name }» se modificó en otra aplicación. El proyecto conserva la versión nueva.

## Modificar

resources-edit-failed = No se ha podido modificar el recurso: { $error }
resources-remove-title = ¿Quitar «{ $title }» del proyecto?
resources-remove-text = Se quita de cada elemento al que está adjunto. Un archivo guardado en el proyecto se va con él; un archivo vinculado y una página web se quedan donde están. Deshacer lo trae de vuelta.
resources-remove-confirm = Quitar
resources-remove-failed = No se ha podido quitar el recurso: { $error }
resources-removed-toast = Se ha quitado «{ $title }» del proyecto.
resources-removed-many-toast = Se han quitado { $count } recursos del proyecto.
resources-detached-toast = Se ha separado «{ $title }».
resources-attached-one-toast = «{ $title }» añadido a los recursos de «{ $item }».
resources-attached-many-toast = { $count } recursos añadidos a «{ $item }».

## En el texto

resources-no-text = Ponga primero el cursor en un texto y luego inserte ahí el recurso.
resources-drop-insert-picture = Insertar la imagen
resources-drop-insert-link = Insertar un enlace al recurso

## Progreso

progress-resources-storing = Copiando los archivos
progress-resources-stored = Archivos copiados

## La página Recursos de una carpeta

segment-folder = Carpeta
resources-folder-count = { $count ->
    [one] Un recurso
   *[other] { $count } recursos
}
resources-view-label = Mostrar los recursos como
resources-view-list = Lista
resources-view-grid = Cuadrícula
resources-grid-label = Recursos
resources-folder-drop-hint = Suelte archivos o direcciones web para adjuntarlos a esta carpeta
resources-folder-empty = Todavía no hay nada adjunto a esta carpeta ni a lo que contiene. Añada archivos o enlaces web con los botones de arriba, o suéltelos aquí: se adjuntan a la carpeta.
resources-attached-to = En
resources-attached-separator = ,
resources-attached-more = { $count ->
    [one] y uno más
   *[other] y { $count } más
}
resources-row-name-attached = { $name }, en { $items }
resources-open-item = Abrir «{ $title }»
resources-detach-from = Separar de «{ $title }»

## Las imágenes que son recursos, en la Galería

gallery-resource-badge = Recurso
gallery-tile-resource = Imagen que es un recurso de { $title }: { $description }

## La página Recursos de los ajustes del proyecto

settings-page-resources = Recursos
settings-desc-resources = Los archivos y enlaces web que ya no lleva ningún elemento: vuelva a adjuntarlos, quítelos o exporte copias
settings-resources-description = Un recurso deja de estar adjunto a nada cuando se separa de su último elemento, o cuando el elemento que lo llevaba se elimina para siempre. Sigue en el proyecto, y un archivo guardado viaja con cada copia y cada copia de seguridad del proyecto, hasta que vuelva a adjuntarlo o lo quite.
settings-resources-group-storage = Almacenamiento
settings-resources-group-unattached = Adjuntos a nada
settings-resources-stored = Archivos guardados en este proyecto: { $size }
settings-resources-folder-advice = Un proyecto guardado como un solo archivo se reescribe entero cada vez que se guarda, sus archivos guardados también. Guardado como carpeta, solo guarda lo que ha cambiado, y es más rápido.
settings-resources-count = { $count ->
    [one] Un recurso
   *[other] { $count } recursos
}
settings-resources-empty = Cada recurso de este proyecto está adjunto a un elemento.
resources-attach-to-items = Adjuntar a elementos…
resources-export-copies = Exportar copias…
resources-export-copy = Exportar una copia…
resources-remove-many-title = ¿Quitar { $count } recursos del proyecto?
resources-attach-title = Adjuntar a elementos
resources-attach-hint = Elija los elementos a los que adjuntar. Ctrl+clic o Mayús+clic elige varios.
resources-attach-empty = Este proyecto todavía no tiene elementos.
resources-attach-confirm = Adjuntar
resources-attached = { $count ->
    [one] Un recurso adjuntado
   *[other] { $count } recursos adjuntados
} a { $items ->
    [one] un elemento
   *[other] { $items } elementos
}
resources-export-title = Exportar copias a una carpeta
resources-exported = { $count ->
    [one] Un recurso exportado
   *[other] { $count } recursos exportados
}
resources-export-missing = { $count ->
    [one] Un archivo no está donde el proyecto lo espera, y se ha dejado fuera: { $names }
   *[other] { $count } archivos no están donde el proyecto los espera, y se han dejado fuera: { $names }
}
resources-export-failed = { $count ->
    [one] Un archivo no se ha podido copiar: { $names }
   *[other] { $count } archivos no se han podido copiar: { $names }
}

## La preferencia de los archivos grandes (Comportamiento del editor)

settings-resources-large-file = Archivos de más de 10 MB
settings-resources-large-ask = Preguntar cada vez
settings-resources-large-store = Guardar siempre en el proyecto
settings-resources-large-link = Vincular siempre al archivo
settings-hint-resources-large-file = Qué ocurre con un archivo de más de 10 MB que adjunta como recurso. Guardado en el proyecto, viaja con cada copia y cada copia de seguridad del proyecto. Vinculado, se queda donde está, y el proyecto solo recuerda dónde encontrarlo.
