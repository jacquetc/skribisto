# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

menu-work = &Proyecto
menu-new-work = N&uevo proyecto
menu-open-work = &Abrir proyecto…
menu-new-window = &Nueva ventana
menu-import-from = I&mportar desde
menu-create-from = &Crear desde
menu-import-plume = &Plume Creator (.plume)…
menu-import-document = &Documentos (Markdown, Word, ODT)…
menu-import-manuskript = &Manuskript (.msk)…
menu-import-scrivener = &Scrivener (.scriv)…
menu-export = &Exportar
menu-export-book = Exportar libro
menu-export-part = Exportar parte
menu-export-chapter = Exportar capítulo
menu-export-scene = Exportar escena
menu-export-note = Exportar nota
menu-export-paratext = Exportar paratexto
menu-export-folder = Exportar carpeta
menu-export-choose = Elegir…
menu-export-none = Abrir un documento para exportar
menu-save = &Guardar
menu-save-as-file = Guardar como arc&hivo único…
menu-save-as-folder = Guar&dar como carpeta…
menu-backup = Crear c&opia de seguridad ahora
menu-close-work = &Cerrar proyecto
menu-welcome = &Bienvenida…
menu-settings = Con&figuración
menu-quit = &Salir

menu-view = &Ver
menu-outline = &Esquema
menu-search = &Buscar en el proyecto
menu-timeline = &Volver al pasado
menu-search-preview = V&ista previa de búsqueda
menu-fullscreen = P&antalla completa
menu-focus-mode = M&odo sin distracciones
menu-format = &Formato
menu-scene-break = In&sertar separador de escena
menu-major-scene-break = Insertar separador de escena &mayor
menu-line-break = Insertar salto de l&ínea

menu-go = &Ir
menu-go-next-scene = &Siguiente escena
menu-go-prev-scene = &Escena anterior
menu-go-next-chapter = Si&guiente capítulo
menu-go-prev-chapter = &Capítulo anterior
menu-go-next-note = S&iguiente nota
menu-go-prev-note = &Nota anterior

menu-tools = &Herramientas
menu-help = &Ayuda

ctx-add = &Añadir
ctx-new-item = &Nuevo elemento
ctx-new-folder = &Nueva carpeta
ctx-rename = &Cambiar nombre
ctx-duplicate = &Duplicar
ctx-import-here = &Importar aquí…
ctx-indent = &Aumentar sangría
ctx-outdent = &Reducir sangría
ctx-trash = &Mover a la papelera
ctx-open-to-side = A&brir al lado
ctx-open = &Abrir
ctx-reveal-in-outline = Mostrar en el es&quema
ctx-move-up = &Subir
ctx-move-down = &Bajar

ctx-tab-close = &Cerrar
ctx-tab-close-others = Ce&rrar las demás
ctx-tab-close-all = C&errar todas
ctx-tab-open-to-side = &Abrir al lado
ctx-tab-open-in-main = Abrir en el panel &principal
ctx-tab-move-to-side = &Mover al lado
ctx-tab-move-to-main = &Mover al panel principal
ctx-tab-move-to-new-window = Mover a una &nueva ventana
ctx-tab-move-window-unsaved = Guarde primero este proyecto: una segunda ventana se abre sobre un archivo en el disco
ctx-tab-pin = &Fijar esta pestaña
ctx-tab-unpin = &Soltar esta pestaña
tab-pinned-tooltip = Fijada: «Cerrar las demás» y «Cerrar todas» la dejan abierta

create-book = Libro
create-part = Parte
create-chapter = Capítulo
create-scene = Escena
create-note = Nota
create-note-folder = Carpeta de notas
create-folder = Carpeta
create-paratext = Paratexto
create-paratext-folder = Carpeta de paratextos
create-book-end = Fin del libro
create-story-bible-entry = Entrada de la biblia narrativa…
type-book-start = Inicio del libro
type-text = Texto

new-item-book = Nuevo libro
new-item-part = Nueva parte
new-item-chapter = Nuevo capítulo
new-item-note = Nueva nota
new-item-note-folder = Nueva carpeta de notas
new-item-folder = Nueva carpeta
new-item-paratext = Nuevo paratexto
new-item-paratext-folder = Nueva carpeta de paratextos
new-item-story-bible-entry = Nueva entrada de la biblia narrativa

placement-inside = dentro
placement-after = después
placement-after-parent = después del padre
placement-top-level = nivel superior


ctx-promote = C&onvertir en
promote-chapter-folder = Carpeta de capítulo
promote-flat-chapter = Capítulo plano
promote-lossy-title = No hay lugar para guardar el texto
promote-lossy-text = Un elemento de tipo { $target } no tiene dónde guardar esto: { $kinds }. Mueva o borre ese texto primero y luego convierta.
content-scene-text = Texto de la escena
content-note-text = Texto de la nota
content-book-title = Título del libro
content-book-subtitle = Subtítulo del libro
content-part-title = Título de la parte
content-chapter-title = Título del capítulo
content-epigraph-text = Epígrafe
content-paratext-text = Paratexto
promote-blocked-title = Capítulo no vacío
promote-blocked-text = { $count ->
    [one] Este capítulo aún contiene { $count } elemento. Muévalo o envíelo a la papelera antes de convertirlo en capítulo plano.
   *[other] Este capítulo aún contiene { $count } elementos. Muévalos o envíelos a la papelera antes de convertirlo en capítulo plano.
}

inspector = Inspector
inspector-empty = Abra un elemento para inspeccionarlo.
inspector-promote = Convertir en…
statusbar-toggle-outline = Mostrar u ocultar el archivador
statusbar-toggle-inspector = Mostrar u ocultar el inspector
statusbar-save-unsaved = Cambios sin guardar. Haga clic para guardar
statusbar-save-saved = Todos los cambios guardados
statusbar-save-autosave = El guardado automático está activado. Los cambios se guardan a medida que escribe
statusbar-saving = Guardando…
statusbar-word-count = { $count ->
    [one] { $count } palabra
   *[other] { $count } palabras
}
statusbar-word-count-tooltip = Palabras de la escena que está editando
statusbar-word-char-count = { $words ->
    [one] { $words } palabra
   *[other] { $words } palabras
} · { $chars ->
    [one] { $chars } carácter
   *[other] { $chars } caracteres
}
session-toggle = Sesión de escritura: iniciar o pausar un sprint concentrado
session-configure = Definir el objetivo de palabras y el límite de tiempo de la sesión
session-configure-title = Sesión de escritura
session-word-goal = Objetivo de palabras
session-time-limit = Límite de tiempo
session-time-limit-unit = min
session-no-goal = Sin objetivo
session-no-limit = Sin límite
session-reset = Reiniciar la sesión
session-readout = { $words ->
    [one] { $words } palabra
   *[other] { $words } palabras
} · { $time }
session-readout-timed = { $words ->
    [one] { $words } palabra
   *[other] { $words } palabras
} · quedan { $time }
statusbar-focus-exit = Salir del modo sin distracciones
statusbar-focus-synopsis = Sinopsis
statusbar-focus-comments = Comentarios
statusbar-focus-go-prev = Elemento anterior (Alt+Arriba)
statusbar-focus-go-next = Elemento siguiente (Alt+Abajo)

rail-settings = Configuración

language = Idioma
theme = Tema
english = Inglés
french = Francés
light = Claro
dark = Oscuro
settings-text-width = Ancho de texto
settings-preview-width = Ancho de la vista previa de búsqueda
settings-autosave = Guardar automáticamente en el disco
settings-show-welcome = Mostrar el lanzador al iniciar
settings-show-welcome-tip = Si está desactivado, se vuelve a abrir el último proyecto.
settings-check-for-updates = Buscar nuevas versiones
settings-check-for-updates-tip = Como mucho una vez al día, Skribisto pregunta al sitio web del proyecto cuál es la versión actual. La solicitud no lleva ningún identificador, ni siquiera la versión que usa: la comparación se hace en este equipo.

settings-title = Configuración
settings-close = Cerrar
settings-search = Buscar en la configuración
settings-reset = Restablecer valores predeterminados
settings-done = Listo
settings-cancel = Cancelar
settings-apply = Aplicar
settings-ok = Aceptar
settings-reset-confirm-title = ¿Restablecer toda la configuración a los valores predeterminados?
settings-reset-confirm-body = Restablece la apariencia, el editor y la configuración de escritura de esta aplicación. La configuración propia de su proyecto, los atajos de teclado y los estilos guardados no se ven afectados. No se puede deshacer.
settings-empty-title = Todavía no hay opciones aquí
settings-empty-hint = Esta sección tendrá opciones en una próxima actualización.

settings-sec-appearance-behaviour = Apariencia y comportamiento
settings-sec-editor = Editor
settings-sec-spelling = Ortografía
settings-sec-backup = Copias de seguridad y sincronización
settings-sec-compile = Compilación y exportación
settings-page-appearance = Apariencia
settings-field-image-size-policy = Imágenes grandes
settings-image-policy-ask = Preguntar cada vez
settings-image-policy-keep = Conservar el original
settings-image-policy-downscale = Optimizar
settings-hint-image-size-policy = Qué hacer cuando una imagen insertada es más grande de lo que necesita el manuscrito.
settings-page-notifications = Notificaciones
settings-page-scene = Escena
settings-page-synopsis = Sinopsis
settings-page-notes = Notas
settings-page-editor-behavior = Comportamiento del editor
settings-page-goals = Objetivos y recuento de palabras
settings-page-games = Juegos de escritura
settings-page-corkboard = Tablón de corcho
settings-page-distraction-free = Sin distracciones
settings-page-dictionaries = Diccionarios
settings-page-autosave = Guardado automático
settings-page-export = Formatos de exportación
settings-page-paratext = Estructuras de paratexto
settings-page-keymap = Asignación de teclas
settings-keymap-filter = Filtrar atajos

settings-desc-sec-appearance-behaviour = El aspecto de la propia aplicación y lo que hace al iniciarse.
settings-desc-sec-editor = La superficie de escritura: su aspecto y lo que hace mientras escribe.
settings-desc-sec-spelling = La revisión ortográfica y los diccionarios en los que se basa.
settings-desc-sec-backup = Cómo llega su trabajo al disco y qué copias se conservan.
settings-desc-sec-compile = Lo que sale de Skribisto, y en qué forma.
settings-desc-sec-work = Configuración propia de este proyecto, que viaja dentro de su archivo.
settings-desc-sec-extensions = Páginas aportadas por las extensiones instaladas aquí.
settings-desc-group-typography = Una página por tipo de texto: tipo de letra, tamaño, interlineado y espaciado.
settings-desc-appearance = Idioma de la interfaz, tema, tamaño del texto y lanzador al iniciar.
settings-desc-notifications = Todos los mensajes mostrados en esta sesión y las acciones que puede repetir.
settings-desc-scene = Cómo se compone la prosa de las escenas: tipo de letra, tamaño, interlineado, sangrías y espaciado.
settings-desc-synopsis = Cómo se compone el panel de sinopsis, con independencia del manuscrito.
settings-desc-notes = Cómo se componen las notas, con independencia del manuscrito.
settings-desc-corkboard = Tamaño de las tarjetas, lo que muestra cada tarjeta y cómo las ordena el tablón.
settings-desc-distraction-free = Tipografía, ancho de columna y barra de controles para escribir a pantalla completa.
settings-desc-distraction-free-themes = La colección de temas para escribir a pantalla completa: los incluidos y los suyos.
settings-desc-editor-behavior = Ancho del texto, posición de la sinopsis, desplazamiento de máquina de escribir y resaltado alrededor del cursor.
settings-desc-punctuation = El estilo tipográfico de la casa con el que empiezan los proyectos nuevos. Todos los proyectos lo siguen salvo que definan el suyo.
settings-desc-goals = Objetivos de palabras y de caracteres, y cómo se cuentan las palabras.
settings-desc-games = Restricciones que usted mismo se impone al redactar, como Siempre hacia adelante.
settings-desc-spellcheck = El interruptor único que activa y desactiva la revisión ortográfica en todas partes.
settings-desc-dictionaries = Instale, quite y explore los diccionarios ortográficos de este equipo.
settings-desc-autosave = Si sus cambios se escriben en el disco por sí solos.
settings-desc-backup = Cuándo se hacen las copias, dónde se guardan y cuántas. Todos los proyectos siguen esto salvo que definan lo suyo.
settings-desc-export = Los estilos con los que se compila cada exportación: los incluidos y los suyos.
settings-desc-paratext = Las páginas preliminares y finales con las que puede empezar un proyecto nuevo.
settings-desc-keymap = Todos los atajos y la acción a la que están asignados.
settings-page-user = Usuario
settings-desc-user = Quién es usted, para los comentarios que escribe.
settings-group-identity = Identidad
settings-field-user-name = Su nombre
settings-field-user-name-placeholder = Opcional
settings-field-user-name-hint = Firma los comentarios y las respuestas que escribe. Es distinto del nombre de autor de un proyecto, que es la firma del libro y viaja dentro del archivo.
settings-field-user-initials = Sus iniciales
settings-field-user-initials-placeholder = A partir de su nombre
settings-field-user-initials-hint = Lo que un procesador de textos muestra junto a su comentario en el margen. Déjelo en blanco para usar las que se muestran, tomadas de su nombre.
settings-field-user-hint = Ambos son opcionales y se aplican a todos los proyectos de este equipo. Al cambiarlos se firman los comentarios que escriba a partir de ahora. Los comentarios ya escritos conservan el nombre con el que se escribieron.
settings-desc-author = El nombre que aparece en este proyecto.
settings-desc-structure = Si los capítulos de este proyecto son carpetas o elementos individuales.
settings-desc-language = El idioma con el que se revisa la prosa de este proyecto.
settings-desc-work-backup = La política de copias de seguridad propia de este proyecto, o la general.
settings-desc-personal-dictionary = Las palabras que este proyecto considera bien escritas.
settings-desc-tags = Las etiquetas de colores con las que este proyecto marca sus elementos.
settings-desc-templates = Las plantillas de notas que puede insertar al escribir en este proyecto.
settings-desc-text-replacements = Atajos que se expanden mientras escribe en este proyecto.
settings-desc-work-punctuation = Las comillas, rayas y espacios que sigue la prosa de este proyecto.

settings-group-typography = Tipografía
settings-group-writing-column = Columna de escritura
settings-group-writing-view = Vista de escritura
settings-group-distraction-free-strip = Barra de controles
settings-group-theme = Tema
settings-group-language = Idioma
settings-group-startup = Inicio
settings-group-updates = Actualizaciones
settings-group-autosave = Guardado automático
settings-field-typeface = Tipo de letra
settings-field-size = Tamaño del texto

editor-size-changed-manuscript = Tamaño del texto del manuscrito: { $percent }
editor-size-changed-synopsis = Tamaño del texto de la sinopsis: { $percent }
editor-size-changed-notes = Tamaño del texto de las notas: { $percent }
editor-size-changed-corkboard = Tamaño del texto de las tarjetas del tablón: { $percent }
editor-size-changed-corkboard-expanded = Tamaño del texto del editor ampliado: { $percent }
editor-size-changed-distraction-free = Tamaño del texto sin distracciones: { $percent }

menu-text-size-increase = A&umentar el tamaño del texto
menu-text-size-decrease = &Reducir el tamaño del texto
menu-text-size-reset = Res&tablecer el tamaño del texto
settings-field-line-height = Interlineado
settings-field-first-line-indent = Sangría de primera línea
settings-field-paragraph-spacing-before = Espacio antes del párrafo
settings-field-paragraph-spacing-after = Espacio después del párrafo
settings-field-column-width = Ancho de columna
settings-distraction-free-width-hint = Solo se aplica mientras el modo sin distracciones está activo. Los anchos de columna de Escena, Sinopsis y Notas no cambian.
settings-distraction-free-title = Mostrar el nombre del elemento
settings-distraction-free-word-count = Mostrar el recuento de palabras
settings-distraction-free-session = Mostrar la sesión de escritura
settings-distraction-free-go-to = Mostrar el botón Ir a…
settings-distraction-free-go = Mostrar los botones Anterior y Siguiente
settings-distraction-free-chrome-hint = El botón Salir permanece siempre, elija lo que elija aquí. Es su salida si la tecla Escape está ocupada.
settings-field-app-theme = Tema
settings-theme-light = Claro
settings-theme-dark = Oscuro
settings-field-text-scale = Tamaño del texto de la interfaz
settings-field-language = Idioma de la interfaz
settings-synopsis-placement = Posición de la sinopsis
settings-synopsis-placement-none = Ninguna
settings-synopsis-placement-top = Arriba
settings-synopsis-placement-side = Al lado
settings-typewriter = Desplazamiento de máquina de escribir
settings-typewriter-tip = Mantiene la línea que escribe a una altura fija mientras el manuscrito se desplaza por debajo. Un clic sigue colocando el cursor donde hace clic.
settings-typewriter-position = Posición de la línea
settings-typewriter-position-top-third = Tercio superior
settings-typewriter-position-middle = Centro
settings-typewriter-position-bottom-quarter = Cuarto inferior
settings-highlight-scope = Resaltado alrededor del cursor
settings-highlight-scope-none = Ninguno
settings-highlight-scope-sentence = Oración
settings-highlight-scope-paragraph = Párrafo
settings-highlight-scope-tip-none = Deja la página lisa. No se sombrea nada mientras escribe.
settings-highlight-scope-tip-sentence = Sombrea la oración que está escribiendo, para que destaque entre las que la rodean.
settings-highlight-scope-tip-paragraph = Sombrea todo el párrafo que está escribiendo, para mantener a la vista el pasaje en el que trabaja.
settings-group-container-views = Vistas de los contenedores
settings-remember-view = Recordar la última vista de cada tipo de elemento
settings-remember-view-tip = Abre un contenedor en la última vista que usó para ese tipo
settings-remember-view-tip-more =
    Un libro, una parte y un capítulo ofrecen varias vistas cada uno (su propia página, el
    manuscrito completo, la sinopsis completa). Active esta opción y cada tipo se vuelve a abrir
    en la última vista que eligió para él: por ejemplo, si cambia un capítulo a Capítulo completo,
    el siguiente capítulo que abra también mostrará Capítulo completo. Cada tipo recuerda su propia vista.
settings-group-counting = Recuento de palabras
settings-counting-auto = Automático (por idioma)
settings-counting-whitespace = Dividir en espacios
settings-counting-unicode-words = Palabras Unicode
settings-counting-cjk-hybrid = Adaptado a CJK (por carácter)
settings-counting-hint = El modo automático cuenta el chino y el japonés por carácter y los demás idiomas por palabra. Cámbielo solo si el recuento parece incorrecto para su idioma.
settings-group-goals-display = Visualización
settings-show-characters = Mostrar el recuento de caracteres en la barra de estado
settings-autosave-hint = Los cambios se escriben automáticamente en el disco mientras escribe.

settings-sec-work = Proyecto
settings-page-structure = Estructura
settings-page-author = Autor
settings-field-author-name = Nombre del autor
settings-field-author-placeholder = Opcional
settings-field-author-hint = Aparece en la portada compilada y en los metadatos de los archivos exportados. Déjelo en blanco para omitirlo. Es la firma del libro. El nombre con el que se firman sus comentarios está en Configuración ▸ Usuario.
settings-group-chapters = Capítulos
tidy-titles-title = Ordenar los títulos de capítulo
tidy-titles-none = Ningún título de capítulo o de parte se limita a repetir su propio número.
tidy-titles-lead = { $count ->
        [one] Un capítulo o parte lleva como título solo su propio número.
       *[other] { $count } capítulos y partes llevan como título solo su propio número.
    }
tidy-titles-explain = Al borrar esos títulos, cada uno queda nombrado por el número que el libro ya conoce, el mismo que imprime la exportación. No cambia nada más, y un solo deshacer los restaura todos.
menu-document-tidy-titles = &Ordenar los títulos de capítulo…
settings-group-numbering = Numeración
settings-number-chapters = Numerar capítulos y partes
settings-number-chapters-tip = Los capítulos y las partes llevan un número calculado según su lugar en el libro, que se muestra aquí junto a su título y que imprime la exportación.
settings-number-chapters-tip-more = El número nunca se guarda en el título, así que sigue siendo correcto cuando reordena, inserta o elimina. Si desactiva esta opción, la exportación imprime solo los títulos, pida lo que pida el estilo de exportación. Para dejar fuera un capítulo (un prólogo, un interludio), use en su lugar el interruptor Numeración del inspector: lo mantiene en el libro pero sin número.
settings-part-resets-chapter = Reiniciar la numeración de capítulos en cada parte
settings-part-resets-chapter-tip = Desactivado por defecto: los capítulos se numeran seguidos a lo largo de las partes de un libro, de modo que la «Segunda parte» empieza en el capítulo once.
settings-part-resets-chapter-tip-more = Es la práctica editorial habitual, y lo que espera un lector. Actívelo para un libro cuyas partes deban leerse como volúmenes independientes, en el que cada una empiece en el capítulo uno.
settings-chapter-flat = Capítulos planos
settings-chapter-flat-hint = Activado: un capítulo es un único elemento. Usted escribe en él y no contiene escenas. Desactivado: un capítulo es una carpeta. También escribe en él, pero además puede contener escenas. Los capítulos nuevos siguen este ajuste; los existentes se convierten con Convertir en.

settings-styles-builtin = Estilos integrados
settings-styles-user = Mis estilos
settings-styles-builtin-badge = Integrado
settings-styles-duplicate = Duplicar
settings-styles-edit = Editar
settings-styles-delete = Eliminar
settings-styles-import = Importar…
settings-styles-export = Exportar…
settings-styles-copy-suffix = (copia)
settings-styles-json-filter = Estilo de exportación
settings-styles-editor-title = Editar estilo
settings-styles-editor-none = Seleccione un estilo personalizado para editarlo, o duplique uno integrado.
settings-styles-imported = Estilo importado
settings-styles-import-failed = No se pudo importar el estilo
settings-styles-exported = Estilo exportado
settings-styles-export-failed = No se pudo exportar el estilo
settings-styles-field-name = Nombre
settings-styles-field-chapters = Encabezados de capítulo
settings-styles-field-parts = Encabezados de parte
settings-styles-field-scene-break = Separador de escena
settings-styles-field-major-scene-break = Separador de escena mayor
settings-styles-field-spacing = Interlineado
settings-styles-field-justify = Justificar el texto
settings-styles-field-notes = Incluir notas
settings-styles-field-synopses = Incluir sinopsis
settings-styles-field-scene-titles = Incluir títulos de escena
settings-styles-field-epigraphs = Incluir epígrafes
settings-styles-field-paratexts = Incluir paratextos
settings-styles-field-epigraph-placement = Posición del epígrafe
settings-styles-epigraph-after = Después del título
settings-styles-epigraph-before = Antes del título
settings-styles-field-footnotes = Incluir notas al pie
settings-styles-field-footnote-numbering = Numeración de las notas al pie
settings-styles-footnote-numbering-continuous = Continua
settings-styles-footnote-numbering-per-chapter = Reiniciar en cada capítulo
settings-styles-footnote-numbering-per-book = Reiniciar en cada libro
settings-styles-field-images = Imágenes
menu-image = I&magen
settings-styles-images-beside = Junto al documento
settings-styles-images-embed = Dentro del documento
settings-styles-images-omit = Omitir
settings-styles-group-round-trip = Envío a un editor
settings-styles-field-comments = Incluir comentarios
settings-styles-field-round-trip-marks = Incluir marcadores de ida y vuelta
settings-styles-round-trip-hint = Solo DOCX y ODT. Los marcadores son identificadores invisibles que permiten que un archivo devuelto actualice este proyecto en lugar de quedar a su lado como una segunda copia.
settings-styles-group-pages = Páginas
settings-styles-field-cover = Empezar con la cubierta
settings-styles-field-word-count = Recuento de palabras en la portada
settings-styles-field-page-books = Nueva página en cada libro
settings-styles-field-page-parts = Nueva página en cada parte
settings-styles-field-page-chapters = Nueva página en cada capítulo
settings-styles-field-page-paratexts = Nueva página en cada paratexto
settings-styles-heading-none = Sin encabezado
settings-styles-heading-numbered = Solo número
settings-styles-heading-title = Solo título
settings-styles-heading-both = Número + título
settings-styles-break-blank = Línea en blanco
settings-styles-break-none = Ninguno
settings-styles-spacing-single = Sencillo
settings-styles-spacing-onehalf = 1½
settings-styles-spacing-double = Doble

welcome-title = Bienvenido a Skribisto
welcome-version = Versión { $version }
welcome-update-available = Versión { $version } está disponible
welcome-update-tip = Abre la página de descarga en su navegador.
welcome-search = Buscar proyectos
welcome-open = Abrir
welcome-new-work = Nuevo proyecto
welcome-create-from = Crear desde…
welcome-recent-works = Proyectos recientes
welcome-empty-recents = Todavía no hay proyectos recientes.
welcome-no-matches = Ningún proyecto reciente coincide con su búsqueda.
welcome-learn-soon = Pronto habrá guías y consejos.
welcome-about-blurb = Skribisto, la aplicación de escritura reescrita en Rust + Teksilo.
welcome-tagline = *Un lugar tranquilo para escribir cosas largas.*
welcome-github = GitHub
welcome-discord = Discord
nav-works = Proyectos
nav-examples = Ejemplos
nav-learn = Aprender
nav-about = Acerca de

synopsis = Sinopsis
epigraph = Epígrafe
pane-manuscript = Manuscrito
corkboard = Tablón de corcho
overview = Vista general

corkboard-card-count = { $count ->
    [one] { $count } tarjeta
   *[other] { $count } tarjetas
}
corkboard-child-count = { $count ->
    [one] { $count } elemento
   *[other] { $count } elementos
}
corkboard-view-nested = Anidado
corkboard-view-flat = Plano
corkboard-layout-hint = Anidado muestra los hijos directos de un contenedor. Abra una tarjeta de carpeta para entrar en ella. Plano muestra todas las escenas del contenedor a la vez.
corkboard-card-size = Tamaño de la tarjeta
corkboard-search-placeholder = Filtrar tarjetas…
corkboard-empty-title = Todavía no hay nada aquí
corkboard-empty-hint = Use «＋ Nuevo» arriba para añadir el primero.
corkboard-new = Nuevo
corkboard-show-card-numbers = Numerar las tarjetas
corkboard-modal-size = Tamaño del editor ampliado
corkboard-scope-hint = Se aplica a todos los tablones de corcho abiertos, en todos los proyectos.
corkboard-sort-manuscript = Orden del manuscrito
corkboard-sort-title-asc = Título A–Z
corkboard-sort-title-desc = Título Z–A
corkboard-card-number = Tarjeta { $number }

duplicate = Duplicar
duplicate-n = { $count ->
    [one] Duplicar { $count } tarjeta
   *[other] Duplicar { $count } tarjetas
}
set-label-n = { $count ->
    [one] Poner rótulo a { $count } tarjeta
   *[other] Poner rótulo a { $count } tarjetas
}
move-to-trash-n = { $count ->
    [one] Mover { $count } tarjeta a la papelera
   *[other] Mover { $count } tarjetas a la papelera
}
reveal-in-outline = Mostrar en el esquema

corkboard-move-to = Mover a…
corkboard-move-to-n = { $count ->
    [one] Mover { $count } tarjeta a…
   *[other] Mover { $count } tarjetas a…
}
corkboard-move-picker-title = Mover a…
corkboard-move-picker-empty = Todavía no hay archivadores. Cree uno primero.
corkboard-move-picker-cancel = Cancelar
corkboard-move-here = Mover aquí
corkboard-moved-ok = { $count ->
    [one] { $count } tarjeta movida
   *[other] { $count } tarjetas movidas
}
corkboard-move-into-self = Un contenedor no se puede mover dentro de sí mismo. Elija un destino fuera de él.
corkboard-move-failed = No se pudieron mover esas tarjetas allí.

overview-col-title = Título
overview-col-type = Tipo
overview-col-label = Rótulo
overview-col-tags = Etiquetas
overview-col-books = Libros
overview-col-own-words = Palabras
overview-col-total-words = Total
overview-row-count = { $count ->
    [one] { $count } elemento
   *[other] { $count } elementos
}
overview-search-placeholder = Filtrar elementos…
overview-expand-all = Expandir todo
overview-collapse-all = Contraer todo
overview-table-label = Contenido
overview-empty-title = Todavía no hay nada aquí
overview-gone-title = Este contenedor ya no existe
overview-gone-hint = Se movió a la papelera. Restáurelo o cierre esta pestaña.
overview-empty-hint = Use «＋ Nuevo» arriba para añadir el primero.
corkboard-grid-label = Tarjetas del tablón de corcho
corkboard-rename-field = Cambiar nombre del elemento
corkboard-expand-synopsis = Ampliar la sinopsis
corkboard-synopsis-modal-title = Sinopsis
corkboard-badge-scene = Escena
corkboard-badge-chapter = Capítulo
corkboard-badge-part = Parte
corkboard-badge-book = Libro
corkboard-badge-note = Nota
corkboard-badge-folder = Carpeta
corkboard-badge-paratext = Paratexto
corkboard-badge-text = Texto
corkboard-badge-end = Fin
settings-group-corkboard-layout = Disposición
settings-group-corkboard-cards = Tarjetas
corkboard-show-word-count = Mostrar el recuento de palabras en las tarjetas
text-heading = Texto
no-content = Este elemento no tiene contenido editable.
untitled = Sin título
placeholder-title = Título…
placeholder-subtitle = Subtítulo…
placeholder-chapter-title = Título del capítulo…
split-editor = Dividir el editor
close-split-view = Cerrar la vista dividida
drop-open-here = Abrir aquí
drop-open-to-side = Abrir al lado

segment-chapter = Capítulo
segment-part = Parte
segment-book = Libro
segment-pace = Ritmo
segment-notes = Notas
segment-story-bible = Biblia narrativa
segment-note-own = Nota
segment-note-details = Detalles
segment-note-in-prose = En la prosa
pace-placeholder = Aquí aparece el planificador de ritmo.
pace-empty-title = Planifique el ritmo de este libro
pace-empty-body = Fije un objetivo de palabras y una fecha límite, y Skribisto calcula el ritmo diario para que llegue a tiempo.
pace-start-planning = Empezar a planificar
pace-section-schedule = Calendario
pace-section-progress = Progreso
pace-advancement = Avance
pace-goal = Objetivo de palabras
pace-deadline = Fecha límite
pace-active = Ritmo activo
pace-day-mon = lun
pace-day-tue = mar
pace-day-wed = mié
pace-day-thu = jue
pace-day-fri = vie
pace-day-sat = sáb
pace-day-sun = dom
pace-card-written = palabras escritas
pace-card-of-goal = del objetivo
pace-card-rate = palabras / día de escritura
pace-card-days-left = días de escritura restantes
pace-card-streak = días seguidos
pace-card-ahead = palabras de adelanto sobre el ritmo
pace-card-behind = palabras de retraso sobre el ritmo
pace-charts-empty = Los gráficos de progreso aparecen aquí cuando se registra su recuento de palabras al guardar.
pace-chart-progression = Palabras escritas frente al objetivo
pace-chart-words-per-day = Palabras por día
pace-series-actual = Real
pace-series-target = Objetivo
pace-series-words-per-day = Palabras/día
pace-daily-target-line = Ritmo regular: { $count ->
    [one] { $count } palabra/día
   *[other] { $count } palabras/día
}
pace-section-holidays = Días libres
pace-section-milestones = Hitos
pace-holidays-none = No hay días libres. Cuentan todos los días programados.
pace-holiday-label = Nombre del día libre
pace-add-holiday = Añadir
pace-remove = Quitar
pace-milestones-none = Todavía no hay hitos. Defina uno en una parte o un capítulo desde el inspector.
full-chapter = Capítulo completo
full-part = Parte completa
full-book = Libro completo
full-synopsis = Sinopsis completa
rename = Cambiar nombre
set-label = Poner rótulo
insert-scene = Insertar escena
insert-chapter = Insertar capítulo
split-scene = Dividir la escena
move-up = Subir
move-down = Bajar
merge-with-previous = Fusionar con la anterior
move-to-trash = Mover a la papelera
rename-chapter = Cambiar el nombre del capítulo
add-scene = Añadir una escena nueva
add-chapter = Añadir un capítulo nuevo
new-scene-title = Nueva escena
placeholder-scene-name = Nombre de la escena…
menu-cut = Cortar
menu-copy = Copiar
menu-paste = Pegar
menu-paste-unformatted = Pegar sin formato
menu-select-all = Seleccionar todo
format-bold = Negrita
format-italic = Cursiva
format-underline = Subrayado
format-strikethrough = Tachado

binder = Archivador
no-work = No hay ningún proyecto cargado
no-recent-works = No hay proyectos recientes
switcher-open-section = Actualmente abierto
switcher-recent-section = Recientes
switcher-this-window = esta ventana
open-project-title = Abrir proyecto
open-project-question = ¿Cómo quiere abrir «{ $title }»?
open-in-new-window = Abrir en una ventana nueva
open-here = Abrir aquí

binder-all = Todos los archivadores
binder-show-all = Mostrar todos los archivadores
binder-new = Nuevo archivador…
binder-default-name = Nuevo archivador
binder-item-count = { $count ->
    [one] { $count } elemento
   *[other] { $count } elementos
}
binder-search-placeholder = Filtrar el esquema…
binder-search-scope = Buscar en todos los archivadores
binder-trash-confirm-title = ¿Mover el archivador a la papelera?
binder-trash-confirm-text = «{ $name }» y todos sus elementos se moverán a la papelera.

dialog-rename = Cambiar nombre
dialog-new-binder = Nuevo archivador
dialog-set-label = Poner rótulo
dialog-new-scene = Nueva escena
close-work-question = ¿Guardar los cambios antes de cerrar el proyecto?
quit-question = ¿Guardar los cambios antes de salir?
quit-save-work-question = ¿Guardar los cambios de { $title } antes de salir?
unsaved-changes = Este proyecto tiene cambios sin guardar.
new-work-unsaved-question = ¿Guardar los cambios antes de crear un proyecto nuevo?
open-work-unsaved-question = ¿Guardar los cambios antes de abrir otro proyecto?
switch-backup-discard-title = ¿Descartar los cambios de esta copia de seguridad?
switch-backup-discard-text = Los cambios en una copia de seguridad no se pueden guardar en ella. Use Guardar como o Restaurar para conservarlos, o descártelos y abra el otro proyecto.
switch-save-failed = No se pudo guardar el proyecto, así que no se ha sustituido: { $error }
switch-save-not-started = No se pudo guardar el proyecto, así que no se ha sustituido.
close-save-failed = No se pudo guardar el proyecto, así que no se ha cerrado: { $error }
close-save-not-started = No se pudo guardar el proyecto, así que no se ha cerrado.
save-not-started = No se pudo guardar el proyecto.

could-not-open-work = No se pudo abrir «{ $file }»
could-not-open-work-too-new = «{ $file }» necesita una versión más reciente de Skribisto
could-not-open-work-too-new-detail = Guardado con el formato { $written_by } de Skribisto; para abrirlo hace falta el formato { $requires } o posterior, y esta versión admite hasta el formato { $supported }. Actualice Skribisto para abrirlo.
could-not-open-work-importing = Una importación está escribiendo «{ $file }»
could-not-open-work-importing-detail = Una importación está escribiendo sobre este proyecto y no ha terminado. Ábralo cuando termine la importación: si lo abre ahora, mostraría el proyecto que la importación está sustituyendo, y su siguiente guardado volvería a escribir ese proyecto sobre la importación.
could-not-open-example = No se pudo abrir el ejemplo: { $error }
could-not-open-link = No se pudo abrir { $url }: { $error }
could-not-create-work = No se pudo crear el proyecto: { $error }
saving-as-file = Guardando como { $target }…
saving-as-folder = Guardando como { $target }/…
saved-as = Guardado en { $target }
save-error = No se pudo guardar: { $error }
save-flush-failed = El guardado se completó, pero estos documentos no se le pudieron entregar y siguen sin guardar: { $titles } ({ $error })
backup-error = No se pudo hacer la copia de seguridad: { $error }
backing-up = Haciendo la copia de seguridad…
backup-nothing-open = No hay ningún proyecto abierto del que hacer copia de seguridad.
backup-already-running = Ya está en marcha una copia de seguridad.
backup-complete = Copia de seguridad completada ({ $ok } guardadas, { $skipped } ya al día)
backup-partial = Copia de seguridad terminada: { $ok } guardadas, { $failed ->
    [one] { $failed } destino ha fallado
   *[other] { $failed } destinos han fallado
}
backup-no-destination-title = No hay ubicación de copia de seguridad disponible
backup-no-destination-text = No se puede acceder a ninguno de los destinos de copia de seguridad configurados (por ejemplo, puede que una unidad externa esté desconectada). Conéctela y vuelva a intentarlo, o salga sin hacer copia de seguridad.
backup-failed-close-title = No se pudo guardar la copia de seguridad
backup-failed-close-text = No se pudo escribir ninguna copia de seguridad antes de cerrar. Todos los destinos han fallado (puede que la unidad se haya retirado, o que esté llena o protegida contra escritura). Corríjalo y vuelva a intentarlo, o salga sin hacer copia de seguridad.
backup-failed-close-error-text = No se pudo escribir ninguna copia de seguridad antes de cerrar: { $error }. Corríjalo y vuelva a intentarlo, o salga sin hacer copia de seguridad.
backup-no-result = La copia de seguridad ha terminado sin indicar si ha hecho una copia. Revise la lista de copias de seguridad para ver si hay una nueva.

backup-choice-title = Archivo de copia de seguridad
backup-choice-heading = Ha abierto una copia de seguridad
backup-choice-subtitle = Es una copia de seguridad de un proyecto en un momento dado.
backup-choice-subtitle-dated = Copia de seguridad hecha el { $date }.
backup-choice-body = Puede abrirla y editarla libremente, pero los cambios solo se pueden conservar con Guardar como. El archivo original del proyecto no se toca. También puede restaurar este proyecto exactamente a esta copia de seguridad.
backup-choice-open = Abrir la copia de seguridad
backup-choice-restore = Restaurar este proyecto a este punto…
backup-choice-not-a-backup = No, abrirlo normalmente
backup-banner-title = Copia de seguridad: los cambios no se pueden guardar aquí
backup-banner-description = Use Guardar como para conservar sus cambios en un archivo nuevo, o Restaurar para sustituir el proyecto original por esta copia de seguridad.
backup-banner-restore = Restaurar…
backup-banner-save-as = Guardar como…
backup-restore-original-missing = No se encuentra el proyecto original sobre el que restaurar. Use Guardar como para conservar esta copia de seguridad como un proyecto nuevo.
backup-restore-close-elsewhere-title = Proyecto abierto en otra ventana
backup-restore-close-elsewhere-text = El proyecto sobre el que está restaurando está abierto en otra ventana. Ciérrelo allí primero y vuelva a intentarlo.
backup-restore-focus-window = Ir a esa ventana
backup-restore-confirm-title = ¿Restaurar esta copia de seguridad?
backup-restore-confirm-text = La versión actual del proyecto se copiará aparte como copia de seguridad de precaución antes de sustituirla por esta.
backup-restore-confirm-ok = Restaurar
backup-restore-error = No se pudo restaurar: { $error }
backup-restored-ok = Proyecto restaurado.
backup-restored-with-safety = Proyecto restaurado. Su versión anterior se guardó en { $path }.
close-backup-discard-title = ¿Descartar los cambios de esta copia de seguridad?
close-backup-discard-text = Los cambios en una copia de seguridad no se pueden guardar en ella. Use Guardar como para conservarlos, o descártelos y cierre.
quit-backup-discard-title = ¿Descartar los cambios y salir?
quit-backup-discard-work-question = ¿Descartar los cambios de { $title } y salir?
quit-backup-discard-text = Los cambios en una copia de seguridad no se pueden guardar en ella. Use Guardar como para conservarlos, o descártelos y salga.
backup-nudge-text = No hay copias de seguridad configuradas para este proyecto.
backup-nudge-action = Configurar copias de seguridad…

menu-backups-list = Co&pias de seguridad…
backups-title = Copias de seguridad
backups-loading = Buscando copias de seguridad…
backups-empty = Todavía no hay copias de seguridad de este proyecto.
backups-open = Abrir
backups-reveal = Mostrar ubicación
backups-delete = Eliminar esta copia de seguridad
backups-delete-confirm-title = ¿Eliminar esta copia de seguridad?
backups-delete-confirm-text = «{ $name }» se eliminará definitivamente. No se puede deshacer.
backups-delete-error = No se pudo eliminar la copia de seguridad: { $error }
backups-refresh = Actualizar
backups-close = Cerrar

settings-page-backup = Copias de seguridad predeterminadas
settings-page-work-backup = Copias de seguridad
settings-backup-general-title = Configuración predeterminada de copias de seguridad
settings-backup-work-title = Copias de seguridad de este proyecto
settings-backup-inherit = Usar la configuración general de copias de seguridad
settings-backup-inheriting = Este proyecto usa la configuración general de copias de seguridad.
settings-backup-none-hint = No hay copias de seguridad automáticas configuradas (todos los activadores están desactivados).
settings-backup-last = Última copia de seguridad: { $date }
settings-backup-last-never = Todavía no hay copias de seguridad.
settings-backup-open-list = Abrir la lista de copias de seguridad…
settings-backup-triggers = Cuándo hacer copia de seguridad
settings-backup-on-close = Hacer copia de seguridad al cerrar el proyecto
settings-backup-on-open = Hacer copia de seguridad al abrir el proyecto
settings-backup-interval = Hacer copia de seguridad periódicamente, cada
settings-backup-destinations = Destinos de las copias de seguridad
settings-backup-default-location = Ubicación predeterminada
settings-backup-usage-since = { $count ->
    [one] { $count } copia de seguridad · { $size } · más antigua { $oldest }
   *[other] { $count } copias de seguridad · { $size } · más antigua { $oldest }
}

settings-backup-usage = { $count ->
    [one] { $count } copia de seguridad · { $size }
   *[other] { $count } copias de seguridad · { $size }
}

settings-backup-usage-empty = Todavía no se guarda nada aquí
settings-backup-usage-measuring = Midiendo…
settings-backup-reveal-root = Mostrar carpeta
settings-backup-dest-none = No hay destinos. Las copias de seguridad se guardan junto al proyecto.
settings-backup-dest-remove = Quitar
settings-backup-dest-add = Añadir carpeta…
settings-backup-dest-refresh = Actualizar
settings-backup-retention = Cuántas conservar
settings-backup-retention-tiered = Por niveles
settings-backup-retention-keep-n = Conservar las N últimas
settings-backup-retention-tip = Cómo se eliminan las copias de seguridad antiguas.
settings-backup-retention-tip-more = Por niveles conserva una copia de seguridad por hora durante un día, una por día durante una semana, una por semana durante un mes y una por mes a partir de ahí, de modo que el historial reciente sigue siendo denso y el antiguo se va aclarando. «Conservar las N últimas» simplemente conserva las N copias más recientes. En ambos modos, las copias más recientes (el mínimo indicado abajo) se conservan siempre.
settings-backup-gfs-hourly = Por hora (últimas 24 h)
settings-backup-gfs-daily = Diario (última semana)
settings-backup-gfs-weekly = Semanal (último mes)
settings-backup-gfs-monthly = Mensual
settings-backup-keep-n = Número que conservar
settings-backup-min-keep = Conservar siempre al menos
settings-backup-dedup = Omitir la copia de seguridad si no ha cambiado nada

new-work-title = Nuevo proyecto
new-work-close = Cerrar
new-work-name = Nombre del proyecto
new-work-name-placeholder = Sin título
new-work-author = Autor
new-work-author-placeholder = Opcional
new-work-format = Formato
new-work-single-file = Archivo único
new-work-bundle = Carpeta
new-work-convert-later = Puede cambiar de formato más adelante.
new-work-location = Ubicación
new-work-will-create = Se creará
new-work-language = Idioma predeterminado
new-work-language-hint = Se aplica a los textos nuevos y a la revisión ortográfica. Cada texto puede cambiarse a otro idioma.
new-work-template = Plantilla
new-work-template-none = Ninguna
new-work-template-empty-novel = Novela vacía
new-work-template-light-novel = Novela ligera
new-work-template-novel = Novela
new-work-template-novel-in-parts = Novela en partes
new-work-template-notebook = Cuaderno
new-work-cancel = Cancelar
new-work-create = Crear proyecto
new-work-step-details = Detalles
new-work-step-language = Idioma y estructura
new-work-step-template = Plantilla
new-work-back = Atrás
new-work-next = Siguiente
new-work-documents-title = Nuevo proyecto a partir de documentos
new-work-step-import = Importar
new-work-create-and-import = Crear e importar…
new-work-documents-next-title = Después vienen sus documentos
new-work-documents-next-body = Al crear el proyecto se abre sobre él el asistente de importación, donde elige los documentos, revisa la estructura que Skribisto lee en ellos y decide dónde se colocan. No se escribe nada en el proyecto hasta que lo confirme allí.
new-work-documents-no-template = Este proyecto empieza vacío a propósito: sin plantilla, para que los documentos importados sean lo único que contenga.
new-work-documents-chapter-scene-hint = Se aplica a los capítulos que crea la importación. Puede cambiarlo más adelante en la configuración del proyecto.
new-work-single-file-desc = Un archivo .skrib (zip). Portátil y fácil de copiar.
new-work-bundle-desc = Una carpeta que contiene todos los textos y recursos. Más cómoda para el control de versiones.
new-work-template-none-count = archivador vacío
new-work-template-empty-novel-count = 1 capítulo
new-work-template-light-novel-count = 15 capítulos
new-work-template-novel-count = 20 capítulos
new-work-template-novel-in-parts-count = 3 partes, 24 capítulos
new-work-template-notebook-count = notas de forma libre
new-work-chapter-scene = Capítulos planos
new-work-chapter-scene-tip = Cada capítulo es un único elemento en el que escribe directamente, sin escenas debajo. Déjelo desactivado para la disposición clásica, en la que un capítulo es una carpeta: también escribe directamente en ella, pero además puede contener escenas.
new-work-chapter-scene-tip-more = Escribe en un capítulo de las dos maneras. La única diferencia es si puede *contener* escenas. El árbol del archivador de Skribisto es solo organizativo, así que ambas disposiciones se compilan en el mismo libro, puede mezclarlas libremente, y Convertir en cambia un capítulo de una a otra sin perder una sola palabra.
new-work-name-required = Escriba un nombre para el proyecto
new-work-name-invalid = Este nombre no tiene caracteres utilizables
new-work-location-required = Elija una ubicación
new-work-location-missing = Esta carpeta no existe
new-work-location-not-folder = Esta ruta no es una carpeta
new-work-location-readonly = No se puede escribir en esta carpeta
new-work-target-exists = Ya hay un proyecto con este nombre en esta carpeta. Al crear el proyecto se le preguntará antes de sustituirlo
new-work-target-is-folder = Ya hay una carpeta con este nombre en esta ubicación. Elija otro nombre
new-work-overwrite-title = ¿Reemplazar el proyecto existente?
new-work-overwrite-text = «{ $name }» ya existe en esta carpeta. ¿Sustituirlo por el proyecto nuevo?
new-work-target-open-title = Este proyecto está abierto
new-work-target-open-text = «{ $name }» está abierto en Skribisto. Crear un proyecto nuevo en su lugar sustituiría el proyecto en el que está trabajando, y su siguiente guardado volvería a escribir ese proyecto sobre el nuevo. Elija otro nombre u otra carpeta, o cierre antes el proyecto.
target-importing-title = Una importación está escribiendo este proyecto
target-importing-text = Una importación sigue escribiendo «{ $name }». Espere a que termine o elija otro nombre.
backup-restore-importing-text = Una importación sigue escribiendo «{ $name }», el proyecto sobre el que se restauraría esta copia de seguridad. Restaure la copia de seguridad cuando termine la importación.

new-work-manuscript = Manuscrito
new-work-notes = Notas
new-work-research = Investigación
new-work-notebook = Cuaderno
new-work-scene = Escena
new-work-note = Nota
new-work-front-matter = Páginas preliminares
new-work-back-matter = Páginas finales
new-work-paratext = Estructura del libro
new-work-paratext-none = Sin estructura
new-work-paratext-hint = Las páginas preliminares y finales con las que una tradición abre y cierra un libro. Después puede mover, renombrar o eliminar cualquiera de ellas.
new-work-tags = Etiquetas
new-work-tags-none = Sin etiquetas
new-work-tags-hint = Una paleta inicial para etiquetar personajes, lugares y lo demás. Es opcional, y cada etiqueta se puede renombrar, cambiar de color o eliminar después.
new-work-note-templates = Plantillas de notas
new-work-note-templates-none = Sin plantillas
new-work-note-templates-hint = La forma con la que empieza una nota de la biblia narrativa. Es opcional, y cada plantilla se puede editar o eliminar después.
new-work-characters = Personajes
new-work-places = Lugares

import-plume-title = Importar un proyecto de Plume Creator
import-plume-source = Proyecto de Plume
import-plume-source-hint = Elija un archivo .plume o .plume_backup (de cualquier versión de Plume Creator).
import-plume-trash-warning = ⚠ Los elementos en la papelera o eliminados no se migran.
import-plume-source-required = Elija un archivo de proyecto de Plume
import-plume-source-missing = Este archivo no existe
import-plume-source-not-file = Esta ruta no es un archivo
import-plume-manuscript-binder = Manuscrito
import-plume-story-bible-binder = Biblia narrativa
import-plume-notes-of = { $name } (notas)
import-plume-content-of = { $name } (texto)
import-plume-progress-title = Importando el proyecto de Plume…
import-plume-done = { $imported ->
    [one] { $imported } elemento importado.
   *[other] { $imported } elementos importados.
} { $skipped ->
    [one] { $skipped } elemento de la papelera no se ha migrado.
   *[other] { $skipped } elementos de la papelera no se han migrado.
}
import-plume-nested-too-deep = El proyecto no se ha importado: su parte «{ $part }» está anidada a más de { $limit } niveles de profundidad. Ningún proyecto de Plume Creator se acerca a eso, y leerlo podría bloquear Skribisto. No se ha escrito nada.
import-plume-declares-entities = El proyecto no se ha importado: su parte «{ $part }» declara entidades XML, cosa que no hace ningún proyecto de Plume Creator. Expandirlas podría requerir más memoria de la que tiene el equipo y bloquear Skribisto. No se ha escrito nada.
import-plume-no-outline = Este archivo es un archivo comprimido, pero no un proyecto de Plume Creator: no tiene esquema. No se ha escrito nada.
import-plume-empty-outline = El archivo de esquema de este proyecto de Plume Creator está vacío, así que no hay nada que importar. No se ha escrito nada.
# $tag is the name of the document the part turned out to be.
import-plume-wrong-document = Esto no es un proyecto de Plume Creator: su parte «{ $part }» es un documento «{ $tag }». No se ha escrito nada.
# $version is the part's own version, $supported the newest this importer reads.
import-plume-too-new = La parte «{ $part }» de este proyecto la escribió una versión de Plume Creator (la { $version }) más reciente de lo que Skribisto puede leer (hasta la { $supported }). No se ha escrito nada.

import-manuskript-title = Importar un proyecto de Manuskript
import-manuskript-source = Proyecto de Manuskript
import-manuskript-source-hint = Elija el archivo .msk o la carpeta del proyecto (de cualquier versión de Manuskript).
import-manuskript-source-file = Elegir archivo…
import-manuskript-source-folder = Elegir carpeta…
import-manuskript-source-required = Elija un proyecto de Manuskript
import-manuskript-source-missing = Este archivo o carpeta no existe
import-manuskript-manuscript-binder = Manuscrito
import-manuskript-story-bible-binder = Biblia narrativa
import-manuskript-characters-group = Personajes
import-manuskript-world-group = Mundo
import-manuskript-plots-group = Tramas
import-manuskript-project-info-note = Información del proyecto
import-manuskript-summary-note = Resumen
import-manuskript-importance-minor = Menor
import-manuskript-importance-secondary = Secundario
import-manuskript-importance-main = Principal
import-manuskript-notes-of = { $title } (notas)
import-manuskript-progress-title = Importando el proyecto de Manuskript…
import-manuskript-done = { $imported ->
    [one] { $imported } elemento importado.
   *[other] { $imported } elementos importados.
} { $revisions ->
    [0] { "" }
    [one] Con él llegó { $revisions } versión anterior.
   *[other] Con ellos llegaron { $revisions } versiones anteriores.
}
import-manuskript-nested-too-deep = El proyecto no se ha importado: su archivo «{ $part }» está anidado a más de { $limit } niveles de profundidad. Ningún proyecto de Manuskript se acerca a eso, y leerlo podría bloquear Skribisto. No se ha escrito nada.
import-manuskript-folders-too-deep = El proyecto no se ha importado: uno de sus archivos está a más de { $limit } carpetas de profundidad. Ningún proyecto de Manuskript se acerca a eso, y leerlo podría bloquear Skribisto. No se ha escrito nada.
import-manuskript-declares-entities = El proyecto no se ha importado: su archivo «{ $part }» declara entidades XML, cosa que no hace ningún proyecto de Manuskript. Expandirlas podría requerir más memoria de la que tiene el equipo y bloquear Skribisto. No se ha escrito nada.
import-manuskript-not-a-project-file = «{ $file }» no es ni un proyecto de Manuskript comprimido en un archivo ni el pequeño archivo .msk que apunta a la carpeta de un proyecto. No se ha escrito nada.
import-manuskript-folder-missing = «{ $stub }» apunta a una carpeta de proyecto llamada «{ $folder }» situada a su lado, y esa carpeta no está. Un proyecto de Manuskript en modo carpeta consta de los dos, y tienen que viajar juntos. No se ha escrito nada.
import-manuskript-not-a-project = Esto no parece un proyecto de Manuskript: no tiene la marca de Manuskript ni esquema. No se ha escrito nada.
import-manuskript-format-too-new = Este proyecto está en el formato { $version } de Manuskript, más reciente de lo que Skribisto puede leer (hasta el { $supported }). No se ha escrito nada.
import-manuskript-too-many-files = El proyecto no se ha importado: contiene más de { $limit } archivos, muchísimos más que cualquier proyecto de Manuskript. No se ha escrito nada.
import-manuskript-too-large = El proyecto no se ha importado: ocupa más de los { $limit } MB que Skribisto lee de un proyecto de Manuskript. No se ha escrito nada.
project-import-close = Cerrar
project-import-location = Carpeta de destino
# The folder dialog's title.
project-import-location-title = Elija una carpeta de destino
project-import-name = Nombre del archivo
project-import-name-placeholder = Nombre del proyecto
project-import-will-create = Se creará
project-import-cancel = Cancelar
project-import-import = Importar
# Field validation
project-import-location-required = Elija una carpeta de destino
project-import-location-missing = Esta carpeta no existe
project-import-location-not-folder = Esta ruta no es una carpeta
project-import-location-readonly = No se puede escribir en esta carpeta
project-import-name-required = Escriba un nombre de archivo
project-import-name-exists = Ya existe un archivo con este nombre aquí. La importación pedirá confirmación antes de sobrescribirlo
project-import-name-is-folder = Aquí ya hay una carpeta con este nombre, quizá un proyecto guardado como carpeta. Una importación no puede sustituirla: elija otro nombre
# Overwrite confirmation
project-import-overwrite-title = ¿Reemplazar el archivo existente?
project-import-overwrite-text = «{ $name }» ya existe. ¿Sustituirlo por el proyecto importado?
# Projects are imported one at a time, whichever importer asks.
project-import-busy-title = Se está importando otro proyecto
project-import-busy-text = Los proyectos se importan de uno en uno. Espere a que termine la importación en curso, o cancélela, y vuelva a empezar esta.
# Spoken by a screen reader once per phase, never shown: the toast's title, then
# the phase's name.
project-import-progress-announcement = { $title } { $phase }
# The notice once Cancel is pressed, until the converter has stopped: it may be
# finishing a file it cannot leave half written.
project-import-stopping = Deteniendo la importación…
project-import-cancelled = Importación cancelada
project-import-open-now = Abrir ahora
project-import-details = Detalles
# The import's report: what the writer should know about what came across.
project-import-report = { $count ->
    [one] 1 cosa que debe saber sobre esta importación
   *[other] { $count } cosas que debe saber sobre esta importación
}
project-import-report-title = Acerca de esta importación
# Error toast: a short reason in the body, the full technical chain behind Details.
project-import-error-title = No se pudo importar el proyecto
# Why a project was not imported, whichever importer read it. A $file is a file
# name, a $folder a full path (it is the folder the writer has to act on), a
# $reason what the system or a parser said, a $limit a number.
project-import-failed-cancelled = Se ha cancelado la importación. No se ha escrito nada.
project-import-failed-target-exists = «{ $file }» ya existe, y la importación no tenía permiso para sustituirlo. No se ha escrito nada.
project-import-failed-target-folder = «{ $file }» es una carpeta, quizá un proyecto guardado como carpeta, y una importación no puede sustituirla. Elija otro nombre. No se ha escrito nada.
project-import-failed-not-written = No se pudo escribir «{ $file }» en «{ $folder }» ({ $reason }). No se ha escrito nada.
project-import-failed-disk-full = No queda espacio suficiente en «{ $folder }» para escribir «{ $file }». Libere espacio allí y vuelva a importar. No se ha escrito nada.
project-import-failed-source-missing = «{ $file }» ya no está. No se ha escrito nada.
project-import-failed-unreadable = Skribisto no pudo leer «{ $file }» ({ $reason }). No se ha escrito nada.
project-import-failed-archive-damaged = «{ $file }» es un archivo comprimido que no se pudo leer, probablemente porque está dañado ({ $reason }). No se ha escrito nada.
project-import-failed-archive-entries = El proyecto no se ha importado: su archivo comprimido enumera { $entries ->
    [one] { $entries } archivo
   *[other] { $entries } archivos
}, más de los { $limit } que puede contener un proyecto, cosa que solo hace un archivo creado para agotar la memoria. No se ha escrito nada.
project-import-failed-archive-member = El proyecto no se ha importado: su parte «{ $part }» se descomprime en más de { $limit } MB, cosa que solo hace un archivo creado para agotar la memoria. No se ha escrito nada.
project-import-failed-archive-size = El proyecto no se ha importado: se descomprime en más de { $limit } MB, cosa que solo hace un archivo creado para agotar la memoria. No se ha escrito nada.
project-import-failed-nested = El proyecto no se ha importado: su parte «{ $part }» está anidada a más de { $limit } niveles de profundidad, y leerla podría bloquear Skribisto. No se ha escrito nada.
project-import-failed-entities = El proyecto no se ha importado: su parte «{ $part }» declara entidades XML. Expandirlas podría requerir más memoria de la que tiene el equipo y bloquear Skribisto. No se ha escrito nada.
project-import-failed-folders = El proyecto no se ha importado: uno de sus archivos está a más de { $limit } carpetas de profundidad, y leerlo podría bloquear Skribisto. No se ha escrito nada.
project-import-failed-malformed = El proyecto no se ha importado: su parte «{ $part }» está dañada y no se puede leer ({ $reason }). No se ha escrito nada.
# A Scrivener project the importer cannot bring across (a Scrivener 1 for Mac project, a
# damaged binder file). Why is the Scrivener importer's own to say; Details holds its key.
project-import-failed-scrivener-refused = Skribisto no puede importar este proyecto de Scrivener. No se ha escrito nada.
# The Scrivener project asks something its review did not answer: a project or a binder
# file appeared in it since.
project-import-failed-scrivener-question = El proyecto de Scrivener plantea una pregunta que su revisión no respondió. Vuelva a empezar su importación, responda a la pregunta y luego impórtelo. No se ha escrito nada.
# The Scrivener project raises a question the writer has to accept before it imports.
project-import-failed-scrivener-not-accepted = Antes de poder importarse, el proyecto de Scrivener necesita que marque una casilla en el paso Proyecto, y no se marcó. Vuelva a empezar su importación, marque la casilla y luego impórtelo. No se ha escrito nada.
# The Scrivener project changed after the writer reviewed it.
project-import-failed-scrivener-changed = El proyecto de Scrivener ha cambiado después de que usted lo revisara. Vuelva a empezar su importación, revíselo y luego impórtelo. No se ha escrito nada.
# An error nothing above accounts for; $detail is its technical description.
project-import-failed-internal = La importación se detuvo por un error que Skribisto no esperaba: { $detail }

import-target-open-title = Este proyecto está abierto
import-target-open-text = «{ $name }» está abierto en Skribisto. Importar sobre él sustituiría el proyecto en el que está trabajando, y su siguiente guardado volvería a escribir ese proyecto sobre la importación. Elija otro nombre de archivo, o cierre antes el proyecto.
import-warnings-announcement = { $result } { $notice }.
# Quitting while an import runs. $name is the file the import is writing.
quit-import-running-title = Todavía hay una importación en curso
quit-import-running-text = Skribisto sigue importando «{ $name }». Si sale ahora, la importación se detiene: no se escribe nada, y el proyecto desde el que importa queda como estaba.
quit-import-stop = Detener la importación y salir

## Import a Scrivener project: the wizard
import-scrivener-title = Importar un proyecto de Scrivener
import-scrivener-step-project = Proyecto
import-scrivener-step-review = Revisión
import-scrivener-step-save = Guardar
import-scrivener-back = Atrás
import-scrivener-next = Siguiente
import-scrivener-import = Importar
import-scrivener-cancel = Cancelar
import-scrivener-help = Ayuda

## Project step: choosing the project
import-scrivener-drop-title = Suelte aquí un proyecto de Scrivener
import-scrivener-drop-hint = Un proyecto .scriv, el archivo .scrivx que contiene, una copia de seguridad .zip o un .scrivtemplate
import-scrivener-source = Proyecto de Scrivener
import-scrivener-source-placeholder = Ruta del proyecto
import-scrivener-choose-file = Elegir archivo…
import-scrivener-choose-folder = Elegir carpeta…
# Windows and Linux show a .scriv project as an ordinary folder.
import-scrivener-source-hint = Elija la carpeta .scriv del proyecto, o ábrala y elija el archivo .scrivx que contiene. También sirve una copia de seguridad .zip.
# macOS shows a .scriv project as a single file when Scrivener is installed, as a folder when it is not.
import-scrivener-source-hint-mac = Si el proyecto aparece como un único archivo, use «Elegir archivo…». Si aparece como una carpeta, use «Elegir carpeta…». También sirve una copia de seguridad .zip.
# File-dialog titles and filter label, resolved when the dialog opens.
import-scrivener-dialog-file-title = Elija un proyecto de Scrivener
import-scrivener-dialog-folder-title = Elija la carpeta de un proyecto de Scrivener
import-scrivener-dialog-filter = Proyecto de Scrivener
import-scrivener-source-missing = No hay nada en esta ruta
# A path whose name is not Unicode: the field shows it with its unreadable letters replaced.
import-scrivener-source-unreadable-name = Este nombre no se puede leer. Cambie el nombre del proyecto en su gestor de archivos y vuelva a elegirlo.
import-scrivener-empty-hint = Elija un proyecto para ver lo que Skribisto encuentra en él.
import-scrivener-reading = Leyendo el proyecto…
import-scrivener-stop-reading = Dejar de leer
import-scrivener-read-failed = No se pudo leer el proyecto
import-scrivener-read-failed-body = Algo ha fallado al leerlo. Elija «Volver a leer el proyecto» para intentarlo otra vez, o «Detalles» para ver qué ha fallado.
import-scrivener-details = Detalles
import-scrivener-check-again = Volver a leer el proyecto
# A refusal this version does not know how to word.
import-scrivener-refusal-unknown-title = Este proyecto no se puede importar
import-scrivener-refusal-unknown-body = Skribisto no sabe cómo traerlo. Los detalles indican el motivo.

## Project step: the project card
import-scrivener-made-with-mac = Guardado por última vez con Scrivener { $version } para macOS
import-scrivener-made-with-windows = Guardado por última vez con Scrivener { $version } para Windows
import-scrivener-made-with-linux = Guardado por última vez con Scrivener { $version } para Linux
import-scrivener-made-with-ios = Guardado por última vez con Scrivener { $version } para iPhone y iPad
import-scrivener-made-with-version = Guardado por última vez con Scrivener { $version }
import-scrivener-made-with-unknown = Proyecto de Scrivener, formato { $format }
import-scrivener-saved-on = Guardado el { $date } en «{ $device }»
import-scrivener-saved-on-no-device = Guardado el { $date }
import-scrivener-card-counts = { $documents ->
    [one] 1 documento
   *[other] { $documents } documentos
}, { $manuscript } en el manuscrito

## Review step
import-scrivener-review-hint = Así es como llegará su archivador. No se escribe nada hasta que importe, y después muchos elementos pueden cambiar de tipo en el archivador, con Convertir en.
import-scrivener-choices-structure = Niveles
import-scrivener-top-level = Carpetas de primer nivel
import-scrivener-choices-text = Texto
import-scrivener-choice-containers = Carpetas del nivel { $depth }
import-scrivener-choice-texts = Documentos del nivel { $depth }
import-scrivener-choice-count = { $count ->
    [one] 1 elemento
   *[other] { $count } elementos
}
import-scrivener-choice-count-example = { $count ->
    [1] 1 elemento: «{ $example }»
   *[other] { $count } elementos, como «{ $example }»
}
import-scrivener-rung-part = Partes
import-scrivener-rung-chapter = Capítulos
import-scrivener-rung-plain = Carpetas simples
import-scrivener-texts-scenes = Escenas
import-scrivener-texts-chapters = Capítulos
import-scrivener-choice-books = Libros del manuscrito
import-scrivener-books-one = Un solo libro
import-scrivener-books-several = Un libro por carpeta
import-scrivener-choice-split = { $count ->
    [1] Dividir «{ $title }» por su encabezado
   *[other] Dividir «{ $title }» por sus { $count } encabezados
}
import-scrivener-root-items = { $count ->
    [one] 1 elemento
   *[other] { $count } elementos
}
import-scrivener-root-items-manuscript = { $count ->
    [one] 1 elemento. Esta carpeta parece un manuscrito.
   *[other] { $count } elementos. Esta carpeta parece un manuscrito.
}
import-scrivener-dest-notes = Notas
import-scrivener-dest-story-bible = Biblia narrativa
import-scrivener-dest-manuscript = Otro libro
import-scrivener-dest-leave-out = Dejar fuera
import-scrivener-choice-trash = { $count ->
    [one] Traer la papelera (1 elemento)
   *[other] Traer la papelera ({ $count } elementos)
}
import-scrivener-trash-detail = { $count ->
    [0] Ninguno de ellos contiene texto.
    [one] 1 de ellos contiene texto.
   *[other] { $count } de ellos contienen texto.
}
import-scrivener-choice-blank-lines = Las líneas en blanco se convierten en separadores de escena
import-scrivener-choice-ornaments = Las líneas de adornos se convierten en separadores de escena
import-scrivener-preview-label = Estructura del proyecto nuevo
import-scrivener-col-title = Título
import-scrivener-col-becomes = Se convierte en
import-scrivener-col-exports = En las exportaciones
import-scrivener-in-exports-yes = Sí
import-scrivener-in-exports-no = No
# What a row the binder has no creation command for becomes.
import-scrivener-becomes-item = Elemento
import-scrivener-becomes-trashed = En la papelera
import-scrivener-before-title = Antes de importar
import-scrivener-review-error-unreadable = No se pudo preparar la revisión. Vuelva a elegir el proyecto.
import-scrivener-review-error-choice = Esta opción no encaja con el proyecto. Vuelva a elegir el proyecto.
import-scrivener-review-error-unmappable = Con esta opción, el archivador no se podría traer sin perder algo. Elija otra opción o informe del problema.

## Save step
import-scrivener-save-hint = La importación crea un proyecto de Skribisto nuevo. Su proyecto de Scrivener solo se lee; nunca se modifica.
import-scrivener-title-field = Título
import-scrivener-author = Autor
import-scrivener-language = Idioma
import-scrivener-location-inside-project = Elija una carpeta fuera del proyecto de Scrivener. Scrivener solo guarda ahí sus propios archivos.
import-scrivener-copy-research-label = Copiar los archivos de investigación junto al proyecto nuevo
import-scrivener-copy-research = { $count ->
    [one] 1 archivo de investigación que Skribisto no puede abrir, como un PDF o un sonido, va a una carpeta junto al proyecto. Si desmarca la casilla, su nota indica dónde está en su proyecto de Scrivener.
   *[other] { $count } archivos de investigación que Skribisto no puede abrir, como PDF o sonidos, van a una carpeta junto al proyecto. Si desmarca la casilla, sus notas indican dónde están en su proyecto de Scrivener.
}
# The file name proposed when the project's name holds nothing a file name can.
import-scrivener-default-name = Proyecto de Scrivener

## Progress and result
import-scrivener-progress-title = Importando el proyecto de Scrivener…
import-scrivener-done-title = Proyecto de Scrivener importado
# How many rows the new project holds: the documents and folders that came across, and
# the rows the import made for them (a document's notes, the project notes).
import-scrivener-done = { $items ->
    [one] El proyecto nuevo contiene 1 elemento.
   *[other] El proyecto nuevo contiene { $items } elementos.
}
# The same, with how many kinds of thing could not be read or need checking.
import-scrivener-done-check = { $items ->
    [one] El proyecto nuevo contiene 1 elemento.
   *[other] El proyecto nuevo contiene { $items } elementos.
} { $findings ->
    [one] 1 cosa requiere su atención.
   *[other] { $findings } cosas requieren su atención.
}
import-scrivener-review-again = Revisar la importación
## The import's report (the dialog See report opens)
import-scrivener-see-report = Ver informe
import-scrivener-report-from = Leído de { $path }
import-scrivener-report-to = Escrito en { $path }
import-scrivener-report-came-across = Lo que ha llegado
import-scrivener-report-nothing-counted = No se ha contado nada.
import-scrivener-report-items = { $count ->
    [one] 1 elemento
   *[other] { $count } elementos
}
import-scrivener-report-words = { $count ->
    [one] 1 palabra
   *[other] { $count } palabras
}
import-scrivener-report-synopses = { $count ->
    [one] 1 sinopsis
   *[other] { $count } sinopsis
}
import-scrivener-report-document-notes = { $count ->
    [one] las notas de 1 documento
   *[other] las notas de { $count } documentos
}
import-scrivener-report-comments = { $count ->
    [one] 1 comentario
   *[other] { $count } comentarios
}
import-scrivener-report-footnotes = { $count ->
    [one] 1 nota al pie
   *[other] { $count } notas al pie
}
import-scrivener-report-pictures = { $count ->
    [one] 1 imagen
   *[other] { $count } imágenes
}
import-scrivener-report-templates = { $count ->
    [one] 1 plantilla de notas
   *[other] { $count } plantillas de notas
}
import-scrivener-report-tags = { $count ->
    [one] 1 etiqueta
   *[other] { $count } etiquetas
}
import-scrivener-report-statuses = { $count ->
    [one] 1 estado
   *[other] { $count } estados
}
import-scrivener-report-trashed = { $count ->
    [one] 1 elemento en la papelera
   *[other] { $count } elementos en la papelera
}
import-scrivener-report-research-files = { $count ->
    [one] 1 archivo de investigación
   *[other] { $count } archivos de investigación
}
import-scrivener-report-choices = Lo que eligió
import-scrivener-report-accepted-mobile = Importó el proyecto sin los cambios hechos en Scrivener para iPhone o iPad que todavía no estaban en él.
import-scrivener-report-accepted-no-text = Importó esta copia con sus documentos vacíos, ya que no contenía nada de su texto.
import-scrivener-report-choice = { $subject }: { $answer }
import-scrivener-report-yes = sí
import-scrivener-report-no = no
import-scrivener-report-research-copied = Los archivos de investigación que Skribisto no puede abrir se han copiado en «{ $folder }», junto al proyecto nuevo.
import-scrivener-report-research-not-copied = Los archivos de investigación que Skribisto no puede abrir no se han copiado. Sus notas indican dónde están en su proyecto de Scrivener.
import-scrivener-report-research-copied-except = { $count ->
    [one] Los archivos de investigación que Skribisto no puede abrir se han copiado en «{ $folder }», junto al proyecto nuevo, salvo 1, que se queda en su proyecto de Scrivener. Su nota indica dónde está.
   *[other] Los archivos de investigación que Skribisto no puede abrir se han copiado en «{ $folder }», junto al proyecto nuevo, salvo { $count }, que se quedan en su proyecto de Scrivener. Sus notas indican dónde están.
}
import-scrivener-report-research-none-copied = { $count ->
    [1] Pidió que se copiaran los archivos de investigación, pero el único que había no se pudo copiar. Se queda en su proyecto de Scrivener, y su nota indica dónde está.
   *[other] Pidió que se copiaran los archivos de investigación, pero no se pudo copiar ninguno de los { $count }. Se quedan en su proyecto de Scrivener, y sus notas indican dónde están.
}
import-scrivener-report-chose-project = Eligió importar «{ $project }», uno de los varios proyectos que había en la carpeta o el archivo comprimido que seleccionó.
import-scrivener-report-chose-manifest = Eligió el archivo del archivador «{ $file }».
import-scrivener-report-chose-enclosing = Seleccionó algo dentro de «{ $project }», así que se importó el proyecto entero.
import-scrivener-report-chose-upgraded = Importó «{ $current }», el proyecto en el que siguió escribiendo, en lugar de la copia que Scrivener guardó antes de una actualización.
import-scrivener-report-chose-backup = Importó «{ $backup }», la copia anterior que Scrivener guardó antes de una actualización.
import-scrivener-report-title-placeholder-rejected = El título de los ajustes del proyecto era un texto provisional de la plantilla, así que Skribisto propuso en su lugar el nombre del proyecto.
import-scrivener-report-author-placeholder-rejected = El nombre del autor en los ajustes del proyecto era un texto provisional de la plantilla, así que Skribisto no propuso ninguno.
import-scrivener-report-title-from-compile-settings = Skribisto propuso el título que indican los ajustes de compilación del proyecto: el nombre que tenía el proyecto al crearse, salvo que se haya definido allí un título en Scrivener.
import-scrivener-report-title-from-template = Skribisto propuso el título de la propia plantilla.
import-scrivener-report-title-from-folder-name = Skribisto propuso el nombre del proyecto como título.
import-scrivener-report-title-from-project-properties = Skribisto propuso el título que indican las propiedades del proyecto.
import-scrivener-report-author-from-project = Skribisto propuso el nombre de autor que indican los ajustes del proyecto. Ahí puede quedar el autor de una plantilla, y se imprimiría en una portada.
import-scrivener-report-language-not-stated = El proyecto no indicaba su idioma, así que Skribisto propuso el idioma en que se muestra el propio Skribisto. El idioma decide cómo se cuentan las palabras y cómo se escriben las comillas.
import-scrivener-report-language-from-script = Skribisto propuso el idioma a partir del sistema de escritura del manuscrito ({ $tag }).
# A proposal of the Save page the writer replaced with their own.
import-scrivener-report-you-changed-it = { $hint } Usted sustituyó esta propuesta.
import-scrivener-report-content-salvaged-text = Faltaba el texto de «{ $item }» en el proyecto. Sus palabras proceden del índice de búsqueda de Scrivener, como texto sin formato y quizá de una versión anterior: compruébelas.
import-scrivener-report-content-salvaged-notes = Faltaban las notas de «{ $item }» en el proyecto. Sus palabras proceden del índice de búsqueda de Scrivener, como texto sin formato y quizá de una versión anterior: compruébelas.
import-scrivener-report-content-salvaged-synopsis = Faltaba la sinopsis de «{ $item }» en el proyecto. Sus palabras proceden del índice de búsqueda de Scrivener, como texto sin formato y quizá de una versión anterior: compruébelas.
# A finding of the document importer found in several places: its sentence about the
# first, then how many more the list under it names.
import-scrivener-report-document-others = { $others ->
    [one] { $sentence } Lo mismo ocurre en 1 lugar más, que se indica abajo.
   *[other] { $sentence } Lo mismo ocurre en { $others } lugares más, que se indican abajo.
}
# An item of the report, by its path in Scrivener's binder, with which of its writings
# the line is about when it is not the text.
import-scrivener-report-item-notes = { $path } (notas del documento)
import-scrivener-report-item-synopsis = { $path } (sinopsis)
import-scrivener-report-item-project-note = { $path } (nota del proyecto)
import-scrivener-report-item-recovered = { $path } (texto recuperado)
# An item with no name in Scrivener.
import-scrivener-report-untitled = Sin título
import-scrivener-report-no-findings = La importación no tiene nada más que señalar.
import-scrivener-report-group-unread = No se pudo leer
import-scrivener-report-group-check = Compruebe esto
import-scrivener-report-group-changed = Traído de otra forma
import-scrivener-report-group-left = Dejado atrás
import-scrivener-report-group-count = { $group } ({ $count })
import-scrivener-report-items-toggle = { $count ->
    [one] 1 elemento
   *[other] { $count } elementos
}
# The same, as a screen reader names it: with the line whose items it lists.
import-scrivener-report-items-toggle-named = { $count ->
    [one] 1 elemento: { $line }
   *[other] { $count } elementos: { $line }
}
import-scrivener-report-names-toggle = { $count ->
    [one] 1 nombre
   *[other] { $count } nombres
}
# The same, as a screen reader hears it, naming the line it belongs to.
import-scrivener-report-names-toggle-named = { $count ->
    [one] 1 nombre: { $line }
   *[other] { $count } nombres: { $line }
}
import-scrivener-report-more = { $count ->
    [one] Y 1 más.
   *[other] Y { $count } más.
}
import-scrivener-report-open-item = Abrir en el editor
import-scrivener-report-links-hint = Abra el proyecto nuevo y vuelva a mostrar este informe desde la paleta de comandos: cada elemento de la lista se abrirá entonces con un clic.
import-scrivener-copy-report = Copiar informe
import-scrivener-report-copied = Informe copiado
import-scrivener-report-copy-failed = No se pudo copiar el informe.
import-scrivener-report-close = Cerrar
import-scrivener-report-none-title = No hay ningún informe de importación
import-scrivener-report-none = Ninguna importación de Scrivener ha terminado desde que se inició Skribisto. Skribisto conserva los informes de las últimas importaciones hasta que se cierra.
import-scrivener-report-gone = { $count ->
    [one] El informe de esta importación ya no se conserva. Skribisto conserva el informe de la última importación hasta que se cierra.
   *[other] El informe de esta importación ya no se conserva. Skribisto conserva los informes de las últimas { $count } importaciones hasta que se cierra.
}

## Words the import writes into the new project, in the language Skribisto shows itself in
import-scrivener-manuscript-binder = Manuscrito
import-scrivener-notes-binder = Notas
import-scrivener-notes-of = { $title } (notas)
import-scrivener-project-notes = Notas del proyecto
import-scrivener-recovered = Textos recuperados
import-scrivener-untitled-template = Plantilla sin título
# The folder beside the new project its research files are copied into.
import-scrivener-research-folder = Archivos de investigación de { $name }
# Between two authors' names.
import-scrivener-list-separator = {", "}
import-scrivener-label-details = Del campo «{ $field }» de Scrivener
import-scrivener-status-details = Del campo «{ $field }» de Scrivener
import-scrivener-unnamed-label = Rótulo { $number }
import-scrivener-unnamed-status = Estado { $number }
import-scrivener-label-tag = { $name } (rótulo)
import-scrivener-untitled-tag = Sin título
# The tags, the pace plan and the "From Scrivener" folder the import makes from what a
# Scrivener project holds beyond its labels and status.
import-scrivener-keyword-details = La palabra clave «{ $path }» de Scrivener
import-scrivener-unnamed-keyword = Palabra clave { $number }
import-scrivener-keyword-tag = { $name } (palabra clave)
import-scrivener-collection-details = Una colección de Scrivener
import-scrivener-collection-tag = { $name } (colección)
import-scrivener-field-details = Del campo «{ $field }» de Scrivener
# A tag made from one option of a list field.
import-scrivener-field-value = { $field }: { $option }
import-scrivener-field-tag = { $name } (campo)
# The day off a pace plan takes on its deadline.
import-scrivener-deadline-day = Fecha límite
import-scrivener-from-scrivener = De Scrivener
import-scrivener-note-project = Proyecto
import-scrivener-note-project-bookmarks = Marcadores del proyecto
import-scrivener-note-replacements = Reemplazos de compilación
import-scrivener-note-auto-complete = Entradas de autocompletado
import-scrivener-note-saved-searches = Búsquedas guardadas
import-scrivener-note-custom-metadata = Metadatos personalizados
import-scrivener-note-targets = Objetivos
import-scrivener-note-bookmarks = Marcadores
import-scrivener-abbreviated-title = Título abreviado
import-scrivener-subjects = Temas
import-scrivener-company = Empresa
import-scrivener-session-target = Objetivo de sesión
import-scrivener-session-target-calculated = Objetivo de sesión, calculado por Scrivener a partir de la fecha límite
import-scrivener-writing-days = Días de escritura
import-scrivener-weekday-monday = lunes
import-scrivener-weekday-tuesday = martes
import-scrivener-weekday-wednesday = miércoles
import-scrivener-weekday-thursday = jueves
import-scrivener-weekday-friday = viernes
import-scrivener-weekday-saturday = sábado
import-scrivener-weekday-sunday = domingo
import-scrivener-draft-target = Objetivo del borrador
import-scrivener-deadline = Fecha límite
# A unit after a count, in parentheses: "1000 (words)".
import-scrivener-unit-words = palabras
import-scrivener-unit-characters = caracteres
import-scrivener-unit-pages = páginas
import-scrivener-search-query = Texto buscado
import-scrivener-search-in = Buscado en
import-scrivener-search-operator = Coincidencia
# What a saved search looks in, by the kind Scrivener names.
import-scrivener-search-in-all = todo
import-scrivener-search-in-title = títulos
import-scrivener-search-in-text = texto
import-scrivener-search-in-notes = notas
import-scrivener-search-in-synopsis = sinopsis
import-scrivener-search-in-keywords = palabras clave
import-scrivener-search-in-label = rótulos
import-scrivener-search-in-status = estados
import-scrivener-search-in-section-type = tipos de sección
import-scrivener-search-in-custom-metadata = metadatos personalizados
# How a saved search's words combine.
import-scrivener-search-any = cualquier palabra
import-scrivener-search-all = todas las palabras
import-scrivener-search-exact = frase exacta
# One line of a listing: a bookmark's place and its note, or a link's words and its address.
import-scrivener-label-value = { $label }: { $value }
import-scrivener-replacement-regex = expresión regular
import-scrivener-replacement-off = desactivado
# Said of a note whose own text lists its custom metadata or bookmarks.
import-scrivener-in-own-note = Figura en la nota en la que se convirtió este documento.


## Import a Scrivener project: what the import says about the project, in the writer's words.
## Every key the converter can give has a line here (see `import_scrivener::messages`).

# A refusal on one line: its title, then what to do.
import-scrivener-paragraph = { $title }. { $body }

## When the project cannot be imported
import-scrivener-refusal-missing-title = No hay nada en esta ruta
import-scrivener-refusal-missing-body = «{ $file }» ya no está. Vuelva a elegir el proyecto.
import-scrivener-refusal-unreadable-title = Este proyecto no se puede leer
import-scrivener-refusal-unreadable-body = Skribisto no pudo leer «{ $file }» ({ $reason }). Compruebe que puede abrirlo y vuelva a intentarlo.
import-scrivener-refusal-not-a-scrivener-project-title = Aquí no hay ningún proyecto de Scrivener
import-scrivener-refusal-not-a-scrivener-project-body = Elija la carpeta .scriv del proyecto, el archivo .scrivx que contiene, una copia de seguridad .zip o un .scrivtemplate.
import-scrivener-refusal-manifest-without-files-title = Los documentos no están en esta copia
import-scrivener-refusal-manifest-without-files-body = «{ $manifest }» está aquí, pero no la carpeta Files, que contiene los documentos que enumera. Puede que un servicio de sincronización no haya terminado de copiar el proyecto, o que solo se haya copiado el archivo .scrivx. Si eligió el archivo .scrivx, elija en su lugar la carpeta del proyecto con «Elegir carpeta…».
import-scrivener-refusal-search-too-wide-title = Esta carpeta contiene demasiado para buscar en ella
import-scrivener-refusal-search-too-wide-body = Tiene más de { $limit } carpetas que recorrer. Elija el propio proyecto de Scrivener.
import-scrivener-refusal-scrivener-one-for-mac-title = Este proyecto se creó con Scrivener 1 para Mac
import-scrivener-refusal-scrivener-one-for-mac-body = Skribisto lee proyectos de Scrivener 2 y posteriores. Ábralo una vez en Scrivener 2 o 3 para actualizarlo, o compílelo en un documento de Word e importe ese documento.
import-scrivener-refusal-ios-stub-without-mobile-data-title = Esto es solo un archivo provisional de Scrivener para iPhone o iPad
import-scrivener-refusal-ios-stub-without-mobile-data-body = El proyecto en sí no está aquí. Sincronícelo desde su iPhone o iPad, o compártalo desde allí como .zip, e importe ese archivo.
import-scrivener-refusal-newer-title = Este proyecto se creó con una versión más reciente de Scrivener
import-scrivener-refusal-unknown-format-version-body = Su formato, { $version }, no es uno que Skribisto conozca. Busque una actualización de Skribisto. Mientras tanto, compílelo en un documento de Word en Scrivener e importe ese documento.
import-scrivener-refusal-newer-format-stamp-body = Sus archivos están marcados como versión { $stamp }, y Skribisto lee hasta la versión { $newest }. Busque una actualización de Skribisto. Mientras tanto, compílelo en un documento de Word en Scrivener e importe ese documento.
import-scrivener-refusal-version-stamp-unreadable-title = No se puede leer la versión del proyecto
import-scrivener-refusal-version-stamp-unreadable-body = Su archivo Files/version.txt no contiene ningún número de versión, así que Skribisto no puede saber si sabe leer este proyecto. Abra el proyecto una vez en Scrivener y vuelva a importarlo.
import-scrivener-refusal-damaged-manifest-title = No se puede leer el archivo del archivador
import-scrivener-refusal-damaged-manifest-body = «{ $manifest }» está dañado. { $spare_copies ->
    [0] Abra el proyecto en Scrivener, que puede repararlo, y vuelva a importarlo.
    [1] Scrivener guarda una copia de reserva dentro del proyecto, que Skribisto no lee. Abra el proyecto en Scrivener, que puede repararlo, y vuelva a importarlo.
   *[other] Scrivener guarda { $spare_copies } copias de reserva dentro del proyecto, que Skribisto no lee. Abra el proyecto en Scrivener, que puede repararlo, y vuelva a importarlo.
}
# The empty file Scrivener writes for a new project before its first save: not damage.
import-scrivener-refusal-damaged-manifest-new-project-placeholder-body = «{ $manifest }» es el archivo vacío que Scrivener escribe para un proyecto que aún no ha guardado. Abra el proyecto en Scrivener y deje que lo guarde; luego vuelva a importarlo.
import-scrivener-refusal-manifest-too-large-title = Este archivo del archivador es demasiado grande
import-scrivener-refusal-manifest-too-large-body = «{ $manifest }» ocupa más de los { $limit } MB que lee Skribisto. Ningún proyecto real se acerca a eso, así que puede que el archivo esté dañado.
import-scrivener-refusal-no-item-identifiers-title = El archivador no enumera ningún documento que Skribisto pueda encontrar
import-scrivener-refusal-no-item-identifiers-body = Ninguno de sus elementos lleva un identificador, así que no se pueden encontrar sus archivos. Abra el proyecto una vez en Scrivener y vuelva a importarlo.
import-scrivener-refusal-incomplete-copy-title = Faltan los documentos en esta copia
import-scrivener-refusal-incomplete-copy-body = La carpeta Files está aquí, pero los documentos que debería contener no. Puede que un servicio de sincronización no haya terminado de copiar el proyecto. Espere a que termine, o vuelva a copiar la carpeta .scriv entera, y después vuelva a leer el proyecto.
import-scrivener-refusal-archive-title = No se puede abrir este archivo comprimido
import-scrivener-refusal-archive-unreadable-body = No se pudo leer «{ $file }» ({ $reason }). Puede que esté dañado. Descomprímalo usted mismo y elija la carpeta .scriv que contiene.
import-scrivener-refusal-archive-unsafe-name-title = Este archivo comprimido no se puede abrir de forma segura
# A name with `..` in it, or an absolute one.
import-scrivener-refusal-archive-unsafe-name-body = «{ $file }» contiene un archivo llamado «{ $member }», que acabaría fuera del archivo comprimido, así que Skribisto no lo abre.
import-scrivener-refusal-archive-unsafe-name-empty-body = «{ $file }» contiene un archivo sin nombre, así que Skribisto no lo abre.
import-scrivener-refusal-archive-unsafe-name-control-character-body = «{ $file }» contiene un archivo cuyo nombre incluye un carácter de control, así que Skribisto no lo abre.
import-scrivener-refusal-archive-too-large-title = Este archivo comprimido es demasiado grande para abrirlo de forma segura
import-scrivener-refusal-archive-too-many-members-body = Contiene más de { $limit } archivos. Descomprímalo usted mismo y elija la carpeta .scriv que contiene.
import-scrivener-refusal-archive-member-too-large-body = «{ $member }» se descomprime en más de { $limit } MB. Descomprima el archivo usted mismo y elija la carpeta .scriv que contiene.
import-scrivener-refusal-archive-too-large-body = Se descomprime en más de { $limit } MB. Descomprímalo usted mismo y elija la carpeta .scriv que contiene.
import-scrivener-refusal-archive-encrypted-title = Este archivo comprimido está protegido con contraseña
import-scrivener-refusal-archive-encrypted-body = Descomprima «{ $file }» usted mismo y elija la carpeta .scriv que contiene.
import-scrivener-refusal-archive-unsupported-compression-title = Este archivo comprimido usa una compresión que Skribisto no puede leer
import-scrivener-refusal-archive-unsupported-compression-body = «{ $member }» está comprimido con { $method }. Descomprima el archivo usted mismo y elija la carpeta .scriv que contiene.
import-scrivener-refusal-duplicate-archive-members-title = Este archivo comprimido enumera algunos archivos dos veces
import-scrivener-refusal-duplicate-archive-members-body = «{ $file }» contiene más de un archivo con el mismo nombre, así que Skribisto no puede saber cuál pertenece al proyecto. Descomprímalo usted mismo y elija la carpeta .scriv que contiene.
# What a file cut short looks like; a file too large, one that is not a template and one
# encoded another way have their own words.
import-scrivener-refusal-template-undecodable-title = Este archivo de plantilla está incompleto
import-scrivener-refusal-template-undecodable-too-large-title = Este archivo de plantilla es demasiado grande para abrirlo
import-scrivener-refusal-template-undecodable-not-a-template-title = Este archivo no es una plantilla de Scrivener
import-scrivener-refusal-template-undecodable-unknown-encoding-title = Este archivo de plantilla está guardado de una forma que Skribisto no conoce
import-scrivener-refusal-template-undecodable-body = «{ $file }» no se puede descodificar: puede que termine antes que el proyecto que contiene. Descárguelo de nuevo, o ábralo en Scrivener y cree un proyecto a partir de él.
import-scrivener-refusal-template-undecodable-too-large-body = «{ $file }» ocupa más de { $limit } MB. Ábralo en Scrivener, cree un proyecto a partir de él e importe ese proyecto.
import-scrivener-refusal-template-undecodable-not-a-template-body = «{ $file }» no contiene un proyecto de Scrivener. Elija el archivo de plantilla que guardó Scrivener, o un proyecto creado a partir de él.
import-scrivener-refusal-template-undecodable-unknown-encoding-body = «{ $file }» guarda su proyecto de una forma que Skribisto no puede descodificar. Ábralo en Scrivener, cree un proyecto a partir de él e importe ese proyecto.
import-scrivener-refusal-cloud-placeholders-title = { $count ->
    [one] 1 archivo de este proyecto sigue en la nube
   *[other] { $count } archivos de este proyecto siguen en la nube
}
import-scrivener-refusal-cloud-placeholders-body = { $count ->
    [1] No se ha descargado en este equipo: { $examples }. Haga que el proyecto esté disponible sin conexión en su aplicación de sincronización y después vuelva a leer el proyecto.
   *[other] No se han descargado en este equipo; por ejemplo, { $examples }. Haga que el proyecto esté disponible sin conexión en su aplicación de sincronización y después vuelva a leer el proyecto.
}
import-scrivener-refusal-unreadable-members-title = { $count ->
    [one] 1 archivo de este proyecto no se puede leer
   *[other] { $count } archivos de este proyecto no se pueden leer
}
import-scrivener-refusal-unreadable-members-body = Por ejemplo, { $examples }. Asegúrese de que tiene permiso para leer todo el proyecto y de que el disco está en buen estado, y después vuelva a leer el proyecto.
import-scrivener-refusal-needed-member-is-link-title = Este proyecto apunta fuera de su propia carpeta
import-scrivener-refusal-needed-member-is-link-body = «{ $member }» es un enlace a otro lugar de este equipo, y Skribisto no sigue enlaces. Copie el proyecto de modo que todos sus archivos estén dentro de él y vuelva a intentarlo.
import-scrivener-refusal-project-changed-during-read-title = El proyecto cambió mientras se leía
import-scrivener-refusal-project-changed-during-read-body = Scrivener o un servicio de sincronización sigue escribiendo en él. Ciérrelo en Scrivener, espere a que termine la sincronización y después vuelva a leer el proyecto.
import-scrivener-refusal-too-many-members-title = Este proyecto contiene demasiados archivos
import-scrivener-refusal-too-many-members-body = Tiene más de { $limit } archivos, más que cualquier proyecto real. Puede que no sea un proyecto de Scrivener, o que esté dañado.
import-scrivener-refusal-too-large-title = Este proyecto es demasiado grande
import-scrivener-refusal-too-large-body = Su texto ocupa más de { $limit } MB, más de lo que Skribisto puede importar con seguridad.
import-scrivener-refusal-structure-unmappable-title = Skribisto no puede convertir este archivador
import-scrivener-refusal-structure-unmappable-body = El archivador no se pudo traer sin perder ni descolocar algo. Es un fallo de Skribisto, no de su proyecto. Informe de él indicando lo que muestra «Detalles».

## When there is a choice to make
import-scrivener-roots-title = { $count ->
    [one] Aquí hay { $count } proyecto. ¿Cuál quiere importar?
   *[other] Aquí hay { $count } proyectos. ¿Cuál quiere importar?
}
import-scrivener-roots-hint = Esta carpeta o este archivo comprimido contiene más de un proyecto de Scrivener. Elija el que quiere importar.
import-scrivener-manifests-title = { $count ->
    [one] Este proyecto tiene { $count } archivo del archivador
   *[other] Este proyecto tiene { $count } archivos del archivador
}
import-scrivener-manifests-hint = Scrivener guarda el archivador, la lista de todos los documentos y carpetas, en un archivo .scrivx. Lo más probable es que los demás los haya dejado un servicio de sincronización. Elija aquel en el que trabajó por última vez.
import-scrivener-manifests-hint-preselected = Scrivener guarda el archivador, la lista de todos los documentos y carpetas, en un archivo .scrivx. Lo más probable es que los demás los haya dejado un servicio de sincronización. El seleccionado es, con toda probabilidad, aquel en el que trabajó por última vez: compruébelo antes de continuar.
import-scrivener-manifest-row = { $file } · guardado el { $date } · { $documents ->
    [one] 1 elemento
   *[other] { $documents } elementos
}
import-scrivener-manifest-row-undated = { $file } · { $documents ->
    [one] 1 elemento
   *[other] { $documents } elementos
}
import-scrivener-manifest-row-unread = { $file } · no se puede leer
import-scrivener-enclosing-title = Esto forma parte del proyecto de Scrivener «{ $project }»
import-scrivener-enclosing-body = Ha elegido un archivo o una carpeta dentro de un proyecto. Skribisto importa el proyecto entero.
import-scrivener-enclosing-use = Importar «{ $project }»
import-scrivener-upgrade-title = Esta es la copia que Scrivener guardó antes de una actualización
import-scrivener-upgrade-body = Cuando Scrivener actualiza un proyecto a una versión más reciente, guarda el antiguo como «{ $backup }». El proyecto en el que siguió escribiendo es «{ $current }», a su lado.
import-scrivener-upgrade-body-unclear = Cuando Scrivener actualiza un proyecto a una versión más reciente, guarda el antiguo como «{ $backup }». El proyecto de al lado, «{ $current }», no se pudo leer bien, así que elija cuál importar.
import-scrivener-upgrade-use-current = Importar «{ $current }», el proyecto en el que siguió escribiendo
import-scrivener-upgrade-keep-backup = Importar esta copia anterior, «{ $backup }»

## What must be accepted before going on
import-scrivener-ios-pending-title = Los cambios hechos en Scrivener para iPhone o iPad todavía no están en esta copia
import-scrivener-ios-pending-body = Scrivener en un equipo incorpora esos cambios la próxima vez que abre el proyecto. Abra el proyecto una vez en Scrivener en su equipo y vuelva a importarlo.
import-scrivener-ios-pending-accept = Importarlo sin esos cambios
import-scrivener-no-text-title = { $count ->
    [one] 1 documento, y ni una sola palabra de texto en esta copia
   *[other] { $count } documentos, y ni una sola palabra de texto en esta copia
}
import-scrivener-no-text-body = Puede que un servicio de sincronización siga copiando el proyecto, o que la copia haya perdido sus documentos. Si lo importa ahora, todos los documentos llegarán vacíos.
import-scrivener-no-text-accept = Importar los documentos vacíos

## Things to know, nothing to decide
import-scrivener-notice-project-found-below-pick-title = El proyecto se encontró dentro de la carpeta que eligió
import-scrivener-notice-project-found-below-pick-body = Está en «{ $path }».
import-scrivener-notice-source-is-template-title = Esto es una plantilla de Scrivener
import-scrivener-notice-source-is-template-named = Esto es la plantilla de Scrivener «{ $title }»
import-scrivener-notice-source-is-template-body = Al importarla se crea un proyecto nuevo con lo que contiene la plantilla, incluidas sus instrucciones.
import-scrivener-notice-source-is-backup-title = Esto parece una copia de seguridad hecha por Scrivener
import-scrivener-notice-source-is-backup-body = Contiene el proyecto tal como estaba cuando se hizo la copia de seguridad. Si siguió escribiendo después, elija el propio proyecto.
import-scrivener-notice-importing-pre-upgrade-copy-title = Está importando la copia que Scrivener guardó antes de una actualización
import-scrivener-notice-importing-pre-upgrade-copy-body = El proyecto en el que siguió escribiendo está a su lado y contiene todo lo escrito desde entonces.
import-scrivener-notice-imported-from-ios-stub-title = Este proyecto se creó en Scrivener para iPhone o iPad
import-scrivener-notice-imported-from-ios-stub-body = Nunca se ha abierto en un equipo, así que se lee de la copia que guarda el iPhone o el iPad.
import-scrivener-notice-stale-mobile-binder-ignored-title = Se aparta un archivador antiguo de un iPhone o iPad
import-scrivener-notice-stale-mobile-binder-ignored-body = Scrivener en un equipo ya incorporó sus cambios.
import-scrivener-notice-identical-manifest-copies-title = { $count ->
    [one] Este proyecto tiene { $count } archivo del archivador idéntico
   *[other] Este proyecto tiene { $count } archivos del archivador idénticos
}
import-scrivener-notice-identical-manifest-copies-body = Contienen lo mismo, así que cualquiera de ellas da el mismo proyecto.
import-scrivener-notice-manifest-copies-ignored-title = Se apartan las copias sobrantes del archivo del archivador
import-scrivener-notice-manifest-copies-ignored-body = «{ $chosen }» es aquel en el que Scrivener trabajó por última vez. { $count ->
    [one] Se aparta 1 copia que dejó un servicio de sincronización.
   *[other] Se apartan { $count } copias que dejó un servicio de sincronización.
}
import-scrivener-notice-pinned-manifest-looks-like-a-copy-title = El archivo del archivador que eligió parece una copia de sincronización
import-scrivener-notice-pinned-manifest-looks-like-a-copy-body = «{ $name }» tiene el nombre de una copia de «{ $original }», que está a su lado. Si la importación parece antigua, elija «{ $original }» en su lugar.
import-scrivener-notice-pinned-manifest-has-siblings-title = { $count ->
    [one] Junto al que eligió hay otro archivo del archivador
   *[other] Junto al que eligió hay otros { $count } archivos del archivador
}
import-scrivener-notice-pinned-manifest-has-siblings-body = Solo se lee el que eligió.
import-scrivener-notice-legacy-v1-files-beside-title = Se apartan archivos de una versión anterior de Scrivener
import-scrivener-notice-legacy-v1-files-beside-body = Se quedaron atrás cuando se actualizó el proyecto. Solo se leen los archivos de la versión actual.
import-scrivener-notice-mixed-payload-trees-title = Este proyecto contiene archivos de dos versiones de Scrivener
import-scrivener-notice-mixed-payload-trees-body = Los documentos se leen donde el archivo del archivador indica que están.
import-scrivener-notice-version-disagreement-title = Los archivos de este proyecto no coinciden todos en su versión
import-scrivener-notice-id-scheme-mismatch-body = El archivo del archivador indica la versión { $version }, pero nombra sus documentos como lo hace otra versión. Se leen donde él indica. Si la importación parece incorrecta, abra el proyecto una vez en Scrivener y vuelva a importarlo.
import-scrivener-notice-version-stamp-mismatch-body = Se lee tal como lo describe su archivo del archivador. Si la importación parece incorrecta, abra el proyecto una vez en Scrivener y vuelva a importarlo.
import-scrivener-notice-unrecognised-writer-title = Este proyecto lo guardó un programa que Skribisto no reconoce
import-scrivener-notice-unrecognised-writer-named = Se identifica como «{ $creator }». Se lee como un proyecto de Scrivener.
import-scrivener-notice-unrecognised-writer-body = No se identifica. Se lee como un proyecto de Scrivener.
import-scrivener-notice-invalid-item-identifier-title = { $count ->
    [one] 1 elemento del archivador lleva un identificador que no es válido
   *[other] { $count } elementos del archivador llevan un identificador que no es válido
}
import-scrivener-notice-invalid-item-identifier-body = Un identificador así se descarta, y el elemento que lo lleva se conserva.
import-scrivener-notice-items-without-identifier-title = { $count ->
    [one] 1 elemento del archivador no tiene identificador
   *[other] { $count } elementos del archivador no tienen identificador
}
import-scrivener-notice-items-without-identifier-body = Sin identificador, los archivos de un elemento no se pueden encontrar, así que el elemento llega vacío.
import-scrivener-notice-duplicate-item-key-title = Dos elementos del archivador comparten un mismo identificador: «{ $item }»
import-scrivener-notice-duplicate-item-key-body = El primero conserva los documentos. El segundo conserva su lugar y llega vacío.
import-scrivener-notice-lock-here-title = Scrivener tiene este proyecto abierto en este equipo
import-scrivener-notice-lock-here-body = Ciérrelo antes en Scrivener, para que se guarden todos los cambios.
import-scrivener-notice-lock-title = Scrivener tenía este proyecto abierto en «{ $host }»
import-scrivener-notice-lock-elsewhere-title = Scrivener tenía este proyecto abierto en otro equipo
import-scrivener-notice-lock-body = Si sigue abierto allí, ciérrelo antes: puede que los archivos aún no contengan los últimos cambios. Si Scrivener se cerró hace tiempo, puede importarlo tal como está.
import-scrivener-notice-archived-while-open-title = Este archivo comprimido se creó mientras Scrivener tenía el proyecto abierto
import-scrivener-notice-archived-while-open-body = Puede que le falten los cambios hechos después de ese momento.
import-scrivener-notice-content-missing-notes = Faltan las notas de «{ $item }»
import-scrivener-notice-content-missing-synopsis = Falta la sinopsis de «{ $item }»
import-scrivener-notice-content-missing-text = Falta el texto de «{ $item }»
import-scrivener-notice-content-missing-body = { $place ->
    [notes] Su archivo no está en esta copia, así que llegan sin contenido. Puede que un servicio de sincronización siga copiando el proyecto.
   *[text] Su archivo no está en esta copia, así que llega sin contenido. Puede que un servicio de sincronización siga copiando el proyecto.
}
import-scrivener-notice-content-missing-salvaged = { $place ->
   *[text] Su archivo no está en esta copia, pero el índice de búsqueda de Scrivener aún conservaba sus palabras. Se conservan como texto sin formato, quizá de una versión anterior.
}
import-scrivener-notice-content-truncated-notes = Las notas de «{ $item }» están vacías en el disco
import-scrivener-notice-content-truncated-synopsis = La sinopsis de «{ $item }» está vacía en el disco
import-scrivener-notice-content-truncated-text = El texto de «{ $item }» está vacío en el disco
import-scrivener-notice-content-truncated-body = { $place ->
    [notes] Según Scrivener tenían texto, pero su archivo no contiene nada: puede que una sincronización se detuviera a medias. Llegan sin contenido.
   *[text] Según Scrivener tenía texto, pero su archivo no contiene nada: puede que una sincronización se detuviera a medias. Llega sin contenido.
}
import-scrivener-notice-missing-media-title = Falta el archivo de «{ $item }»
import-scrivener-notice-missing-media-body = La imagen o el archivo de investigación que contiene no está en esta copia.
import-scrivener-notice-sync-conflict-copies-title = { $count ->
    [one] Se ha encontrado 1 copia de sincronización de un documento
   *[other] Se han encontrado { $count } copias de sincronización de documentos
}
import-scrivener-notice-sync-conflict-copies-body = { $missing ->
    [0] Las copias de sincronización llegan como textos recuperados, en una carpeta propia, para que pueda compararlas con los documentos.
    [one] Las copias de sincronización llegan como textos recuperados, en una carpeta propia. { $missing } de ellas es la copia de un documento que ya no está en el proyecto.
   *[other] Las copias de sincronización llegan como textos recuperados, en una carpeta propia. { $missing } de ellas son copias de documentos que ya no están en el proyecto.
}
import-scrivener-notice-recovered-unreferenced-text-title = { $count ->
    [one] Se ha encontrado 1 documento que no figura en el archivador
   *[other] Se han encontrado { $count } documentos que no figuran en el archivador
}
import-scrivener-notice-recovered-unreferenced-text-body = Los documentos de este tipo llegan como textos recuperados, en una carpeta propia.
import-scrivener-notice-sync-debris-ignored-title = { $count ->
    [one] Se aparta 1 copia sobrante que dejó un servicio de sincronización
   *[other] Se apartan { $count } copias sobrantes que dejó un servicio de sincronización
}
import-scrivener-notice-sync-debris-ignored-body = Scrivener tampoco lee ese tipo de copias. Solo se leen los archivos propios del proyecto.
import-scrivener-notice-unrecognised-files-title = { $count ->
    [one] Se aparta 1 archivo que no forma parte de un proyecto de Scrivener
   *[other] Se apartan { $count } archivos que no forman parte de un proyecto de Scrivener
}
import-scrivener-notice-examples = Por ejemplo, { $examples }.
import-scrivener-notice-symlink-not-followed-title = { $count ->
    [one] No se sigue 1 enlace del interior del proyecto
   *[other] No se siguen { $count } enlaces del interior del proyecto
}
import-scrivener-notice-unreadable-files-skipped-title = { $count ->
    [one] No se pudo leer 1 archivo que la importación no necesita
   *[other] No se pudieron leer { $count } archivos que la importación no necesita
}
import-scrivener-notice-cloud-placeholders-skipped-title = { $count ->
    [one] Se omite 1 archivo que sigue en la nube
   *[other] Se omiten { $count } archivos que siguen en la nube
}
import-scrivener-notice-cloud-placeholders-skipped-body = Nada de lo que necesita la importación sigue en la nube.
import-scrivener-notice-no-text-found-title = { $count ->
    [1] 1 documento, y no contiene texto
   *[other] { $count } documentos, y ninguno contiene texto
}
import-scrivener-notice-no-text-found-body = Este proyecto parece vacío. Se importa tal como está.
import-scrivener-notice-metadata-unreadable-title = No se pudo leer «{ $member }»
import-scrivener-notice-metadata-unreadable-body = Lo que Scrivener guardaba en él no llega.
import-scrivener-notice-dangling-keyword-title = { $count ->
    [one] 1 referencia apunta a una palabra clave que el proyecto no define
   *[other] { $count } referencias apuntan a palabras clave que el proyecto no define
}
import-scrivener-notice-dangling-label-title = { $count ->
    [one] 1 referencia apunta a un rótulo que el proyecto no define
   *[other] { $count } referencias apuntan a rótulos que el proyecto no define
}
import-scrivener-notice-dangling-status-title = { $count ->
    [one] 1 referencia apunta a un estado que el proyecto no define
   *[other] { $count } referencias apuntan a estados que el proyecto no define
}
import-scrivener-notice-dangling-section-type-title = { $count ->
    [one] 1 referencia apunta a un tipo de sección que el proyecto no define
   *[other] { $count } referencias apuntan a tipos de sección que el proyecto no define
}
import-scrivener-notice-dangling-custom-field-title = { $count ->
    [one] 1 referencia apunta a un campo de metadatos personalizados que el proyecto no define
   *[other] { $count } referencias apuntan a campos de metadatos personalizados que el proyecto no define
}
import-scrivener-notice-dangling-item-title = { $count ->
    [one] 1 referencia apunta a un documento que el proyecto no define
   *[other] { $count } referencias apuntan a documentos que el proyecto no define
}
import-scrivener-notice-dangling-references-body = Una referencia a algo que el proyecto no define se deja fuera.
import-scrivener-notice-snapshots-not-imported-line = { $count ->
    [one] 1 instantánea se queda en su proyecto de Scrivener: Skribisto no trae las instantáneas.
   *[other] { $count } instantáneas se quedan en su proyecto de Scrivener: Skribisto no trae las instantáneas.
}
import-scrivener-notice-writing-history-not-imported-line = { $days ->
    [one] El registro de Scrivener de las palabras escritas cada día (1 día) no se trae. Skribisto empieza su propio recuento desde el primer guardado.
   *[other] El registro de Scrivener de las palabras escritas cada día ({ $days } días) no se trae. Skribisto empieza su propio recuento desde el primer guardado.
}
import-scrivener-notice-title-placeholder-rejected-hint = El título de los ajustes del proyecto es un texto provisional de la plantilla, así que en su lugar se propone el nombre del proyecto.
import-scrivener-notice-author-placeholder-rejected-hint = El nombre del autor en los ajustes del proyecto es un texto provisional de la plantilla, así que no se propone ninguno.
import-scrivener-notice-title-from-project-hint-compile-settings = Este título es el que indican los ajustes de compilación del proyecto. Hasta que se define allí un título, Scrivener conserva el nombre que tenía el proyecto al crearse.
import-scrivener-notice-title-from-project-hint-template = Este título es el de la propia plantilla.
import-scrivener-notice-title-from-project-hint-folder-name = Este título es el nombre del proyecto.
import-scrivener-notice-title-from-project-hint-project-properties = Este título es el que indican las propiedades del proyecto.
import-scrivener-notice-author-from-project-hint = Este nombre es el que indican los ajustes del proyecto. Ahí puede quedar el autor de una plantilla, y se imprimiría en una portada.
import-scrivener-notice-language-not-stated-hint = El proyecto no indica su idioma, así que se propone el idioma en que se muestra Skribisto. El idioma decide cómo se cuentan las palabras y cómo se escriben las comillas.
import-scrivener-notice-language-from-script-hint = Propuesto a partir del sistema de escritura del manuscrito ({ $tag }).

## Before you import: what the import does with the binder, or leaves behind
import-scrivener-structure-script-mode-draft-line = { $script ->
    [1] { $documents ->
        [1] El único documento del manuscrito está escrito en modo guion. Skribisto conserva sus palabras, no su maquetación de guion.
       *[other] { $script } de los { $documents } documentos del manuscrito está escrito en modo guion. Skribisto conserva sus palabras, no su maquetación de guion.
    }
   *[other] { $script } de los { $documents } documentos del manuscrito están escritos en modo guion. Skribisto conserva sus palabras, no su maquetación de guion.
}
import-scrivener-structure-one-document-manuscript-line = { $count ->
    [1] Un documento del manuscrito contiene sus capítulos como encabezados.
   *[other] { $count } documentos del manuscrito contienen sus capítulos como encabezados.
}
import-scrivener-structure-title-repeats-number-line = { $count ->
    [one] 1 título de capítulo o de parte incluye también su número, como «Capítulo II» o «Primera parte», así que una exportación puede imprimir el número dos veces.
   *[other] { $count } títulos de capítulo o de parte incluyen también su número, como «Capítulo II» o «Primera parte», así que una exportación puede imprimir el número dos veces.
}
import-scrivener-structure-claim-overruled-line = { $count ->
    [one] 1 elemento parece un libro, una parte o un capítulo, pero está dentro de un elemento que ya es de ese nivel, así que llega a un nivel inferior.
   *[other] { $count } elementos parecen libros, partes o capítulos, pero cada uno está dentro de un elemento que ya es de ese nivel, así que llegan a un nivel inferior.
}
import-scrivener-structure-adopted-by-chapter-line = { $count ->
    [one] 1 documento que sigue a un capítulo pasa a ser el final de ese capítulo.
   *[other] { $count } documentos que siguen a un capítulo pasan a ser el final de ese capítulo.
}
import-scrivener-structure-looks-like-manuscript-line = { $count ->
    [one] 1 carpeta de primer nivel que se queda en las notas parece manuscrito. Puede enviarla al manuscrito como otro libro.
   *[other] { $count } carpetas de primer nivel que se quedan en las notas parecen manuscrito. Puede enviarlas al manuscrito como otros libros.
}
import-scrivener-structure-generic-sheet-title-line = { $count ->
    [one] 1 entrada de la biblia narrativa aún tiene el título de una plantilla, como «Héroe». Skribisto busca ese título en su texto y, en la página En la prosa, enumera cada lugar donde aparece: renombre la entrada si esa palabra es frecuente en su libro.
   *[other] { $count } entradas de la biblia narrativa aún tienen el título de una plantilla, como «Héroe». Skribisto busca esos títulos en su texto y, en la página En la prosa, enumera cada lugar donde aparecen: renombre las entradas si esas palabras son frecuentes en su libro.
}
import-scrivener-structure-placeholder-title-line = { $count ->
    [one] 1 título solo nombraba su nivel y se deja en blanco: el archivador y las exportaciones lo numeran.
   *[other] { $count } títulos solo nombraban su nivel y se dejan en blanco: el archivador y las exportaciones los numeran.
}
import-scrivener-structure-untitled-item-line = { $count ->
    [one] 1 elemento no tenía título en Scrivener y se queda sin título.
   *[other] { $count } elementos no tenían título en Scrivener y se quedan sin título.
}
import-scrivener-structure-title-cleaned-line = { $count ->
    [one] 1 título contenía saltos de línea o tabulaciones, que se eliminan.
   *[other] { $count } títulos contenían saltos de línea o tabulaciones, que se eliminan.
}
import-scrivener-structure-own-text-moved-line = { $count ->
    [one] 1 carpeta tiene texto propio, que pasa a un primer documento dentro de ella.
   *[other] { $count } carpetas tienen texto propio, que pasa a un primer documento dentro de cada una.
}
import-scrivener-structure-text-with-children-line = { $count ->
    [one] 1 documento que contiene otros documentos se convierte en carpeta.
   *[other] { $count } documentos que contienen otros documentos se convierten en carpetas.
}
import-scrivener-structure-empty-folder-plain-line = { $count ->
    [one] 1 carpeta vacía sigue siendo una carpeta simple, para que ninguna exportación imprima un encabezado que Scrivener nunca imprimió.
   *[other] { $count } carpetas vacías siguen siendo carpetas simples, para que ninguna exportación imprima un encabezado que Scrivener nunca imprimió.
}
import-scrivener-structure-book-inside-book-line = { $count ->
    [one] 1 libro dentro de un libro se convierte en una parte.
   *[other] { $count } libros dentro de un libro se convierten en partes.
}
import-scrivener-structure-prologue-not-numbered-line = { $count ->
    [one] 1 prólogo o epílogo queda fuera de la numeración de capítulos.
   *[other] { $count } prólogos o epílogos quedan fuera de la numeración de capítulos.
}
import-scrivener-structure-matter-inside-book-line = { $count ->
    [one] 1 elemento de páginas preliminares o finales situado fuera del manuscrito pasa al interior del libro como paratexto, fuera de las exportaciones hasta que lo incluya.
   *[other] { $count } elementos de páginas preliminares o finales situados fuera del manuscrito pasan al interior del libro como paratexto, fuera de las exportaciones hasta que los incluya.
}
import-scrivener-structure-paratext-not-counted-line = { $documents ->
    [one] 1 documento del manuscrito pasa a ser paratexto (páginas preliminares o finales), así que { $words ->
        [one] { $words } palabra deja
       *[other] { $words } palabras dejan
    } de contar como manuscrito.
   *[other] { $documents } documentos del manuscrito pasan a ser paratexto (páginas preliminares o finales), así que { $words ->
        [one] { $words } palabra deja
       *[other] { $words } palabras dejan
    } de contar como manuscrito.
}
import-scrivener-structure-document-notes-line = { $count ->
    [one] Las notas de 1 documento se convierten en una nota junto a él.
   *[other] Las notas de { $count } documentos se convierten en notas junto a ellos.
}
import-scrivener-structure-draft-content-as-note-line = El texto propio de la carpeta del manuscrito se convierte en una nota al principio del manuscrito.
import-scrivener-structure-story-bible-folder-line = { $count ->
    [one] 1 carpeta de personajes o de lugares se convierte en una etiqueta de la biblia narrativa.
   *[other] { $count } carpetas de personajes o de lugares se convierten en etiquetas de la biblia narrativa.
}
import-scrivener-structure-templates-created-line = { $count ->
    [one] 1 plantilla de documento se convierte en una plantilla de notas.
   *[other] { $count } plantillas de documento se convierten en plantillas de notas.
}
import-scrivener-structure-template-nesting-flattened-line = { $count ->
    [one] 1 carpeta de plantillas situada dentro de otra se aplana: cada texto que contiene se convierte en una plantilla.
   *[other] { $count } carpetas de plantillas situadas dentro de otras se aplanan: cada texto que contienen se convierte en una plantilla.
}
import-scrivener-structure-template-kept-as-note-line = { $count ->
    [one] 1 plantilla contiene lo que una plantilla de notas no puede contener, y se conserva como nota.
   *[other] { $count } plantillas contienen lo que una plantilla de notas no puede contener, y se conservan como notas.
}
import-scrivener-structure-recovered-texts-line = { $count ->
    [one] 1 texto que no figuraba en el archivador llega en una carpeta propia.
   *[other] { $count } textos que no figuraban en el archivador llegan en una carpeta propia.
}
import-scrivener-structure-project-notes-line = { $count ->
    [one] 1 nota del proyecto llega en una carpeta propia.
   *[other] { $count } notas del proyecto llegan en una carpeta propia.
}
import-scrivener-structure-trash-brought-across-line = { $items ->
    [1] La papelera se trae: 1 elemento, que puede restaurar desde la papelera.
   *[other] La papelera se trae: { $items } elementos, { $writing ->
        [0] ninguno de ellos con texto
       *[other] { $writing } de ellos con texto
    }, que puede restaurar desde la papelera.
}
import-scrivener-structure-not-compiled-line = { $count ->
    [one] 1 elemento que Scrivener dejaba fuera de su compilación también queda fuera de las exportaciones.
   *[other] { $count } elementos que Scrivener dejaba fuera de su compilación también quedan fuera de las exportaciones.
}
import-scrivener-structure-media-in-draft-line = { $count ->
    [one] 1 imagen o archivo del manuscrito se convierte en una nota, ya que un libro no puede contenerlo.
   *[other] { $count } imágenes o archivos del manuscrito se convierten en notas, ya que un libro no puede contenerlos.
}
import-scrivener-structure-left-out-line = { $count ->
    [one] Se deja fuera 1 carpeta de primer nivel, con { $items ->
        [one] 1 elemento
       *[other] { $items } elementos
    }.
   *[other] Se dejan fuera { $count } carpetas de primer nivel, con { $items ->
        [one] 1 elemento
       *[other] { $items } elementos
    }.
}
import-scrivener-structure-trash-left-out-line = { $items ->
    [1] La papelera se queda atrás: 1 elemento.
   *[other] La papelera se queda atrás: { $items } elementos, { $writing ->
        [0] ninguno de ellos con texto
       *[other] { $writing } de ellos con texto
    }.
}
import-scrivener-structure-card-image-left-behind-line = { $count ->
    [one] Se deja atrás 1 imagen de tarjeta de un elemento del manuscrito: se imprimiría en el libro.
   *[other] Se dejan atrás { $count } imágenes de tarjeta de elementos del manuscrito: se imprimirían en el libro.
}
import-scrivener-structure-default-template-not-linked-line = { $count ->
    [one] 1 carpeta tenía una plantilla predeterminada para los documentos nuevos que contiene. En Skribisto, solo una etiqueta de la biblia narrativa puede llevar una plantilla, así que ese vínculo no se conserva.
   *[other] { $count } carpetas tenían una plantilla predeterminada para los documentos nuevos que contienen. En Skribisto, solo una etiqueta de la biblia narrativa puede llevar una plantilla, así que esos vínculos no se conservan.
}
import-scrivener-structure-page-break-not-kept-line = { $count ->
    [one] 1 ajuste «Salto de página antes» no tiene cabida en Skribisto y no se conserva.
   *[other] { $count } ajustes «Salto de página antes» no tienen cabida en Skribisto y no se conservan.
}
import-scrivener-structure-compile-as-is-not-kept-line = { $count ->
    [one] 1 ajuste «Compilar tal cual» no tiene equivalente en Skribisto y no se conserva.
   *[other] { $count } ajustes «Compilar tal cual» no tienen equivalente en Skribisto y no se conservan.
}

## What the import did with the labels, the status list and the dates
import-scrivener-metadata-status-categories-guessed-line = { $count ->
    [one] A 1 estado que Skribisto no reconoce se le asigna una categoría según su lugar en la lista. Compruébelo en Configuración ▸ Proyecto ▸ Estados.
   *[other] A { $count } estados que Skribisto no reconoce se les asigna una categoría según su lugar en la lista. Compruébelos en Configuración ▸ Proyecto ▸ Estados.
}
import-scrivener-metadata-dates-unreadable-line = { $count ->
    [one] No se pudieron leer las fechas de 1 elemento: lleva la fecha de la importación.
   *[other] No se pudieron leer las fechas de { $count } elementos: llevan la fecha de la importación.
}
import-scrivener-metadata-labels-became-tags-line = { $tags ->
    [one] Los rótulos se convierten en 1 etiqueta, aplicada a { $rows ->
        [one] { $rows } elemento
       *[other] { $rows } elementos
    }.
   *[other] Los rótulos se convierten en { $tags } etiquetas, aplicadas a { $rows ->
        [one] { $rows } elemento
       *[other] { $rows } elementos
    }.
}
import-scrivener-metadata-statuses-became-ladder-line = { $rungs ->
    [one] La lista de estados se convierte en la lista de estados del proyecto: { $rungs } estado, aplicado a { $rows ->
        [one] { $rows } elemento
       *[other] { $rows } elementos
    }.
   *[other] La lista de estados se convierte en la lista de estados del proyecto: { $rungs } estados, aplicados a { $rows ->
        [one] { $rows } elemento
       *[other] { $rows } elementos
    }.
}
import-scrivener-metadata-ladder-seeded-line = { $count ->
    [1] El proyecto no tenía lista de estados, así que empieza con el estado predeterminado de Skribisto.
   *[other] El proyecto no tenía lista de estados, así que empieza con los { $count } estados predeterminados de Skribisto.
}
import-scrivener-metadata-label-renamed-line = El campo Rótulo se llamaba «{ $title }» en Scrivener. Ese nombre figura en los detalles de cada etiqueta.
import-scrivener-metadata-status-renamed-line = El campo Estado se llamaba «{ $title }» en Scrivener. Ese nombre figura en los detalles de cada estado.
import-scrivener-metadata-none-label-collapsed-line = { $count ->
    [one] El rótulo «{ $name }», guardado junto al rótulo vacío propio de Scrivener con el mismo nombre, se lee como ausencia de rótulo en 1 elemento.
   *[other] El rótulo «{ $name }», guardado junto al rótulo vacío propio de Scrivener con el mismo nombre, se lee como ausencia de rótulo en { $count } elementos.
}
import-scrivener-metadata-none-status-collapsed-line = { $count ->
    [one] El estado «{ $name }», guardado junto al estado vacío propio de Scrivener con el mismo nombre, se lee como ausencia de estado en 1 elemento.
   *[other] El estado «{ $name }», guardado junto al estado vacío propio de Scrivener con el mismo nombre, se lee como ausencia de estado en { $count } elementos.
}
import-scrivener-metadata-unnamed-label-line = Un rótulo sin nombre recibe como nombre su número, { $id }.
import-scrivener-metadata-unnamed-status-line = Un estado sin nombre recibe como nombre su número, { $id }.
import-scrivener-metadata-tag-name-disambiguated-line = { $count ->
    [one] 1 etiqueta recibe un nombre nuevo, porque otra etiqueta ya tiene el suyo.
   *[other] { $count } etiquetas reciben un nombre nuevo, porque otras etiquetas ya tienen el suyo.
}
import-scrivener-metadata-rung-names-disambiguated-line = { $count ->
    [one] 1 estado tiene el mismo nombre que otro, así que lleva un número tras su nombre. Compruébelo en Configuración ▸ Proyecto ▸ Estados.
   *[other] { $count } estados tienen el mismo nombre que otro, así que cada uno lleva un número tras su nombre. Compruébelos en Configuración ▸ Proyecto ▸ Estados.
}
import-scrivener-metadata-unused-labels-skipped-line = { $count ->
    [one] No se importa 1 rótulo que no lleva ningún documento.
   *[other] No se importan { $count } rótulos que no lleva ningún documento.
}
import-scrivener-metadata-labels-without-row-line = { $count ->
    [one] El rótulo de 1 elemento que no tiene lugar en el proyecto nuevo no llega.
   *[other] Los rótulos de { $count } elementos que no tienen lugar en el proyecto nuevo no llegan.
}
import-scrivener-metadata-statuses-without-row-line = { $count ->
    [one] El estado de 1 elemento que no tiene lugar en el proyecto nuevo no llega.
   *[other] Los estados de { $count } elementos que no tienen lugar en el proyecto nuevo no llegan.
}

import-scrivener-metadata-unnamed-keyword-line = Una palabra clave sin nombre recibe como nombre su número, { $id }.
import-scrivener-metadata-keywords-without-row-line = { $count ->
    [one] Las palabras clave de 1 elemento que no tiene lugar en el proyecto nuevo no llegan.
   *[other] Las palabras clave de { $count } elementos que no tienen lugar en el proyecto nuevo no llegan.
}
import-scrivener-metadata-custom-values-without-row-line = { $count ->
    [one] Los metadatos personalizados de 1 elemento que no tiene lugar en el proyecto nuevo no llegan.
   *[other] Los metadatos personalizados de { $count } elementos que no tienen lugar en el proyecto nuevo no llegan.
}
import-scrivener-metadata-bookmarks-without-row-line = { $count ->
    [one] Los marcadores de 1 elemento que no tiene lugar en el proyecto nuevo no llegan.
   *[other] Los marcadores de { $count } elementos que no tienen lugar en el proyecto nuevo no llegan.
}
import-scrivener-metadata-targets-without-row-line = { $count ->
    [one] El objetivo de 1 elemento que no tiene lugar en el proyecto nuevo no llega.
   *[other] Los objetivos de { $count } elementos que no tienen lugar en el proyecto nuevo no llegan.
}

## What the import does with the keywords, custom metadata, collections, bookmarks, targets,
## the deadline, the auto-complete list and the icons. Each line is said twice: on the Review
## page before the import, then in its report, so it reads the same at both times. `$note`
## and `$folder` are the titles the import gives a note and its "From Scrivener" folder;
## `$day` is a day as YYYY-MM-DD.
import-scrivener-metadata-pov-hint-line = { $count ->
    [1] El rótulo o valor de campo { $values } coincide con la entrada de la biblia narrativa { $matches }. Si indica el punto de vista, defínalo en el inspector: la importación no enlaza nada.
   *[other] Los rótulos o valores de campo { $values } coinciden con las entradas de la biblia narrativa { $matches }. Si indican puntos de vista, defínalos en el inspector: la importación no enlaza nada.
}
import-scrivener-metadata-checkbox-default-unknown-line = El campo de casilla «{ $field }» tiene un valor predeterminado que Skribisto no puede leer, así que no puede saber qué documentos están marcados por defecto. Solo llevan su etiqueta los documentos marcados uno a uno.
import-scrivener-metadata-goals-in-other-unit-line = { $count ->
    [one] 1 objetivo está definido en la otra unidad de recuento. Se conserva y aparece cuando cambie la unidad de recuento en Configuración ▸ Proyecto ▸ Estructura.
   *[other] { $count } objetivos están definidos en la otra unidad de recuento. Se conservan y aparecen cuando cambie la unidad de recuento en Configuración ▸ Proyecto ▸ Estructura.
}
import-scrivener-metadata-counting-differs-line = Scrivener calculaba el objetivo del borrador sobre documentos distintos de los que cuenta Skribisto. El objetivo del libro cuenta las escenas incluidas en las exportaciones.
import-scrivener-metadata-writing-days-not-read-line = Los días de escritura «{ $raw }» no se pueden leer, así que el Ritmo del libro cuenta todos los días como días de escritura. Cámbielos allí.
import-scrivener-metadata-script-mode-rows-line = { $count ->
    [one] 1 elemento estaba en modo guion en Scrivener. Skribisto conserva sus palabras como prosa, sin la maquetación de guion.
   *[other] { $count } elementos estaban en modo guion en Scrivener. Skribisto conserva sus palabras como prosa, sin la maquetación de guion.
}
import-scrivener-metadata-keywords-became-tags-line = { $tags ->
    [one] 1 palabra clave se convierte en etiqueta, aplicada a { $rows ->
        [one] { $rows } elemento
       *[other] { $rows } elementos
    }.
   *[other] { $tags } palabras clave se convierten en etiquetas, aplicadas a { $rows ->
        [one] { $rows } elemento
       *[other] { $rows } elementos
    }.
}
import-scrivener-metadata-keyword-hierarchy-flattened-line = { $count ->
    [one] 1 palabra clave estaba dentro de otra en Scrivener. Las etiquetas no tienen jerarquía: los detalles de su etiqueta indican dónde estaba.
   *[other] { $count } palabras clave estaban dentro de otras en Scrivener. Las etiquetas no tienen jerarquía: los detalles de cada etiqueta indican dónde estaba su palabra clave.
}
import-scrivener-metadata-fields-became-tags-line = { $fields ->
    [one] 1 campo de lista o de casilla se convierte en { $tags ->
        [one] { $tags } etiqueta, aplicada
       *[other] { $tags } etiquetas, aplicadas
    } a { $rows ->
        [one] { $rows } elemento
       *[other] { $rows } elementos
    }.
   *[other] { $fields } campos de lista y de casilla se convierten en { $tags ->
        [one] { $tags } etiqueta, aplicada
       *[other] { $tags } etiquetas, aplicadas
    } a { $rows ->
        [one] { $rows } elemento
       *[other] { $rows } elementos
    }.
}
import-scrivener-metadata-collections-became-tags-line = { $tags ->
    [one] 1 colección se convierte en etiqueta, aplicada a { $rows ->
        [one] { $rows } elemento
       *[other] { $rows } elementos
    }.
   *[other] { $tags } colecciones se convierten en etiquetas, aplicadas a { $rows ->
        [one] { $rows } elemento
       *[other] { $rows } elementos
    }.
}
import-scrivener-metadata-collection-order-lost-line = { $count ->
    [one] Una etiqueta no guarda ningún orden, así que no se conserva el orden de los documentos de 1 colección.
   *[other] Una etiqueta no guarda ningún orden, así que no se conserva el orden de los documentos de { $count } colecciones.
}
import-scrivener-metadata-bookmarks-became-links-line = { $count ->
    [one] 1 marcador se convierte en un enlace entre dos notas, o entre una escena y una entrada de la biblia narrativa: consulte Enlaces y Reparto.
   *[other] { $count } marcadores se convierten en enlaces entre notas, o entre escenas y entradas de la biblia narrativa: consulte Enlaces y Reparto.
}
import-scrivener-metadata-goals-kept-line = { $count ->
    [one] El objetivo que Scrivener fijaba para 1 documento pasa a ser su objetivo en Skribisto.
   *[other] Los objetivos que Scrivener fijaba para { $count } documentos pasan a ser sus objetivos en Skribisto.
}
import-scrivener-metadata-draft-target-kept-words-line = { $value ->
    [one] El objetivo del borrador, de 1 palabra, pasa a ser el objetivo del libro.
   *[other] El objetivo del borrador, de { $value } palabras, pasa a ser el objetivo del libro.
}
import-scrivener-metadata-draft-target-kept-characters-line = { $value ->
    [one] El objetivo del borrador, de 1 carácter, pasa a ser el objetivo del libro.
   *[other] El objetivo del borrador, de { $value } caracteres, pasa a ser el objetivo del libro.
}
import-scrivener-metadata-pace-planned-line = La fecha límite, { $day }, pone en marcha el Ritmo del libro, que reparte las palabras entre los días de escritura que eligió en Scrivener, a partir del día de la importación.
import-scrivener-metadata-pace-planned-day-off-line = La fecha límite, { $day }, pone en marcha el Ritmo del libro, que reparte las palabras entre los días de escritura que eligió en Scrivener, a partir del día de la importación. Como en Scrivener, la propia fecha límite es un día libre.
import-scrivener-metadata-dictionary-words-line = { $count ->
    [one] 1 palabra de autocompletado pasa al diccionario personal del proyecto, para que no se marque como error ortográfico.
   *[other] { $count } palabras de autocompletado pasan al diccionario personal del proyecto, para que no se marquen como errores ortográficos.
}
import-scrivener-metadata-listed-project-line = { $count ->
    [one] 1 dato del proyecto figura en la nota «{ $note }» de la carpeta «{ $folder }».
   *[other] { $count } datos del proyecto figuran en la nota «{ $note }» de la carpeta «{ $folder }».
}
import-scrivener-metadata-listed-project-bookmarks-line = { $count ->
    [one] 1 marcador del proyecto figura en la nota «{ $note }» de la carpeta «{ $folder }».
   *[other] { $count } marcadores del proyecto figuran en la nota «{ $note }» de la carpeta «{ $folder }».
}
import-scrivener-metadata-listed-replacements-line = { $count ->
    [one] 1 reemplazo de compilación figura en la nota «{ $note }» de la carpeta «{ $folder }». Skribisto no lo aplica.
   *[other] { $count } reemplazos de compilación figuran en la nota «{ $note }» de la carpeta «{ $folder }». Skribisto no los aplica.
}
import-scrivener-metadata-listed-auto-complete-line = { $count ->
    [one] 1 entrada de autocompletado de varias palabras figura en la nota «{ $note }» de la carpeta «{ $folder }».
   *[other] { $count } entradas de autocompletado de varias palabras figuran en la nota «{ $note }» de la carpeta «{ $folder }».
}
import-scrivener-metadata-listed-saved-searches-line = { $count ->
    [one] 1 búsqueda guardada figura en la nota «{ $note }» de la carpeta «{ $folder }».
   *[other] { $count } búsquedas guardadas figuran en la nota «{ $note }» de la carpeta «{ $folder }».
}
import-scrivener-metadata-listed-custom-metadata-line = { $count ->
    [one] 1 valor de metadatos personalizados que una etiqueta no puede contener figura en la nota «{ $note }» de la carpeta «{ $folder }».
   *[other] { $count } valores de metadatos personalizados que una etiqueta no puede contener figuran en la nota «{ $note }» de la carpeta «{ $folder }».
}
import-scrivener-metadata-listed-targets-line = { $count ->
    [one] 1 objetivo de documento figura en la nota «{ $note }» de la carpeta «{ $folder }».
   *[other] { $count } objetivos de documento figuran en la nota «{ $note }» de la carpeta «{ $folder }».
}
import-scrivener-metadata-listed-bookmarks-line = { $count ->
    [one] 1 marcador que ningún enlace de Skribisto puede mostrar figura en la nota «{ $note }» de la carpeta «{ $folder }».
   *[other] { $count } marcadores que ningún enlace de Skribisto puede mostrar figuran en la nota «{ $note }» de la carpeta «{ $folder }».
}
import-scrivener-metadata-targets-on-notes-line = { $count ->
    [one] 1 objetivo de documento está en una nota, cuyo texto no cuenta para ningún objetivo, así que no se convierte en objetivo.
   *[other] { $count } objetivos de documento están en notas, cuyo texto no cuenta para ningún objetivo, así que no se convierten en objetivos.
}
import-scrivener-metadata-targets-on-split-documents-line = { $count ->
    [one] 1 objetivo de documento está en un documento dividido por sus encabezados, cuyos capítulos ningún objetivo cuenta en conjunto, así que no se convierte en objetivo.
   *[other] { $count } objetivos de documento están en documentos divididos por sus encabezados, cuyos capítulos ningún objetivo cuenta en conjunto, así que no se convierten en objetivos.
}
import-scrivener-metadata-bookmark-ends-without-row-line = { $count ->
    [one] 1 marcador lleva a un documento que no tiene lugar en el proyecto nuevo, como una plantilla de documento. Figura con la ubicación que tenía el documento en Scrivener.
   *[other] { $count } marcadores llevan a documentos que no tienen lugar en el proyecto nuevo, como plantillas de documento. Figuran con la ubicación que tenían los documentos en Scrivener.
}
import-scrivener-metadata-unused-keywords-skipped-line = { $count ->
    [one] No se importa 1 palabra clave que no lleva ningún documento.
   *[other] No se importan { $count } palabras clave que no lleva ningún documento.
}
import-scrivener-metadata-list-value-invalid-line = { $count ->
    [one] 1 valor del campo de lista «{ $field }» no corresponde a ninguna de sus opciones y no se importa.
   *[other] { $count } valores del campo de lista «{ $field }» no corresponden a ninguna de sus opciones y no se importan.
}
import-scrivener-metadata-undefined-custom-field-line = { $count ->
    [one] 1 valor pertenece a un campo personalizado que el proyecto ya no define, y que Scrivener tampoco muestra. No se importa.
   *[other] { $count } valores pertenecen a campos personalizados que el proyecto ya no define, y que Scrivener tampoco muestra. No se importan.
}
import-scrivener-metadata-collection-members-missing-line = { $count ->
    [one] 1 documento de la colección «{ $name }» no tiene lugar en el proyecto nuevo, o ya no está en el proyecto de Scrivener.
   *[other] { $count } documentos de la colección «{ $name }» no tienen lugar en el proyecto nuevo, o ya no están en el proyecto de Scrivener.
}
import-scrivener-metadata-empty-collections-skipped-line = { $count ->
    [one] No se importa 1 colección que no contiene ningún documento.
   *[other] No se importan { $count } colecciones que no contienen ningún documento.
}
import-scrivener-metadata-bookmarks-unresolved-line = { $count ->
    [one] 1 marcador lleva a un documento que el proyecto de Scrivener ya no contiene, y no se importa.
   *[other] { $count } marcadores llevan a documentos que el proyecto de Scrivener ya no contiene, y no se importan.
}
import-scrivener-metadata-targets-in-pages-line = { $count ->
    [one] 1 objetivo de documento cuenta páginas, que Skribisto no cuenta, así que no se convierte en objetivo.
   *[other] { $count } objetivos de documento cuentan páginas, que Skribisto no cuenta, así que no se convierten en objetivos.
}
import-scrivener-metadata-targets-unreadable-line = { $count ->
    [one] No se importa 1 objetivo: su cifra no es un número, es negativa o supera lo que puede contener un objetivo.
   *[other] No se importan { $count } objetivos: sus cifras no son números, son negativas o superan lo que puede contener un objetivo.
}
import-scrivener-metadata-draft-target-in-pages-line = { $value ->
    [one] El objetivo del borrador, de 1 página, no se convierte en objetivo: Skribisto cuenta palabras o caracteres. Figura en la nota «{ $note }» de la carpeta «{ $folder }».
   *[other] El objetivo del borrador, de { $value } páginas, no se convierte en objetivo: Skribisto cuenta palabras o caracteres. Figura en la nota «{ $note }» de la carpeta «{ $folder }».
}
import-scrivener-metadata-draft-target-no-book-words-line = { $value ->
    [one] El objetivo del borrador, de 1 palabra, no se convierte en objetivo, porque el manuscrito forma varios libros, o ninguno. Figura en la nota «{ $note }» de la carpeta «{ $folder }».
   *[other] El objetivo del borrador, de { $value } palabras, no se convierte en objetivo, porque el manuscrito forma varios libros, o ninguno. Figura en la nota «{ $note }» de la carpeta «{ $folder }».
}
import-scrivener-metadata-draft-target-no-book-characters-line = { $value ->
    [one] El objetivo del borrador, de 1 carácter, no se convierte en objetivo, porque el manuscrito forma varios libros, o ninguno. Figura en la nota «{ $note }» de la carpeta «{ $folder }».
   *[other] El objetivo del borrador, de { $value } caracteres, no se convierte en objetivo, porque el manuscrito forma varios libros, o ninguno. Figura en la nota «{ $note }» de la carpeta «{ $folder }».
}
import-scrivener-metadata-deadline-passed-line = La fecha límite, { $day }, ya ha pasado, así que no pone en marcha el Ritmo del libro. Figura en la nota «{ $note }» de la carpeta «{ $folder }».
import-scrivener-metadata-deadline-no-writing-day-line = No queda ningún día de escritura entre el día de la importación y la fecha límite, { $day }, así que no pone en marcha el Ritmo del libro. Figura en la nota «{ $note }» de la carpeta «{ $folder }».
import-scrivener-metadata-deadline-template-line = La fecha límite, { $day }, se guardó en la plantilla, así que no pone en marcha el Ritmo del libro. Figura en la nota «{ $note }» de la carpeta «{ $folder }».
import-scrivener-metadata-deadline-no-word-goal-line = La fecha límite, { $day }, no pone en marcha el Ritmo del libro, porque el borrador no tiene un objetivo de palabras con el que planificar. Figura en la nota «{ $note }» de la carpeta «{ $folder }».
import-scrivener-metadata-deadline-character-goal-line = La fecha límite, { $day }, no pone en marcha el Ritmo del libro, porque el objetivo del borrador cuenta caracteres y el Ritmo cuenta palabras. Figura en la nota «{ $note }» de la carpeta «{ $folder }».
import-scrivener-metadata-deadline-no-book-line = La fecha límite, { $day }, no pone en marcha el Ritmo de ningún libro, porque el manuscrito forma varios libros, o ninguno. Figura en la nota «{ $note }» de la carpeta «{ $folder }».
import-scrivener-metadata-deadline-unreadable-line = La fecha límite no se puede leer, así que no pone en marcha el Ritmo del libro. Figura tal como está escrita en la nota «{ $note }» de la carpeta «{ $folder }».
import-scrivener-metadata-session-target-words-line = { $value ->
    [one] El objetivo de sesión de 1 palabra no se importa: en Skribisto, el objetivo de la sesión de escritura es un único ajuste para todos los proyectos. Figura en la nota «{ $note }» de la carpeta «{ $folder }».
   *[other] El objetivo de sesión de { $value } palabras no se importa: en Skribisto, el objetivo de la sesión de escritura es un único ajuste para todos los proyectos. Figura en la nota «{ $note }» de la carpeta «{ $folder }».
}
import-scrivener-metadata-session-target-characters-line = { $value ->
    [one] El objetivo de sesión de 1 carácter no se importa: en Skribisto, el objetivo de la sesión de escritura es un único ajuste para todos los proyectos, en palabras. Figura en la nota «{ $note }» de la carpeta «{ $folder }».
   *[other] El objetivo de sesión de { $value } caracteres no se importa: en Skribisto, el objetivo de la sesión de escritura es un único ajuste para todos los proyectos, en palabras. Figura en la nota «{ $note }» de la carpeta «{ $folder }».
}
import-scrivener-metadata-session-target-pages-line = { $value ->
    [one] El objetivo de sesión de 1 página no se importa: en Skribisto, el objetivo de la sesión de escritura es un único ajuste para todos los proyectos, en palabras. Figura en la nota «{ $note }» de la carpeta «{ $folder }».
   *[other] El objetivo de sesión de { $value } páginas no se importa: en Skribisto, el objetivo de la sesión de escritura es un único ajuste para todos los proyectos, en palabras. Figura en la nota «{ $note }» de la carpeta «{ $folder }».
}
import-scrivener-metadata-session-target-calculated-words-line = { $value ->
    [one] El objetivo de sesión de 1 palabra, que Scrivener calculó a partir de la fecha límite y de los días de escritura, no se importa: en Skribisto, el objetivo de la sesión de escritura es un único ajuste para todos los proyectos. Figura en la nota «{ $note }» de la carpeta «{ $folder }».
   *[other] El objetivo de sesión de { $value } palabras, que Scrivener calculó a partir de la fecha límite y de los días de escritura, no se importa: en Skribisto, el objetivo de la sesión de escritura es un único ajuste para todos los proyectos. Figura en la nota «{ $note }» de la carpeta «{ $folder }».
}
import-scrivener-metadata-session-target-calculated-characters-line = { $value ->
    [one] El objetivo de sesión de 1 carácter, que Scrivener calculó a partir de la fecha límite y de los días de escritura, no se importa: en Skribisto, el objetivo de la sesión de escritura es un único ajuste para todos los proyectos, en palabras. Figura en la nota «{ $note }» de la carpeta «{ $folder }».
   *[other] El objetivo de sesión de { $value } caracteres, que Scrivener calculó a partir de la fecha límite y de los días de escritura, no se importa: en Skribisto, el objetivo de la sesión de escritura es un único ajuste para todos los proyectos, en palabras. Figura en la nota «{ $note }» de la carpeta «{ $folder }».
}
import-scrivener-metadata-session-target-calculated-pages-line = { $value ->
    [one] El objetivo de sesión de 1 página, que Scrivener calculó a partir de la fecha límite y de los días de escritura, no se importa: en Skribisto, el objetivo de la sesión de escritura es un único ajuste para todos los proyectos, en palabras. Figura en la nota «{ $note }» de la carpeta «{ $folder }».
   *[other] El objetivo de sesión de { $value } páginas, que Scrivener calculó a partir de la fecha límite y de los días de escritura, no se importa: en Skribisto, el objetivo de la sesión de escritura es un único ajuste para todos los proyectos, en palabras. Figura en la nota «{ $note }» de la carpeta «{ $folder }».
}
import-scrivener-metadata-icons-dropped-line = { $count ->
    [one] 1 elemento tenía un icono en el archivador, que no llega: Skribisto muestra el icono de cada elemento según su tipo.
   *[other] { $count } elementos tenían iconos en el archivador, que no llegan: Skribisto muestra el icono de cada elemento según su tipo.
}

## What converting the prose did, as the import's report says it. `$count` is how many texts,
## or how many of a thing, as each line says; `$example` quotes the project.
import-scrivener-prose-unreadable-line = { $count ->
    [one] 1 texto no se pudo leer. Llega vacío.
   *[other] { $count } textos no se pudieron leer. Llegan vacíos.
}
import-scrivener-prose-comments-unreadable-line = { $count ->
    [one] No se pudieron leer los comentarios ni las notas al pie vinculadas de 1 texto. Se conservan las palabras que marcaban.
   *[other] No se pudieron leer los comentarios ni las notas al pie vinculadas de { $count } textos. Se conservan las palabras que marcaban.
}
import-scrivener-prose-read-as-plain-text-line = { $count ->
    [one] 1 texto estaba guardado como texto sin formato en lugar de texto con formato, y se lee como tal.
   *[other] { $count } textos estaban guardados como texto sin formato en lugar de texto con formato, y se leen como tales.
}
import-scrivener-prose-inline-note-unmatched-line = { $count ->
    [one] 1 marca de nota en línea no tiene pareja y se conserva como texto.
   *[other] { $count } marcas de nota en línea no tienen pareja y se conservan como texto.
}
import-scrivener-prose-style-unresolved-line = { $count ->
    [one] 1 marca de estilo nombra un estilo que el proyecto no define. Se elimina, y el texto conserva su formato.
   *[other] { $count } marcas de estilo nombran un estilo que el proyecto no define. Se eliminan, y el texto conserva su formato.
}
import-scrivener-prose-styles-lost-in-export-line = { $count ->
    [one] 1 párrafo puede ser un encabezado que perdió su estilo cuando Scrivener 3 exportó el proyecto a su formato anterior, y llega como un párrafo normal. Compruébelo, o importe el proyecto desde el que se exportó.
   *[other] { $count } párrafos pueden ser encabezados que perdieron su estilo cuando Scrivener 3 exportó el proyecto a su formato anterior, y llegan como párrafos normales. Compruébelos, o importe el proyecto desde el que se exportaron.
}
import-scrivener-prose-comment-anchor-without-entry-line = { $count ->
    [one] 1 comentario o nota al pie había perdido su contenido en Scrivener. Se conservan las palabras que marcaba, sin él.
   *[other] { $count } comentarios o notas al pie habían perdido su contenido en Scrivener. Se conservan las palabras que marcaban, sin ellos.
}
import-scrivener-prose-comment-entry-without-anchor-line = { $count ->
    [one] 1 comentario ya no tenía lugar en su texto y se adjunta al primer párrafo.
   *[other] { $count } comentarios ya no tenían lugar en su texto y se adjuntan al primer párrafo.
}
import-scrivener-prose-footnote-entry-without-anchor-line = { $count ->
    [one] 1 nota al pie vinculada ya no tenía lugar en su texto, así que se cita al final de ese texto.
   *[other] { $count } notas al pie vinculadas ya no tenían lugar en sus textos, así que cada una se cita al final de su propio texto.
}
import-scrivener-prose-range-comment-reading-line = { $count ->
    [one] 1 comentario de una versión anterior de Scrivener se ha colocado con otra lectura de su posición. Compruebe que está sobre las palabras correctas.
   *[other] { $count } comentarios de una versión anterior de Scrivener se han colocado con otra lectura de su posición. Compruebe que están sobre las palabras correctas.
}
import-scrivener-prose-range-comment-reading-markers-counted-line = { $count ->
    [one] 1 comentario de una versión anterior de Scrivener se ha colocado contando las marcas de sus notas en línea. Compruebe que está sobre las palabras correctas.
   *[other] { $count } comentarios de una versión anterior de Scrivener se han colocado contando las marcas de sus notas en línea. Compruebe que están sobre las palabras correctas.
}
import-scrivener-prose-range-comment-reading-pictures-not-counted-line = { $count ->
    [one] 1 comentario de una versión anterior de Scrivener se ha colocado sin contar sus imágenes. Compruebe que está sobre las palabras correctas.
   *[other] { $count } comentarios de una versión anterior de Scrivener se han colocado sin contar sus imágenes. Compruebe que están sobre las palabras correctas.
}
import-scrivener-prose-range-comment-approximate-line = { $count ->
    [one] 1 comentario de una versión anterior de Scrivener no encaja exactamente con ninguna lectura de su posición, y puede quedar desplazado unos caracteres.
   *[other] { $count } comentarios de una versión anterior de Scrivener no encajan exactamente con ninguna lectura de su posición, y pueden quedar desplazados unos caracteres.
}
import-scrivener-prose-document-link-unresolved-line = { $count ->
    [one] 1 enlace apunta a un documento que la importación no ha traído. Sus palabras se conservan.
   *[other] { $count } enlaces apuntan a documentos que la importación no ha traído. Sus palabras se conservan.
}
import-scrivener-prose-linked-picture-not-read-line = { $count ->
    [1] No se ha traído 1 imagen vinculada a un archivo de su disco, «{ $example }».
   *[other] No se han traído { $count } imágenes vinculadas a archivos de su disco, como «{ $example }».
}
import-scrivener-prose-heading-repeats-title-line = { $count ->
    [one] 1 texto empieza con un encabezado que repite su título, así que una exportación puede imprimir el título dos veces.
   *[other] { $count } textos empiezan con un encabezado que repite su título, así que una exportación puede imprimir el título dos veces.
}
import-scrivener-prose-markdown-as-text-line = { $count ->
    [one] 1 párrafo escrito en Markdown se conserva tal cual, como texto.
   *[other] { $count } párrafos escritos en Markdown se conservan tal cual, como texto.
}
import-scrivener-prose-citation-keys-as-text-line = { $count ->
    [one] 1 párrafo contiene claves de cita para un programa de bibliografía, conservadas como texto.
   *[other] { $count } párrafos contienen claves de cita para un programa de bibliografía, conservadas como texto.
}
import-scrivener-prose-placeholders-in-exported-text-line = { $count ->
    [1] 1 comodín de Scrivener, { $example }, se conserva como texto sin formato en lo que exporte. Sustitúyalo a mano.
   *[other] { $count } comodines de Scrivener, como { $example }, se conservan como texto sin formato en lo que exporte. Sustitúyalos a mano.
}
import-scrivener-prose-placeholders-in-other-text-line = { $count ->
    [1] 1 comodín de Scrivener, { $example }, se conserva como texto sin formato en un texto que no se exporta.
   *[other] { $count } comodines de Scrivener, como { $example }, se conservan como texto sin formato en textos que no se exportan.
}
import-scrivener-prose-ornament-lines-kept-line = { $count ->
    [one] 1 línea centrada de adornos puede marcar un separador de escena y se conserva tal como está escrita.
   *[other] { $count } líneas centradas de adornos pueden marcar separadores de escena y se conservan tal como están escritas.
}
import-scrivener-prose-long-document-line = { $count ->
    [one] 1 texto es muy largo y puede tardar en abrirse.
   *[other] { $count } textos son muy largos y pueden tardar en abrirse.
}
import-scrivener-prose-pictures-without-description-line = { $count ->
    [one] 1 imagen no tiene descripción para los lectores que no pueden verla. Puede añadir una en el editor.
   *[other] { $count } imágenes no tienen descripción para los lectores que no pueden verlas. Puede añadir una en el editor.
}
import-scrivener-prose-struck-text-compiled-away-line = { $count ->
    [one] Se conserva el texto tachado de 1 texto. La compilación de Scrivener lo dejaba fuera del libro, pero una exportación de Skribisto lo imprime.
   *[other] Se conserva el texto tachado de { $count } textos. La compilación de Scrivener lo dejaba fuera del libro, pero una exportación de Skribisto lo imprime.
}
import-scrivener-prose-compile-deleted-style-printed-line = { $count ->
    [1] El texto con el estilo «{ $example }», que la compilación de Scrivener eliminaba, se conserva como prosa, y una exportación de Skribisto lo imprime.
   *[other] El texto con { $count } estilos que la compilación de Scrivener eliminaba, como «{ $example }», se conserva como prosa, y una exportación de Skribisto lo imprime.
}
import-scrivener-prose-compile-raw-style-printed-line = { $count ->
    [1] El texto con el estilo «{ $example }», que la compilación de Scrivener transmitía como código para otro programa, se conserva como prosa, y una exportación de Skribisto lo imprime.
   *[other] El texto con { $count } estilos que la compilación de Scrivener transmitía como código para otro programa, como «{ $example }», se conserva como prosa, y una exportación de Skribisto lo imprime.
}
import-scrivener-prose-left-out-after-document-line = { $count ->
    [one] Se deja fuera 1 fragmento de un archivo dañado, situado después de su final.
   *[other] Se dejan fuera { $count } fragmentos de archivos dañados, situados después de su final.
}
import-scrivener-prose-left-out-header-or-footer-line = { $count ->
    [one] Se deja fuera 1 cabecera o pie de página: no forma parte del texto.
   *[other] Se dejan fuera { $count } cabeceras y pies de página: no forman parte del texto.
}
import-scrivener-prose-left-out-list-label-picture-line = { $count ->
    [one] Se deja fuera 1 imagen usada como viñeta de lista.
   *[other] Se dejan fuera { $count } imágenes usadas como viñetas de lista.
}
import-scrivener-prose-left-out-line = { $count ->
    [one] Se deja fuera 1 fragmento de archivo que no pertenece a ningún texto.
   *[other] Se dejan fuera { $count } fragmentos de archivos que no pertenecen a ningún texto.
}
import-scrivener-prose-undecodable-characters-line = { $count ->
    [one] 1 carácter no se pudo descodificar y aparece como signo de sustitución (�) donde estaba.
   *[other] { $count } caracteres no se pudieron descodificar y aparecen como signos de sustitución (�) donde estaban.
}
import-scrivener-prose-inline-notes-merged-line = { $count ->
    [one] 1 nota en línea que Scrivener había dividido donde cambiaba su formato se vuelve a unir.
   *[other] { $count } notas en línea que Scrivener había dividido donde cambiaba su formato se vuelven a unir.
}
import-scrivener-prose-inline-notes-joined-across-paragraphs-line = { $count ->
    [1] 1 nota en línea escrita párrafo a párrafo se une en una sola nota.
   *[other] { $count } notas en línea escritas párrafo a párrafo se unen en notas de varios párrafos.
}
import-scrivener-prose-blank-notes-dropped-line = { $count ->
    [one] Se deja fuera 1 nota en línea vacía.
   *[other] Se dejan fuera { $count } notas en línea vacías.
}
import-scrivener-prose-annotations-became-comments-line = { $count ->
    [one] 1 anotación en línea se ha convertido en comentario.
   *[other] { $count } anotaciones en línea se han convertido en comentarios.
}
import-scrivener-prose-annotation-only-document-kept-line = { $count ->
    [one] 1 documento no contenía más que una anotación. Sus palabras se conservan como prosa.
   *[other] { $count } documentos no contenían más que una anotación. Sus palabras se conservan como prosa.
}
import-scrivener-prose-preserve-formatting-passages-line = { $count ->
    [one] 1 pasaje marcado con «Conservar formato» llega como texto normal.
   *[other] { $count } pasajes marcados con «Conservar formato» llegan como texto normal.
}
import-scrivener-prose-nested-note-unwrapped-line = { $count ->
    [one] 1 nota dentro de un comentario o de una nota al pie se conserva como texto, donde estaba.
   *[other] { $count } notas dentro de comentarios o de notas al pie se conservan como texto, donde estaban.
}
import-scrivener-prose-style-not-carried-line = { $count ->
    [1] El estilo «{ $example }» no tiene equivalente en Skribisto. Su texto conserva sus palabras.
   *[other] { $count } estilos, como «{ $example }», no tienen equivalente en Skribisto. Su texto conserva sus palabras.
}
import-scrivener-prose-document-links-dropped-line = { $count ->
    [one] 1 enlace a otro documento no tiene cabida en Skribisto. Sus palabras se conservan.
   *[other] { $count } enlaces a otros documentos no tienen cabida en Skribisto. Sus palabras se conservan.
}
import-scrivener-prose-document-links-kept-line = { $count ->
    [one] 1 enlace a otro documento se conserva como referencia.
   *[other] { $count } enlaces a otros documentos se conservan como referencias.
}
import-scrivener-prose-other-links-kept-as-text-line = { $count ->
    [one] 1 enlace a otro proyecto o a un archivo se conserva como texto.
   *[other] { $count } enlaces a otros proyectos o a archivos se conservan como texto.
}
import-scrivener-prose-typed-indents-removed-line = { $count ->
    [one] Se elimina 1 sangría tecleada al principio de un párrafo.
   *[other] Se eliminan { $count } sangrías tecleadas al principio de párrafos.
}
import-scrivener-prose-left-indents-not-kept-line = { $count ->
    [one] 1 párrafo con sangría desde el margen pierde su sangría.
   *[other] { $count } párrafos con sangría desde el margen pierden su sangría.
}
import-scrivener-prose-blank-lines-removed-line = { $count ->
    [one] Se elimina 1 párrafo vacío entre otros dos.
   *[other] Se eliminan { $count } párrafos vacíos entre otros.
}
import-scrivener-prose-blank-lines-became-scene-breaks-line = { $count ->
    [one] 1 línea en blanco entre párrafos se ha convertido en separador de escena.
   *[other] { $count } líneas en blanco entre párrafos se han convertido en separadores de escena.
}
import-scrivener-prose-ornaments-became-scene-breaks-line = { $count ->
    [one] 1 línea de adornos se ha convertido en separador de escena.
   *[other] { $count } líneas de adornos se han convertido en separadores de escena.
}
import-scrivener-prose-table-first-row-header-line = { $count ->
    [one] 1 tabla empieza ahora con una fila de encabezado.
   *[other] { $count } tablas empiezan ahora con una fila de encabezado.
}
import-scrivener-prose-table-cells-joined-line = { $count ->
    [one] 1 celda de tabla contenía varios párrafos, ahora unidos en uno.
   *[other] { $count } celdas de tabla contenían varios párrafos, ahora unidos en uno.
}
import-scrivener-prose-formatting-lost-colour-line = { $count ->
    [one] Se elimina el color del texto en 1 texto.
   *[other] Se elimina el color del texto en { $count } textos.
}
import-scrivener-prose-formatting-lost-highlight-line = { $count ->
    [one] Se elimina el resaltado en 1 texto.
   *[other] Se elimina el resaltado en { $count } textos.
}
import-scrivener-prose-formatting-lost-small-caps-line = { $count ->
    [one] Las versalitas aparecen como letras normales en 1 texto.
   *[other] Las versalitas aparecen como letras normales en { $count } textos.
}
import-scrivener-prose-formatting-lost-all-caps-line = { $count ->
    [one] El texto mostrado en mayúsculas aparece tal como se tecleó en 1 texto.
   *[other] El texto mostrado en mayúsculas aparece tal como se tecleó en { $count } textos.
}
import-scrivener-prose-formatting-lost-hidden-line = { $count ->
    [one] Se deja fuera el texto oculto en 1 texto.
   *[other] Se deja fuera el texto oculto en { $count } textos.
}
import-scrivener-prose-formatting-lost-line = { $count ->
    [one] Se elimina el formato para el que Skribisto no tiene cabida en 1 texto.
   *[other] Se elimina el formato para el que Skribisto no tiene cabida en { $count } textos.
}
import-scrivener-prose-justified-text-not-kept-line = { $count ->
    [one] 1 párrafo justificado sigue en su lugar el estilo de la exportación.
   *[other] { $count } párrafos justificados siguen en su lugar el estilo de la exportación.
}
import-scrivener-prose-comment-colours-lost-line = { $count ->
    [one] 1 comentario pierde su color.
   *[other] { $count } comentarios pierden su color.
}
import-scrivener-prose-control-characters-removed-line = { $count ->
    [one] Se elimina 1 carácter de control invisible.
   *[other] Se eliminan { $count } caracteres de control invisibles.
}
import-scrivener-prose-note-on-heading-lost-line = { $count ->
    [one] 1 nota al pie o comentario en un encabezado de capítulo no pudo quedarse allí: un comentario pasa al primer párrafo del capítulo y una nota al pie se deja fuera.
   *[other] { $count } notas al pie o comentarios en encabezados de capítulo no pudieron quedarse allí: un comentario pasa al primer párrafo del capítulo y una nota al pie se deja fuera.
}
import-scrivener-prose-media-file-copied-line = { $count ->
    [one] 1 archivo de investigación que Skribisto no puede abrir se ha copiado junto al proyecto nuevo. Su nota indica dónde.
   *[other] { $count } archivos de investigación que Skribisto no puede abrir se han copiado junto al proyecto nuevo. Sus notas indican dónde.
}
import-scrivener-prose-media-file-not-carried-line = { $count ->
    [one] 1 archivo de investigación que Skribisto no puede abrir se queda en su proyecto de Scrivener. Su nota lo nombra.
   *[other] { $count } archivos de investigación que Skribisto no puede abrir se quedan en su proyecto de Scrivener. Sus notas los nombran.
}

## When the import fails
import-scrivener-error-refused = { $refusal } No se ha escrito nada.
import-scrivener-error-question = El proyecto plantea ahora una pregunta que su revisión no respondió. Elija «Revisar la importación» y luego impórtelo. No se ha escrito nada.
import-scrivener-error-not-accepted = Antes de poder importarse, el proyecto necesita que marque una casilla en el paso Proyecto, y no se marcó. Elija «Revisar la importación», marque la casilla y luego impórtelo. No se ha escrito nada.
import-scrivener-error-changed = El proyecto ha cambiado después de que usted lo revisara. Elija «Revisar la importación» y luego impórtelo. No se ha escrito nada.
import-scrivener-error-choice = La estructura elegida en la revisión ya no encaja con este proyecto. Elija «Revisar la importación» y luego impórtelo. No se ha escrito nada.

export-title = Exportar
export-close = Cerrar
export-scope-label = Qué
export-format-label = Formato
export-style-label = Estilo
export-path-label = Archivo
export-browse = Examinar…
export-show-non-exportable = Mostrar lo no exportable
export-choose-empty = No hay ningún proyecto entre el que elegir.
export-cancel = Cancelar
export-export = Exportar
export-save-dialog-title = Exportar a un archivo
export-section-what = Qué exportar
export-section-style = Estilo
export-section-destination = Destino
export-custom-selection = Selección personalizada
export-selected-count = { $count } seleccionados
export-chip-chapters-none = Capítulos: ninguno
export-chip-chapters-numbered = Capítulos: numerados
export-chip-chapters-title = Capítulos: solo título
export-chip-chapters-both = Capítulos: número + título
export-chip-scene-break-glyph = Separador de escena: { $glyph }
export-chip-scene-break-blank = Separador de escena: línea en blanco
export-chip-scene-break-none = Separador de escena: ninguno
export-chip-major-break-glyph = Separador mayor: { $glyph }
export-chip-major-break-blank = Separador mayor: línea en blanco
export-chip-major-break-none = Separador mayor: ninguno
export-chip-major-break-same = Ambos separadores iguales
export-chip-spacing-single = Interlineado: sencillo
export-chip-spacing-onehalf = Interlineado: 1½
export-chip-spacing-double = Interlineado: doble
export-chip-notes-included = Notas incluidas
export-chip-notes-excluded = Notas excluidas
export-format-docx = Word
export-format-odt = LibreOffice
export-format-html = HTML
export-format-markdown = Markdown
export-format-djot = Djot
export-format-text = Texto
export-format-latex = LaTeX
export-format-epub = EPUB
export-format-pdf = PDF
export-overwrite-title = ¿Reemplazar el archivo existente?
export-overwrite-text = «{ $name }» ya existe. ¿Sustituirlo?
export-progress-title = Exportando…
export-cancelled = Exportación cancelada
export-done = { $count ->
    [one] { $count } elemento exportado
   *[other] { $count } elementos exportados
}
export-open-file = Abrir
export-show-in-folder = Mostrar en la carpeta
export-error-title = No se pudo exportar
export-error-details = Detalles

backup-progress-start = Iniciando…
backup-progress-retention = Limpiando las copias de seguridad antiguas…
backup-progress-done = Listo
backup-progress-destination = Destino { $i } de { $n }
backup-details = Detalles
backup-issues-title = Problemas con la copia de seguridad
backup-failed-title = La copia de seguridad ha fallado
backup-complete-prune-warning = Copia de seguridad completada ({ $ok } guardadas, { $skipped } ya al día). No se pudieron eliminar algunas copias de seguridad antiguas

search = Buscar
search-query-placeholder = Buscar…
search-replace-placeholder = Reemplazar por…
search-replace-toggle = Mostrar u ocultar reemplazar
search-replace-all = Reemplazar todo
search-opt-case = Coincidir mayúsculas y minúsculas
search-opt-whole-word = Palabra completa
search-opt-diacritics = Coincidir acentos
search-scope-body = Cuerpo
search-scope-title = Título
search-scope-synopsis = Sinopsis
search-scope-label = Rótulo
search-facet-book = Libros
search-facet-part = Partes
search-facet-chapter = Capítulos
search-facet-scene = Escenas
search-facet-note = Notas
search-facet-folder = Carpetas
search-tip-case = Coincidir mayúsculas y minúsculas: distingue entre mayúsculas y minúsculas, de modo que «Elena» y «elena» son coincidencias distintas.
search-tip-whole-word = Palabra completa: solo coincide con palabras completas, de modo que «gato» no se encuentra dentro de «gatopardo».
search-tip-diacritics = Coincidir acentos: trata las letras acentuadas como distintas, de modo que «cafe» no coincide con «café».
search-tip-body = Cuerpo: busca en la prosa de las escenas y las notas.
search-tip-title = Título: busca en los títulos de los elementos del archivador.
search-tip-synopsis = Sinopsis: busca en el resumen de cada elemento de escritura.
search-tip-label = Rótulo: busca en la nota breve que aparece bajo el título de un elemento del archivador.
search-tip-comment = Comentarios: busca en el texto de los hilos de comentarios y sus respuestas. Un comentario trata sobre el manuscrito en lugar de formar parte de él, así que tiene su propio interruptor, y al reemplazar, las coincidencias en comentarios quedan sin marcar hasta que usted las marque.
search-tip-book = Libros: el contenedor del libro y sus marcadores de inicio y fin.
search-tip-part = Partes: los divisores de nivel de parte.
search-tip-chapter = Capítulos: los capítulos, sea cual sea la forma en que el proyecto los guarde.
search-tip-scene = Escenas: los elementos que contienen su prosa.
search-tip-note = Notas: notas libres.
search-tip-folder = Carpetas: carpetas de organización simples y separadores.
search-tip-paratext = Un texto que pertenece al libro pero no a su historia: un prefacio, una dedicatoria, un epílogo. Nunca se cuenta en el manuscrito.
search-tip-preserve-case = Conservar mayúsculas: el reemplazo adopta las mayúsculas de lo encontrado, de modo que «ELENA» se convierte en «MARTA» y «Elena» en «Marta».
search-tip-replace = Reemplazar: muestra el campo de reemplazo y Reemplazar todo.
search-error = La búsqueda ha fallado: { $message }
search-no-matches = No hay coincidencias
search-count =
    { $matches ->
        [one] { $matches } coincidencia
       *[other] { $matches } coincidencias
    } en { $items ->
        [one] { $items } documento
       *[other] { $items } documentos
    }
search-count-truncated =
    { $matches ->
        [one] { $matches } coincidencia
       *[other] { $matches } coincidencias
    } en { $items ->
        [one] { $items } documento
       *[other] { $items } documentos
    } (solo los primeros resultados)
search-occurrences = { $count }
search-field-body = Cuerpo
search-field-title = Título
search-field-synopsis = Sinopsis
search-field-label = Rótulo
search-field-epigraph = Epígrafe
search-field-comment = Comentario
search-field-comment-reply = Respuesta
search-field-footnote = Nota al pie
search-include-in-replace = Incluir en Reemplazar todo
search-collapse-all = Contraer todos los resultados
search-replace-here = Reemplazar esta
search-dismiss = Descartar de los resultados
search-undo-dismiss = Recuperar el último resultado descartado
search-replace-nothing = (nada)
search-replace-confirm-title = ¿Reemplazar todas las coincidencias?
search-replace-confirm-text =
    ¿Reemplazar { $occurrences ->
        [one] { $occurrences } aparición
       *[other] { $occurrences } apariciones
    } de «{ $query }» por «{ $replacement }» en { $items ->
        [one] { $items } documento
       *[other] { $items } documentos
    }? Podrá deshacerlo después desde la notificación.
search-replace-done-title = Reemplazo completado
search-replace-done =
    { $occurrences ->
        [one] { $occurrences } aparición reemplazada
       *[other] { $occurrences } apariciones reemplazadas
    } en { $items ->
        [one] { $items } documento
       *[other] { $items } documentos
    }.
search-replace-done-skipped =
    { $occurrences ->
        [one] { $occurrences } aparición reemplazada
       *[other] { $occurrences } apariciones reemplazadas
    } en { $items ->
        [one] { $items } documento
       *[other] { $items } documentos
    }. { $skipped ->
        [one] { $skipped } campo omitido
       *[other] { $skipped } campos omitidos
    } (han cambiado desde la búsqueda).
search-replace-undo = Deshacer
search-replace-failed-title = El reemplazo ha fallado
search-replace-skipped-title = Nada reemplazado
search-replace-skipped-body =
    { $fields ->
        [one] Este texto ha cambiado
       *[other] { $fields } de estos textos han cambiado
    } desde la búsqueda, así que { $fields ->
        [one] no se ha tocado
       *[other] no se han tocado
    }. Vuelva a buscar para ver dónde están ahora las palabras.
search-replace-undo-failed-title = No se pudo deshacer
search-preview = Vista previa
search-preview-empty = Seleccione un resultado para verlo aquí
search-preview-no-prose = Esta coincidencia no tiene texto editable
search-preview-prompt = Para ver una vista previa aquí, haga una búsqueda.
search-preview-open-search = Buscar en el proyecto
search-preview-footnote-prompt = Esta coincidencia está en el propio texto de una nota al pie. Ábrala en el panel Notas al pie para verla y editarla.
search-preview-open-footnotes = Abrir Notas al pie

find-placeholder = Buscar en este documento…
find-count = { $current } de { $total }
find-no-results = No hay resultados
find-close = Cerrar la búsqueda
find-previous = Coincidencia anterior (Mayús+Intro)
find-next = Coincidencia siguiente (Intro)
find-opt-case = Coincidir mayúsculas y minúsculas
find-opt-whole-word = Palabra completa
find-replace-toggle = Mostrar u ocultar reemplazo (Ctrl+R)
find-replace-placeholder = Reemplazar por…
find-replace = Reemplazar
find-replace-all = Reemplazar todo
find-preserve-case = Conservar mayúsculas

dict-download-title = Descargando { $name }…
dict-download-done = { $name } instalado
dict-download-failed = No se pudo descargar { $name }: { $error }
dict-removed = { $name } eliminado
dict-accept-first = Aceptar la licencia antes de descargar { $name }
settings-dict-tab-installed = Instalados
settings-dict-tab-get-more = Obtener más
settings-dict-tab-personal = Palabras personales
dict-installed-empty = Todavía no se han encontrado diccionarios en este equipo.
dict-get-more-search = Buscar idiomas
dict-system-badge = en su sistema
dict-unusable-badge = inutilizable
dict-download-button = Descargar
dict-downloading = Descargando…
dict-installed-label = Instalados
dict-remove = Quitar
dict-view-license = Ver licencia
dict-approx-size = ~{ $size }
dict-personal-empty = Aún no hay palabras personales en este proyecto.
dict-personal-add = Añadir
dict-personal-placeholder = Añadir una palabra…
dict-missing-toast = { $count ->
    [one] Este proyecto usa { $count } diccionario que no tiene instalado
   *[other] Este proyecto usa { $count } diccionarios que no tiene instalados
}
dict-missing-action = Obtener diccionarios
dict-license-title = Licencia de { $name }
dict-license-accept = Aceptar y descargar
dict-license-cancel = Cancelar
dict-license-close = Cerrar
dict-add-button = Añadir diccionario…
dict-add-title = Añadir un diccionario
dict-add-close = Cerrar
dict-add-name = Nombre
dict-add-name-placeholder = p. ej., Mi diccionario de latín
dict-add-code = Código de idioma
dict-add-code-placeholder = p. ej., la o fr-FR-x-custom
dict-add-code-hint = Una etiqueta corta de su elección. Es lo que elegirá como idioma de un documento.
dict-add-aff = Archivo de afijos (.aff)
dict-add-dic = Lista de palabras (.dic)
dict-add-submit = Añadir
dict-add-cancel = Cancelar
dict-add-name-required = Dé un nombre al diccionario.
dict-add-code-required = Introduzca un código de idioma.
dict-add-code-invalid = Use solo letras, cifras y - _ .
dict-add-code-reserved = Es el código de un diccionario integrado. Elija otro.
dict-add-file-required = Elija un archivo.
dict-add-file-missing = Ese archivo no existe.
dict-add-code-taken = Ya hay un diccionario instalado para ese código.
dict-add-done = { $name } añadido
dict-add-unusable = Estos archivos no son un diccionario utilizable: { $error }
dict-add-failed = No se pudo añadir el diccionario: { $error }
inspector-export = Exportar
inspector-exportable = Incluir en las exportaciones
ctx-number = &Numerar este capítulo
ctx-unnumber = &No numerar este capítulo
inspector-numbering = Numeración
inspector-numbered = Numerado
inspector-numbered-tip = Este capítulo ocupa su lugar en la numeración del libro. Desactívelo para un prólogo, un epílogo o un interludio.
inspector-numbered-tip-more = Un capítulo sin numerar sigue en el libro exactamente igual: su encabezado, su prosa y su recuento de palabras no cambian. Simplemente no imprime ningún número ni consume uno: el capítulo que sigue a un prólogo es el capítulo uno, no el dos. Dejarlo fuera de la exportación es otro interruptor, más arriba, y ese sí quita el capítulo del libro por completo.
inspector-apply-to-children = Aplicar a los hijos
inspector-milestone = Fecha del hito
inspector-milestone-none = Sin fecha
inspector-milestone-clear = Borrar
inspector-tags = Etiquetas
inspector-aliases = También conocido como
inspector-dict-language = Idioma
inspector-apply-language-to-children = Aplicar el idioma a los hijos
settings-page-language = Idioma
settings-field-dict-language = Idiomas
dict-tradeoff-hint = Cada idioma adicional acepta más palabras, así que se detectan menos errores.
lang-inherit-hint = Heredado: esta escena usa los idiomas del libro o del proyecto.
lang-pill-list = Idiomas
lang-pill-add = Añadir un idioma
lang-pill-remove = Quitar { $name }
lang-pill-mute = Desactivar la revisión ortográfica para { $name }
lang-pill-unmute = Activar la revisión ortográfica para { $name }

settings-page-personal-dictionary = Diccionario personal
settings-user-dict-desc = Las palabras que añada aquí se consideran bien escritas en toda la aplicación y nunca vuelven a marcarse.
settings-user-dict-add-placeholder = Añadir una palabra…
settings-user-dict-add = Añadir palabra
settings-user-dict-search = Filtrar palabras
settings-user-dict-count = { $count ->
    [one] { $count } palabra
   *[other] { $count } palabras
}
settings-user-dict-rename = Cambiar nombre
settings-user-dict-remove = Quitar
settings-user-dict-empty = Todavía no hay palabras. Añada las palabras que el corrector ortográfico debe ignorar, o importe una lista.
settings-user-dict-import = Importar…
settings-user-dict-export = Exportar…
settings-user-dict-import-tip = Añade palabras desde un archivo de texto sin formato, una palabra por línea. Las palabras existentes se conservan.
settings-user-dict-export-tip = Guarda toda la lista en un archivo de texto sin formato, una palabra por línea.
settings-user-dict-txt-filter = Archivos de texto
settings-user-dict-duplicate = Ya está en el diccionario
settings-user-dict-imported = { $count ->
    [one] { $count } palabra importada
   *[other] { $count } palabras importadas
} ({ $duplicates } ya presentes).
settings-user-dict-import-failed = No se pudo leer la lista de palabras: { $error }
settings-user-dict-exported = { $count ->
    [one] { $count } palabra guardada.
   *[other] { $count } palabras guardadas.
}
settings-user-dict-export-failed = No se pudo guardar la lista de palabras: { $error }

settings-page-text-replacements = Sustituciones de texto
settings-text-repl-desc = Sustituye abreviaturas por el texto completo mientras escribe: «pq» se convierte en «porque» en cuanto escribe un espacio o un signo de puntuación.
settings-text-repl-enable = Usar sustituciones de texto en este proyecto
settings-text-repl-disabled-hint = Active esta opción para definir abreviaturas que se expanden mientras escribe.
settings-text-repl-add = Añadir regla
settings-text-repl-trigger-placeholder = Abreviatura
settings-text-repl-replacement-placeholder = En qué se convierte
settings-text-repl-added = «{ $trigger }» añadida
settings-text-repl-duplicate = «{ $trigger }» ya tiene una regla
settings-text-repl-filter = Filtrar reglas
settings-text-repl-count = { $n ->
    [one] 1 regla
   *[other] { $n } reglas
}
settings-text-repl-row-enabled = Usar esta regla
settings-text-repl-delete = Eliminar la regla de { $trigger }
settings-text-repl-deleted = Regla de «{ $trigger }» eliminada
settings-text-repl-empty = Todavía no hay reglas.
settings-text-repl-csv-filter = Archivos CSV
settings-text-repl-import = Importar…
settings-text-repl-export = Exportar…
settings-text-repl-imported = Importadas: { $added }; omitidas: { $skipped }
settings-text-repl-exported = { $n ->
    [one] 1 regla exportada
   *[other] { $n } reglas exportadas
}

editor-menu-no-suggestions = No hay sugerencias
editor-menu-add-to-dictionary = Añadir «{ $word }» al diccionario
editor-menu-add-words-to-dictionary = Añadir las palabras seleccionadas al diccionario
editor-dict-added = «{ $word }» añadida a su diccionario.
editor-dict-added-multi = { $count ->
    [one] Se ha añadido { $count } palabra a su diccionario.
   *[other] Se han añadido { $count } palabras a su diccionario.
}
toast-undo = Deshacer

undo-superseded-title = Ese paso ya no es el último
undo-superseded-body = Desde entonces ha cambiado otra cosa en este proyecto. Deshacer revertiría eso en su lugar, así que no lo ha hecho. No se ha revertido nada de lo que hizo este paso. Use Editar ▸ Deshacer para retroceder usted mismo por el historial.
undo-failed = No se pudo deshacer: { $error }

menu-edit = &Editar
menu-edit-undo = &Deshacer
menu-edit-redo = &Rehacer
menu-edit-undo-target = &Deshacer { $target }
menu-edit-redo-target = &Rehacer { $target }
shortcut-name-edit-undo = Deshacer
shortcut-name-edit-redo = Rehacer
undo-target-typing = la escritura
undo-target-project = el último cambio en este proyecto
prose-history-reset-title = Historial de escritura reiniciado
prose-history-reset-body = { $count ->
    [one] Una escena abierta se ha restaurado a un estado anterior, así que su historial de escritura ya no es válido.
   *[other] { $count } escenas abiertas se han restaurado a un estado anterior, así que su historial de escritura ya no es válido.
}
undo-target-trash = el envío a la papelera
undo-target-restore = la restauración desde la papelera
undo-target-delete-forever = la eliminación definitiva
undo-target-replace-all = el reemplazo en todo el proyecto
undo-target-import = la importación de documentos
undo-target-duplicate = la duplicación
undo-target-move = el desplazamiento
undo-target-merge = la fusión de dos escenas
undo-target-split = la división de una escena
undo-target-promote = el cambio de tipo
undo-target-tidy-titles = la ordenación de los títulos de capítulo
undo-target-import-tags = la importación de etiquetas
undo-target-import-templates = la importación de plantillas de notas
undo-target-create = la creación
undo-target-remove = la eliminación
undo-target-rename = el cambio de nombre
undo-target-edit = esa modificación
menu-edit-find = &Buscar…
menu-edit-find-next = Buscar si&guiente
menu-edit-find-prev = Buscar a&nterior
menu-edit-replace = Buscar y ree&mplazar…
menu-edit-find-in-project = Buscar en el pro&yecto…
menu-edit-replace-in-project = Reempla&zar en el proyecto…
undo-frozen = Deshacer («Siempre hacia adelante» está activado)
redo-frozen = Rehacer («Siempre hacia adelante» está activado)
menu-edit-cut = Cor&tar
menu-edit-copy = &Copiar
menu-edit-paste = &Pegar
menu-edit-paste-plain = Pegar &sin formato
menu-edit-select-all = S&eleccionar todo
shortcut-name-edit-cut = Cortar
shortcut-name-edit-copy = Copiar
shortcut-name-edit-paste = Pegar
shortcut-name-edit-paste-plain = Pegar sin formato
shortcut-name-edit-select-all = Seleccionar todo

titlebar-spellcheck-on = La revisión ortográfica está activada. Haga clic para dejar de revisar (F7)
titlebar-spellcheck-off = La revisión ortográfica está desactivada. Haga clic para volver a revisar (F7)
titlebar-distraction-free = Entrar en el modo sin distracciones (F11)
menu-spellcheck = Revisar la &ortografía
menu-comments = &Comentarios
settings-page-spellcheck = Revisión ortográfica
settings-group-spellcheck = Revisión ortográfica
settings-spellcheck-enabled = Revisar la ortografía mientras escribo
settings-spellcheck-hint = Subraya las palabras que no conoce ningún diccionario instalado. Al desactivarla se detiene toda la revisión, en todos los proyectos, hasta que vuelva a activarla. Para dejar de revisar un solo idioma, quite su marca en el campo Idioma del proyecto o de un elemento.

menu-trash = &Papelera
trash-title = Papelera
trash-empty-state = La papelera está vacía.
trash-empty-button = Vaciar la papelera…
trash-restore = &Restaurar
trash-restore-to = Restaurar &en…
trash-delete-forever = &Eliminar para siempre
trash-restored-ok = { $count } restaurados.
trash-restore-error = No se pudo restaurar: { $error }
trash-restore-orphaned = El lugar original de este elemento ya no existe. Elija dónde restaurarlo.
trash-restore-no-project = No hay ningún proyecto abierto, así que no hay nada que restaurar.
trash-delete-no-project = No hay ningún proyecto abierto, así que no hay nada que eliminar.
trash-empty-confirm-title = ¿Vaciar la papelera?
trash-empty-confirm-text = { $count ->
    [one] { $count } entrada de la papelera se eliminará definitivamente.
   *[other] { $count } entradas de la papelera se eliminarán definitivamente.
} Salen de la papelera para siempre. Deshacer aún puede recuperarlas hasta que haga otra cosa.
trash-emptied-title = Papelera vaciada
trash-emptied-body = Todo lo que había en la papelera se ha eliminado definitivamente.
trash-empty-no-project = No hay ningún proyecto abierto, así que no hay papelera que vaciar.
trash-delete-forever-confirm-title = ¿Eliminar para siempre?
trash-delete-forever-confirm-text = { $count ->
    [one] { $count } elemento se eliminará definitivamente.
   *[other] { $count } elementos se eliminarán definitivamente.
} Salen de la papelera para siempre. Deshacer aún puede recuperarlos hasta que haga otra cosa.
trash-deleted-title = Eliminado para siempre
trash-deleted-body = { $count } eliminados definitivamente.
trash-undo = Deshacer
trash-restore-picker-title = Restaurar en…
trash-restore-to-confirm-title = ¿Restaurar aquí?
trash-restore-to-confirm-text = ¿Restaurar «{ $item }» en «{ $destination }»?
trash-restore-picker-restore-here = Restaurar aquí
trash-restore-picker-cancel = Cancelar
trash-restore-picker-empty = Todavía no hay archivadores. Cree uno primero.
trash-banner-title = Este elemento está en la papelera
trash-banner-description = No aparecerá en el esquema ni en las exportaciones hasta que lo restaure.
trash-banner-restore = Restaurar…
trash-tab-tooltip = En la papelera

settings-styles-sheet-yes = Sí
settings-styles-sheet-no = No
settings-styles-sheet-font = Tipo de letra
settings-styles-sheet-indent = Sangría de primera línea
settings-styles-sheet-para-spacing = Espaciado entre párrafos
settings-styles-sheet-page = Tamaño de la página
settings-styles-sheet-margins = Márgenes (sup./der./inf./izq.)
settings-styles-sheet-title-page = Portada del libro
settings-styles-sheet-heading-language = Idioma de los encabezados
settings-styles-sheet-auto = Según el texto
settings-styles-sheet-digits = Cifras
settings-styles-sheet-direction = Dirección del texto
settings-styles-sheet-formats = Formatos
settings-styles-sheet-all-formats = Todos

settings-styles-editor-group = Editor de estilo
settings-styles-editor-missing = Ese estilo ya no está disponible.

settings-styles-page-letter = Carta
settings-styles-digits-western = Occidental (0–9)
settings-styles-digits-eastern-arabic = Árabe oriental (٠–٩)
settings-styles-direction-ltr = De izquierda a derecha
settings-styles-direction-rtl = De derecha a izquierda

format-dock-title = Formato
format-panel-empty = Coloque el cursor en una escena, una nota o una sinopsis para ver las opciones de formato.
format-group-marks = Texto
format-group-block = Párrafo
format-group-lists = Listas
format-group-tables = Tabla
format-group-breaks = Separadores de escena
format-superscript = Superíndice
format-subscript = Subíndice
format-link = Enlace…
link-dialog-insert-title = Insertar enlace
link-dialog-edit-title = Editar enlace
link-dialog-text-label = Texto
link-dialog-text-placeholder = las palabras que el lector ve
link-dialog-url-label = Enlaces a
link-dialog-url-placeholder = example.com
link-dialog-insert = Insertar
link-dialog-apply = Aplicar
link-dialog-cancel = Cancelar
link-dialog-remove = Quitar el enlace
link-scheme-refused = Solo se pueden abrir enlaces web y de correo electrónico. No se ha abierto { $url }.
menu-format-link = &Enlace…
format-clear = Borrar formato
format-heading = Nivel de encabezado
format-heading-normal = Texto normal
format-heading-1 = Encabezado 1
format-heading-2 = Encabezado 2
format-heading-3 = Encabezado 3
format-heading-4 = Encabezado 4
format-heading-5 = Encabezado 5
format-heading-6 = Encabezado 6
format-align-left = Alinear a la izquierda
format-align-center = Centrar
format-direction-rtl = Párrafo de derecha a izquierda
format-blockquote = Cita en bloque
format-list-bullet = Lista con viñetas
format-list-numbered = Lista numerada
format-indent = Aumentar sangría
format-outdent = Reducir sangría
format-table-insert = Insertar tabla
format-table-row-above = Insertar fila encima
format-table-row-below = Insertar fila debajo
format-table-col-before = Insertar columna antes
format-table-col-after = Insertar columna después
format-table-row-delete = Eliminar fila
format-table-col-delete = Eliminar columna
format-table-remove = Quitar la tabla

menu-format-marks-bold = &Negrita
menu-format-marks-italic = Cu&rsiva
menu-format-marks-underline = Subra&yado
menu-format-marks-strike = Ta&chado
menu-format-marks-superscript = Su&períndice
menu-format-marks-subscript = Su&bíndice
menu-format-marks-clear = B&orrar formato
menu-format-heading = Encabe&zado
menu-format-heading-normal = Texto &normal
menu-format-heading-1 = Encabezado &1
menu-format-heading-2 = Encabezado &2
menu-format-heading-3 = Encabezado &3
menu-format-heading-4 = Encabezado &4
menu-format-heading-5 = Encabezado &5
menu-format-heading-6 = Encabezado &6
menu-format-alignment = &Alineación
menu-format-align-left = Alinear a la &izquierda
menu-format-align-center = &Centrar
menu-format-direction = Direcci&ón
menu-format-direction-auto = &Automático
menu-format-direction-ltr = De &izquierda a derecha
menu-format-direction-rtl = De &derecha a izquierda
menu-format-blockquote = C&ita en bloque
menu-format-lists = &Listas
menu-format-list-bullet = &Lista con viñetas
menu-format-list-numbered = L&ista numerada
menu-format-indent = &Aumentar sangría
menu-format-outdent = &Reducir sangría
menu-format-table = &Tabla
menu-format-table-insert = &Insertar tabla
menu-format-table-2x2 = &2 x 2
menu-format-table-3x3 = &3 x 3
menu-format-table-4x4 = &4 x 4
menu-format-table-row-above = Insertar fila &encima
menu-format-table-row-below = Insertar fila &debajo
menu-format-table-col-before = Insertar co&lumna antes
menu-format-table-col-after = Insertar columna des&pués
menu-format-table-row-delete = Eliminar &fila
menu-format-table-col-delete = Eliminar &columna
menu-format-table-remove = &Quitar la tabla

menu-format-typo-marks = Marca de &formato
menu-format-typo-nbsp = &Espacio de no separación
menu-format-typo-nbhyphen = &Guion de no separación
menu-format-typo-soft-hyphen = Guion &opcional
menu-format-typo-nnbsp = Espacio &estrecho de no separación
menu-format-typo-zwsp = Espacio de &anchura cero
menu-format-typo-word-joiner = &Unión de palabras


menu-about = &Acerca de Skribisto…
about-title = Acerca de Skribisto
about-version = Versión { $version }
about-update-available = Versión { $version } está disponible
about-update-see-notes = Ver qué ha cambiado
about-update-flathub = Flathub mantiene esta copia actualizada.
about-update-distro = Su distribución mantiene esta copia actualizada.
about-tagline = Una aplicación para escribir novelas y ficción de largo aliento, escrita en Rust con el kit de herramientas Teksilo.
about-license = Publicado bajo la Licencia Pública General de GNU, versión 3.
about-copyright = © 2026 Cyril Jacquet
about-close = Cerrar

native-menu-about = Acerca de { $app }
native-menu-hide = Ocultar { $app }
native-menu-hide-others = Ocultar otros
native-menu-show-all = Mostrar todo
native-menu-services = Servicios
native-menu-quit = Salir de { $app }
native-menu-settings = Configuración…
native-menu-window = Ventana
native-menu-minimize = Minimizar
native-menu-zoom = Zoom

window-title = { $title } - { $app }
window-title-numbered = { $title } - { $app } (Ventana { $n })
window-title-empty = { $app }

first-run-window-title = Bienvenida
first-run-title = Configurar { $app }
first-run-body = { $app } guarda su configuración por separado de Skribisto, así que empieza vacío. Puede copiar ahora sus preferencias, proyectos recientes, diccionarios y disposición de ventanas.
first-run-from = Copiar desde
first-run-to = Copiar en
first-run-copy-note = No se mueve ni se elimina nada. Skribisto conserva toda su propia configuración y sigue funcionando exactamente igual que antes.
first-run-import = Importar la configuración
first-run-start-fresh = Empezar de cero
first-run-import-failed = No se pudo importar parte de la configuración: { $error }

settings-page-punctuation = Puntuación predeterminada
settings-group-punctuation = Puntuación inteligente
settings-page-work-punctuation = Puntuación
settings-punctuation-override = Dar a este proyecto sus propias reglas de puntuación
settings-punctuation-override-hint = Desactivado: el proyecto sigue la preferencia de la aplicación. Estas reglas viajan dentro del .skrib, así que un coautor que abra el archivo escribe con la misma tipografía.
settings-punctuation-dashes = Convertir -- en semirraya y --- en raya
settings-punctuation-ellipsis = Convertir ... en puntos suspensivos
settings-punctuation-quotes = Comillas y apóstrofos tipográficos
settings-quote-style = Comillas
settings-quote-style-locale = Según el idioma
settings-quote-style-curly = “Dobles”
settings-quote-style-curly-single = ‘Simples’
settings-quote-style-guillemets = «Angulares»
settings-quote-style-low-high = „Bajas-altas“
settings-punctuation-spacing = Espacio antes de ; : ! ?
settings-punctuation-spacing-hint = La tipografía francesa pone un espacio estrecho de no separación antes de ; ! ? y uno completo antes de :. Solo se aplica al texto escrito en francés. El espacio dentro de las comillas angulares « » lo aportan las propias comillas.
settings-punctuation-sample = Su idioma le da
settings-punctuation-app-hint = Lo que hace cada proyecto salvo que tenga sus propias reglas en Proyecto ▸ Puntuación.
settings-punctuation-dialogue = Abrir con una raya de diálogo un párrafo que empieza por «- »
settings-punctuation-dialogue-hint = Para los idiomas que marcan el diálogo con una raya en lugar de comillas: francés, español, ruso y otros. Solo actúa al principio mismo de un párrafo.
settings-punctuation-auto-lists = Empezar una lista con un párrafo que empieza por "- ", "* " o "1. "
settings-punctuation-auto-lists-hint = El párrafo se convierte en una lista con viñetas o numerada. Donde la raya de diálogo está activada y el idioma tiene una, "- " abre en su lugar una réplica. Pulse Retroceso justo después para recuperar lo que escribió.

statusbar-go-to = Ir a…
go-to-placeholder = Buscar en el archivador
go-to-no-matches = Ningún elemento coincide con esa búsqueda.
menu-go-to = I&r a…

settings-page-distraction-free-themes = Temas sin distracciones
settings-themes-builtin = Temas integrados
settings-themes-builtin-badge = Integrado
settings-themes-user = Mis temas
settings-themes-editor-group = Editar tema
settings-themes-editor-empty = Elija un tema en «Mis temas» para editarlo.
settings-themes-use = Usar
settings-themes-duplicate = Duplicar
settings-themes-edit = Editar
settings-themes-delete = Eliminar
settings-themes-export = Exportar…
settings-themes-import = Importar…
settings-themes-copy-suffix = copia
settings-themes-json-filter = Tema (JSON)
settings-themes-imported = Tema importado
settings-themes-import-failed = No se pudo importar ese tema
settings-themes-exported = Tema exportado
settings-themes-export-failed = No se pudo exportar ese tema
settings-themes-low-contrast = contraste bajo
settings-themes-low-contrast-band = el resaltado oculta el texto
settings-themes-field-name = Nombre
settings-themes-field-paper = Página
settings-themes-field-ink = Texto
settings-themes-field-general = Fondo
settings-themes-field-widget-text = Texto de la barra de controles
settings-themes-field-caret-band = Resaltado alrededor del cursor

statusbar-focus-settings = Configuración sin distracciones
statusbar-focus-manage-themes = Gestionar temas…

comments-title = Comentarios
comments-document-title = Este documento
comments-empty-project = Todavía no hay comentarios en este proyecto.
comments-empty-document = No hay comentarios en este documento.
comments-filter-all = Todos
comments-filter-open = Abiertos
comments-filter-resolved = Resueltos
comments-filter-orphaned = Huérfanos
comments-status-open = Abierto
comments-status-resolved = Resuelto
comments-status-orphaned = Ha perdido su texto
comments-orphan-snippet = (el texto comentado ya no existe)
comments-status-unplaced = Sin posición en el texto
comments-unplaced-snippet = (no anclado a ningún texto)
comments-reply-count = { $count ->
    [one] { $count } respuesta
   *[other] { $count } respuestas
}
comments-menu-resolve = Resolver
comments-menu-reopen = Reabrir
comments-menu-delete = Eliminar
comments-sort-document = En orden de documento
comments-sort-newest = Más recientes primero
comments-sort-reader = Agrupados por lector
comments-unsigned-author = Sin firma
comments-agreement = { $count ->
    [one] { $count } lector
   *[other] { $count } lectores
}
comments-menu-add = Añadir comentario
comments-menu-add-paragraph = Comentar este párrafo
overview-col-comments = Comentarios
overview-col-total-comments = Total de comentarios
comments-card-placeholder = Escriba un comentario…
comments-card-unknown-author = Autor desconocido
comments-unsigned-toast = Sus comentarios no están firmados: no hay ningún nombre configurado para usted en este equipo.
comments-unsigned-action = Indicar su nombre
comments-card-reply = Responder
comments-card-reply-placeholder = Responder…
comments-card-actions = Acciones del comentario
comments-reply-actions = Acciones de la respuesta
comments-menu-delete-reply = Eliminar la respuesta
comments-menu-delete-all = Eliminar todos los comentarios de aquí
comments-deleted-toast = Comentario eliminado
comments-reply-deleted-toast = Respuesta eliminada
comments-deleted-all-toast = { $count ->
    [one] { $count } comentario eliminado
   *[other] { $count } comentarios eliminados
}
comments-undo = Deshacer

analysis-segment = Análisis
analysis-scope-book = Análisis de este libro
analysis-run = Ejecutar el análisis
analysis-stale = Ha cambiado desde el último análisis
analysis-not-run = Todavía no se ha analizado.
analysis-running = Leyendo el manuscrito…
analysis-failed = El análisis no pudo terminar.
analysis-no-scenes = Todavía no hay escenas en este libro.

analysis-shape = Forma

analysis-arrivals = Llegadas
analysis-arrivals-explainer = Cómo ha llegado el texto a este proyecto mientras ha estado abierto. Indica por qué vía llegaron los caracteres, y nada en absoluto sobre quién los escribió: quien redacta en otro sitio y pega, ha pegado, y quien dicta, ha dictado. Aquí no hay ninguna cifra a la que aspirar.
analysis-arrivals-scope = Todo este proyecto, no solo este libro.
analysis-arrivals-session = Desde que lo abrió. Al cerrar el proyecto, el recuento vuelve a empezar.
analysis-arrivals-nothing = Todavía no ha llegado ningún texto en esta sesión.
analysis-arrivals-typed = Tecleado
analysis-arrivals-pasted = Pegado
analysis-arrivals-dictated = Dictado
analysis-arrivals-imported = Importado
analysis-arrivals-programmatic = Insertado por la aplicación
analysis-arrivals-count = { $count ->
    [one] { $count } carácter
   *[other] { $count } caracteres
}
analysis-arrivals-none = ninguno

analysis-words-per-scene = Palabras por escena
analysis-median-words = La escena mediana de este libro tiene { $count ->
    [one] { $count } palabra.
   *[other] { $count } palabras.
}
analysis-median-line = Mediana: { $count ->
    [one] { $count } palabra
   *[other] { $count } palabras
}
analysis-dialogue = Diálogo
analysis-dialogue-unsupported = Todavía no se mide el diálogo en este idioma.

analysis-footnote-words = Palabras en notas al pie
analysis-footnote-words-count = { $count ->
    [one] { $count } palabra está en las notas al pie de este libro, separada del total del manuscrito.
   *[other] { $count } palabras están en las notas al pie de este libro, separadas del total del manuscrito.
}
analysis-footnote-words-pending = Contando las notas al pie…

analysis-ignore-empty = Ignorar los textos en los que aún no se ha escrito nada
analysis-empty-hidden = { $count ->
        [one] { $count } texto vacío oculto.
       *[other] { $count } textos vacíos ocultos.
    }
analysis-all-texts-empty = Todos los textos de este libro siguen vacíos.

binder-filter-count = { $shown } de { $total } mostrados
binder-filter-clear = Borrar
binder-filter-none = Nada coincide con «{ $query }».

settings-paratext-intro = Las páginas preliminares y finales con las que puede empezar un proyecto nuevo. Cada estructura pertenece a una tradición editorial, y los títulos de sus páginas están escritos en el idioma de esa tradición. Puede renombrarlos libremente una vez creado el proyecto.
settings-paratext-structures = Estructuras
settings-paratext-broken = No se pudo leer
settings-paratext-edit = Editar
settings-paratext-duplicate = Duplicar
settings-paratext-delete = Eliminar
settings-paratext-new = Nueva estructura
settings-paratext-save = Guardar
settings-paratext-editor-hint = Un nombre, las páginas que se crean antes del manuscrito y las que se crean después. Dónde acaban es cosa suya: una vez que existe el proyecto, son elementos normales.


image-insert = Insertar &imagen…
image-no-project = Abra un proyecto antes de insertar una imagen.
image-choose-title = Elija una imagen
image-filter-label = Imágenes
image-large-title = Esta es una imagen grande
image-large-text =
    { $name } tiene { $megapixels } megapíxeles ({ $width }×{ $height }).
    Conservarla tal cual guarda su archivo original en el proyecto: viaja
    con cada copia de seguridad y cada exportación. Optimizar guarda en su lugar
    una copia más pequeña, de hasta 2560 píxeles en su lado más largo.
image-large-keep = Conservar el original
image-large-downscale = Optimizar
image-large-remember = Hacer esto a partir de ahora y no volver a preguntar
image-not-recorded = La imagen se guardó, pero no se pudo registrar en el proyecto.
image-describe-title = Describir la imagen
image-describe-explain = Lo que muestra la imagen, para un lector que no puede verla. No forma parte del manuscrito: nunca se cuenta, se busca ni se exporta como prosa.
image-describe-placeholder = un faro contra un cielo gris
image-resize-title = Redimensionar la imagen
image-resize-explain = Un porcentaje del tamaño con el que se muestra ahora. 100 la deja como está.
image-resize-invalid = Introduzca un número entre 1 y 1000.
image-menu-describe = &Describir la imagen…
image-menu-resize = &Redimensionar la imagen…
image-menu-reset-size = &Tamaño original

cover-heading = Cubierta
cover-pick = Elegir…
cover-change = Cambiar…
cover-remove = Quitar
cover-none = Aún no hay cubierta
cover-missing = Falta el archivo de la imagen en el proyecto
cover-choose-title = Elija una cubierta
cover-set = Se ha establecido la cubierta.
cover-cleared = Se ha quitado la cubierta. La imagen sigue en el proyecto.
card-label = Rótulo
card-exportable = Exportado
card-numbered = Numerado
card-created = Creado
card-modified = Modificado
card-yes = sí
card-no = no
card-type = Tipo
card-position = Posición
card-children = Hijos
card-goal = Objetivo
card-words = Palabras
card-synopsis = Sinopsis
card-point-of-view = Punto de vista
card-aliases = También conocido como

export-orphan-footnotes-title = Notas al pie sin referencia
export-comments-dropped =
    { $count ->
        [one] Un comentario no se pudo colocar en el texto exportado y se ha omitido.
       *[other] { $count } comentarios no se pudieron colocar en el texto exportado y se han omitido.
    }
export-orphan-footnotes =
    { $count ->
        [one] Una nota al pie ya no tiene ninguna referencia en el manuscrito. Su texto no aparecerá en el libro exportado.
       *[other] { $count } notas al pie ya no tienen ninguna referencia en el manuscrito. Su texto no aparecerá en el libro exportado.
    }


menu-footnotes = &Notas al pie
footnotes-title = Notas al pie
footnotes-insert = Insertar nota al &pie
footnotes-empty = Todavía no hay notas al pie. Ponga el cursor en una escena y use + arriba, o Ctrl+Alt+F.
footnotes-filter-all = Todos
footnotes-filter-document = Este documento
footnotes-filter-orphaned = Huérfanas
footnotes-orphaned = Ya nada remite a esta nota
footnotes-untitled-home = Sin título
footnotes-body-placeholder = la nota en sí misma
footnotes-insert-tooltip = Insertar una nota al pie en el cursor (Ctrl+Alt+F)
footnotes-actions = Acciones de la nota al pie
footnotes-delete = &Eliminar la nota y su referencia
footnotes-no-project = Abra un proyecto antes de insertar una nota al pie.
footnotes-no-caret = Ponga el cursor en el texto de una escena para insertar ahí una nota al pie.
footnotes-not-created = No se pudo añadir la nota al pie al proyecto.
footnotes-deleted-toast = Nota eliminada
footnotes-undo-delete = Deshacer

import-document-title = Importar documentos
import-document-close = Cerrar
import-document-step-files = Archivos
import-document-step-review = Revisión
import-document-step-destination = Destino
import-document-step-reconcile = Combinar
import-document-reconcile-hint = Algunos de estos elementos ya los tiene. Indique qué debe pasar con cada uno.
import-document-reconcile-all-new = Nada de este archivo coincide con su proyecto: todos los elementos se añadirán como nuevos.
import-document-duplicate-returns = Estos archivos son varias copias del mismo manuscrito que vuelve. Impórtelas de una en una: la correspondencia solo alinea una copia devuelta con su proyecto, así que importar varias a la vez añadiría una segunda copia del libro en lugar de fusionarla.
import-document-hunk-added = Aceptar este párrafo nuevo
import-document-hunk-removed = Quitar este párrafo
import-document-hunk-changed = Aceptar esta reescritura
import-document-col-stray-prose = Su texto
import-document-stray-as-paratext = Conservar como paratexto
import-document-stray-discard = Descartar el texto
import-document-col-current = En su proyecto
import-document-col-incoming = En este archivo
import-document-col-status = Estado
import-document-col-action = Qué hacer
import-document-status-identical = Igual
import-document-status-editor-edited = Lo editaron ellos
import-document-status-you-edited = Lo editó usted
import-document-status-conflict = Lo editaron ambos
import-document-status-different = Diferente
import-document-status-new = Nuevo
import-document-status-missing = No está en este archivo
import-document-status-moved = Movido
import-document-action-comments-only = Solo comentarios
import-document-action-take-import = Usar esta versión
import-document-action-keep-current = Conservar la mía
import-document-action-create-new = Añadir como nuevo
import-document-action-ignore = Omitir
import-document-compare = Comparar
import-document-compare-legend = Su versión comparada con la de este archivo. Solo lectura.
import-document-compare-close = Cerrar
import-document-drop-title = Suelte los documentos aquí
import-document-drop-hint = Markdown (.md), Word (.docx), OpenDocument (.odt) y texto sin formato (.txt)
import-document-browse = Examinar…
import-document-move-up = Subir
import-document-move-down = Bajar
import-document-remove-file = Quitar
import-document-file-count = { $count ->
    [one] 1 archivo
   *[other] { $count } archivos
}
import-document-no-files = Todavía no se ha elegido ningún archivo.
import-document-col-included = Importar
import-document-col-title = Título
import-document-col-type = Tipo
import-document-col-words = Palabras
import-document-col-breaks = Separadores
import-document-col-comments = Comentarios
import-document-col-footnotes = Notas al pie
import-document-col-epigraph = Epígrafe
import-document-col-source = Origen
import-document-level-rules = Niveles de encabezado
import-document-level-n = Encabezado { $level }
import-document-add-top-level = Añadir nivel superior
import-document-add-top-level-tooltip = Inserta un libro por encima de todos los elementos analizados, para archivos de capítulos que no tienen encabezado de libro
import-document-destination = Destino
import-document-destination-hint = Elija un archivador o un elemento: los elementos nuevos se colocan dentro de una carpeta, o después de una escena.
import-document-destination-empty = Todavía no hay archivadores. Cree uno primero.
import-document-plan-empty = Aún no hay nada que importar.
# How many pictures the items being imported bring into the project, under the
# review tree. Shown only when there is at least one.
import-document-pictures = { $count ->
    [1] Con estos documentos llega una imagen.
   *[other] Con estos documentos llegan { $count } imágenes.
}
import-document-summary = { $rows ->
    [one] 1 elemento
   *[other] { $rows } elementos
} · { $breaks ->
    [one] 1 separador de escena
   *[other] { $breaks } separadores de escena
}
import-document-back = Atrás
import-document-cancel = Cancelar
import-document-analyse = Siguiente
import-document-analysing = Leyendo los documentos…
import-document-step-analysing = Lectura
import-document-cancel-analysis = Dejar de leer
import-document-analyse-failed = No se pudieron leer los documentos.
import-document-details = Detalles
import-document-import = Importar
import-document-done = { $count ->
    [one] 1 elemento importado
   *[other] { $count } elementos importados
}
import-document-undo = Deshacer
import-diagnostic-file-unreadable = No se pudo leer «{ $path }»: { $detail }. Los demás archivos se importan igualmente.
import-diagnostic-nested-too-deep = No se ha leído «{ $path }»: su contenido está anidado a más de { $limit } niveles de profundidad. Ningún documento real se acerca a eso, y leerlo podría bloquear Skribisto, así que se deja fuera. Los demás archivos se importan igualmente.
import-diagnostic-archive-too-large = No se ha leído «{ $path }»: es un archivo comprimido diseñado para descomprimirse en algo mucho mayor de lo que es, lo que podría agotar la memoria y bloquear Skribisto, así que se deja fuera. Los demás archivos se importan igualmente.
import-diagnostic-lossy-decode = { $count ->
    [one] Un carácter de «{ $path }» no se pudo descodificar. Vuelva a guardar el archivo en UTF-8 para conservarlo.
   *[other] { $count } caracteres de «{ $path }» no se pudieron descodificar. Vuelva a guardar el archivo en UTF-8 para conservarlos.
}
import-diagnostic-decoded-from-bom = «{ $path }» se ha descodificado como { $detail }, no como UTF-8.
import-diagnostic-empty-file = «{ $path }» está vacío.
import-diagnostic-no-headings = «{ $path }» no tiene encabezados, así que llega como un único elemento.
import-diagnostic-unsupported-format = No hay nada que lea archivos «.{ $detail }», así que se ha omitido «{ $path }».
import-diagnostic-front-matter-not-flat = Metadatos iniciales de «{ $path }»: «{ $detail }» no es un valor simple y se ha omitido.
import-diagnostic-footnotes-degraded = { $count ->
    [one] Una nota al pie de «{ $path }» llega como texto sin formato: las notas al pie no se leen desde Markdown.
   *[other] { $count } notas al pie de «{ $path }» llegan como texto sin formato: las notas al pie no se leen desde Markdown.
}
import-diagnostic-footnote-not-carried = { $count ->
    [one] Una nota al pie de «{ $path }» no se pudo traer. Las demás sí.
   *[other] { $count } notas al pie de «{ $path }» no se pudieron traer. Las demás sí.
}
import-diagnostic-raw-html-dropped = { $count ->
    [one] Se ha descartado un bloque de HTML sin procesar de «{ $path }».
   *[other] Se han descartado { $count } bloques de HTML sin procesar de «{ $path }».
}
import-diagnostic-nested-break-dropped = { $count ->
    [one] Se ha descartado un separador de escena dentro de una cita o lista de «{ $path }». Solo se conserva un separador en su propia línea.
   *[other] Se han descartado { $count } separadores de escena dentro de citas o listas de «{ $path }». Solo se conserva un separador en su propia línea.
}
# A picture the document shows that does not come with it, one sentence per reason.
# $detail is the picture as the document names it: a file name, a path inside the
# document, or a web address. Nothing is left in the text where it was.
import-diagnostic-image-not-ingested-format = En «{ $path }», la imagen «{ $detail }» está en un formato que Skribisto no puede mostrar, así que se ha dejado fuera. Las imágenes PNG, JPEG, WebP, GIF, TIFF o BMP sí se importan; guárdela en uno de esos formatos e insértela a mano.
import-diagnostic-image-not-ingested-remote = En «{ $path }», la imagen «{ $detail }» está en la web. Las imágenes nunca se descargan durante una importación, así que se ha dejado fuera.
import-diagnostic-image-not-ingested-outside = En «{ $path }», la imagen «{ $detail }» está fuera del documento y de su carpeta, así que se ha dejado fuera. Solo se importan las imágenes guardadas en el documento o en la propia carpeta del documento.
import-diagnostic-image-not-ingested-missing = En «{ $path }», no se encontró la imagen «{ $detail }», así que se ha dejado fuera.
import-diagnostic-image-not-ingested-too-large = En «{ $path }», la imagen «{ $detail }» es más grande de lo que admite un proyecto, así que se ha dejado fuera.
import-diagnostic-image-not-ingested-unreadable = En «{ $path }», la imagen «{ $detail }» no se pudo leer como imagen, así que se ha dejado fuera.
import-diagnostic-image-not-ingested-heading = En «{ $path }», la imagen «{ $detail }» está en un encabezado. Cada encabezado se convierte en el título de un elemento del archivador, y un título no admite imágenes, así que la imagen se ha dejado fuera.
import-diagnostic-image-not-ingested-note = En «{ $path }», la imagen «{ $detail }» está en una nota al pie, una nota final o un comentario. Estos conservan sus palabras pero ninguna imagen, así que se ha dejado fuera.
import-diagnostic-image-not-ingested-html = En «{ $path }», la imagen «{ $detail }» está escrita en HTML, y una importación deja fuera el HTML, así que se ha dejado fuera con él. Escríbala como imagen de Markdown para importarla.
import-diagnostic-image-not-ingested-document-full = En «{ $path }», la imagen «{ $detail }» se ha dejado fuera: las imágenes de este documento superan en conjunto lo que admite una sola importación. Importe el documento por partes, o inserte el resto a mano.
import-diagnostic-picture-first-frame-only = En «{ $path }», la imagen animada «{ $detail }» llega como su primer fotograma. Las imágenes de un proyecto no se mueven.
import-diagnostic-duplicate-title = «{ $title }» aparece { $count } veces. Si ya importó estos archivos antes, se duplicarán.
import-diagnostic-heading-level-jump = «{ $title }» salta del nivel de encabezado { $from } al { $to }; se coloca un nivel por debajo de su padre.
import-diagnostic-illegal-combination = «{ $title }» contiene prosa, pero un elemento de tipo { $kind } no puede contener ninguna. La importación queda en espera hasta que cambie su tipo o lo deje fuera. Si no, no se importa nada en absoluto.
import-diagnostic-tracked-changes-flattened = { $path } estaba en plena revisión: { $count ->
    [one] se ha aceptado { $count } cambio con seguimiento
   *[other] se han aceptado { $count } cambios con seguimiento
}, y se han aplicado las eliminaciones. Ese es el texto final. Para conservar alguna de sus propias formulaciones, use Comparar en el último paso.
import-diagnostic-tracked-changes-flattened-by = { $path } estaba en plena revisión: { $count ->
    [one] se ha aceptado { $count } cambio con seguimiento de { $names }
   *[other] se han aceptado { $count } cambios con seguimiento de { $names }
}, y se han aplicado las eliminaciones. Ese es el texto final. Para conservar alguna de sus propias formulaciones, use Comparar en el último paso.
import-diagnostic-text-box-dropped = { $count ->
    [one] { $path } contiene { $count } cuadro de texto. Su texto está fuera del flujo del documento, así que no se puede saber a qué lugar del manuscrito pertenece. No se importa.
   *[other] { $path } contiene { $count } cuadros de texto. Su texto está fuera del flujo del documento, así que no se puede saber a qué lugar del manuscrito pertenece. No se importan.
}
import-diagnostic-embedded-object-dropped = { $count ->
    [one] { $path } contiene { $count } objeto incrustado: un gráfico, una ecuación o similar. No hay nada en un manuscrito que pudiera contenerlo.
   *[other] { $path } contiene { $count } objetos incrustados: un gráfico, una ecuación o similar. No hay nada en un manuscrito que pueda contenerlos.
}
import-diagnostic-field-flattened = { $count ->
    [one] { $path } contiene { $count } campo: un número de página, una referencia cruzada, una fecha. Conserva el último texto que mostraba y ya no se actualizará.
   *[other] { $path } contiene { $count } campos: un número de página, una referencia cruzada, una fecha. Cada uno conserva el último texto que mostraba y ya no se actualizará.
}
import-diagnostic-unknown-style-level = { $path } usa el estilo «{ $detail }», que parece un encabezado pero no indica ningún nivel. Esos párrafos se importan como prosa en lugar de adivinar su profundidad.
import-diagnostic-comment-unanchored = El comentario «{ $detail }» de { $path } no se pudo vincular a las palabras a las que se refería. Se conserva en su elemento, donde puede moverlo.
import-diagnostic-comment-not-carried = El comentario «{ $detail }» de { $path } no se importa, porque nada del archivo llega como texto en el que se pueda conservar. Cópielo del archivo si lo necesita.
import-diagnostic-comment-replies-flattened = { $count ->
    [one] { $count } respuesta de { $path } hacía referencia a un comentario que no está en el archivo, así que llega como un comentario independiente.
   *[other] { $count } respuestas de { $path } hacían referencia a un comentario que no está en el archivo, así que llegan como comentarios independientes.
}
import-diagnostic-prose-not-verbatim = { $count ->
    [one] { $count } párrafo de { $path } no se pudo traer con su formato exactamente como estaba escrito, así que llega como texto sin formato. Compárelo con el original.
   *[other] { $count } párrafos de { $path } no se pudieron traer con su formato exactamente como estaban escritos, así que llegan como texto sin formato. Compárelos con el original.
}
import-diagnostic-styled-spaces-not-carried = { $count ->
    [one] En { $path }, { $count } tramo de espacio en blanco estaba subrayado o tachado, por ejemplo una línea para rellenar. Skribisto no conserva esas líneas sobre espacios solos, así que llega como espacios normales, o desaparece donde empieza o termina un párrafo.
   *[other] En { $path }, { $count } tramos de espacio en blanco estaban subrayados o tachados, por ejemplo líneas para rellenar. Skribisto no conserva esas líneas sobre espacios solos, así que llegan como espacios normales, o desaparecen donde empiezan o terminan un párrafo.
}
import-diagnostic-list-nesting-flattened = { $count ->
    [one] { $count } elemento de lista de { $path } estaba anidado a más de { $limit } niveles de profundidad. Llega al nivel { $limit }, el más profundo que conserva Skribisto, con sus palabras y su formato.
   *[other] { $count } elementos de lista de { $path } estaban anidados a más de { $limit } niveles de profundidad. Llegan al nivel { $limit }, el más profundo que conserva Skribisto, con sus palabras y su formato.
}
import-diagnostic-table-read-as-paragraphs = { $count ->
    [one] { $count } tabla de { $path } tiene una fila mucho más ancha que las demás, demasiado desigual para disponerla en cuadrícula. Sus celdas llegan como párrafos, uno tras otro, con sus palabras y su formato.
   *[other] { $count } tablas de { $path } tienen una fila mucho más ancha que las demás, demasiado desigual para disponerla en cuadrícula. Sus celdas llegan como párrafos, uno tras otro, con sus palabras y su formato.
}
import-diagnostic-spaces-shortened = { $count ->
    [one] { $count } serie de espacios de { $path } tenía más de { $limit } espacios y llega acortada a { $limit }. Las palabras que la rodean están todas.
   *[other] { $count } series de espacios de { $path } tenían más de { $limit } espacios y llegan acortadas a { $limit }. Las palabras que las rodean están todas.
}
import-diagnostic-epigraph-not-carried = «{ $title }» va encabezado por un epígrafe, pero un elemento de tipo { $kind } no puede contenerlo. La cita se conserva en su lugar al principio de su texto.
import-diagnostic-epigraph-placement-ambiguous = Hay un epígrafe entre «{ $title }» y «{ $below }» que podría encabezar a cualquiera de los dos. Se ha asignado a «{ $title }», que es donde suele ir un epígrafe.
import-document-diagnostics = { $errors ->
    [0] { $warnings ->
            [one] 1 cosa que debe saber
           *[other] { $warnings } cosas que debe saber
        }
   *[other] { $errors ->
            [one] No se pudo leer 1 archivo
           *[other] No se pudieron leer { $errors } archivos
        }
}
import-document-diagnostics-none = Nada que señalar.

versions-title = Versiones
versions-scope-synopsis = Sinopsis
versions-scope-prose = Texto
versions-loading = Revisando sus copias de seguridad…
versions-empty = Todavía no hay ninguna versión anterior de esto
versions-error = No se pudieron leer sus copias de seguridad. No se ha perdido nada, pero esta lista puede estar incompleta
versions-did-not-exist = Aún no existía el { $date }
versions-deleted-after = Eliminado en algún momento después del { $date }
versions-unreadable = { $count ->
    [one] No se pudo leer { $count } copia de seguridad
   *[other] No se pudieron leer { $count } copias de seguridad
}
versions-thinned = { $count ->
    [one] Las versiones antiguas se van espaciando con el tiempo. El historial propio del proyecto ya ha descartado { $count } estado anterior de este texto.
   *[other] Las versiones antiguas se van espaciando con el tiempo. El historial propio del proyecto ya ha descartado { $count } estados anteriores de este texto.
}
versions-source-backup = De una copia de seguridad
versions-source-project = Del historial propio del proyecto
versions-list-caption = Una entrada por cambio, no por copia de seguridad
versions-pick-a-version = Elija una versión para ver qué ha cambiado
versions-earliest = La versión más antigua registrada. No hay nada anterior con lo que compararla.
versions-no-change = No ha cambiado nada en esta parte del elemento
versions-formatting-only = Aquí solo ha cambiado el formato. Las palabras son las mismas
versions-show-unchanged = Mostrar sin cambios
versions-hide-unchanged = Ocultar sin cambios
versions-next-change = Siguiente cambio
versions-near = cerca de «{ $text }»
versions-words-added = { $count ->
    [one] { $count } palabra añadida
   *[other] { $count } palabras añadidas
}
versions-words-removed = { $count ->
    [one] { $count } palabra eliminada
   *[other] { $count } palabras eliminadas
}
versions-blocks-moved = { $count ->
    [one] { $count } párrafo movido
   *[other] { $count } párrafos movidos
}
versions-pin = Fijar la copia de seguridad de la que procede esta versión. La limpieza automática nunca la eliminará
versions-unpin = Soltar la copia de seguridad de la que procede esta versión. La limpieza automática podrá volver a eliminarla
versions-pinned-only = Mostrar solo las versiones fijadas
versions-pin-note = Solo se pueden fijar las versiones procedentes de una copia de seguridad. Fijar conserva un archivo, y el historial propio del proyecto no lo es.
versions-pinned-empty = Todavía no hay nada fijado aquí
versions-pinned-empty-log = Todavía no hay nada fijado aquí. Solo se pueden fijar las versiones procedentes de una copia de seguridad. Fijar conserva un archivo, y el historial propio del proyecto no lo es.
versions-range-filter = Mostrar solo las versiones entre dos fechas
versions-filtered-empty = Ninguna versión coincide con los filtros que ha definido
versions-clear-filters = Borrar los filtros
versions-last-30-days = Últimos 30 días
versions-restore-button = Restaurar esta versión
versions-restore-confirm-title = ¿Sustituir este texto por la versión del { $date }?
versions-restore-confirm-text = Lo que tiene ahora se sustituirá por el texto que este elemento tenía el { $date }.
versions-restore-confirm-with-comments = Lo que tiene ahora se sustituirá por el texto que este elemento tenía el { $date }. { $count ->
    [one] { $count } comentario está anclado en el texto actual y puede quedar huérfano.
   *[other] { $count } comentarios están anclados en el texto actual y pueden quedar huérfanos.
}
versions-restore-confirm-undo-note = Antes se hace una copia de seguridad, y Ctrl+Z lo deshace en un solo paso.
versions-restored-toast = Se ha restaurado la versión del { $date }
versions-undo = Deshacer
versions-restore-row-gone = Ese elemento ya no está en este proyecto
versions-restore-no-home = Este elemento ha cambiado de tipo desde entonces y el texto antiguo no tiene dónde ir en él
versions-restore-no-safety-copy = No se ha hecho la copia de seguridad de precaución, así que no se ha cambiado nada
versions-restore-failed = La restauración ha fallado: { $error }
versions-restore-backup-busy = Ya hay una copia de seguridad en curso. Vuelva a intentarlo dentro de un momento
versions-restore-in-backup-file = Está viendo un archivo de copia de seguridad; abra el propio proyecto para restaurar en él
versions-restore-no-project = No hay ningún proyecto abierto
versions-recreate-button = Recuperar esto…
versions-recreate-picker-title = ¿Dónde debe ir?
versions-recreate-picker-empty = Este proyecto no tiene ningún archivador en el que ponerlo
versions-recreate-picker-confirm = Recuperarlo aquí
versions-recreate-picker-cancel = Cancelar
versions-recreate-untitled = este elemento
versions-recreate-confirm-title = ¿Recuperar «{ $item }»?
versions-recreate-confirm-text = Se añadirá a { $destination }, con el texto que tenía el { $date }.
versions-recreate-confirm-undo-note = Deshacer, en el mensaje que aparece a continuación, lo vuelve a quitar directamente.
versions-recreated-toast = «{ $item }» vuelve a estar en su proyecto
versions-recreated-partial-toast = { $count ->
    [one] «{ $item }» ha vuelto, pero no se pudo leer uno de sus textos
   *[other] «{ $item }» ha vuelto, pero no se pudieron leer { $count } de sus textos
}
versions-recreate-already-here = Ese elemento ya está en su proyecto
versions-recreate-no-destination = Elija un lugar del archivador donde ponerlo
versions-recreate-unreadable = No se pudo leer esa copia de seguridad, así que no se ha añadido nada
versions-recreate-failed = No se pudo recuperar: { $error }
versions-changed-percent = Ha cambiado el { $percent } % de esto
versions-hidden-paragraphs = { $count ->
    [one] … { $count } párrafo sin cambios …
   *[other] … { $count } párrafos sin cambios …
}

timeline-title = Volver al pasado
timeline-coverage = { $count ->
    [one] { $count } versión registrada, desde el { $oldest }
   *[other] { $count } versiones registradas, desde el { $oldest }
}
timeline-loading = Revisando el pasado de su proyecto…
timeline-empty = Todavía no se ha registrado ninguna versión de este proyecto
timeline-no-changes = Nada ha cambiado desde entonces
timeline-no-text-changes = Ningún texto ha cambiado desde entonces
timeline-slider-label = Versión registrada
timeline-series-name = Tamaño del proyecto
timeline-bars-caption = Cada barra es una versión registrada, tan alta como era entonces el proyecto. La resaltada es la que está viendo.
timeline-bars-caption-periods = Hay demasiadas versiones para mostrarlas una a una, así que cada barra representa { $period }, tan alta como era el proyecto al final de ese periodo.
timeline-unit-hour = una hora
timeline-unit-day = un día
timeline-unit-week = una semana
timeline-unit-month = un mes
timeline-range-filter = Mostrar solo las versiones entre dos fechas
timeline-range-empty = No se registró ninguna versión en esas fechas
timeline-open-period = Abrir este periodo
timeline-show-all = Mostrar todo el historial
timeline-changed-since = { $count ->
    [one] { $count } elemento difiere entre el { $date } y su proyecto actual
   *[other] { $count } elementos difieren entre el { $date } y su proyecto actual
}
timeline-kind-added = Escrito desde entonces
timeline-kind-removed = Ya no está en el proyecto
timeline-kind-changed = Editado desde entonces
timeline-kind-moved = Movido desde entonces
timeline-not-yet-written = Esto todavía no existía en ese momento
timeline-no-text-of-its-own = No tiene texto propio. Es un encabezado de lo que contiene
timeline-reader-close = Cerrar
timeline-reader-stamp = Como era el { $date }
timeline-reader-loading = Abriendo la versión registrada…
timeline-reader-unreadable = No se pudo leer este registro. Puede que la copia de seguridad se haya movido o eliminado, o que esté en una unidad que no está conectada.
timeline-reader-compared = Como era el { $date }, comparado con lo que dice ahora
timeline-reader-diff-legend = Lo tachado ha desaparecido desde entonces; lo subrayado se ha añadido.
timeline-reader-view-label = Qué mostrar
timeline-reader-view-diff = Cambios
timeline-reader-view-text = Texto
timeline-reader-deleted = Esto ya no está en su proyecto. Puede leerlo y copiarlo desde aquí.
timeline-prose-only-record = Este punto procede del historial propio del proyecto, que guarda el texto y nada más, así que no se puede mostrar lo que se ha eliminado o movido desde entonces. Para eso, elija una copia de seguridad.

settings-sec-extensions = Extensiones

settings-group-games-forward = Siempre hacia adelante
settings-games-forward-toggle = Jugar a «Siempre hacia adelante»
settings-games-forward-hint =
    Mientras juega, nada de lo que haya escrito se puede retirar: Retroceso, Supr,
    Cortar, arrastrar y soltar y Deshacer quedan desactivados en las superficies que elija abajo.
    Puede seguir escribiendo, pegando, dando formato y desplazándose libremente. El borrador solo crece.
settings-games-session-warning =
    Es una elección para la sesión: nunca se guarda. Cerrar el proyecto o salir de
    Skribisto siempre termina el juego, y todo lo que escribió mientras jugaba vuelve
    a poder deshacerse en ese mismo momento.
settings-group-games-scope = Dónde se aplica
settings-games-in-prose = Prosa del manuscrito
settings-games-in-synopsis = Sinopsis
settings-games-scope-hint =
    Los comentarios, las notas al pie y los títulos nunca se congelan: es donde anota la corrección
    que acaba de prohibirse hacer.
settings-games-inert-warning =
    «Siempre hacia adelante» está activado pero no se aplica a nada. Marque al menos una superficie arriba,
    o no cambiará nada mientras escribe.

games-title = Juegos de escritura
games-forward-name = Siempre hacia adelante
games-forward-blurb =
    Redacte sin retirar nada. Eliminar, cortar y deshacer quedan desactivados
    mientras juega. Escriba la frase siguiente en lugar de corregir la anterior.
games-forward-playing = Jugando: eliminar está desactivado
games-forward-idle = Sin jugar
games-session-note = Termina cuando cierra el proyecto.
games-scope-prose-and-synopsis = Se aplica a su prosa y sinopsis.
games-scope-prose = Se aplica a su prosa.
games-scope-synopsis = Se aplica a sus sinopsis.
games-scope-nothing = Todavía no se aplica a nada. Elija una superficie en Configuración.
games-settings-link = Configuración de los juegos de escritura…
statusbar-games-forward = Siempre hacia adelante
statusbar-games-forward-tooltip =
    «Siempre hacia adelante» está activado: eliminar, cortar y deshacer están desactivados mientras
    redacta. Haga clic para dejar de jugar.
goal-progress-words = { $g ->
    [one] { $count } de { $goal } palabra
   *[other] { $count } de { $goal } palabras
}
goal-progress-characters = { $g ->
    [one] { $count } de { $goal } carácter
   *[other] { $count } de { $goal } caracteres
}
goal-count-words = { $n ->
    [one] { $count } palabra
   *[other] { $count } palabras
}
goal-count-characters = { $n ->
    [one] { $count } carácter
   *[other] { $count } caracteres
}
goal-unit-words = Palabras
goal-unit-characters = Caracteres

inspector-goal = Objetivo
inspector-goal-none = Sin objetivo

overview-col-goal = Objetivo
overview-excluded-from-export = Excluido de la exportación, así que no cuenta para ningún total

goal-subtree-total-words = { $count ->
    [one] { $items } objetivo interior suma { $words } palabras
   *[other] { $items } objetivos interiores suman { $words } palabras
}
goal-subtree-total-characters = { $count ->
    [one] { $items } objetivo interior suma { $words } caracteres
   *[other] { $items } objetivos interiores suman { $words } caracteres
}

new-work-goal-unit = Contar en
new-work-goal-unit-hint = Se usa para todos los objetivos de palabras o de caracteres de este proyecto. Se puede cambiar más adelante.

settings-group-goal-unit = Objetivos
settings-goal-unit-switch-title = Cambiar la unidad de recuento
settings-goal-unit-switch-text = Los objetivos y los hitos ya definidos en este proyecto se introdujeron en { $from }. Cambiar a { $to } no los convierte, así que cada número existente se leerá ahora como { $to }.
settings-goal-unit-switch-informative = No se pierde nada. Vuelva a cambiar en cualquier momento para recuperar la lectura original y luego actualice los objetivos que quiera conservar.
settings-goal-unit-switch-confirm = Cambiar

goal-distribute-action = Repartir…
distribute-title = Repartir el objetivo
distribute-empty = No hay nada dentro entre lo que repartir el objetivo.
distribute-weight = Repartir
distribute-weight-length = Por longitud
distribute-weight-rows = Por elementos
distribute-weight-even = Por igual
distribute-overwrite = Sustituir los objetivos ya definidos
distribute-col-item = Elemento
distribute-col-current = Ahora
distribute-col-proposed = Después
distribute-total = Suman { $total }, frente a un objetivo de { $goal }.
distribute-over-budget = Los objetivos ya definidos dentro superan en { $over } el del propio contenedor. Suba su objetivo, baje los de dentro o sustitúyalos.
distribute-apply = Repartir
distribute-cancel = Cancelar

milestone-target-gone = Objetivo eliminado
milestone-no-target = Sin objetivo
milestone-add = Añadir
milestone-add-label = Qué debe alcanzarse

pace-summary-title = Dónde está el libro
pace-summary-remaining = Faltan { $words }
pace-summary-open = Abrir el plan
pace-summary-close = Cerrar
pace-summary-dont-show = No mostrar esto al abrir
pace-summary-menu = Plan de escritura…

menu-help-topics = &Temas de ayuda
menu-help-shortcuts = Atajos de te&clado…
menu-help-website = &Skribisto en la web
menu-help-report = &Informar de un problema…
menu-help-check-updates = &Buscar actualizaciones
menu-command-palette = &Paleta de comandos…

learn-help-topics = Temas de ayuda
learn-shortcuts = Atajos de teclado
learn-website = Skribisto en la web

help-window-title = Ayuda
help-filter-topics = Filtrar temas
help-no-matching-topic = Ningún tema coincide.
help-topic-missing = Este tema ya no está disponible.
help-back = Atrás
help-not-translated = Esta página aún no se ha traducido, por lo que se muestra en inglés.

help-section-getting-started = Primeros pasos
help-section-writing = Escritura
help-section-reviewing = Revisión
help-section-exchanging = Meter y sacar palabras
help-section-keeping = Proteger su trabajo
help-section-extensions = Extensiones

help-topic-getting-started = Su primer proyecto
help-topic-writing-model = Cómo se construye un libro
help-topic-drafts-and-old-versions = Conservar un borrador anterior
help-topic-goals-and-pace = Objetivos y ritmo
help-topic-comments = Comentarios
help-topic-round-trip = Enviar su libro a un lector
help-topic-export = Exportar
help-topic-import-documents = Importar documentos
help-topic-import-projects = Traer un proyecto completo
help-topic-import-scrivener = Llegar desde Scrivener
help-topic-backups-and-versions = Copias de seguridad y versiones

help-shortcuts-title = Atajos de teclado
help-shortcuts-filter = Filtrar atajos
help-shortcuts-no-matches = Ningún atajo coincide.
help-shortcuts-rebind = Cambiar atajos…
help-shortcuts-close = Cerrar

command-palette-placeholder = Escriba un comando

shortcut-name-binder-duplicate = Duplicar
shortcut-name-comments-add = Añadir comentario
shortcut-name-comments-add-paragraph = Comentar el párrafo
shortcut-name-spellcheck-toggle = Revisar la ortografía
shortcut-name-editor-tab-close = Cerrar pestaña
shortcut-name-editor-tab-pin = Fijar o soltar pestaña
shortcut-name-editor-save = Guardar
shortcut-name-work-export = Exportar…
shortcut-name-work-new = Nuevo proyecto
shortcut-name-work-open = Abrir proyecto
shortcut-name-window-new = Nueva ventana
shortcut-name-work-close = Cerrar proyecto
shortcut-name-work-import-scrivener = Importar un proyecto de Scrivener…
shortcut-name-work-import-scrivener-report = Mostrar el informe de importación de Scrivener
shortcut-name-app-settings = Configuración
shortcut-name-settings-close = Cerrar configuración
shortcut-name-app-quit = Salir
shortcut-name-editor-insert-footnote = Insertar nota al pie
shortcut-name-format-scene-break = Insertar separador de escena
shortcut-name-format-major-scene-break = Insertar separador de escena mayor
shortcut-name-format-link = Insertar enlace
shortcut-name-format-mark-nbsp = Insertar espacio de no separación
shortcut-name-format-mark-nbhyphen = Insertar guion de no separación
shortcut-name-format-mark-nnbsp = Insertar espacio estrecho de no separación
shortcut-name-format-mark-zwsp = Insertar espacio de anchura cero
shortcut-name-go-next = Siguiente
shortcut-name-go-prev = Anterior
shortcut-name-go-to = Ir a
shortcut-name-outline-toggle = Mostrar u ocultar el esquema
shortcut-name-preview-toggle = Mostrar u ocultar la banda de vista previa
shortcut-name-view-fullscreen = Activar o desactivar pantalla completa
shortcut-name-view-focus-mode = Activar o desactivar el modo sin distracciones
shortcut-name-editor-size-increase = Aumentar el tamaño del texto
shortcut-name-editor-size-decrease = Reducir el tamaño del texto
shortcut-name-editor-size-reset = Restablecer el tamaño del texto
shortcut-name-editor-find = Buscar
shortcut-name-editor-replace = Reemplazar
shortcut-name-editor-find-next = Coincidencia siguiente
shortcut-name-editor-find-prev = Coincidencia anterior
shortcut-name-search-show = Buscar en el proyecto
shortcut-name-search-replace = Reemplazar en el proyecto
shortcut-name-outline-open-to-side = Abrir al lado
shortcut-name-help-topics = Temas de ayuda
shortcut-name-help-close = Cerrar ayuda
shortcut-name-help-shortcuts = Atajos de teclado
shortcut-name-help-website = Skribisto en la web
shortcut-name-help-report = Informar de un problema
shortcut-name-help-check-updates = Buscar actualizaciones
update-check-behind = La versión { $version } está disponible.
update-check-current = Está usando la versión más reciente.
update-check-failed = No se pudo comprobar si hay una versión más reciente. { $error }
shortcut-name-command-palette = Paleta de comandos
help-section-reference = Qué es cada cosa

help-concept-scene-break = Separador de escena
help-concept-major-scene-break = Separador de escena mayor
help-concept-find-in-prose = Buscar en la prosa
help-concept-story-bible = Biblia narrativa
help-concept-goal-unit = Palabras o caracteres
help-concept-goal-progress = Progreso
help-concept-manuscript-words = Lo que cuenta como el manuscrito
help-concept-exportable = Excluido de la exportación
help-concept-distribute = Repartir un objetivo
help-concept-subtree-total = Lo que suman los objetivos interiores
help-concept-milestone = Hito
help-concept-pace-plan = Plan de ritmo

help-concept-tag = Etiqueta
help-concept-label = Rótulo
help-concept-point-of-view = Punto de vista
help-concept-epigraph = Epígrafe
help-concept-footnote = Nota al pie
help-concept-chapter-mode = Forma del capítulo
help-concept-comment = Comentario
help-concept-backup = Copia de seguridad
help-concept-version = Versión
help-concept-trash = Papelera
help-concept-spellcheck = Revisión ortográfica
help-concept-search-replace = Buscar y reemplazar
help-concept-note-template = Plantilla de notas
help-concept-story-bible-entry = Entrada de la biblia narrativa
help-concept-text-replacement = Sustitución de texto
help-concept-smart-punctuation = Puntuación inteligente
help-concept-export-style = Estilo de exportación
help-concept-round-trip-marks = Marcadores de ida y vuelta

epigraph-mark-attribution = Línea de fuente
epigraph-mark-attribution-tip = Marca la línea en la que está el cursor como la fuente de la cita, para que se imprima como atribución
margin-lane-name = Marcas del margen
margin-lane-provider-comments = Comentarios
margin-lane-provider-story-bible = Entrada de la biblia narrativa
margin-lane-provider-story-bible-hint = En la lectura En la prosa de una nota, dónde se nombra esa entrada y las escenas narradas a través de sus ojos. También marca esos nombres en la propia prosa.
margin-lane-mark-point-of-view = Narrado desde aquí
margin-lane-mark-named-here = Nombrado aquí
margin-lane-provider-comments-hint = Dónde hay una nota vinculada al texto
margin-lane-provider-search = Resultados de búsqueda
margin-lane-provider-search-hint = Cada resultado de su última búsqueda, esté donde esté en el documento
margin-lane-provider-boundaries = Dónde empieza cada documento
margin-lane-provider-boundaries-hint = Una línea al principio de cada escena de un flujo, para que vea dónde está
margin-lane-provider-spelling = Ortografía
margin-lane-provider-spelling-hint = Dónde el corrector ortográfico ha señalado una palabra. Desactivado salvo que lo pida: marca la opinión de una máquina sobre su prosa
margin-lane-spelling = { $word }, posible error ortográfico
margin-lane-search-hit = { $text }, coincidencia { $index } de { $total }
margin-lane-search-current = { $text }, coincidencia { $index } de { $total }, en la que está
margin-lane-boundary = Inicio de { $title }
margin-lane-boundary-untitled = un documento sin título
settings-page-margin-lane = Marcas del margen
settings-desc-margin-lane = La banda junto a la barra de desplazamiento, y lo que muestra
settings-margin-lane-enabled = Mostrar la banda del margen
settings-margin-lane-enabled-hint = El mismo interruptor que Ver ▸ Marcas del margen.
settings-margin-lane-enabled-more = Nunca dice que algo esté mal: le muestra dónde están las cosas y deja en sus manos qué pensar de ello.
settings-group-margin-lane-marks = Qué marca
settings-margin-lane-no-providers = Todavía no hay nada que marque la banda.
settings-group-margin-lane-texture = Textura del diálogo
settings-margin-lane-texture = Mostrar la textura del diálogo
settings-margin-lane-texture-hint = Una barra por párrafo: su longitud, y qué parte es diálogo.
settings-margin-lane-texture-more = No se mide en los idiomas sin una convención establecida; ahí la barra solo muestra la longitud.
settings-group-margin-lane-surfaces = Dónde aparece
settings-margin-lane-surface-editor = Editor de texto
settings-margin-lane-surface-stream = Flujos
settings-margin-lane-surface-search-preview = Vista previa de búsqueda
menu-margin-lane = &Marcas del margen

status-preset-drafting = Redacción
status-preset-passes = Pasadas de revisión
status-preset-plume = Plume Creator
status-todo = Por hacer
status-draft = Borrador
status-revised = Revisado
status-final = Final
status-outline = Esquema
status-first-edit = 1.ª corrección
status-second-edit = 2.ª corrección
status-done = Listo
status-plume-draft-1 = 1.er borrador
status-plume-draft-2 = 2.º borrador
status-plume-draft-3 = 3.er borrador
status-plume-edit-1 = 1.ª corrección
status-plume-edit-2 = 2.ª corrección
status-plume-edit-3 = 3.ª corrección
status-plume-proofread = Revisión de pruebas
status-plume-finished = Terminado
new-work-statuses = Flujo de trabajo
new-work-statuses-hint = Las etapas por las que pasa una escena. Puede renombrarlas, reordenarlas o añadir otras en cualquier momento.
status-none = Sin estado
inspector-status = Estado
overview-col-status = Estado
overview-status-mixed = las partes de abajo no coinciden
help-concept-status = Estado
status-completion-title = Dónde está el libro
status-completion-headline = { $done } de { $total } escenas terminadas
status-completion-empty = Todavía no hay escenas. Esto lee el manuscrito, así que se va llenando a medida que escribe.
status-completion-open = Dónde está el libro…

settings-page-statuses = Estados
settings-desc-statuses = Renombre, reordene, añada y quite las etapas por las que pasa una escena
settings-statuses-add = Añadir estado
settings-statuses-add-placeholder = Nombre del nuevo estado
settings-statuses-desc =
    Las etapas por las que pasa una escena, en orden, de la menos acabada arriba a la más
    acabada abajo. Ese orden es lo que significa «menos acabado que» en todo el resto
    de la aplicación, así que colóquelas como funciona realmente su proceso.
settings-statuses-apply-preset = Aplicar un conjunto predefinido…
settings-statuses-preset-applied = { $added ->
    [one] 1 estado añadido
   *[other] { $added } estados añadidos
}
settings-statuses-preset-refused = Este proyecto ya tiene una escala. Los conjuntos predefinidos solo rellenan una vacía. Elimine primero los peldaños que no quiera.
settings-statuses-duplicate = «{ $name }» ya está en esta escala
settings-statuses-added = «{ $name }» añadido
settings-statuses-move-up = Subir (menos acabado)
settings-statuses-move-down = Bajar (más acabado)
settings-statuses-details-placeholder = Qué significa esta etapa (opcional)
settings-statuses-delete = Eliminar «{ $name }»
settings-statuses-delete-in-use = { $count ->
    [one] Eliminar «{ $name }»: 1 elemento lo tiene y perderá su estado
   *[other] Eliminar «{ $name }»: { $count } elementos lo tienen y perderán su estado
}
settings-statuses-deleted = «{ $name }» eliminado
settings-statuses-deleted-in-use = { $count ->
    [one] «{ $name }» eliminado. 1 elemento ya no tiene estado.
   *[other] «{ $name }» eliminado. { $count } elementos ya no tienen estado.
}
settings-statuses-empty-title = Todavía no hay flujo de trabajo
settings-statuses-empty-body = Un estado indica lo avanzada que está una escena. Añada uno arriba, o empiece con un conjunto predefinido.
status-category-planned = Planificado
status-category-drafting = Redacción
status-category-needs-work = Necesita trabajo
status-category-revised = Revisado
status-category-final = Final

## Long-operation progress (shared::progress_text). The backend reports phase
## codes it cannot translate; these are their labels. `progress-percent` is the
## toast body on its own, `progress-percent-with-label` with the phase after it.
progress-percent = { $percent } %
progress-percent-with-label = { $percent } % · { $label }
progress-import-opening = Abriendo el proyecto…
progress-import-outline = Leyendo el esquema…
progress-import-converting = Convirtiendo capítulos y escenas…
progress-import-writing = Escribiendo el archivo .skrib…
progress-import-done = Hecho
progress-import-structure = Determinando la estructura…
progress-export-start = Iniciando…
progress-export-writing = Escribiendo…
progress-project-reading = Leyendo el proyecto…
# Settings: the open project's section title, "Work: <its title>".
settings-sec-work-titled = Proyecto: { $title }

## Project-import notices (shared::import_warnings::notices): one line of the
## Plume / Manuskript warnings list each. $file, $folder, $title, $name, $field
## and $error are data (file names, titles, a parser's own message) and stay as
## they are. $container is "single-file" or "folder"; $kind is "scene", "note",
## "synopsis" or "title".
import-notice-flattened-rich-texts = { $count ->
    [one] Un texto estaba anidado a más profundidad de la que admite un proyecto de Skribisto; sus palabras se han conservado como texto sin formato.
   *[other] { $count } textos estaban anidados a más profundidad de la que admite un proyecto de Skribisto; sus palabras se han conservado como texto sin formato.
}
import-notice-free-text-badges = { $count ->
    [one] Una insignia de escena parece texto libre más que una lista de estados, así que se ha conservado como rótulo del elemento en lugar de convertirse en etiqueta.
   *[other] { $count } insignias de escena distintas parecen texto libre más que una lista de estados, así que se han conservado como rótulos de los elementos en lugar de convertirse en etiquetas.
}
import-notice-separator-text-dropped = El separador «{ $name }» contenía texto, que no tiene cabida en un proyecto de Skribisto; ese texto no se ha importado.
import-notice-separator-without-scene = El separador «{ $name }» no tenía ninguna escena antes de él en su capítulo a la que unirse, así que no se ha importado.
import-notice-chapter-without-scenes = El capítulo «{ $name }» no tiene escenas, así que sus separadores no se han importado.
import-notice-rich-text-unconverted = No se pudo convertir parte del texto con formato ({ $error }); no se ha importado.
import-notice-unresolved-cross-link = No se pudo seguir un enlace a la ficha de personaje n.º { $number }: la ficha no existe o estaba en la papelera.
import-notice-read-copy = Se ha leído la copia { $container ->
    [single-file] en archivo único
   *[folder] en carpeta
} de este proyecto.
import-notice-read-copy-dated = Se ha leído la copia { $container ->
    [single-file] en archivo único
   *[folder] en carpeta
} de este proyecto, modificada por última vez el { $day }.
import-notice-original-format = Este proyecto está en el formato original de Manuskript, de 2016. Se ha leído entero; no se ha modificado nada en él.
import-notice-newer-single-file-beside = «{ $file }» es más reciente que esta carpeta ({ $file_day } frente a { $folder_day }). Manuskript deja la carpeta antigua en su sitio cuando un proyecto pasa al modo de archivo único, así que esta carpeta puede ser una copia abandonada; si importa el archivo .msk en su lugar, se leerá la versión más reciente.
import-notice-pickle-skipped = «{ $file }» no se ha leído. Es un archivo pickle de Python, y cargar uno desde un archivo de proyecto era un fallo de seguridad del propio Manuskript (CVE-2021-35196); los ajustes que contiene son preferencias de visualización, no texto.
import-notice-file-unreadable = No se pudo leer «{ $file }» ({ $error }) y se ha omitido; todo lo demás se ha importado.
import-notice-not-utf8 = «{ $file }» no es UTF-8 válido; los caracteres que no se pudieron leer se han sustituido en lugar de detener la importación.
import-notice-table-unreadable = No se pudo leer «{ $file }» ({ $error }); su contenido no se ha importado.
import-notice-outline-missing = Falta «{ $file }», así que este proyecto no tiene manuscrito que importar.
import-notice-outline-unreadable = No se pudo leer «{ $file }» ({ $error }); no se ha importado ningún manuscrito.
import-notice-named-none-row = «{ $file }» empieza con una fila con nombre («{ $name }») donde Manuskript guarda su entrada vacía «none». Se ha omitido para que las filas siguientes conserven los números que usa el manuscrito.
import-notice-unknown-field = «{ $file }» tiene un campo que este importador no conoce: «{ $field }». No se ha importado.
import-notice-colorless-labels = { $count ->
    [one] «{ $file }» tiene una línea sin color. Se ha importado sin color; Manuskript deja de cargar el proyecto ante una línea así.
   *[other] «{ $file }» tiene { $count } líneas sin color. Se han importado sin color; Manuskript deja de cargar el proyecto ante una línea así.
}
import-notice-settings-unreadable = No se pudo leer «{ $file }» ({ $error }); el idioma del proyecto no se ha importado.
import-notice-revisions-unreadable = No se pudo leer «{ $file }» ({ $error }); el historial del proyecto no se ha importado, pero no ha afectado a nada más.
import-notice-foreign-files = { $count ->
    [one] Este proyecto contiene un archivo que no forma parte del formato de Manuskript y no se ha importado: { $files }.
   *[other] Este proyecto contiene { $count } archivos que no forman parte del formato de Manuskript y no se han importado: { $files }.
}
import-notice-foreign-files-more = Este proyecto contiene { $count } archivos que no forman parte del formato de Manuskript y no se han importado: { $files } y { $more ->
    [one] uno más
   *[other] { $more } más
}.
import-notice-not-a-character-file = «{ $file }» no es un archivo de personaje de Manuskript y se ha omitido.
import-notice-not-a-text-item = «{ $file }» no es un elemento de texto de Manuskript y se ha dejado fuera del esquema.
import-notice-missing-blank-line = «{ $file }» no tiene una línea en blanco tras su encabezado. Su último campo se ha conservado, aunque Manuskript lo pierde.
import-notice-folder-without-metadata = «{ $folder }» no tiene { $file }, así que se ha importado como una carpeta simple con el nombre del directorio. Manuskript omite un directorio así y todo lo que contiene.
import-notice-rows-given-ids = { $count ->
    [one] Un elemento del esquema no tenía identificador propio y se le ha asignado uno. Manuskript descarta un elemento así, o deja de cargar el proyecto si es una carpeta.
   *[other] { $count } elementos del esquema no tenían identificador propio y se les ha asignado uno. Manuskript descarta un elemento así, o deja de cargar el proyecto si es una carpeta.
}
import-notice-plots-unreadable = No se pudo leer «{ $file }» ({ $error }); las tramas no se han importado.
import-notice-world-unreadable = No se pudo leer «{ $file }» ({ $error }); las entradas del mundo no se han importado.
import-notice-world-not-opml = «{ $file }» no es un documento OPML; las entradas del mundo no se han importado.
import-notice-text-flattened = El texto de «{ $title }» está anidado a más profundidad de la que admite un proyecto de Skribisto; sus palabras se han conservado como texto sin formato.
import-notice-text-unconverted = No se pudo convertir el texto de «{ $title }» ({ $error }); se ha conservado exactamente como estaba escrito.
import-notice-text-without-file-name = No se pudo dar un nombre de archivo { $kind ->
    [scene] al texto de la escena
    [note] al texto de la nota
    [synopsis] a la sinopsis
   *[title] al título
} de «{ $title }», así que ese contenido no se ha importado.
import-notice-unknown-label = «{ $title }» tiene el rótulo { $index }, que no está en la lista de rótulos de este proyecto. Se ha importado sin rótulo.
import-notice-unknown-status = «{ $title }» tiene el estado { $index }, que no está en la lista de estados de este proyecto. Se ha importado sin estado.
import-notice-unknown-point-of-view = «{ $title }» está narrado por un personaje que no figura entre los personajes de este proyecto. No se ha establecido su punto de vista.
import-notice-unresolved-references = No se pudieron resolver las referencias del texto de «{ $title }» ({ $error }); el texto se ha conservado tal como se convirtió, con las marcas de referencia incluidas.
import-notice-custom-icons = { $count ->
    [one] Un elemento tenía un icono personalizado. Skribisto da a cada elemento el icono de su tipo, así que el icono no se ha importado; nada más ha cambiado en ese elemento.
   *[other] { $count } elementos tenían un icono personalizado. Skribisto da a cada elemento el icono de su tipo, así que los iconos no se han importado; nada más ha cambiado en esos elementos.
}
import-notice-character-colors = { $count ->
    [one] Un personaje tenía un color junto a su nombre. Skribisto colorea las etiquetas, no los elementos, así que el color no se ha importado; el personaje sí.
   *[other] { $count } personajes tenían un color junto a su nombre. Skribisto colorea las etiquetas, no los elementos, así que los colores no se han importado; los personajes sí, todos ellos.
}
import-notice-versions-kept = { $found ->
    [one] Este proyecto había registrado una versión anterior
   *[other] Este proyecto había registrado { $found } versiones anteriores
}; { $kept ->
    [one] se ha conservado una
   *[other] se han conservado { $kept }
}, espaciadas igual que Skribisto espacia su propio historial. Están en el panel Versiones.
import-notice-versions-unplaced = { $count ->
    [one] Una versión anterior pertenece a un elemento que ya no está en el proyecto y no se ha podido ubicar.
   *[other] { $count } versiones anteriores pertenecen a un elemento que ya no está en el proyecto y no se han podido ubicar.
}

## Native file dialogs: the window title, and the label of the file-type filter.
dialog-open-work-title = Abrir un proyecto de Skribisto
dialog-save-as-file-title = Guardar como un único archivo .skrib
dialog-save-as-folder-title = Elija la carpeta en la que guardar el proyecto
dialog-manuskript-file-title = Elija un proyecto de Manuskript
dialog-manuskript-folder-title = Elija la carpeta de un proyecto de Manuskript
dialog-plume-file-title = Elija un proyecto de Plume Creator
dialog-filter-skrib = Proyecto de Skribisto
dialog-filter-manuskript = Proyecto de Manuskript
dialog-filter-plume = Proyecto de Plume Creator
dialog-filter-hunspell-aff = Archivo de afijos de Hunspell (.aff)
dialog-filter-hunspell-dic = Diccionario de Hunspell (.dic)
dialog-filter-export = Archivo exportado
# Help ▸ Keyboard shortcuts / Settings ▸ Keymap / command palette.
shortcut-name-view-margin-lane = Mostrar u ocultar las marcas del margen

## The built-in distraction-free themes' names (distraction_free::theme).
df-theme-paper = Papel
df-theme-sepia = Sepia
df-theme-night = Noche
df-theme-high-contrast = Alto contraste
df-theme-midnight = Medianoche

## Welcome ▸ Examples: the line under a bundled example's title.
welcome-example-byline = de { $author }

## Spoken by a screen reader only.
# Welcome window: the name of the section tab bar.
welcome-sections-a11y = Secciones de la ventana de bienvenida
# A comment's anchored span in the editor: author, plain-text preview, replies.
comments-a11y-summary-author = { $author }: { $text }
comments-a11y-summary-replies = { $text } ({ $count ->
    [one] 1 respuesta
   *[other] { $count } respuestas
})
comments-a11y-summary-author-replies = { $author }: { $text } ({ $count ->
    [one] 1 respuesta
   *[other] { $count } respuestas
})
# Margin marks beside the text: one mark, several merged into one, and the
# lane's own name when nothing more specific was given.
margin-lane-a11y-mark = { $label }, al { $percent } por ciento del texto
margin-lane-a11y-marks = { $label }, { $count } marcas, al { $percent } por ciento del texto
margin-lane-a11y-fallback = { $count ->
    [one] Marcas del margen, 1 marca
   *[other] Marcas del margen, { $count } marcas
}
# Export ▸ Choose: the row standing for a binder that has no name.
export-choose-unnamed-binder = Archivador sin nombre
# The built-in export styles named in words (the national manuscript presets
# keep their own language's name).
export-style-neutral = Neutro
export-style-manuscript-shunn = Manuscrito estándar (Shunn)
export-style-ebook-clean = Libro electrónico limpio

## Dates. Put the parts in the order your language writes a short date; the
## parts arrive already zero-padded where that matters.
# Comment cards, the outline card's Created/Modified.
date-time-short = { $day }/{ $month }/{ $year } { $hour }:{ $minute }
# Pace chart: a day on the axis, a milestone's date, a holiday.
pace-day-label = { $day }/{ $month }

## File sizes (shared::byte_size). $whole and $tenth are the two sides of the
## decimal mark: write your language's mark between them and its unit symbol.
byte-size-bytes = { $value } B
byte-size-kb = { $value } KB
byte-size-mb = { $whole },{ $tenth } MB
byte-size-gb = { $whole },{ $tenth } GB
# A whole percentage (sliders, the Pace card, the text-size toast).
unit-percent = { $value } %
# A whole number of pixels (slider readouts).
unit-pixels = { $value } px
# Settings ▸ Paratext: a preset saved without a `name` line. `name` is the TOML
# key the writer types and stays as it is.
settings-paratext-error-no-name = Una estructura necesita una línea `name`.
# Export: started with no project open (a menu raced a close).
export-error-no-project = No hay ningún proyecto abierto que exportar.

shortcut-group-work = Proyecto
shortcut-group-edit = Editar
shortcut-group-view = Ver
shortcut-group-document = Documento
shortcut-group-format = Formato
shortcut-group-go = Ir
shortcut-group-tools = Herramientas
shortcut-group-help = Ayuda

## Manuskript import: the headings of the notes it writes and the title of a project with no name.
import-manuskript-section-paragraph-summary = Resumen en un párrafo
import-manuskript-section-full-summary = Resumen completo
import-manuskript-section-motivation = Motivación
import-manuskript-section-epiphany = Epifanía
import-manuskript-section-notes = Notas
import-manuskript-section-description = Descripción
import-manuskript-section-passion = Pasión
import-manuskript-section-result = Resultado
import-manuskript-section-summary = Resumen
import-manuskript-section-meta = Metadatos
import-manuskript-section-series = Serie
import-manuskript-section-volume = Volumen
import-manuskript-section-genre = Género
import-manuskript-section-license = Licencia
import-manuskript-section-email = Correo electrónico
import-manuskript-section-situation = Situación
import-manuskript-section-sentence = Una frase
import-manuskript-section-paragraph = Un párrafo
import-manuskript-section-page = Una página
import-manuskript-section-full = Resumen completo
import-manuskript-untitled-project = Proyecto de Manuskript importado

## Plume import: the story-bible groups an old project implies, and the title of a project with no name.
import-plume-group-characters = Personajes
import-plume-group-items = Objetos
import-plume-group-places = Lugares
import-plume-untitled-project = Proyecto de Plume importado
