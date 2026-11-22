# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Ресурси: файли та вебпосилання, прикріплені до елементів підшивки.

## Панель

resources-title = Ресурси
shortcut-name-resources-show = Показати ресурси
resources-add-files = Додати файли
resources-add-files-title = Додати файли до цього елемента
resources-add-link = Додати вебпосилання
resources-drop-hint = Перетягніть сюди файли або вебадреси, щоб прикріпити їх
resources-no-item = Відкрийте елемент, щоб побачити його ресурси.
resources-empty = До цього елемента ще нічого не прикріплено. Додайте файли або вебпосилання кнопками вгорі або перетягніть їх сюди.
resources-actions = Дії
resources-not-found = Не знайдено
resources-row-linked = Пов’язаний файл
resources-row-linked-size = Пов’язаний файл, { $size }
resources-row-name = { $title }, { $kind }, { $detail }
resources-row-name-missing = { $title }, { $kind }, { $detail }, не знайдено

## Типи файлів, як їх промовляє програма зчитування з екрана

resources-kind-picture = зображення
resources-kind-pdf = документ PDF
resources-kind-audio = звукозапис
resources-kind-video = відео
resources-kind-document = документ
resources-kind-ebook = електронна книга
resources-kind-archive = архів
resources-kind-file = файл
resources-kind-web-link = вебпосилання

## Меню рядка

resources-open = Відкрити
resources-show-in-folder = Показати в теці
resources-locate = Знайти знову…
resources-insert-into-text = Вставити в текст
resources-insert-picture = Вставити зображення в текст
resources-insert-link = Вставити посилання в текст
resources-edit = Перейменувати й описати…
resources-detach = Відкріпити від цього елемента
resources-remove = Вилучити з проєкту…

## Два діалоги

resources-link-dialog-title = Додати вебпосилання
resources-link-dialog-address = Адреса
resources-link-dialog-address-placeholder = https://
resources-link-dialog-name = Назва
resources-link-dialog-name-placeholder = Назва сайту, якщо залишити порожнім
resources-link-dialog-add = Додати
resources-edit-dialog-title = Перейменувати й описати
resources-edit-dialog-name = Назва
resources-edit-dialog-description = Опис
resources-edit-dialog-description-placeholder = Що це таке і навіщо ви це зберегли
resources-edit-dialog-save = Зберегти
resources-dialog-cancel = Скасувати

## Додавання

resources-large-title = Великий файл
resources-large-text = «{ $name }» займає { $size }. Якщо зберігати його в проєкті, він подорожує з кожною копією і кожною резервною копією проєкту. Якщо пов’язати його, він залишається на своєму місці, а проєкт лише пам’ятає, де його знайти.
resources-large-text-many = «{ $name }» займає { $size }, і ще { $others ->
    [1] один файл теж великий
    [one] { $others } файл теж великий
    [few] { $others } файли теж великі
    [many] { $others } файлів теж великі
   *[other] { $others } файлу теж великі
}. Якщо зберігати їх у проєкті, вони подорожують з кожною копією і кожною резервною копією проєкту. Якщо пов’язати їх, вони залишаються на своєму місці, а проєкт лише пам’ятає, де їх знайти.
resources-large-store = Зберігати в проєкті
resources-large-link = Краще пов’язати
resources-large-remember = Більше не питати
resources-storing-title = Додавання файлів до проєкту
resources-store-cancel = Скасувати
resources-store-cancelled = Файли не додано.
resources-store-busy = Файли ще додаються. Спробуйте знову, коли додавання завершиться.
resources-store-failed = Не вдалося додати файли: { $error }
resources-no-project = Відкрийте проєкт, перш ніж прикріплювати файли.
resources-moved-away = Проєкт перемістили, поки копіювався файл.
resources-some-failed = { $count ->
    [1] Не вдалося додати один файл: { $names }
    [one] Не вдалося додати { $count } файл: { $names }
    [few] Не вдалося додати { $count } файли: { $names }
    [many] Не вдалося додати { $count } файлів: { $names }
   *[other] Не вдалося додати { $count } файлу: { $names }
}
resources-add-failed = Не вдалося додати ресурси: { $error }
resources-added = { $count ->
    [1] Додано один ресурс
    [one] Додано { $count } ресурс
    [few] Додано { $count } ресурси
    [many] Додано { $count } ресурсів
   *[other] Додано { $count } ресурсу
}
resources-dropped-added = { $count ->
    [1] Додано як ресурс цього елемента й пов’язано в тексті
    [one] { $count } файл додано як ресурс цього елемента й пов’язано в тексті
    [few] { $count } файли додано як ресурси цього елемента й пов’язано в тексті
    [many] { $count } файлів додано як ресурси цього елемента й пов’язано в тексті
   *[other] { $count } файлу додано як ресурси цього елемента й пов’язано в тексті
}
resources-undo = Скасувати
resources-link-refused = «{ $url }» не є вебадресою, яку Skribisto може зберегти. Використайте адресу, що починається з http:// або https://, без імені користувача й пароля.

## Сповіщення для проєкту в одному файлі

resources-folder-title = Проєкт стає великим
resources-folder-text = Збережені в ньому файли вже займають { $size }. Проєкт, що зберігається одним файлом, під час кожного збереження записується повністю, разом зі своїми файлами. Проєкт, що зберігається текою, зберігає лише те, що змінилося. Ви можете зараз зберегти його як теку й надалі працювати в цій теці.
resources-folder-switch = Зберегти як теку…
resources-folder-not-now = Не зараз
resources-folder-never = Більше не питати для цього проєкту

## Відкриття

resources-not-found-toast = «{ $name }» немає там, де його очікує проєкт.
resources-unsafe-title = Відкрити «{ $name }»?
resources-unsafe-text = Skribisto не знає, чи безпечний такий тип файлу. Його відкриття може запустити програму на вашому комп’ютері, а проєкт може надійти від когось іншого. Спершу покажіть файл у його теці, щоб побачити, що це.
resources-open-anyway = Усе одно відкрити
resources-open-failed = Не вдалося відкрити файл: { $error }
resources-no-data-folder = ця система не надає Skribisto теки даних, у якій можна відкрити копію
resources-link-gone = Це посилання веде на ресурс, якого в проєкті вже немає.
resources-locate-title = Де тепер «{ $name }»?
resources-relinked-more = { $count ->
    [1] У тій самій теці знайшовся ще один відсутній файл, і його знову пов’язано.
    [one] У тій самій теці знайшовся ще { $count } відсутній файл, і його знову пов’язано.
    [few] У тій самій теці знайшлися ще { $count } відсутні файли, і їх знову пов’язано.
    [many] У тій самій теці знайшлося ще { $count } відсутніх файлів, і їх знову пов’язано.
   *[other] У тій самій теці знайшлося ще { $count } відсутнього файлу, і їх знову пов’язано.
}
resources-changed-elsewhere = «{ $name }» змінено в іншій програмі. Проєкт зберігає нову версію.

## Редагування

resources-edit-failed = Не вдалося змінити ресурс: { $error }
resources-remove-title = Вилучити «{ $title }» з проєкту?
resources-remove-text = Ресурс відкріплюється від усіх елементів, до яких він прикріплений. Файл, що зберігається в проєкті, зникає разом із ним; пов’язаний файл і вебсторінка залишаються на своєму місці. Скасування повертає його.
resources-remove-confirm = Вилучити
resources-remove-failed = Не вдалося вилучити ресурс: { $error }

## У текст

resources-no-text = Спершу поставте курсор у текст, потім вставте туди ресурс.
resources-drop-insert-picture = Вставити зображення
resources-drop-insert-link = Вставити посилання на ресурс

## Поступ

progress-resources-storing = Копіювання файлів
progress-resources-stored = Файли скопійовано
