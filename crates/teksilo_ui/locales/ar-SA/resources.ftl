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
resources-drop-hint-unattached = أفلت هنا ملفات أو عناوين ويب لإضافتها دون إرفاقها بأي عنصر
resources-no-item = افتح عنصرًا لترى موارده.
resources-empty = لم يُرفق شيء بهذا العنصر بعد. أضف ملفات أو روابط ويب بالأزرار أعلاه، أو أفلتها هنا.
resources-dock-view-label = الموارد المعروضة
resources-dock-view-item = هذا العنصر
resources-dock-view-unattached = غير مرفقة
resources-dock-view-item-hint = ملفات العنصر الذي أنت فيه وروابطه. ما تضيفه يُرفق به.
resources-dock-view-unattached-hint = الملفات والروابط غير المرفقة بأي عنصر، مثل بحث لم ترتّبه بعد. ما تضيفه هنا لا يُرفق بأي شيء.
resources-unattached-empty = كل مورد في هذا المشروع مرفق بعنصر ما. الملفات والروابط المضافة هنا تبقى غير مرفقة بأي شيء حتى ترفقها.
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
resources-dropped-undo-kept = تغيّر النص منذ الإفلات، لذا لم يُتراجع عن شيء.
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
resources-removed-toast = أُزيل «{ $title }» من المشروع.
resources-removed-many-toast = { $count ->
    [two] أُزيل موردان من المشروع.
    [few] أُزيلت { $count } موارد من المشروع.
   *[other] أُزيل { $count } موردًا من المشروع.
}
resources-detached-toast = فُصل «{ $title }».
resources-attached-one-toast = أُضيف «{ $title }» إلى موارد «{ $item }».
resources-attached-many-toast = { $count ->
    [two] أُضيف موردان إلى «{ $item }».
    [few] أُضيفت { $count } موارد إلى «{ $item }».
   *[other] أُضيف { $count } موردًا إلى «{ $item }».
}

## في النص

resources-no-text = ضع المؤشر في نص أولًا، ثم أدرج المورد هناك.
resources-drop-insert-picture = إدراج الصورة
resources-drop-insert-link = إدراج رابط إلى المورد

## التقدم

progress-resources-storing = جارٍ نسخ الملفات
progress-resources-stored = نُسخت الملفات

## صفحة الموارد لمجلد

segment-folder = مجلد
resources-folder-count = { $count ->
    [zero] لا موارد
    [one] مورد واحد
    [two] موردان
    [few] { $count } موارد
    [many] { $count } موردًا
   *[other] { $count } مورد
}
resources-view-label = عرض الموارد
resources-view-list = قائمةً
resources-view-grid = شبكةً
resources-grid-label = الموارد
resources-folder-drop-hint = أفلت هنا ملفات أو عناوين ويب لإرفاقها بهذا المجلد
resources-folder-empty = لم يُرفق شيء بعد بهذا المجلد ولا بما يحتويه. أضف ملفات أو روابط ويب بالأزرار في الأعلى، أو أفلتها هنا: ستُرفق بالمجلد.
resources-attached-to = على
resources-attached-separator = ،
resources-attached-more = { $count ->
    [zero] ولا شيء آخر
    [one] وواحد آخر
    [two] واثنان آخران
    [few] و{ $count } أخرى
    [many] و{ $count } أخرى
   *[other] و{ $count } أخرى
}
resources-row-name-attached = { $name }، على { $items }
resources-open-item = فتح «{ $title }»
resources-detach-from = فصل عن «{ $title }»

## موارد الصور في المعرض

gallery-resource-badge = مورد
gallery-tile-resource = مورد صورة لـ { $title }: { $description }

## صفحة الموارد في إعدادات المشروع

settings-page-resources = الموارد
settings-desc-resources = الملفات وروابط الويب التي لم يعد أي عنصر يحملها: أرفقها من جديد، أو أزلها، أو صدّر نسخًا منها
settings-resources-description = لا يكون المورد مرفقًا بأي شيء بعد فصله عن آخر عنصر له، أو بعد حذف العنصر الذي كان يحمله نهائيًا. يبقى في المشروع، ويرافق الملف المخزَّن كل نسخة وكل نسخة احتياطية من المشروع، إلى أن ترفقه من جديد أو تزيله.
settings-resources-group-storage = التخزين
settings-resources-group-unattached = غير مرفقة بأي شيء
settings-resources-stored = الملفات المخزَّنة في هذا المشروع: { $size }
settings-resources-folder-advice = المشروع المحفوظ كملف واحد يُعاد كتابته كاملًا عند كل حفظ، مع ملفاته المخزَّنة. أما المحفوظ كمجلد فلا يحفظ إلا ما تغيّر، وهذا أسرع.
settings-resources-count = { $count ->
    [zero] لا موارد
    [one] مورد واحد
    [two] موردان
    [few] { $count } موارد
    [many] { $count } موردًا
   *[other] { $count } مورد
}
settings-resources-empty = كل مورد في هذا المشروع مرفق بعنصر ما.
resources-attach-to-items = إرفاق بعناصر…
resources-export-copies = تصدير نسخ…
resources-export-copy = تصدير نسخة…
resources-remove-many-title = هل تريد إزالة الموارد ({ $count }) من المشروع؟
resources-attach-title = إرفاق بعناصر
resources-attach-hint = اختر العناصر التي تريد الإرفاق بها. Ctrl+نقرة أو Shift+نقرة يختار عدة عناصر.
resources-attach-empty = لا يحتوي هذا المشروع على عناصر بعد.
resources-attach-confirm = إرفاق
resources-attached = { $count ->
    [zero] لم يُرفق أي مورد
    [one] أُرفق مورد واحد
    [two] أُرفق موردان
    [few] أُرفقت { $count } موارد
    [many] أُرفق { $count } موردًا
   *[other] أُرفق { $count } مورد
} { $items ->
    [zero] بلا عناصر
    [one] بعنصر واحد
    [two] بعنصرين
    [few] بـ{ $items } عناصر
    [many] بـ{ $items } عنصرًا
   *[other] بـ{ $items } عنصر
}
resources-export-title = تصدير نسخ إلى مجلد
resources-exported = { $count ->
    [zero] لم يُصدَّر أي مورد
    [one] صُدِّر مورد واحد
    [two] صُدِّر موردان
    [few] صُدِّرت { $count } موارد
    [many] صُدِّر { $count } موردًا
   *[other] صُدِّر { $count } مورد
}
resources-export-missing = { $count ->
    [zero] لا ملف مفقود: { $names }
    [one] ملف واحد ليس حيث يتوقعه المشروع، فتُرك: { $names }
    [two] ملفان ليسا حيث يتوقعهما المشروع، فتُركا: { $names }
    [few] { $count } ملفات ليست حيث يتوقعها المشروع، فتُركت: { $names }
    [many] { $count } ملفًا ليست حيث يتوقعها المشروع، فتُركت: { $names }
   *[other] { $count } ملف ليست حيث يتوقعها المشروع، فتُركت: { $names }
}
resources-export-failed = { $count ->
    [zero] لم يفشل نسخ أي ملف: { $names }
    [one] تعذّر نسخ ملف واحد: { $names }
    [two] تعذّر نسخ ملفين: { $names }
    [few] تعذّر نسخ { $count } ملفات: { $names }
    [many] تعذّر نسخ { $count } ملفًا: { $names }
   *[other] تعذّر نسخ { $count } ملف: { $names }
}

## تفضيل الملفات الكبيرة (المحرر ▸ الصور والملفات)

settings-resources-large-file = الملفات الأكبر من 10 ميغابايت
settings-resources-large-ask = السؤال في كل مرة
settings-resources-large-store = التخزين دائمًا في المشروع
settings-resources-large-link = الربط دائمًا بالملف
settings-hint-resources-large-file = ما يحدث لملف أكبر من 10 ميغابايت ترفقه كمورد. إذا خُزّن في المشروع رافق كل نسخة وكل نسخة احتياطية منه. وإذا رُبط بقي في مكانه، ولا يتذكر المشروع إلا مكان العثور عليه.
