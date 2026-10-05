# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

tags-preset-basic = Базовый
tags-preset-scifi = Научная фантастика
tags-preset-fantasy = Фэнтези
tags-preset-mystery = Детектив
tags-preset-historical = Исторический роман


tags-preset-needs-research = нужно изучить
tags-preset-continuity-check = проверить связность
tags-preset-plot-point = поворот сюжета

tags-preset-character = персонаж
tags-preset-place = место
tags-preset-item = предмет

tags-preset-vessel = корабль
tags-preset-planet = планета
tags-preset-organization = организация
tags-preset-creature = существо
tags-preset-faction = фракция
tags-preset-artifact = артефакт
tags-preset-realm = королевство
tags-preset-magic-system = система магии
tags-preset-suspect = подозреваемый
tags-preset-victim = жертва
tags-preset-clue = улика
tags-preset-red-herring = ложный след
tags-preset-historical-figure = историческое лицо
tags-preset-source = источник
tags-preset-period-detail = детали эпохи

tags-pill-list = Теги
tags-pill-add = Добавить тег
tags-none-yet = Нет тегов
tags-pick-list = Доступные теги
tags-pill-remove = Удалить { $name }
tags-pill-filter-placeholder = Фильтр или название нового тега
tags-pill-no-match = Подходящих тегов нет
tags-pill-create = Создать «{ $name }»
tags-pill-create-failed = Не удалось создать «{ $name }»
tags-pill-new-discoverable = Тег с поиском в прозе
tags-pill-new-discoverable-hint = Элементы с этим тегом сопоставляются с вашей прозой, чтобы заполнить список действующих лиц.

tags-alias-list = Другие имена
tags-alias-add = Добавить другое имя
tags-alias-remove = Удалить { $name }
tags-alias-placeholder = Другое имя, затем Enter
tags-alias-hint = Имена, под которыми это встречается в вашей прозе, помимо названия.
tags-alias-collision = Это имя уже принадлежит «{ $name }».

settings-page-tags = Теги
settings-tags-desc = Теги помечают элементы вашей подшивки. Тег с поиском в прозе к тому же велит Skribisto искать имена такого элемента в вашей прозе.
settings-tags-add = Добавить тег
settings-tags-add-placeholder = Назовите новый тег
settings-tags-added = Добавлено «{ $name }»
settings-tags-duplicate = «{ $name }» уже существует
settings-tags-filter = Фильтр тегов
settings-tags-count = { $n ->
    [one] { $n } тег
    [few] { $n } тега
    [many] { $n } тегов
   *[other] { $n } тега
}
settings-tags-details-placeholder = Что означает этот тег
settings-tags-discoverable = Поиск в прозе
settings-tags-creates-in = Новые заметки создаются в
settings-tags-creates-in-unset = Спросить в первый раз
settings-tags-creates-in-untitled = Папка без названия
settings-tags-creates-in-trashed = { $name } (в корзине)
settings-tags-template = Начальный шаблон
settings-tags-template-unset = Пустая заметка
settings-tags-delete = Удалить { $name }
settings-tags-deleted = «{ $name }» удалён и снят со всех элементов
settings-tags-empty = Тегов пока нет.
settings-tags-apply-preset = Применить набор…
settings-tags-preset-applied = Добавлено: { $added }, пропущено уже имеющихся: { $skipped }
settings-tags-csv-filter = Файлы CSV
settings-tags-import = Импорт…
settings-tags-export = Экспорт…
settings-tags-imported = Импортировано: { $added }, пропущено: { $skipped }
settings-tags-exported = Экспортировано: { $n ->
    [one] { $n } тег
    [few] { $n } тега
    [many] { $n } тегов
   *[other] { $n } тега
}

tags-chip-more = { $n ->
    [one] Ещё { $n } тег
    [few] Ещё { $n } тега
    [many] Ещё { $n } тегов
   *[other] Ещё { $n } тега
}

cast-section = Действующие лица
cast-add = Добавить в действующие лица…
cast-add-filter-placeholder = Фильтр библии истории…
cast-add-empty = Нет записей библии истории для добавления
cast-pin = Добавить { $name } в действующие лица
cast-unpin = Убрать { $name } из действующих лиц
cast-empty = Пока никто не закреплён. Добавьте кого-нибудь или оставьте предложенное
cast-unresolved = Больше нет в библии истории

mentions-confirm = Добавить в действующие лица «{ $name }»
mentions-hit-count = { $n ->
    [one] { $n } раз
    [few] { $n } раза
    [many] { $n } раз
   *[other] { $n } раза
}
mentions-point-of-view-badge = Точка зрения
mentions-point-of-view-badge-tooltip = Здесь задано как точка зрения — вручную, а не найдено в прозе.

mentions-roster = Действующие лица
mentions-pin = Добавить { $name } в действующие лица

pov-section = Точка зрения
pov-empty = Точка зрения не задана
pov-add = Задать точку зрения…
pov-multiple = У этой сцены две точки зрения.
pov-remove = Убрать { $name } из точек зрения
pov-unresolved = Здесь была закреплена точка зрения, но эта запись больше не существует или потеряла тег библии истории.

books-section = Относится к
books-empty = Пока не отнесено ни к одной книге
books-add = Отнести к книге…
books-remove = Убрать из { $name }
books-apply-to-children = Применить к вложенным

note-details-name-placeholder = Название…
note-details-tags = Теги
note-details-aliases = Другие имена
note-details-books = Относится к
note-details-books-empty = Пока не отнесено ни к одной книге
note-details-links = Ссылки
note-details-links-empty = Ссылок пока нет. Добавьте ссылку на другую запись вашей библии истории.
note-details-links-add = Ссылка на заметку…
note-details-pov = Точка зрения
note-details-pov-empty = Точка зрения не задана
note-details-pov-unresolved = Здесь была закреплена точка зрения, но эта запись больше не существует или потеряла тег библии истории.
note-details-pov-multiple = У этой заметки две точки зрения.
note-details-backlinks = Встречается в рукописи
note-details-backlinks-empty = Пока нигде. Как только это имя появится в вашей прозе, оно будет показано здесь.
note-details-backlinks-outside = За пределами книг
note-details-backlinks-confirm-all = Подтвердить все появления
note-details-backlinks-confirm-all-tooltip = Добавить эту запись в действующие лица всех показанных здесь документов, где она ещё не подтверждена
note-details-untitled-document = Без названия

note-in-prose-mentions = { $n ->
    [one] { $n } упоминание
    [few] { $n } упоминания
    [many] { $n } упоминаний
   *[other] { $n } упоминания
}
note-in-prose-mention-at = { $current } из { $total }
note-in-prose-mention-previous = Предыдущее упоминание
note-in-prose-mention-next = Следующее упоминание
