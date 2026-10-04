# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

settings-page-templates = Plantillas
settings-templates-description = Piezas de escritura reutilizables se puede caer en cualquier cosa que está escribiendo: una hoja de caracteres en blanco, un perfil de ubicación, una hoja de beats. Se almacenan en este proyecto, por lo que cualquiera que lo abra consigue el mismo conjunto.
settings-templates-filter = Plantillas de filtro
settings-templates-count = { $n ->
    [0] No hay plantillas
    [one] 1 plantilla
   *[other] { $n } plantillas
}
settings-templates-empty = Este proyecto aún no tiene plantillas.
settings-templates-empty-hint = Añádase uno de un preset, importar un archivo .md o .djot, o escribir algo y elegir el documento - Guardar como plantilla.
settings-templates-presets = Añadir un preset
settings-templates-import = Importar...
settings-templates-export = Exportar...
settings-templates-star = Mostrar primero en el menú de inserción
settings-templates-unstar = Deja de mostrar primero.
settings-templates-move-up = Muévanse.
settings-templates-move-down = Muévete.
settings-templates-delete = Borrar plantilla
settings-templates-name = Nombre
settings-templates-name-placeholder = Nombre de la plantilla
settings-templates-body-placeholder = El texto de esta plantilla inserta
settings-templates-duplicate-name = Otra plantilla ya se llama "{ $name }"
settings-templates-words = { $n ->
    [one] 1 palabra
   *[other] { $n } palabras
}

templates-imported = { $added ->
    [one] 1 plantilla importada
   *[other] { $added } plantillas importadas
}
templates-imported-renamed = Renombrado para evitar un choque: { $names }
templates-imported-skipped = No podía leer: { $names }
templates-import-filter = Plantillas (.md, .djot)
templates-exported = { $n ->
    [one] 1 plantilla exportada
   *[other] { $n } plantillas exportadas
}
templates-preset-applied = Añadido "{ $name }"

templates-delete-title = ¿Supir esta plantilla?
templates-delete-body = "{ $name }" se eliminará de este proyecto. Notas que ya creó de él no están afectados.
templates-delete-confirm = Eliminar

menu-document = &Documento
menu-insert-template = &Insertar plantilla
menu-insert-template-none = No hay plantillas en este proyecto
menu-save-as-template = &Guardar como plantilla...

save-as-template-title = Guardar como plantilla
save-as-template-explain = El texto que está editando se convierte en una plantilla que puede caer en cualquier otro lugar.
save-as-template-name = Nombre
save-as-template-placeholder = Ficha de personaje
save-as-template-duplicate = Una plantilla llamada "{ $name }" ya existe
save-as-template-empty-editor = No hay nada que salvar.
save-as-template-confirm = Guardar plantilla
save-as-template-saved = Guardado "{ $name }" como plantilla
template-inserted = Insertado "{ $name }"

note-template-preset-character-sheet = Ficha de personaje
note-template-preset-location = Lugar
note-template-preset-artifact = Objeto
note-template-preset-beat-sheet = Esquema de hitos narrativos
note-template-preset-faction = Facción
note-template-preset-research-note = Nota de investigación

note-template-set-essentials = Esenciales
note-template-set-everything = Cada plantilla

note-template-section-identity = Identidad
note-template-section-appearance = Apariencia
note-template-section-voice = Voz
note-template-section-psychology = Vida interior
note-template-section-history = Historia
note-template-section-arc = Arco narrativo
note-template-section-first-impression = Primera impresión
note-template-section-in-the-story = En la historia
note-template-section-the-thing-itself = El objeto en sí
note-template-section-the-scene = La escena
note-template-section-the-shape = La forma
note-template-section-what-it-is = Lo que es
note-template-section-source = Origen
note-template-section-what-it-says = Lo que dice

note-template-field-full-name = Nombre completo
note-template-field-known-as = También conocido como, rastreado en el campo Alias del Inspector
note-template-field-age = Edad
note-template-field-role-in-story = Papel en la historia
note-template-field-build-and-features = Construir y distinguir características
note-template-field-habitual-bearing = Cojinete orbital
note-template-field-speech-patterns = Patrones de habla y tics verbales
note-template-field-what-they-never-say = Lo que nunca dicen
note-template-field-want = Lo que quieren
note-template-field-need = Lo que necesitan pero no lo saben
note-template-field-fear = Lo que temen
note-template-field-flaw = El defecto que les cuesta
note-template-field-formative-event = Evento formativo
note-template-field-relationships = Relaciones
note-template-field-starts-as = Empieza como
note-template-field-ends-as = Finales como
note-template-field-what-you-notice-first = Lo que notaste primero
note-template-field-sound-and-smell = Sonido y olor
note-template-field-light-and-weather = Luz y clima
note-template-field-what-happened-here = ¿Qué pasó aquí?
note-template-field-who-lives-or-works-here = Quien vive o trabaja aquí
note-template-field-scenes-set-here = Escenas puestas aquí
note-template-field-why-it-matters = Por qué importa
note-template-field-appearance = Apariencia
note-template-field-age-and-origin = Edad y origen
note-template-field-what-it-does = ¿Qué hace?
note-template-field-who-holds-it = Que lo sostiene
note-template-field-who-wants-it = ¿Quién lo quiere?
note-template-field-pov = Punto de vista
note-template-field-time-and-place = Tiempo y lugar
note-template-field-goal = Objetivo
note-template-field-conflict = Conflicto
note-template-field-turn = El giro
note-template-field-exit-emotion = Emoción en el camino
note-template-field-purpose = Propósito
note-template-field-who-leads-it = ¿Quién lo guía?
note-template-field-resources = Recursos
note-template-field-allies-and-enemies = Aliados y enemigos
note-template-field-what-it-wants-now = Lo que quiere ahora
note-template-field-where-from = De donde vino
note-template-field-page-or-link = Página o enlace
note-template-field-key-facts = Principales hechos
note-template-field-how-it-is-used = Cómo se utiliza en la historia
