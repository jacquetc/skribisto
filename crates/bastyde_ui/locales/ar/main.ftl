# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Skribisto: واجهة المستخدم (العربية).
#
# PARTIAL LOCALE — NEEDS REVIEW BY A NATIVE SPEAKER.
#
# This file covers the menus, the welcome window and the block-formatting
# controls; every key it does not define falls back to en-US per key, which
# is how Fluent resolves a missing message. That is deliberate rather than
# unfinished: registering `ar` at all is what lets a writer *select* Arabic,
# and selecting it is what activates bastyde's RTL chrome mirroring — the
# whole point of the locale as far as right-to-left editing is concerned.
#
# There is deliberately no `ar/tooltips.ftl` or `ar/tags.ftl`. Those cover
# the writing-model vocabulary and the preset tag names, which this locale
# does not attempt; empty files registered alongside main.ftl would be
# stubs that look like coverage. Add them when the terms are translated.
#
# The writing-craft vocabulary in particular (binder, corkboard, synopsis,
# beat) was translated without a native reviewer and should be checked
# before this locale is advertised as supported.
#
# Mnemonics: the `&` belongs in the value, as in the other locales. Arabic
# has no established Alt-mnemonic convention, so they are omitted here
# rather than guessed — a missing mnemonic degrades gracefully.

menu-work = المشروع
menu-new-work = مشروع جديد
menu-open-work = فتح مشروع…
menu-import-from = استيراد من
menu-import-plume = ‏Plume Creator ‏(.plume)…
menu-export = تصدير
menu-export-book = تصدير الكتاب
menu-export-part = تصدير الجزء
menu-export-chapter = تصدير الفصل
menu-export-scene = تصدير المشهد
menu-export-note = تصدير الملاحظة
menu-export-folder = تصدير المجلد
menu-export-choose = اختيار…
menu-export-none = افتح مستندًا لتصديره
menu-save = حفظ
menu-save-as-file = حفظ كملف واحد…
menu-save-as-folder = حفظ كمجلد…
menu-backup = نسخ احتياطي الآن
menu-close-work = إغلاق المشروع
menu-welcome = أهلًا بك…
menu-settings = الإعدادات
menu-quit = إنهاء

menu-view = عرض
menu-outline = المخطط
menu-search = البحث في المشروع

# Paragraph direction — the control this locale exists to make reachable.
menu-format-direction = اتجاه النص
menu-format-direction-auto = تلقائي
menu-format-direction-ltr = من اليسار إلى اليمين
menu-format-direction-rtl = من اليمين إلى اليسار

format-direction-rtl = فقرة من اليمين إلى اليسار
format-align-left = محاذاة إلى اليسار
format-align-center = توسيط
format-blockquote = اقتباس
format-list-bullet = قائمة نقطية
format-list-numbered = قائمة مرقّمة
format-indent = زيادة الإزاحة
format-outdent = إنقاص الإزاحة

welcome-title = أهلًا بك في Skribisto
welcome-search = البحث في المشاريع
welcome-open = فتح
welcome-new-work = مشروع جديد
welcome-recent-works = المشاريع الأخيرة
welcome-empty-recents = لا توجد مشاريع حديثة بعد.
welcome-no-matches = لا يطابق أي مشروع حديث بحثك.
