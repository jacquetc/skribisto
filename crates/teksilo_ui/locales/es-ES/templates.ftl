# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

settings-page-templates = Plantillas
settings-templates-description = Fragmentos de texto reutilizables que puede insertar en cualquier cosa que esté escribiendo: una ficha de personaje en blanco, un perfil de lugar, una escaleta. Se guardan en este proyecto, así que cualquiera que lo abra obtiene el mismo conjunto.
settings-templates-filter = Filtrar plantillas
settings-templates-count = { $n ->
    [0] No hay plantillas
    [one] 1 plantilla
   *[other] { $n } plantillas
}
settings-templates-empty = Este proyecto aún no tiene plantillas.
settings-templates-empty-hint = Añada una a partir de un modelo, importe un archivo .md o .djot, o escriba algo y elija Documento ▸ Guardar como plantilla.
settings-templates-presets = Añadir un modelo
settings-templates-import = Importar…
settings-templates-export = Exportar…
settings-templates-star = Mostrar primero en el menú de inserción
settings-templates-unstar = Dejar de mostrar primero
settings-templates-move-up = Subir
settings-templates-move-down = Bajar
settings-templates-delete = Eliminar plantilla
settings-templates-name = Nombre
settings-templates-name-placeholder = Nombre de la plantilla
settings-templates-body-placeholder = El texto que inserta esta plantilla
settings-templates-duplicate-name = Ya hay otra plantilla llamada «{ $name }»
settings-templates-words = { $n ->
    [one] 1 palabra
   *[other] { $n } palabras
}

templates-imported = { $added ->
    [one] 1 plantilla importada
   *[other] { $added } plantillas importadas
}
templates-imported-renamed = Renombradas para evitar un conflicto: { $names }
templates-imported-skipped = No se pudieron leer: { $names }
templates-import-filter = Plantillas (.md, .djot)
templates-exported = { $n ->
    [one] 1 plantilla exportada
   *[other] { $n } plantillas exportadas
}
templates-preset-applied = «{ $name }» añadida

templates-delete-title = ¿Eliminar esta plantilla?
templates-delete-body = «{ $name }» se quitará de este proyecto. Las notas que ya creó a partir de ella no se ven afectadas.
templates-delete-confirm = Eliminar

menu-document = &Documento
menu-insert-template = Insertar p&lantilla
menu-insert-template-none = No hay plantillas en este proyecto
menu-save-as-template = &Guardar como plantilla…

save-as-template-title = Guardar como plantilla
save-as-template-explain = El texto que está editando se convierte en una plantilla que puede insertar en cualquier otro lugar.
save-as-template-name = Nombre
save-as-template-placeholder = Ficha de personaje
save-as-template-duplicate = Ya existe una plantilla llamada «{ $name }»
save-as-template-empty-editor = Este editor está vacío. No hay nada que guardar.
save-as-template-confirm = Guardar plantilla
save-as-template-saved = «{ $name }» guardado como plantilla
template-inserted = «{ $name }» insertada

note-template-preset-character-sheet = Ficha de personaje
note-template-preset-location = Lugar
note-template-preset-artifact = Objeto
note-template-preset-beat-sheet = Escaleta
note-template-preset-faction = Facción
note-template-preset-research-note = Nota de documentación

note-template-set-essentials = Esenciales
note-template-set-everything = Todas las plantillas

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
note-template-field-known-as = También conocido como, registrado en el campo Otros nombres del inspector
note-template-field-age = Edad
note-template-field-role-in-story = Papel en la historia
note-template-field-build-and-features = Complexión y rasgos distintivos
note-template-field-habitual-bearing = Porte habitual
note-template-field-speech-patterns = Patrones de habla y tics verbales
note-template-field-what-they-never-say = Lo que nunca dicen
note-template-field-want = Lo que quieren
note-template-field-need = Lo que necesitan pero no lo saben
note-template-field-fear = Lo que temen
note-template-field-flaw = El defecto que les cuesta
note-template-field-formative-event = Evento formativo
note-template-field-relationships = Relaciones
note-template-field-starts-as = Empieza como
note-template-field-ends-as = Termina como
note-template-field-what-you-notice-first = Lo primero que se nota
note-template-field-sound-and-smell = Sonido y olor
note-template-field-light-and-weather = Luz y clima
note-template-field-what-happened-here = Qué pasó aquí
note-template-field-who-lives-or-works-here = Quién vive o trabaja aquí
note-template-field-scenes-set-here = Escenas que transcurren aquí
note-template-field-why-it-matters = Por qué importa
note-template-field-appearance = Apariencia
note-template-field-age-and-origin = Edad y origen
note-template-field-what-it-does = Qué hace
note-template-field-who-holds-it = Quién lo tiene
note-template-field-who-wants-it = Quién lo quiere
note-template-field-pov = Punto de vista
note-template-field-time-and-place = Tiempo y lugar
note-template-field-goal = Objetivo
note-template-field-conflict = Conflicto
note-template-field-turn = El giro
note-template-field-exit-emotion = Emoción a la salida
note-template-field-purpose = Propósito
note-template-field-who-leads-it = Quién la dirige
note-template-field-resources = Recursos
note-template-field-allies-and-enemies = Aliados y enemigos
note-template-field-what-it-wants-now = Lo que quiere ahora
note-template-field-where-from = De dónde viene
note-template-field-page-or-link = Página o enlace
note-template-field-key-facts = Datos clave
note-template-field-how-it-is-used = Cómo se usa en la historia
