# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

wm-book = Un libro en su proyecto.
wm-book-more = Un libro es tan sencillo o tan estructurado como usted quiera. Un relato corto puede ser solo unas pocas [escenas](:wm-scene). Una obra más larga puede anidar [partes](:wm-part) y [capítulos](:wm-chapter) con tanta profundidad como desee. La forma la decide usted.
wm-part = Una parte que agrupa capítulos.
wm-part-more = Una parte reúne [capítulos](:wm-chapter) bajo un mismo encabezado. Como un capítulo, se extiende hasta la parte siguiente o hasta el final del [libro](:wm-book).
wm-chapter = Un capítulo del libro.
wm-chapter-more = Un capítulo va desde aquí hasta el capítulo siguiente, la [parte](:wm-part) siguiente o el [fin del libro](:wm-end-of-book). Tiene dos formas: una carpeta de capítulo que contiene [escenas](:wm-scene), o un capítulo plano que lleva su propia prosa. Elija cuál usan los capítulos nuevos en Configuración, en Estructura.
wm-scene = Una escena de prosa.
wm-scene-more = Una escena contiene lo que escribe. Cada elemento de escritura tiene dos caras: el texto principal y una [sinopsis](:wm-synopsis) para planificar. Ambos se abren juntos en el editor.
wm-note = Una nota.
wm-note-more = Una nota guarda texto y una [sinopsis](:wm-synopsis), como una [escena](:wm-scene), para documentación, apartes o recordatorios. Cualquier elemento puede formar parte del libro compilado, incluida una nota situada dentro de un [capítulo](:wm-chapter).
wm-note-folder = Una nota que contiene otros elementos.
wm-note-folder-more = Una [nota](:wm-note) que además contiene elementos debajo, para que pueda agrupar notas relacionadas. Conserva su propia [sinopsis](:wm-synopsis), y los elementos de dentro conservan su propio texto.
wm-folder = Una carpeta de organización.
wm-folder-more = Un contenedor para ordenar los elementos del archivador como más le convenga. Lleva una [sinopsis](:wm-synopsis) pero no tiene prosa propia. Los elementos de dentro contienen lo escrito.
wm-end-of-book = Marca dónde termina un libro.
wm-end-of-book-more = Todos los libros comparten una única lista continua, así que el final de un libro no lo decide el anidamiento. Este marcador indica que el [libro](:wm-book) termina aquí. Todo lo que va detrás pertenece al libro siguiente.
wm-story-bible-entry = Una nota con etiquetas y otros nombres sobre su historia.
wm-story-bible-entry-more = Una [nota](:wm-note) sobre un personaje, un lugar o cualquier otra cosa que merezca un registro, creada con un nombre, etiquetas, otros nombres y una plantilla inicial en un solo paso. Después no hay nada en la página que la distinga, ya que sigue siendo una nota normal que puede renombrar, reetiquetar o convertir como cualquier otra.
wm-synopsis = El resumen de un elemento de escritura.
wm-synopsis-more = Un resumen vinculado a cualquier elemento de escritura, normalmente un párrafo breve, aunque puede ser tan largo como quiera. Úselo para planificar y orientarse antes de que exista la prosa. Está junto al texto principal en el editor.

scene-break-minor = Un separador de escena normal: un cambio de tiempo, de lugar o de punto de vista dentro de un capítulo.
scene-break-minor-more =
    Marca un corte *donde usted lo pone*, incluso en medio de una [escena](:wm-scene). Dividir la prosa en dos elementos es una decisión organizativa, así que nunca crea un corte por sí sola.

    Se escribe en la prosa como tres asteriscos separados por espacios. Cómo se imprime lo decide el estilo de exportación, no lo que usted escribe: un manuscrito Shunn lo pone como un solo #, una edición comercial en rústica como un dinkus, y la mayor parte de la edición francesa, alemana, española, rusa e italiana como un simple espacio en blanco sin ninguna marca.
scene-break-major = Una división más marcada: un gran salto en el tiempo o un cambio decisivo de punto de vista.
scene-break-major-more =
    La misma idea que un separador de escena normal, un escalón por encima. Úselo cuando un corte simple se quedaría corto para el salto.

    Se escribe como # # # y el estilo de exportación lo imprime de forma distinta al nivel normal. El formato de manuscrito estándar de Shunn pone un solo # frente a # # # precisamente para esta distinción. Donde una tradición no tiene una marca más fuerte, ambos niveles se imprimen igual.
wm-find-in-prose = Skribisto busca este elemento en su prosa.
wm-find-in-prose-more = Active esta opción para las etiquetas que nombran cosas sobre las que escribe: personajes, lugares, objetos. Todo elemento que lleve una etiqueta así se busca en su prosa por su título y por los otros nombres que le dé, de modo que cada [escena](:wm-scene) indica quién y qué aparece en ella sin que tenga que vincular nada a mano. Déjela desactivada para las etiquetas que describen un elemento en lugar de nombrarlo, como un estado de borrador o un recordatorio para revisar la continuidad. Todo elemento que coincida se incorpora también a su [biblia narrativa](:wm-story-bible).
wm-story-bible = Todos los personajes, lugares y demás entradas etiquetadas, reunidos en una cuadrícula.
wm-story-bible-more = Se abre desde la pestaña propia de cualquier carpeta de notas. Las tarjetas se agrupan por etiqueta, y cada una indica a cuántos otros nombres responde una entrada y en cuántas escenas ha aparecido, a partir de todas las etiquetas [buscar en la prosa](:wm-find-in-prose) del proyecto. Filtrar por libro solo limita qué tarjetas se muestran; nunca decide qué entradas existen.

tooltip-go-to = Saltar a cualquier elemento del archivador (Ctrl+G)
synopsis-collapse-tooltip = Ocultar la columna de sinopsis

wm-paratext = Un texto que no es parte de la historia.
wm-paratext-more = Un prefacio, una dedicatoria, un epílogo, un colofón: textos que pertenecen al libro pero no a su cuerpo. Se exporta allí donde lo ponga y nunca cuenta en el recuento de palabras del manuscrito. Dónde va lo decide usted: las convenciones varían según el país y la editorial.
wm-paratext-folder = Una carpeta para paratextos.
wm-paratext-folder-more = Un lugar para guardar prefacios y epílogos sin que llenen el archivador. Es solo organizativa: lleva una [sinopsis](:wm-synopsis) pero no añade nada al libro exportado, ni siquiera su propio nombre.

goal-target = La extensión que debe tener esta pieza.
goal-target-more = Defina un objetivo en cualquier cosa: una [escena](:wm-scene), un [capítulo](:wm-chapter), una [parte](:wm-part), el [libro](:wm-book) entero. Cada uno es su propio número y es independiente, así que definir un objetivo en un capítulo nunca cambia el de su libro, y el objetivo de un libro nunca es la suma de los capítulos que contiene. Lo que sí se suma es lo escrito: el [progreso](:goal-progress) de un contenedor es todo lo escrito debajo de él. Deje un objetivo a cero y simplemente no estará definido. El número se cuenta en [palabras o caracteres](:goal-unit), según lo que use este proyecto.
goal-unit = Si este proyecto cuenta en palabras o en caracteres.
goal-unit-more = Es una elección para todo el proyecto, porque una unidad de longitud pertenece al manuscrito y a su mercado, no a una sección: la edición alemana y la francesa miden en caracteres, la japonesa en hojas de 400 caracteres, la china por millares de caracteres, y la mayor parte de la edición en inglés en palabras. Ambos números se guardan uno junto a otro y ninguno se convierte nunca en el otro, así que cambiar la unidad hace que cada [objetivo](:goal-target) apunte a la otra cifra, y volver a cambiarla restaura la primera. Lo que no hace es convertir nada, así que los objetivos ya introducidos tendrán que actualizarse a mano.
goal-progress = Cuánto hay escrito, respecto al objetivo.
goal-progress-explained = Una pieza con prosa propia se mide por su propia extensión; un contenedor, por [todo lo escrito debajo de él](:goal-manuscript-words). La barra pasa del rojo al ámbar y al verde a medida que avanza la escritura, y vuelve a cambiar cuando la pieza ha superado claramente lo previsto, algo que conviene notar cuando un capítulo tiene una extensión que respetar.
goal-manuscript-words = Lo que cuenta como el manuscrito.
goal-manuscript-words-more = Solo la prosa que se exportaría realmente: las [escenas](:wm-scene) y los [capítulos](:wm-chapter) que llevan su propio texto. Una [nota](:wm-note) o un [paratexto](:wm-paratext) no forman parte del manuscrito y nunca cuentan para él. Todo lo que está en la papelera queda fuera, igual que todo lo [excluido de la exportación](:goal-exportable).
goal-exportable = Este elemento queda fuera de la exportación.
goal-exportable-more = Su propia extensión se sigue mostrando, porque lo escrito sigue ahí, pero no cuenta para ningún total, exactamente igual que no aparecerá en ningún libro exportado. Dejar fuera un contenedor no deja fuera las piezas que contiene: cada elemento responde por sí mismo, y para eso sirve el botón «Aplicar a los hijos» junto al interruptor.
goal-distribute = Repartir este objetivo entre lo que hay dentro.
goal-distribute-more = Divide el objetivo de un [libro](:wm-book) o de una [parte](:wm-part) entre sus hijos directos para que las piezas sumen exactamente esa cifra. Por defecto solo rellena las que aún no tienen objetivo y no toca sus propios números. Lo que escribe son [objetivos](:goal-target) normales, que puede editar después y que pueden divergir libremente: es una acción puntual, no un acuerdo permanente.
goal-subtree-total = Lo que suman los objetivos interiores.
goal-subtree-total-more = Un dato, no un objetivo. Este proyecto nunca trata el objetivo de un contenedor como la suma de los que contiene, porque así es como la cifra de una carpeta acaba cambiando sola cuando se da un número a una escena de dentro. Esta línea simplemente le dice cuánto suman sus propios números, para que pueda compararlo usted mismo con el [objetivo](:goal-target) del contenedor, o [repartir uno](:goal-distribute).
goal-milestone = Una fecha para alcanzar algo.
goal-milestone-more = Hay dos tipos. Uno indica que el [libro](:wm-book) debe tener una extensión determinada en una fecha, un punto de paso en su propia curva. El otro indica que un [capítulo](:wm-chapter) o una [parte](:wm-part) concretos deben haber alcanzado [su propio objetivo](:goal-target) para entonces, y lee ese número en directo en lugar de guardar una copia, así que al editar el objetivo también se actualiza el hito. Ambos aparecen en el [plan de ritmo](:pace-plan) del libro.
pace-plan = El calendario de escritura del libro.
pace-plan-more = Un [objetivo](:goal-target) y una fecha límite, junto con los días en que escribe, convertidos en lo que hace falta cada día. Lee el propio objetivo del libro, así que definirlo aquí y definirlo en el inspector es el mismo número en dos lugares. Los puntos de paso intermedios son los [hitos](:goal-milestone).

concept-tag = Una etiqueta de color que puede poner en cualquier elemento, reutilizable en todo el proyecto.
concept-tag-more = Etiquete un elemento para marcarlo: un estado, un lugar, un hilo que esté siguiendo. La misma etiqueta puede estar en cualquier número de elementos, y un elemento puede llevar varias. Active [buscar en la prosa](:wm-find-in-prose) para una etiqueta que nombra algo de su libro, un personaje o un lugar, y Skribisto empezará a buscarlo en su prosa.
concept-label = Una nota breve que escribe bajo el título de un elemento, para usted.
concept-label-more = No es una [etiqueta](:concept-tag): un rótulo pertenece a un solo elemento, es texto simple sin color ni catálogo detrás, algo como «1.er punto de giro» o «necesita un nombre». Defínalo desde el menú contextual del elemento, o edítelo directamente en la columna Rótulo de la vista general; aparece como un pequeño subtítulo bajo el título, tanto en el esquema como en el flujo.
concept-point-of-view = A través de qué ojos se narra una escena: uno o varios miembros del reparto de su historia.
concept-point-of-view-more = Se define en el inspector, junto al reparto. Si elige a alguien que aún no está en el reparto, también se añade allí. Más de un punto de vista es una opción válida, útil para una escena compartida o un cambio de mirada a mitad del libro, y Skribisto no lo impide. Los candidatos proceden de los elementos que una etiqueta [buscar en la prosa](:wm-find-in-prose) incorpora a su biblia narrativa.
concept-epigraph = Una cita situada al principio de una parte o de un capítulo.
concept-epigraph-more = Es un campo propio, separado de la prosa del [capítulo](:wm-chapter) o de la [parte](:wm-part); una [escena](:wm-scene) o una [nota](:wm-note) no lo tienen. Escriba uno como una cita normal; escriba varios y cada uno se convierte en su propio bloque de cita, impreso como su propio epígrafe. Sus palabras nunca cuentan como palabras del manuscrito.
concept-footnote = Una referencia marcada en la prosa que se imprime como nota numerada.
concept-footnote-more = La llamada es una sola marca dentro de sus palabras, así que se mueve y se borra exactamente igual que las palabras que la rodean. Su número nunca se guarda: Skribisto lo calcula de nuevo a partir de dónde está cada llamada en todo el manuscrito, así que insertar una renumera todas las notas siguientes. Las palabras de una nota al pie se cuentan aparte del manuscrito y nunca se suman a él.
concept-chapter-mode = Si un capítulo nuevo es una carpeta de escenas o un único elemento plano.
concept-chapter-mode-more = Se define en Configuración, en Estructura. En ambos casos escribe directamente en el [capítulo](:wm-chapter); la única diferencia es si además puede contener escenas debajo. Las dos formas se compilan en el mismo libro, y puede mezclarlas libremente en un proyecto. Convertir en cambia un capítulo existente de una forma a otra sin perder una sola palabra.
concept-comment = Una nota anclada a un fragmento exacto de prosa, mostrada en el margen.
concept-comment-more = Recuerda las palabras a las que apunta y lo que las rodea, no una posición fija, así que sobrevive a la edición normal y a una recarga completa. Responda debajo para continuar un hilo. Si la cita exacta ya no se encuentra, el comentario lo indica en lugar de desplazarse en silencio a la frase equivocada.
concept-backup = Una copia de todo el proyecto en un momento dado, guardada junto a él.
concept-backup-more = Se hace según el calendario que usted elija: al abrir, al cerrar, a intervalos o cualquier combinación, y las copias antiguas se eliminan por antigüedad o por número. Al abrir una, se carga en su propia ventana: puede mirar e incluso editar, pero solo Guardar como o Restaurar conserva algo. Es distinto de una [versión](:concept-version), que sigue el pasado de un solo elemento.
concept-version = El pasado de un elemento: cada cambio en su texto o su sinopsis.
concept-version-more = Se construye a partir del historial de guardado del propio proyecto y de sus copias de seguridad juntos, con una entrada por cambio y no por archivo. Elija una entrada para ver qué cambió y restaure solo el texto de ese elemento a ese estado; el resto del proyecto no se toca, y antes se hace una copia de precaución. Es distinto de una [copia de seguridad](:concept-backup), que es una copia de todo.
concept-trash = Un elemento enviado a la papelera se oculta, no se mueve: se queda exactamente donde estaba.
concept-trash-more = Enviar a la papelera solo cambia un indicador: el elemento conserva su lugar en el archivador, marcado como inactivo en lugar de quitado. Restáurelo y reaparecerá en ese mismo lugar, sin tener que volver a ordenarlo. Vaciar la papelera elimina definitivamente los elementos que contiene, y cada una de estas acciones es un paso que se puede deshacer, como cualquier otra modificación del proyecto.
concept-spellcheck = Subraya las palabras que no reconoce ningún diccionario instalado, según el idioma del proyecto.
concept-spellcheck-more = Cada proyecto tiene su idioma o idiomas de trabajo; un elemento puede sustituirlos para una escena escrita en otra lengua. Añada una palabra que Skribisto no conozca desde el menú contextual del editor, a un diccionario personal que sirve para todos los proyectos. Desactivar la revisión ortográfica la detiene en todas partes, hasta que vuelva a activarla.
concept-search-replace = Encuentra una palabra o frase en todo el proyecto, no solo en el documento abierto.
concept-search-replace-more = Recorre cada escena, nota, título, sinopsis y rótulo, y también los comentarios y las notas al pie. Mayúsculas, palabra completa y acentos son tres interruptores independientes. Reemplazar todo reescribe todas las coincidencias revisadas en una sola pasada y un solo paso de deshacer, así que Ctrl+Z revierte el lote entero, nunca solo el último cambio.
concept-note-template = Un fragmento de texto reutilizable que inserta en cualquier cosa en la que esté trabajando.
concept-note-template-more = Una ficha de personaje en blanco, un perfil de lugar, una escaleta: escríbala una vez y póngala donde la necesite, en cualquier elemento, no solo en una nota. Empiece con un modelo integrado, importe un archivo .md o .djot, o escriba algo y elija Documento ▸ Guardar como plantilla. Se guarda en el proyecto, así que un coautor que lo abra obtiene el mismo conjunto.
concept-text-replacement = Sus propias abreviaturas, expandidas automáticamente mientras escribe.
concept-text-replacement-more = Defina un disparador y su sustitución, «pq» por «porque», y se expande en cuanto escribe un espacio o un signo de puntuación detrás. Las mayúsculas siguen lo que usted escribió: ponga en mayúscula el disparador y la expansión también irá en mayúscula. Un Retroceso justo después de una expansión deshace solo esa sustitución. Se activa por proyecto, y es distinto de la [puntuación inteligente](:concept-smart-punctuation).
concept-smart-punctuation = Tipografía adaptada al idioma aplicada automáticamente mientras escribe: comillas, rayas, puntos suspensivos.
concept-smart-punctuation-more = Convierte las comillas rectas en tipográficas, -- en semirraya y --- en raya, ... en puntos suspensivos y, si lo activa, abre con una raya de diálogo un párrafo que empieza por «- ». Un párrafo que empieza por "- ", "* " o "1. " se convierte en una lista. Se define por proyecto y viaja dentro del .skrib, así que un coautor escribe con las mismas reglas. Es distinto de [sus propias reglas](:concept-text-replacement), que escribe usted mismo.
concept-export-style = Un conjunto con nombre de opciones de exportación: encabezados, espaciado, qué se incluye.
concept-export-style-more = Un estilo que se reutiliza en todas las exportaciones, sea cual sea el formato elegido. Los estilos integrados son de solo lectura; duplique uno para obtener un punto de partida que sí pueda cambiar. Entre sus opciones está incluir [marcadores de ida y vuelta](:concept-round-trip-marks), para un archivo que va a un editor y vuelve.
concept-round-trip-marks = Marcadores invisibles que permiten reconocer un DOCX u ODT editado cuando vuelve.
concept-round-trip-marks-more = Solo se escriben en los dos formatos que un editor puede devolver, DOCX y ODT; ninguna otra exportación los lleva. Son marcadores, no atributos personalizados: LibreOffice elimina los atributos personalizados al guardar pero deja intactos los marcadores. Se activan en cada [estilo de exportación](:concept-export-style), en «Envío a un editor».
concept-status = En qué punto de su propio flujo de trabajo está un elemento: una etapa cada vez, siempre hacia delante.
concept-status-more = No es una [etiqueta](:concept-tag): un elemento tiene exactamente un estado, las etapas están ordenadas y la lista pertenece al proyecto, así que puede renombrarla, reordenarla o ampliarla cuando quiera. Un asterisco junto a un capítulo o un libro significa que no todas las partes de debajo coinciden con él. Defínalo desde el inspector, una tarjeta, un elemento de un flujo o aquí.
