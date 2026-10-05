# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

tags-preset-basic = Базовий
tags-preset-scifi = Наукова фантастика
tags-preset-fantasy = Фентезі
tags-preset-mystery = Детектив
tags-preset-historical = Історичний
tags-preset-needs-research = потребує дослідження
tags-preset-continuity-check = перевірити послідовність
tags-preset-plot-point = сюжетна точка
tags-preset-character = персонаж
tags-preset-place = місце
tags-preset-item = річ
tags-preset-vessel = корабель
tags-preset-planet = планета
tags-preset-organization = організація
tags-preset-creature = істота
tags-preset-faction = угруповання
tags-preset-artifact = артефакт
tags-preset-realm = королівство
tags-preset-magic-system = система магії
tags-preset-suspect = підозрюваний
tags-preset-victim = жертва
tags-preset-clue = доказ
tags-preset-red-herring = хибний слід
tags-preset-historical-figure = історична постать
tags-preset-source = джерело
tags-preset-period-detail = деталь епохи
tags-pill-list = Мітки
tags-pill-add = Додати мітку
tags-none-yet = Без міток
tags-pick-list = Мітки для вибору
tags-pill-remove = Вилучити { $name }
tags-pill-filter-placeholder = Фільтр або назва нової мітки
tags-pill-no-match = Жодна мітка не відповідає
tags-pill-create = Створити «{ $name }»
tags-pill-create-failed = Не вдалося створити «{ $name }»
tags-pill-new-discoverable = Мітка пошуку в прозі
tags-pill-new-discoverable-hint = Елементи з цією міткою зіставляються з вашою прозою, щоб заповнити список дійових осіб.
tags-alias-list = Також відомий як
tags-alias-add = Додати інше ім’я
tags-alias-remove = Вилучити { $name }
tags-alias-placeholder = Інше ім’я, потім Enter
tags-alias-hint = Імена, під якими це трапляється у вашій прозі, крім назви.
tags-alias-collision = { $name } уже відгукується на це ім’я.
settings-page-tags = Мітки
settings-tags-desc = Мітки позначають елементи у вашій підшивці. Мітка пошуку в прозі також каже Skribisto шукати імена цього елемента у вашій прозі.
settings-tags-add = Створити мітку
settings-tags-add-placeholder = Назва нової мітки
settings-tags-added = Додано «{ $name }»
settings-tags-duplicate = «{ $name }» уже існує
settings-tags-filter = Фільтрувати мітки
settings-tags-count = { $n ->
    [one] { $n } мітка
    [few] { $n } мітки
    [many] { $n } міток
   *[other] { $n } мітки
}
settings-tags-details-placeholder = Що означає ця мітка
settings-tags-discoverable = Шукати в прозі
settings-tags-creates-in = Нові нотатки потрапляють до
settings-tags-creates-in-unset = Запитати першого разу
settings-tags-creates-in-untitled = Тека без назви
settings-tags-creates-in-trashed = { $name } (у кошику)
settings-tags-template = Початковий шаблон
settings-tags-template-unset = Порожня нотатка
settings-tags-delete = Видалити { $name }
settings-tags-deleted = «{ $name }» видалено й прибрано з усіх елементів
settings-tags-empty = Міток ще немає.
settings-tags-apply-preset = Застосувати набір…
settings-tags-preset-applied = Додано: { $added }, пропущено вже наявних: { $skipped }
settings-tags-csv-filter = Файли CSV
settings-tags-import = Імпортувати…
settings-tags-export = Експортувати…
settings-tags-imported = Імпортовано: { $added }, пропущено: { $skipped }
settings-tags-exported = Експортовано { $n ->
    [one] { $n } мітку
    [few] { $n } мітки
    [many] { $n } міток
   *[other] { $n } мітки
}
tags-chip-more = { $n ->
    [one] Ще { $n } мітка
    [few] Ще { $n } мітки
    [many] Ще { $n } міток
   *[other] Ще { $n } мітки
}
cast-section = Дійові особи
cast-add = Додати до дійових осіб…
cast-add-filter-placeholder = Фільтрувати довідник історії…
cast-add-empty = Немає записів довідника історії, які можна додати
cast-pin = Додати { $name } до дійових осіб
cast-unpin = Вилучити { $name } з дійових осіб
cast-empty = Ще нікого не додано. Додайте когось або залиште пропозицію
cast-unresolved = Більше немає в довіднику історії
mentions-confirm = Додати до дійових осіб «{ $name }»
mentions-hit-count = { $n ->
    [1] один раз
    [one] { $n } раз
    [few] { $n } рази
    [many] { $n } разів
   *[other] { $n } раза
}
mentions-point-of-view-badge = Точка зору
mentions-point-of-view-badge-tooltip = Тут оголошено точкою зору: задано вручну, а не виявлено в прозі.
mentions-roster = Дійові особи
mentions-pin = Додати { $name } до дійових осіб
pov-section = Точка зору
pov-empty = Точку зору не задано
pov-add = Задати точку зору…
pov-multiple = Ця сцена має дві точки зору.
pov-remove = Прибрати { $name } як точку зору
pov-unresolved = Тут було закріплено точку зору, але цього запису більше немає або він утратив мітку довідника історії.
books-section = Належить до
books-empty = Ще не належить до жодної книги
books-add = Віднести до книги…
books-remove = Вилучити з { $name }
books-apply-to-children = Застосувати до дочірніх
note-details-name-placeholder = Назва…
note-details-tags = Мітки
note-details-aliases = Інші імена
note-details-books = Належить до
note-details-books-empty = Ще не належить до жодної книги
note-details-links = Зв’язки
note-details-links-empty = Зв’язків ще немає. Додайте зв’язок з іншим записом вашого довідника історії.
note-details-links-add = Зв’язати з нотаткою…
note-details-pov = Точка зору
note-details-pov-empty = Точку зору не задано
note-details-pov-unresolved = Тут було закріплено точку зору, але цього запису більше немає або він утратив мітку довідника історії.
note-details-pov-multiple = Ця нотатка має дві точки зору.
note-details-backlinks = Трапляється в рукописі
note-details-backlinks-empty = Поки нічого. Щойно це ім’я з’явиться у вашій прозі, воно з’явиться й тут.
note-details-backlinks-outside = Поза книгами
note-details-backlinks-confirm-all = Підтвердити всі появи
note-details-backlinks-confirm-all-tooltip = Додати цей запис до дійових осіб кожного показаного тут документа, де його ще не підтверджено
note-details-untitled-document = Без назви
note-in-prose-mentions = { $n ->
    [one] { $n } згадка
    [few] { $n } згадки
    [many] { $n } згадок
   *[other] { $n } згадки
}
note-in-prose-mention-at = { $current } з { $total }
note-in-prose-mention-previous = Попередня згадка
note-in-prose-mention-next = Наступна згадка
