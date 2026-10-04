# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

menu-work = П&роект
menu-new-work = Н&овый проект
menu-open-work = О&ткрыть проект…
menu-new-window = &Новое окно
menu-import-from = &Импортировать из
menu-create-from = &Создать из
menu-import-plume = &Plume Creator (.plume)…
menu-import-document = &Документы (Markdown, Word, ODT) …
menu-import-manuskript = &Manuskript (.msk)…
menu-export = &Экспорт
menu-export-book = Экспортировать книгу
menu-export-part = Экспортировать часть
menu-export-chapter = Экспортировать главу
menu-export-scene = Экспортировать сцену
menu-export-note = Экспортировать заметку
menu-export-paratext = Экспортировать паратекст
menu-export-folder = Экспортировать папку
menu-export-choose = Выбрать…
menu-export-none = Откройте документ для экспорта
menu-save = &Сохранить
menu-save-as-file = Сохранит&ь одним файлом…
menu-save-as-folder = Со&хранить папкой…
menu-backup = Соз&дать резервную копию
menu-close-work = &Закрыть проект
menu-welcome = До&бро пожаловать…
menu-settings = Н&астройки
menu-quit = &Выход

menu-view = &Вид
menu-outline = С&труктура
menu-search = &Поиск в проекте
menu-timeline = &Вернуться в прошлое
menu-search-preview = П&редпросмотр поиска
menu-fullscreen = П&олный экран
menu-focus-mode = Р&ежим без отвлечений
menu-format = &Формат
menu-scene-break = Вкл&ючить прерывание сцены
menu-major-scene-break = Включ&ить основную сцену перерыва

menu-go = П&ереход
menu-go-next-scene = &Следующая сцена
menu-go-prev-scene = &Предыдущая сцена
menu-go-next-chapter = С&ледующая глава
menu-go-prev-chapter = П&редыдущая глава
menu-go-next-note = Сл&едующая запись
menu-go-prev-note = Пре&дыдущий Note

menu-tools = &Инструменты
menu-help = &Справка

ctx-add = &Добавить
ctx-new-item = &Новый элемент
ctx-new-folder = &Новая папка
ctx-rename = П&ереименовать
ctx-duplicate = Д&ублировать
ctx-import-here = &Импортировать сюда…
ctx-indent = У&величить отступ
ctx-outdent = У&меньшить отступ
ctx-trash = Пе&реместить в корзину
ctx-open-to-side = &Откройте на стороне
ctx-open = &Открыть
ctx-reveal-in-outline = &Показать в Outline
ctx-move-up = &Двигаться вверх
ctx-move-down = &Двигаться вниз

ctx-tab-close = &Закрыть
ctx-tab-close-others = За&крыть остальные
ctx-tab-close-all = З&акрыть все
ctx-tab-open-to-side = &Откройте на стороне
ctx-tab-open-in-main = &Открыто в главном щите
ctx-tab-move-to-side = &Двигайтесь в сторону
ctx-tab-move-to-main = П&ерейти на главную панель
ctx-tab-move-to-new-window = &Перейдите в новое окно
ctx-tab-move-window-unsaved = Сохраните этот проект первым: второе окно открывается на файл на диске.
ctx-tab-pin = П&ин на этой вкладке
ctx-tab-unpin = &Развяжи эту вкладку
tab-pinned-tooltip = Слоган: «Close others» и «Close all»

create-book = Книга
create-part = Часть
create-chapter = Глава
create-scene = Сцена
create-note = Заметка
create-note-folder = Папка заметок
create-folder = Папка
create-paratext = Паратекст
create-paratext-folder = Папка паратекстов
create-book-end = Конец книги
create-story-bible-entry = Запись энциклопедии истории…
type-book-start = Начало книги
type-text = Текст

new-item-book = Новая книга
new-item-part = Новая часть
new-item-chapter = Новая глава
new-item-note = Новая заметка
new-item-note-folder = Новый Note Folder
new-item-folder = Новая папка
new-item-paratext = Новый Паратекст
new-item-paratext-folder = Новая паратекстовая папка
new-item-story-bible-entry = Новая история Библейский вход

placement-inside = внутри
placement-after = после
placement-after-parent = после родителя
placement-top-level = верхний уровень


ctx-promote = &Преобразовать в
promote-chapter-folder = Глава-папка
promote-flat-chapter = Глава без сцен
promote-lossy-title = Негде хранить текст
promote-lossy-text = { $target } не имеет ничего, чтобы сохранить это: { $kinds }. Сначала переместите или очистите этот текст, а затем преобразуйте.
content-scene-text = Сценический текст
content-note-text = Текст примечания
content-book-title = Название книги
content-book-subtitle = Подзаголовок книги
content-part-title = Название части
content-chapter-title = Название главы
content-epigraph-text = Эпиграф
content-paratext-text = Паратекст
promote-blocked-title = Глава не пустая
promote-blocked-text = { $count ->
    [one] Эта глава по-прежнему содержит { $count } пункт. Переместите или помойте его, прежде чем преобразовать в плоскую главу.
    [few] Эта глава по-прежнему содержит { $count } предмета. Переместите или выбросьте их, прежде чем преобразовать в плоскую главу.
    [many] Эта глава по-прежнему содержит { $count } предметов. Переместите или выбросьте их, прежде чем преобразовать в плоскую главу.
   *[other] Эта глава по-прежнему содержит { $count } предметов. Переместите или помойте их, прежде чем преобразовать в плоскую главу.
}

inspector = Свойства
inspector-empty = Откройте предмет, чтобы проверить его.
inspector-promote = Преобразовать на…
statusbar-toggle-outline = Показать или скрыть навигатор
statusbar-toggle-inspector = Показать или скрыть свойства
statusbar-save-unsaved = Несохраненные изменения. Нажмите, чтобы сохранить
statusbar-save-saved = Все изменения сохранены
statusbar-save-autosave = Автосохранение включено. Изменения сохраняются, когда вы пишете
statusbar-saving = Сохранение…
statusbar-word-count = { $count ->
    [one] { $count } слово
    [few] { $count } слова
    [many] { $count } слов
   *[other] { $count } слова
}

statusbar-word-count-tooltip = Слова в сцене, которые вы редактируете
statusbar-word-char-count = { $words ->
    [one] { $words } слово
    [few] { $words } слова
    [many] { $words } слов
   *[other] { $words } слова
} · { $chars ->
    [one] { $chars } символ
    [few] { $chars } символа
    [many] { $chars } символов
   *[other] { $chars } символа
}

session-toggle = Письменная сессия: начать или приостановить сфокусированный спринт
session-configure = Установите цель сеанса и ограничение по времени
session-configure-title = Письменная сессия
session-word-goal = Цель в словах
session-time-limit = предельный срок
session-time-limit-unit = мин.
session-no-goal = Нет цели
session-no-limit = Нет предела
session-reset = Сессия сброса
session-readout = Слов: { $words } · { $time }
session-readout-timed = { $words } слова · { $time } слева
statusbar-focus-exit = Режим Exit Distraction-Free
statusbar-focus-synopsis = Синопсис
statusbar-focus-go-prev = Предыдущая статья (Alt+Up)
statusbar-focus-go-next = Следующая статья (Alt+Down)

rail-settings = Настройки

language = Язык
theme = Тема
english = английский
french = французский
light = Светлая
dark = Тёмная
settings-text-width = Ширина текста
settings-preview-width = Ширина предварительного просмотра
settings-autosave = Автосохранение на диск
settings-show-welcome = Показать пусковую установку при запуске
settings-show-welcome-tip = После завершения работы, последний проект возобновляется.
settings-check-for-updates = Проверьте новые версии
settings-check-for-updates-tip = Не более одного раза в день Skribisto задает веб-сайту проекта вопрос о том, какая версия является текущей. Запрос не содержит идентификатора и даже версии, которую вы используете: сравнение происходит на этом компьютере.

settings-title = Настройки
settings-close = Закрыть
settings-search = Параметры поиска
settings-reset = Восстановить настройки по умолчанию
settings-done = Готово
settings-cancel = Отмена
settings-apply = Применить
settings-ok = ОК
settings-reset-confirm-title = Сброс всех настроек по умолчанию?
settings-reset-confirm-body = Восстанавливает внешний вид этого приложения, настройки редактора и записи. Собственные настройки вашего проекта, ярлыки клавиатуры и сохраненные стили не затронуты. Его нельзя отменить.
settings-empty-title = Здесь пока нет настроек
settings-empty-hint = Этот раздел получит опции в будущем обновлении.

settings-sec-appearance-behaviour = Поведение внешности
settings-sec-editor = редактор
settings-sec-spelling = Правописание
settings-sec-backup = Резервный синхронизатор
settings-sec-compile = Компиляция и экспорт
settings-page-appearance = внешний вид
settings-field-image-size-policy = Большие изображения
settings-image-policy-ask = Спрашивайте каждый раз
settings-image-policy-keep = Держите оригинал
settings-image-policy-downscale = Оптимизировать
settings-hint-image-size-policy = Что делать, если вставленное изображение больше, чем нужно рукописи?
settings-page-notifications = Уведомления
settings-page-scene = Сцена
settings-page-synopsis = Синопсис
settings-page-notes = Заметки
settings-page-editor-behavior = Поведение редактора
settings-page-goals = Голосование Word Count
settings-page-games = Написание игр
settings-page-corkboard = Пробковая доска
settings-page-distraction-free = без отвлекающих факторов
settings-page-dictionaries = Словари
settings-page-autosave = Автосохранение
settings-page-export = Форматы экспорта
settings-page-paratext = Паратекстовые структуры
settings-page-keymap = Сочетания клавиш
settings-keymap-filter = Фильтровать ярлыки

settings-desc-sec-appearance-behaviour = Как выглядит приложение и что оно делает, когда оно начинается.
settings-desc-sec-editor = Поверхность письма: как она выглядит и что она делает, когда вы вводите.
settings-desc-sec-spelling = Проверка орфографии и словари, стоящие за ней.
settings-desc-sec-backup = Как ваша работа достигает диска и какие копии хранятся.
settings-desc-sec-compile = Что оставляет Skribisto, и в какой форме.
settings-desc-sec-work = Настройки, которые относятся к этому проекту и перемещаются внутри его файла.
settings-desc-sec-extensions = Страницы, добавленные расширениями, установленными здесь.
settings-desc-group-typography = Одна страница на вид текста: шрифт, размер, высота строки и расстояние.
settings-desc-appearance = Язык интерфейса, тема, размер текста и начальное окно.
settings-desc-notifications = Каждое сообщение, которое показал этот сеанс, и действия, которые вы можете воспроизвести.
settings-desc-scene = Как устанавливается проза сцены: шрифт, размер, высота линии, отступы и расстояние.
settings-desc-synopsis = Как устанавливается панель синопсиса, независимо от рукописи.
settings-desc-notes = Как записываются заметки, независимо от рукописи.
settings-desc-corkboard = Размер карты, что показывает карта, и как доска их устраивает.
settings-desc-distraction-free = Типография, ширина колонки и полоса управления для полноэкранного письма.
settings-desc-distraction-free-themes = Тематическая библиотека для полноэкранного письма: отгруженные и собственные.
settings-desc-editor-behavior = Ширина текста, положение синопсиса, прокрутка пишущей машинки и подсветка заботы.
settings-desc-punctuation = Начинаются новые проекты в стиле типографского дома. Каждый проект следует этому, если он не устанавливает свой собственный.
settings-desc-goals = Слова и характер цели, и как слова подсчитаны.
settings-desc-games = Ограничения, которые вы накладываете на себя во время составления, такие как Всегда вперед.
settings-desc-spellcheck = Единственный переключатель, который включает и выключает проверку орфографии везде.
settings-desc-dictionaries = Установите, удалите и просмотрите словари орфографии на этой машине.
settings-desc-autosave = Если ваши правки написаны на диск самостоятельно.
settings-desc-backup = Когда берутся копии, где они хранятся, и сколько. Каждый проект следует этому, если он не устанавливает свой собственный.
settings-desc-export = Стили, через которые проходит каждый экспорт: отгруженные и ваши собственные.
settings-desc-paratext = Спереди и сзади может начаться новый проект.
settings-desc-keymap = Каждое короткое замыкание и то, к чему оно привязано.
settings-page-user = Пользователь
settings-desc-user = Кто вы, для комментариев, которые вы пишете.
settings-group-identity = Идентичность
settings-field-user-name = Твое имя
settings-field-user-name-placeholder = необязательный
settings-field-user-name-hint = Подписывает комментарии и ответы, которые вы пишете. Отличается от имени автора проекта, которое является строкой книги и перемещается внутри файла.
settings-field-user-initials = Ваши инициалы
settings-field-user-initials-placeholder = От твоего имени
settings-field-user-initials-hint = Что текстовый процессор показывает рядом с вашим комментарием на полях. Оставьте его пустым, чтобы использовать показанные, взятые из вашего имени.
settings-field-user-hint = Оба являются необязательными и применяются к каждому проекту на этом компьютере. Изменение их подписывает комментарии, которые вы пишете дальше. Комментарии, уже написанные, сохраняют имя, под которым они были написаны.
settings-desc-author = Название, которое идет в этом проекте.
settings-desc-structure = Являются ли главы этого проекта папками или отдельными элементами.
settings-desc-language = Язык прозы этого проекта проверяется.
settings-desc-work-backup = Собственная резервная политика проекта, или общая.
settings-desc-personal-dictionary = Слова, которые этот проект рассматривает как правильно написанные.
settings-desc-tags = Цветные ярлыки, которыми этот проект обозначает свои предметы.
settings-desc-templates = Шаблоны заметок вы можете вставить во время написания в этом проекте.
settings-desc-text-replacements = Ярлыки, которые расширяются по мере того, как вы вводите в этот проект.
settings-desc-work-punctuation = Цитаты, тире и интервалы прозы этого проекта следуют.

settings-group-typography = Типография
settings-group-writing-column = Написание колонки
settings-group-writing-view = Письменный взгляд
settings-group-distraction-free-strip = Контрольная полоса
settings-group-theme = Тема
settings-group-language = Язык
settings-group-startup = Запуск
settings-group-updates = Обновления
settings-group-autosave = Автосохранение
settings-field-typeface = Гарнитура
settings-field-size = Размер текста

editor-size-changed-manuscript = Размер текста рукописи: { $percent }
editor-size-changed-synopsis = Размер текста: { $percent }
editor-size-changed-notes = Размер текста: { $percent }
editor-size-changed-corkboard = Размер текста карточки: { $percent }
editor-size-changed-corkboard-expanded = Размер текста редактора: { $percent }
editor-size-changed-distraction-free = Размер текста без отвлечения: { $percent }

menu-text-size-increase = &Увеличить размер текста
menu-text-size-decrease = Уме&ньшить размер текста
menu-text-size-reset = С&брос размера текста
settings-field-line-height = Высота линии
settings-field-first-line-indent = отступ первой линии
settings-field-paragraph-spacing-before = Пространство перед пунктом
settings-field-paragraph-spacing-after = Пространство после абзаца
settings-field-column-width = Ширина колонки
settings-distraction-free-width-hint = Применяется только в режиме без отвлечения. Ширина колонок Scene, Synopsis и Notes не тронута.
settings-distraction-free-title = Сохранить название предмета
settings-distraction-free-word-count = Держите слово счет
settings-distraction-free-session = Сохраните письменную сессию
settings-distraction-free-go-to = Держите кнопку Go to…
settings-distraction-free-go = Держите кнопки «Предыдущее и следующее»
settings-distraction-free-chrome-hint = Кнопка выхода всегда остается, что бы вы ни выбрали, это ваш выход, если Escape занят.
settings-field-app-theme = Тема
settings-theme-light = Светлая
settings-theme-dark = Тёмная
settings-field-text-scale = Размер текста интерфейса
settings-field-language = Язык интерфейса
settings-synopsis-placement = Позиция синопсиса
settings-synopsis-placement-none = Никто.
settings-synopsis-placement-top = Выше
settings-synopsis-placement-side = Рядом
settings-typewriter = Прокрутка пишущей машинки
settings-typewriter-tip = Удерживает строку, которую вы пишете, на фиксированной высоте, в то время как рукопись прокручивается под ней. Нажатие по-прежнему ставит заботу, куда вы нажимаете.
settings-typewriter-position = Расположение линии
settings-typewriter-position-top-third = Топ третье место
settings-typewriter-position-middle = Средний
settings-typewriter-position-bottom-quarter = Нижний квартал
settings-highlight-scope = Подсветка вокруг заботы
settings-highlight-scope-none = Никто.
settings-highlight-scope-sentence = Приговор
settings-highlight-scope-paragraph = Пункт
settings-highlight-scope-tip-none = Ничего не затеняется, пока ты пишешь.
settings-highlight-scope-tip-sentence = Отбросьте предложение, которое вы пишете, чтобы оно выделялось из окружающих.
settings-highlight-scope-tip-paragraph = Отметьте весь абзац, который вы пишете, чтобы сохранить в поле зрения отрывок, над которым вы работаете.
settings-group-container-views = Виды контейнеров
settings-remember-view = Запомните последний вид для каждого типа элемента
settings-remember-view-tip = Откройте контейнер с видом, который вы в последний раз использовали для этого типа.
settings-remember-view-tip-more =
    Книга, часть и глава предлагают несколько просмотров (собственная страница, полная версия).
    Рукопись, полный синопсис. Включите это и каждый тип снова откроется на
    вид, который вы в последний раз выбрали для него, например, переключите одну главу на полную главу и
    Следующая глава, которую вы откроете, также показывает Полную главу. Каждый тип запоминает свой собственный взгляд.
settings-group-counting = Подсчет слов
settings-counting-auto = Автоматический (по языку)
settings-counting-whitespace = Расщепление на пространства
settings-counting-unicode-words = Однокодовые слова
settings-counting-cjk-hybrid = CJK-aware (персонаж)
settings-counting-hint = Автоматически подсчитывает китайский и японский по характеру и любой другой язык по слову. Измените его, только если счет выглядит неправильно для вашего языка.
settings-group-goals-display = отображение
settings-show-characters = Показать количество персонажей в статус-баре
settings-autosave-hint = Изменения записываются на диск автоматически, когда вы пишете.

settings-sec-work = Проект
settings-page-structure = Структура
settings-page-author = Автор
settings-field-author-name = Имя автора
settings-field-author-placeholder = необязательный
settings-field-author-hint = Появляется на скомпилированной титульной странице и в экспортированных метаданных файла. Оставьте его пустым, чтобы опустить. Это строка книги. Имя, с которым подписаны ваши комментарии, находится в Настройках ▸ Пользователь.
settings-group-chapters = Главы
tidy-titles-title = Типичные заголовки глав
tidy-titles-none = Ни одна глава или часть заголовка не повторяет свой собственный номер.
tidy-titles-lead = { $count ->
    [one] { $count } глава или часть озаглавлены только собственным номером.
    [few] { $count } главы и части названы только их собственным номером.
    [many] { $count } глав и частей озаглавлены не чем иным, как собственным номером.
   *[other] { $count } главы и части названы только их собственным номером.
    }
tidy-titles-explain = Очистка этих титулов оставляет каждого по имени по номеру, который уже знает книга, по тому же номеру, который печатается на экспорте. Ничто больше не меняется, и одно аннулирование возвращает их все.
menu-document-tidy-titles = &Типичные заголовки глав …
settings-group-numbering = нумерация
settings-number-chapters = Численные главы и части
settings-number-chapters-tip = Главы и части несут число, отработанное там, где они находятся в книге, показанное рядом с их названием здесь и напечатанное экспортом.
settings-number-chapters-tip-more = Номер никогда не хранится в заголовке, поэтому он остается правильным, когда вы перезаказываете, вставляете или удаляете. Выключите это, и экспорт печатает только заголовки, независимо от стиля экспорта. Чтобы оставить одну главу (пролог, интерлюдия), используйте выключатель нумерации инспектора вместо этого: это сохраняет его в книге, но останавливает его, принимая номер.
settings-part-resets-chapter = Перезапуск номеров глав в каждой части
settings-part-resets-chapter-tip = Выключено по умолчанию: главы проходят прямо по частям книги, поэтому «Часть вторая» открывается на одиннадцатой главе.
settings-part-resets-chapter-tip-more = Это обычная торговая практика и то, что ожидает читатель. Включите ее для книги, части которой предназначены для чтения в виде отдельных томов, где каждый открывается на первой главе.
settings-chapter-flat = Главы без сцен
settings-chapter-flat-hint = На: глава - это один элемент. Вы пишете в нее, и в ней нет сцен. Выключено: глава - это папка. Вы все еще пишете в нее, но она также может содержать сцены. Новые главы следуют за этой настройкой; существующие конвертируются через Promote.

settings-styles-builtin = Встроенные стили
settings-styles-user = Мои стили
settings-styles-builtin-badge = встроенный
settings-styles-duplicate = Дублировать
settings-styles-edit = Правка
settings-styles-delete = Удалить
settings-styles-import = Импорт …
settings-styles-export = Экспорт …
settings-styles-copy-suffix = (копия)
settings-styles-json-filter = Экспортный стиль
settings-styles-editor-title = Редактировать стиль
settings-styles-editor-none = Выберите пользовательский стиль для редактирования или создайте копию встроенного.
settings-styles-imported = Стиль импортный
settings-styles-import-failed = Невозможно импортировать стиль
settings-styles-exported = Стиль экспортируется
settings-styles-export-failed = Невозможно экспортировать стиль
settings-styles-field-name = Название
settings-styles-field-chapters = Заголовки глав
settings-styles-field-parts = Заголовки частей
settings-styles-field-scene-break = Сценический прорыв
settings-styles-field-major-scene-break = Основная сцена прорыва
settings-styles-field-spacing = Расстояние между линиями
settings-styles-field-justify = Обосновывать текст
settings-styles-field-notes = Включить примечания
settings-styles-field-synopses = Включая синопсисы
settings-styles-field-scene-titles = Включает названия сцен
settings-styles-field-epigraphs = Включая эпиграфы
settings-styles-field-paratexts = Включить паратексты
settings-styles-field-epigraph-placement = Положение эпиграфа
settings-styles-epigraph-after = После титула
settings-styles-epigraph-before = Перед титулом
settings-styles-field-footnotes = Включить сноски
settings-styles-field-footnote-numbering = нумерация сноски
settings-styles-footnote-numbering-continuous = непрерывный
settings-styles-footnote-numbering-per-chapter = Перезагрузите каждую главу
settings-styles-footnote-numbering-per-book = Перезагрузить каждую книгу
settings-styles-field-images = Изображения
menu-image = И&зображение
settings-styles-images-beside = Помимо документа
settings-styles-images-embed = Внутри документа
settings-styles-images-omit = Уходите.
settings-styles-group-round-trip = Отправка редактору
settings-styles-field-comments = Включить замечания
settings-styles-field-round-trip-marks = Добавить метки обратного импорта
settings-styles-round-trip-hint = Только DOCX и ODT. Невидимые метки позволяют обновить этот проект при импорте отредактированного файла, а не создать вторую копию.
settings-styles-group-pages = Страницы
settings-styles-field-cover = Открыть с помощью обложки
settings-styles-field-word-count = Количество слов на титульном листе
settings-styles-field-page-books = Новая страница в каждой книге
settings-styles-field-page-parts = Новая страница в каждой части
settings-styles-field-page-chapters = Новая страница в каждой главе
settings-styles-field-page-paratexts = Новая страница в каждом паратексте
settings-styles-heading-none = Без заголовков.
settings-styles-heading-numbered = Только число
settings-styles-heading-title = Только название
settings-styles-heading-both = Номер + название
settings-styles-break-blank = Глазная линия
settings-styles-break-none = Никто.
settings-styles-spacing-single = одиночный
settings-styles-spacing-onehalf = 1½
settings-styles-spacing-double = двойной

welcome-title = Добро пожаловать в Skribisto
welcome-version = Версия { $version }
welcome-update-available = Версия { $version } доступна
welcome-update-tip = Открывает страницу загрузки в вашем браузере.
welcome-search = Поисковые работы
welcome-open = Открыть
welcome-new-work = Новый проект
welcome-create-from = Создать из…
welcome-recent-works = Недавние проекты
welcome-empty-recents = Недавних работ пока нет.
welcome-no-matches = Ни одна недавняя работа не соответствует вашему поиску.
welcome-learn-soon = Скоро появятся путеводители и советы.
welcome-about-blurb = Skribisto, Rust + Teksilo переписывает приложение для письма.
welcome-tagline = *Тихое место для написания длинных вещей.*
welcome-github = GitHub
welcome-discord = разлад
nav-works = Проекты
nav-examples = Примеры
nav-learn = Учиться
nav-about = О нас

synopsis = Синопсис
epigraph = Эпиграф
pane-manuscript = рукопись
corkboard = Пробковая доска
overview = Обзор

corkboard-card-count = { $count ->
    [one] { $count } карточка
    [few] { $count } карты
    [many] { $count } карточек
   *[other] { $count } карты
}
corkboard-child-count = { $count ->
    [one] { $count } пункт
    [few] { $count } пункта
    [many] { $count } элементов
   *[other] { $count } единицы
}
corkboard-view-nested = вложенный
corkboard-view-flat = плоский
corkboard-layout-hint = На ней изображены прямые дочерние элементы контейнера. Откройте папку, чтобы войти в нее. Плоская показывает каждую сцену во всем контейнере сразу.
corkboard-card-size = Размер карты
corkboard-search-placeholder = Фильтровые карты …
corkboard-empty-title = Пока ничего нет.
corkboard-empty-hint = Используйте «＋ New» выше, чтобы добавить первый.
corkboard-new = Новый
corkboard-show-card-numbers = Номер карточки
corkboard-modal-size = Расширенный размер редактора
corkboard-scope-hint = Применяется к каждой открытой пробке, в каждом проекте.
corkboard-sort-manuscript = Рукописный приказ
corkboard-sort-title-asc = Название: A-Z
corkboard-sort-title-desc = Название: Z-A
corkboard-card-number = Карта { $number }

duplicate = Дублировать
duplicate-n = { $count ->
    [one] Дублировать { $count } карточку
    [few] Дублировать { $count } карты
    [many] Дублировать { $count } карт
   *[other] Дублировать карты { $count }
}
set-label-n = { $count ->
    [one] Установите этикетку на { $count } карточке
    [few] Установите этикетку на { $count } карточках
    [many] Установите этикетку на { $count } карточках
   *[other] Установите этикетку на карточках { $count }
}
move-to-trash-n = { $count ->
    [one] Переместить { $count } карту в мусор
    [few] Переместить { $count } карты в мусор
    [many] Переместить { $count } карт в мусор
   *[other] Переместите { $count } карт в корзину
}
reveal-in-outline = Показать в общих чертах

corkboard-move-to = Перейти на …
corkboard-move-to-n = { $count ->
    [one] Переместить { $count } карту в …
    [few] Переместите { $count } карты на …
    [many] Переместите { $count } карт на …
   *[other] Переместить карты { $count } в …
}
corkboard-move-picker-title = Перейти на …
corkboard-move-picker-empty = Пока никаких связующих.Создай сначала.
corkboard-move-picker-cancel = Отмена
corkboard-move-here = Двигайся сюда
corkboard-moved-ok = { $count ->
    [one] { $count } карточка перемещена
    [few] { $count } карты перемещены
    [many] { $count } карточек перемещено
   *[other] { $count } карты
}
corkboard-move-into-self = Контейнер не может быть перемещен внутрь себя. Выберите пункт назначения за его пределами.
corkboard-move-failed = Эти карты не могли быть перенесены туда.

overview-col-title = Заголовок
overview-col-type = Тип
overview-col-label = Подпись
overview-col-tags = Метки
overview-col-books = Книги
overview-col-own-words = Слова
overview-col-total-words = Итого
overview-row-count = { $count ->
    [one] { $count } пункт
    [few] { $count } пункта
    [many] { $count } элементов
   *[other] { $count } единицы
}
overview-search-placeholder = Фильтровые элементы …
overview-expand-all = Расширять все
overview-collapse-all = Свернуть всё
overview-table-label = Содержание
overview-empty-title = Пока ничего нет.
overview-gone-title = Этот контейнер пропал.
overview-gone-hint = Его перенесли в мусорное ведро, либо восстановили, либо закрыли эту вкладку.
overview-empty-hint = Используйте «＋ New» выше, чтобы добавить первый.
corkboard-grid-label = Картонные карточки
corkboard-rename-field = Переименовать элемент
corkboard-expand-synopsis = Расширить синопсис
corkboard-synopsis-modal-title = Синопсис
corkboard-badge-scene = Сцена
corkboard-badge-chapter = Глава
corkboard-badge-part = Часть
corkboard-badge-book = Книга
corkboard-badge-note = Заметка
corkboard-badge-folder = Папка
corkboard-badge-paratext = Паратекст
corkboard-badge-text = Текст
corkboard-badge-end = Конец
settings-group-corkboard-layout = укладка
settings-group-corkboard-cards = Карты
corkboard-show-word-count = Показать количество слов на карточках
text-heading = Текст
no-content = Этот элемент не имеет редактируемого контента.
untitled = Без названия
placeholder-title = Заголовок…
placeholder-subtitle = Подзаголовок…
placeholder-chapter-title = Глава …
split-editor = Редактор Split
close-split-view = Близкое разделение
drop-open-here = Открыть здесь
drop-open-to-side = Открывай в сторону

segment-chapter = Глава
segment-part = Часть
segment-book = Книга
segment-pace = Темп
segment-notes = Заметки
segment-story-bible = Энциклопедия истории
segment-note-own = Заметка
segment-note-details = Подробности
segment-note-in-prose = В прозе
pace-placeholder = Планировщик Пейса появляется здесь.
pace-empty-title = Планируйте темп этой книги
pace-empty-body = Установите цель слова и крайний срок, и Skribisto выработает ежедневный темп, чтобы доставить вас туда.
pace-start-planning = Начать планирование
pace-section-schedule = Расписание
pace-section-progress = Прогресс
pace-advancement = Улучшение положения
pace-goal = Цель в словах
pace-deadline = Срок
pace-active = План работы активен
pace-day-mon = пн
pace-day-tue = вт
pace-day-wed = ср
pace-day-thu = чт
pace-day-fri = пт
pace-day-sat = сб
pace-day-sun = вс
pace-card-written = Слова, написанные
pace-card-of-goal = Для цели
pace-card-rate = День письма / Writing Day
pace-card-days-left = Письменные дни остались
pace-card-streak = дневная полоса
pace-card-ahead = Слова опережают темп
pace-card-behind = Слова в темпе
pace-charts-empty = Графики прогресса появляются здесь, как только ваше количество слов записывается на сейв.
pace-chart-progression = Написанные слова vs. целевой
pace-chart-words-per-day = Слова в день
pace-series-actual = Фактический
pace-series-target = Цель
pace-series-words-per-day = Слова/день
pace-daily-target-line = Даже темп: { $count ->
    [one] { $count } слово/день
    [few] { $count } слова/день
    [many] { $count } слов/день
   *[other] { $count } слова/день
}
pace-section-holidays = Каникулы
pace-section-milestones = Вехи
pace-holidays-none = Никаких праздников, каждый запланированный день считается.
pace-holiday-label = Название праздника
pace-add-holiday = Добавить
pace-remove = Убрать
pace-milestones-none = Пока никаких вех. Установите один на часть или главу в Инспекторе.
full-chapter = Полная глава
full-part = Полная часть
full-book = Полная книга
full-synopsis = Полный синопсис
rename = Переименовать
set-label = Установить этикетку
insert-scene = Включить сцену
insert-chapter = Включить главу
split-scene = Разделённая сцена
move-up = Вставай.
move-down = Спускайся.
merge-with-previous = Слияние с предыдущим
move-to-trash = Двигаться до мусора
rename-chapter = Переименовать главу
add-scene = Добавить новую сцену
add-chapter = Добавить новую главу
new-scene-title = Новая сцена
placeholder-scene-name = Оригинальное название: …
menu-cut = Сокращение
menu-copy = Копия
menu-paste = паста
menu-paste-unformatted = Паста неформатированная
menu-select-all = Выберите все
format-bold = Полужирный
format-italic = Курсив
format-underline = Подчёркнутый
format-strikethrough = Зачёркнутый

binder = Навигатор
no-work = Нет работы заряженной
no-recent-works = Недавние работы
switcher-open-section = В настоящее время открыт
switcher-recent-section = недавнее
switcher-this-window = Это окно
open-project-title = Открытый проект
open-project-question = Как открыть «{ $title }»?
open-in-new-window = Открыть в новом окне
open-here = Открыть здесь

binder-all = Все навигаторы
binder-show-all = Показать все связующие
binder-new = Новое связующее …
binder-default-name = Новое связующее вещество
binder-item-count = { $count ->
    [one] { $count } пункт
    [few] { $count } пункта
    [many] { $count } элементов
   *[other] { $count } единицы
}
binder-search-placeholder = Фильтр схемы …
binder-search-scope = Поиск всех связующих
binder-trash-confirm-title = Переместить связующее в мусор?
binder-trash-confirm-text = «{ $name }» и все его предметы будут перемещены в мусорный бак.

dialog-rename = Переименовать
dialog-new-binder = Новое связующее вещество
dialog-set-label = Установить этикетку
dialog-new-scene = Новая сцена
close-work-question = Сохранить изменения перед закрытием работы.
quit-question = Сохранить изменения перед уходом?
quit-save-work-question = Сохранить изменения до { $title } перед уходом?
unsaved-changes = Эта работа имеет несохранившиеся изменения.
new-work-unsaved-question = Сохранить изменения перед созданием новой работы.
open-work-unsaved-question = Сохранить изменения перед открытием новой работы.
switch-backup-discard-title = Откажитесь от изменений в этой резервной копии?
switch-backup-discard-text = Изменения в резервной копии не могут быть сохранены. Используйте Save As или Restore, чтобы сохранить их, или откажитесь от них и откройте другую работу.
switch-save-failed = Работу не удалось спасти, поэтому она не была заменена: { $error }
switch-save-not-started = Работу не удалось спасти, поэтому она не была заменена.
close-save-failed = Работу не удалось спасти, поэтому она не была закрыта: { $error }
close-save-not-started = Работу не удалось спасти, поэтому она не была закрыта.
save-not-started = Работу не удалось спасти.

could-not-open-work = Не удалось открыть "{ $file }"
could-not-open-work-too-new = "{ $file }" нуждается в новом Skribisto
could-not-open-work-too-new-detail = Сохраняется форматом Skribisto { $written_by }; для его открытия требуется формат { $requires } или более новый, и эта сборка поддерживает формат { $supported }. Обновление Skribisto для его открытия.
could-not-open-work-importing = "{ $file }" пишется импортным
could-not-open-work-importing-detail = Откройте его, как только импорт будет завершен: откройте сейчас, это покажет проект, импорт заменяется, и его следующая экономия напишет этот проект обратно над импортом.
could-not-open-example = Не удалось открыть пример: { $error }
could-not-open-link = Не удалось открыть { $url }: { $error }
could-not-create-work = Не удалось создать работы: { $error }
saving-as-file = Экономия как { $target }…
saving-as-folder = Экономия как { $target }/…
saved-as = Спасено до { $target }
save-error = Не удалось спасти: { $error }
save-flush-failed = Спасение прошло, но эти документы не могли быть переданы ему и до сих пор не спасены: { $titles } ({ $error })
backup-error = Не удалось: { $error }
backing-up = Резервное копирование …
backup-nothing-open = Ни один проект не открыт для поддержки.
backup-already-running = Резервная копия уже в процессе.
backup-complete = Полное резервное копирование ({ $ok } сохранено, { $skipped } уже включено)
backup-partial = Резервное копирование завершено: { $ok } сохранено. { $failed ->
    [one] { $failed } место назначения провалилось
    [few] { $failed } Путешествия потерпели неудачу
    [many] { $failed } направлений провалились
   *[other] { $failed } Неудачные направления
}
backup-no-destination-title = Нет места резервного копирования
backup-no-destination-text = Ни один из сконфигурированных пунктов резервного копирования не может быть достигнут (например, внешний диск может быть отключен). Включите его и повторите или выйдите без резервного копирования.
backup-failed-close-title = Резервное копирование не удалось спасти
backup-failed-close-text = Ни одна резервная копия не может быть написана до закрытия. Каждый пункт назначения вышел из строя (привод, возможно, был удален, или он может быть полным или защищенным от записи). Исправьте его и повторите или выйдите без резервного копирования.

backup-choice-title = Файл резервного копирования
backup-choice-heading = Вы открыли резервную копию
backup-choice-subtitle = Это резервная копия проекта в момент времени.
backup-choice-subtitle-dated = Резервное копирование { $date }.
backup-choice-body = Вы можете открыть его и свободно редактировать, но изменения могут быть сохранены только с помощью Save As. Оригинальный файл проекта не затрагивается. Или восстановить этот проект именно в эту резервную копию.
backup-choice-open = Откройте резервную копию
backup-choice-restore = Восстановить этот проект до этой точки …
backup-choice-not-a-backup = Нет, открой нормально.
backup-banner-title = Резервная копия: изменения здесь не сохранить
backup-banner-description = Используйте Save As, чтобы сохранить ваши правки в новом файле, или Restore, чтобы заменить оригинальный проект этой резервной копией.
backup-banner-restore = Восстановить…
backup-banner-save-as = Сохранить: …
backup-restore-original-missing = Не могу найти оригинальный проект для восстановления. Используйте Save As, чтобы сохранить эту резервную копию в качестве нового проекта.
backup-restore-close-elsewhere-title = Проект открывается в другом окне
backup-restore-close-elsewhere-text = Проект, который вы восстанавливаете, открыт в другом окне. Пожалуйста, сначала закройте его, а затем повторите.
backup-restore-focus-window = Сосредоточьте это окно
backup-restore-confirm-title = Восстановить резервную копию?
backup-restore-confirm-text = Текущая версия проекта будет скопирована в качестве резервной копии, прежде чем будет заменена на эту.
backup-restore-confirm-ok = Восстановить
backup-restore-error = Не удалось восстановить: { $error }
backup-restored-ok = Проект восстановлен.
backup-restored-with-safety = Ваша предыдущая версия была сохранена до { $path }.
close-backup-discard-title = Откажитесь от изменений в этой резервной копии?
close-backup-discard-text = Изменения в резервной копии не могут быть сохранены. Используйте Save As, чтобы сохранить их, или отбросить и закрыть.
quit-backup-discard-title = Откажитесь от изменений и бросьте?
quit-backup-discard-work-question = Откажитесь от изменений до { $title } и прекратите?
quit-backup-discard-text = Изменения в резервной копии не могут быть сохранены. Используйте Save As, чтобы сохранить их, или отбросьте и бросьте.
backup-nudge-text = Никаких резервных копий для этого проекта не создано.
backup-nudge-action = Настройка резервных копий …

menu-backups-list = &Резервное копирование …
backups-title = Резервное копирование
backups-loading = Поиск резервных копий…
backups-empty = Резервных копий для этого проекта пока не найдено.
backups-open = Открыть
backups-reveal = Показать расположение
backups-delete = Удалите эту резервную копию
backups-delete-confirm-title = Удалить резервную копию?
backups-delete-confirm-text = "{ $name }" будет навсегда удалена.
backups-delete-error = Не удалось удалить резервную копию: { $error }
backups-refresh = Обновить
backups-close = Закрыть

settings-page-backup = Дефолты резервного копирования
settings-page-work-backup = Резервное копирование
settings-backup-general-title = Параметры резервного копирования по умолчанию
settings-backup-work-title = Резервное копирование для этого проекта
settings-backup-inherit = Используйте общие настройки резервного копирования
settings-backup-inheriting = Этот проект использует общие настройки резервного копирования.
settings-backup-none-hint = Автоматическое резервное копирование не настраивается (выключается каждый триггер).
settings-backup-last = Последнее резервное копирование: { $date }
settings-backup-last-never = Никаких резервных копий.
settings-backup-open-list = Открытый список резервных копий …
settings-backup-triggers = Когда отступать
settings-backup-on-close = Резервное копирование при закрытии проекта
settings-backup-on-open = Назад при открытии проекта
settings-backup-interval = Регулярно, каждый
settings-backup-destinations = Резервные пункты назначения
settings-backup-default-location = Местоположение по умолчанию
settings-backup-usage-since = { $count ->
    [one] { $count } резервный · { $size } · старейший { $oldest }
    [few] { $count } резервных копии · { $size } · старейший { $oldest }
    [many] { $count } резервных копий · { $size } · старейший { $oldest }
   *[other] { $count } резервные копии · { $size } · старейший { $oldest }
}
settings-backup-usage = { $count ->
    [one] { $count } резервная копия · { $size }
    [few] { $count } резервных копии · { $size }
    [many] { $count } резервных копий · { $size }
   *[other] { $count } резервные копии · { $size }
}
settings-backup-usage-empty = Ничего не сохранилось здесь до сих пор
settings-backup-usage-measuring = Измерение …
settings-backup-reveal-root = Показать папку
settings-backup-dest-none = Резервные копии сохраняются рядом с проектом.
settings-backup-dest-remove = Убрать
settings-backup-dest-add = Добавить папку …
settings-backup-dest-refresh = Обновить
settings-backup-retention = Сколько оставить
settings-backup-retention-tiered = По уровням
settings-backup-retention-keep-n = Сохраните последний N
settings-backup-retention-tip = Как обрезаны старые резервные копии.
settings-backup-retention-tip-more = Tiered хранит одну резервную копию в час в течение дня, в день в течение недели, в неделю в течение месяца и в месяц за его пределами, поэтому недавняя история остается плотной, а старая история истончается. «Keep last N» просто сохраняет последние копии N. В обоих режимах всегда хранятся новейшие копии (минимум ниже).
settings-backup-gfs-hourly = Часовой (последние 24 часа)
settings-backup-gfs-daily = Daily (последняя неделя)
settings-backup-gfs-weekly = Еженедельно (последний месяц)
settings-backup-gfs-monthly = ежемесячно
settings-backup-keep-n = Номер, чтобы сохранить
settings-backup-min-keep = Всегда держите хотя бы
settings-backup-dedup = Пропустить резервную копию, когда ничего не изменилось

new-work-title = Новый проект
new-work-close = Закрыть
new-work-name = Название проекта
new-work-name-placeholder = Без названия
new-work-author = Автор
new-work-author-placeholder = необязательный
new-work-format = Формат
new-work-single-file = Один файл
new-work-bundle = Папка проекта
new-work-convert-later = Вы можете конвертировать между форматами позже.
new-work-location = Место
new-work-will-create = будет создавать
new-work-language = Язык по умолчанию
new-work-language-hint = Применительно к новым текстам проверка орфографии. Каждый текст можно переключить на другой язык.
new-work-template = Шаблон
new-work-template-none = Никто.
new-work-template-empty-novel = Пустой роман
new-work-template-light-novel = Световой роман
new-work-template-novel = роман
new-work-template-novel-in-parts = Роман по частям
new-work-template-notebook = Блокнот
new-work-cancel = Отмена
new-work-create = Создать проект
new-work-step-details = Подробности
new-work-step-language = Языковая структура
new-work-step-template = Шаблон
new-work-back = Назад
new-work-next = Далее
new-work-documents-title = Новый проект из документов
new-work-step-import = Импорт
new-work-create-and-import = Разработчик: Import…
new-work-documents-next-title = Ваши документы будут следующими
new-work-documents-next-body = Создание проекта открывает маг импорта над ним, где вы выбираете документы, просматриваете структуру Skribisto, читаете из них, и выбираете, где он приземляется.Ничего не записывается в проект, пока вы не подтвердите его там.
new-work-documents-no-template = Этот проект начинается пустым по назначению: никакого шаблона, поэтому ввозимые документы - единственное, что в нем есть.
new-work-documents-chapter-scene-hint = Применяется к главам, которые создает импорт. Вы можете изменить его позже в настройках проекта.
new-work-single-file-desc = Один архив .skrib (zip). Портативный, простой в резервном копировании.
new-work-bundle-desc = Папка с каждым текстовым активом, более дружественная к управлению версиями.
new-work-template-none-count = пустой связующий
new-work-template-empty-novel-count = 1 глава
new-work-template-light-novel-count = 15 глав
new-work-template-novel-count = 20 глав
new-work-template-novel-in-parts-count = 3 части, 24 главы
new-work-template-notebook-count = ноты свободной формы
new-work-chapter-scene = Главы без сцен
new-work-chapter-scene-tip = Каждая глава - это отдельный элемент, в который вы пишете прямо, без сцен под ним. Оставьте это для классического макета, где глава - папка: вы пишете прямо в нее тоже, но она может содержать сцены.
new-work-chapter-scene-tip-more = Разница только в том, может ли он содержать сцены. Дерево связующего Skribisto является только организационным, поэтому оба макета компилируются в одну и ту же книгу, вы можете свободно смешивать их и продвигать преобразование главы между ними без потери слова.
new-work-name-required = Введите имя для работы
new-work-name-invalid = Это имя не имеет полезных символов.
new-work-location-required = Выберите местоположение
new-work-location-missing = Такой папки не существует
new-work-location-not-folder = Этот путь не является папкой.
new-work-location-readonly = Эта папка не пишется
new-work-target-exists = Проект с таким названием уже есть в этой папке.Создание работы спросит перед ее заменой
new-work-target-is-folder = Папка с таким именем уже в этом месте. Выберите другое имя.
new-work-overwrite-title = Заменить существующий проект?
new-work-overwrite-text = "{ $name }" уже существует в этой папке. Заменить ее новым проектом?
new-work-target-open-title = Этот проект открыт
new-work-target-open-text = «{ $name }» открыт в Skribisto. Создание нового проекта на его месте заменило бы проект, над которым вы работаете, и ваш следующий сохранения напишет этот проект обратно над новым. Выберите другое имя или папку, или закройте проект первым.
target-importing-title = Импорт пишет этот проект.
target-importing-text = Импорт все еще пишет "{ $name }". Подождите, пока он не закончит, или выберите другое имя.
import-plume-busy-title = Еще один проект Plume Creator импортируется
import-plume-busy-text = Проекты Plume Creator импортируются по одному за раз. Подождите, пока импорт не закончится, или отмените его, затем начните этот снова.
import-manuskript-busy-title = Еще один проект Manuskript импортируется
import-manuskript-busy-text = Проекты Manuskript импортируются по одному за раз. Подождите, пока импорт не закончится, или отмените его, затем начните этот снова.
backup-restore-importing-text = Импорт все еще пишет "{ $name }", проект эта резервная копия будет восстановлена. Восстановите резервную копию после того, как импорт закончился.

new-work-manuscript = рукопись
new-work-notes = Заметки
new-work-research = Исследования
new-work-notebook = Блокнот
new-work-scene = Сцена
new-work-note = Заметка
new-work-front-matter = Передний вопрос
new-work-back-matter = Обратная сторона вопроса
new-work-paratext = Структура книги
new-work-paratext-none = Никакой структуры
new-work-paratext-hint = Спереди и сзади традиция открывает и закрывает книгу. Вы можете переместить, переименовать или удалить любую из них позже.
new-work-tags = Метки
new-work-tags-none = Никаких тегов
new-work-tags-hint = Стартовая палитра для маркировки символов, мест и прочего.Необязательно, и каждый тег может быть переименован, перекрашен или удален впоследствии.
new-work-note-templates = Заметить шаблоны
new-work-note-templates-none = Никаких шаблонов
new-work-note-templates-hint = Форма примечания начинается с заметки "история-библия". Необязательно, и каждый шаблон может быть отредактирован или удален позже.
new-work-characters = Персонажи
new-work-places = Места

import-plume-title = Импорт Plume Creator проект
import-plume-close = Закрыть
import-plume-source = Проект Plume
import-plume-source-hint = Выберите файл .plume или .plume_backup (любая версия Plume Creator).
import-plume-location = Папка назначения
import-plume-name = Файловое имя
import-plume-name-placeholder = Название проекта
import-plume-will-create = будет создавать
import-plume-trash-warning = Трэшированные / удаленные элементы не мигрируют.
import-plume-cancel = Отмена
import-plume-import = Импорт
import-plume-source-required = Выберите файл проекта Plume
import-plume-source-missing = Этот файл не существует
import-plume-source-not-file = Этот путь не является файлом
import-plume-location-required = Выберите папку назначения
import-plume-location-missing = Такой папки не существует
import-plume-location-not-folder = Этот путь не является папкой.
import-plume-location-readonly = Эта папка не пишется
import-plume-name-required = Введите имя файла
import-plume-name-exists = Файл с этим именем уже существует. Импорт подтвердит перезапись
import-plume-overwrite-title = Заменить существующий файл?
import-plume-overwrite-text = "{ $name }" уже существует. Заменить его на импортный проект?
import-plume-manuscript-binder = рукопись
import-plume-story-bible-binder = История Библии
import-plume-progress-title = Импорт Plume project…
import-plume-cancel-import = Отмена
import-plume-cancelled = Импорт отменили
import-plume-done = импортный { $imported ->
    [one] { $imported } пункт.
    [few] { $imported } предмета.
    [many] { $imported } предметов.
   *[other] { $imported } единиц.
} { $skipped ->
    [one] { $skipped } пункт обвалов не был вывезен.
    [few] { $skipped } разбитых предмета не были вывезены.
    [many] { $skipped } разбитых предметов не были вывезены.
   *[other] { $skipped } товарные позиции не были вывезены.
}
import-plume-open-now = Открыто сейчас
import-plume-warnings = { $count ->
    [one] { $count } вещь нельзя было импортировать точно
    [few] { $count } Вещи нельзя было импортировать точно
    [many] { $count } вещей, которые нельзя импортировать точно
   *[other] { $count } Вещи нельзя было импортировать точно
}
import-plume-details = Подробности
import-plume-warnings-title = Предупреждения об импорте
import-plume-error-title = Не удалось импортировать проект
import-plume-nested-too-deep = Проект не был импортирован: его часть «{ $part }» вложена более чем в { $limit } уровней глубины. No Plume Creator проект приближается к этому, и чтение его может привести к краху Skribisto. Ничего не было написано.
import-plume-declares-entities = Проект не был импортирован: его часть «{ $part }» декларирует объекты XML, чего не делает проект Plume Creator. Расширение их может занять больше памяти, чем у компьютера, и сбой Skribisto. Ничего не было написано.
import-plume-error-details = Подробности

import-manuskript-title = Импорт Manuskript проект
import-manuskript-close = Закрыть
import-manuskript-source = Проект Manuskript
import-manuskript-source-hint = Выберите файл .msk или папку проекта (любая версия Manuskript).
import-manuskript-source-file = Выберите файл …
import-manuskript-source-folder = Выберите папку …
import-manuskript-location = Папка назначения
import-manuskript-name = Файловое имя
import-manuskript-name-placeholder = Название проекта
import-manuskript-will-create = будет создавать
import-manuskript-cancel = Отмена
import-manuskript-import = Импорт
import-manuskript-source-required = Выберите проект Manuskript
import-manuskript-source-missing = Этот файл или папка не существует.
import-manuskript-location-required = Выберите папку назначения
import-manuskript-location-missing = Такой папки не существует
import-manuskript-location-not-folder = Этот путь не является папкой.
import-manuskript-location-readonly = Эта папка не пишется
import-manuskript-name-required = Введите имя файла
import-manuskript-name-exists = Файл с этим именем уже существует. Импорт подтвердит перезапись
import-manuskript-overwrite-title = Заменить существующий файл?
import-manuskript-overwrite-text = "{ $name }" уже существует. Заменить его на импортный проект?
import-manuskript-manuscript-binder = рукопись
import-manuskript-story-bible-binder = Энциклопедия истории
import-manuskript-characters-group = Персонажи
import-manuskript-world-group = Мир
import-manuskript-plots-group = Заговоры
import-manuskript-project-info-note = Информация о проекте
import-manuskript-summary-note = Резюме
import-manuskript-importance-minor = несовершеннолетний
import-manuskript-importance-secondary = вторичный
import-manuskript-importance-main = Основной
import-manuskript-progress-title = Импорт Manuskript проект …
import-manuskript-cancel-import = Отмена
import-manuskript-cancelled = Импорт отменили
import-manuskript-done = { $imported ->
    [one] { $imported } Товар импортный.
    [few] { $imported } Ввозимые товары.
    [many] { $imported } Товары импортные.
   *[other] { $imported } Товары импортные.
} { $revisions ->
    [0] { "" }
    [one] С ним поступила { $revisions } ранняя версия.
    [few] С ними пришли { $revisions } более ранние версии.
    [many] С ними вышло { $revisions } более ранних версий.
   *[other] С ними поступило { $revisions } более ранних версий.
}
import-manuskript-open-now = Открыто сейчас
import-manuskript-warnings = { $count ->
    [one] { $count } Что нужно знать об этом импорте
    [few] { $count } вещи, которые нужно знать об этом импорте
    [many] { $count } вещей, которые нужно знать об этом импорте
   *[other] { $count } Что нужно знать об этом импорте
}
import-manuskript-details = Подробности
import-manuskript-warnings-title = Об этом импорте
import-manuskript-error-title = Не удалось импортировать проект
import-manuskript-nested-too-deep = Проект не был импортирован: его файл «{ $part }» вложен более чем в { $limit } уровней глубины. No Manuskript проект приближается к этому, и чтение его может привести к краху Skribisto. Ничего не было написано.
import-manuskript-folders-too-deep = Проект не был импортирован: один из его файлов содержит более { $limit } папок в глубину. No Manuskript проект приближается к этому, и чтение его может привести к сбою Skribisto. Ничего не было написано.
import-manuskript-declares-entities = Проект не был импортирован: его файл «{ $part }» заявляет о XML-сущностях, чего не делает проект Manuskript. Расширение их могло занять больше памяти, чем у компьютера, и сбой Skribisto. Ничего не было написано.
import-manuskript-error-details = Подробности

import-target-open-title = Этот проект открыт
import-target-open-text = "{ $name }" открыт в Skribisto. Импорт над ним заменит проект, над которым вы работаете, и ваше следующее сохранение напишет этот проект обратно над импортом. Выберите другое имя файла или закройте проект сначала.
import-warnings-announcement = { $result } { $notice }.

export-title = Экспорт
export-close = Закрыть
export-scope-label = Что?
export-format-label = Формат
export-style-label = Стиль
export-path-label = Файлы
export-browse = Обзор…
export-show-non-exportable = Показать не экспортируемый
export-choose-empty = Нет проекта на выбор.
export-cancel = Отмена
export-export = Экспорт
export-save-dialog-title = Экспорт в файл
export-section-what = Что экспортировать
export-section-style = Стиль
export-section-destination = Направление
export-custom-selection = Пользовательский подбор
export-selected-count = { $count } выбранный
export-chip-chapters-none = Главы: нет
export-chip-chapters-numbered = Главы: нумерованные
export-chip-chapters-title = Главы: только название
export-chip-chapters-both = Главы: номер + название
export-chip-scene-break-glyph = Сценический перерыв: { $glyph }
export-chip-scene-break-blank = Сцена разрыва: пустая линия
export-chip-scene-break-none = Сценический прорыв: нет
export-chip-major-break-glyph = Основной прорыв { $glyph }
export-chip-major-break-blank = Основной прорыв: пустая линия
export-chip-major-break-none = Главный прорыв: нет
export-chip-major-break-same = Оба разбивают ярусы одинаково
export-chip-spacing-single = Интервал: одинарный
export-chip-spacing-onehalf = Пространство: 11⁄2
export-chip-spacing-double = Пространство: двойное
export-chip-notes-included = Включенные примечания
export-chip-notes-excluded = Примечания исключены
export-format-docx = Слово
export-format-odt = LibreOffice
export-format-html = HTML
export-format-markdown = Markdown
export-format-djot = Djot
export-format-text = Текст
export-format-latex = LaTeX
export-format-epub = EPUB
export-format-pdf = PDF
export-overwrite-title = Заменить существующий файл?
export-overwrite-text = "{ $name }" уже существует.
export-progress-title = Экспорт …
export-cancelled = Экспорт отменили
export-done = экспортируемый { $count ->
    [one] { $count } пункт
    [few] { $count } пункта
    [many] { $count } элементов
   *[other] { $count } единицы
}
export-open-file = Открыть
export-show-in-folder = Показать в папке
export-error-title = Нельзя экспортировать
export-error-details = Подробности

backup-progress-start = Запуск…
backup-progress-retention = Очистка старых резервных копий …
backup-progress-done = Готово
backup-progress-destination = Направление { $i } из { $n }
backup-details = Подробности
backup-issues-title = Вопросы резервного копирования
backup-failed-title = Резервное копирование провалилось
backup-complete-prune-warning = Полное резервное копирование ({ $ok } сохранено, { $skipped } уже включено). Некоторые старые резервные копии не могут быть удалены.

search = Поиск
search-query-placeholder = Поиск…
search-replace-placeholder = Заменить на …
search-replace-toggle = Замена Toggle
search-replace-all = Заменить всё
search-opt-case = Случай матча
search-opt-whole-word = Целое слово
search-opt-diacritics = Совместные акценты
search-scope-body = Текст
search-scope-title = Заголовок
search-scope-synopsis = Синопсис
search-scope-label = Подпись
search-facet-book = Книги
search-facet-part = Части
search-facet-chapter = Главы
search-facet-scene = Сцены
search-facet-note = Заметки
search-facet-folder = Складки
search-tip-case = Матч-кейс: относитесь к верхнему и нижнему регистрам как к разным, поэтому «Елена» и «Елена» :  это отдельные матчи.
search-tip-whole-word = Целое слово: сопоставляйте только полные слова, так что «кошка» не находится внутри «категории».
search-tip-diacritics = Совместные акценты: относитесь к акцентированным буквам как к отдельным, поэтому «кафе» не соответствует «кафе».
search-tip-body = Тело: поиск прозы сцен и заметок.
search-tip-title = Оригинальное название: Search the titles of binder items
search-tip-synopsis = Синопсис: ищите резюме каждого пишущего пункта.
search-tip-label = Ярлык: ищите короткую заметку, показанную под заголовком связующего элемента.
search-tip-comment = Комментарии: ищите текст строк комментариев и их ответы. Комментарий - это о рукописи, а не ее части, поэтому у нее есть свой собственный переключатель, и замена оставляет комментарии не тикающими, пока вы их не пометите.
search-tip-book = Книги: книжный контейнер и его маркеры начала/конца.
search-tip-part = Части: разделители на уровне частей.
search-tip-chapter = Главы: главы, в зависимости от того, каким образом проект их хранит.
search-tip-scene = Сцены: предметы, которые держат прозу.
search-tip-note = Примечания: примечания свободной формы.
search-tip-folder = Папки: простые организующие папки и разделители.
search-tip-paratext = Текст, который принадлежит к книге, но не к ее истории: предисловие, посвящение, послесловие. Никогда не считался в рукописи.
search-tip-preserve-case = Запасной случай: замена берет найденный случай, поэтому «Элена» становится «Мартой», а «Елена» :  «Мартой».
search-tip-replace = Замените: покажите поле замены и замените все.
search-error = Поиск не удался: { $message }
search-no-matches = Никаких матчей
search-count =
    { $matches ->
    [one] { $matches } матч
    [few] { $matches } матча
    [many] { $matches } матчей
   *[other] { $matches } матчи
    } в { $items ->
    [one] { $items } документ
    [few] { $items } Документы
    [many] { $items } Документы
   *[other] { $items } Документы
    }
search-count-truncated =
    { $matches ->
    [one] { $matches } матч
    [few] { $matches } матча
    [many] { $matches } матчей
   *[other] { $matches } матчи
    } в { $items ->
    [one] { $items } документ
    [few] { $items } Документы
    [many] { $items } Документы
   *[other] { $items } Документы
    } (только первые результаты)
search-occurrences = { $count }
search-field-body = Текст
search-field-title = Заголовок
search-field-synopsis = Синопсис
search-field-label = Подпись
search-field-epigraph = Эпиграф
search-field-comment = Комментарий
search-field-comment-reply = Ответить
search-field-footnote = Сноска
search-include-in-replace = Включите в замену все
search-collapse-all = Крах всех результатов
search-replace-here = Заменить это
search-dismiss = Увольнение из результатов
search-undo-dismiss = Верните последний отклоненный результат
search-replace-nothing = (ничего)
search-replace-confirm-title = Заменить все совпадения?
search-replace-confirm-text =
    Заменить { $occurrences ->
    [one] { $occurrences } случайность
    [few] { $occurrences } события
    [many] { $occurrences } случаев
   *[other] { $occurrences } случаев
    } из "{ $query }" с "{ $replacement }" в поперечнике { $items ->
    [one] { $items } документ
    [few] { $items } Документы
    [many] { $items } Документы
   *[other] { $items } Документы
    }, вы можете отменить это после уведомления.
search-replace-done-title = Заменить полностью
search-replace-done =
    { $occurrences ->
    [one] { $occurrences } случайность
    [few] { $occurrences } события
    [many] { $occurrences } случаев
   *[other] { $occurrences } случаев
    } заменен на { $items ->
    [one] { $items } документ
    [few] { $items } Документы
    [many] { $items } Документы
   *[other] { $items } Документы
    }.
search-replace-done-skipped =
    { $occurrences ->
    [one] { $occurrences } случайность
    [few] { $occurrences } события
    [many] { $occurrences } случаев
   *[other] { $occurrences } случаев
    } заменен на { $items ->
    [one] { $items } документ
    [few] { $items } Документы
    [many] { $items } Документы
   *[other] { $items } Документы
    }. { $skipped ->
    [one] { $skipped } поле
    [few] { $skipped } поля
    [many] { $skipped } Полей
   *[other] { $skipped } поля
    } пропущен (изменен с момента поиска).
search-replace-undo = Отменить
search-replace-failed-title = Замена неудавшегося
search-replace-skipped-title = Ничто не заменяет
search-replace-skipped-body =
    { $fields ->
    [one] Этот текст
    [few] { $fields } из этих текстов
    [many] { $fields } из этих текстов
   *[other] { $fields } этих текстов
    } изменился с момента поисков, так что { $fields ->
    [one] Это было
    [few] Они были
    [many] Они были
   *[other] Они были
    } оставили в покое. Поищите снова, чтобы увидеть, где слова сейчас.
search-replace-undo-failed-title = Неудачный Undo
search-preview = Предварительный просмотр
search-preview-empty = Выберите результат, чтобы просмотреть его здесь
search-preview-no-prose = Этот матч не имеет редактируемого текста.
search-preview-prompt = Чтобы увидеть предварительный просмотр здесь, запустите поиск.
search-preview-open-search = Поиск в проекте
search-preview-footnote-prompt = Это совпадение в тексте сноски. Откройте его в доке сносок, чтобы увидеть и отредактировать.
search-preview-open-footnotes = Открытые сноски

find-placeholder = Найти в этом документе …
find-count = { $current } из { $total }
find-no-results = Нет результатов
find-close = Близкий поиск
find-previous = Предыдущий матч (Shift+Enter)
find-next = Следующий матч (вход)
find-opt-case = Случай матча
find-opt-whole-word = Целое слово
find-replace-toggle = Показать или скрыть замену (Ctrl+R)
find-replace-placeholder = Заменить на …
find-replace = Заменить
find-replace-all = Заменить всё
find-preserve-case = Сохранить дело

dict-download-title = Скачать { $name }…
dict-download-done = Установка { $name }
dict-download-failed = Не удалось скачать { $name }: { $error }
dict-removed = Снято { $name }
dict-accept-first = Принять лицензию перед загрузкой { $name }
settings-dict-tab-installed = установленный
settings-dict-tab-get-more = Загрузить ещё
settings-dict-tab-personal = Личные слова
dict-installed-empty = Словарей на этом компьютере пока не найдено.
dict-get-more-search = Языки поиска
dict-system-badge = В вашей системе
dict-unusable-badge = непригодный для использования
dict-download-button = скачать
dict-downloading = Скачать …
dict-installed-label = установленный
dict-remove = Убрать
dict-view-license = Просмотр лицензии
dict-approx-size = ~{ $size }
dict-personal-empty = Никаких личных слов в этом проекте пока нет.
dict-personal-add = Добавить
dict-personal-placeholder = Добавить слово …
dict-missing-toast = { $count ->
    [one] В этом проекте используется { $count } словарь, который вы не установили
    [few] Этот проект использует { $count } словаря, которые вы не установили.
    [many] Этот проект использует { $count } словарей, которые вы не установили.
   *[other] В этом проекте используется { $count } словарей, которые вы не установили.
}
dict-missing-action = Получить словари
dict-license-title = { $name } Лицензия
dict-license-accept = Принять Download
dict-license-cancel = Отмена
dict-license-close = Закрыть
dict-add-button = Добавить словарь …
dict-add-title = Добавить словарь
dict-add-close = Закрыть
dict-add-name = Название
dict-add-name-placeholder = Например, мой латинский словарь
dict-add-code = Языковой код
dict-add-code-placeholder = например, la или fr-FR-x-custom
dict-add-code-hint = Короткий тег по вашему выбору. Это то, что вы выбираете в качестве языка документа.
dict-add-aff = Файл аффиксов (.aff)
dict-add-dic = Список слов (.dic)
dict-add-submit = Добавить
dict-add-cancel = Отмена
dict-add-name-required = Дайте словарю имя.
dict-add-code-required = Введите языковой код.
dict-add-code-invalid = Используйте только буквы, цифры и - .
dict-add-code-reserved = Это встроенный словарный код. Выберите другой.
dict-add-file-required = Выберите файл.
dict-add-file-missing = Этот файл не существует.
dict-add-code-taken = Словарь для этого кода уже установлен.
dict-add-done = Добавлено { $name }
dict-add-unusable = Эти файлы не являются используемым словарем: { $error }
dict-add-failed = Не могу добавить словарь: { $error }
inspector-export = Экспорт
inspector-exportable = Включать в экспорт
ctx-number = &Номер этой главы
ctx-unnumber = &Не пронумеруйте эту главу.
inspector-numbering = нумерация
inspector-numbered = нумерованный
inspector-numbered-tip = Эта глава занимает свое место в нумерации книги. Выключите ее для пролога, эпилога или интерлюдии.
inspector-numbered-tip-more = Ненумерованная глава остается в книге точно такой, какой она была: ее заголовок, проза и количество слов не тронуты. Она просто не печатает номер и не использует его вверх: глава после пролога - это глава первая, а не глава вторая. Выход из экспорта вместо этого - другой переключатель, выше, и что один удаляет главу из книги вообще.
inspector-apply-to-children = Применять к детям
inspector-milestone = Дата Милестоун
inspector-milestone-none = Нет даты
inspector-milestone-clear = Чисто.
inspector-tags = Метки
inspector-aliases = Другие имена
inspector-dict-language = Язык
inspector-apply-language-to-children = Применять язык к детям
settings-page-language = Язык
settings-field-dict-language = Языки
dict-tradeoff-hint = Каждый дополнительный язык принимает больше слов, поэтому ошибок становится меньше.
lang-inherit-hint = Наследственное: в этой сцене используются языки книги или проекта.
lang-pill-list = Языки
lang-pill-add = Добавить язык
lang-pill-remove = Удалить { $name }
lang-pill-mute = Отключить проверку орфографии для { $name }
lang-pill-unmute = Включите проверку орфографии для { $name }

settings-page-personal-dictionary = Личный словарь
settings-user-dict-desc = Слова, которые вы добавляете здесь, рассматриваются как правильно написанные везде в приложении и никогда не помеченные снова.
settings-user-dict-add-placeholder = Добавить слово …
settings-user-dict-add = Добавить слово
settings-user-dict-search = Фильтр слов
settings-user-dict-count = { $count ->
    [one] { $count } слово
    [few] { $count } слова
    [many] { $count } слов
   *[other] { $count } слова
}

settings-user-dict-rename = Переименовать
settings-user-dict-remove = Убрать
settings-user-dict-empty = Добавьте слова, которые проверка орфографии должна игнорировать, или импортируйте список.
settings-user-dict-import = Импорт …
settings-user-dict-export = Экспорт …
settings-user-dict-import-tip = Добавьте слова из простого текстового файла, по одному слову на строку. Существующие слова сохраняются.
settings-user-dict-export-tip = Сохраните весь список в простой текстовый файл, по одному слову в строке.
settings-user-dict-txt-filter = Текстовые файлы
settings-user-dict-duplicate = Уже в словаре
settings-user-dict-imported = импортный { $count ->
    [one] { $count } слово
    [few] { $count } слова
    [many] { $count } слов
   *[other] { $count } слов
} ({ $duplicates } уже присутствует).
settings-user-dict-import-failed = Не смог прочитать список слов: { $error }
settings-user-dict-exported = Спасенный { $count ->
    [one] { $count } слово.
    [few] { $count } слова.
    [many] { $count } слов.
   *[other] { $count } слов.
}
settings-user-dict-export-failed = Не удалось сохранить список слов: { $error }

settings-page-text-replacements = Замена текста
settings-text-repl-desc = Замените стенограмму полным текстом по мере ввода: «btw» становится «кстати» в тот момент, когда вы вводите пробел или пунктуацию.
settings-text-repl-enable = Используйте замену текста в этом проекте
settings-text-repl-disabled-hint = Включите это, чтобы определить сокращение, которое расширяется по мере написания.
settings-text-repl-add = Добавить правило
settings-text-repl-trigger-placeholder = Сокращение
settings-text-repl-replacement-placeholder = Что становится
settings-text-repl-added = Добавлено "{ $trigger }"
settings-text-repl-duplicate = "{ $trigger }" уже имеет правило
settings-text-repl-filter = Правила фильтрации
settings-text-repl-count = { $n ->
    [one] { $n } правило
    [few] { $n } Правила
    [many] { $n } Правила
   *[other] { $n } Правила
}
settings-text-repl-row-enabled = Используйте это правило
settings-text-repl-delete = Исключить правило для { $trigger }
settings-text-repl-deleted = Исключено правило "{ $trigger }"
settings-text-repl-empty = Никаких правил.
settings-text-repl-csv-filter = CSV файлов
settings-text-repl-import = Импорт …
settings-text-repl-export = Экспорт …
settings-text-repl-imported = Импортировано { $added }, пропущено { $skipped }
settings-text-repl-exported = экспортируемый { $n ->
    [one] { $n } правило
    [few] { $n } Правила
    [many] { $n } Правила
   *[other] { $n } Правила
}

editor-menu-no-suggestions = Никаких предложений
editor-menu-add-to-dictionary = Добавить «{ $word }» в словарь
editor-menu-add-words-to-dictionary = Добавить выбранные слова в словарь
editor-dict-added = Добавить «{ $word }» в словарь.
editor-dict-added-multi = { $count ->
    [one] Добавлено { $count } слово в словарь.
    [few] Добавьте { $count } слова в свой словарь.
    [many] Добавьте { $count } слов в свой словарь.
   *[other] В ваш словарь добавлено { $count } слов.
}
toast-undo = Отменить

undo-superseded-title = Этот шаг уже не последний.
undo-superseded-body = Что-то еще изменилось в этом проекте с тех пор. Ундо взял бы это обратно, поэтому этого не произошло. Ничего этого шага не было отменено. Используйте Edit ▸ Ундо, чтобы самостоятельно шагнуть назад через историю.
undo-failed = Не удалось: { $error }

menu-edit = &Правка
menu-edit-undo = &Отменить
menu-edit-redo = По&вторить
menu-edit-undo-target = &Отменить: { $target }
menu-edit-redo-target = &Повторить: { $target }
shortcut-name-edit-undo = Отменить
shortcut-name-edit-redo = Повторить
undo-target-typing = печатать
undo-target-project = Последние изменения в этом проекте
prose-history-reset-title = Скачать History Reset
prose-history-reset-body = { $count ->
    [one] { $count } открытая сцена была восстановлена из более раннего состояния, поэтому ее история типизации больше не применяется.
    [few] { $count } открытые сцены были восстановлены из более раннего состояния, поэтому их история набора текста больше не применяется.
    [many] { $count } открытых сцен были восстановлены из более раннего состояния, поэтому их история набора текста больше не применяется.
   *[other] { $count } открытые сцены были восстановлены из более раннего состояния, поэтому их история типизации больше не применяется.
}
undo-target-trash = Переезд в мусорный бак
undo-target-restore = Восстановление из мусора
undo-target-delete-forever = Удаление навсегда
undo-target-replace-all = Замена по всему проекту
undo-target-import = Документ импортирует
undo-target-duplicate = дублирование
undo-target-move = двигаться
undo-target-merge = Слияние двух сцен
undo-target-split = Раздвоение сцены
undo-target-promote = Изменение типа
undo-target-tidy-titles = Уборка заголовков глав
undo-target-import-tags = Импорт этикеток
undo-target-import-templates = Шаблоны импортных заметок
undo-target-create = создавать
undo-target-remove = удалять
undo-target-rename = переименование
undo-target-edit = редактировать
menu-edit-find = &Найти…
menu-edit-find-next = На&йди следующий
menu-edit-find-prev = Найти п&редыдущее
menu-edit-replace = Н&айти и заменить …
menu-edit-find-in-project = Найти в &Project…
menu-edit-replace-in-project = &Заменить в проекте …
undo-frozen = Undo («Всегда вперед»)
redo-frozen = (перенаправлено с «Always forward»)
menu-edit-cut = &Сокращение
menu-edit-copy = &Копия
menu-edit-paste = &паста
menu-edit-paste-plain = Пас&та без форматирования
menu-edit-select-all = В&ыберите все
shortcut-name-edit-cut = Сокращение
shortcut-name-edit-copy = Копия
shortcut-name-edit-paste = паста
shortcut-name-edit-paste-plain = Паста без форматирования
shortcut-name-edit-select-all = Выберите все

titlebar-spellcheck-on = Проверка орфографии включена. Нажмите, чтобы остановить проверку (F7)
titlebar-spellcheck-off = Проверка заклинаний отключена. Нажмите, чтобы проверить еще раз (F7)
menu-spellcheck = &Проверь правописание
menu-comments = &Комментарии
settings-page-spellcheck = Проверка орфографии
settings-group-spellcheck = Проверка орфографии
settings-spellcheck-enabled = Проверьте орфографию, как я пишу
settings-spellcheck-hint = Подчеркивает слова, которые не знает ни один установленный словарь. Отключение этого останавливает все проверки в каждом проекте, пока вы не включите его. Чтобы перестать проверять только один язык, очистите его контрольный знак на языковом поле работы или предмета.

menu-trash = &Корзина
trash-title = Корзина
trash-empty-state = Мусор пуст.
trash-empty-button = Очистить корзину…
trash-restore = &Восстановить
trash-restore-to = &Восстановление до …
trash-delete-forever = &Удалить навсегда
trash-restored-ok = { $count } восстановлено.
trash-restore-error = Не удалось восстановить: { $error }
trash-restore-orphaned = Исходное место этого предмета исчезло. Выберите, где его восстановить.
trash-restore-no-project = Ни один проект не открыт, поэтому восстанавливать нечего.
trash-delete-no-project = Ни один проект не открыт, поэтому удалять нечего.
trash-empty-confirm-title = Выбросьте мусор?
trash-empty-confirm-text = { $count ->
    [one] { $count } стрейч-вход будет навсегда удален.
    [few] { $count } разбитые записи будут навсегда удалены.
    [many] { $count } разбитых записей будут навсегда удалены.
   *[other] { $count } трэшированные записи будут навсегда удалены.
} Они оставляют мусор навсегда. Ундо все еще может вернуть их, пока вы не сделаете что-то еще.
trash-emptied-title = Выброшенный мусор
trash-emptied-body = Все, что находилось в мусорном баке, было удалено навсегда.
trash-empty-no-project = Ни один проект не открыт, поэтому нет мусора, чтобы опустеть.
trash-delete-forever-confirm-title = Удалить навсегда?
trash-delete-forever-confirm-text = { $count ->
    [one] { $count } пункт будет навсегда удален.
    [few] { $count } пункта будут удалены навсегда.
    [many] { $count } предметов будут удалены навсегда.
   *[other] { $count } элементов будут удалены навсегда.
} Они оставляют мусор навсегда. Ундо все еще может вернуть их, пока вы не сделаете что-то еще.
trash-deleted-title = Удален навсегда
trash-deleted-body = { $count } навсегда удален.
trash-undo = Отменить
trash-restore-picker-title = Восстановление до …
trash-restore-to-confirm-title = Восстановить здесь?
trash-restore-to-confirm-text = Восстановить "{ $item }" в "{ $destination }"?
trash-restore-picker-restore-here = Восстановить здесь
trash-restore-picker-cancel = Отмена
trash-restore-picker-empty = Пока никаких связующих.Создай сначала.
trash-banner-title = Этот предмет находится в мусоре
trash-banner-description = Он не будет отображаться в схеме или экспорте, пока вы не восстановите его.
trash-banner-restore = Восстановить…
trash-tab-tooltip = В мусорке

settings-styles-sheet-yes = Да.
settings-styles-sheet-no = Нет.
settings-styles-sheet-font = шрифт
settings-styles-sheet-indent = отступ первой линии
settings-styles-sheet-para-spacing = Параграф интервала
settings-styles-sheet-page = Размер страницы
settings-styles-sheet-margins = Поля (верх/право/низ/лево)
settings-styles-sheet-title-page = Заголовок книги
settings-styles-sheet-heading-language = Головной язык
settings-styles-sheet-auto = По тексту
settings-styles-sheet-digits = Числа
settings-styles-sheet-direction = Направление текста
settings-styles-sheet-formats = Формы
settings-styles-sheet-all-formats = Все

settings-styles-editor-group = Стиль редактора
settings-styles-editor-missing = Этот стиль больше не доступен.

settings-styles-page-letter = письмо
settings-styles-digits-western = Западный (0-9)
settings-styles-digits-eastern-arabic = Восточный арабский ( ⁇ - ⁇ )
settings-styles-direction-ltr = Слева направо
settings-styles-direction-rtl = Справа налево

format-dock-title = Формат
format-panel-empty = Поместите курсор в сцену, заметку или синопсис, чтобы увидеть варианты форматирования.
format-group-marks = Текст
format-group-block = Пункт
format-group-lists = Списки
format-group-tables = Столик
format-group-breaks = Сцена ломается
format-superscript = Надстрочный
format-subscript = Подстрочный
format-link = Ссылка…
link-dialog-insert-title = Включить ссылку
link-dialog-edit-title = Редактировать ссылку
link-dialog-text-label = Текст
link-dialog-text-placeholder = Слова, которые видит читатель
link-dialog-url-label = Ссылки на
link-dialog-url-placeholder = example.com
link-dialog-insert = Включить
link-dialog-apply = Применить
link-dialog-cancel = Отмена
link-dialog-remove = Удалить ссылку
link-scheme-refused = Можно открыть только веб-ссылки и ссылки по электронной почте. { $url } остался один.
menu-format-link = Сс&ылка…
format-clear = Четкий формат
format-heading = Заголовок
format-heading-normal = Нормальный текст
format-heading-1 = Заголовок 1
format-heading-2 = Заголовок 2
format-heading-3 = Заголовок 3
format-heading-4 = Заголовок 4
format-heading-5 = Заголовок 5
format-heading-6 = Заголовок 6
format-align-left = Элемент слева
format-align-center = По центру
format-direction-rtl = Пункт справа налево
format-blockquote = Цитата
format-list-bullet = Маркированный список
format-list-numbered = Номерной список
format-indent = Увеличить отступ
format-outdent = Уменьшить отступ
format-table-insert = Включить таблицу
format-table-row-above = Включить строку выше
format-table-row-below = Включить строку ниже
format-table-col-before = Включить колонку перед
format-table-col-after = Включить колонку после
format-table-row-delete = Исключить строку
format-table-col-delete = Исключить колонку
format-table-remove = Удалить стол

menu-format-marks-bold = По&лужирный
menu-format-marks-italic = &Курсив
menu-format-marks-underline = По&дчёркнутый
menu-format-marks-strike = З&ачёркнутый
menu-format-marks-superscript = &Надстрочный
menu-format-marks-subscript = П&одстрочный
menu-format-marks-clear = &Четкое форматирование
menu-format-heading = &Заголовок
menu-format-heading-normal = &Нормальный текст
menu-format-heading-1 = &Заголовок 1
menu-format-heading-2 = З&аголовок 2
menu-format-heading-3 = За&головок 3
menu-format-heading-4 = Заг&оловок 4
menu-format-heading-5 = Заго&ловок 5
menu-format-heading-6 = Заголо&вок 6
menu-format-alignment = &Выравнивание
menu-format-align-left = &Выравнивание левой
menu-format-align-center = &По центру
menu-format-direction = на&правление
menu-format-direction-auto = &автоматический
menu-format-direction-ltr = &Слева направо
menu-format-direction-rtl = &Право налево
menu-format-blockquote = &Цитата
menu-format-lists = &Списки
menu-format-list-bullet = &Маркированный список
menu-format-list-numbered = &Номерной список
menu-format-indent = У&величить отступ
menu-format-outdent = &Уменьшить отступ
menu-format-table = С&толик
menu-format-table-insert = &Включить таблицу
menu-format-table-2x2 = &2 x 2
menu-format-table-3x3 = &3 x 3 x 3
menu-format-table-4x4 = &4 x 4 x 4
menu-format-table-row-above = Вст&авить Row Above
menu-format-table-row-below = В&ставить строку ниже
menu-format-table-col-before = Вставит&ь колонку перед
menu-format-table-col-after = Вс&тавить колонну после
menu-format-table-row-delete = У&далить Row
menu-format-table-col-delete = &Исключить колонку
menu-format-table-remove = &Удалить стол

menu-format-typo-marks = &Форматирование Марка
menu-format-typo-nbsp = &No-break Космос
menu-format-typo-nbhyphen = &Безопасный петух
menu-format-typo-soft-hyphen = &Мягкий перенос
menu-format-typo-nnbsp = &Узкое No-break пространство
menu-format-typo-zwsp = &Пространство нулевой ширины
menu-format-typo-word-joiner = &Соединитель слов


menu-about = &Около Skribisto…
about-title = Около Skribisto
about-version = Версия { $version }
about-update-available = Версия { $version } доступна
about-update-see-notes = Посмотрите, что изменилось
about-update-flathub = Flathub обновляет эту копию.
about-update-distro = Ваш дистрибутив сохраняет эту копию в актуальном состоянии.
about-tagline = Приложение для написания романов для художественной литературы в длинных формах, написанное на Rust с помощью инструментария Teksilo.
about-license = Выпущено под лицензией GNU General Public License, версия 3.
about-copyright = © 2026 Сирил Жак
about-close = Закрыть

native-menu-about = Около { $app }
native-menu-hide = Скрыть { $app }
native-menu-quit = Выйти из { $app }
native-menu-settings = Настройки …
native-menu-window = окно
native-menu-minimize = Свернуть
native-menu-zoom = Зум

window-title = { $title } - { $app }
window-title-numbered = { $title } - { $app } (Окно { $n })
window-title-empty = { $app }

first-run-window-title = Добро пожаловать.
first-run-title = Настройка { $app }
first-run-body = { $app } сохраняет свои настройки отдельно от Skribisto, поэтому он начинается пустым. Ваши предпочтения, последние проекты, словари и макет окна могут быть скопированы сейчас.
first-run-from = Копия из
first-run-to = Копировать
first-run-copy-note = Skribisto сохраняет все свои собственные настройки и продолжает работать точно так же, как и раньше.
first-run-import = Параметры импорта
first-run-start-fresh = Начинайте свежо
first-run-import-failed = Некоторые настройки не могли быть импортированы: { $error }

settings-page-punctuation = Дефолты пунктуации
settings-group-punctuation = Умная пунктуация
settings-page-work-punctuation = Пунктуация
settings-punctuation-override = Дайте этому проекту свои правила пунктуации.
settings-punctuation-override-hint = Выключено: проект следует предпочтениям приложения. Эти правила путешествуют внутри .skrib, поэтому соавтор, открывающий файл, пишет с той же типографикой.
settings-punctuation-dashes = Превратить - в тире, - в тире эм
settings-punctuation-ellipsis = Превратить... в эллипс
settings-punctuation-quotes = Кривая кавычек и апострофов
settings-quote-style = Квотные знаки
settings-quote-style-locale = Язык по умолчанию
settings-quote-style-curly = «Двойной»
settings-quote-style-curly-single = «Одинокий»
settings-quote-style-guillemets = «Guillemets»
settings-quote-style-low-high = «Низко-высокий»
settings-punctuation-spacing = Пространство перед ; : !
settings-punctuation-spacing-hint = Французская типография задает перед ; ! ? и перед полной :. Это относится только к тексту, написанному по-французски. Пространство внутри гильметов «» поставляется с самими цитатами гильмета.
settings-punctuation-sample = Ваш язык дает
settings-punctuation-app-hint = Что делает каждый проект, если он не принимает свои собственные правила в Проекте ▸ Пунктуация.
settings-punctuation-dialogue = Откройте абзац, набранный как "-" с диалоговой тире
settings-punctuation-dialogue-hint = Для языков, которые обозначают речь тире, а не кавычками: французский, испанский, русский и другие. Она горит только в самом начале абзаца.

statusbar-go-to = Перейти на …
go-to-placeholder = Поиск связующего
go-to-no-matches = Ни один предмет не соответствует этому поиску.
menu-go-to = &Перейти на …

settings-page-distraction-free-themes = Темы, свободные от отвлекающих факторов
settings-themes-builtin = Встроенные темы
settings-themes-builtin-badge = встроенный
settings-themes-user = Мои темы
settings-themes-editor-group = Редактировать тему
settings-themes-editor-empty = Выберите тему под «Мои темы», чтобы отредактировать ее.
settings-themes-use = Использовать
settings-themes-duplicate = Дублировать
settings-themes-edit = Правка
settings-themes-delete = Удалить
settings-themes-export = Экспорт …
settings-themes-import = Импорт …
settings-themes-copy-suffix = копия
settings-themes-json-filter = Тема (JSON)
settings-themes-imported = Тема импортная
settings-themes-import-failed = Я не мог перенять эту тему.
settings-themes-exported = Тема экспортируется
settings-themes-export-failed = Мы не можем экспортировать эту тему.
settings-themes-low-contrast = Низкий контраст
settings-themes-low-contrast-band = Блеск скрывает текст
settings-themes-field-name = Название
settings-themes-field-paper = Страница
settings-themes-field-ink = Текст
settings-themes-field-general = Предыстория
settings-themes-field-widget-text = Контрольная полоска текста
settings-themes-field-caret-band = Подсветка вокруг заботы

statusbar-focus-settings = Безотвлекательные настройки
statusbar-focus-manage-themes = Управление темами …

comments-title = Комментарии
comments-document-title = Этот документ
comments-empty-project = Комментариев в этом проекте пока нет.
comments-empty-document = Комментариев к этому документу нет.
comments-filter-all = Все
comments-filter-open = Открыть
comments-filter-resolved = Решено
comments-filter-orphaned = Без привязки к тексту
comments-status-open = Открыть
comments-status-resolved = Решено
comments-status-orphaned = Исходный текст утрачен
comments-orphan-snippet = (комментированный текст исчез)
comments-status-unplaced = Нет текстовой позиции
comments-unplaced-snippet = (не привязан к какому-либо тексту)
comments-reply-count = { $count ->
    [one] { $count } ответ
    [few] { $count } ответа
    [many] { $count } ответов
   *[other] { $count } Ответы
}
comments-menu-resolve = Отметить решённым
comments-menu-reopen = Открыть
comments-menu-delete = Удалить
comments-sort-document = В документальном порядке
comments-sort-newest = Сначала новые
comments-sort-reader = Сгруппированный читатель
comments-unsigned-author = Неподписанный
comments-agreement = { $count ->
    [one] { $count } читатель
    [few] { $count } читателя
    [many] { $count } читателей
   *[other] { $count } Читатели
}
comments-menu-add = Добавить комментарий
comments-menu-add-paragraph = Комментарий к этому пункту
overview-col-comments = Комментарии
overview-col-total-comments = Общие замечания
comments-card-placeholder = Напишите комментарий …
comments-card-unknown-author = Неизвестный автор
comments-unsigned-toast = Ваши комментарии не подписаны: на этом компьютере для вас не установлено имя.
comments-unsigned-action = Установите свое имя
comments-card-reply = Ответить
comments-card-reply-placeholder = Ответ …
comments-card-actions = Комментарии
comments-reply-actions = Ответные действия
comments-menu-delete-reply = Исключить ответ
comments-menu-delete-all = Удалить все комментарии здесь
comments-deleted-toast = Комментарий удален
comments-reply-deleted-toast = Ответ удален
comments-deleted-all-toast = { $count ->
    [one] { $count } комментарий удален
    [few] { $count } комментария удалены
    [many] { $count } комментариев удалено
   *[other] { $count } Комментарии удалены
}
comments-undo = Отменить

analysis-segment = Анализ
analysis-scope-book = Анализ этой книги
analysis-run = Провести анализ
analysis-stale = Изменился с тех пор, как это произошло
analysis-not-run = Пока не проанализировано.
analysis-running = Читать рукопись …
analysis-failed = Анализ не мог закончиться.
analysis-no-scenes = Сцен в этой книге пока нет.

analysis-shape = форма

analysis-arrivals = Прибытие
analysis-arrivals-explainer = Как текст попал в этот проект, пока он был открыт. В нем говорится, по какому маршруту спустились персонажи, и вообще ничего о том, кто их написал: писатель, который рисует в другом месте и вставляет, и тот, кто диктует, продиктовал. Здесь нет числа, к которому можно стремиться.
analysis-arrivals-scope = Весь этот проект, а не только эта книга.
analysis-arrivals-session = С тех пор, как вы открыли его, закрытие проекта снова начинает отсчет.
analysis-arrivals-nothing = Текста на эту сессию пока не поступало.
analysis-arrivals-typed = типизированный
analysis-arrivals-pasted = Вставлено
analysis-arrivals-dictated = продиктованный
analysis-arrivals-imported = импортный
analysis-arrivals-programmatic = Вставлено для вас
analysis-arrivals-count = { $count ->
    [one] { $count } персонаж
    [few] { $count } персонажа
    [many] { $count } персонажей
   *[other] { $count } символы
}
analysis-arrivals-none = ни одного

analysis-words-per-scene = Слова на сцену
analysis-median-words = Медианная сцена этой книги проходит { $count ->
    [one] { $count } слово.
    [few] { $count } слова.
    [many] { $count } слов.
   *[other] { $count } слов.
}
analysis-median-line = Медиана: { $count ->
    [one] { $count } слово
    [few] { $count } слова
    [many] { $count } слов
   *[other] { $count } слов
}
analysis-dialogue = Диалог
analysis-dialogue-unsupported = Диалог для этого языка пока не измеряется.

analysis-footnote-words = Сноска слов
analysis-footnote-words-count = { $count ->
    [one] { $count } слово в сносках этой книги, отдельно от общей рукописи.
    [few] { $count } слова находятся в сносках этой книги, отдельно от общей рукописи.
    [many] { $count } слов в сносках этой книги, которые хранятся отдельно от общей рукописи.
   *[other] В сносках этой книги содержится { $count } слов, отдельно от общей суммы рукописей.
}
analysis-footnote-words-pending = Подсчет сносок …

analysis-ignore-empty = Игнорировать тексты, которые еще не написаны
analysis-empty-hidden = { $count } пустой { $count ->
    [one] текст
    [few] тексты
    [many] тексты
   *[other] тексты
    } спрятаны.
analysis-all-texts-empty = Каждый текст в этой книге пуст.

binder-filter-count = { $shown } из { $total } показан
binder-filter-clear = Чисто.
binder-filter-none = Ничего не совпадает с «{ $query }».

settings-paratext-intro = Спереди и сзади может начаться новый проект. Каждая структура принадлежит издательской традиции, а названия страниц написаны на родном языке этой традиции. Переименуйте их свободно, как только проект будет создан.
settings-paratext-structures = Структуры
settings-paratext-broken = Нельзя было читать
settings-paratext-edit = Правка
settings-paratext-duplicate = Дублировать
settings-paratext-delete = Удалить
settings-paratext-new = Новая структура
settings-paratext-save = Сохранить
settings-paratext-editor-hint = Имя, страницы, которые нужно создать до рукописи, и страницы, которые нужно создать после нее. Где они в конечном итоге ваши: они являются обычными предметами, как только проект существует.


image-insert = Вкл&ючить изображение …
image-no-project = Откройте проект, прежде чем вставлять изображение.
image-choose-title = Выберите изображение
image-filter-label = Изображения
image-large-title = Это большое изображение
image-large-text =
    { $name } :  { $megapixels } мегапикселей ({ $width }×{ $height }).
    Сохраняя его таким, какой он есть, храните свой оригинальный файл в проекте: он путешествует.
    с каждой резервной копией и каждым экспортом. Оптимизация хранит меньшую копию
    Вместо этого, до 2560 пикселей на самой длинной стороне.
image-large-keep = Держите оригинал
image-large-downscale = Оптимизировать
image-large-remember = Делай это с этого момента, не спрашивай снова
image-not-recorded = Изображение было сохранено, но не могло быть записано в проекте.
image-describe-title = Опишите изображение
image-describe-explain = То, что изображено на картине, для читателя, который ее не видит, не является частью рукописи: она никогда не считается, не обыскивается и не вывозится в виде прозы.
image-describe-placeholder = Маяк на фоне серого неба
image-resize-title = Изменить размер изображения
image-resize-explain = Процент от размера, который он показывает сейчас. 100 оставляет его таким, какой он есть.
image-resize-invalid = Введите число от 1 до 1000.
image-menu-describe = &Опишите изображение …
image-menu-resize = &Размер изображения …
image-menu-reset-size = &Оригинальный размер

cover-choose = &Книжная обложка …
cover-clear = Сни&мите крышку
cover-choose-title = Выберите покрытие
cover-set = Обложка установлена.
cover-cleared = Обложка снята. Картина все еще в проекте.
card-label = Подпись
card-exportable = экспортируемый
card-numbered = нумерованный
card-created = созданный
card-modified = модифицированный
card-yes = да.
card-no = нет
card-type = Тип
card-position = Позиция
card-children = Дети
card-goal = Цель
card-words = Слова
card-synopsis = Синопсис
card-point-of-view = Точка зрения
card-aliases = Другие имена

export-orphan-footnotes-title = Сноски без ссылки
export-comments-dropped =
    { $count ->
    [one] { $count } комментарий не мог быть помещен в экспортируемый текст и был исключен.
    [few] { $count } замечания не могли быть помещены в экспортируемый текст и были исключены.
    [many] { $count } комментариев не могли быть помещены в экспортируемый текст и были исключены.
   *[other] { $count } комментарии не могли быть помещены в экспортируемый текст и были исключены.
    }
export-orphan-footnotes =
    { $count ->
    [one] { $count } сноска больше нигде в рукописи не упоминается, её текст не появится в экспортируемой книге.
    [few] { $count } сноски больше нигде в рукописи не упоминаются, их текст не появится в экспортируемой книге.
    [many] { $count } сносок больше нигде в рукописи не упоминаются, их текст не появится в экспортируемой книге.
   *[other] Сноски { $count } больше нигде в рукописи не упоминаются, их текст не появится в экспортируемой книге.
    }


menu-footnotes = &Сноски
footnotes-title = Сноски
footnotes-insert = В&ключить сноску
footnotes-empty = Поместите курсор в сцену, затем используйте + выше или Ctrl + Alt + F.
footnotes-filter-all = Все
footnotes-filter-document = Этот документ
footnotes-filter-orphaned = Без привязки к тексту
footnotes-orphaned = Ничто больше не указывает на эту записку.
footnotes-untitled-home = Без названия
footnotes-body-placeholder = Сама записка
footnotes-insert-tooltip = Включить сноску на курсоре (Ctrl+Alt+F)
footnotes-actions = Сноски сносок
footnotes-delete = &Исключить примечание и ссылку на него
footnotes-no-project = Откройте проект, прежде чем вставить сноску.
footnotes-no-caret = Поместите курсор в текст сцены, чтобы вставить там сноску.
footnotes-not-created = Сноску к проекту добавить не удалось.
footnotes-deleted-toast = Записка удалена
footnotes-undo-delete = Отменить

import-document-title = Импортные документы
import-document-close = Закрыть
import-document-step-files = Файлы
import-document-step-review = Обзор
import-document-step-destination = Направление
import-document-step-reconcile = Объединить
import-document-reconcile-hint = Некоторые из этих вещей у вас уже есть. Скажи, что должно случиться с каждым из них.
import-document-reconcile-all-new = Ничто в этом файле не соответствует вашему проекту: каждый элемент будет добавлен как новый.
import-document-duplicate-returns = Эти файлы являются более чем одной копией одной и той же рукописи, возвращающейся. Импортируйте их по одному за раз: сопоставляя только одну возвращающуюся копию с вашим проектом, поэтому импорт нескольких вместе добавит вторую копию книги вместо ее объединения.
import-document-hunk-added = Возьмем новый пункт
import-document-hunk-removed = Опустить этот пункт
import-document-hunk-changed = Возьми это переписать
import-document-col-stray-prose = Его текст
import-document-stray-as-paratext = Сохраняйте как паратекст
import-document-stray-discard = Бросьте текст
import-document-col-current = В вашем проекте
import-document-col-incoming = В этом файле
import-document-col-status = статус
import-document-col-action = Что делать
import-document-status-identical = Без изменений
import-document-status-editor-edited = Изменено читателем
import-document-status-you-edited = Изменено вами
import-document-status-conflict = Изменено обеими сторонами
import-document-status-different = Отличается
import-document-status-new = Новый
import-document-status-missing = Нет в этом файле
import-document-status-moved = Перемещено
import-document-action-comments-only = Только комментарии
import-document-action-take-import = Принять эту версию
import-document-action-keep-current = Оставить мою
import-document-action-create-new = Добавить как новый
import-document-action-ignore = Пропустить
import-document-compare = Сравнить
import-document-compare-legend = Ваша версия по сравнению с этим файлом.
import-document-compare-close = Закрыть
import-document-drop-title = Бросьте документы здесь
import-document-drop-hint = Markdown (.md), Word (.docx), OpenDocument (.odt) и простой текст (.txt)
import-document-browse = Обзор…
import-document-move-up = Вставай.
import-document-move-down = Спускайся.
import-document-remove-file = Убрать
import-document-file-count = { $count ->
    [one] { $count } файл
    [few] { $count } Файлы
    [many] { $count } файлов
   *[other] { $count } файлов
}
import-document-no-files = Файлы еще не выбраны.
import-document-col-included = Импорт
import-document-col-title = Заголовок
import-document-col-type = Тип
import-document-col-words = Слова
import-document-col-breaks = Разрывы
import-document-col-comments = Комментарии
import-document-col-footnotes = Сноски
import-document-col-epigraph = Эпиграф
import-document-col-source = Источник
import-document-level-rules = Уровни заголовков
import-document-level-n = Заголовок { $level }
import-document-add-top-level = Добавить верхний уровень
import-document-add-top-level-tooltip = Вставьте книгу выше каждого проанализированного элемента для файлов глав, в которых нет заголовка книги.
import-document-destination = Направление
import-document-destination-hint = Выберите связующее или элемент: новые предметы попадают в папку или после сцены.
import-document-destination-empty = Пока никаких связующих.Создай сначала.
import-document-plan-empty = Импортировать пока нечего.
import-document-summary = { $rows ->
    [one] { $rows } пункт
    [few] { $rows } пункта
    [many] { $rows } элементов
   *[other] { $rows } единицы
} · { $breaks ->
    [one] { $breaks } эпизод
    [few] { $breaks } Сцена разрывается
    [many] { $breaks } Сцена разрывается
   *[other] { $breaks } Сцена разрывается
}
import-document-back = Назад
import-document-cancel = Отмена
import-document-analyse = Далее
import-document-analysing = Чтение документов…
import-document-step-analysing = читать
import-document-cancel-analysis = Хватит читать.
import-document-analyse-failed = Документы читать не удалось.
import-document-details = Подробности
import-document-import = Импорт
import-document-done = { $count ->
    [one] { $count } Ввозимые товары
    [few] { $count } Ввозимые товары
    [many] { $count } Ввозимые товары
   *[other] { $count } Ввезенные товары
}
import-document-undo = Отменить
import-diagnostic-file-unreadable = "{ $path }" читать нельзя: { $detail }. Остальные файлы все еще импортируются.
import-diagnostic-nested-too-deep = "{ $path }" не было прочитано: его содержимое вложено более чем в { $limit } уровней глубины. Ни один реальный документ не приближается к этому, и чтение его может привести к сбою Skribisto, поэтому он исключен. Другие файлы все еще импортируются.
import-diagnostic-archive-too-large = "{ $path }" не был прочитан: это сжатый файл, построенный для распаковки гораздо большего, чем он есть, который может исчерпать память и сломать Skribisto, поэтому он остается в стороне.
import-diagnostic-lossy-decode = { $count ->
    [one] { $count } символ в «{ $path }» не декодировал. Для сохранения файла в качестве UTF-8.
    [few] { $count } символа в «{ $path }» не декодировали. Для сохранения файла в качестве UTF-8.
    [many] { $count } символов в «{ $path }» не расшифровывались. Для их хранения повторно сохраняют файл как UTF-8.
   *[other] { $count } символов в «{ $path }» не расшифровывали. Для их сохранения повторно сохраняют файл как UTF-8.
}
import-diagnostic-decoded-from-bom = «{ $path }» был расшифрован как { $detail }, а не UTF-8.
import-diagnostic-empty-file = "{ $path }" пуст.
import-diagnostic-no-headings = «{ $path }» не имеет заголовков, поэтому он поставляется в виде одного элемента.
import-diagnostic-unsupported-format = Файлы «.{ $detail }» не читаются, поэтому файл «{ $path }» был пропущен.
import-diagnostic-front-matter-not-flat = Передняя материя в «{ $path }»: «{ $detail }» не является простым значением и была пропущена.
import-diagnostic-footnotes-degraded = { $count ->
    [one] { $count } сноска в «{ $path }» поставляется в виде простого текста: сноски не читаются с Markdown.
    [few] { $count } сноски в «{ $path }» поставляются в виде простого текста: сноски не читаются с Markdown.
    [many] { $count } сносок в «{ $path }» поставляются в виде простого текста: сноски не читаются с Markdown.
   *[other] { $count } сноски в «{ $path }» поступают в виде простого текста: сноски не читаются с Markdown.
}
import-diagnostic-footnote-not-carried = { $count ->
    [one] { $count } сноска в "{ $path }" не могла быть приведена.
    [few] { $count } сноски в "{ $path }" не могли быть приведены.
    [many] { $count } сносок в "{ $path }" не могли быть приведены.
   *[other] { $count } сноски в "{ $path }" не могли быть доставлены.
}
import-diagnostic-raw-html-dropped = { $count ->
    [one] { $count } блок сырого HTML в «{ $path }» был сброшен.
    [few] { $count } блока сырого HTML в «{ $path }» были сброшены.
    [many] Было сброшено { $count } блоков сырого HTML в «{ $path }».
   *[other] { $count } блоки сырого HTML в «{ $path }» были сброшены.
}
import-diagnostic-nested-break-dropped = { $count ->
    [one] { $count } сценический перерыв внутри цитаты или списка в «{ $path }» был отброшен. Сохраняется только перерыв на собственной линии.
    [few] { $count } сценические перерывы внутри котировок или списков в «{ $path }» были отброшены. Сохраняется только перерыв на собственной линии.
    [many] { $count } сценических перерывов внутри котировок или списков в "{ $path }" были отброшены. Сохраняется только перерыв на собственной линии.
   *[other] { $count } сцены разрывов внутри котировок или списков в "{ $path }" были сброшены. Сохраняется только перерыв на собственной линии.
}
import-diagnostic-image-not-ingested = "{ $path }" относится к изображению "{ $detail }". Ссылка поступает в виде текста; само изображение не копируется.
import-diagnostic-duplicate-title = "{ $title }" появляется { $count } раз. Если вы импортировали эти файлы раньше, это будет дублировать их.
import-diagnostic-heading-level-jump = «{ $title }» перепрыгивает с уровня товарной позиции { $from } на { $to }; он помещается на один уровень под своим материнским.
import-diagnostic-illegal-combination = "{ $title }" несет прозу, но { $kind } не может содержать ничего. Импорт удерживается до тех пор, пока вы не измените его тип или не оставите его. Ничего не импортируется вообще.
import-diagnostic-tracked-changes-flattened = { $path } был в середине пересмотра: { $count ->
    [one] { $count } Отслеженное изменение принято
    [few] Принято { $count } отслеживаемых изменения
    [many] Принято { $count } отслеживаемых изменений
   *[other] { $count } гусеничные изменения были приняты
}, и удаление отброшено. Это окончательный текст. Чтобы сохранить любую из ваших собственных формулировок, используйте Compare на последнем шаге.
import-diagnostic-tracked-changes-flattened-by = { $path } был в середине пересмотра: { $count ->
    [one] { $count } отслеживаемое изменение к { $names }
    [few] { $count } отслеживаемых изменения к { $names } были приняты
    [many] Принято { $count } отслеживаемых изменений к { $names }
   *[other] { $count } гусеничные изменения к { $names } были приняты
}, и удаление отброшено. Это окончательный текст. Чтобы сохранить любую из ваших собственных формулировок, используйте Compare на последнем шаге.
import-diagnostic-text-box-dropped = { $count ->
    [one] { $path } содержит { $count } текстовый ящик. Его текст находится вне потока документа, поэтому на то, где он находится в рукописи, нельзя ответить. Он не импортируется.
    [few] { $path } содержит { $count } текстовых ящика. Их текст находится вне потока документа, поэтому на то, где он находится в рукописи, нельзя ответить. Они не импортируются.
    [many] { $path } содержит { $count } текстовых ящиков. Их текст находится вне потока документа, поэтому на то, где он находится в рукописи, нельзя ответить. Они не импортируются.
   *[other] { $path } содержит { $count } текстовых ящиков. Их текст находится вне потока документа, поэтому на то, где он находится в рукописи, нельзя ответить. Они не импортируются.
}
import-diagnostic-embedded-object-dropped = { $count ->
    [one] { $path } содержит { $count } встроенный объект: диаграмму, уравнение или подобное. В рукописи нет ничего, что могло бы содержать его.
    [few] { $path } содержит { $count } встроенных объекта: диаграмму, уравнение или подобное. В рукописи нет ничего, что могло бы их содержать.
    [many] { $path } содержит { $count } встроенных объектов: диаграмму, уравнение или подобное. В рукописи нет ничего, что могло бы их содержать.
   *[other] { $path } содержит { $count } встроенных объектов: диаграмму, уравнение или подобное. В рукописи нет ничего, что могло бы содержать их.
}
import-diagnostic-field-flattened = { $count ->
    [one] { $path } содержит { $count } поле: номер страницы, перекрестную ссылку, дату. Он сохраняет текст, который он показывал в последний раз, и не будет обновляться снова.
    [few] { $path } содержит { $count } поля: номер страницы, перекрестную ссылку, дату. Каждый сохраняет текст, который он показывал в последний раз, и не будет обновляться снова.
    [many] { $path } содержит { $count } полей: номер страницы, перекрестную ссылку, дату. Каждый сохраняет текст, который он показывал в последний раз, и не будет обновляться снова.
   *[other] { $path } содержит { $count } полей: номер страницы, перекрестную ссылку, дату. Каждый сохраняет текст, который он показывал в последний раз, и не будет обновляться снова.
}
import-diagnostic-unknown-style-level = { $path } использует стиль "{ $detail }", который выглядит как заголовок, но не называет уровня. Эти абзацы импортируются как проза, а не угадываются на глубине.
import-diagnostic-comment-unanchored = Комментарий "{ $detail }" в { $path } не мог быть прикреплен к словам, о которых он был. Он хранится на своем предмете, где его можно перемещать.
import-diagnostic-comment-not-carried = Комментарий «{ $detail }» в { $path } не импортирован, потому что в файл ничего не поступает в виде текста, который можно было бы сохранить.
import-diagnostic-comment-replies-flattened = { $count ->
    [one] { $count } ответ в { $path } называется комментарием, которого нет в файле, поэтому он поступает как собственный комментарий.
    [few] { $count } ответа в { $path } назвали комментарий, которого нет в файле, поэтому они поступают как собственные комментарии.
    [many] { $count } ответов в { $path } назвали комментарий, которого нет в файле, поэтому они поступают как собственные комментарии.
   *[other] В { $count } ответов в { $path } назван комментарий, которого нет в файле, поэтому они приходят в качестве собственных комментариев.
}
import-diagnostic-prose-not-verbatim = { $count ->
    [one] { $count } параграф в { $path } не может быть приведен с его форматированием точно так же, как написано, поэтому он приходит как простой текст.
    [few] { $count } абзаца в { $path } не могли быть приведены с их форматированием точно так же, как написано, поэтому они прибывают в виде простого текста.
    [many] { $count } параграфов в { $path } не могли быть приведены с их форматированием точно так же, как написано, поэтому они прибывают в виде простого текста.
   *[other] { $count } абзацы в { $path } не могли быть приведены с их форматированием точно так же, как написано, поэтому они прибывают в виде простого текста.
}
import-diagnostic-styled-spaces-not-carried = { $count ->
    [one] В { $path } { $count } участок пустого пространства был подчеркнут или пробит, например, линия, оставленная для заполнения. Skribisto не держит ни одной линии на одних только пространствах, поэтому он прибывает в виде простых пространств или вообще не там, где он начинается или заканчивается абзацем.
    [few] В { $path } { $count } участка пустого пространства были подчеркнуты или пробиты, например, линии, оставленные для заполнения. Skribisto не сохраняет ни одной линии только на пространствах, поэтому они прибывают как простые пространства или вообще не там, где они начинают или заканчивают абзац.
    [many] В { $path } { $count } участков пустого пространства были подчеркнуты или пробиты, например, линии, оставленные для заполнения. Skribisto не сохраняет ни одной линии на одних только пространствах, поэтому они прибывают в виде простых пространств или вообще не там, где они начинают или заканчивают абзац.
   *[other] В { $path } { $count } участки пустого пространства были подчеркнуты или пробиты, например, линии, оставленные для заполнения. Skribisto не сохраняет ни одной линии на одних только пространствах, поэтому они прибывают в виде простых пространств или вообще не там, где они начинают или заканчивают абзац.
}
import-diagnostic-list-nesting-flattened = { $count ->
    [one] { $count } пункт списка в { $path } был вложен более чем в { $limit } уровней глубины.Он прибывает на уровень { $limit }, самый глубокий Skribisto сохраняет, с его словами и форматированием.
    [few] { $count } пункта списка в { $path } были вложены более чем в { $limit } уровней глубиной.Они достигают уровня { $limit }, самого глубокого Skribisto сохраняет, с их словами и форматированием.
    [many] { $count } пунктов списка в { $path } были вложены более чем в { $limit } уровней глубиной.Они достигают уровня { $limit }, самого глубокого Skribisto сохраняет, с их словами и форматированием.
   *[other] { $count } списки предметов в { $path } были вложены более чем в { $limit } уровней глубиной.Они достигают уровня { $limit }, самого глубокого Skribisto сохраняет, с их словами и форматированием.
}
import-diagnostic-table-read-as-paragraphs = { $count ->
    [one] { $count } таблица в { $path } имеет ряд гораздо шире остальных, слишком однобокий, чтобы выкладываться в виде сетки.Его ячейки прибывают в виде абзацев, один за другим, со своими словами и форматированием.
    [few] { $count } таблицы в { $path } имеют ряд гораздо шире остальных, слишком однобокий, чтобы выкладываться в виде сетки. Их ячейки прибывают в виде абзацев, один за другим, со своими словами и форматированием.
    [many] { $count } таблиц в { $path } имеют ряд гораздо шире остальных, слишком однобокий, чтобы выкладываться в виде сетки. Их ячейки прибывают в виде абзацев, один за другим, со своими словами и форматированием.
   *[other] { $count } таблицы в { $path } имеют ряд гораздо шире остальных, слишком однобокий, чтобы выкладываться в виде сетки. Их ячейки прибывают в виде абзацев, один за другим, со своими словами и форматированием.
}
import-diagnostic-spaces-shortened = { $count ->
    [one] { $count } пробег пространств в { $path } был длиннее { $limit } пространств, а прибывает сокращенный до { $limit }.
    [few] { $count } пробега пространств в { $path } были длиннее { $limit } пространств, а прибывшие сократились до { $limit }.Слова вокруг них все там.
    [many] { $count } пробегов пространств в { $path } были длиннее { $limit } пространств, а прибывшие сократились до { $limit }.Слова вокруг них все там.
   *[other] { $count } пробегов пространств в { $path } были длиннее { $limit } пробелов, а прибывшие сократились до { $limit }.Слова вокруг них все там.
}
import-diagnostic-epigraph-not-carried = «{ $title }» возглавляется эпиграфом, но { $kind } не может удержать его. Цитата держится в верхней части ее текста.
import-diagnostic-epigraph-placement-ambiguous = Эпиграф находится между «{ $title }» и «{ $below }» и может возглавлять любой из них. Он был дан «{ $title }», куда обычно идет эпиграф.
import-document-diagnostics = { $errors ->
    [0] { $warnings ->
    [one] { $errors } вещь, которую нужно знать
    [few] { $warnings } Что нужно знать
    [many] { $warnings } Что нужно знать
   *[other] { $warnings } Что нужно знать
        }
   *[other] { $errors ->
    [one] { $errors } файл не может быть прочитан
    [few] { $errors } Файлы не читаются
    [many] { $errors } файлов, которые невозможно прочитать
   *[other] { $errors } файлы не могут быть прочитаны
        }
}
import-document-diagnostics-none = Нечего докладывать.

versions-title = Версии
versions-scope-synopsis = Синопсис
versions-scope-prose = Текст
versions-loading = Просмотр ваших резервных копий …
versions-empty = Более ранней версии этого пока нет.
versions-error = Ничего не потеряно, но этот список может быть неполным
versions-did-not-exist = Не было еще { $date }
versions-deleted-after = Удален через некоторое время после { $date }
versions-unreadable = { $count ->
    [one] { $count } Резервное копирование невозможно прочитать
    [few] { $count } резервных копии не могут быть прочитаны
    [many] { $count } резервных копий не могут быть прочитаны
   *[other] { $count } Резервные копии не могут быть прочитаны
}
versions-thinned = { $count ->
    [one] Старые версии с возрастом истончаются.Собственная история проекта уже отбросила { $count } более раннее состояние этого текста.
    [few] Старые версии с возрастом истончаются.Собственная история проекта уже отбросила { $count } более ранних состояния этого текста.
    [many] Старые версии с возрастом истончаются.Собственная история проекта уже отбросила { $count } более ранних состояний этого текста.
   *[other] Старые версии с возрастом истончаются.Собственная история проекта уже выпала на { $count } более ранних состояний этого текста.
}
versions-source-backup = Из резервной копии
versions-source-project = Из собственной истории проекта
versions-list-caption = Одна запись за изменение, а не за резервную копию
versions-pick-a-version = Выберите версию, чтобы увидеть, что изменилось
versions-earliest = Самая ранняя версия на записи. Нет ничего старше, чтобы сравнить это с.
versions-no-change = Ничего не изменилось в этой части статьи
versions-formatting-only = Изменилось только форматирование. Слова одинаковые.
versions-show-unchanged = Показать неизмененный
versions-hide-unchanged = Прятаться без изменений
versions-next-change = Следующее изменение
versions-near = рядом с "{ $text }"
versions-words-added = { $count ->
    [one] Добавлено { $count } слово
    [few] Добавлено { $count } слова
    [many] Добавлено { $count } слов
   *[other] { $count } слов добавлено
}
versions-words-removed = { $count ->
    [one] { $count } слово удалено
    [few] { $count } слова удалены
    [many] { $count } Удаленные слова
   *[other] { $count } Удаленные слова
}
versions-blocks-moved = { $count ->
    [one] { $count } абзац сдвинуто
    [few] { $count } абзаца сдвинуты
    [many] { $count } Перенесенные пункты
   *[other] { $count } Пункты перенесены
}
versions-pin = Заполните резервную копию этой версии. Автоматическая очистка никогда не удалит ее.
versions-unpin = Отключите резервную копию этой версии. Автоматическая очистка может удалить ее снова
versions-pinned-only = Показать только пин-версии
versions-pin-note = Только версии из резервной копии могут быть прикреплены. Пин хранит файл, а собственная история проекта не является таковой.
versions-pinned-empty = Здесь пока ничего не приколото.
versions-pinned-empty-log = Здесь ничего не прикреплено. Только версии из резервной копии могут быть прикреплены. Пин хранит файл, а собственная история проекта не является таковой.
versions-range-filter = Показывать только версии между двумя датами
versions-filtered-empty = Ни одна версия не соответствует фильтрам, которые вы установили
versions-clear-filters = Очистите фильтры
versions-last-30-days = Последние 30 дней
versions-restore-button = Восстановите эту версию
versions-restore-confirm-title = Заменить этот текст версией от { $date }?
versions-restore-confirm-text = То, что у вас есть сейчас, будет заменено текстом этого пункта на { $date }.
versions-restore-confirm-with-comments = То, что у вас есть сейчас, будет заменено текстом этого пункта на { $date }. { $count ->
    [one] { $count } комментарий закреплен в текущем тексте и может быть оставлен сиротой.
    [few] { $count } замечания закреплены в текущем тексте и могут быть оставлены сиротами.
    [many] { $count } комментариев закреплены в текущем тексте и могут быть оставлены сиротами.
   *[other] Комментарии { $count } приведены в настоящем тексте и могут быть оставлены сиротами.
}
versions-restore-confirm-undo-note = Сначала делается резервная копия, и Ctrl + Z снимает это за один шаг.
versions-restored-toast = Восстановлена версия от { $date }
versions-undo = Отменить
versions-restore-row-gone = Этого пункта больше нет в этом проекте.
versions-restore-no-home = Этот пункт с тех пор изменил тип, и старому тексту некуда идти в нем.
versions-restore-no-safety-copy = Ваша резервная копия не работала, так что ничего не изменилось.
versions-restore-failed = Восстановление не удалось: { $error }
versions-restore-backup-busy = Уже работает резервная копия. Попробуй еще раз через минуту.
versions-restore-in-backup-file = Вы просматриваете файл резервного копирования; откройте сам проект, чтобы восстановить его.
versions-restore-no-project = Ни один проект не открыт
versions-recreate-button = Принесите это обратно …
versions-recreate-picker-title = Куда она должна пойти?
versions-recreate-picker-empty = Этот проект не имеет связующего звена, чтобы поместить его в
versions-recreate-picker-confirm = Возвращайся сюда.
versions-recreate-picker-cancel = Отмена
versions-recreate-untitled = этот пункт
versions-recreate-confirm-title = Вернуть «{ $item }»?
versions-recreate-confirm-text = Он будет добавлен к { $destination }, с текстом, который он имел на { $date }.
versions-recreate-confirm-undo-note = Ундо, в следующем послании, вытаскивает его обратно.
versions-recreated-toast = «{ $item }» возвращается в проект
versions-recreated-partial-toast = { $count ->
    [one] «{ $item }» вернулся, но { $count } из его текстов не удалось прочитать
    [few] «{ $item }» вернулся, но { $count } из его текстов не удалось прочитать
    [many] «{ $item }» вернулся, но { $count } его текстов не удалось прочитать
   *[other] «{ $item }» вернулся, но { $count } его текстов не удалось прочитать.
}
versions-recreate-already-here = Этот пункт уже есть в вашем проекте.
versions-recreate-no-destination = Выберите место в связующем, чтобы поместить его
versions-recreate-unreadable = Эта резервная копия не могла быть прочитана, поэтому ничего не было добавлено.
versions-recreate-failed = Не удалось вернуть: { $error }
versions-changed-percent = { $percent }% изменятся
versions-hidden-paragraphs = { $count ->
    [one] … { $count } неизмененный пункт …
    [few] … { $count } неизмененные пункты …
    [many] … { $count } неизмененных пунктов …
   *[other] … { $count } Неизменные пункты …
}

timeline-title = Вернуться в прошлое
timeline-coverage = { $count ->
    [one] { $count } версия записана, возвращаясь к { $oldest }
    [few] Записано { $count } версии, начиная с { $oldest }
    [many] Записано { $count } версий, начиная с { $oldest }
   *[other] { $count } версии записаны, возвращаясь к { $oldest }
}
timeline-loading = Просматривайте прошлое вашего проекта …
timeline-empty = Версия этого проекта пока не зафиксирована.
timeline-no-changes = С тех пор ничего не изменилось.
timeline-no-text-changes = С тех пор текст не изменился.
timeline-slider-label = Записанная версия
timeline-series-name = Размер проекта
timeline-bars-caption = Каждый бар - это записанная версия, такая же высокая, как и проект. Выделенный - это то, что вы смотрите.
timeline-bars-caption-periods = Слишком много версий, чтобы показать одну за другой, так что каждый бар { $period }, такой же высокий, как проект был к концу его.
timeline-unit-hour = час
timeline-unit-day = день
timeline-unit-week = неделя
timeline-unit-month = месяц
timeline-range-filter = Показывать только версии между двумя датами
timeline-range-empty = Ни одна версия не была записана в эти даты.
timeline-open-period = Откройте этот период
timeline-show-all = Показать всю историю
timeline-changed-since = { $count ->
    [one] { $count } пункт отличается от { $date } и вашего проекта сейчас
    [few] { $count } пункта отличаются между { $date } и вашим проектом сейчас
    [many] { $count } элементов отличаются между { $date } и вашим проектом сейчас
   *[other] Элементы { $count } различаются между { $date } и вашим проектом.
}
timeline-kind-added = Написано с тех пор
timeline-kind-removed = Больше не в проекте
timeline-kind-changed = Отредактировано с тех пор
timeline-kind-moved = С тех пор переехал
timeline-not-yet-written = Этого еще не существовало в тот момент.
timeline-no-text-of-its-own = У этого нет собственного текста. Это заголовок того, что внутри него.
timeline-reader-close = Закрыть
timeline-reader-stamp = Как это было на { $date }
timeline-reader-loading = Открытие записанной версии…
timeline-reader-unreadable = Эта запись не могла быть прочитана. Резервная копия могла быть перемещена, удалена или находиться на диске, который не подключен.
timeline-reader-compared = Как это было на { $date } против того, что он говорит сейчас.
timeline-reader-diff-legend = С тех пор прошло много времени, было добавлено подчеркивание.
timeline-reader-view-label = Что показать
timeline-reader-view-diff = Изменения
timeline-reader-view-text = Текст
timeline-reader-deleted = Этого больше нет в вашем проекте. Вы можете прочитать и скопировать его здесь.
timeline-prose-only-record = Эта точка исходит из собственной истории проекта, которая хранит текст и ничего больше, так что то, что было удалено или перемещено, так как это не может быть показано.

settings-sec-extensions = Расширения

settings-group-games-forward = Всегда вперед
settings-games-forward-toggle = Спектакль «Всегда вперед»
settings-games-forward-hint =
    Пока вы играете, ничто из написанного вами не может быть возвращено: Backspace, Удалить,
    Резка, перетаскивание и ундо - все они отключены на поверхностях, которые вы выбираете ниже.
    Вы все еще можете печатать, вставлять, форматировать и свободно перемещаться.
settings-games-session-warning =
    Это выбор на каждую сессию: он никогда не сохраняется. Закрытие проекта или уход из него.
    Skribisto, всегда заканчивает игру, и все, что вы написали во время игры, становится
    Снова отменяется в тот момент, когда это происходит.
settings-group-games-scope = Где это применяется
settings-games-in-prose = Рукописная проза
settings-games-in-synopsis = синопсис
settings-games-scope-hint =
    Комментарии, сноски и заголовки никогда не замораживаются: они там, где вы отмечаете исправление.
    Вы только что запретили себе это делать.
settings-games-inert-warning =
    "Всегда вперед" включено, но не применяется ни к чему.
    Ничего не меняет, как вы пишете.

games-title = Написание игр
games-forward-name = Всегда вперед
games-forward-blurb =
    Удаление, резка и отмена отключены, ничего не беря назад.
    Пока вы играете, напишите следующее предложение вместо того, чтобы фиксировать последнее.
games-forward-playing = Играть: удаление отключено
games-forward-idle = Не играть
games-session-note = Заканчивается, когда проект закрывается.
games-scope-prose-and-synopsis = Применяется к вашей прозе и синопсисам.
games-scope-prose = Применяется к вашей прозе.
games-scope-synopsis = Применяется к вашим синопсисам.
games-scope-nothing = Пока ничего не подходит. Выберите поверхность в настройках.
games-settings-link = Настройка игры …
statusbar-games-forward = Всегда вперед
statusbar-games-forward-tooltip =
    «Всегда вперед» включен: удаление, резка и отмена отключены, пока вы
    Нажмите, чтобы перестать играть.
goal-progress-words = { $g ->
    [one] { $count } из { $goal } слово
    [few] { $count } из { $goal } слов
    [many] { $count } из { $goal } слов
   *[other] { $count } из { $goal } слов
}
goal-progress-characters = { $g ->
    [one] { $count } { $goal } символ
    [few] { $count } из { $goal } символов
    [many] { $count } из { $goal } символов
   *[other] { $count } из { $goal } символов
}
goal-count-words = { $n ->
    [one] { $count } слово
    [few] { $count } слова
    [many] { $count } слов
   *[other] { $count } слова
}

goal-count-characters = { $n ->
    [one] { $count } символ
    [few] { $count } символа
    [many] { $count } символов
   *[other] { $count } символа
}

goal-unit-words = Слова
goal-unit-characters = Персонажи

inspector-goal = Цель
inspector-goal-none = Нет цели

overview-col-goal = Цель
overview-excluded-from-export = Выброшенный из экспорта, поэтому он не учитывается в общей сумме

goal-subtree-total-words = { $count ->
    [one] { $items } цель внутри добавляет до { $words } слов
    [few] { $items } цели внутри складываются в { $words } слов
    [many] { $items } цели внутри складываются в { $words } слов
   *[other] { $items } цели внутри складываются в { $words } слов
}
goal-subtree-total-characters = { $count ->
    [one] { $items } цель внутри добавляет до { $words } символов
    [few] { $items } цели внутри складываются в { $words } символов
    [many] { $items } цели внутри складываются в { $words } символов
   *[other] { $items } цели внутри складываются в { $words } символов
}

new-work-goal-unit = Подсчитывать
new-work-goal-unit-hint = Используется для каждого слова или символа в этом проекте.

settings-group-goal-unit = Цели
settings-goal-unit-switch-title = Переключатель счетной единицы
settings-goal-unit-switch-text = Цели и вехи, уже поставленные для этого проекта, были введены в { $from }. Переключение на { $to } их не преобразует, поэтому каждое существующее число теперь будет читаться как { $to }.
settings-goal-unit-switch-informative = Ничего не потеряно. Переключитесь в любое время, чтобы восстановить первоначальное чтение, а затем обновите цели, которые вы хотите сохранить.
settings-goal-unit-switch-confirm = переключатель

goal-distribute-action = Дистрибутив …
distribute-title = Распределите цель
distribute-empty = Внутри этого нет ничего, чтобы разделить цель.
distribute-weight = Поделиться
distribute-weight-length = по длине
distribute-weight-rows = По кусочкам
distribute-weight-even = Поровну
distribute-overwrite = Заменить цели, которые уже установлены
distribute-col-item = Пункт
distribute-col-current = Теперь
distribute-col-proposed = После
distribute-total = Они составляют до { $total }, против цели { $goal }.
distribute-over-budget = Цели, уже установленные внутри, достигают { $over } больше, чем у этого контейнера. Поднимите его цель, опустите их или замените их.
distribute-apply = распределять
distribute-cancel = Отмена

milestone-target-gone = Цель удалена
milestone-no-target = Нет цели
milestone-add = Добавить
milestone-add-label = Что должно быть достигнуто

pace-summary-title = Где находится книга
pace-summary-remaining = { $words }, чтобы пойти
pace-summary-open = Откройте план
pace-summary-close = Закрыть
pace-summary-dont-show = Не показывайте это при открытии.
pace-summary-menu = Написание плана …

menu-help-topics = &Помогите темам
menu-help-shortcuts = К&лавиатура Shortcuts…
menu-help-website = &Skribisto в Интернете
menu-help-report = &Сообщение о проблеме …
menu-help-check-updates = П&роверить обновления
menu-command-palette = &Командная палетка …

learn-help-topics = Темы помощи
learn-shortcuts = Сочетания клавиш
learn-website = Skribisto в Интернете

help-window-title = Справка
help-filter-topics = Темы фильтров
help-no-matching-topic = Никаких совпадений по теме.
help-topic-missing = Эта тема больше не доступна.
help-back = Назад
help-not-translated = Эта страница пока не переведена и отображается на английском языке.

help-section-getting-started = Начало работы
help-section-writing = Писать
help-section-reviewing = Обзор
help-section-exchanging = Вводить и выводить слова
help-section-keeping = Сохранение вашей работы в безопасности
help-section-extensions = Расширения

help-topic-getting-started = Ваш первый проект
help-topic-writing-model = Как складывается книга
help-topic-drafts-and-old-versions = Сохранить старый проект
help-topic-goals-and-pace = Цели и темпы
help-topic-comments = Комментарии
help-topic-round-trip = Отправить свою книгу читателю
help-topic-export = экспортировать
help-topic-import-documents = Импорт документов
help-topic-import-projects = Внедрение целого проекта в
help-topic-backups-and-versions = Резервные копии и версии

help-shortcuts-title = Сочетания клавиш
help-shortcuts-filter = Фильтровать ярлыки
help-shortcuts-no-matches = Никаких ярлыков.
help-shortcuts-rebind = Изменение ярлыков …
help-shortcuts-close = Закрыть

command-palette-placeholder = Введите команду

shortcut-name-binder-duplicate = Дублировать
shortcut-name-comments-add = Добавить комментарий
shortcut-name-comments-add-paragraph = Комментарий к пункту
shortcut-name-spellcheck-toggle = Проверить правописание
shortcut-name-editor-tab-close = Закрыть вкладку
shortcut-name-editor-tab-pin = Закрепить или открепить вкладку
shortcut-name-editor-save = Сохранить
shortcut-name-work-export = Экспорт …
shortcut-name-work-new = Новый проект
shortcut-name-work-open = Открытый проект
shortcut-name-window-new = Новое окно
shortcut-name-work-close = Закрыть проект
shortcut-name-app-settings = Настройки
shortcut-name-app-quit = Выход
shortcut-name-editor-insert-footnote = Включить сноску
shortcut-name-format-scene-break = Вставить Scene Break
shortcut-name-format-major-scene-break = Вставить Major Scene Break
shortcut-name-format-link = Включить ссылку
shortcut-name-format-mark-nbsp = Включить No-break Space
shortcut-name-format-mark-nbhyphen = Включить No-break Hyphen
shortcut-name-format-mark-nnbsp = Узкое пространство No Break Space
shortcut-name-format-mark-zwsp = Включить пространство нулевой ширины
shortcut-name-go-next = Далее
shortcut-name-go-prev = Предыдущее
shortcut-name-go-to = Сходи
shortcut-name-outline-toggle = Показать или скрыть структуру
shortcut-name-preview-toggle = Показать или скрыть предпросмотр
shortcut-name-view-fullscreen = Переключить полноэкранный режим
shortcut-name-view-focus-mode = Toggle Distraction-Free режим
shortcut-name-editor-size-increase = Увеличить размер текста
shortcut-name-editor-size-decrease = Уменьшить размер текста
shortcut-name-editor-size-reset = Перезагрузить размер текста
shortcut-name-editor-find = Найти
shortcut-name-editor-replace = Заменить
shortcut-name-editor-find-next = Следующий матч
shortcut-name-editor-find-prev = Предыдущий Матч
shortcut-name-search-show = Поиск в проекте
shortcut-name-search-replace = Заменить в проекте
shortcut-name-outline-open-to-side = Откройте на стороне
shortcut-name-help-topics = Помогите темам
shortcut-name-help-shortcuts = Клавишные ярлыки
shortcut-name-help-website = Skribisto в Интернете
shortcut-name-help-report = Сообщить о проблеме
shortcut-name-help-check-updates = Проверить обновления
update-check-behind = Доступна версия { $version }.
update-check-current = Вы используете новейшую версию.
update-check-failed = Не удалось проверить на более новую версию. { $error }
shortcut-name-command-palette = Командная палитра
help-section-reference = Что такое вещи

help-concept-scene-break = Сценический прорыв
help-concept-major-scene-break = Основная сцена прорыва
help-concept-find-in-prose = Искать в тексте
help-concept-story-bible = Энциклопедия истории
help-concept-goal-unit = Слова или персонажи
help-concept-goal-progress = Прогресс
help-concept-manuscript-words = Что считается рукописью
help-concept-exportable = Выброшены из экспорта
help-concept-distribute = Делиться целью
help-concept-subtree-total = К чему приведут цели внутри
help-concept-milestone = Веха
help-concept-pace-plan = План работы

help-concept-tag = Метка
help-concept-label = Подпись
help-concept-point-of-view = Точка зрения
help-concept-epigraph = Эпиграф
help-concept-footnote = Сноска
help-concept-chapter-mode = Форма головы
help-concept-comment = Комментарий
help-concept-backup = Резервное копирование
help-concept-version = Версия
help-concept-trash = Корзина
help-concept-spellcheck = Проверка заклинаний
help-concept-search-replace = Поиск и замена
help-concept-note-template = Запись шаблона
help-concept-story-bible-entry = Библейская история Вступление
help-concept-text-replacement = Замена текста
help-concept-smart-punctuation = Умная пунктуация
help-concept-export-style = Экспортный стиль
help-concept-round-trip-marks = Круглые дорожные маркеры

epigraph-mark-attribution = Источник:
epigraph-mark-attribution-tip = Отметьте строку, в которой находится попечитель, как источник цитаты, поэтому она печатается как атрибуция.
margin-lane-name = Маржинальные отметки
margin-lane-provider-comments = Комментарии
margin-lane-provider-story-bible = Библейская история Вступление
margin-lane-provider-story-bible-hint = На заметке В прозаическом чтении, где эта запись названа, и сцены, рассказанные ее глазами. Также отмечает эти имена в самой прозе.
margin-lane-mark-point-of-view = Сказано отсюда
margin-lane-mark-named-here = Названо здесь
margin-lane-provider-comments-hint = Когда примечание прилагается к тексту
margin-lane-provider-search = Поисковые хиты
margin-lane-provider-search-hint = Каждый удар за то, что вы в последний раз искали, где бы он ни находился в документе.
margin-lane-provider-boundaries = Где начинается каждый документ
margin-lane-provider-boundaries-hint = Правило в верхней части каждой сцены в потоке, чтобы вы могли видеть, где вы находитесь.
margin-lane-provider-spelling = Правописание
margin-lane-provider-spelling-hint = Если только вы не попросите об этом: это отметит мнение машины о вашей прозе.
margin-lane-spelling = { $word }, возможная опечатка
margin-lane-search-hit = { $text }, матч { $index } из { $total }
margin-lane-search-current = { $text }, матч { $index } из { $total }, тот, на котором вы находитесь
margin-lane-boundary = Начало работы { $title }
margin-lane-boundary-untitled = Безымянный документ
settings-page-margin-lane = Маржинальные отметки
settings-desc-margin-lane = Полоса рядом с полосой прокрутки, и что она показывает
settings-margin-lane-enabled = Показать маржинальную полосу
settings-margin-lane-enabled-hint = Такой же переключатель, как и маржинальные отметки View ▸.
settings-margin-lane-enabled-more = Он никогда не говорит, что что-то не так: он показывает вам, где вещи, и оставляет вам то, что с ними делать.
settings-group-margin-lane-marks = Что это значит
settings-margin-lane-no-providers = Пока ничего не указывает на полосу.
settings-group-margin-lane-texture = Текстура диалога
settings-margin-lane-texture = Показать текстуру диалога
settings-margin-lane-texture-hint = Один бар за абзац: как долго он длится и сколько на нем говорят.
settings-margin-lane-texture-more = Не измеряется для языков без кураторской конвенции, где строка показывает длину в одиночку.
settings-group-margin-lane-surfaces = Где появляется
settings-margin-lane-surface-editor = Текстовый редактор
settings-margin-lane-surface-stream = потоки
settings-margin-lane-surface-search-preview = Предварительный поиск
menu-margin-lane = &Маржинальные отметки

status-preset-drafting = Подготовка проекта
status-preset-passes = Пересмотр пропусков
status-preset-plume = Plume Creator
status-todo = Делать
status-draft = Проект проекта
status-revised = пересмотренный
status-final = Финал
status-outline = Структура
status-first-edit = 1-е редактирование
status-second-edit = 2-е редактирование
status-done = Готово
status-plume-draft-1 = 1-й проект
status-plume-draft-2 = 2-й проект
status-plume-draft-3 = 3-й проект
status-plume-edit-1 = 1-е редактирование
status-plume-edit-2 = 2-е редактирование
status-plume-edit-3 = Третья редактура
status-plume-proofread = Корректура
status-plume-finished = Закончено
new-work-statuses = Рабочий процесс
new-work-statuses-hint = Сцены, через которые вы перемещаете сцену, вы можете переименовать, переупорядочить или добавить к ним в любое время.
status-none = Нет статуса
inspector-status = статус
overview-col-status = статус
overview-status-mixed = Приведенные ниже разделы не согласны
help-concept-status = статус
status-completion-title = Где находится книга
status-completion-headline = { $done } из { $total } сцен завершены
status-completion-empty = Это читает рукопись, так что она заполняется, когда вы пишете.
status-completion-open = Где находится книга …

settings-page-statuses = Статус
settings-desc-statuses = Переименовать, переупорядочить, добавить и удалить этапы, через которые вы перемещаете сцену.
settings-statuses-add = Добавить статус
settings-statuses-add-placeholder = Новое статусное имя
settings-statuses-desc =
    Этапы, через которые проходит сцена, по порядку, от наименее завершенных наверху до большинства.
    Этот порядок - то, что "менее закончено, чем" означает везде.
    В приложении, так что поместите их так, как ваш процесс на самом деле работает.
settings-statuses-apply-preset = Применять пресет …
settings-statuses-preset-applied = { $added ->
    [one] { $added } Статус добавлен
    [few] Добавлено { $added } статуса
    [many] Добавлено { $added } статусов
   *[other] Добавлены { $added } статусов
}
settings-statuses-preset-refused = В этом проекте уже есть лестница. Предустановки заполняют только пустую. Сначала удалите ступени, которые вам не нужны.
settings-statuses-duplicate = "{ $name }" уже на этой лестнице
settings-statuses-added = Добавлено "{ $name }"
settings-statuses-move-up = Встать (менее закончено)
settings-statuses-move-down = Спускайся (больше готово)
settings-statuses-details-placeholder = Что означает этот этап (факультативно)
settings-statuses-delete = Исключить "{ $name }"
settings-statuses-delete-in-use = { $count ->
    [one] Удалить "{ $name }": { $count } пункт на нем и потеряет свой статус
    [few] Удалить "{ $name }": на нем { $count } предмета и потеряют статус
    [many] Удалить "{ $name }": на нем { $count } предметов и потеряют статус
   *[other] Удалить "{ $name }": на нем находятся { $count } предметов и потеряют статус
}
settings-statuses-deleted = Исключено "{ $name }"
settings-statuses-deleted-in-use = { $count ->
    [one] Исключено "{ $name }". { $count } пункт больше не имеет статуса.
    [few] Исключено "{ $name }". { $count } пункта больше не имеют статуса.
    [many] Исключено "{ $name }". { $count } позиций больше не имеют статуса.
   *[other] Исключено "{ $name }". { $count } Пункты больше не имеют статуса.
}
settings-statuses-empty-title = Пока нет рабочего процесса
settings-statuses-empty-body = Статус говорит, как далеко находится сцена. Добавьте одну выше или начните с предустановленного.
status-category-planned = запланированный
status-category-drafting = Подготовка проекта
status-category-needs-work = Нужна работа
status-category-revised = пересмотренный
status-category-final = Финал
