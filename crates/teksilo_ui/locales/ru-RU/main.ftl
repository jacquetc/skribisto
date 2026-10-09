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

import-target-open-title = Этот проект открыт
import-target-open-text = Проект «{ $name }» открыт в Skribisto. Импорт поверх него заменит проект, над которым вы работаете, а следующее сохранение запишет тот проект обратно поверх импорта. Выберите другое имя файла или сначала закройте проект.
import-warnings-announcement = { $result } { $notice }.

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
        [one] Этот текст изменился
        [few] { $fields } из этих текстов изменились
        [many] { $fields } из этих текстов изменились
       *[other] { $fields } из этих текстов изменились
    } после поиска, поэтому { $fields ->
        [one] он оставлен
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

cover-choose = Обложка &книги…
cover-clear = &Убрать обложку
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
shortcut-name-app-settings = Настройки
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
