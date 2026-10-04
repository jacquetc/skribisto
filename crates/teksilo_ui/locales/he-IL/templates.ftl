# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

settings-page-templates = תבניות
settings-templates-description = פיסות כתיבה שניתן להוריד לתוך כל מה שאתה כותב: גיליון אופי ריק, פרופיל מיקום, גיליון מנצח.הם מאוחסנים בפרויקט זה, כך שכל מי שפותח אותו מקבל את אותה קבוצה.
settings-templates-filter = תבניות מסנן
settings-templates-count = { $n ->
    [0] אין תבניות
    [one] 1 תבנית
   *[other] { $n } תבניות
}
settings-templates-empty = לפרוייקט הזה עדיין אין תבניות.
settings-templates-empty-hint = הוסף אחד מראש, לייבא .md או .djot קובץ, או לכתוב משהו ולבחור מסמך קידוד שמור כתבנית.
settings-templates-presets = הוסף preset
settings-templates-import = ייבוא…
settings-templates-export = ייצוא…
settings-templates-star = הצג ראשון בתפריט הכנס
settings-templates-unstar = תפסיקו להראות קודם
settings-templates-move-up = הזזה למעלה
settings-templates-move-down = הזזה למטה
settings-templates-delete = המונחים:
settings-templates-name = שם
settings-templates-name-placeholder = שם תבנית
settings-templates-body-placeholder = הטקסט הזה מוסיף
settings-templates-duplicate-name = תבנית נוספת נקראת "{ $name }"
settings-templates-words = { $n ->
    [one] 1 מילה
   *[other] { $n } מילים
}

templates-imported = { $added ->
    [one] 1 תבנית מיובא
   *[other] { $added } תבניות מיובאות
}
templates-imported-renamed = נקרא להימנע מעימות: { $names }
templates-imported-skipped = לא ניתן לקרוא: { $names }
templates-import-filter = תבניות (.md, .djot)
templates-exported = { $n ->
    [one] 1 תבנית מייצא
   *[other] { $n } תבניות ייצוא
}
templates-preset-applied = { $name }

templates-delete-title = למחוק את התבנית הזו?
templates-delete-body = "{ $name }" יוסר מהפרויקט הזה.הערות שיצרת ממנו כבר אינן מושפעות.
templates-delete-confirm = מחיקה

menu-document = מסמך (&D)
menu-insert-template = המונחים תבנית (&L)
menu-insert-template-none = אין תבניות בפרויקט הזה
menu-save-as-template = שמור כתבנית... (&V)

save-as-template-title = שמור כתבנית
save-as-template-explain = הטקסט שאתה עורך הופך לתבנית שאתה יכול ליפול בכל מקום אחר.
save-as-template-name = שם
save-as-template-placeholder = תגית:
save-as-template-duplicate = תבנית הנקראת "{ $name }" כבר קיימת
save-as-template-empty-editor = עורך זה ריק ואין מה להציל.
save-as-template-confirm = שמור תבנית
save-as-template-saved = שמור "{ $name }" כתבנית
template-inserted = שם הסרטון:{ $name }

note-template-preset-character-sheet = תגית:
note-template-preset-location = מיקום Location
note-template-preset-artifact = אובייקטים
note-template-preset-beat-sheet = Beat גיליון
note-template-preset-faction = FIT
note-template-preset-research-note = מחקר Note

note-template-set-essentials = יסודות
note-template-set-everything = כל תבנית

note-template-section-identity = זהות זהות
note-template-section-appearance = מראה
note-template-section-voice = קול קול
note-template-section-psychology = החיים הפנימיים
note-template-section-history = היסטוריה
note-template-section-arc = קשת Arc
note-template-section-first-impression = הרושם הראשוני
note-template-section-in-the-story = בסיפור
note-template-section-the-thing-itself = האובייקט עצמו
note-template-section-the-scene = הסצנה
note-template-section-the-shape = הצורה
note-template-section-what-it-is = מה זה
note-template-section-source = מקור Source
note-template-section-what-it-says = מה זה אומר

note-template-field-full-name = שם מלא
note-template-field-known-as = ידוע גם בשם The Inspector's Alias
note-template-field-age = גיל גיל גיל הגיל
note-template-field-role-in-story = תפקיד בסיפור
note-template-field-build-and-features = בניית וזיהוי תכונות
note-template-field-habitual-bearing = המונחים:
note-template-field-speech-patterns = דפוסי דיבור וטכנולוגיות מילוליות
note-template-field-what-they-never-say = מה הם אף פעם לא אומרים
note-template-field-want = מה הם רוצים
note-template-field-need = מה הם צריכים אבל לא יודעים
note-template-field-fear = מה הם מפחדים
note-template-field-flaw = הפגמים שעולים להם
note-template-field-formative-event = אירוע פורמלי
note-template-field-relationships = יחסים
note-template-field-starts-as = מתחילים כמו
note-template-field-ends-as = הסוףים כמו
note-template-field-what-you-notice-first = מה אתה מבחין קודם
note-template-field-sound-and-smell = סאונד וריח
note-template-field-light-and-weather = אור ומזג אוויר
note-template-field-what-happened-here = מה קרה כאן
note-template-field-who-lives-or-works-here = מי גר כאן או פרויקט
note-template-field-scenes-set-here = סצנות כאן
note-template-field-why-it-matters = למה זה משנה
note-template-field-appearance = מראה
note-template-field-age-and-origin = גיל ומקור
note-template-field-what-it-does = מה זה עושה
note-template-field-who-holds-it = מי מחזיק אותו
note-template-field-who-wants-it = מי רוצה
note-template-field-pov = נקודת מבט
note-template-field-time-and-place = זמן ומקום
note-template-field-goal = יעד
note-template-field-conflict = סכסוך
note-template-field-turn = The Turn
note-template-field-exit-emotion = רגשות בדרך החוצה
note-template-field-purpose = מטרה
note-template-field-who-leads-it = מי מוביל אותו
note-template-field-resources = משאבים
note-template-field-allies-and-enemies = בעלות ברית ואויבים
note-template-field-what-it-wants-now = מה שהוא רוצה עכשיו
note-template-field-where-from = מהיכן הגיע
note-template-field-page-or-link = עמוד או קישור
note-template-field-key-facts = עובדות מפתח
note-template-field-how-it-is-used = איך זה נעשה שימוש בסיפור
