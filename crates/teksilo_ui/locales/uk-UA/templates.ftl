# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

settings-page-templates = Шаблони
settings-templates-description = Незмінні частини запису ви можете скинути до всього, що ви пишете: порожнього рядка символів, профілю розташування, бітового аркуша. Ці елементи зберігаються у цьому проекті, отже, кожен, хто відкриває його, отримає той самий набір.
settings-templates-filter = Шаблони фільтрів
settings-templates-count = { $n ->
    [0] Немає шаблонів
    [one] { $n } шаблон
    [few] { $n } шаблони
    [many] { $n } шаблонів
   *[other] { $n } шаблони
}
settings-templates-empty = У проекту ще немає шаблонів.
settings-templates-empty-hint = Додайте один з набору, імпортуйте .md або .djot, або запишіть щось і скористайтеся пунктом меню Документ ▸ Зберегти як шаблон.
settings-templates-presets = Додати набір
settings-templates-import = Імпорт …
settings-templates-export = Export…
settings-templates-star = Показувати першим у меню вставки
settings-templates-unstar = Зупинити показ першим
settings-templates-move-up = Пересунути вгору
settings-templates-move-down = Пересунути вниз
settings-templates-delete = Вилучити шаблон
settings-templates-name = Назва
settings-templates-name-placeholder = Назва шаблона
settings-templates-body-placeholder = Текст, що вставляє цей шаблон
settings-templates-duplicate-name = Інший шаблон вже називається "{ $name }"
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
   *[other] Імпортовано { $added }
}
templates-imported-renamed = Перейменовано для уникнення конфлікту: { $names }
templates-imported-skipped = Не вдалося прочитати: { $names }
templates-import-filter = Шаблони (.md, .djot)
templates-exported = { $n ->
    [one] { $n } шаблон експортовано
    [few] { $n } Шаблони експортовано
    [many] { $n } Шаблонів експортовано
   *[other] Експортовано { $n }
}
templates-preset-applied = Додано "{ $name }"

templates-delete-title = Вилучити цей шаблон?
templates-delete-body = З цього проекту буде вилучено " { $name } ." Це не вплине на нотатки, які ви вже створили.
templates-delete-confirm = Видалити

menu-document = Д&окумент
menu-insert-template = В&ставити шаблон
menu-insert-template-none = У цьому проекті немає шаблонів
menu-save-as-template = Збере&гти як шаблон…

save-as-template-title = Зберегти як шаблон
save-as-template-explain = Текст, який ви редагуєте, стає шаблоном, за допомогою якого ви можете перейти до будь- якого іншого тексту.
save-as-template-name = Назва
save-as-template-placeholder = Анкета персонажа
save-as-template-duplicate = Шаблон з назвою "{ $name }" вже існує
save-as-template-empty-editor = Цей редактор порожній. Немає чого зберігати.
save-as-template-confirm = Зберегти шаблон
save-as-template-saved = Врятований "{ $name }" як шаблон
template-inserted = Вставлений "{ $name }"

note-template-preset-character-sheet = Анкета персонажа
note-template-preset-location = Місце
note-template-preset-artifact = Предмет
note-template-preset-beat-sheet = План сюжетних подій
note-template-preset-faction = Фракція
note-template-preset-research-note = Дослідницька нотатка

note-template-set-essentials = Основне
note-template-set-everything = Кожен шаблон

note-template-section-identity = Профіль
note-template-section-appearance = Вигляд
note-template-section-voice = Голос
note-template-section-psychology = Внутрішнє життя
note-template-section-history = Журнал
note-template-section-arc = Сюжетна арка
note-template-section-first-impression = Перше враження
note-template-section-in-the-story = У історії
note-template-section-the-thing-itself = Сам об' єкт
note-template-section-the-scene = Сцена
note-template-section-the-shape = Форма
note-template-section-what-it-is = Що це?
note-template-section-source = Джерело
note-template-section-what-it-says = Що він каже?

note-template-field-full-name = Повне ім' я
note-template-field-known-as = Також відомий як, слідований у полі Псевдоніми інспектора
note-template-field-age = Вік
note-template-field-role-in-story = Роль в історії
note-template-field-build-and-features = Створити і розрізнити можливості
note-template-field-habitual-bearing = Зв' язок з програмою
note-template-field-speech-patterns = Взірці мовлення і словесні позначки
note-template-field-what-they-never-say = Що вони ніколи не кажуть
note-template-field-want = Чого вони хочуть
note-template-field-need = Що їм потрібно, але не знаю.
note-template-field-fear = Чого вони бояться?
note-template-field-flaw = Недоліки, які їх коштують
note-template-field-formative-event = Форматована подія
note-template-field-relationships = Зв' язки
note-template-field-starts-as = Починається як
note-template-field-ends-as = Завершує як
note-template-field-what-you-notice-first = Те, що ви помітили спочатку
note-template-field-sound-and-smell = Звук і запах
note-template-field-light-and-weather = Світло і погода
note-template-field-what-happened-here = Що тут сталося?
note-template-field-who-lives-or-works-here = Хто тут живе і працює?
note-template-field-scenes-set-here = Сцени, встановлені тут
note-template-field-why-it-matters = Чому це важливо
note-template-field-appearance = Вигляд
note-template-field-age-and-origin = Вік і походження
note-template-field-what-it-does = Що він робить?
note-template-field-who-holds-it = Хто тримає його
note-template-field-who-wants-it = Хто хоче?
note-template-field-pov = Точка зору
note-template-field-time-and-place = Час і місце
note-template-field-goal = Мета
note-template-field-conflict = Конфлікт
note-template-field-turn = Поворот
note-template-field-exit-emotion = Емоції на вихід
note-template-field-purpose = Мета
note-template-field-who-leads-it = Хто веде?
note-template-field-resources = Ресурси
note-template-field-allies-and-enemies = Антанта і вороги
note-template-field-what-it-wants-now = Що він хоче зараз
note-template-field-where-from = Звідки воно взялося?
note-template-field-page-or-link = Сторінка або посилання
note-template-field-key-facts = Ключові факти
note-template-field-how-it-is-used = Як вона використовується в історії
