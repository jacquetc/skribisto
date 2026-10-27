# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

menu-work = П&роект
menu-new-work = Н&овый проект
menu-open-work = О&ткрыть проект…
menu-new-window = &Новое окно
menu-import-from = &Импортировать из
menu-create-from = &Создать из
menu-import-plume = &Plume Creator (.plume)…
menu-import-document = &Документы (Markdown, Word, ODT)…
menu-import-manuskript = &Manuskript (.msk)…
menu-import-scrivener = &Scrivener (.scriv)…
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
menu-scene-break = Вставить &разрыв сцены
menu-major-scene-break = Вставить &большой разрыв сцены
menu-line-break = Встав&ить разрыв строки

menu-go = П&ереход
menu-go-next-scene = &Следующая сцена
menu-go-prev-scene = &Предыдущая сцена
menu-go-next-chapter = С&ледующая глава
menu-go-prev-chapter = П&редыдущая глава
menu-go-next-note = Сл&едующая заметка
menu-go-prev-note = Пре&дыдущая заметка

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
ctx-open-to-side = Открыть &сбоку
ctx-open = &Открыть
ctx-reveal-in-outline = &Показать в структуре
ctx-move-up = Переместить &вверх
ctx-move-down = Переместить в&низ

ctx-tab-close = &Закрыть
ctx-tab-close-others = За&крыть остальные
ctx-tab-close-all = З&акрыть все
ctx-tab-open-to-side = Открыть &сбоку
ctx-tab-open-in-main = Открыть в &основной панели
ctx-tab-move-to-side = &Переместить в боковую панель
ctx-tab-move-to-main = &Переместить в основную панель
ctx-tab-move-to-new-window = Переместить в &новое окно
ctx-tab-move-window-unsaved = Сначала сохраните проект: второе окно открывает файл на диске
ctx-tab-pin = Закр&епить эту вкладку
ctx-tab-unpin = Откр&епить эту вкладку
tab-pinned-tooltip = Закреплена: команды «Закрыть остальные» и «Закрыть все» её не закрывают

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
create-story-bible-entry = Запись библии истории…
type-book-start = Начало книги
type-text = Текст

new-item-book = Новая книга
new-item-part = Новая часть
new-item-chapter = Новая глава
new-item-note = Новая заметка
new-item-note-folder = Новая папка заметок
new-item-folder = Новая папка
new-item-paratext = Новый паратекст
new-item-paratext-folder = Новая папка паратекстов
new-item-story-bible-entry = Новая запись библии истории

placement-inside = внутри
placement-after = после
placement-after-parent = после родительского элемента
placement-top-level = верхний уровень


ctx-promote = &Преобразовать в
promote-chapter-folder = Глава-папка
promote-flat-chapter = Плоская глава
promote-lossy-title = Негде хранить текст
promote-lossy-text = У типа «{ $target }» негде хранить следующее: { $kinds }. Сначала переместите или очистите этот текст, затем выполните преобразование.
content-scene-text = Текст сцены
content-note-text = Текст заметки
content-book-title = Название книги
content-book-subtitle = Подзаголовок книги
content-part-title = Название части
content-chapter-title = Название главы
content-epigraph-text = Эпиграф
content-paratext-text = Паратекст
promote-blocked-title = Глава не пустая
promote-blocked-text = { $count ->
    [one] В этой главе ещё есть { $count } элемент. Переместите его или отправьте в корзину, прежде чем преобразовать главу в плоскую.
    [few] В этой главе ещё есть { $count } элемента. Переместите их или отправьте в корзину, прежде чем преобразовать главу в плоскую.
    [many] В этой главе ещё есть { $count } элементов. Переместите их или отправьте в корзину, прежде чем преобразовать главу в плоскую.
   *[other] В этой главе ещё есть { $count } элемента. Переместите их или отправьте в корзину, прежде чем преобразовать главу в плоскую.
}

inspector = Инспектор
inspector-empty = Откройте элемент, чтобы увидеть его свойства.
inspector-promote = Преобразовать в…
statusbar-toggle-outline = Показать или скрыть структуру
statusbar-toggle-inspector = Показать или скрыть инспектор
statusbar-save-unsaved = Есть несохранённые изменения. Щёлкните, чтобы сохранить
statusbar-save-saved = Все изменения сохранены
statusbar-save-autosave = Автосохранение включено. Изменения сохраняются по ходу работы
statusbar-saving = Сохранение…
statusbar-word-count = { $count ->
    [one] { $count } слово
    [few] { $count } слова
    [many] { $count } слов
   *[other] { $count } слова
}

statusbar-word-count-tooltip = Количество слов в редактируемой сцене
statusbar-word-char-count = { $words ->
    [one] { $words } слово
    [few] { $words } слова
    [many] { $words } слов
   *[other] { $words } слова
} · { $chars ->
    [one] { $chars } знак
    [few] { $chars } знака
    [many] { $chars } знаков
   *[other] { $chars } знака
}

session-toggle = Сеанс письма: начать или приостановить спринт
session-configure = Задать цель сеанса по словам и ограничение времени
session-configure-title = Сеанс письма
session-word-goal = Цель по словам
session-time-limit = Ограничение времени
session-time-limit-unit = мин.
session-no-goal = Без цели
session-no-limit = Без ограничения
session-reset = Сбросить сеанс
session-readout = { $words ->
    [one] { $words } слово
    [few] { $words } слова
    [many] { $words } слов
   *[other] { $words } слова
} · { $time }
session-readout-timed = { $words ->
    [one] { $words } слово
    [few] { $words } слова
    [many] { $words } слов
   *[other] { $words } слова
} · осталось { $time }
statusbar-focus-exit = Выйти из режима без отвлечений
statusbar-focus-synopsis = Синопсис
statusbar-focus-comments = Комментарии
statusbar-focus-go-prev = Предыдущий элемент (Alt+Вверх)
statusbar-focus-go-next = Следующий элемент (Alt+Вниз)

rail-settings = Настройки

language = Язык
theme = Тема
english = Английский
french = Французский
light = Светлая
dark = Тёмная
settings-text-width = Ширина текста
settings-preview-width = Ширина предпросмотра поиска
settings-autosave = Автосохранение на диск
settings-show-welcome = Показывать окно запуска при старте
settings-show-welcome-tip = Если выключено, вместо него открывается последний проект.
settings-check-for-updates = Проверять наличие новых версий
settings-check-for-updates-tip = Не чаще раза в день Skribisto спрашивает у сайта проекта, какая версия актуальна. Запрос не содержит ни идентификатора, ни даже номера используемой вами версии: сравнение выполняется на этом компьютере.

settings-title = Настройки
settings-close = Закрыть
settings-search = Поиск в настройках
settings-reset = Восстановить настройки по умолчанию
settings-done = Готово
settings-cancel = Отмена
settings-apply = Применить
settings-ok = ОК
settings-reset-confirm-title = Сбросить все настройки к значениям по умолчанию?
settings-reset-confirm-body = Восстанавливает настройки внешнего вида, редактора и письма этого приложения. Собственные настройки проекта, сочетания клавиш и сохранённые стили не затрагиваются. Это действие нельзя отменить.
settings-empty-title = Здесь пока нет настроек
settings-empty-hint = Параметры этого раздела появятся в одном из будущих обновлений.

settings-sec-appearance-behaviour = Внешний вид и поведение
settings-sec-editor = Редактор
settings-sec-spelling = Орфография
settings-sec-backup = Резервное копирование и синхронизация
settings-sec-compile = Сборка и экспорт
settings-page-appearance = Внешний вид
settings-field-image-size-policy = Большие изображения
settings-image-policy-ask = Спрашивать каждый раз
settings-image-policy-keep = Сохранять оригинал
settings-image-policy-downscale = Оптимизировать
settings-hint-image-size-policy = Что делать, если вставленное изображение больше, чем нужно рукописи.
settings-page-notifications = Уведомления
settings-page-scene = Сцена
settings-page-synopsis = Синопсис
settings-page-notes = Заметки
settings-page-editor-behavior = Поведение редактора
settings-page-goals = Цели и подсчёт слов
settings-page-games = Писательские игры
settings-page-corkboard = Пробковая доска
settings-page-distraction-free = Без отвлечений
settings-page-dictionaries = Словари
settings-page-autosave = Автосохранение
settings-page-export = Форматы экспорта
settings-page-paratext = Структуры паратекста
settings-page-keymap = Сочетания клавиш
settings-keymap-filter = Фильтр сочетаний клавиш

settings-desc-sec-appearance-behaviour = Как выглядит само приложение и что оно делает при запуске.
settings-desc-sec-editor = Область письма: как она выглядит и как ведёт себя при вводе.
settings-desc-sec-spelling = Проверка орфографии и словари, которые она использует.
settings-desc-sec-backup = Как ваша работа попадает на диск и какие копии сохраняются.
settings-desc-sec-compile = Что выходит из Skribisto и в каком виде.
settings-desc-sec-work = Настройки, которые принадлежат этому проекту и хранятся в его файле.
settings-desc-sec-extensions = Страницы, добавленные расширениями, установленными здесь.
settings-desc-group-typography = По странице на каждый вид текста: шрифт, размер, высота строки и интервалы.
settings-desc-appearance = Язык интерфейса, тема, размер текста и окно запуска при старте.
settings-desc-notifications = Все сообщения, показанные за этот сеанс, и действия, которые можно повторить.
settings-desc-scene = Оформление прозы сцен: шрифт, размер, высота строки, отступы и интервалы.
settings-desc-synopsis = Оформление панели синопсиса, независимо от рукописи.
settings-desc-notes = Оформление заметок, независимо от рукописи.
settings-desc-corkboard = Размер карточек, что на них показано и как доска их располагает.
settings-desc-distraction-free = Типографика, ширина колонки и панель управления для письма во весь экран.
settings-desc-distraction-free-themes = Библиотека тем для письма во весь экран: встроенные и ваши собственные.
settings-desc-editor-behavior = Ширина текста, положение синопсиса, прокрутка как в пишущей машинке и подсветка у курсора.
settings-desc-punctuation = Типографские правила, с которых начинаются новые проекты. Каждый проект следует им, если не задаёт собственные.
settings-desc-goals = Цели по словам и знакам и способ подсчёта слов.
settings-desc-games = Ограничения, которые вы ставите себе при написании черновика, например «Только вперёд».
settings-desc-spellcheck = Единый переключатель, который включает и выключает проверку орфографии повсюду.
settings-desc-dictionaries = Установка, удаление и просмотр орфографических словарей на этом компьютере.
settings-desc-autosave = Записываются ли ваши правки на диск автоматически.
settings-desc-backup = Когда создаются копии, где они хранятся и сколько их. Каждый проект следует этим правилам, если не задаёт собственные.
settings-desc-export = Стили, через которые проходит любой экспорт: встроенные и ваши собственные.
settings-desc-paratext = Вступительные и заключительные материалы, с которыми может начинаться новый проект.
settings-desc-keymap = Все сочетания клавиш и команды, к которым они привязаны.
settings-page-user = Пользователь
settings-desc-user = Кто вы — для подписи ваших комментариев.
settings-group-identity = Личные данные
settings-field-user-name = Ваше имя
settings-field-user-name-placeholder = Необязательно
settings-field-user-name-hint = Этим именем подписываются ваши комментарии и ответы. Оно не совпадает с именем автора проекта: то печатается на книге и хранится в файле проекта.
settings-field-user-initials = Ваши инициалы
settings-field-user-initials-placeholder = Из вашего имени
settings-field-user-initials-hint = То, что текстовый процессор показывает рядом с вашим комментарием на полях. Оставьте пустым, чтобы использовать показанные инициалы, взятые из вашего имени.
settings-field-user-hint = Оба поля необязательны и действуют для всех проектов на этом компьютере. Изменение касается только комментариев, которые вы напишете после него. Уже написанные комментарии сохраняют прежнюю подпись.
settings-desc-author = Имя, которое стоит на этом проекте.
settings-desc-structure = Являются ли главы этого проекта папками или отдельными элементами.
settings-desc-language = Язык, по которому проверяется проза этого проекта.
settings-desc-work-backup = Собственные правила резервного копирования этого проекта или общие.
settings-desc-personal-dictionary = Слова, которые этот проект считает написанными правильно.
settings-desc-tags = Цветные теги, которыми этот проект помечает свои элементы.
settings-desc-templates = Шаблоны заметок, которые можно вставлять при работе над этим проектом.
settings-desc-text-replacements = Сокращения, которые раскрываются по мере ввода в этом проекте.
settings-desc-work-punctuation = Кавычки, тире и пробелы, которым следует проза этого проекта.

settings-group-typography = Типографика
settings-group-writing-column = Колонка текста
settings-group-writing-view = Область письма
settings-group-distraction-free-strip = Панель управления
settings-group-theme = Тема
settings-group-language = Язык
settings-group-startup = Запуск
settings-group-updates = Обновления
settings-group-autosave = Автосохранение
settings-field-typeface = Шрифт
settings-field-size = Размер текста

editor-size-changed-manuscript = Размер текста рукописи: { $percent }
editor-size-changed-synopsis = Размер текста синопсиса: { $percent }
editor-size-changed-notes = Размер текста заметок: { $percent }
editor-size-changed-corkboard = Размер текста карточек доски: { $percent }
editor-size-changed-corkboard-expanded = Размер текста развёрнутого редактора: { $percent }
editor-size-changed-distraction-free = Размер текста в режиме без отвлечений: { $percent }

menu-text-size-increase = &Увеличить размер текста
menu-text-size-decrease = Уме&ньшить размер текста
menu-text-size-reset = С&бросить размер текста
settings-field-line-height = Высота строки
settings-field-first-line-indent = Отступ первой строки
settings-field-paragraph-spacing-before = Интервал перед абзацем
settings-field-paragraph-spacing-after = Интервал после абзаца
settings-field-column-width = Ширина колонки
settings-distraction-free-width-hint = Действует только в режиме без отвлечений. Ширина колонок сцен, синопсиса и заметок не меняется.
settings-distraction-free-title = Оставить название элемента
settings-distraction-free-word-count = Оставить счётчик слов
settings-distraction-free-session = Оставить сеанс письма
settings-distraction-free-go-to = Оставить кнопку «Перейти к…»
settings-distraction-free-go = Оставить кнопки «Предыдущий» и «Следующий»
settings-distraction-free-chrome-hint = Кнопка выхода остаётся всегда, что бы вы здесь ни выбрали. Это ваш путь наружу, если клавиша Escape занята.
settings-field-app-theme = Тема
settings-theme-light = Светлая
settings-theme-dark = Тёмная
settings-field-text-scale = Размер текста интерфейса
settings-field-language = Язык интерфейса
settings-synopsis-placement = Положение синопсиса
settings-synopsis-placement-none = Нет
settings-synopsis-placement-top = Сверху
settings-synopsis-placement-side = Сбоку
settings-typewriter = Прокрутка как в пишущей машинке
settings-typewriter-tip = Удерживает строку, которую вы пишете, на постоянной высоте, а рукопись прокручивается под ней. Щелчок по-прежнему ставит курсор туда, куда вы щёлкнули.
settings-typewriter-position = Положение строки
settings-typewriter-position-top-third = Верхняя треть
settings-typewriter-position-middle = Середина
settings-typewriter-position-bottom-quarter = Нижняя четверть
settings-highlight-scope = Подсветка вокруг курсора
settings-highlight-scope-none = Нет
settings-highlight-scope-sentence = Предложение
settings-highlight-scope-paragraph = Абзац
settings-highlight-scope-tip-none = Страница остаётся чистой. Во время письма ничего не подсвечивается.
settings-highlight-scope-tip-sentence = Подсвечивать предложение, которое вы пишете, чтобы оно выделялось среди соседних.
settings-highlight-scope-tip-paragraph = Подсвечивать весь абзац, который вы пишете, чтобы фрагмент, над которым вы работаете, оставался на виду.
settings-group-container-views = Представления контейнеров
settings-remember-view = Запоминать последнее представление для каждого типа элемента
settings-remember-view-tip = Открывать контейнер в представлении, которое вы последним использовали для этого типа
settings-remember-view-tip-more =
    Книга, часть и глава предлагают несколько представлений (собственная страница,
    вся рукопись, весь синопсис). Если включить этот параметр, каждый тип будет
    открываться в представлении, выбранном для него последним: например, переключите
    одну главу на «Вся глава», и следующая открытая глава тоже покажет «Вся глава».
    Каждый тип запоминает своё представление.
settings-group-counting = Подсчёт слов
settings-counting-auto = Автоматически (по языку)
settings-counting-whitespace = Разделять по пробелам
settings-counting-unicode-words = Слова по Unicode
settings-counting-cjk-hybrid = С учётом CJK (по знакам)
settings-counting-hint = В автоматическом режиме китайский и японский текст считается по знакам, а все остальные языки — по словам. Меняйте этот параметр, только если подсчёт для вашего языка выглядит неверным.
settings-group-goals-display = Отображение
settings-show-characters = Показывать количество знаков в строке состояния
settings-autosave-hint = Изменения автоматически записываются на диск по ходу работы.

settings-sec-work = Проект
settings-page-structure = Устройство проекта
settings-page-author = Автор
settings-field-author-name = Имя автора
settings-field-author-placeholder = Необязательно
settings-field-author-hint = Выводится на титульном листе при сборке и в метаданных экспортированных файлов. Оставьте пустым, чтобы не выводить. Это имя автора книги. Имя, которым подписываются ваши комментарии, задаётся в разделе Настройки ▸ Пользователь.
settings-group-chapters = Главы
tidy-titles-title = Упорядочить названия глав
tidy-titles-none = Ни у одной главы или части название не повторяет её собственный номер.
tidy-titles-lead = { $count ->
        [one] { $count } глава или часть названа только собственным номером.
        [few] { $count } главы и части названы только собственными номерами.
        [many] { $count } глав и частей названы только собственными номерами.
       *[other] { $count } главы и части названы только собственными номерами.
    }
tidy-titles-explain = После очистки этих названий каждая глава будет обозначаться номером, который книга уже знает, — тем же, что печатается при экспорте. Больше ничего не меняется, а одна отмена вернёт всё обратно.
menu-document-tidy-titles = Упорядочить &названия глав…
settings-group-numbering = Нумерация
settings-number-chapters = Нумеровать главы и части
settings-number-chapters-tip = Главы и части получают номер, вычисленный по их месту в книге; он показывается здесь рядом с названием и печатается при экспорте.
settings-number-chapters-tip-more = Номер никогда не хранится в названии, поэтому он остаётся верным, когда вы переставляете, вставляете или удаляете главы. Если выключить этот параметр, экспорт печатает только названия, что бы ни требовал стиль экспорта. Чтобы исключить одну главу (пролог, интерлюдию), используйте переключатель «Нумерация» в инспекторе: глава останется в книге, но не получит номера.
settings-part-resets-chapter = Начинать нумерацию глав заново в каждой части
settings-part-resets-chapter-tip = По умолчанию выключено: главы нумеруются сквозь все части книги, так что «Часть вторая» открывается главой одиннадцатой.
settings-part-resets-chapter-tip-more = Это обычная издательская практика, и читатель ожидает именно её. Включите параметр для книги, части которой задуманы как отдельные тома, каждый из которых начинается с первой главы.
settings-chapter-flat = Плоские главы
settings-chapter-flat-hint = Включено: глава — это один элемент. Вы пишете в неё, и она не содержит сцен. Выключено: глава — это папка. Вы по-прежнему пишете в неё, но она может также содержать сцены. Новые главы следуют этому параметру; существующие преобразуются командой «Преобразовать в».

settings-styles-builtin = Встроенные стили
settings-styles-user = Мои стили
settings-styles-builtin-badge = Встроенный
settings-styles-duplicate = Дублировать
settings-styles-edit = Изменить
settings-styles-delete = Удалить
settings-styles-import = Импорт…
settings-styles-export = Экспорт…
settings-styles-copy-suffix = (копия)
settings-styles-json-filter = Стиль экспорта
settings-styles-editor-title = Изменить стиль
settings-styles-editor-none = Выберите пользовательский стиль для редактирования или создайте копию встроенного.
settings-styles-imported = Стиль импортирован
settings-styles-import-failed = Не удалось импортировать стиль
settings-styles-exported = Стиль экспортирован
settings-styles-export-failed = Не удалось экспортировать стиль
settings-styles-field-name = Название
settings-styles-field-chapters = Заголовки глав
settings-styles-field-parts = Заголовки частей
settings-styles-field-scene-break = Разрыв сцены
settings-styles-field-major-scene-break = Большой разрыв сцены
settings-styles-field-spacing = Межстрочный интервал
settings-styles-field-justify = Выравнивать по ширине
settings-styles-field-notes = Включать заметки
settings-styles-field-synopses = Включать синопсисы
settings-styles-field-scene-titles = Включать названия сцен
settings-styles-field-epigraphs = Включать эпиграфы
settings-styles-field-paratexts = Включать паратексты
settings-styles-field-epigraph-placement = Положение эпиграфа
settings-styles-epigraph-after = После заголовка
settings-styles-epigraph-before = Перед заголовком
settings-styles-field-footnotes = Включать сноски
settings-styles-field-footnote-numbering = Нумерация сносок
settings-styles-footnote-numbering-continuous = Сквозная
settings-styles-footnote-numbering-per-chapter = Заново в каждой главе
settings-styles-footnote-numbering-per-book = Заново в каждой книге
settings-styles-field-images = Изображения
menu-image = И&зображение
settings-styles-images-beside = Рядом с документом
settings-styles-images-embed = Внутри документа
settings-styles-images-omit = Не включать
settings-styles-group-round-trip = Отправка редактору
settings-styles-field-comments = Включать комментарии
settings-styles-field-round-trip-marks = Включать метки обратного импорта
settings-styles-round-trip-hint = Только DOCX и ODT. Метки — это невидимые идентификаторы, благодаря которым вернувшийся файл обновляет этот проект, а не ложится рядом второй копией.
settings-styles-group-pages = Страницы
settings-styles-field-cover = Начинать с обложки
settings-styles-field-word-count = Количество слов на титульном листе
settings-styles-field-page-books = Новая страница перед каждой книгой
settings-styles-field-page-parts = Новая страница перед каждой частью
settings-styles-field-page-chapters = Новая страница перед каждой главой
settings-styles-field-page-paratexts = Новая страница перед каждым паратекстом
settings-styles-heading-none = Без заголовка
settings-styles-heading-numbered = Только номер
settings-styles-heading-title = Только название
settings-styles-heading-both = Номер + название
settings-styles-break-blank = Пустая строка
settings-styles-break-none = Нет
settings-styles-spacing-single = Одинарный
settings-styles-spacing-onehalf = 1½
settings-styles-spacing-double = Двойной

welcome-title = Добро пожаловать в Skribisto
welcome-version = Версия { $version }
welcome-update-available = Доступна версия { $version }
welcome-update-tip = Открывает страницу загрузки в вашем браузере.
welcome-search = Поиск проектов
welcome-open = Открыть
welcome-new-work = Новый проект
welcome-create-from = Создать из…
welcome-recent-works = Недавние проекты
welcome-empty-recents = Недавних проектов пока нет.
welcome-no-matches = Ни один недавний проект не соответствует поиску.
welcome-learn-soon = Руководства и советы скоро появятся.
welcome-about-blurb = Skribisto — приложение для письма, переписанное на Rust + Teksilo.
welcome-tagline = *Тихое место, чтобы писать длинные тексты.*
welcome-github = GitHub
welcome-discord = Discord
nav-works = Проекты
nav-examples = Примеры
nav-learn = Обучение
nav-about = О программе

synopsis = Синопсис
epigraph = Эпиграф
pane-manuscript = Рукопись
corkboard = Пробковая доска
overview = Обзор

corkboard-card-count = { $count ->
    [one] { $count } карточка
    [few] { $count } карточки
    [many] { $count } карточек
   *[other] { $count } карточки
}
corkboard-child-count = { $count ->
    [one] { $count } элемент
    [few] { $count } элемента
    [many] { $count } элементов
   *[other] { $count } элемента
}
corkboard-view-nested = Вложенный
corkboard-view-flat = Плоский
corkboard-layout-hint = «Вложенный» показывает прямых потомков контейнера. Откройте карточку папки, чтобы войти в неё. «Плоский» показывает сразу все сцены контейнера.
corkboard-card-size = Размер карточек
corkboard-search-placeholder = Фильтр карточек…
corkboard-empty-title = Здесь пока ничего нет
corkboard-empty-hint = Нажмите «＋ Новый» выше, чтобы добавить первую.
corkboard-new = Новый
corkboard-show-card-numbers = Нумеровать карточки
corkboard-modal-size = Размер развёрнутого редактора
corkboard-scope-hint = Действует для всех открытых пробковых досок во всех проектах.
corkboard-sort-manuscript = Порядок рукописи
corkboard-sort-title-asc = Название: А–Я
corkboard-sort-title-desc = Название: Я–А
corkboard-card-number = Карточка { $number }

duplicate = Дублировать
duplicate-n = { $count ->
    [one] Дублировать { $count } карточку
    [few] Дублировать { $count } карточки
    [many] Дублировать { $count } карточек
   *[other] Дублировать { $count } карточки
}
set-label-n = { $count ->
    [one] Задать метку для { $count } карточки
    [few] Задать метку для { $count } карточек
    [many] Задать метку для { $count } карточек
   *[other] Задать метку для { $count } карточки
}
move-to-trash-n = { $count ->
    [one] Отправить { $count } карточку в корзину
    [few] Отправить { $count } карточки в корзину
    [many] Отправить { $count } карточек в корзину
   *[other] Отправить { $count } карточки в корзину
}
reveal-in-outline = Показать в структуре

corkboard-move-to = Переместить в…
corkboard-move-to-n = { $count ->
    [one] Переместить { $count } карточку в…
    [few] Переместить { $count } карточки в…
    [many] Переместить { $count } карточек в…
   *[other] Переместить { $count } карточки в…
}
corkboard-move-picker-title = Переместить в…
corkboard-move-picker-empty = Подшивок пока нет. Сначала создайте подшивку.
corkboard-move-picker-cancel = Отмена
corkboard-move-here = Переместить сюда
corkboard-moved-ok = { $count ->
    [one] { $count } карточка перемещена
    [few] { $count } карточки перемещены
    [many] { $count } карточек перемещено
   *[other] { $count } карточки перемещено
}
corkboard-move-into-self = Контейнер нельзя переместить внутрь самого себя. Выберите место назначения вне его.
corkboard-move-failed = Не удалось переместить туда эти карточки.

overview-col-title = Название
overview-col-type = Тип
overview-col-label = Метка
overview-col-tags = Теги
overview-col-books = Книги
overview-col-own-words = Слова
overview-col-total-words = Итого
overview-row-count = { $count ->
    [one] { $count } элемент
    [few] { $count } элемента
    [many] { $count } элементов
   *[other] { $count } элемента
}
overview-search-placeholder = Фильтр элементов…
overview-expand-all = Развернуть всё
overview-collapse-all = Свернуть всё
overview-table-label = Содержание
overview-empty-title = Здесь пока ничего нет
overview-gone-title = Этот контейнер больше не существует
overview-gone-hint = Он перемещён в корзину. Восстановите его или закройте эту вкладку.
overview-empty-hint = Нажмите «＋ Новый» выше, чтобы добавить первый элемент.
corkboard-grid-label = Карточки пробковой доски
corkboard-rename-field = Переименовать элемент
corkboard-expand-synopsis = Развернуть синопсис
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
settings-group-corkboard-layout = Компоновка
settings-group-corkboard-cards = Карточки
corkboard-show-word-count = Показывать количество слов на карточках
text-heading = Текст
no-content = У этого элемента нет редактируемого содержимого.
untitled = Без названия
placeholder-title = Название…
placeholder-subtitle = Подзаголовок…
placeholder-chapter-title = Название главы…
split-editor = Разделить редактор
close-split-view = Закрыть разделённый вид
drop-open-here = Открыть здесь
drop-open-to-side = Открыть сбоку

segment-chapter = Глава
segment-part = Часть
segment-book = Книга
segment-pace = Темп
segment-notes = Заметки
segment-story-bible = Библия истории
segment-note-own = Заметка
segment-note-details = Подробности
segment-note-in-prose = В прозе
pace-placeholder = Здесь появится планировщик темпа.
pace-empty-title = Спланируйте темп этой книги
pace-empty-body = Задайте цель по словам и срок, и Skribisto рассчитает дневной темп, который приведёт вас к цели.
pace-start-planning = Начать планирование
pace-section-schedule = Расписание
pace-section-progress = Прогресс
pace-advancement = Продвижение
pace-goal = Цель по словам
pace-deadline = Срок
pace-active = Темп включён
pace-day-mon = Пн
pace-day-tue = Вт
pace-day-wed = Ср
pace-day-thu = Чт
pace-day-fri = Пт
pace-day-sat = Сб
pace-day-sun = Вс
pace-card-written = слов написано
pace-card-of-goal = от цели
pace-card-rate = слов в рабочий день
pace-card-days-left = рабочих дней осталось
pace-card-streak = дней подряд
pace-card-ahead = слов с опережением темпа
pace-card-behind = слов с отставанием от темпа
pace-charts-empty = Графики прогресса появятся здесь, как только при сохранении будет записано количество слов.
pace-chart-progression = Написано слов и цель
pace-chart-words-per-day = Слов в день
pace-series-actual = Факт
pace-series-target = Цель
pace-series-words-per-day = Слов/день
pace-daily-target-line = Равномерный темп: { $count ->
    [one] { $count } слово/день
    [few] { $count } слова/день
    [many] { $count } слов/день
   *[other] { $count } слова/день
}
pace-section-holidays = Выходные
pace-section-milestones = Вехи
pace-holidays-none = Выходных нет. Учитывается каждый запланированный день.
pace-holiday-label = Название выходного
pace-add-holiday = Добавить
pace-remove = Убрать
pace-milestones-none = Вех пока нет. Задайте веху для части или главы в инспекторе.
full-chapter = Вся глава
full-part = Вся часть
full-book = Вся книга
full-synopsis = Весь синопсис
rename = Переименовать
set-label = Задать метку
insert-scene = Вставить сцену
insert-chapter = Вставить главу
split-scene = Разделить сцену
move-up = Переместить вверх
move-down = Переместить вниз
merge-with-previous = Объединить с предыдущим
move-to-trash = Отправить в корзину
rename-chapter = Переименовать главу
add-scene = Добавить новую сцену
add-chapter = Добавить новую главу
new-scene-title = Новая сцена
placeholder-scene-name = Название сцены…
menu-cut = Вырезать
menu-copy = Копировать
menu-paste = Вставить
menu-paste-unformatted = Вставить без форматирования
menu-select-all = Выделить всё
format-bold = Полужирный
format-italic = Курсив
format-underline = Подчёркнутый
format-strikethrough = Зачёркнутый

binder = Подшивка
no-work = Проект не загружен
no-recent-works = Нет недавних проектов
switcher-open-section = Сейчас открыты
switcher-recent-section = Недавние
switcher-this-window = это окно
open-project-title = Открытие проекта
open-project-question = Как открыть «{ $title }»?
open-in-new-window = Открыть в новом окне
open-here = Открыть здесь

binder-all = Все подшивки
binder-show-all = Показать все подшивки
binder-new = Новая подшивка…
binder-default-name = Новая подшивка
binder-item-count = { $count ->
    [one] { $count } элемент
    [few] { $count } элемента
    [many] { $count } элементов
   *[other] { $count } элемента
}
binder-search-placeholder = Фильтр структуры…
binder-search-scope = Искать во всех подшивках
binder-trash-confirm-title = Отправить подшивку в корзину?
binder-trash-confirm-text = «{ $name }» и все её элементы будут перемещены в корзину.

dialog-rename = Переименовать
dialog-new-binder = Новая подшивка
dialog-set-label = Задать метку
dialog-new-scene = Новая сцена
close-work-question = Сохранить изменения перед закрытием проекта?
quit-question = Сохранить изменения перед выходом?
quit-save-work-question = Сохранить изменения в «{ $title }» перед выходом?
unsaved-changes = В этом проекте есть несохранённые изменения.
new-work-unsaved-question = Сохранить изменения перед созданием нового проекта?
open-work-unsaved-question = Сохранить изменения перед открытием другого проекта?
switch-backup-discard-title = Отменить изменения в этой резервной копии?
switch-backup-discard-text = Изменения в резервной копии нельзя сохранить в неё саму. Чтобы сохранить их, используйте «Сохранить как» или «Восстановить», либо отмените их и откройте другой проект.
switch-save-failed = Не удалось сохранить проект, поэтому он не был заменён: { $error }
switch-save-not-started = Не удалось сохранить проект, поэтому он не был заменён.
close-save-failed = Не удалось сохранить проект, поэтому он не был закрыт: { $error }
close-save-not-started = Не удалось сохранить проект, поэтому он не был закрыт.
save-not-started = Не удалось сохранить проект.

could-not-open-work = Не удалось открыть «{ $file }»
could-not-open-work-too-new = Для «{ $file }» нужна более новая версия Skribisto
could-not-open-work-too-new-detail = Сохранено в формате Skribisto { $written_by }; для открытия нужен формат { $requires } или новее, а эта сборка поддерживает форматы до { $supported }. Обновите Skribisto, чтобы открыть этот проект.
could-not-open-work-importing = В «{ $file }» сейчас идёт запись импорта
could-not-open-work-importing-detail = Импорт записывает данные поверх этого проекта и ещё не завершён. Откройте проект, когда импорт закончится: если открыть его сейчас, вы увидите проект, который импорт заменяет, а следующее сохранение запишет его обратно поверх результата импорта.
could-not-open-example = Не удалось открыть пример: { $error }
could-not-open-link = Не удалось открыть { $url }: { $error }
could-not-create-work = Не удалось создать проект: { $error }
saving-as-file = Сохранение как { $target }…
saving-as-folder = Сохранение как { $target }/…
saved-as = Сохранено в { $target }
save-error = Не удалось сохранить: { $error }
save-flush-failed = Сохранение выполнено, но эти документы не удалось в него передать, и они всё ещё не сохранены: { $titles } ({ $error })
backup-error = Не удалось создать резервную копию: { $error }
backing-up = Резервное копирование…
backup-nothing-open = Нет открытого проекта для резервного копирования.
backup-already-running = Резервное копирование уже выполняется.
backup-complete = Резервное копирование завершено (сохранено: { $ok }, уже актуальны: { $skipped })
backup-partial = Резервное копирование завершено, сохранено: { $ok }. { $failed ->
    [one] { $failed } место назначения недоступно
    [few] { $failed } места назначения недоступны
    [many] { $failed } мест назначения недоступны
   *[other] { $failed } места назначения недоступны
}
backup-no-destination-title = Нет доступного места для резервной копии
backup-no-destination-text = Ни одно из настроенных мест резервного копирования недоступно (например, внешний диск может быть отключён). Подключите его и повторите попытку или выйдите без резервного копирования.
backup-failed-close-title = Не удалось сохранить резервную копию
backup-failed-close-text = Перед закрытием не удалось записать ни одной резервной копии. Все места назначения дали сбой (диск мог быть извлечён, переполнен или защищён от записи). Устраните проблему и повторите попытку или выйдите без резервного копирования.
backup-failed-close-error-text = Перед закрытием не удалось записать ни одной резервной копии: { $error }. Устраните проблему и повторите попытку или выйдите без резервного копирования.
backup-no-result = Резервное копирование завершилось, не сообщив, была ли создана копия. Проверьте список резервных копий: нет ли там новой.

backup-choice-title = Файл резервной копии
backup-choice-heading = Вы открыли резервную копию
backup-choice-subtitle = Это резервная копия проекта на определённый момент времени.
backup-choice-subtitle-dated = Резервная копия создана { $date }.
backup-choice-body = Вы можете открыть её и свободно редактировать, но сохранить изменения можно только командой «Сохранить как». Исходный файл проекта не затрагивается. Либо восстановите проект точно в состояние этой резервной копии.
backup-choice-open = Открыть резервную копию
backup-choice-restore = Восстановить проект до этого момента…
backup-choice-not-a-backup = Нет, открыть как обычно
backup-banner-title = Резервная копия: изменения здесь сохранить нельзя
backup-banner-description = Используйте «Сохранить как», чтобы сохранить правки в новом файле, или «Восстановить», чтобы заменить исходный проект этой резервной копией.
backup-banner-restore = Восстановить…
backup-banner-save-as = Сохранить как…
backup-restore-original-missing = Не удаётся найти исходный проект, который нужно восстановить. Используйте «Сохранить как», чтобы сохранить эту резервную копию как новый проект.
backup-restore-close-elsewhere-title = Проект открыт в другом окне
backup-restore-close-elsewhere-text = Проект, который вы восстанавливаете, открыт в другом окне. Сначала закройте его там, затем повторите попытку.
backup-restore-focus-window = Перейти к этому окну
backup-restore-confirm-title = Восстановить эту резервную копию?
backup-restore-confirm-text = Перед заменой текущая версия проекта будет скопирована в сторону как страховочная резервная копия.
backup-restore-confirm-ok = Восстановить
backup-restore-error = Не удалось восстановить: { $error }
backup-restored-ok = Проект восстановлен.
backup-restored-with-safety = Проект восстановлен. Предыдущая версия сохранена в { $path }.
close-backup-discard-title = Отменить изменения в этой резервной копии?
close-backup-discard-text = Изменения в резервной копии нельзя сохранить в неё саму. Используйте «Сохранить как», чтобы сохранить их, или отмените их и закройте.
quit-backup-discard-title = Отменить изменения и выйти?
quit-backup-discard-work-question = Отменить изменения в «{ $title }» и выйти?
quit-backup-discard-text = Изменения в резервной копии нельзя сохранить в неё саму. Используйте «Сохранить как», чтобы сохранить их, или отмените их и выйдите.
backup-nudge-text = Для этого проекта не настроено резервное копирование.
backup-nudge-action = Настроить резервное копирование…

menu-backups-list = &Резервные копии…
backups-title = Резервные копии
backups-loading = Поиск резервных копий…
backups-empty = Резервных копий для этого проекта пока не найдено.
backups-open = Открыть
backups-reveal = Показать в папке
backups-delete = Удалить эту резервную копию
backups-delete-confirm-title = Удалить эту резервную копию?
backups-delete-confirm-text = «{ $name }» будет удалена безвозвратно. Это действие нельзя отменить.
backups-delete-error = Не удалось удалить резервную копию: { $error }
backups-refresh = Обновить
backups-close = Закрыть

settings-page-backup = Резервное копирование по умолчанию
settings-page-work-backup = Резервные копии
settings-backup-general-title = Параметры резервного копирования по умолчанию
settings-backup-work-title = Резервные копии этого проекта
settings-backup-inherit = Использовать общие настройки резервного копирования
settings-backup-inheriting = Этот проект использует общие настройки резервного копирования.
settings-backup-none-hint = Автоматическое резервное копирование не настроено (все условия запуска выключены).
settings-backup-last = Последняя резервная копия: { $date }
settings-backup-last-never = Резервных копий пока нет.
settings-backup-open-list = Открыть список резервных копий…
settings-backup-triggers = Когда создавать резервные копии
settings-backup-on-close = Создавать резервную копию при закрытии проекта
settings-backup-on-open = Создавать резервную копию при открытии проекта
settings-backup-interval = Создавать резервную копию регулярно, каждые
settings-backup-destinations = Места хранения резервных копий
settings-backup-default-location = Расположение по умолчанию
settings-backup-usage-since = { $count ->
    [one] { $count } резервная копия · { $size } · самая старая: { $oldest }
    [few] { $count } резервные копии · { $size } · самая старая: { $oldest }
    [many] { $count } резервных копий · { $size } · самая старая: { $oldest }
   *[other] { $count } резервной копии · { $size } · самая старая: { $oldest }
}
settings-backup-usage = { $count ->
    [one] { $count } резервная копия · { $size }
    [few] { $count } резервные копии · { $size }
    [many] { $count } резервных копий · { $size }
   *[other] { $count } резервной копии · { $size }
}
settings-backup-usage-empty = Здесь пока ничего не хранится
settings-backup-usage-measuring = Подсчёт…
settings-backup-reveal-root = Показать папку
settings-backup-dest-none = Мест хранения нет. Резервные копии сохраняются рядом с проектом.
settings-backup-dest-remove = Убрать
settings-backup-dest-add = Добавить папку…
settings-backup-dest-refresh = Обновить
settings-backup-retention = Сколько хранить
settings-backup-retention-tiered = По уровням
settings-backup-retention-keep-n = Последние N
settings-backup-retention-tip = Как удаляются старые резервные копии.
settings-backup-retention-tip-more = Режим «По уровням» хранит по одной резервной копии на каждый час в течение суток, на каждый день в течение недели, на каждую неделю в течение месяца и на каждый месяц далее: недавняя история остаётся подробной, а старая постепенно прореживается. Режим «Последние N» просто хранит N самых свежих копий. В обоих режимах самые новые копии (минимум, указанный ниже) хранятся всегда.
settings-backup-gfs-hourly = Ежечасно (последние 24 ч)
settings-backup-gfs-daily = Ежедневно (последняя неделя)
settings-backup-gfs-weekly = Еженедельно (последний месяц)
settings-backup-gfs-monthly = Ежемесячно
settings-backup-keep-n = Сколько хранить
settings-backup-min-keep = Всегда хранить не менее
settings-backup-dedup = Пропускать резервное копирование, если ничего не изменилось

new-work-title = Новый проект
new-work-close = Закрыть
new-work-name = Название проекта
new-work-name-placeholder = Без названия
new-work-author = Автор
new-work-author-placeholder = Необязательно
new-work-format = Формат
new-work-single-file = Один файл
new-work-bundle = Папка проекта
new-work-convert-later = Формат можно будет изменить позже.
new-work-location = Расположение
new-work-will-create = Будет создано
new-work-language = Язык по умолчанию
new-work-language-hint = Применяется к новым текстам и проверке орфографии. Для каждого текста можно выбрать другой язык.
new-work-template = Шаблон
new-work-template-none = Нет
new-work-template-empty-novel = Пустой роман
new-work-template-light-novel = Лайт-новел
new-work-template-novel = Роман
new-work-template-novel-in-parts = Роман в частях
new-work-template-notebook = Блокнот
new-work-cancel = Отмена
new-work-create = Создать проект
new-work-step-details = Подробности
new-work-step-language = Язык и структура
new-work-step-template = Шаблон
new-work-back = Назад
new-work-next = Далее
new-work-documents-title = Новый проект из документов
new-work-step-import = Импорт
new-work-create-and-import = Создать и импортировать…
new-work-documents-next-title = Затем — ваши документы
new-work-documents-next-body = После создания проекта поверх него откроется мастер импорта: там вы выберете документы, проверите структуру, которую Skribisto в них распознал, и укажете, куда её поместить. В проект ничего не записывается, пока вы не подтвердите это там.
new-work-documents-no-template = Этот проект намеренно начинается пустым, без шаблона: в нём будут только импортированные документы.
new-work-documents-chapter-scene-hint = Применяется к главам, которые создаст импорт. Изменить это можно позже в настройках проекта.
new-work-single-file-desc = Один архив .skrib (zip). Легко переносить и резервировать.
new-work-bundle-desc = Папка со всеми текстами и ресурсами. Удобнее для систем контроля версий.
new-work-template-none-count = пустая подшивка
new-work-template-empty-novel-count = 1 глава
new-work-template-light-novel-count = 15 глав
new-work-template-novel-count = 20 глав
new-work-template-novel-in-parts-count = 3 части, 24 главы
new-work-template-notebook-count = заметки в свободной форме
new-work-chapter-scene = Плоские главы
new-work-chapter-scene-tip = Каждая глава — это один элемент, в который вы пишете напрямую, без сцен внутри. Оставьте выключенным для классической структуры, где глава — это папка: в неё тоже можно писать напрямую, но она может содержать и сцены.
new-work-chapter-scene-tip-more = В главу вы пишете в любом случае. Разница лишь в том, может ли она *содержать* сцены. Дерево подшивки в Skribisto служит только для организации, поэтому обе структуры собираются в одну и ту же книгу, их можно свободно смешивать, а команда «Преобразовать в» переводит главу из одной формы в другую без потери единого слова.
new-work-name-required = Введите название проекта
new-work-name-invalid = В этом названии нет пригодных символов
new-work-location-required = Выберите расположение
new-work-location-missing = Такой папки не существует
new-work-location-not-folder = Этот путь не является папкой
new-work-location-readonly = В эту папку нельзя записывать
new-work-target-exists = В этой папке уже есть проект с таким названием. Перед заменой будет запрошено подтверждение
new-work-target-is-folder = Здесь уже есть папка с таким названием. Выберите другое название
new-work-overwrite-title = Заменить существующий проект?
new-work-overwrite-text = «{ $name }» уже существует в этой папке. Заменить его новым проектом?
new-work-target-open-title = Этот проект открыт
new-work-target-open-text = Проект «{ $name }» открыт в Skribisto. Создание нового проекта на его месте заменит проект, над которым вы работаете, а следующее сохранение запишет тот проект обратно поверх нового. Выберите другое название или папку либо сначала закройте проект.
target-importing-title = В этот проект идёт запись импорта
target-importing-text = Импорт всё ещё записывает «{ $name }». Дождитесь его завершения или выберите другое название.
backup-restore-importing-text = Импорт всё ещё записывает «{ $name }» — проект, поверх которого будет восстановлена эта резервная копия. Восстановите копию после завершения импорта.

new-work-manuscript = Рукопись
new-work-notes = Заметки
new-work-research = Исследования
new-work-notebook = Блокнот
new-work-scene = Сцена
new-work-note = Заметка
new-work-front-matter = Вступительные материалы
new-work-back-matter = Заключительные материалы
new-work-paratext = Структура книги
new-work-paratext-none = Без структуры
new-work-paratext-hint = Вступительные и заключительные материалы, которыми традиция открывает и закрывает книгу. Любой из них можно потом переместить, переименовать или удалить.
new-work-tags = Теги
new-work-tags-none = Без тегов
new-work-tags-hint = Начальный набор тегов для персонажей, мест и прочего. Необязательно; любой тег потом можно переименовать, перекрасить или удалить.
new-work-note-templates = Шаблоны заметок
new-work-note-templates-none = Без шаблонов
new-work-note-templates-hint = Форма, с которой начинается заметка библии истории. Необязательно; любой шаблон потом можно изменить или удалить.
new-work-characters = Персонажи
new-work-places = Места

import-plume-title = Импорт проекта Plume Creator
import-plume-source = Проект Plume
import-plume-source-hint = Выберите файл .plume или .plume_backup (любая версия Plume Creator).
import-plume-trash-warning = ⚠ Элементы из корзины и удалённые элементы не переносятся.
import-plume-source-required = Выберите файл проекта Plume
import-plume-source-missing = Этот файл не существует
import-plume-source-not-file = Этот путь не является файлом
import-plume-manuscript-binder = Рукопись
import-plume-story-bible-binder = Библия истории
import-plume-notes-of = { $name } (заметки)
import-plume-content-of = { $name } (текст)
import-plume-progress-title = Импорт проекта Plume…
import-plume-done = Импортировано: { $imported ->
    [one] { $imported } элемент.
    [few] { $imported } элемента.
    [many] { $imported } элементов.
   *[other] { $imported } элемента.
} { $skipped ->
    [one] { $skipped } элемент из корзины не перенесён.
    [few] { $skipped } элемента из корзины не перенесены.
    [many] { $skipped } элементов из корзины не перенесены.
   *[other] { $skipped } элемента из корзины не перенесено.
}
import-plume-nested-too-deep = Проект не импортирован: его часть «{ $part }» имеет вложенность глубже { $limit } уровней. Ни один проект Plume Creator и близко к этому не подходит, а чтение такой части может привести к аварийному завершению Skribisto. Ничего не записано.
import-plume-declares-entities = Проект не импортирован: его часть «{ $part }» объявляет XML-сущности, чего не делает ни один проект Plume Creator. Их раскрытие может потребовать больше памяти, чем есть у компьютера, и привести к аварийному завершению Skribisto. Ничего не записано.
import-plume-no-outline = Этот файл — сжатый архив, но не проект Plume Creator: в нём нет структуры. Ничего не записано.
import-plume-empty-outline = Файл структуры этого проекта Plume Creator пуст, поэтому импортировать нечего. Ничего не записано.
# $tag is the name of the document the part turned out to be.
import-plume-wrong-document = Это не проект Plume Creator: его часть «{ $part }» — документ «{ $tag }». Ничего не записано.
# $version is the part's own version, $supported the newest this importer reads.
import-plume-too-new = Часть «{ $part }» этого проекта записана более новой версией Plume Creator (версия { $version }), чем может прочитать Skribisto (до { $supported }). Ничего не записано.

import-manuskript-title = Импорт проекта Manuskript
import-manuskript-source = Проект Manuskript
import-manuskript-source-hint = Выберите файл .msk или папку проекта (любая версия Manuskript).
import-manuskript-source-file = Выбрать файл…
import-manuskript-source-folder = Выбрать папку…
import-manuskript-source-required = Выберите проект Manuskript
import-manuskript-source-missing = Этот файл или папка не существует
import-manuskript-manuscript-binder = Рукопись
import-manuskript-story-bible-binder = Библия истории
import-manuskript-characters-group = Персонажи
import-manuskript-world-group = Мир
import-manuskript-plots-group = Сюжетные линии
import-manuskript-project-info-note = Информация о проекте
import-manuskript-summary-note = Краткое содержание
import-manuskript-importance-minor = Эпизодический
import-manuskript-importance-secondary = Второстепенный
import-manuskript-importance-main = Главный
import-manuskript-notes-of = { $title } (заметки)
import-manuskript-progress-title = Импорт проекта Manuskript…
import-manuskript-done = { $imported ->
    [one] Импортирован { $imported } элемент.
    [few] Импортировано { $imported } элемента.
    [many] Импортировано { $imported } элементов.
   *[other] Импортировано { $imported } элемента.
} { $revisions ->
    [0] { "" }
    [one] Вместе с ними перенесена { $revisions } более ранняя версия.
    [few] Вместе с ними перенесены { $revisions } более ранние версии.
    [many] Вместе с ними перенесено { $revisions } более ранних версий.
   *[other] Вместе с ними перенесено { $revisions } более ранней версии.
}
import-manuskript-nested-too-deep = Проект не импортирован: его файл «{ $part }» имеет вложенность глубже { $limit } уровней. Ни один проект Manuskript и близко к этому не подходит, а чтение такого файла может привести к аварийному завершению Skribisto. Ничего не записано.
import-manuskript-folders-too-deep = Проект не импортирован: один из его файлов лежит глубже { $limit } уровней папок. Ни один проект Manuskript и близко к этому не подходит, а чтение такого файла может привести к аварийному завершению Skribisto. Ничего не записано.
import-manuskript-declares-entities = Проект не импортирован: его файл «{ $part }» объявляет XML-сущности, чего не делает ни один проект Manuskript. Их раскрытие может потребовать больше памяти, чем есть у компьютера, и привести к аварийному завершению Skribisto. Ничего не записано.
import-manuskript-not-a-project-file = «{ $file }» — не архив проекта Manuskript и не маленький файл .msk, указывающий на папку проекта. Ничего не записано.
import-manuskript-folder-missing = «{ $stub }» указывает на папку проекта «{ $folder }» рядом с собой, но этой папки нет. Проект Manuskript в режиме папки состоит из них обоих, и переносить их нужно вместе. Ничего не записано.
import-manuskript-not-a-project = Это не похоже на проект Manuskript: в нём нет ни маркера Manuskript, ни структуры. Ничего не записано.
import-manuskript-format-too-new = Этот проект сохранён в формате Manuskript { $version }, более новом, чем может прочитать Skribisto (до { $supported }). Ничего не записано.
import-manuskript-too-many-files = Проект не импортирован: в нём больше { $limit } файлов, гораздо больше, чем в любом проекте Manuskript. Ничего не записано.
import-manuskript-too-large = Проект не импортирован: его размер превышает { $limit } МБ — предел, который Skribisto читает из проекта Manuskript. Ничего не записано.
project-import-close = Закрыть
project-import-location = Папка назначения
# The title of the folder dialog, like the dialog-*-title keys: word it as a window title
# (in many languages an infinitive or a noun phrase), not as the instruction
# project-import-location-required gives under the field.
project-import-location-title = Выберите папку назначения
project-import-name = Имя файла
project-import-name-placeholder = Название проекта
project-import-will-create = Будет создано
project-import-cancel = Отмена
project-import-import = Импорт
# Field validation
project-import-location-required = Выберите папку назначения
project-import-location-missing = Такой папки не существует
project-import-location-not-folder = Этот путь не является папкой
project-import-location-readonly = В эту папку нельзя записывать
project-import-name-required = Введите имя файла
project-import-name-exists = Файл с таким именем здесь уже существует. Перед перезаписью импорт запросит подтверждение
project-import-name-is-folder = Здесь уже есть папка с таким именем, возможно, проект, сохранённый папкой. Импорт не может её заменить: выберите другое имя
# Overwrite confirmation
project-import-overwrite-title = Заменить существующий файл?
project-import-overwrite-text = «{ $name }» уже существует. Заменить его импортированным проектом?
# Projects are imported one at a time, whichever importer asks.
project-import-busy-title = Уже импортируется другой проект
project-import-busy-text = Проекты импортируются по одному. Дождитесь завершения текущего импорта или отмените его, затем запустите этот снова.
# Spoken by a screen reader once per phase, never shown: the toast's title, then
# the phase's name.
project-import-progress-announcement = { $title } { $phase }
# The notice once Cancel is pressed, until the converter has stopped: it may be
# finishing a file it cannot leave half written.
project-import-stopping = Остановка импорта…
project-import-cancelled = Импорт отменён
project-import-open-now = Открыть сейчас
project-import-details = Подробности
# The import's report: what the writer should know about what came across.
project-import-report = { $count ->
    [one] { $count } замечание об этом импорте
    [few] { $count } замечания об этом импорте
    [many] { $count } замечаний об этом импорте
   *[other] { $count } замечания об этом импорте
}
project-import-report-title = Об этом импорте
# Error toast: a short reason in the body, the full technical chain behind Details.
project-import-error-title = Не удалось импортировать проект
# Why a project was not imported, whichever importer read it. A $file is a file
# name, a $folder a full path (it is the folder the writer has to act on), a
# $reason what the system or a parser said, a $limit a number.
project-import-failed-cancelled = Импорт отменён. Ничего не записано.
project-import-failed-target-exists = «{ $file }» уже существует, а заменять его импорту не разрешено. Ничего не записано.
project-import-failed-target-folder = «{ $file }» — это папка, возможно, проект, сохранённый папкой, и импорт не может её заменить. Выберите другое имя. Ничего не записано.
project-import-failed-not-written = Не удалось записать «{ $file }» в «{ $folder }» ({ $reason }). Ничего не записано.
project-import-failed-disk-full = В «{ $folder }» недостаточно места, чтобы записать «{ $file }». Освободите там место, затем повторите импорт. Ничего не записано.
project-import-failed-source-missing = «{ $file }» больше нет на месте. Ничего не записано.
project-import-failed-unreadable = Skribisto не удалось прочитать «{ $file }» ({ $reason }). Ничего не записано.
project-import-failed-archive-damaged = «{ $file }» — сжатый файл, который не удалось прочитать, вероятно, потому что он повреждён ({ $reason }). Ничего не записано.
project-import-failed-archive-entries = Проект не импортирован: в его архиве { $entries ->
    [one] { $entries } файл
    [few] { $entries } файла
    [many] { $entries } файлов
   *[other] { $entries } файла
}, а проект может содержать не больше { $limit }. Так бывает только с файлом, созданным, чтобы исчерпать память. Ничего не записано.
project-import-failed-archive-member = Проект не импортирован: его часть «{ $part }» после распаковки занимает больше { $limit } МБ. Так бывает только с файлом, созданным, чтобы исчерпать память. Ничего не записано.
project-import-failed-archive-size = Проект не импортирован: после распаковки он занимает больше { $limit } МБ. Так бывает только с файлом, созданным, чтобы исчерпать память. Ничего не записано.
project-import-failed-nested = Проект не импортирован: его часть «{ $part }» имеет вложенность глубже { $limit } уровней, а её чтение может привести к аварийному завершению Skribisto. Ничего не записано.
project-import-failed-entities = Проект не импортирован: его часть «{ $part }» объявляет XML-сущности. Их раскрытие может потребовать больше памяти, чем есть у компьютера, и привести к аварийному завершению Skribisto. Ничего не записано.
project-import-failed-folders = Проект не импортирован: один из его файлов лежит глубже { $limit } уровней папок, а его чтение может привести к аварийному завершению Skribisto. Ничего не записано.
project-import-failed-malformed = Проект не импортирован: его часть «{ $part }» повреждена и не читается ({ $reason }). Ничего не записано.
# A Scrivener project the importer cannot bring across (a Scrivener 1 for Mac project, a
# damaged binder file). Why is the Scrivener importer's own to say; Details holds its key.
project-import-failed-scrivener-refused = Skribisto не может импортировать этот проект Scrivener. Ничего не записано.
# The Scrivener project asks something its review did not answer: a project or a binder
# file appeared in it since. Shown with no button to press.
project-import-failed-scrivener-question = Проект Scrivener задаёт вопрос, на который при проверке не было ответа. Запустите его импорт заново и ответьте на вопрос, затем импортируйте. Ничего не записано.
# The Scrivener project needs the writer to tick a box before it imports (to import it
# without changes made on an iPhone or iPad, or with its documents empty), and the import
# was started without it. Shown with no button to press.
project-import-failed-scrivener-not-accepted = Чтобы проект Scrivener можно было импортировать, нужно установить флажок на шаге «Проект», а он не установлен. Запустите его импорт заново и установите флажок, затем импортируйте. Ничего не записано.
# The Scrivener project changed after the writer reviewed it. Shown with no button to press.
project-import-failed-scrivener-changed = Проект Scrivener изменился после вашей проверки. Запустите его импорт заново и проверьте проект ещё раз, затем импортируйте. Ничего не записано.
# An error nothing above accounts for; $detail is its technical description.
project-import-failed-internal = Импорт остановлен из-за ошибки, которой Skribisto не ожидал: { $detail }

import-target-open-title = Этот проект открыт
import-target-open-text = Проект «{ $name }» открыт в Skribisto. Импорт поверх него заменит проект, над которым вы работаете, а следующее сохранение запишет тот проект обратно поверх импорта. Выберите другое имя файла или сначала закройте проект.
import-warnings-announcement = { $result } { $notice }.
# Quitting while an import runs. $name is the file the import is writing.
quit-import-running-title = Импорт ещё выполняется
quit-import-running-text = Skribisto ещё импортирует «{ $name }». Если выйти сейчас, импорт остановится: ничего не будет записано, а проект, из которого вы импортируете, останется таким, каким был.
quit-import-stop = Остановить импорт и выйти

## Import a Scrivener project: the wizard
import-scrivener-title = Импорт проекта Scrivener
import-scrivener-step-project = Проект
import-scrivener-step-review = Проверка
import-scrivener-step-save = Сохранение
import-scrivener-back = Назад
import-scrivener-next = Далее
import-scrivener-import = Импорт
import-scrivener-cancel = Отмена
import-scrivener-help = Справка

## Project step: choosing the project
import-scrivener-drop-title = Перетащите сюда проект Scrivener
import-scrivener-drop-hint = Проект .scriv, файл .scrivx внутри него, резервная копия .zip или файл .scrivtemplate
import-scrivener-source = Проект Scrivener
import-scrivener-source-placeholder = Путь к проекту
import-scrivener-choose-file = Выбрать файл…
import-scrivener-choose-folder = Выбрать папку…
# Windows and Linux show a .scriv project as an ordinary folder.
import-scrivener-source-hint = Выберите папку проекта .scriv или откройте её и выберите файл .scrivx внутри. Подойдёт и резервная копия .zip.
# macOS shows a .scriv project as a single file when Scrivener is installed, as a folder when it is not.
import-scrivener-source-hint-mac = Если проект выглядит как один файл, нажмите «Выбрать файл…». Если он выглядит как папка, нажмите «Выбрать папку…». Подойдёт и резервная копия .zip.
# File-dialog titles and filter label, resolved when the dialog opens.
import-scrivener-dialog-file-title = Выберите проект Scrivener
import-scrivener-dialog-folder-title = Выберите папку проекта Scrivener
import-scrivener-dialog-filter = Проект Scrivener
import-scrivener-source-missing = По этому пути ничего нет
# A path whose name is not Unicode: the field shows it with its unreadable letters replaced.
import-scrivener-source-unreadable-name = Это имя не удаётся прочитать. Переименуйте проект в файловом менеджере, затем выберите его снова.
import-scrivener-empty-hint = Выберите проект, чтобы увидеть, что Skribisto в нём найдёт.
import-scrivener-reading = Чтение проекта…
import-scrivener-stop-reading = Остановить чтение
import-scrivener-read-failed = Не удалось прочитать проект
import-scrivener-read-failed-body = Во время чтения что-то пошло не так. Нажмите «Перечитать проект», чтобы попробовать ещё раз, или «Подробности», чтобы узнать, что случилось.
import-scrivener-details = Подробности
# The wizard's button that reads the Scrivener project from disk again, as it is now:
# after a refusal whose cause may go away, or a reading that failed. Not the action of
# import-scrivener-review-again: keep the two labels clearly different.
import-scrivener-check-again = Перечитать проект
# A refusal this version does not know how to word.
import-scrivener-refusal-unknown-title = Этот проект нельзя импортировать
import-scrivener-refusal-unknown-body = Skribisto не знает, как его перенести. Причина указана в подробностях.

## Project step: the project card
import-scrivener-made-with-mac = Последнее сохранение: Scrivener { $version } для macOS
import-scrivener-made-with-windows = Последнее сохранение: Scrivener { $version } для Windows
import-scrivener-made-with-linux = Последнее сохранение: Scrivener { $version } для Linux
import-scrivener-made-with-ios = Последнее сохранение: Scrivener { $version } для iPhone и iPad
import-scrivener-made-with-version = Последнее сохранение: Scrivener { $version }
import-scrivener-made-with-unknown = Проект Scrivener, формат { $format }
import-scrivener-saved-on = Сохранено { $date } на устройстве «{ $device }»
import-scrivener-saved-on-no-device = Сохранено { $date }
import-scrivener-card-counts = { $documents ->
    [one] { $documents } документ
    [few] { $documents } документа
    [many] { $documents } документов
   *[other] { $documents } документа
}, из них в рукописи: { $manuscript }

## Review step
import-scrivener-review-hint = Так будет перенесена ваша подшивка. Ничего не записывается, пока вы не запустите импорт, а тип многих элементов потом можно изменить в подшивке командой «Преобразовать в».
import-scrivener-choices-structure = Уровни
import-scrivener-top-level = Папки верхнего уровня
import-scrivener-choices-text = Текст
import-scrivener-choice-containers = Папки на уровне { $depth }
import-scrivener-choice-texts = Документы на уровне { $depth }
import-scrivener-choice-count = { $count ->
    [one] { $count } элемент
    [few] { $count } элемента
    [many] { $count } элементов
   *[other] { $count } элемента
}
import-scrivener-choice-count-example = { $count ->
    [1] { $count } элемент: «{ $example }»
    [one] { $count } элемент, например «{ $example }»
    [few] { $count } элемента, например «{ $example }»
    [many] { $count } элементов, например «{ $example }»
   *[other] { $count } элемента, например «{ $example }»
}
import-scrivener-rung-part = Части
import-scrivener-rung-chapter = Главы
import-scrivener-rung-plain = Обычные папки
import-scrivener-texts-scenes = Сцены
import-scrivener-texts-chapters = Главы
import-scrivener-choice-books = Книги в рукописи
import-scrivener-books-one = Одна книга
import-scrivener-books-several = По книге на папку
import-scrivener-choice-split = { $count ->
    [1] Разделить «{ $title }» по его заголовку
    [one] Разделить «{ $title }» по его { $count } заголовку
    [few] Разделить «{ $title }» по его { $count } заголовкам
    [many] Разделить «{ $title }» по его { $count } заголовкам
   *[other] Разделить «{ $title }» по его { $count } заголовкам
}
import-scrivener-root-items = { $count ->
    [one] { $count } элемент
    [few] { $count } элемента
    [many] { $count } элементов
   *[other] { $count } элемента
}
import-scrivener-root-items-manuscript = { $count ->
    [one] { $count } элемент. Эта папка похожа на рукопись.
    [few] { $count } элемента. Эта папка похожа на рукопись.
    [many] { $count } элементов. Эта папка похожа на рукопись.
   *[other] { $count } элемента. Эта папка похожа на рукопись.
}
import-scrivener-dest-notes = Заметки
import-scrivener-dest-story-bible = Библия истории
import-scrivener-dest-manuscript = Ещё одна книга
import-scrivener-dest-leave-out = Не переносить
import-scrivener-choice-trash = { $count ->
    [one] Перенести корзину ({ $count } элемент)
    [few] Перенести корзину ({ $count } элемента)
    [many] Перенести корзину ({ $count } элементов)
   *[other] Перенести корзину ({ $count } элемента)
}
import-scrivener-trash-detail = { $count ->
    [0] Ни в одном из них нет текста.
    [one] Текст есть в { $count } из них.
    [few] Текст есть в { $count } из них.
    [many] Текст есть в { $count } из них.
   *[other] Текст есть в { $count } из них.
}
import-scrivener-choice-blank-lines = Пустые строки становятся разрывами сцен
import-scrivener-choice-ornaments = Строки из декоративных знаков становятся разрывами сцен
import-scrivener-preview-label = Структура нового проекта
import-scrivener-col-title = Название
import-scrivener-col-becomes = Станет
import-scrivener-col-exports = В экспорте
import-scrivener-in-exports-yes = Да
import-scrivener-in-exports-no = Нет
# What a row the binder has no creation command for becomes.
import-scrivener-becomes-item = Элемент
import-scrivener-becomes-trashed = В корзине
import-scrivener-before-title = Перед импортом
import-scrivener-review-error-unreadable = Не удалось подготовить проверку. Выберите проект снова.
import-scrivener-review-error-choice = Этот выбор не подходит к проекту. Выберите проект снова.
import-scrivener-review-error-unmappable = При таком выборе подшивку не удаётся перенести без потерь. Выберите иначе или сообщите о проблеме.

## Save step
import-scrivener-save-hint = Импорт создаёт новый проект Skribisto. Ваш проект Scrivener только читается и никогда не изменяется.
import-scrivener-title-field = Название
import-scrivener-author = Автор
import-scrivener-language = Язык
import-scrivener-location-inside-project = Выберите папку вне проекта Scrivener. Scrivener хранит там только собственные файлы.
import-scrivener-copy-research-label = Скопировать файлы исследований рядом с новым проектом
import-scrivener-copy-research = { $count ->
    [one] { $count } файл исследований, который Skribisto не умеет открывать, например PDF или звук, будет помещён в папку рядом с проектом. Если флажок снят, в его заметке будет указано, где он лежит в вашем проекте Scrivener.
    [few] { $count } файла исследований, которые Skribisto не умеет открывать, например PDF или звуки, будут помещены в папку рядом с проектом. Если флажок снят, в их заметках будет указано, где они лежат в вашем проекте Scrivener.
    [many] { $count } файлов исследований, которые Skribisto не умеет открывать, например PDF или звуки, будут помещены в папку рядом с проектом. Если флажок снят, в их заметках будет указано, где они лежат в вашем проекте Scrivener.
   *[other] { $count } файла исследований, которые Skribisto не умеет открывать, например PDF или звуки, будут помещены в папку рядом с проектом. Если флажок снят, в их заметках будет указано, где они лежат в вашем проекте Scrivener.
}
# The file name proposed when the project's name holds nothing a file name can.
import-scrivener-default-name = Проект Scrivener

## Progress and result
import-scrivener-progress-title = Импорт проекта Scrivener…
import-scrivener-done-title = Проект Scrivener импортирован
# How many rows the new project holds: the documents and folders that came across, and
# the rows the import made for them (a document's notes, the project notes).
import-scrivener-done = { $items ->
    [one] В новом проекте { $items } элемент.
    [few] В новом проекте { $items } элемента.
    [many] В новом проекте { $items } элементов.
   *[other] В новом проекте { $items } элемента.
}
# The same, with how many kinds of thing could not be read or need checking.
import-scrivener-done-check = { $items ->
    [one] В новом проекте { $items } элемент.
    [few] В новом проекте { $items } элемента.
    [many] В новом проекте { $items } элементов.
   *[other] В новом проекте { $items } элемента.
} { $findings ->
    [one] { $findings } пункт требует внимания.
    [few] { $findings } пункта требуют внимания.
    [many] { $findings } пунктов требуют внимания.
   *[other] { $findings } пункта требуют внимания.
}
# The button of the notice a failed import shows: it opens the import wizard again on the
# same Scrivener project, with what the writer chose for the import that failed. Not the
# action of import-scrivener-check-again. Keep it short: it shares one line of the notice
# with “Details”.
import-scrivener-review-again = Вернуться к проверке

## The import's report (the dialog See report opens)
import-scrivener-see-report = Открыть отчёт
import-scrivener-report-from = Прочитано из { $path }
import-scrivener-report-to = Записано в { $path }
import-scrivener-report-came-across = Что перенесено
import-scrivener-report-nothing-counted = Ничего не подсчитано.
import-scrivener-report-items = { $count ->
    [one] { $count } элемент
    [few] { $count } элемента
    [many] { $count } элементов
   *[other] { $count } элемента
}
import-scrivener-report-words = { $count ->
    [one] { $count } слово
    [few] { $count } слова
    [many] { $count } слов
   *[other] { $count } слова
}
import-scrivener-report-synopses = { $count ->
    [one] { $count } синопсис
    [few] { $count } синопсиса
    [many] { $count } синопсисов
   *[other] { $count } синопсиса
}
import-scrivener-report-document-notes = { $count ->
    [one] заметки { $count } документа
    [few] заметки { $count } документов
    [many] заметки { $count } документов
   *[other] заметки { $count } документа
}
import-scrivener-report-comments = { $count ->
    [one] { $count } комментарий
    [few] { $count } комментария
    [many] { $count } комментариев
   *[other] { $count } комментария
}
import-scrivener-report-footnotes = { $count ->
    [one] { $count } сноска
    [few] { $count } сноски
    [many] { $count } сносок
   *[other] { $count } сноски
}
import-scrivener-report-pictures = { $count ->
    [one] { $count } изображение
    [few] { $count } изображения
    [many] { $count } изображений
   *[other] { $count } изображения
}
import-scrivener-report-templates = { $count ->
    [one] { $count } шаблон заметки
    [few] { $count } шаблона заметок
    [many] { $count } шаблонов заметок
   *[other] { $count } шаблона заметки
}
import-scrivener-report-tags = { $count ->
    [one] { $count } тег
    [few] { $count } тега
    [many] { $count } тегов
   *[other] { $count } тега
}
import-scrivener-report-statuses = { $count ->
    [one] { $count } статус
    [few] { $count } статуса
    [many] { $count } статусов
   *[other] { $count } статуса
}
import-scrivener-report-trashed = { $count ->
    [one] { $count } элемент в корзине
    [few] { $count } элемента в корзине
    [many] { $count } элементов в корзине
   *[other] { $count } элемента в корзине
}
import-scrivener-report-research-files = { $count ->
    [one] { $count } файл исследований
    [few] { $count } файла исследований
    [many] { $count } файлов исследований
   *[other] { $count } файла исследований
}
import-scrivener-report-choices = Ваш выбор
import-scrivener-report-accepted-mobile = Вы импортировали проект без изменений, сделанных в Scrivener для iPhone или iPad, которые ещё не попали в него.
import-scrivener-report-accepted-no-text = Вы импортировали эту копию с пустыми документами, потому что их текста в ней не было.
import-scrivener-report-choice = { $subject }: { $answer }
import-scrivener-report-yes = да
import-scrivener-report-no = нет
import-scrivener-report-research-copied = Файлы исследований, которые Skribisto не умеет открывать, скопированы в «{ $folder }» рядом с новым проектом.
import-scrivener-report-research-not-copied = Файлы исследований, которые Skribisto не умеет открывать, не скопированы. В их заметках указано, где они лежат в вашем проекте Scrivener.
import-scrivener-report-research-copied-except = { $count ->
    [one] Файлы исследований, которые Skribisto не умеет открывать, скопированы в «{ $folder }» рядом с новым проектом, кроме { $count } файла, который остаётся в вашем проекте Scrivener. В его заметке указано, где он лежит.
    [few] Файлы исследований, которые Skribisto не умеет открывать, скопированы в «{ $folder }» рядом с новым проектом, кроме { $count } файлов, которые остаются в вашем проекте Scrivener. В их заметках указано, где они лежат.
    [many] Файлы исследований, которые Skribisto не умеет открывать, скопированы в «{ $folder }» рядом с новым проектом, кроме { $count } файлов, которые остаются в вашем проекте Scrivener. В их заметках указано, где они лежат.
   *[other] Файлы исследований, которые Skribisto не умеет открывать, скопированы в «{ $folder }» рядом с новым проектом, кроме { $count } файла, которые остаются в вашем проекте Scrivener. В их заметках указано, где они лежат.
}
import-scrivener-report-research-none-copied = { $count ->
    [1] Вы попросили скопировать файлы исследований, но единственный такой файл скопировать не удалось. Он остаётся в вашем проекте Scrivener, а в его заметке указано, где он лежит.
    [one] Вы попросили скопировать файлы исследований, но не удалось скопировать ни одного из { $count } файла. Они остаются в вашем проекте Scrivener, а в их заметках указано, где они лежат.
    [few] Вы попросили скопировать файлы исследований, но не удалось скопировать ни одного из { $count } файлов. Они остаются в вашем проекте Scrivener, а в их заметках указано, где они лежат.
    [many] Вы попросили скопировать файлы исследований, но не удалось скопировать ни одного из { $count } файлов. Они остаются в вашем проекте Scrivener, а в их заметках указано, где они лежат.
   *[other] Вы попросили скопировать файлы исследований, но не удалось скопировать ни одного из { $count } файла. Они остаются в вашем проекте Scrivener, а в их заметках указано, где они лежат.
}
import-scrivener-report-chose-project = Вы выбрали для импорта «{ $project }» — один из нескольких проектов в указанной вами папке или архиве.
import-scrivener-report-chose-manifest = Вы выбрали файл подшивки «{ $file }».
import-scrivener-report-chose-enclosing = Вы указали что-то внутри «{ $project }», поэтому импортирован весь проект.
import-scrivener-report-chose-upgraded = Вы импортировали «{ $current }» — проект, в котором вы продолжили писать, а не копию, которую Scrivener сохранил перед обновлением.
import-scrivener-report-chose-backup = Вы импортировали «{ $backup }» — более старую копию, которую Scrivener сохранил перед обновлением.
import-scrivener-report-title-placeholder-rejected = Название в настройках проекта было заполнителем из шаблона, поэтому Skribisto предложил вместо него имя проекта.
import-scrivener-report-author-placeholder-rejected = Имя автора в настройках проекта было заполнителем из шаблона, поэтому Skribisto не предложил никакого имени.
import-scrivener-report-title-from-compile-settings = Skribisto предложил название, указанное в настройках компиляции проекта: это имя, которое проект получил при создании, если только в Scrivener там не задали другое название.
import-scrivener-report-title-from-template = Skribisto предложил собственное название шаблона.
import-scrivener-report-title-from-folder-name = Skribisto предложил в качестве названия имя проекта.
import-scrivener-report-title-from-project-properties = Skribisto предложил название, указанное в свойствах проекта.
import-scrivener-report-author-from-project = Skribisto предложил имя автора, указанное в настройках проекта. Там может остаться автор шаблона, и тогда это имя будет напечатано на титульном листе.
import-scrivener-report-language-not-stated = В проекте не был указан язык, поэтому Skribisto предложил язык своего интерфейса. От языка зависит, как считаются слова и как ставятся кавычки.
import-scrivener-report-language-from-script = Skribisto предложил язык по письменности, которой написана рукопись ({ $tag }).
# A proposal of the Save page the writer replaced with their own. $hint is the sentence
# saying what Skribisto proposed: a title, an author's name or a language.
import-scrivener-report-you-changed-it = { $hint } Вы заменили это предложение своим.
import-scrivener-report-content-salvaged-text = Текста «{ $item }» в проекте не было. Его слова взяты из поискового индекса Scrivener, простым текстом и, возможно, из более старой версии: проверьте их.
import-scrivener-report-content-salvaged-notes = Заметок к «{ $item }» в проекте не было. Их слова взяты из поискового индекса Scrivener, простым текстом и, возможно, из более старой версии: проверьте их.
import-scrivener-report-content-salvaged-synopsis = Синопсиса «{ $item }» в проекте не было. Его слова взяты из поискового индекса Scrivener, простым текстом и, возможно, из более старой версии: проверьте их.
# A finding of the document importer found in several places: its sentence about the
# first, then how many more the list under it names.
import-scrivener-report-document-others = { $others ->
    [one] { $sentence } То же самое ещё в { $others } месте, оно указано ниже.
    [few] { $sentence } То же самое ещё в { $others } местах, они указаны ниже.
    [many] { $sentence } То же самое ещё в { $others } местах, они указаны ниже.
   *[other] { $sentence } То же самое ещё в { $others } местах, они указаны ниже.
}
# An item of the report, by its path in Scrivener's binder, with which of its writings
# the line is about when it is not the text.
import-scrivener-report-item-notes = { $path } (заметки документа)
import-scrivener-report-item-synopsis = { $path } (синопсис)
import-scrivener-report-item-project-note = { $path } (заметка проекта)
import-scrivener-report-item-recovered = { $path } (восстановленный текст)
# An item with no name in Scrivener.
import-scrivener-report-untitled = Без названия
import-scrivener-report-no-findings = Больше импорту сообщить нечего.
import-scrivener-report-group-unread = Не удалось прочитать
import-scrivener-report-group-check = Проверьте
import-scrivener-report-group-changed = Перенесено иначе
import-scrivener-report-group-left = Не перенесено
import-scrivener-report-group-count = { $group } ({ $count })
import-scrivener-report-items-toggle = { $count ->
    [one] { $count } элемент
    [few] { $count } элемента
    [many] { $count } элементов
   *[other] { $count } элемента
}
# The same, as a screen reader names it: with the line whose items it lists.
import-scrivener-report-items-toggle-named = { $count ->
    [one] { $count } элемент: { $line }
    [few] { $count } элемента: { $line }
    [many] { $count } элементов: { $line }
   *[other] { $count } элемента: { $line }
}
import-scrivener-report-names-toggle = { $count ->
    [one] { $count } имя
    [few] { $count } имени
    [many] { $count } имён
   *[other] { $count } имени
}
# The same, as a screen reader hears it, naming the line it belongs to.
import-scrivener-report-names-toggle-named = { $count ->
    [one] { $count } имя: { $line }
    [few] { $count } имени: { $line }
    [many] { $count } имён: { $line }
   *[other] { $count } имени: { $line }
}
import-scrivener-report-more = { $count ->
    [one] И ещё { $count }.
    [few] И ещё { $count }.
    [many] И ещё { $count }.
   *[other] И ещё { $count }.
}
import-scrivener-report-open-item = Открыть в редакторе
import-scrivener-report-links-hint = Откройте новый проект и снова покажите этот отчёт из палитры команд: тогда каждый указанный здесь элемент будет открываться щелчком.
import-scrivener-copy-report = Копировать отчёт
import-scrivener-report-copied = Отчёт скопирован
import-scrivener-report-copy-failed = Не удалось скопировать отчёт.
import-scrivener-report-close = Закрыть
import-scrivener-report-none-title = Нет отчёта об импорте
import-scrivener-report-none = С момента запуска Skribisto ни один импорт из Scrivener не завершился. Skribisto хранит отчёты о последних импортах, пока его не закроют.
import-scrivener-report-gone = Отчёт об этом импорте больше не хранится. Skribisto хранит отчёты о { $count ->
    [one] последнем { $count } импорте
    [few] последних { $count } импортах
    [many] последних { $count } импортах
   *[other] последних { $count } импортах
}, пока его не закроют.

## Words the import writes into the new project, in the language Skribisto shows itself in
import-scrivener-manuscript-binder = Рукопись
import-scrivener-notes-binder = Заметки
import-scrivener-notes-of = { $title } (заметки)
import-scrivener-project-notes = Заметки проекта
import-scrivener-recovered = Восстановленные тексты
import-scrivener-untitled-template = Шаблон без названия
# The folder beside the new project its research files are copied into. A folder name on
# disk: keep it free of the characters a file name cannot hold (/ \ : * ? " < > |).
import-scrivener-research-folder = { $name } (файлы исследований)
# Between two authors' names.
import-scrivener-list-separator = {", "}
import-scrivener-label-details = Из поля «{ $field }» в Scrivener
import-scrivener-status-details = Из поля «{ $field }» в Scrivener
import-scrivener-unnamed-label = Метка { $number }
import-scrivener-unnamed-status = Статус { $number }
import-scrivener-label-tag = { $name } (метка)
import-scrivener-untitled-tag = Без названия
# The tags, the pace plan and the "From Scrivener" folder the import makes from what a
# Scrivener project holds beyond its labels and status.
import-scrivener-keyword-details = Ключевое слово «{ $path }» в Scrivener
import-scrivener-unnamed-keyword = Ключевое слово { $number }
import-scrivener-keyword-tag = { $name } (ключевое слово)
import-scrivener-collection-details = Коллекция в Scrivener
import-scrivener-collection-tag = { $name } (коллекция)
import-scrivener-field-details = Из поля «{ $field }» в Scrivener
# A tag made from one option of a list field.
import-scrivener-field-value = { $field }: { $option }
import-scrivener-field-tag = { $name } (поле)
# The day off a pace plan takes on its deadline.
import-scrivener-deadline-day = Срок
import-scrivener-from-scrivener = Из Scrivener
import-scrivener-note-project = Проект
import-scrivener-note-project-bookmarks = Закладки проекта
import-scrivener-note-replacements = Замены при компиляции
import-scrivener-note-auto-complete = Записи автодополнения
import-scrivener-note-saved-searches = Сохранённые поиски
import-scrivener-note-custom-metadata = Пользовательские метаданные
import-scrivener-note-targets = Целевые объёмы
import-scrivener-note-bookmarks = Закладки
import-scrivener-abbreviated-title = Краткое название
import-scrivener-subjects = Темы
import-scrivener-company = Организация
import-scrivener-session-target = Целевой объём сеанса
import-scrivener-session-target-calculated = Целевой объём сеанса, рассчитанный Scrivener по сроку
import-scrivener-writing-days = Рабочие дни
import-scrivener-weekday-monday = понедельник
import-scrivener-weekday-tuesday = вторник
import-scrivener-weekday-wednesday = среда
import-scrivener-weekday-thursday = четверг
import-scrivener-weekday-friday = пятница
import-scrivener-weekday-saturday = суббота
import-scrivener-weekday-sunday = воскресенье
import-scrivener-draft-target = Целевой объём рукописи
import-scrivener-deadline = Срок
# The unit of a target, in parentheses after its number in the note that lists the
# project's targets: “80000 (words)”. A label for the unit, not part of a sentence, and no
# count reaches it: use the form your language gives a unit standing alone, as in a
# column heading. The same for the two keys below.
import-scrivener-unit-words = слов
import-scrivener-unit-characters = знаков
import-scrivener-unit-pages = страниц
import-scrivener-search-query = Что искали
import-scrivener-search-in = Где искали
import-scrivener-search-operator = Совпадение
# What a saved search looks in, by the kind Scrivener names.
import-scrivener-search-in-all = везде
import-scrivener-search-in-title = названия
import-scrivener-search-in-text = текст
import-scrivener-search-in-notes = заметки
import-scrivener-search-in-synopsis = синопсисы
import-scrivener-search-in-keywords = ключевые слова
import-scrivener-search-in-label = метки
import-scrivener-search-in-status = статусы
import-scrivener-search-in-section-type = типы разделов
import-scrivener-search-in-custom-metadata = пользовательские метаданные
# How a saved search's words combine.
import-scrivener-search-any = любое слово
import-scrivener-search-all = все слова
import-scrivener-search-exact = точная фраза
# One line of a listing: a bookmark's place and its note, or a link's words and its address.
import-scrivener-label-value = { $label }: { $value }
import-scrivener-replacement-regex = регулярное выражение
import-scrivener-replacement-off = выключено
# In the notes that list custom metadata and bookmarks, said after the path of a document
# that became a note: its values or bookmarks are listed in that note's own text.
import-scrivener-in-own-note = Перечислено в заметке, в которую превратился этот документ.
# A refusal on one line: its title, then what to do.
import-scrivener-paragraph = { $title }. { $body }

## When the project cannot be imported
## A body that ends “then read the project again” is shown in the wizard beside the button
## import-scrivener-check-again, so it uses the same words. A failed import's notice shows
## the same body beside import-scrivener-review-again, which reads the project again too:
## the body quotes no button.
import-scrivener-refusal-missing-title = По этому пути ничего нет
import-scrivener-refusal-missing-body = «{ $file }» больше нет на месте. Выберите проект снова.
import-scrivener-refusal-unreadable-title = Этот проект не удаётся прочитать
import-scrivener-refusal-unreadable-body = Skribisto не удалось прочитать «{ $file }» ({ $reason }). Убедитесь, что его можно открыть, и попробуйте снова.
import-scrivener-refusal-not-a-scrivener-project-title = Здесь нет проекта Scrivener
import-scrivener-refusal-not-a-scrivener-project-body = Выберите папку проекта .scriv, файл .scrivx внутри неё, резервную копию .zip или файл .scrivtemplate.
import-scrivener-refusal-manifest-without-files-title = В этой копии нет документов
import-scrivener-refusal-manifest-without-files-body = «{ $manifest }» здесь есть, а папки Files с перечисленными в нём документами нет. Возможно, служба синхронизации ещё не закончила копировать проект или скопирован только файл .scrivx. Если вы выбрали файл .scrivx, выберите вместо него папку проекта кнопкой «Выбрать папку…».
import-scrivener-refusal-search-too-wide-title = В этой папке слишком много всего для поиска
import-scrivener-refusal-search-too-wide-body = В ней больше { $limit } папок для просмотра. Выберите сам проект Scrivener.
import-scrivener-refusal-scrivener-one-for-mac-title = Этот проект создан в Scrivener 1 для Mac
import-scrivener-refusal-scrivener-one-for-mac-body = Skribisto читает проекты Scrivener 2 и более поздних версий. Откройте его один раз в Scrivener 2 или 3, чтобы обновить, или скомпилируйте его в документ Word и импортируйте этот документ.
import-scrivener-refusal-ios-stub-without-mobile-data-title = Это лишь файл-заглушка из Scrivener для iPhone или iPad
import-scrivener-refusal-ios-stub-without-mobile-data-body = Самого проекта здесь нет. Синхронизируйте его с iPhone или iPad или отправьте оттуда в виде .zip и импортируйте этот файл.
import-scrivener-refusal-newer-title = Этот проект создан более новой версией Scrivener
import-scrivener-refusal-unknown-format-version-body = Его формат, { $version }, Skribisto не знаком. Проверьте, нет ли обновления Skribisto. А пока скомпилируйте проект в Scrivener в документ Word и импортируйте этот документ.
import-scrivener-refusal-newer-format-stamp-body = Его файлы помечены версией { $stamp }, а Skribisto читает версии до { $newest }. Проверьте, нет ли обновления Skribisto. А пока скомпилируйте проект в Scrivener в документ Word и импортируйте этот документ.
import-scrivener-refusal-version-stamp-unreadable-title = Не удаётся прочитать версию проекта
import-scrivener-refusal-version-stamp-unreadable-body = В его файле Files/version.txt нет номера версии, поэтому Skribisto не может понять, умеет ли он читать этот проект. Откройте проект один раз в Scrivener, затем импортируйте его снова.
import-scrivener-refusal-damaged-manifest-title = Не удаётся прочитать файл подшивки
import-scrivener-refusal-damaged-manifest-body = «{ $manifest }» повреждён. { $spare_copies ->
    [0] Откройте проект в Scrivener (он умеет исправлять такие файлы), затем импортируйте проект снова.
    [1] Scrivener хранит внутри проекта запасную копию этого файла, но Skribisto её не читает. Откройте проект в Scrivener (он умеет исправлять такие файлы), затем импортируйте проект снова.
    [one] Scrivener хранит внутри проекта { $spare_copies } запасную копию этого файла, но Skribisto их не читает. Откройте проект в Scrivener (он умеет исправлять такие файлы), затем импортируйте проект снова.
    [few] Scrivener хранит внутри проекта { $spare_copies } запасные копии этого файла, но Skribisto их не читает. Откройте проект в Scrivener (он умеет исправлять такие файлы), затем импортируйте проект снова.
    [many] Scrivener хранит внутри проекта { $spare_copies } запасных копий этого файла, но Skribisto их не читает. Откройте проект в Scrivener (он умеет исправлять такие файлы), затем импортируйте проект снова.
   *[other] Scrivener хранит внутри проекта { $spare_copies } запасной копии этого файла, но Skribisto их не читает. Откройте проект в Scrivener (он умеет исправлять такие файлы), затем импортируйте проект снова.
}
# The empty file Scrivener writes for a new project before its first save: not damage.
import-scrivener-refusal-damaged-manifest-new-project-placeholder-body = «{ $manifest }» — пустой файл, который Scrivener создаёт для ещё не сохранённого проекта. Откройте проект в Scrivener и дайте ему сохраниться, затем импортируйте проект снова.
import-scrivener-refusal-manifest-too-large-title = Этот файл подшивки слишком велик
import-scrivener-refusal-manifest-too-large-body = «{ $manifest }» больше { $limit } МБ — предела, который читает Skribisto. Ни один настоящий проект и близко к этому не подходит, так что файл, возможно, повреждён.
import-scrivener-refusal-no-item-identifiers-title = В подшивке нет ни одного документа, который Skribisto может найти
import-scrivener-refusal-no-item-identifiers-body = Ни у одного из её элементов нет идентификатора, поэтому их файлы не найти. Откройте проект один раз в Scrivener, затем импортируйте его снова.
import-scrivener-refusal-incomplete-copy-title = В этой копии не хватает документов
import-scrivener-refusal-incomplete-copy-body = Папка Files здесь есть, а документов в ней нет. Возможно, служба синхронизации ещё не закончила копировать проект. Дождитесь, пока она закончит, или скопируйте всю папку .scriv заново, затем перечитайте проект.
import-scrivener-refusal-archive-title = Этот архив не удаётся открыть
import-scrivener-refusal-archive-unreadable-body = Не удалось прочитать «{ $file }» ({ $reason }). Возможно, он повреждён. Распакуйте его сами, затем выберите папку .scriv внутри.
import-scrivener-refusal-archive-unsafe-name-title = Этот архив нельзя безопасно открыть
# A name with `..` in it, or an absolute one.
import-scrivener-refusal-archive-unsafe-name-body = В «{ $file }» есть файл с именем «{ $member }», который оказался бы за пределами архива, поэтому Skribisto не открывает этот архив.
import-scrivener-refusal-archive-unsafe-name-empty-body = В «{ $file }» есть файл без имени, поэтому Skribisto не открывает этот архив.
import-scrivener-refusal-archive-unsafe-name-control-character-body = В «{ $file }» есть файл, имя которого содержит управляющий символ, поэтому Skribisto не открывает этот архив.
import-scrivener-refusal-archive-too-large-title = Этот архив слишком велик, чтобы безопасно его открыть
import-scrivener-refusal-archive-too-many-members-body = В нём больше { $limit } файлов. Распакуйте его сами, затем выберите папку .scriv внутри.
import-scrivener-refusal-archive-member-too-large-body = «{ $member }» после распаковки занимает больше { $limit } МБ. Распакуйте архив сами, затем выберите папку .scriv внутри.
import-scrivener-refusal-archive-too-large-body = После распаковки он занимает больше { $limit } МБ. Распакуйте его сами, затем выберите папку .scriv внутри.
import-scrivener-refusal-archive-encrypted-title = Этот архив защищён паролем
import-scrivener-refusal-archive-encrypted-body = Распакуйте «{ $file }» сами, затем выберите папку .scriv внутри.
import-scrivener-refusal-archive-unsupported-compression-title = Этот архив сжат способом, который Skribisto не умеет читать
import-scrivener-refusal-archive-unsupported-compression-body = «{ $member }» сжат методом { $method }. Распакуйте архив сами, затем выберите папку .scriv внутри.
import-scrivener-refusal-duplicate-archive-members-title = В этом архиве некоторые файлы перечислены дважды
import-scrivener-refusal-duplicate-archive-members-body = В «{ $file }» несколько файлов с одинаковым именем, поэтому Skribisto не может понять, какой из них относится к проекту. Распакуйте его сами, затем выберите папку .scriv внутри.
# What a file cut short looks like; a file too large, one that is not a template and one
# encoded another way have their own words.
import-scrivener-refusal-template-undecodable-title = Этот файл шаблона неполон
import-scrivener-refusal-template-undecodable-too-large-title = Этот файл шаблона слишком велик, чтобы его открыть
import-scrivener-refusal-template-undecodable-not-a-template-title = Этот файл не является шаблоном Scrivener
import-scrivener-refusal-template-undecodable-unknown-encoding-title = Этот файл шаблона сохранён в незнакомом Skribisto виде
import-scrivener-refusal-template-undecodable-body = Не удаётся декодировать «{ $file }»: возможно, он обрывается раньше, чем заканчивается проект внутри него. Скачайте его снова или откройте в Scrivener и создайте из него проект.
import-scrivener-refusal-template-undecodable-too-large-body = «{ $file }» больше { $limit } МБ. Откройте его в Scrivener, создайте из него проект, затем импортируйте этот проект.
import-scrivener-refusal-template-undecodable-not-a-template-body = В «{ $file }» нет проекта Scrivener. Выберите файл шаблона, сохранённый Scrivener, или проект, созданный из него.
import-scrivener-refusal-template-undecodable-unknown-encoding-body = «{ $file }» хранит свой проект в виде, который Skribisto не умеет декодировать. Откройте его в Scrivener, создайте из него проект, затем импортируйте этот проект.
import-scrivener-refusal-cloud-placeholders-title = { $count ->
    [one] { $count } файл этого проекта ещё в облаке
    [few] { $count } файла этого проекта ещё в облаке
    [many] { $count } файлов этого проекта ещё в облаке
   *[other] { $count } файла этого проекта ещё в облаке
}
import-scrivener-refusal-cloud-placeholders-body = { $count ->
    [1] Он не загружен на этот компьютер: { $examples }. Сделайте проект доступным офлайн в приложении синхронизации, затем перечитайте проект.
   *[other] Они не загружены на этот компьютер, например { $examples }. Сделайте проект доступным офлайн в приложении синхронизации, затем перечитайте проект.
}
import-scrivener-refusal-unreadable-members-title = { $count ->
    [one] { $count } файл этого проекта не удаётся прочитать
    [few] { $count } файла этого проекта не удаётся прочитать
    [many] { $count } файлов этого проекта не удаётся прочитать
   *[other] { $count } файла этого проекта не удаётся прочитать
}
import-scrivener-refusal-unreadable-members-body = Например, { $examples }. Убедитесь, что у вас есть право читать весь проект и что диск исправен, затем перечитайте проект.
import-scrivener-refusal-needed-member-is-link-title = Этот проект ссылается за пределы своей папки
import-scrivener-refusal-needed-member-is-link-body = «{ $member }» — ссылка на другое место на этом компьютере, а Skribisto не переходит по ссылкам. Скопируйте проект так, чтобы все его файлы были внутри него, и попробуйте снова.
import-scrivener-refusal-project-changed-during-read-title = Проект изменился во время чтения
import-scrivener-refusal-project-changed-during-read-body = Scrivener или служба синхронизации ещё записывает в него. Закройте его в Scrivener, дождитесь окончания синхронизации, затем перечитайте проект.
import-scrivener-refusal-too-many-members-title = В этом проекте слишком много файлов
import-scrivener-refusal-too-many-members-body = В нём больше { $limit } файлов, больше, чем в любом настоящем проекте. Возможно, это не проект Scrivener или он повреждён.
import-scrivener-refusal-too-large-title = Этот проект слишком велик
import-scrivener-refusal-too-large-body = Его тексты занимают больше { $limit } МБ — больше, чем Skribisto может безопасно импортировать.
import-scrivener-refusal-structure-unmappable-title = Skribisto не удаётся перенести эту подшивку
import-scrivener-refusal-structure-unmappable-body = Подшивку не удалось перенести так, чтобы ничего не потерять и ничего не поставить не на своё место. Это ошибка Skribisto, а не вашего проекта. Пожалуйста, сообщите о ней и приложите то, что показывают «Подробности».

## When there is a choice to make
import-scrivener-roots-title = { $count ->
    [one] Здесь { $count } проект. Какой импортировать?
    [few] Здесь { $count } проекта. Какой импортировать?
    [many] Здесь { $count } проектов. Какой импортировать?
   *[other] Здесь { $count } проекта. Какой импортировать?
}
import-scrivener-roots-hint = В этой папке или архиве несколько проектов Scrivener. Выберите тот, который нужно импортировать.
import-scrivener-manifests-title = { $count ->
    [one] В этом проекте { $count } файл подшивки
    [few] В этом проекте { $count } файла подшивки
    [many] В этом проекте { $count } файлов подшивки
   *[other] В этом проекте { $count } файла подшивки
}
import-scrivener-manifests-hint = Scrivener хранит подшивку, то есть список всех документов и папок, в файле .scrivx. Лишние файлы, скорее всего, оставила служба синхронизации. Выберите тот, в котором вы работали последним.
import-scrivener-manifests-hint-preselected = Scrivener хранит подшивку, то есть список всех документов и папок, в файле .scrivx. Лишние файлы, скорее всего, оставила служба синхронизации. Выбран тот, в котором вы, скорее всего, работали последним: проверьте это, прежде чем продолжить.
import-scrivener-manifest-row = { $file } · сохранён { $date } · { $documents ->
    [one] { $documents } элемент
    [few] { $documents } элемента
    [many] { $documents } элементов
   *[other] { $documents } элемента
}
import-scrivener-manifest-row-undated = { $file } · { $documents ->
    [one] { $documents } элемент
    [few] { $documents } элемента
    [many] { $documents } элементов
   *[other] { $documents } элемента
}
import-scrivener-manifest-row-unread = { $file } · не читается
import-scrivener-enclosing-title = Это часть проекта Scrivener «{ $project }»
import-scrivener-enclosing-body = Вы выбрали файл или папку внутри проекта. Skribisto импортирует проект целиком.
import-scrivener-enclosing-use = Импортировать «{ $project }»
import-scrivener-upgrade-title = Это копия, которую Scrivener сохранил перед обновлением
import-scrivener-upgrade-body = Обновляя проект до новой версии, Scrivener сохраняет старый под именем «{ $backup }». Проект, в котором вы продолжили писать, — «{ $current }», он лежит рядом.
import-scrivener-upgrade-body-unclear = Обновляя проект до новой версии, Scrivener сохраняет старый под именем «{ $backup }». Проект рядом с ним, «{ $current }», не удалось прочитать без ошибок, поэтому выберите, какой импортировать.
import-scrivener-upgrade-use-current = Импортировать «{ $current }» — проект, в котором вы продолжили писать
import-scrivener-upgrade-keep-backup = Импортировать эту более старую копию, «{ $backup }»

## What must be accepted before going on
import-scrivener-ios-pending-title = Изменений, сделанных в Scrivener для iPhone или iPad, в этой копии ещё нет
import-scrivener-ios-pending-body = Scrivener на компьютере внесёт эти изменения в проект, когда в следующий раз его откроет. Откройте проект один раз в Scrivener на компьютере, затем импортируйте его снова.
import-scrivener-ios-pending-accept = Импортировать без этих изменений
import-scrivener-no-text-title = { $count ->
    [one] { $count } документ, и в этой копии нет ни слова текста
    [few] { $count } документа, и в этой копии нет ни слова текста
    [many] { $count } документов, и в этой копии нет ни слова текста
   *[other] { $count } документа, и в этой копии нет ни слова текста
}
import-scrivener-no-text-body = Возможно, служба синхронизации ещё копирует проект или копия потеряла свои документы. Если импортировать сейчас, все документы будут перенесены пустыми.
import-scrivener-no-text-accept = Импортировать документы пустыми

## Things to know, nothing to decide
import-scrivener-notice-project-found-below-pick-title = Проект найден внутри выбранной папки
import-scrivener-notice-project-found-below-pick-body = Он находится в «{ $path }».
import-scrivener-notice-source-is-template-title = Это шаблон Scrivener
import-scrivener-notice-source-is-template-named = Это шаблон Scrivener «{ $title }»
import-scrivener-notice-source-is-template-body = При импорте из содержимого шаблона, включая его инструкции, создаётся новый проект.
import-scrivener-notice-source-is-backup-title = Похоже, это резервная копия, сделанная Scrivener
import-scrivener-notice-source-is-backup-body = В ней проект в том виде, каким он был в момент создания копии. Если после этого вы продолжали писать, выберите сам проект.
import-scrivener-notice-importing-pre-upgrade-copy-title = Вы импортируете копию, которую Scrivener сохранил перед обновлением
import-scrivener-notice-importing-pre-upgrade-copy-body = Проект, в котором вы продолжили писать, лежит рядом, и в нём есть всё, что написано с тех пор.
import-scrivener-notice-imported-from-ios-stub-title = Этот проект создан в Scrivener для iPhone или iPad
import-scrivener-notice-imported-from-ios-stub-body = Его ещё ни разу не открывали на компьютере, поэтому он читается из копии, которую хранит iPhone или iPad.
import-scrivener-notice-stale-mobile-binder-ignored-title = Старая подшивка с iPhone или iPad не учитывается
import-scrivener-notice-stale-mobile-binder-ignored-body = Scrivener на компьютере уже внёс её изменения в проект.
import-scrivener-notice-identical-manifest-copies-title = { $count ->
    [one] В этом проекте { $count } одинаковый файл подшивки
    [few] В этом проекте { $count } одинаковых файла подшивки
    [many] В этом проекте { $count } одинаковых файлов подшивки
   *[other] В этом проекте { $count } одинакового файла подшивки
}
import-scrivener-notice-identical-manifest-copies-body = В них одно и то же, поэтому любая из них даёт тот же проект.
import-scrivener-notice-manifest-copies-ignored-title = Лишние копии файла подшивки не учитываются
import-scrivener-notice-manifest-copies-ignored-body = «{ $chosen }» — тот, в котором Scrivener работал последним. { $count ->
    [one] { $count } копия, оставленная службой синхронизации, не учитывается.
    [few] { $count } копии, оставленные службой синхронизации, не учитываются.
    [many] { $count } копий, оставленных службой синхронизации, не учитываются.
   *[other] { $count } копии, оставленные службой синхронизации, не учитываются.
}
import-scrivener-notice-pinned-manifest-looks-like-a-copy-title = Выбранный вами файл подшивки похож на копию, созданную синхронизацией
import-scrivener-notice-pinned-manifest-looks-like-a-copy-body = «{ $name }» назван как копия «{ $original }», который лежит рядом. Если результат импорта покажется устаревшим, выберите вместо него «{ $original }».
import-scrivener-notice-pinned-manifest-has-siblings-title = { $count ->
    [one] Рядом с выбранным вами лежит ещё { $count } файл подшивки
    [few] Рядом с выбранным вами лежат ещё { $count } файла подшивки
    [many] Рядом с выбранным вами лежат ещё { $count } файлов подшивки
   *[other] Рядом с выбранным вами лежат ещё { $count } файла подшивки
}
import-scrivener-notice-pinned-manifest-has-siblings-body = Читается только выбранный вами.
import-scrivener-notice-legacy-v1-files-beside-title = Файлы старой версии Scrivener не учитываются
import-scrivener-notice-legacy-v1-files-beside-body = Они остались после обновления проекта. Читаются только файлы текущей версии.
import-scrivener-notice-mixed-payload-trees-title = В этом проекте есть файлы двух версий Scrivener
import-scrivener-notice-mixed-payload-trees-body = Документы читаются оттуда, где их указывает файл подшивки.
import-scrivener-notice-version-disagreement-title = Файлы этого проекта по-разному указывают его версию
import-scrivener-notice-id-scheme-mismatch-body = Файл подшивки указывает версию { $version }, но называет документы так, как это делает другая версия. Они читаются там, куда он указывает. Если результат импорта выглядит неправильным, откройте проект один раз в Scrivener и импортируйте его снова.
import-scrivener-notice-version-stamp-mismatch-body = Он читается так, как его описывает файл подшивки. Если результат импорта выглядит неправильным, откройте проект один раз в Scrivener и импортируйте его снова.
import-scrivener-notice-unrecognised-writer-title = Этот проект сохранён программой, которую Skribisto не распознаёт
import-scrivener-notice-unrecognised-writer-named = Она называет себя «{ $creator }». Проект читается как проект Scrivener.
import-scrivener-notice-unrecognised-writer-body = Она никак себя не называет. Проект читается как проект Scrivener.
import-scrivener-notice-invalid-item-identifier-title = { $count ->
    [one] У { $count } элемента подшивки недопустимый идентификатор
    [few] У { $count } элементов подшивки недопустимые идентификаторы
    [many] У { $count } элементов подшивки недопустимые идентификаторы
   *[other] У { $count } элемента подшивки недопустимые идентификаторы
}
import-scrivener-notice-invalid-item-identifier-body = Такой идентификатор не учитывается, а элемент, у которого он есть, сохраняется.
import-scrivener-notice-items-without-identifier-title = { $count ->
    [one] У { $count } элемента подшивки нет идентификатора
    [few] У { $count } элементов подшивки нет идентификатора
    [many] У { $count } элементов подшивки нет идентификатора
   *[other] У { $count } элемента подшивки нет идентификатора
}
import-scrivener-notice-items-without-identifier-body = Без идентификатора файлы элемента не найти, поэтому он переносится пустым.
import-scrivener-notice-duplicate-item-key-title = У двух элементов подшивки один и тот же идентификатор: «{ $item }»
import-scrivener-notice-duplicate-item-key-body = Первый сохраняет документы. Второй сохраняет своё место, но переносится пустым.
import-scrivener-notice-lock-here-title = Этот проект открыт в Scrivener на этом компьютере
import-scrivener-notice-lock-here-body = Сначала закройте его в Scrivener, чтобы все изменения были сохранены.
import-scrivener-notice-lock-title = Этот проект был открыт в Scrivener на компьютере «{ $host }»
import-scrivener-notice-lock-elsewhere-title = Этот проект был открыт в Scrivener на другом компьютере
import-scrivener-notice-lock-body = Если он там ещё открыт, сначала закройте его: в файлах может ещё не быть последних изменений. Если Scrivener закрыли давно, проект можно импортировать как есть.
import-scrivener-notice-archived-while-open-title = Этот архив создан, когда проект был открыт в Scrivener
import-scrivener-notice-archived-while-open-body = Изменений, сделанных после этого момента, в нём может не быть.
import-scrivener-notice-content-missing-notes = Нет заметок к «{ $item }»
import-scrivener-notice-content-missing-synopsis = Нет синопсиса «{ $item }»
import-scrivener-notice-content-missing-text = Нет текста «{ $item }»
import-scrivener-notice-content-missing-body = { $place ->
    [notes] Их файла нет в этой копии, поэтому они переносятся пустыми. Возможно, служба синхронизации ещё копирует проект.
    [synopsis] Его файла нет в этой копии, поэтому он переносится пустым. Возможно, служба синхронизации ещё копирует проект.
   *[text] Его файла нет в этой копии, поэтому он переносится пустым. Возможно, служба синхронизации ещё копирует проект.
}
import-scrivener-notice-content-missing-salvaged = { $place ->
    [notes] Их файла нет в этой копии, но их слова ещё оставались в поисковом индексе Scrivener. Они сохраняются простым текстом, возможно, из более старой версии.
    [synopsis] Его файла нет в этой копии, но его слова ещё оставались в поисковом индексе Scrivener. Они сохраняются простым текстом, возможно, из более старой версии.
   *[text] Его файла нет в этой копии, но его слова ещё оставались в поисковом индексе Scrivener. Они сохраняются простым текстом, возможно, из более старой версии.
}
import-scrivener-notice-content-truncated-notes = Заметки к «{ $item }» на диске пусты
import-scrivener-notice-content-truncated-synopsis = Синопсис «{ $item }» на диске пуст
import-scrivener-notice-content-truncated-text = Текст «{ $item }» на диске пуст
import-scrivener-notice-content-truncated-body = { $place ->
    [notes] По записям Scrivener в них есть текст, но их файл пуст: возможно, синхронизация остановилась на полпути. Они переносятся пустыми.
    [synopsis] По записям Scrivener в нём есть текст, но его файл пуст: возможно, синхронизация остановилась на полпути. Он переносится пустым.
   *[text] По записям Scrivener в нём есть текст, но его файл пуст: возможно, синхронизация остановилась на полпути. Он переносится пустым.
}
import-scrivener-notice-missing-media-title = Нет файла «{ $item }»
import-scrivener-notice-missing-media-body = Изображения или файла исследований, который в нём хранится, в этой копии нет.
import-scrivener-notice-sync-conflict-copies-title = { $count ->
    [one] Найдена { $count } копия документа, созданная синхронизацией
    [few] Найдены { $count } копии документов, созданные синхронизацией
    [many] Найдено { $count } копий документов, созданных синхронизацией
   *[other] Найдено { $count } копии документов, созданные синхронизацией
}
import-scrivener-notice-sync-conflict-copies-body = { $missing ->
    [0] Копии, созданные синхронизацией, переносятся как восстановленные тексты, в отдельную папку, чтобы вы могли сравнить их с документами.
    [1] Копии, созданные синхронизацией, переносятся как восстановленные тексты, в отдельную папку. Одна из них — копия документа, которого в проекте больше нет.
   *[other] Копии, созданные синхронизацией, переносятся как восстановленные тексты, в отдельную папку. { $missing } из них — копии документов, которых в проекте больше нет.
}
import-scrivener-notice-recovered-unreferenced-text-title = { $count ->
    [one] Найден { $count } документ, которого нет в подшивке
    [few] Найдены { $count } документа, которых нет в подшивке
    [many] Найдено { $count } документов, которых нет в подшивке
   *[other] Найдено { $count } документа, которых нет в подшивке
}
import-scrivener-notice-recovered-unreferenced-text-body = Такие документы переносятся как восстановленные тексты, в отдельную папку.
import-scrivener-notice-sync-debris-ignored-title = { $count ->
    [one] { $count } лишняя копия, оставленная службой синхронизации, не учитывается
    [few] { $count } лишние копии, оставленные службой синхронизации, не учитываются
    [many] { $count } лишних копий, оставленных службой синхронизации, не учитываются
   *[other] { $count } лишней копии, оставленные службой синхронизации, не учитываются
}
import-scrivener-notice-sync-debris-ignored-body = Scrivener такие копии тоже не читает. Читаются только собственные файлы проекта.
import-scrivener-notice-unrecognised-files-title = { $count ->
    [one] { $count } файл, не относящийся к проекту Scrivener, не учитывается
    [few] { $count } файла, не относящиеся к проекту Scrivener, не учитываются
    [many] { $count } файлов, не относящихся к проекту Scrivener, не учитываются
   *[other] { $count } файла, не относящиеся к проекту Scrivener, не учитываются
}
import-scrivener-notice-examples = Например, { $examples }.
import-scrivener-notice-symlink-not-followed-title = { $count ->
    [one] Skribisto не переходит по { $count } ссылке внутри проекта
    [few] Skribisto не переходит по { $count } ссылкам внутри проекта
    [many] Skribisto не переходит по { $count } ссылкам внутри проекта
   *[other] Skribisto не переходит по { $count } ссылкам внутри проекта
}
import-scrivener-notice-unreadable-files-skipped-title = { $count ->
    [one] Не удалось прочитать { $count } файл, который не нужен для импорта
    [few] Не удалось прочитать { $count } файла, которые не нужны для импорта
    [many] Не удалось прочитать { $count } файлов, которые не нужны для импорта
   *[other] Не удалось прочитать { $count } файла, которые не нужны для импорта
}
import-scrivener-notice-cloud-placeholders-skipped-title = { $count ->
    [one] { $count } файл, который ещё в облаке, пропущен
    [few] { $count } файла, которые ещё в облаке, пропущены
    [many] { $count } файлов, которые ещё в облаке, пропущены
   *[other] { $count } файла, которые ещё в облаке, пропущены
}
import-scrivener-notice-cloud-placeholders-skipped-body = Ничего из того, что нужно для импорта, в облаке не осталось.
import-scrivener-notice-no-text-found-title = { $count ->
    [1] { $count } документ, и в нём нет текста
    [one] { $count } документ, и ни в одном нет текста
    [few] { $count } документа, и ни в одном нет текста
    [many] { $count } документов, и ни в одном нет текста
   *[other] { $count } документа, и ни в одном нет текста
}
import-scrivener-notice-no-text-found-body = Этот проект выглядит пустым. Он импортируется как есть.
import-scrivener-notice-metadata-unreadable-title = Не удалось прочитать «{ $member }»
import-scrivener-notice-metadata-unreadable-body = То, что Scrivener хранил в нём, не переносится.
import-scrivener-notice-dangling-keyword-title = { $count ->
    [one] { $count } ссылка указывает на ключевое слово, которого в проекте нет
    [few] { $count } ссылки указывают на ключевые слова, которых в проекте нет
    [many] { $count } ссылок указывают на ключевые слова, которых в проекте нет
   *[other] { $count } ссылки указывают на ключевые слова, которых в проекте нет
}
import-scrivener-notice-dangling-label-title = { $count ->
    [one] { $count } ссылка указывает на метку, которой в проекте нет
    [few] { $count } ссылки указывают на метки, которых в проекте нет
    [many] { $count } ссылок указывают на метки, которых в проекте нет
   *[other] { $count } ссылки указывают на метки, которых в проекте нет
}
import-scrivener-notice-dangling-status-title = { $count ->
    [one] { $count } ссылка указывает на статус, которого в проекте нет
    [few] { $count } ссылки указывают на статусы, которых в проекте нет
    [many] { $count } ссылок указывают на статусы, которых в проекте нет
   *[other] { $count } ссылки указывают на статусы, которых в проекте нет
}
import-scrivener-notice-dangling-section-type-title = { $count ->
    [one] { $count } ссылка указывает на тип раздела, которого в проекте нет
    [few] { $count } ссылки указывают на типы разделов, которых в проекте нет
    [many] { $count } ссылок указывают на типы разделов, которых в проекте нет
   *[other] { $count } ссылки указывают на типы разделов, которых в проекте нет
}
import-scrivener-notice-dangling-custom-field-title = { $count ->
    [one] { $count } ссылка указывает на пользовательское поле метаданных, которого в проекте нет
    [few] { $count } ссылки указывают на пользовательские поля метаданных, которых в проекте нет
    [many] { $count } ссылок указывают на пользовательские поля метаданных, которых в проекте нет
   *[other] { $count } ссылки указывают на пользовательские поля метаданных, которых в проекте нет
}
import-scrivener-notice-dangling-item-title = { $count ->
    [one] { $count } ссылка указывает на документ, которого в проекте нет
    [few] { $count } ссылки указывают на документы, которых в проекте нет
    [many] { $count } ссылок указывают на документы, которых в проекте нет
   *[other] { $count } ссылки указывают на документы, которых в проекте нет
}
import-scrivener-notice-dangling-references-body = Ссылка на то, что в проекте не определено, не переносится.
import-scrivener-notice-snapshots-not-imported-line = { $count ->
    [one] { $count } снимок остаётся в вашем проекте Scrivener: Skribisto не переносит снимки.
    [few] { $count } снимка остаются в вашем проекте Scrivener: Skribisto не переносит снимки.
    [many] { $count } снимков остаются в вашем проекте Scrivener: Skribisto не переносит снимки.
   *[other] { $count } снимка остаются в вашем проекте Scrivener: Skribisto не переносит снимки.
}
import-scrivener-notice-writing-history-not-imported-line = { $days ->
    [one] Ежедневная статистика написанных слов из Scrivener ({ $days } день), не переносится. Skribisto начинает собственный подсчёт с первого сохранения.
    [few] Ежедневная статистика написанных слов из Scrivener ({ $days } дня), не переносится. Skribisto начинает собственный подсчёт с первого сохранения.
    [many] Ежедневная статистика написанных слов из Scrivener ({ $days } дней), не переносится. Skribisto начинает собственный подсчёт с первого сохранения.
   *[other] Ежедневная статистика написанных слов из Scrivener ({ $days } дня), не переносится. Skribisto начинает собственный подсчёт с первого сохранения.
}
import-scrivener-notice-title-placeholder-rejected-hint = Название в настройках проекта — заполнитель из шаблона, поэтому вместо него предложено имя проекта.
import-scrivener-notice-author-placeholder-rejected-hint = Имя автора в настройках проекта — заполнитель из шаблона, поэтому никакое имя не предложено.
import-scrivener-notice-title-from-project-hint-compile-settings = Это название указано в настройках компиляции проекта. Пока там не задано название, Scrivener хранит имя, которое проект получил при создании.
import-scrivener-notice-title-from-project-hint-template = Это собственное название шаблона.
import-scrivener-notice-title-from-project-hint-folder-name = Это название — имя проекта.
import-scrivener-notice-title-from-project-hint-project-properties = Это название указано в свойствах проекта.
import-scrivener-notice-author-from-project-hint = Это имя указано в настройках проекта. Там может остаться автор шаблона, и тогда это имя будет напечатано на титульном листе.
import-scrivener-notice-language-not-stated-hint = В проекте не указан язык, поэтому предложен язык интерфейса Skribisto. От языка зависит, как считаются слова и как ставятся кавычки.
import-scrivener-notice-language-from-script-hint = Предложен по письменности, которой написана рукопись ({ $tag }).

## Before you import: what the import does with the binder, or leaves behind
import-scrivener-structure-script-mode-draft-line = { $script ->
    [1] { $documents ->
        [1] Единственный документ рукописи написан в режиме сценария. Skribisto сохраняет его слова, но не вёрстку сценария.
        [one] { $script } из { $documents } документа рукописи написан в режиме сценария. Skribisto сохраняет его слова, но не вёрстку сценария.
        [few] { $script } из { $documents } документов рукописи написан в режиме сценария. Skribisto сохраняет его слова, но не вёрстку сценария.
        [many] { $script } из { $documents } документов рукописи написан в режиме сценария. Skribisto сохраняет его слова, но не вёрстку сценария.
       *[other] { $script } из { $documents } документа рукописи написан в режиме сценария. Skribisto сохраняет его слова, но не вёрстку сценария.
    }
    [one] { $documents ->
        [one] { $script } из { $documents } документа рукописи написан в режиме сценария. Skribisto сохраняет их слова, но не вёрстку сценария.
        [few] { $script } из { $documents } документов рукописи написан в режиме сценария. Skribisto сохраняет их слова, но не вёрстку сценария.
        [many] { $script } из { $documents } документов рукописи написан в режиме сценария. Skribisto сохраняет их слова, но не вёрстку сценария.
       *[other] { $script } из { $documents } документа рукописи написан в режиме сценария. Skribisto сохраняет их слова, но не вёрстку сценария.
    }
    [few] { $documents ->
        [one] { $script } из { $documents } документа рукописи написаны в режиме сценария. Skribisto сохраняет их слова, но не вёрстку сценария.
        [few] { $script } из { $documents } документов рукописи написаны в режиме сценария. Skribisto сохраняет их слова, но не вёрстку сценария.
        [many] { $script } из { $documents } документов рукописи написаны в режиме сценария. Skribisto сохраняет их слова, но не вёрстку сценария.
       *[other] { $script } из { $documents } документа рукописи написаны в режиме сценария. Skribisto сохраняет их слова, но не вёрстку сценария.
    }
    [many] { $documents ->
        [one] { $script } из { $documents } документа рукописи написаны в режиме сценария. Skribisto сохраняет их слова, но не вёрстку сценария.
        [few] { $script } из { $documents } документов рукописи написаны в режиме сценария. Skribisto сохраняет их слова, но не вёрстку сценария.
        [many] { $script } из { $documents } документов рукописи написаны в режиме сценария. Skribisto сохраняет их слова, но не вёрстку сценария.
       *[other] { $script } из { $documents } документа рукописи написаны в режиме сценария. Skribisto сохраняет их слова, но не вёрстку сценария.
    }
   *[other] { $documents ->
        [one] { $script } из { $documents } документа рукописи написаны в режиме сценария. Skribisto сохраняет их слова, но не вёрстку сценария.
        [few] { $script } из { $documents } документов рукописи написаны в режиме сценария. Skribisto сохраняет их слова, но не вёрстку сценария.
        [many] { $script } из { $documents } документов рукописи написаны в режиме сценария. Skribisto сохраняет их слова, но не вёрстку сценария.
       *[other] { $script } из { $documents } документа рукописи написаны в режиме сценария. Skribisto сохраняет их слова, но не вёрстку сценария.
    }
}
import-scrivener-structure-one-document-manuscript-line = { $count ->
    [1] Один документ рукописи содержит свои главы в виде заголовков.
    [one] { $count } документ рукописи содержит свои главы в виде заголовков.
    [few] { $count } документа рукописи содержат свои главы в виде заголовков.
    [many] { $count } документов рукописи содержат свои главы в виде заголовков.
   *[other] { $count } документа рукописи содержат свои главы в виде заголовков.
}
import-scrivener-structure-title-repeats-number-line = { $count ->
    [one] { $count } название главы или части содержит и свой номер, например «Глава II» или «Часть первая», поэтому при экспорте номер может быть напечатан дважды.
    [few] { $count } названия глав или частей содержат и свои номера, например «Глава II» или «Часть первая», поэтому при экспорте номер может быть напечатан дважды.
    [many] { $count } названий глав или частей содержат и свои номера, например «Глава II» или «Часть первая», поэтому при экспорте номер может быть напечатан дважды.
   *[other] { $count } названия глав или частей содержат и свои номера, например «Глава II» или «Часть первая», поэтому при экспорте номер может быть напечатан дважды.
}
import-scrivener-structure-claim-overruled-line = { $count ->
    [1] { $count } элемент похож на книгу, часть или главу, но находится внутри элемента, уже стоящего на этом уровне, поэтому переносится на более низкий уровень.
    [one] { $count } элемент похож на книги, части или главы, но каждый из них находится внутри элемента, уже стоящего на этом уровне, поэтому они переносятся на более низкий уровень.
    [few] { $count } элемента похожи на книги, части или главы, но каждый из них находится внутри элемента, уже стоящего на этом уровне, поэтому они переносятся на более низкий уровень.
    [many] { $count } элементов похожи на книги, части или главы, но каждый из них находится внутри элемента, уже стоящего на этом уровне, поэтому они переносятся на более низкий уровень.
   *[other] { $count } элемента похожи на книги, части или главы, но каждый из них находится внутри элемента, уже стоящего на этом уровне, поэтому они переносятся на более низкий уровень.
}
import-scrivener-structure-adopted-by-chapter-line = { $count ->
    [one] { $count } документ, идущий после главы, становится концом этой главы.
    [few] { $count } документа, идущие после главы, становятся концом этой главы.
    [many] { $count } документов, идущих после главы, становятся концом этой главы.
   *[other] { $count } документа, идущие после главы, становятся концом этой главы.
}
import-scrivener-structure-looks-like-manuscript-line = { $count ->
    [one] { $count } папка верхнего уровня, оставленная в заметках, похожа на рукопись. Её можно отправить в рукопись как ещё одну книгу.
    [few] { $count } папки верхнего уровня, оставленные в заметках, похожи на рукопись. Их можно отправить в рукопись как другие книги.
    [many] { $count } папок верхнего уровня, оставленных в заметках, похожи на рукопись. Их можно отправить в рукопись как другие книги.
   *[other] { $count } папки верхнего уровня, оставленные в заметках, похожи на рукопись. Их можно отправить в рукопись как другие книги.
}
import-scrivener-structure-generic-sheet-title-line = { $count ->
    [1] У одной записи библии истории всё ещё название из шаблона, например «Герой». Skribisto ищет это название в вашем тексте и показывает каждое место, где оно встречается, на странице «В прозе»: переименуйте запись, если это слово часто употребляется в вашей книге.
    [one] У { $count } записи библии истории всё ещё названия из шаблона, например «Герой». Skribisto ищет эти названия в вашем тексте и показывает каждое место, где они встречаются, на странице «В прозе»: переименуйте эти записи, если эти слова часто употребляются в вашей книге.
    [few] У { $count } записей библии истории всё ещё названия из шаблона, например «Герой». Skribisto ищет эти названия в вашем тексте и показывает каждое место, где они встречаются, на странице «В прозе»: переименуйте эти записи, если эти слова часто употребляются в вашей книге.
    [many] У { $count } записей библии истории всё ещё названия из шаблона, например «Герой». Skribisto ищет эти названия в вашем тексте и показывает каждое место, где они встречаются, на странице «В прозе»: переименуйте эти записи, если эти слова часто употребляются в вашей книге.
   *[other] У { $count } записи библии истории всё ещё названия из шаблона, например «Герой». Skribisto ищет эти названия в вашем тексте и показывает каждое место, где они встречаются, на странице «В прозе»: переименуйте эти записи, если эти слова часто употребляются в вашей книге.
}
import-scrivener-structure-placeholder-title-line = { $count ->
    [one] { $count } название лишь называло уровень элемента и оставлено пустым: подшивка и экспорт пронумеруют его сами.
    [few] { $count } названия лишь называли уровень элемента и оставлены пустыми: подшивка и экспорт пронумеруют их сами.
    [many] { $count } названий лишь называли уровень элемента и оставлены пустыми: подшивка и экспорт пронумеруют их сами.
   *[other] { $count } названия лишь называли уровень элемента и оставлены пустыми: подшивка и экспорт пронумеруют их сами.
}
import-scrivener-structure-untitled-item-line = { $count ->
    [one] У { $count } элемента в Scrivener не было названия, и он остаётся без названия.
    [few] У { $count } элементов в Scrivener не было названий, и они остаются без названий.
    [many] У { $count } элементов в Scrivener не было названий, и они остаются без названий.
   *[other] У { $count } элемента в Scrivener не было названий, и они остаются без названий.
}
import-scrivener-structure-title-cleaned-line = { $count ->
    [one] { $count } название содержало разрывы строк или табуляции, они удалены.
    [few] { $count } названия содержали разрывы строк или табуляции, они удалены.
    [many] { $count } названий содержали разрывы строк или табуляции, они удалены.
   *[other] { $count } названия содержали разрывы строк или табуляции, они удалены.
}
import-scrivener-structure-own-text-moved-line = { $count ->
    [one] У { $count } папки есть собственный текст; он переносится в новый первый документ внутри неё.
    [few] У { $count } папок есть собственный текст; он переносится в новый первый документ внутри каждой из них.
    [many] У { $count } папок есть собственный текст; он переносится в новый первый документ внутри каждой из них.
   *[other] У { $count } папки есть собственный текст; он переносится в новый первый документ внутри каждой из них.
}
import-scrivener-structure-text-with-children-line = { $count ->
    [one] { $count } документ, внутри которого есть другие документы, становится папкой.
    [few] { $count } документа, внутри которых есть другие документы, становятся папками.
    [many] { $count } документов, внутри которых есть другие документы, становятся папками.
   *[other] { $count } документа, внутри которых есть другие документы, становятся папками.
}
import-scrivener-structure-empty-folder-plain-line = { $count ->
    [one] { $count } пустая папка остаётся обычной папкой, чтобы экспорт не напечатал заголовок, которого Scrivener никогда не печатал.
    [few] { $count } пустые папки остаются обычными папками, чтобы экспорт не напечатал заголовок, которого Scrivener никогда не печатал.
    [many] { $count } пустых папок остаются обычными папками, чтобы экспорт не напечатал заголовок, которого Scrivener никогда не печатал.
   *[other] { $count } пустой папки остаются обычными папками, чтобы экспорт не напечатал заголовок, которого Scrivener никогда не печатал.
}
import-scrivener-structure-book-inside-book-line = { $count ->
    [one] { $count } книга внутри книги становится частью.
    [few] { $count } книги внутри книги становятся частями.
    [many] { $count } книг внутри книги становятся частями.
   *[other] { $count } книги внутри книги становятся частями.
}
import-scrivener-structure-prologue-not-numbered-line = { $count ->
    [one] { $count } пролог или эпилог не участвует в нумерации глав.
    [few] { $count } пролога или эпилога не участвуют в нумерации глав.
    [many] { $count } прологов или эпилогов не участвуют в нумерации глав.
   *[other] { $count } пролога или эпилога не участвуют в нумерации глав.
}
import-scrivener-structure-matter-inside-book-line = { $count ->
    [one] { $count } элемент вступительных или заключительных материалов, находившийся вне рукописи, попадает внутрь книги как паратекст и не входит в экспорт, пока вы его не включите.
    [few] { $count } элемента вступительных или заключительных материалов, находившиеся вне рукописи, попадают внутрь книги как паратекст и не входят в экспорт, пока вы их не включите.
    [many] { $count } элементов вступительных или заключительных материалов, находившихся вне рукописи, попадают внутрь книги как паратекст и не входят в экспорт, пока вы их не включите.
   *[other] { $count } элемента вступительных или заключительных материалов, находившиеся вне рукописи, попадают внутрь книги как паратекст и не входят в экспорт, пока вы их не включите.
}
import-scrivener-structure-paratext-not-counted-line = { $documents ->
    [one] { $documents } документ рукописи становится паратекстом (вступительными или заключительными материалами), поэтому { $words ->
        [one] { $words } слово больше не считается
        [few] { $words } слова больше не считаются
        [many] { $words } слов больше не считаются
       *[other] { $words } слова больше не считаются
    } частью рукописи.
    [few] { $documents } документа рукописи становятся паратекстом (вступительными или заключительными материалами), поэтому { $words ->
        [one] { $words } слово больше не считается
        [few] { $words } слова больше не считаются
        [many] { $words } слов больше не считаются
       *[other] { $words } слова больше не считаются
    } частью рукописи.
    [many] { $documents } документов рукописи становятся паратекстом (вступительными или заключительными материалами), поэтому { $words ->
        [one] { $words } слово больше не считается
        [few] { $words } слова больше не считаются
        [many] { $words } слов больше не считаются
       *[other] { $words } слова больше не считаются
    } частью рукописи.
   *[other] { $documents } документа рукописи становятся паратекстом (вступительными или заключительными материалами), поэтому { $words ->
        [one] { $words } слово больше не считается
        [few] { $words } слова больше не считаются
        [many] { $words } слов больше не считаются
       *[other] { $words } слова больше не считаются
    } частью рукописи.
}
import-scrivener-structure-document-notes-line = { $count ->
    [one] Заметки { $count } документа становятся заметкой рядом с ним.
    [few] Заметки { $count } документов становятся заметками рядом с ними.
    [many] Заметки { $count } документов становятся заметками рядом с ними.
   *[other] Заметки { $count } документа становятся заметками рядом с ними.
}
import-scrivener-structure-draft-content-as-note-line = Собственный текст папки рукописи становится заметкой в начале рукописи.
import-scrivener-structure-story-bible-folder-line = { $count ->
    [one] { $count } папка персонажей или мест становится тегом библии истории.
    [few] { $count } папки персонажей или мест становятся тегами библии истории.
    [many] { $count } папок персонажей или мест становятся тегами библии истории.
   *[other] { $count } папки персонажей или мест становятся тегами библии истории.
}
import-scrivener-structure-templates-created-line = { $count ->
    [one] { $count } шаблон документа становится шаблоном заметки.
    [few] { $count } шаблона документов становятся шаблонами заметок.
    [many] { $count } шаблонов документов становятся шаблонами заметок.
   *[other] { $count } шаблона документа становятся шаблонами заметок.
}
import-scrivener-structure-template-nesting-flattened-line = { $count ->
    [one] { $count } папка шаблонов внутри другой папки разворачивается в один уровень: каждый текст в ней становится шаблоном.
    [few] { $count } папки шаблонов внутри других папок разворачиваются в один уровень: каждый текст в них становится шаблоном.
    [many] { $count } папок шаблонов внутри других папок разворачиваются в один уровень: каждый текст в них становится шаблоном.
   *[other] { $count } папки шаблонов внутри других папок разворачиваются в один уровень: каждый текст в них становится шаблоном.
}
import-scrivener-structure-template-kept-as-note-line = { $count ->
    [one] { $count } шаблон содержит то, чего не может содержать шаблон заметки, поэтому сохраняется как заметка.
    [few] { $count } шаблона содержат то, чего не может содержать шаблон заметки, поэтому сохраняются как заметки.
    [many] { $count } шаблонов содержат то, чего не может содержать шаблон заметки, поэтому сохраняются как заметки.
   *[other] { $count } шаблона содержат то, чего не может содержать шаблон заметки, поэтому сохраняются как заметки.
}
import-scrivener-structure-recovered-texts-line = { $count ->
    [one] { $count } текст, которого не было в подшивке, переносится в отдельную папку.
    [few] { $count } текста, которых не было в подшивке, переносятся в отдельную папку.
    [many] { $count } текстов, которых не было в подшивке, переносятся в отдельную папку.
   *[other] { $count } текста, которых не было в подшивке, переносятся в отдельную папку.
}
import-scrivener-structure-project-notes-line = { $count ->
    [one] { $count } заметка проекта переносится в отдельную папку.
    [few] { $count } заметки проекта переносятся в отдельную папку.
    [many] { $count } заметок проекта переносятся в отдельную папку.
   *[other] { $count } заметки проекта переносятся в отдельную папку.
}
import-scrivener-structure-trash-brought-across-line = { $items ->
    [1] Корзина переносится: { $items } элемент, его можно восстановить из корзины.
    [one] Корзина переносится: { $items } элемент, { $writing ->
        [0] ни в одном из них нет текста
       *[other] в { $writing } из них есть текст
    }, и их можно восстановить из корзины.
    [few] Корзина переносится: { $items } элемента, { $writing ->
        [0] ни в одном из них нет текста
       *[other] в { $writing } из них есть текст
    }, и их можно восстановить из корзины.
    [many] Корзина переносится: { $items } элементов, { $writing ->
        [0] ни в одном из них нет текста
       *[other] в { $writing } из них есть текст
    }, и их можно восстановить из корзины.
   *[other] Корзина переносится: { $items } элемента, { $writing ->
        [0] ни в одном из них нет текста
       *[other] в { $writing } из них есть текст
    }, и их можно восстановить из корзины.
}
import-scrivener-structure-not-compiled-line = { $count ->
    [one] { $count } элемент, который Scrivener исключал из компиляции, исключается и из экспорта.
    [few] { $count } элемента, которые Scrivener исключал из компиляции, исключаются и из экспорта.
    [many] { $count } элементов, которые Scrivener исключал из компиляции, исключаются и из экспорта.
   *[other] { $count } элемента, которые Scrivener исключал из компиляции, исключаются и из экспорта.
}
import-scrivener-structure-media-in-draft-line = { $count ->
    [one] { $count } изображение или файл в рукописи становится заметкой, потому что книга не может его содержать.
    [few] { $count } изображения или файла в рукописи становятся заметками, потому что книга не может их содержать.
    [many] { $count } изображений или файлов в рукописи становятся заметками, потому что книга не может их содержать.
   *[other] { $count } изображения или файла в рукописи становятся заметками, потому что книга не может их содержать.
}
import-scrivener-structure-left-out-line = { $count ->
    [one] { $count } папка верхнего уровня не переносится, а с ней { $items ->
        [one] { $items } элемент
        [few] { $items } элемента
        [many] { $items } элементов
       *[other] { $items } элемента
    }.
    [few] { $count } папки верхнего уровня не переносятся, а с ними { $items ->
        [one] { $items } элемент
        [few] { $items } элемента
        [many] { $items } элементов
       *[other] { $items } элемента
    }.
    [many] { $count } папок верхнего уровня не переносятся, а с ними { $items ->
        [one] { $items } элемент
        [few] { $items } элемента
        [many] { $items } элементов
       *[other] { $items } элемента
    }.
   *[other] { $count } папки верхнего уровня не переносятся, а с ними { $items ->
        [one] { $items } элемент
        [few] { $items } элемента
        [many] { $items } элементов
       *[other] { $items } элемента
    }.
}
import-scrivener-structure-trash-left-out-line = { $items ->
    [1] Корзина не переносится: { $items } элемент.
    [one] Корзина не переносится: { $items } элемент, { $writing ->
        [0] ни в одном из них нет текста
       *[other] в { $writing } из них есть текст
    }.
    [few] Корзина не переносится: { $items } элемента, { $writing ->
        [0] ни в одном из них нет текста
       *[other] в { $writing } из них есть текст
    }.
    [many] Корзина не переносится: { $items } элементов, { $writing ->
        [0] ни в одном из них нет текста
       *[other] в { $writing } из них есть текст
    }.
   *[other] Корзина не переносится: { $items } элемента, { $writing ->
        [0] ни в одном из них нет текста
       *[other] в { $writing } из них есть текст
    }.
}
import-scrivener-structure-card-image-left-behind-line = { $count ->
    [one] { $count } изображение на карточке элемента рукописи не переносится: оно было бы напечатано в книге.
    [few] { $count } изображения на карточках элементов рукописи не переносятся: они были бы напечатаны в книге.
    [many] { $count } изображений на карточках элементов рукописи не переносятся: они были бы напечатаны в книге.
   *[other] { $count } изображения на карточках элементов рукописи не переносятся: они были бы напечатаны в книге.
}
import-scrivener-structure-default-template-not-linked-line = { $count ->
    [1] У { $count } папки был шаблон по умолчанию для новых документов в ней. В Skribisto шаблон может быть только у тега библии истории, поэтому эта связь не сохраняется.
    [one] У { $count } папки были шаблоны по умолчанию для новых документов в них. В Skribisto шаблон может быть только у тега библии истории, поэтому эти связи не сохраняются.
    [few] У { $count } папок были шаблоны по умолчанию для новых документов в них. В Skribisto шаблон может быть только у тега библии истории, поэтому эти связи не сохраняются.
    [many] У { $count } папок были шаблоны по умолчанию для новых документов в них. В Skribisto шаблон может быть только у тега библии истории, поэтому эти связи не сохраняются.
   *[other] У { $count } папки были шаблоны по умолчанию для новых документов в них. В Skribisto шаблон может быть только у тега библии истории, поэтому эти связи не сохраняются.
}
import-scrivener-structure-page-break-not-kept-line = { $count ->
    [one] { $count } настройка «Page Break Before» (разрыв страницы перед документом) не сохраняется: в Skribisto для неё нет места.
    [few] { $count } настройки «Page Break Before» (разрыв страницы перед документом) не сохраняются: в Skribisto для них нет места.
    [many] { $count } настроек «Page Break Before» (разрыв страницы перед документом) не сохраняются: в Skribisto для них нет места.
   *[other] { $count } настройки «Page Break Before» (разрыв страницы перед документом) не сохраняются: в Skribisto для них нет места.
}
import-scrivener-structure-compile-as-is-not-kept-line = { $count ->
    [one] { $count } настройка «Compile As-Is» (компилировать как есть) не сохраняется: в Skribisto у неё нет аналога.
    [few] { $count } настройки «Compile As-Is» (компилировать как есть) не сохраняются: в Skribisto у них нет аналога.
    [many] { $count } настроек «Compile As-Is» (компилировать как есть) не сохраняются: в Skribisto у них нет аналога.
   *[other] { $count } настройки «Compile As-Is» (компилировать как есть) не сохраняются: в Skribisto у них нет аналога.
}

## What the import did with the labels, the status list and the dates
import-scrivener-metadata-status-categories-guessed-line = { $count ->
    [one] { $count } статус, который Skribisto не распознаёт, получает тип по своему месту в списке. Проверьте его в разделе Настройки ▸ Проект ▸ Статусы.
    [few] { $count } статуса, которые Skribisto не распознаёт, получают тип по своему месту в списке. Проверьте их в разделе Настройки ▸ Проект ▸ Статусы.
    [many] { $count } статусов, которые Skribisto не распознаёт, получают тип по своему месту в списке. Проверьте их в разделе Настройки ▸ Проект ▸ Статусы.
   *[other] { $count } статуса, которые Skribisto не распознаёт, получают тип по своему месту в списке. Проверьте их в разделе Настройки ▸ Проект ▸ Статусы.
}
import-scrivener-metadata-dates-unreadable-line = { $count ->
    [one] Даты { $count } элемента не удалось прочитать: ему присвоена дата импорта.
    [few] Даты { $count } элементов не удалось прочитать: им присвоена дата импорта.
    [many] Даты { $count } элементов не удалось прочитать: им присвоена дата импорта.
   *[other] Даты { $count } элемента не удалось прочитать: им присвоена дата импорта.
}
import-scrivener-metadata-labels-became-tags-line = { $tags ->
    [one] Метки становятся { $tags } тегом на { $rows ->
        [one] { $rows } элементе
        [few] { $rows } элементах
        [many] { $rows } элементах
       *[other] { $rows } элемента
    }.
    [few] Метки становятся { $tags } тегами на { $rows ->
        [one] { $rows } элементе
        [few] { $rows } элементах
        [many] { $rows } элементах
       *[other] { $rows } элемента
    }.
    [many] Метки становятся { $tags } тегами на { $rows ->
        [one] { $rows } элементе
        [few] { $rows } элементах
        [many] { $rows } элементах
       *[other] { $rows } элемента
    }.
   *[other] Метки становятся { $tags } тегами на { $rows ->
        [one] { $rows } элементе
        [few] { $rows } элементах
        [many] { $rows } элементах
       *[other] { $rows } элемента
    }.
}
import-scrivener-metadata-statuses-became-ladder-line = { $rungs ->
    [one] Список статусов Scrivener переносится как список статусов проекта: { $rungs } статус на { $rows ->
        [one] { $rows } элементе
        [few] { $rows } элементах
        [many] { $rows } элементах
       *[other] { $rows } элемента
    }.
    [few] Список статусов Scrivener переносится как список статусов проекта: { $rungs } статуса на { $rows ->
        [one] { $rows } элементе
        [few] { $rows } элементах
        [many] { $rows } элементах
       *[other] { $rows } элемента
    }.
    [many] Список статусов Scrivener переносится как список статусов проекта: { $rungs } статусов на { $rows ->
        [one] { $rows } элементе
        [few] { $rows } элементах
        [many] { $rows } элементах
       *[other] { $rows } элемента
    }.
   *[other] Список статусов Scrivener переносится как список статусов проекта: { $rungs } статуса на { $rows ->
        [one] { $rows } элементе
        [few] { $rows } элементах
        [many] { $rows } элементах
       *[other] { $rows } элемента
    }.
}
import-scrivener-metadata-ladder-seeded-line = { $count ->
    [1] В проекте не было списка статусов, поэтому он получает статус Skribisto по умолчанию.
    [one] В проекте не было списка статусов, поэтому он получает { $count } статус Skribisto по умолчанию.
    [few] В проекте не было списка статусов, поэтому он получает { $count } статуса Skribisto по умолчанию.
    [many] В проекте не было списка статусов, поэтому он получает { $count } статусов Skribisto по умолчанию.
   *[other] В проекте не было списка статусов, поэтому он получает { $count } статуса Skribisto по умолчанию.
}
import-scrivener-metadata-label-renamed-line = В Scrivener поле «Метка» называлось «{ $title }». Это название указано в описании каждого тега.
import-scrivener-metadata-status-renamed-line = В Scrivener поле «Статус» называлось «{ $title }». Это название указано в описании каждого статуса.
import-scrivener-metadata-none-label-collapsed-line = { $count ->
    [one] Метка «{ $name }», хранившаяся рядом с собственной пустой меткой Scrivener под тем же именем, считается отсутствием метки на { $count } элементе.
    [few] Метка «{ $name }», хранившаяся рядом с собственной пустой меткой Scrivener под тем же именем, считается отсутствием метки на { $count } элементах.
    [many] Метка «{ $name }», хранившаяся рядом с собственной пустой меткой Scrivener под тем же именем, считается отсутствием метки на { $count } элементах.
   *[other] Метка «{ $name }», хранившаяся рядом с собственной пустой меткой Scrivener под тем же именем, считается отсутствием метки на { $count } элемента.
}
import-scrivener-metadata-none-status-collapsed-line = { $count ->
    [one] Статус «{ $name }», хранившийся рядом с собственным пустым статусом Scrivener под тем же именем, считается отсутствием статуса на { $count } элементе.
    [few] Статус «{ $name }», хранившийся рядом с собственным пустым статусом Scrivener под тем же именем, считается отсутствием статуса на { $count } элементах.
    [many] Статус «{ $name }», хранившийся рядом с собственным пустым статусом Scrivener под тем же именем, считается отсутствием статуса на { $count } элементах.
   *[other] Статус «{ $name }», хранившийся рядом с собственным пустым статусом Scrivener под тем же именем, считается отсутствием статуса на { $count } элемента.
}
import-scrivener-metadata-unnamed-label-line = Метка без названия получает имя по своему номеру: { $id }.
import-scrivener-metadata-unnamed-status-line = Статус без названия получает имя по своему номеру: { $id }.
import-scrivener-metadata-tag-name-disambiguated-line = { $count ->
    [one] { $count } тег получает новое имя, потому что его имя уже занято другим тегом.
    [few] { $count } тега получают новые имена, потому что их имена уже заняты другими тегами.
    [many] { $count } тегов получают новые имена, потому что их имена уже заняты другими тегами.
   *[other] { $count } тега получают новые имена, потому что их имена уже заняты другими тегами.
}
import-scrivener-metadata-rung-names-disambiguated-line = { $count ->
    [one] У { $count } статуса такое же название, как у другого, поэтому к его названию добавлен номер. Проверьте его в разделе Настройки ▸ Проект ▸ Статусы.
    [few] У { $count } статусов такие же названия, как у других, поэтому к названию каждого добавлен номер. Проверьте их в разделе Настройки ▸ Проект ▸ Статусы.
    [many] У { $count } статусов такие же названия, как у других, поэтому к названию каждого добавлен номер. Проверьте их в разделе Настройки ▸ Проект ▸ Статусы.
   *[other] У { $count } статуса такие же названия, как у других, поэтому к названию каждого добавлен номер. Проверьте их в разделе Настройки ▸ Проект ▸ Статусы.
}
import-scrivener-metadata-unused-labels-skipped-line = { $count ->
    [one] { $count } метка, которой не отмечен ни один документ, не импортируется.
    [few] { $count } метки, которыми не отмечен ни один документ, не импортируются.
    [many] { $count } меток, которыми не отмечен ни один документ, не импортируются.
   *[other] { $count } метки, которыми не отмечен ни один документ, не импортируются.
}
import-scrivener-metadata-labels-without-row-line = { $count ->
    [one] Метка { $count } элемента, которому нет места в новом проекте, не переносится.
    [few] Метки { $count } элементов, которым нет места в новом проекте, не переносятся.
    [many] Метки { $count } элементов, которым нет места в новом проекте, не переносятся.
   *[other] Метки { $count } элемента, которым нет места в новом проекте, не переносятся.
}
import-scrivener-metadata-statuses-without-row-line = { $count ->
    [one] Статус { $count } элемента, которому нет места в новом проекте, не переносится.
    [few] Статусы { $count } элементов, которым нет места в новом проекте, не переносятся.
    [many] Статусы { $count } элементов, которым нет места в новом проекте, не переносятся.
   *[other] Статусы { $count } элемента, которым нет места в новом проекте, не переносятся.
}
import-scrivener-metadata-unnamed-keyword-line = Ключевое слово без названия получает имя по своему номеру: { $id }.
import-scrivener-metadata-keywords-without-row-line = { $count ->
    [one] Ключевые слова { $count } элемента, которому нет места в новом проекте, не переносятся.
    [few] Ключевые слова { $count } элементов, которым нет места в новом проекте, не переносятся.
    [many] Ключевые слова { $count } элементов, которым нет места в новом проекте, не переносятся.
   *[other] Ключевые слова { $count } элемента, которым нет места в новом проекте, не переносятся.
}
import-scrivener-metadata-custom-values-without-row-line = { $count ->
    [one] Пользовательские метаданные { $count } элемента, которому нет места в новом проекте, не переносятся.
    [few] Пользовательские метаданные { $count } элементов, которым нет места в новом проекте, не переносятся.
    [many] Пользовательские метаданные { $count } элементов, которым нет места в новом проекте, не переносятся.
   *[other] Пользовательские метаданные { $count } элемента, которым нет места в новом проекте, не переносятся.
}
import-scrivener-metadata-bookmarks-without-row-line = { $count ->
    [one] Закладки { $count } элемента, которому нет места в новом проекте, не переносятся.
    [few] Закладки { $count } элементов, которым нет места в новом проекте, не переносятся.
    [many] Закладки { $count } элементов, которым нет места в новом проекте, не переносятся.
   *[other] Закладки { $count } элемента, которым нет места в новом проекте, не переносятся.
}
import-scrivener-metadata-targets-without-row-line = { $count ->
    [one] Целевой объём { $count } элемента, которому нет места в новом проекте, не переносится.
    [few] Целевые объёмы { $count } элементов, которым нет места в новом проекте, не переносятся.
    [many] Целевые объёмы { $count } элементов, которым нет места в новом проекте, не переносятся.
   *[other] Целевые объёмы { $count } элемента, которым нет места в новом проекте, не переносятся.
}

## What the import does with the keywords, custom metadata, collections, bookmarks, targets,
## the deadline, the auto-complete list and the icons. Each line is said twice: on the Review
## page before the import, then in its report, so it reads the same at both times. `$note`
## and `$folder` are the titles the import gives a note and its "From Scrivener" folder;
## `$day` is a day as YYYY-MM-DD.
import-scrivener-metadata-pov-hint-line = { $count ->
    [1] Метка или значение поля { $values } совпадает с записью библии истории { $matches }. Если это точка зрения, задайте её в инспекторе: импорт ничего не связывает.
    [one] Метки или значения полей { $values } совпадают с записями библии истории { $matches }. Если это точки зрения, задайте их в инспекторе: импорт ничего не связывает.
    [few] Метки или значения полей { $values } совпадают с записями библии истории { $matches }. Если это точки зрения, задайте их в инспекторе: импорт ничего не связывает.
    [many] Метки или значения полей { $values } совпадают с записями библии истории { $matches }. Если это точки зрения, задайте их в инспекторе: импорт ничего не связывает.
   *[other] Метки или значения полей { $values } совпадают с записями библии истории { $matches }. Если это точки зрения, задайте их в инспекторе: импорт ничего не связывает.
}
import-scrivener-metadata-checkbox-default-unknown-line = У поля-флажка «{ $field }» значение по умолчанию, которое Skribisto не может прочитать, поэтому неизвестно, какие документы отмечены по умолчанию. Его тег получают только документы, отмеченные по отдельности.
import-scrivener-metadata-goals-in-other-unit-line = { $count ->
    [one] { $count } цель задана в другой единице подсчёта. Она сохраняется и появится, когда вы смените единицу подсчёта в разделе Настройки ▸ Проект ▸ Устройство проекта.
    [few] { $count } цели заданы в другой единице подсчёта. Они сохраняются и появятся, когда вы смените единицу подсчёта в разделе Настройки ▸ Проект ▸ Устройство проекта.
    [many] { $count } целей заданы в другой единице подсчёта. Они сохраняются и появятся, когда вы смените единицу подсчёта в разделе Настройки ▸ Проект ▸ Устройство проекта.
   *[other] { $count } цели заданы в другой единице подсчёта. Они сохраняются и появятся, когда вы смените единицу подсчёта в разделе Настройки ▸ Проект ▸ Устройство проекта.
}
import-scrivener-metadata-counting-differs-line = Scrivener считал целевой объём рукописи по другим документам, чем Skribisto. Цель книги учитывает сцены, включённые в экспорт.
import-scrivener-metadata-writing-days-not-read-line = Рабочие дни «{ $raw }» не удаётся прочитать, поэтому Темп книги считает рабочим каждый день. Измените их там.
import-scrivener-metadata-script-mode-rows-line = { $count ->
    [one] { $count } элемент в Scrivener был в режиме сценария. Skribisto сохраняет его слова как прозу, без вёрстки сценария.
    [few] { $count } элемента в Scrivener были в режиме сценария. Skribisto сохраняет их слова как прозу, без вёрстки сценария.
    [many] { $count } элементов в Scrivener были в режиме сценария. Skribisto сохраняет их слова как прозу, без вёрстки сценария.
   *[other] { $count } элемента в Scrivener были в режиме сценария. Skribisto сохраняет их слова как прозу, без вёрстки сценария.
}
import-scrivener-metadata-keywords-became-tags-line = { $tags ->
    [one] { $tags } ключевое слово становится тегом на { $rows ->
        [one] { $rows } элементе
        [few] { $rows } элементах
        [many] { $rows } элементах
       *[other] { $rows } элемента
    }.
    [few] { $tags } ключевых слова становятся тегами на { $rows ->
        [one] { $rows } элементе
        [few] { $rows } элементах
        [many] { $rows } элементах
       *[other] { $rows } элемента
    }.
    [many] { $tags } ключевых слов становятся тегами на { $rows ->
        [one] { $rows } элементе
        [few] { $rows } элементах
        [many] { $rows } элементах
       *[other] { $rows } элемента
    }.
   *[other] { $tags } ключевого слова становятся тегами на { $rows ->
        [one] { $rows } элементе
        [few] { $rows } элементах
        [many] { $rows } элементах
       *[other] { $rows } элемента
    }.
}
import-scrivener-metadata-keyword-hierarchy-flattened-line = { $count ->
    [one] { $count } ключевое слово в Scrivener было вложено в другое. Теги не вкладываются друг в друга: где оно было, указано в описании его тега.
    [few] { $count } ключевых слова в Scrivener были вложены в другие. Теги не вкладываются друг в друга: где было каждое ключевое слово, указано в описании его тега.
    [many] { $count } ключевых слов в Scrivener были вложены в другие. Теги не вкладываются друг в друга: где было каждое ключевое слово, указано в описании его тега.
   *[other] { $count } ключевого слова в Scrivener были вложены в другие. Теги не вкладываются друг в друга: где было каждое ключевое слово, указано в описании его тега.
}
import-scrivener-metadata-fields-became-tags-line = { $fields ->
    [one] { $fields } поле-список или поле-флажок становится { $tags ->
        [one] { $tags } тегом
        [few] { $tags } тегами
        [many] { $tags } тегами
       *[other] { $tags } тегами
    } на { $rows ->
        [one] { $rows } элементе
        [few] { $rows } элементах
        [many] { $rows } элементах
       *[other] { $rows } элемента
    }.
    [few] { $fields } поля-списка и поля-флажка становятся { $tags ->
        [one] { $tags } тегом
        [few] { $tags } тегами
        [many] { $tags } тегами
       *[other] { $tags } тегами
    } на { $rows ->
        [one] { $rows } элементе
        [few] { $rows } элементах
        [many] { $rows } элементах
       *[other] { $rows } элемента
    }.
    [many] { $fields } полей-списков и полей-флажков становятся { $tags ->
        [one] { $tags } тегом
        [few] { $tags } тегами
        [many] { $tags } тегами
       *[other] { $tags } тегами
    } на { $rows ->
        [one] { $rows } элементе
        [few] { $rows } элементах
        [many] { $rows } элементах
       *[other] { $rows } элемента
    }.
   *[other] { $fields } поля-списка и поля-флажка становятся { $tags ->
        [one] { $tags } тегом
        [few] { $tags } тегами
        [many] { $tags } тегами
       *[other] { $tags } тегами
    } на { $rows ->
        [one] { $rows } элементе
        [few] { $rows } элементах
        [many] { $rows } элементах
       *[other] { $rows } элемента
    }.
}
import-scrivener-metadata-collections-became-tags-line = { $tags ->
    [one] { $tags } коллекция становится тегом на { $rows ->
        [one] { $rows } элементе
        [few] { $rows } элементах
        [many] { $rows } элементах
       *[other] { $rows } элемента
    }.
    [few] { $tags } коллекции становятся тегами на { $rows ->
        [one] { $rows } элементе
        [few] { $rows } элементах
        [many] { $rows } элементах
       *[other] { $rows } элемента
    }.
    [many] { $tags } коллекций становятся тегами на { $rows ->
        [one] { $rows } элементе
        [few] { $rows } элементах
        [many] { $rows } элементах
       *[other] { $rows } элемента
    }.
   *[other] { $tags } коллекции становятся тегами на { $rows ->
        [one] { $rows } элементе
        [few] { $rows } элементах
        [many] { $rows } элементах
       *[other] { $rows } элемента
    }.
}
import-scrivener-metadata-collection-order-lost-line = { $count ->
    [one] Тег не хранит порядок, поэтому порядок документов в { $count } коллекции не сохраняется.
    [few] Тег не хранит порядок, поэтому порядок документов в { $count } коллекциях не сохраняется.
    [many] Тег не хранит порядок, поэтому порядок документов в { $count } коллекциях не сохраняется.
   *[other] Тег не хранит порядок, поэтому порядок документов в { $count } коллекции не сохраняется.
}
import-scrivener-metadata-bookmarks-became-links-line = { $count ->
    [one] { $count } закладка становится ссылкой между двумя заметками или между сценой и записью библии истории: см. «Ссылки» и «Действующие лица».
    [few] { $count } закладки становятся ссылками между заметками или между сценами и записями библии истории: см. «Ссылки» и «Действующие лица».
    [many] { $count } закладок становятся ссылками между заметками или между сценами и записями библии истории: см. «Ссылки» и «Действующие лица».
   *[other] { $count } закладки становятся ссылками между заметками или между сценами и записями библии истории: см. «Ссылки» и «Действующие лица».
}
import-scrivener-metadata-goals-kept-line = { $count ->
    [1] Целевой объём, заданный в Scrivener для { $count } документа, становится его целью в Skribisto.
    [one] Целевые объёмы, заданные в Scrivener для { $count } документа, становятся их целями в Skribisto.
    [few] Целевые объёмы, заданные в Scrivener для { $count } документов, становятся их целями в Skribisto.
    [many] Целевые объёмы, заданные в Scrivener для { $count } документов, становятся их целями в Skribisto.
   *[other] Целевые объёмы, заданные в Scrivener для { $count } документа, становятся их целями в Skribisto.
}
import-scrivener-metadata-draft-target-kept-words-line = { $value ->
    [one] Целевой объём рукописи в { $value } слово становится целью книги.
    [few] Целевой объём рукописи в { $value } слова становится целью книги.
    [many] Целевой объём рукописи в { $value } слов становится целью книги.
   *[other] Целевой объём рукописи в { $value } слова становится целью книги.
}
import-scrivener-metadata-draft-target-kept-characters-line = { $value ->
    [one] Целевой объём рукописи в { $value } знак становится целью книги.
    [few] Целевой объём рукописи в { $value } знака становится целью книги.
    [many] Целевой объём рукописи в { $value } знаков становится целью книги.
   *[other] Целевой объём рукописи в { $value } знака становится целью книги.
}
import-scrivener-metadata-pace-planned-line = Срок, { $day }, задаёт Темп книги, который планирует слова на каждый рабочий день, выбранный в Scrivener, начиная со дня импорта.
import-scrivener-metadata-pace-planned-day-off-line = Срок, { $day }, задаёт Темп книги, который планирует слова на каждый рабочий день, выбранный в Scrivener, начиная со дня импорта. Как и в Scrivener, сам день срока — выходной.
import-scrivener-metadata-dictionary-words-line = { $count ->
    [one] { $count } слово из автодополнения попадает в личный словарь проекта, поэтому оно не помечается как ошибка.
    [few] { $count } слова из автодополнения попадают в личный словарь проекта, поэтому они не помечаются как ошибки.
    [many] { $count } слов из автодополнения попадают в личный словарь проекта, поэтому они не помечаются как ошибки.
   *[other] { $count } слова из автодополнения попадают в личный словарь проекта, поэтому они не помечаются как ошибки.
}
import-scrivener-metadata-listed-project-line = { $count ->
    [one] { $count } свойство проекта перечислено в заметке «{ $note }» в папке «{ $folder }».
    [few] { $count } свойства проекта перечислены в заметке «{ $note }» в папке «{ $folder }».
    [many] { $count } свойств проекта перечислены в заметке «{ $note }» в папке «{ $folder }».
   *[other] { $count } свойства проекта перечислены в заметке «{ $note }» в папке «{ $folder }».
}
import-scrivener-metadata-listed-project-bookmarks-line = { $count ->
    [one] { $count } закладка проекта перечислена в заметке «{ $note }» в папке «{ $folder }».
    [few] { $count } закладки проекта перечислены в заметке «{ $note }» в папке «{ $folder }».
    [many] { $count } закладок проекта перечислены в заметке «{ $note }» в папке «{ $folder }».
   *[other] { $count } закладки проекта перечислены в заметке «{ $note }» в папке «{ $folder }».
}
import-scrivener-metadata-listed-replacements-line = { $count ->
    [one] { $count } замена при компиляции перечислена в заметке «{ $note }» в папке «{ $folder }». Skribisto её не выполняет.
    [few] { $count } замены при компиляции перечислены в заметке «{ $note }» в папке «{ $folder }». Skribisto их не выполняет.
    [many] { $count } замен при компиляции перечислены в заметке «{ $note }» в папке «{ $folder }». Skribisto их не выполняет.
   *[other] { $count } замены при компиляции перечислены в заметке «{ $note }» в папке «{ $folder }». Skribisto их не выполняет.
}
import-scrivener-metadata-listed-auto-complete-line = { $count ->
    [one] { $count } запись автодополнения из нескольких слов перечислена в заметке «{ $note }» в папке «{ $folder }».
    [few] { $count } записи автодополнения из нескольких слов перечислены в заметке «{ $note }» в папке «{ $folder }».
    [many] { $count } записей автодополнения из нескольких слов перечислены в заметке «{ $note }» в папке «{ $folder }».
   *[other] { $count } записи автодополнения из нескольких слов перечислены в заметке «{ $note }» в папке «{ $folder }».
}
import-scrivener-metadata-listed-saved-searches-line = { $count ->
    [one] { $count } сохранённый поиск перечислен в заметке «{ $note }» в папке «{ $folder }».
    [few] { $count } сохранённых поиска перечислены в заметке «{ $note }» в папке «{ $folder }».
    [many] { $count } сохранённых поисков перечислены в заметке «{ $note }» в папке «{ $folder }».
   *[other] { $count } сохранённого поиска перечислены в заметке «{ $note }» в папке «{ $folder }».
}
import-scrivener-metadata-listed-custom-metadata-line = { $count ->
    [one] { $count } значение пользовательских метаданных, которое не может хранить тег, перечислено в заметке «{ $note }» в папке «{ $folder }».
    [few] { $count } значения пользовательских метаданных, которые не может хранить тег, перечислены в заметке «{ $note }» в папке «{ $folder }».
    [many] { $count } значений пользовательских метаданных, которые не может хранить тег, перечислены в заметке «{ $note }» в папке «{ $folder }».
   *[other] { $count } значения пользовательских метаданных, которые не может хранить тег, перечислены в заметке «{ $note }» в папке «{ $folder }».
}
import-scrivener-metadata-listed-targets-line = { $count ->
    [one] { $count } целевой объём документа перечислен в заметке «{ $note }» в папке «{ $folder }».
    [few] { $count } целевых объёма документов перечислены в заметке «{ $note }» в папке «{ $folder }».
    [many] { $count } целевых объёмов документов перечислены в заметке «{ $note }» в папке «{ $folder }».
   *[other] { $count } целевого объёма документов перечислены в заметке «{ $note }» в папке «{ $folder }».
}
import-scrivener-metadata-listed-bookmarks-line = { $count ->
    [one] { $count } закладка, которую не показывает ни одна ссылка Skribisto, перечислена в заметке «{ $note }» в папке «{ $folder }».
    [few] { $count } закладки, которые не показывает ни одна ссылка Skribisto, перечислены в заметке «{ $note }» в папке «{ $folder }».
    [many] { $count } закладок, которые не показывает ни одна ссылка Skribisto, перечислены в заметке «{ $note }» в папке «{ $folder }».
   *[other] { $count } закладки, которые не показывает ни одна ссылка Skribisto, перечислены в заметке «{ $note }» в папке «{ $folder }».
}
import-scrivener-metadata-targets-on-notes-line = { $count ->
    [one] { $count } целевой объём документа относится к заметке, текст которой не учитывает ни одна цель, поэтому он не становится целью.
    [few] { $count } целевых объёма документов относятся к заметкам, текст которых не учитывает ни одна цель, поэтому они не становятся целями.
    [many] { $count } целевых объёмов документов относятся к заметкам, текст которых не учитывает ни одна цель, поэтому они не становятся целями.
   *[other] { $count } целевого объёма документов относятся к заметкам, текст которых не учитывает ни одна цель, поэтому они не становятся целями.
}
import-scrivener-metadata-targets-on-split-documents-line = { $count ->
    [one] { $count } целевой объём документа относится к документу, разделённому по заголовкам, а его главы ни одна цель не учитывает вместе, поэтому он не становится целью.
    [few] { $count } целевых объёма документов относятся к документам, разделённым по заголовкам, а их главы ни одна цель не учитывает вместе, поэтому они не становятся целями.
    [many] { $count } целевых объёмов документов относятся к документам, разделённым по заголовкам, а их главы ни одна цель не учитывает вместе, поэтому они не становятся целями.
   *[other] { $count } целевого объёма документов относятся к документам, разделённым по заголовкам, а их главы ни одна цель не учитывает вместе, поэтому они не становятся целями.
}
import-scrivener-metadata-bookmark-ends-without-row-line = { $count ->
    [one] { $count } закладка ведёт к документу, которому нет места в новом проекте, например к шаблону документа. Она указана по тому, где этот документ был в Scrivener.
    [few] { $count } закладки ведут к документам, которым нет места в новом проекте, например к шаблонам документов. Они указаны по тому, где эти документы были в Scrivener.
    [many] { $count } закладок ведут к документам, которым нет места в новом проекте, например к шаблонам документов. Они указаны по тому, где эти документы были в Scrivener.
   *[other] { $count } закладки ведут к документам, которым нет места в новом проекте, например к шаблонам документов. Они указаны по тому, где эти документы были в Scrivener.
}
import-scrivener-metadata-unused-keywords-skipped-line = { $count ->
    [one] { $count } ключевое слово, которым не отмечен ни один документ, не импортируется.
    [few] { $count } ключевых слова, которыми не отмечен ни один документ, не импортируются.
    [many] { $count } ключевых слов, которыми не отмечен ни один документ, не импортируются.
   *[other] { $count } ключевого слова, которыми не отмечен ни один документ, не импортируются.
}
import-scrivener-metadata-list-value-invalid-line = { $count ->
    [one] { $count } значение поля-списка «{ $field }» не соответствует ни одному из его вариантов и не импортируется.
    [few] { $count } значения поля-списка «{ $field }» не соответствуют ни одному из его вариантов и не импортируются.
    [many] { $count } значений поля-списка «{ $field }» не соответствуют ни одному из его вариантов и не импортируются.
   *[other] { $count } значения поля-списка «{ $field }» не соответствуют ни одному из его вариантов и не импортируются.
}
import-scrivener-metadata-undefined-custom-field-line = { $count ->
    [one] { $count } значение относится к пользовательскому полю, которого в проекте больше нет и которое Scrivener тоже не показывает. Оно не импортируется.
    [few] { $count } значения относятся к пользовательским полям, которых в проекте больше нет и которые Scrivener тоже не показывает. Они не импортируются.
    [many] { $count } значений относятся к пользовательским полям, которых в проекте больше нет и которые Scrivener тоже не показывает. Они не импортируются.
   *[other] { $count } значения относятся к пользовательским полям, которых в проекте больше нет и которые Scrivener тоже не показывает. Они не импортируются.
}
import-scrivener-metadata-collection-members-missing-line = { $count ->
    [one] { $count } документ коллекции «{ $name }» не имеет места в новом проекте или исчез из проекта Scrivener.
    [few] { $count } документа коллекции «{ $name }» не имеют места в новом проекте или исчезли из проекта Scrivener.
    [many] { $count } документов коллекции «{ $name }» не имеют места в новом проекте или исчезли из проекта Scrivener.
   *[other] { $count } документа коллекции «{ $name }» не имеют места в новом проекте или исчезли из проекта Scrivener.
}
import-scrivener-metadata-empty-collections-skipped-line = { $count ->
    [one] { $count } коллекция, в которой нет ни одного документа, не импортируется.
    [few] { $count } коллекции, в которых нет ни одного документа, не импортируются.
    [many] { $count } коллекций, в которых нет ни одного документа, не импортируются.
   *[other] { $count } коллекции, в которых нет ни одного документа, не импортируются.
}
import-scrivener-metadata-bookmarks-unresolved-line = { $count ->
    [one] { $count } закладка ведёт к документу, которого в проекте Scrivener больше нет, и не импортируется.
    [few] { $count } закладки ведут к документам, которых в проекте Scrivener больше нет, и не импортируются.
    [many] { $count } закладок ведут к документам, которых в проекте Scrivener больше нет, и не импортируются.
   *[other] { $count } закладки ведут к документам, которых в проекте Scrivener больше нет, и не импортируются.
}
import-scrivener-metadata-targets-in-pages-line = { $count ->
    [one] { $count } целевой объём документа задан в страницах, а Skribisto страницы не считает, поэтому он не становится целью.
    [few] { $count } целевых объёма документов заданы в страницах, а Skribisto страницы не считает, поэтому они не становятся целями.
    [many] { $count } целевых объёмов документов заданы в страницах, а Skribisto страницы не считает, поэтому они не становятся целями.
   *[other] { $count } целевого объёма документов заданы в страницах, а Skribisto страницы не считает, поэтому они не становятся целями.
}
import-scrivener-metadata-targets-unreadable-line = { $count ->
    [one] { $count } целевой объём не импортирован: его значение не является числом, меньше нуля или больше, чем может вместить цель.
    [few] { $count } целевых объёма не импортированы: их значения не являются числами, меньше нуля или больше, чем может вместить цель.
    [many] { $count } целевых объёмов не импортированы: их значения не являются числами, меньше нуля или больше, чем может вместить цель.
   *[other] { $count } целевого объёма не импортированы: их значения не являются числами, меньше нуля или больше, чем может вместить цель.
}
import-scrivener-metadata-draft-target-in-pages-line = { $value ->
    [one] Целевой объём рукописи в { $value } страницу не становится целью: Skribisto считает слова или знаки. Он указан в заметке «{ $note }» в папке «{ $folder }».
    [few] Целевой объём рукописи в { $value } страницы не становится целью: Skribisto считает слова или знаки. Он указан в заметке «{ $note }» в папке «{ $folder }».
    [many] Целевой объём рукописи в { $value } страниц не становится целью: Skribisto считает слова или знаки. Он указан в заметке «{ $note }» в папке «{ $folder }».
   *[other] Целевой объём рукописи в { $value } страницы не становится целью: Skribisto считает слова или знаки. Он указан в заметке «{ $note }» в папке «{ $folder }».
}
import-scrivener-metadata-draft-target-no-book-words-line = { $value ->
    [one] Целевой объём рукописи в { $value } слово не становится целью, потому что рукопись образует несколько книг или ни одной. Он указан в заметке «{ $note }» в папке «{ $folder }».
    [few] Целевой объём рукописи в { $value } слова не становится целью, потому что рукопись образует несколько книг или ни одной. Он указан в заметке «{ $note }» в папке «{ $folder }».
    [many] Целевой объём рукописи в { $value } слов не становится целью, потому что рукопись образует несколько книг или ни одной. Он указан в заметке «{ $note }» в папке «{ $folder }».
   *[other] Целевой объём рукописи в { $value } слова не становится целью, потому что рукопись образует несколько книг или ни одной. Он указан в заметке «{ $note }» в папке «{ $folder }».
}
import-scrivener-metadata-draft-target-no-book-characters-line = { $value ->
    [one] Целевой объём рукописи в { $value } знак не становится целью, потому что рукопись образует несколько книг или ни одной. Он указан в заметке «{ $note }» в папке «{ $folder }».
    [few] Целевой объём рукописи в { $value } знака не становится целью, потому что рукопись образует несколько книг или ни одной. Он указан в заметке «{ $note }» в папке «{ $folder }».
    [many] Целевой объём рукописи в { $value } знаков не становится целью, потому что рукопись образует несколько книг или ни одной. Он указан в заметке «{ $note }» в папке «{ $folder }».
   *[other] Целевой объём рукописи в { $value } знака не становится целью, потому что рукопись образует несколько книг или ни одной. Он указан в заметке «{ $note }» в папке «{ $folder }».
}
import-scrivener-metadata-deadline-passed-line = Срок, { $day }, уже прошёл, поэтому он не задаёт Темп книги. Срок указан в заметке «{ $note }» в папке «{ $folder }».
import-scrivener-metadata-deadline-no-writing-day-line = Между днём импорта и сроком, { $day }, не осталось ни одного рабочего дня, поэтому срок не задаёт Темп книги. Он указан в заметке «{ $note }» в папке «{ $folder }».
import-scrivener-metadata-deadline-template-line = Срок, { $day }, был сохранён в шаблоне, поэтому он не задаёт Темп книги. Срок указан в заметке «{ $note }» в папке «{ $folder }».
import-scrivener-metadata-deadline-no-word-goal-line = Срок, { $day }, не задаёт Темп книги, потому что у рукописи нет целевого объёма в словах, по которому можно было бы планировать. Срок указан в заметке «{ $note }» в папке «{ $folder }».
import-scrivener-metadata-deadline-character-goal-line = Срок, { $day }, не задаёт Темп книги, потому что целевой объём рукописи задан в знаках, а Темп считает слова. Срок указан в заметке «{ $note }» в папке «{ $folder }».
import-scrivener-metadata-deadline-no-book-line = Срок, { $day }, не задаёт Темп ни одной книги, потому что рукопись образует несколько книг или ни одной. Срок указан в заметке «{ $note }» в папке «{ $folder }».
import-scrivener-metadata-deadline-unreadable-line = Срок не удаётся прочитать, поэтому он не задаёт Темп книги. Он указан как есть в заметке «{ $note }» в папке «{ $folder }».
import-scrivener-metadata-session-target-words-line = { $value ->
    [one] Целевой объём сеанса в { $value } слово не импортируется: в Skribisto цель сеанса письма — одна настройка для всех проектов. Он указан в заметке «{ $note }» в папке «{ $folder }».
    [few] Целевой объём сеанса в { $value } слова не импортируется: в Skribisto цель сеанса письма — одна настройка для всех проектов. Он указан в заметке «{ $note }» в папке «{ $folder }».
    [many] Целевой объём сеанса в { $value } слов не импортируется: в Skribisto цель сеанса письма — одна настройка для всех проектов. Он указан в заметке «{ $note }» в папке «{ $folder }».
   *[other] Целевой объём сеанса в { $value } слова не импортируется: в Skribisto цель сеанса письма — одна настройка для всех проектов. Он указан в заметке «{ $note }» в папке «{ $folder }».
}
import-scrivener-metadata-session-target-characters-line = { $value ->
    [one] Целевой объём сеанса в { $value } знак не импортируется: в Skribisto цель сеанса письма — одна настройка для всех проектов, в словах. Он указан в заметке «{ $note }» в папке «{ $folder }».
    [few] Целевой объём сеанса в { $value } знака не импортируется: в Skribisto цель сеанса письма — одна настройка для всех проектов, в словах. Он указан в заметке «{ $note }» в папке «{ $folder }».
    [many] Целевой объём сеанса в { $value } знаков не импортируется: в Skribisto цель сеанса письма — одна настройка для всех проектов, в словах. Он указан в заметке «{ $note }» в папке «{ $folder }».
   *[other] Целевой объём сеанса в { $value } знака не импортируется: в Skribisto цель сеанса письма — одна настройка для всех проектов, в словах. Он указан в заметке «{ $note }» в папке «{ $folder }».
}
import-scrivener-metadata-session-target-pages-line = { $value ->
    [one] Целевой объём сеанса в { $value } страницу не импортируется: в Skribisto цель сеанса письма — одна настройка для всех проектов, в словах. Он указан в заметке «{ $note }» в папке «{ $folder }».
    [few] Целевой объём сеанса в { $value } страницы не импортируется: в Skribisto цель сеанса письма — одна настройка для всех проектов, в словах. Он указан в заметке «{ $note }» в папке «{ $folder }».
    [many] Целевой объём сеанса в { $value } страниц не импортируется: в Skribisto цель сеанса письма — одна настройка для всех проектов, в словах. Он указан в заметке «{ $note }» в папке «{ $folder }».
   *[other] Целевой объём сеанса в { $value } страницы не импортируется: в Skribisto цель сеанса письма — одна настройка для всех проектов, в словах. Он указан в заметке «{ $note }» в папке «{ $folder }».
}
import-scrivener-metadata-session-target-calculated-words-line = { $value ->
    [one] Целевой объём сеанса в { $value } слово, рассчитанный Scrivener по сроку и рабочим дням, не импортируется: в Skribisto цель сеанса письма — одна настройка для всех проектов. Он указан в заметке «{ $note }» в папке «{ $folder }».
    [few] Целевой объём сеанса в { $value } слова, рассчитанный Scrivener по сроку и рабочим дням, не импортируется: в Skribisto цель сеанса письма — одна настройка для всех проектов. Он указан в заметке «{ $note }» в папке «{ $folder }».
    [many] Целевой объём сеанса в { $value } слов, рассчитанный Scrivener по сроку и рабочим дням, не импортируется: в Skribisto цель сеанса письма — одна настройка для всех проектов. Он указан в заметке «{ $note }» в папке «{ $folder }».
   *[other] Целевой объём сеанса в { $value } слова, рассчитанный Scrivener по сроку и рабочим дням, не импортируется: в Skribisto цель сеанса письма — одна настройка для всех проектов. Он указан в заметке «{ $note }» в папке «{ $folder }».
}
import-scrivener-metadata-session-target-calculated-characters-line = { $value ->
    [one] Целевой объём сеанса в { $value } знак, рассчитанный Scrivener по сроку и рабочим дням, не импортируется: в Skribisto цель сеанса письма — одна настройка для всех проектов, в словах. Он указан в заметке «{ $note }» в папке «{ $folder }».
    [few] Целевой объём сеанса в { $value } знака, рассчитанный Scrivener по сроку и рабочим дням, не импортируется: в Skribisto цель сеанса письма — одна настройка для всех проектов, в словах. Он указан в заметке «{ $note }» в папке «{ $folder }».
    [many] Целевой объём сеанса в { $value } знаков, рассчитанный Scrivener по сроку и рабочим дням, не импортируется: в Skribisto цель сеанса письма — одна настройка для всех проектов, в словах. Он указан в заметке «{ $note }» в папке «{ $folder }».
   *[other] Целевой объём сеанса в { $value } знака, рассчитанный Scrivener по сроку и рабочим дням, не импортируется: в Skribisto цель сеанса письма — одна настройка для всех проектов, в словах. Он указан в заметке «{ $note }» в папке «{ $folder }».
}
import-scrivener-metadata-session-target-calculated-pages-line = { $value ->
    [one] Целевой объём сеанса в { $value } страницу, рассчитанный Scrivener по сроку и рабочим дням, не импортируется: в Skribisto цель сеанса письма — одна настройка для всех проектов, в словах. Он указан в заметке «{ $note }» в папке «{ $folder }».
    [few] Целевой объём сеанса в { $value } страницы, рассчитанный Scrivener по сроку и рабочим дням, не импортируется: в Skribisto цель сеанса письма — одна настройка для всех проектов, в словах. Он указан в заметке «{ $note }» в папке «{ $folder }».
    [many] Целевой объём сеанса в { $value } страниц, рассчитанный Scrivener по сроку и рабочим дням, не импортируется: в Skribisto цель сеанса письма — одна настройка для всех проектов, в словах. Он указан в заметке «{ $note }» в папке «{ $folder }».
   *[other] Целевой объём сеанса в { $value } страницы, рассчитанный Scrivener по сроку и рабочим дням, не импортируется: в Skribisto цель сеанса письма — одна настройка для всех проектов, в словах. Он указан в заметке «{ $note }» в папке «{ $folder }».
}
import-scrivener-metadata-icons-dropped-line = { $count ->
    [one] У { $count } элемента был значок в подшивке; он не переносится: Skribisto показывает значок каждого элемента по его типу.
    [few] У { $count } элементов были значки в подшивке; они не переносятся: Skribisto показывает значок каждого элемента по его типу.
    [many] У { $count } элементов были значки в подшивке; они не переносятся: Skribisto показывает значок каждого элемента по его типу.
   *[other] У { $count } элемента были значки в подшивке; они не переносятся: Skribisto показывает значок каждого элемента по его типу.
}

## What converting the prose did, as the import's report says it. `$count` is how many texts,
## or how many of a thing, as each line says; `$example` quotes the project.
import-scrivener-prose-unreadable-line = { $count ->
    [one] { $count } текст не удалось прочитать. Он переносится пустым.
    [few] { $count } текста не удалось прочитать. Они переносятся пустыми.
    [many] { $count } текстов не удалось прочитать. Они переносятся пустыми.
   *[other] { $count } текста не удалось прочитать. Они переносятся пустыми.
}
import-scrivener-prose-comments-unreadable-line = { $count ->
    [one] Комментарии и связанные сноски { $count } текста не удалось прочитать. Слова, которые они отмечали, сохраняются.
    [few] Комментарии и связанные сноски { $count } текстов не удалось прочитать. Слова, которые они отмечали, сохраняются.
    [many] Комментарии и связанные сноски { $count } текстов не удалось прочитать. Слова, которые они отмечали, сохраняются.
   *[other] Комментарии и связанные сноски { $count } текста не удалось прочитать. Слова, которые они отмечали, сохраняются.
}
import-scrivener-prose-read-as-plain-text-line = { $count ->
    [one] { $count } текст хранился как простой текст, а не как форматированный, и читается именно так.
    [few] { $count } текста хранились как простой текст, а не как форматированный, и читаются именно так.
    [many] { $count } текстов хранились как простой текст, а не как форматированный, и читаются именно так.
   *[other] { $count } текста хранились как простой текст, а не как форматированный, и читаются именно так.
}
import-scrivener-prose-inline-note-unmatched-line = { $count ->
    [one] { $count } маркер встроенного примечания не имеет пары и сохраняется как текст.
    [few] { $count } маркера встроенных примечаний не имеют пары и сохраняются как текст.
    [many] { $count } маркеров встроенных примечаний не имеют пары и сохраняются как текст.
   *[other] { $count } маркера встроенных примечаний не имеют пары и сохраняются как текст.
}
import-scrivener-prose-style-unresolved-line = { $count ->
    [one] { $count } маркер стиля ссылается на стиль, которого нет в проекте. Он удаляется, а текст сохраняет своё форматирование.
    [few] { $count } маркера стилей ссылаются на стиль, которого нет в проекте. Они удаляются, а текст сохраняет своё форматирование.
    [many] { $count } маркеров стилей ссылаются на стиль, которого нет в проекте. Они удаляются, а текст сохраняет своё форматирование.
   *[other] { $count } маркера стилей ссылаются на стиль, которого нет в проекте. Они удаляются, а текст сохраняет своё форматирование.
}
import-scrivener-prose-styles-lost-in-export-line = { $count ->
    [one] { $count } абзац, возможно, был заголовком, который потерял свой стиль, когда Scrivener 3 экспортировал проект в свой старый формат, и переносится как обычный абзац. Проверьте его или импортируйте проект, из которого был сделан этот экспорт.
    [few] { $count } абзаца, возможно, были заголовками, которые потеряли свой стиль, когда Scrivener 3 экспортировал проект в свой старый формат, и переносятся как обычные абзацы. Проверьте их или импортируйте проект, из которого был сделан этот экспорт.
    [many] { $count } абзацев, возможно, были заголовками, которые потеряли свой стиль, когда Scrivener 3 экспортировал проект в свой старый формат, и переносятся как обычные абзацы. Проверьте их или импортируйте проект, из которого был сделан этот экспорт.
   *[other] { $count } абзаца, возможно, были заголовками, которые потеряли свой стиль, когда Scrivener 3 экспортировал проект в свой старый формат, и переносятся как обычные абзацы. Проверьте их или импортируйте проект, из которого был сделан этот экспорт.
}
import-scrivener-prose-comment-anchor-without-entry-line = { $count ->
    [one] У { $count } комментария или сноски содержимое пропало ещё в Scrivener. Отмеченные слова сохраняются, но без комментария или сноски.
    [few] У { $count } комментариев или сносок содержимое пропало ещё в Scrivener. Отмеченные слова сохраняются, но без самих комментариев и сносок.
    [many] У { $count } комментариев или сносок содержимое пропало ещё в Scrivener. Отмеченные слова сохраняются, но без самих комментариев и сносок.
   *[other] У { $count } комментария или сноски содержимое пропало ещё в Scrivener. Отмеченные слова сохраняются, но без самих комментариев и сносок.
}
import-scrivener-prose-comment-entry-without-anchor-line = { $count ->
    [one] { $count } комментарию не нашлось места в его тексте, и он прикреплён к первому абзацу.
    [few] { $count } комментариям не нашлось места в их тексте, и они прикреплены к первому абзацу.
    [many] { $count } комментариям не нашлось места в их тексте, и они прикреплены к первому абзацу.
   *[other] { $count } комментария не нашлось места в их тексте, и они прикреплены к первому абзацу.
}
import-scrivener-prose-footnote-entry-without-anchor-line = { $count ->
    [1] { $count } связанной сноске не нашлось места в её тексте, поэтому она приводится в конце этого текста.
    [one] { $count } связанной сноске не нашлось места в их текстах, поэтому каждая приводится в конце своего текста.
    [few] { $count } связанным сноскам не нашлось места в их текстах, поэтому каждая приводится в конце своего текста.
    [many] { $count } связанным сноскам не нашлось места в их текстах, поэтому каждая приводится в конце своего текста.
   *[other] { $count } связанной сноски не нашлось места в их текстах, поэтому каждая приводится в конце своего текста.
}
import-scrivener-prose-range-comment-reading-line = { $count ->
    [one] { $count } комментарий из более старой версии Scrivener размещён по другому прочтению его позиции. Проверьте, что он стоит на нужных словах.
    [few] { $count } комментария из более старой версии Scrivener размещены по другому прочтению их позиции. Проверьте, что они стоят на нужных словах.
    [many] { $count } комментариев из более старой версии Scrivener размещены по другому прочтению их позиции. Проверьте, что они стоят на нужных словах.
   *[other] { $count } комментария из более старой версии Scrivener размещены по другому прочтению их позиции. Проверьте, что они стоят на нужных словах.
}
import-scrivener-prose-range-comment-reading-markers-counted-line = { $count ->
    [one] { $count } комментарий из более старой версии Scrivener размещён с учётом маркеров его встроенных примечаний. Проверьте, что он стоит на нужных словах.
    [few] { $count } комментария из более старой версии Scrivener размещены с учётом маркеров их встроенных примечаний. Проверьте, что они стоят на нужных словах.
    [many] { $count } комментариев из более старой версии Scrivener размещены с учётом маркеров их встроенных примечаний. Проверьте, что они стоят на нужных словах.
   *[other] { $count } комментария из более старой версии Scrivener размещены с учётом маркеров их встроенных примечаний. Проверьте, что они стоят на нужных словах.
}
import-scrivener-prose-range-comment-reading-pictures-not-counted-line = { $count ->
    [one] { $count } комментарий из более старой версии Scrivener размещён без учёта его изображений. Проверьте, что он стоит на нужных словах.
    [few] { $count } комментария из более старой версии Scrivener размещены без учёта их изображений. Проверьте, что они стоят на нужных словах.
    [many] { $count } комментариев из более старой версии Scrivener размещены без учёта их изображений. Проверьте, что они стоят на нужных словах.
   *[other] { $count } комментария из более старой версии Scrivener размещены без учёта их изображений. Проверьте, что они стоят на нужных словах.
}
import-scrivener-prose-range-comment-approximate-line = { $count ->
    [one] { $count } комментарий из более старой версии Scrivener не совпадает точно ни с одним прочтением своей позиции и может быть смещён на несколько символов.
    [few] { $count } комментария из более старой версии Scrivener не совпадают точно ни с одним прочтением своей позиции и могут быть смещены на несколько символов.
    [many] { $count } комментариев из более старой версии Scrivener не совпадают точно ни с одним прочтением своей позиции и могут быть смещены на несколько символов.
   *[other] { $count } комментария из более старой версии Scrivener не совпадают точно ни с одним прочтением своей позиции и могут быть смещены на несколько символов.
}
import-scrivener-prose-document-link-unresolved-line = { $count ->
    [one] { $count } ссылка ведёт на документ, который импорт не перенёс. Её слова остаются.
    [few] { $count } ссылки ведут на документы, которые импорт не перенёс. Их слова остаются.
    [many] { $count } ссылок ведут на документы, которые импорт не перенёс. Их слова остаются.
   *[other] { $count } ссылки ведут на документы, которые импорт не перенёс. Их слова остаются.
}
import-scrivener-prose-linked-picture-not-read-line = { $count ->
    [1] { $count } изображение, связанное с файлом на вашем диске, «{ $example }», не перенесено.
    [one] { $count } изображение, связанное с файлами на вашем диске, например «{ $example }», не перенесено.
    [few] { $count } изображения, связанные с файлами на вашем диске, например «{ $example }», не перенесены.
    [many] { $count } изображений, связанных с файлами на вашем диске, например «{ $example }», не перенесены.
   *[other] { $count } изображения, связанные с файлами на вашем диске, например «{ $example }», не перенесены.
}
import-scrivener-prose-heading-repeats-title-line = { $count ->
    [one] { $count } текст начинается с заголовка, повторяющего его название, поэтому при экспорте название может быть напечатано дважды.
    [few] { $count } текста начинаются с заголовка, повторяющего их название, поэтому при экспорте название может быть напечатано дважды.
    [many] { $count } текстов начинаются с заголовка, повторяющего их название, поэтому при экспорте название может быть напечатано дважды.
   *[other] { $count } текста начинаются с заголовка, повторяющего их название, поэтому при экспорте название может быть напечатано дважды.
}
import-scrivener-prose-markdown-as-text-line = { $count ->
    [one] { $count } абзац, написанный в Markdown, сохраняется как есть, в виде текста.
    [few] { $count } абзаца, написанные в Markdown, сохраняются как есть, в виде текста.
    [many] { $count } абзацев, написанных в Markdown, сохраняются как есть, в виде текста.
   *[other] { $count } абзаца, написанные в Markdown, сохраняются как есть, в виде текста.
}
import-scrivener-prose-citation-keys-as-text-line = { $count ->
    [one] В { $count } абзаце есть ключи цитирования для программы управления библиографией; они сохраняются как текст.
    [few] В { $count } абзацах есть ключи цитирования для программы управления библиографией; они сохраняются как текст.
    [many] В { $count } абзацах есть ключи цитирования для программы управления библиографией; они сохраняются как текст.
   *[other] В { $count } абзаца есть ключи цитирования для программы управления библиографией; они сохраняются как текст.
}
import-scrivener-prose-placeholders-in-exported-text-line = { $count ->
    [1] { $count } заполнитель Scrivener, { $example }, сохраняется простым текстом в том, что вы экспортируете. Замените его вручную.
    [one] { $count } заполнитель Scrivener, например { $example }, сохраняется простым текстом в том, что вы экспортируете. Замените их вручную.
    [few] { $count } заполнителя Scrivener, например { $example }, сохраняются простым текстом в том, что вы экспортируете. Замените их вручную.
    [many] { $count } заполнителей Scrivener, например { $example }, сохраняются простым текстом в том, что вы экспортируете. Замените их вручную.
   *[other] { $count } заполнителя Scrivener, например { $example }, сохраняются простым текстом в том, что вы экспортируете. Замените их вручную.
}
import-scrivener-prose-placeholders-in-other-text-line = { $count ->
    [1] { $count } заполнитель Scrivener, { $example }, сохраняется простым текстом в тексте, который не экспортируется.
    [one] { $count } заполнитель Scrivener, например { $example }, сохраняется простым текстом в текстах, которые не экспортируются.
    [few] { $count } заполнителя Scrivener, например { $example }, сохраняются простым текстом в текстах, которые не экспортируются.
    [many] { $count } заполнителей Scrivener, например { $example }, сохраняются простым текстом в текстах, которые не экспортируются.
   *[other] { $count } заполнителя Scrivener, например { $example }, сохраняются простым текстом в текстах, которые не экспортируются.
}
import-scrivener-prose-ornament-lines-kept-line = { $count ->
    [one] { $count } строка из декоративных знаков по центру может обозначать разрыв сцены и сохраняется в том виде, в каком написана.
    [few] { $count } строки из декоративных знаков по центру могут обозначать разрывы сцен и сохраняются в том виде, в каком написаны.
    [many] { $count } строк из декоративных знаков по центру могут обозначать разрывы сцен и сохраняются в том виде, в каком написаны.
   *[other] { $count } строки из декоративных знаков по центру могут обозначать разрывы сцен и сохраняются в том виде, в каком написаны.
}
import-scrivener-prose-long-document-line = { $count ->
    [one] { $count } текст очень длинный и может медленно открываться.
    [few] { $count } текста очень длинные и могут медленно открываться.
    [many] { $count } текстов очень длинные и могут медленно открываться.
   *[other] { $count } текста очень длинные и могут медленно открываться.
}
import-scrivener-prose-pictures-without-description-line = { $count ->
    [one] У { $count } изображения нет описания для читателей, которые его не видят. Описание можно добавить в редакторе.
    [few] У { $count } изображений нет описания для читателей, которые их не видят. Описание можно добавить в редакторе.
    [many] У { $count } изображений нет описания для читателей, которые их не видят. Описание можно добавить в редакторе.
   *[other] У { $count } изображения нет описания для читателей, которые их не видят. Описание можно добавить в редакторе.
}
import-scrivener-prose-struck-text-compiled-away-line = { $count ->
    [one] Зачёркнутый текст в { $count } тексте сохраняется. Компиляция Scrivener убирала его из книги, но экспорт из Skribisto его печатает.
    [few] Зачёркнутый текст в { $count } текстах сохраняется. Компиляция Scrivener убирала его из книги, но экспорт из Skribisto его печатает.
    [many] Зачёркнутый текст в { $count } текстах сохраняется. Компиляция Scrivener убирала его из книги, но экспорт из Skribisto его печатает.
   *[other] Зачёркнутый текст в { $count } текста сохраняется. Компиляция Scrivener убирала его из книги, но экспорт из Skribisto его печатает.
}
import-scrivener-prose-compile-deleted-style-printed-line = { $count ->
    [1] Текст в стиле «{ $example }», который компиляция Scrivener удаляла, сохраняется как проза, и экспорт из Skribisto его печатает.
    [one] Текст в { $count } стиле, который компиляция Scrivener удаляла, например «{ $example }», сохраняется как проза, и экспорт из Skribisto его печатает.
    [few] Текст в { $count } стилях, которые компиляция Scrivener удаляла, например «{ $example }», сохраняется как проза, и экспорт из Skribisto его печатает.
    [many] Текст в { $count } стилях, которые компиляция Scrivener удаляла, например «{ $example }», сохраняется как проза, и экспорт из Skribisto его печатает.
   *[other] Текст в { $count } стиля, которые компиляция Scrivener удаляла, например «{ $example }», сохраняется как проза, и экспорт из Skribisto его печатает.
}
import-scrivener-prose-compile-raw-style-printed-line = { $count ->
    [1] Текст в стиле «{ $example }», который компиляция Scrivener передавала как код для другой программы, сохраняется как проза, и экспорт из Skribisto его печатает.
    [one] Текст в { $count } стиле, который компиляция Scrivener передавала как код для другой программы, например «{ $example }», сохраняется как проза, и экспорт из Skribisto его печатает.
    [few] Текст в { $count } стилях, которые компиляция Scrivener передавала как код для другой программы, например «{ $example }», сохраняется как проза, и экспорт из Skribisto его печатает.
    [many] Текст в { $count } стилях, которые компиляция Scrivener передавала как код для другой программы, например «{ $example }», сохраняется как проза, и экспорт из Skribisto его печатает.
   *[other] Текст в { $count } стиля, которые компиляция Scrivener передавала как код для другой программы, например «{ $example }», сохраняется как проза, и экспорт из Skribisto его печатает.
}
import-scrivener-prose-left-out-after-document-line = { $count ->
    [one] { $count } фрагмент повреждённого файла, стоящий после его конца, не переносится.
    [few] { $count } фрагмента повреждённых файлов, стоящие после их конца, не переносятся.
    [many] { $count } фрагментов повреждённых файлов, стоящих после их конца, не переносятся.
   *[other] { $count } фрагмента повреждённых файлов, стоящие после их конца, не переносятся.
}
import-scrivener-prose-left-out-header-or-footer-line = { $count ->
    [one] { $count } колонтитул не переносится: он не является частью текста.
    [few] { $count } колонтитула не переносятся: они не являются частью текста.
    [many] { $count } колонтитулов не переносятся: они не являются частью текста.
   *[other] { $count } колонтитула не переносятся: они не являются частью текста.
}
import-scrivener-prose-left-out-list-label-picture-line = { $count ->
    [one] { $count } изображение, использованное как маркер списка, не переносится.
    [few] { $count } изображения, использованные как маркеры списков, не переносятся.
    [many] { $count } изображений, использованных как маркеры списков, не переносятся.
   *[other] { $count } изображения, использованные как маркеры списков, не переносятся.
}
import-scrivener-prose-left-out-line = { $count ->
    [one] { $count } фрагмент файла, не входящий ни в один текст, не переносится.
    [few] { $count } фрагмента файлов, не входящие ни в один текст, не переносятся.
    [many] { $count } фрагментов файлов, не входящих ни в один текст, не переносятся.
   *[other] { $count } фрагмента файлов, не входящие ни в один текст, не переносятся.
}
import-scrivener-prose-undecodable-characters-line = { $count ->
    [one] { $count } символ не удалось декодировать, и на его месте стоит знак замены (�).
    [few] { $count } символа не удалось декодировать, и на их месте стоят знаки замены (�).
    [many] { $count } символов не удалось декодировать, и на их месте стоят знаки замены (�).
   *[other] { $count } символа не удалось декодировать, и на их месте стоят знаки замены (�).
}
import-scrivener-prose-inline-notes-merged-line = { $count ->
    [one] { $count } встроенное примечание, которое Scrivener разбил там, где менялось форматирование, снова собрано воедино.
    [few] { $count } встроенных примечания, которые Scrivener разбил там, где менялось форматирование, снова собраны воедино.
    [many] { $count } встроенных примечаний, которые Scrivener разбил там, где менялось форматирование, снова собраны воедино.
   *[other] { $count } встроенного примечания, которые Scrivener разбил там, где менялось форматирование, снова собраны воедино.
}
import-scrivener-prose-inline-notes-joined-across-paragraphs-line = { $count ->
    [1] { $count } встроенное примечание, записанное по одному абзацу, объединено в одно примечание.
    [one] { $count } встроенное примечание, записанное по одному абзацу, объединено в примечания из нескольких абзацев.
    [few] { $count } встроенных примечания, записанные по одному абзацу, объединены в примечания из нескольких абзацев.
    [many] { $count } встроенных примечаний, записанных по одному абзацу, объединены в примечания из нескольких абзацев.
   *[other] { $count } встроенного примечания, записанные по одному абзацу, объединены в примечания из нескольких абзацев.
}
import-scrivener-prose-blank-notes-dropped-line = { $count ->
    [one] { $count } пустое встроенное примечание не переносится.
    [few] { $count } пустых встроенных примечания не переносятся.
    [many] { $count } пустых встроенных примечаний не переносятся.
   *[other] { $count } пустого встроенного примечания не переносятся.
}
import-scrivener-prose-annotations-became-comments-line = { $count ->
    [one] { $count } встроенная аннотация стала комментарием.
    [few] { $count } встроенные аннотации стали комментариями.
    [many] { $count } встроенных аннотаций стали комментариями.
   *[other] { $count } встроенной аннотации стали комментариями.
}
import-scrivener-prose-annotation-only-document-kept-line = { $count ->
    [one] В { $count } документе не было ничего, кроме аннотации. Её слова сохраняются как проза.
    [few] В { $count } документах не было ничего, кроме аннотации. Их слова сохраняются как проза.
    [many] В { $count } документах не было ничего, кроме аннотации. Их слова сохраняются как проза.
   *[other] В { $count } документа не было ничего, кроме аннотации. Их слова сохраняются как проза.
}
import-scrivener-prose-preserve-formatting-passages-line = { $count ->
    [one] { $count } фрагмент с пометкой «Preserve Formatting» (сохранить форматирование) переносится как обычный текст.
    [few] { $count } фрагмента с пометкой «Preserve Formatting» (сохранить форматирование) переносятся как обычный текст.
    [many] { $count } фрагментов с пометкой «Preserve Formatting» (сохранить форматирование) переносятся как обычный текст.
   *[other] { $count } фрагмента с пометкой «Preserve Formatting» (сохранить форматирование) переносятся как обычный текст.
}
import-scrivener-prose-nested-note-unwrapped-line = { $count ->
    [one] { $count } примечание внутри комментария или сноски сохраняется как текст на своём месте.
    [few] { $count } примечания внутри комментариев или сносок сохраняются как текст на своём месте.
    [many] { $count } примечаний внутри комментариев или сносок сохраняются как текст на своём месте.
   *[other] { $count } примечания внутри комментариев или сносок сохраняются как текст на своём месте.
}
import-scrivener-prose-style-not-carried-line = { $count ->
    [1] У стиля «{ $example }» нет аналога в Skribisto. Слова, оформленные этим стилем, сохраняются.
    [one] У { $count } стиля, например «{ $example }», нет аналога в Skribisto. Слова, оформленные этими стилями, сохраняются.
    [few] У { $count } стилей, например «{ $example }», нет аналога в Skribisto. Слова, оформленные этими стилями, сохраняются.
    [many] У { $count } стилей, например «{ $example }», нет аналога в Skribisto. Слова, оформленные этими стилями, сохраняются.
   *[other] У { $count } стиля, например «{ $example }», нет аналога в Skribisto. Слова, оформленные этими стилями, сохраняются.
}
import-scrivener-prose-document-links-dropped-line = { $count ->
    [one] { $count } ссылке на другой документ нет места в Skribisto. Её слова остаются.
    [few] { $count } ссылкам на другие документы нет места в Skribisto. Их слова остаются.
    [many] { $count } ссылкам на другие документы нет места в Skribisto. Их слова остаются.
   *[other] { $count } ссылки на другие документы нет места в Skribisto. Их слова остаются.
}
import-scrivener-prose-document-links-kept-line = { $count ->
    [one] { $count } ссылка на другой документ сохраняется как связь между элементами.
    [few] { $count } ссылки на другие документы сохраняются как связи между элементами.
    [many] { $count } ссылок на другие документы сохраняются как связи между элементами.
   *[other] { $count } ссылки на другие документы сохраняются как связи между элементами.
}
import-scrivener-prose-other-links-kept-as-text-line = { $count ->
    [one] { $count } ссылка на другой проект или на файл сохраняется как текст.
    [few] { $count } ссылки на другие проекты или на файлы сохраняются как текст.
    [many] { $count } ссылок на другие проекты или на файлы сохраняются как текст.
   *[other] { $count } ссылки на другие проекты или на файлы сохраняются как текст.
}
import-scrivener-prose-typed-indents-removed-line = { $count ->
    [one] { $count } отступ, набранный в начале абзаца, удалён.
    [few] { $count } отступа, набранные в начале абзацев, удалены.
    [many] { $count } отступов, набранных в начале абзацев, удалены.
   *[other] { $count } отступа, набранные в начале абзацев, удалены.
}
import-scrivener-prose-left-indents-not-kept-line = { $count ->
    [one] { $count } абзац с отступом от поля теряет отступ.
    [few] { $count } абзаца с отступом от поля теряют отступ.
    [many] { $count } абзацев с отступом от поля теряют отступ.
   *[other] { $count } абзаца с отступом от поля теряют отступ.
}
import-scrivener-prose-blank-lines-removed-line = { $count ->
    [one] { $count } пустой абзац между двумя другими удалён.
    [few] { $count } пустых абзаца между другими абзацами удалены.
    [many] { $count } пустых абзацев между другими абзацами удалены.
   *[other] { $count } пустого абзаца между другими абзацами удалены.
}
import-scrivener-prose-blank-lines-became-scene-breaks-line = { $count ->
    [one] { $count } пустая строка между абзацами стала разрывом сцены.
    [few] { $count } пустые строки между абзацами стали разрывами сцен.
    [many] { $count } пустых строк между абзацами стали разрывами сцен.
   *[other] { $count } пустой строки между абзацами стали разрывами сцен.
}
import-scrivener-prose-ornaments-became-scene-breaks-line = { $count ->
    [one] { $count } строка из декоративных знаков стала разрывом сцены.
    [few] { $count } строки из декоративных знаков стали разрывами сцен.
    [many] { $count } строк из декоративных знаков стали разрывами сцен.
   *[other] { $count } строки из декоративных знаков стали разрывами сцен.
}
import-scrivener-prose-table-first-row-header-line = { $count ->
    [one] { $count } таблица теперь начинается со строки заголовка.
    [few] { $count } таблицы теперь начинаются со строки заголовка.
    [many] { $count } таблиц теперь начинаются со строки заголовка.
   *[other] { $count } таблицы теперь начинаются со строки заголовка.
}
import-scrivener-prose-table-cells-joined-line = { $count ->
    [one] В { $count } ячейке таблицы было несколько абзацев, теперь они объединены в один.
    [few] В { $count } ячейках таблиц было несколько абзацев, теперь они объединены в один.
    [many] В { $count } ячейках таблиц было несколько абзацев, теперь они объединены в один.
   *[other] В { $count } ячейки таблиц было несколько абзацев, теперь они объединены в один.
}
import-scrivener-prose-formatting-lost-colour-line = { $count ->
    [one] Цвет текста удалён в { $count } тексте.
    [few] Цвет текста удалён в { $count } текстах.
    [many] Цвет текста удалён в { $count } текстах.
   *[other] Цвет текста удалён в { $count } текста.
}
import-scrivener-prose-formatting-lost-highlight-line = { $count ->
    [one] Выделение цветом удалено в { $count } тексте.
    [few] Выделение цветом удалено в { $count } текстах.
    [many] Выделение цветом удалено в { $count } текстах.
   *[other] Выделение цветом удалено в { $count } текста.
}
import-scrivener-prose-formatting-lost-small-caps-line = { $count ->
    [one] Капитель отображается обычными буквами в { $count } тексте.
    [few] Капитель отображается обычными буквами в { $count } текстах.
    [many] Капитель отображается обычными буквами в { $count } текстах.
   *[other] Капитель отображается обычными буквами в { $count } текста.
}
import-scrivener-prose-formatting-lost-all-caps-line = { $count ->
    [one] Текст, показанный прописными буквами, отображается так, как был набран, в { $count } тексте.
    [few] Текст, показанный прописными буквами, отображается так, как был набран, в { $count } текстах.
    [many] Текст, показанный прописными буквами, отображается так, как был набран, в { $count } текстах.
   *[other] Текст, показанный прописными буквами, отображается так, как был набран, в { $count } текста.
}
import-scrivener-prose-formatting-lost-hidden-line = { $count ->
    [one] Скрытый текст не переносится в { $count } тексте.
    [few] Скрытый текст не переносится в { $count } текстах.
    [many] Скрытый текст не переносится в { $count } текстах.
   *[other] Скрытый текст не переносится в { $count } текста.
}
import-scrivener-prose-formatting-lost-line = { $count ->
    [one] Форматирование, которому нет места в Skribisto, удалено в { $count } тексте.
    [few] Форматирование, которому нет места в Skribisto, удалено в { $count } текстах.
    [many] Форматирование, которому нет места в Skribisto, удалено в { $count } текстах.
   *[other] Форматирование, которому нет места в Skribisto, удалено в { $count } текста.
}
import-scrivener-prose-justified-text-not-kept-line = { $count ->
    [one] { $count } абзац с выравниванием по ширине теперь следует стилю экспорта.
    [few] { $count } абзаца с выравниванием по ширине теперь следуют стилю экспорта.
    [many] { $count } абзацев с выравниванием по ширине теперь следуют стилю экспорта.
   *[other] { $count } абзаца с выравниванием по ширине теперь следуют стилю экспорта.
}
import-scrivener-prose-comment-colours-lost-line = { $count ->
    [one] { $count } комментарий теряет свой цвет.
    [few] { $count } комментария теряют свой цвет.
    [many] { $count } комментариев теряют свой цвет.
   *[other] { $count } комментария теряют свой цвет.
}
import-scrivener-prose-control-characters-removed-line = { $count ->
    [one] { $count } невидимый управляющий символ удалён.
    [few] { $count } невидимых управляющих символа удалены.
    [many] { $count } невидимых управляющих символов удалены.
   *[other] { $count } невидимого управляющего символа удалены.
}
import-scrivener-prose-note-on-heading-lost-line = { $count ->
    [one] { $count } сноске или комментарию на заголовке главы не нашлось там места: комментарий переходит к первому абзацу главы, а сноска не переносится.
    [few] { $count } сноскам или комментариям на заголовках глав не нашлось там места: комментарий переходит к первому абзацу главы, а сноска не переносится.
    [many] { $count } сноскам или комментариям на заголовках глав не нашлось там места: комментарий переходит к первому абзацу главы, а сноска не переносится.
   *[other] { $count } сноски или комментария на заголовках глав не нашлось там места: комментарий переходит к первому абзацу главы, а сноска не переносится.
}
import-scrivener-prose-media-file-copied-line = { $count ->
    [one] { $count } файл исследований, который Skribisto не умеет открывать, скопирован рядом с новым проектом. Где он лежит, указано в его заметке.
    [few] { $count } файла исследований, которые Skribisto не умеет открывать, скопированы рядом с новым проектом. Где они лежат, указано в их заметках.
    [many] { $count } файлов исследований, которые Skribisto не умеет открывать, скопированы рядом с новым проектом. Где они лежат, указано в их заметках.
   *[other] { $count } файла исследований, которые Skribisto не умеет открывать, скопированы рядом с новым проектом. Где они лежат, указано в их заметках.
}
import-scrivener-prose-media-file-not-carried-line = { $count ->
    [one] { $count } файл исследований, который Skribisto не умеет открывать, остаётся в вашем проекте Scrivener. Он назван в своей заметке.
    [few] { $count } файла исследований, которые Skribisto не умеет открывать, остаются в вашем проекте Scrivener. Они названы в своих заметках.
    [many] { $count } файлов исследований, которые Skribisto не умеет открывать, остаются в вашем проекте Scrivener. Они названы в своих заметках.
   *[other] { $count } файла исследований, которые Skribisto не умеет открывать, остаются в вашем проекте Scrivener. Они названы в своих заметках.
}

## When the import fails
## Shown in the notice of a failed import, above the button import-scrivener-review-again,
## which these sentences quote.
import-scrivener-error-refused = { $refusal } Ничего не записано.
import-scrivener-error-question = Теперь проект задаёт вопрос, на который при проверке не было ответа. Нажмите «Вернуться к проверке», затем импортируйте. Ничего не записано.
import-scrivener-error-not-accepted = Чтобы проект можно было импортировать, нужно установить флажок на шаге «Проект», а он не установлен. Нажмите «Вернуться к проверке», установите флажок, затем импортируйте. Ничего не записано.
import-scrivener-error-changed = Проект изменился после вашей проверки. Нажмите «Вернуться к проверке», затем импортируйте. Ничего не записано.
import-scrivener-error-choice = Структура, выбранная при проверке, больше не подходит к этому проекту. Нажмите «Вернуться к проверке», затем импортируйте. Ничего не записано.

export-title = Экспорт
export-close = Закрыть
export-scope-label = Что
export-format-label = Формат
export-style-label = Стиль
export-path-label = Файл
export-browse = Выбрать…
export-show-non-exportable = Показывать неэкспортируемые
export-choose-empty = Нет проекта для выбора.
export-cancel = Отмена
export-export = Экспорт
export-save-dialog-title = Экспорт в файл
export-section-what = Что экспортировать
export-section-style = Стиль
export-section-destination = Куда сохранить
export-custom-selection = Произвольный выбор
export-selected-count = Выбрано: { $count }
export-chip-chapters-none = Главы: нет
export-chip-chapters-numbered = Главы: нумерованные
export-chip-chapters-title = Главы: только название
export-chip-chapters-both = Главы: номер + название
export-chip-scene-break-glyph = Разрыв сцены: { $glyph }
export-chip-scene-break-blank = Разрыв сцены: пустая строка
export-chip-scene-break-none = Разрыв сцены: нет
export-chip-major-break-glyph = Большой разрыв: { $glyph }
export-chip-major-break-blank = Большой разрыв: пустая строка
export-chip-major-break-none = Большой разрыв: нет
export-chip-major-break-same = Оба уровня разрыва одинаковы
export-chip-spacing-single = Интервал: одинарный
export-chip-spacing-onehalf = Интервал: 1½
export-chip-spacing-double = Интервал: двойной
export-chip-notes-included = Заметки включены
export-chip-notes-excluded = Заметки исключены
export-format-docx = Word
export-format-odt = LibreOffice
export-format-html = HTML
export-format-markdown = Markdown
export-format-djot = Djot
export-format-text = Текст
export-format-latex = LaTeX
export-format-epub = EPUB
export-format-pdf = PDF
export-overwrite-title = Заменить существующий файл?
export-overwrite-text = «{ $name }» уже существует. Заменить его?
export-progress-title = Экспорт…
export-cancelled = Экспорт отменён
export-done = Экспортировано: { $count ->
    [one] { $count } элемент
    [few] { $count } элемента
    [many] { $count } элементов
   *[other] { $count } элемента
}
export-open-file = Открыть
export-show-in-folder = Показать в папке
export-error-title = Не удалось выполнить экспорт
export-error-details = Подробности

backup-progress-start = Подготовка…
backup-progress-retention = Удаление старых резервных копий…
backup-progress-done = Готово
backup-progress-destination = Место назначения { $i } из { $n }
backup-details = Подробности
backup-issues-title = Проблемы резервного копирования
backup-failed-title = Резервное копирование не удалось
backup-complete-prune-warning = Резервное копирование завершено (сохранено: { $ok }, уже актуальны: { $skipped }). Некоторые старые резервные копии не удалось удалить

search = Поиск
search-query-placeholder = Поиск…
search-replace-placeholder = Заменить на…
search-replace-toggle = Показать или скрыть замену
search-replace-all = Заменить всё
search-opt-case = С учётом регистра
search-opt-whole-word = Целое слово
search-opt-diacritics = С учётом диакритики
search-scope-body = Текст
search-scope-title = Название
search-scope-synopsis = Синопсис
search-scope-label = Метка
search-facet-book = Книги
search-facet-part = Части
search-facet-chapter = Главы
search-facet-scene = Сцены
search-facet-note = Заметки
search-facet-folder = Папки
search-tip-case = С учётом регистра: прописные и строчные буквы различаются, поэтому «Елена» и «елена» — разные совпадения.
search-tip-whole-word = Слово целиком: находить только целые слова, поэтому «кот» не будет найден внутри «котлета».
search-tip-diacritics = С учётом диакритики: буквы с диакритическими знаками считаются отдельными, поэтому «все» не совпадает с «всё».
search-tip-body = Текст: поиск в прозе сцен и заметок.
search-tip-title = Название: поиск в названиях элементов подшивки.
search-tip-synopsis = Синопсис: поиск в кратком содержании каждого текстового элемента.
search-tip-label = Метка: поиск в короткой пометке под названием элемента подшивки.
search-tip-comment = Комментарии: поиск в тексте обсуждений и ответов на них. Комментарий касается рукописи, но не является её частью, поэтому у него свой переключатель, а при замене совпадения в комментариях остаются неотмеченными, пока вы их не отметите.
search-tip-book = Книги: контейнер книги и его метки начала и конца.
search-tip-part = Части: разделители уровня частей.
search-tip-chapter = Главы: главы, каким бы способом проект их ни хранил.
search-tip-scene = Сцены: элементы, содержащие вашу прозу.
search-tip-note = Заметки: заметки в свободной форме.
search-tip-folder = Папки: простые организующие папки и разделители.
search-tip-paratext = Текст, который принадлежит книге, но не её истории: предисловие, посвящение, послесловие. Никогда не учитывается в рукописи.
search-tip-preserve-case = Сохранять регистр: замена перенимает регистр найденного, поэтому «ЕЛЕНА» становится «МАРТА», а «Елена» — «Марта».
search-tip-replace = Замена: показать поле замены и кнопку «Заменить всё».
search-error = Поиск не удался: { $message }
search-no-matches = Совпадений нет
search-count =
    { $matches ->
        [one] { $matches } совпадение
        [few] { $matches } совпадения
        [many] { $matches } совпадений
       *[other] { $matches } совпадения
    } в { $items ->
        [one] { $items } документе
        [few] { $items } документах
        [many] { $items } документах
       *[other] { $items } документах
    }
search-count-truncated =
    { $matches ->
        [one] { $matches } совпадение
        [few] { $matches } совпадения
        [many] { $matches } совпадений
       *[other] { $matches } совпадения
    } в { $items ->
        [one] { $items } документе
        [few] { $items } документах
        [many] { $items } документах
       *[other] { $items } документах
    } (только первые результаты)
search-occurrences = { $count }
search-field-body = Текст
search-field-title = Название
search-field-synopsis = Синопсис
search-field-label = Метка
search-field-epigraph = Эпиграф
search-field-comment = Комментарий
search-field-comment-reply = Ответ
search-field-footnote = Сноска
search-include-in-replace = Включить в «Заменить всё»
search-collapse-all = Свернуть все результаты
search-replace-here = Заменить это
search-dismiss = Убрать из результатов
search-undo-dismiss = Вернуть последний убранный результат
search-replace-nothing = (ничего)
search-replace-confirm-title = Заменить все совпадения?
search-replace-confirm-text =
    Заменить { $occurrences ->
        [one] { $occurrences } вхождение
        [few] { $occurrences } вхождения
        [many] { $occurrences } вхождений
       *[other] { $occurrences } вхождения
    } «{ $query }» на «{ $replacement }» в { $items ->
        [one] { $items } документе
        [few] { $items } документах
        [many] { $items } документах
       *[other] { $items } документах
    }? Потом это можно отменить из уведомления.
search-replace-done-title = Замена завершена
search-replace-done =
    Заменено { $occurrences ->
        [one] { $occurrences } вхождение
        [few] { $occurrences } вхождения
        [many] { $occurrences } вхождений
       *[other] { $occurrences } вхождения
    } в { $items ->
        [one] { $items } документе
        [few] { $items } документах
        [many] { $items } документах
       *[other] { $items } документах
    }.
search-replace-done-skipped =
    Заменено { $occurrences ->
        [one] { $occurrences } вхождение
        [few] { $occurrences } вхождения
        [many] { $occurrences } вхождений
       *[other] { $occurrences } вхождения
    } в { $items ->
        [one] { $items } документе
        [few] { $items } документах
        [many] { $items } документах
       *[other] { $items } документах
    }. { $skipped ->
        [one] { $skipped } поле пропущено
        [few] { $skipped } поля пропущено
        [many] { $skipped } полей пропущено
       *[other] { $skipped } поля пропущено
    } (изменились после поиска).
search-replace-undo = Отменить
search-replace-failed-title = Не удалось выполнить замену
search-replace-skipped-title = Ничего не заменено
search-replace-skipped-body =
    { $fields ->
        [1] Этот текст изменился
        [one] { $fields } из этих текстов изменился
        [few] { $fields } из этих текстов изменились
        [many] { $fields } из этих текстов изменились
       *[other] { $fields } из этих текстов изменились
    } после поиска, поэтому { $fields ->
        [1] он оставлен
        [one] они оставлены
        [few] они оставлены
        [many] они оставлены
       *[other] они оставлены
    } без изменений. Выполните поиск снова, чтобы увидеть, где теперь находятся слова.
search-replace-undo-failed-title = Не удалось отменить
search-preview = Предпросмотр
search-preview-empty = Выберите результат, чтобы увидеть его здесь
search-preview-no-prose = В этом совпадении нет редактируемого текста
search-preview-prompt = Чтобы увидеть здесь предпросмотр, выполните поиск.
search-preview-open-search = Поиск в проекте
search-preview-footnote-prompt = Это совпадение находится в тексте сноски. Откройте его на панели сносок, чтобы просмотреть и изменить.
search-preview-open-footnotes = Открыть сноски

find-placeholder = Найти в этом документе…
find-count = { $current } из { $total }
find-no-results = Нет результатов
find-close = Закрыть поиск
find-previous = Предыдущее совпадение (Shift+Enter)
find-next = Следующее совпадение (Enter)
find-opt-case = С учётом регистра
find-opt-whole-word = Целое слово
find-replace-toggle = Показать или скрыть замену (Ctrl+R)
find-replace-placeholder = Заменить на…
find-replace = Заменить
find-replace-all = Заменить всё
find-preserve-case = Сохранять регистр

dict-download-title = Загрузка { $name }…
dict-download-done = Установлено: { $name }
dict-download-failed = Не удалось загрузить { $name }: { $error }
dict-removed = Удалено: { $name }
dict-accept-first = Примите лицензию перед загрузкой { $name }
settings-dict-tab-installed = Установленные
settings-dict-tab-get-more = Загрузить ещё
settings-dict-tab-personal = Личные слова
dict-installed-empty = Словарей на этом компьютере пока не найдено.
dict-get-more-search = Поиск языков
dict-system-badge = в вашей системе
dict-unusable-badge = непригоден
dict-download-button = Загрузить
dict-downloading = Загрузка…
dict-installed-label = Установлен
dict-remove = Удалить
dict-view-license = Просмотр лицензии
dict-approx-size = ~{ $size }
dict-personal-empty = В этом проекте пока нет личных слов.
dict-personal-add = Добавить
dict-personal-placeholder = Добавить слово…
dict-missing-toast = { $count ->
    [one] В этом проекте используется { $count } словарь, который у вас не установлен
    [few] В этом проекте используются { $count } словаря, которые у вас не установлены
    [many] В этом проекте используются { $count } словарей, которые у вас не установлены
   *[other] В этом проекте используются { $count } словаря, которые у вас не установлены
}
dict-missing-action = Получить словари
dict-license-title = Лицензия { $name }
dict-license-accept = Принять и загрузить
dict-license-cancel = Отмена
dict-license-close = Закрыть
dict-add-button = Добавить словарь…
dict-add-title = Добавить словарь
dict-add-close = Закрыть
dict-add-name = Название
dict-add-name-placeholder = например, Мой латинский словарь
dict-add-code = Код языка
dict-add-code-placeholder = например, la или fr-FR-x-custom
dict-add-code-hint = Короткий тег на ваш выбор. Именно его вы выбираете в качестве языка документа.
dict-add-aff = Файл аффиксов (.aff)
dict-add-dic = Список слов (.dic)
dict-add-submit = Добавить
dict-add-cancel = Отмена
dict-add-name-required = Дайте словарю имя.
dict-add-code-required = Введите код языка.
dict-add-code-invalid = Используйте только буквы, цифры и символы - _ .
dict-add-code-reserved = Это код встроенного словаря. Выберите другой.
dict-add-file-required = Выберите файл.
dict-add-file-missing = Этот файл не существует.
dict-add-code-taken = Словарь для этого кода уже установлен.
dict-add-done = Добавлен: { $name }
dict-add-unusable = Эти файлы не образуют пригодный словарь: { $error }
dict-add-failed = Не удалось добавить словарь: { $error }
inspector-export = Экспорт
inspector-exportable = Включать в экспорт
ctx-number = &Нумеровать эту главу
ctx-unnumber = &Не нумеровать эту главу
inspector-numbering = Нумерация
inspector-numbered = Нумеруется
inspector-numbered-tip = Эта глава участвует в нумерации книги. Выключите для пролога, эпилога или интерлюдии.
inspector-numbered-tip-more = Ненумерованная глава остаётся в книге в точности такой, какой была: её заголовок, проза и количество слов не меняются. Она просто не печатает номера и не расходует его: глава после пролога — первая, а не вторая. Исключение главы из экспорта — это другой переключатель, выше, и он убирает главу из книги целиком.
inspector-apply-to-children = Применить к вложенным
inspector-milestone = Дата вехи
inspector-milestone-none = Нет даты
inspector-milestone-clear = Очистить
inspector-tags = Теги
inspector-aliases = Другие имена
inspector-dict-language = Язык
inspector-apply-language-to-children = Применить язык к вложенным
settings-page-language = Язык
settings-field-dict-language = Языки
dict-tradeoff-hint = Каждый дополнительный язык принимает больше слов, поэтому выявляется меньше ошибок.
lang-inherit-hint = Наследуется: в этой сцене используются языки книги или проекта.
lang-pill-list = Языки
lang-pill-add = Добавить язык
lang-pill-remove = Удалить { $name }
lang-pill-mute = Отключить проверку орфографии для { $name }
lang-pill-unmute = Включить проверку орфографии для { $name }

settings-page-personal-dictionary = Личный словарь
settings-user-dict-desc = Слова, добавленные здесь, считаются написанными правильно во всём приложении и больше никогда не помечаются.
settings-user-dict-add-placeholder = Добавить слово…
settings-user-dict-add = Добавить слово
settings-user-dict-search = Фильтр слов
settings-user-dict-count = { $count ->
    [one] { $count } слово
    [few] { $count } слова
    [many] { $count } слов
   *[other] { $count } слова
}

settings-user-dict-rename = Переименовать
settings-user-dict-remove = Удалить
settings-user-dict-empty = Слов пока нет. Добавьте слова, которые проверка орфографии должна пропускать, или импортируйте список.
settings-user-dict-import = Импорт…
settings-user-dict-export = Экспорт…
settings-user-dict-import-tip = Добавить слова из обычного текстового файла, по одному слову в строке. Имеющиеся слова сохраняются.
settings-user-dict-export-tip = Сохранить весь список в обычный текстовый файл, по одному слову в строке.
settings-user-dict-txt-filter = Текстовые файлы
settings-user-dict-duplicate = Уже в словаре
settings-user-dict-imported = Импортировано: { $count ->
    [one] { $count } слово
    [few] { $count } слова
    [many] { $count } слов
   *[other] { $count } слова
} (уже было в словаре: { $duplicates }).
settings-user-dict-import-failed = Не удалось прочитать список слов: { $error }
settings-user-dict-exported = Сохранено: { $count ->
    [one] { $count } слово.
    [few] { $count } слова.
    [many] { $count } слов.
   *[other] { $count } слова.
}
settings-user-dict-export-failed = Не удалось сохранить список слов: { $error }

settings-page-text-replacements = Автозамена
settings-text-repl-desc = Заменяет сокращение полным текстом по мере ввода: «кст» превращается в «кстати», как только вы вводите пробел или знак препинания.
settings-text-repl-enable = Использовать автозамену в этом проекте
settings-text-repl-disabled-hint = Включите, чтобы задавать сокращения, которые раскрываются по мере письма.
settings-text-repl-add = Добавить правило
settings-text-repl-trigger-placeholder = Сокращение
settings-text-repl-replacement-placeholder = Во что превращается
settings-text-repl-added = Добавлено «{ $trigger }»
settings-text-repl-duplicate = Для «{ $trigger }» уже есть правило
settings-text-repl-filter = Фильтр правил
settings-text-repl-count = { $n ->
    [one] { $n } правило
    [few] { $n } правила
    [many] { $n } правил
   *[other] { $n } правила
}
settings-text-repl-row-enabled = Использовать это правило
settings-text-repl-delete = Удалить правило для { $trigger }
settings-text-repl-deleted = Удалено правило для «{ $trigger }»
settings-text-repl-empty = Правил пока нет.
settings-text-repl-csv-filter = Файлы CSV
settings-text-repl-import = Импорт…
settings-text-repl-export = Экспорт…
settings-text-repl-imported = Импортировано: { $added }, пропущено: { $skipped }
settings-text-repl-exported = Экспортировано: { $n ->
    [one] { $n } правило
    [few] { $n } правила
    [many] { $n } правил
   *[other] { $n } правила
}

editor-menu-no-suggestions = Нет вариантов
editor-menu-add-to-dictionary = Добавить «{ $word }» в словарь
editor-menu-add-words-to-dictionary = Добавить выбранные слова в словарь
editor-dict-added = «{ $word }» добавлено в ваш словарь.
editor-dict-added-multi = { $count ->
    [one] В ваш словарь добавлено { $count } слово.
    [few] В ваш словарь добавлено { $count } слова.
    [many] В ваш словарь добавлено { $count } слов.
   *[other] В ваш словарь добавлено { $count } слова.
}
toast-undo = Отменить

undo-superseded-title = Этот шаг уже не последний
undo-superseded-body = С тех пор в проекте изменилось что-то ещё. Отмена затронула бы именно это изменение, поэтому она не выполнена. Ничего из сделанного на этом шаге не отменено. Чтобы пройти назад по истории самостоятельно, используйте Правка ▸ Отменить.
undo-failed = Не удалось отменить: { $error }

menu-edit = &Правка
menu-edit-undo = &Отменить
menu-edit-redo = По&вторить
menu-edit-undo-target = &Отменить: { $target }
menu-edit-redo-target = По&вторить: { $target }
shortcut-name-edit-undo = Отменить
shortcut-name-edit-redo = Повторить
undo-target-typing = ввод текста
undo-target-project = последнее изменение в этом проекте
prose-history-reset-title = История ввода сброшена
prose-history-reset-body = { $count ->
    [one] { $count } открытая сцена восстановлена из более раннего состояния, поэтому её история ввода больше не действует.
    [few] { $count } открытые сцены восстановлены из более раннего состояния, поэтому их история ввода больше не действует.
    [many] { $count } открытых сцен восстановлены из более раннего состояния, поэтому их история ввода больше не действует.
   *[other] { $count } открытой сцены восстановлены из более раннего состояния, поэтому их история ввода больше не действует.
}
undo-target-trash = перемещение в корзину
undo-target-restore = восстановление из корзины
undo-target-delete-forever = окончательное удаление
undo-target-replace-all = замена во всём проекте
undo-target-import = импорт документа
undo-target-duplicate = дублирование
undo-target-move = перемещение
undo-target-merge = объединение двух сцен
undo-target-split = разделение сцены
undo-target-promote = изменение типа
undo-target-tidy-titles = упорядочивание названий глав
undo-target-import-tags = импорт тегов
undo-target-import-templates = импорт шаблонов заметок
undo-target-create = создание
undo-target-remove = удаление
undo-target-rename = переименование
undo-target-edit = эту правку
menu-edit-find = &Найти…
menu-edit-find-next = Найти &далее
menu-edit-find-prev = Найти &предыдущее
menu-edit-replace = Найти и за&менить…
menu-edit-find-in-project = Найти в проек&те…
menu-edit-replace-in-project = &Заменить в проекте…
undo-frozen = Отменить (включён режим «Только вперёд»)
redo-frozen = Повторить (включён режим «Только вперёд»)
menu-edit-cut = Вы&резать
menu-edit-copy = &Копировать
menu-edit-paste = Вст&авить
menu-edit-paste-plain = Вставить &без форматирования
menu-edit-select-all = Выделить в&сё
shortcut-name-edit-cut = Вырезать
shortcut-name-edit-copy = Копировать
shortcut-name-edit-paste = Вставить
shortcut-name-edit-paste-plain = Вставить без форматирования
shortcut-name-edit-select-all = Выделить всё

titlebar-spellcheck-on = Проверка орфографии включена. Щёлкните, чтобы выключить (F7)
titlebar-spellcheck-off = Проверка орфографии выключена. Щёлкните, чтобы включить (F7)
titlebar-distraction-free = Перейти в режим без отвлечений (F11)
menu-spellcheck = Проверять &орфографию
menu-comments = &Комментарии
settings-page-spellcheck = Проверка орфографии
settings-group-spellcheck = Проверка орфографии
settings-spellcheck-enabled = Проверять орфографию при вводе
settings-spellcheck-hint = Подчёркивает слова, которых не знает ни один установленный словарь. Выключение останавливает всю проверку во всех проектах, пока вы не включите её снова. Чтобы перестать проверять только один язык, снимите его флажок в поле «Язык» проекта или элемента.

menu-trash = &Корзина
trash-title = Корзина
trash-empty-state = Корзина пуста.
trash-empty-button = Очистить корзину…
trash-restore = &Восстановить
trash-restore-to = Восстановить &в…
trash-delete-forever = &Удалить навсегда
trash-restored-ok = Восстановлено: { $count }.
trash-restore-error = Не удалось восстановить: { $error }
trash-restore-orphaned = Исходного места этого элемента больше нет. Выберите, куда его восстановить.
trash-restore-no-project = Нет открытого проекта, поэтому восстанавливать нечего.
trash-delete-no-project = Нет открытого проекта, поэтому удалять нечего.
trash-empty-confirm-title = Очистить корзину?
trash-empty-confirm-text = { $count ->
    [one] { $count } запись из корзины будет удалена безвозвратно.
    [few] { $count } записи из корзины будут удалены безвозвратно.
    [many] { $count } записей из корзины будут удалены безвозвратно.
   *[other] { $count } записи из корзины будут удалены безвозвратно.
} Они навсегда покинут корзину. Отмена ещё может вернуть их, пока вы не сделаете что-нибудь другое.
trash-emptied-title = Корзина очищена
trash-emptied-body = Всё содержимое корзины удалено безвозвратно.
trash-empty-no-project = Нет открытого проекта, поэтому нечего очищать.
trash-delete-forever-confirm-title = Удалить навсегда?
trash-delete-forever-confirm-text = { $count ->
    [one] { $count } элемент будет удалён безвозвратно.
    [few] { $count } элемента будут удалены безвозвратно.
    [many] { $count } элементов будут удалены безвозвратно.
   *[other] { $count } элемента будут удалены безвозвратно.
} Они навсегда покинут корзину. Отмена ещё может вернуть их, пока вы не сделаете что-нибудь другое.
trash-deleted-title = Удалено навсегда
trash-deleted-body = Удалено безвозвратно: { $count }.
trash-undo = Отменить
trash-restore-picker-title = Восстановить в…
trash-restore-to-confirm-title = Восстановить здесь?
trash-restore-to-confirm-text = Восстановить «{ $item }» в «{ $destination }»?
trash-restore-picker-restore-here = Восстановить здесь
trash-restore-picker-cancel = Отмена
trash-restore-picker-empty = Подшивок пока нет. Сначала создайте подшивку.
trash-banner-title = Этот элемент находится в корзине
trash-banner-description = Он не появится в структуре и при экспорте, пока вы его не восстановите.
trash-banner-restore = Восстановить…
trash-tab-tooltip = В корзине

settings-styles-sheet-yes = Да
settings-styles-sheet-no = Нет
settings-styles-sheet-font = Шрифт
settings-styles-sheet-indent = Отступ первой строки
settings-styles-sheet-para-spacing = Интервалы между абзацами
settings-styles-sheet-page = Размер страницы
settings-styles-sheet-margins = Поля (В/П/Н/Л)
settings-styles-sheet-title-page = Титульный лист книги
settings-styles-sheet-heading-language = Язык заголовков
settings-styles-sheet-auto = По тексту
settings-styles-sheet-digits = Цифры
settings-styles-sheet-direction = Направление текста
settings-styles-sheet-formats = Форматы
settings-styles-sheet-all-formats = Все

settings-styles-editor-group = Редактор стиля
settings-styles-editor-missing = Этот стиль больше не доступен.

settings-styles-page-letter = Letter
settings-styles-digits-western = Западные (0–9)
settings-styles-digits-eastern-arabic = Восточноарабские (٠–٩)
settings-styles-direction-ltr = Слева направо
settings-styles-direction-rtl = Справа налево

format-dock-title = Формат
format-panel-empty = Поместите курсор в сцену, заметку или синопсис, чтобы увидеть варианты форматирования.
format-group-marks = Текст
format-group-block = Абзац
format-group-lists = Списки
format-group-tables = Таблица
format-group-breaks = Разрывы сцен
format-superscript = Надстрочный
format-subscript = Подстрочный
format-link = Ссылка…
link-dialog-insert-title = Вставить ссылку
link-dialog-edit-title = Изменить ссылку
link-dialog-text-label = Текст
link-dialog-text-placeholder = слова, которые видит читатель
link-dialog-url-label = Адрес ссылки
link-dialog-url-placeholder = example.com
link-dialog-insert = Вставить
link-dialog-apply = Применить
link-dialog-cancel = Отмена
link-dialog-remove = Удалить ссылку
link-scheme-refused = Открывать можно только веб-ссылки и ссылки на электронную почту. Ссылка { $url } оставлена без изменений.
menu-format-link = Сс&ылка…
format-clear = Очистить форматирование
format-heading = Уровень заголовка
format-heading-normal = Обычный текст
format-heading-1 = Заголовок 1
format-heading-2 = Заголовок 2
format-heading-3 = Заголовок 3
format-heading-4 = Заголовок 4
format-heading-5 = Заголовок 5
format-heading-6 = Заголовок 6
format-align-left = По левому краю
format-align-center = По центру
format-direction-rtl = Абзац справа налево
format-blockquote = Цитата
format-list-bullet = Маркированный список
format-list-numbered = Нумерованный список
format-indent = Увеличить отступ
format-outdent = Уменьшить отступ
format-table-insert = Вставить таблицу
format-table-row-above = Вставить строку выше
format-table-row-below = Вставить строку ниже
format-table-col-before = Вставить столбец слева
format-table-col-after = Вставить столбец справа
format-table-row-delete = Удалить строку
format-table-col-delete = Удалить столбец
format-table-remove = Удалить таблицу

menu-format-marks-bold = По&лужирный
menu-format-marks-italic = &Курсив
menu-format-marks-underline = По&дчёркнутый
menu-format-marks-strike = З&ачёркнутый
menu-format-marks-superscript = &Надстрочный
menu-format-marks-subscript = П&одстрочный
menu-format-marks-clear = О&чистить форматирование
menu-format-heading = &Заголовок
menu-format-heading-normal = &Обычный текст
menu-format-heading-1 = Заголовок &1
menu-format-heading-2 = Заголовок &2
menu-format-heading-3 = Заголовок &3
menu-format-heading-4 = Заголовок &4
menu-format-heading-5 = Заголовок &5
menu-format-heading-6 = Заголовок &6
menu-format-alignment = &Выравнивание
menu-format-align-left = По &левому краю
menu-format-align-center = &По центру
menu-format-direction = Направл&ение
menu-format-direction-auto = &Автоматически
menu-format-direction-ltr = С&лева направо
menu-format-direction-rtl = &Справа налево
menu-format-blockquote = &Цитата
menu-format-lists = &Списки
menu-format-list-bullet = &Маркированный список
menu-format-list-numbered = &Нумерованный список
menu-format-indent = У&величить отступ
menu-format-outdent = &Уменьшить отступ
menu-format-table = &Таблица
menu-format-table-insert = &Вставить таблицу
menu-format-table-2x2 = &2 x 2
menu-format-table-3x3 = &3 x 3
menu-format-table-4x4 = &4 x 4
menu-format-table-row-above = Вставить строку в&ыше
menu-format-table-row-below = Вставить строку &ниже
menu-format-table-col-before = Вставить столбец с&лева
menu-format-table-col-after = Вставить столбец с&права
menu-format-table-row-delete = &Удалить строку
menu-format-table-col-delete = Удалить &столбец
menu-format-table-remove = Удалить &таблицу

menu-format-typo-marks = Знак &форматирования
menu-format-typo-nbsp = &Неразрывный пробел
menu-format-typo-nbhyphen = Неразрывный &дефис
menu-format-typo-soft-hyphen = &Мягкий перенос
menu-format-typo-nnbsp = &Узкий неразрывный пробел
menu-format-typo-zwsp = &Пробел нулевой ширины
menu-format-typo-word-joiner = &Соединитель слов


menu-about = &О программе Skribisto…
about-title = О программе Skribisto
about-version = Версия { $version }
about-update-available = Доступна версия { $version }
about-update-see-notes = Что изменилось
about-update-flathub = Эту копию обновляет Flathub.
about-update-distro = Эту копию обновляет ваш дистрибутив.
about-tagline = Приложение для писателей, работающих над крупной прозой, написанное на Rust с инструментарием Teksilo.
about-license = Распространяется на условиях GNU General Public License версии 3.
about-copyright = © 2026 Cyril Jacquet
about-close = Закрыть

native-menu-about = О программе { $app }
native-menu-hide = Скрыть { $app }
native-menu-hide-others = Скрыть остальные
native-menu-show-all = Показать все
native-menu-services = Службы
native-menu-quit = Завершить { $app }
native-menu-settings = Настройки…
native-menu-window = Окно
native-menu-minimize = Свернуть
native-menu-zoom = Масштаб

window-title = { $title } - { $app }
window-title-numbered = { $title } - { $app } (Окно { $n })
window-title-empty = { $app }

first-run-window-title = Добро пожаловать
first-run-title = Настройка { $app }
first-run-body = { $app } хранит свои настройки отдельно от Skribisto, поэтому изначально они пусты. Ваши предпочтения, недавние проекты, словари и расположение окон можно скопировать прямо сейчас.
first-run-from = Откуда копировать
first-run-to = Куда копировать
first-run-copy-note = Ничего не перемещается и не удаляется. Skribisto сохраняет все свои настройки и продолжает работать точно так же, как раньше.
first-run-import = Импортировать настройки
first-run-start-fresh = Начать с чистого листа
first-run-import-failed = Некоторые настройки не удалось импортировать: { $error }

settings-page-punctuation = Пунктуация по умолчанию
settings-group-punctuation = Умная пунктуация
settings-page-work-punctuation = Пунктуация
settings-punctuation-override = Задать для этого проекта собственные правила пунктуации
settings-punctuation-override-hint = Выключено: проект следует настройкам приложения. Эти правила хранятся внутри файла .skrib, поэтому соавтор, открывший файл, пишет с той же типографикой.
settings-punctuation-dashes = Заменять -- на короткое тире, --- на длинное тире
settings-punctuation-ellipsis = Заменять ... на многоточие
settings-punctuation-quotes = Типографские кавычки и апострофы
settings-quote-style = Кавычки
settings-quote-style-locale = По языку
settings-quote-style-curly = “Двойные”
settings-quote-style-curly-single = ‘Одинарные’
settings-quote-style-guillemets = «Ёлочки»
settings-quote-style-low-high = „Лапки“
settings-punctuation-spacing = Пробел перед ; : ! ?
settings-punctuation-spacing-hint = Французская типографика ставит узкий неразрывный пробел перед ; ! ? и обычный — перед :. Это действует только для текста на французском языке. Пробелы внутри кавычек-ёлочек « » добавляются вместе с самими кавычками.
settings-punctuation-sample = Для вашего языка получится
settings-punctuation-app-hint = Что делают все проекты, если не задают собственных правил в разделе Проект ▸ Пунктуация.
settings-punctuation-dialogue = Начинать абзац, набранный как «- », с тире прямой речи
settings-punctuation-dialogue-hint = Для языков, где прямая речь обозначается тире, а не кавычками: французского, испанского, русского и других. Срабатывает только в самом начале абзаца.
settings-punctuation-auto-lists = Начинать список с абзаца, набранного как "- ", "* " или "1. "
settings-punctuation-auto-lists-hint = Абзац становится маркированным или нумерованным списком. Если тире прямой речи включено и в языке оно есть, "- " вместо этого открывает реплику. Нажмите Backspace сразу после этого, чтобы вернуть набранное.

statusbar-go-to = Перейти к…
go-to-placeholder = Поиск в подшивке
go-to-no-matches = Ни один элемент не соответствует запросу.
menu-go-to = &Перейти к…

settings-page-distraction-free-themes = Темы режима без отвлечений
settings-themes-builtin = Встроенные темы
settings-themes-builtin-badge = Встроенная
settings-themes-user = Мои темы
settings-themes-editor-group = Изменить тему
settings-themes-editor-empty = Выберите тему в разделе «Мои темы», чтобы изменить её.
settings-themes-use = Использовать
settings-themes-duplicate = Дублировать
settings-themes-edit = Изменить
settings-themes-delete = Удалить
settings-themes-export = Экспорт…
settings-themes-import = Импорт…
settings-themes-copy-suffix = копия
settings-themes-json-filter = Тема (JSON)
settings-themes-imported = Тема импортирована
settings-themes-import-failed = Не удалось импортировать эту тему
settings-themes-exported = Тема экспортирована
settings-themes-export-failed = Не удалось экспортировать эту тему
settings-themes-low-contrast = низкий контраст
settings-themes-low-contrast-band = подсветка скрывает текст
settings-themes-field-name = Название
settings-themes-field-paper = Страница
settings-themes-field-ink = Текст
settings-themes-field-general = Фон
settings-themes-field-widget-text = Текст панели управления
settings-themes-field-caret-band = Подсветка вокруг курсора

statusbar-focus-settings = Настройки режима без отвлечений
statusbar-focus-manage-themes = Управление темами…

comments-title = Комментарии
comments-document-title = Этот документ
comments-empty-project = Комментариев в этом проекте пока нет.
comments-empty-document = Комментариев к этому документу нет.
comments-filter-all = Все
comments-filter-open = Открытые
comments-filter-resolved = Решённые
comments-filter-orphaned = Потерянные
comments-status-open = Открыт
comments-status-resolved = Решён
comments-status-orphaned = Текст утрачен
comments-orphan-snippet = (прокомментированный текст исчез)
comments-status-unplaced = Нет позиции в тексте
comments-unplaced-snippet = (не привязан к какому-либо тексту)
comments-reply-count = { $count ->
    [one] { $count } ответ
    [few] { $count } ответа
    [many] { $count } ответов
   *[other] { $count } ответа
}
comments-menu-resolve = Отметить решённым
comments-menu-reopen = Открыть снова
comments-menu-delete = Удалить
comments-sort-document = В порядке документа
comments-sort-newest = Сначала новые
comments-sort-reader = По читателям
comments-unsigned-author = Без подписи
comments-agreement = { $count ->
    [one] { $count } читатель
    [few] { $count } читателя
    [many] { $count } читателей
   *[other] { $count } читателя
}
comments-menu-add = Добавить комментарий
comments-menu-add-paragraph = Комментировать этот абзац
overview-col-comments = Комментарии
overview-col-total-comments = Всего комментариев
comments-card-placeholder = Напишите комментарий…
comments-card-unknown-author = Неизвестный автор
comments-unsigned-toast = Ваши комментарии не подписаны: на этом компьютере для вас не установлено имя.
comments-unsigned-action = Указать своё имя
comments-card-reply = Ответить
comments-card-reply-placeholder = Ответ…
comments-card-actions = Действия с комментарием
comments-reply-actions = Действия с ответом
comments-menu-delete-reply = Удалить ответ
comments-menu-delete-all = Удалить все комментарии здесь
comments-deleted-toast = Комментарий удалён
comments-reply-deleted-toast = Ответ удалён
comments-deleted-all-toast = { $count ->
    [one] { $count } комментарий удалён
    [few] { $count } комментария удалено
    [many] { $count } комментариев удалено
   *[other] { $count } комментария удалено
}
comments-undo = Отменить

analysis-segment = Анализ
analysis-scope-book = Анализ этой книги
analysis-run = Выполнить анализ
analysis-stale = Изменилось после анализа
analysis-not-run = Пока не проанализировано.
analysis-running = Чтение рукописи…
analysis-failed = Не удалось завершить анализ.
analysis-no-scenes = Сцен в этой книге пока нет.

analysis-shape = Форма

analysis-arrivals = Поступление текста
analysis-arrivals-explainer = Каким путём текст попадал в этот проект, пока он был открыт. Здесь видно, каким путём пришли знаки, и ничего не сказано о том, кто их написал: писатель, который пишет черновик в другом месте и вставляет его, вставил текст, а тот, кто диктует, продиктовал. Здесь нет числа, к которому нужно стремиться.
analysis-arrivals-scope = Весь этот проект, а не только эта книга.
analysis-arrivals-session = С момента открытия проекта. При закрытии проекта подсчёт начинается заново.
analysis-arrivals-nothing = За этот сеанс текст ещё не поступал.
analysis-arrivals-typed = Набрано
analysis-arrivals-pasted = Вставлено
analysis-arrivals-dictated = Продиктовано
analysis-arrivals-imported = Импортировано
analysis-arrivals-programmatic = Вставлено программой
analysis-arrivals-count = { $count ->
    [one] { $count } знак
    [few] { $count } знака
    [many] { $count } знаков
   *[other] { $count } знака
}
analysis-arrivals-none = нет

analysis-words-per-scene = Слов на сцену
analysis-median-words = Медианная сцена этой книги содержит { $count ->
    [one] { $count } слово.
    [few] { $count } слова.
    [many] { $count } слов.
   *[other] { $count } слова.
}
analysis-median-line = Медиана: { $count ->
    [one] { $count } слово
    [few] { $count } слова
    [many] { $count } слов
   *[other] { $count } слова
}
analysis-dialogue = Диалог
analysis-dialogue-unsupported = Диалог для этого языка пока не измеряется.

analysis-footnote-words = Слова в сносках
analysis-footnote-words-count = { $count ->
    [one] В сносках этой книги { $count } слово; оно учитывается отдельно от объёма рукописи.
    [few] В сносках этой книги { $count } слова; они учитываются отдельно от объёма рукописи.
    [many] В сносках этой книги { $count } слов; они учитываются отдельно от объёма рукописи.
   *[other] В сносках этой книги { $count } слова; они учитываются отдельно от объёма рукописи.
}
analysis-footnote-words-pending = Подсчёт сносок…

analysis-ignore-empty = Не учитывать тексты, в которых ещё ничего не написано
analysis-empty-hidden = { $count ->
        [one] Скрыт { $count } пустой текст.
        [few] Скрыто { $count } пустых текста.
        [many] Скрыто { $count } пустых текстов.
       *[other] Скрыто { $count } пустого текста.
    }
analysis-all-texts-empty = Все тексты этой книги пока пусты.

binder-filter-count = Показано { $shown } из { $total }
binder-filter-clear = Очистить
binder-filter-none = Ничего не совпадает с «{ $query }».

settings-paratext-intro = Вступительные и заключительные материалы, с которыми может начинаться новый проект. Каждая структура принадлежит определённой издательской традиции, и названия её страниц написаны на языке этой традиции. После создания проекта их можно свободно переименовать.
settings-paratext-structures = Структуры
settings-paratext-broken = Не удалось прочитать
settings-paratext-edit = Изменить
settings-paratext-duplicate = Дублировать
settings-paratext-delete = Удалить
settings-paratext-new = Новая структура
settings-paratext-save = Сохранить
settings-paratext-editor-hint = Название, страницы, создаваемые перед рукописью, и страницы, создаваемые после неё. Куда они попадут — решать вам: после создания проекта это обычные элементы.


image-insert = Вставить &изображение…
image-no-project = Откройте проект, прежде чем вставлять изображение.
image-choose-title = Выберите изображение
image-filter-label = Изображения
image-large-title = Это большое изображение
image-large-text =
    { $name } — это { $megapixels } Мп ({ $width }×{ $height }).
    Если оставить как есть, в проекте сохранится исходный файл: он попадёт
    в каждую резервную копию и каждый экспорт. При оптимизации вместо него
    сохраняется уменьшенная копия, до 2560 пикселей по длинной стороне.
image-large-keep = Сохранить оригинал
image-large-downscale = Оптимизировать
image-large-remember = Делать так всегда и больше не спрашивать
image-not-recorded = Изображение сохранено, но его не удалось записать в проект.
image-describe-title = Опишите изображение
image-describe-explain = Что изображено на картинке — для читателя, который её не видит. Это не часть рукописи: описание никогда не учитывается в подсчёте, не участвует в поиске и не экспортируется как проза.
image-describe-placeholder = маяк на фоне серого неба
image-resize-title = Изменить размер изображения
image-resize-explain = Процент от текущего отображаемого размера. 100 — оставить как есть.
image-resize-invalid = Введите число от 1 до 1000.
image-menu-describe = &Описать изображение…
image-menu-resize = &Изменить размер изображения…
image-menu-reset-size = Исходный &размер

cover-heading = Обложка
cover-pick = Выбрать…
cover-change = Заменить…
cover-remove = Убрать
cover-none = Обложки пока нет
cover-missing = Файла с изображением нет в проекте
cover-choose-title = Выберите обложку
cover-set = Обложка задана.
cover-cleared = Обложка удалена. Изображение по-прежнему хранится в проекте.
card-label = Метка
card-exportable = Экспортируется
card-numbered = Нумеруется
card-created = Создан
card-modified = Изменён
card-yes = да
card-no = нет
card-type = Тип
card-position = Положение
card-children = Вложенные
card-goal = Цель
card-words = Слов
card-synopsis = Синопсис
card-point-of-view = Точка зрения
card-aliases = Другие имена

export-orphan-footnotes-title = Сноски без ссылки
export-comments-dropped =
    { $count ->
        [one] { $count } комментарий не удалось разместить в экспортированном тексте, и он был опущен.
        [few] { $count } комментария не удалось разместить в экспортированном тексте, и они были опущены.
        [many] { $count } комментариев не удалось разместить в экспортированном тексте, и они были опущены.
       *[other] { $count } комментария не удалось разместить в экспортированном тексте, и они были опущены.
    }
export-orphan-footnotes =
    { $count ->
        [one] На { $count } сноску больше нет ссылок в рукописи. Её текст не появится в экспортированной книге.
        [few] На { $count } сноски больше нет ссылок в рукописи. Их текст не появится в экспортированной книге.
        [many] На { $count } сносок больше нет ссылок в рукописи. Их текст не появится в экспортированной книге.
       *[other] На { $count } сноски больше нет ссылок в рукописи. Их текст не появится в экспортированной книге.
    }


menu-footnotes = &Сноски
footnotes-title = Сноски
footnotes-insert = Вставить &сноску
footnotes-empty = Сносок пока нет. Поставьте курсор в сцену и нажмите + выше или Ctrl+Alt+F.
footnotes-filter-all = Все
footnotes-filter-document = Этот документ
footnotes-filter-orphaned = Потерянные
footnotes-orphaned = На эту сноску больше ничто не ссылается
footnotes-untitled-home = Без названия
footnotes-body-placeholder = текст сноски
footnotes-insert-tooltip = Вставить сноску в позиции курсора (Ctrl+Alt+F)
footnotes-actions = Действия со сноской
footnotes-delete = &Удалить сноску и ссылку на неё
footnotes-no-project = Откройте проект, прежде чем вставлять сноску.
footnotes-no-caret = Поместите курсор в текст сцены, чтобы вставить там сноску.
footnotes-not-created = Не удалось добавить сноску в проект.
footnotes-deleted-toast = Сноска удалена
footnotes-undo-delete = Отменить

import-document-title = Импорт документов
import-document-close = Закрыть
import-document-step-files = Файлы
import-document-step-review = Проверка
import-document-step-destination = Место назначения
import-document-step-reconcile = Объединить
import-document-reconcile-hint = Некоторые из этих элементов у вас уже есть. Укажите, что сделать с каждым.
import-document-reconcile-all-new = Ничто в этом файле не соответствует вашему проекту: каждый элемент будет добавлен как новый.
import-document-duplicate-returns = Эти файлы — несколько вернувшихся копий одной и той же рукописи. Импортируйте их по одной: сопоставление сверяет с проектом только одну вернувшуюся копию, поэтому при совместном импорте нескольких копий книга будет добавлена второй копией, а не объединена.
import-document-hunk-added = Принять этот новый абзац
import-document-hunk-removed = Убрать этот абзац
import-document-hunk-changed = Принять эту правку
import-document-col-stray-prose = Его текст
import-document-stray-as-paratext = Сохранить как паратекст
import-document-stray-discard = Отбросить текст
import-document-col-current = В вашем проекте
import-document-col-incoming = В этом файле
import-document-col-status = Статус
import-document-col-action = Что делать
import-document-status-identical = Совпадает
import-document-status-editor-edited = Изменено редактором
import-document-status-you-edited = Изменено вами
import-document-status-conflict = Изменено обеими сторонами
import-document-status-different = Отличается
import-document-status-new = Новый
import-document-status-missing = Нет в этом файле
import-document-status-moved = Перемещено
import-document-action-comments-only = Только комментарии
import-document-action-take-import = Принять эту версию
import-document-action-keep-current = Оставить мою версию
import-document-action-create-new = Добавить как новый
import-document-action-ignore = Пропустить
import-document-compare = Сравнить
import-document-compare-legend = Ваша версия в сравнении с версией из этого файла. Только чтение.
import-document-compare-close = Закрыть
import-document-drop-title = Перетащите документы сюда
import-document-drop-hint = Markdown (.md), Word (.docx), OpenDocument (.odt) и обычный текст (.txt)
import-document-browse = Выбрать…
import-document-move-up = Переместить вверх
import-document-move-down = Переместить вниз
import-document-remove-file = Убрать
import-document-file-count = { $count ->
    [one] { $count } файл
    [few] { $count } файла
    [many] { $count } файлов
   *[other] { $count } файла
}
import-document-no-files = Файлы ещё не выбраны.
import-document-col-included = Импорт
import-document-col-title = Название
import-document-col-type = Тип
import-document-col-words = Слов
import-document-col-breaks = Разрывы
import-document-col-comments = Комментарии
import-document-col-footnotes = Сноски
import-document-col-epigraph = Эпиграф
import-document-col-source = Источник
import-document-level-rules = Уровни заголовков
import-document-level-n = Заголовок { $level }
import-document-add-top-level = Добавить верхний уровень
import-document-add-top-level-tooltip = Добавить книгу над всеми проанализированными элементами — для файлов глав без заголовка книги
import-document-destination = Место назначения
import-document-destination-hint = Выберите подшивку или элемент: новые элементы попадут внутрь папки или после сцены.
import-document-destination-empty = Подшивок пока нет. Сначала создайте подшивку.
import-document-plan-empty = Импортировать пока нечего.
# How many pictures the items being imported bring into the project, under the
# review tree. Shown only when there is at least one.
import-document-pictures = { $count ->
    [1] Вместе с этими документами импортируется одно изображение.
    [one] Вместе с этими документами импортируется { $count } изображение.
    [few] Вместе с этими документами импортируются { $count } изображения.
    [many] Вместе с этими документами импортируются { $count } изображений.
   *[other] Вместе с этими документами импортируются { $count } изображения.
}
import-document-summary = { $rows ->
    [one] { $rows } элемент
    [few] { $rows } элемента
    [many] { $rows } элементов
   *[other] { $rows } элемента
} · { $breaks ->
    [one] { $breaks } разрыв сцены
    [few] { $breaks } разрыва сцены
    [many] { $breaks } разрывов сцены
   *[other] { $breaks } разрыва сцены
}
import-document-back = Назад
import-document-cancel = Отмена
import-document-analyse = Далее
import-document-analysing = Чтение документов…
import-document-step-analysing = Чтение
import-document-cancel-analysis = Остановить чтение
import-document-analyse-failed = Не удалось прочитать документы.
import-document-details = Подробности
import-document-import = Импорт
import-document-done = { $count ->
    [one] Импортирован { $count } элемент
    [few] Импортировано { $count } элемента
    [many] Импортировано { $count } элементов
   *[other] Импортировано { $count } элемента
}
import-document-undo = Отменить
import-diagnostic-file-unreadable = Не удалось прочитать «{ $path }»: { $detail }. Остальные файлы будут импортированы.
import-diagnostic-nested-too-deep = «{ $path }» не прочитан: вложенность его содержимого глубже { $limit } уровней. Ни один настоящий документ и близко к этому не подходит, а чтение такого файла может привести к аварийному завершению Skribisto, поэтому он пропущен. Остальные файлы будут импортированы.
import-diagnostic-archive-too-large = «{ $path }» не прочитан: это сжатый файл, рассчитанный на распаковку в объём, намного превышающий его размер, что может исчерпать память и привести к аварийному завершению Skribisto, поэтому он пропущен. Остальные файлы будут импортированы.
import-diagnostic-lossy-decode = { $count ->
    [one] { $count } символ в «{ $path }» не удалось декодировать. Чтобы сохранить его, пересохраните файл в кодировке UTF-8.
    [few] { $count } символа в «{ $path }» не удалось декодировать. Чтобы сохранить их, пересохраните файл в кодировке UTF-8.
    [many] { $count } символов в «{ $path }» не удалось декодировать. Чтобы сохранить их, пересохраните файл в кодировке UTF-8.
   *[other] { $count } символа в «{ $path }» не удалось декодировать. Чтобы сохранить их, пересохраните файл в кодировке UTF-8.
}
import-diagnostic-decoded-from-bom = «{ $path }» декодирован как { $detail }, а не как UTF-8.
import-diagnostic-empty-file = «{ $path }» пуст.
import-diagnostic-no-headings = В «{ $path }» нет заголовков, поэтому он импортируется как один элемент.
import-diagnostic-unsupported-format = Файлы «.{ $detail }» не поддерживаются, поэтому «{ $path }» пропущен.
import-diagnostic-front-matter-not-flat = Метаданные в начале «{ $path }»: «{ $detail }» не является простым значением и пропущено.
import-diagnostic-footnotes-degraded = { $count ->
    [one] { $count } сноска в «{ $path }» импортируется как обычный текст: сноски из Markdown не читаются.
    [few] { $count } сноски в «{ $path }» импортируются как обычный текст: сноски из Markdown не читаются.
    [many] { $count } сносок в «{ $path }» импортируются как обычный текст: сноски из Markdown не читаются.
   *[other] { $count } сноски в «{ $path }» импортируются как обычный текст: сноски из Markdown не читаются.
}
import-diagnostic-footnote-not-carried = { $count ->
    [one] { $count } сноску в «{ $path }» не удалось перенести. Остальные перенесены.
    [few] { $count } сноски в «{ $path }» не удалось перенести. Остальные перенесены.
    [many] { $count } сносок в «{ $path }» не удалось перенести. Остальные перенесены.
   *[other] { $count } сноски в «{ $path }» не удалось перенести. Остальные перенесены.
}
import-diagnostic-raw-html-dropped = { $count ->
    [one] { $count } блок необработанного HTML в «{ $path }» отброшен.
    [few] { $count } блока необработанного HTML в «{ $path }» отброшены.
    [many] { $count } блоков необработанного HTML в «{ $path }» отброшены.
   *[other] { $count } блока необработанного HTML в «{ $path }» отброшены.
}
import-diagnostic-nested-break-dropped = { $count ->
    [one] { $count } разрыв сцены внутри цитаты или списка в «{ $path }» отброшен. Сохраняются только разрывы на отдельной строке.
    [few] { $count } разрыва сцены внутри цитат или списков в «{ $path }» отброшены. Сохраняются только разрывы на отдельной строке.
    [many] { $count } разрывов сцены внутри цитат или списков в «{ $path }» отброшены. Сохраняются только разрывы на отдельной строке.
   *[other] { $count } разрыва сцены внутри цитат или списков в «{ $path }» отброшены. Сохраняются только разрывы на отдельной строке.
}
# A picture the document shows that does not come with it, one sentence per reason.
# $detail is the picture as the document names it: a file name, a path inside the
# document, or a web address. Nothing is left in the text where it was.
import-diagnostic-image-not-ingested-format = В «{ $path }» изображение «{ $detail }» сохранено в формате, который Skribisto не умеет показывать, поэтому оно не импортировано. Импортируются изображения PNG, JPEG, WebP, GIF, TIFF и BMP: сохраните его в одном из этих форматов и вставьте вручную.
import-diagnostic-image-not-ingested-remote = В «{ $path }» изображение «{ $detail }» находится в Интернете. Во время импорта изображения никогда не загружаются, поэтому оно не импортировано.
import-diagnostic-image-not-ingested-outside = В «{ $path }» изображение «{ $detail }» находится вне документа и его папки, поэтому оно не импортировано. Импортируются только изображения, хранящиеся в самом документе или в его папке.
import-diagnostic-image-not-ingested-missing = В «{ $path }» не удалось найти изображение «{ $detail }», поэтому оно не импортировано.
import-diagnostic-image-not-ingested-too-large = В «{ $path }» изображение «{ $detail }» больше, чем допускает проект, поэтому оно не импортировано.
import-diagnostic-image-not-ingested-unreadable = В «{ $path }» изображение «{ $detail }» не удалось прочитать как изображение, поэтому оно не импортировано.
import-diagnostic-image-not-ingested-heading = В «{ $path }» изображение «{ $detail }» стоит в заголовке. Каждый заголовок становится названием элемента подшивки, а в названии изображений не бывает, поэтому изображение не импортировано.
import-diagnostic-image-not-ingested-note = В «{ $path }» изображение «{ $detail }» стоит в сноске, концевой сноске или комментарии. Они сохраняют только слова, без изображений, поэтому оно не импортировано.
import-diagnostic-image-not-ingested-html = В «{ $path }» изображение «{ $detail }» записано в HTML, а импорт пропускает HTML, поэтому изображение тоже не импортировано. Запишите его как изображение Markdown, чтобы импортировать.
import-diagnostic-image-not-ingested-document-full = В «{ $path }» изображение «{ $detail }» не импортировано: все изображения этого документа вместе превышают то, что может перенести один импорт. Импортируйте документ по частям или вставьте остальные изображения вручную.
import-diagnostic-picture-first-frame-only = В «{ $path }» анимированное изображение «{ $detail }» импортируется только первым кадром. Изображения в проекте не анимируются.
import-diagnostic-duplicate-title = «{ $title }» встречается { $count ->
    [one] { $count } раз
    [few] { $count } раза
    [many] { $count } раз
   *[other] { $count } раза
}. Если вы уже импортировали эти файлы раньше, они будут продублированы.
import-diagnostic-heading-level-jump = «{ $title }» перескакивает с уровня заголовка { $from } на { $to }; элемент размещается на один уровень ниже родительского.
import-diagnostic-illegal-combination = «{ $title }» содержит прозу, но тип «{ $kind }» не может её хранить. Импорт приостановлен, пока вы не измените тип или не исключите элемент. Иначе не будет импортировано ничего.
import-diagnostic-tracked-changes-flattened = { $path } находился в процессе рецензирования: { $count ->
    [one] { $count } отслеживаемое изменение принято
    [few] { $count } отслеживаемых изменения приняты
    [many] { $count } отслеживаемых изменений приняты
   *[other] { $count } отслеживаемого изменения приняты
}, а удаления применены. Это окончательный текст. Чтобы сохранить какие-то свои формулировки, используйте «Сравнить» на последнем шаге.
import-diagnostic-tracked-changes-flattened-by = { $path } находился в процессе рецензирования: { $count ->
    [one] { $count } отслеживаемое изменение ({ $names }) принято
    [few] { $count } отслеживаемых изменения ({ $names }) приняты
    [many] { $count } отслеживаемых изменений ({ $names }) приняты
   *[other] { $count } отслеживаемого изменения ({ $names }) приняты
}, а удаления применены. Это окончательный текст. Чтобы сохранить какие-то свои формулировки, используйте «Сравнить» на последнем шаге.
import-diagnostic-text-box-dropped = { $count ->
    [one] В { $path } есть { $count } надпись (текстовое поле). Её текст находится вне основного потока документа, поэтому невозможно определить, где ему место в рукописи. Она не импортируется.
    [few] В { $path } есть { $count } надписи (текстовые поля). Их текст находится вне основного потока документа, поэтому невозможно определить, где ему место в рукописи. Они не импортируются.
    [many] В { $path } есть { $count } надписей (текстовых полей). Их текст находится вне основного потока документа, поэтому невозможно определить, где ему место в рукописи. Они не импортируются.
   *[other] В { $path } есть { $count } надписи (текстовые поля). Их текст находится вне основного потока документа, поэтому невозможно определить, где ему место в рукописи. Они не импортируются.
}
import-diagnostic-embedded-object-dropped = { $count ->
    [one] В { $path } есть { $count } внедрённый объект: диаграмма, формула или что-то подобное. В рукописи нет ничего, что могло бы его вместить.
    [few] В { $path } есть { $count } внедрённых объекта: диаграммы, формулы или что-то подобное. В рукописи нет ничего, что могло бы их вместить.
    [many] В { $path } есть { $count } внедрённых объектов: диаграммы, формулы или что-то подобное. В рукописи нет ничего, что могло бы их вместить.
   *[other] В { $path } есть { $count } внедрённого объекта: диаграммы, формулы или что-то подобное. В рукописи нет ничего, что могло бы их вместить.
}
import-diagnostic-field-flattened = { $count ->
    [one] В { $path } есть { $count } поле: номер страницы, перекрёстная ссылка, дата. Оно сохраняет последний показанный текст и больше не будет обновляться.
    [few] В { $path } есть { $count } поля: номера страниц, перекрёстные ссылки, даты. Каждое сохраняет последний показанный текст и больше не будет обновляться.
    [many] В { $path } есть { $count } полей: номера страниц, перекрёстные ссылки, даты. Каждое сохраняет последний показанный текст и больше не будет обновляться.
   *[other] В { $path } есть { $count } поля: номера страниц, перекрёстные ссылки, даты. Каждое сохраняет последний показанный текст и больше не будет обновляться.
}
import-diagnostic-unknown-style-level = { $path } использует стиль «{ $detail }», который похож на заголовок, но не указывает уровень. Такие абзацы импортируются как проза, а не размещаются на угаданной глубине.
import-diagnostic-comment-unanchored = Комментарий «{ $detail }» в { $path } не удалось привязать к словам, к которым он относился. Он сохранён на своём элементе, где его можно переместить.
import-diagnostic-comment-not-carried = Комментарий «{ $detail }» в { $path } не импортирован: в файле нет текста, к которому его можно было бы прикрепить. Если он вам нужен, скопируйте его из файла.
import-diagnostic-comment-replies-flattened = { $count ->
    [one] { $count } ответ в { $path } относится к комментарию, которого нет в файле, поэтому он импортируется как отдельный комментарий.
    [few] { $count } ответа в { $path } относятся к комментарию, которого нет в файле, поэтому они импортируются как отдельные комментарии.
    [many] { $count } ответов в { $path } относятся к комментарию, которого нет в файле, поэтому они импортируются как отдельные комментарии.
   *[other] { $count } ответа в { $path } относятся к комментарию, которого нет в файле, поэтому они импортируются как отдельные комментарии.
}
import-diagnostic-prose-not-verbatim = { $count ->
    [one] { $count } абзац в { $path } не удалось перенести с форматированием в точности как в оригинале, поэтому он импортируется как обычный текст. Сравните его с оригиналом.
    [few] { $count } абзаца в { $path } не удалось перенести с форматированием в точности как в оригинале, поэтому они импортируются как обычный текст. Сравните их с оригиналом.
    [many] { $count } абзацев в { $path } не удалось перенести с форматированием в точности как в оригинале, поэтому они импортируются как обычный текст. Сравните их с оригиналом.
   *[other] { $count } абзаца в { $path } не удалось перенести с форматированием в точности как в оригинале, поэтому они импортируются как обычный текст. Сравните их с оригиналом.
}
import-diagnostic-styled-spaces-not-carried = { $count ->
    [one] В { $path } { $count } фрагмент из одних пробелов был подчёркнут или зачёркнут — например, строка для заполнения. Skribisto не хранит такие линии на одних пробелах, поэтому фрагмент импортируется как обычные пробелы, а в начале или конце абзаца не импортируется вовсе.
    [few] В { $path } { $count } фрагмента из одних пробелов были подчёркнуты или зачёркнуты — например, строки для заполнения. Skribisto не хранит такие линии на одних пробелах, поэтому фрагменты импортируются как обычные пробелы, а в начале или конце абзаца не импортируются вовсе.
    [many] В { $path } { $count } фрагментов из одних пробелов были подчёркнуты или зачёркнуты — например, строки для заполнения. Skribisto не хранит такие линии на одних пробелах, поэтому фрагменты импортируются как обычные пробелы, а в начале или конце абзаца не импортируются вовсе.
   *[other] В { $path } { $count } фрагмента из одних пробелов были подчёркнуты или зачёркнуты — например, строки для заполнения. Skribisto не хранит такие линии на одних пробелах, поэтому фрагменты импортируются как обычные пробелы, а в начале или конце абзаца не импортируются вовсе.
}
import-diagnostic-list-nesting-flattened = { $count ->
    [one] { $count } пункт списка в { $path } вложен глубже { $limit } уровней. Он помещается на уровень { $limit } — самый глубокий, который поддерживает Skribisto, — со всеми словами и форматированием.
    [few] { $count } пункта списка в { $path } вложены глубже { $limit } уровней. Они помещаются на уровень { $limit } — самый глубокий, который поддерживает Skribisto, — со всеми словами и форматированием.
    [many] { $count } пунктов списка в { $path } вложены глубже { $limit } уровней. Они помещаются на уровень { $limit } — самый глубокий, который поддерживает Skribisto, — со всеми словами и форматированием.
   *[other] { $count } пункта списка в { $path } вложены глубже { $limit } уровней. Они помещаются на уровень { $limit } — самый глубокий, который поддерживает Skribisto, — со всеми словами и форматированием.
}
import-diagnostic-table-read-as-paragraphs = { $count ->
    [one] В { $count } таблице в { $path } одна строка намного шире остальных, и её нельзя разложить в сетку. Её ячейки импортируются как абзацы, один за другим, со словами и форматированием.
    [few] В { $count } таблицах в { $path } одна строка намного шире остальных, и их нельзя разложить в сетку. Их ячейки импортируются как абзацы, один за другим, со словами и форматированием.
    [many] В { $count } таблицах в { $path } одна строка намного шире остальных, и их нельзя разложить в сетку. Их ячейки импортируются как абзацы, один за другим, со словами и форматированием.
   *[other] В { $count } таблицах в { $path } одна строка намного шире остальных, и их нельзя разложить в сетку. Их ячейки импортируются как абзацы, один за другим, со словами и форматированием.
}
import-diagnostic-spaces-shortened = { $count ->
    [one] { $count } серия пробелов в { $path } длиннее { $limit } пробелов и сокращена до { $limit }. Все слова вокруг неё на месте.
    [few] { $count } серии пробелов в { $path } длиннее { $limit } пробелов и сокращены до { $limit }. Все слова вокруг них на месте.
    [many] { $count } серий пробелов в { $path } длиннее { $limit } пробелов и сокращены до { $limit }. Все слова вокруг них на месте.
   *[other] { $count } серии пробелов в { $path } длиннее { $limit } пробелов и сокращены до { $limit }. Все слова вокруг них на месте.
}
import-diagnostic-epigraph-not-carried = Перед «{ $title }» стоит эпиграф, но тип «{ $kind }» не может его хранить. Цитата сохранена в начале текста элемента.
import-diagnostic-epigraph-placement-ambiguous = Эпиграф находится между «{ $title }» и «{ $below }» и может относиться к любому из них. Он отнесён к «{ $title }», так как обычно эпиграф ставится именно там.
import-document-diagnostics = { $errors ->
    [0] { $warnings ->
            [one] { $warnings } замечание
            [few] { $warnings } замечания
            [many] { $warnings } замечаний
           *[other] { $warnings } замечания
        }
   *[other] { $errors ->
            [one] { $errors } файл не удалось прочитать
            [few] { $errors } файла не удалось прочитать
            [many] { $errors } файлов не удалось прочитать
           *[other] { $errors } файла не удалось прочитать
        }
}
import-document-diagnostics-none = Замечаний нет.

versions-title = Версии
versions-scope-synopsis = Синопсис
versions-scope-prose = Текст
versions-loading = Просмотр ваших резервных копий…
versions-empty = Более ранних версий пока нет
versions-error = Не удалось прочитать резервные копии. Ничего не потеряно, но список может быть неполным
versions-did-not-exist = { $date } ещё не существовало
versions-deleted-after = Удалено через некоторое время после { $date }
versions-unreadable = { $count ->
    [one] { $count } резервную копию не удалось прочитать
    [few] { $count } резервные копии не удалось прочитать
    [many] { $count } резервных копий не удалось прочитать
   *[other] { $count } резервной копии не удалось прочитать
}
versions-thinned = { $count ->
    [one] Со временем старые версии прореживаются. Собственная история проекта уже не содержит { $count } более раннего состояния этого текста.
    [few] Со временем старые версии прореживаются. Собственная история проекта уже не содержит { $count } более ранних состояний этого текста.
    [many] Со временем старые версии прореживаются. Собственная история проекта уже не содержит { $count } более ранних состояний этого текста.
   *[other] Со временем старые версии прореживаются. Собственная история проекта уже не содержит { $count } более раннего состояния этого текста.
}
versions-source-backup = Из резервной копии
versions-source-project = Из собственной истории проекта
versions-list-caption = Одна запись на изменение, а не на резервную копию
versions-pick-a-version = Выберите версию, чтобы увидеть, что изменилось
versions-earliest = Самая ранняя сохранившаяся версия. Сравнивать её не с чем.
versions-no-change = В этой части элемента ничего не изменилось
versions-formatting-only = Здесь изменилось только форматирование. Слова те же
versions-show-unchanged = Показать неизменённое
versions-hide-unchanged = Скрыть неизменённое
versions-next-change = Следующее изменение
versions-near = рядом с «{ $text }»
versions-words-added = { $count ->
    [one] Добавлено { $count } слово
    [few] Добавлено { $count } слова
    [many] Добавлено { $count } слов
   *[other] Добавлено { $count } слова
}
versions-words-removed = { $count ->
    [one] Удалено { $count } слово
    [few] Удалено { $count } слова
    [many] Удалено { $count } слов
   *[other] Удалено { $count } слова
}
versions-blocks-moved = { $count ->
    [one] Перемещён { $count } абзац
    [few] Перемещено { $count } абзаца
    [many] Перемещено { $count } абзацев
   *[other] Перемещено { $count } абзаца
}
versions-pin = Закрепить резервную копию, из которой взята эта версия. Автоматическая очистка никогда её не удалит
versions-unpin = Открепить резервную копию, из которой взята эта версия. Автоматическая очистка снова сможет её удалить
versions-pinned-only = Показывать только закреплённые версии
versions-pin-note = Закрепить можно только версии из резервных копий. Закрепление сохраняет файл, а собственная история проекта файлом не является.
versions-pinned-empty = Здесь пока ничего не закреплено
versions-pinned-empty-log = Здесь пока ничего не закреплено. Закрепить можно только версии из резервных копий. Закрепление сохраняет файл, а собственная история проекта файлом не является.
versions-range-filter = Показывать только версии между двумя датами
versions-filtered-empty = Ни одна версия не соответствует фильтрам, которые вы установили
versions-clear-filters = Сбросить фильтры
versions-last-30-days = Последние 30 дней
versions-restore-button = Восстановить эту версию
versions-restore-confirm-title = Заменить этот текст версией от { $date }?
versions-restore-confirm-text = Текущий текст будет заменён текстом, который был у этого элемента { $date }.
versions-restore-confirm-with-comments = Текущий текст будет заменён текстом, который был у этого элемента { $date }. { $count ->
    [one] { $count } комментарий привязан к текущему тексту и может потерять привязку.
    [few] { $count } комментария привязаны к текущему тексту и могут потерять привязку.
    [many] { $count } комментариев привязаны к текущему тексту и могут потерять привязку.
   *[other] { $count } комментария привязаны к текущему тексту и могут потерять привязку.
}
versions-restore-confirm-undo-note = Сначала создаётся резервная копия, а Ctrl+Z отменяет это действие за один шаг.
versions-restored-toast = Восстановлена версия от { $date }
versions-undo = Отменить
versions-restore-row-gone = Этого элемента больше нет в проекте
versions-restore-no-home = С тех пор тип элемента изменился, и старому тексту в нём некуда деться
versions-restore-no-safety-copy = Страховочная резервная копия не создана, поэтому ничего не изменено
versions-restore-failed = Восстановление не удалось: { $error }
versions-restore-backup-busy = Резервное копирование уже выполняется. Повторите попытку чуть позже
versions-restore-in-backup-file = Вы просматриваете файл резервной копии; чтобы восстановить в проект, откройте сам проект
versions-restore-no-project = Нет открытого проекта
versions-recreate-button = Вернуть…
versions-recreate-picker-title = Куда его поместить?
versions-recreate-picker-empty = В этом проекте нет подшивки, куда его можно поместить
versions-recreate-picker-confirm = Вернуть сюда
versions-recreate-picker-cancel = Отмена
versions-recreate-untitled = этот элемент
versions-recreate-confirm-title = Вернуть «{ $item }»?
versions-recreate-confirm-text = Он будет добавлен в { $destination } с текстом, который был у него { $date }.
versions-recreate-confirm-undo-note = Кнопка «Отменить» в следующем сообщении сразу уберёт его обратно.
versions-recreated-toast = «{ $item }» снова в вашем проекте
versions-recreated-partial-toast = { $count ->
    [one] «{ $item }» возвращён, но { $count } из его текстов не удалось прочитать
    [few] «{ $item }» возвращён, но { $count } из его текстов не удалось прочитать
    [many] «{ $item }» возвращён, но { $count } из его текстов не удалось прочитать
   *[other] «{ $item }» возвращён, но { $count } из его текстов не удалось прочитать
}
versions-recreate-already-here = Этот элемент уже есть в вашем проекте
versions-recreate-no-destination = Выберите в подшивке место, куда его поместить
versions-recreate-unreadable = Эту резервную копию не удалось прочитать, поэтому ничего не добавлено
versions-recreate-failed = Не удалось вернуть: { $error }
versions-changed-percent = Изменено: { $percent }%
versions-hidden-paragraphs = { $count ->
    [one] … { $count } неизменённый абзац …
    [few] … { $count } неизменённых абзаца …
    [many] … { $count } неизменённых абзацев …
   *[other] … { $count } неизменённого абзаца …
}

timeline-title = Вернуться в прошлое
timeline-coverage = { $count ->
    [one] Записана { $count } версия, начиная с { $oldest }
    [few] Записано { $count } версии, начиная с { $oldest }
    [many] Записано { $count } версий, начиная с { $oldest }
   *[other] Записано { $count } версии, начиная с { $oldest }
}
timeline-loading = Просмотр истории вашего проекта…
timeline-empty = Ни одной версии этого проекта пока не записано
timeline-no-changes = С тех пор ничего не изменилось
timeline-no-text-changes = С тех пор текст не изменился
timeline-slider-label = Записанная версия
timeline-series-name = Размер проекта
timeline-bars-caption = Каждый столбец — записанная версия, высотой с тогдашний объём проекта. Выделен тот, который вы просматриваете.
timeline-bars-caption-periods = Версий слишком много, чтобы показать их по одной, поэтому каждый столбец — { $period }, высотой с объём проекта на конец этого периода.
timeline-unit-hour = час
timeline-unit-day = день
timeline-unit-week = неделя
timeline-unit-month = месяц
timeline-range-filter = Показывать только версии между двумя датами
timeline-range-empty = За эти даты не записано ни одной версии
timeline-open-period = Открыть этот период
timeline-show-all = Показать всю историю
timeline-changed-since = { $count ->
    [one] { $count } элемент отличается между { $date } и текущим проектом
    [few] { $count } элемента отличаются между { $date } и текущим проектом
    [many] { $count } элементов отличаются между { $date } и текущим проектом
   *[other] { $count } элемента отличаются между { $date } и текущим проектом
}
timeline-kind-added = Написано позже
timeline-kind-removed = Больше нет в проекте
timeline-kind-changed = Изменено позже
timeline-kind-moved = Перемещено позже
timeline-not-yet-written = На тот момент этого ещё не существовало
timeline-no-text-of-its-own = У этого элемента нет собственного текста. Это заголовок для того, что внутри него
timeline-reader-close = Закрыть
timeline-reader-stamp = В состоянии на { $date }
timeline-reader-loading = Открытие записанной версии…
timeline-reader-unreadable = Не удалось прочитать эту запись. Резервная копия могла быть перемещена, удалена или находиться на неподключённом диске.
timeline-reader-compared = В состоянии на { $date } в сравнении с текущим текстом
timeline-reader-diff-legend = Зачёркнутое с тех пор удалено; подчёркнутое добавлено.
timeline-reader-view-label = Что показать
timeline-reader-view-diff = Изменения
timeline-reader-view-text = Текст
timeline-reader-deleted = Этого больше нет в вашем проекте. Здесь можно прочитать текст и скопировать его.
timeline-prose-only-record = Эта точка взята из собственной истории проекта, которая хранит только текст, поэтому показать, что с тех пор было удалено или перемещено, нельзя. Для этого выберите резервную копию.

settings-sec-extensions = Расширения

settings-group-games-forward = Только вперёд
settings-games-forward-toggle = Играть в «Только вперёд»
settings-games-forward-hint =
    Пока вы играете, ничего написанного нельзя забрать обратно: Backspace, Delete,
    вырезание, перетаскивание и отмена отключены в областях, выбранных ниже.
    Можно по-прежнему печатать, вставлять, форматировать и свободно перемещаться. Черновик только растёт.
settings-games-session-warning =
    Этот выбор действует только в текущем сеансе и никогда не сохраняется. Закрытие проекта
    или выход из Skribisto всегда завершает игру, и всё, что вы написали во время игры,
    в тот же момент снова можно отменить.
settings-group-games-scope = Где действует
settings-games-in-prose = Проза рукописи
settings-games-in-synopsis = Синопсисы
settings-games-scope-hint =
    Комментарии, сноски и названия никогда не блокируются: именно в них вы записываете
    исправление, которое только что запретили себе вносить.
settings-games-inert-warning =
    Режим «Только вперёд» включён, но ни к чему не применяется. Отметьте выше хотя бы одну область,
    иначе при письме ничего не изменится.

games-title = Писательские игры
games-forward-name = Только вперёд
games-forward-blurb =
    Пишите черновик, ничего не забирая обратно. Пока вы играете, удаление, вырезание и отмена
    отключены. Пишите следующее предложение вместо того, чтобы исправлять предыдущее.
games-forward-playing = Игра идёт: удаление отключено
games-forward-idle = Игра не идёт
games-session-note = Завершается при закрытии проекта.
games-scope-prose-and-synopsis = Применяется к вашей прозе и синопсисам.
games-scope-prose = Применяется к вашей прозе.
games-scope-synopsis = Применяется к вашим синопсисам.
games-scope-nothing = Пока ни к чему не применяется. Выберите область в настройках.
games-settings-link = Настройки писательских игр…
statusbar-games-forward = Только вперёд
statusbar-games-forward-tooltip =
    Режим «Только вперёд» включён: пока вы пишете черновик, удаление, вырезание и отмена
    отключены. Щёлкните, чтобы закончить игру.
goal-progress-words = { $g ->
    [one] { $count } из { $goal } слова
    [few] { $count } из { $goal } слов
    [many] { $count } из { $goal } слов
   *[other] { $count } из { $goal } слова
}
goal-progress-characters = { $g ->
    [one] { $count } из { $goal } знака
    [few] { $count } из { $goal } знаков
    [many] { $count } из { $goal } знаков
   *[other] { $count } из { $goal } знака
}
goal-count-words = { $n ->
    [one] { $count } слово
    [few] { $count } слова
    [many] { $count } слов
   *[other] { $count } слова
}

goal-count-characters = { $n ->
    [one] { $count } знак
    [few] { $count } знака
    [many] { $count } знаков
   *[other] { $count } знака
}

goal-unit-words = Слова
goal-unit-characters = Знаки

inspector-goal = Цель
inspector-goal-none = Без цели

overview-col-goal = Цель
overview-excluded-from-export = Исключено из экспорта, поэтому не учитывается ни в каких итогах

goal-subtree-total-words = { $count ->
    [one] { $items } цель внутри в сумме, слов: { $words }
    [few] { $items } цели внутри в сумме, слов: { $words }
    [many] { $items } целей внутри в сумме, слов: { $words }
   *[other] { $items } цели внутри в сумме, слов: { $words }
}
goal-subtree-total-characters = { $count ->
    [one] { $items } цель внутри в сумме, знаков: { $words }
    [few] { $items } цели внутри в сумме, знаков: { $words }
    [many] { $items } целей внутри в сумме, знаков: { $words }
   *[other] { $items } цели внутри в сумме, знаков: { $words }
}

new-work-goal-unit = Единица подсчёта
new-work-goal-unit-hint = Используется для всех целей по словам или знакам в этом проекте. Можно изменить позже.

settings-group-goal-unit = Цели
settings-goal-unit-switch-title = Сменить единицу подсчёта
settings-goal-unit-switch-text = Цели и вехи этого проекта были заданы в единицах «{ $from }». Переключение на «{ $to }» их не пересчитывает, поэтому каждое существующее число теперь будет считаться в единицах «{ $to }».
settings-goal-unit-switch-informative = Ничего не теряется. Переключитесь обратно в любой момент, чтобы вернуть прежнее значение, а затем обновите цели, которые хотите сохранить.
settings-goal-unit-switch-confirm = Переключить

goal-distribute-action = Распределить…
distribute-title = Распределить цель
distribute-empty = Внутри нет ничего, между чем можно распределить цель.
distribute-weight = Способ распределения
distribute-weight-length = По объёму
distribute-weight-rows = По элементам
distribute-weight-even = Поровну
distribute-overwrite = Заменить цели, которые уже установлены
distribute-col-item = Элемент
distribute-col-current = Сейчас
distribute-col-proposed = После
distribute-total = В сумме { $total } при цели { $goal }.
distribute-over-budget = Цели, уже заданные внутри, превышают собственную цель этого контейнера на { $over }. Увеличьте его цель, уменьшите их цели или замените их.
distribute-apply = Распределить
distribute-cancel = Отмена

milestone-target-gone = Цель удалена
milestone-no-target = Без цели
milestone-add = Добавить
milestone-add-label = Чего нужно достичь

pace-summary-title = Положение дел с книгой
pace-summary-remaining = Осталось: { $words }
pace-summary-open = Открыть план
pace-summary-close = Закрыть
pace-summary-dont-show = Не показывать при открытии
pace-summary-menu = План работы…

menu-help-topics = &Темы справки
menu-help-shortcuts = Сочетания &клавиш…
menu-help-website = Skribisto в &Интернете
menu-help-report = &Сообщить о проблеме…
menu-help-check-updates = Проверить о&бновления
menu-command-palette = &Палитра команд…

learn-help-topics = Темы справки
learn-shortcuts = Сочетания клавиш
learn-website = Skribisto в Интернете

help-window-title = Справка
help-filter-topics = Фильтр тем
help-no-matching-topic = Подходящих тем нет.
help-topic-missing = Эта тема больше не доступна.
help-back = Назад
help-not-translated = Эта страница пока не переведена и отображается на английском языке.

help-section-getting-started = Начало работы
help-section-writing = Письмо
help-section-reviewing = Редактура
help-section-exchanging = Ввоз и вывоз текста
help-section-keeping = Сохранность вашей работы
help-section-extensions = Расширения

help-topic-getting-started = Ваш первый проект
help-topic-writing-model = Как складывается книга
help-topic-drafts-and-old-versions = Как сохранить старый черновик
help-topic-goals-and-pace = Цели и темп
help-topic-comments = Комментарии
help-topic-round-trip = Отправка книги читателю
help-topic-export = Экспорт
help-topic-import-documents = Импорт документов
help-topic-import-projects = Перенос целого проекта
help-topic-import-scrivener = Переход с Scrivener
help-topic-backups-and-versions = Резервные копии и версии

help-shortcuts-title = Сочетания клавиш
help-shortcuts-filter = Фильтр сочетаний клавиш
help-shortcuts-no-matches = Подходящих сочетаний нет.
help-shortcuts-rebind = Изменить сочетания клавиш…
help-shortcuts-close = Закрыть

command-palette-placeholder = Введите команду

shortcut-name-binder-duplicate = Дублировать
shortcut-name-comments-add = Добавить комментарий
shortcut-name-comments-add-paragraph = Комментировать абзац
shortcut-name-spellcheck-toggle = Проверять орфографию
shortcut-name-editor-tab-close = Закрыть вкладку
shortcut-name-editor-tab-pin = Закрепить или открепить вкладку
shortcut-name-editor-save = Сохранить
shortcut-name-work-export = Экспорт…
shortcut-name-work-new = Новый проект
shortcut-name-work-open = Открыть проект
shortcut-name-window-new = Новое окно
shortcut-name-work-close = Закрыть проект
shortcut-name-work-import-scrivener = Импортировать проект Scrivener…
shortcut-name-work-import-scrivener-report = Показать отчёт об импорте из Scrivener
shortcut-name-app-settings = Настройки
shortcut-name-settings-close = Закрыть настройки
shortcut-name-app-quit = Выход
shortcut-name-editor-insert-footnote = Вставить сноску
shortcut-name-format-scene-break = Вставить разрыв сцены
shortcut-name-format-major-scene-break = Вставить большой разрыв сцены
shortcut-name-format-link = Вставить ссылку
shortcut-name-format-mark-nbsp = Вставить неразрывный пробел
shortcut-name-format-mark-nbhyphen = Вставить неразрывный дефис
shortcut-name-format-mark-nnbsp = Вставить узкий неразрывный пробел
shortcut-name-format-mark-zwsp = Вставить пробел нулевой ширины
shortcut-name-go-next = Далее
shortcut-name-go-prev = Назад
shortcut-name-go-to = Перейти к
shortcut-name-outline-toggle = Показать или скрыть структуру
shortcut-name-preview-toggle = Показать или скрыть предпросмотр
shortcut-name-view-fullscreen = Полноэкранный режим
shortcut-name-view-focus-mode = Режим без отвлечений
shortcut-name-editor-size-increase = Увеличить размер текста
shortcut-name-editor-size-decrease = Уменьшить размер текста
shortcut-name-editor-size-reset = Сбросить размер текста
shortcut-name-editor-find = Найти
shortcut-name-editor-replace = Заменить
shortcut-name-editor-find-next = Следующее совпадение
shortcut-name-editor-find-prev = Предыдущее совпадение
shortcut-name-search-show = Поиск в проекте
shortcut-name-search-replace = Заменить в проекте
shortcut-name-outline-open-to-side = Открыть сбоку
shortcut-name-help-topics = Темы справки
shortcut-name-help-close = Закрыть справку
shortcut-name-help-shortcuts = Сочетания клавиш
shortcut-name-help-website = Skribisto в Интернете
shortcut-name-help-report = Сообщить о проблеме
shortcut-name-help-check-updates = Проверить обновления
update-check-behind = Доступна версия { $version }.
update-check-current = У вас самая новая версия.
update-check-failed = Не удалось проверить наличие новой версии. { $error }
shortcut-name-command-palette = Палитра команд
help-section-reference = Что есть что

help-concept-scene-break = Разрыв сцены
help-concept-major-scene-break = Большой разрыв сцены
help-concept-find-in-prose = Поиск в тексте
help-concept-story-bible = Библия истории
help-concept-goal-unit = Слова или знаки
help-concept-goal-progress = Прогресс
help-concept-manuscript-words = Что считается рукописью
help-concept-exportable = Исключение из экспорта
help-concept-distribute = Распределение цели
help-concept-subtree-total = Сумма целей внутри
help-concept-milestone = Веха
help-concept-pace-plan = План темпа

help-concept-tag = Тег
help-concept-label = Метка
help-concept-point-of-view = Точка зрения
help-concept-epigraph = Эпиграф
help-concept-footnote = Сноска
help-concept-chapter-mode = Форма главы
help-concept-comment = Комментарий
help-concept-backup = Резервная копия
help-concept-version = Версия
help-concept-trash = Корзина
help-concept-spellcheck = Проверка орфографии
help-concept-search-replace = Поиск и замена
help-concept-note-template = Шаблон заметки
help-concept-story-bible-entry = Запись библии истории
help-concept-text-replacement = Автозамена
help-concept-smart-punctuation = Умная пунктуация
help-concept-export-style = Стиль экспорта
help-concept-round-trip-marks = Метки обратного импорта

epigraph-mark-attribution = Строка источника
epigraph-mark-attribution-tip = Пометить строку, где стоит курсор, как источник цитаты, чтобы она печаталась как подпись автора
margin-lane-name = Пометки на полях
margin-lane-provider-comments = Комментарии
margin-lane-provider-story-bible = Запись библии истории
margin-lane-provider-story-bible-hint = На странице «В прозе» заметки — места, где упоминается эта запись, и сцены, рассказанные её глазами. Также выделяет эти имена в самой прозе.
margin-lane-mark-point-of-view = Рассказано отсюда
margin-lane-mark-named-here = Упоминается здесь
margin-lane-provider-comments-hint = Где к тексту прикреплён комментарий
margin-lane-provider-search = Результаты поиска
margin-lane-provider-search-hint = Все совпадения по последнему поиску, где бы они ни находились в документе
margin-lane-provider-boundaries = Где начинается каждый документ
margin-lane-provider-boundaries-hint = Черта в начале каждой сцены потока, чтобы было видно, где вы находитесь
margin-lane-provider-spelling = Орфография
margin-lane-provider-spelling-hint = Где проверка орфографии пометила слово. Выключено, пока вы не попросите: это пометки мнения машины о вашей прозе
margin-lane-spelling = { $word }, возможная опечатка
margin-lane-search-hit = { $text }, совпадение { $index } из { $total }
margin-lane-search-current = { $text }, совпадение { $index } из { $total }, текущее
margin-lane-boundary = Начало: { $title }
margin-lane-boundary-untitled = документ без названия
settings-page-margin-lane = Пометки на полях
settings-desc-margin-lane = Полоса рядом с полосой прокрутки и то, что на ней показано
settings-margin-lane-enabled = Показывать полосу пометок на полях
settings-margin-lane-enabled-hint = Тот же переключатель, что и Вид ▸ Пометки на полях.
settings-margin-lane-enabled-more = Она никогда не говорит, что что-то не так: она показывает, где что находится, и оставляет выводы вам.
settings-group-margin-lane-marks = Что она отмечает
settings-margin-lane-no-providers = Пока нечего отмечать на полосе.
settings-group-margin-lane-texture = Текстура диалога
settings-margin-lane-texture = Показать текстуру диалога
settings-margin-lane-texture-hint = По одному столбику на абзац: его длина и доля прямой речи в нём.
settings-margin-lane-texture-more = Для языков без проверенных правил оформления диалогов не измеряется; там столбик показывает только длину.
settings-group-margin-lane-surfaces = Где показывается
settings-margin-lane-surface-editor = Текстовый редактор
settings-margin-lane-surface-stream = Потоки
settings-margin-lane-surface-search-preview = Предпросмотр поиска
menu-margin-lane = По&метки на полях

status-preset-drafting = Черновая работа
status-preset-passes = Этапы редактуры
status-preset-plume = Plume Creator
status-todo = Предстоит
status-draft = Черновик
status-revised = Исправлено
status-final = Финальная версия
status-outline = План
status-first-edit = 1-я редактура
status-second-edit = 2-я редактура
status-done = Готово
status-plume-draft-1 = 1-й черновик
status-plume-draft-2 = 2-й черновик
status-plume-draft-3 = 3-й черновик
status-plume-edit-1 = 1-я редактура
status-plume-edit-2 = 2-я редактура
status-plume-edit-3 = 3-я редактура
status-plume-proofread = Вычитано
status-plume-finished = Завершено
new-work-statuses = Рабочий процесс
new-work-statuses-hint = Этапы, через которые проходит сцена. Их можно в любой момент переименовать, переупорядочить или дополнить.
status-none = Нет статуса
inspector-status = Статус
overview-col-status = Статус
overview-status-mixed = вложенные элементы расходятся
help-concept-status = Статус
status-completion-title = Положение дел с книгой
status-completion-headline = Завершено сцен: { $done } из { $total }
status-completion-empty = Сцен пока нет. Здесь читается рукопись, поэтому данные появятся по мере письма.
status-completion-open = Положение дел с книгой…

settings-page-statuses = Статусы
settings-desc-statuses = Переименование, порядок, добавление и удаление этапов, через которые проходит сцена
settings-statuses-add = Добавить статус
settings-statuses-add-placeholder = Название нового статуса
settings-statuses-desc =
    Этапы, через которые проходит сцена, по порядку: от наименее готового вверху до самого
    готового внизу. Именно этот порядок означает «менее готово, чем» во всём приложении,
    поэтому расположите их так, как на самом деле идёт ваша работа.
settings-statuses-apply-preset = Применить набор…
settings-statuses-preset-applied = { $added ->
    [one] Добавлен { $added } статус
    [few] Добавлено { $added } статуса
    [many] Добавлено { $added } статусов
   *[other] Добавлено { $added } статуса
}
settings-statuses-preset-refused = В этом проекте уже есть шкала статусов. Наборы заполняют только пустую шкалу. Сначала удалите ненужные ступени.
settings-statuses-duplicate = «{ $name }» уже есть в этой шкале
settings-statuses-added = Добавлено «{ $name }»
settings-statuses-move-up = Переместить вверх (менее готово)
settings-statuses-move-down = Переместить вниз (более готово)
settings-statuses-details-placeholder = Что означает этот этап (необязательно)
settings-statuses-delete = Удалить «{ $name }»
settings-statuses-delete-in-use = { $count ->
    [one] Удалить «{ $name }»: { $count } элемент с этим статусом потеряет его
    [few] Удалить «{ $name }»: { $count } элемента с этим статусом потеряют его
    [many] Удалить «{ $name }»: { $count } элементов с этим статусом потеряют его
   *[other] Удалить «{ $name }»: { $count } элемента с этим статусом потеряют его
}
settings-statuses-deleted = Удалено «{ $name }»
settings-statuses-deleted-in-use = { $count ->
    [one] Удалено «{ $name }». У { $count } элемента больше нет статуса.
    [few] Удалено «{ $name }». У { $count } элементов больше нет статуса.
    [many] Удалено «{ $name }». У { $count } элементов больше нет статуса.
   *[other] Удалено «{ $name }». У { $count } элемента больше нет статуса.
}
settings-statuses-empty-title = Пока нет рабочего процесса
settings-statuses-empty-body = Статус показывает, насколько готова сцена. Добавьте статус выше или начните с готового набора.
status-category-planned = Запланировано
status-category-drafting = Пишется
status-category-needs-work = Требует доработки
status-category-revised = Исправлено
status-category-final = Финал

## Long-operation progress (shared::progress_text). The backend reports phase
## codes it cannot translate; these are their labels. `progress-percent` is the
## toast body on its own, `progress-percent-with-label` with the phase after it.
progress-percent = { $percent }%
progress-percent-with-label = { $percent }% · { $label }
progress-import-opening = Открытие проекта…
progress-import-outline = Чтение структуры…
progress-import-converting = Преобразование глав и сцен…
progress-import-writing = Запись файла .skrib…
progress-import-done = Готово
progress-import-structure = Определение структуры…
progress-export-start = Запуск…
progress-export-writing = Запись…
progress-project-reading = Чтение проекта…
# Settings: the open project's section title, "Work: <its title>".
settings-sec-work-titled = Проект: { $title }

## Project-import notices (shared::import_warnings::notices): one line of the
## Plume / Manuskript warnings list each. $file, $folder, $title, $name, $field
## and $error are data (file names, titles, a parser's own message) and stay as
## they are. $container is "single-file" or "folder"; $kind is "scene", "note",
## "synopsis" or "title".
import-notice-flattened-rich-texts = { $count ->
    [one] { $count } текст был вложен глубже, чем допускает проект Skribisto; слова сохранены как простой текст, без форматирования.
    [few] { $count } текста были вложены глубже, чем допускает проект Skribisto; слова сохранены как простой текст, без форматирования.
    [many] { $count } текстов были вложены глубже, чем допускает проект Skribisto; слова сохранены как простой текст, без форматирования.
   *[other] { $count } текста были вложены глубже, чем допускает проект Skribisto; слова сохранены как простой текст, без форматирования.
}
import-notice-free-text-badges = { $count ->
    [one] { $count } значок сцены похож на произвольный текст, а не на список статусов, и потому сохранён как метка элемента, а не как тег.
    [few] { $count } разных значка сцен похожи на произвольный текст, а не на список статусов, и потому сохранены как метки элементов, а не как теги.
    [many] { $count } разных значков сцен похожи на произвольный текст, а не на список статусов, и потому сохранены как метки элементов, а не как теги.
   *[other] { $count } разных значка сцен похожи на произвольный текст, а не на список статусов, и потому сохранены как метки элементов, а не как теги.
}
import-notice-separator-text-dropped = Разделитель «{ $name }» содержал текст, которому нет места в проекте Skribisto; этот текст не импортирован.
import-notice-separator-without-scene = Перед разделителем «{ $name }» в его главе не было сцены, к которой его можно было бы присоединить, поэтому он не импортирован.
import-notice-chapter-without-scenes = В главе «{ $name }» нет сцен, поэтому её разделители не импортированы.
import-notice-rich-text-unconverted = Часть форматированного текста не удалось преобразовать ({ $error }); она не импортирована.
import-notice-unresolved-cross-link = Не удалось перейти по ссылке на действующее лицо №{ $number }: такой записи нет или она была в корзине.
import-notice-read-copy = Прочитана копия этого проекта { $container ->
    [single-file] в виде одного файла
   *[folder] в виде папки
}.
import-notice-read-copy-dated = Прочитана копия этого проекта { $container ->
    [single-file] в виде одного файла
   *[folder] в виде папки
}, последнее изменение: { $day }.
import-notice-original-format = Этот проект сохранён в исходном формате Manuskript 2016 года. Он прочитан полностью; в нём ничего не изменено.
import-notice-newer-single-file-beside = Файл «{ $file }» новее этой папки ({ $file_day } против { $folder_day }). Когда проект переводят в однофайловый режим, Manuskript оставляет старую папку на месте, так что эта папка может быть заброшенной копией; если импортировать вместо неё файл .msk, будет прочитана более новая версия.
import-notice-pickle-skipped = Файл «{ $file }» не прочитан. Это файл pickle языка Python, а загрузка такого файла из проекта была уязвимостью в самом Manuskript (CVE-2021-35196); хранящиеся в нём настройки касаются только отображения, а не текста.
import-notice-file-unreadable = Не удалось прочитать «{ $file }» ({ $error }), файл пропущен; всё остальное импортировано.
import-notice-not-utf8 = «{ $file }» не является корректным текстом UTF-8; нечитаемые символы заменены, импорт не прерван.
import-notice-table-unreadable = Не удалось прочитать «{ $file }» ({ $error }); его содержимое не импортировано.
import-notice-outline-missing = Файл «{ $file }» отсутствует, поэтому в этом проекте нет рукописи для импорта.
import-notice-outline-unreadable = Не удалось прочитать «{ $file }» ({ $error }); рукопись не импортирована.
import-notice-named-none-row = «{ $file }» начинается с именованной строки («{ $name }») там, где Manuskript хранит свою пустую запись «none». Эта строка пропущена, чтобы следующие за ней строки сохранили номера, которые использует рукопись.
import-notice-unknown-field = В «{ $file }» есть поле, неизвестное этому импортёру: «{ $field }». Оно не импортировано.
import-notice-colorless-labels = { $count ->
    [one] В «{ $file }» { $count } строка не имеет цвета и импортирована без него; встретив такую строку, Manuskript прекращает загрузку проекта.
    [few] В «{ $file }» { $count } строки не имеют цвета и импортированы без него; встретив такую строку, Manuskript прекращает загрузку проекта.
    [many] В «{ $file }» { $count } строк не имеют цвета и импортированы без него; встретив такую строку, Manuskript прекращает загрузку проекта.
   *[other] В «{ $file }» { $count } строки не имеют цвета и импортированы без него; встретив такую строку, Manuskript прекращает загрузку проекта.
}
import-notice-settings-unreadable = Не удалось прочитать «{ $file }» ({ $error }); язык проекта не импортирован.
import-notice-revisions-unreadable = Не удалось прочитать «{ $file }» ({ $error }); история проекта не импортирована, но больше ничего не затронуто.
import-notice-foreign-files = { $count ->
    [one] В этом проекте есть { $count } файл, который не относится к формату Manuskript и не импортирован: { $files }.
    [few] В этом проекте есть { $count } файла, которые не относятся к формату Manuskript и не импортированы: { $files }.
    [many] В этом проекте есть { $count } файлов, которые не относятся к формату Manuskript и не импортированы: { $files }.
   *[other] В этом проекте есть { $count } файла, которые не относятся к формату Manuskript и не импортированы: { $files }.
}
import-notice-foreign-files-more = Файлы этого проекта, которые не относятся к формату Manuskript и не импортированы ({ $count }): { $files } и ещё { $more }.
import-notice-not-a-character-file = «{ $file }» не является файлом персонажа Manuskript и пропущен.
import-notice-not-a-text-item = «{ $file }» не является текстовым элементом Manuskript и не включён в структуру.
import-notice-missing-blank-line = В «{ $file }» после заголовка нет пустой строки. Последнее поле сохранено, хотя Manuskript его теряет.
import-notice-folder-without-metadata = В папке «{ $folder }» нет файла { $file }, поэтому она импортирована как обычная папка с именем каталога. Manuskript пропускает такой каталог вместе со всем содержимым.
import-notice-rows-given-ids = { $count ->
    [one] { $count } строка структуры не имела собственного ID и получила его. Manuskript отбрасывает такую строку, а если это папка, прекращает загрузку проекта.
    [few] { $count } строки структуры не имели собственного ID и получили его. Manuskript отбрасывает такую строку, а если это папка, прекращает загрузку проекта.
    [many] { $count } строк структуры не имели собственного ID и получили его. Manuskript отбрасывает такую строку, а если это папка, прекращает загрузку проекта.
   *[other] { $count } строки структуры не имели собственного ID и получили его. Manuskript отбрасывает такую строку, а если это папка, прекращает загрузку проекта.
}
import-notice-plots-unreadable = Не удалось прочитать «{ $file }» ({ $error }); сюжетные линии не импортированы.
import-notice-world-unreadable = Не удалось прочитать «{ $file }» ({ $error }); записи о мире не импортированы.
import-notice-world-not-opml = «{ $file }» не является документом OPML; записи о мире не импортированы.
import-notice-text-flattened = Текст «{ $title }» вложен глубже, чем допускает проект Skribisto; его слова сохранены как простой текст, без форматирования.
import-notice-text-unconverted = Текст «{ $title }» не удалось преобразовать ({ $error }); он сохранён точно в том виде, в каком был написан.
import-notice-text-without-file-name = Для { $kind ->
    [scene] текста сцены
    [note] текста заметки
    [synopsis] синопсиса
   *[title] названия
} «{ $title }» не удалось подобрать имя файла, поэтому эта часть не импортирована.
import-notice-unknown-label = У «{ $title }» метка { $index }, которой нет в списке меток этого проекта. Элемент импортирован без метки.
import-notice-unknown-status = У «{ $title }» статус { $index }, которого нет в списке статусов этого проекта. Элемент импортирован без статуса.
import-notice-unknown-point-of-view = Повествование в «{ $title }» ведётся от лица персонажа, которого нет среди действующих лиц этого проекта. Точка зрения не задана.
import-notice-unresolved-references = Не удалось разрешить ссылки в тексте «{ $title }» ({ $error }); текст сохранён в преобразованном виде, вместе с маркерами ссылок.
import-notice-custom-icons = { $count ->
    [one] У { $count } строки был собственный значок. Skribisto даёт каждой строке значок её типа, поэтому значок не импортирован; больше ничего не изменилось.
    [few] У { $count } строк были собственные значки. Skribisto даёт каждой строке значок её типа, поэтому значки не импортированы; больше ничего не изменилось.
    [many] У { $count } строк были собственные значки. Skribisto даёт каждой строке значок её типа, поэтому значки не импортированы; больше ничего не изменилось.
   *[other] У { $count } строки были собственные значки. Skribisto даёт каждой строке значок её типа, поэтому значки не импортированы; больше ничего не изменилось.
}
import-notice-character-colors = { $count ->
    [one] У { $count } персонажа был цвет рядом с именем. Skribisto окрашивает теги, а не строки, поэтому цвет не импортирован; сам персонаж импортирован.
    [few] У { $count } персонажей был цвет рядом с именем. Skribisto окрашивает теги, а не строки, поэтому цвета не импортированы; сами персонажи импортированы все.
    [many] У { $count } персонажей был цвет рядом с именем. Skribisto окрашивает теги, а не строки, поэтому цвета не импортированы; сами персонажи импортированы все.
   *[other] У { $count } персонажа был цвет рядом с именем. Skribisto окрашивает теги, а не строки, поэтому цвета не импортированы; сами персонажи импортированы все.
}
import-notice-versions-kept = { $found ->
    [one] В этом проекте записана { $found } более ранняя версия
    [few] В этом проекте записаны { $found } более ранние версии
    [many] В этом проекте записано { $found } более ранних версий
   *[other] В этом проекте записано { $found } более ранней версии
}; { $kept ->
    [one] сохранена { $kept }
    [few] сохранены { $kept }
    [many] сохранено { $kept }
   *[other] сохранено { $kept }
}, с прореживанием по тому же принципу, что и собственная история Skribisto. Они находятся на панели «Версии».
import-notice-versions-unplaced = { $count ->
    [one] { $count } более ранняя версия относится к строке, которой больше нет в проекте, и не может быть размещена.
    [few] { $count } более ранние версии относятся к строке, которой больше нет в проекте, и их не удалось разместить.
    [many] { $count } более ранних версий относятся к строке, которой больше нет в проекте, и их не удалось разместить.
   *[other] { $count } более ранней версии относятся к строке, которой больше нет в проекте, и их не удалось разместить.
}

## Native file dialogs: the window title, and the label of the file-type filter.
dialog-open-work-title = Открыть проект Skribisto
dialog-save-as-file-title = Сохранить как один файл .skrib
dialog-save-as-folder-title = Выберите родительскую папку для проекта
dialog-manuskript-file-title = Выберите проект Manuskript
dialog-manuskript-folder-title = Выберите папку проекта Manuskript
dialog-plume-file-title = Выберите проект Plume Creator
dialog-filter-skrib = Проект Skribisto
dialog-filter-manuskript = Проект Manuskript
dialog-filter-plume = Проект Plume Creator
dialog-filter-hunspell-aff = Файл аффиксов Hunspell (.aff)
dialog-filter-hunspell-dic = Словарь Hunspell (.dic)
dialog-filter-export = Экспортированный файл
# Help ▸ Keyboard shortcuts / Settings ▸ Keymap / command palette.
shortcut-name-view-margin-lane = Показать или скрыть пометки на полях

## The built-in distraction-free themes' names (distraction_free::theme).
df-theme-paper = Бумага
df-theme-sepia = Сепия
df-theme-night = Ночь
df-theme-high-contrast = Высокая контрастность
df-theme-midnight = Полночь

## Welcome ▸ Examples: the line under a bundled example's title.
welcome-example-byline = Автор: { $author }

## Spoken by a screen reader only.
# Welcome window: the name of the section tab bar.
welcome-sections-a11y = Разделы окна приветствия
# A comment's anchored span in the editor: author, plain-text preview, replies.
comments-a11y-summary-author = { $author }: { $text }
comments-a11y-summary-replies = { $text } ({ $count ->
    [one] { $count } ответ
    [few] { $count } ответа
    [many] { $count } ответов
   *[other] { $count } ответа
})
comments-a11y-summary-author-replies = { $author }: { $text } ({ $count ->
    [one] { $count } ответ
    [few] { $count } ответа
    [many] { $count } ответов
   *[other] { $count } ответа
})
# Margin marks beside the text: one mark, several merged into one, and the
# lane's own name when nothing more specific was given.
margin-lane-a11y-mark = { $label }, { $percent }% от начала
margin-lane-a11y-marks = { $label }, пометок: { $count }, { $percent }% от начала
margin-lane-a11y-fallback = { $count ->
    [one] Пометки на полях, { $count } пометка
    [few] Пометки на полях, { $count } пометки
    [many] Пометки на полях, { $count } пометок
   *[other] Пометки на полях, { $count } пометки
}
# Export ▸ Choose: the row standing for a binder that has no name.
export-choose-unnamed-binder = Подшивка без названия
# The built-in export styles named in words (the national manuscript presets
# keep their own language's name).
export-style-neutral = Нейтральный
export-style-manuscript-shunn = Стандартная рукопись (Shunn)
export-style-ebook-clean = Чистая электронная книга

## Dates. Put the parts in the order your language writes a short date; the
## parts arrive already zero-padded where that matters.
# Comment cards, the outline card's Created/Modified.
date-time-short = { $day }.{ $month }.{ $year } { $hour }:{ $minute }
# Pace chart: a day on the axis, a milestone's date, a holiday.
pace-day-label = { $day }.{ $month }

## File sizes (shared::byte_size). $whole and $tenth are the two sides of the
## decimal mark: write your language's mark between them and its unit symbol.
byte-size-bytes = { $value } Б
byte-size-kb = { $value } КБ
byte-size-mb = { $whole },{ $tenth } МБ
byte-size-gb = { $whole },{ $tenth } ГБ
# A whole percentage (sliders, the Pace card, the text-size toast).
unit-percent = { $value }%
# A whole number of pixels (slider readouts).
unit-pixels = { $value } px
# Settings ▸ Paratext: a preset saved without a `name` line. `name` is the TOML
# key the writer types and stays as it is.
settings-paratext-error-no-name = У структуры должна быть строка `name`.
# Export: started with no project open (a menu raced a close).
export-error-no-project = Нет открытого проекта для экспорта.

shortcut-group-work = Проект
shortcut-group-edit = Правка
shortcut-group-view = Вид
shortcut-group-document = Документ
shortcut-group-format = Формат
shortcut-group-go = Переход
shortcut-group-tools = Инструменты
shortcut-group-help = Справка

## Manuskript import: the headings of the notes it writes and the title of a
## project with no name. Written into the imported project once, in the
## interface language.
import-manuskript-section-paragraph-summary = Содержание в одном абзаце
import-manuskript-section-full-summary = Полное содержание
import-manuskript-section-motivation = Мотивация
import-manuskript-section-epiphany = Прозрение
import-manuskript-section-notes = Заметки
import-manuskript-section-description = Описание
import-manuskript-section-passion = Страсть
import-manuskript-section-result = Итог
import-manuskript-section-summary = Краткое содержание
import-manuskript-section-meta = Примечания
import-manuskript-section-series = Серия
import-manuskript-section-volume = Том
import-manuskript-section-genre = Жанр
import-manuskript-section-license = Лицензия
import-manuskript-section-email = Электронная почта
import-manuskript-section-situation = Ситуация
import-manuskript-section-sentence = Одно предложение
import-manuskript-section-paragraph = Один абзац
import-manuskript-section-page = Одна страница
import-manuskript-section-full = Полное содержание
import-manuskript-untitled-project = Импортированный проект Manuskript

## Plume import: the story-bible groups an old project implies, and the title
## of a project with no name.
import-plume-group-characters = Персонажи
import-plume-group-items = Предметы
import-plume-group-places = Места
import-plume-untitled-project = Импортированный проект Plume
