# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

tags-preset-basic = Простий
tags-preset-scifi = Наукова вигадка
tags-preset-fantasy = ФантазіCity in Alaska USA
tags-preset-mystery = Таємниця
tags-preset-historical = Історичні


tags-preset-needs-research = потребує дослідження
tags-preset-continuity-check = Безперервна перевірка
tags-preset-plot-point = точка графіка

tags-preset-character = символ
tags-preset-place = місце
tags-preset-item = item

tags-preset-vessel = посудина
tags-preset-planet = планета
tags-preset-organization = організація
tags-preset-creature = merge
tags-preset-faction = фракція
tags-preset-artifact = Рельєф
tags-preset-realm = царство
tags-preset-magic-system = магічна система
tags-preset-suspect = підозрюваний
tags-preset-victim = жертва
tags-preset-clue = Питання
tags-preset-red-herring = червоний сережок
tags-preset-historical-figure = Історична фігура
tags-preset-source = джерело
tags-preset-period-detail = докладно про період

tags-pill-list = Мітки
tags-pill-add = Додати мітку
tags-none-yet = Без міток
tags-pick-list = Доступні мітки
tags-pill-remove = Вилучити { $name }
tags-pill-filter-placeholder = Фільтрувати або назвати нову мітку
tags-pill-no-match = Без відповідників мітки
tags-pill-create = Створити " { $name } "
tags-pill-create-failed = Не вдалося створити "{ $name }"
tags-pill-new-discoverable = Мітка пошуку у проза
tags-pill-new-discoverable-hint = Тела с этой тегой совпадают с вашей прозой, чтобы заполнить конюшню.

tags-alias-list = Інші імена
tags-alias-add = Додати іншу назву
tags-alias-remove = Вилучити { $name }
tags-alias-placeholder = Ще одне ім' я, потім Enter
tags-alias-hint = Імена, які з'являються у вашій прозі, поряд з її назвою.
tags-alias-collision = { $name } вже відповідає на це ім'я.

settings-page-tags = Мітки
settings-tags-desc = Мітки позначають елементи вашої програми. Мітка find- in- prose також наказує Skribisto шукати назви цього елемента у прозах.
settings-tags-add = Додати мітку
settings-tags-add-placeholder = Назва нової мітки
settings-tags-added = Додано "{ $name }"
settings-tags-duplicate = " { $name } " вже існує
settings-tags-filter = Мітки фільтрування
settings-tags-count = { $n ->
    [one] { $n } мітка
    [few] { $n } мітки
    [many] { $n } міток
   *[other] { $n } мітки
}
settings-tags-details-placeholder = Що означає ця мітка
settings-tags-discoverable = Шукати в тексті
settings-tags-creates-in = Нові нотатки
settings-tags-creates-in-unset = Спитай мене вперше.
settings-tags-creates-in-untitled = Тека без назви
settings-tags-creates-in-trashed = { $name } (в смітнику)
settings-tags-template = Початковий шаблон
settings-tags-template-unset = Порожня нота
settings-tags-delete = Вилучити { $name }
settings-tags-deleted = Вилучено{ $name }" і видаляла його з кожного елемента.
settings-tags-empty = Пока нет тегов.
settings-tags-apply-preset = Застосувати набір…
settings-tags-preset-applied = Додано { $added }, вже пропущено { $skipped }
settings-tags-csv-filter = CSV файли
settings-tags-import = Імпорт …
settings-tags-export = Export…
settings-tags-imported = Імпортовано { $added }, пропущено { $skipped }
settings-tags-exported = Експортовано { $n ->
    [one] { $n } мітка
    [few] { $n } мітки
    [many] { $n } міток
   *[other] { $n } мітки
}

tags-chip-more = { $n ->
    [one] Ще { $n } мітка
    [few] Додаткові { $n } мітки
    [many] { $n } додаткових теґів
   *[other] { $n } більше теґів
}

cast-section = Учасники сцени
cast-add = Додати до tag…
cast-add-filter-placeholder = Історія фільтра bible…
cast-add-empty = Немає записів, які можна додати
cast-pin = Додати { $name } буде відкинуто
cast-unpin = Вилучити { $name } від кидання
cast-empty = Додайте або зберігайте пропозицію
cast-unresolved = Не в історії більше

mentions-confirm = Додати до елемента { $name }
mentions-hit-count = { $n ->
    [one] { $n } раз
    [few] { $n } рази
    [many] { $n } разів
   *[other] { $n } разів
}
mentions-point-of-view-badge = Точка зору
mentions-point-of-view-badge-tooltip = Визнаний як точка зору тут, встановлений вручну, не виявлений в прозі.

mentions-roster = Учасники сцени
mentions-pin = Додати { $name } буде відкинуто

pov-section = Точка зору
pov-empty = Не встановлено жодної точки
pov-add = Встановити точку перегляду…
pov-multiple = Ця сцена має дві точки зору.
pov-remove = Вилучити { $name } як точка зору
pov-unresolved = Тут була закріплена точка зору, але цей запис більше не існує і не втратив свого придатного для сюжету теґу.

books-section = Віднесено до
books-empty = Ще не подано у книгу
books-add = Файл у книзі…
books-remove = Вилучити з { $name }
books-apply-to-children = Застосувати до дочірніх елементів

note-details-name-placeholder = Назва…
note-details-tags = Мітки
note-details-aliases = Інші імена
note-details-books = Віднесено до
note-details-books-empty = Ще не подано у книгу
note-details-links = Посилання
note-details-links-empty = Жодних посилань. Додайте один до іншого запису у вашій історії.
note-details-links-add = Посилання на нотатку…
note-details-pov = Точка зору
note-details-pov-empty = Не встановлено жодної точки
note-details-pov-unresolved = Тут була закріплена точка зору, але цей запис більше не існує і не втратив свого придатного для сюжету теґу.
note-details-pov-multiple = Ця нотатка має дві точки зору.
note-details-backlinks = Появляється в рукописі.
note-details-backlinks-empty = Коли це ім'я з'явиться у вашій прозі, воно з'явиться тут.
note-details-backlinks-outside = Поза книгами
note-details-backlinks-confirm-all = Підтвердити кожен вигляд
note-details-backlinks-confirm-all-tooltip = Додати цей запис до набору всіх показаних документів, які ще не підтверджували його
note-details-untitled-document = Без назви

note-in-prose-mentions = { $n ->
    [one] { $n } згадка
    [few] { $n } згадане
    [many] { $n } Згадка
   *[other] { $n } remember
}
note-in-prose-mention-at = { $current } з { $total }
note-in-prose-mention-previous = Попередня згадка
note-in-prose-mention-next = Наступне ім' я
