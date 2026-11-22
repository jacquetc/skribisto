# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# الموارد: ملفات وروابط ويب مرفقة بعناصر الحافظة.

## اللوحة

resources-title = الموارد
shortcut-name-resources-show = إظهار الموارد
resources-add-files = إضافة ملفات
resources-add-files-title = إضافة ملفات إلى هذا العنصر
resources-add-link = إضافة رابط ويب
resources-drop-hint = أفلت هنا ملفات أو عناوين ويب لإرفاقها
resources-no-item = افتح عنصرًا لترى موارده.
resources-empty = لم يُرفق شيء بهذا العنصر بعد. أضف ملفات أو روابط ويب بالأزرار أعلاه، أو أفلتها هنا.
resources-actions = إجراءات
resources-not-found = غير موجود
resources-row-linked = ملف مرتبط
resources-row-linked-size = ملف مرتبط، { $size }
resources-row-name = { $title }، { $kind }، { $detail }
resources-row-name-missing = { $title }، { $kind }، { $detail }، غير موجود

## أنواع الملفات، كما يقرؤها قارئ الشاشة

resources-kind-picture = صورة
resources-kind-pdf = مستند PDF
resources-kind-audio = تسجيل صوتي
resources-kind-video = فيديو
resources-kind-document = مستند
resources-kind-ebook = كتاب إلكتروني
resources-kind-archive = أرشيف
resources-kind-file = ملف
resources-kind-web-link = رابط ويب

## قائمة الصف

resources-open = فتح
resources-show-in-folder = الإظهار في المجلد
resources-locate = تحديد المكان…
resources-insert-into-text = إدراج في النص
resources-insert-picture = إدراج الصورة في النص
resources-insert-link = إدراج رابط في النص
resources-edit = إعادة التسمية والوصف…
resources-detach = فصل عن هذا العنصر
resources-remove = إزالة من المشروع…

## مربعا الحوار

resources-link-dialog-title = إضافة رابط ويب
resources-link-dialog-address = العنوان
resources-link-dialog-address-placeholder = https://
resources-link-dialog-name = الاسم
resources-link-dialog-name-placeholder = اسم الموقع، إذا تُرك فارغًا
resources-link-dialog-add = إضافة
resources-edit-dialog-title = إعادة التسمية والوصف
resources-edit-dialog-name = الاسم
resources-edit-dialog-description = الوصف
resources-edit-dialog-description-placeholder = ما هذا، ولماذا احتفظت به
resources-edit-dialog-save = حفظ
resources-dialog-cancel = إلغاء

## الإضافة

resources-large-title = ملف كبير
resources-large-text = حجم «{ $name }» هو { $size }. إذا خُزّن في المشروع، فإنه ينتقل مع كل نسخة منه وكل نسخة احتياطية. وإذا رُبط، يبقى في مكانه، ولا يتذكر المشروع إلا أين يجده.
resources-large-text-many = حجم «{ $name }» هو { $size }، وهناك { $others ->
    [zero] { $others } ملف كبير آخر
    [one] ملف آخر كبير
    [two] ملفان آخران كبيران
    [few] { $others } ملفات أخرى كبيرة
    [many] { $others } ملفًا كبيرًا آخر
   *[other] { $others } ملف كبير آخر
} أيضًا. إذا خُزّنت في المشروع، فإنها تنتقل مع كل نسخة منه وكل نسخة احتياطية. وإذا رُبطت، تبقى في أماكنها، ولا يتذكر المشروع إلا أين يجدها.
resources-large-store = تخزينه في المشروع
resources-large-link = ربطه بدلًا من ذلك
resources-large-remember = عدم السؤال مجددًا
resources-storing-title = إضافة الملفات إلى المشروع
resources-store-cancel = إلغاء
resources-store-cancelled = لم يُضف أي ملف.
resources-store-busy = ما زالت الملفات تُضاف. حاول مرة أخرى عند اكتمال إضافتها.
resources-store-failed = تعذّرت إضافة الملفات: { $error }
resources-no-project = افتح مشروعًا قبل إرفاق الملفات.
resources-moved-away = نُقل المشروع أثناء نسخ الملف.
resources-some-failed = { $count ->
    [zero] تعذّرت إضافة { $count } ملف: { $names }
    [one] تعذّرت إضافة ملف واحد: { $names }
    [two] تعذّرت إضافة ملفين: { $names }
    [few] تعذّرت إضافة { $count } ملفات: { $names }
    [many] تعذّرت إضافة { $count } ملفًا: { $names }
   *[other] تعذّرت إضافة { $count } ملف: { $names }
}
resources-add-failed = تعذّرت إضافة الموارد: { $error }
resources-added = { $count ->
    [zero] لم يُضف أي مورد
    [one] أُضيف مورد واحد
    [two] أُضيف موردان
    [few] أُضيفت { $count } موارد
    [many] أُضيف { $count } موردًا
   *[other] أُضيف { $count } مورد
}
resources-dropped-added = { $count ->
    [zero] لم يُضف أي ملف كمورد لهذا العنصر
    [one] أُضيف كمورد لهذا العنصر، ورُبط في النص
    [two] أُضيف ملفان كموردين لهذا العنصر، ورُبطا في النص
    [few] أُضيفت { $count } ملفات كموارد لهذا العنصر، ورُبطت في النص
    [many] أُضيف { $count } ملفًا كموارد لهذا العنصر، ورُبطت في النص
   *[other] أُضيف { $count } ملف كموارد لهذا العنصر، ورُبطت في النص
}
resources-undo = تراجع
resources-link-refused = «{ $url }» ليس عنوان ويب يستطيع Skribisto الاحتفاظ به. استخدم عنوانًا يبدأ بـ http:// أو https://، من دون اسم مستخدم أو كلمة مرور.

## التنبيه الخاص بمشروع محفوظ في ملف واحد

resources-folder-title = هذا المشروع يكبر
resources-folder-text = تشغل ملفاته المخزّنة الآن { $size }. المشروع المحفوظ كملف واحد يُكتب كاملًا عند كل حفظ، بما فيه ملفاته. أما المحفوظ كمجلد، فلا يحفظ إلا ما تغيّر. يمكنك حفظه كمجلد الآن، ومتابعة العمل في المجلد من بعدها.
resources-folder-switch = الحفظ كمجلد…
resources-folder-not-now = ليس الآن
resources-folder-never = عدم السؤال مجددًا عن هذا المشروع

## الفتح

resources-not-found-toast = «{ $name }» ليس في المكان الذي يتوقعه المشروع.
resources-unsafe-title = هل تريد فتح «{ $name }»؟
resources-unsafe-text = لا يعرف Skribisto إن كان هذا النوع من الملفات آمنًا. قد يؤدي فتحه إلى تشغيل برنامج على حاسوبك، وقد يأتي المشروع من شخص آخر. أظهره أولًا في مجلده لترى ما هو.
resources-open-anyway = الفتح على أي حال
resources-open-failed = تعذّر فتح الملف: { $error }
resources-no-data-folder = لا يوفّر هذا النظام لـ Skribisto أي مجلد بيانات يفتح فيه نسخة
resources-link-gone = يشير هذا الرابط إلى مورد لم يعد في المشروع.
resources-locate-title = أين «{ $name }» الآن؟
resources-relinked-more = { $count ->
    [zero] لم يُعثر على أي ملف مفقود آخر في المجلد نفسه.
    [one] عُثر على ملف مفقود آخر في المجلد نفسه، ورُبط من جديد.
    [two] عُثر على ملفين مفقودين آخرين في المجلد نفسه، ورُبطا من جديد.
    [few] عُثر على { $count } ملفات مفقودة أخرى في المجلد نفسه، ورُبطت من جديد.
    [many] عُثر على { $count } ملفًا مفقودًا آخر في المجلد نفسه، ورُبطت من جديد.
   *[other] عُثر على { $count } ملف مفقود آخر في المجلد نفسه، ورُبطت من جديد.
}
resources-changed-elsewhere = تغيّر «{ $name }» في تطبيق آخر. يحتفظ المشروع بالنسخة الجديدة.

## التحرير

resources-edit-failed = تعذّر تغيير المورد: { $error }
resources-remove-title = هل تريد إزالة «{ $title }» من المشروع؟
resources-remove-text = يُفصل عن كل عنصر مرفق به. الملف المخزّن في المشروع يذهب معه، أما الملف المرتبط وصفحة الويب فيبقيان في مكانهما. التراجع يعيده.
resources-remove-confirm = إزالة
resources-remove-failed = تعذّرت إزالة المورد: { $error }

## في النص

resources-no-text = ضع المؤشر في نص أولًا، ثم أدرج المورد هناك.
resources-drop-insert-picture = إدراج الصورة
resources-drop-insert-link = إدراج رابط إلى المورد

## التقدم

progress-resources-storing = جارٍ نسخ الملفات
progress-resources-stored = نُسخت الملفات
