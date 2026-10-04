# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

wm-book = Un libro en su proyecto.
wm-book-more = Un libro es tan simple o tan capa como quieras. Una historia corta puede ser sólo unos pocos [escenas](:wm-scene). Un trabajo más largo puede anidar [partes](:wm-part) y [capítulos](:wm-chapter) tan profundo como quieras. La forma es tuya.
wm-part = Una parte agrupando capítulos.
wm-part-more = Una parte reúne [capítulos](:wm-chapter) bajo un título. Como un capítulo, se ejecuta hasta la siguiente parte o el final de la [libro](:wm-book).
wm-chapter = Un capítulo del libro.
wm-chapter-more = Un capítulo va desde aquí hasta el siguiente capítulo, el siguiente [Parte](:wm-part), o el [final del libro](:wm-end-of-book). Viene en dos formas: una carpeta de capítulo que contiene [escenas](:wm-scene), o un capítulo plano que lleva su propia prosa. Elija qué capítulos nuevos usan en Ajustes, bajo Estructura.
wm-scene = Una escena de prosa.
wm-scene-more = Cada artículo de la escritura tiene dos lados: el texto principal y un [sinopsis](:wm-synopsis) para planificar. Ambos abiertos juntos en el editor.
wm-note = Una nota.
wm-note-more = Una nota guarda texto y un [sinopsis](:wm-synopsis), como un [escena](:wm-scene), para investigación, a un lado o recordatorios. Cualquier artículo puede ser parte del libro compilado, incluyendo una nota sentada dentro de un [Capítulo](:wm-chapter).
wm-note-folder = Una nota que contiene otros artículos.
wm-note-folder-more = A [nota](:wm-note) que también contiene artículos debajo de él, por lo que puede agrupar notas relacionadas juntos. Conserva su propio [sinopsis](:wm-synopsis), y los elementos dentro guardan su propio texto.
wm-folder = Una carpeta de organización.
wm-folder-more = Un contenedor para la organización de artículos en la carpeta sin embargo le conviene. Lleva un [sinopsis](:wm-synopsis) pero sin prosa propia. Los artículos en el interior sostienen la escritura.
wm-end-of-book = Marca donde termina un libro.
wm-end-of-book-more = Cada libro comparte una lista continua, por lo que el final de un libro no se decide por anidar. Este marcador dice que el [libro](:wm-book) se detiene aquí. Cualquier cosa después de que pertenece al próximo libro.
wm-story-bible-entry = Una nota etiquetada, aliada sobre tu historia.
wm-story-bible-entry-more = Un [nota](:wm-note) sobre un personaje, una ubicación, o cualquier otra cosa que vale la pena presentar, creado con un nombre, etiquetas, alias y una plantilla de inicio todo en un solo paso. Nada en la página lo distingue después, ya que se mantiene una nota ordinaria que puede renombrar, etiquetar o convertir como cualquier otro.
wm-synopsis = Un resumen de un tema por escrito.
wm-synopsis-more = Un resumen adjunto a cualquier artículo de escritura, por lo general un párrafo corto pero siempre que desee. Úsalo para planificar y navegar antes de que exista la prosa. Se sienta al lado del texto principal en el editor.

scene-break-minor = Un descanso de escena normal: un cambio de tiempo, lugar o punto de vista dentro de un capítulo.
scene-break-minor-more =
    Marca un salto *donde lo insertas*, incluso en medio de una [escena](:wm-scene). Dividir el texto en dos elementos es una decisión organizativa y no crea un salto por sí sola.

    Se escribe en la prosa como tres asteriscos separados por espacios. Cómo se imprime se decide por el estilo de exportación, no por lo que escribe: un manuscrito de Shunn lo establece como un solo #, un papelback comercial como un dinkus, y la mayoría de la publicación francesa, alemana, española, rusa e italiana como una brecha sin marca en absoluto.
scene-break-major = Una división más fuerte: un salto de tiempo grande, o un cambio decisivo de punto de vista.
scene-break-major-more =
    La misma idea que un salto de escena ordinario, un paso hacia arriba. Utilízalo cuando un descanso claro subestimaría el salto.

    Escribe como # # #, e imprimió diferentemente del nivel ordinario por el estilo de exportación. El formato manuscrito estándar de Shunn establece un solo # contra # # para exactamente esta distinción. Donde una tradición no tiene marca más fuerte, ambos niveles imprimen lo mismo.
wm-find-in-prose = Skribisto busca este artículo en su prosa.
wm-find-in-prose-more = Enciende esto para etiquetas que nombren cosas sobre las que escribes: personajes, lugares, objetos. Cualquier artículo que lleve tal etiqueta se iguala a tu prosa por su título y por los otros nombres que le das, así que cada uno [escena](:wm-scene) lista quién y lo que aparece en él sin que vincules nada a mano. Déjalo para etiquetas que describen un artículo en lugar de nombre uno, como un borrador de estado o un recordatorio para comprobar la continuidad. Cada artículo que coincida también se une a su [historia bíblica](:wm-story-bible).
wm-story-bible = Cada personaje, lugar y otra entrada etiquetada, se reunieron en una cuadrícula.
wm-story-bible-more = Abrir de la propia ficha de cualquier carpeta de notas. Grupo de tarjetas por etiqueta, y cada nombre cuántos alias una entrada responde y cuántas escenas ha aparecido, dibujado de cada etiqueta [encontrar en prose](:wm-find-in-prose) en el proyecto. Filtrar por libro sólo estrecha qué tarjetas muestran; nunca decide qué entradas existen.

tooltip-go-to = Saltar a cualquier artículo en el carpeta (Ctrl+G)
synopsis-collapse-tooltip = Ocultar la columna de sinopsis

wm-paratext = Un texto que no es parte de la historia.
wm-paratext-more = Un prefacio, una dedicación, una palabra posterior, una colofonía: la escritura que pertenece al libro pero no a su cuerpo. Exportado dondequiera que lo pongas, y nunca contado en el recuento de la palabra del manuscrito. Donde depende de ti: convenciones difieren por país y editor.
wm-paratext-folder = Una carpeta para paratextos.
wm-paratext-folder-more = En algún lugar para mantener prefacio y post-palabras para que no arruinen el carpeta. Organizando solamente: lleva un [sinopsis](:wm-synopsis) pero no añade nada al libro exportado, ni siquiera su propio nombre.

goal-target = Cuánto tiempo esta pieza está destinada a ser.
goal-target-more = Establecer un objetivo en cualquier cosa: a [escena](:wm-scene), a [Capítulo](:wm-chapter), a [Parte](:wm-part), todo el [libro](:wm-book). Cada uno es su propio número y se para por sí mismo, por lo que establecer un objetivo en un capítulo nunca cambia el libro, y el objetivo de un libro nunca es una suma de los capítulos dentro de él. Lo que agrega es la escritura: un contenedor [Avances](:goal-progress) está todo escrito debajo de él. Deja un objetivo a cero y simplemente no está fijado. El número se cuenta en [palabras o caracteres](:goal-unit), cualquiera que sea el proyecto que utilice.
goal-unit = Si este proyecto cuenta con palabras o caracteres.
goal-unit-more = Una elección en todo el proyecto, porque una unidad de longitud pertenece al manuscrito y su mercado en lugar de a una sección: medida editorial alemana y francesa en signos, japonés en hojas de 400 caracteres, chino por los miles de caracteres, la mayoría de la publicación en inglés en palabras. Ambos números se mantienen lado a lado y ninguno se convierte en el otro, por lo que cambiar los puntos de la unidad cada [objetivo](:goal-target) en la otra figura y cambiar de nuevo no hace nada.
goal-progress = Cuanto está escrito, contra el objetivo.
goal-progress-explained = Una pieza con prosa propia se mide contra su propia longitud; un contenedor contra [todo escrito debajo](:goal-manuscript-words). La barra se calienta de rojo a ámbar a verde a medida que entra la escritura, y cambia una vez que la pieza ha pasado bien lo que estaba planeado, lo que vale la pena notar cuando un capítulo tiene una longitud a mantener.
goal-manuscript-words = Lo que cuenta como el manuscrito.
goal-manuscript-words-more = Sólo prosa que sería exportada en realidad: [escenas](:wm-scene) y [capítulos](:wm-chapter) llevando su propio texto. Un [nota](:wm-note) o un [paratexto](:wm-paratext) no es parte del manuscrito y nunca se cuenta hacia uno. Cualquier cosa en la basura cae, y así hace cualquier cosa [excluido de la exportación](:goal-exportable).
goal-exportable = Este artículo queda fuera de la exportación.
goal-exportable-more = Su propia longitud todavía muestra, porque la escritura todavía está allí, pero no cuenta con un total, exactamente como aparecerá en ningún libro exportado. Dejar un contenedor fuera no deja las piezas dentro de él hacia fuera: cada artículo responde por sí mismo, que es el botón "Aplicar a los elementos secundarios" al lado del interruptor es para.
goal-distribute = Comparte este objetivo a través de lo que hay dentro.
goal-distribute-more = Se divide un objetivo de [libro](:wm-book) o [parte](:wm-part) a través de sus elementos secundarios inmediatos, por lo que las piezas se suman exactamente a él. De forma predeterminada sólo llena los que aún no tienen objetivo y deja solos sus propios números. Lo que escribe son ordinarios [metas](:goal-target), libre de editar después y libre de separarse: esta es una acción, no un arreglo permanente.
goal-subtree-total = Lo que los objetivos en su interior se suman.
goal-subtree-total-more = Un hecho, no un objetivo. Este proyecto nunca trata el objetivo de un contenedor como la suma de los dentro de él, porque así es como la figura de una carpeta termina moviéndose por su cuenta cuando una escena dentro de ella se da un número. Esta línea simplemente le dice a qué vienen sus propios números, por lo que puede compararlo con el contenedor [objetivo](:goal-target) usted mismo, o [compartir uno fuera](:goal-distribute).
goal-milestone = Una cita para llegar a algo.
goal-milestone-more = Dos tipos. Uno dice que el [libro](:wm-book) debe permanecer a una longitud dada por una fecha, un punto de referencia en su propia curva. El otro dice que un particular [Capítulo](:wm-chapter) o [Parte](:wm-part) debe haber alcanzado [su propio objetivo](:goal-target) para entonces, y lee que el número vive en lugar de mantener una copia de ella, por lo que la edición del objetivo actualiza el hito también. Ambos aparecen en el libro [Plan de acción](:pace-plan).
pace-plan = El programa de escritura del libro.
pace-plan-more = Un [objetivo](:goal-target) y un plazo, con los días en los que escribe, se convirtió en lo que un día necesita. Lee el propio objetivo del libro, por lo que establecerlo aquí y establecerlo en el Inspector son el mismo número en dos lugares. Waypoints along the way are [hitos](:goal-milestone).

concept-tag = Una etiqueta codificada por colores que puede poner en cualquier artículo, reutilizable en todo el proyecto.
concept-tag-more = Etiquete un artículo para marcarlo: un estado, una ubicación, un hilo que estás rastreando. La misma etiqueta puede sentarse en cualquier número de elementos, y un artículo puede llevar varios. Enciende [encontrar en prose](:wm-find-in-prose) para una etiqueta que nombre algo en su libro, un personaje o un lugar, y Skribisto comienza a buscarlo en su prosa.
concept-label = Una nota corta que escribes bajo el título de un artículo, para tus propios ojos.
concept-label-more = No es un [etiqueta](:concept-tag): una etiqueta pertenece a un solo artículo, texto plano sin color ni catálogo detrás de él, algo como "punto de trama 1" o "necesita un nombre". Ponlo en el menú contextual del artículo, o editarlo en línea en la columna de etiqueta de la vista general; se muestra como un pequeño subtítulo bajo el título, en el contorno y en la secuencia por igual.
concept-point-of-view = Los ojos de quién se narra una escena: uno o más del reparto de tu historia.
concept-point-of-view-more = Elegir a alguien que aún no está en el reparto también los añade allí. Más de un punto de vista es una opción legal, útil para una escena compartida o un cambio de ojos medio libro, no algo Skribisto bloques. Los candidatos vienen de los artículos una etiqueta [encontrar en prose](:wm-find-in-prose) pone en su historia Biblia.
concept-epigraph = Una cita fijada en la cabeza de una parte o un capítulo.
concept-epigraph-more = Su propio campo, separado de la [Capítulo](:wm-chapter) o de la propia prosa [Parte](:wm-part); un [escena](:wm-scene) o un [nota](:wm-note) no lleva ninguno. Escribe uno como una cita ordinaria; escribe varios y cada uno se convierte en su propio bloque, impreso como su propio epígrafe. Sus palabras nunca se cuentan como palabras de manuscrito.
concept-footnote = Una cita marcada en la prosa, imprimiendo como nota numerada.
concept-footnote-more = La referencia es una sola marca dentro de sus palabras, por lo que se mueve y elimina exactamente como lo hacen las palabras alrededor de ella. Su número nunca se almacena: Skribisto lo hace fresco desde donde cada referencia se sienta en todo el manuscrito, por lo que insertar uno renumera cada nota después de ella. Las palabras de una nota de pie se cuentan aparte del manuscrito, nunca plegado en él.
concept-chapter-mode = Ya sea un nuevo capítulo es una carpeta de escenas o un solo elemento plano.
concept-chapter-mode-more = Establecido en Ajustes, bajo Estructura. Escribe directamente en un [Capítulo](:wm-chapter) de cualquier manera; la única diferencia es si también puede contener escenas debajo de él. Ambas codificacións compilan al mismo libro, y puede mezclarlos libremente en un proyecto. Promover convierte un capítulo existente entre los dos sin perder una palabra.
concept-comment = Una nota anclada a un estiramiento exacto de la prosa, mostrada en el margen.
concept-comment-more = Recuerda las palabras en las que apunta y lo que las rodea, no una posición fija, por lo que sobrevive la edición ordinaria y una recarga completa. Responder debajo para mantener un hilo en marcha. Si la cita exacta ya no se puede encontrar, el comentario dice así en lugar de desviarse silenciosamente a la frase equivocada.
concept-backup = Una copia puntual de todo el proyecto, guardada al lado de él.
concept-backup-more = Tomado en un horario que elijas: en abierto, de cerca, en un intervalo, o en cualquier mezcla, con copias viejas podadas por edad o por cuenta. Abrir una carga en su propia ventana: puedes mirar e incluso editar, pero sólo Guardar como o Restaurar mantiene cualquier cosa. Diferente de un [versión](:concept-version), que rastrea el propio pasado de un artículo.
concept-version = Un artículo es el propio pasado: cada cambio a su texto o sinopsis.
concept-version-more = Construido de la propia historia de ahorro del proyecto y sus respaldos juntos, una entrada por cambio en lugar de uno por archivo. Escoge una entrada para ver qué cambió y restaurar sólo el texto de ese artículo a él; el resto del proyecto es intacto, y una copia de seguridad se toma primero. Diferente de un [copia de seguridad](:concept-backup), que es una copia de todo.
concept-trash = Un elemento destrozado está oculto, no movido: se queda exactamente donde estaba.
concept-trash-more = Trashing sólo da vueltas una bandera: el artículo mantiene su lugar en el carpeta, marcado inactivo en lugar de retirar. Restaurar y reaparece en ese lugar exacto, sin refilar para hacer. Empty Trash elimina los elementos destrozado para el bien, y cada uno de ellos es un paso indomable, como cualquier otro proyecto de edición.
concept-spellcheck = Subraya palabras que ningún diccionario instalado reconoce, por idioma del proyecto.
concept-spellcheck-more = Cada proyecto tiene su propio lenguaje de trabajo o idiomas; un artículo puede anularlos para una escena escrita en otra lengua. Añada una palabra Skribisto no sabe del menú contextual del editor, en un diccionario personal que lleva a cada proyecto. Desactivar el hechizo lo detiene en todas partes, hasta que lo vuelvas a encender.
concept-search-replace = Encuentra una palabra o frase en todas partes del proyecto, no sólo el documento abierto.
concept-search-replace-more = Se ve a través de cada escena, nota, título, sinopsis y etiqueta, y a través de comentarios y notas de pie también. Caso, la combinación de palabras enteras y el emparejamiento de acento son tres interruptores separados. Reemplazar Todos reescribe cada partido revisado en un pase y un paso de deshacer, por lo que Ctrl+Z recupera todo el lote, nunca sólo el último cambio.
concept-note-template = Una pieza reutilizable de escribir que insertas en cualquier cosa en la que estés trabajando.
concept-note-template-more = Una hoja de caracteres en blanco, un perfil de ubicación, una hoja de cálculo: escríbala una vez y déjala en cualquier lugar que lo necesite, en cualquier artículo, no sólo una nota. Comenzar desde un preset incorporado, importar un archivo .md o .djot, o escribir algo y elegir Documento − Guardar como plantilla. Almacenado en el proyecto, por lo que un coautor que abre consigue el mismo conjunto.
concept-text-replacement = Su propio cortocircuito, se expandió automáticamente a medida que escribe.
concept-text-replacement-more = Establece un gatillo y su reemplazo, "btw" para "por cierto", y se expande el momento en que escribe un espacio o puntuación después de él. Caso sigue lo que usted escribió: capitalice el gatillo y la expansión capitaliza también. A Backspace justo después de una expansión se deshace sólo que una sustitución. Enciende por proyecto, distinto de [puntuación inteligente](:concept-smart-punctuation).
concept-smart-punctuation = Tipografía de conocimiento local se aplica automáticamente como escribe: presupuestos, pestañas, elipses.
concept-smart-punctuation-more = Curls citas rectas en tipografías, se convierte en un dash en y - en un em dash, se convierte ... en una elipsis, y, donde se enciende, abre un párrafo escrito como "- " con un dash diálogo. Conjunto por proyecto, y viaja dentro del .skrib por lo que un coautor escribe con las mismas reglas. Distinto de [tus propias reglas](:concept-text-replacement), que usted escribe usted mismo.
concept-export-style = Un paquete llamado de opciones de exportación: títulos, espaciamiento, lo que se incluye.
concept-export-style-more = Un estilo, reutilizado a través de cada exportación cualquier formato que elija. Los estilos incorporados son sólo lectura; duplicar uno para obtener un punto de partida que realmente puede cambiar. Entre sus opciones es si incluir [marcadores de ida y vuelta](:concept-round-trip-marks), para un archivo vinculado a un editor y espalda.
concept-round-trip-marks = Marcas invisibles que permiten una edición de DOCX o ODT volver a casa reconocida.
concept-round-trip-marks-more = Escrito sólo en los dos formatos que un editor puede enviar de vuelta, DOCX y ODT; cada otra exportación no lleva ninguno. Un marcador, no un atributo personalizado: LibreOffice tiras atributos personalizados en ahorro pero deja un marcador intacto. Convertido en por [estilo de exportación](:concept-export-style), bajo "Enviando a un editor".
concept-status = Donde un artículo se encuentra en su propio flujo de trabajo: una etapa a la vez, avanzando.
concept-status-more = No es un [etiqueta](:concept-tag): un artículo tiene exactamente un estado, las etapas se ordenan, y el proyecto posee la lista, por lo que puede renombrar, reordenar o añadir a él cuando quiera. Un asterisco junto a un capítulo o libro significa que las partes inferiores no todos están de acuerdo con él. Establecelo desde el Inspector, una tarjeta, un artículo de corriente, o aquí.
