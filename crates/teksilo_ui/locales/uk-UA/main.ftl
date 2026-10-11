# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

menu-work = &Проєкт
menu-new-work = Н&овий проєкт
menu-open-work = Ві&дкрити проєкт…
menu-new-window = &Нове вікно
menu-import-from = І&мпортувати з
menu-create-from = &Створити з
menu-import-plume = &Plume Creator (.plume)…
menu-import-document = &Документи (Markdown, Word, ODT)…
menu-import-manuskript = &Manuskript (.msk)…
menu-import-scrivener = &Scrivener (.scriv)…
menu-export = &Експорт
menu-export-book = Експортувати книгу
menu-export-part = Експортувати частину
menu-export-chapter = Експортувати розділ
menu-export-scene = Експортувати сцену
menu-export-note = Експортувати нотатку
menu-export-paratext = Експортувати паратекст
menu-export-folder = Експортувати теку
menu-export-choose = Вибрати…
menu-export-none = Відкрийте документ, щоб експортувати його
menu-save = &Зберегти
menu-save-as-file = Зберег&ти одним файлом…
menu-save-as-folder = Збере&гти як теку…
menu-backup = &Створити резервну копію
menu-close-work = За&крити проєкт
menu-welcome = В&ітання…
menu-settings = Н&алаштування
menu-quit = &Вийти
menu-view = &Вигляд
menu-outline = &План
menu-search = П&ошук у проєкті
menu-timeline = Пов&ернутися в минуле
menu-search-preview = Попере&дній перегляд пошуку
menu-fullscreen = По&вноекранний режим
menu-focus-mode = &Режим без відволікань
menu-format = &Формат
menu-scene-break = &Вставити розрив сцени
menu-major-scene-break = Вставити &головний розрив сцени
menu-go = П&ерехід
menu-go-next-scene = Н&аступна сцена
menu-go-prev-scene = &Попередня сцена
menu-go-next-chapter = Наступний &розділ
menu-go-prev-chapter = Поп&ередній розділ
menu-go-next-note = &Наступна нотатка
menu-go-prev-note = П&опередня нотатка
menu-tools = &Інструменти
menu-help = &Довідка
ctx-add = &Додати
ctx-new-item = &Новий елемент
ctx-new-folder = Нова &тека
ctx-rename = П&ерейменувати
ctx-duplicate = Д&ублювати
ctx-import-here = &Імпортувати сюди…
ctx-indent = З&більшити відступ
ctx-outdent = &Зменшити відступ
ctx-trash = Пе&ремістити в кошик
ctx-open-to-side = &Відкрити збоку
ctx-open = &Відкрити
ctx-reveal-in-outline = Показати в &плані
ctx-move-up = Пересунути &вгору
ctx-move-down = Пересунути в&низ
ctx-tab-close = &Закрити
ctx-tab-close-others = Закрити ін&ші
ctx-tab-close-all = Закрити &всі
ctx-tab-open-to-side = Відкрити з&боку
ctx-tab-open-in-main = Відкрити на &головній панелі
ctx-tab-move-to-side = &Перемістити вбік
ctx-tab-move-to-main = &Перемістити на головну панель
ctx-tab-move-to-new-window = Перемістити в &нове вікно
ctx-tab-move-window-unsaved = Спершу збережіть цей проєкт: друге вікно відкриває файл на диску
ctx-tab-pin = З&акріпити цю вкладку
ctx-tab-unpin = В&ідкріпити цю вкладку
tab-pinned-tooltip = Закріплено: «Закрити інші» і «Закрити всі» її не закривають
create-book = Книга
create-part = Частина
create-chapter = Розділ
create-scene = Сцена
create-note = Нотатка
create-note-folder = Тека нотаток
create-folder = Тека
create-paratext = Паратекст
create-paratext-folder = Тека паратексту
create-book-end = Кінець книги
create-story-bible-entry = Запис довідника історії…
type-book-start = Початок книги
type-text = Текст
new-item-book = Нова книга
new-item-part = Нова частина
new-item-chapter = Новий розділ
new-item-note = Нова нотатка
new-item-note-folder = Нова тека нотаток
new-item-folder = Нова тека
new-item-paratext = Новий паратекст
new-item-paratext-folder = Нова тека паратексту
new-item-story-bible-entry = Новий запис довідника історії
placement-inside = всередину
placement-after = після
placement-after-parent = після батьківського
placement-top-level = верхній рівень
ctx-promote = &Перетворити на
promote-chapter-folder = Розділ-тека
promote-flat-chapter = Простий розділ
promote-lossy-title = Тексту ніде зберігатися
promote-lossy-text = У типі «{ $target }» немає місця для цього: { $kinds }. Спершу перемістіть або очистьте цей текст, а потім перетворіть.
content-scene-text = Текст сцени
content-note-text = Текст нотатки
content-book-title = Назва книги
content-book-subtitle = Підзаголовок книги
content-part-title = Назва частини
content-chapter-title = Назва розділу
content-epigraph-text = Епіграф
content-paratext-text = Паратекст
promote-blocked-title = Розділ не порожній
promote-blocked-text = { $count ->
    [one] Цей розділ ще містить { $count } елемент. Перемістіть його або надішліть у кошик, перш ніж перетворювати розділ на простий.
    [few] Цей розділ ще містить { $count } елементи. Перемістіть їх або надішліть у кошик, перш ніж перетворювати розділ на простий.
    [many] Цей розділ ще містить { $count } елементів. Перемістіть їх або надішліть у кошик, перш ніж перетворювати розділ на простий.
   *[other] Цей розділ ще містить { $count } елемента. Перемістіть їх або надішліть у кошик, перш ніж перетворювати розділ на простий.
}
inspector = Інспектор
inspector-empty = Відкрийте елемент, щоб переглянути його властивості.
inspector-promote = Перетворити на…
statusbar-toggle-outline = Показати або сховати підшивку
statusbar-toggle-inspector = Показати або сховати інспектор
statusbar-save-unsaved = Є незбережені зміни. Клацніть, щоб зберегти
statusbar-save-saved = Усі зміни збережено
statusbar-save-autosave = Автозбереження увімкнено. Зміни зберігаються під час письма
statusbar-saving = Збереження…
statusbar-word-count = { $count ->
    [one] { $count } слово
    [few] { $count } слова
    [many] { $count } слів
   *[other] { $count } слова
}
statusbar-word-count-tooltip = Слова у сцені, яку ви редагуєте
statusbar-word-char-count = { $words ->
    [one] { $words } слово
    [few] { $words } слова
    [many] { $words } слів
   *[other] { $words } слова
} · { $chars ->
    [one] { $chars } символ
    [few] { $chars } символи
    [many] { $chars } символів
   *[other] { $chars } символу
}
session-toggle = Сеанс письма: почати або призупинити зосереджений спринт
session-configure = Задати для сеансу мету в словах і обмеження часу
session-configure-title = Сеанс письма
session-word-goal = Мета в словах
session-time-limit = Обмеження часу
session-time-limit-unit = хв
session-no-goal = Без мети
session-no-limit = Без обмеження
session-reset = Скинути сеанс
session-readout = { $words ->
    [one] { $words } слово
    [few] { $words } слова
    [many] { $words } слів
   *[other] { $words } слова
} · { $time }
session-readout-timed = { $words ->
    [one] { $words } слово
    [few] { $words } слова
    [many] { $words } слів
   *[other] { $words } слова
} · залишилося { $time }
statusbar-focus-exit = Вийти з режиму без відволікань
statusbar-focus-synopsis = Синопсис
statusbar-focus-go-prev = Попередній елемент (Alt+Up)
statusbar-focus-go-next = Наступний елемент (Alt+Down)
rail-settings = Налаштування
language = Мова
theme = Тема
english = Англійська
french = Французька
light = Світла
dark = Темна
settings-text-width = Ширина тексту
settings-preview-width = Ширина попереднього перегляду пошуку
settings-autosave = Автоматично зберігати на диск
settings-show-welcome = Показувати вікно запуску під час старту
settings-show-welcome-tip = Якщо вимкнено, замість нього знову відкривається останній проєкт.
settings-check-for-updates = Перевіряти наявність нових версій
settings-check-for-updates-tip = Не частіше ніж раз на день Skribisto запитує вебсайт проєкту, яка версія поточна. Запит не містить жодного ідентифікатора і навіть номера вашої версії: порівняння відбувається на цьому комп’ютері.
settings-title = Налаштування
settings-close = Закрити
settings-search = Шукати в налаштуваннях
settings-reset = Відновити типові
settings-done = Готово
settings-cancel = Скасувати
settings-apply = Застосувати
settings-ok = Гаразд
settings-reset-confirm-title = Відновити всі типові налаштування?
settings-reset-confirm-body = Відновлює налаштування вигляду, редактора й письма цієї програми. Власні налаштування вашого проєкту, клавіатурні скорочення та збережені стилі не змінюються. Цю дію не можна скасувати.
settings-empty-title = Тут поки що немає налаштувань
settings-empty-hint = У майбутньому оновленні в цьому розділі з’являться параметри.
settings-sec-appearance-behaviour = Вигляд і поведінка
settings-sec-editor = Редактор
settings-sec-spelling = Правопис
settings-sec-backup = Резервні копії та синхронізація
settings-sec-compile = Компонування та експорт
settings-page-appearance = Зовнішній вигляд
settings-field-image-size-policy = Великі зображення
settings-image-policy-ask = Щоразу питати
settings-image-policy-keep = Зберігати оригінал
settings-image-policy-downscale = Оптимізувати
settings-hint-image-size-policy = Що робити, коли вставлене зображення більше, ніж потрібно рукопису.
settings-page-notifications = Сповіщення
settings-page-scene = Сцена
settings-page-synopsis = Синопсис
settings-page-notes = Нотатки
settings-page-editor-behavior = Поведінка редактора
settings-page-goals = Цілі та кількість слів
settings-page-games = Письменницькі ігри
settings-page-corkboard = Коркова дошка
settings-page-distraction-free = Без відволікань
settings-page-dictionaries = Словники
settings-page-autosave = Автозбереження
settings-page-export = Формати експорту
settings-page-paratext = Структури паратексту
settings-page-keymap = Розкладка скорочень
settings-keymap-filter = Фільтрувати скорочення
settings-desc-sec-appearance-behaviour = Як виглядає сама програма і що вона робить під час запуску.
settings-desc-sec-editor = Поверхня письма: як вона виглядає і що робить, поки ви друкуєте.
settings-desc-sec-spelling = Перевірка правопису і словники, на яких вона ґрунтується.
settings-desc-sec-backup = Як ваша робота потрапляє на диск і які копії зберігаються.
settings-desc-sec-compile = Що виходить зі Skribisto і в якому вигляді.
settings-desc-sec-work = Налаштування, які належать цьому проєкту й зберігаються в його файлі.
settings-desc-sec-extensions = Сторінки, додані встановленими тут розширеннями.
settings-desc-group-typography = Окрема сторінка для кожного виду тексту: шрифт, розмір, висота рядка та інтервали.
settings-desc-appearance = Мова інтерфейсу, тема, розмір тексту та вікно запуску під час старту.
settings-desc-notifications = Усі повідомлення, показані за цей сеанс, і дії, які можна повторити.
settings-desc-scene = Як оформлено прозу сцен: шрифт, розмір, висота рядка, відступи та інтервали.
settings-desc-synopsis = Як оформлено панель синопсису, незалежно від рукопису.
settings-desc-notes = Як оформлено нотатки, незалежно від рукопису.
settings-desc-corkboard = Розмір карток, що на них показано і як дошка їх розташовує.
settings-desc-distraction-free = Типографіка, ширина колонки та панель керування для повноекранного письма.
settings-desc-distraction-free-themes = Бібліотека тем для повноекранного письма: вбудовані та ваші власні.
settings-desc-editor-behavior = Ширина тексту, розташування синопсису, прокручування «друкарська машинка» та підсвічування навколо курсора.
settings-desc-punctuation = Типографічний стиль, з якого починаються нові проєкти. Кожен проєкт дотримується його, якщо не задає власного.
settings-desc-goals = Цілі за словами й символами та спосіб підрахунку слів.
settings-desc-games = Обмеження, які ви ставите собі під час роботи над чернеткою, як-от «Завжди вперед».
settings-desc-spellcheck = Єдиний перемикач, що вмикає й вимикає перевірку правопису всюди.
settings-desc-dictionaries = Встановлення, вилучення та перегляд словників правопису на цьому комп’ютері.
settings-desc-autosave = Чи записуються ваші зміни на диск автоматично.
settings-desc-backup = Коли створюються копії, де вони зберігаються і скільки їх. Кожен проєкт дотримується цього, якщо не задає власного.
settings-desc-export = Стилі, через які компонується кожен експорт: вбудовані та ваші власні.
settings-desc-paratext = Початкові та кінцеві сторінки книги, з якими може починатися новий проєкт.
settings-desc-keymap = Усі скорочення і дії, до яких їх прив’язано.
settings-page-user = Користувач
settings-desc-user = Хто ви, для підпису коментарів, які ви пишете.
settings-group-identity = Особисті дані
settings-field-user-name = Ваше ім’я
settings-field-user-name-placeholder = Необов’язково
settings-field-user-name-hint = Підписує коментарі та відповіді, які ви пишете. Це не ім’я автора проєкту: воно стоїть на обкладинці книги та зберігається у файлі.
settings-field-user-initials = Ваші ініціали
settings-field-user-initials-placeholder = З вашого імені
settings-field-user-initials-hint = Те, що текстовий процесор показує поруч із вашим коментарем на полях. Залиште порожнім, щоб використати показані, взяті з вашого імені.
settings-field-user-hint = Обидва поля необов’язкові й діють для всіх проєктів на цьому комп’ютері. Зміна стосується коментарів, які ви напишете далі. Уже написані коментарі зберігають ім’я, з яким їх написано.
settings-desc-author = Ім’я, яке стоїть на цьому проєкті.
settings-desc-structure = Чи є розділи цього проєкту теками, чи окремими елементами.
settings-desc-language = Мова, за якою перевіряється проза цього проєкту.
settings-desc-work-backup = Власне правило резервного копіювання цього проєкту або загальне.
settings-desc-personal-dictionary = Слова, які цей проєкт вважає написаними правильно.
settings-desc-tags = Кольорові мітки, якими цей проєкт позначає свої елементи.
settings-desc-templates = Шаблони нотаток, які можна вставляти під час письма в цьому проєкті.
settings-desc-text-replacements = Скорочення, що розгортаються, коли ви друкуєте в цьому проєкті.
settings-desc-work-punctuation = Лапки, тире та пробіли, яких дотримується проза цього проєкту.
settings-group-typography = Типографіка
settings-group-writing-column = Колонка письма
settings-group-writing-view = Вигляд письма
settings-group-distraction-free-strip = Панель керування
settings-group-theme = Тема
settings-group-language = Мова
settings-group-startup = Під час старту
settings-group-updates = Оновлення
settings-group-autosave = Автозбереження
settings-field-typeface = Шрифт
settings-field-size = Розмір тексту
editor-size-changed-manuscript = Розмір тексту рукопису: { $percent }
editor-size-changed-synopsis = Розмір тексту синопсису: { $percent }
editor-size-changed-notes = Розмір тексту нотаток: { $percent }
editor-size-changed-corkboard = Розмір тексту карток коркової дошки: { $percent }
editor-size-changed-corkboard-expanded = Розмір тексту розгорнутого редактора: { $percent }
editor-size-changed-distraction-free = Розмір тексту в режимі без відволікань: { $percent }
menu-text-size-increase = З&більшити розмір тексту
menu-text-size-decrease = З&меншити розмір тексту
menu-text-size-reset = Ск&инути розмір тексту
settings-field-line-height = Висота рядка
settings-field-first-line-indent = Абзацний відступ
settings-field-paragraph-spacing-before = Інтервал перед абзацом
settings-field-paragraph-spacing-after = Інтервал після абзацу
settings-field-column-width = Ширина колонки
settings-distraction-free-width-hint = Діє лише в режимі без відволікань. Ширина колонок сцени, синопсису та нотаток не змінюється.
settings-distraction-free-title = Залишати назву елемента
settings-distraction-free-word-count = Залишати кількість слів
settings-distraction-free-session = Залишати сеанс письма
settings-distraction-free-go-to = Залишати кнопку «Перейти до…»
settings-distraction-free-go = Залишати кнопки «Попередній» і «Наступний»
settings-distraction-free-chrome-hint = Кнопка «Вийти» залишається завжди, хоч би що ви тут вибрали. Це ваш вихід, якщо клавіша Escape зайнята.
settings-field-app-theme = Тема
settings-theme-light = Світла
settings-theme-dark = Темна
settings-field-text-scale = Розмір тексту інтерфейсу
settings-field-language = Мова інтерфейсу
settings-synopsis-placement = Розташування синопсису
settings-synopsis-placement-none = Не показувати
settings-synopsis-placement-top = Згори
settings-synopsis-placement-side = Збоку
settings-typewriter = Прокручування «друкарська машинка»
settings-typewriter-tip = Утримує рядок, який ви пишете, на сталій висоті, а рукопис прокручується під ним. Клацання, як і раніше, ставить курсор туди, де ви клацнули.
settings-typewriter-position = Положення рядка
settings-typewriter-position-top-third = Верхня третина
settings-typewriter-position-middle = Посередині
settings-typewriter-position-bottom-quarter = Нижня чверть
settings-highlight-scope = Підсвічування навколо курсора
settings-highlight-scope-none = Немає
settings-highlight-scope-sentence = Речення
settings-highlight-scope-paragraph = Абзац
settings-highlight-scope-tip-none = Сторінка залишається чистою. Під час письма нічого не затіняється.
settings-highlight-scope-tip-sentence = Затіняти речення, яке ви пишете, щоб воно виділялося серед сусідніх.
settings-highlight-scope-tip-paragraph = Затіняти весь абзац, який ви пишете, щоб уривок, над яким ви працюєте, був перед очима.
settings-group-container-views = Подання контейнерів
settings-remember-view = Запам’ятовувати останнє подання для кожного типу елементів
settings-remember-view-tip = Відкривати контейнер у поданні, яке ви востаннє використовували для цього типу
settings-remember-view-tip-more =
    Книга, частина й розділ мають кілька подань (власна сторінка, повний
    рукопис, повний синопсис). Якщо це ввімкнути, кожен тип відкриватиметься
    в поданні, яке ви востаннє для нього вибрали: наприклад, перемкніть один розділ
    на «Увесь розділ», і наступний відкритий розділ теж покаже «Увесь розділ». Кожен тип пам’ятає своє подання.
settings-group-counting = Підрахунок слів
settings-counting-auto = Автоматично (за мовою)
settings-counting-whitespace = Ділити за пробілами
settings-counting-unicode-words = Слова за Unicode
settings-counting-cjk-hybrid = З урахуванням CJK (за символами)
settings-counting-hint = В автоматичному режимі китайський і японський тексти рахуються за символами, а всі інші мови — за словами. Змінюйте це, лише якщо підрахунок для вашої мови здається хибним.
settings-group-goals-display = Відображення
settings-show-characters = Показувати кількість символів у рядку стану
settings-autosave-hint = Зміни автоматично записуються на диск під час письма.
settings-sec-work = Проєкт
settings-page-structure = Структура
settings-page-author = Автор
settings-field-author-name = Ім’я автора
settings-field-author-placeholder = Необов’язково
settings-field-author-hint = З’являється на титульній сторінці та в метаданих експортованих файлів. Залиште порожнім, щоб не вказувати. Це ім’я автора книги. Ім’я, яким підписуються ваші коментарі, задається в «Налаштування ▸ Користувач».
settings-group-chapters = Розділи
tidy-titles-title = Упорядкувати назви розділів
tidy-titles-none = Немає жодного розділу чи частини, назва якої лише повторює власний номер.
tidy-titles-lead = { $count ->
        [one] { $count } розділ або частина має назвою лише власний номер.
        [few] { $count } розділи й частини мають назвою лише власний номер.
        [many] { $count } розділів і частин мають назвою лише власний номер.
       *[other] { $count } розділу й частини мають назвою лише власний номер.
    }
tidy-titles-explain = Після очищення цих назв кожен розділ позначатиметься номером, який книга вже знає, тим самим, що його друкує експорт. Більше нічого не змінюється, і одне скасування повертає все назад.
menu-document-tidy-titles = Упорядкувати &назви розділів…
settings-group-numbering = Нумерація
settings-number-chapters = Нумерувати розділи й частини
settings-number-chapters-tip = Розділи й частини отримують номер, обчислений за їхнім місцем у книзі: він показується тут поруч із назвою і друкується під час експорту.
settings-number-chapters-tip-more = Номер ніколи не зберігається в назві, тож залишається правильним, коли ви змінюєте порядок, вставляєте чи видаляєте. Якщо вимкнути це, експорт друкує лише назви, хоч би що вимагав стиль експорту. Щоб пропустити один розділ (пролог, інтерлюдію), скористайтеся перемикачем «Нумерація» в інспекторі: розділ залишиться в книзі, але не отримає номера.
settings-part-resets-chapter = Починати нумерацію розділів заново в кожній частині
settings-part-resets-chapter-tip = Типово вимкнено: розділи нумеруються наскрізно через усі частини книги, тож «Частина друга» починається з розділу одинадцятого.
settings-part-resets-chapter-tip-more = Так заведено у видавничій практиці, і саме цього очікує читач. Увімкніть для книги, частини якої мають читатися як окремі томи, де кожна починається з розділу першого.
settings-chapter-flat = Прості розділи
settings-chapter-flat-hint = Увімкнено: розділ є окремим елементом. Ви пишете в ньому, і він не містить сцен. Вимкнено: розділ є текою. Ви так само пишете в ньому, але він може містити й сцени. Нові розділи створюються відповідно до цього налаштування; наявні перетворюються командою «Перетворити на».
settings-styles-builtin = Вбудовані стилі
settings-styles-user = Мої стилі
settings-styles-builtin-badge = Вбудований
settings-styles-duplicate = Дублювати
settings-styles-edit = Змінити
settings-styles-delete = Видалити
settings-styles-import = Імпортувати…
settings-styles-export = Експортувати…
settings-styles-copy-suffix = (копія)
settings-styles-json-filter = Стиль експорту
settings-styles-editor-title = Зміна стилю
settings-styles-editor-none = Виберіть власний стиль, щоб змінити його, або дублюйте вбудований.
settings-styles-imported = Стиль імпортовано
settings-styles-import-failed = Не вдалося імпортувати стиль
settings-styles-exported = Стиль експортовано
settings-styles-export-failed = Не вдалося експортувати стиль
settings-styles-field-name = Назва
settings-styles-field-chapters = Заголовки розділів
settings-styles-field-parts = Заголовки частин
settings-styles-field-scene-break = Розрив сцени
settings-styles-field-major-scene-break = Головний розрив сцени
settings-styles-field-spacing = Міжрядковий інтервал
settings-styles-field-justify = Вирівнювати за шириною
settings-styles-field-notes = Додавати нотатки
settings-styles-field-synopses = Додавати синопсиси
settings-styles-field-scene-titles = Додавати назви сцен
settings-styles-field-epigraphs = Додавати епіграфи
settings-styles-field-paratexts = Додавати паратексти
settings-styles-field-epigraph-placement = Розташування епіграфа
settings-styles-epigraph-after = Після назви
settings-styles-epigraph-before = Перед назвою
settings-styles-field-footnotes = Додавати виноски
settings-styles-field-footnote-numbering = Нумерація виносок
settings-styles-footnote-numbering-continuous = Наскрізна
settings-styles-footnote-numbering-per-chapter = Заново в кожному розділі
settings-styles-footnote-numbering-per-book = Заново в кожній книзі
settings-styles-field-images = Зображення
menu-image = Зо&браження
settings-styles-images-beside = Поруч із документом
settings-styles-images-embed = Усередині документа
settings-styles-images-omit = Не додавати
settings-styles-group-round-trip = Надсилання редакторові
settings-styles-field-comments = Додавати коментарі
settings-styles-field-round-trip-marks = Додавати маркери повернення
settings-styles-round-trip-hint = Лише DOCX і ODT. Маркери — це невидимі ідентифікатори, завдяки яким файл, що повертається, оновлює цей проєкт, а не потрапляє поруч як друга копія.
settings-styles-group-pages = Сторінки
settings-styles-field-cover = Починати з обкладинки
settings-styles-field-word-count = Кількість слів на титульній сторінці
settings-styles-field-page-books = Нова сторінка для кожної книги
settings-styles-field-page-parts = Нова сторінка для кожної частини
settings-styles-field-page-chapters = Нова сторінка для кожного розділу
settings-styles-field-page-paratexts = Нова сторінка для кожного паратексту
settings-styles-heading-none = Без заголовка
settings-styles-heading-numbered = Лише номер
settings-styles-heading-title = Лише назва
settings-styles-heading-both = Номер і назва
settings-styles-break-blank = Порожній рядок
settings-styles-break-none = Немає
settings-styles-spacing-single = Одинарний
settings-styles-spacing-onehalf = 1½
settings-styles-spacing-double = Подвійний
welcome-title = Вітаємо у Skribisto
welcome-version = Версія { $version }
welcome-update-available = Доступна версія { $version }
welcome-update-tip = Відкриває сторінку завантаження у вашому браузері.
welcome-search = Пошук проєктів
welcome-open = Відкрити
welcome-new-work = Новий проєкт
welcome-create-from = Створити з…
welcome-recent-works = Нещодавні проєкти
welcome-empty-recents = Нещодавніх проєктів ще немає.
welcome-no-matches = Жоден нещодавній проєкт не відповідає пошуку.
welcome-learn-soon = Посібники й поради з’являться незабаром.
welcome-about-blurb = Skribisto — програма для письма, переписана на Rust і Teksilo.
welcome-tagline = *Тихе місце, щоб писати довгі тексти.*
welcome-github = GitHub
welcome-discord = Discord
nav-works = Проєкти
nav-examples = Приклади
nav-learn = Навчання
nav-about = Про програму
synopsis = Синопсис
epigraph = Епіграф
pane-manuscript = Рукопис
corkboard = Коркова дошка
overview = Огляд
corkboard-card-count = { $count ->
    [one] { $count } картка
    [few] { $count } картки
    [many] { $count } карток
   *[other] { $count } картки
}
corkboard-child-count = { $count ->
    [one] { $count } елемент
    [few] { $count } елементи
    [many] { $count } елементів
   *[other] { $count } елемента
}
corkboard-view-nested = Вкладене
corkboard-view-flat = Пласке
corkboard-layout-hint = «Вкладене» показує безпосередні дочірні елементи контейнера. Відкрийте картку теки, щоб увійти в неї. «Пласке» показує одразу всі сцени цілого контейнера.
corkboard-card-size = Розмір карток
corkboard-search-placeholder = Фільтрувати картки…
corkboard-empty-title = Тут поки що нічого немає
corkboard-empty-hint = Скористайтеся кнопкою «＋ Створити» вгорі, щоб додати перший елемент.
corkboard-new = Створити
corkboard-show-card-numbers = Нумерувати картки
corkboard-modal-size = Розмір розгорнутого редактора
corkboard-scope-hint = Застосовується до всіх відкритих коркових дошок у всіх проєктах.
corkboard-sort-manuscript = Порядок рукопису
corkboard-sort-title-asc = Назва А–Я
corkboard-sort-title-desc = Назва Я–А
corkboard-card-number = Картка { $number }
duplicate = Дублювати
duplicate-n = { $count ->
    [one] Дублювати { $count } картку
    [few] Дублювати { $count } картки
    [many] Дублювати { $count } карток
   *[other] Дублювати { $count } картки
}
set-label-n = { $count ->
    [one] Задати позначку для { $count } картки
    [few] Задати позначку для { $count } карток
    [many] Задати позначку для { $count } карток
   *[other] Задати позначку для { $count } картки
}
move-to-trash-n = { $count ->
    [one] Перемістити { $count } картку в кошик
    [few] Перемістити { $count } картки в кошик
    [many] Перемістити { $count } карток у кошик
   *[other] Перемістити { $count } картки в кошик
}
reveal-in-outline = Показати в плані
corkboard-move-to = Перемістити до…
corkboard-move-to-n = { $count ->
    [one] Перемістити { $count } картку до…
    [few] Перемістити { $count } картки до…
    [many] Перемістити { $count } карток до…
   *[other] Перемістити { $count } картки до…
}
corkboard-move-picker-title = Перемістити до…
corkboard-move-picker-empty = Підшивок ще немає. Спершу створіть одну.
corkboard-move-picker-cancel = Скасувати
corkboard-move-here = Перемістити сюди
corkboard-moved-ok = { $count ->
    [one] Переміщено { $count } картку
    [few] Переміщено { $count } картки
    [many] Переміщено { $count } карток
   *[other] Переміщено { $count } картки
}
corkboard-move-into-self = Контейнер не можна перемістити всередину нього самого. Виберіть місце призначення поза ним.
corkboard-move-failed = Не вдалося перемістити туди ці картки.
overview-col-title = Назва
overview-col-type = Тип
overview-col-label = Позначка
overview-col-tags = Мітки
overview-col-books = Книги
overview-col-own-words = Слова
overview-col-total-words = Усього
overview-row-count = { $count ->
    [one] { $count } елемент
    [few] { $count } елементи
    [many] { $count } елементів
   *[other] { $count } елемента
}
overview-search-placeholder = Фільтрувати елементи…
overview-expand-all = Розгорнути все
overview-collapse-all = Згорнути все
overview-table-label = Вміст
overview-empty-title = Тут поки що нічого немає
overview-gone-title = Цього контейнера більше немає
overview-gone-hint = Його переміщено в кошик. Відновіть його або закрийте цю вкладку.
overview-empty-hint = Скористайтеся кнопкою «＋ Створити» вгорі, щоб додати перший елемент.
corkboard-grid-label = Картки коркової дошки
corkboard-rename-field = Перейменувати елемент
corkboard-expand-synopsis = Розгорнути синопсис
corkboard-synopsis-modal-title = Синопсис
corkboard-badge-scene = Сцена
corkboard-badge-chapter = Розділ
corkboard-badge-part = Частина
corkboard-badge-book = Книга
corkboard-badge-note = Нотатка
corkboard-badge-folder = Тека
corkboard-badge-paratext = Паратекст
corkboard-badge-text = Текст
corkboard-badge-end = Кінець
settings-group-corkboard-layout = Макет
settings-group-corkboard-cards = Картки
corkboard-show-word-count = Показувати кількість слів на картках
text-heading = Текст
no-content = Цей елемент не має вмісту, який можна редагувати.
untitled = Без назви
placeholder-title = Назва…
placeholder-subtitle = Підзаголовок…
placeholder-chapter-title = Назва розділу…
split-editor = Розділити редактор
close-split-view = Закрити розділене подання
drop-open-here = Відкрити тут
drop-open-to-side = Відкрити збоку
segment-chapter = Розділ
segment-part = Частина
segment-book = Книга
segment-pace = Темп
segment-notes = Нотатки
segment-story-bible = Довідник історії
segment-note-own = Нотатка
segment-note-details = Подробиці
segment-note-in-prose = У прозі
pace-placeholder = Тут з’явиться планувальник темпу.
pace-empty-title = Сплануйте темп цієї книги
pace-empty-body = Задайте мету в словах і кінцевий термін, і Skribisto обчислить щоденний темп, що приведе вас до мети.
pace-start-planning = Почати планування
pace-section-schedule = Розклад
pace-section-progress = Поступ
pace-advancement = Просування
pace-goal = Мета в словах
pace-deadline = Кінцевий термін
pace-active = Темп увімкнено
pace-day-mon = Пн
pace-day-tue = Вт
pace-day-wed = Ср
pace-day-thu = Чт
pace-day-fri = Пт
pace-day-sat = Сб
pace-day-sun = Нд
pace-card-written = написано слів
pace-card-of-goal = від мети
pace-card-rate = слів за день письма
pace-card-days-left = днів письма залишилося
pace-card-streak = днів поспіль
pace-card-ahead = слів із випередженням темпу
pace-card-behind = слів із відставанням від темпу
pace-charts-empty = Графіки поступу з’являться тут, щойно кількість слів буде записано під час збереження.
pace-chart-progression = Написано слів порівняно з ціллю
pace-chart-words-per-day = Слів за день
pace-series-actual = Фактично
pace-series-target = Ціль
pace-series-words-per-day = Слів/день
pace-daily-target-line = Рівномірний темп: { $count ->
    [one] { $count } слово/день
    [few] { $count } слова/день
    [many] { $count } слів/день
   *[other] { $count } слова/день
}
pace-section-holidays = Вихідні
pace-section-milestones = Віхи
pace-holidays-none = Вихідних немає. Кожен запланований день враховується.
pace-holiday-label = Назва вихідного
pace-add-holiday = Додати
pace-remove = Вилучити
pace-milestones-none = Віх ще немає. Задайте віху для частини чи розділу в інспекторі.
full-chapter = Увесь розділ
full-part = Уся частина
full-book = Уся книга
full-synopsis = Увесь синопсис
rename = Перейменувати
set-label = Задати позначку
insert-scene = Вставити сцену
insert-chapter = Вставити розділ
split-scene = Розділити сцену
move-up = Пересунути вгору
move-down = Пересунути вниз
merge-with-previous = Об’єднати з попередньою
move-to-trash = Перемістити в кошик
rename-chapter = Перейменувати розділ
add-scene = Додати нову сцену
add-chapter = Додати новий розділ
new-scene-title = Нова сцена
placeholder-scene-name = Назва сцени…
menu-cut = Вирізати
menu-copy = Копіювати
menu-paste = Вставити
menu-paste-unformatted = Вставити неформатований текст
menu-select-all = Виділити все
format-bold = Жирний
format-italic = Курсив
format-underline = Підкреслений
format-strikethrough = Перекреслений
binder = Підшивка
no-work = Жоден проєкт не завантажено
no-recent-works = Нещодавніх проєктів немає
switcher-open-section = Зараз відкрито
switcher-recent-section = Нещодавні
switcher-this-window = це вікно
open-project-title = Відкриття проєкту
open-project-question = Як ви хочете відкрити «{ $title }»?
open-in-new-window = Відкрити в новому вікні
open-here = Відкрити тут
binder-all = Усі підшивки
binder-show-all = Показати всі підшивки
binder-new = Нова підшивка…
binder-default-name = Нова підшивка
binder-item-count = { $count ->
    [one] { $count } елемент
    [few] { $count } елементи
    [many] { $count } елементів
   *[other] { $count } елемента
}
binder-search-placeholder = Фільтрувати план…
binder-search-scope = Шукати в усіх підшивках
binder-trash-confirm-title = Перемістити підшивку в кошик?
binder-trash-confirm-text = «{ $name }» і всі її елементи буде переміщено в кошик.
dialog-rename = Перейменувати
dialog-new-binder = Нова підшивка
dialog-set-label = Задати позначку
dialog-new-scene = Нова сцена
close-work-question = Зберегти зміни перед закриттям проєкту?
quit-question = Зберегти зміни перед виходом?
quit-save-work-question = Зберегти зміни в «{ $title }» перед виходом?
unsaved-changes = У цьому проєкті є незбережені зміни.
new-work-unsaved-question = Зберегти зміни перед створенням нового проєкту?
open-work-unsaved-question = Зберегти зміни перед відкриттям іншого проєкту?
switch-backup-discard-title = Відкинути зміни в цій резервній копії?
switch-backup-discard-text = Зміни в резервній копії не можна зберегти в неї саму. Щоб зберегти їх, скористайтеся командою «Зберегти як» або «Відновити», або відкиньте їх і відкрийте інший проєкт.
switch-save-failed = Не вдалося зберегти проєкт, тому його не замінено: { $error }
switch-save-not-started = Не вдалося зберегти проєкт, тому його не замінено.
close-save-failed = Не вдалося зберегти проєкт, тому його не закрито: { $error }
close-save-not-started = Не вдалося зберегти проєкт, тому його не закрито.
save-not-started = Не вдалося зберегти проєкт.
could-not-open-work = Не вдалося відкрити «{ $file }»
could-not-open-work-too-new = Для «{ $file }» потрібна новіша версія Skribisto
could-not-open-work-too-new-detail = Збережено у форматі Skribisto { $written_by }; щоб відкрити, потрібен формат { $requires } або новіший, а ця збірка підтримує формати до { $supported }. Оновіть Skribisto, щоб відкрити його.
could-not-open-work-importing = У «{ $file }» зараз записує імпорт
could-not-open-work-importing-detail = Імпорт перезаписує цей проєкт і ще не завершився. Відкрийте його, коли імпорт закінчиться: якщо відкрити зараз, ви побачите проєкт, який імпорт замінює, а наступне збереження записало б цей проєкт поверх імпортованого.
could-not-open-example = Не вдалося відкрити приклад: { $error }
could-not-open-link = Не вдалося відкрити { $url }: { $error }
could-not-create-work = Не вдалося створити проєкт: { $error }
saving-as-file = Збереження як { $target }…
saving-as-folder = Збереження як { $target }/…
saved-as = Збережено в { $target }
save-error = Не вдалося зберегти: { $error }
save-flush-failed = Збереження виконано, але ці документи не вдалося до нього передати, і вони досі не збережені: { $titles } ({ $error })
backup-error = Не вдалося створити резервну копію: { $error }
backing-up = Створення резервної копії…
backup-nothing-open = Немає відкритого проєкту для резервного копіювання.
backup-already-running = Резервне копіювання вже триває.
backup-complete = Резервне копіювання завершено (збережено: { $ok }, уже актуальних: { $skipped })
backup-partial = Резервне копіювання завершено: збережено { $ok }, { $failed ->
    [one] { $failed } місце призначення недоступне
    [few] { $failed } місця призначення недоступні
    [many] { $failed } місць призначення недоступні
   *[other] { $failed } місця призначення недоступні
}
backup-no-destination-title = Немає доступного місця для резервних копій
backup-no-destination-text = Жодне з налаштованих місць для резервних копій недоступне (наприклад, зовнішній диск може бути від’єднано). Під’єднайте його і повторіть спробу або вийдіть без резервного копіювання.
backup-failed-close-title = Не вдалося зберегти резервну копію
backup-failed-close-text = Перед закриттям не вдалося записати жодної резервної копії. Усі місця призначення виявилися недоступними (диск могли від’єднати, або він заповнений чи захищений від запису). Виправте це й повторіть спробу або вийдіть без резервного копіювання.
backup-failed-close-error-text = Перед закриттям не вдалося записати жодної резервної копії: { $error }. Виправте це й повторіть спробу або вийдіть без резервного копіювання.
backup-no-result = Резервне копіювання завершилося, не повідомивши, чи створено копію. Перевірте список резервних копій, чи з’явилася там нова.
backup-choice-title = Файл резервної копії
backup-choice-heading = Ви відкрили резервну копію
backup-choice-subtitle = Це резервна копія проєкту на певний момент часу.
backup-choice-subtitle-dated = Резервну копію створено { $date }.
backup-choice-body = Ви можете відкрити її і вільно редагувати, але зберегти зміни можна лише командою «Зберегти як». Оригінальний файл проєкту не змінюється. Або ж відновіть цей проєкт точно до стану цієї резервної копії.
backup-choice-open = Відкрити резервну копію
backup-choice-restore = Відновити проєкт до цього стану…
backup-choice-not-a-backup = Ні, відкрити звичайним чином
backup-banner-title = Резервна копія: зміни тут не можна зберегти
backup-banner-description = Скористайтеся командою «Зберегти як», щоб зберегти зміни в новому файлі, або «Відновити», щоб замінити оригінальний проєкт цією резервною копією.
backup-banner-restore = Відновити…
backup-banner-save-as = Зберегти як…
backup-restore-original-missing = Не вдалося знайти оригінальний проєкт, який слід відновити. Натомість скористайтеся командою «Зберегти як», щоб зберегти цю резервну копію як новий проєкт.
backup-restore-close-elsewhere-title = Проєкт відкрито в іншому вікні
backup-restore-close-elsewhere-text = Проєкт, який ви відновлюєте, відкрито в іншому вікні. Спершу закрийте його там, а потім повторіть спробу.
backup-restore-focus-window = Перейти до того вікна
backup-restore-confirm-title = Відновити цю резервну копію?
backup-restore-confirm-text = Перед заміною поточну версію проєкту буде скопійовано окремо як страхувальну резервну копію.
backup-restore-confirm-ok = Відновити
backup-restore-error = Не вдалося відновити: { $error }
backup-restored-ok = Проєкт відновлено.
backup-restored-with-safety = Проєкт відновлено. Вашу попередню версію збережено в { $path }.
close-backup-discard-title = Відкинути зміни в цій резервній копії?
close-backup-discard-text = Зміни в резервній копії не можна зберегти в неї саму. Скористайтеся командою «Зберегти як», щоб зберегти їх, або відкиньте їх і закрийте.
quit-backup-discard-title = Відкинути зміни й вийти?
quit-backup-discard-work-question = Відкинути зміни в «{ $title }» і вийти?
quit-backup-discard-text = Зміни в резервній копії не можна зберегти в неї саму. Скористайтеся командою «Зберегти як», щоб зберегти їх, або відкиньте їх і вийдіть.
backup-nudge-text = Для цього проєкту не налаштовано резервних копій.
backup-nudge-action = Налаштувати резервні копії…
menu-backups-list = &Резервні копії…
backups-title = Резервні копії
backups-loading = Пошук резервних копій…
backups-empty = Для цього проєкту ще не знайдено резервних копій.
backups-open = Відкрити
backups-reveal = Показати
backups-delete = Видалити цю резервну копію
backups-delete-confirm-title = Видалити цю резервну копію?
backups-delete-confirm-text = «{ $name }» буде остаточно видалено. Цю дію не можна скасувати.
backups-delete-error = Не вдалося видалити резервну копію: { $error }
backups-refresh = Оновити
backups-close = Закрити
settings-page-backup = Типові параметри резервних копій
settings-page-work-backup = Резервні копії
settings-backup-general-title = Типові налаштування резервних копій
settings-backup-work-title = Резервні копії цього проєкту
settings-backup-inherit = Використовувати загальні налаштування резервних копій
settings-backup-inheriting = Цей проєкт використовує загальні налаштування резервних копій.
settings-backup-none-hint = Автоматичні резервні копії не налаштовано (усі умови вимкнено).
settings-backup-last = Остання резервна копія: { $date }
settings-backup-last-never = Резервних копій ще немає.
settings-backup-open-list = Відкрити список резервних копій…
settings-backup-triggers = Коли створювати резервні копії
settings-backup-on-close = Створювати резервну копію під час закриття проєкту
settings-backup-on-open = Створювати резервну копію під час відкриття проєкту
settings-backup-interval = Створювати резервну копію періодично, кожні
settings-backup-destinations = Місця для резервних копій
settings-backup-default-location = Типове розташування
settings-backup-usage-since = { $count ->
    [one] { $count } резервна копія · { $size } · найстаріша { $oldest }
    [few] { $count } резервні копії · { $size } · найстаріша { $oldest }
    [many] { $count } резервних копій · { $size } · найстаріша { $oldest }
   *[other] { $count } резервної копії · { $size } · найстаріша { $oldest }
}
settings-backup-usage = { $count ->
    [one] { $count } резервна копія · { $size }
    [few] { $count } резервні копії · { $size }
    [many] { $count } резервних копій · { $size }
   *[other] { $count } резервної копії · { $size }
}
settings-backup-usage-empty = Тут поки що нічого не зберігається
settings-backup-usage-measuring = Вимірювання…
settings-backup-reveal-root = Показати теку
settings-backup-dest-none = Місць призначення немає. Резервні копії зберігаються поруч із проєктом.
settings-backup-dest-remove = Вилучити
settings-backup-dest-add = Додати теку…
settings-backup-dest-refresh = Оновити
settings-backup-retention = Скільки зберігати
settings-backup-retention-tiered = За рівнями
settings-backup-retention-keep-n = Зберігати останні N
settings-backup-retention-tip = Як вилучаються старі резервні копії.
settings-backup-retention-tip-more = Режим «За рівнями» зберігає одну копію на годину протягом доби, одну на день протягом тижня, одну на тиждень протягом місяця й одну на місяць далі, тож недавня історія лишається докладною, а давня поступово рідшає. «Зберігати останні N» просто зберігає N найновіших копій. В обох режимах найновіші копії (мінімум, указаний нижче) зберігаються завжди.
settings-backup-gfs-hourly = Щогодини (останні 24 год)
settings-backup-gfs-daily = Щодня (останній тиждень)
settings-backup-gfs-weekly = Щотижня (останній місяць)
settings-backup-gfs-monthly = Щомісяця
settings-backup-keep-n = Скільки зберігати
settings-backup-min-keep = Завжди зберігати щонайменше
settings-backup-dedup = Пропускати резервне копіювання, якщо нічого не змінилося
new-work-title = Новий проєкт
new-work-close = Закрити
new-work-name = Назва проєкту
new-work-name-placeholder = Без назви
new-work-author = Автор
new-work-author-placeholder = Необов’язково
new-work-format = Формат
new-work-single-file = Один файл
new-work-bundle = Пакет
new-work-convert-later = Формат можна буде змінити пізніше.
new-work-location = Розташування
new-work-will-create = Буде створено
new-work-language = Типова мова
new-work-language-hint = Застосовується до нових текстів і перевірки правопису. Для кожного тексту можна вибрати іншу мову.
new-work-template = Шаблон
new-work-template-none = Немає
new-work-template-empty-novel = Порожній роман
new-work-template-light-novel = Ранобе
new-work-template-novel = Роман
new-work-template-novel-in-parts = Роман у частинах
new-work-template-notebook = Записник
new-work-cancel = Скасувати
new-work-create = Створити проєкт
new-work-step-details = Подробиці
new-work-step-language = Мова і структура
new-work-step-template = Шаблон
new-work-back = Назад
new-work-next = Далі
new-work-documents-title = Новий проєкт із документів
new-work-step-import = Імпорт
new-work-create-and-import = Створити й імпортувати…
new-work-documents-next-title = Далі — ваші документи
new-work-documents-next-body = Після створення проєкту над ним відкриється майстер імпорту, де ви виберете документи, перевірите структуру, яку Skribisto в них розпізнав, і вкажете, куди її помістити. Нічого не записується в проєкт, доки ви не підтвердите це там.
new-work-documents-no-template = Цей проєкт навмисно починається порожнім, без шаблону, тож імпортовані документи будуть єдиним його вмістом.
new-work-documents-chapter-scene-hint = Стосується розділів, які створить імпорт. Пізніше це можна змінити в налаштуваннях проєкту.
new-work-single-file-desc = Один архів .skrib (zip). Зручно переносити і створювати резервні копії.
new-work-bundle-desc = Тека з усіма текстами й ресурсами. Зручніше для систем керування версіями.
new-work-template-none-count = порожня підшивка
new-work-template-empty-novel-count = 1 розділ
new-work-template-light-novel-count = 15 розділів
new-work-template-novel-count = 20 розділів
new-work-template-novel-in-parts-count = 3 частини, 24 розділи
new-work-template-notebook-count = довільні нотатки
new-work-chapter-scene = Прості розділи
new-work-chapter-scene-tip = Кожен розділ є окремим елементом, у якому ви пишете безпосередньо, без сцен усередині. Залиште вимкненим для класичної будови, де розділ є текою: у ньому теж можна писати безпосередньо, але він може містити й сцени.
new-work-chapter-scene-tip-more = В обох випадках ви пишете в розділі. Різниця лише в тому, чи може він *містити* сцени. Дерево підшивки в Skribisto служить лише для впорядкування, тож обидві будови компонуються в ту саму книгу, їх можна вільно поєднувати, а команда «Перетворити на» змінює будову розділу, не втрачаючи жодного слова.
new-work-name-required = Введіть назву проєкту
new-work-name-invalid = У цій назві немає придатних символів
new-work-location-required = Виберіть розташування
new-work-location-missing = Цієї теки не існує
new-work-location-not-folder = Цей шлях не є текою
new-work-location-readonly = До цієї теки не можна записувати
new-work-target-exists = У цій теці вже є проєкт із такою назвою. Перед заміною його буде запитано підтвердження
new-work-target-is-folder = У цьому розташуванні вже є тека з такою назвою. Виберіть іншу назву
new-work-overwrite-title = Замінити наявний проєкт?
new-work-overwrite-text = «{ $name }» уже існує в цій теці. Замінити його новим проєктом?
new-work-target-open-title = Цей проєкт відкрито
new-work-target-open-text = «{ $name }» відкрито в Skribisto. Якщо створити на його місці новий проєкт, він замінить той, над яким ви працюєте, а наступне збереження запише той проєкт поверх нового. Виберіть іншу назву чи теку або спершу закрийте проєкт.
target-importing-title = У цей проєкт зараз записує імпорт
target-importing-text = Імпорт ще записує «{ $name }». Зачекайте, доки він завершиться, або виберіть іншу назву.
backup-restore-importing-text = Імпорт ще записує «{ $name }», проєкт, поверх якого буде відновлено цю резервну копію. Відновіть резервну копію, коли імпорт завершиться.
new-work-manuscript = Рукопис
new-work-notes = Нотатки
new-work-research = Дослідження
new-work-notebook = Записник
new-work-scene = Сцена
new-work-note = Нотатка
new-work-front-matter = Початкові сторінки
new-work-back-matter = Кінцеві сторінки
new-work-paratext = Структура книги
new-work-paratext-none = Без структури
new-work-paratext-hint = Початкові й кінцеві сторінки, якими певна традиція відкриває й завершує книгу. Будь-яку з них можна потім перемістити, перейменувати чи видалити.
new-work-tags = Мітки
new-work-tags-none = Без міток
new-work-tags-hint = Початковий набір міток для персонажів, місць та іншого. Необов’язково; кожну мітку можна потім перейменувати, перефарбувати чи видалити.
new-work-note-templates = Шаблони нотаток
new-work-note-templates-none = Без шаблонів
new-work-note-templates-hint = Форма, з якої починається нотатка довідника історії. Необов’язково; кожен шаблон можна потім змінити чи видалити.
new-work-characters = Персонажі
new-work-places = Місця
import-plume-title = Імпорт проєкту Plume Creator
import-plume-source = Проєкт Plume
import-plume-source-hint = Виберіть файл .plume або .plume_backup (будь-якої версії Plume Creator).
import-plume-trash-warning = ⚠ Елементи з кошика та видалені елементи не переносяться.
import-plume-source-required = Виберіть файл проєкту Plume
import-plume-source-missing = Цього файлу не існує
import-plume-source-not-file = Цей шлях не є файлом
import-plume-manuscript-binder = Рукопис
import-plume-story-bible-binder = Довідник історії
import-plume-notes-of = { $name } (нотатки)
import-plume-content-of = { $name } (текст)
import-plume-progress-title = Імпорт проєкту Plume…
import-plume-done = Імпортовано { $imported ->
    [one] { $imported } елемент.
    [few] { $imported } елементи.
    [many] { $imported } елементів.
   *[other] { $imported } елемента.
} { $skipped ->
    [one] { $skipped } елемент із кошика не перенесено.
    [few] { $skipped } елементи з кошика не перенесено.
    [many] { $skipped } елементів із кошика не перенесено.
   *[other] { $skipped } елемента з кошика не перенесено.
}
import-plume-nested-too-deep = Проєкт не імпортовано: його частину «{ $part }» вкладено глибше ніж на { $limit } рівнів. Жоден проєкт Plume Creator не наближається до цього, а читання такого файлу могло б призвести до аварійного завершення Skribisto. Нічого не записано.
import-plume-declares-entities = Проєкт не імпортовано: його частина «{ $part }» оголошує сутності XML, чого не робить жоден проєкт Plume Creator. Їх розгортання могло б зайняти більше пам’яті, ніж є в комп’ютері, і призвести до аварійного завершення Skribisto. Нічого не записано.
import-plume-no-outline = Цей файл є стисненим архівом, але не проєктом Plume Creator: у ньому немає плану. Нічого не записано.
import-plume-empty-outline = Файл плану цього проєкту Plume Creator порожній, тож імпортувати нічого. Нічого не записано.
# $tag is the name of the document the part turned out to be.
import-plume-wrong-document = Це не проєкт Plume Creator: його частина «{ $part }» є документом «{ $tag }». Нічого не записано.
# $version is the part's own version, $supported the newest this importer reads.
import-plume-too-new = Частину «{ $part }» цього проєкту записано новішою версією Plume Creator ({ $version }), ніж може прочитати Skribisto (до { $supported } включно). Нічого не записано.
import-manuskript-title = Імпорт проєкту Manuskript
import-manuskript-source = Проєкт Manuskript
import-manuskript-source-hint = Виберіть файл .msk або теку проєкту (будь-якої версії Manuskript).
import-manuskript-source-file = Вибрати файл…
import-manuskript-source-folder = Вибрати теку…
import-manuskript-source-required = Виберіть проєкт Manuskript
import-manuskript-source-missing = Цього файлу чи теки не існує
import-manuskript-manuscript-binder = Рукопис
import-manuskript-story-bible-binder = Довідник історії
import-manuskript-characters-group = Персонажі
import-manuskript-world-group = Світ
import-manuskript-plots-group = Сюжети
import-manuskript-project-info-note = Відомості про проєкт
import-manuskript-summary-note = Стислий виклад
import-manuskript-importance-minor = Незначна
import-manuskript-importance-secondary = Другорядна
import-manuskript-importance-main = Головна
import-manuskript-notes-of = { $title } (нотатки)
import-manuskript-progress-title = Імпорт проєкту Manuskript…
import-manuskript-done = { $imported ->
    [one] Імпортовано { $imported } елемент.
    [few] Імпортовано { $imported } елементи.
    [many] Імпортовано { $imported } елементів.
   *[other] Імпортовано { $imported } елемента.
} { $revisions ->
    [0] { "" }
    [one] Разом із ними перенесено { $revisions } попередню версію.
    [few] Разом із ними перенесено { $revisions } попередні версії.
    [many] Разом із ними перенесено { $revisions } попередніх версій.
   *[other] Разом із ними перенесено { $revisions } попередньої версії.
}
import-manuskript-nested-too-deep = Проєкт не імпортовано: його файл «{ $part }» вкладено глибше ніж на { $limit } рівнів. Жоден проєкт Manuskript не наближається до цього, а читання такого файлу могло б призвести до аварійного завершення Skribisto. Нічого не записано.
import-manuskript-folders-too-deep = Проєкт не імпортовано: один із його файлів лежить глибше ніж у { $limit } вкладених теках. Жоден проєкт Manuskript не наближається до цього, а читання такого файлу могло б призвести до аварійного завершення Skribisto. Нічого не записано.
import-manuskript-declares-entities = Проєкт не імпортовано: його файл «{ $part }» оголошує сутності XML, чого не робить жоден проєкт Manuskript. Їх розгортання могло б зайняти більше пам’яті, ніж є в комп’ютері, і призвести до аварійного завершення Skribisto. Нічого не записано.
import-manuskript-not-a-project-file = «{ $file }» не є ні архівом проєкту Manuskript, ні маленьким файлом .msk, що вказує на теку проєкту. Нічого не записано.
import-manuskript-folder-missing = «{ $stub }» вказує на теку проєкту «{ $folder }» поруч із собою, але цієї теки там немає. Проєкт Manuskript у режимі теки складається з обох, і переносити їх треба разом. Нічого не записано.
import-manuskript-not-a-project = Це не схоже на проєкт Manuskript: у ньому немає ні маркера Manuskript, ні плану. Нічого не записано.
import-manuskript-format-too-new = Цей проєкт збережено у форматі Manuskript { $version }, новішому, ніж може прочитати Skribisto (до { $supported } включно). Нічого не записано.
import-manuskript-too-many-files = Проєкт не імпортовано: файлів у ньому більше, ніж { $limit }, тобто набагато більше, ніж у будь-якому проєкті Manuskript. Нічого не записано.
import-manuskript-too-large = Проєкт не імпортовано: він більший за { $limit } МБ, які Skribisto читає з проєкту Manuskript. Нічого не записано.
project-import-close = Закрити
project-import-location = Тека призначення
# The folder dialog's title.
project-import-location-title = Виберіть теку призначення
project-import-name = Назва файлу
project-import-name-placeholder = Назва проєкту
project-import-will-create = Буде створено
project-import-cancel = Скасувати
project-import-import = Імпортувати
# Field validation
project-import-location-required = Виберіть теку призначення
project-import-location-missing = Цієї теки не існує
project-import-location-not-folder = Цей шлях не є текою
project-import-location-readonly = До цієї теки не можна записувати
project-import-name-required = Введіть назву файлу
project-import-name-exists = Файл із такою назвою тут уже існує. Перед перезаписом імпорт запитає підтвердження
project-import-name-is-folder = Тут уже є тека з такою назвою, можливо, проєкт, збережений як тека. Імпорт не може її замінити: виберіть іншу назву
# Overwrite confirmation
project-import-overwrite-title = Замінити наявний файл?
project-import-overwrite-text = «{ $name }» уже існує. Замінити його імпортованим проєктом?
# Projects are imported one at a time, whichever importer asks.
project-import-busy-title = Уже імпортується інший проєкт
project-import-busy-text = Проєкти імпортуються по одному. Зачекайте, доки поточний імпорт завершиться, або скасуйте його, а потім почніть цей знову.
# Spoken by a screen reader once per phase, never shown: the toast's title, then
# the phase's name.
project-import-progress-announcement = { $title } { $phase }
# The notice once Cancel is pressed, until the converter has stopped: it may be
# finishing a file it cannot leave half written.
project-import-stopping = Зупинення імпорту…
project-import-cancelled = Імпорт скасовано
project-import-open-now = Відкрити зараз
project-import-details = Подробиці
# The import's report: what the writer should know about what came across.
project-import-report = { $count ->
    [one] { $count } примітка щодо цього імпорту
    [few] { $count } примітки щодо цього імпорту
    [many] { $count } приміток щодо цього імпорту
   *[other] { $count } примітки щодо цього імпорту
}
project-import-report-title = Про цей імпорт
# Error toast: a short reason in the body, the full technical chain behind Details.
project-import-error-title = Не вдалося імпортувати проєкт
# Why a project was not imported, whichever importer read it. A $file is a file
# name, a $folder a full path (it is the folder the writer has to act on), a
# $reason what the system or a parser said, a $limit a number.
project-import-failed-cancelled = Імпорт скасовано. Нічого не записано.
project-import-failed-target-exists = «{ $file }» уже існує, а імпорт не мав дозволу його замінити. Нічого не записано.
project-import-failed-target-folder = «{ $file }» є текою, можливо, проєктом, збереженим як тека, і імпорт не може її замінити. Виберіть іншу назву. Нічого не записано.
project-import-failed-not-written = Не вдалося записати «{ $file }» у «{ $folder }» ({ $reason }). Нічого не записано.
project-import-failed-disk-full = У «{ $folder }» бракує місця, щоб записати «{ $file }». Звільніть там місце, а потім імпортуйте знову. Нічого не записано.
project-import-failed-source-missing = «{ $file }» більше немає. Нічого не записано.
project-import-failed-unreadable = Skribisto не зміг прочитати «{ $file }» ({ $reason }). Нічого не записано.
project-import-failed-archive-damaged = «{ $file }» є стисненим файлом, який не вдалося прочитати, імовірно, через пошкодження ({ $reason }). Нічого не записано.
project-import-failed-archive-entries = { $entries ->
    [one] Проєкт не імпортовано: його архів перелічує { $entries } файл, тоді як проєкт може містити щонайбільше { $limit }. Так буває лише з файлом, створеним, щоб вичерпати пам’ять. Нічого не записано.
    [few] Проєкт не імпортовано: його архів перелічує { $entries } файли, тоді як проєкт може містити щонайбільше { $limit }. Так буває лише з файлом, створеним, щоб вичерпати пам’ять. Нічого не записано.
    [many] Проєкт не імпортовано: його архів перелічує { $entries } файлів, тоді як проєкт може містити щонайбільше { $limit }. Так буває лише з файлом, створеним, щоб вичерпати пам’ять. Нічого не записано.
   *[other] Проєкт не імпортовано: його архів перелічує { $entries } файлу, тоді як проєкт може містити щонайбільше { $limit }. Так буває лише з файлом, створеним, щоб вичерпати пам’ять. Нічого не записано.
}
project-import-failed-archive-member = Проєкт не імпортовано: його частина «{ $part }» розпаковується в понад { $limit } МБ, а так буває лише з файлом, створеним, щоб вичерпати пам’ять. Нічого не записано.
project-import-failed-archive-size = Проєкт не імпортовано: він розпаковується в понад { $limit } МБ, а так буває лише з файлом, створеним, щоб вичерпати пам’ять. Нічого не записано.
project-import-failed-nested = Проєкт не імпортовано: його частину «{ $part }» вкладено глибше ніж на { $limit } рівнів, і її читання могло б призвести до аварійного завершення Skribisto. Нічого не записано.
project-import-failed-entities = Проєкт не імпортовано: його частина «{ $part }» оголошує сутності XML. Їх розгортання могло б зайняти більше пам’яті, ніж є в комп’ютері, і призвести до аварійного завершення Skribisto. Нічого не записано.
project-import-failed-folders = Проєкт не імпортовано: один із його файлів лежить глибше ніж у { $limit } вкладених теках, і його читання могло б призвести до аварійного завершення Skribisto. Нічого не записано.
project-import-failed-malformed = Проєкт не імпортовано: його частину «{ $part }» пошкоджено, і її не вдається прочитати ({ $reason }). Нічого не записано.
# A Scrivener project the importer cannot bring across (a Scrivener 1 for Mac project, a
# damaged binder file). Why is the Scrivener importer's own to say; Details holds its key.
project-import-failed-scrivener-refused = Skribisto не може імпортувати цей проєкт Scrivener. Нічого не записано.
# The Scrivener project asks something its review did not answer: a project or a binder
# file appeared in it since.
project-import-failed-scrivener-question = Проєкт Scrivener ставить питання, на яке його перевірка не дала відповіді. Поверніться до перевірки, а потім імпортуйте. Нічого не записано.
# The Scrivener project raises a question the writer has to accept before it imports.
project-import-failed-scrivener-not-accepted = Проєкт Scrivener ставить питання, на яке ви ще не дали згоди. Поверніться до перевірки, а потім імпортуйте. Нічого не записано.
# The Scrivener project changed after the writer reviewed it.
project-import-failed-scrivener-changed = Проєкт Scrivener змінився після того, як ви його перевірили. Поверніться до перевірки, а потім імпортуйте. Нічого не записано.
# An error nothing above accounts for; $detail is its technical description.
project-import-failed-internal = Імпорт зупинився через помилку, якої Skribisto не очікував: { $detail }
import-target-open-title = Цей проєкт відкрито
import-target-open-text = «{ $name }» відкрито в Skribisto. Імпорт поверх нього замінив би проєкт, над яким ви працюєте, а наступне збереження записало б той проєкт поверх імпортованого. Виберіть іншу назву файлу або спершу закрийте проєкт.
import-warnings-announcement = { $result } { $notice }.
# Quitting while an import runs. $name is the file the import is writing.
quit-import-running-title = Імпорт ще триває
quit-import-running-text = Skribisto ще імпортує «{ $name }». Якщо вийти зараз, імпорт зупиниться: нічого не буде записано, а проєкт, з якого ви імпортуєте, лишиться таким, яким був.
quit-import-stop = Зупинити імпорт і вийти

## Import a Scrivener project: the wizard
import-scrivener-title = Імпорт проєкту Scrivener
import-scrivener-step-project = Проєкт
import-scrivener-step-review = Перевірка
import-scrivener-step-save = Збереження
import-scrivener-back = Назад
import-scrivener-next = Далі
import-scrivener-import = Імпортувати
import-scrivener-cancel = Скасувати
import-scrivener-help = Довідка

## Project step: choosing the project
import-scrivener-drop-title = Перетягніть сюди проєкт Scrivener
import-scrivener-drop-hint = Проєкт .scriv, файл .scrivx у ньому, резервна копія .zip або файл .scrivtemplate
import-scrivener-source = Проєкт Scrivener
import-scrivener-source-placeholder = Шлях до проєкту
import-scrivener-choose-file = Вибрати файл…
import-scrivener-choose-folder = Вибрати теку…
# Windows and Linux show a .scriv project as an ordinary folder.
import-scrivener-source-hint = Виберіть теку .scriv проєкту або відкрийте її та виберіть файл .scrivx усередині. Підійде й резервна копія .zip.
# macOS shows a .scriv project as a single file when Scrivener is installed, as a folder when it is not.
import-scrivener-source-hint-mac = Якщо проєкт показано як один файл, скористайтеся кнопкою «Вибрати файл…». Якщо його показано як теку, скористайтеся кнопкою «Вибрати теку…». Підійде й резервна копія .zip.
# File-dialog titles and filter label, resolved when the dialog opens.
import-scrivener-dialog-file-title = Виберіть проєкт Scrivener
import-scrivener-dialog-folder-title = Виберіть теку проєкту Scrivener
import-scrivener-dialog-filter = Проєкт Scrivener
import-scrivener-source-missing = За цим шляхом нічого немає
# A path whose name is not Unicode: the field shows it with its unreadable letters replaced.
import-scrivener-source-unreadable-name = Цю назву неможливо прочитати. Перейменуйте проєкт у файловому менеджері, а потім виберіть його знову.
import-scrivener-empty-hint = Виберіть проєкт, щоб побачити, що в ньому знайде Skribisto.
import-scrivener-reading = Читання проєкту…
import-scrivener-stop-reading = Зупинити читання
import-scrivener-read-failed = Не вдалося прочитати проєкт
import-scrivener-read-failed-body = Під час читання щось пішло не так. Натисніть «Перечитати проєкт» або перегляньте подробиці.
import-scrivener-details = Подробиці
import-scrivener-check-again = Перечитати проєкт
# A refusal this version does not know how to word.
import-scrivener-refusal-unknown-title = Цей проєкт не можна імпортувати
import-scrivener-refusal-unknown-body = Skribisto не знає, як його перенести. Причину вказано в подробицях.

## Project step: the project card
import-scrivener-made-with-mac = Востаннє збережено в Scrivener { $version } для macOS
import-scrivener-made-with-windows = Востаннє збережено в Scrivener { $version } для Windows
import-scrivener-made-with-linux = Востаннє збережено в Scrivener { $version } для Linux
import-scrivener-made-with-ios = Востаннє збережено в Scrivener { $version } для iPhone та iPad
import-scrivener-made-with-version = Востаннє збережено в Scrivener { $version }
import-scrivener-made-with-unknown = Проєкт Scrivener, формат { $format }
import-scrivener-saved-on = Збережено { $date } на «{ $device }»
import-scrivener-saved-on-no-device = Збережено { $date }
import-scrivener-card-counts = { $documents ->
    [one] { $documents } документ
    [few] { $documents } документи
    [many] { $documents } документів
   *[other] { $documents } документа
}, з них у рукописі: { $manuscript }

## Review step
import-scrivener-review-hint = Ось якою буде ваша підшивка після перенесення. Нічого не записується, доки ви не натиснете «Імпортувати», а тип багатьох елементів можна потім змінити в підшивці командою «Перетворити на».
import-scrivener-choices-structure = Рівні
import-scrivener-top-level = Теки верхнього рівня
import-scrivener-choices-text = Текст
import-scrivener-choice-containers = Теки рівня { $depth }
import-scrivener-choice-texts = Документи рівня { $depth }
import-scrivener-choice-count = { $count ->
    [one] { $count } елемент
    [few] { $count } елементи
    [many] { $count } елементів
   *[other] { $count } елемента
}
import-scrivener-choice-count-example = { $count ->
    [1] { $count } елемент: «{ $example }»
    [one] { $count } елемент, наприклад «{ $example }»
    [few] { $count } елементи, наприклад «{ $example }»
    [many] { $count } елементів, наприклад «{ $example }»
   *[other] { $count } елемента, наприклад «{ $example }»
}
import-scrivener-rung-part = Частини
import-scrivener-rung-chapter = Розділи
import-scrivener-rung-plain = Звичайні теки
import-scrivener-texts-scenes = Сцени
import-scrivener-texts-chapters = Розділи
import-scrivener-choice-books = Книги в рукописі
import-scrivener-books-one = Одна книга
import-scrivener-books-several = Окрема книга для кожної теки
import-scrivener-choice-split = { $count ->
    [1] Розділити «{ $title }» за його заголовком
    [one] Розділити «{ $title }» за його { $count } заголовком
    [few] Розділити «{ $title }» за його { $count } заголовками
    [many] Розділити «{ $title }» за його { $count } заголовками
   *[other] Розділити «{ $title }» за його { $count } заголовка
}
import-scrivener-root-items = { $count ->
    [one] { $count } елемент
    [few] { $count } елементи
    [many] { $count } елементів
   *[other] { $count } елемента
}
import-scrivener-root-items-manuscript = { $count ->
    [one] { $count } елемент, схоже на рукопис
    [few] { $count } елементи, схоже на рукопис
    [many] { $count } елементів, схоже на рукопис
   *[other] { $count } елемента, схоже на рукопис
}
import-scrivener-dest-notes = Нотатки
import-scrivener-dest-story-bible = Довідник історії
import-scrivener-dest-manuscript = Інша книга
import-scrivener-dest-leave-out = Не переносити
import-scrivener-choice-trash = { $count ->
    [one] Перенести кошик ({ $count } елемент)
    [few] Перенести кошик ({ $count } елементи)
    [many] Перенести кошик ({ $count } елементів)
   *[other] Перенести кошик ({ $count } елемента)
}
import-scrivener-trash-detail = { $count ->
    [0] Жоден із них не містить тексту.
    [one] { $count } з них містить текст.
    [few] { $count } з них містять текст.
    [many] { $count } з них містять текст.
   *[other] { $count } з них містять текст.
}
import-scrivener-choice-blank-lines = Порожні рядки стають розривами сцен
import-scrivener-choice-ornaments = Рядки з декоративних знаків стають розривами сцен
import-scrivener-preview-label = Структура нового проєкту
import-scrivener-col-title = Назва
import-scrivener-col-becomes = Стає
import-scrivener-col-exports = В експорті
import-scrivener-in-exports-yes = Так
import-scrivener-in-exports-no = Ні
# What a row the binder has no creation command for becomes.
import-scrivener-becomes-item = Елемент
import-scrivener-becomes-trashed = У кошику
import-scrivener-before-title = Перед імпортом
import-scrivener-review-error-unreadable = Не вдалося підготувати перевірку. Виберіть проєкт ще раз.
import-scrivener-review-error-choice = Цей вибір не підходить до проєкту. Виберіть проєкт ще раз.
import-scrivener-review-error-unmappable = З таким вибором підшивку не вдалося перенести без втрат. Виберіть інакше або повідомте про проблему.

## Save step
import-scrivener-save-hint = Імпорт створює новий проєкт Skribisto. Ваш проєкт Scrivener лише читається й ніколи не змінюється.
import-scrivener-title-field = Назва
import-scrivener-author = Автор
import-scrivener-language = Мова
import-scrivener-location-inside-project = Виберіть теку поза проєктом Scrivener. Scrivener зберігає там лише власні файли.
import-scrivener-copy-research-label = Копіювати файли досліджень поруч із новим проєктом
import-scrivener-copy-research = { $count ->
    [1] Файл досліджень, який Skribisto не може відкрити (наприклад, PDF чи звук), потрапить до теки поруч із проєктом. Якщо прапорець знято, його нотатка вказуватиме, де він лежить у вашому проєкті Scrivener.
    [one] { $count } файл досліджень, який Skribisto не може відкрити (наприклад, PDF чи звук), потрапить до теки поруч із проєктом. Якщо прапорець знято, його нотатка вказуватиме, де він лежить у вашому проєкті Scrivener.
    [few] { $count } файли досліджень, які Skribisto не може відкрити (наприклад, PDF чи звуки), потраплять до теки поруч із проєктом. Якщо прапорець знято, їхні нотатки вказуватимуть, де вони лежать у вашому проєкті Scrivener.
    [many] { $count } файлів досліджень, які Skribisto не може відкрити (наприклад, PDF чи звуки), потраплять до теки поруч із проєктом. Якщо прапорець знято, їхні нотатки вказуватимуть, де вони лежать у вашому проєкті Scrivener.
   *[other] { $count } файлу досліджень, які Skribisto не може відкрити (наприклад, PDF чи звуки), потраплять до теки поруч із проєктом. Якщо прапорець знято, їхні нотатки вказуватимуть, де вони лежать у вашому проєкті Scrivener.
}
# The file name proposed when the project's name holds nothing a file name can.
import-scrivener-default-name = Проєкт Scrivener

## Progress and result
import-scrivener-progress-title = Імпорт проєкту Scrivener…
import-scrivener-done-title = Проєкт Scrivener імпортовано
# How many rows the new project holds: the documents and folders that came across, and
# the rows the import made for them (a document's notes, the project notes).
import-scrivener-done = { $items ->
    [one] Новий проєкт містить { $items } елемент.
    [few] Новий проєкт містить { $items } елементи.
    [many] Новий проєкт містить { $items } елементів.
   *[other] Новий проєкт містить { $items } елемента.
}
# The same, with how many kinds of thing could not be read or need checking.
import-scrivener-done-check = { $items ->
    [one] Новий проєкт містить { $items } елемент.
    [few] Новий проєкт містить { $items } елементи.
    [many] Новий проєкт містить { $items } елементів.
   *[other] Новий проєкт містить { $items } елемента.
} { $findings ->
    [one] Уваги потребує { $findings } пункт.
    [few] Уваги потребують { $findings } пункти.
    [many] Уваги потребують { $findings } пунктів.
   *[other] Уваги потребують { $findings } пункту.
}
import-scrivener-review-again = Повернутися до перевірки
## The import's report (the dialog See report opens)
import-scrivener-see-report = Переглянути звіт
import-scrivener-report-from = Прочитано з { $path }
import-scrivener-report-to = Записано в { $path }
import-scrivener-report-came-across = Що перенесено
import-scrivener-report-nothing-counted = Нічого не пораховано.
import-scrivener-report-items = { $count ->
    [one] { $count } елемент
    [few] { $count } елементи
    [many] { $count } елементів
   *[other] { $count } елемента
}
import-scrivener-report-words = { $count ->
    [one] { $count } слово
    [few] { $count } слова
    [many] { $count } слів
   *[other] { $count } слова
}
import-scrivener-report-synopses = { $count ->
    [one] { $count } синопсис
    [few] { $count } синопсиси
    [many] { $count } синопсисів
   *[other] { $count } синопсису
}
import-scrivener-report-document-notes = { $count ->
    [one] нотатки { $count } документа
    [few] нотатки { $count } документів
    [many] нотатки { $count } документів
   *[other] нотатки { $count } документа
}
import-scrivener-report-comments = { $count ->
    [one] { $count } коментар
    [few] { $count } коментарі
    [many] { $count } коментарів
   *[other] { $count } коментаря
}
import-scrivener-report-footnotes = { $count ->
    [one] { $count } виноска
    [few] { $count } виноски
    [many] { $count } виносок
   *[other] { $count } виноски
}
import-scrivener-report-pictures = { $count ->
    [one] { $count } зображення
    [few] { $count } зображення
    [many] { $count } зображень
   *[other] { $count } зображення
}
import-scrivener-report-templates = { $count ->
    [one] { $count } шаблон нотатки
    [few] { $count } шаблони нотаток
    [many] { $count } шаблонів нотаток
   *[other] { $count } шаблону нотатки
}
import-scrivener-report-tags = { $count ->
    [one] { $count } мітка
    [few] { $count } мітки
    [many] { $count } міток
   *[other] { $count } мітки
}
import-scrivener-report-statuses = { $count ->
    [one] { $count } стан
    [few] { $count } стани
    [many] { $count } станів
   *[other] { $count } стану
}
import-scrivener-report-trashed = { $count ->
    [one] { $count } елемент у кошику
    [few] { $count } елементи в кошику
    [many] { $count } елементів у кошику
   *[other] { $count } елемента в кошику
}
import-scrivener-report-research-files = { $count ->
    [one] { $count } файл досліджень
    [few] { $count } файли досліджень
    [many] { $count } файлів досліджень
   *[other] { $count } файлу досліджень
}
import-scrivener-report-choices = Що ви вибрали
import-scrivener-report-accepted-mobile = Ви імпортували проєкт без змін, зроблених у Scrivener для iPhone або iPad, яких у ньому ще не було.
import-scrivener-report-accepted-no-text = Ви імпортували цю копію з порожніми документами, бо їхнього тексту в ній не було.
import-scrivener-report-choice = { $subject }: { $answer }
import-scrivener-report-yes = так
import-scrivener-report-no = ні
import-scrivener-report-research-copied = Файли досліджень, які Skribisto не може відкрити, скопійовано до «{ $folder }» поруч із новим проєктом.
import-scrivener-report-research-not-copied = Файли досліджень, які Skribisto не може відкрити, не скопійовано. Їхні нотатки вказують, де вони лежать у вашому проєкті Scrivener.
import-scrivener-report-research-copied-except = { $count ->
    [1] Файли досліджень, які Skribisto не може відкрити, скопійовано до «{ $folder }» поруч із новим проєктом, крім одного, який лишається у вашому проєкті Scrivener. Його нотатка вказує, де він лежить.
    [one] Файли досліджень, які Skribisto не може відкрити, скопійовано до «{ $folder }» поруч із новим проєктом, крім { $count } файлу, які лишаються у вашому проєкті Scrivener. Їхні нотатки вказують, де вони лежать.
    [few] Файли досліджень, які Skribisto не може відкрити, скопійовано до «{ $folder }» поруч із новим проєктом, крім { $count } файлів, які лишаються у вашому проєкті Scrivener. Їхні нотатки вказують, де вони лежать.
    [many] Файли досліджень, які Skribisto не може відкрити, скопійовано до «{ $folder }» поруч із новим проєктом, крім { $count } файлів, які лишаються у вашому проєкті Scrivener. Їхні нотатки вказують, де вони лежать.
   *[other] Файли досліджень, які Skribisto не може відкрити, скопійовано до «{ $folder }» поруч із новим проєктом, крім { $count } файлу, які лишаються у вашому проєкті Scrivener. Їхні нотатки вказують, де вони лежать.
}
import-scrivener-report-research-none-copied = { $count ->
    [1] Ви попросили скопіювати файли досліджень, але один файл скопіювати не вдалося. Він лишається у вашому проєкті Scrivener, а його нотатка вказує, де він лежить.
    [one] Ви попросили скопіювати файли досліджень, але { $count } файл скопіювати не вдалося. Вони лишаються у вашому проєкті Scrivener, а їхні нотатки вказують, де вони лежать.
    [few] Ви попросили скопіювати файли досліджень, але { $count } файли скопіювати не вдалося. Вони лишаються у вашому проєкті Scrivener, а їхні нотатки вказують, де вони лежать.
    [many] Ви попросили скопіювати файли досліджень, але { $count } файлів скопіювати не вдалося. Вони лишаються у вашому проєкті Scrivener, а їхні нотатки вказують, де вони лежать.
   *[other] Ви попросили скопіювати файли досліджень, але { $count } файлу скопіювати не вдалося. Вони лишаються у вашому проєкті Scrivener, а їхні нотатки вказують, де вони лежать.
}
import-scrivener-report-chose-project = Ви вирішили імпортувати «{ $project }», один із проєктів у тому, що ви вибрали.
import-scrivener-report-chose-manifest = Ви вибрали файл підшивки «{ $file }».
import-scrivener-report-chose-enclosing = Ви вибрали щось усередині «{ $project }», тож імпортовано весь проєкт.
import-scrivener-report-chose-upgraded = Ви імпортували «{ $current }», проєкт, у якому ви продовжили писати, а не копію, яку Scrivener зберіг перед оновленням.
import-scrivener-report-chose-backup = Ви імпортували «{ $backup }», старішу копію, яку Scrivener зберіг перед оновленням.
import-scrivener-report-title-placeholder-rejected = Назва в налаштуваннях проєкту була заповнювачем із шаблону, тож Skribisto натомість запропонував ім’я проєкту.
import-scrivener-report-author-placeholder-rejected = Ім’я автора в налаштуваннях проєкту було заповнювачем із шаблону, тож Skribisto не запропонував жодного.
import-scrivener-report-title-from-compile-settings = Skribisto запропонував назву, указану в налаштуваннях компонування проєкту: це ім’я, яке проєкт мав під час створення, якщо там у Scrivener не задано іншої назви.
import-scrivener-report-title-from-template = Skribisto запропонував власну назву шаблону.
import-scrivener-report-title-from-folder-name = Skribisto запропонував ім’я проєкту як його назву.
import-scrivener-report-title-from-project-properties = Skribisto запропонував назву, указану у властивостях проєкту.
import-scrivener-report-author-from-project = Skribisto запропонував ім’я автора, указане в налаштуваннях проєкту. Там може лишитися автор шаблону, і тоді його ім’я надрукується на титульній сторінці.
import-scrivener-report-language-not-stated = Проєкт не вказував своєї мови, тож Skribisto запропонував мову власного інтерфейсу. Від неї залежить, як рахуються слова й пишуться лапки.
import-scrivener-report-language-from-script = Skribisto запропонував мову за писемністю, якою написано рукопис ({ $tag }).
# A proposal of the Save page the writer replaced with their own.
import-scrivener-report-you-changed-it = { $hint } Ви це змінили.
import-scrivener-report-content-salvaged-text = Тексту «{ $item }» у проєкті не було. Його слова взято з пошукового індексу Scrivener, як простий текст і, можливо, зі старішої версії: перевірте їх.
import-scrivener-report-content-salvaged-notes = Нотаток «{ $item }» у проєкті не було. Їхні слова взято з пошукового індексу Scrivener, як простий текст і, можливо, зі старішої версії: перевірте їх.
import-scrivener-report-content-salvaged-synopsis = Синопсису «{ $item }» у проєкті не було. Його слова взято з пошукового індексу Scrivener, як простий текст і, можливо, зі старішої версії: перевірте їх.
# A finding of the document importer found in several places: its sentence about the
# first, then how many more the list under it names.
import-scrivener-report-document-others = { $others ->
    [one] { $sentence } Те саме трапляється ще в { $others } місці, наведеному нижче.
    [few] { $sentence } Те саме трапляється ще в { $others } місцях, наведених нижче.
    [many] { $sentence } Те саме трапляється ще в { $others } місцях, наведених нижче.
   *[other] { $sentence } Те саме трапляється ще в { $others } місця, наведених нижче.
}
# An item of the report, by its path in Scrivener's binder, with which of its writings
# the line is about when it is not the text.
import-scrivener-report-item-notes = { $path } (нотатки документа)
import-scrivener-report-item-synopsis = { $path } (синопсис)
import-scrivener-report-item-project-note = { $path } (нотатка проєкту)
import-scrivener-report-item-recovered = { $path } (відновлений текст)
# An item with no name in Scrivener.
import-scrivener-report-untitled = Без назви
import-scrivener-report-no-findings = Імпорту більше нема про що повідомити.
import-scrivener-report-group-unread = Не вдалося прочитати
import-scrivener-report-group-check = Перевірте
import-scrivener-report-group-changed = Перенесено інакше
import-scrivener-report-group-left = Не перенесено
import-scrivener-report-group-count = { $group } ({ $count })
import-scrivener-report-items-toggle = { $count ->
    [one] { $count } елемент
    [few] { $count } елементи
    [many] { $count } елементів
   *[other] { $count } елемента
}
# The same, as a screen reader names it: with the line whose items it lists.
import-scrivener-report-items-toggle-named = { $count ->
    [one] { $count } елемент: { $line }
    [few] { $count } елементи: { $line }
    [many] { $count } елементів: { $line }
   *[other] { $count } елемента: { $line }
}
import-scrivener-report-names-toggle = { $count ->
    [one] { $count } назва
    [few] { $count } назви
    [many] { $count } назв
   *[other] { $count } назви
}
# The same, as a screen reader hears it, naming the line it belongs to.
import-scrivener-report-names-toggle-named = { $count ->
    [one] { $count } назва: { $line }
    [few] { $count } назви: { $line }
    [many] { $count } назв: { $line }
   *[other] { $count } назви: { $line }
}
import-scrivener-report-more = І ще { $count }.
import-scrivener-report-open-item = Відкрити в редакторі
import-scrivener-report-links-hint = Відкрийте новий проєкт, а потім знову покажіть цей звіт із палітри команд: тоді кожен перелічений тут елемент відкриватиметься клацанням.
import-scrivener-copy-report = Копіювати звіт
import-scrivener-report-copied = Звіт скопійовано
import-scrivener-report-copy-failed = Не вдалося скопіювати звіт.
import-scrivener-report-close = Закрити
import-scrivener-report-none-title = Немає звіту про імпорт
import-scrivener-report-none = Відтоді як запущено Skribisto, не завершився жоден імпорт зі Scrivener. Skribisto зберігає звіти про останні імпорти, доки не закриється.
import-scrivener-report-gone = { $count ->
    [one] Звіт про цей імпорт більше не зберігається. Skribisto зберігає звіти про { $count } останній імпорт, доки не закриється.
    [few] Звіт про цей імпорт більше не зберігається. Skribisto зберігає звіти про { $count } останні імпорти, доки не закриється.
    [many] Звіт про цей імпорт більше не зберігається. Skribisto зберігає звіти про { $count } останніх імпортів, доки не закриється.
   *[other] Звіт про цей імпорт більше не зберігається. Skribisto зберігає звіти про { $count } останнього імпорту, доки не закриється.
}

## Words the import writes into the new project, in the language Skribisto shows itself in
import-scrivener-manuscript-binder = Рукопис
import-scrivener-notes-binder = Нотатки
import-scrivener-notes-of = { $title } (нотатки)
import-scrivener-project-notes = Нотатки проєкту
import-scrivener-recovered = Відновлені тексти
import-scrivener-untitled-template = Шаблон без назви
# The folder beside the new project its research files are copied into.
import-scrivener-research-folder = { $name } (файли досліджень)
# Between two authors' names.
import-scrivener-list-separator = {", "}
import-scrivener-label-details = З поля «{ $field }» у Scrivener
import-scrivener-status-details = З поля «{ $field }» у Scrivener
import-scrivener-unnamed-label = Позначка { $number }
import-scrivener-unnamed-status = Стан { $number }
import-scrivener-label-tag = { $name } (позначка)
import-scrivener-untitled-tag = Без назви
# The tags, the pace plan and the "From Scrivener" folder the import makes from what a
# Scrivener project holds beyond its labels and status.
import-scrivener-keyword-details = Ключове слово «{ $path }» у Scrivener
import-scrivener-unnamed-keyword = Ключове слово { $number }
import-scrivener-keyword-tag = { $name } (ключове слово)
import-scrivener-collection-details = Колекція в Scrivener
import-scrivener-collection-tag = { $name } (колекція)
import-scrivener-field-details = З поля «{ $field }» у Scrivener
# A tag made from one option of a list field.
import-scrivener-field-value = { $field }: { $option }
import-scrivener-field-tag = { $name } (поле)
# The day off a pace plan takes on its deadline.
import-scrivener-deadline-day = Кінцевий термін
import-scrivener-from-scrivener = Зі Scrivener
import-scrivener-note-project = Проєкт
import-scrivener-note-project-bookmarks = Закладки проєкту
import-scrivener-note-replacements = Заміни під час компонування
import-scrivener-note-auto-complete = Записи автодоповнення
import-scrivener-note-saved-searches = Збережені пошуки
import-scrivener-note-custom-metadata = Власні метадані
import-scrivener-note-targets = Норми
import-scrivener-note-bookmarks = Закладки
import-scrivener-abbreviated-title = Скорочена назва
import-scrivener-subjects = Теми
import-scrivener-company = Компанія
import-scrivener-session-target = Норма сеансу
import-scrivener-session-target-calculated = Норма сеансу, обчислена Scrivener за кінцевим терміном
import-scrivener-writing-days = Дні письма
import-scrivener-weekday-monday = понеділок
import-scrivener-weekday-tuesday = вівторок
import-scrivener-weekday-wednesday = середа
import-scrivener-weekday-thursday = четвер
import-scrivener-weekday-friday = п’ятниця
import-scrivener-weekday-saturday = субота
import-scrivener-weekday-sunday = неділя
import-scrivener-draft-target = Норма чернетки
import-scrivener-deadline = Кінцевий термін
# A unit after a count, in parentheses: "1000 (words)".
import-scrivener-unit-words = слів
import-scrivener-unit-characters = символів
import-scrivener-unit-pages = сторінок
import-scrivener-search-query = Пошуковий запит
import-scrivener-search-in = Область пошуку
import-scrivener-search-operator = Відповідність
# What a saved search looks in, by the kind Scrivener names.
import-scrivener-search-in-all = усе
import-scrivener-search-in-title = назви
import-scrivener-search-in-text = текст
import-scrivener-search-in-notes = нотатки
import-scrivener-search-in-synopsis = синопсиси
import-scrivener-search-in-keywords = ключові слова
import-scrivener-search-in-label = позначки
import-scrivener-search-in-status = стани
import-scrivener-search-in-section-type = типи секцій
import-scrivener-search-in-custom-metadata = власні метадані
# How a saved search's words combine.
import-scrivener-search-any = будь-яке слово
import-scrivener-search-all = усі слова
import-scrivener-search-exact = точна фраза
# One line of a listing: a bookmark's place and its note, or a link's words and its address.
import-scrivener-label-value = { $label }: { $value }
import-scrivener-replacement-regex = регулярний вираз
import-scrivener-replacement-off = вимкнено
# Said of a note whose own text lists its custom metadata or bookmarks.
import-scrivener-in-own-note = Перелічено у власній нотатці.


## Import a Scrivener project: what the import says about the project, in the writer's words.
## Every key the converter can give has a line here (see `import_scrivener::messages`).

# A refusal on one line: its title, then what to do.
import-scrivener-paragraph = { $title }. { $body }

## When the project cannot be imported
import-scrivener-refusal-missing-title = За цим шляхом нічого немає
import-scrivener-refusal-missing-body = «{ $file }» більше немає. Виберіть проєкт ще раз.
import-scrivener-refusal-unreadable-title = Цей проєкт не вдається прочитати
import-scrivener-refusal-unreadable-body = Skribisto не зміг прочитати «{ $file }» ({ $reason }). Переконайтеся, що можете його відкрити, а потім спробуйте ще раз.
import-scrivener-refusal-not-a-scrivener-project-title = Тут немає проєкту Scrivener
import-scrivener-refusal-not-a-scrivener-project-body = Виберіть теку .scriv проєкту, файл .scrivx у ній, резервну копію .zip або файл .scrivtemplate.
import-scrivener-refusal-manifest-without-files-title = У цій копії немає документів
import-scrivener-refusal-manifest-without-files-body = «{ $manifest }» тут є, але немає теки Files, у якій лежать перелічені в ньому документи. Можливо, служба синхронізації ще не завершила копіювання проєкту, або скопійовано лише файл .scrivx. Якщо ви вибрали файл .scrivx, виберіть натомість теку проєкту кнопкою «Вибрати теку…».
import-scrivener-refusal-search-too-wide-title = Ця тека завелика для пошуку
import-scrivener-refusal-search-too-wide-body = Тек для перегляду в ній більше, ніж { $limit }. Виберіть сам проєкт Scrivener.
import-scrivener-refusal-scrivener-one-for-mac-title = Цей проєкт створено в Scrivener 1 для Mac
import-scrivener-refusal-scrivener-one-for-mac-body = Skribisto читає проєкти Scrivener 2 і новіших версій. Відкрийте його один раз у Scrivener 2 або 3, щоб оновити, або скомпонуйте його в документ Word та імпортуйте цей документ.
import-scrivener-refusal-ios-stub-without-mobile-data-title = Це лише заглушка зі Scrivener для iPhone або iPad
import-scrivener-refusal-ios-stub-without-mobile-data-body = Самого проєкту тут немає. Синхронізуйте його з iPhone чи iPad або надішліть звідти як .zip та імпортуйте цей файл.
import-scrivener-refusal-newer-title = Цей проєкт створено в новішій версії Scrivener
import-scrivener-refusal-unknown-format-version-body = Його формату, { $version }, Skribisto не знає. Перевірте, чи є оновлення Skribisto. А доти скомпонуйте проєкт у Scrivener у документ Word та імпортуйте цей документ.
import-scrivener-refusal-newer-format-stamp-body = Його файли позначено як версію { $stamp }, а Skribisto читає версії до { $newest } включно. Перевірте, чи є оновлення Skribisto. А доти скомпонуйте проєкт у Scrivener у документ Word та імпортуйте цей документ.
import-scrivener-refusal-version-stamp-unreadable-title = Не вдається прочитати версію проєкту
import-scrivener-refusal-version-stamp-unreadable-body = Його файл Files/version.txt не містить номера версії, тож Skribisto не може визначити, чи вміє читати цей проєкт. Відкрийте проєкт один раз у Scrivener, а потім імпортуйте його знову.
import-scrivener-refusal-damaged-manifest-title = Не вдається прочитати файл підшивки
import-scrivener-refusal-damaged-manifest-body = «{ $manifest }» пошкоджено. { $spare_copies ->
    [0] Відкрийте проєкт у Scrivener, який уміє відновити цей файл, а потім імпортуйте проєкт знову.
    [1] Scrivener зберігає всередині проєкту його запасну копію, якої Skribisto не читає. Відкрийте проєкт у Scrivener, який уміє відновити цей файл, а потім імпортуйте проєкт знову.
    [one] Scrivener зберігає всередині проєкту { $spare_copies } запасну копію цього файлу, яких Skribisto не читає. Відкрийте проєкт у Scrivener, який уміє відновити цей файл, а потім імпортуйте проєкт знову.
    [few] Scrivener зберігає всередині проєкту { $spare_copies } запасні копії цього файлу, яких Skribisto не читає. Відкрийте проєкт у Scrivener, який уміє відновити цей файл, а потім імпортуйте проєкт знову.
    [many] Scrivener зберігає всередині проєкту { $spare_copies } запасних копій цього файлу, яких Skribisto не читає. Відкрийте проєкт у Scrivener, який уміє відновити цей файл, а потім імпортуйте проєкт знову.
   *[other] Scrivener зберігає всередині проєкту { $spare_copies } запасної копії цього файлу, яких Skribisto не читає. Відкрийте проєкт у Scrivener, який уміє відновити цей файл, а потім імпортуйте проєкт знову.
}
# The empty file Scrivener writes for a new project before its first save: not damage.
import-scrivener-refusal-damaged-manifest-new-project-placeholder-body = «{ $manifest }» є порожнім файлом, який Scrivener записує для ще не збереженого проєкту. Відкрийте проєкт у Scrivener і дайте йому зберегтися, а потім імпортуйте проєкт знову.
import-scrivener-refusal-manifest-too-large-title = Цей файл підшивки завеликий
import-scrivener-refusal-manifest-too-large-body = «{ $manifest }» більший за { $limit } МБ, які читає Skribisto. Жоден справжній проєкт не наближається до цього, тож файл, можливо, пошкоджено.
import-scrivener-refusal-no-item-identifiers-title = У підшивці немає жодного документа, який Skribisto міг би знайти
import-scrivener-refusal-no-item-identifiers-body = Жоден її елемент не має ідентифікатора, тож їхні файли неможливо знайти. Відкрийте проєкт один раз у Scrivener, а потім імпортуйте його знову.
import-scrivener-refusal-incomplete-copy-title = У цій копії бракує документів
import-scrivener-refusal-incomplete-copy-body = Тека Files є, але документів у ній немає. Можливо, служба синхронізації ще не завершила копіювання проєкту. Зачекайте, доки вона завершить, або скопіюйте всю теку .scriv ще раз, а потім натисніть «Перечитати проєкт».
import-scrivener-refusal-archive-title = Цей архів не вдається відкрити
import-scrivener-refusal-archive-unreadable-body = Не вдалося прочитати «{ $file }» ({ $reason }). Можливо, його пошкоджено. Розпакуйте його самостійно, а потім виберіть теку .scriv усередині.
import-scrivener-refusal-archive-unsafe-name-title = Цей архів неможливо безпечно відкрити
# A name with `..` in it, or an absolute one.
import-scrivener-refusal-archive-unsafe-name-body = «{ $file }» містить файл «{ $member }», який опинився б поза архівом, тож Skribisto не відкриває цей архів.
import-scrivener-refusal-archive-unsafe-name-empty-body = «{ $file }» містить файл без назви, тож Skribisto не відкриває цей архів.
import-scrivener-refusal-archive-unsafe-name-control-character-body = «{ $file }» містить файл, у назві якого є керівний символ, тож Skribisto не відкриває цей архів.
import-scrivener-refusal-archive-too-large-title = Цей архів завеликий, щоб безпечно його відкрити
import-scrivener-refusal-archive-too-many-members-body = Файлів у ньому більше, ніж { $limit }. Розпакуйте його самостійно, а потім виберіть теку .scriv усередині.
import-scrivener-refusal-archive-member-too-large-body = «{ $member }» розпаковується в понад { $limit } МБ. Розпакуйте архів самостійно, а потім виберіть теку .scriv усередині.
import-scrivener-refusal-archive-too-large-body = Він розпаковується в понад { $limit } МБ. Розпакуйте його самостійно, а потім виберіть теку .scriv усередині.
import-scrivener-refusal-archive-encrypted-title = Цей архів захищено паролем
import-scrivener-refusal-archive-encrypted-body = Розпакуйте «{ $file }» самостійно, а потім виберіть теку .scriv усередині.
import-scrivener-refusal-archive-unsupported-compression-title = Цей архів запаковано способом, який Skribisto не вміє читати
import-scrivener-refusal-archive-unsupported-compression-body = «{ $member }» стиснуто методом { $method }. Розпакуйте архів самостійно, а потім виберіть теку .scriv усередині.
import-scrivener-refusal-duplicate-archive-members-title = У цьому архіві деякі файли перелічено двічі
import-scrivener-refusal-duplicate-archive-members-body = «{ $file }» містить кілька файлів з однаковою назвою, тож Skribisto не може визначити, який із них належить до проєкту. Розпакуйте його самостійно, а потім виберіть теку .scriv усередині.
# What a file cut short looks like; a file too large, one that is not a template and one
# encoded another way have their own words.
import-scrivener-refusal-template-undecodable-title = Цей файл шаблону неповний
import-scrivener-refusal-template-undecodable-too-large-title = Цей файл шаблону завеликий, щоб його відкрити
import-scrivener-refusal-template-undecodable-not-a-template-title = Цей файл не є шаблоном Scrivener
import-scrivener-refusal-template-undecodable-unknown-encoding-title = Цей файл шаблону збережено у формі, якої Skribisto не знає
import-scrivener-refusal-template-undecodable-body = «{ $file }» не вдається декодувати: можливо, він обривається раніше, ніж закінчується проєкт у ньому. Завантажте його ще раз або відкрийте в Scrivener і створіть із нього проєкт.
import-scrivener-refusal-template-undecodable-too-large-body = «{ $file }» більший за { $limit } МБ. Відкрийте його в Scrivener, створіть із нього проєкт, а потім імпортуйте цей проєкт.
import-scrivener-refusal-template-undecodable-not-a-template-body = «{ $file }» не містить проєкту Scrivener. Виберіть файл шаблону, збережений Scrivener, або проєкт, створений із нього.
import-scrivener-refusal-template-undecodable-unknown-encoding-body = «{ $file }» зберігає свій проєкт у спосіб, який Skribisto не вміє декодувати. Відкрийте його в Scrivener, створіть із нього проєкт, а потім імпортуйте цей проєкт.
import-scrivener-refusal-cloud-placeholders-title = { $count ->
    [one] { $count } файл цього проєкту ще в хмарі
    [few] { $count } файли цього проєкту ще в хмарі
    [many] { $count } файлів цього проєкту ще в хмарі
   *[other] { $count } файлу цього проєкту ще в хмарі
}
import-scrivener-refusal-cloud-placeholders-body = Їх не завантажено на цей комп’ютер, наприклад { $examples }. Зробіть проєкт доступним офлайн у програмі синхронізації, а потім натисніть «Перечитати проєкт».
import-scrivener-refusal-unreadable-members-title = { $count ->
    [one] { $count } файл цього проєкту не вдається прочитати
    [few] { $count } файли цього проєкту не вдається прочитати
    [many] { $count } файлів цього проєкту не вдається прочитати
   *[other] { $count } файлу цього проєкту не вдається прочитати
}
import-scrivener-refusal-unreadable-members-body = Наприклад, { $examples }. Переконайтеся, що маєте право читати весь проєкт і що диск справний, а потім натисніть «Перечитати проєкт».
import-scrivener-refusal-needed-member-is-link-title = Цей проєкт посилається за межі власної теки
import-scrivener-refusal-needed-member-is-link-body = «{ $member }» є посиланням на інше місце на цьому комп’ютері, а Skribisto не переходить за посиланнями. Скопіюйте проєкт так, щоб усі файли були всередині нього, а потім спробуйте ще раз.
import-scrivener-refusal-project-changed-during-read-title = Проєкт змінився під час читання
import-scrivener-refusal-project-changed-during-read-body = Scrivener або служба синхронізації досі записує в нього. Закрийте його в Scrivener, зачекайте, доки синхронізація завершиться, а потім натисніть «Перечитати проєкт».
import-scrivener-refusal-too-many-members-title = У цьому проєкті забагато файлів
import-scrivener-refusal-too-many-members-body = Файлів у ньому більше, ніж { $limit }, тобто більше, ніж у будь-якому справжньому проєкті. Можливо, це не проєкт Scrivener або його пошкоджено.
import-scrivener-refusal-too-large-title = Цей проєкт завеликий
import-scrivener-refusal-too-large-body = Його текст займає понад { $limit } МБ, а це більше, ніж Skribisto може безпечно імпортувати.
import-scrivener-refusal-structure-unmappable-title = Skribisto не може відтворити цю підшивку
import-scrivener-refusal-structure-unmappable-body = Підшивку не вдалося перенести, нічого не втративши й не переставивши. Це помилка Skribisto, а не вашого проєкту. Будь ласка, повідомте про неї, додавши те, що показують «Подробиці».

## When there is a choice to make
import-scrivener-roots-title = { $count ->
    [one] Тут { $count } проєкт. Який із них?
    [few] Тут { $count } проєкти. Який із них?
    [many] Тут { $count } проєктів. Який із них?
   *[other] Тут { $count } проєкту. Який із них?
}
import-scrivener-roots-hint = У цій теці чи архіві кілька проєктів Scrivener. Виберіть той, який слід імпортувати.
import-scrivener-manifests-title = { $count ->
    [one] У цьому проєкті { $count } файл підшивки
    [few] У цьому проєкті { $count } файли підшивки
    [many] У цьому проєкті { $count } файлів підшивки
   *[other] У цьому проєкті { $count } файлу підшивки
}
import-scrivener-manifests-hint = Scrivener зберігає підшивку, тобто перелік усіх документів і тек, у файлі .scrivx. Зайві файли, імовірно, залишила служба синхронізації. Виберіть той, у якому ви працювали востаннє.
import-scrivener-manifests-hint-preselected = Scrivener зберігає підшивку, тобто перелік усіх документів і тек, у файлі .scrivx. Зайві файли, імовірно, залишила служба синхронізації. Вибраний файл, найімовірніше, той, у якому ви працювали востаннє: перевірте це, перш ніж продовжити.
import-scrivener-manifest-row = { $file } · збережено { $date } · { $documents ->
    [one] { $documents } елемент
    [few] { $documents } елементи
    [many] { $documents } елементів
   *[other] { $documents } елемента
}
import-scrivener-manifest-row-undated = { $file } · { $documents ->
    [one] { $documents } елемент
    [few] { $documents } елементи
    [many] { $documents } елементів
   *[other] { $documents } елемента
}
import-scrivener-manifest-row-unread = { $file } · не вдається прочитати
import-scrivener-enclosing-title = Це частина проєкту Scrivener «{ $project }»
import-scrivener-enclosing-body = Ви вибрали файл або теку всередині проєкту. Skribisto імпортує весь проєкт.
import-scrivener-enclosing-use = Імпортувати «{ $project }»
import-scrivener-upgrade-title = Це копія, яку Scrivener зберіг перед оновленням
import-scrivener-upgrade-body = Коли Scrivener оновлює проєкт до новішої версії, він зберігає старий як «{ $backup }». Проєкт, у якому ви продовжили писати, називається «{ $current }» і лежить поруч.
import-scrivener-upgrade-body-unclear = Коли Scrivener оновлює проєкт до новішої версії, він зберігає старий як «{ $backup }». Проєкт поруч, «{ $current }», не вдалося прочитати без помилок, тож виберіть, який із них імпортувати.
import-scrivener-upgrade-use-current = Імпортувати «{ $current }», проєкт, у якому ви продовжили писати
import-scrivener-upgrade-keep-backup = Імпортувати цю старішу копію, «{ $backup }»

## What must be accepted before going on
import-scrivener-ios-pending-title = Зміни, зроблені в Scrivener для iPhone або iPad, ще не потрапили в цю копію
import-scrivener-ios-pending-body = Scrivener на комп’ютері переносить ці зміни, коли наступного разу відкриває проєкт. Відкрийте проєкт один раз у Scrivener на комп’ютері, а потім імпортуйте його знову.
import-scrivener-ios-pending-accept = Імпортувати без цих змін
import-scrivener-no-text-title = { $count ->
    [one] { $count } документ, і в цій копії немає жодного слова тексту
    [few] { $count } документи, і в цій копії немає жодного слова тексту
    [many] { $count } документів, і в цій копії немає жодного слова тексту
   *[other] { $count } документа, і в цій копії немає жодного слова тексту
}
import-scrivener-no-text-body = Можливо, служба синхронізації ще копіює проєкт, або копія втратила свої документи. Якщо імпортувати зараз, усі документи надійдуть порожніми.
import-scrivener-no-text-accept = Імпортувати документи порожніми

## Things to know, nothing to decide
import-scrivener-notice-project-found-below-pick-title = Проєкт знайдено всередині вибраної теки
import-scrivener-notice-project-found-below-pick-body = Він розташований тут: «{ $path }».
import-scrivener-notice-source-is-template-title = Це шаблон Scrivener
import-scrivener-notice-source-is-template-named = Це шаблон Scrivener «{ $title }»
import-scrivener-notice-source-is-template-body = Імпорт створить новий проєкт із вмісту шаблону, разом з його інструкціями.
import-scrivener-notice-source-is-backup-title = Схоже, це резервна копія, створена Scrivener
import-scrivener-notice-source-is-backup-body = Вона містить проєкт у тому стані, в якому він був на момент створення копії. Якщо ви писали й після того, виберіть сам проєкт.
import-scrivener-notice-importing-pre-upgrade-copy-title = Ви імпортуєте копію, яку Scrivener зберіг перед оновленням
import-scrivener-notice-importing-pre-upgrade-copy-body = Проєкт, у якому ви продовжили писати, лежить поруч і містить усе, що написано відтоді.
import-scrivener-notice-imported-from-ios-stub-title = Цей проєкт створено в Scrivener для iPhone або iPad
import-scrivener-notice-imported-from-ios-stub-body = Його ще жодного разу не відкривали на комп’ютері, тож він читається з копії, яку зберігає iPhone чи iPad.
import-scrivener-notice-stale-mobile-binder-ignored-title = Стару підшивку з iPhone чи iPad не враховано
import-scrivener-notice-stale-mobile-binder-ignored-body = Scrivener на комп’ютері вже переніс її зміни.
import-scrivener-notice-identical-manifest-copies-title = { $count ->
    [1] Тут є копія файлу підшивки
    [one] Тут { $count } однакова копія файлу підшивки
    [few] Тут { $count } однакові копії файлу підшивки
    [many] Тут { $count } однакових копій файлу підшивки
   *[other] Тут { $count } однакової копії файлу підшивки
}
import-scrivener-notice-identical-manifest-copies-body = Вони однакові, тож будь-яка з них дає той самий проєкт.
import-scrivener-notice-manifest-copies-ignored-title = Зайві копії файлу підшивки не враховано
import-scrivener-notice-manifest-copies-ignored-body = Scrivener востаннє працював у «{ $chosen }». { $count ->
    [one] { $count } копію, залишену службою синхронізації, не враховано.
    [few] { $count } копії, залишені службою синхронізації, не враховано.
    [many] { $count } копій, залишених службою синхронізації, не враховано.
   *[other] { $count } копії, залишеної службою синхронізації, не враховано.
}
import-scrivener-notice-pinned-manifest-looks-like-a-copy-title = Вибраний файл підшивки схожий на копію, створену синхронізацією
import-scrivener-notice-pinned-manifest-looks-like-a-copy-body = «{ $name }» названо як копію «{ $original }», який лежить поруч. Якщо імпорт виглядатиме застарілим, виберіть натомість «{ $original }».
import-scrivener-notice-pinned-manifest-has-siblings-title = { $count ->
    [one] Поруч із вибраним файлом підшивки є ще { $count } файл підшивки
    [few] Поруч із вибраним файлом підшивки є ще { $count } файли підшивки
    [many] Поруч із вибраним файлом підшивки є ще { $count } файлів підшивки
   *[other] Поруч із вибраним файлом підшивки є ще { $count } файлу підшивки
}
import-scrivener-notice-pinned-manifest-has-siblings-body = Читається лише вибраний вами файл.
import-scrivener-notice-legacy-v1-files-beside-title = Файли зі старішого Scrivener не враховано
import-scrivener-notice-legacy-v1-files-beside-body = Вони залишилися після оновлення проєкту. Читаються лише файли поточної версії.
import-scrivener-notice-mixed-payload-trees-title = У цьому проєкті є файли двох версій Scrivener
import-scrivener-notice-mixed-payload-trees-body = Документи читаються там, де їх указує файл підшивки.
import-scrivener-notice-version-disagreement-title = Файли цього проєкту не узгоджуються щодо його версії
import-scrivener-notice-id-scheme-mismatch-body = Файл підшивки вказує версію { $version }, але називає документи так, як це робить інша версія. Вони читаються там, куди він указує. Якщо імпорт виглядатиме неправильно, відкрийте проєкт один раз у Scrivener та імпортуйте його знову.
import-scrivener-notice-version-stamp-mismatch-body = Він читається так, як його описує файл підшивки. Якщо імпорт виглядатиме неправильно, відкрийте проєкт один раз у Scrivener та імпортуйте його знову.
import-scrivener-notice-unrecognised-writer-title = Цей проєкт збережено програмою, якої Skribisto не розпізнає
import-scrivener-notice-unrecognised-writer-named = Вона називає себе «{ $creator }». Проєкт читається як проєкт Scrivener.
import-scrivener-notice-unrecognised-writer-body = Вона ніяк себе не називає. Проєкт читається як проєкт Scrivener.
import-scrivener-notice-invalid-item-identifier-title = { $count ->
    [one] { $count } елемент підшивки має недійсний ідентифікатор
    [few] { $count } елементи підшивки мають недійсні ідентифікатори
    [many] { $count } елементів підшивки мають недійсні ідентифікатори
   *[other] { $count } елемента підшивки мають недійсні ідентифікатори
}
import-scrivener-notice-invalid-item-identifier-body = Ці ідентифікатори не враховано. Елементи збережено.
import-scrivener-notice-items-without-identifier-title = { $count ->
    [one] { $count } елемент підшивки не має ідентифікатора
    [few] { $count } елементи підшивки не мають ідентифікатора
    [many] { $count } елементів підшивки не мають ідентифікатора
   *[other] { $count } елемента підшивки не мають ідентифікатора
}
import-scrivener-notice-items-without-identifier-body = Їхні файли неможливо знайти, тож вони надходять порожніми.
import-scrivener-notice-duplicate-item-key-title = Два елементи підшивки мають однаковий ідентифікатор: «{ $item }»
import-scrivener-notice-duplicate-item-key-body = Документи отримує перший. Другий зберігає своє місце й надходить порожнім.
import-scrivener-notice-lock-here-title = Цей проєкт відкрито в Scrivener на цьому комп’ютері
import-scrivener-notice-lock-here-body = Спершу закрийте його в Scrivener, щоб усі зміни було збережено.
import-scrivener-notice-lock-title = Цей проєкт був відкритий у Scrivener на «{ $host }»
import-scrivener-notice-lock-elsewhere-title = Цей проєкт був відкритий у Scrivener на іншому комп’ютері
import-scrivener-notice-lock-body = Якщо він досі там відкритий, спершу закрийте його: у файлах може ще не бути останніх змін. Якщо Scrivener закрили давно, проєкт можна імпортувати як є.
import-scrivener-notice-archived-while-open-title = Цей архів створено, коли проєкт був відкритий у Scrivener
import-scrivener-notice-archived-while-open-body = Змін, зроблених після того моменту, у ньому може не бути.
import-scrivener-notice-content-missing-notes = Нотаток «{ $item }» немає
import-scrivener-notice-content-missing-synopsis = Синопсису «{ $item }» немає
import-scrivener-notice-content-missing-text = Тексту «{ $item }» немає
import-scrivener-notice-content-missing-body = Відповідного файлу немає в цій копії, тож цей вміст надходить порожнім. Можливо, служба синхронізації ще копіює проєкт.
import-scrivener-notice-content-missing-salvaged = Відповідного файлу немає в цій копії, але пошуковий індекс Scrivener ще зберігав ці слова. Їх збережено як простий текст, можливо, зі старішої версії.
import-scrivener-notice-content-truncated-notes = Нотатки «{ $item }» на диску порожні
import-scrivener-notice-content-truncated-synopsis = Синопсис «{ $item }» на диску порожній
import-scrivener-notice-content-truncated-text = Текст «{ $item }» на диску порожній
import-scrivener-notice-content-truncated-body = Scrivener записав, що тут є текст, але файл нічого не містить: можливо, синхронізація зупинилася на півдорозі. Цей вміст надходить порожнім.
import-scrivener-notice-missing-media-title = Файлу «{ $item }» немає
import-scrivener-notice-missing-media-body = Зображення чи файлу досліджень, що міститься в ньому, немає в цій копії.
import-scrivener-notice-sync-conflict-copies-title = { $count ->
    [one] Знайдено { $count } копію документа, створену синхронізацією
    [few] Знайдено { $count } копії документів, створені синхронізацією
    [many] Знайдено { $count } копій документів, створених синхронізацією
   *[other] Знайдено { $count } копії документів, створеної синхронізацією
}
import-scrivener-notice-sync-conflict-copies-body = { $missing ->
    [0] Вони переносяться як відновлені тексти, в окремій теці, щоб ви могли порівняти їх із документами.
   *[other] Вони переносяться як відновлені тексти, в окремій теці. Деякі з них є копіями документів, яких уже немає в проєкті.
}
import-scrivener-notice-recovered-unreferenced-text-title = { $count ->
    [one] Знайдено { $count } документ, не вказаний у жодному елементі підшивки
    [few] Знайдено { $count } документи, не вказані в жодному елементі підшивки
    [many] Знайдено { $count } документів, не вказаних у жодному елементі підшивки
   *[other] Знайдено { $count } документа, не вказаного в жодному елементі підшивки
}
import-scrivener-notice-recovered-unreferenced-text-body = Вони переносяться як відновлені тексти, в окремій теці.
import-scrivener-notice-sync-debris-ignored-title = { $count ->
    [one] { $count } зайву копію, залишену службою синхронізації, не враховано
    [few] { $count } зайві копії, залишені службою синхронізації, не враховано
    [many] { $count } зайвих копій, залишених службою синхронізації, не враховано
   *[other] { $count } зайвої копії, залишеної службою синхронізації, не враховано
}
import-scrivener-notice-sync-debris-ignored-body = Scrivener теж їх не читає. Читаються лише власні файли проєкту.
import-scrivener-notice-unrecognised-files-title = { $count ->
    [one] { $count } файл, що не належить до проєкту Scrivener, не враховано
    [few] { $count } файли, що не належать до проєкту Scrivener, не враховано
    [many] { $count } файлів, що не належать до проєкту Scrivener, не враховано
   *[other] { $count } файлу, що не належать до проєкту Scrivener, не враховано
}
import-scrivener-notice-examples = Наприклад, { $examples }.
import-scrivener-notice-symlink-not-followed-title = { $count ->
    [one] За { $count } посиланням усередині проєкту Skribisto не переходить
    [few] За { $count } посиланнями всередині проєкту Skribisto не переходить
    [many] За { $count } посиланнями всередині проєкту Skribisto не переходить
   *[other] За { $count } посилання всередині проєкту Skribisto не переходить
}
import-scrivener-notice-unreadable-files-skipped-title = { $count ->
    [one] { $count } файл, непотрібний для імпорту, не вдалося прочитати
    [few] { $count } файли, непотрібні для імпорту, не вдалося прочитати
    [many] { $count } файлів, непотрібних для імпорту, не вдалося прочитати
   *[other] { $count } файлу, непотрібного для імпорту, не вдалося прочитати
}
import-scrivener-notice-cloud-placeholders-skipped-title = { $count ->
    [one] { $count } файл, що ще в хмарі, пропущено
    [few] { $count } файли, що ще в хмарі, пропущено
    [many] { $count } файлів, що ще в хмарі, пропущено
   *[other] { $count } файлу, що ще в хмарі, пропущено
}
import-scrivener-notice-cloud-placeholders-skipped-body = Вони не потрібні для імпорту.
import-scrivener-notice-no-text-found-title = { $count ->
    [1] { $count } документ, і в ньому немає тексту
    [one] { $count } документ, і в жодному з них немає тексту
    [few] { $count } документи, і в жодному з них немає тексту
    [many] { $count } документів, і в жодному з них немає тексту
   *[other] { $count } документа, і в жодному з них немає тексту
}
import-scrivener-notice-no-text-found-body = Цей проєкт схожий на порожній. Він імпортується таким, як є.
import-scrivener-notice-metadata-unreadable-title = Не вдалося прочитати «{ $member }»
import-scrivener-notice-metadata-unreadable-body = Те, що Scrivener у ньому зберігав, не переноситься.
import-scrivener-notice-dangling-keyword-title = { $count ->
    [one] { $count } посилання вказує на ключове слово, якого проєкт не визначає
    [few] { $count } посилання вказують на ключові слова, яких проєкт не визначає
    [many] { $count } посилань вказують на ключові слова, яких проєкт не визначає
   *[other] { $count } посилання вказують на ключові слова, яких проєкт не визначає
}
import-scrivener-notice-dangling-label-title = { $count ->
    [one] { $count } посилання вказує на позначку, якої проєкт не визначає
    [few] { $count } посилання вказують на позначки, яких проєкт не визначає
    [many] { $count } посилань вказують на позначки, яких проєкт не визначає
   *[other] { $count } посилання вказують на позначки, яких проєкт не визначає
}
import-scrivener-notice-dangling-status-title = { $count ->
    [one] { $count } посилання вказує на стан, якого проєкт не визначає
    [few] { $count } посилання вказують на стани, яких проєкт не визначає
    [many] { $count } посилань вказують на стани, яких проєкт не визначає
   *[other] { $count } посилання вказують на стани, яких проєкт не визначає
}
import-scrivener-notice-dangling-section-type-title = { $count ->
    [one] { $count } посилання вказує на тип секції, якого проєкт не визначає
    [few] { $count } посилання вказують на типи секцій, яких проєкт не визначає
    [many] { $count } посилань вказують на типи секцій, яких проєкт не визначає
   *[other] { $count } посилання вказують на типи секцій, яких проєкт не визначає
}
import-scrivener-notice-dangling-custom-field-title = { $count ->
    [one] { $count } посилання вказує на власне поле метаданих, якого проєкт не визначає
    [few] { $count } посилання вказують на власні поля метаданих, яких проєкт не визначає
    [many] { $count } посилань вказують на власні поля метаданих, яких проєкт не визначає
   *[other] { $count } посилання вказують на власні поля метаданих, яких проєкт не визначає
}
import-scrivener-notice-dangling-item-title = { $count ->
    [one] { $count } посилання вказує на документ, якого проєкт не визначає
    [few] { $count } посилання вказують на документи, яких проєкт не визначає
    [many] { $count } посилань вказують на документи, яких проєкт не визначає
   *[other] { $count } посилання вказують на документи, яких проєкт не визначає
}
import-scrivener-notice-dangling-references-body = Їх пропущено.
import-scrivener-notice-snapshots-not-imported-line = { $count ->
    [one] { $count } знімок лишається у вашому проєкті Scrivener: Skribisto не переносить знімки.
    [few] { $count } знімки лишаються у вашому проєкті Scrivener: Skribisto не переносить знімки.
    [many] { $count } знімків лишаються у вашому проєкті Scrivener: Skribisto не переносить знімки.
   *[other] { $count } знімка лишаються у вашому проєкті Scrivener: Skribisto не переносить знімки.
}
import-scrivener-notice-writing-history-not-imported-line = { $days ->
    [one] Записи Scrivener про кількість слів, написаних щодня ({ $days } день), не переносяться. Skribisto починає власний облік з першого збереження.
    [few] Записи Scrivener про кількість слів, написаних щодня ({ $days } дні), не переносяться. Skribisto починає власний облік з першого збереження.
    [many] Записи Scrivener про кількість слів, написаних щодня ({ $days } днів), не переносяться. Skribisto починає власний облік з першого збереження.
   *[other] Записи Scrivener про кількість слів, написаних щодня ({ $days } дня), не переносяться. Skribisto починає власний облік з першого збереження.
}
import-scrivener-notice-title-placeholder-rejected-hint = Назва в налаштуваннях проєкту є заповнювачем із шаблону, тож натомість запропоновано ім’я проєкту.
import-scrivener-notice-author-placeholder-rejected-hint = Ім’я автора в налаштуваннях проєкту є заповнювачем із шаблону, тож жодного не запропоновано.
import-scrivener-notice-title-from-project-hint-compile-settings = Цю назву вказано в налаштуваннях компонування проєкту. Доки там не задано назви, Scrivener зберігає ім’я, яке проєкт мав під час створення.
import-scrivener-notice-title-from-project-hint-template = Це власна назва шаблону.
import-scrivener-notice-title-from-project-hint-folder-name = Це ім’я проєкту.
import-scrivener-notice-title-from-project-hint-project-properties = Цю назву вказано у властивостях проєкту.
import-scrivener-notice-author-from-project-hint = Це ім’я вказано в налаштуваннях проєкту. Там може лишитися автор шаблону, і тоді його ім’я надрукується на титульній сторінці.
import-scrivener-notice-language-not-stated-hint = Проєкт не вказує своєї мови, тож запропоновано мову інтерфейсу Skribisto. Від неї залежить, як рахуються слова й пишуться лапки.
import-scrivener-notice-language-from-script-hint = Запропоновано за писемністю, якою написано рукопис ({ $tag }).

## Before you import: what the import does with the binder, or leaves behind
import-scrivener-structure-script-mode-draft-line = { $documents ->
    [1] Єдиний документ рукопису написано в режимі сценарію. Skribisto зберігає його слова, але не сценарне форматування.
    [one] { $script ->
        [1] Один із { $documents } документа рукопису написано в режимі сценарію. Skribisto зберігає його слова, але не сценарне форматування.
       *[other] { $script } з { $documents } документа рукопису написано в режимі сценарію. Skribisto зберігає їхні слова, але не сценарне форматування.
    }
   *[other] { $script ->
        [1] Один із { $documents } документів рукопису написано в режимі сценарію. Skribisto зберігає його слова, але не сценарне форматування.
       *[other] { $script } з { $documents } документів рукопису написано в режимі сценарію. Skribisto зберігає їхні слова, але не сценарне форматування.
    }
}
import-scrivener-structure-one-document-manuscript-line = { $count ->
    [1] Один документ рукопису містить свої розділи як заголовки.
    [one] { $count } документ рукопису містить свої розділи як заголовки.
    [few] { $count } документи рукопису містять свої розділи як заголовки.
    [many] { $count } документів рукопису містять свої розділи як заголовки.
   *[other] { $count } документа рукопису містять свої розділи як заголовки.
}
import-scrivener-structure-title-repeats-number-line = { $count ->
    [one] { $count } назва розділу чи частини містить і свій номер, тож експорт може надрукувати номер двічі.
    [few] { $count } назви розділів чи частин містять і свої номери, тож експорт може надрукувати номер двічі.
    [many] { $count } назв розділів чи частин містять і свої номери, тож експорт може надрукувати номер двічі.
   *[other] { $count } назви розділів чи частин містять і свої номери, тож експорт може надрукувати номер двічі.
}
import-scrivener-structure-claim-overruled-line = { $count ->
    [one] { $count } елемент отримує рівень, нижчий за заявлений, бо елемент вище вже займає цей рівень.
    [few] { $count } елементи отримують рівень, нижчий за заявлений, бо елемент вище вже займає цей рівень.
    [many] { $count } елементів отримують рівень, нижчий за заявлений, бо елемент вище вже займає цей рівень.
   *[other] { $count } елемента отримують рівень, нижчий за заявлений, бо елемент вище вже займає цей рівень.
}
import-scrivener-structure-adopted-by-chapter-line = { $count ->
    [one] { $count } документ, що йде після розділу, стає кінцем цього розділу.
    [few] { $count } документи, що йдуть після розділу, стають кінцем цього розділу.
    [many] { $count } документів, що йдуть після розділу, стають кінцем цього розділу.
   *[other] { $count } документа, що йдуть після розділу, стають кінцем цього розділу.
}
import-scrivener-structure-looks-like-manuscript-line = { $count ->
    [one] { $count } тека верхнього рівня, залишена в нотатках, схожа на рукопис. Її можна перенести до рукопису як іншу книгу.
    [few] { $count } теки верхнього рівня, залишені в нотатках, схожі на рукопис. Їх можна перенести до рукопису як інші книги.
    [many] { $count } тек верхнього рівня, залишених у нотатках, схожі на рукопис. Їх можна перенести до рукопису як інші книги.
   *[other] { $count } теки верхнього рівня, залишеної в нотатках, схожі на рукопис. Їх можна перенести до рукопису як інші книги.
}
import-scrivener-structure-generic-sheet-title-line = { $count ->
    [one] { $count } запис довідника історії має назву з шаблону, наприклад «Герой», і згадки шукатимуть її в тексті.
    [few] { $count } записи довідника історії мають назви з шаблону, наприклад «Герой», і згадки шукатимуть їх у тексті.
    [many] { $count } записів довідника історії мають назви з шаблону, наприклад «Герой», і згадки шукатимуть їх у тексті.
   *[other] { $count } запису довідника історії мають назви з шаблону, наприклад «Герой», і згадки шукатимуть їх у тексті.
}
import-scrivener-structure-placeholder-title-line = { $count ->
    [one] { $count } назва лише називала свій рівень і лишається порожньою: підшивка й експорт нумерують її самі.
    [few] { $count } назви лише називали свій рівень і лишаються порожніми: підшивка й експорт нумерують їх самі.
    [many] { $count } назв лише називали свій рівень і лишаються порожніми: підшивка й експорт нумерують їх самі.
   *[other] { $count } назви лише називали свій рівень і лишаються порожніми: підшивка й експорт нумерують їх самі.
}
import-scrivener-structure-untitled-item-line = { $count ->
    [one] { $count } елемент не мав назви в Scrivener і лишається без назви.
    [few] { $count } елементи не мали назви в Scrivener і лишаються без назви.
    [many] { $count } елементів не мали назви в Scrivener і лишаються без назви.
   *[other] { $count } елемента не мали назви в Scrivener і лишаються без назви.
}
import-scrivener-structure-title-cleaned-line = { $count ->
    [one] { $count } назва містила розриви рядків чи табуляції, їх вилучено.
    [few] { $count } назви містили розриви рядків чи табуляції, їх вилучено.
    [many] { $count } назв містили розриви рядків чи табуляції, їх вилучено.
   *[other] { $count } назви містили розриви рядків чи табуляції, їх вилучено.
}
import-scrivener-structure-own-text-moved-line = { $count ->
    [one] { $count } тека має власний текст, який переноситься в перший документ усередині неї.
    [few] { $count } теки мають власний текст, який переноситься в перший документ усередині них.
    [many] { $count } тек мають власний текст, який переноситься в перший документ усередині них.
   *[other] { $count } теки мають власний текст, який переноситься в перший документ усередині них.
}
import-scrivener-structure-text-with-children-line = { $count ->
    [one] { $count } документ із вкладеними документами стає текою.
    [few] { $count } документи з вкладеними документами стають теками.
    [many] { $count } документів із вкладеними документами стають теками.
   *[other] { $count } документа з вкладеними документами стають теками.
}
import-scrivener-structure-empty-folder-plain-line = { $count ->
    [one] { $count } порожня тека лишається звичайною текою, щоб жоден експорт не надрукував заголовка, якого Scrivener ніколи не друкував.
    [few] { $count } порожні теки лишаються звичайними теками, щоб жоден експорт не надрукував заголовка, якого Scrivener ніколи не друкував.
    [many] { $count } порожніх тек лишаються звичайними теками, щоб жоден експорт не надрукував заголовка, якого Scrivener ніколи не друкував.
   *[other] { $count } порожньої теки лишаються звичайними теками, щоб жоден експорт не надрукував заголовка, якого Scrivener ніколи не друкував.
}
import-scrivener-structure-book-inside-book-line = { $count ->
    [one] { $count } книга всередині книги стає частиною.
    [few] { $count } книги всередині книги стають частинами.
    [many] { $count } книг усередині книги стають частинами.
   *[other] { $count } книги всередині книги стають частинами.
}
import-scrivener-structure-prologue-not-numbered-line = { $count ->
    [one] { $count } пролог чи епілог не входить до нумерації розділів.
    [few] { $count } прологи чи епілоги не входять до нумерації розділів.
    [many] { $count } прологів чи епілогів не входять до нумерації розділів.
   *[other] { $count } прологу чи епілогу не входять до нумерації розділів.
}
import-scrivener-structure-matter-inside-book-line = { $count ->
    [one] { $count } елемент початкових чи кінцевих сторінок з-поза рукопису переходить усередину книги як паратекст і не потрапляє в експорт, доки ви не ввімкнете для нього «Включати в експорт».
    [few] { $count } елементи початкових чи кінцевих сторінок з-поза рукопису переходять усередину книги як паратекст і не потрапляють в експорт, доки ви не ввімкнете для них «Включати в експорт».
    [many] { $count } елементів початкових чи кінцевих сторінок з-поза рукопису переходять усередину книги як паратекст і не потрапляють в експорт, доки ви не ввімкнете для них «Включати в експорт».
   *[other] { $count } елемента початкових чи кінцевих сторінок з-поза рукопису переходять усередину книги як паратекст і не потрапляють в експорт, доки ви не ввімкнете для них «Включати в експорт».
}
import-scrivener-structure-paratext-not-counted-line = { $documents ->
    [one] { $documents } документ рукопису стає паратекстом (початковими чи кінцевими сторінками): його { $words ->
        [one] { $words } слово більше не зараховується
        [few] { $words } слова більше не зараховуються
        [many] { $words } слів більше не зараховуються
       *[other] { $words } слова більше не зараховуються
    } до рукопису.
    [few] { $documents } документи рукопису стають паратекстом (початковими чи кінцевими сторінками): їхні { $words ->
        [one] { $words } слово більше не зараховується
        [few] { $words } слова більше не зараховуються
        [many] { $words } слів більше не зараховуються
       *[other] { $words } слова більше не зараховуються
    } до рукопису.
    [many] { $documents } документів рукопису стають паратекстом (початковими чи кінцевими сторінками): їхні { $words ->
        [one] { $words } слово більше не зараховується
        [few] { $words } слова більше не зараховуються
        [many] { $words } слів більше не зараховуються
       *[other] { $words } слова більше не зараховуються
    } до рукопису.
   *[other] { $documents } документа рукопису стають паратекстом (початковими чи кінцевими сторінками): їхні { $words ->
        [one] { $words } слово більше не зараховується
        [few] { $words } слова більше не зараховуються
        [many] { $words } слів більше не зараховуються
       *[other] { $words } слова більше не зараховуються
    } до рукопису.
}
import-scrivener-structure-document-notes-line = { $count ->
    [one] Нотатки { $count } документа стають нотаткою поруч із ним.
    [few] Нотатки { $count } документів стають нотатками поруч із ними.
    [many] Нотатки { $count } документів стають нотатками поруч із ними.
   *[other] Нотатки { $count } документа стають нотатками поруч із ними.
}
import-scrivener-structure-draft-content-as-note-line = Власний текст теки рукопису стає нотаткою на початку рукопису.
import-scrivener-structure-story-bible-folder-line = { $count ->
    [one] { $count } тека персонажів чи місць стає міткою довідника історії.
    [few] { $count } теки персонажів чи місць стають мітками довідника історії.
    [many] { $count } тек персонажів чи місць стають мітками довідника історії.
   *[other] { $count } теки персонажів чи місць стають мітками довідника історії.
}
import-scrivener-structure-templates-created-line = { $count ->
    [one] { $count } шаблон документа стає шаблоном нотатки.
    [few] { $count } шаблони документів стають шаблонами нотаток.
    [many] { $count } шаблонів документів стають шаблонами нотаток.
   *[other] { $count } шаблону документа стають шаблонами нотаток.
}
import-scrivener-structure-template-nesting-flattened-line = { $count ->
    [one] { $count } теку шаблонів, вкладену в іншу, розгорнуто: кожен текст у ній стає шаблоном.
    [few] { $count } теки шаблонів, вкладені в інші, розгорнуто: кожен текст у них стає шаблоном.
    [many] { $count } тек шаблонів, вкладених в інші, розгорнуто: кожен текст у них стає шаблоном.
   *[other] { $count } теки шаблонів, вкладеної в інші, розгорнуто: кожен текст у них стає шаблоном.
}
import-scrivener-structure-template-kept-as-note-line = { $count ->
    [one] { $count } шаблон містить те, чого не може містити шаблон нотатки, тож його збережено як нотатку.
    [few] { $count } шаблони містять те, чого не може містити шаблон нотатки, тож їх збережено як нотатки.
    [many] { $count } шаблонів містять те, чого не може містити шаблон нотатки, тож їх збережено як нотатки.
   *[other] { $count } шаблону містять те, чого не може містити шаблон нотатки, тож їх збережено як нотатки.
}
import-scrivener-structure-recovered-texts-line = { $count ->
    [one] { $count } текст, не вказаний у жодному елементі підшивки, переноситься в окрему теку.
    [few] { $count } тексти, не вказані в жодному елементі підшивки, переносяться в окрему теку.
    [many] { $count } текстів, не вказаних у жодному елементі підшивки, переносяться в окрему теку.
   *[other] { $count } тексту, не вказаного в жодному елементі підшивки, переносяться в окрему теку.
}
import-scrivener-structure-project-notes-line = { $count ->
    [one] { $count } нотатка проєкту переноситься в окрему теку.
    [few] { $count } нотатки проєкту переносяться в окрему теку.
    [many] { $count } нотаток проєкту переносяться в окрему теку.
   *[other] { $count } нотатки проєкту переносяться в окрему теку.
}
import-scrivener-structure-trash-brought-across-line = { $items ->
    [1] Кошик переноситься: { $items } елемент, який можна відновити з кошика.
    [one] Кошик переноситься: { $items } елемент, із них { $writing } з текстом, і їх можна відновити з кошика.
    [few] Кошик переноситься: { $items } елементи, із них { $writing } з текстом, і їх можна відновити з кошика.
    [many] Кошик переноситься: { $items } елементів, із них { $writing } з текстом, і їх можна відновити з кошика.
   *[other] Кошик переноситься: { $items } елемента, із них { $writing } з текстом, і їх можна відновити з кошика.
}
import-scrivener-structure-not-compiled-line = { $count ->
    [one] { $count } елемент, який Scrivener не включав у компонування, так само не включається в експорт.
    [few] { $count } елементи, які Scrivener не включав у компонування, так само не включаються в експорт.
    [many] { $count } елементів, які Scrivener не включав у компонування, так само не включаються в експорт.
   *[other] { $count } елемента, які Scrivener не включав у компонування, так само не включаються в експорт.
}
import-scrivener-structure-media-in-draft-line = { $count ->
    [one] { $count } зображення чи файл у рукописі стає нотаткою, бо книга не може його містити.
    [few] { $count } зображення чи файли в рукописі стають нотатками, бо книга не може їх містити.
    [many] { $count } зображень чи файлів у рукописі стають нотатками, бо книга не може їх містити.
   *[other] { $count } зображення чи файлу в рукописі стають нотатками, бо книга не може їх містити.
}
import-scrivener-structure-left-out-line = { $count ->
    [one] { $count } теку верхнього рівня не перенесено разом з { $items ->
        [one] { $items } елементом
        [few] { $items } елементами
        [many] { $items } елементами
       *[other] { $items } елемента
    }.
    [few] { $count } теки верхнього рівня не перенесено разом з { $items ->
        [one] { $items } елементом
        [few] { $items } елементами
        [many] { $items } елементами
       *[other] { $items } елемента
    }.
    [many] { $count } тек верхнього рівня не перенесено разом з { $items ->
        [one] { $items } елементом
        [few] { $items } елементами
        [many] { $items } елементами
       *[other] { $items } елемента
    }.
   *[other] { $count } теки верхнього рівня не перенесено разом з { $items ->
        [one] { $items } елементом
        [few] { $items } елементами
        [many] { $items } елементами
       *[other] { $items } елемента
    }.
}
import-scrivener-structure-trash-left-out-line = { $items ->
    [1] Кошик не перенесено: { $items } елемент.
    [one] Кошик не перенесено: { $items } елемент, із них { $writing } з текстом.
    [few] Кошик не перенесено: { $items } елементи, із них { $writing } з текстом.
    [many] Кошик не перенесено: { $items } елементів, із них { $writing } з текстом.
   *[other] Кошик не перенесено: { $items } елемента, із них { $writing } з текстом.
}
import-scrivener-structure-card-image-left-behind-line = { $count ->
    [one] { $count } зображення на картці елемента рукопису не перенесено: воно надрукувалося б у книзі.
    [few] { $count } зображення на картках елементів рукопису не перенесено: вони надрукувалися б у книзі.
    [many] { $count } зображень на картках елементів рукопису не перенесено: вони надрукувалися б у книзі.
   *[other] { $count } зображення на картках елементів рукопису не перенесено: вони надрукувалися б у книзі.
}
import-scrivener-structure-default-template-not-linked-line = { $count ->
    [one] Типовий шаблон документа { $count } теки не може перейти разом із нею.
    [few] Типові шаблони документів { $count } тек не можуть перейти разом із ними.
    [many] Типові шаблони документів { $count } тек не можуть перейти разом із ними.
   *[other] Типові шаблони документів { $count } теки не можуть перейти разом із ними.
}
import-scrivener-structure-page-break-not-kept-line = { $count ->
    [one] { $count } налаштування «Розрив сторінки перед» не має відповідника в Skribisto, тож його не збережено.
    [few] { $count } налаштування «Розрив сторінки перед» не мають відповідника в Skribisto, тож їх не збережено.
    [many] { $count } налаштувань «Розрив сторінки перед» не мають відповідника в Skribisto, тож їх не збережено.
   *[other] { $count } налаштування «Розрив сторінки перед» не мають відповідника в Skribisto, тож їх не збережено.
}
import-scrivener-structure-compile-as-is-not-kept-line = { $count ->
    [one] { $count } налаштування «Компонувати як є» не має відповідника в Skribisto, тож його не збережено.
    [few] { $count } налаштування «Компонувати як є» не мають відповідника в Skribisto, тож їх не збережено.
    [many] { $count } налаштувань «Компонувати як є» не мають відповідника в Skribisto, тож їх не збережено.
   *[other] { $count } налаштування «Компонувати як є» не мають відповідника в Skribisto, тож їх не збережено.
}

## What the import did with the labels, the status list and the dates
import-scrivener-metadata-status-categories-guessed-line = { $count ->
    [one] { $count } стан, який Skribisto не розпізнає, отримує категорію за своїм місцем у списку. Перевірте його в «Налаштування ▸ Проєкт ▸ Стани».
    [few] { $count } стани, які Skribisto не розпізнає, отримують категорію за своїм місцем у списку. Перевірте їх у «Налаштування ▸ Проєкт ▸ Стани».
    [many] { $count } станів, які Skribisto не розпізнає, отримують категорію за своїм місцем у списку. Перевірте їх у «Налаштування ▸ Проєкт ▸ Стани».
   *[other] { $count } стану, які Skribisto не розпізнає, отримують категорію за своїм місцем у списку. Перевірте їх у «Налаштування ▸ Проєкт ▸ Стани».
}
import-scrivener-metadata-dates-unreadable-line = { $count ->
    [one] Не вдалося прочитати дати { $count } елемента: він отримує дату імпорту.
    [few] Не вдалося прочитати дати { $count } елементів: вони отримують дату імпорту.
    [many] Не вдалося прочитати дати { $count } елементів: вони отримують дату імпорту.
   *[other] Не вдалося прочитати дати { $count } елемента: вони отримують дату імпорту.
}
import-scrivener-metadata-labels-became-tags-line = { $tags ->
    [one] Позначки перетворюються на { $tags } мітку, застосовану до { $rows ->
        [one] { $rows } елемента
        [few] { $rows } елементів
        [many] { $rows } елементів
       *[other] { $rows } елемента
    }.
    [few] Позначки перетворюються на { $tags } мітки, застосовані до { $rows ->
        [one] { $rows } елемента
        [few] { $rows } елементів
        [many] { $rows } елементів
       *[other] { $rows } елемента
    }.
    [many] Позначки перетворюються на { $tags } міток, застосованих до { $rows ->
        [one] { $rows } елемента
        [few] { $rows } елементів
        [many] { $rows } елементів
       *[other] { $rows } елемента
    }.
   *[other] Позначки перетворюються на { $tags } мітки, застосовані до { $rows ->
        [one] { $rows } елемента
        [few] { $rows } елементів
        [many] { $rows } елементів
       *[other] { $rows } елемента
    }.
}
import-scrivener-metadata-statuses-became-ladder-line = { $rungs ->
    [one] Список станів перетворюється на { $rungs } стан проєкту, застосований до { $rows ->
        [one] { $rows } елемента
        [few] { $rows } елементів
        [many] { $rows } елементів
       *[other] { $rows } елемента
    }.
    [few] Список станів перетворюється на { $rungs } стани проєкту, застосовані до { $rows ->
        [one] { $rows } елемента
        [few] { $rows } елементів
        [many] { $rows } елементів
       *[other] { $rows } елемента
    }.
    [many] Список станів перетворюється на { $rungs } станів проєкту, застосованих до { $rows ->
        [one] { $rows } елемента
        [few] { $rows } елементів
        [many] { $rows } елементів
       *[other] { $rows } елемента
    }.
   *[other] Список станів перетворюється на { $rungs } стану проєкту, застосовані до { $rows ->
        [one] { $rows } елемента
        [few] { $rows } елементів
        [many] { $rows } елементів
       *[other] { $rows } елемента
    }.
}
import-scrivener-metadata-ladder-seeded-line = { $count ->
    [1] У проєкті не було списку станів, тож він починає з типового стану Skribisto.
    [one] У проєкті не було списку станів, тож він починає з { $count } типового стану Skribisto.
    [few] У проєкті не було списку станів, тож він починає з { $count } типових станів Skribisto.
    [many] У проєкті не було списку станів, тож він починає з { $count } типових станів Skribisto.
   *[other] У проєкті не було списку станів, тож він починає з { $count } типового стану Skribisto.
}
import-scrivener-metadata-label-renamed-line = Поле «Позначка» в Scrivener називалося «{ $title }». Цю назву вказано в подробицях кожної мітки.
import-scrivener-metadata-status-renamed-line = Поле «Стан» у Scrivener називалося «{ $title }». Цю назву вказано в подробицях кожного стану.
import-scrivener-metadata-none-label-collapsed-line = { $count ->
    [one] Позначка «{ $name }», що існує поряд із власною порожньою позначкою Scrivener під тією самою назвою, читається як відсутність позначки на { $count } елементі.
    [few] Позначка «{ $name }», що існує поряд із власною порожньою позначкою Scrivener під тією самою назвою, читається як відсутність позначки на { $count } елементах.
    [many] Позначка «{ $name }», що існує поряд із власною порожньою позначкою Scrivener під тією самою назвою, читається як відсутність позначки на { $count } елементах.
   *[other] Позначка «{ $name }», що існує поряд із власною порожньою позначкою Scrivener під тією самою назвою, читається як відсутність позначки на { $count } елемента.
}
import-scrivener-metadata-none-status-collapsed-line = { $count ->
    [one] Стан «{ $name }», що існує поряд із власним порожнім станом Scrivener під тією самою назвою, читається як відсутність стану на { $count } елементі.
    [few] Стан «{ $name }», що існує поряд із власним порожнім станом Scrivener під тією самою назвою, читається як відсутність стану на { $count } елементах.
    [many] Стан «{ $name }», що існує поряд із власним порожнім станом Scrivener під тією самою назвою, читається як відсутність стану на { $count } елементах.
   *[other] Стан «{ $name }», що існує поряд із власним порожнім станом Scrivener під тією самою назвою, читається як відсутність стану на { $count } елемента.
}
import-scrivener-metadata-unnamed-label-line = Позначка без назви отримує назву за своїм номером, { $id }.
import-scrivener-metadata-unnamed-status-line = Стан без назви отримує назву за своїм номером, { $id }.
import-scrivener-metadata-tag-name-disambiguated-line = { $count ->
    [one] { $count } мітка отримує нову назву, бо її назву вже має інша мітка.
    [few] { $count } мітки отримують нові назви, бо їхні назви вже мають інші мітки.
    [many] { $count } міток отримують нові назви, бо їхні назви вже мають інші мітки.
   *[other] { $count } мітки отримують нові назви, бо їхні назви вже мають інші мітки.
}
import-scrivener-metadata-rung-names-disambiguated-line = { $count ->
    [one] { $count } стан має таку саму назву, як інший, тож отримує номер після назви. Перевірте його в «Налаштування ▸ Проєкт ▸ Стани».
    [few] { $count } стани мають таку саму назву, як інші, тож кожен отримує номер після назви. Перевірте їх у «Налаштування ▸ Проєкт ▸ Стани».
    [many] { $count } станів мають таку саму назву, як інші, тож кожен отримує номер після назви. Перевірте їх у «Налаштування ▸ Проєкт ▸ Стани».
   *[other] { $count } стану мають таку саму назву, як інші, тож кожен отримує номер після назви. Перевірте їх у «Налаштування ▸ Проєкт ▸ Стани».
}
import-scrivener-metadata-unused-labels-skipped-line = { $count ->
    [one] { $count } позначку, якої не має жоден документ, не імпортовано.
    [few] { $count } позначки, яких не має жоден документ, не імпортовано.
    [many] { $count } позначок, яких не має жоден документ, не імпортовано.
   *[other] { $count } позначки, яких не має жоден документ, не імпортовано.
}
import-scrivener-metadata-labels-without-row-line = { $count ->
    [one] Позначка { $count } елемента, якому немає місця в новому проєкті, не переноситься.
    [few] Позначки { $count } елементів, яким немає місця в новому проєкті, не переносяться.
    [many] Позначки { $count } елементів, яким немає місця в новому проєкті, не переносяться.
   *[other] Позначки { $count } елемента, яким немає місця в новому проєкті, не переносяться.
}
import-scrivener-metadata-statuses-without-row-line = { $count ->
    [one] Стан { $count } елемента, якому немає місця в новому проєкті, не переноситься.
    [few] Стани { $count } елементів, яким немає місця в новому проєкті, не переносяться.
    [many] Стани { $count } елементів, яким немає місця в новому проєкті, не переносяться.
   *[other] Стани { $count } елемента, яким немає місця в новому проєкті, не переносяться.
}

import-scrivener-metadata-unnamed-keyword-line = Ключове слово без назви отримує назву за своїм номером, { $id }.
import-scrivener-metadata-keywords-without-row-line = { $count ->
    [one] Ключові слова { $count } елемента, якому немає місця в новому проєкті, не переносяться.
    [few] Ключові слова { $count } елементів, яким немає місця в новому проєкті, не переносяться.
    [many] Ключові слова { $count } елементів, яким немає місця в новому проєкті, не переносяться.
   *[other] Ключові слова { $count } елемента, яким немає місця в новому проєкті, не переносяться.
}
import-scrivener-metadata-custom-values-without-row-line = { $count ->
    [one] Власні метадані { $count } елемента, якому немає місця в новому проєкті, не переносяться.
    [few] Власні метадані { $count } елементів, яким немає місця в новому проєкті, не переносяться.
    [many] Власні метадані { $count } елементів, яким немає місця в новому проєкті, не переносяться.
   *[other] Власні метадані { $count } елемента, яким немає місця в новому проєкті, не переносяться.
}
import-scrivener-metadata-bookmarks-without-row-line = { $count ->
    [one] Закладки { $count } елемента, якому немає місця в новому проєкті, не переносяться.
    [few] Закладки { $count } елементів, яким немає місця в новому проєкті, не переносяться.
    [many] Закладки { $count } елементів, яким немає місця в новому проєкті, не переносяться.
   *[other] Закладки { $count } елемента, яким немає місця в новому проєкті, не переносяться.
}
import-scrivener-metadata-targets-without-row-line = { $count ->
    [one] Норма { $count } елемента, якому немає місця в новому проєкті, не переноситься.
    [few] Норми { $count } елементів, яким немає місця в новому проєкті, не переносяться.
    [many] Норми { $count } елементів, яким немає місця в новому проєкті, не переносяться.
   *[other] Норми { $count } елемента, яким немає місця в новому проєкті, не переносяться.
}

## What the import does with the keywords, custom metadata, collections, bookmarks, targets,
## the deadline, the auto-complete list and the icons. Each line is said twice: on the Review
## page before the import, then in its report, so it reads the same at both times. `$note`
## and `$folder` are the titles the import gives a note and its "From Scrivener" folder;
## `$day` is a day as YYYY-MM-DD.
import-scrivener-metadata-pov-hint-line = { $count ->
    [1] Позначка чи значення поля { $values } збігається із записом довідника історії { $matches }. Якщо так позначено точку зору, задайте її в інспекторі: імпорт нічого не пов’язує.
   *[other] Позначки чи значення полів { $values } збігаються із записами довідника історії { $matches }. Якщо так позначено точки зору, задайте їх в інспекторі: імпорт нічого не пов’язує.
}
import-scrivener-metadata-checkbox-default-unknown-line = Поле-прапорець «{ $field }» має типове значення, якого Skribisto не може прочитати, тож неможливо визначити, які документи позначено за замовчуванням. Його мітку отримують лише документи, позначені окремо.
import-scrivener-metadata-goals-in-other-unit-line = { $count ->
    [one] { $count } ціль задано в іншій одиниці підрахунку. Її збережено, і вона з’явиться, щойно ви зміните одиницю підрахунку в «Налаштування ▸ Проєкт ▸ Структура».
    [few] { $count } цілі задано в іншій одиниці підрахунку. Їх збережено, і вони з’являться, щойно ви зміните одиницю підрахунку в «Налаштування ▸ Проєкт ▸ Структура».
    [many] { $count } цілей задано в іншій одиниці підрахунку. Їх збережено, і вони з’являться, щойно ви зміните одиницю підрахунку в «Налаштування ▸ Проєкт ▸ Структура».
   *[other] { $count } цілі задано в іншій одиниці підрахунку. Їх збережено, і вони з’являться, щойно ви зміните одиницю підрахунку в «Налаштування ▸ Проєкт ▸ Структура».
}
import-scrivener-metadata-counting-differs-line = Scrivener рахував норму чернетки за іншими документами, ніж рахує Skribisto. Ціль книги враховує сцени, включені в експорт.
import-scrivener-metadata-writing-days-not-read-line = Дні письма «{ $raw }» не вдається прочитати, тож «Темп» книги вважає кожен день днем письма. Змініть їх там.
import-scrivener-metadata-script-mode-rows-line = { $count ->
    [one] { $count } елемент у Scrivener був у режимі сценарію. Skribisto зберігає його слова як прозу, без сценарного форматування.
    [few] { $count } елементи в Scrivener були в режимі сценарію. Skribisto зберігає їхні слова як прозу, без сценарного форматування.
    [many] { $count } елементів у Scrivener були в режимі сценарію. Skribisto зберігає їхні слова як прозу, без сценарного форматування.
   *[other] { $count } елемента в Scrivener були в режимі сценарію. Skribisto зберігає їхні слова як прозу, без сценарного форматування.
}
import-scrivener-metadata-keywords-became-tags-line = { $tags ->
    [one] { $tags } ключове слово стає міткою, застосованою до { $rows ->
        [one] { $rows } елемента
        [few] { $rows } елементів
        [many] { $rows } елементів
       *[other] { $rows } елемента
    }.
    [few] { $tags } ключові слова стають мітками, застосованими до { $rows ->
        [one] { $rows } елемента
        [few] { $rows } елементів
        [many] { $rows } елементів
       *[other] { $rows } елемента
    }.
    [many] { $tags } ключових слів стають мітками, застосованими до { $rows ->
        [one] { $rows } елемента
        [few] { $rows } елементів
        [many] { $rows } елементів
       *[other] { $rows } елемента
    }.
   *[other] { $tags } ключового слова стають мітками, застосованими до { $rows ->
        [one] { $rows } елемента
        [few] { $rows } елементів
        [many] { $rows } елементів
       *[other] { $rows } елемента
    }.
}
import-scrivener-metadata-keyword-hierarchy-flattened-line = { $count ->
    [1] Одне ключове слово в Scrivener було вкладене в інше. Мітки не мають вкладеності: у подробицях його мітки вказано, де воно було.
    [one] { $count } ключове слово в Scrivener було вкладене в інші. Мітки не мають вкладеності: у подробицях кожної мітки вказано, де було її ключове слово.
    [few] { $count } ключові слова в Scrivener були вкладені в інші. Мітки не мають вкладеності: у подробицях кожної мітки вказано, де було її ключове слово.
    [many] { $count } ключових слів у Scrivener були вкладені в інші. Мітки не мають вкладеності: у подробицях кожної мітки вказано, де було її ключове слово.
   *[other] { $count } ключового слова в Scrivener були вкладені в інші. Мітки не мають вкладеності: у подробицях кожної мітки вказано, де було її ключове слово.
}
import-scrivener-metadata-fields-became-tags-line = { $fields ->
    [one] { $fields } поле-список чи поле-прапорець перетворюється на { $tags ->
        [one] { $tags } мітку, застосовану
        [few] { $tags } мітки, застосовані
        [many] { $tags } міток, застосованих
       *[other] { $tags } мітки, застосовані
    } до { $rows ->
        [one] { $rows } елемента
        [few] { $rows } елементів
        [many] { $rows } елементів
       *[other] { $rows } елемента
    }.
    [few] { $fields } поля-списки й поля-прапорці перетворюються на { $tags ->
        [one] { $tags } мітку, застосовану
        [few] { $tags } мітки, застосовані
        [many] { $tags } міток, застосованих
       *[other] { $tags } мітки, застосовані
    } до { $rows ->
        [one] { $rows } елемента
        [few] { $rows } елементів
        [many] { $rows } елементів
       *[other] { $rows } елемента
    }.
    [many] { $fields } полів-списків і полів-прапорців перетворюються на { $tags ->
        [one] { $tags } мітку, застосовану
        [few] { $tags } мітки, застосовані
        [many] { $tags } міток, застосованих
       *[other] { $tags } мітки, застосовані
    } до { $rows ->
        [one] { $rows } елемента
        [few] { $rows } елементів
        [many] { $rows } елементів
       *[other] { $rows } елемента
    }.
   *[other] { $fields } поля-списку чи поля-прапорця перетворюються на { $tags ->
        [one] { $tags } мітку, застосовану
        [few] { $tags } мітки, застосовані
        [many] { $tags } міток, застосованих
       *[other] { $tags } мітки, застосовані
    } до { $rows ->
        [one] { $rows } елемента
        [few] { $rows } елементів
        [many] { $rows } елементів
       *[other] { $rows } елемента
    }.
}
import-scrivener-metadata-collections-became-tags-line = { $tags ->
    [one] { $tags } колекція стає міткою, застосованою до { $rows ->
        [one] { $rows } елемента
        [few] { $rows } елементів
        [many] { $rows } елементів
       *[other] { $rows } елемента
    }.
    [few] { $tags } колекції стають мітками, застосованими до { $rows ->
        [one] { $rows } елемента
        [few] { $rows } елементів
        [many] { $rows } елементів
       *[other] { $rows } елемента
    }.
    [many] { $tags } колекцій стають мітками, застосованими до { $rows ->
        [one] { $rows } елемента
        [few] { $rows } елементів
        [many] { $rows } елементів
       *[other] { $rows } елемента
    }.
   *[other] { $tags } колекції стають мітками, застосованими до { $rows ->
        [one] { $rows } елемента
        [few] { $rows } елементів
        [many] { $rows } елементів
       *[other] { $rows } елемента
    }.
}
import-scrivener-metadata-collection-order-lost-line = { $count ->
    [one] Мітка не зберігає порядку, тож порядок документів у { $count } колекції не збережено.
    [few] Мітка не зберігає порядку, тож порядок документів у { $count } колекціях не збережено.
    [many] Мітка не зберігає порядку, тож порядок документів у { $count } колекціях не збережено.
   *[other] Мітка не зберігає порядку, тож порядок документів у { $count } колекції не збережено.
}
import-scrivener-metadata-bookmarks-became-links-line = { $count ->
    [one] { $count } закладка стає зв’язком між двома нотатками або між сценою та записом довідника історії: див. «Зв’язки» і «Дійові особи».
    [few] { $count } закладки стають зв’язками між нотатками або між сценами та записами довідника історії: див. «Зв’язки» і «Дійові особи».
    [many] { $count } закладок стають зв’язками між нотатками або між сценами та записами довідника історії: див. «Зв’язки» і «Дійові особи».
   *[other] { $count } закладки стають зв’язками між нотатками або між сценами та записами довідника історії: див. «Зв’язки» і «Дійові особи».
}
import-scrivener-metadata-goals-kept-line = { $count ->
    [one] { $count } норма документа стає ціллю.
    [few] { $count } норми документів стають цілями.
    [many] { $count } норм документів стають цілями.
   *[other] { $count } норми документа стають цілями.
}
import-scrivener-metadata-draft-target-kept-words-line = { $value ->
    [one] Норма чернетки в { $value } слово стає ціллю книги.
    [few] Норма чернетки в { $value } слова стає ціллю книги.
    [many] Норма чернетки в { $value } слів стає ціллю книги.
   *[other] Норма чернетки в { $value } слова стає ціллю книги.
}
import-scrivener-metadata-draft-target-kept-characters-line = { $value ->
    [one] Норма чернетки в { $value } символ стає ціллю книги.
    [few] Норма чернетки в { $value } символи стає ціллю книги.
    [many] Норма чернетки в { $value } символів стає ціллю книги.
   *[other] Норма чернетки в { $value } символу стає ціллю книги.
}
import-scrivener-metadata-pace-planned-line = Кінцевий термін, { $day }, стає основою «Темпу» книги, який розподіляє слова на кожен день письма, вибраний вами в Scrivener, починаючи з дня імпорту.
import-scrivener-metadata-pace-planned-day-off-line = Кінцевий термін, { $day }, стає основою «Темпу» книги, який розподіляє слова на кожен день письма, вибраний вами в Scrivener, починаючи з дня імпорту. Як і в Scrivener, сам кінцевий термін є вихідним.
import-scrivener-metadata-dictionary-words-line = { $count ->
    [one] { $count } слово автодоповнення потрапляє до особистого словника проєкту, тож його не буде позначено як помилку.
    [few] { $count } слова автодоповнення потрапляють до особистого словника проєкту, тож їх не буде позначено як помилки.
    [many] { $count } слів автодоповнення потрапляють до особистого словника проєкту, тож їх не буде позначено як помилки.
   *[other] { $count } слова автодоповнення потрапляють до особистого словника проєкту, тож їх не буде позначено як помилки.
}
import-scrivener-metadata-listed-project-line = { $count ->
    [one] { $count } параметр проєкту перелічено в нотатці «{ $note }» теки «{ $folder }».
    [few] { $count } параметри проєкту перелічено в нотатці «{ $note }» теки «{ $folder }».
    [many] { $count } параметрів проєкту перелічено в нотатці «{ $note }» теки «{ $folder }».
   *[other] { $count } параметра проєкту перелічено в нотатці «{ $note }» теки «{ $folder }».
}
import-scrivener-metadata-listed-project-bookmarks-line = { $count ->
    [one] { $count } закладку проєкту перелічено в нотатці «{ $note }» теки «{ $folder }».
    [few] { $count } закладки проєкту перелічено в нотатці «{ $note }» теки «{ $folder }».
    [many] { $count } закладок проєкту перелічено в нотатці «{ $note }» теки «{ $folder }».
   *[other] { $count } закладки проєкту перелічено в нотатці «{ $note }» теки «{ $folder }».
}
import-scrivener-metadata-listed-replacements-line = { $count ->
    [one] { $count } заміну під час компонування перелічено в нотатці «{ $note }» теки «{ $folder }». Skribisto її не виконує.
    [few] { $count } заміни під час компонування перелічено в нотатці «{ $note }» теки «{ $folder }». Skribisto їх не виконує.
    [many] { $count } замін під час компонування перелічено в нотатці «{ $note }» теки «{ $folder }». Skribisto їх не виконує.
   *[other] { $count } заміни під час компонування перелічено в нотатці «{ $note }» теки «{ $folder }». Skribisto їх не виконує.
}
import-scrivener-metadata-listed-auto-complete-line = { $count ->
    [one] { $count } запис автодоповнення з кількох слів перелічено в нотатці «{ $note }» теки «{ $folder }».
    [few] { $count } записи автодоповнення з кількох слів перелічено в нотатці «{ $note }» теки «{ $folder }».
    [many] { $count } записів автодоповнення з кількох слів перелічено в нотатці «{ $note }» теки «{ $folder }».
   *[other] { $count } запису автодоповнення з кількох слів перелічено в нотатці «{ $note }» теки «{ $folder }».
}
import-scrivener-metadata-listed-saved-searches-line = { $count ->
    [one] { $count } збережений пошук перелічено в нотатці «{ $note }» теки «{ $folder }».
    [few] { $count } збережені пошуки перелічено в нотатці «{ $note }» теки «{ $folder }».
    [many] { $count } збережених пошуків перелічено в нотатці «{ $note }» теки «{ $folder }».
   *[other] { $count } збереженого пошуку перелічено в нотатці «{ $note }» теки «{ $folder }».
}
import-scrivener-metadata-listed-custom-metadata-line = { $count ->
    [one] { $count } значення власних метаданих, якого не може вмістити мітка, перелічено в нотатці «{ $note }» теки «{ $folder }».
    [few] { $count } значення власних метаданих, яких не може вмістити мітка, перелічено в нотатці «{ $note }» теки «{ $folder }».
    [many] { $count } значень власних метаданих, яких не може вмістити мітка, перелічено в нотатці «{ $note }» теки «{ $folder }».
   *[other] { $count } значення власних метаданих, яких не може вмістити мітка, перелічено в нотатці «{ $note }» теки «{ $folder }».
}
import-scrivener-metadata-listed-targets-line = { $count ->
    [one] { $count } норму документа перелічено в нотатці «{ $note }» теки «{ $folder }».
    [few] { $count } норми документів перелічено в нотатці «{ $note }» теки «{ $folder }».
    [many] { $count } норм документів перелічено в нотатці «{ $note }» теки «{ $folder }».
   *[other] { $count } норми документа перелічено в нотатці «{ $note }» теки «{ $folder }».
}
import-scrivener-metadata-listed-bookmarks-line = { $count ->
    [one] { $count } закладку, якої не показує жоден зв’язок Skribisto, перелічено в нотатці «{ $note }» теки «{ $folder }».
    [few] { $count } закладки, яких не показує жоден зв’язок Skribisto, перелічено в нотатці «{ $note }» теки «{ $folder }».
    [many] { $count } закладок, яких не показує жоден зв’язок Skribisto, перелічено в нотатці «{ $note }» теки «{ $folder }».
   *[other] { $count } закладки, яких не показує жоден зв’язок Skribisto, перелічено в нотатці «{ $note }» теки «{ $folder }».
}
import-scrivener-metadata-targets-on-notes-line = { $count ->
    [one] { $count } норма документа задана для нотатки, текст якої не враховує жодна ціль, тож вона не стає ціллю.
    [few] { $count } норми документів задані для нотаток, текст яких не враховує жодна ціль, тож вони не стають цілями.
    [many] { $count } норм документів задані для нотаток, текст яких не враховує жодна ціль, тож вони не стають цілями.
   *[other] { $count } норми документа задані для нотаток, текст яких не враховує жодна ціль, тож вони не стають цілями.
}
import-scrivener-metadata-targets-on-split-documents-line = { $count ->
    [one] { $count } норма документа задана для документа, розділеного за заголовками: його розділи разом не враховує жодна ціль, тож вона не стає ціллю.
    [few] { $count } норми документів задані для документів, розділених за заголовками: їхні розділи разом не враховує жодна ціль, тож вони не стають цілями.
    [many] { $count } норм документів задані для документів, розділених за заголовками: їхні розділи разом не враховує жодна ціль, тож вони не стають цілями.
   *[other] { $count } норми документа задані для документів, розділених за заголовками: їхні розділи разом не враховує жодна ціль, тож вони не стають цілями.
}
import-scrivener-metadata-bookmark-ends-without-row-line = { $count ->
    [one] { $count } закладка веде до документа, якому немає місця в новому проєкті, наприклад до шаблону документа. Її перелічено за місцем, де документ був у Scrivener.
    [few] { $count } закладки ведуть до документів, яким немає місця в новому проєкті, наприклад до шаблонів документів. Їх перелічено за місцями, де документи були в Scrivener.
    [many] { $count } закладок ведуть до документів, яким немає місця в новому проєкті, наприклад до шаблонів документів. Їх перелічено за місцями, де документи були в Scrivener.
   *[other] { $count } закладки ведуть до документів, яким немає місця в новому проєкті, наприклад до шаблонів документів. Їх перелічено за місцями, де документи були в Scrivener.
}
import-scrivener-metadata-unused-keywords-skipped-line = { $count ->
    [one] { $count } ключове слово, якого не має жоден документ, не імпортовано.
    [few] { $count } ключові слова, яких не має жоден документ, не імпортовано.
    [many] { $count } ключових слів, яких не має жоден документ, не імпортовано.
   *[other] { $count } ключового слова, яких не має жоден документ, не імпортовано.
}
import-scrivener-metadata-list-value-invalid-line = { $count ->
    [one] { $count } значення поля-списку «{ $field }» не відповідає жодному з його варіантів, тож його не імпортовано.
    [few] { $count } значення поля-списку «{ $field }» не відповідають жодному з його варіантів, тож їх не імпортовано.
    [many] { $count } значень поля-списку «{ $field }» не відповідають жодному з його варіантів, тож їх не імпортовано.
   *[other] { $count } значення поля-списку «{ $field }» не відповідають жодному з його варіантів, тож їх не імпортовано.
}
import-scrivener-metadata-undefined-custom-field-line = { $count ->
    [one] { $count } значення належить власному полю, якого проєкт більше не визначає і якого Scrivener теж не показує. Його не імпортовано.
    [few] { $count } значення належать власним полям, яких проєкт більше не визначає і яких Scrivener теж не показує. Їх не імпортовано.
    [many] { $count } значень належать власним полям, яких проєкт більше не визначає і яких Scrivener теж не показує. Їх не імпортовано.
   *[other] { $count } значення належать власним полям, яких проєкт більше не визначає і яких Scrivener теж не показує. Їх не імпортовано.
}
import-scrivener-metadata-collection-members-missing-line = { $count ->
    [one] { $count } документ колекції «{ $name }» не має місця в новому проєкті або зник із проєкту Scrivener.
    [few] { $count } документи колекції «{ $name }» не мають місця в новому проєкті або зникли з проєкту Scrivener.
    [many] { $count } документів колекції «{ $name }» не мають місця в новому проєкті або зникли з проєкту Scrivener.
   *[other] { $count } документа колекції «{ $name }» не мають місця в новому проєкті або зникли з проєкту Scrivener.
}
import-scrivener-metadata-empty-collections-skipped-line = { $count ->
    [one] { $count } колекцію без документів не імпортовано.
    [few] { $count } колекції без документів не імпортовано.
    [many] { $count } колекцій без документів не імпортовано.
   *[other] { $count } колекції без документів не імпортовано.
}
import-scrivener-metadata-bookmarks-unresolved-line = { $count ->
    [one] { $count } закладка веде до документа, якого вже немає в проєкті Scrivener, тож її не імпортовано.
    [few] { $count } закладки ведуть до документів, яких уже немає в проєкті Scrivener, тож їх не імпортовано.
    [many] { $count } закладок ведуть до документів, яких уже немає в проєкті Scrivener, тож їх не імпортовано.
   *[other] { $count } закладки ведуть до документів, яких уже немає в проєкті Scrivener, тож їх не імпортовано.
}
import-scrivener-metadata-targets-in-pages-line = { $count ->
    [one] { $count } норма документа рахує сторінки, яких Skribisto не рахує, тож вона не стає ціллю.
    [few] { $count } норми документів рахують сторінки, яких Skribisto не рахує, тож вони не стають цілями.
    [many] { $count } норм документів рахують сторінки, яких Skribisto не рахує, тож вони не стають цілями.
   *[other] { $count } норми документа рахують сторінки, яких Skribisto не рахує, тож вони не стають цілями.
}
import-scrivener-metadata-targets-unreadable-line = { $count ->
    [one] { $count } норму не імпортовано: її значення не є числом, менше за нуль або більше, ніж може вмістити ціль.
    [few] { $count } норми не імпортовано: їхні значення не є числами, менші за нуль або більші, ніж може вмістити ціль.
    [many] { $count } норм не імпортовано: їхні значення не є числами, менші за нуль або більші, ніж може вмістити ціль.
   *[other] { $count } норми не імпортовано: їхні значення не є числами, менші за нуль або більші, ніж може вмістити ціль.
}
import-scrivener-metadata-draft-target-in-pages-line = { $value ->
    [one] Норма чернетки в { $value } сторінку не стає ціллю: Skribisto рахує слова або символи. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
    [few] Норма чернетки в { $value } сторінки не стає ціллю: Skribisto рахує слова або символи. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
    [many] Норма чернетки в { $value } сторінок не стає ціллю: Skribisto рахує слова або символи. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
   *[other] Норма чернетки в { $value } сторінки не стає ціллю: Skribisto рахує слова або символи. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
}
import-scrivener-metadata-draft-target-no-book-words-line = { $value ->
    [one] Норма чернетки в { $value } слово не стає ціллю, бо рукопис утворює кілька книг або жодної. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
    [few] Норма чернетки в { $value } слова не стає ціллю, бо рукопис утворює кілька книг або жодної. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
    [many] Норма чернетки в { $value } слів не стає ціллю, бо рукопис утворює кілька книг або жодної. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
   *[other] Норма чернетки в { $value } слова не стає ціллю, бо рукопис утворює кілька книг або жодної. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
}
import-scrivener-metadata-draft-target-no-book-characters-line = { $value ->
    [one] Норма чернетки в { $value } символ не стає ціллю, бо рукопис утворює кілька книг або жодної. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
    [few] Норма чернетки в { $value } символи не стає ціллю, бо рукопис утворює кілька книг або жодної. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
    [many] Норма чернетки в { $value } символів не стає ціллю, бо рукопис утворює кілька книг або жодної. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
   *[other] Норма чернетки в { $value } символу не стає ціллю, бо рукопис утворює кілька книг або жодної. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
}
import-scrivener-metadata-deadline-passed-line = Кінцевий термін, { $day }, уже минув, тож «Темп» із нього не створюється. Його перелічено в нотатці «{ $note }» теки «{ $folder }».
import-scrivener-metadata-deadline-no-writing-day-line = Від дня імпорту до кінцевого терміну, { $day }, не лишається жодного дня письма, тож «Темп» із нього не створюється. Його перелічено в нотатці «{ $note }» теки «{ $folder }».
import-scrivener-metadata-deadline-template-line = Кінцевий термін, { $day }, було збережено в шаблоні, тож «Темп» із нього не створюється. Його перелічено в нотатці «{ $note }» теки «{ $folder }».
import-scrivener-metadata-deadline-no-word-goal-line = Із кінцевого терміну, { $day }, не створюється «Темп», бо чернетка не має норми в словах, до якої можна планувати. Його перелічено в нотатці «{ $note }» теки «{ $folder }».
import-scrivener-metadata-deadline-character-goal-line = Із кінцевого терміну, { $day }, не створюється «Темп», бо норма чернетки рахує символи, а «Темп» рахує слова. Його перелічено в нотатці «{ $note }» теки «{ $folder }».
import-scrivener-metadata-deadline-no-book-line = Із кінцевого терміну, { $day }, не створюється «Темп», бо рукопис утворює кілька книг або жодної. Його перелічено в нотатці «{ $note }» теки «{ $folder }».
import-scrivener-metadata-deadline-unreadable-line = Кінцевий термін не вдається прочитати, тож «Темп» із нього не створюється. Його перелічено так, як записано, в нотатці «{ $note }» теки «{ $folder }».
import-scrivener-metadata-session-target-words-line = { $value ->
    [one] Норму сеансу в { $value } слово не імпортовано: у Skribisto мета сеансу письма є одним налаштуванням для всіх проєктів. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
    [few] Норму сеансу в { $value } слова не імпортовано: у Skribisto мета сеансу письма є одним налаштуванням для всіх проєктів. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
    [many] Норму сеансу в { $value } слів не імпортовано: у Skribisto мета сеансу письма є одним налаштуванням для всіх проєктів. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
   *[other] Норму сеансу в { $value } слова не імпортовано: у Skribisto мета сеансу письма є одним налаштуванням для всіх проєктів. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
}
import-scrivener-metadata-session-target-characters-line = { $value ->
    [one] Норму сеансу в { $value } символ не імпортовано: у Skribisto мета сеансу письма є одним налаштуванням для всіх проєктів і задається в словах. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
    [few] Норму сеансу в { $value } символи не імпортовано: у Skribisto мета сеансу письма є одним налаштуванням для всіх проєктів і задається в словах. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
    [many] Норму сеансу в { $value } символів не імпортовано: у Skribisto мета сеансу письма є одним налаштуванням для всіх проєктів і задається в словах. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
   *[other] Норму сеансу в { $value } символу не імпортовано: у Skribisto мета сеансу письма є одним налаштуванням для всіх проєктів і задається в словах. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
}
import-scrivener-metadata-session-target-pages-line = { $value ->
    [one] Норму сеансу в { $value } сторінку не імпортовано: у Skribisto мета сеансу письма є одним налаштуванням для всіх проєктів і задається в словах. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
    [few] Норму сеансу в { $value } сторінки не імпортовано: у Skribisto мета сеансу письма є одним налаштуванням для всіх проєктів і задається в словах. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
    [many] Норму сеансу в { $value } сторінок не імпортовано: у Skribisto мета сеансу письма є одним налаштуванням для всіх проєктів і задається в словах. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
   *[other] Норму сеансу в { $value } сторінки не імпортовано: у Skribisto мета сеансу письма є одним налаштуванням для всіх проєктів і задається в словах. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
}
import-scrivener-metadata-session-target-calculated-words-line = { $value ->
    [one] Норму сеансу в { $value } слово, яку Scrivener обчислив за кінцевим терміном і днями письма, не імпортовано: у Skribisto мета сеансу письма є одним налаштуванням для всіх проєктів. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
    [few] Норму сеансу в { $value } слова, яку Scrivener обчислив за кінцевим терміном і днями письма, не імпортовано: у Skribisto мета сеансу письма є одним налаштуванням для всіх проєктів. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
    [many] Норму сеансу в { $value } слів, яку Scrivener обчислив за кінцевим терміном і днями письма, не імпортовано: у Skribisto мета сеансу письма є одним налаштуванням для всіх проєктів. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
   *[other] Норму сеансу в { $value } слова, яку Scrivener обчислив за кінцевим терміном і днями письма, не імпортовано: у Skribisto мета сеансу письма є одним налаштуванням для всіх проєктів. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
}
import-scrivener-metadata-session-target-calculated-characters-line = { $value ->
    [one] Норму сеансу в { $value } символ, яку Scrivener обчислив за кінцевим терміном і днями письма, не імпортовано: у Skribisto мета сеансу письма є одним налаштуванням для всіх проєктів і задається в словах. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
    [few] Норму сеансу в { $value } символи, яку Scrivener обчислив за кінцевим терміном і днями письма, не імпортовано: у Skribisto мета сеансу письма є одним налаштуванням для всіх проєктів і задається в словах. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
    [many] Норму сеансу в { $value } символів, яку Scrivener обчислив за кінцевим терміном і днями письма, не імпортовано: у Skribisto мета сеансу письма є одним налаштуванням для всіх проєктів і задається в словах. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
   *[other] Норму сеансу в { $value } символу, яку Scrivener обчислив за кінцевим терміном і днями письма, не імпортовано: у Skribisto мета сеансу письма є одним налаштуванням для всіх проєктів і задається в словах. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
}
import-scrivener-metadata-session-target-calculated-pages-line = { $value ->
    [one] Норму сеансу в { $value } сторінку, яку Scrivener обчислив за кінцевим терміном і днями письма, не імпортовано: у Skribisto мета сеансу письма є одним налаштуванням для всіх проєктів і задається в словах. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
    [few] Норму сеансу в { $value } сторінки, яку Scrivener обчислив за кінцевим терміном і днями письма, не імпортовано: у Skribisto мета сеансу письма є одним налаштуванням для всіх проєктів і задається в словах. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
    [many] Норму сеансу в { $value } сторінок, яку Scrivener обчислив за кінцевим терміном і днями письма, не імпортовано: у Skribisto мета сеансу письма є одним налаштуванням для всіх проєктів і задається в словах. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
   *[other] Норму сеансу в { $value } сторінки, яку Scrivener обчислив за кінцевим терміном і днями письма, не імпортовано: у Skribisto мета сеансу письма є одним налаштуванням для всіх проєктів і задається в словах. Її перелічено в нотатці «{ $note }» теки «{ $folder }».
}
import-scrivener-metadata-icons-dropped-line = { $count ->
    [one] { $count } елемент мав власний значок у підшивці, який не переноситься: Skribisto показує значок кожного елемента за його типом.
    [few] { $count } елементи мали власні значки в підшивці, які не переносяться: Skribisto показує значок кожного елемента за його типом.
    [many] { $count } елементів мали власні значки в підшивці, які не переносяться: Skribisto показує значок кожного елемента за його типом.
   *[other] { $count } елемента мали власні значки в підшивці, які не переносяться: Skribisto показує значок кожного елемента за його типом.
}

## What converting the prose did, as the import's report says it. `$count` is how many texts,
## or how many of a thing, as each line says; `$example` quotes the project.
import-scrivener-prose-unreadable-line = { $count ->
    [one] { $count } текст не вдалося прочитати. Він надходить порожнім.
    [few] { $count } тексти не вдалося прочитати. Вони надходять порожніми.
    [many] { $count } текстів не вдалося прочитати. Вони надходять порожніми.
   *[other] { $count } тексту не вдалося прочитати. Вони надходять порожніми.
}
import-scrivener-prose-comments-unreadable-line = { $count ->
    [one] Не вдалося прочитати коментарі й пов’язані виноски { $count } тексту. Слова, які вони позначали, збережено.
    [few] Не вдалося прочитати коментарі й пов’язані виноски { $count } текстів. Слова, які вони позначали, збережено.
    [many] Не вдалося прочитати коментарі й пов’язані виноски { $count } текстів. Слова, які вони позначали, збережено.
   *[other] Не вдалося прочитати коментарі й пов’язані виноски { $count } тексту. Слова, які вони позначали, збережено.
}
import-scrivener-prose-read-as-plain-text-line = { $count ->
    [one] { $count } текст було збережено як простий, а не форматований текст, і він читається саме так.
    [few] { $count } тексти було збережено як простий, а не форматований текст, і вони читаються саме так.
    [many] { $count } текстів було збережено як простий, а не форматований текст, і вони читаються саме так.
   *[other] { $count } тексту було збережено як простий, а не форматований текст, і вони читаються саме так.
}
import-scrivener-prose-inline-note-unmatched-line = { $count ->
    [one] { $count } маркер вбудованої примітки не має пари й зберігається як текст.
    [few] { $count } маркери вбудованих приміток не мають пари й зберігаються як текст.
    [many] { $count } маркерів вбудованих приміток не мають пари й зберігаються як текст.
   *[other] { $count } маркера вбудованих приміток не мають пари й зберігаються як текст.
}
import-scrivener-prose-style-unresolved-line = { $count ->
    [one] { $count } маркер стилю називає стиль, якого проєкт не визначає. Його вилучено, а текст зберігає своє форматування.
    [few] { $count } маркери стилів називають стилі, яких проєкт не визначає. Їх вилучено, а текст зберігає своє форматування.
    [many] { $count } маркерів стилів називають стилі, яких проєкт не визначає. Їх вилучено, а текст зберігає своє форматування.
   *[other] { $count } маркера стилів називають стилі, яких проєкт не визначає. Їх вилучено, а текст зберігає своє форматування.
}
import-scrivener-prose-styles-lost-in-export-line = { $count ->
    [one] { $count } абзац, можливо, є заголовком, що втратив свій стиль, коли Scrivener 3 експортував проєкт у свій старіший формат, і переноситься як звичайний абзац. Перевірте його або імпортуйте проєкт, з якого його експортовано.
    [few] { $count } абзаци, можливо, є заголовками, що втратили свій стиль, коли Scrivener 3 експортував проєкт у свій старіший формат, і переносяться як звичайні абзаци. Перевірте їх або імпортуйте проєкт, з якого їх експортовано.
    [many] { $count } абзаців, можливо, є заголовками, що втратили свій стиль, коли Scrivener 3 експортував проєкт у свій старіший формат, і переносяться як звичайні абзаци. Перевірте їх або імпортуйте проєкт, з якого їх експортовано.
   *[other] { $count } абзацу, можливо, є заголовками, що втратили свій стиль, коли Scrivener 3 експортував проєкт у свій старіший формат, і переносяться як звичайні абзаци. Перевірте їх або імпортуйте проєкт, з якого їх експортовано.
}
import-scrivener-prose-comment-anchor-without-entry-line = { $count ->
    [one] У { $count } коментаря чи виноски вміст було втрачено ще в Scrivener. Позначені ними слова збережено, але без них.
    [few] У { $count } коментарів чи виносок вміст було втрачено ще в Scrivener. Позначені ними слова збережено, але без них.
    [many] У { $count } коментарів чи виносок вміст було втрачено ще в Scrivener. Позначені ними слова збережено, але без них.
   *[other] У { $count } коментаря чи виноски вміст було втрачено ще в Scrivener. Позначені ними слова збережено, але без них.
}
import-scrivener-prose-comment-entry-without-anchor-line = { $count ->
    [one] { $count } коментар утратив своє місце в тексті, тож його прикріплено до першого абзацу.
    [few] { $count } коментарі втратили своє місце в тексті, тож їх прикріплено до першого абзацу.
    [many] { $count } коментарів утратили своє місце в тексті, тож їх прикріплено до першого абзацу.
   *[other] { $count } коментаря втратили своє місце в тексті, тож їх прикріплено до першого абзацу.
}
import-scrivener-prose-footnote-entry-without-anchor-line = { $count ->
    [one] { $count } пов’язана виноска втратила своє місце в тексті, тож посилання на неї поставлено в кінці тексту.
    [few] { $count } пов’язані виноски втратили своє місце в тексті, тож посилання на них поставлено в кінці тексту.
    [many] { $count } пов’язаних виносок утратили своє місце в тексті, тож посилання на них поставлено в кінці тексту.
   *[other] { $count } пов’язаної виноски втратили своє місце в тексті, тож посилання на них поставлено в кінці тексту.
}
import-scrivener-prose-range-comment-reading-line = { $count ->
    [one] { $count } коментар зі старішого Scrivener розміщено за іншим прочитанням його позиції. Перевірте, чи він стоїть на потрібних словах.
    [few] { $count } коментарі зі старішого Scrivener розміщено за іншим прочитанням їхньої позиції. Перевірте, чи вони стоять на потрібних словах.
    [many] { $count } коментарів зі старішого Scrivener розміщено за іншим прочитанням їхньої позиції. Перевірте, чи вони стоять на потрібних словах.
   *[other] { $count } коментаря зі старішого Scrivener розміщено за іншим прочитанням їхньої позиції. Перевірте, чи вони стоять на потрібних словах.
}
import-scrivener-prose-range-comment-reading-markers-counted-line = { $count ->
    [one] { $count } коментар зі старішого Scrivener розміщено з урахуванням маркерів його вбудованих приміток. Перевірте, чи він стоїть на потрібних словах.
    [few] { $count } коментарі зі старішого Scrivener розміщено з урахуванням маркерів їхніх вбудованих приміток. Перевірте, чи вони стоять на потрібних словах.
    [many] { $count } коментарів зі старішого Scrivener розміщено з урахуванням маркерів їхніх вбудованих приміток. Перевірте, чи вони стоять на потрібних словах.
   *[other] { $count } коментаря зі старішого Scrivener розміщено з урахуванням маркерів їхніх вбудованих приміток. Перевірте, чи вони стоять на потрібних словах.
}
import-scrivener-prose-range-comment-reading-pictures-not-counted-line = { $count ->
    [one] { $count } коментар зі старішого Scrivener розміщено без урахування зображень у тексті. Перевірте, чи він стоїть на потрібних словах.
    [few] { $count } коментарі зі старішого Scrivener розміщено без урахування зображень у тексті. Перевірте, чи вони стоять на потрібних словах.
    [many] { $count } коментарів зі старішого Scrivener розміщено без урахування зображень у тексті. Перевірте, чи вони стоять на потрібних словах.
   *[other] { $count } коментаря зі старішого Scrivener розміщено без урахування зображень у тексті. Перевірте, чи вони стоять на потрібних словах.
}
import-scrivener-prose-range-comment-approximate-line = { $count ->
    [one] { $count } коментар зі старішого Scrivener не відповідає точно жодному прочитанню своєї позиції й може бути зсунутий на кілька символів.
    [few] { $count } коментарі зі старішого Scrivener не відповідають точно жодному прочитанню своєї позиції й можуть бути зсунуті на кілька символів.
    [many] { $count } коментарів зі старішого Scrivener не відповідають точно жодному прочитанню своєї позиції й можуть бути зсунуті на кілька символів.
   *[other] { $count } коментаря зі старішого Scrivener не відповідають точно жодному прочитанню своєї позиції й можуть бути зсунуті на кілька символів.
}
import-scrivener-prose-document-link-unresolved-line = { $count ->
    [one] { $count } посилання вказує на документ, який імпорт не переніс. Його слова лишаються.
    [few] { $count } посилання вказують на документи, які імпорт не переніс. Їхні слова лишаються.
    [many] { $count } посилань вказують на документи, які імпорт не переніс. Їхні слова лишаються.
   *[other] { $count } посилання вказують на документи, які імпорт не переніс. Їхні слова лишаються.
}
import-scrivener-prose-linked-picture-not-read-line = { $count ->
    [1] Зображення, пов’язане з файлом на вашому диску, «{ $example }», не перенесено.
    [one] { $count } зображення, пов’язане з файлами на вашому диску, наприклад «{ $example }», не перенесено.
    [few] { $count } зображення, пов’язані з файлами на вашому диску, наприклад «{ $example }», не перенесено.
    [many] { $count } зображень, пов’язаних із файлами на вашому диску, наприклад «{ $example }», не перенесено.
   *[other] { $count } зображення, пов’язаного з файлами на вашому диску, наприклад «{ $example }», не перенесено.
}
import-scrivener-prose-heading-repeats-title-line = { $count ->
    [one] { $count } текст починається із заголовка, що повторює його назву, тож експорт може надрукувати назву двічі.
    [few] { $count } тексти починаються із заголовка, що повторює їхню назву, тож експорт може надрукувати назву двічі.
    [many] { $count } текстів починаються із заголовка, що повторює їхню назву, тож експорт може надрукувати назву двічі.
   *[other] { $count } тексту починаються із заголовка, що повторює їхню назву, тож експорт може надрукувати назву двічі.
}
import-scrivener-prose-markdown-as-text-line = { $count ->
    [one] { $count } абзац, написаний у Markdown, збережено таким, як є.
    [few] { $count } абзаци, написані в Markdown, збережено такими, як є.
    [many] { $count } абзаців, написаних у Markdown, збережено такими, як є.
   *[other] { $count } абзацу, написаного в Markdown, збережено такими, як є.
}
import-scrivener-prose-citation-keys-as-text-line = { $count ->
    [one] { $count } абзац містить ключі цитувань для бібліографічної програми, їх збережено як текст.
    [few] { $count } абзаци містять ключі цитувань для бібліографічної програми, їх збережено як текст.
    [many] { $count } абзаців містять ключі цитувань для бібліографічної програми, їх збережено як текст.
   *[other] { $count } абзацу містять ключі цитувань для бібліографічної програми, їх збережено як текст.
}
import-scrivener-prose-placeholders-in-exported-text-line = { $count ->
    [1] Заповнювач Scrivener { $example } збережено як простий текст у тому, що ви експортуєте. Замініть його вручну.
    [one] { $count } заповнювач Scrivener, наприклад { $example }, збережено як простий текст у тому, що ви експортуєте. Замініть їх вручну.
    [few] { $count } заповнювачі Scrivener, наприклад { $example }, збережено як простий текст у тому, що ви експортуєте. Замініть їх вручну.
    [many] { $count } заповнювачів Scrivener, наприклад { $example }, збережено як простий текст у тому, що ви експортуєте. Замініть їх вручну.
   *[other] { $count } заповнювача Scrivener, наприклад { $example }, збережено як простий текст у тому, що ви експортуєте. Замініть їх вручну.
}
import-scrivener-prose-placeholders-in-other-text-line = { $count ->
    [1] Заповнювач Scrivener { $example } збережено як простий текст у тексті, що не експортується.
    [one] { $count } заповнювач Scrivener, наприклад { $example }, збережено як простий текст у текстах, що не експортуються.
    [few] { $count } заповнювачі Scrivener, наприклад { $example }, збережено як простий текст у текстах, що не експортуються.
    [many] { $count } заповнювачів Scrivener, наприклад { $example }, збережено як простий текст у текстах, що не експортуються.
   *[other] { $count } заповнювача Scrivener, наприклад { $example }, збережено як простий текст у текстах, що не експортуються.
}
import-scrivener-prose-ornament-lines-kept-line = { $count ->
    [one] { $count } вирівняний по центру рядок із декоративних знаків, можливо, позначає розрив сцени, тож його збережено так, як написано.
    [few] { $count } вирівняні по центру рядки з декоративних знаків, можливо, позначають розриви сцен, тож їх збережено так, як написано.
    [many] { $count } вирівняних по центру рядків із декоративних знаків, можливо, позначають розриви сцен, тож їх збережено так, як написано.
   *[other] { $count } вирівняного по центру рядка з декоративних знаків, можливо, позначають розриви сцен, тож їх збережено так, як написано.
}
import-scrivener-prose-long-document-line = { $count ->
    [one] { $count } текст дуже довгий і може повільно відкриватися.
    [few] { $count } тексти дуже довгі й можуть повільно відкриватися.
    [many] { $count } текстів дуже довгі й можуть повільно відкриватися.
   *[other] { $count } тексту дуже довгі й можуть повільно відкриватися.
}
import-scrivener-prose-pictures-without-description-line = { $count ->
    [one] { $count } зображення не має опису для читачів, які не можуть його бачити. Опис можна додати в редакторі.
    [few] { $count } зображення не мають опису для читачів, які не можуть їх бачити. Опис можна додати в редакторі.
    [many] { $count } зображень не мають опису для читачів, які не можуть їх бачити. Опис можна додати в редакторі.
   *[other] { $count } зображення не мають опису для читачів, які не можуть їх бачити. Опис можна додати в редакторі.
}
import-scrivener-prose-struck-text-compiled-away-line = { $count ->
    [one] Перекреслений текст у { $count } тексті збережено. Компонування Scrivener вилучало його з книги, але експорт зі Skribisto його друкує.
    [few] Перекреслений текст у { $count } текстах збережено. Компонування Scrivener вилучало його з книги, але експорт зі Skribisto його друкує.
    [many] Перекреслений текст у { $count } текстах збережено. Компонування Scrivener вилучало його з книги, але експорт зі Skribisto його друкує.
   *[other] Перекреслений текст у { $count } тексту збережено. Компонування Scrivener вилучало його з книги, але експорт зі Skribisto його друкує.
}
import-scrivener-prose-compile-deleted-style-printed-line = { $count ->
    [1] Текст у стилі «{ $example }», який компонування Scrivener видаляло, збережено як прозу, і експорт зі Skribisto його друкує.
    [one] Текст у { $count } стилі, які компонування Scrivener видаляло (наприклад, «{ $example }»), збережено як прозу, і експорт зі Skribisto його друкує.
    [few] Текст у { $count } стилях, які компонування Scrivener видаляло (наприклад, «{ $example }»), збережено як прозу, і експорт зі Skribisto його друкує.
    [many] Текст у { $count } стилях, які компонування Scrivener видаляло (наприклад, «{ $example }»), збережено як прозу, і експорт зі Skribisto його друкує.
   *[other] Текст у { $count } стилю, які компонування Scrivener видаляло (наприклад, «{ $example }»), збережено як прозу, і експорт зі Skribisto його друкує.
}
import-scrivener-prose-compile-raw-style-printed-line = { $count ->
    [1] Текст у стилі «{ $example }», який компонування Scrivener передавало як код для іншої програми, збережено як прозу, і експорт зі Skribisto його друкує.
    [one] Текст у { $count } стилі, які компонування Scrivener передавало як код для іншої програми (наприклад, «{ $example }»), збережено як прозу, і експорт зі Skribisto його друкує.
    [few] Текст у { $count } стилях, які компонування Scrivener передавало як код для іншої програми (наприклад, «{ $example }»), збережено як прозу, і експорт зі Skribisto його друкує.
    [many] Текст у { $count } стилях, які компонування Scrivener передавало як код для іншої програми (наприклад, «{ $example }»), збережено як прозу, і експорт зі Skribisto його друкує.
   *[other] Текст у { $count } стилю, які компонування Scrivener передавало як код для іншої програми (наприклад, «{ $example }»), збережено як прозу, і експорт зі Skribisto його друкує.
}
import-scrivener-prose-left-out-after-document-line = { $count ->
    [one] { $count } фрагмент пошкодженого файлу після його кінця не перенесено.
    [few] { $count } фрагменти пошкоджених файлів після їхнього кінця не перенесено.
    [many] { $count } фрагментів пошкоджених файлів після їхнього кінця не перенесено.
   *[other] { $count } фрагмента пошкоджених файлів після їхнього кінця не перенесено.
}
import-scrivener-prose-left-out-header-or-footer-line = { $count ->
    [one] { $count } колонтитул не перенесено: він не є частиною тексту.
    [few] { $count } колонтитули не перенесено: вони не є частиною тексту.
    [many] { $count } колонтитулів не перенесено: вони не є частиною тексту.
   *[other] { $count } колонтитула не перенесено: вони не є частиною тексту.
}
import-scrivener-prose-left-out-list-label-picture-line = { $count ->
    [one] { $count } зображення, використане як маркер списку, не перенесено.
    [few] { $count } зображення, використані як маркери списку, не перенесено.
    [many] { $count } зображень, використаних як маркери списку, не перенесено.
   *[other] { $count } зображення, використаного як маркер списку, не перенесено.
}
import-scrivener-prose-left-out-line = { $count ->
    [one] { $count } фрагмент файлу, що не належить жодному тексту, не перенесено.
    [few] { $count } фрагменти файлів, що не належать жодному тексту, не перенесено.
    [many] { $count } фрагментів файлів, що не належать жодному тексту, не перенесено.
   *[other] { $count } фрагмента файлів, що не належать жодному тексту, не перенесено.
}
import-scrivener-prose-undecodable-characters-line = { $count ->
    [one] { $count } символ не вдалося декодувати, і на його місці показано знак заміни (�).
    [few] { $count } символи не вдалося декодувати, і на їхньому місці показано знаки заміни (�).
    [many] { $count } символів не вдалося декодувати, і на їхньому місці показано знаки заміни (�).
   *[other] { $count } символу не вдалося декодувати, і на їхньому місці показано знаки заміни (�).
}
import-scrivener-prose-inline-notes-merged-line = { $count ->
    [one] { $count } вбудовану примітку, яку Scrivener розділив там, де змінювалося її форматування, знову з’єднано.
    [few] { $count } вбудовані примітки, які Scrivener розділив там, де змінювалося їхнє форматування, знову з’єднано.
    [many] { $count } вбудованих приміток, які Scrivener розділив там, де змінювалося їхнє форматування, знову з’єднано.
   *[other] { $count } вбудованої примітки, які Scrivener розділив там, де змінювалося їхнє форматування, знову з’єднано.
}
import-scrivener-prose-inline-notes-joined-across-paragraphs-line = { $count ->
    [1] Вбудовану примітку, написану окремо в кожному абзаці, об’єднано в одну примітку.
    [one] { $count } вбудовану примітку, написану окремо в кожному абзаці, об’єднано в примітки з кількох абзаців.
    [few] { $count } вбудовані примітки, написані окремо в кожному абзаці, об’єднано в примітки з кількох абзаців.
    [many] { $count } вбудованих приміток, написаних окремо в кожному абзаці, об’єднано в примітки з кількох абзаців.
   *[other] { $count } вбудованої примітки, написаної окремо в кожному абзаці, об’єднано в примітки з кількох абзаців.
}
import-scrivener-prose-blank-notes-dropped-line = { $count ->
    [one] { $count } порожню вбудовану примітку не перенесено.
    [few] { $count } порожні вбудовані примітки не перенесено.
    [many] { $count } порожніх вбудованих приміток не перенесено.
   *[other] { $count } порожньої вбудованої примітки не перенесено.
}
import-scrivener-prose-annotations-became-comments-line = { $count ->
    [one] { $count } вбудована анотація стала коментарем.
    [few] { $count } вбудовані анотації стали коментарями.
    [many] { $count } вбудованих анотацій стали коментарями.
   *[other] { $count } вбудованої анотації стали коментарями.
}
import-scrivener-prose-annotation-only-document-kept-line = { $count ->
    [one] { $count } документ не містив нічого, крім анотації. Його слова збережено як прозу.
    [few] { $count } документи не містили нічого, крім анотації. Їхні слова збережено як прозу.
    [many] { $count } документів не містили нічого, крім анотації. Їхні слова збережено як прозу.
   *[other] { $count } документа не містили нічого, крім анотації. Їхні слова збережено як прозу.
}
import-scrivener-prose-preserve-formatting-passages-line = { $count ->
    [one] { $count } фрагмент, позначений «Зберегти форматування», надходить як звичайний текст.
    [few] { $count } фрагменти, позначені «Зберегти форматування», надходять як звичайний текст.
    [many] { $count } фрагментів, позначених «Зберегти форматування», надходять як звичайний текст.
   *[other] { $count } фрагмента, позначеного «Зберегти форматування», надходять як звичайний текст.
}
import-scrivener-prose-nested-note-unwrapped-line = { $count ->
    [one] { $count } примітку всередині коментаря чи виноски збережено як текст на її місці.
    [few] { $count } примітки всередині коментарів чи виносок збережено як текст на їхньому місці.
    [many] { $count } приміток усередині коментарів чи виносок збережено як текст на їхньому місці.
   *[other] { $count } примітки всередині коментарів чи виносок збережено як текст на їхньому місці.
}
import-scrivener-prose-style-not-carried-line = { $count ->
    [1] Стиль «{ $example }» не має відповідника в Skribisto. Його текст зберігає свої слова.
    [one] { $count } стиль, наприклад «{ $example }», не має відповідника в Skribisto. Текст у цих стилях зберігає свої слова.
    [few] { $count } стилі, наприклад «{ $example }», не мають відповідника в Skribisto. Текст у цих стилях зберігає свої слова.
    [many] { $count } стилів, наприклад «{ $example }», не мають відповідника в Skribisto. Текст у цих стилях зберігає свої слова.
   *[other] { $count } стилю, наприклад «{ $example }», не мають відповідника в Skribisto. Текст у цих стилях зберігає свої слова.
}
import-scrivener-prose-document-links-dropped-line = { $count ->
    [one] { $count } посилання на інший документ не має відповідника в Skribisto. Його слова лишаються.
    [few] { $count } посилання на інші документи не мають відповідника в Skribisto. Їхні слова лишаються.
    [many] { $count } посилань на інші документи не мають відповідника в Skribisto. Їхні слова лишаються.
   *[other] { $count } посилання на інші документи не мають відповідника в Skribisto. Їхні слова лишаються.
}
import-scrivener-prose-document-links-kept-line = { $count ->
    [one] { $count } посилання на інший документ збережено як зв’язок.
    [few] { $count } посилання на інші документи збережено як зв’язки.
    [many] { $count } посилань на інші документи збережено як зв’язки.
   *[other] { $count } посилання на інші документи збережено як зв’язки.
}
import-scrivener-prose-other-links-kept-as-text-line = { $count ->
    [one] { $count } посилання на інший проєкт чи файл збережено як текст.
    [few] { $count } посилання на інші проєкти чи файли збережено як текст.
    [many] { $count } посилань на інші проєкти чи файли збережено як текст.
   *[other] { $count } посилання на інші проєкти чи файли збережено як текст.
}
import-scrivener-prose-soft-breaks-split-line = { $count ->
    [one] { $count } розрив рядка всередині абзацу тепер починає окремий абзац.
    [few] { $count } розриви рядків усередині абзаців тепер починають окремі абзаци.
    [many] { $count } розривів рядків усередині абзаців тепер починають окремі абзаци.
   *[other] { $count } розриву рядка всередині абзаців тепер починають окремі абзаци.
}
import-scrivener-prose-typed-indents-removed-line = { $count ->
    [one] { $count } відступ, набраний на початку абзацу, вилучено.
    [few] { $count } відступи, набрані на початку абзаців, вилучено.
    [many] { $count } відступів, набраних на початку абзаців, вилучено.
   *[other] { $count } відступу, набраного на початку абзаців, вилучено.
}
import-scrivener-prose-left-indents-not-kept-line = { $count ->
    [one] { $count } абзац із відступом від поля втрачає свій відступ.
    [few] { $count } абзаци з відступом від поля втрачають свій відступ.
    [many] { $count } абзаців із відступом від поля втрачають свій відступ.
   *[other] { $count } абзацу з відступом від поля втрачають свій відступ.
}
import-scrivener-prose-blank-lines-removed-line = { $count ->
    [one] { $count } порожній абзац між двома іншими вилучено.
    [few] { $count } порожні абзаци між іншими вилучено.
    [many] { $count } порожніх абзаців між іншими вилучено.
   *[other] { $count } порожнього абзацу між іншими вилучено.
}
import-scrivener-prose-verse-stanza-gaps-lost-line = { $count ->
    [one] { $count } порожній рядок між строфами вірша вилучено, тож строфи йдуть одна за одною.
    [few] { $count } порожні рядки між строфами вірша вилучено, тож строфи йдуть одна за одною.
    [many] { $count } порожніх рядків між строфами вірша вилучено, тож строфи йдуть одна за одною.
   *[other] { $count } порожнього рядка між строфами вірша вилучено, тож строфи йдуть одна за одною.
}
import-scrivener-prose-blank-lines-became-scene-breaks-line = { $count ->
    [one] { $count } порожній рядок між абзацами став розривом сцени.
    [few] { $count } порожні рядки між абзацами стали розривами сцен.
    [many] { $count } порожніх рядків між абзацами стали розривами сцен.
   *[other] { $count } порожнього рядка між абзацами стали розривами сцен.
}
import-scrivener-prose-ornaments-became-scene-breaks-line = { $count ->
    [one] { $count } рядок із декоративних знаків став розривом сцени.
    [few] { $count } рядки з декоративних знаків стали розривами сцен.
    [many] { $count } рядків із декоративних знаків стали розривами сцен.
   *[other] { $count } рядка з декоративних знаків стали розривами сцен.
}
import-scrivener-prose-table-first-row-header-line = { $count ->
    [one] { $count } таблиця тепер починається з рядка заголовків.
    [few] { $count } таблиці тепер починаються з рядка заголовків.
    [many] { $count } таблиць тепер починаються з рядка заголовків.
   *[other] { $count } таблиці тепер починаються з рядка заголовків.
}
import-scrivener-prose-table-cells-joined-line = { $count ->
    [one] { $count } клітинка таблиці містила кілька абзаців, тепер їх об’єднано в один.
    [few] { $count } клітинки таблиць містили кілька абзаців, тепер їх об’єднано в один.
    [many] { $count } клітинок таблиць містили кілька абзаців, тепер їх об’єднано в один.
   *[other] { $count } клітинки таблиць містили кілька абзаців, тепер їх об’єднано в один.
}
import-scrivener-prose-formatting-lost-colour-line = { $count ->
    [one] Колір тексту вилучено в { $count } тексті.
    [few] Колір тексту вилучено в { $count } текстах.
    [many] Колір тексту вилучено в { $count } текстах.
   *[other] Колір тексту вилучено в { $count } тексту.
}
import-scrivener-prose-formatting-lost-highlight-line = { $count ->
    [one] Виділення кольором вилучено в { $count } тексті.
    [few] Виділення кольором вилучено в { $count } текстах.
    [many] Виділення кольором вилучено в { $count } текстах.
   *[other] Виділення кольором вилучено в { $count } тексту.
}
import-scrivener-prose-formatting-lost-small-caps-line = { $count ->
    [one] Капітель показано звичайними літерами в { $count } тексті.
    [few] Капітель показано звичайними літерами в { $count } текстах.
    [many] Капітель показано звичайними літерами в { $count } текстах.
   *[other] Капітель показано звичайними літерами в { $count } тексту.
}
import-scrivener-prose-formatting-lost-all-caps-line = { $count ->
    [one] Текст, що відображався великими літерами, тепер показано так, як його набрано, в { $count } тексті.
    [few] Текст, що відображався великими літерами, тепер показано так, як його набрано, в { $count } текстах.
    [many] Текст, що відображався великими літерами, тепер показано так, як його набрано, в { $count } текстах.
   *[other] Текст, що відображався великими літерами, тепер показано так, як його набрано, в { $count } тексту.
}
import-scrivener-prose-formatting-lost-hidden-line = { $count ->
    [one] Прихований текст не перенесено в { $count } тексті.
    [few] Прихований текст не перенесено в { $count } текстах.
    [many] Прихований текст не перенесено в { $count } текстах.
   *[other] Прихований текст не перенесено в { $count } тексту.
}
import-scrivener-prose-formatting-lost-line = { $count ->
    [one] Форматування, для якого в Skribisto немає місця, вилучено в { $count } тексті.
    [few] Форматування, для якого в Skribisto немає місця, вилучено в { $count } текстах.
    [many] Форматування, для якого в Skribisto немає місця, вилучено в { $count } текстах.
   *[other] Форматування, для якого в Skribisto немає місця, вилучено в { $count } тексту.
}
import-scrivener-prose-justified-text-not-kept-line = { $count ->
    [one] { $count } абзац, вирівняний за шириною, натомість дотримується стилю експорту.
    [few] { $count } абзаци, вирівняні за шириною, натомість дотримуються стилю експорту.
    [many] { $count } абзаців, вирівняних за шириною, натомість дотримуються стилю експорту.
   *[other] { $count } абзацу, вирівняного за шириною, натомість дотримуються стилю експорту.
}
import-scrivener-prose-comment-colours-lost-line = { $count ->
    [one] { $count } коментар утрачає свій колір.
    [few] { $count } коментарі втрачають свій колір.
    [many] { $count } коментарів утрачають свій колір.
   *[other] { $count } коментаря втрачають свій колір.
}
import-scrivener-prose-control-characters-removed-line = { $count ->
    [one] { $count } невидимий керівний символ вилучено.
    [few] { $count } невидимі керівні символи вилучено.
    [many] { $count } невидимих керівних символів вилучено.
   *[other] { $count } невидимого керівного символу вилучено.
}
import-scrivener-prose-note-on-heading-lost-line = { $count ->
    [one] { $count } виноска чи коментар на заголовку розділу не може там лишитися: коментар переходить до першого абзацу розділу, а виноску не перенесено.
    [few] { $count } виноски чи коментарі на заголовках розділів не можуть там лишитися: коментар переходить до першого абзацу розділу, а виноску не перенесено.
    [many] { $count } виносок чи коментарів на заголовках розділів не можуть там лишитися: коментар переходить до першого абзацу розділу, а виноску не перенесено.
   *[other] { $count } виноски чи коментаря на заголовках розділів не можуть там лишитися: коментар переходить до першого абзацу розділу, а виноску не перенесено.
}
import-scrivener-prose-media-file-copied-line = { $count ->
    [one] { $count } файл досліджень, який Skribisto не може відкрити, скопійовано поруч із новим проєктом. Його нотатка вказує, куди саме.
    [few] { $count } файли досліджень, які Skribisto не може відкрити, скопійовано поруч із новим проєктом. Їхні нотатки вказують, куди саме.
    [many] { $count } файлів досліджень, які Skribisto не може відкрити, скопійовано поруч із новим проєктом. Їхні нотатки вказують, куди саме.
   *[other] { $count } файлу досліджень, які Skribisto не може відкрити, скопійовано поруч із новим проєктом. Їхні нотатки вказують, куди саме.
}
import-scrivener-prose-media-file-not-carried-line = { $count ->
    [one] { $count } файл досліджень, який Skribisto не може відкрити, лишається у вашому проєкті Scrivener. Його нотатка вказує, який це файл.
    [few] { $count } файли досліджень, які Skribisto не може відкрити, лишаються у вашому проєкті Scrivener. Їхні нотатки вказують, які це файли.
    [many] { $count } файлів досліджень, які Skribisto не може відкрити, лишаються у вашому проєкті Scrivener. Їхні нотатки вказують, які це файли.
   *[other] { $count } файлу досліджень, які Skribisto не може відкрити, лишаються у вашому проєкті Scrivener. Їхні нотатки вказують, які це файли.
}

## When the import fails
import-scrivener-error-refused = { $refusal } Нічого не записано.
import-scrivener-error-question = Проєкт тепер ставить питання, на яке його перевірка не дала відповіді. Поверніться до перевірки, а потім імпортуйте. Нічого не записано.
import-scrivener-error-not-accepted = Проєкт ставить питання, на яке ви ще не дали згоди. Поверніться до перевірки, а потім імпортуйте. Нічого не записано.
import-scrivener-error-changed = Проєкт змінився після того, як ви його перевірили. Поверніться до перевірки, а потім імпортуйте. Нічого не записано.
import-scrivener-error-choice = Структура, вибрана під час перевірки, більше не відповідає цьому проєкту. Поверніться до перевірки, а потім імпортуйте. Нічого не записано.
export-title = Експорт
export-close = Закрити
export-scope-label = Що
export-format-label = Формат
export-style-label = Стиль
export-path-label = Файл
export-browse = Вибрати…
export-show-non-exportable = Показувати елементи, що не експортуються
export-choose-empty = Немає проєкту, з якого можна вибрати.
export-cancel = Скасувати
export-export = Експортувати
export-save-dialog-title = Експорт у файл
export-section-what = Що експортувати
export-section-style = Стиль
export-section-destination = Місце призначення
export-custom-selection = Власний вибір
export-selected-count = Вибрано: { $count }
export-chip-chapters-none = Розділи: без заголовка
export-chip-chapters-numbered = Розділи: номер
export-chip-chapters-title = Розділи: лише назва
export-chip-chapters-both = Розділи: номер і назва
export-chip-scene-break-glyph = Розрив сцени: { $glyph }
export-chip-scene-break-blank = Розрив сцени: порожній рядок
export-chip-scene-break-none = Розрив сцени: немає
export-chip-major-break-glyph = Головний розрив: { $glyph }
export-chip-major-break-blank = Головний розрив: порожній рядок
export-chip-major-break-none = Головний розрив: немає
export-chip-major-break-same = Обидва рівні розривів однакові
export-chip-spacing-single = Інтервал: одинарний
export-chip-spacing-onehalf = Інтервал: 1½
export-chip-spacing-double = Інтервал: подвійний
export-chip-notes-included = З нотатками
export-chip-notes-excluded = Без нотаток
export-format-docx = Word
export-format-odt = LibreOffice
export-format-html = HTML
export-format-markdown = Markdown
export-format-djot = Djot
export-format-text = Простий текст
export-format-latex = LaTeX
export-format-epub = EPUB
export-format-pdf = PDF
export-overwrite-title = Замінити наявний файл?
export-overwrite-text = «{ $name }» уже існує. Замінити його?
export-progress-title = Експорт…
export-cancelled = Експорт скасовано
export-done = Експортовано { $count ->
    [one] { $count } елемент
    [few] { $count } елементи
    [many] { $count } елементів
   *[other] { $count } елемента
}
export-open-file = Відкрити
export-show-in-folder = Показати в теці
export-error-title = Не вдалося експортувати
export-error-details = Подробиці
backup-progress-start = Початок…
backup-progress-retention = Очищення старих резервних копій…
backup-progress-done = Готово
backup-progress-destination = Місце призначення { $i } з { $n }
backup-details = Подробиці
backup-issues-title = Проблеми з резервним копіюванням
backup-failed-title = Не вдалося створити резервну копію
backup-complete-prune-warning = Резервне копіювання завершено (збережено: { $ok }, уже актуальних: { $skipped }). Деякі старі резервні копії не вдалося вилучити
search = Пошук
search-query-placeholder = Шукати…
search-replace-placeholder = Замінити на…
search-replace-toggle = Показати або сховати заміну
search-replace-all = Замінити все
search-opt-case = Враховувати регістр
search-opt-whole-word = Ціле слово
search-opt-diacritics = Враховувати діакритику
search-scope-body = Основний текст
search-scope-title = Назва
search-scope-synopsis = Синопсис
search-scope-label = Позначка
search-facet-book = Книги
search-facet-part = Частини
search-facet-chapter = Розділи
search-facet-scene = Сцени
search-facet-note = Нотатки
search-facet-folder = Теки
search-tip-case = Враховувати регістр: великі й малі літери вважаються різними, тож «Олена» та «олена» — окремі збіги.
search-tip-whole-word = Ціле слово: шукати лише цілі слова, тож «кіт» не знаходиться всередині «кітель».
search-tip-diacritics = Враховувати діакритику: літери з діакритичними знаками вважаються окремими, тож «cafe» не збігається з «café».
search-tip-body = Основний текст: шукати в прозі сцен і нотаток.
search-tip-title = Назва: шукати в назвах елементів підшивки.
search-tip-synopsis = Синопсис: шукати в стислому викладі кожного елемента з текстом.
search-tip-label = Позначка: шукати в короткому тексті, що показується під назвою елемента підшивки.
search-tip-comment = Коментарі: шукати в тексті обговорень і відповідей на них. Коментар стосується рукопису, а не є його частиною, тому має власний перемикач, а під час заміни збіги в коментарях лишаються невідміченими, доки ви їх не відмітите.
search-tip-book = Книги: контейнер книги та її маркери початку й кінця.
search-tip-part = Частини: роздільники рівня частини.
search-tip-chapter = Розділи: розділи, хоч би як їх зберігав проєкт.
search-tip-scene = Сцени: елементи, що містять вашу прозу.
search-tip-note = Нотатки: довільні нотатки.
search-tip-folder = Теки: звичайні теки для впорядкування та роздільники.
search-tip-paratext = Текст, що належить книзі, але не її історії: передмова, присвята, післямова. Ніколи не враховується в рукописі.
search-tip-preserve-case = Зберігати регістр: заміна набуває регістру знайденого, тож «ОЛЕНА» стає «МАРТА», а «Олена» — «Марта».
search-tip-replace = Заміна: показати поле заміни та кнопку «Замінити все».
search-error = Помилка пошуку: { $message }
search-no-matches = Збігів немає
search-count =
    { $matches ->
        [one] { $matches } збіг
        [few] { $matches } збіги
        [many] { $matches } збігів
       *[other] { $matches } збігу
    } у { $items ->
        [one] { $items } документі
        [few] { $items } документах
        [many] { $items } документах
       *[other] { $items } документа
    }
search-count-truncated =
    { $matches ->
        [one] { $matches } збіг
        [few] { $matches } збіги
        [many] { $matches } збігів
       *[other] { $matches } збігу
    } у { $items ->
        [one] { $items } документі
        [few] { $items } документах
        [many] { $items } документах
       *[other] { $items } документа
    } (лише перші результати)
search-occurrences = { $count }
search-field-body = Основний текст
search-field-title = Назва
search-field-synopsis = Синопсис
search-field-label = Позначка
search-field-epigraph = Епіграф
search-field-comment = Коментар
search-field-comment-reply = Відповідь на коментар
search-field-footnote = Виноска
search-include-in-replace = Включити в «Замінити все»
search-collapse-all = Згорнути всі результати
search-replace-here = Замінити тут
search-dismiss = Прибрати з результатів
search-undo-dismiss = Повернути останній прибраний результат
search-replace-nothing = (нічого)
search-replace-confirm-title = Замінити всі збіги?
search-replace-confirm-text =
    Замінити { $occurrences ->
        [one] { $occurrences } входження
        [few] { $occurrences } входження
        [many] { $occurrences } входжень
       *[other] { $occurrences } входження
    } «{ $query }» на «{ $replacement }» у { $items ->
        [one] { $items } документі
        [few] { $items } документах
        [many] { $items } документах
       *[other] { $items } документа
    }? Потім цю дію можна скасувати зі сповіщення.
search-replace-done-title = Заміну завершено
search-replace-done =
    Замінено { $occurrences ->
        [one] { $occurrences } входження
        [few] { $occurrences } входження
        [many] { $occurrences } входжень
       *[other] { $occurrences } входження
    } у { $items ->
        [one] { $items } документі
        [few] { $items } документах
        [many] { $items } документах
       *[other] { $items } документа
    }.
search-replace-done-skipped =
    Замінено { $occurrences ->
        [one] { $occurrences } входження
        [few] { $occurrences } входження
        [many] { $occurrences } входжень
       *[other] { $occurrences } входження
    } у { $items ->
        [one] { $items } документі
        [few] { $items } документах
        [many] { $items } документах
       *[other] { $items } документа
    }. Пропущено { $skipped ->
        [one] { $skipped } поле
        [few] { $skipped } поля
        [many] { $skipped } полів
       *[other] { $skipped } поля
    } (змінено після пошуку).
search-replace-undo = Скасувати
search-replace-failed-title = Не вдалося виконати заміну
search-replace-skipped-title = Нічого не замінено
search-replace-skipped-body =
    { $fields ->
        [1] Цей текст змінився після пошуку, тому його залишено без змін.
        [one] { $fields } з цих текстів змінився після пошуку, тому його залишено без змін.
        [few] { $fields } з цих текстів змінилися після пошуку, тому їх залишено без змін.
        [many] { $fields } з цих текстів змінилися після пошуку, тому їх залишено без змін.
       *[other] { $fields } з цих текстів змінилися після пошуку, тому їх залишено без змін.
    } Пошукайте ще раз, щоб побачити, де тепер ці слова.
search-replace-undo-failed-title = Не вдалося скасувати
search-preview = Попередній перегляд
search-preview-empty = Виберіть результат, щоб переглянути його тут
search-preview-no-prose = У цьому збігу немає тексту, який можна редагувати
search-preview-prompt = Щоб побачити тут попередній перегляд, виконайте пошук.
search-preview-open-search = Пошук у проєкті
search-preview-footnote-prompt = Цей збіг міститься в тексті виноски. Відкрийте його на панелі «Виноски», щоб переглянути й змінити.
search-preview-open-footnotes = Відкрити виноски
find-placeholder = Знайти в цьому документі…
find-count = { $current } з { $total }
find-no-results = Нічого не знайдено
find-close = Закрити пошук
find-previous = Попередній збіг (Shift+Enter)
find-next = Наступний збіг (Enter)
find-opt-case = Враховувати регістр
find-opt-whole-word = Ціле слово
find-replace-toggle = Показати або сховати заміну (Ctrl+R)
find-replace-placeholder = Замінити на…
find-replace = Замінити
find-replace-all = Замінити все
find-preserve-case = Зберігати регістр
dict-download-title = Завантаження { $name }…
dict-download-done = Встановлено { $name }
dict-download-failed = Не вдалося завантажити { $name }: { $error }
dict-removed = Вилучено { $name }
dict-accept-first = Прийміть ліцензію, перш ніж завантажувати { $name }
settings-dict-tab-installed = Встановлені
settings-dict-tab-get-more = Отримати більше
settings-dict-tab-personal = Особисті слова
dict-installed-empty = На цьому комп’ютері ще не знайдено словників.
dict-get-more-search = Шукати мови
dict-system-badge = у вашій системі
dict-unusable-badge = непридатний
dict-download-button = Завантажити
dict-downloading = Завантаження…
dict-installed-label = Встановлено
dict-remove = Вилучити
dict-view-license = Переглянути ліцензію
dict-approx-size = ~{ $size }
dict-personal-empty = У цьому проєкті ще немає особистих слів.
dict-personal-add = Додати
dict-personal-placeholder = Додайте слово…
dict-missing-toast = { $count ->
    [one] Цей проєкт використовує { $count } словник, який у вас не встановлено
    [few] Цей проєкт використовує { $count } словники, які у вас не встановлено
    [many] Цей проєкт використовує { $count } словників, які у вас не встановлено
   *[other] Цей проєкт використовує { $count } словника, які у вас не встановлено
}
dict-missing-action = Отримати словники
dict-license-title = Ліцензія { $name }
dict-license-accept = Прийняти й завантажити
dict-license-cancel = Скасувати
dict-license-close = Закрити
dict-add-button = Додати словник…
dict-add-title = Додавання словника
dict-add-close = Закрити
dict-add-name = Назва
dict-add-name-placeholder = напр. Мій латинський словник
dict-add-code = Код мови
dict-add-code-placeholder = напр. la або fr-FR-x-custom
dict-add-code-hint = Короткий тег на ваш вибір. Саме його ви вибиратимете як мову документа.
dict-add-aff = Файл афіксів (.aff)
dict-add-dic = Список слів (.dic)
dict-add-submit = Додати
dict-add-cancel = Скасувати
dict-add-name-required = Дайте словнику назву.
dict-add-code-required = Введіть код мови.
dict-add-code-invalid = Використовуйте лише латинські літери, цифри та - _ .
dict-add-code-reserved = Це код вбудованого словника. Виберіть інший.
dict-add-file-required = Виберіть файл.
dict-add-file-missing = Такого файлу не існує.
dict-add-code-taken = Словник для цього коду вже встановлено.
dict-add-done = Додано { $name }
dict-add-unusable = Ці файли не є придатним словником: { $error }
dict-add-failed = Не вдалося додати словник: { $error }
inspector-export = Експорт
inspector-exportable = Включати в експорт
ctx-number = &Нумерувати цей розділ
ctx-unnumber = &Не нумерувати цей розділ
inspector-numbering = Нумерація
inspector-numbered = Нумерований
inspector-numbered-tip = Цей розділ займає своє місце в нумерації книги. Вимкніть це для прологу, епілогу чи інтерлюдії.
inspector-numbered-tip-more = Ненумерований розділ лишається в книзі таким, яким був: його заголовок, проза і кількість слів не змінюються. Він просто не друкує номера й не забирає його: розділ після прологу буде першим, а не другим. Виключити розділ з експорту — це інший перемикач, вище, і він вилучає розділ із книги повністю.
inspector-apply-to-children = Застосувати до дочірніх
inspector-milestone = Дата віхи
inspector-milestone-none = Без дати
inspector-milestone-clear = Очистити
inspector-tags = Мітки
inspector-aliases = Також відомий як
inspector-dict-language = Мова
inspector-apply-language-to-children = Застосувати мову до дочірніх
settings-page-language = Мова
settings-field-dict-language = Мови
dict-tradeoff-hint = Кожна додаткова мова приймає більше слів, тож виявляється менше помилок.
lang-inherit-hint = Успадковано: ця сцена використовує мови книги або проєкту.
lang-pill-list = Мови
lang-pill-add = Додати мову
lang-pill-remove = Вилучити { $name }
lang-pill-mute = Вимкнути перевірку правопису для { $name }
lang-pill-unmute = Увімкнути перевірку правопису для { $name }
settings-page-personal-dictionary = Особистий словник
settings-user-dict-desc = Слова, які ви тут додасте, вважаються написаними правильно всюди в програмі й більше ніколи не позначаються.
settings-user-dict-add-placeholder = Додайте слово…
settings-user-dict-add = Додати слово
settings-user-dict-search = Фільтрувати слова
settings-user-dict-count = { $count ->
    [one] { $count } слово
    [few] { $count } слова
    [many] { $count } слів
   *[other] { $count } слова
}
settings-user-dict-rename = Перейменувати
settings-user-dict-remove = Вилучити
settings-user-dict-empty = Слів ще немає. Додайте слова, які перевірка правопису має пропускати, або імпортуйте список.
settings-user-dict-import = Імпортувати…
settings-user-dict-export = Експортувати…
settings-user-dict-import-tip = Додати слова з простого текстового файлу, по одному слову в рядку. Наявні слова зберігаються.
settings-user-dict-export-tip = Зберегти весь список у простий текстовий файл, по одному слову в рядку.
settings-user-dict-txt-filter = Текстові файли
settings-user-dict-duplicate = Уже є в словнику
settings-user-dict-imported = Імпортовано { $count ->
    [one] { $count } слово
    [few] { $count } слова
    [many] { $count } слів
   *[other] { $count } слова
} (уже наявних: { $duplicates }).
settings-user-dict-import-failed = Не вдалося прочитати список слів: { $error }
settings-user-dict-exported = Збережено { $count ->
    [one] { $count } слово.
    [few] { $count } слова.
    [many] { $count } слів.
   *[other] { $count } слова.
}
settings-user-dict-export-failed = Не вдалося зберегти список слів: { $error }
settings-page-text-replacements = Заміна тексту
settings-text-repl-desc = Замінює скорочення повним текстом під час друку: «дч» стає «до речі», щойно ви введете пробіл чи розділовий знак.
settings-text-repl-enable = Використовувати заміну тексту в цьому проєкті
settings-text-repl-disabled-hint = Увімкніть це, щоб задати скорочення, які розгортаються під час письма.
settings-text-repl-add = Додати правило
settings-text-repl-trigger-placeholder = Скорочення
settings-text-repl-replacement-placeholder = На що замінити
settings-text-repl-added = Додано «{ $trigger }»
settings-text-repl-duplicate = Для «{ $trigger }» уже є правило
settings-text-repl-filter = Фільтрувати правила
settings-text-repl-count = { $n ->
    [one] { $n } правило
    [few] { $n } правила
    [many] { $n } правил
   *[other] { $n } правила
}
settings-text-repl-row-enabled = Використовувати це правило
settings-text-repl-delete = Видалити правило для { $trigger }
settings-text-repl-deleted = Видалено правило для «{ $trigger }»
settings-text-repl-empty = Правил ще немає.
settings-text-repl-csv-filter = Файли CSV
settings-text-repl-import = Імпортувати…
settings-text-repl-export = Експортувати…
settings-text-repl-imported = Імпортовано: { $added }, пропущено: { $skipped }
settings-text-repl-exported = Експортовано { $n ->
    [one] { $n } правило
    [few] { $n } правила
    [many] { $n } правил
   *[other] { $n } правила
}
editor-menu-no-suggestions = Немає пропозицій
editor-menu-add-to-dictionary = Додати «{ $word }» до словника
editor-menu-add-words-to-dictionary = Додати виділені слова до словника
editor-dict-added = «{ $word }» додано до вашого словника.
editor-dict-added-multi = { $count ->
    [one] До вашого словника додано { $count } слово.
    [few] До вашого словника додано { $count } слова.
    [many] До вашого словника додано { $count } слів.
   *[other] До вашого словника додано { $count } слова.
}
toast-undo = Скасувати
undo-superseded-title = Цей крок уже не останній
undo-superseded-body = Відтоді в проєкті змінилося дещо інше. Скасування відмінило б саме те, тому його не виконано. Нічого з того, що зробив цей крок, не скасовано. Скористайтеся командою «Зміни ▸ Скасувати», щоб самостійно пройти історією назад.
undo-failed = Не вдалося скасувати: { $error }
menu-edit = &Зміни
menu-edit-undo = &Скасувати
menu-edit-redo = &Повторити
menu-edit-undo-target = &Скасувати { $target }
menu-edit-redo-target = &Повторити { $target }
shortcut-name-edit-undo = Скасувати
shortcut-name-edit-redo = Повторити
undo-target-typing = введення тексту
undo-target-project = останню зміну в цьому проєкті
prose-history-reset-title = Історію введення скинуто
prose-history-reset-body = { $count ->
    [1] Одну відкриту сцену відновлено з попереднього стану, тому її історія введення більше не застосовується.
    [one] { $count } відкриту сцену відновлено з попереднього стану, тому її історія введення більше не застосовується.
    [few] { $count } відкриті сцени відновлено з попереднього стану, тому їхня історія введення більше не застосовується.
    [many] { $count } відкритих сцен відновлено з попереднього стану, тому їхня історія введення більше не застосовується.
   *[other] { $count } відкритої сцени відновлено з попереднього стану, тому їхня історія введення більше не застосовується.
}
undo-target-trash = переміщення в кошик
undo-target-restore = відновлення з кошика
undo-target-delete-forever = остаточне видалення
undo-target-replace-all = заміну в усьому проєкті
undo-target-import = імпорт документа
undo-target-duplicate = дублювання
undo-target-move = переміщення
undo-target-merge = об’єднання двох сцен
undo-target-split = розділення сцени
undo-target-promote = зміну типу
undo-target-tidy-titles = упорядкування назв розділів
undo-target-import-tags = імпорт міток
undo-target-import-templates = імпорт шаблонів нотаток
undo-target-create = створення
undo-target-remove = видалення
undo-target-rename = перейменування
undo-target-edit = цю зміну
menu-edit-find = &Знайти…
menu-edit-find-next = Знайти &наступне
menu-edit-find-prev = Знайти попе&реднє
menu-edit-replace = Знайти й заміни&ти…
menu-edit-find-in-project = Знайти в про&єкті…
menu-edit-replace-in-project = Замінити &у проєкті…
undo-frozen = Скасувати (увімкнено «Завжди вперед»)
redo-frozen = Повторити (увімкнено «Завжди вперед»)
menu-edit-cut = &Вирізати
menu-edit-copy = &Копіювати
menu-edit-paste = Вст&авити
menu-edit-paste-plain = Вставити &без форматування
menu-edit-select-all = Ви&ділити все
shortcut-name-edit-cut = Вирізати
shortcut-name-edit-copy = Копіювати
shortcut-name-edit-paste = Вставити
shortcut-name-edit-paste-plain = Вставити без форматування
shortcut-name-edit-select-all = Виділити все
titlebar-spellcheck-on = Перевірку правопису ввімкнено. Клацніть, щоб припинити перевірку (F7)
titlebar-spellcheck-off = Перевірку правопису вимкнено. Клацніть, щоб знову перевіряти (F7)
menu-spellcheck = &Перевіряти правопис
menu-comments = &Коментарі
settings-page-spellcheck = Перевірка правопису
settings-group-spellcheck = Перевірка правопису
settings-spellcheck-enabled = Перевіряти правопис під час письма
settings-spellcheck-hint = Підкреслює слова, яких не знає жоден установлений словник. Якщо вимкнути, перевірка припиняється повністю, у кожному проєкті, доки ви не ввімкнете її знову. Щоб припинити перевірку лише однієї мови, зніміть прапорець цієї мови в полі «Мова» проєкту або елемента.
menu-trash = &Кошик
trash-title = Кошик
trash-empty-state = Кошик порожній.
trash-empty-button = Очистити кошик…
trash-restore = &Відновити
trash-restore-to = Відновити &до…
trash-delete-forever = Видалити &назавжди
trash-restored-ok = Відновлено: { $count }.
trash-restore-error = Не вдалося відновити: { $error }
trash-restore-orphaned = Початкового місця цього елемента більше немає. Виберіть, куди його відновити.
trash-restore-no-project = Жоден проєкт не відкрито, тож відновлювати нічого.
trash-delete-no-project = Жоден проєкт не відкрито, тож видаляти нічого.
trash-empty-confirm-title = Очистити кошик?
trash-empty-confirm-text = { $count ->
    [one] { $count } запис у кошику буде остаточно видалено.
    [few] { $count } записи в кошику буде остаточно видалено.
    [many] { $count } записів у кошику буде остаточно видалено.
   *[other] { $count } запису в кошику буде остаточно видалено.
} Вони назавжди покинуть кошик. Їх ще можна повернути скасуванням, доки ви не зробите щось інше.
trash-emptied-title = Кошик очищено
trash-emptied-body = Усе, що було в кошику, остаточно видалено.
trash-empty-no-project = Жоден проєкт не відкрито, тож немає кошика, який можна очистити.
trash-delete-forever-confirm-title = Видалити назавжди?
trash-delete-forever-confirm-text = { $count ->
    [one] { $count } елемент буде остаточно видалено.
    [few] { $count } елементи буде остаточно видалено.
    [many] { $count } елементів буде остаточно видалено.
   *[other] { $count } елемента буде остаточно видалено.
} Вони назавжди покинуть кошик. Їх ще можна повернути скасуванням, доки ви не зробите щось інше.
trash-deleted-title = Видалено назавжди
trash-deleted-body = Остаточно видалено: { $count }.
trash-undo = Скасувати
trash-restore-picker-title = Відновити до…
trash-restore-to-confirm-title = Відновити сюди?
trash-restore-to-confirm-text = Відновити «{ $item }» у «{ $destination }»?
trash-restore-picker-restore-here = Відновити сюди
trash-restore-picker-cancel = Скасувати
trash-restore-picker-empty = Підшивок ще немає. Спершу створіть одну.
trash-banner-title = Цей елемент у кошику
trash-banner-description = Він не з’явиться в плані та експорті, доки ви його не відновите.
trash-banner-restore = Відновити…
trash-tab-tooltip = У кошику
settings-styles-sheet-yes = Так
settings-styles-sheet-no = Ні
settings-styles-sheet-font = Шрифт
settings-styles-sheet-indent = Абзацний відступ
settings-styles-sheet-para-spacing = Інтервали між абзацами
settings-styles-sheet-page = Розмір сторінки
settings-styles-sheet-margins = Поля (В/П/Н/Л)
settings-styles-sheet-title-page = Титульна сторінка книги
settings-styles-sheet-heading-language = Мова заголовків
settings-styles-sheet-auto = Як у тексті
settings-styles-sheet-digits = Цифри
settings-styles-sheet-direction = Напрямок тексту
settings-styles-sheet-formats = Формати
settings-styles-sheet-all-formats = Усі
settings-styles-editor-group = Редактор стилю
settings-styles-editor-missing = Цей стиль більше недоступний.
settings-styles-page-letter = Letter
settings-styles-digits-western = Західні (0–9)
settings-styles-digits-eastern-arabic = Східноарабські (٠–٩)
settings-styles-direction-ltr = Зліва направо
settings-styles-direction-rtl = Справа наліво
format-dock-title = Формат
format-panel-empty = Поставте курсор у сцену, нотатку чи синопсис, щоб побачити параметри форматування.
format-group-marks = Накреслення
format-group-block = Абзац
format-group-lists = Списки
format-group-tables = Таблиця
format-group-breaks = Розриви сцен
format-superscript = Верхній індекс
format-subscript = Нижній індекс
format-link = Посилання…
link-dialog-insert-title = Вставлення посилання
link-dialog-edit-title = Зміна посилання
link-dialog-text-label = Текст посилання
link-dialog-text-placeholder = слова, які бачить читач
link-dialog-url-label = Веде на
link-dialog-url-placeholder = example.com
link-dialog-insert = Вставити
link-dialog-apply = Застосувати
link-dialog-cancel = Скасувати
link-dialog-remove = Вилучити посилання
link-scheme-refused = Відкривати можна лише вебпосилання та посилання на електронну пошту. { $url } залишено без змін.
menu-format-link = Посил&ання…
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
format-direction-rtl = Абзац справа наліво
format-blockquote = Цитата
format-list-bullet = Маркований список
format-list-numbered = Нумерований список
format-indent = Збільшити відступ
format-outdent = Зменшити відступ
format-table-insert = Вставити таблицю
format-table-row-above = Вставити рядок вище
format-table-row-below = Вставити рядок нижче
format-table-col-before = Вставити стовпець ліворуч
format-table-col-after = Вставити стовпець праворуч
format-table-row-delete = Видалити рядок
format-table-col-delete = Видалити стовпець
format-table-remove = Видалити таблицю
menu-format-marks-bold = &Жирний
menu-format-marks-italic = &Курсив
menu-format-marks-underline = &Підкреслений
menu-format-marks-strike = П&ерекреслений
menu-format-marks-superscript = Верхній &індекс
menu-format-marks-subscript = &Нижній індекс
menu-format-marks-clear = &Очистити форматування
menu-format-heading = &Заголовок
menu-format-heading-normal = &Звичайний текст
menu-format-heading-1 = Заголовок &1
menu-format-heading-2 = Заголовок &2
menu-format-heading-3 = Заголовок &3
menu-format-heading-4 = Заголовок &4
menu-format-heading-5 = Заголовок &5
menu-format-heading-6 = Заголовок &6
menu-format-alignment = Ви&рівнювання
menu-format-align-left = &Ліворуч
menu-format-align-center = По &центру
menu-format-direction = Напр&ямок
menu-format-direction-auto = &Автоматично
menu-format-direction-ltr = &Зліва направо
menu-format-direction-rtl = &Справа наліво
menu-format-blockquote = &Цитата
menu-format-lists = Сп&иски
menu-format-list-bullet = &Маркований список
menu-format-list-numbered = &Нумерований список
menu-format-indent = &Збільшити відступ
menu-format-outdent = Зм&еншити відступ
menu-format-table = &Таблиця
menu-format-table-insert = &Вставити таблицю
menu-format-table-2x2 = &2 × 2
menu-format-table-3x3 = &3 × 3
menu-format-table-4x4 = &4 × 4
menu-format-table-row-above = Вставити рядок в&ище
menu-format-table-row-below = Вставити рядок &нижче
menu-format-table-col-before = Вставити стовпець &ліворуч
menu-format-table-col-after = Вставити стовпець п&раворуч
menu-format-table-row-delete = Видалити р&ядок
menu-format-table-col-delete = Видалити &стовпець
menu-format-table-remove = Ви&далити таблицю
menu-format-typo-marks = Недр&укований символ
menu-format-typo-nbsp = &Нерозривний пробіл
menu-format-typo-nbhyphen = Нерозривний &дефіс
menu-format-typo-soft-hyphen = &М’який перенос
menu-format-typo-nnbsp = &Вузький нерозривний пробіл
menu-format-typo-zwsp = Пробіл нульової &ширини
menu-format-typo-word-joiner = &З’єднувач слів
menu-about = &Про Skribisto…
about-title = Про Skribisto
about-version = Версія { $version }
about-update-available = Доступна версія { $version }
about-update-see-notes = Що змінилося
about-update-flathub = Цю копію оновлює Flathub.
about-update-distro = Цю копію оновлює ваш дистрибутив.
about-tagline = Програма для письма романів і великої прози, написана на Rust з інструментарієм Teksilo.
about-license = Випущено на умовах GNU General Public License, версія 3.
about-copyright = © 2026 Cyril Jacquet
about-close = Закрити
native-menu-about = Про { $app }
native-menu-hide = Сховати { $app }
native-menu-quit = Вийти з { $app }
native-menu-settings = Налаштування…
native-menu-window = Вікно
native-menu-minimize = Згорнути
native-menu-zoom = Масштабувати
window-title = { $title } - { $app }
window-title-numbered = { $title } - { $app } (вікно { $n })
window-title-empty = { $app }
first-run-window-title = Вітаємо
first-run-title = Налаштування { $app }
first-run-body = { $app } зберігає свої налаштування окремо від Skribisto, тому спочатку вони порожні. Ваші вподобання, нещодавні проєкти, словники та розташування вікон можна скопіювати зараз.
first-run-from = Копіювати з
first-run-to = Копіювати до
first-run-copy-note = Нічого не переміщується і не вилучається. Skribisto зберігає всі свої налаштування й працює так само, як і раніше.
first-run-import = Імпортувати налаштування
first-run-start-fresh = Почати з нуля
first-run-import-failed = Деякі налаштування не вдалося імпортувати: { $error }
settings-page-punctuation = Типова пунктуація
settings-group-punctuation = Розумна пунктуація
settings-page-work-punctuation = Пунктуація
settings-punctuation-override = Задати для цього проєкту власні правила пунктуації
settings-punctuation-override-hint = Вимкнено: проєкт дотримується налаштування програми. Ці правила зберігаються всередині файлу .skrib, тож співавтор, який відкриє файл, писатиме з тією самою типографікою.
settings-punctuation-dashes = Перетворювати -- на коротке тире, --- на довге тире
settings-punctuation-ellipsis = Перетворювати ... на три крапки
settings-punctuation-quotes = Робити лапки й апострофи типографськими
settings-quote-style = Лапки
settings-quote-style-locale = Типові для мови
settings-quote-style-curly = “Подвійні”
settings-quote-style-curly-single = ‘Одинарні’
settings-quote-style-guillemets = «Ялинки»
settings-quote-style-low-high = „Нижні-верхні“
settings-punctuation-spacing = Пробіл перед ; : ! ?
settings-punctuation-spacing-hint = Французька типографіка ставить вузький нерозривний пробіл перед ; ! ? і звичайний перед :. Застосовується лише до тексту, написаного французькою. Пробіл усередині лапок-ялинок « » додається разом із самими лапками.
settings-punctuation-sample = Ваша мова дає
settings-punctuation-app-hint = Що робить кожен проєкт, якщо він не має власних правил у «Проєкт ▸ Пунктуація».
settings-punctuation-dialogue = Починати абзац, набраний як «- », з тире діалогу
settings-punctuation-dialogue-hint = Для мов, що позначають пряму мову тире, а не лапками: української, французької, іспанської, російської та інших. Спрацьовує лише на самому початку абзацу.
statusbar-go-to = Перейти до…
go-to-placeholder = Шукати в підшивці
go-to-no-matches = Жоден елемент не відповідає цьому пошуку.
menu-go-to = Перейти &до…
settings-page-distraction-free-themes = Теми режиму без відволікань
settings-themes-builtin = Вбудовані теми
settings-themes-builtin-badge = Вбудована
settings-themes-user = Мої теми
settings-themes-editor-group = Зміна теми
settings-themes-editor-empty = Виберіть тему в розділі «Мої теми», щоб змінити її.
settings-themes-use = Використати
settings-themes-duplicate = Дублювати
settings-themes-edit = Змінити
settings-themes-delete = Видалити
settings-themes-export = Експортувати…
settings-themes-import = Імпортувати…
settings-themes-copy-suffix = копія
settings-themes-json-filter = Тема (JSON)
settings-themes-imported = Тему імпортовано
settings-themes-import-failed = Не вдалося імпортувати цю тему
settings-themes-exported = Тему експортовано
settings-themes-export-failed = Не вдалося експортувати цю тему
settings-themes-low-contrast = низька контрастність
settings-themes-low-contrast-band = підсвічування приховує текст
settings-themes-field-name = Назва
settings-themes-field-paper = Сторінка
settings-themes-field-ink = Колір тексту
settings-themes-field-general = Тло
settings-themes-field-widget-text = Текст панелі керування
settings-themes-field-caret-band = Підсвічування навколо курсора
statusbar-focus-settings = Налаштування режиму без відволікань
statusbar-focus-manage-themes = Керувати темами…
comments-title = Коментарі
comments-document-title = Цей документ
comments-empty-project = У цьому проєкті ще немає коментарів.
comments-empty-document = До цього документа немає коментарів.
comments-filter-all = Усі
comments-filter-open = Відкриті
comments-filter-resolved = Розв’язані
comments-filter-orphaned = Без прив’язки
comments-status-open = Відкритий
comments-status-resolved = Розв’язаний
comments-status-orphaned = Текст утрачено
comments-orphan-snippet = (прокоментованого тексту більше немає)
comments-status-unplaced = Без позиції в тексті
comments-unplaced-snippet = (не прив’язаний до тексту)
comments-reply-count = { $count ->
    [one] { $count } відповідь
    [few] { $count } відповіді
    [many] { $count } відповідей
   *[other] { $count } відповіді
}
comments-menu-resolve = Позначити розв’язаним
comments-menu-reopen = Відкрити знову
comments-menu-delete = Видалити
comments-sort-document = У порядку документа
comments-sort-newest = Спершу найновіші
comments-sort-reader = За читачами
comments-unsigned-author = Без підпису
comments-agreement = { $count ->
    [one] { $count } читач
    [few] { $count } читачі
    [many] { $count } читачів
   *[other] { $count } читача
}
comments-menu-add = Додати коментар
comments-menu-add-paragraph = Прокоментувати цей абзац
overview-col-comments = Коментарі
overview-col-total-comments = Усього коментарів
comments-card-placeholder = Напишіть коментар…
comments-card-unknown-author = Невідомий автор
comments-unsigned-toast = Ваші коментарі не підписано: на цьому комп’ютері для вас не задано імені.
comments-unsigned-action = Задати ім’я
comments-card-reply = Відповісти
comments-card-reply-placeholder = Ваша відповідь…
comments-card-actions = Дії з коментарем
comments-reply-actions = Дії з відповіддю
comments-menu-delete-reply = Видалити відповідь
comments-menu-delete-all = Видалити всі коментарі тут
comments-deleted-toast = Коментар видалено
comments-reply-deleted-toast = Відповідь видалено
comments-deleted-all-toast = { $count ->
    [one] Видалено { $count } коментар
    [few] Видалено { $count } коментарі
    [many] Видалено { $count } коментарів
   *[other] Видалено { $count } коментаря
}
comments-undo = Скасувати
analysis-segment = Аналіз
analysis-scope-book = Аналіз цієї книги
analysis-run = Запустити аналіз
analysis-stale = Змінено після останнього аналізу
analysis-not-run = Ще не проаналізовано.
analysis-running = Читання рукопису…
analysis-failed = Аналіз не вдалося завершити.
analysis-no-scenes = У цій книзі ще немає сцен.
analysis-shape = Форма
analysis-arrivals = Надходження
analysis-arrivals-explainer = Як текст потрапляв у цей проєкт, поки він був відкритий. Тут видно, яким шляхом надійшли символи, і нічого не сказано про те, хто їх написав: письменник, який пише чернетку деінде і вставляє її, вставив, а той, хто диктує, продиктував. Жодної цифри, до якої слід прагнути, тут немає.
analysis-arrivals-scope = Увесь проєкт, а не лише ця книга.
analysis-arrivals-session = Відтоді, як ви його відкрили. Після закриття проєкту підрахунок починається знову.
analysis-arrivals-nothing = У цьому сеансі текст ще не надходив.
analysis-arrivals-typed = Набрано
analysis-arrivals-pasted = Вставлено
analysis-arrivals-dictated = Продиктовано
analysis-arrivals-imported = Імпортовано
analysis-arrivals-programmatic = Вставлено програмою
analysis-arrivals-count = { $count ->
    [one] { $count } символ
    [few] { $count } символи
    [many] { $count } символів
   *[other] { $count } символу
}
analysis-arrivals-none = немає
analysis-words-per-scene = Слів у сцені
analysis-median-words = Медіанна сцена цієї книги має { $count ->
    [one] { $count } слово.
    [few] { $count } слова.
    [many] { $count } слів.
   *[other] { $count } слова.
}
analysis-median-line = Медіана: { $count ->
    [one] { $count } слово
    [few] { $count } слова
    [many] { $count } слів
   *[other] { $count } слова
}
analysis-dialogue = Діалоги
analysis-dialogue-unsupported = Для цієї мови діалоги поки що не вимірюються.
analysis-footnote-words = Слова у виносках
analysis-footnote-words-count = { $count ->
    [one] У виносках цієї книги { $count } слово; воно не входить до загальної кількості слів рукопису.
    [few] У виносках цієї книги { $count } слова; вони не входять до загальної кількості слів рукопису.
    [many] У виносках цієї книги { $count } слів; вони не входять до загальної кількості слів рукопису.
   *[other] У виносках цієї книги { $count } слова; вони не входять до загальної кількості слів рукопису.
}
analysis-footnote-words-pending = Підрахунок виносок…
analysis-ignore-empty = Пропускати тексти, у яких ще нічого не написано
analysis-empty-hidden = { $count ->
        [one] Приховано { $count } порожній текст.
        [few] Приховано { $count } порожні тексти.
        [many] Приховано { $count } порожніх текстів.
       *[other] Приховано { $count } порожнього тексту.
    }
analysis-all-texts-empty = Усі тексти цієї книги ще порожні.
binder-filter-count = Показано { $shown } з { $total }
binder-filter-clear = Очистити
binder-filter-none = Ніщо не відповідає «{ $query }».
settings-paratext-intro = Початкові й кінцеві сторінки, з якими може починатися новий проєкт. Кожна структура належить певній видавничій традиції, і назви її сторінок написано мовою цієї традиції. Після створення проєкту їх можна вільно перейменувати.
settings-paratext-structures = Структури
settings-paratext-broken = Не вдалося прочитати
settings-paratext-edit = Змінити
settings-paratext-duplicate = Дублювати
settings-paratext-delete = Видалити
settings-paratext-new = Нова структура
settings-paratext-save = Зберегти
settings-paratext-editor-hint = Назва, сторінки, які створюються перед рукописом, і сторінки, які створюються після нього. Де вони опиняться, вирішуєте ви: щойно проєкт створено, це звичайні елементи.
image-insert = Вставити зобра&ження…
image-no-project = Відкрийте проєкт, перш ніж вставляти зображення.
image-choose-title = Виберіть зображення
image-filter-label = Файли зображень
image-large-title = Це велике зображення
image-large-text =
    { $name } має { $megapixels } мегапікселів ({ $width }×{ $height }).
    Якщо залишити його як є, у проєкті зберігається ваш оригінальний файл: він потрапляє
    в кожну резервну копію і кожен експорт. Оптимізація натомість зберігає меншу копію,
    до 2560 пікселів за довшою стороною.
image-large-keep = Зберегти оригінал
image-large-downscale = Оптимізувати
image-large-remember = Робити так і надалі, більше не питати
image-not-recorded = Зображення збережено, але не вдалося записати його в проєкт.
image-describe-title = Опис зображення
image-describe-explain = Що показано на зображенні, для читача, який не може його побачити. Опис не є частиною рукопису: він ніколи не враховується, не шукається й не експортується як проза.
image-describe-placeholder = маяк на тлі сірого неба
image-resize-title = Зміна розміру зображення
image-resize-explain = Відсоток від розміру, у якому воно показане зараз. 100 залишає його без змін.
image-resize-invalid = Введіть число від 1 до 1000.
image-menu-describe = &Описати зображення…
image-menu-resize = &Змінити розмір зображення…
image-menu-reset-size = &Початковий розмір
cover-choose = Обкладинка книг&и…
cover-clear = &Прибрати обкладинку
cover-choose-title = Виберіть обкладинку
cover-set = Обкладинку задано.
cover-cleared = Обкладинку прибрано. Зображення досі є в проєкті.
card-label = Позначка
card-exportable = Експортується
card-numbered = Нумерований
card-created = Створено
card-modified = Змінено
card-yes = так
card-no = ні
card-type = Тип
card-position = Позиція
card-children = Дочірні елементи
card-goal = Ціль
card-words = Слова
card-synopsis = Синопсис
card-point-of-view = Точка зору
card-aliases = Також відомий як
export-orphan-footnotes-title = Виноски без посилань
export-comments-dropped =
    { $count ->
        [1] Один коментар не вдалося розмістити в експортованому тексті, тому його пропущено.
        [one] { $count } коментар не вдалося розмістити в експортованому тексті, тому його пропущено.
        [few] { $count } коментарі не вдалося розмістити в експортованому тексті, тому їх пропущено.
        [many] { $count } коментарів не вдалося розмістити в експортованому тексті, тому їх пропущено.
       *[other] { $count } коментаря не вдалося розмістити в експортованому тексті, тому їх пропущено.
    }
export-orphan-footnotes =
    { $count ->
        [1] На одну виноску більше ніде в рукописі немає посилання. Її текст не з’явиться в експортованій книзі.
        [one] На { $count } виноску більше ніде в рукописі немає посилань. Її текст не з’явиться в експортованій книзі.
        [few] На { $count } виноски більше ніде в рукописі немає посилань. Їхній текст не з’явиться в експортованій книзі.
        [many] На { $count } виносок більше ніде в рукописі немає посилань. Їхній текст не з’явиться в експортованій книзі.
       *[other] На { $count } виноски більше ніде в рукописі немає посилань. Їхній текст не з’явиться в експортованій книзі.
    }
menu-footnotes = Ви&носки
footnotes-title = Виноски
footnotes-insert = Вставити &виноску
footnotes-empty = Виносок ще немає. Поставте курсор у сцену, а потім натисніть «+» вгорі або Ctrl+Alt+F.
footnotes-filter-all = Усі
footnotes-filter-document = Цей документ
footnotes-filter-orphaned = Без прив’язки
footnotes-orphaned = На цю виноску більше ніщо не посилається
footnotes-untitled-home = Без назви
footnotes-body-placeholder = текст виноски
footnotes-insert-tooltip = Вставити виноску в позиції курсора (Ctrl+Alt+F)
footnotes-actions = Дії з виноскою
footnotes-delete = &Видалити виноску та посилання на неї
footnotes-no-project = Відкрийте проєкт, перш ніж вставляти виноску.
footnotes-no-caret = Поставте курсор у текст сцени, щоб вставити туди виноску.
footnotes-not-created = Не вдалося додати виноску до проєкту.
footnotes-deleted-toast = Виноску видалено
footnotes-undo-delete = Скасувати
import-document-title = Імпорт документів
import-document-close = Закрити
import-document-step-files = Файли
import-document-step-review = Перевірка
import-document-step-destination = Місце призначення
import-document-step-reconcile = Об’єднання
import-document-reconcile-hint = Деякі з цих елементів у вас уже є. Укажіть, що робити з кожним.
import-document-reconcile-all-new = Ніщо в цьому файлі не збігається з вашим проєктом: усі елементи буде додано як нові.
import-document-duplicate-returns = Ці файли є кількома копіями одного й того самого рукопису, що повертаються. Імпортуйте їх по одному: зіставлення порівнює з вашим проєктом лише одну повернену копію, тож імпорт кількох разом додав би другу копію книги замість об’єднання.
import-document-hunk-added = Прийняти цей новий абзац
import-document-hunk-removed = Прибрати цей абзац
import-document-hunk-changed = Прийняти цю нову редакцію
import-document-col-stray-prose = Його текст
import-document-stray-as-paratext = Залишити як паратекст
import-document-stray-discard = Відкинути текст
import-document-col-current = У вашому проєкті
import-document-col-incoming = У цьому файлі
import-document-col-status = Стан
import-document-col-action = Що зробити
import-document-status-identical = Однакові
import-document-status-editor-edited = Змінено редактором
import-document-status-you-edited = Змінено вами
import-document-status-conflict = Змінено обома
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
import-document-compare-legend = Ваша версія порівняно з версією з цього файлу. Лише для читання.
import-document-compare-close = Закрити
import-document-drop-title = Перетягніть документи сюди
import-document-drop-hint = Markdown (.md), Word (.docx), OpenDocument (.odt) і звичайний текст (.txt)
import-document-browse = Вибрати…
import-document-move-up = Пересунути вгору
import-document-move-down = Пересунути вниз
import-document-remove-file = Вилучити
import-document-file-count = { $count ->
    [one] { $count } файл
    [few] { $count } файли
    [many] { $count } файлів
   *[other] { $count } файлу
}
import-document-no-files = Файли ще не вибрано.
import-document-col-included = Імпортувати
import-document-col-title = Назва
import-document-col-type = Тип
import-document-col-words = Слова
import-document-col-breaks = Розриви
import-document-col-comments = Коментарі
import-document-col-footnotes = Виноски
import-document-col-epigraph = Епіграф
import-document-col-source = Джерело
import-document-level-rules = Рівні заголовків
import-document-level-n = Заголовок { $level }
import-document-add-top-level = Додати верхній рівень
import-document-add-top-level-tooltip = Вставити книгу над усіма проаналізованими елементами, для файлів розділів без заголовка книги
import-document-destination = Місце призначення
import-document-destination-hint = Виберіть підшивку або елемент: нові елементи потрапляють усередину теки або після сцени.
import-document-destination-empty = Підшивок ще немає. Спершу створіть одну.
import-document-plan-empty = Поки що нічого імпортувати.
# How many pictures the items being imported bring into the project, under the
# review tree. Shown only when there is at least one.
import-document-pictures = { $count ->
    [1] Разом із цими документами надходить одне зображення.
    [one] Разом із цими документами надходить { $count } зображення.
    [few] Разом із цими документами надходять { $count } зображення.
    [many] Разом із цими документами надходять { $count } зображень.
   *[other] Разом із цими документами надходить { $count } зображення.
}
import-document-summary = { $rows ->
    [one] { $rows } елемент
    [few] { $rows } елементи
    [many] { $rows } елементів
   *[other] { $rows } елемента
} · { $breaks ->
    [one] { $breaks } розрив сцени
    [few] { $breaks } розриви сцен
    [many] { $breaks } розривів сцен
   *[other] { $breaks } розриву сцени
}
import-document-back = Назад
import-document-cancel = Скасувати
import-document-analyse = Далі
import-document-analysing = Читання документів…
import-document-step-analysing = Читання
import-document-cancel-analysis = Зупинити читання
import-document-analyse-failed = Не вдалося прочитати документи.
import-document-details = Подробиці
import-document-import = Імпортувати
import-document-done = { $count ->
    [one] Імпортовано { $count } елемент
    [few] Імпортовано { $count } елементи
    [many] Імпортовано { $count } елементів
   *[other] Імпортовано { $count } елемента
}
import-document-undo = Скасувати
import-diagnostic-file-unreadable = Не вдалося прочитати «{ $path }»: { $detail }. Інші файли все одно імпортуються.
import-diagnostic-nested-too-deep = «{ $path }» не прочитано: його вміст вкладено глибше ніж на { $limit } рівнів. Жоден справжній документ не наближається до цього, а читання могло б призвести до аварійного завершення Skribisto, тож файл пропущено. Інші файли все одно імпортуються.
import-diagnostic-archive-too-large = «{ $path }» не прочитано: це стиснений файл, створений так, щоб розпаковуватися в набагато більший обсяг, ніж він сам; це могло б вичерпати пам’ять і призвести до аварійного завершення Skribisto, тож файл пропущено. Інші файли все одно імпортуються.
import-diagnostic-lossy-decode = { $count ->
    [1] Один символ у «{ $path }» не вдалося декодувати. Щоб зберегти його, перезбережіть файл у кодуванні UTF-8.
    [one] { $count } символ у «{ $path }» не вдалося декодувати. Щоб зберегти їх, перезбережіть файл у кодуванні UTF-8.
    [few] { $count } символи в «{ $path }» не вдалося декодувати. Щоб зберегти їх, перезбережіть файл у кодуванні UTF-8.
    [many] { $count } символів у «{ $path }» не вдалося декодувати. Щоб зберегти їх, перезбережіть файл у кодуванні UTF-8.
   *[other] { $count } символу в «{ $path }» не вдалося декодувати. Щоб зберегти їх, перезбережіть файл у кодуванні UTF-8.
}
import-diagnostic-decoded-from-bom = «{ $path }» декодовано як { $detail }, а не як UTF-8.
import-diagnostic-empty-file = «{ $path }» порожній.
import-diagnostic-no-headings = «{ $path }» не має заголовків, тому надходить як один елемент.
import-diagnostic-unsupported-format = Файли «.{ $detail }» нічим не читаються, тому «{ $path }» пропущено.
import-diagnostic-front-matter-not-flat = Вступні метадані в «{ $path }»: «{ $detail }» не є простим значенням, тому його пропущено.
import-diagnostic-footnotes-degraded = { $count ->
    [1] Одна виноска в «{ $path }» надходить як звичайний текст: виноски з Markdown не зчитуються.
    [one] { $count } виноска в «{ $path }» надходить як звичайний текст: виноски з Markdown не зчитуються.
    [few] { $count } виноски в «{ $path }» надходять як звичайний текст: виноски з Markdown не зчитуються.
    [many] { $count } виносок у «{ $path }» надходять як звичайний текст: виноски з Markdown не зчитуються.
   *[other] { $count } виноски в «{ $path }» надходять як звичайний текст: виноски з Markdown не зчитуються.
}
import-diagnostic-footnote-not-carried = { $count ->
    [1] Одну виноску в «{ $path }» не вдалося перенести. Решту перенесено.
    [one] { $count } виноску в «{ $path }» не вдалося перенести. Решту перенесено.
    [few] { $count } виноски в «{ $path }» не вдалося перенести. Решту перенесено.
    [many] { $count } виносок у «{ $path }» не вдалося перенести. Решту перенесено.
   *[other] { $count } виноски в «{ $path }» не вдалося перенести. Решту перенесено.
}
import-diagnostic-raw-html-dropped = { $count ->
    [1] Один блок необробленого HTML у «{ $path }» пропущено.
    [one] { $count } блок необробленого HTML у «{ $path }» пропущено.
    [few] { $count } блоки необробленого HTML у «{ $path }» пропущено.
    [many] { $count } блоків необробленого HTML у «{ $path }» пропущено.
   *[other] { $count } блоку необробленого HTML у «{ $path }» пропущено.
}
import-diagnostic-nested-break-dropped = { $count ->
    [1] Один розрив сцени всередині цитати чи списку в «{ $path }» пропущено. Зберігається лише розрив в окремому рядку.
    [one] { $count } розрив сцени всередині цитат чи списків у «{ $path }» пропущено. Зберігається лише розрив в окремому рядку.
    [few] { $count } розриви сцен усередині цитат чи списків у «{ $path }» пропущено. Зберігається лише розрив в окремому рядку.
    [many] { $count } розривів сцен усередині цитат чи списків у «{ $path }» пропущено. Зберігається лише розрив в окремому рядку.
   *[other] { $count } розриву сцени всередині цитат чи списків у «{ $path }» пропущено. Зберігається лише розрив в окремому рядку.
}
# A picture the document shows that does not come with it, one sentence per reason.
# $detail is the picture as the document names it: a file name, a path inside the
# document, or a web address. Nothing is left in the text where it was.
import-diagnostic-image-not-ingested-format = У «{ $path }» зображення «{ $detail }» має формат, який Skribisto не може показати, тож його пропущено. Імпортуються зображення у форматах PNG, JPEG, WebP, GIF, TIFF або BMP: збережіть його в одному з них і вставте вручну.
import-diagnostic-image-not-ingested-remote = У «{ $path }» зображення «{ $detail }» розміщено в інтернеті. Під час імпорту зображення ніколи не завантажуються, тож його пропущено.
import-diagnostic-image-not-ingested-outside = У «{ $path }» зображення «{ $detail }» лежить поза документом і його текою, тож його пропущено. Імпортуються лише зображення, що зберігаються в самому документі або в теці цього документа.
import-diagnostic-image-not-ingested-missing = У «{ $path }» зображення «{ $detail }» не знайдено, тож його пропущено.
import-diagnostic-image-not-ingested-too-large = У «{ $path }» зображення «{ $detail }» більше, ніж приймає проєкт, тож його пропущено.
import-diagnostic-image-not-ingested-unreadable = У «{ $path }» зображення «{ $detail }» не вдалося прочитати як зображення, тож його пропущено.
import-diagnostic-image-not-ingested-heading = У «{ $path }» зображення «{ $detail }» стоїть у заголовку. Заголовок стає назвою, а назва не містить зображень, тож його пропущено.
import-diagnostic-image-not-ingested-note = У «{ $path }» зображення «{ $detail }» стоїть у виносці, кінцевій виносці чи коментарі. Вони зберігають свої слова, але не зображення, тож його пропущено.
import-diagnostic-image-not-ingested-html = У «{ $path }» зображення «{ $detail }» записано як HTML, а імпорт пропускає HTML, тож зображення пропущено разом із ним. Щоб перенести його, запишіть його як зображення Markdown.
import-diagnostic-image-not-ingested-document-full = У «{ $path }» зображення «{ $detail }» пропущено: разом зображення цього документа перевищують те, що вміщує один імпорт. Імпортуйте документ частинами або вставте решту вручну.
import-diagnostic-picture-first-frame-only = У «{ $path }» анімоване зображення «{ $detail }» надходить як його перший кадр. Зображення в проєкті не рухаються.
import-diagnostic-duplicate-title = «{ $title }» трапляється кілька разів ({ $count }). Якщо ви вже імпортували ці файли раніше, їх буде продубльовано.
import-diagnostic-heading-level-jump = «{ $title }» перескакує з рівня заголовка { $from } на { $to }; його розміщено на один рівень нижче від батьківського.
import-diagnostic-illegal-combination = «{ $title }» містить прозу, але тип «{ $kind }» не може її містити. Імпорт призупинено, доки ви не зміните його тип або не виключите його. Інакше не імпортується нічого.
import-diagnostic-tracked-changes-flattened = Файл { $path } був у процесі редагування: { $count ->
    [one] прийнято { $count } відстежувану зміну
    [few] прийнято { $count } відстежувані зміни
    [many] прийнято { $count } відстежуваних змін
   *[other] прийнято { $count } відстежуваної зміни
}, а видалення відкинуто. Це остаточний текст. Щоб натомість залишити якісь ваші формулювання, скористайтеся кнопкою «Порівняти» на останньому кроці.
import-diagnostic-tracked-changes-flattened-by = Файл { $path } був у процесі редагування: { $count ->
    [one] прийнято { $count } відстежувану зміну від { $names }
    [few] прийнято { $count } відстежувані зміни від { $names }
    [many] прийнято { $count } відстежуваних змін від { $names }
   *[other] прийнято { $count } відстежуваної зміни від { $names }
}, а видалення відкинуто. Це остаточний текст. Щоб натомість залишити якісь ваші формулювання, скористайтеся кнопкою «Порівняти» на останньому кроці.
import-diagnostic-text-box-dropped = { $count ->
    [one] { $path } містить { $count } текстове поле. Його текст розташований поза потоком документа, тож неможливо визначити, де йому місце в рукописі. Його не імпортовано.
    [few] { $path } містить { $count } текстові поля. Їхній текст розташований поза потоком документа, тож неможливо визначити, де йому місце в рукописі. Їх не імпортовано.
    [many] { $path } містить { $count } текстових полів. Їхній текст розташований поза потоком документа, тож неможливо визначити, де йому місце в рукописі. Їх не імпортовано.
   *[other] { $path } містить { $count } текстового поля. Їхній текст розташований поза потоком документа, тож неможливо визначити, де йому місце в рукописі. Їх не імпортовано.
}
import-diagnostic-embedded-object-dropped = { $count ->
    [one] { $path } містить { $count } вбудований об’єкт: діаграму, формулу чи щось подібне. У рукописі немає нічого, що могло б його вмістити.
    [few] { $path } містить { $count } вбудовані об’єкти: діаграми, формули чи щось подібне. У рукописі немає нічого, що могло б їх вмістити.
    [many] { $path } містить { $count } вбудованих об’єктів: діаграми, формули чи щось подібне. У рукописі немає нічого, що могло б їх вмістити.
   *[other] { $path } містить { $count } вбудованого об’єкта: діаграми, формули чи щось подібне. У рукописі немає нічого, що могло б їх вмістити.
}
import-diagnostic-field-flattened = { $count ->
    [one] { $path } містить { $count } поле: номер сторінки, перехресне посилання, дату. Воно зберігає текст, який показувало востаннє, і більше не оновлюватиметься.
    [few] { $path } містить { $count } поля: номер сторінки, перехресне посилання, дату. Кожне зберігає текст, який показувало востаннє, і більше не оновлюватиметься.
    [many] { $path } містить { $count } полів: номер сторінки, перехресне посилання, дату. Кожне зберігає текст, який показувало востаннє, і більше не оновлюватиметься.
   *[other] { $path } містить { $count } поля: номер сторінки, перехресне посилання, дату. Кожне зберігає текст, який показувало востаннє, і більше не оновлюватиметься.
}
import-diagnostic-unknown-style-level = { $path } використовує стиль «{ $detail }», який схожий на заголовок, але не вказує рівня. Ці абзаци імпортовано як прозу, щоб не вгадувати їхню глибину.
import-diagnostic-comment-unanchored = Коментар «{ $detail }» у { $path } не вдалося прикріпити до слів, яких він стосувався. Його збережено на елементі, де його можна перемістити.
import-diagnostic-comment-not-carried = Коментар «{ $detail }» у { $path } не імпортовано, бо з файлу не надходить жодного тексту, на якому його можна зберегти. Скопіюйте його з файлу, якщо він вам потрібен.
import-diagnostic-comment-replies-flattened = { $count ->
    [one] { $count } відповідь у { $path } посилалася на коментар, якого немає у файлі, тож вона надходить як окремий коментар.
    [few] { $count } відповіді в { $path } посилалися на коментар, якого немає у файлі, тож вони надходять як окремі коментарі.
    [many] { $count } відповідей у { $path } посилалися на коментар, якого немає у файлі, тож вони надходять як окремі коментарі.
   *[other] { $count } відповіді в { $path } посилалися на коментар, якого немає у файлі, тож вони надходять як окремі коментарі.
}
import-diagnostic-prose-not-verbatim = { $count ->
    [one] { $count } абзац у { $path } не вдалося перенести з форматуванням точно таким, як написано, тож він надходить як звичайний текст. Порівняйте його з оригіналом.
    [few] { $count } абзаци в { $path } не вдалося перенести з форматуванням точно таким, як написано, тож вони надходять як звичайний текст. Порівняйте їх з оригіналом.
    [many] { $count } абзаців у { $path } не вдалося перенести з форматуванням точно таким, як написано, тож вони надходять як звичайний текст. Порівняйте їх з оригіналом.
   *[other] { $count } абзацу в { $path } не вдалося перенести з форматуванням точно таким, як написано, тож вони надходять як звичайний текст. Порівняйте їх з оригіналом.
}
import-diagnostic-styled-spaces-not-carried = { $count ->
    [one] У { $path } { $count } проміжок із пробілів був підкреслений або перекреслений, наприклад рядок, залишений для заповнення. Skribisto не зберігає таких ліній на самих пробілах, тож він надходить як звичайні пробіли або зовсім зникає, якщо починає чи завершує абзац.
    [few] У { $path } { $count } проміжки з пробілів були підкреслені або перекреслені, наприклад рядки, залишені для заповнення. Skribisto не зберігає таких ліній на самих пробілах, тож вони надходять як звичайні пробіли або зовсім зникають, якщо починають чи завершують абзац.
    [many] У { $path } { $count } проміжків із пробілів були підкреслені або перекреслені, наприклад рядки, залишені для заповнення. Skribisto не зберігає таких ліній на самих пробілах, тож вони надходять як звичайні пробіли або зовсім зникають, якщо починають чи завершують абзац.
   *[other] У { $path } { $count } проміжку з пробілів були підкреслені або перекреслені, наприклад рядки, залишені для заповнення. Skribisto не зберігає таких ліній на самих пробілах, тож вони надходять як звичайні пробіли або зовсім зникають, якщо починають чи завершують абзац.
}
import-diagnostic-list-nesting-flattened = { $count ->
    [one] { $count } пункт списку в { $path } вкладено глибше ніж на { $limit } рівнів. Він надходить на рівень { $limit }, найглибший, який зберігає Skribisto, разом зі своїми словами й форматуванням.
    [few] { $count } пункти списку в { $path } вкладено глибше ніж на { $limit } рівнів. Вони надходять на рівень { $limit }, найглибший, який зберігає Skribisto, разом зі своїми словами й форматуванням.
    [many] { $count } пунктів списку в { $path } вкладено глибше ніж на { $limit } рівнів. Вони надходять на рівень { $limit }, найглибший, який зберігає Skribisto, разом зі своїми словами й форматуванням.
   *[other] { $count } пункту списку в { $path } вкладено глибше ніж на { $limit } рівнів. Вони надходять на рівень { $limit }, найглибший, який зберігає Skribisto, разом зі своїми словами й форматуванням.
}
import-diagnostic-table-read-as-paragraphs = { $count ->
    [one] { $count } таблиця в { $path } має рядок, набагато ширший за інші, і надто нерівна, щоб скласти з неї сітку. Її клітинки надходять як абзаци, один за одним, разом зі своїми словами й форматуванням.
    [few] { $count } таблиці в { $path } мають рядок, набагато ширший за інші, і надто нерівні, щоб скласти з них сітку. Їхні клітинки надходять як абзаци, один за одним, разом зі своїми словами й форматуванням.
    [many] { $count } таблиць у { $path } мають рядок, набагато ширший за інші, і надто нерівні, щоб скласти з них сітку. Їхні клітинки надходять як абзаци, один за одним, разом зі своїми словами й форматуванням.
   *[other] { $count } таблиці в { $path } мають рядок, набагато ширший за інші, і надто нерівні, щоб скласти з них сітку. Їхні клітинки надходять як абзаци, один за одним, разом зі своїми словами й форматуванням.
}
import-diagnostic-spaces-shortened = { $count ->
    [one] { $count } послідовність пробілів у { $path } була довшою за { $limit } пробілів і надходить скороченою до { $limit }. Усі слова навколо неї на місці.
    [few] { $count } послідовності пробілів у { $path } були довшими за { $limit } пробілів і надходять скороченими до { $limit }. Усі слова навколо них на місці.
    [many] { $count } послідовностей пробілів у { $path } були довшими за { $limit } пробілів і надходять скороченими до { $limit }. Усі слова навколо них на місці.
   *[other] { $count } послідовності пробілів у { $path } були довшими за { $limit } пробілів і надходять скороченими до { $limit }. Усі слова навколо них на місці.
}
import-diagnostic-epigraph-not-carried = «{ $title }» починається з епіграфа, але тип «{ $kind }» не може його містити. Натомість цитату збережено на початку його тексту.
import-diagnostic-epigraph-placement-ambiguous = Епіграф стоїть між «{ $title }» і «{ $below }» і міг би відкривати будь-який із них. Його віддано «{ $title }», бо саме там зазвичай стоїть епіграф.
import-document-diagnostics = { $errors ->
    [0] { $warnings ->
            [one] { $warnings } примітка
            [few] { $warnings } примітки
            [many] { $warnings } приміток
           *[other] { $warnings } примітки
        }
   *[other] { $errors ->
            [one] { $errors } файл не вдалося прочитати
            [few] { $errors } файли не вдалося прочитати
            [many] { $errors } файлів не вдалося прочитати
           *[other] { $errors } файлу не вдалося прочитати
        }
}
import-document-diagnostics-none = Немає про що повідомити.
versions-title = Версії
versions-scope-synopsis = Синопсис
versions-scope-prose = Основний текст
versions-loading = Перегляд ваших резервних копій…
versions-empty = Попередніх версій цього ще немає
versions-error = Не вдалося прочитати ваші резервні копії. Нічого не втрачено, але цей список може бути неповним
versions-did-not-exist = Станом на { $date } ще не існувало
versions-deleted-after = Видалено десь після { $date }
versions-unreadable = { $count ->
    [one] { $count } резервну копію не вдалося прочитати
    [few] { $count } резервні копії не вдалося прочитати
    [many] { $count } резервних копій не вдалося прочитати
   *[other] { $count } резервної копії не вдалося прочитати
}
versions-thinned = { $count ->
    [one] Зі старінням старі версії рідшають. Власна історія проєкту вже відкинула { $count } попередній стан цього тексту.
    [few] Зі старінням старі версії рідшають. Власна історія проєкту вже відкинула { $count } попередні стани цього тексту.
    [many] Зі старінням старі версії рідшають. Власна історія проєкту вже відкинула { $count } попередніх станів цього тексту.
   *[other] Зі старінням старі версії рідшають. Власна історія проєкту вже відкинула { $count } попереднього стану цього тексту.
}
versions-source-backup = З резервної копії
versions-source-project = З власної історії проєкту
versions-list-caption = Один запис на кожну зміну, а не на кожну резервну копію
versions-pick-a-version = Виберіть версію, щоб побачити, що змінилося
versions-earliest = Найраніша записана версія. Старішої, з якою можна її порівняти, немає.
versions-no-change = У цій частині елемента нічого не змінилося
versions-formatting-only = Тут змінилося лише форматування. Слова ті самі
versions-show-unchanged = Показати незмінене
versions-hide-unchanged = Сховати незмінене
versions-next-change = Наступна зміна
versions-near = біля «{ $text }»
versions-words-added = { $count ->
    [one] Додано { $count } слово
    [few] Додано { $count } слова
    [many] Додано { $count } слів
   *[other] Додано { $count } слова
}
versions-words-removed = { $count ->
    [one] Вилучено { $count } слово
    [few] Вилучено { $count } слова
    [many] Вилучено { $count } слів
   *[other] Вилучено { $count } слова
}
versions-blocks-moved = { $count ->
    [one] Переміщено { $count } абзац
    [few] Переміщено { $count } абзаци
    [many] Переміщено { $count } абзаців
   *[other] Переміщено { $count } абзацу
}
versions-pin = Закріпити резервну копію, з якої походить ця версія. Автоматичне очищення ніколи її не видалить
versions-unpin = Відкріпити резервну копію, з якої походить ця версія. Автоматичне очищення знову зможе її видалити
versions-pinned-only = Показувати лише закріплені версії
versions-pin-note = Закріпити можна лише версії з резервної копії. Закріплення зберігає файл, а власна історія проєкту файлом не є.
versions-pinned-empty = Тут ще нічого не закріплено
versions-pinned-empty-log = Тут ще нічого не закріплено. Закріпити можна лише версії з резервної копії. Закріплення зберігає файл, а власна історія проєкту файлом не є.
versions-range-filter = Показувати лише версії між двома датами
versions-filtered-empty = Жодна версія не відповідає заданим фільтрам
versions-clear-filters = Очистити фільтри
versions-last-30-days = Останні 30 днів
versions-restore-button = Відновити цю версію
versions-restore-confirm-title = Замінити цей текст версією від { $date }?
versions-restore-confirm-text = Поточний текст буде замінено текстом, який цей елемент мав { $date }.
versions-restore-confirm-with-comments = Поточний текст буде замінено текстом, який цей елемент мав { $date }. { $count ->
    [one] { $count } коментар прив’язаний до поточного тексту й може втратити прив’язку.
    [few] { $count } коментарі прив’язані до поточного тексту й можуть утратити прив’язку.
    [many] { $count } коментарів прив’язані до поточного тексту й можуть утратити прив’язку.
   *[other] { $count } коментаря прив’язані до поточного тексту й можуть утратити прив’язку.
}
versions-restore-confirm-undo-note = Спершу створюється резервна копія, а Ctrl+Z скасовує це за один крок.
versions-restored-toast = Відновлено версію від { $date }
versions-undo = Скасувати
versions-restore-row-gone = Цього елемента більше немає в проєкті
versions-restore-no-home = Відтоді тип цього елемента змінився, і старому тексту немає в ньому місця
versions-restore-no-safety-copy = Страхувальна резервна копія не створилася, тож нічого не змінено
versions-restore-failed = Не вдалося відновити: { $error }
versions-restore-backup-busy = Резервне копіювання вже триває. Спробуйте ще раз за мить
versions-restore-in-backup-file = Ви переглядаєте файл резервної копії; відкрийте сам проєкт, щоб відновити в нього
versions-restore-no-project = Жоден проєкт не відкрито
versions-recreate-button = Повернути це…
versions-recreate-picker-title = Куди його помістити?
versions-recreate-picker-empty = У цьому проєкті немає підшивки, куди його можна помістити
versions-recreate-picker-confirm = Повернути сюди
versions-recreate-picker-cancel = Скасувати
versions-recreate-untitled = цей елемент
versions-recreate-confirm-title = Повернути «{ $item }»?
versions-recreate-confirm-text = Його буде додано до { $destination } з текстом, який він мав { $date }.
versions-recreate-confirm-undo-note = Кнопка «Скасувати» в повідомленні, яке з’явиться, одразу прибере його знову.
versions-recreated-toast = «{ $item }» знову у вашому проєкті
versions-recreated-partial-toast = { $count ->
    [1] «{ $item }» повернуто, але один із його текстів не вдалося прочитати
    [one] «{ $item }» повернуто, але { $count } його текст не вдалося прочитати
    [few] «{ $item }» повернуто, але { $count } його тексти не вдалося прочитати
    [many] «{ $item }» повернуто, але { $count } його текстів не вдалося прочитати
   *[other] «{ $item }» повернуто, але { $count } його тексту не вдалося прочитати
}
versions-recreate-already-here = Цей елемент уже є у вашому проєкті
versions-recreate-no-destination = Виберіть місце в підшивці, куди його помістити
versions-recreate-unreadable = Цю резервну копію не вдалося прочитати, тож нічого не додано
versions-recreate-failed = Не вдалося повернути: { $error }
versions-changed-percent = Змінилося { $percent }%
versions-hidden-paragraphs = { $count ->
    [one] … { $count } незмінений абзац …
    [few] … { $count } незмінені абзаци …
    [many] … { $count } незмінених абзаців …
   *[other] … { $count } незміненого абзацу …
}
timeline-title = Повернутися в минуле
timeline-coverage = { $count ->
    [one] Записано { $count } версію, від { $oldest }
    [few] Записано { $count } версії, від { $oldest }
    [many] Записано { $count } версій, від { $oldest }
   *[other] Записано { $count } версії, від { $oldest }
}
timeline-loading = Перегляд минулого вашого проєкту…
timeline-empty = Жодної версії цього проєкту ще не записано
timeline-no-changes = Відтоді нічого не змінилося
timeline-no-text-changes = Відтоді жоден текст не змінився
timeline-slider-label = Записана версія
timeline-series-name = Розмір проєкту
timeline-bars-caption = Кожен стовпчик — це записана версія, його висота відповідає тодішньому розміру проєкту. Виділений стовпчик — той, який ви переглядаєте.
timeline-bars-caption-periods = Версій забагато, щоб показати їх по одній, тому кожен стовпчик охоплює проміжок «{ $period }», а його висота відповідає розміру проєкту наприкінці цього проміжку.
timeline-unit-hour = година
timeline-unit-day = день
timeline-unit-week = тиждень
timeline-unit-month = місяць
timeline-range-filter = Показувати лише версії між двома датами
timeline-range-empty = У ці дати не записано жодної версії
timeline-open-period = Відкрити цей проміжок
timeline-show-all = Показати всю історію
timeline-changed-since = { $count ->
    [one] { $count } елемент відрізняється між { $date } і вашим теперішнім проєктом
    [few] { $count } елементи відрізняються між { $date } і вашим теперішнім проєктом
    [many] { $count } елементів відрізняються між { $date } і вашим теперішнім проєктом
   *[other] { $count } елемента відрізняються між { $date } і вашим теперішнім проєктом
}
timeline-kind-added = Написано відтоді
timeline-kind-removed = Більше немає в проєкті
timeline-kind-changed = Змінено відтоді
timeline-kind-moved = Переміщено відтоді
timeline-not-yet-written = На той момент цього ще не існувало
timeline-no-text-of-its-own = Це не має власного тексту. Це заголовок для того, що всередині
timeline-reader-close = Закрити
timeline-reader-stamp = Яким це було { $date }
timeline-reader-loading = Відкриття записаної версії…
timeline-reader-unreadable = Цей запис не вдалося прочитати. Резервну копію могли перемістити чи видалити, або вона на диску, який не під’єднано.
timeline-reader-compared = Яким це було { $date }, порівняно з тим, що є тепер
timeline-reader-diff-legend = Перекреслене відтоді зникло; підкреслене додано.
timeline-reader-view-label = Що показати
timeline-reader-view-diff = Відмінності
timeline-reader-view-text = Текст
timeline-reader-deleted = Цього більше немає у вашому проєкті. Тут його можна прочитати й скопіювати.
timeline-prose-only-record = Ця точка походить із власної історії проєкту, яка зберігає лише текст, тож неможливо показати, що відтоді видалено чи переміщено. Для цього виберіть резервну копію.
settings-sec-extensions = Розширення
settings-group-games-forward = Завжди вперед
settings-games-forward-toggle = Грати в «Завжди вперед»
settings-games-forward-hint =
    Поки ви граєте, нічого з написаного не можна забрати назад: Backspace, Delete,
    вирізання, перетягування та скасування вимкнено в тих місцях, які ви виберете нижче.
    Ви, як і раніше, можете вільно друкувати, вставляти, форматувати й переміщатися. Чернетка лише росте.
settings-games-session-warning =
    Це вибір на один сеанс: він ніколи не зберігається. Закриття проєкту або вихід зі
    Skribisto завжди завершує гру, і щойно це стається, усе, що ви написали під час гри,
    знову можна скасувати.
settings-group-games-scope = Де діє
settings-games-in-prose = Проза рукопису
settings-games-in-synopsis = Синопсиси
settings-games-scope-hint =
    Коментарі, виноски й назви ніколи не заморожуються: саме там ви занотовуєте виправлення,
    яке щойно заборонили собі робити.
settings-games-inert-warning =
    «Завжди вперед» увімкнено, але воно ні до чого не застосовується. Позначте вище хоча б одне місце,
    інакше під час письма нічого не зміниться.
games-title = Письменницькі ігри
games-forward-name = Завжди вперед
games-forward-blurb =
    Пишіть чернетку, нічого не забираючи назад. Видалення, вирізання й скасування вимкнено,
    поки ви граєте. Напишіть наступне речення замість того, щоб виправляти останнє.
games-forward-playing = Гра триває: видалення вимкнено
games-forward-idle = Гру не розпочато
games-session-note = Завершується, коли ви закриваєте проєкт.
games-scope-prose-and-synopsis = Діє для вашої прози й синопсисів.
games-scope-prose = Діє для вашої прози.
games-scope-synopsis = Діє для ваших синопсисів.
games-scope-nothing = Поки ні до чого не застосовується. Виберіть місце в налаштуваннях.
games-settings-link = Налаштування письменницьких ігор…
statusbar-games-forward = Завжди вперед
statusbar-games-forward-tooltip =
    «Завжди вперед» увімкнено: видалення, вирізання й скасування вимкнено, поки ви
    пишете чернетку. Клацніть, щоб завершити гру.
goal-progress-words = { $g ->
    [one] { $count } з { $goal } слова
    [few] { $count } з { $goal } слів
    [many] { $count } з { $goal } слів
   *[other] { $count } з { $goal } слова
}
goal-progress-characters = { $g ->
    [one] { $count } з { $goal } символу
    [few] { $count } з { $goal } символів
    [many] { $count } з { $goal } символів
   *[other] { $count } з { $goal } символу
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
   *[other] { $count } символу
}
goal-unit-words = Слова
goal-unit-characters = Символи
inspector-goal = Ціль
inspector-goal-none = Без цілі
overview-col-goal = Ціль
overview-excluded-from-export = Виключено з експорту, тож не враховується в жодній сумі
goal-subtree-total-words = { $count ->
    [one] { $items } ціль усередині дає разом слів: { $words }
    [few] { $items } цілі всередині дають разом слів: { $words }
    [many] { $items } цілей усередині дають разом слів: { $words }
   *[other] { $items } цілі всередині дають разом слів: { $words }
}
goal-subtree-total-characters = { $count ->
    [one] { $items } ціль усередині дає разом символів: { $words }
    [few] { $items } цілі всередині дають разом символів: { $words }
    [many] { $items } цілей усередині дають разом символів: { $words }
   *[other] { $items } цілі всередині дають разом символів: { $words }
}
new-work-goal-unit = Рахувати в
new-work-goal-unit-hint = Використовується для всіх цілей за словами чи символами в цьому проєкті. Можна змінити пізніше.
settings-group-goal-unit = Цілі
settings-goal-unit-switch-title = Зміна одиниці підрахунку
settings-goal-unit-switch-text = Цілі та віхи, уже задані для цього проєкту, введено в одиницях «{ $from }». Перехід на «{ $to }» не перетворює їх, тож кожне наявне число тепер читатиметься в одиницях «{ $to }».
settings-goal-unit-switch-informative = Нічого не втрачається. Поверніться назад будь-коли, щоб відновити початкове прочитання, а потім оновіть цілі, які хочете зберегти.
settings-goal-unit-switch-confirm = Перемкнути
goal-distribute-action = Розподілити…
distribute-title = Розподіл цілі
distribute-empty = Усередині немає нічого, між чим можна розподілити ціль.
distribute-weight = Спосіб розподілу
distribute-weight-length = За довжиною
distribute-weight-rows = За кількістю частин
distribute-weight-even = Порівну
distribute-overwrite = Замінити вже задані цілі
distribute-col-item = Елемент
distribute-col-current = Зараз
distribute-col-proposed = Після
distribute-total = Разом це { $total } за цілі { $goal }.
distribute-over-budget = Цілі, уже задані всередині, перевищують власну ціль цього контейнера на { $over }. Збільште його ціль, зменште їхні або замініть їх.
distribute-apply = Розподілити
distribute-cancel = Скасувати
milestone-target-gone = Ціль видалено
milestone-no-target = Без цілі
milestone-add = Додати
milestone-add-label = Чого слід досягти
pace-summary-title = Стан книги
pace-summary-remaining = Залишилося: { $words }
pace-summary-open = Відкрити план
pace-summary-close = Закрити
pace-summary-dont-show = Не показувати під час відкриття
pace-summary-menu = План письма…
menu-help-topics = &Розділи довідки
menu-help-shortcuts = Клавіатурні &скорочення…
menu-help-website = Skribisto в &інтернеті
menu-help-report = Пові&домити про проблему…
menu-help-check-updates = Перевірити &оновлення
menu-command-palette = Палітра &команд…
learn-help-topics = Розділи довідки
learn-shortcuts = Клавіатурні скорочення
learn-website = Skribisto в інтернеті
help-window-title = Довідка
help-filter-topics = Фільтрувати розділи
help-no-matching-topic = Жоден розділ не відповідає.
help-topic-missing = Цей розділ більше недоступний.
help-back = Назад
help-not-translated = Цю сторінку ще не перекладено, тому її показано англійською.
help-section-getting-started = Початок роботи
help-section-writing = Письмо
help-section-reviewing = Рецензування
help-section-exchanging = Як увести й вивести текст
help-section-keeping = Безпека вашої роботи
help-section-extensions = Розширення
help-topic-getting-started = Ваш перший проєкт
help-topic-writing-model = Як складається книга
help-topic-drafts-and-old-versions = Як зберегти старішу чернетку
help-topic-goals-and-pace = Цілі та темп
help-topic-comments = Коментарі
help-topic-round-trip = Надсилання книги читачеві
help-topic-export = Експорт
help-topic-import-documents = Імпорт документів
help-topic-import-projects = Перенесення цілого проєкту
help-topic-import-scrivener = Перехід зі Scrivener
help-topic-backups-and-versions = Резервні копії та версії
help-shortcuts-title = Клавіатурні скорочення
help-shortcuts-filter = Фільтрувати скорочення
help-shortcuts-no-matches = Жодне скорочення не відповідає.
help-shortcuts-rebind = Змінити скорочення…
help-shortcuts-close = Закрити
command-palette-placeholder = Введіть команду
shortcut-name-binder-duplicate = Дублювати
shortcut-name-comments-add = Додати коментар
shortcut-name-comments-add-paragraph = Прокоментувати абзац
shortcut-name-spellcheck-toggle = Перевіряти правопис
shortcut-name-editor-tab-close = Закрити вкладку
shortcut-name-editor-tab-pin = Закріпити або відкріпити вкладку
shortcut-name-editor-save = Зберегти
shortcut-name-work-export = Експорт…
shortcut-name-work-new = Новий проєкт
shortcut-name-work-open = Відкрити проєкт
shortcut-name-window-new = Нове вікно
shortcut-name-work-close = Закрити проєкт
shortcut-name-work-import-scrivener = Імпортувати проєкт Scrivener…
shortcut-name-work-import-scrivener-report = Показати звіт про імпорт зі Scrivener
shortcut-name-app-settings = Налаштування
shortcut-name-app-quit = Вийти
shortcut-name-editor-insert-footnote = Вставити виноску
shortcut-name-format-scene-break = Вставити розрив сцени
shortcut-name-format-major-scene-break = Вставити головний розрив сцени
shortcut-name-format-link = Вставити посилання
shortcut-name-format-mark-nbsp = Вставити нерозривний пробіл
shortcut-name-format-mark-nbhyphen = Вставити нерозривний дефіс
shortcut-name-format-mark-nnbsp = Вставити вузький нерозривний пробіл
shortcut-name-format-mark-zwsp = Вставити пробіл нульової ширини
shortcut-name-go-next = Наступний
shortcut-name-go-prev = Попередній
shortcut-name-go-to = Перейти до
shortcut-name-outline-toggle = Показати або сховати план
shortcut-name-preview-toggle = Показати або сховати смугу попереднього перегляду
shortcut-name-view-fullscreen = Увімкнути або вимкнути повноекранний режим
shortcut-name-view-focus-mode = Увімкнути або вимкнути режим без відволікань
shortcut-name-editor-size-increase = Збільшити розмір тексту
shortcut-name-editor-size-decrease = Зменшити розмір тексту
shortcut-name-editor-size-reset = Скинути розмір тексту
shortcut-name-editor-find = Знайти
shortcut-name-editor-replace = Знайти й замінити
shortcut-name-editor-find-next = Наступний збіг
shortcut-name-editor-find-prev = Попередній збіг
shortcut-name-search-show = Пошук у проєкті
shortcut-name-search-replace = Замінити в проєкті
shortcut-name-outline-open-to-side = Відкрити збоку
shortcut-name-help-topics = Розділи довідки
shortcut-name-help-shortcuts = Клавіатурні скорочення
shortcut-name-help-website = Skribisto в інтернеті
shortcut-name-help-report = Повідомити про проблему
shortcut-name-help-check-updates = Перевірити оновлення
update-check-behind = Доступна версія { $version }.
update-check-current = У вас найновіша версія.
update-check-failed = Не вдалося перевірити наявність новішої версії. { $error }
shortcut-name-command-palette = Палітра команд
help-section-reference = Що є що
help-concept-scene-break = Розрив сцени
help-concept-major-scene-break = Головний розрив сцени
help-concept-find-in-prose = Пошук у прозі
help-concept-story-bible = Довідник історії
help-concept-goal-unit = Слова чи символи
help-concept-goal-progress = Поступ
help-concept-manuscript-words = Що належить до рукопису
help-concept-exportable = Виключено з експорту
help-concept-distribute = Розподіл цілі
help-concept-subtree-total = Сума цілей усередині
help-concept-milestone = Віха
help-concept-pace-plan = План темпу
help-concept-tag = Мітка
help-concept-label = Позначка
help-concept-point-of-view = Точка зору
help-concept-epigraph = Епіграф
help-concept-footnote = Виноска
help-concept-chapter-mode = Будова розділів
help-concept-comment = Коментар
help-concept-backup = Резервна копія
help-concept-version = Версія
help-concept-trash = Кошик
help-concept-spellcheck = Перевірка правопису
help-concept-search-replace = Пошук і заміна
help-concept-note-template = Шаблон нотатки
help-concept-story-bible-entry = Запис довідника історії
help-concept-text-replacement = Заміна тексту
help-concept-smart-punctuation = Розумна пунктуація
help-concept-export-style = Стиль експорту
help-concept-round-trip-marks = Маркери повернення
epigraph-mark-attribution = Рядок джерела
epigraph-mark-attribution-tip = Позначити рядок, у якому стоїть курсор, як джерело цитати, щоб він друкувався як підпис автора
margin-lane-name = Знаки на полях
margin-lane-provider-comments = Коментарі
margin-lane-provider-story-bible = Запис довідника історії
margin-lane-provider-story-bible-hint = У поданні «У прозі» нотатки: де згадано цей запис і які сцени розказано його очима. Також позначає ці імена в самій прозі.
margin-lane-mark-point-of-view = Розказано звідси
margin-lane-mark-named-here = Згадано тут
margin-lane-provider-comments-hint = Де до тексту прикріплено коментар
margin-lane-provider-search = Результати пошуку
margin-lane-provider-search-hint = Усі збіги з тим, що ви шукали востаннє, будь-де в документі
margin-lane-provider-boundaries = Де починається кожен документ
margin-lane-provider-boundaries-hint = Лінія на початку кожної сцени в потоці, щоб ви бачили, де перебуваєте
margin-lane-provider-spelling = Правопис
margin-lane-provider-spelling-hint = Де перевірка правопису позначила слово. Вимкнено, якщо ви не попросите: це лише думка машини про вашу прозу
margin-lane-spelling = { $word }, можлива помилка
margin-lane-search-hit = { $text }, збіг { $index } з { $total }
margin-lane-search-current = { $text }, збіг { $index } з { $total }, поточний
margin-lane-boundary = Початок «{ $title }»
margin-lane-boundary-untitled = документа без назви
settings-page-margin-lane = Знаки на полях
settings-desc-margin-lane = Смуга біля смуги прокручування і що на ній показано
settings-margin-lane-enabled = Показувати смугу знаків на полях
settings-margin-lane-enabled-hint = Той самий перемикач, що й «Вигляд ▸ Знаки на полях».
settings-margin-lane-enabled-more = Вона ніколи не каже, що щось не так: вона показує, де що є, а висновки залишає вам.
settings-group-margin-lane-marks = Що позначається
settings-margin-lane-no-providers = Смугу поки що ніщо не позначає.
settings-group-margin-lane-texture = Фактура діалогів
settings-margin-lane-texture = Показувати фактуру діалогів
settings-margin-lane-texture-hint = Одна смужка на абзац: яка його довжина і яка частина його — пряма мова.
settings-margin-lane-texture-more = Не вимірюється для мов без визначених правил, де смужка показує лише довжину.
settings-group-margin-lane-surfaces = Де показується
settings-margin-lane-surface-editor = Текстовий редактор
settings-margin-lane-surface-stream = Потоки
settings-margin-lane-surface-search-preview = Попередній перегляд пошуку
menu-margin-lane = Знаки на по&лях
status-preset-drafting = Написання чернетки
status-preset-passes = Етапи редагування
status-preset-plume = Plume Creator
status-todo = Зробити
status-draft = Чернетка
status-revised = Переглянуто
status-final = Остаточний
status-outline = Начерк
status-first-edit = 1-ше редагування
status-second-edit = 2-ге редагування
status-done = Готово
status-plume-draft-1 = 1-ша чернетка
status-plume-draft-2 = 2-га чернетка
status-plume-draft-3 = 3-тя чернетка
status-plume-edit-1 = 1-ше редагування
status-plume-edit-2 = 2-ге редагування
status-plume-edit-3 = 3-тє редагування
status-plume-proofread = Коректура
status-plume-finished = Завершено
new-work-statuses = Робочий процес
new-work-statuses-hint = Етапи, через які проходить сцена. Їх можна будь-коли перейменувати, переставити чи доповнити.
status-none = Без стану
inspector-status = Стан
overview-col-status = Стан
overview-status-mixed = частини нижче мають різні стани
help-concept-status = Стан
status-completion-title = Стан книги
status-completion-headline = Завершено сцен: { $done } з { $total }
status-completion-empty = Сцен ще немає. Це зведення читає рукопис, тож заповнюється в міру того, як ви пишете.
status-completion-open = Стан книги…
settings-page-statuses = Стани
settings-desc-statuses = Перейменування, впорядкування, додавання й вилучення етапів, через які проходить сцена
settings-statuses-add = Додати стан
settings-statuses-add-placeholder = Назва нового стану
settings-statuses-desc =
    Етапи, через які проходить сцена, по порядку: від найменш завершеного вгорі до найбільш
    завершеного внизу. Саме цей порядок означає «менш завершений» усюди в
    програмі, тож розташуйте їх так, як насправді відбувається ваша робота.
settings-statuses-apply-preset = Застосувати набір…
settings-statuses-preset-applied = { $added ->
    [one] Додано { $added } стан
    [few] Додано { $added } стани
    [many] Додано { $added } станів
   *[other] Додано { $added } стану
}
settings-statuses-preset-refused = У цьому проєкті вже є шкала станів. Набори заповнюють лише порожню шкалу. Спершу видаліть непотрібні щаблі.
settings-statuses-duplicate = «{ $name }» уже є на цій шкалі
settings-statuses-added = Додано «{ $name }»
settings-statuses-move-up = Пересунути вгору (менш завершений)
settings-statuses-move-down = Пересунути вниз (більш завершений)
settings-statuses-details-placeholder = Що означає цей етап (необов’язково)
settings-statuses-delete = Видалити «{ $name }»
settings-statuses-delete-in-use = { $count ->
    [one] Видалити «{ $name }»: { $count } елемент має цей стан і втратить його
    [few] Видалити «{ $name }»: { $count } елементи мають цей стан і втратять його
    [many] Видалити «{ $name }»: { $count } елементів мають цей стан і втратять його
   *[other] Видалити «{ $name }»: { $count } елемента мають цей стан і втратять його
}
settings-statuses-deleted = Видалено «{ $name }»
settings-statuses-deleted-in-use = { $count ->
    [one] Видалено «{ $name }». { $count } елемент більше не має стану.
    [few] Видалено «{ $name }». { $count } елементи більше не мають стану.
    [many] Видалено «{ $name }». { $count } елементів більше не мають стану.
   *[other] Видалено «{ $name }». { $count } елемента більше не мають стану.
}
settings-statuses-empty-title = Робочого процесу ще немає
settings-statuses-empty-body = Стан показує, наскільки просунулася сцена. Додайте його вгорі або почніть із набору.
status-category-planned = Заплановано
status-category-drafting = Написання чернетки
status-category-needs-work = Потребує доопрацювання
status-category-revised = Переглянуто
status-category-final = Остаточний

## Long-operation progress (shared::progress_text). The backend reports phase
## codes it cannot translate; these are their labels. `progress-percent` is the
## toast body on its own, `progress-percent-with-label` with the phase after it.
progress-percent = { $percent }%
progress-percent-with-label = { $percent }% · { $label }
progress-import-opening = Відкриття проєкту…
progress-import-outline = Читання плану…
progress-import-converting = Перетворення розділів і сцен…
progress-import-writing = Запис файлу .skrib…
progress-import-done = Готово
progress-import-structure = Визначення структури…
progress-export-start = Запуск…
progress-export-writing = Запис…
progress-project-reading = Читання проєкту…
# Settings: the open project's section title, "Work: <its title>".
settings-sec-work-titled = Проєкт: { $title }

## Project-import notices (shared::import_warnings::notices): one line of the
## Plume / Manuskript warnings list each. $file, $folder, $title, $name, $field
## and $error are data (file names, titles, a parser's own message) and stay as
## they are. $container is "single-file" or "folder"; $kind is "scene", "note",
## "synopsis" or "title".
import-notice-flattened-rich-texts = { $count ->
    [one] { $count } текст був вкладений глибше, ніж дозволяє проєкт Skribisto; слова збережено як простий текст, без форматування.
    [few] { $count } тексти були вкладені глибше, ніж дозволяє проєкт Skribisto; слова збережено як простий текст, без форматування.
    [many] { $count } текстів були вкладені глибше, ніж дозволяє проєкт Skribisto; слова збережено як простий текст, без форматування.
   *[other] { $count } тексту були вкладені глибше, ніж дозволяє проєкт Skribisto; слова збережено як простий текст, без форматування.
}
import-notice-free-text-badges = { $count ->
    [one] { $count } значок сцени схожий на довільний текст, а не на список станів, тож його збережено як позначку елемента, а не як мітку.
    [few] { $count } різні значки сцен схожі на довільний текст, а не на список станів, тож їх збережено як позначки елементів, а не як мітки.
    [many] { $count } різних значків сцен схожі на довільний текст, а не на список станів, тож їх збережено як позначки елементів, а не як мітки.
   *[other] { $count } різного значка сцен схожі на довільний текст, а не на список станів, тож їх збережено як позначки елементів, а не як мітки.
}
import-notice-separator-text-dropped = Роздільник «{ $name }» містив текст, якому немає місця в проєкті Skribisto; цей текст не імпортовано.
import-notice-separator-without-scene = Перед роздільником «{ $name }» у його розділі не було сцени, до якої його можна було б приєднати, тож його не імпортовано.
import-notice-chapter-without-scenes = У розділі «{ $name }» немає сцен, тож його роздільники не імпортовано.
import-notice-rich-text-unconverted = Частину форматованого тексту не вдалося перетворити ({ $error }); її не імпортовано.
import-notice-unresolved-cross-link = Не вдалося перейти за посиланням на дійову особу №{ $number }: такого запису немає або він був у кошику.
import-notice-read-copy = Прочитано копію цього проєкту { $container ->
    [single-file] у вигляді одного файлу
   *[folder] у вигляді теки
}.
import-notice-read-copy-dated = Прочитано копію цього проєкту { $container ->
    [single-file] у вигляді одного файлу
   *[folder] у вигляді теки
}, остання зміна: { $day }.
import-notice-original-format = Цей проєкт збережено в первісному форматі Manuskript 2016 року. Його прочитано повністю; нічого в ньому не змінено.
import-notice-newer-single-file-beside = Файл «{ $file }» новіший за цю теку ({ $file_day } проти { $folder_day }). Коли проєкт переводять в однофайловий режим, Manuskript залишає стару теку на місці, тож ця тека може бути покинутою копією; якщо імпортувати натомість файл .msk, буде прочитано новішу версію.
import-notice-pickle-skipped = Файл «{ $file }» не прочитано. Це файл pickle мови Python, а завантаження такого файлу з проєкту було вразливістю в самому Manuskript (CVE-2021-35196); налаштування в ньому стосуються лише відображення, а не тексту.
import-notice-file-unreadable = Не вдалося прочитати «{ $file }» ({ $error }), файл пропущено; усе інше імпортовано.
import-notice-not-utf8 = «{ $file }» не є коректним текстом UTF-8; символи, які не вдалося прочитати, замінено, а імпорт не перервано.
import-notice-table-unreadable = Не вдалося прочитати «{ $file }» ({ $error }); його вміст не імпортовано.
import-notice-outline-missing = Файл «{ $file }» відсутній, тож у цьому проєкті немає рукопису для імпорту.
import-notice-outline-unreadable = Не вдалося прочитати «{ $file }» ({ $error }); рукопис не імпортовано.
import-notice-named-none-row = «{ $file }» починається з іменованого рядка («{ $name }») там, де Manuskript зберігає свій порожній запис «none». Цей рядок пропущено, щоб наступні рядки зберегли номери, які використовує рукопис.
import-notice-unknown-field = У «{ $file }» є поле, невідоме цьому імпортеру: «{ $field }». Його не імпортовано.
import-notice-colorless-labels = { $count ->
    [one] У «{ $file }» { $count } рядок не має кольору й імпортований без нього; натрапивши на такий рядок, Manuskript припиняє завантаження проєкту.
    [few] У «{ $file }» { $count } рядки не мають кольору й імпортовані без нього; натрапивши на такий рядок, Manuskript припиняє завантаження проєкту.
    [many] У «{ $file }» { $count } рядків не мають кольору й імпортовані без нього; натрапивши на такий рядок, Manuskript припиняє завантаження проєкту.
   *[other] У «{ $file }» { $count } рядка не мають кольору й імпортовані без нього; натрапивши на такий рядок, Manuskript припиняє завантаження проєкту.
}
import-notice-settings-unreadable = Не вдалося прочитати «{ $file }» ({ $error }); мову проєкту не імпортовано.
import-notice-revisions-unreadable = Не вдалося прочитати «{ $file }» ({ $error }); історію проєкту не імпортовано, але більше нічого це не зачепило.
import-notice-foreign-files = { $count ->
    [one] У цьому проєкті є { $count } файл, який не належить до формату Manuskript і не імпортований: { $files }.
    [few] У цьому проєкті є { $count } файли, які не належать до формату Manuskript і не імпортовані: { $files }.
    [many] У цьому проєкті є { $count } файлів, які не належать до формату Manuskript і не імпортовані: { $files }.
   *[other] У цьому проєкті є { $count } файлу, які не належать до формату Manuskript і не імпортовані: { $files }.
}
import-notice-foreign-files-more = Файли цього проєкту, які не належать до формату Manuskript і не імпортовані ({ $count }): { $files } і ще { $more }.
import-notice-not-a-character-file = «{ $file }» не є файлом персонажа Manuskript, тож його пропущено.
import-notice-not-a-text-item = «{ $file }» не є текстовим елементом Manuskript, тож його не включено до плану.
import-notice-missing-blank-line = У «{ $file }» після заголовка немає порожнього рядка. Останнє поле збережено, хоча Manuskript його втрачає.
import-notice-folder-without-metadata = У теці «{ $folder }» немає файлу { $file }, тож її імпортовано як звичайну теку з назвою каталогу. Manuskript пропускає такий каталог разом з усім його вмістом.
import-notice-rows-given-ids = { $count ->
    [one] { $count } рядок плану не мав власного ID і отримав його. Manuskript відкидає такий рядок, а якщо це тека, припиняє завантаження проєкту.
    [few] { $count } рядки плану не мали власного ID і отримали його. Manuskript відкидає такий рядок, а якщо це тека, припиняє завантаження проєкту.
    [many] { $count } рядків плану не мали власного ID і отримали його. Manuskript відкидає такий рядок, а якщо це тека, припиняє завантаження проєкту.
   *[other] { $count } рядка плану не мали власного ID і отримали його. Manuskript відкидає такий рядок, а якщо це тека, припиняє завантаження проєкту.
}
import-notice-plots-unreadable = Не вдалося прочитати «{ $file }» ({ $error }); сюжети не імпортовано.
import-notice-world-unreadable = Не вдалося прочитати «{ $file }» ({ $error }); записи про світ не імпортовано.
import-notice-world-not-opml = «{ $file }» не є документом OPML; записи про світ не імпортовано.
import-notice-text-flattened = Текст «{ $title }» вкладений глибше, ніж дозволяє проєкт Skribisto; слова збережено як простий текст, без форматування.
import-notice-text-unconverted = Текст «{ $title }» не вдалося перетворити ({ $error }); його збережено точно таким, яким його написано.
import-notice-text-without-file-name = Для { $kind ->
    [scene] тексту сцени
    [note] тексту нотатки
    [synopsis] синопсису
   *[title] назви
} «{ $title }» не вдалося дібрати назву файлу, тож цю частину не імпортовано.
import-notice-unknown-label = «{ $title }» має позначку { $index }, якої немає в списку позначок цього проєкту. Елемент імпортовано без позначки.
import-notice-unknown-status = «{ $title }» має стан { $index }, якого немає в списку станів цього проєкту. Елемент імпортовано без стану.
import-notice-unknown-point-of-view = Оповідь у «{ $title }» ведеться від імені персонажа, якого немає серед дійових осіб цього проєкту. Точку зору не задано.
import-notice-unresolved-references = Не вдалося розв’язати посилання в тексті «{ $title }» ({ $error }); текст збережено в перетвореному вигляді, разом із маркерами посилань.
import-notice-custom-icons = { $count ->
    [one] { $count } рядок мав власний значок. Skribisto дає кожному рядку значок його типу, тож значок не імпортовано; більше нічого не змінилося.
    [few] { $count } рядки мали власні значки. Skribisto дає кожному рядку значок його типу, тож значки не імпортовано; більше нічого не змінилося.
    [many] { $count } рядків мали власні значки. Skribisto дає кожному рядку значок його типу, тож значки не імпортовано; більше нічого не змінилося.
   *[other] { $count } рядка мали власні значки. Skribisto дає кожному рядку значок його типу, тож значки не імпортовано; більше нічого не змінилося.
}
import-notice-character-colors = { $count ->
    [one] { $count } персонаж мав колір біля імені. Skribisto розфарбовує мітки, а не рядки, тож колір не імпортовано; самого персонажа імпортовано.
    [few] { $count } персонажі мали колір біля імені. Skribisto розфарбовує мітки, а не рядки, тож кольори не імпортовано; самих персонажів імпортовано всіх.
    [many] { $count } персонажів мали колір біля імені. Skribisto розфарбовує мітки, а не рядки, тож кольори не імпортовано; самих персонажів імпортовано всіх.
   *[other] { $count } персонажа мали колір біля імені. Skribisto розфарбовує мітки, а не рядки, тож кольори не імпортовано; самих персонажів імпортовано всіх.
}
import-notice-versions-kept = { $found ->
    [one] У цьому проєкті записано { $found } попередню версію
    [few] У цьому проєкті записано { $found } попередні версії
    [many] У цьому проєкті записано { $found } попередніх версій
   *[other] У цьому проєкті записано { $found } попередньої версії
}; { $kept ->
    [one] збережено { $kept }
    [few] збережено { $kept }
    [many] збережено { $kept }
   *[other] збережено { $kept }
}, проріджених так само, як Skribisto проріджує власну історію. Вони на панелі «Версії».
import-notice-versions-unplaced = { $count ->
    [one] { $count } попередня версія належить рядку, якого вже немає в проєкті, і не може бути розміщена.
    [few] { $count } попередні версії належать рядку, якого вже немає в проєкті, і їх не вдалося розмістити.
    [many] { $count } попередніх версій належать рядку, якого вже немає в проєкті, і їх не вдалося розмістити.
   *[other] { $count } попередньої версії належать рядку, якого вже немає в проєкті, і їх не вдалося розмістити.
}

## Native file dialogs: the window title, and the label of the file-type filter.
dialog-open-work-title = Відкрити проєкт Skribisto
dialog-save-as-file-title = Зберегти як один файл .skrib
dialog-save-as-folder-title = Виберіть батьківську теку для проєкту
dialog-manuskript-file-title = Виберіть проєкт Manuskript
dialog-manuskript-folder-title = Виберіть теку проєкту Manuskript
dialog-plume-file-title = Виберіть проєкт Plume Creator
dialog-filter-skrib = Проєкт Skribisto
dialog-filter-manuskript = Проєкт Manuskript
dialog-filter-plume = Проєкт Plume Creator
dialog-filter-hunspell-aff = Файл афіксів Hunspell (.aff)
dialog-filter-hunspell-dic = Словник Hunspell (.dic)
dialog-filter-export = Експортований файл
# Help ▸ Keyboard shortcuts / Settings ▸ Keymap / command palette.
shortcut-name-view-margin-lane = Показати або сховати знаки на полях

## The built-in distraction-free themes' names (distraction_free::theme).
df-theme-paper = Папір
df-theme-sepia = Сепія
df-theme-night = Ніч
df-theme-high-contrast = Висока контрастність
df-theme-midnight = Північ

## Welcome ▸ Examples: the line under a bundled example's title.
welcome-example-byline = Автор: { $author }

## Spoken by a screen reader only.
# Welcome window: the name of the section tab bar.
welcome-sections-a11y = Розділи вікна привітання
# A comment's anchored span in the editor: author, plain-text preview, replies.
comments-a11y-summary-author = { $author }: { $text }
comments-a11y-summary-replies = { $text } ({ $count ->
    [one] { $count } відповідь
    [few] { $count } відповіді
    [many] { $count } відповідей
   *[other] { $count } відповіді
})
comments-a11y-summary-author-replies = { $author }: { $text } ({ $count ->
    [one] { $count } відповідь
    [few] { $count } відповіді
    [many] { $count } відповідей
   *[other] { $count } відповіді
})
# Margin marks beside the text: one mark, several merged into one, and the
# lane's own name when nothing more specific was given.
margin-lane-a11y-mark = { $label }, { $percent }% від початку
margin-lane-a11y-marks = { $label }, знаків: { $count }, { $percent }% від початку
margin-lane-a11y-fallback = { $count ->
    [one] Знаки на полях, { $count } знак
    [few] Знаки на полях, { $count } знаки
    [many] Знаки на полях, { $count } знаків
   *[other] Знаки на полях, { $count } знака
}
# Export ▸ Choose: the row standing for a binder that has no name.
export-choose-unnamed-binder = Підшивка без назви
# The built-in export styles named in words (the national manuscript presets
# keep their own language's name).
export-style-neutral = Нейтральний
export-style-manuscript-shunn = Стандартний рукопис (Shunn)
export-style-ebook-clean = Чиста електронна книга

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
settings-paratext-error-no-name = Структура повинна мати рядок `name`.
# Export: started with no project open (a menu raced a close).
export-error-no-project = Немає відкритого проєкту для експорту.

shortcut-group-work = Проєкт
shortcut-group-edit = Зміни
shortcut-group-view = Вигляд
shortcut-group-document = Документ
shortcut-group-format = Формат
shortcut-group-go = Перехід
shortcut-group-tools = Інструменти
shortcut-group-help = Довідка

## Manuskript import: the headings of the notes it writes and the title of a
## project with no name. Written into the imported project once, in the
## interface language.
import-manuskript-section-paragraph-summary = Виклад в одному абзаці
import-manuskript-section-full-summary = Повний виклад
import-manuskript-section-motivation = Мотивація
import-manuskript-section-epiphany = Прозріння
import-manuskript-section-notes = Нотатки
import-manuskript-section-description = Опис
import-manuskript-section-passion = Пристрасть
import-manuskript-section-result = Підсумок
import-manuskript-section-summary = Стислий виклад
import-manuskript-section-meta = Примітки
import-manuskript-section-series = Серія
import-manuskript-section-volume = Том
import-manuskript-section-genre = Жанр
import-manuskript-section-license = Ліцензія
import-manuskript-section-email = Електронна пошта
import-manuskript-section-situation = Ситуація
import-manuskript-section-sentence = Одне речення
import-manuskript-section-paragraph = Один абзац
import-manuskript-section-page = Одна сторінка
import-manuskript-section-full = Повний виклад
import-manuskript-untitled-project = Імпортований проєкт Manuskript

## Plume import: the story-bible groups an old project implies, and the title
## of a project with no name.
import-plume-group-characters = Персонажі
import-plume-group-items = Предмети
import-plume-group-places = Місця
import-plume-untitled-project = Імпортований проєкт Plume
