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

import-target-open-title = Este proyecto está abierto
import-target-open-text = «{ $name }» está abierto en Skribisto. Importar sobre él sustituiría el proyecto en el que está trabajando, y su siguiente guardado volvería a escribir ese proyecto sobre la importación. Elija otro nombre de archivo, o cierre antes el proyecto.
import-warnings-announcement = { $result } { $notice }.

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

cover-choose = Cubierta del &libro…
cover-clear = &Quitar la cubierta
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
shortcut-name-app-settings = Configuración
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
