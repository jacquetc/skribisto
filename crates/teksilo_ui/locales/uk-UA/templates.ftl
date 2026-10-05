# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

settings-page-templates = Шаблони
settings-templates-description = Багаторазові фрагменти тексту, які можна вставити в будь-що, над чим ви працюєте: порожню анкету персонажа, опис місця, план сюжетних подій. Вони зберігаються в цьому проєкті, тож кожен, хто його відкриє, отримає той самий набір.
settings-templates-filter = Фільтрувати шаблони
settings-templates-count = { $n ->
    [0] Немає шаблонів
    [one] { $n } шаблон
    [few] { $n } шаблони
    [many] { $n } шаблонів
   *[other] { $n } шаблону
}
settings-templates-empty = У цьому проєкті ще немає шаблонів.
settings-templates-empty-hint = Додайте шаблон із набору, імпортуйте файл .md або .djot або напишіть щось і виберіть «Документ ▸ Зберегти як шаблон».
settings-templates-presets = Додати з набору
settings-templates-import = Імпортувати…
settings-templates-export = Експортувати…
settings-templates-star = Показувати першим у меню вставлення
settings-templates-unstar = Більше не показувати першим
settings-templates-move-up = Пересунути вгору
settings-templates-move-down = Пересунути вниз
settings-templates-delete = Видалити шаблон
settings-templates-name = Назва
settings-templates-name-placeholder = Назва шаблону
settings-templates-body-placeholder = Текст, який вставляє цей шаблон
settings-templates-duplicate-name = Інший шаблон уже називається «{ $name }»
settings-templates-words = { $n ->
    [one] { $n } слово
    [few] { $n } слова
    [many] { $n } слів
   *[other] { $n } слова
}
templates-imported = { $added ->
    [one] Імпортовано { $added } шаблон
    [few] Імпортовано { $added } шаблони
    [many] Імпортовано { $added } шаблонів
   *[other] Імпортовано { $added } шаблону
}
templates-imported-renamed = Перейменовано, щоб уникнути збігу назв: { $names }
templates-imported-skipped = Не вдалося прочитати: { $names }
templates-import-filter = Шаблони (.md, .djot)
templates-exported = { $n ->
    [one] Експортовано { $n } шаблон
    [few] Експортовано { $n } шаблони
    [many] Експортовано { $n } шаблонів
   *[other] Експортовано { $n } шаблону
}
templates-preset-applied = Додано «{ $name }»
templates-delete-title = Видалити цей шаблон?
templates-delete-body = «{ $name }» буде вилучено з цього проєкту. Нотатки, які ви вже створили з нього, не зміняться.
templates-delete-confirm = Видалити
menu-document = Д&окумент
menu-insert-template = Вставити &шаблон
menu-insert-template-none = У цьому проєкті немає шаблонів
menu-save-as-template = Зберегти &як шаблон…
save-as-template-title = Зберегти як шаблон
save-as-template-explain = Текст, який ви редагуєте, стане шаблоном, який можна вставити будь-де.
save-as-template-name = Назва
save-as-template-placeholder = Анкета персонажа
save-as-template-duplicate = Шаблон із назвою «{ $name }» уже існує
save-as-template-empty-editor = Цей редактор порожній. Немає чого зберігати.
save-as-template-confirm = Зберегти шаблон
save-as-template-saved = «{ $name }» збережено як шаблон
template-inserted = Вставлено «{ $name }»
note-template-preset-character-sheet = Анкета персонажа
note-template-preset-location = Локація
note-template-preset-artifact = Предмет
note-template-preset-beat-sheet = План сюжетних подій
note-template-preset-faction = Угруповання
note-template-preset-research-note = Дослідницька нотатка
note-template-set-essentials = Основне
note-template-set-everything = Усі шаблони
note-template-section-identity = Особа
note-template-section-appearance = Зовнішність
note-template-section-voice = Голос
note-template-section-psychology = Внутрішній світ
note-template-section-history = Історія
note-template-section-arc = Дуга розвитку
note-template-section-first-impression = Перше враження
note-template-section-in-the-story = В історії
note-template-section-the-thing-itself = Сам предмет
note-template-section-the-scene = Про сцену
note-template-section-the-shape = Будова
note-template-section-what-it-is = Що це
note-template-section-source = Джерело
note-template-section-what-it-says = Що в ньому сказано
note-template-field-full-name = Повне ім’я
note-template-field-known-as = Також відомий як (зберігається в полі «Також відомий як» інспектора)
note-template-field-age = Вік
note-template-field-role-in-story = Роль в історії
note-template-field-build-and-features = Статура й особливі прикмети
note-template-field-habitual-bearing = Звична постава
note-template-field-speech-patterns = Манера мовлення та словесні звички
note-template-field-what-they-never-say = Чого ніколи не каже
note-template-field-want = Чого хоче
note-template-field-need = Чого потребує, сам того не знаючи
note-template-field-fear = Чого боїться
note-template-field-flaw = Вада, яка дорого коштує
note-template-field-formative-event = Подія, що сформувала
note-template-field-relationships = Стосунки
note-template-field-starts-as = На початку
note-template-field-ends-as = Наприкінці
note-template-field-what-you-notice-first = Що помічаєш першим
note-template-field-sound-and-smell = Звуки й запахи
note-template-field-light-and-weather = Світло й погода
note-template-field-what-happened-here = Що тут сталося
note-template-field-who-lives-or-works-here = Хто тут живе чи працює
note-template-field-scenes-set-here = Сцени, що відбуваються тут
note-template-field-why-it-matters = Чому це важливо
note-template-field-appearance = Зовнішній вигляд
note-template-field-age-and-origin = Вік і походження
note-template-field-what-it-does = Що він робить
note-template-field-who-holds-it = У кого він
note-template-field-who-wants-it = Хто його прагне
note-template-field-pov = Точка зору
note-template-field-time-and-place = Час і місце
note-template-field-goal = Мета
note-template-field-conflict = Конфлікт
note-template-field-turn = Поворот
note-template-field-exit-emotion = Емоція на виході
note-template-field-purpose = Призначення
note-template-field-who-leads-it = Хто його очолює
note-template-field-resources = Ресурси
note-template-field-allies-and-enemies = Союзники й вороги
note-template-field-what-it-wants-now = Чого воно прагне зараз
note-template-field-where-from = Звідки походить
note-template-field-page-or-link = Сторінка або посилання
note-template-field-key-facts = Ключові факти
note-template-field-how-it-is-used = Як це використано в історії
