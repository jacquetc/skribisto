# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

settings-page-templates = תבניות
settings-templates-description = קטעי כתיבה לשימוש חוזר שאפשר להוסיף לכל דבר שכותבים: גיליון דמות ריק, פרופיל של מקום, גיליון פעימות. הם נשמרים בפרויקט הזה, כך שכל מי שפותח אותו מקבל את אותה הערכה.
settings-templates-filter = סינון תבניות
settings-templates-count = { $n ->
    [0] אין תבניות
    [one] תבנית אחת
   *[other] { $n } תבניות
}
settings-templates-empty = לפרויקט הזה אין עדיין תבניות.
settings-templates-empty-hint = אפשר להוסיף תבנית מערכה מוכנה, לייבא קובץ ‎.md או ‎.djot, או לכתוב משהו ולבחור מסמך ▸ שמירה כתבנית.
settings-templates-presets = הוספת תבנית מוכנה
settings-templates-import = ייבוא…
settings-templates-export = ייצוא…
settings-templates-star = הצגה ראשונה בתפריט ההוספה
settings-templates-unstar = הפסקת ההצגה הראשונה
settings-templates-move-up = הזזה למעלה
settings-templates-move-down = הזזה למטה
settings-templates-delete = מחיקת התבנית
settings-templates-name = שם
settings-templates-name-placeholder = שם התבנית
settings-templates-body-placeholder = הטקסט שהתבנית הזו מוסיפה
settings-templates-duplicate-name = כבר יש תבנית אחרת בשם "{ $name }"
settings-templates-words = { $n ->
    [one] מילה אחת
   *[other] { $n } מילים
}

templates-imported = { $added ->
    [one] יובאה תבנית אחת
   *[other] יובאו { $added } תבניות
}
templates-imported-renamed = שמן שונה כדי למנוע התנגשות: { $names }
templates-imported-skipped = לא ניתן היה לקרוא: { $names }
templates-import-filter = תבניות (.md, .djot)
templates-exported = { $n ->
    [one] יוצאה תבנית אחת
   *[other] יוצאו { $n } תבניות
}
templates-preset-applied = "{ $name }" נוספה

templates-delete-title = למחוק את התבנית הזו?
templates-delete-body = "{ $name }" תוסר מהפרויקט הזה. פתקים שכבר נוצרו ממנה לא יושפעו.
templates-delete-confirm = מחיקה

menu-document = מסמך (&D)
menu-insert-template = הוספת תבנית (&L)
menu-insert-template-none = אין תבניות בפרויקט הזה
menu-save-as-template = שמירה כתבנית… (&V)

save-as-template-title = שמירה כתבנית
save-as-template-explain = הטקסט שבעריכה הופך לתבנית שאפשר להוסיף בכל מקום אחר.
save-as-template-name = שם
save-as-template-placeholder = גיליון דמות
save-as-template-duplicate = כבר קיימת תבנית בשם "{ $name }"
save-as-template-empty-editor = העורך הזה ריק. אין מה לשמור.
save-as-template-confirm = שמירת התבנית
save-as-template-saved = "{ $name }" נשמר כתבנית
template-inserted = "{ $name }" נוסף

note-template-preset-character-sheet = גיליון דמות
note-template-preset-location = מקום
note-template-preset-artifact = חפץ
note-template-preset-beat-sheet = גיליון פעימות
note-template-preset-faction = פלג
note-template-preset-research-note = פתק מחקר

note-template-set-essentials = הבסיס
note-template-set-everything = כל התבניות

note-template-section-identity = זהות
note-template-section-appearance = מראה
note-template-section-voice = קול
note-template-section-psychology = עולם פנימי
note-template-section-history = היסטוריה
note-template-section-arc = קשת התפתחות
note-template-section-first-impression = רושם ראשון
note-template-section-in-the-story = בסיפור
note-template-section-the-thing-itself = החפץ עצמו
note-template-section-the-scene = הסצנה
note-template-section-the-shape = המבנה
note-template-section-what-it-is = מה זה
note-template-section-source = מקור
note-template-section-what-it-says = מה כתוב בו

note-template-field-full-name = שם מלא
note-template-field-known-as = מוכר גם בשם, נשמר בשדה "מוכר גם בשם" בחלונית המאפיינים
note-template-field-age = גיל
note-template-field-role-in-story = תפקיד בסיפור
note-template-field-build-and-features = מבנה גוף וסימני היכר
note-template-field-habitual-bearing = הליכות ויציבה
note-template-field-speech-patterns = דפוסי דיבור והרגלי לשון
note-template-field-what-they-never-say = מה הדמות אף פעם לא אומרת
note-template-field-want = מה הדמות רוצה
note-template-field-need = מה הדמות צריכה בלי לדעת זאת
note-template-field-fear = ממה הדמות מפחדת
note-template-field-flaw = הפגם שעולה לה ביוקר
note-template-field-formative-event = אירוע מעצב
note-template-field-relationships = יחסים
note-template-field-starts-as = מתחילה כ…
note-template-field-ends-as = מסיימת כ…
note-template-field-what-you-notice-first = מה שמים לב אליו קודם
note-template-field-sound-and-smell = קולות וריחות
note-template-field-light-and-weather = אור ומזג אוויר
note-template-field-what-happened-here = מה קרה כאן
note-template-field-who-lives-or-works-here = מי גר או עובד כאן
note-template-field-scenes-set-here = סצנות שמתרחשות כאן
note-template-field-why-it-matters = למה זה חשוב
note-template-field-appearance = מראה
note-template-field-age-and-origin = גיל ומוצא
note-template-field-what-it-does = מה הוא עושה
note-template-field-who-holds-it = מי מחזיק בו
note-template-field-who-wants-it = מי רוצה אותו
note-template-field-pov = נקודת מבט
note-template-field-time-and-place = זמן ומקום
note-template-field-goal = מטרה
note-template-field-conflict = קונפליקט
note-template-field-turn = התפנית
note-template-field-exit-emotion = הרגש ביציאה
note-template-field-purpose = תכלית
note-template-field-who-leads-it = מי מוביל אותו
note-template-field-resources = משאבים
note-template-field-allies-and-enemies = בעלי ברית ואויבים
note-template-field-what-it-wants-now = מה הוא רוצה עכשיו
note-template-field-where-from = מאין זה הגיע
note-template-field-page-or-link = עמוד או קישור
note-template-field-key-facts = עובדות מרכזיות
note-template-field-how-it-is-used = איך זה משמש בסיפור
