# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

menu-work = &Proyecto
menu-new-work = N&uevo proyecto
menu-open-work = &Abrir proyecto…
menu-new-window = &Nueva ventana
menu-import-from = I&mportar desde
menu-create-from = &Crear desde
menu-import-plume = &Plume Creator (.plume)…
menu-import-document = &Documentos (Markdown, Word, ODT)...
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
menu-outline = &Estructura
menu-search = &Buscar en el proyecto
menu-timeline = &Volver al pasado
menu-search-preview = V&ista previa de búsqueda
menu-fullscreen = P&antalla completa
menu-focus-mode = M&odo sin distracciones
menu-format = &Formato
menu-scene-break = In&sertar el descanso de la escena
menu-major-scene-break = Insertar el salto &de escena principal

menu-go = &Ir
menu-go-next-scene = &Siguiente escena
menu-go-prev-scene = &Escena anterior
menu-go-next-chapter = Si&guiente Capítulo
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
ctx-open-to-side = A&bierto al lado
ctx-open = &Abrir
ctx-reveal-in-outline = &Reveal in Esquema
ctx-move-up = &Muévanse.
ctx-move-down = &Muévete.

ctx-tab-close = &Cerrar
ctx-tab-close-others = Ce&rrar las demás
ctx-tab-close-all = C&errar todas
ctx-tab-open-to-side = &Abierto al lado
ctx-tab-open-in-main = &Abierto en el panel principal
ctx-tab-move-to-side = Mu&évete al lado.
ctx-tab-move-to-main = Mu&évete al panel principal
ctx-tab-move-to-new-window = &Muévete en una nueva ventana
ctx-tab-move-window-unsaved = Guardar este proyecto primero: una segunda ventana se abre a un archivo en disco
ctx-tab-pin = &Pinta esta pestaña
ctx-tab-unpin = &Unpin esta pestaña
tab-pinned-tooltip = Pinned: "Cerrar a los demás" y "Cerrar todo" dejarlo abierto

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
new-item-paratext = Nuevo texto
new-item-paratext-folder = Nueva carpeta paratexto
new-item-story-bible-entry = Nueva historia Biblia Entrada

placement-inside = dentro
placement-after = después
placement-after-parent = después de padre
placement-top-level = nivel superior


ctx-promote = C&onvertir en
promote-chapter-folder = Carpeta de capítulo
promote-flat-chapter = Capítulo único
promote-lossy-title = No hay lugar para guardar el texto
promote-lossy-text = Un { $target } no tiene lugar para guardar estos: { $kinds }. Mueva o despeje el texto primero, luego conviértelo.
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
    [one] Este capítulo todavía contiene { $count } artículo. Mover o traspasarlo antes de convertirlo a un capítulo plano.
   *[other] Este capítulo todavía contiene { $count } artículos. Muévelos o desperdiciarlos antes de convertirlo a un capítulo plano.
}

inspector = Propiedades
inspector-empty = Abre un artículo para inspeccionarlo.
inspector-promote = Convertir en…
statusbar-toggle-outline = Mostrar u ocultar el archivador
statusbar-toggle-inspector = Mostrar u ocultar las propiedades
statusbar-save-unsaved = Cambios sin salvar. Haga clic para guardar
statusbar-save-saved = Todos los cambios guardados
statusbar-save-autosave = Los cambios se guardan a medida que escribes
statusbar-saving = Guardando…
statusbar-word-count = { $count ->
    [one] { $count } palabra
   *[other] { $count } palabras
}
statusbar-word-count-tooltip = Palabras en la escena que estás editando
statusbar-word-char-count = { $words ->
    [one] { $words } palabra
   *[other] { $words } palabras
} · { $chars ->
    [one] { $chars } carácter
   *[other] { $chars } caracteres
}
session-toggle = Sesión de escritura: iniciar o pausar una huella enfocada
session-configure = Establecer el objetivo de la sesión y el límite de tiempo
session-configure-title = Sesión de redacción
session-word-goal = Objetivo de palabras
session-time-limit = Plazo límite
session-time-limit-unit = min
session-no-goal = No hay meta
session-no-limit = No hay límite
session-reset = Período de sesiones de reasentamiento
session-readout = { $words } palabras · { $time }
session-readout-timed = { $words } palabras · { $time } izquierda
statusbar-focus-exit = Modo libre de distracción
statusbar-focus-synopsis = Sinopsis
statusbar-focus-go-prev = Artículo anterior (Alt+Up)
statusbar-focus-go-next = Siguiente artículo (Alt+Down)

rail-settings = Configuración

language = Idioma
theme = Tema
english = Inglés
french = Francés
light = Claro
dark = Oscuro
settings-text-width = Ancho de texto
settings-preview-width = Buscar vista previa
settings-autosave = Autosalvar al disco
settings-show-welcome = Mostrar el lanzador en el arranque
settings-show-welcome-tip = Cuando se apaga, el último proyecto se reabre en su lugar.
settings-check-for-updates = Comprobación de nuevas versiones
settings-check-for-updates-tip = Una vez al día a lo sumo, Skribisto pregunta el sitio web del proyecto que versión es actual. La solicitud no lleva identificador, y ni siquiera la versión que está ejecutando: la comparación ocurre en este equipo.

settings-title = Configuración
settings-close = Cerrar
settings-search = Ajustes de búsqueda
settings-reset = Restablecer valores predeterminados
settings-done = Listo
settings-cancel = Cancelar
settings-apply = Aplicar
settings-ok = Aceptar
settings-reset-confirm-title = ¿Reiniciar todos los ajustes a predeterminados?
settings-reset-confirm-body = Restaura la apariencia de esta aplicación, el editor y la configuración de escritura. La configuración de su proyecto, los atajos del teclado y los estilos guardados no se ven afectados.
settings-empty-title = Todavía no hay ajustes aquí
settings-empty-hint = Esta sección ganará opciones en una actualización futura.

settings-sec-appearance-behaviour = Comportamiento de Apariencia
settings-sec-editor = Editor
settings-sec-spelling = Ortografía
settings-sec-backup = Backup Sync
settings-sec-compile = Compilación y exportación
settings-page-appearance = Apariencia
settings-field-image-size-policy = Grandes imágenes
settings-image-policy-ask = Pregunte cada vez
settings-image-policy-keep = Mantener original
settings-image-policy-downscale = Optimizar
settings-hint-image-size-policy = Qué hacer cuando una imagen insertada es más grande que el manuscrito necesita.
settings-page-notifications = Notificaciones
settings-page-scene = Escena
settings-page-synopsis = Sinopsis
settings-page-notes = Notas
settings-page-editor-behavior = Comportamiento del editor
settings-page-goals = Conteo de palabras de objetivos
settings-page-games = Escribir juegos
settings-page-corkboard = Tablero de fichas
settings-page-distraction-free = Distracción libre
settings-page-dictionaries = Diccionarios
settings-page-autosave = Guardado automático
settings-page-export = Formatos de exportación
settings-page-paratext = Estructuras paralelas
settings-page-keymap = Atajos de teclado
settings-keymap-filter = Atajos de filtro

settings-desc-sec-appearance-behaviour = Cómo se ve la aplicación en sí misma, y qué hace cuando empieza.
settings-desc-sec-editor = La superficie de escritura: cómo se ve, y lo que hace como escribes.
settings-desc-sec-spelling = Comprobación de hechizos, y los diccionarios detrás de él.
settings-desc-sec-backup = Cómo su trabajo llega al disco, y qué copias se guardan.
settings-desc-sec-compile = Lo que deja Skribisto, y en qué forma.
settings-desc-sec-work = Ajustes que pertenecen a este proyecto y viajan dentro de su archivo.
settings-desc-sec-extensions = Páginas aportadas por las extensiones instaladas aquí.
settings-desc-group-typography = Una página por tipo de texto: tipografía, tamaño, altura de línea y espaciamiento.
settings-desc-appearance = Idioma de la interfaz, tema, tamaño del texto y ventana inicial.
settings-desc-notifications = Cada mensaje que esta sesión ha mostrado, y las acciones que puedes repetir.
settings-desc-scene = Cómo se establece la prosa de escena: tipografía, tamaño, altura de línea, indents y espaciamiento.
settings-desc-synopsis = Cómo se establece el panel de sinopsis, independientemente del manuscrito.
settings-desc-notes = Cómo se establecen las notas, independientemente del manuscrito.
settings-desc-corkboard = Tamaño de la tarjeta, lo que una tarjeta muestra, y cómo la tabla los arregla.
settings-desc-distraction-free = Tipografía, ancho de columna y tira de control para la escritura de pantalla completa.
settings-desc-distraction-free-themes = La biblioteca temática para la escritura de pantalla completa: los enviados y los suyos.
settings-desc-editor-behavior = Ancho de texto, posición de sinopsis, desplazamiento de máquina de escribir y el punto culminante.
settings-desc-punctuation = Los nuevos proyectos tipográficos de estilo de casa comienzan. Cada proyecto sigue esto a menos que establezca su propio.
settings-desc-goals = Palabras y objetivos de carácter, y cómo se cuentan las palabras.
settings-desc-games = Constraints usted se estableció mientras la redacción, como Siempre hacia adelante.
settings-desc-spellcheck = El único interruptor que gira la comprobación de hechizos en y apagado por todas partes.
settings-desc-dictionaries = Instalar, eliminar y navegar por los diccionarios de ortografía en esta máquina.
settings-desc-autosave = Ya sea que sus ediciones estén escritas en disco por su cuenta.
settings-desc-backup = Cuando se toman copias, donde se guardan, y cuántos. Cada proyecto sigue esto a menos que establezca su propio.
settings-desc-export = Los estilos que cada exportación compila a través de: los enviados y los suyos.
settings-desc-paratext = El frente y la espalda importan un nuevo proyecto puede comenzar con.
settings-desc-keymap = Cada atajo, y a lo que está obligado.
settings-page-user = Usuario
settings-desc-user = Quien eres, por los comentarios que escribes.
settings-group-identity = Identidad
settings-field-user-name = Tu nombre
settings-field-user-name-placeholder = Facultativo
settings-field-user-name-hint = Señala los comentarios y las respuestas que escribe. Distinta del nombre de autor de un proyecto, que es el byline del libro y viaja dentro del archivo.
settings-field-user-initials = Sus iniciales
settings-field-user-initials-placeholder = De tu nombre
settings-field-user-initials-hint = Lo que un procesador de palabras muestra al lado de su comentario en el margen. Déjalo en blanco para utilizar los que se muestran, tomado de su nombre.
settings-field-user-hint = Ambos son opcionales y aplican a cada proyecto en este ordenador. Cambiarlos firma los comentarios que escribas a continuación. Comentarios ya escritos guardan el nombre bajo el cual fueron escritos.
settings-desc-author = El nombre que aparece en este proyecto.
settings-desc-structure = Si los capítulos de este proyecto son carpetas o elementos individuales.
settings-desc-language = El lenguaje en el que se revisa la prosa de este proyecto.
settings-desc-work-backup = La propia política de respaldo de este proyecto, o la general.
settings-desc-personal-dictionary = Las palabras que este proyecto trata como correctamente deletreado.
settings-desc-tags = Las etiquetas codificadas por colores este proyecto marca sus artículos con.
settings-desc-templates = Las plantillas de notas que puede insertar al escribir en este proyecto.
settings-desc-text-replacements = Atajos que se expanden mientras escribes en este proyecto.
settings-desc-work-punctuation = Las citas, las dashes y el espaciamiento de la prosa de este proyecto siguen.

settings-group-typography = Tipografía
settings-group-writing-column = Escribir columna
settings-group-writing-view = Escribir la vista
settings-group-distraction-free-strip = Tira de control
settings-group-theme = Tema
settings-group-language = Idioma
settings-group-startup = Inicio
settings-group-updates = Actualizaciones
settings-group-autosave = Guardado automático
settings-field-typeface = Tipo de letra
settings-field-size = Tamaño del texto

editor-size-changed-manuscript = Tamaño del texto del manuscrito: { $percent }
editor-size-changed-synopsis = Tamaño del texto sinopsis: { $percent }
editor-size-changed-notes = Notas tamaño del texto: { $percent }
editor-size-changed-corkboard = Tamaño del texto de la tarjeta de cartón: { $percent }
editor-size-changed-corkboard-expanded = Tamaño ampliado del texto del editor: { $percent }
editor-size-changed-distraction-free = Tamaño del texto libre de Distracción: { $percent }

menu-text-size-increase = A&umentar el tamaño del texto
menu-text-size-decrease = &Disminución del tamaño del texto
menu-text-size-reset = &Cambiar el tamaño del texto
settings-field-line-height = Altura de la línea
settings-field-first-line-indent = Indent de primera línea
settings-field-paragraph-spacing-before = Espacio antes del párrafo
settings-field-paragraph-spacing-after = Espacio después del párrafo
settings-field-column-width = Ancho de columna
settings-distraction-free-width-hint = Se aplica sólo mientras que el modo libre de distracción está encendido. Sus anchos de columna Escena, Sinopsis y Notas son intactos.
settings-distraction-free-title = Mantenga el nombre del artículo
settings-distraction-free-word-count = Mantenga la palabra cuenta
settings-distraction-free-session = Mantener la sesión de redacción
settings-distraction-free-go-to = Mantenga el botón Go a...
settings-distraction-free-go = Mantenga los botones anteriores y siguientes
settings-distraction-free-chrome-hint = El botón de salida siempre se queda, lo que elijas aquí. Es tu manera de salir si Escape está ocupado.
settings-field-app-theme = Tema
settings-theme-light = Claro
settings-theme-dark = Oscuro
settings-field-text-scale = Tamaño del texto de la interfaz
settings-field-language = Lenguaje de interfaz
settings-synopsis-placement = Posición de sinopsis
settings-synopsis-placement-none = Ninguno
settings-synopsis-placement-top = Arriba
settings-synopsis-placement-side = Al lado
settings-typewriter = Tiposcriptor de desplazamiento
settings-typewriter-tip = Sostenga la línea que está escribiendo a una altura fija mientras que el manuscrito se desplaza debajo de ella. El clic todavía pone el cuidado donde se hace clic.
settings-typewriter-position = Posición de línea
settings-typewriter-position-top-third = Tercero superior
settings-typewriter-position-middle = Medio ambiente
settings-typewriter-position-bottom-quarter = Cuartel general
settings-highlight-scope = Destacar alrededor del cuidado
settings-highlight-scope-none = Ninguno
settings-highlight-scope-sentence = Sentencia
settings-highlight-scope-paragraph = Párrafo
settings-highlight-scope-tip-none = Deja la página plana. Nada está sombreado como escribes.
settings-highlight-scope-tip-sentence = Averigua la frase que estás escribiendo, así que se destaca de los que la rodean.
settings-highlight-scope-tip-paragraph = Averigua todo el párrafo que estás escribiendo, para mantener el pasaje en el que estás trabajando en la vista.
settings-group-container-views = Vistas de los contenedores
settings-remember-view = Recuerde la última vista para cada tipo de artículo
settings-remember-view-tip = Abra un contenedor en la vista que utilizó por última vez para ese tipo
settings-remember-view-tip-more =
    Un Libro, Parte y Capítulo cada uno ofrece varias vistas (su propia página, el completo
    manuscrito, la sinopsis completa). Enciende esto y cada tipo reopens en el
    vista que elegiste por última vez, por ejemplo, cambiar un Capítulo a Capítulo completo y el
    siguiente Capítulo que abre muestra Full Chapter también. Cada tipo recuerda su propia vista.
settings-group-counting = Palabra contando
settings-counting-auto = Automático (por idioma)
settings-counting-whitespace = Dividir en espacios
settings-counting-unicode-words = Palabras Unicode
settings-counting-cjk-hybrid = CJK-aware (por personaje)
settings-counting-hint = Automático cuenta chino y japonés por carácter y cada otro idioma por palabra. Cambialo sólo si el conteo se ve mal para tu idioma.
settings-group-goals-display = Visualización
settings-show-characters = Mostrar el número de caracteres en la barra de estado
settings-autosave-hint = Los cambios se escriben en el disco automáticamente como escribes.

settings-sec-work = Proyecto
settings-page-structure = Estructura
settings-page-author = Autor
settings-field-author-name = Nombre del autor
settings-field-author-placeholder = Facultativo
settings-field-author-hint = Aparece en la página de título compilado y en metadatos de archivos exportados. Déjalo en blanco para omitirlo. Este es el byline del libro. El nombre con el que se firman sus comentarios está en Ajustes - Usuario.
settings-group-chapters = Capítulos
tidy-titles-title = Tidy capítulo títulos
tidy-titles-none = Ningún título de capítulo o parte está simplemente repitiendo su propio número.
tidy-titles-lead = { $count ->
        [one] Un capítulo o parte se titula con nada más que su propio número.
       *[other] { $count } capítulos y partes se titulan con nada más que su propio número.
    }
tidy-titles-explain = Limpiar esos títulos deja cada uno llamado por el número que el libro ya sabe, el mismo número de las impresiones de exportación. Nada más cambia, y un deshacer los pone de vuelta.
menu-document-tidy-titles = &Tidy capítulo títulos...
settings-group-numbering = Número
settings-number-chapters = Número de capítulos y partes
settings-number-chapters-tip = Capítulos y partes llevan un número trabajado desde donde se sientan en el libro, mostrado junto a su título aquí, e impreso por la exportación.
settings-number-chapters-tip-more = El número nunca se almacena en el título, por lo que se mantiene a la derecha cuando se reordena, inserta o elimina. Apaga esto y los títulos de impresión de exportación solos, cualquiera que sea el estilo de exportación pide. Dejar un capítulo fuera (un prólogo, un interludio), utilizar el interruptor de numeración del Inspector en su lugar: que lo mantiene en el libro pero deja de tomar un número.
settings-part-resets-chapter = Reiniciar los números del capítulo en cada parte
settings-part-resets-chapter-tip = De forma predeterminada: los capítulos se ejecutan directamente a través de las partes de un libro, por lo que "Parte Dos" se abre en el Capítulo Once.
settings-part-resets-chapter-tip-more = Esa es la práctica habitual del comercio, y lo que un lector espera. Enciende para un libro cuyas partes están destinadas a leer como volúmenes separados, donde cada uno se abre en el Capítulo Uno.
settings-chapter-flat = Capítulos únicos
settings-chapter-flat-hint = En: un capítulo es un solo elemento. Escribes en él, y no tiene escenas. Off: un capítulo es una carpeta. Aún escribes en él, pero también puede tener escenas. Nuevos capítulos siguen este ajuste; los existentes se convierten a través de Promote.

settings-styles-builtin = Estilos incorporados
settings-styles-user = Mis estilos
settings-styles-builtin-badge = Incorporado
settings-styles-duplicate = Duplicar
settings-styles-edit = Editar
settings-styles-delete = Eliminar
settings-styles-import = Importar...
settings-styles-export = Exportar...
settings-styles-copy-suffix = (copia)
settings-styles-json-filter = Estilo de exportación
settings-styles-editor-title = Estilo de edición
settings-styles-editor-none = Selecciona un estilo personalizado para editarlo o duplica uno integrado.
settings-styles-imported = Estilo importado
settings-styles-import-failed = No podía importar estilo
settings-styles-exported = Estilo exportado
settings-styles-export-failed = No podía exportar estilo
settings-styles-field-name = Nombre
settings-styles-field-chapters = Capítulo epígrafes
settings-styles-field-parts = Títulos de parte
settings-styles-field-scene-break = Escena de descanso
settings-styles-field-major-scene-break = Un gran descanso de la escena
settings-styles-field-spacing = Línea de espaciamiento
settings-styles-field-justify = Justificar el texto
settings-styles-field-notes = Incluir notas
settings-styles-field-synopses = Include synops
settings-styles-field-scene-titles = Incluye títulos de escena
settings-styles-field-epigraphs = Incluir las epigrafías
settings-styles-field-paratexts = Incluir paratextos
settings-styles-field-epigraph-placement = Posición de epigrafo
settings-styles-epigraph-after = Después del título
settings-styles-epigraph-before = Antes del título
settings-styles-field-footnotes = Incluir notas de pie de página
settings-styles-field-footnote-numbering = Nota de pie de página
settings-styles-footnote-numbering-continuous = Contínuo
settings-styles-footnote-numbering-per-chapter = Reiniciar cada capítulo
settings-styles-footnote-numbering-per-book = Reiniciar cada libro
settings-styles-field-images = Imágenes
menu-image = I&magen
settings-styles-images-beside = Aparte del documento
settings-styles-images-embed = Dentro del documento
settings-styles-images-omit = Vete.
settings-styles-group-round-trip = Envío a un editor
settings-styles-field-comments = Incluir comentarios
settings-styles-field-round-trip-marks = Incluir marcadores de reimportación
settings-styles-round-trip-hint = Solo DOCX y ODT. Estos marcadores invisibles permiten actualizar el proyecto al reimportar el archivo revisado, en lugar de crear otra copia.
settings-styles-group-pages = Páginas
settings-styles-field-cover = Abrir con la cubierta
settings-styles-field-word-count = Cuenta la palabra con la página de título
settings-styles-field-page-books = Nueva página en cada libro
settings-styles-field-page-parts = Nueva página en cada parte
settings-styles-field-page-chapters = Nueva página en cada capítulo
settings-styles-field-page-paratexts = Nueva página en cada paratexto
settings-styles-heading-none = No.
settings-styles-heading-numbered = Número sólo
settings-styles-heading-title = Título únicamente
settings-styles-heading-both = Número + título
settings-styles-break-blank = Línea de blanco
settings-styles-break-none = Ninguno
settings-styles-spacing-single = Individual
settings-styles-spacing-onehalf = 1½
settings-styles-spacing-double = Doble

welcome-title = Bienvenido a Skribisto
welcome-version = Versión { $version }
welcome-update-available = Versión { $version } está disponible
welcome-update-tip = Abre la página de descarga en su navegador.
welcome-search = Búsqueda de obras
welcome-open = Abrir
welcome-new-work = Nuevo proyecto
welcome-create-from = Crear desde…
welcome-recent-works = Proyectos recientes
welcome-empty-recents = Todavía no hay obras recientes.
welcome-no-matches = Ningún trabajo reciente coincide con su búsqueda.
welcome-learn-soon = Pronto vendrán guías y consejos.
welcome-about-blurb = Skribisto, un Rust + Teksilo reescribir de la aplicación de escritura.
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
corkboard = Tablero de fichas
overview = Sinopsis

corkboard-card-count = { $count ->
    [one] Tarjeta { $count }
   *[other] { $count } tarjetas
}
corkboard-child-count = { $count ->
    [one] { $count } tema
   *[other] { $count } artículos
}
corkboard-view-nested = Anidado
corkboard-view-flat = Piso
corkboard-layout-hint = Anidado muestra a los elementos secundarios directos de un contenedor. Abre una tarjeta de carpeta para entrar en él. El plano muestra cada escena en todo el contenedor a la vez.
corkboard-card-size = Tamaño de la tarjeta
corkboard-search-placeholder = Tarjetas de filtro...
corkboard-empty-title = Aún no hay nada aquí.
corkboard-empty-hint = Use “+ Nuevo” arriba para añadir el primero.
corkboard-new = Nuevo
corkboard-show-card-numbers = Número de tarjetas
corkboard-modal-size = Tamaño ampliado del editor
corkboard-scope-hint = Se aplica a cada corkboard abierto, en cada proyecto.
corkboard-sort-manuscript = Orden de manuscrito
corkboard-sort-title-asc = Título A–Z
corkboard-sort-title-desc = Título Z-A
corkboard-card-number = Tarjeta { $number }

duplicate = Duplicar
duplicate-n = { $count ->
    [one] Duplicar { $count } tarjeta
   *[other] Duplicar { $count } tarjetas
}
set-label-n = { $count ->
    [one] Marcar la etiqueta en la tarjeta { $count }
   *[other] Marcar etiquetas en tarjetas { $count }
}
move-to-trash-n = { $count ->
    [one] Mover { $count } tarjeta a la basura
   *[other] Mover { $count } cartas a la basura
}
reveal-in-outline = Revestimiento en el esbozo

corkboard-move-to = Muévete a...
corkboard-move-to-n = { $count ->
    [one] Mover { $count } tarjeta a...
   *[other] Mueva { $count } tarjetas a...
}
corkboard-move-picker-title = Muévete a...
corkboard-move-picker-empty = No hay carpetas todavía.
corkboard-move-picker-cancel = Cancelar
corkboard-move-here = Muévete.
corkboard-moved-ok = { $count ->
    [one] { $count } tarjeta movida
   *[other] { $count } tarjetas movidas
}
corkboard-move-into-self = Un contenedor no se puede mover dentro de sí mismo. Escoge un destino fuera de él.
corkboard-move-failed = Esas cartas no podían ser trasladadas allí.

overview-col-title = Título
overview-col-type = Tipo
overview-col-label = Anotación
overview-col-tags = Etiquetas
overview-col-books = Libros
overview-col-own-words = Palabras
overview-col-total-words = Total
overview-row-count = { $count ->
    [one] { $count } tema
   *[other] { $count } artículos
}
overview-search-placeholder = Filtrar elementos...
overview-expand-all = Ampliar todos
overview-collapse-all = Contraer todo
overview-table-label = Índice
overview-empty-title = Aún no hay nada aquí.
overview-gone-title = Este contenedor se ha ido.
overview-gone-hint = Se movió a la Papelera. Restaurarla, o cerrar esta ficha.
overview-empty-hint = Use “+ Nuevo” arriba para añadir el primero.
corkboard-grid-label = Tarjetas de cartón
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
corkboard-badge-end = Final
settings-group-corkboard-layout = - ¿Qué?
settings-group-corkboard-cards = Tarjetas
corkboard-show-word-count = Mostrar palabras contar con tarjetas
text-heading = Texto
no-content = Este artículo no tiene contenido editable.
untitled = Sin título
placeholder-title = Título…
placeholder-subtitle = Subtítulo…
placeholder-chapter-title = Título del capítulo...
split-editor = Editor de Split
close-split-view = Vista dividida
drop-open-here = Abre aquí.
drop-open-to-side = Abierto al lado

segment-chapter = Capítulo
segment-part = Parte
segment-book = Libro
segment-pace = Ritmo
segment-notes = Notas
segment-story-bible = Biblia narrativa
segment-note-own = Nota
segment-note-details = Detalles
segment-note-in-prose = En prose
pace-placeholder = El planificador Pace aparece aquí.
pace-empty-title = Planifique el ritmo de este libro
pace-empty-body = Establecer un objetivo de palabra y un plazo, y Skribisto funciona el ritmo diario para llegar allí.
pace-start-planning = Planificación inicial
pace-section-schedule = Cuadro
pace-section-progress = Progresos
pace-advancement = Adelanto
pace-goal = Objetivo de palabras
pace-deadline = Fecha límite
pace-active = Plan de ritmo activo
pace-day-mon = lun
pace-day-tue = mar
pace-day-wed = mié
pace-day-thu = jue
pace-day-fri = vie
pace-day-sat = sáb
pace-day-sun = dom
pace-card-written = palabras escritas
pace-card-of-goal = de la meta
pace-card-rate = palabras / día de escritura
pace-card-days-left = días de escritura
pace-card-streak = día raak
pace-card-ahead = palabras por delante del ritmo
pace-card-behind = palabras detrás del ritmo
pace-charts-empty = Los gráficos de progreso aparecen aquí una vez que su cuenta de palabra se registra en un ahorro.
pace-chart-progression = Palabras escritas vs. blanco
pace-chart-words-per-day = Palabras por día
pace-series-actual = Actualidad
pace-series-target = Objetivo
pace-series-words-per-day = Palabras/día
pace-daily-target-line = Incluso el ritmo: { $count ->
    [one] { $count } palabras/día
   *[other] { $count } palabras/día
}
pace-section-holidays = Vacaciones
pace-section-milestones = Hitos
pace-holidays-none = No hay vacaciones, cada día programado cuenta.
pace-holiday-label = Nombre de la fiesta
pace-add-holiday = Añadir
pace-remove = Quitar
pace-milestones-none = No hay hitos todavía. Establecer uno en una parte o capítulo en el Inspector.
full-chapter = Capítulo completo
full-part = Parte completa
full-book = Libro completo
full-synopsis = Sinopsis completa
rename = Cambiar nombre
set-label = Marca de conjunto
insert-scene = Insertar escena
insert-chapter = Insertar capítulo
split-scene = Escena dividida
move-up = Muévanse.
move-down = Muévete.
merge-with-previous = Merge con anterioridad
move-to-trash = Muévete a la basura.
rename-chapter = Capítulo de renombre
add-scene = Agregar nueva escena
add-chapter = Añada nuevo capítulo
new-scene-title = Escena nueva
placeholder-scene-name = Nombre de escena...
menu-cut = Corte
menu-copy = Copiado
menu-paste = Pegado
menu-paste-unformatted = Paste Unformato
menu-select-all = Seleccione todos
format-bold = Negrita
format-italic = Cursiva
format-underline = Subrayado
format-strikethrough = Tachado

binder = Archivador
no-work = No hay trabajo cargado
no-recent-works = No hay obras recientes
switcher-open-section = Actualmente abierto
switcher-recent-section = Recientes
switcher-this-window = esta ventana
open-project-title = Proyecto abierto
open-project-question = ¿Cómo quieres abrir “{ $title }”?
open-in-new-window = Abrir en nueva ventana
open-here = Abre aquí.

binder-all = Todos los archivadores
binder-show-all = Mostrar todos los carpetas
binder-new = Nuevo binder...
binder-default-name = Nuevo binderista
binder-item-count = { $count ->
    [one] { $count } tema
   *[other] { $count } artículos
}
binder-search-placeholder = Filtra el esbozo...
binder-search-scope = Buscar todos los carpetas
binder-trash-confirm-title = ¿Mueva la carpeta a la basura?
binder-trash-confirm-text = “{ $name }” y todos sus elementos serán trasladados a la basura.

dialog-rename = Cambiar nombre
dialog-new-binder = Nuevo binderista
dialog-set-label = Marca de conjunto
dialog-new-scene = Nueva escena
close-work-question = ¿Ahorrar cambios antes de cerrar el trabajo?
quit-question = ¿Ahorrar cambios antes de dejar de fumar?
quit-save-work-question = Guardar cambios a { $title } antes de dejar de fumar?
unsaved-changes = Este trabajo tiene cambios sin salvar.
new-work-unsaved-question = ¿Ahorrar cambios antes de crear un nuevo trabajo?
open-work-unsaved-question = ¿Ahorrar cambios antes de abrir otro trabajo?
switch-backup-discard-title = ¿Discutir cambios en esta copia de seguridad?
switch-backup-discard-text = Los cambios en una copia de seguridad no se pueden guardar en ella. Utilice Guardar como o Restaurar para guardarlos, o descartarlos y abrir el otro trabajo.
switch-save-failed = El trabajo no pudo ser salvado, por lo que no fue reemplazado: { $error }
switch-save-not-started = El trabajo no podía ser salvado, así que no fue reemplazado.
close-save-failed = El trabajo no podía ser salvado, por lo que no estaba cerrado: { $error }
close-save-not-started = El trabajo no podía ser salvado, así que no estaba cerrado.
save-not-started = El trabajo no podía ser salvado.

could-not-open-work = No podía abrir "{ $file }"
could-not-open-work-too-new = "{ $file }" necesita un nuevo Skribisto
could-not-open-work-too-new-detail = Guardado por Skribisto formato { $written_by }; apertura necesita formato { $requires } o más reciente, y esta compilación soporta hasta el formato { $supported }. Actualizar Skribisto para abrirlo.
could-not-open-work-importing = "{ $file }" está siendo escrito por una importación
could-not-open-work-importing-detail = Una importación está escribiendo sobre este proyecto y no ha terminado. Ábrelo una vez que se haga la importación: abierto ahora, mostraría el proyecto que la importación está reemplazando, y su próximo ahorro escribiría ese proyecto de nuevo sobre la importación.
could-not-open-example = No podía abrir el ejemplo: { $error }
could-not-open-link = No podía abrir { $url }: { $error }
could-not-create-work = No podía crear trabajo: { $error }
saving-as-file = Salvando como { $target }...
saving-as-folder = Salvado como { $target }/...
saved-as = Ahorrado a { $target }
save-error = No se pudo salvar: { $error }
save-flush-failed = El ahorro pasó, pero estos documentos no pudieron ser entregados y todavía no son salvos: { $titles } ({ $error })
backup-error = No podía retroceder: { $error }
backing-up = Retrocediendo...
backup-nothing-open = Ningún proyecto está abierto al respaldo.
backup-already-running = Ya está en marcha una copia de seguridad.
backup-complete = Respaldo completo ({ $ok } salvado, { $skipped } ya actual)
backup-partial = Terminado el respaldo: { $ok } salvado, { $failed ->
    [one] { $failed } destino fallado
   *[other] { $failed } destinos fallados
}
backup-no-destination-title = No hay ubicación de copia de seguridad disponible
backup-no-destination-text = Ninguno de los destinos de copia de seguridad configurados puede ser alcanzado (por ejemplo, una unidad externa puede ser desplegada). Enchufe y reingrese, o salida sin respaldo.
backup-failed-close-title = La copia de seguridad no puede ser salvada
backup-failed-close-text = No se puede escribir copia de seguridad antes de cerrar. Cada destino falló (la unidad puede haber sido removida, o puede estar llena o protegida por escrito). Arreglarla y vuelva a entrar, o salir sin respaldo.

backup-choice-title = Archivo de copia de seguridad
backup-choice-heading = Has abierto una copia de seguridad.
backup-choice-subtitle = Esto es una copia de seguridad puntual de un proyecto.
backup-choice-subtitle-dated = Respaldo tomado { $date }.
backup-choice-body = Puede abrirlo y editarlo libremente, pero los cambios sólo se pueden guardar con Save As. El archivo original del proyecto no se toca. O restaurar este proyecto a exactamente esta copia de seguridad.
backup-choice-open = Abra la copia de seguridad
backup-choice-restore = Restaurar este proyecto hasta este punto...
backup-choice-not-a-backup = No, abrelo normalmente.
backup-banner-title = Copia de seguridad: los cambios no se pueden guardar aquí
backup-banner-description = Utilice Guardar como para mantener sus ediciones en un nuevo archivo, o Restaurar para reemplazar el proyecto original con esta copia de seguridad.
backup-banner-restore = Restaurar…
backup-banner-save-as = Salvar como...
backup-restore-original-missing = No se puede encontrar el proyecto original para restaurar. Use Save As para mantener esta copia de seguridad como un nuevo proyecto en su lugar.
backup-restore-close-elsewhere-title = Proyecto abierto en otra ventana
backup-restore-close-elsewhere-text = El proyecto que estás restaurando está abierto en otra ventana. Por favor, cierra allí primero, luego vuelve a entrar.
backup-restore-focus-window = Concéntrate en esa ventana.
backup-restore-confirm-title = ¿Restorear esta copia de seguridad?
backup-restore-confirm-text = La versión actual del proyecto será copiada aparte como una copia de seguridad antes de ser reemplazada por ésta.
backup-restore-confirm-ok = Restaurar
backup-restore-error = No se puede restaurar: { $error }
backup-restored-ok = Proyecto restaurado.
backup-restored-with-safety = Proyecto restaurado. Su versión anterior fue guardada a { $path }.
close-backup-discard-title = ¿Discutir cambios en esta copia de seguridad?
close-backup-discard-text = Los cambios en una copia de seguridad no se pueden guardar en ella. Use Save As para mantenerlos, o descarte y cierre.
quit-backup-discard-title = ¿Rechazar cambios y dejar de fumar?
quit-backup-discard-work-question = ¿Rechazar cambios a { $title } y dejar de fumar?
quit-backup-discard-text = Los cambios en una copia de seguridad no se pueden guardar en ella. Use Save As para mantenerlos, o descarte y dejar de fumar.
backup-nudge-text = No hay respaldos para este proyecto.
backup-nudge-action = Configurar copias de seguridad...

menu-backups-list = &Respaldos...
backups-title = Respaldos
backups-loading = Buscando copias de seguridad…
backups-empty = Todavía no se han encontrado respaldos para este proyecto.
backups-open = Abrir
backups-reveal = Mostrar ubicación
backups-delete = Eliminar esta copia de seguridad
backups-delete-confirm-title = ¿Supir esta copia de seguridad?
backups-delete-confirm-text = “{ $name }” será eliminado permanentemente. Esto no se puede deshacer.
backups-delete-error = No se pudo eliminar la copia de seguridad: { $error }
backups-refresh = Actualizar
backups-close = Cerrar

settings-page-backup = Por defecto de copia de seguridad
settings-page-work-backup = Respaldos
settings-backup-general-title = Ajustes de copia de seguridad predeterminados
settings-backup-work-title = Respaldos para este proyecto
settings-backup-inherit = Utilice la configuración de copia de seguridad general
settings-backup-inheriting = Este proyecto utiliza los ajustes generales de copia de seguridad.
settings-backup-none-hint = No se establecen copias de seguridad automáticas (todo el gatillo está apagado).
settings-backup-last = Última copia de seguridad: { $date }
settings-backup-last-never = Todavía no hay refuerzos.
settings-backup-open-list = Abrir la lista de respaldos...
settings-backup-triggers = Cuándo apoyarse
settings-backup-on-close = Atrás al cerrar el proyecto
settings-backup-on-open = Retrocede al abrir el proyecto
settings-backup-interval = Retrocede periódicamente, cada uno
settings-backup-destinations = Destinos de respaldo
settings-backup-default-location = Ubicación predeterminada
settings-backup-usage-since = { $count ->
    [one] { $count } copia de seguridad · { $size } · más antigua { $oldest }
   *[other] { $count } copias de seguridad · { $size } · más antigua { $oldest }
}

settings-backup-usage = { $count ->
    [one] { $count } copia de seguridad · { $size }
   *[other] { $count } copias de seguridad · { $size }
}

settings-backup-usage-empty = Todavía no hay nada aquí.
settings-backup-usage-measuring = Measuring...
settings-backup-reveal-root = Mostrar carpeta
settings-backup-dest-none = No hay destinos. Los respaldos se guardan junto al proyecto.
settings-backup-dest-remove = Quitar
settings-backup-dest-add = Añadir carpeta...
settings-backup-dest-refresh = Actualizar
settings-backup-retention = ¿Cuántos?
settings-backup-retention-tiered = Por niveles
settings-backup-retention-keep-n = Mantén la última N
settings-backup-retention-tip = Cuán viejas copias de seguridad están podadas.
settings-backup-retention-tip-more = Tiered mantiene una copia de seguridad por hora para un día, por día durante una semana, por semana durante un mes, y por mes más allá, por lo que la historia reciente se mantiene densa y la historia vieja se deslumbra. “Mantén la última N” simplemente guarda las copias más recientes de N. En ambos modos siempre se guardan las copias más recientes (el mínimo inferior).
settings-backup-gfs-hourly = Hora (último 24 h)
settings-backup-gfs-daily = Diario (última semana)
settings-backup-gfs-weekly = Semanal (último mes)
settings-backup-gfs-monthly = Mensual
settings-backup-keep-n = Número para mantener
settings-backup-min-keep = Siempre manténganse al menos
settings-backup-dedup = Saltar una copia de seguridad cuando nada ha cambiado

new-work-title = Nuevo proyecto
new-work-close = Cerrar
new-work-name = Nombre del proyecto
new-work-name-placeholder = Sin título
new-work-author = Autor
new-work-author-placeholder = Facultativo
new-work-format = Formato
new-work-single-file = Archivo único
new-work-bundle = Carpeta de proyecto
new-work-convert-later = Puedes convertir entre formatos más tarde.
new-work-location = Lugar
new-work-will-create = creará
new-work-language = Idioma predeterminado
new-work-language-hint = Aplicado a nuevos textos de verificación de hechizos. Cada texto se puede cambiar a otro idioma.
new-work-template = Plantilla
new-work-template-none = Ninguno
new-work-template-empty-novel = Novela vacía
new-work-template-light-novel = Light Novel
new-work-template-novel = Novela
new-work-template-novel-in-parts = Novela en las piezas
new-work-template-notebook = Cuaderno
new-work-cancel = Cancelar
new-work-create = Crear proyecto
new-work-step-details = Detalles
new-work-step-language = Estructura lingüística
new-work-step-template = Plantilla
new-work-back = Atrás
new-work-next = Siguiente
new-work-documents-title = Nuevo proyecto de documentos
new-work-step-import = Importar
new-work-create-and-import = Crear importación...
new-work-documents-next-title = Sus documentos son los siguientes.
new-work-documents-next-body = Creando el proyecto abre el asistente de importación sobre él, donde elija los documentos, revise la estructura Skribisto lee de ellos, y elija dónde aterriza. Nada está escrito en el proyecto hasta que lo confirme allí.
new-work-documents-no-template = Este proyecto comienza vacío a propósito: sin plantilla, por lo que los documentos importados son la única cosa en él.
new-work-documents-chapter-scene-hint = Aplica a los capítulos que crea la importación. Puedes cambiarla más adelante en la configuración del proyecto.
new-work-single-file-desc = Un archivo .skrib (zip). Portable, fácil de respaldar.
new-work-bundle-desc = Una carpeta que contiene todos los activos de texto.
new-work-template-none-count = carpeta vacía
new-work-template-empty-novel-count = 1 capítulo
new-work-template-light-novel-count = 15 capítulos
new-work-template-novel-count = 20 capítulos
new-work-template-novel-in-parts-count = 3 partes, 24 capítulos
new-work-template-notebook-count = notas de forma libre
new-work-chapter-scene = Capítulos únicos
new-work-chapter-scene-tip = Cada capítulo es un solo elemento que escribes directamente, sin escenas debajo de él. Deja esto para el diseño clásico, donde un capítulo es una carpeta: escribes directamente en eso también, pero también puede tener escenas.
new-work-chapter-scene-tip-more = Usted escribe en un capítulo de cualquier manera. La única diferencia es si puede *contener* escenas. El árbol de carpetas de Skribisto es sólo organizacional, por lo que ambos diseños compilan al mismo libro, usted puede mezclarlos libremente, y Promover convierte un capítulo entre los dos sin perder una palabra.
new-work-name-required = Introduzca un nombre para el trabajo
new-work-name-invalid = Este nombre no tiene caracteres utilizables
new-work-location-required = Elija una ubicación
new-work-location-missing = Esta carpeta no existe
new-work-location-not-folder = Este camino no es una carpeta
new-work-location-readonly = Esta carpeta no es deseable
new-work-target-exists = Un proyecto con este nombre ya está en esta carpeta. Crear el trabajo preguntará antes de reemplazarlo
new-work-target-is-folder = Una carpeta con este nombre ya está en esta ubicación. Elija otro nombre
new-work-overwrite-title = ¿Reemplazar el proyecto existente?
new-work-overwrite-text = “{ $name }” ya existe en esta carpeta. Reemplazarlo con el nuevo proyecto?
new-work-target-open-title = Este proyecto está abierto
new-work-target-open-text = “{ $name }” está abierto en Skribisto. Crear un nuevo proyecto en su lugar sustituiría el proyecto en el que está trabajando, y su próximo ahorro escribiría ese proyecto de nuevo sobre el nuevo. Elija otro nombre o carpeta, o cierre el proyecto primero.
target-importing-title = Una importación está escribiendo este proyecto
target-importing-text = Una importación sigue escribiendo “{ $name }”. Espere hasta que haya terminado, o elija otro nombre.
import-plume-busy-title = Se está importando otro proyecto Plume Creator
import-plume-busy-text = Plume Creator proyectos se importan uno a la vez. Espere hasta que la importación en curso haya terminado, o cancelarlo, a continuación, comience esta otra vez.
import-manuskript-busy-title = Se está importando otro proyecto Manuskript
import-manuskript-busy-text = Manuskript proyectos se importan uno a la vez. Espere hasta que la importación en curso haya terminado, o cancelarlo, a continuación, comience esta otra vez.
backup-restore-importing-text = Una importación sigue escribiendo “{ $name }”, el proyecto que esta copia de seguridad sería restaurada. Restaurar la copia de seguridad una vez que la importación haya terminado.

new-work-manuscript = Manuscrito
new-work-notes = Notas
new-work-research = Investigación
new-work-notebook = Cuaderno
new-work-scene = Escena
new-work-note = Nota
new-work-front-matter = Primera materia
new-work-back-matter = Retrocede.
new-work-paratext = Estructura del libro
new-work-paratext-none = No hay estructura
new-work-paratext-hint = El frente y la espalda importan una tradición que abre y cierra un libro con. Usted puede mover, renombrar o eliminar cualquiera de él después.
new-work-tags = Etiquetas
new-work-tags-none = No hay etiquetas
new-work-tags-hint = Una paleta inicial para etiquetar caracteres, lugares y el resto. Opcional, y cada etiqueta puede ser renombrada, recolorada o eliminada después.
new-work-note-templates = Plantillas de notas
new-work-note-templates-none = No hay plantillas
new-work-note-templates-hint = La forma en la que comienza una nota de la historia. Opcional, y cada plantilla puede ser editada o eliminada después.
new-work-characters = Personajes
new-work-places = Lugares

import-plume-title = Proyecto de importación Plume Creator
import-plume-close = Cerrar
import-plume-source = Proyecto de Plume
import-plume-source-hint = Elija un archivo .plume o .plume_backup (cualquier versión Plume Creator).
import-plume-location = Carpeta de destino
import-plume-name = Nombre del archivo
import-plume-name-placeholder = Nombre del proyecto
import-plume-will-create = creará
import-plume-trash-warning = Г Trashed / borrados artículos no son migrados.
import-plume-cancel = Cancelar
import-plume-import = Importar
import-plume-source-required = Elija un archivo de proyecto Plume
import-plume-source-missing = Este archivo no existe
import-plume-source-not-file = Esta ruta no es un archivo
import-plume-location-required = Elija una carpeta de destino
import-plume-location-missing = Esta carpeta no existe
import-plume-location-not-folder = Este camino no es una carpeta
import-plume-location-readonly = Esta carpeta no es deseable
import-plume-name-required = Introduzca un nombre de archivo
import-plume-name-exists = Un archivo con este nombre ya existe aquí. Importación confirmará sobreescritura
import-plume-overwrite-title = ¿Reemplazar el archivo existente?
import-plume-overwrite-text = “{ $name }” ya existe. Reemplazarlo con el proyecto importado?
import-plume-manuscript-binder = Manuscrito
import-plume-story-bible-binder = Historia Biblia
import-plume-progress-title = Importando proyecto Plume...
import-plume-cancel-import = Cancelar
import-plume-cancelled = Importación cancelada
import-plume-done = Importados { $imported ->
    [one] { $imported } artículo.
   *[other] { $imported } artículos.
} { $skipped ->
    [one] { $skipped } artículo destrozado no fue migrado.
   *[other] { $skipped } artículos destrozados no fueron migrados.
}
import-plume-open-now = Abre ahora.
import-plume-warnings = { $count ->
    [one] 1 cosa no se podía importar exactamente
   *[other] { $count } cosas no se podían importar exactamente
}
import-plume-details = Detalles
import-plume-warnings-title = Advertencias de importación
import-plume-error-title = No podía importar el proyecto
import-plume-nested-too-deep = El proyecto no fue importado: su parte “{ $part }” está anidada más de { $limit } niveles de profundidad. No Plume Creator proyecto se acerca a eso, y la lectura podría chocar Skribisto. Nada fue escrito.
import-plume-declares-entities = El proyecto no fue importado: su parte “{ $part }” declara entidades XML, que ningún proyecto Plume Creator lo hace. Ampliarlas podría tomar más memoria de lo que el ordenador tiene y chocar Skribisto. Nada fue escrito.
import-plume-error-details = Detalles

import-manuskript-title = Proyecto de importación Manuskript
import-manuskript-close = Cerrar
import-manuskript-source = Proyecto Manuskript
import-manuskript-source-hint = Elija el archivo .msk o la carpeta del proyecto (cualquier versión Manuskript).
import-manuskript-source-file = Elija el archivo...
import-manuskript-source-folder = Elija la carpeta...
import-manuskript-location = Carpeta de destino
import-manuskript-name = Nombre del archivo
import-manuskript-name-placeholder = Nombre del proyecto
import-manuskript-will-create = creará
import-manuskript-cancel = Cancelar
import-manuskript-import = Importar
import-manuskript-source-required = Elija un proyecto Manuskript
import-manuskript-source-missing = Este archivo o carpeta no existe
import-manuskript-location-required = Elija una carpeta de destino
import-manuskript-location-missing = Esta carpeta no existe
import-manuskript-location-not-folder = Este camino no es una carpeta
import-manuskript-location-readonly = Esta carpeta no es deseable
import-manuskript-name-required = Introduzca un nombre de archivo
import-manuskript-name-exists = Un archivo con este nombre ya existe aquí. Importación confirmará sobreescritura
import-manuskript-overwrite-title = ¿Reemplazar el archivo existente?
import-manuskript-overwrite-text = “{ $name }” ya existe. Reemplazarlo con el proyecto importado?
import-manuskript-manuscript-binder = Manuscrito
import-manuskript-story-bible-binder = Biblia narrativa
import-manuskript-characters-group = Personajes
import-manuskript-world-group = Mundo
import-manuskript-plots-group = Parcelas
import-manuskript-project-info-note = Información sobre proyectos
import-manuskript-summary-note = Resumen
import-manuskript-importance-minor = Menores
import-manuskript-importance-secondary = Secundaria
import-manuskript-importance-main = Principal
import-manuskript-progress-title = Importación del proyecto Manuskript...
import-manuskript-cancel-import = Cancelar
import-manuskript-cancelled = Importación cancelada
import-manuskript-done = { $imported ->
    [one] { $imported } artículo importado.
   *[other] { $imported } artículos importados.
} { $revisions ->
    [0] { "" }
    [one] { $revisions } versión anterior vino con él.
   *[other] { $revisions } versiones anteriores vinieron con ellos.
}
import-manuskript-open-now = Abre ahora.
import-manuskript-warnings = { $count ->
    [one] 1 cosa para saber sobre esta importación
   *[other] { $count } cosas para saber sobre esta importación
}
import-manuskript-details = Detalles
import-manuskript-warnings-title = Sobre esta importación
import-manuskript-error-title = No podía importar el proyecto
import-manuskript-nested-too-deep = El proyecto no fue importado: su archivo "{ $part }" está anidado más de { $limit } niveles de profundidad. No Manuskript proyecto se acerca a eso, y la lectura podría chocar Skribisto. Nada fue escrito.
import-manuskript-folders-too-deep = El proyecto no fue importado: uno de sus archivos se encuentra más de { $limit } carpetas profundas. No Manuskript proyecto se acerca a eso, y la lectura podría chocar Skribisto. Nada fue escrito.
import-manuskript-declares-entities = El proyecto no fue importado: su archivo "{ $part }" declara entidades XML, que ningún proyecto Manuskript hace. Ampliarlas podría tomar más memoria de lo que el ordenador tiene y chocar Skribisto. Nada fue escrito.
import-manuskript-error-details = Detalles

import-target-open-title = Este proyecto está abierto
import-target-open-text = “{ $name }” está abierto en Skribisto. Importar sobre él reemplazaría el proyecto en el que está trabajando, y su siguiente ahorro escribiría ese proyecto de nuevo sobre la importación. Elija otro nombre de archivo, o cierre el proyecto primero.
import-warnings-announcement = { $result } { $notice }.

export-title = Exportar
export-close = Cerrar
export-scope-label = ¿Qué?
export-format-label = Formato
export-style-label = Estilo
export-path-label = Archivo
export-browse = Examinar…
export-show-non-exportable = Mostrar no exportable
export-choose-empty = No hay proyecto para elegir.
export-cancel = Cancelar
export-export = Exportar
export-save-dialog-title = Exportar al archivo
export-section-what = Qué exportar
export-section-style = Estilo
export-section-destination = Destino
export-custom-selection = Selección personalizada
export-selected-count = { $count } seleccionada
export-chip-chapters-none = Capítulos: ninguno
export-chip-chapters-numbered = Capítulos: numerados
export-chip-chapters-title = Capítulos: título solamente
export-chip-chapters-both = Capítulos: número + título
export-chip-scene-break-glyph = Escena de descanso: { $glyph }
export-chip-scene-break-blank = Rotura de escena: línea en blanco
export-chip-scene-break-none = Escena de descanso: ninguna
export-chip-major-break-glyph = Gran quiebra { $glyph }
export-chip-major-break-blank = Principales rupturas: línea en blanco
export-chip-major-break-none = Principales rupturas: ninguna
export-chip-major-break-same = Ambos rompen las baldosas por igual
export-chip-spacing-single = Interlineado: sencillo
export-chip-spacing-onehalf = Espaciado: 11⁄2
export-chip-spacing-double = Espaciado: doble
export-chip-notes-included = Notas incluidas
export-chip-notes-excluded = Notas excluidas
export-format-docx = Palabra
export-format-odt = LibreOffice
export-format-html = HTML
export-format-markdown = Markdown
export-format-djot = Djot
export-format-text = Texto
export-format-latex = LaTeX
export-format-epub = EPUB
export-format-pdf = PDF
export-overwrite-title = ¿Reemplazar el archivo existente?
export-overwrite-text = “{ $name }” ya existe. Reemplazarlo?
export-progress-title = Exportando...
export-cancelled = Exportación cancelada
export-done = Exportado { $count ->
    [one] { $count } tema
   *[other] { $count } artículos
}
export-open-file = Abrir
export-show-in-folder = Mostrar en la carpeta
export-error-title = No se puede exportar
export-error-details = Detalles

backup-progress-start = Iniciando…
backup-progress-retention = Limpiando viejas copias de seguridad...
backup-progress-done = Listo
backup-progress-destination = Destino { $i } de { $n }
backup-details = Detalles
backup-issues-title = Problemas de respaldo
backup-failed-title = Respaldo fallado
backup-complete-prune-warning = Respaldo completo ({ $ok } salvado, { $skipped } ya actual). Algunas copias de seguridad antiguas no se pueden eliminar

search = Buscar
search-query-placeholder = Buscar…
search-replace-placeholder = Reemplazar con...
search-replace-toggle = Reemplazar
search-replace-all = Reemplazar todo
search-opt-case = Caso de coincidencia
search-opt-whole-word = Toda palabra
search-opt-diacritics = Acentos de partido
search-scope-body = Texto
search-scope-title = Título
search-scope-synopsis = Sinopsis
search-scope-label = Anotación
search-facet-book = Libros
search-facet-part = Partes
search-facet-chapter = Capítulos
search-facet-scene = Escenas
search-facet-note = Notas
search-facet-folder = Carpetas
search-tip-case = Caso de coincidencia: tratar la maleta superior y la minúscula como diferente, por lo que “Elena” y “elena” son partidos separados.
search-tip-whole-word = Toda la palabra: coincide sólo con palabras completas, por lo que “cat” no se encuentra dentro de la “categoría”.
search-tip-diacritics = Destacados: tratar letras acentuadas como distintas, por lo que “cafe” no coincide con “café”.
search-tip-body = Cuerpo: busque la prosa de escenas y notas.
search-tip-title = Título: busque los títulos de los artículos de carpeta.
search-tip-synopsis = Sinopsis: busque el resumen de cada artículo escrito.
search-tip-label = Etiqueta: busque la nota corta mostrada bajo el título de un artículo de carpeta.
search-tip-comment = Comentarios: buscar el texto de los hilos de comentario y sus respuestas. Un comentario es sobre el manuscrito en lugar de parte de él, por lo que tiene su propio cambio, y un comentario de hojas de reemplazo coincide sin trazar hasta que usted los garrapata.
search-tip-book = Libros: el contenedor de libros y sus marcadores de inicio / final.
search-tip-part = Partes: divisores de nivel parcial.
search-tip-chapter = Capítulos: capítulos, de cualquier manera el proyecto los almacena.
search-tip-scene = Escenas: los elementos que sostienen su prosa.
search-tip-note = Notas: notas de forma gratuita.
search-tip-folder = Carpetas: carpetas de organización simple y separadores.
search-tip-paratext = Un texto que pertenece al libro pero no a su historia: un prefacio, una dedicación, un postpalabra. Nunca contado en el manuscrito.
search-tip-preserve-case = Caso Preserve: un reemplazo toma el caso que encontró, por lo que “ELENA” se convierte en “MARTA” y “Elena” se convierte en “Marta”.
search-tip-replace = Reemplazar: mostrar el campo de reemplazo y Reemplazar Todo.
search-error = Fallo de la búsqueda: { $message }
search-no-matches = No hay coincidencias
search-count =
    { $matches ->
        [one] { $matches } partido
       *[other] { $matches } partidos
    } en { $items ->
        [one] Documento { $items }
       *[other] { $items } documentos
    }
search-count-truncated =
    { $matches ->
        [one] { $matches } partido
       *[other] { $matches } partidos
    } en { $items ->
        [one] Documento { $items }
       *[other] { $items } documentos
    } (sólo los primeros resultados)
search-occurrences = { $count }
search-field-body = Texto
search-field-title = Título
search-field-synopsis = Sinopsis
search-field-label = Anotación
search-field-epigraph = Epígrafe
search-field-comment = Comentario
search-field-comment-reply = Responder
search-field-footnote = Nota de pie
search-include-in-replace = Incluido en Reemplazar Todos
search-collapse-all = Colapso de todos los resultados
search-replace-here = Reemplaza esto
search-dismiss = Desestimación de los resultados
search-undo-dismiss = Traiga el último resultado desestimado
search-replace-nothing = (Nada)
search-replace-confirm-title = ¿Reemplazar todas las coincidencias?
search-replace-confirm-text =
    Reemplazamiento { $occurrences ->
        [one] { $occurrences } casos
       *[other] { $occurrences } sucesos
    } de "{ $query }" con "{ $replacement }" { $items ->
        [one] Documento { $items }
       *[other] { $items } documentos
    }? Usted puede deshacer esto de la notificación después.
search-replace-done-title = Sustitúyase completa
search-replace-done =
    { $occurrences ->
        [one] { $occurrences } casos
       *[other] { $occurrences } sucesos
    } sustituido en { $items ->
        [one] Documento { $items }
       *[other] { $items } documentos
    }.
search-replace-done-skipped =
    { $occurrences ->
        [one] { $occurrences } casos
       *[other] { $occurrences } sucesos
    } sustituido en { $items ->
        [one] Documento { $items }
       *[other] { $items } documentos
    }. { $skipped ->
        [one] { $skipped } campo
       *[other] { $skipped } campos
    } saltado (cambiado desde la búsqueda).
search-replace-undo = Deshacer
search-replace-failed-title = Reemplazamiento fallido
search-replace-skipped-title = Nada reemplazado
search-replace-skipped-body =
    { $fields ->
        [one] Este texto
       *[other] { $fields } de estos textos
    } cambió desde la búsqueda, por lo que { $fields ->
        [one] Era
       *[other] Eran
    } izquierda sola. Buscar de nuevo para ver dónde están las palabras ahora.
search-replace-undo-failed-title = Undo falló
search-preview = Vista previa
search-preview-empty = Seleccione un resultado para verlo aquí
search-preview-no-prose = Este partido no tiene texto editable
search-preview-prompt = Para ver una vista previa aquí, haga una búsqueda.
search-preview-open-search = Buscar en el proyecto
search-preview-footnote-prompt = Este partido está en el propio texto de una nota de pie de página. Ábrelo en el muelle de notas para ver y editarlo.
search-preview-open-footnotes = Notas de pie de página abiertas

find-placeholder = Encontrar en este documento...
find-count = { $current } de { $total }
find-no-results = No hay resultados
find-close = Encontrar cerca
find-previous = Previa coincidencia (Shift+Enter)
find-next = Siguiente partido (Enter)
find-opt-case = Caso de coincidencia
find-opt-whole-word = Toda palabra
find-replace-toggle = Mostrar u ocultar reemplazo (Ctrl+R)
find-replace-placeholder = Reemplazar con...
find-replace = Reemplazar
find-replace-all = Reemplazar todo
find-preserve-case = Caso Preserve

dict-download-title = Descargar { $name }...
dict-download-done = Instalado { $name }
dict-download-failed = No se puede descargar { $name }: { $error }
dict-removed = Eliminado { $name }
dict-accept-first = Aceptar la licencia antes de descargar { $name }
settings-dict-tab-installed = Instalados
settings-dict-tab-get-more = Obtener más
settings-dict-tab-personal = Palabras personales
dict-installed-empty = No hay diccionarios encontrados en este ordenador todavía.
dict-get-more-search = Idiomas de búsqueda
dict-system-badge = en su sistema
dict-unusable-badge = inutilizable
dict-download-button = Descargar
dict-downloading = Descargando...
dict-installed-label = Instalados
dict-remove = Quitar
dict-view-license = Ver licencia
dict-approx-size = ~{ $size }
dict-personal-empty = Aún no hay palabras personales en este proyecto.
dict-personal-add = Añadir
dict-personal-placeholder = Agregue una palabra...
dict-missing-toast = { $count ->
    [one] Este proyecto utiliza el diccionario { $count } que no tiene instalado
   *[other] Este proyecto utiliza { $count } diccionarios que no ha instalado
}
dict-missing-action = Consigue diccionarios
dict-license-title = { $name } licencias
dict-license-accept = Aceptar Descargar
dict-license-cancel = Cancelar
dict-license-close = Cerrar
dict-add-button = Añadir diccionario...
dict-add-title = Agregar un diccionario
dict-add-close = Cerrar
dict-add-name = Nombre
dict-add-name-placeholder = Por ejemplo, mi diccionario latino
dict-add-code = Código de idioma
dict-add-code-placeholder = p. ej., la o fr-FR-x-custom
dict-add-code-hint = Esto es lo que eliges como lenguaje de documento.
dict-add-aff = Archivo de afijos (.aff)
dict-add-dic = Lista de palabras (.dic)
dict-add-submit = Añadir
dict-add-cancel = Cancelar
dict-add-name-required = Dale un nombre al diccionario.
dict-add-code-required = Introduzca un código de idioma.
dict-add-code-invalid = Utilice sólo letras, dígitos, y - _ .
dict-add-code-reserved = Es un código de diccionario incorporado. Elija otro.
dict-add-file-required = Elige un archivo.
dict-add-file-missing = Ese archivo no existe.
dict-add-code-taken = Un diccionario para ese código ya está instalado.
dict-add-done = Añadido { $name }
dict-add-unusable = Estos archivos no son un diccionario utilizable: { $error }
dict-add-failed = No se pudo añadir el diccionario: { $error }
inspector-export = Exportar
inspector-exportable = Incluir en las exportaciones
ctx-number = &Número de este capítulo
ctx-unnumber = &No numerar este capítulo
inspector-numbering = Número
inspector-numbered = Número
inspector-numbered-tip = Este capítulo ocupa su lugar en la numeración del libro. Apágalo para un prólogo, un epílogo o un interludio.
inspector-numbered-tip-more = Un capítulo no numerado permanece en el libro exactamente como era: su encabezado, su prosa y su cuenta de palabra son intactos. Simplemente no imprime ningún número, y no utiliza uno arriba: el capítulo después de un prólogo es capítulo uno, no capítulo dos. Dejarlo fuera de la exportación en lugar es un cambio diferente, arriba, y que uno elimina el capítulo del libro por completo.
inspector-apply-to-children = Aplicar a los elementos secundarios
inspector-milestone = Fecha de Milestone
inspector-milestone-none = No hay fecha
inspector-milestone-clear = Despejado
inspector-tags = Etiquetas
inspector-aliases = Otros nombres
inspector-dict-language = Idioma
inspector-apply-language-to-children = Aplicar el idioma a los elementos secundarios
settings-page-language = Idioma
settings-field-dict-language = Idiomas
dict-tradeoff-hint = Cada idioma adicional acepta más palabras, por lo que menos errores son atrapados.
lang-inherit-hint = Heredado: esta escena utiliza los lenguajes del libro o del proyecto.
lang-pill-list = Idiomas
lang-pill-add = Agregar un idioma
lang-pill-remove = Eliminar { $name }
lang-pill-mute = Apaga el hechizo para { $name }
lang-pill-unmute = Activar la comprobación de hechizos para { $name }

settings-page-personal-dictionary = Diccionario personal
settings-user-dict-desc = Las palabras que añadas aquí son tratadas como correctamente deletreadas en todas partes de la aplicación, y nunca insignias de nuevo.
settings-user-dict-add-placeholder = Agregue una palabra...
settings-user-dict-add = Añadir palabra
settings-user-dict-search = Palabras filtrantes
settings-user-dict-count = { $count ->
    [one] { $count } palabra
   *[other] { $count } palabras
}
settings-user-dict-rename = Cambiar nombre
settings-user-dict-remove = Quitar
settings-user-dict-empty = No hay palabras todavía. Agregue palabras que el corrector de hechizos debe ignorar, o importar una lista.
settings-user-dict-import = Importar...
settings-user-dict-export = Exportar...
settings-user-dict-import-tip = Añadir palabras de un archivo de texto llano, una palabra por línea. Las palabras existentes se guardan.
settings-user-dict-export-tip = Guardar toda la lista a un archivo de texto llano, una palabra por línea.
settings-user-dict-txt-filter = Archivos de texto
settings-user-dict-duplicate = Ya en el diccionario
settings-user-dict-imported = Importados { $count ->
    [one] { $count } palabra
   *[other] { $count } palabras
} ({ $duplicates } ya presentes).
settings-user-dict-import-failed = No podía leer la lista de palabras: { $error }
settings-user-dict-exported = Guardado { $count ->
    [one] { $count } palabra.
   *[other] { $count } palabras.
}
settings-user-dict-export-failed = No podía guardar la lista de palabras: { $error }

settings-page-text-replacements = Sustituciones de texto
settings-text-repl-desc = Sustitúyase brevemente con el texto completo como usted escribe: "btw" se convierte "por el camino" en el momento en que escriba un espacio o puntuación.
settings-text-repl-enable = Utilizar reemplazos de texto en este proyecto
settings-text-repl-disabled-hint = Enciende esto para definir el cortocircuito que se expande como escribes.
settings-text-repl-add = Añádase la regla
settings-text-repl-trigger-placeholder = Abreviatura
settings-text-repl-replacement-placeholder = Lo que se hace
settings-text-repl-added = Añadido "{ $trigger }"
settings-text-repl-duplicate = "{ $trigger }" ya tiene una regla
settings-text-repl-filter = Reglas de filtro
settings-text-repl-count = { $n ->
    [one] 1 regla
   *[other] Reglas { $n }
}
settings-text-repl-row-enabled = Use esta regla
settings-text-repl-delete = Suprímase la regla para { $trigger }
settings-text-repl-deleted = Suprimió la regla para "{ $trigger }"
settings-text-repl-empty = Aún no hay reglas.
settings-text-repl-csv-filter = CSV archivos
settings-text-repl-import = Importar...
settings-text-repl-export = Exportar...
settings-text-repl-imported = Importado { $added }, saltado { $skipped }
settings-text-repl-exported = Exportado { $n ->
    [one] 1 regla
   *[other] Reglas { $n }
}

editor-menu-no-suggestions = No hay sugerencias
editor-menu-add-to-dictionary = Añadir “{ $word }” al diccionario
editor-menu-add-words-to-dictionary = Añadir palabras seleccionadas al diccionario
editor-dict-added = Añadido “{ $word }” a su diccionario.
editor-dict-added-multi = { $count ->
    [one] Se agregó { $count } palabra a su diccionario.
   *[other] Se agregó { $count } palabras a su diccionario.
}
toast-undo = Deshacer

undo-superseded-title = Ese paso ya no es el último.
undo-superseded-body = Algo más ha cambiado en este proyecto desde entonces. Deshacer tomaría eso en su lugar, por lo que no lo ha hecho. Nada de este paso ha sido revertido. Use Edit − Undo para pasar por la historia usted mismo.
undo-failed = Undo falló: { $error }

menu-edit = &Editar
menu-edit-undo = &Deshacer
menu-edit-redo = &Rehacer
menu-edit-undo-target = &Deshacer { $target }
menu-edit-redo-target = &Rehacer { $target }
shortcut-name-edit-undo = Deshacer
shortcut-name-edit-redo = Rehacer
undo-target-typing = mecanografía
undo-target-project = el último cambio a este proyecto
prose-history-reset-title = Realización de la historia de la escritura
prose-history-reset-body = { $count ->
    [one] Una escena abierta fue restaurada de un estado anterior, por lo que su historia de la escritura ya no se aplica.
   *[other] { $count } escenas abiertas fueron restauradas de un estado anterior, por lo que su historia de la escritura ya no se aplica.
}
undo-target-trash = moviéndose a la basura
undo-target-restore = restaurando de la basura
undo-target-delete-forever = borrar para el bien
undo-target-replace-all = sustitución en todo el proyecto
undo-target-import = la importación del documento
undo-target-duplicate = duplicación
undo-target-move = moviéndose
undo-target-merge = fusionando dos escenas
undo-target-split = dividiendo una escena
undo-target-promote = cambiar el tipo
undo-target-tidy-titles = ordenando los títulos del capítulo
undo-target-import-tags = Importación de etiquetas
undo-target-import-templates = plantillas de notas importadoras
undo-target-create = crear
undo-target-remove = eliminación
undo-target-rename = cambio de nombre
undo-target-edit = que edita
menu-edit-find = &Buscar…
menu-edit-find-next = Encontr&ar el siguiente
menu-edit-find-prev = E&ncontrar anterior
menu-edit-replace = Encontrar &y reemplazar...
menu-edit-find-in-project = Encon&trar en el proyecto...
menu-edit-replace-in-project = Ree&mplazar en proyecto...
undo-frozen = Undo (“Siempre adelante” está encendido)
redo-frozen = Redo (“Siempre adelante” está encendido)
menu-edit-cut = &Corte
menu-edit-copy = C&opiado
menu-edit-paste = &Pegado
menu-edit-paste-plain = Pe&gar sin formatear
menu-edit-select-all = &Seleccione todo
shortcut-name-edit-cut = Corte
shortcut-name-edit-copy = Copiado
shortcut-name-edit-paste = Pegado
shortcut-name-edit-paste-plain = Pegar sin formatear
shortcut-name-edit-select-all = Seleccione todo

titlebar-spellcheck-on = El cheque está encendido. Haga clic para dejar de comprobar (F7)
titlebar-spellcheck-off = El cheque está apagado. Haga clic para comprobar de nuevo (F7)
menu-spellcheck = &Comprobación de ortografía
menu-comments = &Comentarios
settings-page-spellcheck = Comprobación de la función
settings-group-spellcheck = Comprobación de la función
settings-spellcheck-enabled = Verificar la ortografía como escribo
settings-spellcheck-hint = Subraya las palabras que ningún diccionario instalado sabe. Apagar esto detiene todas las comprobaciones, en cada proyecto, hasta que la vuelva a encender. Para dejar de revisar un solo idioma, despejar su marca de verificación en el campo del idioma del trabajo o de un artículo.

menu-trash = &Papelera
trash-title = Papelera
trash-empty-state = La basura está vacía.
trash-empty-button = Vaciar papelera…
trash-restore = &Restaurar
trash-restore-to = &Restaurar a...
trash-delete-forever = &Eliminar para siempre
trash-restored-ok = { $count } restaurado.
trash-restore-error = No se puede restaurar: { $error }
trash-restore-orphaned = El punto original de este artículo se ha ido. Elige dónde restaurarlo.
trash-restore-no-project = Ningún proyecto está abierto, así que no hay nada que restaurar.
trash-delete-no-project = Ningún proyecto está abierto, así que no hay nada que eliminar.
trash-empty-confirm-title = ¿Vacía la basura?
trash-empty-confirm-text = { $count ->
    [one] { $count } La entrada de la trituración será eliminada permanentemente.
   *[other] { $count } entradas destrozadas serán eliminadas permanentemente.
} Dejan la basura para siempre. El deshacer todavía puede traerlos de vuelta hasta que hagas algo más.
trash-emptied-title = Trash vaciado
trash-emptied-body = Todo en la basura fue eliminado permanentemente.
trash-empty-no-project = Ningún proyecto está abierto, así que no hay basura para vaciar.
trash-delete-forever-confirm-title = ¿Supir para siempre?
trash-delete-forever-confirm-text = { $count ->
    [one] { $count } artículo será eliminado permanentemente.
   *[other] { $count } artículos serán eliminados permanentemente.
} Dejan la basura para siempre. El deshacer todavía puede traerlos de vuelta hasta que hagas algo más.
trash-deleted-title = Suprimido para siempre
trash-deleted-body = { $count } eliminado permanentemente.
trash-undo = Deshacer
trash-restore-picker-title = Restaurar a...
trash-restore-to-confirm-title = ¿Restorear aquí?
trash-restore-to-confirm-text = ¿Restaurar "{ $item }" en "{ $destination }"?
trash-restore-picker-restore-here = Restaurar aquí
trash-restore-picker-cancel = Cancelar
trash-restore-picker-empty = No hay carpetas todavía.
trash-banner-title = Este artículo está en la papelera
trash-banner-description = No aparecerá en el contorno o en las exportaciones hasta que lo restituyas.
trash-banner-restore = Restaurar…
trash-tab-tooltip = En Trash

settings-styles-sheet-yes = Sí.
settings-styles-sheet-no = No.
settings-styles-sheet-font = Fuente
settings-styles-sheet-indent = Indent de primera línea
settings-styles-sheet-para-spacing = Párrafo espaciamiento
settings-styles-sheet-page = Tamaño de la página
settings-styles-sheet-margins = Márgenes (sup./der./inf./izq.)
settings-styles-sheet-title-page = Página del título del libro
settings-styles-sheet-heading-language = Lenguaje de referencia
settings-styles-sheet-auto = Según el texto
settings-styles-sheet-digits = Números
settings-styles-sheet-direction = Dirección de texto
settings-styles-sheet-formats = Formatos
settings-styles-sheet-all-formats = Todos

settings-styles-editor-group = Editor de estilo
settings-styles-editor-missing = Ese estilo ya no está disponible.

settings-styles-page-letter = Carta
settings-styles-digits-western = Occidental (0–9)
settings-styles-digits-eastern-arabic = Árabe oriental ( ⁇ – ⁇ )
settings-styles-direction-ltr = Izquierda a derecha
settings-styles-direction-rtl = Derecho a la izquierda

format-dock-title = Formato
format-panel-empty = Coloque su cursor en una escena, nota o sinopsis para ver opciones de formato.
format-group-marks = Texto
format-group-block = Párrafo
format-group-lists = Listas
format-group-tables = Cuadro
format-group-breaks = Escenas.
format-superscript = Superíndice
format-subscript = Subíndice
format-link = Enlace…
link-dialog-insert-title = Insertar el enlace
link-dialog-edit-title = Editar enlace
link-dialog-text-label = Texto
link-dialog-text-placeholder = las palabras que el lector ve
link-dialog-url-label = Enlaces a
link-dialog-url-placeholder = ejemplo.com
link-dialog-insert = Insertar
link-dialog-apply = Aplicar
link-dialog-cancel = Cancelar
link-dialog-remove = Eliminar el enlace
link-scheme-refused = Sólo se pueden abrir enlaces web y correo electrónico. { $url } quedó solo.
menu-format-link = &Enlace…
format-clear = Formatos claros
format-heading = Nivel de dirección
format-heading-normal = Texto normal
format-heading-1 = Título 1
format-heading-2 = Título 2
format-heading-3 = Título 3
format-heading-4 = Título 4
format-heading-5 = Título 5
format-heading-6 = Título 6
format-align-left = Alinear la izquierda
format-align-center = Centrar
format-direction-rtl = Párrafo derecho a la izquierda
format-blockquote = Cita en bloque
format-list-bullet = Lista con viñetas
format-list-numbered = Lista numerada
format-indent = Aumentar sangría
format-outdent = Reducir sangría
format-table-insert = Insertar la tabla
format-table-row-above = Insertar la fila arriba
format-table-row-below = Insertar la fila abajo
format-table-col-before = Insertar la columna antes
format-table-col-after = Insertar la columna después
format-table-row-delete = Eliminar la fila
format-table-col-delete = Suprimir la columna
format-table-remove = Quitar la mesa

menu-format-marks-bold = &Negrita
menu-format-marks-italic = Cu&rsiva
menu-format-marks-underline = Subra&yado
menu-format-marks-strike = Ta&chado
menu-format-marks-superscript = Su&períndice
menu-format-marks-subscript = Su&bíndice
menu-format-marks-clear = &Formato claro
menu-format-heading = &Título
menu-format-heading-normal = T&exto normal
menu-format-heading-1 = &Título 1
menu-format-heading-2 = T&ítulo 2
menu-format-heading-3 = Tít&ulo 3
menu-format-heading-4 = Títu&lo 4
menu-format-heading-5 = Títul&o 5
menu-format-heading-6 = Título &6
menu-format-alignment = &Alineación
menu-format-align-left = &Align Left
menu-format-align-center = &Centrar
menu-format-direction = Direcci&ón
menu-format-direction-auto = &Automático
menu-format-direction-ltr = &Izquierda a la derecha
menu-format-direction-rtl = &Derecho a la izquierda
menu-format-blockquote = C&ita en bloque
menu-format-lists = &Listas
menu-format-list-bullet = &Lista con viñetas
menu-format-list-numbered = L&ista numerada
menu-format-indent = &Aumentar sangría
menu-format-outdent = &Reducir sangría
menu-format-table = C&uadro
menu-format-table-insert = &Insertar la tabla
menu-format-table-2x2 = &2 x 2
menu-format-table-3x3 = &3 x 3
menu-format-table-4x4 = &4 x 4
menu-format-table-row-above = Ins&ertar la fila por encima
menu-format-table-row-below = I&nsertar la fila abajo
menu-format-table-col-before = In&sertar Columna Antes
menu-format-table-col-after = Inse&rtar la columna después
menu-format-table-row-delete = &Borrar la fila
menu-format-table-col-delete = B&orrar la columna
menu-format-table-remove = &Quitar la mesa

menu-format-typo-marks = For&mato de marca
menu-format-typo-nbsp = &Espacio sin ruptura
menu-format-typo-nbhyphen = &No romper Hyphen
menu-format-typo-soft-hyphen = &Guion opcional
menu-format-typo-nnbsp = &No romper el espacio
menu-format-typo-zwsp = &Espacio de ancho cero
menu-format-typo-word-joiner = &Unión de palabras


menu-about = A&lrededor de Skribisto...
about-title = Acerca de Skribisto
about-version = Versión { $version }
about-update-available = Versión { $version } está disponible
about-update-see-notes = Mira lo que cambió
about-update-flathub = Flathub mantiene esta copia hasta la fecha.
about-update-distro = Su distribución mantiene esta copia actualizada.
about-tagline = Una aplicación de escritura novedosa para ficción de larga duración, escrita en Rust con el kit de herramientas Teksilo.
about-license = Publicado bajo la Licencia Pública General de GNU, versión 3.
about-copyright = © 2026 Cyril Jacquet
about-close = Cerrar

native-menu-about = Acerca de { $app }
native-menu-hide = Ocultar { $app }
native-menu-quit = Salir de { $app }
native-menu-settings = Ajustes...
native-menu-window = Ventana
native-menu-minimize = Minimizar
native-menu-zoom = Zoom

window-title = { $title } - { $app }
window-title-numbered = { $title } - { $app } (Ventana { $n })
window-title-empty = { $app }

first-run-window-title = Bienvenido.
first-run-title = Configuración { $app }
first-run-body = { $app } mantiene sus configuraciones separadas de Skribisto, por lo que comienza vacía. Sus preferencias, proyectos recientes, diccionarios y diseño de ventanas se pueden copiar en todo ahora.
first-run-from = Copiado
first-run-to = Copiado
first-run-copy-note = Nada se mueve o se elimina. Skribisto mantiene cada uno de sus propios ajustes, y sigue trabajando exactamente como antes.
first-run-import = Ajustes de importación
first-run-start-fresh = Comienzo fresco
first-run-import-failed = No se pueden importar algunos ajustes: { $error }

settings-page-punctuation = Predeterminados de puntuación
settings-group-punctuation = Puntualización inteligente
settings-page-work-punctuation = Puntuación
settings-punctuation-override = Dar a este proyecto sus propias reglas de puntuación
settings-punctuation-override-hint = Off: el proyecto sigue la preferencia de aplicación. Estas reglas viajan dentro del .skrib, por lo que un coautor que abre el archivo escribe con la misma tipografía.
settings-punctuation-dashes = Convierte... en un dash en un em dash,
settings-punctuation-ellipsis = Convierta ... en una elipsis
settings-punctuation-quotes = Marcas de comillas y apostrofes
settings-quote-style = Marcas de calificación
settings-quote-style-locale = Por defecto de idioma
settings-quote-style-curly = "Doble"
settings-quote-style-curly-single = "Single"
settings-quote-style-guillemets = «Guillemets»
settings-quote-style-low-high = "Low-high"
settings-punctuation-spacing = Espacio antes de ; : !
settings-punctuation-spacing-hint = La tipografía francesa establece un espacio estrecho sin romper antes; ! ? y uno completo antes :. Se aplica sólo al texto escrito en francés. El espacio dentro de los guillemets « » viene con el guillemet citas ellos mismos.
settings-punctuation-sample = Su idioma le da
settings-punctuation-app-hint = Lo que hace cada proyecto a menos que tome reglas propias en el Proyecto - Punturación.
settings-punctuation-dialogue = Abrir un párrafo escrito como "-" con una barra de diálogo
settings-punctuation-dialogue-hint = Para los idiomas que marcan el discurso con un dash en lugar de comillas: francés, español, ruso y otros. Sólo se dispara al comienzo mismo de un párrafo.

statusbar-go-to = Ve a...
go-to-placeholder = Buscar el carpeta
go-to-no-matches = Ningún artículo coincide con esa búsqueda.
menu-go-to = &Ve a...

settings-page-distraction-free-themes = Temas libres de distracción
settings-themes-builtin = Temas incorporados
settings-themes-builtin-badge = Incorporado
settings-themes-user = Mis temas
settings-themes-editor-group = Editar el tema
settings-themes-editor-empty = Escoge un tema bajo "Mis temas" para editarlo.
settings-themes-use = Uso
settings-themes-duplicate = Duplicar
settings-themes-edit = Editar
settings-themes-delete = Eliminar
settings-themes-export = Exportar...
settings-themes-import = Importar...
settings-themes-copy-suffix = Copia
settings-themes-json-filter = Tema (JSON)
settings-themes-imported = Tema importado
settings-themes-import-failed = No podía importar ese tema
settings-themes-exported = Tema exportado
settings-themes-export-failed = No podía exportar ese tema
settings-themes-low-contrast = bajo contraste
settings-themes-low-contrast-band = destaca el texto
settings-themes-field-name = Nombre
settings-themes-field-paper = Página
settings-themes-field-ink = Texto
settings-themes-field-general = Antecedentes
settings-themes-field-widget-text = Texto de la tira de control
settings-themes-field-caret-band = Destacar alrededor del cuidado

statusbar-focus-settings = Ajustes libres de Distracción
statusbar-focus-manage-themes = Gestionar temas...

comments-title = Comentarios
comments-document-title = Este documento
comments-empty-project = Aún no hay comentarios en este proyecto.
comments-empty-document = No hay comentarios sobre este documento.
comments-filter-all = Todos
comments-filter-open = Abrir
comments-filter-resolved = Resuelto
comments-filter-orphaned = Sin texto asociado
comments-status-open = Abrir
comments-status-resolved = Resuelto
comments-status-orphaned = Ha perdido su texto
comments-orphan-snippet = (el texto comentado se ha ido)
comments-status-unplaced = No hay posición de texto
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
comments-sort-reader = Grupo de lectores
comments-unsigned-author = No asignados
comments-agreement = { $count ->
    [one] Lector { $count }
   *[other] { $count } lectores
}
comments-menu-add = Agregar comentario
comments-menu-add-paragraph = Comentario sobre este párrafo
overview-col-comments = Comentarios
overview-col-total-comments = Total de observaciones
comments-card-placeholder = Escribe un comentario...
comments-card-unknown-author = Autor desconocido
comments-unsigned-toast = Sus comentarios no están firmados: ningún nombre está establecido para usted en este equipo.
comments-unsigned-action = Pon tu nombre
comments-card-reply = Responder
comments-card-reply-placeholder = Responder...
comments-card-actions = Medidas de observación
comments-reply-actions = Respuestas
comments-menu-delete-reply = Suprímase respuesta
comments-menu-delete-all = Eliminar todos los comentarios aquí
comments-deleted-toast = Comentario eliminado
comments-reply-deleted-toast = Respuesta eliminada
comments-deleted-all-toast = { $count ->
    [one] { $count } comentario borrado
   *[other] { $count } comentarios borrados
}
comments-undo = Deshacer

analysis-segment = Análisis
analysis-scope-book = Analizando este libro
analysis-run = Análisis de ejecución
analysis-stale = Cambio desde que esto corrió
analysis-not-run = Todavía no analizado.
analysis-running = Leyendo el manuscrito...
analysis-failed = El análisis no pudo terminar.
analysis-no-scenes = Todavía no hay escenas en este libro.

analysis-shape = Forma

analysis-arrivals = Llegadas
analysis-arrivals-explainer = Cómo llegó el texto a este proyecto mientras que ha estado abierto. Dice qué ruta descendieron los personajes, y nada sobre quién los escribió: un escritor que redactó en otros lugares y pastas ha pegado, y uno que dicta. No hay ningún número para apuntar aquí.
analysis-arrivals-scope = Todo este proyecto, no sólo este libro.
analysis-arrivals-session = Desde que lo abriste, cerrar el proyecto comienza la cuenta otra vez.
analysis-arrivals-nothing = Todavía no ha llegado ningún texto en este período de sesiones.
analysis-arrivals-typed = Tipos
analysis-arrivals-pasted = Pegado
analysis-arrivals-dictated = Dictado
analysis-arrivals-imported = Importados
analysis-arrivals-programmatic = Insertado para usted
analysis-arrivals-count = { $count ->
    [one] { $count } carácter
   *[other] { $count } caracteres
}
analysis-arrivals-none = ninguno

analysis-words-per-scene = Palabras por escena
analysis-median-words = La escena mediana de este libro corre { $count ->
    [one] { $count } palabra.
   *[other] { $count } palabras.
}
analysis-median-line = Mediano: { $count ->
    [one] { $count } palabra
   *[other] { $count } palabras
}
analysis-dialogue = Diálogo
analysis-dialogue-unsupported = Aún no se mide el diálogo para este idioma.

analysis-footnote-words = Palabras de pie de página
analysis-footnote-words-count = { $count ->
    [one] { $count } palabra está en las notas de pie de página de este libro, apartado del total del manuscrito.
   *[other] { $count } palabras están en las notas de pie de página de este libro, separadas del total del manuscrito.
}
analysis-footnote-words-pending = Contando las notas de pie de página...

analysis-ignore-empty = Ignorar textos con nada escrito todavía
analysis-empty-hidden = { $count } vacío { $count ->
        [one] texto
       *[other] textos
    } ocultos.
analysis-all-texts-empty = Cada texto de este libro sigue vacío.

binder-filter-count = { $shown } de { $total } mostrado
binder-filter-clear = Despejado
binder-filter-none = Nada coincide con “{ $query }”.

settings-paratext-intro = La materia delantera y trasera puede comenzar con un nuevo proyecto. Cada estructura pertenece a una tradición editorial, y sus títulos de página están escritos en el propio idioma de esa tradición. Renombrarlos libremente una vez que se crea un proyecto.
settings-paratext-structures = Estructuras
settings-paratext-broken = No se puede leer
settings-paratext-edit = Editar
settings-paratext-duplicate = Duplicar
settings-paratext-delete = Eliminar
settings-paratext-new = Nueva estructura
settings-paratext-save = Guardar
settings-paratext-editor-hint = Un nombre, las páginas para crear antes del manuscrito, y las páginas para crear después de él. Donde terminan es tuyo: son artículos ordinarios una vez que el proyecto existe.


image-insert = I&nsertar la imagen...
image-no-project = Abra un proyecto antes de insertar una imagen.
image-choose-title = Elija una imagen
image-filter-label = Imágenes
image-large-title = Esta es una imagen grande
image-large-text =
    { $name } es { $megapixels } megapíxeles ({ $width }×{ $height }).
    Mantenerlo como es almacena su archivo original en el proyecto: viaja
    con cada copia de seguridad y cada exportación. Optimising almacena una copia más pequeña
    en su lugar, hasta 2560 píxeles en su lado más largo.
image-large-keep = Mantener original
image-large-downscale = Optimizar
image-large-remember = Haz esto de ahora en adelante, no preguntes otra vez
image-not-recorded = La imagen fue salvada pero no se pudo grabar en el proyecto.
image-describe-title = Describir la imagen
image-describe-explain = Lo que muestra la imagen, para un lector que no puede verla. No es parte del manuscrito: nunca se cuenta, se registra o exporta como prosa.
image-describe-placeholder = un faro contra un cielo gris
image-resize-title = Redimensionar la imagen
image-resize-explain = Un porcentaje del tamaño que se muestra ahora. 100 lo deja como es.
image-resize-invalid = Introduzca un número entre 1 y 1000.
image-menu-describe = &Describir la imagen...
image-menu-resize = &Redimensionar la imagen...
image-menu-reset-size = &Tamaño original

cover-choose = Cub&ierta de libro...
cover-clear = Re&tire la cubierta
cover-choose-title = Elija una cubierta
cover-set = La cubierta está lista.
cover-cleared = La cubierta ha sido eliminada, la imagen sigue en el proyecto.
card-label = Anotación
card-exportable = Exportado
card-numbered = Número
card-created = Creado
card-modified = Modificado
card-yes = Sí.
card-no = no
card-type = Tipo
card-position = Posición
card-children = Niños
card-goal = Objetivo
card-words = Palabras
card-synopsis = Sinopsis
card-point-of-view = Punto de vista
card-aliases = Otros nombres

export-orphan-footnotes-title = Notas de pie sin referencia
export-comments-dropped =
    { $count ->
        [one] Un comentario no se pudo colocar en el texto exportado y se dejó fuera.
       *[other] { $count } comentarios no pudieron ser colocados en el texto exportado y fueron dejados fuera.
    }
export-orphan-footnotes =
    { $count ->
        [one] Una nota de pie de página ya no se hace referencia en ninguna parte del manuscrito. Su texto no aparecerá en el libro exportado.
       *[other] { $count } notas de pie ya no se refieren en ninguna parte del manuscrito. Su texto no aparecerá en el libro exportado.
    }


menu-footnotes = &Notas de pie de página
footnotes-title = Notas de pie de página
footnotes-insert = In&sertar la nota de pie de página
footnotes-empty = Aún no hay notas de pie. Ponga el cursor en una escena, luego utilice + arriba, o Ctrl+Alt+F.
footnotes-filter-all = Todos
footnotes-filter-document = Este documento
footnotes-filter-orphaned = Sin texto asociado
footnotes-orphaned = Nada apunta más a esta nota
footnotes-untitled-home = Sin título
footnotes-body-placeholder = la nota en sí misma
footnotes-insert-tooltip = Insertar una nota de pie de página en el cursor (Ctrl+Alt+F)
footnotes-actions = Medidas de nota de pie de página
footnotes-delete = &Borrar la nota y su referencia
footnotes-no-project = Abra un proyecto antes de insertar una nota de pie de página.
footnotes-no-caret = Ponga el cursor en el texto de una escena para insertar una nota de pie de página allí.
footnotes-not-created = La nota de pie de página no se puede añadir al proyecto.
footnotes-deleted-toast = Nota eliminada
footnotes-undo-delete = Deshacer

import-document-title = Documentos de importación
import-document-close = Cerrar
import-document-step-files = Archivos
import-document-step-review = Examen
import-document-step-destination = Destino
import-document-step-reconcile = Combinar
import-document-reconcile-hint = Algunos de estos artículos son los que ya tienes. Di lo que debe pasar a cada uno.
import-document-reconcile-all-new = Nada en este archivo coincide con su proyecto: cada artículo será añadido como nuevo.
import-document-duplicate-returns = Estos archivos son más de una copia del mismo manuscrito que regresa. Importarlos uno a la vez: emparejar sólo líneas arriba una copia de vuelta contra su proyecto, por lo que importar varios juntos agregaría una segunda copia del libro en lugar de fusionarlo.
import-document-hunk-added = Tomar este nuevo párrafo
import-document-hunk-removed = Suprimir este párrafo
import-document-hunk-changed = Toma esta reescritura
import-document-col-stray-prose = Su texto
import-document-stray-as-paratext = Mantenerse como pretexto
import-document-stray-discard = Suelta el texto
import-document-col-current = En su proyecto
import-document-col-incoming = En este archivo
import-document-col-status = Situación
import-document-col-action = Qué hacer
import-document-status-identical = Igual
import-document-status-editor-edited = Modificado por el lector
import-document-status-you-edited = Modificado por ti
import-document-status-conflict = Modificado por ambos
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
import-document-drop-hint = Markdown (.md), Word (.docx), OpenDocument (.odt) y texto llano (.txt)
import-document-browse = Examinar…
import-document-move-up = Muévanse.
import-document-move-down = Muévete.
import-document-remove-file = Quitar
import-document-file-count = { $count ->
    [one] 1 archivo
   *[other] { $count } archivos
}
import-document-no-files = Todavía no hay archivos escogidos.
import-document-col-included = Importar
import-document-col-title = Título
import-document-col-type = Tipo
import-document-col-words = Palabras
import-document-col-breaks = Saltos
import-document-col-comments = Comentarios
import-document-col-footnotes = Notas de pie de página
import-document-col-epigraph = Epígrafe
import-document-col-source = Origen
import-document-level-rules = Niveles de dirección
import-document-level-n = Dirección { $level }
import-document-add-top-level = Añadir nivel superior
import-document-add-top-level-tooltip = Insertar un libro sobre todos los elementos analizados, para los archivos del capítulo que no tienen título de libro
import-document-destination = Destino
import-document-destination-hint = Elija un carpeta o elemento: nuevos elementos aterrizan dentro de una carpeta, o después de una escena.
import-document-destination-empty = No hay carpetas todavía.
import-document-plan-empty = Aún no hay nada que importar.
import-document-summary = { $rows ->
    [one] 1 tema
   *[other] { $rows } artículos
} · { $breaks ->
    [one] 1 escena de descanso
   *[other] { $breaks } pausas de escena
}
import-document-back = Atrás
import-document-cancel = Cancelar
import-document-analyse = Siguiente
import-document-analysing = Leyendo documentos...
import-document-step-analysing = Lectura
import-document-cancel-analysis = Deja de leer
import-document-analyse-failed = Los documentos no pueden leerse.
import-document-details = Detalles
import-document-import = Importar
import-document-done = { $count ->
    [one] 1 objeto importado
   *[other] { $count } artículos importados
}
import-document-undo = Deshacer
import-diagnostic-file-unreadable = “{ $path }” no se podía leer: { $detail }. Los otros archivos todavía importan.
import-diagnostic-nested-too-deep = “{ $path }” no fue leído: su contenido está anidado más de { $limit } niveles de profundidad. Ningún documento real se acerca a eso, y la lectura podría chocar Skribisto, por lo que se deja fuera. Los otros archivos todavía importan.
import-diagnostic-archive-too-large = “{ $path }” no se leía: es un archivo comprimido construido para desempacar mucho más grande de lo que es, que podría agotar la memoria y estrellar Skribisto, por lo que se deja fuera. Los otros archivos todavía importan.
import-diagnostic-lossy-decode = { $count ->
    [one] Un personaje en “{ $path }” no decodificaba. Re-salvar el archivo como UTF-8 para mantenerlo.
   *[other] { $count } caracteres en “{ $path }” no decodificaron. Re-salvar el archivo como UTF-8 para mantenerlos.
}
import-diagnostic-decoded-from-bom = “{ $path }” fue decodificado como { $detail }, no UTF-8.
import-diagnostic-empty-file = “{ $path }” está vacío.
import-diagnostic-no-headings = “{ $path }” no tiene títulos, por lo que llega como un elemento.
import-diagnostic-unsupported-format = Nada lee “.{ $detail }” archivos, por lo que “{ $path }” fue saltado.
import-diagnostic-front-matter-not-flat = La materia frontal en “{ $path }”: “{ $detail }” no es un valor simple y se saltó.
import-diagnostic-footnotes-degraded = { $count ->
    [one] Una nota de pie de página en “{ $path }” llega como texto llano: las notas de pie no se leen desde Markdown.
   *[other] { $count } notas de pie en “{ $path }” llegan como texto llano: las notas de pie no se leen en Markdown.
}
import-diagnostic-footnote-not-carried = { $count ->
    [one] Una nota de pie de página en “{ $path }” no podía ser presentada.
   *[other] { $count } notas de pie en “{ $path }” no pudieron ser traídas. Los otros fueron.
}
import-diagnostic-raw-html-dropped = { $count ->
    [one] Un bloque de crudo HTML en “{ $path }” fue bajado.
   *[other] { $count } bloques de crudo HTML en “{ $path }” fueron retirados.
}
import-diagnostic-nested-break-dropped = { $count ->
    [one] Una escena se rompe dentro de una cita o lista en “{ $path }” fue bajada. Sólo se mantiene un descanso en su propia línea.
   *[other] { $count } saltos de escena dentro de comillas o listas en “{ $path }” fueron retirados. Sólo se mantiene un descanso en su propia línea.
}
import-diagnostic-image-not-ingested = “{ $path }” se refiere a la imagen “{ $detail }”. La referencia llega como texto; la imagen misma no se copia en.
import-diagnostic-duplicate-title = “{ $title }” aparece { $count } veces. Si usted ha importado estos archivos antes, esto los duplicará.
import-diagnostic-heading-level-jump = “{ $title }” salta del nivel de partida { $from } al { $to }; se coloca un nivel bajo su padre.
import-diagnostic-illegal-combination = “{ $title }” lleva prosa, pero un { $kind } no puede contener ninguna. La importación se mantiene hasta que cambie su tipo o lo deje fuera. Nada se importa en absoluto de otra manera.
import-diagnostic-tracked-changes-flattened = { $path } fue de revisión media: { $count ->
    [one] Se aceptó el cambio rastreado { $count }
   *[other] Se aceptaron { $count } cambios rastreados
}, y las eliminaciones cayeron. Ese es el texto final. Para guardar cualquier de su propia redacción en su lugar, use Compare en el último paso.
import-diagnostic-tracked-changes-flattened-by = { $path } fue de revisión media: { $count ->
    [one] { $count } cambio seguido por { $names } fue aceptado
   *[other] Se aceptaron { $count } cambios rastreados en { $names }
}, y las eliminaciones cayeron. Ese es el texto final. Para guardar cualquier de su propia redacción en su lugar, use Compare en el último paso.
import-diagnostic-text-box-dropped = { $count ->
    [one] { $path } tiene { $count } caja de texto. Su texto se encuentra fuera del flujo del documento, por lo que donde pertenece en un manuscrito no puede ser contestado. No es importado.
   *[other] { $path } contiene { $count } cajas de texto. Su texto se encuentra fuera del flujo del documento, por lo que donde pertenece en un manuscrito no puede ser contestado. Ellos no son importados.
}
import-diagnostic-embedded-object-dropped = { $count ->
    [one] { $path } contiene { $count } objeto incrustado: un gráfico, una ecuación o similar. No hay nada en un manuscrito que pudiera contenerlo.
   *[other] { $path } contiene { $count } objetos incrustados: un gráfico, una ecuación o similar. No hay nada en un manuscrito que pueda contenerlos.
}
import-diagnostic-field-flattened = { $count ->
    [one] { $path } tiene { $count } campo: un número de página, una referencia cruzada, una fecha. Conserva el texto que fue la última muestra y no se actualizará de nuevo.
   *[other] { $path } tiene { $count } campos: un número de página, una referencia cruzada, una fecha. Cada mantiene el texto que fue la última muestra y no se actualizará de nuevo.
}
import-diagnostic-unknown-style-level = { $path } utiliza el estilo “{ $detail }”, que parece un título pero no tiene ningún nivel. Esos párrafos se importan como prosa en lugar de adivinar a fondo.
import-diagnostic-comment-unanchored = El comentario “{ $detail }” en { $path } no se puede adjuntar a las palabras que se trataba. Se mantiene en su artículo, donde se puede mover.
import-diagnostic-comment-not-carried = El comentario “{ $detail }” en { $path } no es importado, porque nada en el archivo llega como texto que podría ser guardado. Copiarlo del archivo si lo necesita.
import-diagnostic-comment-replies-flattened = { $count ->
    [one] { $count } respuesta en { $path } nombre un comentario que no está en el archivo, por lo que llega como un comentario propio.
   *[other] { $count } respuestas en { $path } nombraron un comentario que no está en el archivo, por lo que llegan como comentarios propios.
}
import-diagnostic-prose-not-verbatim = { $count ->
    [one] { $count } párrafo en { $path } no se pudo llevar a cabo con su formato exactamente como escrito, por lo que llega como texto llano. Compare con el original.
   *[other] { $count } párrafos en { $path } no pudieron ser presentados con su formato exactamente como escrito, por lo que llegan como texto simple. Compare con el original.
}
import-diagnostic-styled-spaces-not-carried = { $count ->
    [one] En { $path }, { $count } se subrayó o golpeó el espacio en blanco, una línea izquierda para rellenar por ejemplo. Skribisto no mantiene ninguna línea en los espacios solos, por lo que llega como espacios simples, o no en absoluto donde se inicia o termina un párrafo.
   *[other] En { $path }, { $count } tramos de espacio en blanco fueron subrayados o golpeados a través de líneas dejadas para rellenar por ejemplo. Skribisto no mantiene ninguna línea en los espacios solos, por lo que llegan como espacios planos, o no en absoluto donde comienzan o terminan un párrafo.
}
import-diagnostic-list-nesting-flattened = { $count ->
    [one] { $count } elemento de lista en { $path } fue anidado más de { $limit } niveles de profundidad. Llega al nivel { $limit }, el más profundo Skribisto guarda, con sus palabras y formato.
   *[other] { $count } artículos de lista en { $path } fueron anidados más de { $limit } niveles de profundidad. Llegan al nivel { $limit }, el más profundo Skribisto guarda, con sus palabras y formato.
}
import-diagnostic-table-read-as-paragraphs = { $count ->
    [one] { $count } tabla en { $path } tiene una fila mucho más amplia que el resto, demasiado disipado para establecerse como una cuadrícula. Sus células llegan como párrafos, uno tras otro, con sus palabras y formato.
   *[other] { $count } tablas en { $path } tienen una fila mucho más amplia que el resto, demasiado disipado para establecerse como una cuadrícula. Sus células llegan como párrafos, uno tras otro, con sus palabras y formato.
}
import-diagnostic-spaces-shortened = { $count ->
    [one] { $count } recorrido de espacios en { $path } fue más largo que { $limit } espacios, y llega acortado a { $limit }. Las palabras a su alrededor están todas allí.
   *[other] { $count } pistas de espacios en { $path } fueron más largos que { $limit } espacios, y llegan acortados a { $limit }. Las palabras a su alrededor están todas allí.
}
import-diagnostic-epigraph-not-carried = “{ $title }” está dirigido por un epígrafe, pero un { $kind } no puede contener uno. La cita se mantiene en la parte superior de su texto en su lugar.
import-diagnostic-epigraph-placement-ambiguous = Un epígrafe se encuentra entre “{ $title }” y “{ $below }” y podría ir bien. Se le dio a “{ $title }”, que es donde suele ir un epígrafe.
import-document-diagnostics = { $errors ->
    [0] { $warnings ->
            [one] 1 cosa para saber
           *[other] { $warnings } cosas para saber
        }
   *[other] { $errors ->
            [one] 1 archivo no se puede leer
           *[other] { $errors } archivos no se pueden leer
        }
}
import-document-diagnostics-none = Nada que reportar.

versions-title = Versiones
versions-scope-synopsis = Sinopsis
versions-scope-prose = Texto
versions-loading = Mirando a través de tus respaldos...
versions-empty = No hay versión anterior de esto todavía
versions-error = No podía leer sus copias de seguridad. No se ha perdido nada, pero esta lista puede estar incompleta.
versions-did-not-exist = Aún no existía el { $date }
versions-deleted-after = Suprimido un poco de tiempo después de { $date }
versions-unreadable = { $count ->
    [one] { $count } copias de seguridad no se podía leer
   *[other] { $count } copias de seguridad no se podía leer
}
versions-thinned = { $count ->
    [one] Las versiones más antiguas se reducen a medida que envejecen. La propia historia del proyecto ya ha bajado { $count } estado anterior de este texto.
   *[other] Las versiones más antiguas se reducen a medida que envejecen. La propia historia del proyecto ya ha bajado { $count } estados anteriores de este texto.
}
versions-source-backup = De una copia de seguridad
versions-source-project = De la historia del proyecto
versions-list-caption = Una entrada por cambio, no por respaldo
versions-pick-a-version = Escoge una versión para ver qué cambió
versions-earliest = No hay nada más viejo para compararlo.
versions-no-change = Nada cambió en esta parte del artículo
versions-formatting-only = Las palabras son las mismas.
versions-show-unchanged = Mostrar sin cambios
versions-hide-unchanged = Ocultar sin cambios
versions-next-change = Siguiente cambio
versions-near = cerca de "{ $text }"
versions-words-added = { $count ->
    [one] { $count } palabras agregadas
   *[other] { $count } palabras agregadas
}
versions-words-removed = { $count ->
    [one] Se elimina la palabra { $count }
   *[other] { $count } palabras eliminadas
}
versions-blocks-moved = { $count ->
    [one] Movido del párrafo { $count }
   *[other] Movido de los párrafos { $count }
}
versions-pin = Pin la copia de seguridad de esta versión vino de. Limpieza automática nunca borrará
versions-unpin = Unpin la copia de seguridad de esta versión vino de. Limpieza automática puede borrarla de nuevo
versions-pinned-only = Mostrar sólo versiones grabadas
versions-pin-note = Sólo se pueden grabar versiones de una copia de seguridad. Un pin guarda un archivo, y la propia historia del proyecto no es una.
versions-pinned-empty = Nada aquí está grabado aún.
versions-pinned-empty-log = Nada aquí está grabado todavía. Sólo se pueden conectar versiones de una copia de seguridad. Un pin guarda un archivo, y la propia historia del proyecto no es una.
versions-range-filter = Mostrar sólo versiones entre dos fechas
versions-filtered-empty = Ninguna versión coincide con los filtros que has fijado
versions-clear-filters = Limpiar los filtros
versions-last-30-days = Últimos 30 días
versions-restore-button = Restaurar esta versión
versions-restore-confirm-title = Reemplazar este texto con la versión de { $date }?
versions-restore-confirm-text = Lo que ahora tiene será reemplazado por el texto que este artículo tuvo el { $date }.
versions-restore-confirm-with-comments = Lo que ahora tiene será reemplazado por el texto que este artículo tuvo el { $date }. { $count ->
    [one] El comentario { $count } está anclado en el texto actual y puede dejarse huérfano.
   *[other] { $count } comentarios están anclados en el texto actual y pueden dejarse huérfanos.
}
versions-restore-confirm-undo-note = Una copia de seguridad se hace primero, y Ctrl+Z lo deshace en un paso.
versions-restored-toast = Restaurado la versión de { $date }
versions-undo = Deshacer
versions-restore-row-gone = Ese tema ya no está en este proyecto.
versions-restore-no-home = Este artículo ha cambiado de tipo desde entonces, y el viejo texto no tiene donde ir en él
versions-restore-no-safety-copy = Tu respaldo de seguridad no funcionó, así que nada cambió
versions-restore-failed = La restauración falló: { $error }
versions-restore-backup-busy = Una copia de seguridad ya está funcionando. Prueba de nuevo en un momento
versions-restore-in-backup-file = Estás mirando un archivo de copia de seguridad; abre el proyecto mismo para restaurarlo
versions-restore-no-project = Ningún proyecto está abierto
versions-recreate-button = Trae esto de vuelta...
versions-recreate-picker-title = ¿Adónde debería ir?
versions-recreate-picker-empty = Este proyecto no tiene carpeta para ponerlo en
versions-recreate-picker-confirm = Tráelo aquí.
versions-recreate-picker-cancel = Cancelar
versions-recreate-untitled = este tema
versions-recreate-confirm-title = ¿Devolver el “{ $item }”?
versions-recreate-confirm-text = Se añadirá a { $destination }, con el texto que tuvo el { $date }.
versions-recreate-confirm-undo-note = Deshacer, en el mensaje que sigue, lo lleva directamente hacia fuera.
versions-recreated-toast = “{ $item }” está de vuelta en su proyecto
versions-recreated-partial-toast = { $count ->
    [one] “{ $item }” ha vuelto, pero uno de sus textos no puede leerse
   *[other] “{ $item }” ha vuelto, pero { $count } de sus textos no pudieron leerse
}
versions-recreate-already-here = Ese artículo ya está en su proyecto.
versions-recreate-no-destination = Escoge un lugar en el carpeta para ponerlo
versions-recreate-unreadable = Esa copia de seguridad no podía ser leída, así que no se añadió nada.
versions-recreate-failed = No pude traerlo de vuelta: { $error }
versions-changed-percent = { $percent }% de esto cambió
versions-hidden-paragraphs = { $count ->
    [one] ... { $count } párrafo sin cambios ...
   *[other] ... { $count } párrafos sin cambios ...
}

timeline-title = Volver al pasado
timeline-coverage = { $count ->
    [one] { $count } versión grabada, volver a { $oldest }
   *[other] { $count } versiones grabadas, volver a { $oldest }
}
timeline-loading = Mirando el pasado de su proyecto…
timeline-empty = Todavía no se ha registrado ninguna versión de este proyecto
timeline-no-changes = Nada ha cambiado desde entonces
timeline-no-text-changes = Ningún texto ha cambiado desde entonces
timeline-slider-label = Versión grabada
timeline-series-name = Tamaño del proyecto
timeline-bars-caption = Cada barra es una versión grabada, tan alta como el proyecto era entonces. El resaltado es lo que estás mirando.
timeline-bars-caption-periods = Demasiados versiones para mostrar uno por uno, por lo que cada bar es un { $period }, tan alto como el proyecto era al final de él.
timeline-unit-hour = hora
timeline-unit-day = día
timeline-unit-week = semana
timeline-unit-month = mes
timeline-range-filter = Mostrar sólo versiones entre dos fechas
timeline-range-empty = No se registró ninguna versión en esas fechas
timeline-open-period = Abierto este período
timeline-show-all = Mostrar toda la historia
timeline-changed-since = { $count ->
    [one] { $count } artículo difiere entre { $date } y su proyecto ahora
   *[other] { $count } artículos difieren entre { $date } y su proyecto ahora
}
timeline-kind-added = Escrito desde
timeline-kind-removed = Ya no en el proyecto
timeline-kind-changed = Editado desde entonces
timeline-kind-moved = Muévete desde
timeline-not-yet-written = Esto aún no existía en ese momento.
timeline-no-text-of-its-own = Esto no tiene ningún texto propio.
timeline-reader-close = Cerrar
timeline-reader-stamp = Como era el { $date }
timeline-reader-loading = Apertura de la versión grabada…
timeline-reader-unreadable = Esta grabación no se podía leer. La copia de seguridad puede haber sido movida, eliminada, o estar en una unidad que no está conectada.
timeline-reader-compared = Como era en { $date }, contra lo que dice ahora
timeline-reader-diff-legend = La lucha ha pasado desde entonces; se ha añadido subrayado.
timeline-reader-view-label = Qué mostrar
timeline-reader-view-diff = Cambios
timeline-reader-view-text = Texto
timeline-reader-deleted = Esto ya no está en tu proyecto, puedes leerlo y copiarlo aquí.
timeline-prose-only-record = Este punto viene de la propia historia del proyecto, que guarda texto y nada más, así que lo que fue borrado o movido ya que no se puede mostrar. Escoge una copia de seguridad para eso.

settings-sec-extensions = Extensiones

settings-group-games-forward = Siempre adelante.
settings-games-forward-toggle = Juega "Siempre adelante"
settings-games-forward-hint =
    Mientras juegas, nada que hayas escrito se puede recuperar: Backspace, Eliminar,
    Cortar, arrastrar y soltar y Deshacer están todos deshabilitados en las superficies que elija a continuación.
    Usted todavía puede escribir, pegar, formatear y moverse libremente. El borrador sólo crece.
settings-games-session-warning =
    Esta es una opción por sesión: nunca se salva. Cierre del proyecto, o dejar de fumar
    Skribisto, siempre termina el juego, y todo lo que escribió mientras juega se convierte en
    indomable de nuevo en el momento en que lo hace.
settings-group-games-scope = Cuando se aplica
settings-games-in-prose = Manuscrito prose
settings-games-in-synopsis = Sinopsis
settings-games-scope-hint =
    Comentarios, notas de pie y títulos nunca se congelan: son donde se nota la solución
    Acabas de prohibirte que lo hagas.
settings-games-inert-warning =
    "Siempre adelante" está encendida pero se aplica a nada. Ataque al menos una superficie arriba,
    o no cambia nada como escribes.

games-title = Escribir juegos
games-forward-name = Siempre adelante.
games-forward-blurb =
    Borrar sin tomar nada. Eliminar, cortar y deshacerse son deshabilitados
    Escribe la siguiente frase en vez de arreglar la última.
games-forward-playing = Jugar: eliminar es deshabilitado
games-forward-idle = No juegas.
games-session-note = Termina cuando cierras el proyecto.
games-scope-prose-and-synopsis = Se aplica a su prosa y sinopsis.
games-scope-prose = Se aplica a su prosa.
games-scope-synopsis = Se aplica a sus sinopses.
games-scope-nothing = No se aplica todavía a nada. Elige una superficie en Ajustes.
games-settings-link = Escribir la configuración del juego…
statusbar-games-forward = Siempre adelante.
statusbar-games-forward-tooltip =
    "Siempre adelante" está en: borrar, cortar y deshacerse están deshabilitados mientras usted
    borrador. Haga clic para dejar de jugar.
goal-progress-words = { $g ->
    [one] { $count } de { $goal } palabra
   *[other] { $count } de { $goal } palabras
}
goal-progress-characters = { $g ->
    [one] { $count } de { $goal } caracteres
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
goal-unit-characters = Personajes

inspector-goal = Objetivo
inspector-goal-none = Sin objetivo

overview-col-goal = Objetivo
overview-excluded-from-export = Salió de la exportación, por lo que no cuenta hacia un total

goal-subtree-total-words = { $count ->
    [one] { $items } objetivo dentro añade hasta { $words } palabras
   *[other] { $items } objetivos dentro de añadir hasta { $words } palabras
}
goal-subtree-total-characters = { $count ->
    [one] { $items } objetivo dentro añade hasta { $words } caracteres
   *[other] { $items } objetivos dentro de añadir hasta { $words } caracteres
}

new-work-goal-unit = Cuenta en
new-work-goal-unit-hint = Se utiliza para cada palabra o objetivo de carácter en este proyecto.

settings-group-goal-unit = Metas
settings-goal-unit-switch-title = Unidad de conteo de interruptores
settings-goal-unit-switch-text = Los objetivos y los hitos ya establecidos para este proyecto fueron introducidos en { $from }. El cambio a { $to } no los convierte, por lo que cada número existente se leerá ahora como { $to }.
settings-goal-unit-switch-informative = Nada se pierde. Cambia en cualquier momento para restaurar la lectura original, y luego actualiza los objetivos que quieres mantener.
settings-goal-unit-switch-confirm = Cambio

goal-distribute-action = Distribución…
distribute-title = Distribuir el objetivo
distribute-empty = No hay nada dentro de esto para compartir el objetivo.
distribute-weight = Compartir fuera
distribute-weight-length = Por longitud
distribute-weight-rows = Por piezas
distribute-weight-even = Por igual
distribute-overwrite = Sustitúyase objetivos que ya están fijados
distribute-col-item = Tema
distribute-col-current = Ahora.
distribute-col-proposed = Después
distribute-total = Estas sumas ascienden a { $total }, frente a un objetivo de { $goal }.
distribute-over-budget = Los objetivos ya establecidos dentro vienen a { $over } más que el propio contenedor. Aumentar su objetivo, bajar el suyo, o reemplazarlos.
distribute-apply = Distribuir
distribute-cancel = Cancelar

milestone-target-gone = Objetivo eliminado
milestone-no-target = Sin objetivo
milestone-add = Añadir
milestone-add-label = Qué debe alcanzarse

pace-summary-title = Donde está el libro
pace-summary-remaining = { $words } para ir
pace-summary-open = Abra el plan
pace-summary-close = Cerrar
pace-summary-dont-show = No mostrar esto cuando se abre
pace-summary-menu = Escribir plan…

menu-help-topics = &Ayuda Temas
menu-help-shortcuts = A&tajos de teclado…
menu-help-website = &Skribisto en la Web
menu-help-report = &Informe un problema…
menu-help-check-updates = &Buscar actualizaciones
menu-command-palette = &Mando Palette…

learn-help-topics = Temas de ayuda
learn-shortcuts = Atajos de teclado
learn-website = Skribisto en la web

help-window-title = Ayuda
help-filter-topics = Temas filtrantes
help-no-matching-topic = No hay coincidencias de tema.
help-topic-missing = Este tema ya no está disponible.
help-back = Atrás
help-not-translated = Esta página aún no se ha traducido, por lo que se muestra en inglés.

help-section-getting-started = Comienzo
help-section-writing = Escribir
help-section-reviewing = Examen
help-section-exchanging = Consiguiendo palabras dentro y fuera
help-section-keeping = Mantener su trabajo seguro
help-section-extensions = Extensiones

help-topic-getting-started = Su primer proyecto
help-topic-writing-model = Cómo se une un libro
help-topic-drafts-and-old-versions = Mantener un borrador antiguo
help-topic-goals-and-pace = Metas y ritmo
help-topic-comments = Comentarios
help-topic-round-trip = Enviar su libro a un lector
help-topic-export = Exportación
help-topic-import-documents = Importación de documentos
help-topic-import-projects = Traer un proyecto entero
help-topic-backups-and-versions = Respaldos y versiones

help-shortcuts-title = Atajos de teclado
help-shortcuts-filter = Atajos de filtro
help-shortcuts-no-matches = No hay coincidencias de atajo.
help-shortcuts-rebind = Cambios atajos…
help-shortcuts-close = Cerrar

command-palette-placeholder = Introduzca un comando

shortcut-name-binder-duplicate = Duplicar
shortcut-name-comments-add = Agregar comentario
shortcut-name-comments-add-paragraph = Observación sobre el párrafo
shortcut-name-spellcheck-toggle = Revisar ortografía
shortcut-name-editor-tab-close = Cerrar Tab
shortcut-name-editor-tab-pin = Fijar o desfijar pestaña
shortcut-name-editor-save = Guardar
shortcut-name-work-export = Exportar...
shortcut-name-work-new = Nuevo proyecto
shortcut-name-work-open = Proyecto abierto
shortcut-name-window-new = Nueva ventana
shortcut-name-work-close = Cerrar proyecto
shortcut-name-app-settings = Configuración
shortcut-name-app-quit = Salir
shortcut-name-editor-insert-footnote = Insertar nota de pie de página
shortcut-name-format-scene-break = Introducir la ruptura de escena
shortcut-name-format-major-scene-break = Inserte la ruptura de escenas principales
shortcut-name-format-link = Insertar enlace
shortcut-name-format-mark-nbsp = Insertar espacio sin romper
shortcut-name-format-mark-nbhyphen = Inserte No-break Hyphen
shortcut-name-format-mark-nnbsp = Inserte el espacio sin romper estrecho
shortcut-name-format-mark-zwsp = Insertar espacio de ancho cero
shortcut-name-go-next = Siguiente
shortcut-name-go-prev = Anterior
shortcut-name-go-to = Ve.
shortcut-name-outline-toggle = Mostrar u ocultar estructura
shortcut-name-preview-toggle = Mostrar u ocultar vista previa
shortcut-name-view-fullscreen = Alternar pantalla completa
shortcut-name-view-focus-mode = Modo libre de Distracción
shortcut-name-editor-size-increase = Aumentar el tamaño del texto
shortcut-name-editor-size-decrease = Disminuir el tamaño del texto
shortcut-name-editor-size-reset = Reiniciar el tamaño del texto
shortcut-name-editor-find = Encontrar
shortcut-name-editor-replace = Reemplazar
shortcut-name-editor-find-next = Siguiente partido
shortcut-name-editor-find-prev = Previa coincidencia
shortcut-name-search-show = Buscar en el proyecto
shortcut-name-search-replace = Reemplazamiento en el proyecto
shortcut-name-outline-open-to-side = Abierto al lado
shortcut-name-help-topics = Ayuda Temas
shortcut-name-help-shortcuts = Atajos de teclado
shortcut-name-help-website = Skribisto en la Web
shortcut-name-help-report = Reportar un problema
shortcut-name-help-check-updates = Buscar actualizaciones
update-check-behind = Versión { $version } está disponible.
update-check-current = Estás ejecutando la versión más reciente.
update-check-failed = No se pudo comprobar una versión más reciente. { $error }
shortcut-name-command-palette = Mando Palette
help-section-reference = ¿Qué cosas son

help-concept-scene-break = Escena de descanso
help-concept-major-scene-break = Un gran descanso de la escena
help-concept-find-in-prose = Buscar en el texto
help-concept-story-bible = Biblia narrativa
help-concept-goal-unit = Palabras o caracteres
help-concept-goal-progress = Progresos
help-concept-manuscript-words = Lo que cuenta como el manuscrito
help-concept-exportable = Izquierda de la exportación
help-concept-distribute = Compartir un objetivo
help-concept-subtree-total = Lo que los objetivos en su interior agregan hasta
help-concept-milestone = Hito
help-concept-pace-plan = Plan de ritmo

help-concept-tag = Etiqueta
help-concept-label = Anotación
help-concept-point-of-view = Punto de vista
help-concept-epigraph = Epígrafe
help-concept-footnote = Nota de pie
help-concept-chapter-mode = Forma del capítulo
help-concept-comment = Comentario
help-concept-backup = Copia de seguridad
help-concept-version = Versión
help-concept-trash = Papelera
help-concept-spellcheck = Comprobación de gastos
help-concept-search-replace = Búsqueda y sustitución
help-concept-note-template = Plantilla de notas
help-concept-story-bible-entry = Entrada bíblica
help-concept-text-replacement = Sustitución de texto
help-concept-smart-punctuation = Puntualización inteligente
help-concept-export-style = Estilo de exportación
help-concept-round-trip-marks = Marcadores de ida y vuelta

epigraph-mark-attribution = Línea de fuentes
epigraph-mark-attribution-tip = Marcar la línea que el cuidado está dentro como la fuente de la cita, por lo que se imprime como una atribución
margin-lane-name = Marcas marginales
margin-lane-provider-comments = Comentarios
margin-lane-provider-story-bible = Entrada bíblica
margin-lane-provider-story-bible-hint = En una nota de lectura de prosa, donde se llama esa entrada, y las escenas se cuentan a través de sus ojos. También marca esos nombres en la prosa misma.
margin-lane-mark-point-of-view = Se lo dije desde aquí
margin-lane-mark-named-here = Nombre aquí
margin-lane-provider-comments-hint = Cuando se adjunta una nota al texto
margin-lane-provider-search = Resultados de la búsqueda
margin-lane-provider-search-hint = Cada golpe por lo que buscó por última vez, donde sea que esté en el documento
margin-lane-provider-boundaries = Donde cada documento comienza
margin-lane-provider-boundaries-hint = Una regla en la parte superior de cada escena en un arroyo, para que puedas ver dónde estás
margin-lane-provider-spelling = Ortografía
margin-lane-provider-spelling-hint = Donde el corrector de hechizos ha marcado una palabra. A menos que usted lo pida: marca la opinión de una máquina de su prosa
margin-lane-spelling = { $word }, posible error
margin-lane-search-hit = { $text }, partido { $index } de { $total }
margin-lane-search-current = { $text }, partido { $index } de { $total }, el que usted está en
margin-lane-boundary = Inicio de { $title }
margin-lane-boundary-untitled = un documento sin título
settings-page-margin-lane = Marcas marginales
settings-desc-margin-lane = La tira al lado de la barra de desplazamiento, y lo que muestra
settings-margin-lane-enabled = Mostrar el carril del margen
settings-margin-lane-enabled-hint = El mismo interruptor que las marcas Vista ▸ Margin.
settings-margin-lane-enabled-more = Nunca dice que nada está mal: te muestra dónde están las cosas, y te deja qué hacer de eso.
settings-group-margin-lane-marks = Lo que marca
settings-margin-lane-no-providers = Nada marca el carril todavía.
settings-group-margin-lane-texture = Textura del diálogo
settings-margin-lane-texture = Mostrar la textura del diálogo
settings-margin-lane-texture-hint = Una barra por párrafo: cuánto tiempo es, y cuánto de ella se habla.
settings-margin-lane-texture-more = No se mide para idiomas sin convención curada, donde la barra muestra la longitud sola.
settings-group-margin-lane-surfaces = Donde aparece
settings-margin-lane-surface-editor = Editor de texto
settings-margin-lane-surface-stream = Corrientes
settings-margin-lane-surface-search-preview = Buscar vista previa
menu-margin-lane = &Marcas marginales

status-preset-drafting = Redacción
status-preset-passes = Pases de revisión
status-preset-plume = Plume Creator
status-todo = Para hacer
status-draft = Proyecto
status-revised = Revisado
status-final = Final
status-outline = Estructura
status-first-edit = 1a edición
status-second-edit = 2a edición
status-done = Listo
status-plume-draft-1 = Primer proyecto
status-plume-draft-2 = Segundo proyecto
status-plume-draft-3 = Tercer proyecto
status-plume-edit-1 = 1a edición
status-plume-edit-2 = 2a edición
status-plume-edit-3 = Tercera revisión
status-plume-proofread = Corrección final
status-plume-finished = Terminado
new-work-statuses = Flujo de trabajo
new-work-statuses-hint = Las etapas que mueves una escena a través. Puedes renombrar, reordenar o añadir a éstas en cualquier momento.
status-none = No estatus
inspector-status = Situación
overview-col-status = Situación
overview-status-mixed = las partes que figuran a continuación no están de acuerdo
help-concept-status = Situación
status-completion-title = Donde está el libro
status-completion-headline = { $done } de { $total } escenas terminadas
status-completion-empty = Esto lee el manuscrito, así que se llena como escribes.
status-completion-open = Donde está el libro…

settings-page-statuses = Situación
settings-desc-statuses = Renombrar, reordenar, añadir y eliminar las etapas que mueve una escena a través de
settings-statuses-add = Añada el estado
settings-statuses-add-placeholder = Nuevo nombre de estatus
settings-statuses-desc =
    Las etapas de una escena pasan, en orden, de menos acabado en la parte superior a la mayoría
    terminada en el fondo. Ese orden es lo que "menos acabados que" significa en todas partes
    en la aplicación, así que ponlos en la forma en que tu proceso realmente funciona.
settings-statuses-apply-preset = Aplique un preset...
settings-statuses-preset-applied = { $added ->
    [one] 1 estado añadido
   *[other] { $added } estados agregados
}
settings-statuses-preset-refused = Este proyecto ya tiene una escalera. Los presets sólo llenan una vacía. Eliminar los peldaños que no quieres primero.
settings-statuses-duplicate = "{ $name }" ya está en esta escalera
settings-statuses-added = Añadido "{ $name }"
settings-statuses-move-up = Muévete (sin terminar)
settings-statuses-move-down = Muévete (más terminado)
settings-statuses-details-placeholder = Lo que esta etapa significa (opcional)
settings-statuses-delete = Eliminar "{ $name }"
settings-statuses-delete-in-use = { $count ->
    [one] Eliminar "{ $name }": 1 artículo está en él y perderá su estatus
   *[other] Eliminar "{ $name }": { $count } artículos están en él y perderán su estatus
}
settings-statuses-deleted = Eliminado "{ $name }"
settings-statuses-deleted-in-use = { $count ->
    [one] Eliminado "{ $name }". 1 artículo ya no tiene un estatus.
   *[other] Eliminado "{ $name }". { $count } artículos ya no tienen un estatus.
}
settings-statuses-empty-title = No hay flujo de trabajo todavía
settings-statuses-empty-body = Un estado dice lo lejos que es una escena. Añadir uno arriba, o empezar desde un preset.
status-category-planned = Planificación prevista
status-category-drafting = Redacción
status-category-needs-work = Las necesidades de trabajo
status-category-revised = Revisado
status-category-final = Final
