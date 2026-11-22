# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Ресурсы: файлы и веб-ссылки, прикреплённые к элементам подшивки.

## Панель

resources-title = Ресурсы
shortcut-name-resources-show = Показать ресурсы
resources-add-files = Добавить файлы
resources-add-files-title = Добавить файлы к этому элементу
resources-add-link = Добавить веб-ссылку
resources-drop-hint = Перетащите сюда файлы или веб-адреса, чтобы прикрепить их
resources-no-item = Откройте элемент, чтобы увидеть его ресурсы.
resources-empty = К этому элементу пока ничего не прикреплено. Добавьте файлы или веб-ссылки кнопками выше или перетащите их сюда.
resources-actions = Действия
resources-not-found = Не найден
resources-row-linked = Связанный файл
resources-row-linked-size = Связанный файл, { $size }
resources-row-name = { $title }, { $kind }, { $detail }
resources-row-name-missing = { $title }, { $kind }, { $detail }, не найден

## Типы файлов, как их произносит программа чтения с экрана

resources-kind-picture = изображение
resources-kind-pdf = документ PDF
resources-kind-audio = звукозапись
resources-kind-video = видео
resources-kind-document = документ
resources-kind-ebook = электронная книга
resources-kind-archive = архив
resources-kind-file = файл
resources-kind-web-link = веб-ссылка

## Меню строки

resources-open = Открыть
resources-show-in-folder = Показать в папке
resources-locate = Найти заново…
resources-insert-into-text = Вставить в текст
resources-insert-picture = Вставить изображение в текст
resources-insert-link = Вставить ссылку в текст
resources-edit = Переименовать и описать…
resources-detach = Открепить от этого элемента
resources-remove = Удалить из проекта…

## Два диалога

resources-link-dialog-title = Добавить веб-ссылку
resources-link-dialog-address = Адрес
resources-link-dialog-address-placeholder = https://
resources-link-dialog-name = Название
resources-link-dialog-name-placeholder = Название сайта, если оставить пустым
resources-link-dialog-add = Добавить
resources-edit-dialog-title = Переименовать и описать
resources-edit-dialog-name = Название
resources-edit-dialog-description = Описание
resources-edit-dialog-description-placeholder = Что это такое и зачем вы это сохранили
resources-edit-dialog-save = Сохранить
resources-dialog-cancel = Отмена

## Добавление

resources-large-title = Большой файл
resources-large-text = «{ $name }» занимает { $size }. Если хранить его в проекте, он путешествует с каждой копией и каждой резервной копией проекта. Если связать его, он остаётся на своём месте, а проект только помнит, где его найти.
resources-large-text-many = «{ $name }» занимает { $size }, и ещё { $others ->
    [1] один файл тоже большой
    [one] { $others } файл тоже большой
    [few] { $others } файла тоже большие
    [many] { $others } файлов тоже большие
   *[other] { $others } файла тоже большие
}. Если хранить их в проекте, они путешествуют с каждой копией и каждой резервной копией проекта. Если связать их, они остаются на своём месте, а проект только помнит, где их найти.
resources-large-store = Хранить в проекте
resources-large-link = Лучше связать
resources-large-remember = Больше не спрашивать
resources-storing-title = Добавление файлов в проект
resources-store-cancel = Отмена
resources-store-cancelled = Файлы не добавлены.
resources-store-busy = Файлы ещё добавляются. Попробуйте снова, когда добавление закончится.
resources-store-failed = Не удалось добавить файлы: { $error }
resources-no-project = Откройте проект, прежде чем прикреплять файлы.
resources-moved-away = Проект переместили, пока копировался файл.
resources-some-failed = { $count ->
    [1] Не удалось добавить один файл: { $names }
    [one] Не удалось добавить { $count } файл: { $names }
    [few] Не удалось добавить { $count } файла: { $names }
    [many] Не удалось добавить { $count } файлов: { $names }
   *[other] Не удалось добавить { $count } файла: { $names }
}
resources-add-failed = Не удалось добавить ресурсы: { $error }
resources-added = { $count ->
    [1] Добавлен один ресурс
    [one] Добавлен { $count } ресурс
    [few] Добавлено { $count } ресурса
    [many] Добавлено { $count } ресурсов
   *[other] Добавлено { $count } ресурса
}
resources-dropped-added = { $count ->
    [1] Добавлен как ресурс этого элемента и связан в тексте
    [one] { $count } файл добавлен как ресурс этого элемента и связан в тексте
    [few] { $count } файла добавлены как ресурсы этого элемента и связаны в тексте
    [many] { $count } файлов добавлены как ресурсы этого элемента и связаны в тексте
   *[other] { $count } файла добавлены как ресурсы этого элемента и связаны в тексте
}
resources-undo = Отменить
resources-link-refused = «{ $url }» не является веб-адресом, который Skribisto может сохранить. Используйте адрес, начинающийся с http:// или https://, без имени пользователя и пароля.

## Уведомление для проекта в одном файле

resources-folder-title = Проект становится большим
resources-folder-text = Хранящиеся в нём файлы занимают уже { $size }. Проект, хранящийся одним файлом, при каждом сохранении записывается целиком, вместе со своими файлами. Проект, хранящийся папкой, сохраняет только то, что изменилось. Вы можете сейчас сохранить его как папку и дальше работать в этой папке.
resources-folder-switch = Сохранить как папку…
resources-folder-not-now = Не сейчас
resources-folder-never = Больше не спрашивать для этого проекта

## Открытие

resources-not-found-toast = «{ $name }» не там, где его ожидает проект.
resources-unsafe-title = Открыть «{ $name }»?
resources-unsafe-text = Skribisto не знает, безопасен ли такой тип файла. Открытие может запустить программу на вашем компьютере, а проект может прийти от кого-то другого. Сначала покажите файл в его папке, чтобы увидеть, что это.
resources-open-anyway = Всё равно открыть
resources-open-failed = Не удалось открыть файл: { $error }
resources-no-data-folder = эта система не даёт Skribisto папки данных, в которой можно открыть копию
resources-link-gone = Эта ссылка ведёт на ресурс, которого больше нет в проекте.
resources-locate-title = Где теперь «{ $name }»?
resources-relinked-more = { $count ->
    [1] В той же папке нашёлся ещё один недостающий файл, и он снова связан.
    [one] В той же папке нашёлся ещё { $count } недостающий файл, и он снова связан.
    [few] В той же папке нашлись ещё { $count } недостающих файла, и они снова связаны.
    [many] В той же папке нашлись ещё { $count } недостающих файлов, и они снова связаны.
   *[other] В той же папке нашлись ещё { $count } недостающих файла, и они снова связаны.
}
resources-changed-elsewhere = «{ $name }» изменён в другом приложении. Проект сохраняет новую версию.

## Изменение

resources-edit-failed = Не удалось изменить ресурс: { $error }
resources-remove-title = Удалить «{ $title }» из проекта?
resources-remove-text = Ресурс открепляется от всех элементов, к которым он прикреплён. Файл, хранящийся в проекте, уходит вместе с ним; связанный файл и веб-страница остаются на своём месте. Отмена возвращает его.
resources-remove-confirm = Удалить
resources-remove-failed = Не удалось удалить ресурс: { $error }

## В текст

resources-no-text = Сначала поставьте курсор в текст, затем вставьте туда ресурс.
resources-drop-insert-picture = Вставить изображение
resources-drop-insert-link = Вставить ссылку на ресурс

## Ход выполнения

progress-resources-storing = Копирование файлов
progress-resources-stored = Файлы скопированы
