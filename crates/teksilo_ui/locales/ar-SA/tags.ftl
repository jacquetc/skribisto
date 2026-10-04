# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

tags-preset-basic = أساسي
tags-preset-scifi = الخيال العلمي
tags-preset-fantasy = فانتازيا
tags-preset-mystery = الغامض
tags-preset-historical = تاريخي


tags-preset-needs-research = الاحتياجات من البحوث
tags-preset-continuity-check = التحقق من الاستمرارية
tags-preset-plot-point = نقطة النهاية

tags-preset-character = الشخصية
tags-preset-place = المكان
tags-preset-item = البند

tags-preset-vessel = وعاء
tags-preset-planet = كوكب الأرض
tags-preset-organization = المنظمة
tags-preset-creature = المخلوقات
tags-preset-faction = فصيل
tags-preset-artifact = القطع الأثرية
tags-preset-realm = العالم
tags-preset-magic-system = نظام سحري
tags-preset-suspect = مشتبه به
tags-preset-victim = الضحية
tags-preset-clue = دليل
tags-preset-red-herring = الرنجة الحمراء
tags-preset-historical-figure = الشكل التاريخي
tags-preset-source = المصدر
tags-preset-period-detail = الفترة

tags-pill-list = الوسوم
tags-pill-add = أضف علامة
tags-none-yet = لا علامات
tags-pick-list = تاغز للاختيار من
tags-pill-remove = إزالة { $name }
tags-pill-filter-placeholder = فيلم أو اسم بطاقة جديدة
tags-pill-no-match = لا تطابق للعلامات
tags-pill-create = إنشاء "{ $name }"
tags-pill-create-failed = لا يمكن خلق "{ $name }"
tags-pill-new-discoverable = معدّل جدّي
tags-pill-new-discoverable-hint = الأشياء مع هذه العلامة متطابقة مع موكّلك لملء القائمة.

tags-alias-list = المعروف أيضا
tags-alias-add = إضافة اسم آخر
tags-alias-remove = إزالة { $name }
tags-alias-placeholder = اسم آخر، ثم Enter
tags-alias-hint = الاسماء التي تظهر تحت مظهرك، بجانب عنوانه.
tags-alias-collision = { $name } بالفعل إجابات على هذا الاسم.

settings-page-tags = الوسوم
settings-tags-desc = تاجز) يصف الأشياء في مقبضك) وعلامة البحث في العمليّة تقول أيضاً Skribisto للبحث عن أسماء هذا البند في ملفك
settings-tags-add = مضافا إليه
settings-tags-add-placeholder = اسم بطاقة جديدة
settings-tags-added = مضاف "{ $name }"
settings-tags-duplicate = "{ $name }" موجود بالفعل
settings-tags-filter = بطاقات المصورين
settings-tags-count = { $n ->
    [one] بطاقة واحدة
   *[other] { $n } العلامة
}
settings-tags-details-placeholder = ماذا تعني هذه العلامة؟
settings-tags-discoverable = اعثروا على اثارة
settings-tags-creates-in = ملاحظات جديدة تذهب
settings-tags-creates-in-unset = إسألني المرة الأولى
settings-tags-creates-in-untitled = ملف غير مسموع
settings-tags-creates-in-trashed = { $name } (في القمامة)
settings-tags-template = نموذج البداية
settings-tags-template-unset = ملاحظة فارغة
settings-tags-delete = تحذف { $name }
settings-tags-deleted = حُذف "{ $name }" وأُزيل من جميع العناصر
settings-tags-empty = لا علامات حتى الآن.
settings-tags-apply-preset = تطبيق ما قبل...
settings-tags-preset-applied = مضافا إليه: { $added }، متجاوزا رقم { $skipped } الموجود بالفعل
settings-tags-csv-filter = ملفات CSV
settings-tags-import = استيراد…
settings-tags-export = تصدير…
settings-tags-imported = Imported { $added }, jumped { $skipped }
settings-tags-exported = المصدرة { $n ->
    [one] بطاقة واحدة
   *[other] { $n } العلامة
}

tags-chip-more = { $n ->
    [one] بطاقة واحدة أخرى
   *[other] { $n } علامة أخرى
}

cast-section = الشخصيات
cast-add = أضف إلى الطرح...
cast-add-filter-placeholder = قصّةِ مُقَرَّرةِ الإنجيلِ
cast-add-empty = لا توجد قيود قابلة للقراءة
cast-pin = إضافة { $name } إلى الشخصيات
cast-unpin = إزالة { $name } من الشخصيات
cast-empty = لم يعلق أحد بعد أضف أو أحتفظ بإقتراح
cast-unresolved = لم يعد في الكتاب المقدس

mentions-confirm = مضافا إليها { $name }
mentions-hit-count = { $n ->
    [one] مرة واحدة
   *[other] { $n } مرة
}
mentions-point-of-view-badge = وجهة النظر
mentions-point-of-view-badge-tooltip = ذُكِرَ كنقطة نظر هنا، مُحدّد باليد، لَمْ يُكتشفَ في المحترفِ.

mentions-roster = الشخصيات
mentions-pin = إضافة { $name } إلى الشخصيات

pov-section = وجهة النظر
pov-empty = لم تُحدد وجهة نظر
pov-add = وضع وجهة نظر...
pov-multiple = هذا المشهد له وجهة نظر
pov-remove = إزالة { $name } من وجهات النظر
pov-unresolved = وجهة نظر مثبتة هنا، ولكن هذا الدخول لم يعد موجوداً أو خسر بطاقة قصته.

books-section = مأخوذة تحت
books-empty = لم تُقدّم تحت كتاب بعد
books-add = اطير تحت كتاب...
books-remove = نقل من: { $name }
books-apply-to-children = تقديم الطلبات إلى الأطفال

note-details-name-placeholder = الاسم…
note-details-tags = الوسوم
note-details-aliases = أسماء أخرى
note-details-books = مأخوذة تحت
note-details-books-empty = لم تُقدّم بموجب كتاب بعد
note-details-links = الروابط
note-details-links-empty = لا وصلات بعد أضف واحداً إلى نقطة أخرى في كتابك المقدس
note-details-links-add = وصلة إلى ملاحظة...
note-details-pov = وجهة النظر
note-details-pov-empty = لم تُحدد وجهة نظر
note-details-pov-unresolved = وجهة نظر مثبتة هنا، ولكن هذا الدخول لم يعد موجوداً أو خسر بطاقة قصته.
note-details-pov-multiple = ولهذه المذكرة وجهة نظر.
note-details-backlinks = تظهر في المخطوطة
note-details-backlinks-empty = لا شيء بعد، بمجرد أن يظهر هذا الاسم في ملفك، سيظهر هنا.
note-details-backlinks-outside = خارج الكتب
note-details-backlinks-confirm-all = تأكد من كل مظهر
note-details-backlinks-confirm-all-tooltip = يضاف هذا القيد إلى مجموعة كل وثيقة موضحة هنا لم تؤكده بعد
note-details-untitled-document = غير مترجم

note-in-prose-mentions = { $n ->
    [one] ذكر واحد
   *[other] { $n } mentions
}
note-in-prose-mention-at = { $current } من { $total }
note-in-prose-mention-previous = ذكر سابق
note-in-prose-mention-next = التذكير التالي
