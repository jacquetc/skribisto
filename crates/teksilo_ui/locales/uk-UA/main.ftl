# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

menu-work = &Проєкт
menu-new-work = Н&овий проєкт
menu-open-work = Ві&дкрити проєкт…
menu-new-window = &Нове вікно
menu-import-from = І&мпортувати з
menu-create-from = &Створити з
menu-import-plume = &Plume Creator (.plume)…
menu-import-document = &Документи (Markdown, Слово ODT)…
menu-import-manuskript = &Manuskript (.msk)…
menu-export = &Експорт
menu-export-book = Експортувати книгу
menu-export-part = Експортувати частину
menu-export-chapter = Експортувати розділ
menu-export-scene = Експортувати сцену
menu-export-note = Експортувати нотатку
menu-export-paratext = Експортувати паратекст
menu-export-folder = Експортувати теку
menu-export-choose = Вибрати…
menu-export-none = Відкрити документ для експорту
menu-save = &Зберегти
menu-save-as-file = Зберег&ти одним файлом…
menu-save-as-folder = Збере&гти як теку…
menu-backup = &Створити резервну копію
menu-close-work = За&крити проєкт
menu-welcome = В&ітаємо…
menu-settings = Н&алаштування
menu-quit = &Вийти

menu-view = &Вигляд
menu-outline = &Структура
menu-search = П&ошук у проєкті
menu-timeline = Пов&ернутися в минуле
menu-search-preview = Попере&дній перегляд пошуку
menu-fullscreen = По&вний екран
menu-focus-mode = &Режим без відволікань
menu-format = &Формат
menu-scene-break = &Вставити розрив сцени
menu-major-scene-break = Вставити &головну сцену

menu-go = П&ерехід
menu-go-next-scene = Н&аступна сцена
menu-go-prev-scene = &Попередня сцена
menu-go-next-chapter = Наступний &розділ

menu-go-prev-chapter = Поп&ередній розділ
menu-go-next-note = &Наступна нота
menu-go-prev-note = П&опередня нотатка

menu-tools = &Інструменти
menu-help = &Довідка

ctx-add = &Додати
ctx-new-item = &Новий елемент
ctx-new-folder = &Нова тека
ctx-rename = П&ерейменувати
ctx-duplicate = Д&ублювати
ctx-import-here = &Імпортувати сюди…
ctx-indent = З&більшити відступ
ctx-outdent = &Зменшити відступ
ctx-trash = Пе&ремістити в кошик
ctx-open-to-side = &Відкрити убік
ctx-open = &Відкрити
ctx-reveal-in-outline = &Відкрити на контурі
ctx-move-up = &Пересунути вгору
ctx-move-down = &Пересунути вниз

ctx-tab-close = &Закрити
ctx-tab-close-others = З&акрити інші
ctx-tab-close-all = За&крити всі
ctx-tab-open-to-side = &Відкрити убік
ctx-tab-open-in-main = &Відкрити на головній панелі
ctx-tab-move-to-side = &Пересунути на сторону
ctx-tab-move-to-main = &Перейти до головної панелі
ctx-tab-move-to-new-window = П&ересунути в нове вікно
ctx-tab-move-window-unsaved = Зберегти цей проект першим: друге вікно відкриває файл на диску
ctx-tab-pin = &Швартувати цю вкладку
ctx-tab-unpin = В&илучити цю вкладку
tab-pinned-tooltip = Прикрашено: "Закрити інші" і "Закрити все" - не закривати

create-book = Книга
create-part = Частина
create-chapter = Розділ
create-scene = Сцена
create-note = Нотатка
create-note-folder = Тека нотаток
create-folder = Тека
create-paratext = Паратекст
create-paratext-folder = Тека паратекстів
create-book-end = Кінець книги
create-story-bible-entry = Запис енциклопедії історії…
type-book-start = Початок книги
type-text = Текст

new-item-book = Нова книга
new-item-part = Нова частина
new-item-chapter = Новий розділ
new-item-note = Нова нотатка
new-item-note-folder = Нова тека нотаток
new-item-folder = Нова тека
new-item-paratext = Новий текст
new-item-paratext-folder = Нова паратекстова тека
new-item-story-bible-entry = Біблійний запис

placement-inside = всередині
placement-after = після
placement-after-parent = після батьківського
placement-top-level = верхній рівень


ctx-promote = &Перетворити на
promote-chapter-folder = Розділ-тека
promote-flat-chapter = Розділ без сцен
promote-lossy-title = Ніде не зберігати текст
promote-lossy-text = { $target } не має де зберігати цей текст: { $kinds }.
content-scene-text = Текст сцени
content-note-text = Текст нотатки
content-book-title = Назва книги
content-book-subtitle = Субтитри книги
content-part-title = Назва частини
content-chapter-title = Заголовок розділу
content-epigraph-text = Епіграф
content-paratext-text = Паратекст
promote-blocked-title = Глава не порожня
promote-blocked-text = Цей розділ ще містить елементи. Кількість: { $count }. Перемістіть їх або надішліть у кошик, перш ніж перетворювати розділ на розділ без сцен.

inspector = Властивості
inspector-empty = Відкрий предмет, щоб перевірити його.
inspector-promote = Перетворити на…
statusbar-toggle-outline = Показати або приховати навігатор
statusbar-toggle-inspector = Показати або приховати властивості
statusbar-save-unsaved = Незбережені зміни. Клацніть, щоб зберегти
statusbar-save-saved = Всі зміни збережено
statusbar-save-autosave = Автозбереження увімкнено. Зміни зберігаються під час запису
statusbar-saving = Збереження…
statusbar-word-count = { $count ->
    [one] { $count } слово
    [few] { $count } слова
    [many] { $count } слів
   *[other] { $count } слова
}

statusbar-word-count-tooltip = Слова у вікні, яку ви редагуєте
statusbar-word-char-count = { $words ->
    [one] { $words } слово
    [few] { $words } слова
    [many] { $words } слів
   *[other] { $words } слова
} · { $chars ->
    [one] { $chars } символ
    [few] { $chars } символи
    [many] { $chars } символів
   *[other] { $chars } символа
}

session-toggle = Запис сеансу: розпочати або призупинити біг
session-configure = Встановити межу слова і часу сеансу
session-configure-title = Запис сеансу
session-word-goal = Мета в словах
session-time-limit = Обмеження часу
session-time-limit-unit = хв
session-no-goal = Без мети
session-no-limit = Без обмеження
session-reset = Скинути сеанс
session-readout = Слів: { $words } · { $time }
session-readout-timed = { $words } слів · { $time } лишилося
statusbar-focus-exit = Вийти з режиму вільного відволікання
statusbar-focus-synopsis = Синопсис
statusbar-focus-go-prev = Попередній елемент (Alt+Up)
statusbar-focus-go-next = Наступний елемент (Alt+Down)

rail-settings = Налаштування

language = Мова
theme = Тема
english = Англійська
french = ФранцузькаName
light = Світла
dark = Темна
settings-text-width = Ширина тексту
settings-preview-width = Шукати ширину перегляду
settings-autosave = Автозбереження на диск
settings-show-welcome = Показувати інструмент запуску під час запуску
settings-show-welcome-tip = Якщо цей проект вимкнено, замість нього буде відкрито останній.
settings-check-for-updates = Перевірити на нові версії
settings-check-for-updates-tip = Раз на день Skribisto запитує про поточну версію проекту. Запит не містить ідентифікаторів, а навіть версії, яку ви працюєте: порівняння відбувається на цьому комп' ютері.

settings-title = Налаштування
settings-close = Закрити
settings-search = Параметри пошуку
settings-reset = Відновити типові налаштування
settings-done = Готово
settings-cancel = Скасувати
settings-apply = Застосувати
settings-ok = Гаразд
settings-reset-confirm-title = Скинути всі параметри до типових?
settings-reset-confirm-body = Відновлює параметри вигляду, редактора і запису програми. Це не впливає на власні параметри вашого проекту, клавіатурні скорочення і збережені стилі. Це не можна скасувати.
settings-empty-title = Тут ще немає параметрів
settings-empty-hint = У цьому розділі ви знайдете пункти для наступних оновлень.

settings-sec-appearance-behaviour = Поведінка вигляду
settings-sec-editor = Редактор
settings-sec-spelling = Правопис
settings-sec-backup = Резервна синхронізація
settings-sec-compile = Компіляція та експорт
settings-page-appearance = Вигляд
settings-field-image-size-policy = Великі зображення
settings-image-policy-ask = Запитувати кожного разу
settings-image-policy-keep = Зберегти початковий
settings-image-policy-downscale = Оптимізувати
settings-hint-image-size-policy = Що робити, коли вставлено зображення більше, ніж потрібен рукопис.
settings-page-notifications = Сповіщення
settings-page-scene = Сцена
settings-page-synopsis = Синопсис
settings-page-notes = Нотатки
settings-page-editor-behavior = Поведінка редактора
settings-page-goals = Кількість цілей
settings-page-games = Запис ігор
settings-page-corkboard = Коркова дошка
settings-page-distraction-free = Від' єднання без
settings-page-dictionaries = Словники
settings-page-autosave = Автозбереження
settings-page-export = Формати експорту
settings-page-paratext = Структури паратексту
settings-page-keymap = Сполучення клавіш
settings-keymap-filter = Скорочення фільтрування

settings-desc-sec-appearance-behaviour = Як сама програма виглядає, і що вона робить, коли вона починається.
settings-desc-sec-editor = Письменна поверхня: як вона виглядає, і що вона робить під час друку.
settings-desc-sec-spelling = Перевірка правопису і словники за нею.
settings-desc-sec-backup = Як ваша робота досягає диску і які копії зберігаються.
settings-desc-sec-compile = Залишається Skribisto, і в якій формі.
settings-desc-sec-work = Параметри, які належать до цього проекту і рухаються у його файлі.
settings-desc-sec-extensions = Сторінки, які було надано суфіксами, встановлених тут.
settings-desc-group-typography = Одна сторінка на тип тексту: тип тексту, розмір, висота рядка і інтервали.
settings-desc-appearance = Мова інтерфейсу, тема, розмір тексту та початкове вікно.
settings-desc-notifications = Кожне повідомлення, яке було показано у цьому сеансі, і дії, які ви можете повторити.
settings-desc-scene = Визначає спосіб встановлення прози: тип, розмір, висота лінії, відступи і інтервали.
settings-desc-synopsis = Як встановлено панель синапсису, незалежно від рукопису.
settings-desc-notes = Як встановлюються нотатки, незалежно від рукопису.
settings-desc-corkboard = Розмір картки, те, що показує карта, і як дошка робить їх.
settings-desc-distraction-free = Typgraphy, width column and custom c стрічці керування для повноекранного запису.
settings-desc-distraction-free-themes = Бібліотека тем для повноекранного запису: ті, що постачаються, і ваші власні.
settings-desc-editor-behavior = Ширина тексту, розташування синопсісу, гортання друкарської машинки і підсвічування пензлем.
settings-desc-punctuation = Нова у стилі друкарського будинку, з якої починаються проекти. Кожен проект виконує цю функцію, якщо не встановлює власну.
settings-desc-goals = Слово і характер мішені, і спосіб, яким рахуються слова.
settings-desc-games = Обмеження, які ви встановлюєте під час створення чернеток, наприклад, Завжди вперед.
settings-desc-spellcheck = Єдиний перемикач, який вмикає перевірку правопису всюди.
settings-desc-dictionaries = Встановити, вилучити і переглянути словники правопису на цьому комп' ютері.
settings-desc-autosave = Чи ваші зміни записуватимуться на власний диск.
settings-desc-backup = Кожен проект виконується, якщо він не встановлює свій власний.
settings-desc-export = Стилі для кожного експорту буде зібрано за допомогою таких стилів і ваших власних.
settings-desc-paratext = З переднього і заднього плану може початися новий проект.
settings-desc-keymap = Будь-який шлях, і що він пов'язаний.
settings-page-user = Користувач
settings-desc-user = Хто ти такий, за коментарі, які пишеш.
settings-group-identity = Профіль
settings-field-user-name = Ваше ім' я
settings-field-user-name-placeholder = Необов' язковий
settings-field-user-name-hint = Підписує коментарі та відповіді, які ви пишете. Розрізняється на основі імені автора проекту, яка є почерговою книгою і рухається всередині файла.
settings-field-user-initials = Ваші ініціали
settings-field-user-initials-placeholder = З вашого імені
settings-field-user-initials-hint = Те, що показує текстовий процесор поруч з вашим коментарем на полях. Залиште порожнім, щоб використовувати показані, вибрані з вашого імені.
settings-field-user-hint = Обидва варіанти є необов' язковими, їх можна застосувати до всіх проектів на цьому комп' ютері. Зміна їх підписів позначає коментарі, які ви пишете. Коментарі вже написані, не заповнюйте їх назвою.
settings-desc-author = Назва проекту.
settings-desc-structure = Чи розділи цього проекту є теками, чи окремими елементами.
settings-desc-language = Прозу перевіряють.
settings-desc-work-backup = Це власне резервне правило, або загальне.
settings-desc-personal-dictionary = Слова, які цей проект пишеться правильно.
settings-desc-tags = Мітки, закодовані кольорами, за допомогою яких цей проект позначає елементи проекту.
settings-desc-templates = Шаблони нотаток, які ви можете вставити у цей проект.
settings-desc-text-replacements = Клавіатурні скорочення, які розгоряються під час введення вашого проекту.
settings-desc-work-punctuation = Далі йдуть лапки, тире і інтервали між прозою цього проекту.

settings-group-typography = Typgraphy
settings-group-writing-column = Запис стовпчика
settings-group-writing-view = Перегляд запису
settings-group-distraction-free-strip = Смуга керування
settings-group-theme = Тема
settings-group-language = Мова
settings-group-startup = Запуск
settings-group-updates = Оновлює
settings-group-autosave = Автозбереження
settings-field-typeface = Гарнітура
settings-field-size = Розмір тексту

editor-size-changed-manuscript = Розмір тексту: { $percent }
editor-size-changed-synopsis = Розмір тексту підрядка: { $percent }
editor-size-changed-notes = Розмір тексту нотаток: { $percent }
editor-size-changed-corkboard = Розмір тексту картки: { $percent }
editor-size-changed-corkboard-expanded = Широкий розмір тексту редактора: { $percent }
editor-size-changed-distraction-free = Розмір без виділення: { $percent }

menu-text-size-increase = З&більшити розмір тексту
menu-text-size-decrease = З&меншити розмір тексту
menu-text-size-reset = В&ідновити розмір тексту
settings-field-line-height = Висота лінії
settings-field-first-line-indent = Відступ у першому рядку
settings-field-paragraph-spacing-before = Пробіл перед абзацом
settings-field-paragraph-spacing-after = Пробіл за абзацом
settings-field-column-width = Ширина стовпчика
settings-distraction-free-width-hint = Застосовує режим, вільний від відволікання. Ваша сцена, Синопсія і Ширина стовпчиків Нотатки не буде змінено.
settings-distraction-free-title = Зберегти назву елемента
settings-distraction-free-word-count = Зберігати кількість слів
settings-distraction-free-session = Залишати сеанс запису
settings-distraction-free-go-to = Тримайтеся… button
settings-distraction-free-go = Зберегти попередню і наступну кнопки
settings-distraction-free-chrome-hint = Кнопка Вийти завжди залишається, все, що ви оберете тут. Це ваш вихід, якщо Escape є зайнятою.
settings-field-app-theme = Тема
settings-theme-light = Світла
settings-theme-dark = Темна
settings-field-text-scale = Розмір тексту інтерфейсу
settings-field-language = Мова інтерфейсу
settings-synopsis-placement = Розташування Короткий опис
settings-synopsis-placement-none = Немає
settings-synopsis-placement-top = Вище
settings-synopsis-placement-side = Поруч
settings-typewriter = Прокручування друкарської машинки
settings-typewriter-tip = За допомогою цього пункту можна утримувати рядок, який ви пишете на фіксованій висоті, доки рукопис буде проведено під ним. Натискання все ще призведе до того, що об' єкт буде розташовано у полі, де ви клацаєте кнопкою миші.
settings-typewriter-position = Позиція лінії
settings-typewriter-position-top-third = Верхній
settings-typewriter-position-middle = Середня
settings-typewriter-position-bottom-quarter = Нижній квартал
settings-highlight-scope = Підсвітити навколо каркасу
settings-highlight-scope-none = Немає
settings-highlight-scope-sentence = Речення
settings-highlight-scope-paragraph = Абзац
settings-highlight-scope-tip-none = Не затінюйте сторінки. Під час запису нічого не буде зафарбовано.
settings-highlight-scope-tip-sentence = Затіни речення, яке ти пишеш, щоб воно відрізнялося від усього, що навколо.
settings-highlight-scope-tip-paragraph = Затініть весь абзац, який ви пишете, для того, щоб мати справу з описом, над яким працюєте.
settings-group-container-views = Перегляди контейнерів
settings-remember-view = Запам' ятати останній перегляд для кожного типу елементів
settings-remember-view-tip = Відкрити контейнер на панелі перегляду, який ви востаннє використовували для цього типу
settings-remember-view-tip-more =
    Книга, Частина і розділ містять декілька переглядів (його власна сторінка повністю
    Увімкнення цього рукопису, повного синапсу).
    перегляд, який ви обрали останнім, наприклад, перемикнути один розділ на повний і
    Наступний розділ, який ви відкриєте, також показує повний розділ. Кожен з типів запам' ятає власний вигляд.
settings-group-counting = Кількість слів
settings-counting-auto = Автоматично (мовами)
settings-counting-whitespace = Розділити на пробіли
settings-counting-unicode-words = Слова Unicode
settings-counting-cjk-hybrid = Програма CJK- awre (на символ)
settings-counting-hint = Автоматично визначати за характером китайську та японську. Змініть її, лише якщо кількість не збігається з вашою мовою.
settings-group-goals-display = Показ
settings-show-characters = Показувати кількість символів у рядку стану
settings-autosave-hint = Зміни буде автоматично записано на диск під час запису.

settings-sec-work = Проєкт
settings-page-structure = Структура
settings-page-author = Автор
settings-field-author-name = Назва автора
settings-field-author-placeholder = Необов' язковий
settings-field-author-hint = На скомпільованій сторінці заголовка і у експортованому файлі метадані. Залиште це поле порожнім, щоб його не було у списку. Назва вашої книги підписана у меню Параметри ▸ Користувач.
settings-group-chapters = Глави
tidy-titles-title = Забрати заголовки розділів
tidy-titles-none = Жоден розділ чи частина не є тільки повторенням свого власного числа.
tidy-titles-lead = Деякі розділи й частини мають у заголовку лише власний номер. Кількість: { $count }.

tidy-titles-explain = Очищення цих заголовків призведе до того, що кожен з них має назву за номером, про який вже знає книга, з тим самим номером, що і друк на основі експорту. Нічого іншого не змінюється, і одне зі скасувань поверне всі ці дані.
menu-document-tidy-titles = З&абрати заголовки розділів…
settings-group-numbering = Нумерація
settings-number-chapters = Номери розділів і частин
settings-number-chapters-tip = У розділах і частинах міститься певна кількість робіт, які можна знайти у книжці, поряд з назвою, а також надруковані експортом.
settings-number-chapters-tip-more = Число ніколи не зберігатиметься у назві, отже під час перевпорядкування, вставлення або вилучення його не буде збережено. Вимкніть цей параметр, а назви для експорту не буде показано у стилі експорту. Щоб залишити одну главу (пролог, інтерлегація), скористайтеся перемикачем " Номерування " інспектора: цей параметр зберігає її у книзі, але зупиняє її використання.
settings-part-resets-chapter = Перезапустити номери розділів з кожної частини
settings-part-resets-chapter-tip = Типово: розділи перекриваються частинами книги, отже " Частина два " відкриває одинадцятий розділ.
settings-part-resets-chapter-tip-more = Це звичайна торгова практика, і що читач сподівається.
settings-chapter-flat = Розділи без сцен
settings-chapter-flat-hint = On: розділ є одним елементом. Ви пишете до нього, у ньому немає сцен. Вимкнено: глава є текою. Ви все ще пишете до неї, але у ній також можуть міститися сцени. Нові розділи слідують за цим параметром; існуючі буде перетворено за допомогою Apport.

settings-styles-builtin = Вбудовані стилі
settings-styles-user = Мої стилі
settings-styles-builtin-badge = Вбудований
settings-styles-duplicate = Дублювати
settings-styles-edit = Редагування
settings-styles-delete = Видалити
settings-styles-import = Імпорт …
settings-styles-export = Export…
settings-styles-copy-suffix = (копія)
settings-styles-json-filter = Експортувати стиль
settings-styles-editor-title = Змінити стиль
settings-styles-editor-none = Виберіть власний стиль для редагування або створіть копію вбудованого.
settings-styles-imported = Стиль імпортовано
settings-styles-import-failed = Не вдалося імпортувати стиль
settings-styles-exported = Стиль експортовано
settings-styles-export-failed = Не вдалося експортувати стиль
settings-styles-field-name = Назва
settings-styles-field-chapters = Підказки розділів
settings-styles-field-parts = Заголовки частин
settings-styles-field-scene-break = Переривання сцени
settings-styles-field-major-scene-break = Розрив головної сцени
settings-styles-field-spacing = Інтервал між лініями
settings-styles-field-justify = Вирівняти текст
settings-styles-field-notes = Включити нотатки
settings-styles-field-synopses = Включити synops
settings-styles-field-scene-titles = Включити заголовки сцен
settings-styles-field-epigraphs = Включити епіграфи
settings-styles-field-paratexts = Додати паратексти
settings-styles-field-epigraph-placement = Позиція Епіграфа
settings-styles-epigraph-after = Після назви
settings-styles-epigraph-before = Перед заголовком
settings-styles-field-footnotes = Включити примітки
settings-styles-field-footnote-numbering = Кількість адрес
settings-styles-footnote-numbering-continuous = Безперервний
settings-styles-footnote-numbering-per-chapter = Перезапустити кожен розділ
settings-styles-footnote-numbering-per-book = Перезапустити кожну книгу
settings-styles-field-images = Зображення
menu-image = &Зображення
settings-styles-images-beside = Поза документом
settings-styles-images-embed = Всередині документа
settings-styles-images-omit = Вийти
settings-styles-group-round-trip = Надсилання редактору
settings-styles-field-comments = Включити коментарі
settings-styles-field-round-trip-marks = Додати позначки зворотного імпорту
settings-styles-round-trip-hint = Лише DOCX і ODT. Невидимі позначки дають змогу оновити цей проєкт під час імпорту відредагованого файлу, а не створити другу копію.
settings-styles-group-pages = Сторінки
settings-styles-field-cover = Відкрити за допомогою обкладинки
settings-styles-field-word-count = Кількість слів на титульній сторінці
settings-styles-field-page-books = Нова сторінка у кожній книзі
settings-styles-field-page-parts = Нова сторінка з кожної частини
settings-styles-field-page-chapters = Нова сторінка у кожному розділі
settings-styles-field-page-paratexts = Нова сторінка з кожного тексту
settings-styles-heading-none = Без заголовка
settings-styles-heading-numbered = Лише номер
settings-styles-heading-title = Лише заголовок
settings-styles-heading-both = Число + назва
settings-styles-break-blank = Порожній рядок
settings-styles-break-none = Немає
settings-styles-spacing-single = Одинарна
settings-styles-spacing-onehalf = 1½
settings-styles-spacing-double = Подвійний

welcome-title = Ласкаво просимо до Skribisto
welcome-version = Версія { $version }
welcome-update-available = Версія { $version }
welcome-update-tip = Відкриває сторінку звантаження у вашому переглядачі.
welcome-search = Пошук працює
welcome-open = Відкрити
welcome-new-work = Новий проєкт
welcome-create-from = Створити з…
welcome-recent-works = Нещодавні проєкти
welcome-empty-recents = Никаких последних дел.
welcome-no-matches = Нещодавні роботи не збігаються з пошуками.
welcome-learn-soon = Напрямки і поради скоро прибудуть.
welcome-about-blurb = Skribisto, Russ + Teksilo-перепис програм для написання.
welcome-tagline = Тихе місце для написання довгих речей.
welcome-github = GitHub
welcome-discord = Disord
nav-works = Проекти
nav-examples = Приклади
nav-learn = Вчити
nav-about = Про програму

synopsis = Синопсис
epigraph = Епіграф
pane-manuscript = Рукопис
corkboard = Коркова дошка
overview = Огляд

corkboard-card-count = { $count ->
    [one] { $count } карта
    [few] { $count } карти
    [many] { $count } карток
   *[other] { $count } карти
}
corkboard-child-count = { $count ->
    [one] { $count } елемент
    [few] { $count } елементи
    [many] { $count } елементів
   *[other] { $count } елементи
}
corkboard-view-nested = Вкладених
corkboard-view-flat = Плоска
corkboard-layout-hint = У вмонтованому місці показано прямі дочірні елементи контейнера. Відкрийте карту тек, щоб увійти до неї. Плоска програма показує всі сцени у всьому контейнері одночасно.
corkboard-card-size = Розмір картки
corkboard-search-placeholder = Фільтрувати карти…
corkboard-empty-title = Тут нічого.
corkboard-empty-hint = Для додавання першого з них скористайтеся пунктом " ＋ " вище.
corkboard-new = Новий
corkboard-show-card-numbers = Нумерувати карти
corkboard-modal-size = Розширений розмір редактора
corkboard-scope-hint = Застосовується до кожного відкритого стенда в кожному проекті.
corkboard-sort-manuscript = Порядок рукописів
corkboard-sort-title-asc = Титул A'Z
corkboard-sort-title-desc = Титул
corkboard-card-number = Картка { $number }

duplicate = Дублювати
duplicate-n = { $count ->
    [one] Дублювати { $count } карту
    [few] Дублювати { $count } карти
    [many] Дублювати { $count } карт
   *[other] Дублювати { $count } карт
}
set-label-n = { $count ->
    [one] Встановити мітку на { $count } карті
    [few] Встановити мітку на { $count } карти
    [many] Встановити мітку на { $count } картках
   *[other] Встановити мітку { $count } карти
}
move-to-trash-n = { $count ->
    [one] Пересунути { $count } карту у смітник
    [few] Пересунути { $count } карти у смітник
    [many] Пересунути { $count } карт у смітник
   *[other] Пересунути { $count } карт у смітник
}
reveal-in-outline = Відкрити контуром

corkboard-move-to = Пересунути до…
corkboard-move-to-n = { $count ->
    [one] Пересуньте { $count } карту до 86543211
    [few] Перемістити { $count } карти у 86543211
    [many] Перемістити { $count } карт у 86543211
   *[other] Пересунути { $count } карти до…
}
corkboard-move-picker-title = Пересунути до…
corkboard-move-picker-empty = Ще немає прив' язувачів. Створіть перший.
corkboard-move-picker-cancel = Скасувати
corkboard-move-here = Пересунути сюди
corkboard-moved-ok = { $count ->
    [one] Переслано { $count } карту
    [few] Переслано { $count } карти
    [many] Переслано { $count } карт
   *[other] { $count } карти пересунуто
}
corkboard-move-into-self = Контейнер не можна пересувати всередині себе. Виберіть місце призначення.
corkboard-move-failed = Ці карти не можна було перемістити туди.

overview-col-title = Заголовок
overview-col-type = Тип
overview-col-label = Підпис
overview-col-tags = Мітки
overview-col-books = Книги
overview-col-own-words = Слова
overview-col-total-words = Загалом
overview-row-count = { $count ->
    [one] { $count } елемент
    [few] { $count } елементи
    [many] { $count } елементів
   *[other] { $count } елементи
}
overview-search-placeholder = Фільтрувати елементи…
overview-expand-all = Розгорнути всі
overview-collapse-all = Згорнути все
overview-table-label = Вміст
overview-empty-title = Тут нічого.
overview-gone-title = Цього контейнера немає
overview-gone-hint = Його було пересунуто до смітника. Відновити його або закрити цю вкладку.
overview-empty-hint = Для додавання першого з них скористайтеся пунктом " ＋ " вище.
corkboard-grid-label = Картки
corkboard-rename-field = Перейменувати елемент
corkboard-expand-synopsis = Розгорнути синопсію
corkboard-synopsis-modal-title = Синопсис
corkboard-badge-scene = Сцена
corkboard-badge-chapter = Розділ
corkboard-badge-part = Частина
corkboard-badge-book = Книга
corkboard-badge-note = Нотатка
corkboard-badge-folder = Тека
corkboard-badge-paratext = Паратекст
corkboard-badge-text = Текст
corkboard-badge-end = End
settings-group-corkboard-layout = Розкладка
settings-group-corkboard-cards = Картки
corkboard-show-word-count = Показувати кількість слів на картках
text-heading = Текст
no-content = Цей елемент не має придатного для редагування вмісту.
untitled = Без назви
placeholder-title = Заголовок…
placeholder-subtitle = Підзаголовок…
placeholder-chapter-title = Заголовок розділу…
split-editor = Розділити редактор
close-split-view = Закрити розділювач
drop-open-here = Відкрити тут
drop-open-to-side = Відкрити на стороні

segment-chapter = Розділ
segment-part = Частина
segment-book = Книга
segment-pace = Темп
segment-notes = Нотатки
segment-story-bible = Енциклопедія історії
segment-note-own = Нотатка
segment-note-details = Подробиці
segment-note-in-prose = У проза
pace-placeholder = Тут з'явився планувальник Пейсу.
pace-empty-title = Планувати темп цієї книги
pace-empty-body = Установи термінову мету і Skribisto.
pace-start-planning = Почати планування
pace-section-schedule = Розклад
pace-section-progress = Поступ
pace-advancement = Поступ
pace-goal = Мета в словах
pace-deadline = Кінцевий термін
pace-active = План роботи активний
pace-day-mon = пн
pace-day-tue = вт
pace-day-wed = ср
pace-day-thu = чт
pace-day-fri = пт
pace-day-sat = сб
pace-day-sun = нд
pace-card-written = написані слова
pace-card-of-goal = мети
pace-card-rate = слова/ день запису
pace-card-days-left = Запис днів ліворуч
pace-card-streak = денна смужка
pace-card-ahead = слова перед темпом
pace-card-behind = слова за темпом
pace-charts-empty = Тут буде показано діаграми поступу після запису вашої кількості слів під час збереження.
pace-chart-progression = Слова, написані проти цілі
pace-chart-words-per-day = Слова на день
pace-series-actual = Поточна
pace-series-target = Мета
pace-series-words-per-day = Слова/ день
pace-daily-target-line = Ще один темп: { $count ->
    [one] { $count } слово/ день
    [few] { $count } слова/ день
    [many] { $count } слів/ днів
   *[other] { $count } слів/ днів
}
pace-section-holidays = Вихідні
pace-section-milestones = Віхи
pace-holidays-none = Немає свят. Кожен запланований день.
pace-holiday-label = Назва свята
pace-add-holiday = Додати
pace-remove = Вилучити
pace-milestones-none = Ще немає віх на частини або на главі інспектора.
full-chapter = Повний розділ
full-part = Повна частина
full-book = Повна книга
full-synopsis = Повна синопсія
rename = Перейменувати
set-label = Встановити мітку
insert-scene = Вставити сцену
insert-chapter = Вставити розділ
split-scene = Розбити сцену
move-up = Пересунути вгору
move-down = Пересунути вниз
merge-with-previous = Об' єднати з попереднім
move-to-trash = Пересунути до смітника
rename-chapter = Перейменувати главу
add-scene = Додати нову сцену
add-chapter = Додати нову главу
new-scene-title = Нова сцена
placeholder-scene-name = Назва сцени…
menu-cut = Вирізати
menu-copy = Копіювати
menu-paste = Вставити
menu-paste-unformatted = Вставити без форматування
menu-select-all = Вибрати все
format-bold = Жирний
format-italic = Курсив
format-underline = Підкреслення
format-strikethrough = Закреслення

binder = Навігатор
no-work = Не завантажено жодної роботи
no-recent-works = Нещодавні дії
switcher-open-section = Зараз відкрито
switcher-recent-section = Нещодавні
switcher-this-window = це вікно
open-project-title = Відкрити проект
open-project-question = Як ви хочете відкрити "{ $title }"?
open-in-new-window = Відкрити у новому вікні
open-here = Відкрити тут

binder-all = Усі навігатори
binder-show-all = Показати всі прив' язки
binder-new = Новий прив' язок…
binder-default-name = Новий прив' язок
binder-item-count = { $count ->
    [one] { $count } елемент
    [few] { $count } елементи
    [many] { $count } елементів
   *[other] { $count } елементи
}
binder-search-placeholder = Фільтрувати контур …
binder-search-scope = Шукати всі прив' язки
binder-trash-confirm-title = Пересунути прив' язку до смітника?
binder-trash-confirm-text = " { $name } " - всі елементи буде пересунуто до смітника.

dialog-rename = Перейменувати
dialog-new-binder = Новий прив' язок
dialog-set-label = Встановити мітку
dialog-new-scene = Нова сцена
close-work-question = Зберегти зміни перед закриттям роботи?
quit-question = Зберегти зміни перед виходом?
quit-save-work-question = Зберегти зміни у { $title } перед тем, как уволиться?
unsaved-changes = Ця робота має незбережені зміни.
new-work-unsaved-question = Зберегти зміни перед створенням нової роботи?
open-work-unsaved-question = Зберегти зміни перед відкриттям іншої роботи?
switch-backup-discard-title = Відкинути зміни до цієї резервної копії?
switch-backup-discard-text = Зміни до резервної копії не можна зберегти. Скористайтеся пунктом Зберегти як або Відновити, щоб зберегти їх, або відкиньте їх і відкрийте іншу роботу.
switch-save-failed = Роботу неможливо зберегти, тому її не було замінено: { $error }
switch-save-not-started = Робота не могла бути врятована, тому її не було замінено.
close-save-failed = Роботу неможливо зберегти, тому її не було закрито: { $error }
close-save-not-started = Роботу неможливо врятувати, тому її не було закрито.
save-not-started = Роботу неможливо врятувати.

could-not-open-work = Не вдалося відкрити "{ $file }"
could-not-open-work-too-new = " { $file } " потребує новішого Skribisto
could-not-open-work-too-new-detail = Збережено Skribisto формат { $written_by }; Щоб відкрити його, потрібен формат { $requires } або новіша, і ця збірка підтримує форматування { $supported }. Оновити Skribisto щоб відкрити її.
could-not-open-work-importing = " { $file } " було написано імпортуванням
could-not-open-work-importing-detail = Імпорт записується над цим проектом і ще не завершено. Відкрийте його після завершення імпортування: відкрийте зараз, програма покаже проект, який буде замінено, а його наступне збереження запише цей проект на час імпортування.
could-not-open-example = Не вдалося відкрити приклад: { $error }
could-not-open-link = Не вдалося відкрити { $url }: { $error }
could-not-create-work = Не вдалося створити роботу: { $error }
saving-as-file = Збереження як { $target }…
saving-as-folder = Як { $target }/…
saved-as = Збережено до { $target }
save-error = Не вдалося зберегти: { $error }
save-flush-failed = Збережено, але ці документи не можна було передати до них, їх ще не збережено: { $titles } ({ $error })
backup-error = Не вдалося створити резервну копію: { $error }
backing-up = Створення резервної копії…
backup-nothing-open = Жоден проект не працює.
backup-already-running = Створення резервної копії вже виконується.
backup-complete = Резервування завершено ({ $ok }), { $skipped } вже наявне)
backup-partial = Резервне копіювання завершено. Збережено: { $ok }; місць призначення з помилкою: { $failed }.

backup-no-destination-title = Немає місця для резервної копії
backup-no-destination-text = До жодної з налаштованих резервних адрес не можна дістатися (наприклад, ви можете вимкнути зовнішній пристрій). Застібніть і повторіть спробу, або вийдіть без підтримки.
backup-failed-close-title = Не вдалося зберегти резервну копію
backup-failed-close-text = Не вдалося записати резервну копію перед закриттям. Будь- яке призначення зазнало невдачі (можна бути вилученим або повним або захищеним від запису). Виправте його і повторіть спробу або виходьте без підтримки.

backup-choice-title = Файл резервної копії
backup-choice-heading = Ви відкрили резервну копію
backup-choice-subtitle = Це резервна копія проекту.
backup-choice-subtitle-dated = Подкрепление сняло { $date }.
backup-choice-body = Ви можете відкрити його і редагувати без змін, але зміни можна зберегти лише за допомогою пункту Зберегти як. Початковий файл проекту не буде доторкнувся. або відновити цей проект, щоб створити таку резервну копію.
backup-choice-open = Відкрити резервну копію
backup-choice-restore = Відновити проект до цієї точки…
backup-choice-not-a-backup = Ні, зазвичай
backup-banner-title = Резервна копія: зміни неможливо зберегти
backup-banner-description = Скористайтеся пунктом меню Зберегти, щоб зберегти ваші зміни у новому файлі, або Відновити, щоб замінити початковий проект цією резервною копією.
backup-banner-restore = Відновити…
backup-banner-save-as = Зберегти як…
backup-restore-original-missing = Не вдалося знайти початковий проект для відновлення. Скористайтеся пунктом меню Зберегти як новий проект.
backup-restore-close-elsewhere-title = Проект відкритий в іншому вікні
backup-restore-close-elsewhere-text = Проект, який ви відновлюєте, відкрито в іншому вікні.
backup-restore-focus-window = Фокусувати це вікно
backup-restore-confirm-title = Відновити цю резервну копію?
backup-restore-confirm-text = Поточну версію проекту буде скопійовано убік як резервну копію, перш ніж її буде замінено на цю.
backup-restore-confirm-ok = Відновити
backup-restore-error = Не вдалося відновити: { $error }
backup-restored-ok = Проект відновлено.
backup-restored-with-safety = Проект було відновлено. Вашу попередню версію збережено до { $path }.
close-backup-discard-title = Відкинути зміни до цієї резервної копії?
close-backup-discard-text = Зміни до резервної копії не можна зберегти. Скористайтеся кнопкою Зберегти як для того, щоб зберегти їх або відкинути і закрити.
quit-backup-discard-title = Відкинути зміни і вийти?
quit-backup-discard-work-question = Відкинути зміни у { $title } и уволиться?
quit-backup-discard-text = Зміни до резервної копії неможливо зберегти. Скористайтеся пунктом Зберегти як, щоб зберегти їх або відкинути і вийти.
backup-nudge-text = Для цього проекту не встановлено резервних копій.
backup-nudge-action = Налаштування резервних копій…

menu-backups-list = &Резервні копії…
backups-title = Резервні копії
backups-loading = Пошук резервних копій…
backups-empty = Для цього проекту ще не знайдено резервних копій.
backups-open = Відкрити
backups-reveal = Показати розташування
backups-delete = Вилучити цю резервну копію
backups-delete-confirm-title = Вилучити цю резервну копію?
backups-delete-confirm-text = " { $name } " буде назавжди вилучено. Це неможливо скасувати.
backups-delete-error = Не вдалося вилучити резервну копію: { $error }
backups-refresh = Оновити
backups-close = Закрити

settings-page-backup = Типові резервні копії
settings-page-work-backup = Резервні копії
settings-backup-general-title = Типові параметри резервування
settings-backup-work-title = Резерви для цього проекту
settings-backup-inherit = Використовувати загальні параметри резервування
settings-backup-inheriting = Цей проект використовує загальні параметри резервування.
settings-backup-none-hint = Не налаштовано автоматичних резервних копій (кожен з рушіїв вимкнено).
settings-backup-last = Остання резервна копія: { $date }
settings-backup-last-never = Подкрепление еще не подкрепление.
settings-backup-open-list = Відкрити список резервних копій…
settings-backup-triggers = Коли повертатися
settings-backup-on-close = Назад під час закриття проекту
settings-backup-on-open = Назад під час відкриття проекту
settings-backup-interval = Час від часу, кожні
settings-backup-destinations = Призначення резервної копії
settings-backup-default-location = Типова адреса
settings-backup-usage-since = { $count ->
    [one] { $count } резерв · { $size } · найстаріший { $oldest }
    [few] { $count } резервні копії { $size } · найстаріших { $oldest }
    [many] { $count } резервних копій { $size } · найстаріших { $oldest }
   *[other] { $count } резервні копії ·  { $size }  · найстаріший { $oldest }
}
settings-backup-usage = { $count ->
    [one] { $count } резервна копія { $size }
    [few] { $count } резервні копії { $size }
    [many] { $count } резервних копій { $size }
   *[other] { $count } резервні копії ·  { $size }
}
settings-backup-usage-empty = Тут нічого не залишилося.
settings-backup-usage-measuring = Вимірювання …
settings-backup-reveal-root = Показати теку
settings-backup-dest-none = Немає призначення. Резерви зберігаються поряд з проектом.
settings-backup-dest-remove = Вилучити
settings-backup-dest-add = Додати теку…
settings-backup-dest-refresh = Оновити
settings-backup-retention = Скільки залишилося
settings-backup-retention-tiered = За рівнями
settings-backup-retention-keep-n = Залишити останній N
settings-backup-retention-tip = Как старые подкрепления.
settings-backup-retention-tip-more = Змінено одну резервну копію на годину протягом дня, день на тиждень, тиждень, а потім місяць, отже, нещодавня історія залишається щільною і застарілою. " Залишати останню N " просто зберігає N найсвіжіші копії. У обох режимах найновіші копії (найменші копії нижче) завжди зберігаються.
settings-backup-gfs-hourly = Години (остання 24 h)
settings-backup-gfs-daily = Щодня (попередній тиждень)
settings-backup-gfs-weekly = Тиждень (попередній місяць)
settings-backup-gfs-monthly = Щомісяця
settings-backup-keep-n = Номер, який слід залишити
settings-backup-min-keep = Завжди зберігати
settings-backup-dedup = Пропустити резервну копію, якщо нічого не змінилося

new-work-title = Новий проєкт
new-work-close = Закрити
new-work-name = Назва проекту
new-work-name-placeholder = Без назви
new-work-author = Автор
new-work-author-placeholder = Необов' язковий
new-work-format = Формат
new-work-single-file = Один файл
new-work-bundle = Тека проєкту
new-work-convert-later = Пізніше ви зможете перетворити дані у форматах.
new-work-location = Місце
new-work-will-create = Створить
new-work-language = Типова мова
new-work-language-hint = Застосовано до нових текстів перевірки правопису. Кожен з текстів можна перейти на іншу мову.
new-work-template = Шаблон
new-work-template-none = Немає
new-work-template-empty-novel = Порожній роман
new-work-template-light-novel = Світло
new-work-template-novel = Novel
new-work-template-novel-in-parts = Новл у частинах
new-work-template-notebook = Записник
new-work-cancel = Скасувати
new-work-create = Створити проект
new-work-step-details = Подробиці
new-work-step-language = Мовна структура
new-work-step-template = Шаблон
new-work-back = Назад
new-work-next = Далі
new-work-documents-title = Новий проект з документів
new-work-step-import = Імпорт
new-work-create-and-import = Створити імпорт…
new-work-documents-next-title = Наступні ваші документи
new-work-documents-next-body = Створення проекту відкриє вікно майстра імпортування, за допомогою якого ви зможете обрати документ, переглянути структуру Skribisto, а також визначити місце, де він опиниться. У проекті нічого не буде записано, аж доки ви не підтвердите його.
new-work-documents-no-template = З метою цей проект починається порожнім: шаблона немає, отже імпортовані документи - це єдине, що в ньому є.
new-work-documents-chapter-scene-hint = Застосовує до розділів, які створює імпорт. Пізніше ви зможете змінити його за допомогою параметрів проекту.
new-work-single-file-desc = Один архів .skrib (zip). Портативний, простий для резервного копіювання.
new-work-bundle-desc = Тека, що містить всі текстові активи. Другорядність для керування версіями.
new-work-template-none-count = порожній прив' язок
new-work-template-empty-novel-count = 1 розділ
new-work-template-light-novel-count = 15 розділів
new-work-template-novel-count = 20 розділів
new-work-template-novel-in-parts-count = 3 частини, 24 розділи
new-work-template-notebook-count = нотатки для вільної форми
new-work-chapter-scene = Розділи без сцен
new-work-chapter-scene-tip = Кожна з розділів - це окремий елемент, до якого ви просто пишете без сцен. Зніміть позначку з цього пункту для класичного компонування, де розділ є текою: ви також пишете його одразу, але у ньому також можуть міститися сцени.
new-work-chapter-scene-tip-more = У будь- якому разі ви пишете главу. Єдиною відмінністю є те, чи зможе вона містити сцени *. Ієрархія прив' язки Skribisto є лише організаційною, отже, обидва розкладки, зібрані у одній книзі, ви можете вільно їх змішувати, а також сприяти перетворенням глави між цими двома розділами без втрати слова.
new-work-name-required = Введіть назву для роботи
new-work-name-invalid = Це ім' я не має придатних символів
new-work-location-required = Виберіть адресу
new-work-location-missing = Цю теку не існує
new-work-location-not-folder = Цей шлях не є текою
new-work-location-readonly = Неможливо записати в цю теку
new-work-target-exists = Проект з цією назвою вже знаходиться у цій теці. Створення роботи запитає вас про це перед заміною
new-work-target-is-folder = Тека з такою назвою вже існує. Оберіть іншу назву
new-work-overwrite-title = Замінити існуючий проект?
new-work-overwrite-text = "{ $name }" вже існує у цій теці. Замінити її новим проектом?
new-work-target-open-title = Цей проект відкритий
new-work-target-open-text = " { $name } " відкрито у Skribisto. Створення нового проекту на його місце замінить проект, над яким ви працюєте, і ваше наступне збереження напише цей проект у новому. Оберіть інше ім' я або теку або спочатку закрийте проект.
target-importing-title = Імпорт записує цей проект
target-importing-text = Імпорт все ще пишеться "{ $name }". Почекай аж поки не закінчиться, або вибери інше ім'я.
import-plume-busy-title = Інший Plume Creator проект імпортується
import-plume-busy-text = проекти Plume Creator імпортуються одночасно. Зачекайте доки імпорт не завершиться, або не скасуйте його, а потім знову запустіть цей проект.
import-manuskript-busy-title = Інший Manuskript проект імпортується
import-manuskript-busy-text = проекти Manuskript імпортуються одночасно. Зачекайте доки імпорт не завершиться, або не скасуйте його, а потім знову запустіть цей проект.
backup-restore-importing-text = Імпорт все ще пишеться "{ $name }Відновлення резервної копії після завершення імпортування.

new-work-manuscript = Рукопис
new-work-notes = Нотатки
new-work-research = Дослідження
new-work-notebook = Записник
new-work-scene = Сцена
new-work-note = Нотатка
new-work-front-matter = Передня матерія
new-work-back-matter = Зворотна матерія
new-work-paratext = Структура книги
new-work-paratext-none = Без структури
new-work-paratext-hint = Суть переднього і заднього плану відкриває і закриває книгу. Ви можете пересувати, перейменовувати або вилучати будь- яку з них після цього.
new-work-tags = Мітки
new-work-tags-none = Без міток
new-work-tags-hint = Початкова палітра для символів, місць і решти. За допомогою цього пункту ви зможете перейменувати, змінити або вилучити всі мітки.
new-work-note-templates = Шаблони нотаток
new-work-note-templates-none = Немає шаблонів
new-work-note-templates-hint = Форма нотатки, яку можна використовувати для перегляду. Необов' язкові, кожен з шаблонів можна буде потім змінити або вилучити.
new-work-characters = Символи
new-work-places = Місця

import-plume-title = Імпортувати Plume Creator проект
import-plume-close = Закрити
import-plume-source = Проєкт Plume
import-plume-source-hint = Вибрати .plume або .plume_backup файл (всі Plume Creator version).
import-plume-location = Тека призначення
import-plume-name = Назва файла
import-plume-name-placeholder = Назва проекту
import-plume-will-create = Створить
import-plume-trash-warning = біса Переміщені/ вилучені елементи не переводяться.
import-plume-cancel = Скасувати
import-plume-import = Імпорт
import-plume-source-required = Виберіть файл проекту Plume
import-plume-source-missing = Цього файла не існує
import-plume-source-not-file = Цей шлях не є файлом
import-plume-location-required = Виберіть теку призначення
import-plume-location-missing = Цю теку не існує
import-plume-location-not-folder = Цей шлях не є текою
import-plume-location-readonly = Неможливо записати в цю теку
import-plume-name-required = Введіть назву файла
import-plume-name-exists = Файл з такою назвою вже існує. Імпорт підтвердить перезапис
import-plume-overwrite-title = Замінити існуючий файл?
import-plume-overwrite-text = "{ $name }" вже існує. Замініть його імпортованим проектом?
import-plume-manuscript-binder = Рукопис
import-plume-story-bible-binder = Біблія
import-plume-progress-title = Імпортування проекту Plume …
import-plume-cancel-import = Скасувати
import-plume-cancelled = Імпорт скасовано
import-plume-done = Імпортовано { $imported ->
    [one] { $imported } пункт.
    [few] { $imported } предмети.
    [many] { $imported } речей.
   *[other] { $imported } елементів.
} { $skipped ->
    [one] { $skipped } Викинутий елемент не було змінено.
    [few] { $skipped } випорожнені предмети не були переправлені.
    [many] { $skipped } Викинутих речей не перевозили.
   *[other] { $skipped } предметів не було переселено.
}
import-plume-open-now = Відкрити зараз
import-plume-warnings = Не вдалося точно перенести деякі дані. Кількість зауважень: { $count }.

import-plume-details = Подробиці
import-plume-warnings-title = Імпортувати попередження
import-plume-error-title = Не вдалося імпортувати проект
import-plume-nested-too-deep = Проект не було імпортовано: його{ $part }"частина в гнізді більше, ніж { $limit } глибини рівнів. Ні Plume Creator проект наближається до того, і читання може розбитись SkribistoНічого не написано.
import-plume-declares-entities = Проект не було імпортовано: його частина " { $part } " запускає елементи XML, яку не виконує жоден з Plume Creator проектів. Розширювання таких об' єктів може тривати довше, ніж на комп' ютері, і завершить роботу Skribisto. Нічого не написано.
import-plume-error-details = Подробиці

import-manuskript-title = Імпортувати Manuskript проект
import-manuskript-close = Закрити
import-manuskript-source = Manuskript проект
import-manuskript-source-hint = Оберіть файл .msk або теку проекту (відміну Manuskript).
import-manuskript-source-file = Виберіть файл…
import-manuskript-source-folder = Виберіть теку…
import-manuskript-location = Тека призначення
import-manuskript-name = Назва файла
import-manuskript-name-placeholder = Назва проекту
import-manuskript-will-create = Створить
import-manuskript-cancel = Скасувати
import-manuskript-import = Імпорт
import-manuskript-source-required = Вибрати Manuskript проект
import-manuskript-source-missing = Цього файла або теки не існує
import-manuskript-location-required = Виберіть теку призначення
import-manuskript-location-missing = Цю теку не існує
import-manuskript-location-not-folder = Цей шлях не є текою
import-manuskript-location-readonly = Неможливо записати в цю теку
import-manuskript-name-required = Введіть назву файла
import-manuskript-name-exists = Файл з такою назвою вже існує. Імпорт підтвердить перезапис
import-manuskript-overwrite-title = Замінити існуючий файл?
import-manuskript-overwrite-text = "{ $name }" вже існує. Замініть його імпортованим проектом?
import-manuskript-manuscript-binder = Рукопис
import-manuskript-story-bible-binder = Енциклопедія історії
import-manuskript-characters-group = Символи
import-manuskript-world-group = Світ
import-manuskript-plots-group = Графіки
import-manuskript-project-info-note = Інформація про проект
import-manuskript-summary-note = Зведення
import-manuskript-importance-minor = Незначна
import-manuskript-importance-secondary = Другорядний
import-manuskript-importance-main = Основний
import-manuskript-progress-title = Імпортування Manuskript проекту …
import-manuskript-cancel-import = Скасувати
import-manuskript-cancelled = Імпорт скасовано
import-manuskript-done = Імпортовано елементів: { $imported }. Також перенесено попередніх версій: { $revisions }.

import-manuskript-open-now = Відкрити зараз
import-manuskript-warnings = { $count ->
    [one] { $count } річ, яку слід знати про це імпорт
    [few] { $count } речі для знання про це імпорт
    [many] { $count } речей, які слід знати про це імпорт
   *[other] { $count } речей, які слід знати про це імпорт
}
import-manuskript-details = Подробиці
import-manuskript-warnings-title = Про це імпорт
import-manuskript-error-title = Не вдалося імпортувати проект
import-manuskript-nested-too-deep = Проект не було імпортовано: його файл "{ $part }" в гнізді більше, ніж { $limit } глибини рівнів. Ні Manuskript проект наближається до того, і читання може розбитись SkribistoНічого не написано.
import-manuskript-folders-too-deep = Проект не було імпортовано: один з його файлів знаходиться у глибині понад { $limit } тек. Наближається жоден проект Manuskript, який читається, він може аварійно завершити роботу Skribisto. Нічого не написано.
import-manuskript-declares-entities = Проект не було імпортовано: його файл "{ $part }" визначає елементи XML, яких немає Manuskript project. Розширювання їх може зайняти більше пам' яті, ніж було на комп' ютері і аварійно SkribistoНічого не написано.
import-manuskript-error-details = Подробиці

import-target-open-title = Цей проект відкритий
import-target-open-text = " { $name } " відкрито у Skribisto. Імпортування за його допомогою замінить проект, над яким ви працюєте, і ваше наступне збереження напише цей проект під час імпортування. Оберіть іншу назву або спочатку закрийте проект.
import-warnings-announcement = { $result } { $notice }.

export-title = Експорт
export-close = Закрити
export-scope-label = Що
export-format-label = Формат
export-style-label = Стиль
export-path-label = Файл
export-browse = Огляд…
export-show-non-exportable = Показувати непридатні
export-choose-empty = Немає проекту, з якого слід обирати.
export-cancel = Скасувати
export-export = Експорт
export-save-dialog-title = Експортувати до файла
export-section-what = Що експортувати
export-section-style = Стиль
export-section-destination = Призначення
export-custom-selection = Нетиповий вибір
export-selected-count = Вибрано { $count }
export-chip-chapters-none = Глави: немає
export-chip-chapters-numbered = Глави: номер
export-chip-chapters-title = Глави: лише назва
export-chip-chapters-both = Глави: номер + заголовок
export-chip-scene-break-glyph = Розрив сцен: { $glyph }
export-chip-scene-break-blank = Розрив сцени: порожній рядок
export-chip-scene-break-none = Розрив сцен: немає
export-chip-major-break-glyph = Основна перерва { $glyph }
export-chip-major-break-blank = Головна перерва: порожній рядок
export-chip-major-break-none = Основна перерва: немає
export-chip-major-break-same = Розбити обидва титри
export-chip-spacing-single = Інтервал: одинарний
export-chip-spacing-onehalf = Проміжки: 11⁄2
export-chip-spacing-double = Інтервал: double
export-chip-notes-included = Включені нотатки
export-chip-notes-excluded = Нотатки виключено
export-format-docx = Слово
export-format-odt = LibreOffice
export-format-html = HTML
export-format-markdown = Markdown
export-format-djot = Djot
export-format-text = Текст
export-format-latex = LaTeX
export-format-epub = EPUB
export-format-pdf = PDF
export-overwrite-title = Замінити існуючий файл?
export-overwrite-text = "{ $name }" Вже існує.
export-progress-title = Експортування до …
export-cancelled = Експорт скасовано
export-done = Експортовано { $count ->
    [one] { $count } елемент
    [few] { $count } елементи
    [many] { $count } елементів
   *[other] { $count } елементи
}
export-open-file = Відкрити
export-show-in-folder = Показувати у теці
export-error-title = Не вдалося експортувати
export-error-details = Подробиці

backup-progress-start = Запуск…
backup-progress-retention = Очищення старих резервних копій…
backup-progress-done = Готово
backup-progress-destination = Призначення { $i } з { $n }
backup-details = Подробиці
backup-issues-title = Проблеми з резервним копіюванням
backup-failed-title = Резервування зазнало невдачі
backup-complete-prune-warning = Резервування завершено ({ $ok }), { $skipped } вже було збережено. Деякі застарілі резервні копії не вдалося вилучити

search = Пошук
search-query-placeholder = Пошук…
search-replace-placeholder = Замінити на…
search-replace-toggle = Перемкнути заміну
search-replace-all = Замінити все
search-opt-case = З врахуванням регістру
search-opt-whole-word = Ціле слово
search-opt-diacritics = Відповідні акценти
search-scope-body = Текст
search-scope-title = Заголовок
search-scope-synopsis = Синопсис
search-scope-label = Підпис
search-facet-book = Книги
search-facet-part = Частини
search-facet-chapter = Глави
search-facet-scene = Сцени
search-facet-note = Нотатки
search-facet-folder = Теки
search-tip-case = Регістр порівняння: трактувати верхній і нижній регістр як інший, тому " Елена " і " елена " - це окремі відповідники.
search-tip-whole-word = Ціле слово: відповідає лише повним словам, отже " cat " не знаходиться у " категорії ."
search-tip-diacritics = Відповідні акценти: відмінні акцентовані літери, тому " cape " не відповідає " café ."
search-tip-body = Текст: пошук в основному тексті елементів.
search-tip-title = Заголовок: пошук у заголовках елементів навігатора.
search-tip-synopsis = Синопсис: пошук у синопсисі кожного елемента.
search-tip-label = Підпис: пошук у короткій примітці під заголовком елемента.
search-tip-comment = Коментарі: пошукайте у тексті гілки коментарів і їх відповіді. Коментар пов' язано з рукописом, а не з його частиною, отже, у ньому є власний перемикач, а заміна рядка коментарів буде розглянуто, аж доки ви їх не знімете.
search-tip-book = Книжки: контейнер книги і її початкові або кінцеві позначки.
search-tip-part = Частини: розділювачі частин.
search-tip-chapter = Глави: розділи, у яких їх зберігає проект.
search-tip-scene = Картини.
search-tip-note = Нотатки: безформні нотатки.
search-tip-folder = Теки: просте впорядкування тек і роздільників.
search-tip-paratext = Текст, що належить до книги, але не до її оповідання: передмова, присвячення, послід. Ніколи не враховувався в рукописі.
search-tip-preserve-case = Регістр збереження: заміна приймає випадок, який було знайдено, отже " ELENA " стає " MARA ," а " Elena " - " Marta ."
search-tip-replace = Замінити: показати поле заміни і Замінити всі.
search-error = Помилка пошуку: { $message }
search-no-matches = Без відповідників
search-count =
    { $matches ->
    [one] { $matches } збіг
        [few] { $matches } збіги
    [many] { $matches } збігів
   *[other] { $matches } збігається
    } дюймunit description in lists { $items ->
    [one] { $items } документ
        [few] { $items } документи
    [many] { $items } документів
   *[other] { $items } документи
    }
search-count-truncated =
    { $matches ->
    [one] { $matches } збіг
        [few] { $matches } збіги
    [many] { $matches } збігів
   *[other] { $matches } збігається
    } дюймunit description in lists { $items ->
    [one] { $items } документ
        [few] { $items } документи
    [many] { $items } документів
   *[other] { $items } документи
    } (лише перший результат)
search-occurrences = { $count }
search-field-body = Текст
search-field-title = Заголовок
search-field-synopsis = Синопсис
search-field-label = Підпис
search-field-epigraph = Епіграф
search-field-comment = Коментар
search-field-comment-reply = Відповісти
search-field-footnote = Примітка
search-include-in-replace = Включити до заміни всі
search-collapse-all = Згорнути всі результати
search-replace-here = Замінити
search-dismiss = Вийти з результатів
search-undo-dismiss = Повернути останній відділений результат
search-replace-nothing = (Нічого)
search-replace-confirm-title = Замінити всі збіги?
search-replace-confirm-text = Замінити збіги «{ $query }» на «{ $replacement }»? Кількість збігів: { $occurrences }; документів: { $items }. Після цього дію можна скасувати з повідомлення.

search-replace-done-title = Замінити завершено
search-replace-done = Замінено збігів: { $occurrences }; документів: { $items }.

search-replace-done-skipped = Замінено збігів: { $occurrences }; документів: { $items }. Пропущено полів, змінених після пошуку: { $skipped }.

search-replace-undo = Скасувати дію
search-replace-failed-title = Спроба заміни завершилася невдало
search-replace-skipped-title = Нічого не замінено
search-replace-skipped-body = Після пошуку змінилися тексти. Кількість: { $fields }. Їх залишено без змін. Повторіть пошук, щоб побачити актуальні результати.

search-replace-undo-failed-title = Спроба скасування зазнала невдачіNoun, name of the user action
search-preview = Перегляд
search-preview-empty = Виберіть тут результат для перегляду
search-preview-no-prose = Цей збіг не має придатного для редагування тексту
search-preview-prompt = Щоб побачити тут попередній перегляд, виконайте пошук.
search-preview-open-search = Пошук у проєкті
search-preview-footnote-prompt = Цей збіг знаходиться у власному тексті примітки. Відкрийте його у полі Списки, щоб побачити і змінити його.
search-preview-open-footnotes = Відкрити список

find-placeholder = Знайти у цьому документі…
find-count = { $current } з { $total }
find-no-results = Без результатів
find-close = Закрити
find-previous = Попередній збіг (Shift+Ентр)
find-next = Наступний збіг (Enter)
find-opt-case = З врахуванням регістру
find-opt-whole-word = Ціле слово
find-replace-toggle = Показати або приховати заміну (Ctrl+R)
find-replace-placeholder = Замінити на…
find-replace = Замінити
find-replace-all = Замінити все
find-preserve-case = Зберігати регістр

dict-download-title = Звантаження { $name }…
dict-download-done = Встановлено { $name }
dict-download-failed = Не вдалося звантажити { $name }: { $error }
dict-removed = Вилучено { $name }
dict-accept-first = Прийняти ліцензію перед звантаженням { $name }
settings-dict-tab-installed = Встановлено
settings-dict-tab-get-more = Завантажити ще
settings-dict-tab-personal = Особисті слова
dict-installed-empty = На цьому комп' ютері ще не знайдено словників.
dict-get-more-search = Пошук мов
dict-system-badge = у вашій системі
dict-unusable-badge = непридатний
dict-download-button = Звантажити
dict-downloading = Звантаження…
dict-installed-label = Встановлено
dict-remove = Вилучити
dict-view-license = Переглянути ліцензію
dict-approx-size = ~{ $size }
dict-personal-empty = У цьому проекті ще немає особистих слів.
dict-personal-add = Додати
dict-personal-placeholder = Додати слово…
dict-missing-toast = { $count ->
    [one] Цей проект використовує { $count } словник, якого ви ще не встановили
    [few] Цей проект використовує { $count } словники, яких ви не встановили
    [many] Цей проект використовує { $count } словників, яких ви ще не встановили
   *[other] Цей проект використовує { $count } словників, яких ви не встановили
}
dict-missing-action = Отримати словники
dict-license-title = { $name } ліцензія
dict-license-accept = Прийняти звантаження
dict-license-cancel = Скасувати
dict-license-close = Закрити
dict-add-button = Додати словник…
dict-add-title = Додати словник
dict-add-close = Закрити
dict-add-name = Назва
dict-add-name-placeholder = Наприклад, мій латинський словник
dict-add-code = Код мови
dict-add-code-placeholder = наприклад, la або fr-FR-x-custom
dict-add-code-hint = Короткий теґ обраної вами мови. Це те, що ви обрали як мову документа.
dict-add-aff = Файл афіксів (.aff)
dict-add-dic = Список слів (dic)
dict-add-submit = Додати
dict-add-cancel = Скасувати
dict-add-name-required = Дайте словнику ім'я.
dict-add-code-required = Введіть код мови.
dict-add-code-invalid = Використовуйте тільки літери, цифри і - _.
dict-add-code-reserved = Це вбудований код словника. Виберіть інший.
dict-add-file-required = Виберіть файл.
dict-add-file-missing = Цей файл не існує.
dict-add-code-taken = Словник для цього коду вже встановлено.
dict-add-done = Додано { $name }
dict-add-unusable = Ці файли не придатні для використання: { $error }
dict-add-failed = Не вдалося додати словник: { $error }
inspector-export = Експорт
inspector-exportable = Додавати до експорту
ctx-number = &Нумерувати цю главу
ctx-unnumber = &Не нумерувати цю главу
inspector-numbering = Нумерація
inspector-numbered = Пронумеровано
inspector-numbered-tip = Цей розділ займає своє місце в нумерації книги.
inspector-numbered-tip-more = Незчисленна глава залишається у тому самому вигляді, у якому вона була: її заголовок, проза і кількість слів у ній не змінено. Вона просто не друкує жодного номера, і не використовує його вгору: глава після прологи - це перша глава, а не друга. Якщо ви полишите її замість експорту, буде використано інший перемикач, описаний вище, - розділ буде вилучено з книги взагалі.
inspector-apply-to-children = Застосувати до дочірніх елементів
inspector-milestone = Дата етапу
inspector-milestone-none = Без дати
inspector-milestone-clear = Очистити
inspector-tags = Мітки
inspector-aliases = Інші імена
inspector-dict-language = Мова
inspector-apply-language-to-children = Застосувати мову до дочірніх елементів
settings-page-language = Мова
settings-field-dict-language = Мови
dict-tradeoff-hint = Кожна додаткова мова приймає більше слів, отже менше помилок ловлять.
lang-inherit-hint = Успадковується: у цій сцені використовується мова або мова проекту.
lang-pill-list = Мови
lang-pill-add = Додати мову
lang-pill-remove = Вилучити { $name }
lang-pill-mute = Вимкнути перевірку правопису на { $name }
lang-pill-unmute = Увімкнути перевірку правопису на { $name }

settings-page-personal-dictionary = Особистий словник
settings-user-dict-desc = Слова, які ви тут додаєте, вважаються правильними в усьому додатку і ніколи більше не фіксують.
settings-user-dict-add-placeholder = Додати слово…
settings-user-dict-add = Додати слово
settings-user-dict-search = Слова фільтрування
settings-user-dict-count = { $count ->
    [one] { $count } слово
    [few] { $count } слова
    [many] { $count } слів
   *[other] { $count } слова
}

settings-user-dict-rename = Перейменувати
settings-user-dict-remove = Вилучити
settings-user-dict-empty = Немає слів. Додайте слова, які перевірку правопису слід ігнорувати, або імпортуйте список.
settings-user-dict-import = Імпорт …
settings-user-dict-export = Export…
settings-user-dict-import-tip = Додати слова з звичайного текстового файла, одне слово на рядок. Існуючі слова зберігаються.
settings-user-dict-export-tip = Зберегти весь список до звичайного текстового файла, одного слова у рядку.
settings-user-dict-txt-filter = Текстові файли
settings-user-dict-duplicate = Вже у словнику
settings-user-dict-imported = Імпортовано { $count ->
    [one] { $count } слово
    [few] { $count } слова
    [many] { $count } слів
   *[other] { $count } слова
} ({ $duplicates } вже є).
settings-user-dict-import-failed = Не вдалося прочитати список слів: { $error }
settings-user-dict-exported = Збережено { $count ->
    [one] { $count } слово.
    [few] { $count } слова.
    [many] { $count } слов.
   *[other] { $count } слів.
}
settings-user-dict-export-failed = Не вдалося зберегти список слів: { $error }

settings-page-text-replacements = Заміна тексту
settings-text-repl-desc = Замінити скорочення на повний текст після введення: " btw " стає " до речі ," коли ви вводите пробіл або розділовий знак.
settings-text-repl-enable = Використовувати заміну тексту у цьому проекті
settings-text-repl-disabled-hint = Позначте цей пункт, щоб визначити скорочення, яке буде розширюватися під час написання.
settings-text-repl-add = Додати правило
settings-text-repl-trigger-placeholder = Скорочення
settings-text-repl-replacement-placeholder = Що з цього вийде
settings-text-repl-added = Додано "{ $trigger }"
settings-text-repl-duplicate = У " { $trigger } " вже є правило
settings-text-repl-filter = Правила фільтрування
settings-text-repl-count = { $n ->
    [one] { $n } правило
    [few] { $n } правила
    [many] { $n } правил
   *[other] { $n } правила
}
settings-text-repl-row-enabled = Використовувати це правило
settings-text-repl-delete = Вилучити правило { $trigger }
settings-text-repl-deleted = Вилучено правило для{ $trigger }"
settings-text-repl-empty = Жодних правил.
settings-text-repl-csv-filter = CSV файли
settings-text-repl-import = Імпорт …
settings-text-repl-export = Export…
settings-text-repl-imported = Імпортовано { $added }, пропущено { $skipped }
settings-text-repl-exported = Експортовано { $n ->
    [one] { $n } правило
    [few] { $n } правила
    [many] { $n } правил
   *[other] { $n } правила
}

editor-menu-no-suggestions = Немає пропозицій
editor-menu-add-to-dictionary = Додати до словника " { $word } "
editor-menu-add-words-to-dictionary = Додати вибрані слова до словника
editor-dict-added = Додано "{ $word }" до вашого словника.
editor-dict-added-multi = { $count ->
    [one] До вашого словника додати { $count } слово.
    [few] Додав { $count } слова до вашого словника.
    [many] Додав { $count } слів до вашого словника.
   *[other] До вашого словника додано { $count } слів.
}
toast-undo = Скасувати дію

undo-superseded-title = Цей крок вже не останній.
undo-superseded-body = У цьому проекті щось змінилось з того часу, як. За допомогою пункту Вернути можна повернути назад, отже, цей проект вже не було змінено. Скористайтеся пунктом меню Правка ▸ Вернути, щоб повернутися до попереднього кроку.
undo-failed = Спроба скасування зазнала невдачі: { $error }

menu-edit = &Редагування
menu-edit-undo = &Скасувати дію
menu-edit-redo = &Повторити
menu-edit-undo-target = &Скасувати: { $target }
menu-edit-redo-target = &Повторити: { $target }
shortcut-name-edit-undo = Скасувати дію
shortcut-name-edit-redo = Повторити
undo-target-typing = набирання
undo-target-project = остання зміна у цьому проекті
prose-history-reset-title = Перезапуск історії
prose-history-reset-body = Відкриті сцени відновлено з попереднього стану. Кількість: { $count }. Історія введення для них більше не застосовується.

undo-target-trash = пересування до смітника
undo-target-restore = відновлення зі смітника
undo-target-delete-forever = вилучення на користь
undo-target-replace-all = заміна проекту
undo-target-import = імпортування документа
undo-target-duplicate = дублювання
undo-target-move = пересування
undo-target-merge = об' єднання двох сцен
undo-target-split = Розбиття сцени
undo-target-promote = зміна типу
undo-target-tidy-titles = Очищення заголовків розділів
undo-target-import-tags = Імпорт міток
undo-target-import-templates = Імпорт шаблонів нотаток
undo-target-create = створення
undo-target-remove = вилучення
undo-target-rename = перейменування
undo-target-edit = що редагується
menu-edit-find = &Знайти…
menu-edit-find-next = Зн&айти далі
menu-edit-find-prev = Зна&йти позаду
menu-edit-replace = З&найти і замінити…
menu-edit-find-in-project = Знай&ти в проекті…
menu-edit-replace-in-project = За&мінити в проекті…
undo-frozen = Вернути (" Завжди вперед ")
redo-frozen = Повторити (" Завжди вперед ")
menu-edit-cut = В&ирізати
menu-edit-copy = &Копіювати
menu-edit-paste = &Вставити
menu-edit-paste-plain = Вставити б&ез форматування
menu-edit-select-all = Ви&брати все
shortcut-name-edit-cut = Вирізати
shortcut-name-edit-copy = Копіювати
shortcut-name-edit-paste = Вставити
shortcut-name-edit-paste-plain = Вставити без форматування
shortcut-name-edit-select-all = Вибрати все

titlebar-spellcheck-on = Перевірка правопису увімкнено. Клацніть, щоб зупинити перевірку (F7)
titlebar-spellcheck-off = Перевірка правопису вимкнено. Натисніть, щоб знов перевірити (F7)
menu-spellcheck = &Перевірити правопис
menu-comments = &Коментарі
settings-page-spellcheck = Перевірка правопису
settings-group-spellcheck = Перевірка правопису
settings-spellcheck-enabled = Перевіряти правопис
settings-spellcheck-hint = Підкреслює слова, яких не знає жоден з встановлених словників. Вимикання цього параметра зупиняє перевірку у кожному проекті, аж доки ви не увімкнете його. Щоб припинити перевірку лише однієї мови, зніміть позначку з поля Мова роботи або запису.

menu-trash = &Кошик
trash-title = Кошик
trash-empty-state = Смітник порожній.
trash-empty-button = Очистити кошик…
trash-restore = &Відновити
trash-restore-to = &Відновити до…
trash-delete-forever = &Вилучити назавжди
trash-restored-ok = { $count } Відновлено.
trash-restore-error = Не вдалося відновити: { $error }
trash-restore-orphaned = Початкове місце цього елемента зникло. Виберіть, де його відновити.
trash-restore-no-project = Жоден проект не відкритий, тому нема чого відновити.
trash-delete-no-project = Не відкрито жодного проекту, отже немає чого вилучати.
trash-empty-confirm-title = Випорожнити сміття?
trash-empty-confirm-text = { $count ->
    [one] { $count } викинутий запис буде назавжди вилучено.
    [few] { $count } вилучені записи буде назавжди вилучено.
    [many] { $count } Витерті з пам' яті записи.
   *[other] { $count } Записи буде вилучено назавжди.
} Вони назавжди полишають смітник. Вернути може повернути їх до того часу, доки ви не зробите щось інше.
trash-emptied-title = Смітник спорожнено
trash-emptied-body = Все у смітнику було назавжди вилучено.
trash-empty-no-project = Не відкрито жодного проекту, отже немає сміття для спорожнення.
trash-delete-forever-confirm-title = Витерти назавжди?
trash-delete-forever-confirm-text = { $count ->
    [one] { $count } елемент буде назавжди вилучено.
    [few] { $count } елементи буде назавжди вилучено.
    [many] { $count } елементів буде назавжди вилучено.
   *[other] { $count } елементи буде назавжди вилучено.
} Вони назавжди полишають смітник. Вернути може повернути їх до того часу, доки ви не зробите щось інше.
trash-deleted-title = Вилучено назавжди
trash-deleted-body = { $count } назавжди вилучено.
trash-undo = Скасувати дію
trash-restore-picker-title = Відновити до…
trash-restore-to-confirm-title = Відновлювати тут?
trash-restore-to-confirm-text = Відновити " { $item } " в " { $destination }"?
trash-restore-picker-restore-here = Відновити тут
trash-restore-picker-cancel = Скасувати
trash-restore-picker-empty = Ще немає прив' язувачів. Створіть перший.
trash-banner-title = Цей елемент знаходиться у смітнику
trash-banner-description = Він не з'явиться в контурі чи експорті, поки не відновить його.
trash-banner-restore = Відновити…
trash-tab-tooltip = У смітнику

settings-styles-sheet-yes = Так
settings-styles-sheet-no = Ні
settings-styles-sheet-font = Шрифт
settings-styles-sheet-indent = Відступ у першому рядку
settings-styles-sheet-para-spacing = Інтервал між абзацами
settings-styles-sheet-page = Розмір сторінки
settings-styles-sheet-margins = Поля (верх/праворуч/низ/ліворуч)
settings-styles-sheet-title-page = Сторінка заголовка книги
settings-styles-sheet-heading-language = Мова заголовків
settings-styles-sheet-auto = За текстом
settings-styles-sheet-digits = Числа
settings-styles-sheet-direction = Напрям тексту
settings-styles-sheet-formats = Формати
settings-styles-sheet-all-formats = Всі

settings-styles-editor-group = Редактор стилів
settings-styles-editor-missing = Такого стилю вже немає.

settings-styles-page-letter = Letter
settings-styles-digits-western = Західний округrussia_ subjects. kgm
settings-styles-digits-eastern-arabic = Східна 0. 5
settings-styles-direction-ltr = Зліва праворуч
settings-styles-direction-rtl = Справа ліворуч

format-dock-title = Формат
format-panel-empty = Розташуйте курсор у вікні, ноті або синопсі, щоб побачити параметри форматування.
format-group-marks = Текст
format-group-block = Абзац
format-group-lists = Списки
format-group-tables = Таблиця
format-group-breaks = Переривання сцен
format-superscript = Надрядковий
format-subscript = Підрядковий
format-link = Посилання…
link-dialog-insert-title = Вставити посилання
link-dialog-edit-title = Редагувати посилання
link-dialog-text-label = Текст
link-dialog-text-placeholder = слова, які бачить читач
link-dialog-url-label = Посилання на
link-dialog-url-placeholder = example.com
link-dialog-insert = Вставити
link-dialog-apply = Застосувати
link-dialog-cancel = Скасувати
link-dialog-remove = Вилучити посилання
link-scheme-refused = Ви можете відкривати лише посилання на веб і повідомлення електронної пошти. { $url } залишилася одна.
menu-format-link = &Посилання…
format-clear = Очистити форматування
format-heading = Рівень заголовка
format-heading-normal = Звичайний текст
format-heading-1 = Заголовок 1
format-heading-2 = Заголовок 2
format-heading-3 = Заголовок 3
format-heading-4 = Заголовок 4
format-heading-5 = Заголовок 5
format-heading-6 = Заголовок 6
format-align-left = Вирівняти ліворуч
format-align-center = По центру
format-direction-rtl = Параграф з правого боку
format-blockquote = Цитата
format-list-bullet = Маркований список
format-list-numbered = Пронумерований список
format-indent = Збільшити відступ
format-outdent = Зменшити відступ
format-table-insert = Вставити таблицю
format-table-row-above = Вставити рядок вище
format-table-row-below = Вставити рядок нижче
format-table-col-before = Вставити стовпчик раніше
format-table-col-after = Вставити стовпчик після
format-table-row-delete = Вилучити рядок
format-table-col-delete = Вилучити стовпчик
format-table-remove = Вилучити таблицю

menu-format-marks-bold = &Жирний
menu-format-marks-italic = &Курсив
menu-format-marks-underline = П&ідкреслення
menu-format-marks-strike = З&акреслення
menu-format-marks-superscript = На&дрядковий
menu-format-marks-subscript = Під&рядковий
menu-format-marks-clear = &Очистити форматування
menu-format-heading = &Заголовок
menu-format-heading-normal = Зв&ичайний текст
menu-format-heading-1 = &Заголовок 1
menu-format-heading-2 = З&аголовок 2
menu-format-heading-3 = За&головок 3
menu-format-heading-4 = Заг&оловок 4
menu-format-heading-5 = Заго&ловок 5
menu-format-heading-6 = Заголо&вок 6
menu-format-alignment = В&ирівнювання
menu-format-align-left = &Вирівняти ліворуч
menu-format-align-center = &По центру
menu-format-direction = &Напрямок
menu-format-direction-auto = &Автоматично
menu-format-direction-ltr = &Зліва праворуч
menu-format-direction-rtl = &Справа ліворуч
menu-format-blockquote = &Цитата
menu-format-lists = &Списки
menu-format-list-bullet = &Маркований список
menu-format-list-numbered = &Пронумерований список
menu-format-indent = З&більшити відступ
menu-format-outdent = &Зменшити відступ
menu-format-table = &Таблиця
menu-format-table-insert = &Вставити таблицю
menu-format-table-2x2 = &2 x 2
menu-format-table-3x3 = &3 x 3
menu-format-table-4x4 = &4 x 4
menu-format-table-row-above = Вс&тавити рядок вище
menu-format-table-row-below = Вставити &рядок нижче
menu-format-table-col-before = Вст&авити колонку перед
menu-format-table-col-after = В&ставити стовпчик після
menu-format-table-row-delete = Вил&учити рядок
menu-format-table-col-delete = В&илучити стовпчик
menu-format-table-remove = Ви&лучити таблицю

menu-format-typo-marks = &ФорматуванняComment
menu-format-typo-nbsp = &Пробіл
menu-format-typo-nbhyphen = &Гіп.
menu-format-typo-soft-hyphen = &М’який перенос
menu-format-typo-nnbsp = &Вузький беззбройний простір
menu-format-typo-zwsp = &Пробіл нульової ширини
menu-format-typo-word-joiner = &З’єднувач слів


menu-about = П&ро програму Skribisto…
about-title = Близько Skribisto
about-version = Версія { $version }
about-update-available = Версія { $version }
about-update-see-notes = Див. що змінилося
about-update-flathub = Флетх тримає цю копію до сьогодні.
about-update-distro = Ваш розподіл зберігає цю копію до сьогодні.
about-tagline = Програма для написання романів для довгої фантастики, написана у стилі "Тексіло."
about-license = Випущено за умов дотримання GNU General Public License, версія 3.
about-copyright = © 2026 Cyrill Jacquet
about-close = Закрити

native-menu-about = Близько { $app }
native-menu-hide = Сховати { $app }
native-menu-quit = Вийти з { $app }
native-menu-settings = Параметри…
native-menu-window = Вікно
native-menu-minimize = Згорнути
native-menu-zoom = Масштабування

window-title = { $title } - { $app }
window-title-numbered = { $title } - { $app } (Вікно { $n })
window-title-empty = { $app }

first-run-window-title = Ласкаво просимо
first-run-title = Налаштування { $app }
first-run-body = { $app } зберігає свої параметри окремо від Skribisto, отже, він починається з порожнього. Ваші налаштування, нещодавні проекти, словники та компонування вікон можна зараз скопіювати.
first-run-from = Копіювати з
first-run-to = Скопіювати до
first-run-copy-note = Нічого не переміщується або видаляється. Skribisto зберігають всі свої власні параметри і продовжують працювати точно так, як раніше.
first-run-import = Параметри імпорту
first-run-start-fresh = Почати з початку
first-run-import-failed = Деякі параметри неможливо імпортувати: { $error }

settings-page-punctuation = Типові значення пунктуації
settings-group-punctuation = Кмітливий розділювач
settings-page-work-punctuation = Пунктуація
settings-punctuation-override = Надати цьому проекту власні правила пунктуації
settings-punctuation-override-hint = Вимкнено: параметри проекту відповідають пріоритету програми. Ці правила подорожують у межах .skrib, отже, один з авторів відкриває файл з тією самою типографією.
settings-punctuation-dashes = Перетвориться на кільчак, --- на тире
settings-punctuation-ellipsis = Перетворити... на еліпс
settings-punctuation-quotes = Позначки завитків і апострофиKCharselect unicode block name
settings-quote-style = Позначки цитати
settings-quote-style-locale = Типове значення мови
settings-quote-style-curly = Дубле!
settings-quote-style-curly-single = Озера
settings-quote-style-guillemets = Е- ҐольметсCity in Alberta Canada
settings-quote-style-low-high = ♪Low- high"
settings-punctuation-spacing = Простір перед:!
settings-punctuation-spacing-hint = Французька друкарська машинка встановлює вузьке непрохідне місце перед ;!? і повне місце перед:. Воно стосується лише тексту, написаного французькою мовою. Простір всередині Guillemets  сягне разом з guillmem цитуваннями.
settings-punctuation-sample = Надано вашою мовою
settings-punctuation-app-hint = Що кожен проект робить, якщо він не приймає власних правил у проекті ▸ пунктуація.
settings-punctuation-dialogue = Відкрити абзац, введений як " - " з дефісом
settings-punctuation-dialogue-hint = Для мов, які позначають мову не лапками, а фразою: французькою, іспанською, російською та іншими.

statusbar-go-to = Перейти до…
go-to-placeholder = Шукати прив' язку
go-to-no-matches = Жодного елемента не відповідає цьому пошуку.
menu-go-to = &Перейти до…

settings-page-distraction-free-themes = Відокремлені теми
settings-themes-builtin = Вбудовані теми
settings-themes-builtin-badge = Вбудований
settings-themes-user = Мої теми
settings-themes-editor-group = Змінити тему
settings-themes-editor-empty = Виберіть тему на сторінці " Моя тема ," щоб її змінити.
settings-themes-use = Користування
settings-themes-duplicate = Дублювати
settings-themes-edit = Редагування
settings-themes-delete = Видалити
settings-themes-export = Export…
settings-themes-import = Імпорт …
settings-themes-copy-suffix = копіювати
settings-themes-json-filter = Тема (JSON)
settings-themes-imported = Тему імпортовано
settings-themes-import-failed = Не вдалося імпортувати цю тему
settings-themes-exported = Тему експортовано
settings-themes-export-failed = Не вдалося експортувати тему
settings-themes-low-contrast = мала контрастність
settings-themes-low-contrast-band = сховати текст
settings-themes-field-name = Назва
settings-themes-field-paper = Сторінка
settings-themes-field-ink = Текст
settings-themes-field-general = Тло
settings-themes-field-widget-text = Керування текстом стрічки
settings-themes-field-caret-band = Підсвітити навколо каркасу

statusbar-focus-settings = Параметри розв' язання
statusbar-focus-manage-themes = Керування темами…

comments-title = Коментарі
comments-document-title = Цей документ
comments-empty-project = У цьому проекті ще немає коментарів.
comments-empty-document = Немає коментарів щодо цього документа.
comments-filter-all = Всі
comments-filter-open = Відкрити
comments-filter-resolved = Вирішено
comments-filter-orphaned = Без прив’язки до тексту
comments-status-open = Відкрити
comments-status-resolved = Вирішено
comments-status-orphaned = Початковий текст утрачено
comments-orphan-snippet = (відсутній коментар)
comments-status-unplaced = Без розташування тексту
comments-unplaced-snippet = (не прив' язано до жодного тексту)
comments-reply-count = Відповідей: { $count }

comments-menu-resolve = Позначити вирішеним
comments-menu-reopen = Повторно відкрити
comments-menu-delete = Видалити
comments-sort-document = У порядку документів
comments-sort-newest = Спочатку найновіші
comments-sort-reader = Груповано за читанням
comments-unsigned-author = Без підпису
comments-agreement = Читачів: { $count }

comments-menu-add = Додати коментар
comments-menu-add-paragraph = Коментар до цього абзацу
overview-col-comments = Коментарі
overview-col-total-comments = Загалом коментарів
comments-card-placeholder = Записати коментар…
comments-card-unknown-author = Невідомий автор
comments-unsigned-toast = Ваші коментарі непідписані: на цьому комп' ютері не вказано імені.
comments-unsigned-action = Вкажіть ваше ім' я
comments-card-reply = Відповісти
comments-card-reply-placeholder = Відповісти…
comments-card-actions = Коментарі
comments-reply-actions = Відповісти
comments-menu-delete-reply = Вилучити відповідь
comments-menu-delete-all = Вилучити всі коментарі
comments-deleted-toast = Коментар вилучено
comments-reply-deleted-toast = Відповісти вилучено
comments-deleted-all-toast = Видалено коментарів: { $count }

comments-undo = Скасувати дію

analysis-segment = Аналіз
analysis-scope-book = Аналіз цієї книги
analysis-run = Виконати аналіз
analysis-stale = Змінено з часу виконання
analysis-not-run = Ще не проаналізовано.
analysis-running = Читання рукопису…
analysis-failed = Аналіз не вдалося завершити.
analysis-no-scenes = Ще немає сцен у цій книжці.

analysis-shape = Форма

analysis-arrivals = Прибуття
analysis-arrivals-explainer = Спосіб, у який текст дійшов до цього проекту, коли він був відкритий. У ньому показано маршрут, у якому було показано символи, і взагалі немає нічого про того, хто їх написав: автор, який вставляє чернетки в інше місце і вставляє, а той, хто диктує. У цьому випадку немає числа.
analysis-arrivals-scope = Весь цей проект, не тільки ця книга.
analysis-arrivals-session = Після того, як ви її відкрили. Закриття проекту знову починає рахунок.
analysis-arrivals-nothing = У цьому сеансі ще не було знайдено тексту.
analysis-arrivals-typed = Надруковано
analysis-arrivals-pasted = Вставлено
analysis-arrivals-dictated = Установлений
analysis-arrivals-imported = Імпортовано
analysis-arrivals-programmatic = Вставлено для вас
analysis-arrivals-count = Символів: { $count }

analysis-arrivals-none = немає

analysis-words-per-scene = Слова на сцену
analysis-median-words = Гравітація цієї книги { $count ->
    [one] { $count } слово.
    [few] { $count } слова.
    [many] { $count } слов.
   *[other] { $count } слів.
}
analysis-median-line = Медіана: { $count ->
    [one] { $count } слово
    [few] { $count } слова
    [many] { $count } слів
   *[other] { $count } слова
}
analysis-dialogue = Діалогове вікно
analysis-dialogue-unsupported = Діалогове вікно ще не вимірюється для цієї мови.

analysis-footnote-words = Примітка
analysis-footnote-words-count = Кількість слів у виносках цієї книги: { $count }. Вони обліковуються окремо від рукопису.

analysis-footnote-words-pending = Підрахунки…

analysis-ignore-empty = Ігнорувати тексти без запису
analysis-empty-hidden = Приховано порожніх текстів: { $count }.

analysis-all-texts-empty = Кожен текст у цій книжці все ще порожній.

binder-filter-count = { $shown } з { $total } показано
binder-filter-clear = Очистити
binder-filter-none = Нічого не збігається "{ $query }".

settings-paratext-intro = Сутність переднього і заднього плану, з якою може починатися новий проект. Кожна з структур належить до видавничої традиції, а назви її сторінок пишуться рідною мовою цієї традиції. Перейменуйте їх одразу після створення проекту.
settings-paratext-structures = Структури
settings-paratext-broken = Не вдалося прочитати
settings-paratext-edit = Редагування
settings-paratext-duplicate = Дублювати
settings-paratext-delete = Видалити
settings-paratext-new = Нова структура
settings-paratext-save = Зберегти
settings-paratext-editor-hint = Назва, сторінки, які слід створити перед рукописом, і сторінки, які слід створити після нього. Там, де вони опиняються, буде і ваше: вони є звичайними елементами після появи проекту.


image-insert = Вс&тавити зображення…
image-no-project = Відкрити проект перед вставленням зображення.
image-choose-title = Виберіть зображення
image-filter-label = Зображення
image-large-title = Це велике зображення
image-large-text =
    { $name } є { $megapixels } мегапікселів ({ $width }×{ $height }).
    Зберігати ваш початковий файл у проекті: він подорожує
    з кожною резервною копією і кожним експортом. Оптимізація зберігає менше
    Замість цього, до 2560 пікселів на найдовшій стороні.
image-large-keep = Зберегти початковий
image-large-downscale = Оптимізувати
image-large-remember = Відтепер, не питай.
image-not-recorded = Зображення було збережено, але його не вдалося записати у проекті.
image-describe-title = Опис зображення
image-describe-explain = Те, що показує картинка для читача, який її не бачить. Цей рукопис не є частиною рукопису: його ніколи не враховано, не обшукують і не експортують як прозу.
image-describe-placeholder = маяк біля сірого неба
image-resize-title = Змінити розмір зображення
image-resize-explain = Відсоток розміру, який буде показано. 100 залишають його у такому вигляді.
image-resize-invalid = Введіть число між 1 і 1000.
image-menu-describe = &Опис зображення…
image-menu-resize = &Змінити розмір зображення…
image-menu-reset-size = &Початковий розмір

cover-choose = О&бкладинка книги…
cover-clear = В&илучити обкладинку
cover-choose-title = Виберіть обкладинку
cover-set = Обкладинка готова.
cover-cleared = Обкладинку вилучено. Картина все ще в проекті.
card-label = Підпис
card-exportable = Експортовано
card-numbered = Пронумеровано
card-created = Створено
card-modified = Змінено
card-yes = так
card-no = ні
card-type = Тип
card-position = Позиція
card-children = Діти
card-goal = Мета
card-words = Слова
card-synopsis = Синопсис
card-point-of-view = Точка зору
card-aliases = Інші імена

export-orphan-footnotes-title = Примітки без посилання
export-comments-dropped = Не вдалося розмістити коментарі в експортованому тексті. Кількість: { $count }. Їх пропущено.

export-orphan-footnotes = У рукописі більше немає посилань на деякі виноски. Кількість: { $count }. Їхній текст не з’явиться в експортованій книзі.

menu-footnotes = &Примітки
footnotes-title = Примітки
footnotes-insert = &Вставити примітку
footnotes-empty = Підказок ще немає. Розташуйте курсор у вікні, а потім використовуйте +, або Ctrl+Alt+F.
footnotes-filter-all = Всі
footnotes-filter-document = Цей документ
footnotes-filter-orphaned = Без прив’язки до тексту
footnotes-orphaned = Більше нічого не потрібно
footnotes-untitled-home = Без назви
footnotes-body-placeholder = сама нота
footnotes-insert-tooltip = Вставити примітку до курсора (Ctrl+Alt+F)
footnotes-actions = Дії з текстом
footnotes-delete = &Вилучити нотатку і її посилання
footnotes-no-project = Відкрити проект перед вставленням примітки.
footnotes-no-caret = Переводить курсор у текст сцени, щоб вставити у це поле примітку.
footnotes-not-created = Не вдалося додати примітку до проекту.
footnotes-deleted-toast = Нотатку вилучено
footnotes-undo-delete = Скасувати дію

import-document-title = Імпортувати документи
import-document-close = Закрити
import-document-step-files = Файли
import-document-step-review = Перегляд
import-document-step-destination = Призначення
import-document-step-reconcile = Об’єднати
import-document-reconcile-hint = Деякі з них ви вже маєте.
import-document-reconcile-all-new = У цьому файлі нічого не відповідає вашому проекту: кожен елемент буде додано як новий.
import-document-duplicate-returns = Ці файли є більше ніж однією копією одного рукопису, який повертається до вашого проекту. Імпортувати їх по одному: не об' єднувати, а додавати ще одну копію.
import-document-hunk-added = Візьміть цей новий абзац
import-document-hunk-removed = Викинути цей абзац
import-document-hunk-changed = Підняти цей перепис
import-document-col-stray-prose = Текст
import-document-stray-as-paratext = Зберігати як текст
import-document-stray-discard = Скинути текст
import-document-col-current = У вашому проекті
import-document-col-incoming = У цьому файлі
import-document-col-status = Стан
import-document-col-action = Що робити
import-document-status-identical = Без змін
import-document-status-editor-edited = Змінено читачем
import-document-status-you-edited = Змінено вами
import-document-status-conflict = Змінено обома сторонами
import-document-status-different = Відрізняється
import-document-status-new = Новий
import-document-status-missing = Немає в цьому файлі
import-document-status-moved = Переміщено
import-document-action-comments-only = Лише коментарі
import-document-action-take-import = Прийняти цю версію
import-document-action-keep-current = Залишити мою
import-document-action-create-new = Додати як новий
import-document-action-ignore = Пропустити
import-document-compare = Порівняти
import-document-compare-legend = Ваша версія у порівнянні з версією цього файла. Лише для читання.
import-document-compare-close = Закрити
import-document-drop-title = Скиньте сюди документи
import-document-drop-hint = Markdown (.md), Слово (.docx), OpenDocument (.odt) і звичайний текст (.txt)
import-document-browse = Огляд…
import-document-move-up = Пересунути вгору
import-document-move-down = Пересунути вниз
import-document-remove-file = Вилучити
import-document-file-count = { $count ->
    [one] { $count } файл
    [few] { $count } файли
    [many] { $count } файлів
   *[other] { $count } файли
}
import-document-no-files = Ще не обрано файлів.
import-document-col-included = Імпорт
import-document-col-title = Заголовок
import-document-col-type = Тип
import-document-col-words = Слова
import-document-col-breaks = Розриви
import-document-col-comments = Коментарі
import-document-col-footnotes = Примітки
import-document-col-epigraph = Епіграф
import-document-col-source = Джерело

import-document-level-rules = Головні рівні
import-document-level-n = Заголовок { $level }
import-document-add-top-level = Додати верхній рівень
import-document-add-top-level-tooltip = Вставити книгу над кожним проаналізованим елементом, для файлів розділів, у яких немає заголовка книги
import-document-destination = Призначення
import-document-destination-hint = Оберіть складальник або елемент: нові елементи у теці або після сцени.
import-document-destination-empty = Ще немає прив' язувачів. Створіть перший.
import-document-plan-empty = Ще нема що імпортувати.
import-document-summary = { $rows ->
    [one] { $rows } елемент
    [few] { $rows } елементи
    [many] { $rows } елементів
   *[other] { $rows } елементи
} · { $breaks ->
    [one] Перерва на { $breaks } сцену
    [few] { $breaks } перерви
    [many] { $breaks } перерв
   *[other] { $breaks } розриви сцени
}
import-document-back = Назад
import-document-cancel = Скасувати
import-document-analyse = Далі
import-document-analysing = Читання документів…
import-document-step-analysing = Читання
import-document-cancel-analysis = Зупинити читання
import-document-analyse-failed = Документи неможливо прочитати.
import-document-details = Подробиці
import-document-import = Імпорт
import-document-done = Імпортовано елементів: { $count }

import-document-undo = Скасувати дію
import-diagnostic-file-unreadable = Не вдалося прочитати «{ $path }»: { $detail }. Інші файли буде імпортовано.

import-diagnostic-nested-too-deep = Файл «{ $path }» пропущено: його вміст має понад { $limit } рівнів вкладеності. Звичайним документам така вкладеність не потрібна, а читання може спричинити аварійне завершення Skribisto. Інші файли буде імпортовано.

import-diagnostic-archive-too-large = Файл «{ $path }» пропущено: після розпакування його розмір надто зростає, що може вичерпати пам’ять і спричинити аварійне завершення Skribisto. Інші файли буде імпортовано.

import-diagnostic-lossy-decode = Не вдалося декодувати символи у «{ $path }». Кількість: { $count }. Щоб зберегти їх, збережіть вихідний файл у кодуванні UTF-8.

import-diagnostic-decoded-from-bom = Файл «{ $path }» прочитано в кодуванні { $detail }, а не UTF-8.

import-diagnostic-empty-file = Файл «{ $path }» порожній.

import-diagnostic-no-headings = У «{ $path }» немає заголовків, тому вміст буде імпортовано як один елемент.

import-diagnostic-unsupported-format = Формат «.{ $detail }» не підтримується, тому файл «{ $path }» пропущено.

import-diagnostic-front-matter-not-flat = Метадані на початку «{ $path }»: значення «{ $detail }» має складну структуру, тому його пропущено.

import-diagnostic-footnotes-degraded = Виноски з «{ $path }» імпортовано як звичайний текст. Кількість: { $count }. Читання виносок з Markdown не підтримується.

import-diagnostic-footnote-not-carried = Не вдалося перенести виноски з «{ $path }». Кількість: { $count }. Решту перенесено.

import-diagnostic-raw-html-dropped = Пропущено блоки необробленого HTML у «{ $path }». Кількість: { $count }.

import-diagnostic-nested-break-dropped = Пропущено роздільники сцен усередині цитат або списків у «{ $path }». Кількість: { $count }. Зберігаються лише роздільники на окремому рядку.

import-diagnostic-image-not-ingested = Файл «{ $path }» посилається на зображення «{ $detail }». Посилання імпортовано як текст; саме зображення не скопійовано.

import-diagnostic-duplicate-title = Заголовок «{ $title }» трапляється { $count } разів. Якщо ці файли вже імпортувалися, буде створено дублікати.

import-diagnostic-heading-level-jump = Заголовок «{ $title }» переходить з рівня { $from } на { $to }; його буде розміщено на один рівень нижче батьківського елемента.

import-diagnostic-illegal-combination = Елемент «{ $title }» містить текст, але тип «{ $kind }» не може його зберігати. Змініть тип або виключіть елемент. До цього нічого не буде імпортовано.

import-diagnostic-tracked-changes-flattened = Файл { $path } містив незавершені виправлення. Прийнято змін: { $count }; вилучений текст видалено. Це остаточний текст. Щоб зберегти частину власного формулювання, скористайтеся порівнянням на останньому кроці.

import-diagnostic-tracked-changes-flattened-by = Файл { $path } містив незавершені виправлення авторів: { $names }. Прийнято змін: { $count }; вилучений текст видалено. Це остаточний текст. Щоб зберегти частину власного формулювання, скористайтеся порівнянням на останньому кроці.

import-diagnostic-text-box-dropped = У { $path } є текстові поля. Кількість: { $count }. Їхній текст розташований поза основним потоком документа, тому неможливо визначити його місце в рукописі. Ці поля не імпортуються.

import-diagnostic-embedded-object-dropped = У { $path } є вбудовані об’єкти: діаграми, рівняння тощо. Кількість: { $count }. Рукопис не підтримує такі об’єкти.

import-diagnostic-field-flattened = У { $path } є поля: номери сторінок, перехресні посилання або дати. Кількість: { $count }. Вони зберігають останній відображений текст і більше не оновлюються.

import-diagnostic-unknown-style-level = У { $path } використано стиль «{ $detail }», схожий на заголовок, але без указаного рівня. Такі абзаци імпортовано як звичайний текст.

import-diagnostic-comment-unanchored = Коментар «{ $detail }» у { $path } не вдалося прив’язати до початкових слів. Він залишається на своєму елементі, де його можна перемістити.

import-diagnostic-comment-not-carried = Коментар «{ $detail }» з { $path } не імпортовано, оскільки файл не містить тексту, до якого його можна прив’язати. За потреби скопіюйте коментар з вихідного файлу.

import-diagnostic-comment-replies-flattened = Відповіді у { $path } посилалися на відсутні коментарі. Кількість: { $count }. Їх імпортовано як окремі коментарі.

import-diagnostic-prose-not-verbatim = { $count ->
    [one] { $count } параграф у { $path } не може бути перекладений з його форматуванням точно так, як було написано, отже він приходить як звичайний текст. Порівняйте його з оригіналом.
    [few] { $count } абзаци { $path } не можуть бути переведені з їхнім форматуванням точно так, як було написано, отже, вони з'являються як звичайний текст. Порівняйте їх з оригіналом.
    [many] { $count } абзаців { $path } не можна перенести з їхнім форматуванням точно так, як було написано, отже, вони з'являються як звичайний текст. Порівняйте їх з оригіналом.
   *[other] { $count } абзаців { $path } не можна було перенести з їхнім форматуванням точно так, як це було написано, отже, вони постачаються як звичайний текст. Порівняйте їх з оригіналом.
}
import-diagnostic-styled-spaces-not-carried = { $count ->
    [one] У { $path }, було підкреслено або пропущено { $count } проміжок, рядок, який слід заповнити, наприклад. Skribisto не зберігає жодного рядка на пробілах, отже, він з' являється як звичайний пробіл, або взагалі не знаходиться там, де він починається або закінчується абзацом.
    [few] У { $path } { $count } ділянки порожнього місця було підкреслено або пропущено, рядки, які слід заповнити, наприклад. Skribisto не зберігає жодного рядка на пробілах, отже, вони прибувають у вигляді звичайних пробілів, або зовсім не мають місця, де вони починаються або закінчують абзац.
    [many] У { $path } { $count } ділянок порожнього простору було підкреслено або пропущено через, рядки, які слід заповнити, наприклад. Skribisto не зберігає жодного рядка на пробілах, отже, вони з' являються як звичайні пробіли, або зовсім не мають місця, де вони починаються або закінчуються абзацом.
   *[other] У { $path }, { $count } ділянках порожнього місця було підкреслено або пропущено через, рядки, які слід заповнити, наприклад. Skribisto не має жодного рядка на пробілах, отже, вони з' являються як звичайні пробіли, або зовсім не мають місця, де вони починаються або закінчуються абзацом.
}
import-diagnostic-list-nesting-flattened = { $count ->
    [one] { $count } пункт списку у { $path } був вкладений на більш ніж { $limit } рівнів. Він досягає рівня { $limit }, найглибший 8765414 зберігається зі своїми словами і форматуванням.
    [few] { $count } елементи у { $path } утворилися на глибині більше { $limit } рівнів. Вони досягають рівня { $limit }, найглибші 8765414 зберігаються зі словами і форматуванням.
    [many] { $count } елементів у { $path } утворились на глибині більше { $limit } рівнів. Вони опиняються на рівні { $limit }, найглибші 8765414 зберігаються зі словами і форматуванням.
   *[other] Список { $count } елементів { $path } було укладено на більш ніж { $limit } рівнів. Вони опиняються на рівні { $limit }, найглибші Skribisto зберігаються зі своїми словами і форматуванням.
}
import-diagnostic-table-read-as-paragraphs = { $count ->
    [one] { $count } таблиця у { $path } має рядок, набагато ширший за решту, занадто брукований, щоб розкластися як ґратка. Його комірки з' являються у вигляді абзаців, одна за одною, з їх словами і форматуванням.
    [few] { $count } таблиці у { $path } мають рядок, набагато ширший за решту, занадто гнучкий, щоб розкластися як ґратка. Їх комірки надсилаються як абзаци, один за одним, з їх словами і форматуванням.
    [many] { $count } таблиць у { $path } мають рядок, набагато ширший за решту, занадто гнучкий, щоб розкластися як ґратка. Їх комірки надсилаються як абзаци, один за одним, з їх словами і форматуванням.
   *[other] { $count } таблиці { $path } рядок, набагато ширший за решту, занадто близкий, щоб розкластися як ґратка. Їх комірки надсилаються як абзаци, один за одним, зі словами і форматуванням.
}
import-diagnostic-spaces-shortened = { $count ->
    [one] { $count } запуск проміжків у { $path } був довший за { $limit } проміжків і скоротився до { $limit }. Слова навколо нього всі.
    [few] { $count } пропуски у { $path } були довшими за { $limit } проходів, які скоротились до { $limit }. Слова навколо них є всі.
    [many] { $count } проходів у { $path } були довшими за { $limit } місць, які скоротились до { $limit }.
   *[other] { $count } запуск пробілів { $path } було більше, ніж { $limit } пробіли і прибуття до { $limit }Всі ці слова навколо них.
}
import-diagnostic-epigraph-not-carried = Над " { $title } " відповідає епіграф, але { $kind } не може тримати один. Цитатата зберігається у верхній частині тексту.
import-diagnostic-epigraph-placement-ambiguous = Епіграф знаходиться між "{ $title }" і "{ $below }" і може бути "{ $title }," ось де зазвичай іде епіграф.
import-document-diagnostics = { $errors ->
    [0] { $warnings ->
    [one] { $errors } річ, яку слід знати
            [few] Знати 8765411
    [many] Знати 8765411
   *[other] { $warnings } речі, які слід знати
        }
   *[other] { $errors ->
    [one] Не вдалося прочитати { $errors } файл
            [few] { $errors } файли неможливо прочитати
    [many] { $errors } файлів неможливо прочитати
   *[other] { $errors } файли неможливо прочитати
        }
}
import-document-diagnostics-none = Нема про що доповісти.

versions-title = Версії
versions-scope-synopsis = Синопсис
versions-scope-prose = Текст
versions-loading = Перегляд ваших резервних копій…
versions-empty = Попередня версія цього
versions-error = Не вдалося прочитати резервні копії. Нічого не було втрачено, але цей список може бути неповним
versions-did-not-exist = Ще не було { $date }
versions-deleted-after = Вилучено через деякий час { $date }
versions-unreadable = { $count ->
    [one] { $count } Не вдалося прочитати резервну копію
    [few] { $count } резервні копії неможливо прочитати
    [many] { $count } Не вдалося прочитати резервні копії
   *[other] { $count } резервні копії неможливо прочитати
}
versions-thinned = { $count ->
    [one] Вже знизився стан на { $count } раніше.
    [few] У старіших версіях сам проект зменшився на { $count } попередні стани.
    [many] У старіших версіях проекту зменшилось на { $count } попередніх станів.
   *[other] З віком старіші версії. { $count } раніші стани цього тексту.
}
versions-source-backup = З резервної копії
versions-source-project = З власної історії проекту
versions-list-caption = Один запис на зміну, а не на резервну копію
versions-pick-a-version = Виберіть версію, щоб побачити, що змінено
versions-earliest = Найдавніша в історії версія.
versions-no-change = У цій частині елемента нічого не змінилося
versions-formatting-only = Тут змінено лише форматування. Слова однакові
versions-show-unchanged = Показати без змін
versions-hide-unchanged = Сховати без змін
versions-next-change = Наступна зміна
versions-near = біля "{ $text }."
versions-words-added = { $count ->
    [one] Додано { $count } слово
    [few] Додано { $count } слова
    [many] Додано { $count } слів
   *[other] Додано { $count }
}
versions-words-removed = { $count ->
    [one] Вилучено { $count } слово
    [few] { $count } слова вилучено
    [many] Вилучено { $count } слів
   *[other] { $count } вилучені слова
}
versions-blocks-moved = { $count ->
    [one] { $count } абзац пересунуто
    [few] { $count } абзаци пересунуто
    [many] { $count } абзаци пересунуто
   *[other] Переслано { $count } абзаців
}
versions-pin = Прибрати резервну копію цієї версії. Автоматичне спорожнення ніколи не вилучить
versions-unpin = Зняти резервну копію з цієї версії. Автоматичне очищення може знову її вилучити
versions-pinned-only = Показувати тільки втиснуті версії
versions-pin-note = Можна закріпити лише версії резервної копії. Шпилька зберігає файл, а історія проекту - ні.
versions-pinned-empty = Тут нічого ще не закріплено
versions-pinned-empty-log = Тут нічого ще не закріплено. Можна закріпити лише версії резервної копії. Шпилька зберігає файл, а сама історія проекту - ні.
versions-range-filter = Показувати лише версії між двома датами
versions-filtered-empty = Немає версії, що відповідає вказаним вами фільтрам
versions-clear-filters = Очистити фільтри
versions-last-30-days = Попередні 30 днів
versions-restore-button = Відновити цю версію
versions-restore-confirm-title = Замінити цей текст на версію з { $date }?
versions-restore-confirm-text = Те, що ви маєте тепер буде замінено на текст, який цей елемент мав на { $date }.
versions-restore-confirm-with-comments = Поточний текст буде замінено текстом цього елемента станом на { $date }. Коментарі, прив’язані до поточного тексту, можуть втратити прив’язку. Кількість: { $count }.

versions-restore-confirm-undo-note = Спочатку створено резервну копію, і Ctrl+Z скасовує це на одному кроці.
versions-restored-toast = Відновлено версію з { $date }
versions-undo = Скасувати дію
versions-restore-row-gone = Цей елемент більше не в цьому проекті
versions-restore-no-home = З того часу цей елемент змінився, а старий текст не має в ньому місця.
versions-restore-no-safety-copy = Ваша охорона не працювала, так що нічого не змінилося
versions-restore-failed = Спроба відновлення зазнала невдачі: { $error }
versions-restore-backup-busy = Створення резервної копії вже виконується. За мить повторіть спробу
versions-restore-in-backup-file = Ви дивитесь на резервний файл; відкрийте проект, щоб відновити його
versions-restore-no-project = Не відкрито жодного проекту
versions-recreate-button = Поверніть це назад…
versions-recreate-picker-title = Куди він подівся?
versions-recreate-picker-empty = У цьому проекті немає прив' язки
versions-recreate-picker-confirm = Принесіть його сюди
versions-recreate-picker-cancel = Скасувати
versions-recreate-untitled = цей елемент
versions-recreate-confirm-title = Принесіть "{ $item }"?
versions-recreate-confirm-text = Його буде додано до { $destination }, з текстом, який він мав на { $date }.
versions-recreate-confirm-undo-note = Скасовує наступне повідомлення і повертає повідомлення назад.
versions-recreated-toast = Елемент «{ $item }» знову у вашому проєкті

versions-recreated-partial-toast = Елемент «{ $item }» відновлено, але не вдалося прочитати деякі його тексти. Кількість: { $count }.

versions-recreate-already-here = Цей пункт вже є у вашому проекті
versions-recreate-no-destination = Виберіть місце в палітурці, щоб покласти його
versions-recreate-unreadable = Цю резервну копію неможливо прочитати, тому нічого не було додано
versions-recreate-failed = Не вдалося повернути його: { $error }
versions-changed-percent = { $percent }% з цього змінено
versions-hidden-paragraphs = … Незмінених абзаців: { $count } …

timeline-title = Повернутися в минуле
timeline-coverage = { $count ->
    [one] { $count } версія записана, повертає до 8765411
    [few] { $count } версії записаного, повертаємося до { $oldest }
    [many] { $count } версій записаного, повертаємось до { $oldest }
   *[other] Записані { $count } версії, повернення до { $oldest }
}
timeline-loading = Перегляд минулого вашого проекту…
timeline-empty = Версія цього проекту ще не записана
timeline-no-changes = Відтоді нічого не змінилося.
timeline-no-text-changes = Відтоді жоден текст не змінився
timeline-slider-label = Зареєстрована версія
timeline-series-name = Розмір проекту
timeline-bars-caption = Кожна з панелей є записаною версією, яка була дуже високою у той час. Ви бачите позначену.
timeline-bars-caption-periods = Забагато версій, щоб їх було показано по одному, отже кожен стовпчик є { $period }, такий же високий, як і під кінець проекту.
timeline-unit-hour = година
timeline-unit-day = день
timeline-unit-week = тиждень
timeline-unit-month = місяць
timeline-range-filter = Показувати лише версії між двома датами
timeline-range-empty = У цих датах не записано жодної версії
timeline-open-period = Відкрити цей період
timeline-show-all = Показати весь журнал
timeline-changed-since = { $count ->
    [one] { $count } елемент відрізняється між 8765411 і вашим проектом
    [few] { $count } пункти тепер відрізняються між { $date } і вашим проектом
    [many] { $count } елементів, які зараз відрізняються між { $date } і вашим проектом
   *[other] Об' єкти { $count } відрізняються від { $date } до вашого проекту
}
timeline-kind-added = Писаний з того часу
timeline-kind-removed = Немає більше в проекті
timeline-kind-changed = Виправлений з часу
timeline-kind-moved = Згорнуто
timeline-not-yet-written = Цього ще не було.
timeline-no-text-of-its-own = Це не має власного тексту, це шлях до того, що всередині
timeline-reader-close = Закрити
timeline-reader-stamp = Як було. { $date }
timeline-reader-loading = Відкриття записаної версії…
timeline-reader-unreadable = Цей запис неможливо прочитати. Резервну копію, можливо, було пересунуто, вилучено або встановлено на пристрої, який не з' єднано.
timeline-reader-compared = Як було. { $date }, встановити проти того, що він каже
timeline-reader-diff-legend = З того часу минуло, і до нього додано підкреслене.
timeline-reader-view-label = Що показувати
timeline-reader-view-diff = Зміни
timeline-reader-view-text = Текст
timeline-reader-deleted = Цього більше немає у вашому проекті.
timeline-prose-only-record = Цей пункт взято з власної історії проекту, яка зберігає текст і все інше, отже те, що було вилучено або пересунуто, оскільки його неможливо показати. Для цього оберіть резервну копію.

settings-sec-extensions = Розширення

settings-group-games-forward = Завжди пересилати
settings-games-forward-toggle = Грати " Завжди вперед"
settings-games-forward-hint =
    Під час гри ви нічого не можете отримати: Backspace, Delete,
    Вирізати, перетягувати і скидати, і Вернути на поверхні, яку ви оберете нижче.
    Ви все ж можете набирати, вставляти, форматувати і пересуватися вільно. чернетка лише зростає.
settings-games-session-warning =
    Вибір для окремого сеансу: його ніколи не збережено. Закриття проекту або завершення роботи
    Skribisto, завжди закінчується гра, і все, що ви написали, коли гралися, стає
    Його знову можна скасувати, як тільки він це зробить.
settings-group-games-scope = Де він застосовується
settings-games-in-prose = Прозу рукопису
settings-games-in-synopsis = Синапси
settings-games-scope-hint =
    Коментарі, примітки і титули ніколи не замерзають: вони є місцем, де ви бачите фіксацію.
    Ты просто запретил себе делать это.
settings-games-inert-warning =
    "Завжди вперед" увімкнено, але це не стосується нічого.
    або нічого не змінюється, коли ти пишеш.

games-title = Запис ігор
games-forward-name = Завжди пересилати
games-forward-blurb =
    Чернетка, не беручи нічого назад. Вилучення, вирізання і скасування вимкнено
    поки ви граєте. Напишіть наступне речення замість виправлення останнього.
games-forward-playing = Відтворення: вилучення вимкнено
games-forward-idle = Не відтворюється
games-session-note = Завершується, коли ви закриєте проект.
games-scope-prose-and-synopsis = Залежить від прози і синапсів.
games-scope-prose = Задовольняється своєю прозою.
games-scope-synopsis = Задовольняє ваші синапси.
games-scope-nothing = Застосовує до нічого. Оберіть поверхню у параметрах.
games-settings-link = Запис параметрів гри…
statusbar-games-forward = Завжди пересилати
statusbar-games-forward-tooltip =
    Увімкнено " Завжди вперед ": вилучення, вирізання і скасування буде вимкнено
    чернетку. Клацніть, щоб припинити гру.
goal-progress-words = { $g ->
    [one] { $count } з 8765412 слів
    [few] { $count } слів
    [many] { $count } слів
   *[other] { $count } з { $goal } слова
}
goal-progress-characters = { $g ->
    [one] { $count } з 8765412 символів
    [few] { $count } символів
    [many] { $count } символів
   *[other] { $count } з { $goal } символи
}
goal-count-words = { $n ->
    [one] { $count } слово
    [few] { $count } слова
    [many] { $count } слів
   *[other] { $count } слова
}

goal-count-characters = { $n ->
    [one] { $count } символ
    [few] { $count } символи
    [many] { $count } символів
   *[other] { $count } символа
}

goal-unit-words = Слова
goal-unit-characters = Символи

inspector-goal = Мета
inspector-goal-none = Немає мети

overview-col-goal = Мета
overview-excluded-from-export = Не експортовано, отже, загалом

goal-subtree-total-words = { $count ->
    [one] У { $items } - { $words } слів
    [few] У межах { $items } - { $words } слів
    [many] У межах { $items } - { $words } слів
   *[other] У межах { $items } - { $words } слів
}
goal-subtree-total-characters = { $count ->
    [one] У { $items } символів
    [few] { $items } символів
    [many] { $items } символів
   *[other] { $items } цілі у сумі { $words } символи
}

new-work-goal-unit = Врахувати
new-work-goal-unit-hint = Використовується для кожного слова або символу у цьому проекті. Пізніше можна змінити.

settings-group-goal-unit = Цілі
settings-goal-unit-switch-title = Перемикнути одиницю обчислення
settings-goal-unit-switch-text = Цілі та етапи, які вже було встановлено для цього проекту, було введено у { $from }. Перемикання на { $to } не змінює їх, отже кожне існуюче число буде прочитано як { $to }.
settings-goal-unit-switch-informative = Нічого не втрачено. Перемкніться у будь- який момент, щоб відновити оригінальне читання, а потім оновлюйте цілі, які ви хочете зберегти.
settings-goal-unit-switch-confirm = Перемкнути

goal-distribute-action = Розподілити…
distribute-title = Розподілити призначення
distribute-empty = Всередині немає нічого спільного з ціллю.
distribute-weight = Спільний доступ
distribute-weight-length = За довжиною
distribute-weight-rows = За шматками
distribute-weight-even = Порівну
distribute-overwrite = Замінити цілі, які вже встановлено
distribute-col-item = Елемент
distribute-col-current = Зараз
distribute-col-proposed = Після
distribute-total = Вони становлять { $total }, проти цілі - { $goal }.
distribute-over-budget = Цілі, які вже встановлені всередині, мають { $over } більше, ніж власні контейнери. Підніміть ціль, зменшіть її або замініть її.
distribute-apply = Розподілити
distribute-cancel = Скасувати

milestone-target-gone = Ціль вилучено
milestone-no-target = Немає мети
milestone-add = Додати
milestone-add-label = Що має бути досягнуто

pace-summary-title = Де знаходиться книга
pace-summary-remaining = На поступання { $words }
pace-summary-open = Відкрити план
pace-summary-close = Закрити
pace-summary-dont-show = Не показувати це під час відкриття
pace-summary-menu = Запис плану…

menu-help-topics = &Довідка Тема
menu-help-shortcuts = К&лавіатурні скорочення …
menu-help-website = &Skribisto у мережі
menu-help-report = П&овідомити про проблему…
menu-help-check-updates = &Перевірити оновлення
menu-command-palette = &Команда Policy…

learn-help-topics = Питання довідки
learn-shortcuts = Сполучення клавіш
learn-website = Skribisto у Тенетах

help-window-title = Довідка
help-filter-topics = Фільтрувати теми
help-no-matching-topic = Теми не збігаються.
help-topic-missing = Ця тема більше не доступна.
help-back = Назад
help-not-translated = Цю сторінку ще не перекладено, тому вона відображається англійською мовою.

help-section-getting-started = Перші кроки
help-section-writing = Запис
help-section-reviewing = Повторення
help-section-exchanging = Отримання слів
help-section-keeping = Підтримуй свою роботу.
help-section-extensions = Розширення

help-topic-getting-started = Ваш перший проект
help-topic-writing-model = Як складається книга
help-topic-drafts-and-old-versions = Тримаючи стару чернетку
help-topic-goals-and-pace = Цілі і темп
help-topic-comments = Коментарі
help-topic-round-trip = Надсилання вашої книги до читача
help-topic-export = Експортування
help-topic-import-documents = Імпортування документів
help-topic-import-projects = Перенесення цілого проекту через
help-topic-backups-and-versions = Резервні копії і версії

help-shortcuts-title = Сполучення клавіш
help-shortcuts-filter = Скорочення фільтрування
help-shortcuts-no-matches = Не знайдено клавіатурних скорочень.
help-shortcuts-rebind = Змінити скорочення…
help-shortcuts-close = Закрити

command-palette-placeholder = Введіть команду

shortcut-name-binder-duplicate = Дублювати
shortcut-name-comments-add = Додати коментар
shortcut-name-comments-add-paragraph = Коментар про абзац
shortcut-name-spellcheck-toggle = Перевірити правопис
shortcut-name-editor-tab-close = Закрити вкладку
shortcut-name-editor-tab-pin = Закріпити або відкріпити вкладку
shortcut-name-editor-save = Зберегти
shortcut-name-work-export = Export…
shortcut-name-work-new = Новий проєкт
shortcut-name-work-open = Відкрити проект
shortcut-name-window-new = Нове вікно
shortcut-name-work-close = Закрити проєкт
shortcut-name-app-settings = Налаштування
shortcut-name-app-quit = Вийти
shortcut-name-editor-insert-footnote = Вставити адресу
shortcut-name-format-scene-break = Вставка Розрив сцен
shortcut-name-format-major-scene-break = Вставити розрив головної сцени
shortcut-name-format-link = Вставити посилання
shortcut-name-format-mark-nbsp = Вставити пропуск
shortcut-name-format-mark-nbhyphen = Insert No break Hyphen
shortcut-name-format-mark-nnbsp = Вставити вузький беззбройний простір
shortcut-name-format-mark-zwsp = Вставити пробіл з нульовою частотою
shortcut-name-go-next = Далі
shortcut-name-go-prev = Попередній
shortcut-name-go-to = Перейти до
shortcut-name-outline-toggle = Показати або приховати структуру
shortcut-name-preview-toggle = Показати або приховати попередній перегляд
shortcut-name-view-fullscreen = Перемкнути повноекранний режим
shortcut-name-view-focus-mode = Перемкнути режим вільного трасування
shortcut-name-editor-size-increase = Збільшити розмір тексту
shortcut-name-editor-size-decrease = Зменшити розмір тексту
shortcut-name-editor-size-reset = Відновити розмір тексту
shortcut-name-editor-find = Знайти
shortcut-name-editor-replace = Замінити
shortcut-name-editor-find-next = Наступний збіг
shortcut-name-editor-find-prev = Попередній збіг
shortcut-name-search-show = Пошук у проєкті
shortcut-name-search-replace = Замінити в проекті
shortcut-name-outline-open-to-side = Відкрити убік
shortcut-name-help-topics = Довідка Тема
shortcut-name-help-shortcuts = Клавіатурні скорочення
shortcut-name-help-website = Skribisto у мережі
shortcut-name-help-report = Повідомити про проблему
shortcut-name-help-check-updates = Перевірити оновлення
update-check-behind = Версія { $version }.
update-check-current = Ви користуєтеся найновішою версією.
update-check-failed = Не вдалося перевірити наявність новішої версії. { $error }
shortcut-name-command-palette = Палітра команд
help-section-reference = What things

help-concept-scene-break = Переривання сцени
help-concept-major-scene-break = Розрив головної сцени
help-concept-find-in-prose = Шукати в тексті
help-concept-story-bible = Енциклопедія історії
help-concept-goal-unit = Слова або символи
help-concept-goal-progress = Поступ
help-concept-manuscript-words = Те, що вважається рукописом
help-concept-exportable = Залишилося без експортування
help-concept-distribute = Спільне використання цілі
help-concept-subtree-total = До чого додають цілі всередині
help-concept-milestone = Віха
help-concept-pace-plan = План роботи

help-concept-tag = Мітка
help-concept-label = Підпис
help-concept-point-of-view = Точка зору
help-concept-epigraph = Епіграф
help-concept-footnote = Примітка
help-concept-chapter-mode = Форма глави
help-concept-comment = Коментар
help-concept-backup = Резерв
help-concept-version = Версія
help-concept-trash = Кошик
help-concept-spellcheck = Перевірка правопису
help-concept-search-replace = Пошук і заміна
help-concept-note-template = Шаблон нотатки
help-concept-story-bible-entry = Запис для історії
help-concept-text-replacement = Заміна
help-concept-smart-punctuation = Кмітливий розділювач
help-concept-export-style = Експортувати стиль
help-concept-round-trip-marks = Позначки раунду

epigraph-mark-attribution = Рядок джерела
epigraph-mark-attribution-tip = Позначає рядок, у якому знаходиться позначка лапки, отже, вона виводить як внесок
margin-lane-name = Позначки на полях
margin-lane-provider-comments = Коментарі
margin-lane-provider-story-bible = Запис для історії
margin-lane-provider-story-bible-hint = На записці In proe, де цей запис названо, і сцени, які показуються через його очі, також позначають ці імена в самій прозі.
margin-lane-mark-point-of-view = Розказано звідси
margin-lane-mark-named-here = Названо тут
margin-lane-provider-comments-hint = Де до тексту додано нотатку
margin-lane-provider-search = Пошук результатів
margin-lane-provider-search-hint = Будь-який удар за тим, що ви востаннє шукали, де він знаходиться у документі
margin-lane-provider-boundaries = Місце, де починається кожен документ
margin-lane-provider-boundaries-hint = Правило у верхній частині кожної сцени у потоці, так що ви можете бачити, де ви знаходитесь
margin-lane-provider-spelling = Правопис
margin-lane-provider-spelling-hint = Якщо ви не запитаєте про це, шашка напише вам слово: вона позначає думку вашого проза
margin-lane-spelling = { $word }, можливе неправильне написання
margin-lane-search-hit = { $text }, відповідає { $index } з { $total }
margin-lane-search-current = { $text }, відповідає { $index } з { $total }, у якому ви працюєте
margin-lane-boundary = Початок { $title }
margin-lane-boundary-untitled = документ без назви
settings-page-margin-lane = Позначки на полях
settings-desc-margin-lane = Смужка поряд з смужкою, і те, що вона показує
settings-margin-lane-enabled = Показувати лінію полів
settings-margin-lane-enabled-hint = Той самий перемикач, що і Перегляд ▸ Следы относительно.
settings-margin-lane-enabled-more = Вона ніколи не говорить про щось погане: вона показує, де речі, і залишає їх вам.
settings-group-margin-lane-marks = Що він позначає
settings-margin-lane-no-providers = Ничто еще не замечает проход.
settings-group-margin-lane-texture = Фігура діалогового вікна
settings-margin-lane-texture = Показувати текстуру діалогу
settings-margin-lane-texture-hint = Одна риска на абзац: скільки часу вона триває, і скільки часу вона вимовляється.
settings-margin-lane-texture-more = Не вимірюється на мови без приборканого конгресу, де видно довжину бару.
settings-group-margin-lane-surfaces = Де він з'являється
settings-margin-lane-surface-editor = Текстовий редактор
settings-margin-lane-surface-stream = Потік
settings-margin-lane-surface-search-preview = Перегляд пошуку
menu-margin-lane = По&значки на полях

status-preset-drafting = Чернетка
status-preset-passes = Модифікація
status-preset-plume = Plume Creator
status-todo = Виконати
status-draft = Чернетка
status-revised = Виправлений
status-final = Остаточний
status-outline = Структура
status-first-edit = 1- й редагування
status-second-edit = 2- й редагування
status-done = Готово
status-plume-draft-1 = 1- ий чернетка
status-plume-draft-2 = 2- й чернетка
status-plume-draft-3 = Три чернетки
status-plume-edit-1 = 1- й редагування
status-plume-edit-2 = 2- й редагування
status-plume-edit-3 = Третя редактура
status-plume-proofread = Коректура
status-plume-finished = Завершено
new-work-statuses = Процес
new-work-statuses-hint = Кроки, які ви пересуватимете. Ви можете перейменувати, перевпорядкувати або додавати до них будь- коли.
status-none = Без стану
inspector-status = Стан
overview-col-status = Стан
overview-status-mixed = частини, наведені нижче, не збігаються
help-concept-status = Стан
status-completion-title = Де знаходиться книга
status-completion-headline = { $done } з { $total } сцени завершено
status-completion-empty = Тут ще немає сцен, тому, коли ви пишете, вони заповнюються.
status-completion-open = Де знаходиться книга…

settings-page-statuses = Стани
settings-desc-statuses = Перейменування, перевпорядкування, додавання та вилучення етапів, через які програма проходить
settings-statuses-add = Додати стан
settings-statuses-add-placeholder = Нова назва стану
settings-statuses-desc =
    Стадія, через яку проходить сцена, за порядком, від принаймні завершеного у верхній частині до більшості
    Виконаний внизу. Це те, що "менше завершено" означає всюди
    так что положите их в программу, как и на самом деле.
settings-statuses-apply-preset = Застосувати набір…
settings-statuses-preset-applied = { $added ->
    [one] Додано { $added } стан
    [few] Додано { $added } стани
    [many] Додано { $added } станів
   *[other] Додано { $added } станів
}
settings-statuses-preset-refused = У цьому проекті вже є драбина. Набори параметрів заповнюють порожній проект. Вилучіть початкові пропуски.
settings-statuses-duplicate = "{ $name }" вже на цій драбині.
settings-statuses-added = Додано "{ $name }"
settings-statuses-move-up = Пересунути вгору (менше завершено)
settings-statuses-move-down = Пересунути вниз (більше завершено)
settings-statuses-details-placeholder = Що означає ця стадія (не обов' язково)
settings-statuses-delete = Вилучити " { $name } "
settings-statuses-delete-in-use = { $count ->
    [one] Вилучити " { $name } ": на ньому перебуває { $count } елемент і він втратить свій стан
    [few] Вилучити " 86543211 ": там буде { $count } пункти і втратить свій стан
    [many] Вилучити " { $name } ": { $count } елементів і втратити їх стан
   *[other] Вилучити " { $name } ": { $count } елементів і втратить свій стан
}
settings-statuses-deleted = Вилучено{ $name }"
settings-statuses-deleted-in-use = { $count ->
    [one] Вилучено " 86543211 ." { $count } елемент більше не має стану.
    [few] Вилучено " 86543211 ." { $count } елементи вже не мають стану.
    [many] Вилучено " 86543211 ." { $count } елементів більше не мають стану.
   *[other] Вилучено{ $name }". { $count } речі більше не мають стану.
}
settings-statuses-empty-title = Робота ще не виконується
settings-statuses-empty-body = За допомогою стану можна визначити, наскільки далеко знаходиться сцена. Додайте один вище або почніть з набору.
status-category-planned = Заплановано
status-category-drafting = Чернетка
status-category-needs-work = Потребує роботи
status-category-revised = Виправлений
status-category-final = Остаточний
