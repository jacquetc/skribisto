# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

settings-page-templates = القوالب
settings-templates-description = أجزاء الكتابة القابلة للإستعمال يمكن أن تسقط إلى أي شيء تكتبه، ورقة شخصية فارغة، ملف موقعي، ورقة ضرب، مخزنة في هذا المشروع
settings-templates-filter = نماذج الملفات
settings-templates-count = { $n ->
    [0] لا نماذج
    [one] نموذج واحد
   *[other] النموذجان { $n }
}
settings-templates-empty = لا توجد قوالب في هذا المشروع بعد.
settings-templates-empty-hint = يضاف ملف من ملف سابق، أو يستورد رقم .md أو .djot، أو يكتب شيئاً ويختار الوثيقة ◂ يحفظ كنموذج.
settings-templates-presets = أضف مسبقاً
settings-templates-import = استيراد…
settings-templates-export = تصدير…
settings-templates-star = عرض أولاً في قائمة الطعام
settings-templates-unstar = توقف عن الظهور أولاً
settings-templates-move-up = نقل لأعلى
settings-templates-move-down = نقل لأسفل
settings-templates-delete = يحذف النموذج
settings-templates-name = الاسم
settings-templates-name-placeholder = اسم النموذج
settings-templates-body-placeholder = يضاف نص هذا النموذج
settings-templates-duplicate-name = نموذج آخر يسمى "{ $name }"
settings-templates-words = { $n ->
    [one] كلمة واحدة
   *[other] { $n } كلمة
}

templates-imported = { $added ->
    [one] نموذج واحد مستورد
   *[other] { $added } النماذج المستوردة
}
templates-imported-renamed = أُعيدت التسمية لتجنب التعارض: { $names }
templates-imported-skipped = لا يمكن أن يكون نصها: { $names }
templates-import-filter = النماذج (.md، .djot)
templates-exported = { $n ->
    [one] نموذج واحد مصدّر
   *[other] { $n } نموذج مصدّر
}
templates-preset-applied = مضاف "{ $name }"

templates-delete-title = تحذف هذا النموذج؟
templates-delete-body = "{ $name }" سوف تُحذف من هذا المشروع تلاحظ أنّك خلقت من هذا المشروع لم تتأثر
templates-delete-confirm = حذف

menu-document = الوثيقة (&D)
menu-insert-template = النموذج الخفيف (&L)
menu-insert-template-none = لا توجد قوالب في هذا المشروع
menu-save-as-template = احفظ كنموذج... (&V)

save-as-template-title = وفّر كنموذج
save-as-template-explain = النص الذي تحررينه يصبح نموذجاً يمكنكِ أن تنزليه في أي مكان آخر
save-as-template-name = الاسم
save-as-template-placeholder = صحيفة سمات
save-as-template-duplicate = نموذج يسمى "{ $name }" موجود بالفعل
save-as-template-empty-editor = هذا المحرّر فارغ، لا يوجد شيء لإنقاذه.
save-as-template-confirm = نموذج إنقاذ
save-as-template-saved = وفّر "{ $name }" كنموذج
template-inserted = أُدرج "{ $name }"

note-template-preset-character-sheet = صحيفة سمات
note-template-preset-location = الموقع
note-template-preset-artifact = اعتراض
note-template-preset-beat-sheet = إضربْ ورقةَ
note-template-preset-faction = الشكل
note-template-preset-research-note = مذكرة بحثية

note-template-set-essentials = النواحي الأساسية
note-template-set-everything = كل نموذج

note-template-section-identity = الهوية
note-template-section-appearance = المظهر
note-template-section-voice = الصوت
note-template-section-psychology = الحياة الداخلية
note-template-section-history = التاريخ
note-template-section-arc = مسار تطور الشخصية
note-template-section-first-impression = الانطباع الأول
note-template-section-in-the-story = في القصة
note-template-section-the-thing-itself = الجسم نفسه
note-template-section-the-scene = المشهد
note-template-section-the-shape = الشكل
note-template-section-what-it-is = ما هو عليه
note-template-section-source = المصدر
note-template-section-what-it-says = ماذا تقول؟

note-template-field-full-name = الاسم الكامل
note-template-field-known-as = معروف أيضاً، مُتعقب في حقل المفتش (ألياس)
note-template-field-age = العمر
note-template-field-role-in-story = دور في القصة
note-template-field-build-and-features = السمات المتعلقة بالبناء والتمييز
note-template-field-habitual-bearing = حمل هابيتي
note-template-field-speech-patterns = أنماط الكلام والتحف اللفظية
note-template-field-what-they-never-say = ما يقولونه أبداً
note-template-field-want = ما يريدونه
note-template-field-need = ما يحتاجونه لكن لا يعرفونه
note-template-field-fear = ما يخشاه
note-template-field-flaw = العيب الذي يكلفهم
note-template-field-formative-event = مناسبة نموذجية
note-template-field-relationships = العلاقات
note-template-field-starts-as = تبدأ كما
note-template-field-ends-as = تنتهي كما
note-template-field-what-you-notice-first = ما لاحظته أولاً
note-template-field-sound-and-smell = الصوت والرائحة
note-template-field-light-and-weather = النور والطقس
note-template-field-what-happened-here = ماذا حدث هنا؟
note-template-field-who-lives-or-works-here = الذين يعيشون أو مشاريع هنا
note-template-field-scenes-set-here = السيناريوهات هنا
note-template-field-why-it-matters = لماذا يهم
note-template-field-appearance = المظهر
note-template-field-age-and-origin = السن والأصل
note-template-field-what-it-does = ماذا تفعل؟
note-template-field-who-holds-it = الذي يمسكه
note-template-field-who-wants-it = من يريدها
note-template-field-pov = وجهة النظر
note-template-field-time-and-place = الوقت والمكان
note-template-field-goal = الهدف
note-template-field-conflict = الصراع
note-template-field-turn = الدور
note-template-field-exit-emotion = العاطفة في طريق الخروج
note-template-field-purpose = الغرض
note-template-field-who-leads-it = من يقودها
note-template-field-resources = الموارد
note-template-field-allies-and-enemies = الحلفاء والأعداء
note-template-field-what-it-wants-now = ما يريده الآن
note-template-field-where-from = من أين أتت؟
note-template-field-page-or-link = الصفحة أو الصلة
note-template-field-key-facts = الوقائع الرئيسية
note-template-field-how-it-is-used = كيف تستخدم في القصة
