# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

tags-preset-basic = Базовый
tags-preset-scifi = Научная фантастика
tags-preset-fantasy = Фантастика
tags-preset-mystery = Тайна
tags-preset-historical = Исторический


tags-preset-needs-research = Нужны исследования
tags-preset-continuity-check = Проверка непрерывности
tags-preset-plot-point = сюжетная точка

tags-preset-character = характер
tags-preset-place = место
tags-preset-item = пункт

tags-preset-vessel = судно
tags-preset-planet = планета
tags-preset-organization = организация
tags-preset-creature = существо
tags-preset-faction = фракция
tags-preset-artifact = артефакт
tags-preset-realm = царство
tags-preset-magic-system = Магическая система
tags-preset-suspect = подозреваемый
tags-preset-victim = жертва
tags-preset-clue = ключ
tags-preset-red-herring = красная сельдь
tags-preset-historical-figure = Историческая фигура
tags-preset-source = источник
tags-preset-period-detail = Подробная информация о периоде

tags-pill-list = Метки
tags-pill-add = Добавить тег
tags-none-yet = Никаких тегов
tags-pick-list = Доступные метки
tags-pill-remove = Удалить { $name }
tags-pill-filter-placeholder = Фильтр или название нового тега
tags-pill-no-match = Никаких спичек
tags-pill-create = Создание «{ $name }»
tags-pill-create-failed = Не удалось создать "{ $name }"
tags-pill-new-discoverable = Тег Find-in-Prose
tags-pill-new-discoverable-hint = Элементы с этим тегом сопоставляются с вашей прозой, чтобы заполнить список.

tags-alias-list = Другие имена
tags-alias-add = Добавить другое имя
tags-alias-remove = Удалить { $name }
tags-alias-placeholder = Другое название, затем введите
tags-alias-hint = Названия, которые появляются в вашей прозе, помимо ее названия.
tags-alias-collision = { $name } уже отвечает на это имя.

settings-page-tags = Метки
settings-tags-desc = Тег Find-in-Prose также сообщает Skribisto, чтобы найти имена этого элемента в вашей прозе.
settings-tags-add = Добавить тег
settings-tags-add-placeholder = Назовите новый тег
settings-tags-added = Добавлено "{ $name }"
settings-tags-duplicate = "{ $name }" уже существует
settings-tags-filter = Теги фильтра
settings-tags-count = { $n ->
    [one] { $n } тег
    [few] { $n } метки
    [many] { $n } тегов
   *[other] { $n } метки
}
settings-tags-details-placeholder = Что означает этот тег
settings-tags-discoverable = Искать в тексте
settings-tags-creates-in = Новые заметки идут к
settings-tags-creates-in-unset = Спроси меня в первый раз
settings-tags-creates-in-untitled = Безымянная папка
settings-tags-creates-in-trashed = { $name } (в мусорном баке)
settings-tags-template = Запуск шаблона
settings-tags-template-unset = Бланковая нота
settings-tags-delete = Исключить { $name }
settings-tags-deleted = Удален "{ $name }" и удален из каждого пункта
settings-tags-empty = Пока нет бирок.
settings-tags-apply-preset = Применять пресет …
settings-tags-preset-applied = Добавлено { $added }, пропущено { $skipped } уже представлено
settings-tags-csv-filter = CSV файлов
settings-tags-import = Импорт …
settings-tags-export = Экспорт …
settings-tags-imported = Импортировано { $added }, пропущено { $skipped }
settings-tags-exported = экспортируемый { $n ->
    [one] { $n } тег
    [few] { $n } метки
    [many] { $n } тегов
   *[other] { $n } метки
}

tags-chip-more = { $n ->
    [one] Еще { $n } тег
    [few] { $n } Больше тегов
    [many] Еще { $n } тегов
   *[other] { $n } Больше тегов
}

cast-section = Участники сцены
cast-add = Добавить в cast…
cast-add-filter-placeholder = Фильтр Библия …
cast-add-empty = Нет сюжетно-библейских записей, чтобы добавить
cast-pin = Добавить { $name } в литье
cast-unpin = Удалить { $name } из литья
cast-empty = Никто еще не прижался. Добавьте или сохраните предложение.
cast-unresolved = Больше не в библии историй

mentions-confirm = Добавить в состав { $name }
mentions-hit-count = { $n ->
    [one] { $n } раз
    [few] { $n } раза
    [many] { $n } раз
   *[other] { $n } раз
}
mentions-point-of-view-badge = Точка зрения
mentions-point-of-view-badge-tooltip = Объявлена здесь точка зрения, поставленная от руки, не обнаруженная в прозе.

mentions-roster = Участники сцены
mentions-pin = Добавить { $name } в литье

pov-section = Точка зрения
pov-empty = Нет точки зрения установленной
pov-add = Заданная точка зрения …
pov-multiple = Эта сцена имеет две точки зрения.
pov-remove = Удалить { $name } в качестве точки зрения
pov-unresolved = Здесь была прикреплена точка зрения, но эта запись больше не существует или потеряла свою библию.

books-section = Отнесено к
books-empty = Не подано еще под книгу
books-add = Файл под книгой …
books-remove = Удалить из { $name }
books-apply-to-children = Подать заявку на дочерних элементов

note-details-name-placeholder = Название…
note-details-tags = Метки
note-details-aliases = Другие имена
note-details-books = Отнесено к
note-details-books-empty = Не подано ни одной книги
note-details-links = Ссылки
note-details-links-empty = Еще нет ссылок. Добавьте одну, чтобы указать на другую запись в вашей библии.
note-details-links-add = Ссылка на примечание …
note-details-pov = Точка зрения
note-details-pov-empty = Нет точки зрения установленной
note-details-pov-unresolved = Здесь была прикреплена точка зрения, но эта запись больше не существует или потеряла свою библию.
note-details-pov-multiple = Эта записка имеет две точки зрения.
note-details-backlinks = Появляется в рукописи
note-details-backlinks-empty = Пока ничего. Как только это имя появится в вашей прозе, оно появится здесь.
note-details-backlinks-outside = За пределами книг
note-details-backlinks-confirm-all = Подтвердите каждое появление
note-details-backlinks-confirm-all-tooltip = Добавьте эту запись в актерский состав каждого документа, показанного здесь, который еще не подтвердил это.
note-details-untitled-document = Без названия

note-in-prose-mentions = { $n ->
    [one] { $n } Упоминание
    [few] { $n } Упоминания
    [many] { $n } Упоминания
   *[other] { $n } Упоминания
}
note-in-prose-mention-at = { $current } из { $total }
note-in-prose-mention-previous = Предыдущий упоминание
note-in-prose-mention-next = Следующее упоминание
