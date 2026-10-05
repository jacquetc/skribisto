# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

settings-page-templates = القوالب
settings-templates-description = نصوص قابلة لإعادة الاستخدام يمكنك إدراجها في أي شيء تكتبه: بطاقة شخصية فارغة، ملف مكان، مخطط إيقاع. تُخزَّن في هذا المشروع، فيحصل كل من يفتحه على المجموعة نفسها.
settings-templates-filter = تصفية القوالب
settings-templates-count = { $n ->
    [0] لا قوالب
    [zero] لا قوالب
    [one] قالب واحد
    [two] قالبان
    [few] { $n } قوالب
    [many] { $n } قالبًا
   *[other] { $n } قالب
}
settings-templates-empty = لا قوالب في هذا المشروع بعد.
settings-templates-empty-hint = أضف قالبًا من إعداد مسبق، أو استورد ملف ‎.md أو ‎.djot، أو اكتب شيئًا واختر المستند ▸ حفظ كقالب.
settings-templates-presets = إضافة إعداد مسبق
settings-templates-import = استيراد…
settings-templates-export = تصدير…
settings-templates-star = الإظهار أولًا في قائمة الإدراج
settings-templates-unstar = إيقاف الإظهار أولًا
settings-templates-move-up = نقل لأعلى
settings-templates-move-down = نقل لأسفل
settings-templates-delete = حذف القالب
settings-templates-name = الاسم
settings-templates-name-placeholder = اسم القالب
settings-templates-body-placeholder = النص الذي يدرجه هذا القالب
settings-templates-duplicate-name = يوجد قالب آخر باسم «{ $name }»
settings-templates-words = { $n ->
    [zero] لا كلمات
    [one] كلمة واحدة
    [two] كلمتان
    [few] { $n } كلمات
    [many] { $n } كلمة
   *[other] { $n } كلمة
}
templates-imported = { $added ->
    [zero] لم يُستورد أي قالب
    [one] استُورد قالب واحد
    [two] استُورد قالبان
    [few] استُوردت { $added } قوالب
    [many] استُورد { $added } قالبًا
   *[other] استُورد { $added } قالب
}
templates-imported-renamed = أُعيدت تسميتها لتجنّب التعارض: { $names }
templates-imported-skipped = تعذّرت قراءة: { $names }
templates-import-filter = القوالب (‎.md و‎.djot)
templates-exported = { $n ->
    [zero] لم يُصدَّر أي قالب
    [one] صُدّر قالب واحد
    [two] صُدّر قالبان
    [few] صُدّرت { $n } قوالب
    [many] صُدّر { $n } قالبًا
   *[other] صُدّر { $n } قالب
}
templates-preset-applied = أُضيف «{ $name }»
templates-delete-title = حذف هذا القالب؟
templates-delete-body = سيُزال «{ $name }» من هذا المشروع. لا تتأثر الملاحظات التي أنشأتها منه سابقًا.
templates-delete-confirm = حذف
menu-document = المستند (&D)
menu-insert-template = إدراج قالب (&L)
menu-insert-template-none = لا قوالب في هذا المشروع
menu-save-as-template = حفظ كقالب… (&V)
save-as-template-title = الحفظ كقالب
save-as-template-explain = يصبح النص الذي تحرره قالبًا يمكنك إدراجه في أي مكان آخر.
save-as-template-name = الاسم
save-as-template-placeholder = بطاقة شخصية
save-as-template-duplicate = يوجد قالب باسم «{ $name }» بالفعل
save-as-template-empty-editor = هذا المحرر فارغ. لا يوجد ما يُحفظ.
save-as-template-confirm = حفظ القالب
save-as-template-saved = حُفظ «{ $name }» كقالب
template-inserted = أُدرج «{ $name }»
note-template-preset-character-sheet = بطاقة شخصية
note-template-preset-location = مكان
note-template-preset-artifact = غرض
note-template-preset-beat-sheet = مخطط الإيقاع
note-template-preset-faction = فصيل
note-template-preset-research-note = ملاحظة بحثية
note-template-set-essentials = الأساسيات
note-template-set-everything = كل القوالب
note-template-section-identity = الهوية
note-template-section-appearance = المظهر
note-template-section-voice = الصوت
note-template-section-psychology = الحياة الداخلية
note-template-section-history = التاريخ
note-template-section-arc = المسار
note-template-section-first-impression = الانطباع الأول
note-template-section-in-the-story = في القصة
note-template-section-the-thing-itself = الغرض نفسه
note-template-section-the-scene = المشهد
note-template-section-the-shape = الشكل
note-template-section-what-it-is = ما هو
note-template-section-source = المصدر
note-template-section-what-it-says = ما يقوله
note-template-field-full-name = الاسم الكامل
note-template-field-known-as = يُعرف أيضًا باسم، ويُتابَع في حقل الأسماء البديلة في لوحة الخصائص
note-template-field-age = العمر
note-template-field-role-in-story = الدور في القصة
note-template-field-build-and-features = البنية والسمات المميزة
note-template-field-habitual-bearing = الهيئة المعتادة
note-template-field-speech-patterns = أنماط الكلام واللوازم اللفظية
note-template-field-what-they-never-say = ما لا يقوله أبدًا
note-template-field-want = ما يريده
note-template-field-need = ما يحتاج إليه دون أن يدري
note-template-field-fear = ما يخشاه
note-template-field-flaw = العيب الذي يكلّفه
note-template-field-formative-event = الحدث المؤسِّس
note-template-field-relationships = العلاقات
note-template-field-starts-as = يبدأ بوصفه
note-template-field-ends-as = ينتهي بوصفه
note-template-field-what-you-notice-first = أول ما تلاحظه
note-template-field-sound-and-smell = الصوت والرائحة
note-template-field-light-and-weather = الضوء والطقس
note-template-field-what-happened-here = ما حدث هنا
note-template-field-who-lives-or-works-here = من يعيش أو يعمل هنا
note-template-field-scenes-set-here = المشاهد التي تجري هنا
note-template-field-why-it-matters = لماذا يهم
note-template-field-appearance = المظهر
note-template-field-age-and-origin = العمر والأصل
note-template-field-what-it-does = ما يفعله
note-template-field-who-holds-it = من يملكه
note-template-field-who-wants-it = من يريده
note-template-field-pov = وجهة النظر
note-template-field-time-and-place = الزمان والمكان
note-template-field-goal = الهدف
note-template-field-conflict = الصراع
note-template-field-turn = التحوّل
note-template-field-exit-emotion = الشعور عند الخروج
note-template-field-purpose = الغاية
note-template-field-who-leads-it = من يقوده
note-template-field-resources = الموارد
note-template-field-allies-and-enemies = الحلفاء والأعداء
note-template-field-what-it-wants-now = ما يريده الآن
note-template-field-where-from = من أين أتى
note-template-field-page-or-link = الصفحة أو الرابط
note-template-field-key-facts = الحقائق الأساسية
note-template-field-how-it-is-used = كيف يُستخدم في القصة
