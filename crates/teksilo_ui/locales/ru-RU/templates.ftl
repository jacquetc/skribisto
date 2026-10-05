# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

settings-page-templates = Шаблоны
settings-templates-description = Многоразовые фрагменты текста, которые можно вставить во что угодно, над чем вы работаете: пустая анкета персонажа, описание места, поэпизодный план. Они хранятся в этом проекте, поэтому любой, кто его откроет, получит тот же набор.
settings-templates-filter = Фильтр шаблонов
settings-templates-count = { $n ->
    [0] Нет шаблонов
    [one] { $n } шаблон
    [few] { $n } шаблона
    [many] { $n } шаблонов
   *[other] { $n } шаблона
}
settings-templates-empty = У этого проекта пока нет шаблонов.
settings-templates-empty-hint = Добавьте шаблон из готового набора, импортируйте файл .md или .djot или напишите что-нибудь и выберите Документ ▸ Сохранить как шаблон.
settings-templates-presets = Добавить из набора
settings-templates-import = Импорт…
settings-templates-export = Экспорт…
settings-templates-star = Показывать первым в меню вставки
settings-templates-unstar = Не показывать первым
settings-templates-move-up = Переместить вверх
settings-templates-move-down = Переместить вниз
settings-templates-delete = Удалить шаблон
settings-templates-name = Название
settings-templates-name-placeholder = Название шаблона
settings-templates-body-placeholder = Текст, который вставляет этот шаблон
settings-templates-duplicate-name = Шаблон с названием «{ $name }» уже есть
settings-templates-words = { $n ->
    [one] { $n } слово
    [few] { $n } слова
    [many] { $n } слов
   *[other] { $n } слова
}

templates-imported = { $added ->
    [one] Импортирован { $added } шаблон
    [few] Импортировано { $added } шаблона
    [many] Импортировано { $added } шаблонов
   *[other] Импортировано { $added } шаблона
}
templates-imported-renamed = Переименованы во избежание совпадений: { $names }
templates-imported-skipped = Не удалось прочитать: { $names }
templates-import-filter = Шаблоны (.md, .djot)
templates-exported = { $n ->
    [one] Экспортирован { $n } шаблон
    [few] Экспортировано { $n } шаблона
    [many] Экспортировано { $n } шаблонов
   *[other] Экспортировано { $n } шаблона
}
templates-preset-applied = Добавлено «{ $name }»

templates-delete-title = Удалить этот шаблон?
templates-delete-body = «{ $name }» будет удалён из этого проекта. Заметки, уже созданные по нему, не затрагиваются.
templates-delete-confirm = Удалить

menu-document = &Документ
menu-insert-template = Вставить &шаблон
menu-insert-template-none = В этом проекте нет шаблонов
menu-save-as-template = Со&хранить как шаблон…

save-as-template-title = Сохранить как шаблон
save-as-template-explain = Редактируемый текст станет шаблоном, который можно вставить в любое другое место.
save-as-template-name = Название
save-as-template-placeholder = Анкета персонажа
save-as-template-duplicate = Шаблон под названием «{ $name }» уже существует
save-as-template-empty-editor = Этот редактор пуст. Сохранять нечего.
save-as-template-confirm = Сохранить шаблон
save-as-template-saved = «{ $name }» сохранён как шаблон
template-inserted = Вставлено «{ $name }»

note-template-preset-character-sheet = Анкета персонажа
note-template-preset-location = Место
note-template-preset-artifact = Предмет
note-template-preset-beat-sheet = Поэпизодный план
note-template-preset-faction = Фракция
note-template-preset-research-note = Заметка с материалами

note-template-set-essentials = Основное
note-template-set-everything = Все шаблоны

note-template-section-identity = Личность
note-template-section-appearance = Внешность
note-template-section-voice = Голос
note-template-section-psychology = Внутренняя жизнь
note-template-section-history = История
note-template-section-arc = Арка персонажа
note-template-section-first-impression = Первое впечатление
note-template-section-in-the-story = В истории
note-template-section-the-thing-itself = Сам предмет
note-template-section-the-scene = Сцена
note-template-section-the-shape = Композиция
note-template-section-what-it-is = Что это такое
note-template-section-source = Источник
note-template-section-what-it-says = Что там написано

note-template-field-full-name = Полное имя
note-template-field-known-as = Другие имена (ведутся в поле инспектора «Другие имена»)
note-template-field-age = Возраст
note-template-field-role-in-story = Роль в истории
note-template-field-build-and-features = Телосложение и особые приметы
note-template-field-habitual-bearing = Привычная манера держаться
note-template-field-speech-patterns = Манера речи и словечки
note-template-field-what-they-never-say = Чего персонаж никогда не скажет
note-template-field-want = Чего персонаж хочет
note-template-field-need = Что ему нужно, хотя он этого не знает
note-template-field-fear = Чего персонаж боится
note-template-field-flaw = Недостаток, который ему дорого обходится
note-template-field-formative-event = Формирующее событие
note-template-field-relationships = Отношения
note-template-field-starts-as = Начинается как
note-template-field-ends-as = Заканчивает как
note-template-field-what-you-notice-first = Что замечаешь в первую очередь
note-template-field-sound-and-smell = Звук и запах
note-template-field-light-and-weather = Свет и погода
note-template-field-what-happened-here = Что здесь произошло
note-template-field-who-lives-or-works-here = Кто живет или работает здесь
note-template-field-scenes-set-here = Сцены, происходящие здесь
note-template-field-why-it-matters = Почему это важно
note-template-field-appearance = Внешний вид
note-template-field-age-and-origin = Возраст и происхождение
note-template-field-what-it-does = Что он делает
note-template-field-who-holds-it = У кого он сейчас
note-template-field-who-wants-it = Кто хочет его заполучить
note-template-field-pov = Точка зрения
note-template-field-time-and-place = Время и место
note-template-field-goal = Цель
note-template-field-conflict = Конфликт
note-template-field-turn = Поворот
note-template-field-exit-emotion = Эмоции на выходе
note-template-field-purpose = Назначение
note-template-field-who-leads-it = Кто её возглавляет
note-template-field-resources = Средства
note-template-field-allies-and-enemies = Союзники и враги
note-template-field-what-it-wants-now = Чего она хочет сейчас
note-template-field-where-from = Откуда это взято
note-template-field-page-or-link = Страница или ссылка
note-template-field-key-facts = Ключевые факты
note-template-field-how-it-is-used = Как это используется в истории
