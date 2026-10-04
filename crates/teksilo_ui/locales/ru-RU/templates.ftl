# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

settings-page-templates = Шаблоны
settings-templates-description = Многоразовые куски письма вы можете опустить в все, что вы пишете: чистый лист символов, профиль местоположения, битовый лист. Они хранятся в этом проекте, поэтому любой, кто открывает его, получает тот же набор.
settings-templates-filter = Шаблоны фильтров
settings-templates-count = { $n ->
    [0] Никаких шаблонов
    [one] { $n } шаблон
    [few] { $n } шаблона
    [many] { $n } шаблонов
   *[other] { $n } шаблоны
}
settings-templates-empty = У этого проекта пока нет шаблонов.
settings-templates-empty-hint = Добавьте один из предустановленного, импортируйте файл .md или .djot или напишите что-нибудь и выберите Документ ▸ Сохранить в качестве шаблона.
settings-templates-presets = Добавить предустановленный
settings-templates-import = Импорт …
settings-templates-export = Экспорт …
settings-templates-star = Показать первым в меню вставки
settings-templates-unstar = Прекратите сначала показывать
settings-templates-move-up = Вставай.
settings-templates-move-down = Спускайся.
settings-templates-delete = Удалить шаблон
settings-templates-name = Название
settings-templates-name-placeholder = Название шаблона
settings-templates-body-placeholder = Текст этого шаблона вставляет
settings-templates-duplicate-name = Другой шаблон уже называется «{ $name }».
settings-templates-words = { $n ->
    [one] { $n } слово
    [few] { $n } слова
    [many] { $n } слов
   *[other] { $n } слов
}

templates-imported = { $added ->
    [one] { $added } шаблон импортный
    [few] { $added } ввозимые шаблоны
    [many] { $added } импортных шаблонов
   *[other] { $added } импортные шаблоны
}
templates-imported-renamed = Переименован во избежание столкновения: { $names }
templates-imported-skipped = Невозможно прочитать: { $names }
templates-import-filter = Шаблоны (.md, .djot)
templates-exported = { $n ->
    [one] { $n } шаблон экспортируется
    [few] { $n } Экспортные шаблоны
    [many] { $n } экспортируемых шаблонов
   *[other] { $n } вывезенные шаблоны
}
templates-preset-applied = Добавлено "{ $name }"

templates-delete-title = Удалить этот шаблон?
templates-delete-body = "{ $name }" будет удален из этого проекта. Заметки, которые вы уже создали из него, не затрагиваются.
templates-delete-confirm = Удалить

menu-document = &Документ
menu-insert-template = Вк&лючить шаблон
menu-insert-template-none = Никаких шаблонов в этом проекте
menu-save-as-template = &Сохранить в шаблоне …

save-as-template-title = Сохранить как шаблон
save-as-template-explain = Текст, который вы редактируете, становится шаблоном, который вы можете найти в любом другом месте.
save-as-template-name = Название
save-as-template-placeholder = Анкета персонажа
save-as-template-duplicate = Шаблон под названием «{ $name }» уже существует
save-as-template-empty-editor = Этот редактор пуст. Нечего спасать.
save-as-template-confirm = Сохранить шаблон
save-as-template-saved = Сохранил "{ $name }" в качестве шаблона
template-inserted = Вставлено "{ $name }"

note-template-preset-character-sheet = Анкета персонажа
note-template-preset-location = Место
note-template-preset-artifact = Предмет
note-template-preset-beat-sheet = План сюжетных событий
note-template-preset-faction = Фракция
note-template-preset-research-note = Исследовательская заметка

note-template-set-essentials = Основное
note-template-set-everything = Каждый шаблон

note-template-section-identity = Идентичность
note-template-section-appearance = внешний вид
note-template-section-voice = Голос
note-template-section-psychology = Внутренняя жизнь
note-template-section-history = История
note-template-section-arc = Сюжетная арка
note-template-section-first-impression = Первое впечатление
note-template-section-in-the-story = В этой истории
note-template-section-the-thing-itself = Сам объект
note-template-section-the-scene = Место действия
note-template-section-the-shape = Форма
note-template-section-what-it-is = Что это такое
note-template-section-source = Источник
note-template-section-what-it-says = Что там написано

note-template-field-full-name = Полное имя
note-template-field-known-as = Также известен как, отслеживается в поле инспектора Алиас
note-template-field-age = возраст
note-template-field-role-in-story = Роль в истории
note-template-field-build-and-features = Строить и различать особенности
note-template-field-habitual-bearing = Обычный подшипник
note-template-field-speech-patterns = Рисунки речи и словесные тики
note-template-field-what-they-never-say = О чем никогда не говорят
note-template-field-want = Чего они хотят
note-template-field-need = Что им нужно, но они этого не знают
note-template-field-fear = Чего они боятся
note-template-field-flaw = Недостаток, который стоит им
note-template-field-formative-event = Формирующее событие
note-template-field-relationships = Отношения
note-template-field-starts-as = Начинается как
note-template-field-ends-as = Конец как
note-template-field-what-you-notice-first = Что вы заметите в первую очередь
note-template-field-sound-and-smell = Звук и запах
note-template-field-light-and-weather = Свет и погода
note-template-field-what-happened-here = Что здесь произошло?
note-template-field-who-lives-or-works-here = Кто живет или работает здесь
note-template-field-scenes-set-here = Сцены, установленные здесь
note-template-field-why-it-matters = Почему это важно
note-template-field-appearance = внешний вид
note-template-field-age-and-origin = Возраст и происхождение
note-template-field-what-it-does = Что он делает
note-template-field-who-holds-it = Кто держит его
note-template-field-who-wants-it = Кто хочет этого?
note-template-field-pov = Точка зрения
note-template-field-time-and-place = Время и место
note-template-field-goal = Цель
note-template-field-conflict = конфликт
note-template-field-turn = Поворот
note-template-field-exit-emotion = Эмоции на выходе
note-template-field-purpose = Цель
note-template-field-who-leads-it = Кто руководит
note-template-field-resources = Средства
note-template-field-allies-and-enemies = Союзники и враги
note-template-field-what-it-wants-now = Чего мы хотим сейчас
note-template-field-where-from = Откуда он взялся
note-template-field-page-or-link = Страница или ссылка
note-template-field-key-facts = Ключевые факты
note-template-field-how-it-is-used = Как это используется в истории
