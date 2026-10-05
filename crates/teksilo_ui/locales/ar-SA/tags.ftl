# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

tags-preset-basic = أساسية
tags-preset-scifi = خيال علمي
tags-preset-fantasy = فانتازيا
tags-preset-mystery = غموض
tags-preset-historical = تاريخية
tags-preset-needs-research = يحتاج إلى بحث
tags-preset-continuity-check = مراجعة الاتساق
tags-preset-plot-point = نقطة حبكة
tags-preset-character = شخصية
tags-preset-place = مكان
tags-preset-item = غرض
tags-preset-vessel = مركبة
tags-preset-planet = كوكب
tags-preset-organization = منظمة
tags-preset-creature = مخلوق
tags-preset-faction = فصيل
tags-preset-artifact = قطعة أثرية
tags-preset-realm = مملكة
tags-preset-magic-system = نظام سحري
tags-preset-suspect = مشتبه به
tags-preset-victim = ضحية
tags-preset-clue = دليل
tags-preset-red-herring = دليل مضلِّل
tags-preset-historical-figure = شخصية تاريخية
tags-preset-source = مصدر
tags-preset-period-detail = تفصيل من الحقبة
tags-pill-list = الوسوم
tags-pill-add = إضافة وسم
tags-none-yet = بلا وسوم
tags-pick-list = الوسوم المتاحة للاختيار
tags-pill-remove = إزالة { $name }
tags-pill-filter-placeholder = صفِّ الوسوم أو سمِّ وسمًا جديدًا
tags-pill-no-match = لا يوجد وسم مطابق
tags-pill-create = إنشاء «{ $name }»
tags-pill-create-failed = تعذّر إنشاء «{ $name }»
tags-pill-new-discoverable = وسم بحث في النص
tags-pill-new-discoverable-hint = العناصر التي تحمل هذا الوسم تُطابَق مع نصك لملء قائمة الحاضرين.
tags-alias-list = يُعرف أيضًا باسم
tags-alias-add = إضافة اسم آخر
tags-alias-remove = إزالة { $name }
tags-alias-placeholder = اسم آخر، ثم Enter
tags-alias-hint = الأسماء التي يظهر بها هذا في نصك، إلى جانب عنوانه.
tags-alias-collision = { $name } يُعرف بهذا الاسم بالفعل.
settings-page-tags = الوسوم
settings-tags-desc = تميّز الوسوم العناصر في حافظتك. ووسم البحث في النص يطلب أيضًا من Skribisto البحث عن أسماء ذلك العنصر في نصك.
settings-tags-add = إضافة وسم
settings-tags-add-placeholder = اسم الوسم الجديد
settings-tags-added = أُضيف «{ $name }»
settings-tags-duplicate = «{ $name }» موجود بالفعل
settings-tags-filter = تصفية الوسوم
settings-tags-count = { $n ->
    [zero] لا وسوم
    [one] وسم واحد
    [two] وسمان
    [few] { $n } وسوم
    [many] { $n } وسمًا
   *[other] { $n } وسم
}
settings-tags-details-placeholder = ما يعنيه هذا الوسم
settings-tags-discoverable = البحث في النص
settings-tags-creates-in = توضع الملاحظات الجديدة في
settings-tags-creates-in-unset = السؤال في المرة الأولى
settings-tags-creates-in-untitled = مجلد بلا عنوان
settings-tags-creates-in-trashed = { $name } (في سلة المهملات)
settings-tags-template = القالب الأولي
settings-tags-template-unset = ملاحظة فارغة
settings-tags-delete = حذف { $name }
settings-tags-deleted = حُذف «{ $name }» وأُزيل من كل العناصر
settings-tags-empty = لا وسوم بعد.
settings-tags-apply-preset = تطبيق إعداد مسبق…
settings-tags-preset-applied = المضافة: { $added }، المتخطاة لوجودها مسبقًا: { $skipped }
settings-tags-csv-filter = ملفات CSV
settings-tags-import = استيراد…
settings-tags-export = تصدير…
settings-tags-imported = المستوردة: { $added }، المتخطاة: { $skipped }
settings-tags-exported = { $n ->
    [zero] لم يُصدَّر أي وسم
    [one] صُدّر وسم واحد
    [two] صُدّر وسمان
    [few] صُدّرت { $n } وسوم
    [many] صُدّر { $n } وسمًا
   *[other] صُدّر { $n } وسم
}
tags-chip-more = { $n ->
    [zero] لا وسوم أخرى
    [one] وسم آخر
    [two] وسمان آخران
    [few] { $n } وسوم أخرى
    [many] { $n } وسمًا آخر
   *[other] { $n } وسم آخر
}
cast-section = الحاضرون
cast-add = إضافة إلى الحاضرين…
cast-add-filter-placeholder = تصفية مرجع القصة…
cast-add-empty = لا مدخلات في مرجع القصة لإضافتها
cast-pin = إضافة { $name } إلى الحاضرين
cast-unpin = إزالة { $name } من الحاضرين
cast-empty = لم يُثبَّت أحد بعد. أضف أحدًا أو احتفظ باقتراح
cast-unresolved = لم يعد في مرجع القصة
mentions-confirm = إضافة إلى الحاضرين في { $name }
mentions-hit-count = { $n ->
    [zero] ولا مرة
    [one] مرة واحدة
    [two] مرتان
    [few] { $n } مرات
    [many] { $n } مرة
   *[other] { $n } مرة
}
mentions-point-of-view-badge = وجهة النظر
mentions-point-of-view-badge-tooltip = مُعلَن كوجهة النظر هنا، حُدّد يدويًا ولم يُكتشف في النص.
mentions-roster = الحاضرون
mentions-pin = إضافة { $name } إلى الحاضرين
pov-section = وجهة النظر
pov-empty = لم تُحدَّد وجهة نظر
pov-add = تحديد وجهة النظر…
pov-multiple = لهذا المشهد وجهتا نظر.
pov-remove = إزالة { $name } كوجهة نظر
pov-unresolved = ثُبّتت هنا وجهة نظر، لكن ذلك المدخل لم يعد موجودًا أو فقد وسم مرجع القصة.
books-section = مدرج ضمن
books-empty = لم يُدرج ضمن أي كتاب بعد
books-add = الإدراج ضمن كتاب…
books-remove = الإزالة من { $name }
books-apply-to-children = تطبيق الإدراج على العناصر الفرعية
note-details-name-placeholder = الاسم…
note-details-tags = الوسوم
note-details-aliases = أسماء أخرى
note-details-books = مدرج ضمن
note-details-books-empty = لم يُدرج ضمن أي كتاب بعد
note-details-links = الروابط
note-details-links-empty = لا روابط بعد. أضف رابطًا يشير إلى مدخل آخر في مرجع قصتك.
note-details-links-add = ربط بملاحظة…
note-details-pov = وجهة النظر
note-details-pov-empty = لم تُحدَّد وجهة نظر
note-details-pov-unresolved = ثُبّتت هنا وجهة نظر، لكن ذلك المدخل لم يعد موجودًا أو فقد وسم مرجع القصة.
note-details-pov-multiple = لهذه الملاحظة وجهتا نظر.
note-details-backlinks = يظهر في المخطوطة
note-details-backlinks-empty = لا شيء بعد. حين يظهر هذا الاسم في نصك، سيظهر هنا.
note-details-backlinks-outside = خارج الكتب
note-details-backlinks-confirm-all = تأكيد كل الظهورات
note-details-backlinks-confirm-all-tooltip = إضافة هذا المدخل إلى الحاضرين في كل مستند معروض هنا لم يؤكده بعد
note-details-untitled-document = بلا عنوان
note-in-prose-mentions = { $n ->
    [zero] لا إشارات
    [one] إشارة واحدة
    [two] إشارتان
    [few] { $n } إشارات
    [many] { $n } إشارة
   *[other] { $n } إشارة
}
note-in-prose-mention-at = { $current } من { $total }
note-in-prose-mention-previous = الإشارة السابقة
note-in-prose-mention-next = الإشارة التالية
