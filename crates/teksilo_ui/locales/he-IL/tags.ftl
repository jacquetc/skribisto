# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

tags-preset-basic = בסיסי
tags-preset-scifi = מדע בדיוני
tags-preset-fantasy = פנטזיה
tags-preset-mystery = מסתורין
tags-preset-historical = היסטורי

tags-preset-needs-research = דורש מחקר
tags-preset-continuity-check = בדיקת רציפות
tags-preset-plot-point = נקודת עלילה

tags-preset-character = דמות
tags-preset-place = מקום
tags-preset-item = חפץ

tags-preset-vessel = כלי שיט
tags-preset-planet = כוכב לכת
tags-preset-organization = ארגון
tags-preset-creature = יצור
tags-preset-faction = פלג
tags-preset-artifact = חפץ קדום
tags-preset-realm = ממלכה
tags-preset-magic-system = מערכת קסם
tags-preset-suspect = חשוד
tags-preset-victim = קורבן
tags-preset-clue = רמז
tags-preset-red-herring = הטעיה
tags-preset-historical-figure = דמות היסטורית
tags-preset-source = מקור
tags-preset-period-detail = פרט תקופתי

tags-pill-list = תגים
tags-pill-add = הוספת תג
tags-none-yet = אין תגים
tags-pick-list = תגים לבחירה
tags-pill-remove = הסרת { $name }
tags-pill-filter-placeholder = סינון או שם לתג חדש
tags-pill-no-match = אין תג תואם
tags-pill-create = יצירת "{ $name }"
tags-pill-create-failed = לא ניתן היה ליצור את "{ $name }"
tags-pill-new-discoverable = תג לאיתור בטקסט
tags-pill-new-discoverable-hint = פריטים עם התג הזה מותאמים לטקסט שלכם כדי למלא את צוות הדמויות.

tags-alias-list = מוכר גם בשם
tags-alias-add = הוספת שם נוסף
tags-alias-remove = הסרת { $name }
tags-alias-placeholder = שם נוסף, ואז Enter
tags-alias-hint = שמות שבהם זה מופיע בטקסט שלכם, לצד הכותרת שלו.
tags-alias-collision = { $name } כבר עונה לשם הזה.

settings-page-tags = תגים
settings-tags-desc = תגים מסמנים את הפריטים שבקלסר. תג לאיתור בטקסט גם אומר ל־Skribisto לחפש את השמות של הפריט הזה בטקסט שלכם.
settings-tags-add = הוספת תג
settings-tags-add-placeholder = שם לתג חדש
settings-tags-added = "{ $name }" נוסף
settings-tags-duplicate = "{ $name }" כבר קיים
settings-tags-filter = סינון תגים
settings-tags-count = { $n ->
    [one] תג אחד
   *[other] { $n } תגים
}
settings-tags-details-placeholder = מה התג הזה אומר
settings-tags-discoverable = איתור בטקסט
settings-tags-creates-in = פתקים חדשים נכנסים אל
settings-tags-creates-in-unset = לשאול אותי בפעם הראשונה
settings-tags-creates-in-untitled = תיקייה ללא כותרת
settings-tags-creates-in-trashed = { $name } (באשפה)
settings-tags-template = תבנית התחלתית
settings-tags-template-unset = פתק ריק
settings-tags-delete = מחיקת { $name }
settings-tags-deleted = "{ $name }" נמחק והוסר מכל הפריטים
settings-tags-empty = אין עדיין תגים.
settings-tags-apply-preset = החלת ערכה מוכנה…
settings-tags-preset-applied = נוספו { $added }, דולגו { $skipped } שכבר קיימים
settings-tags-csv-filter = קובצי CSV
settings-tags-import = ייבוא…
settings-tags-export = ייצוא…
settings-tags-imported = יובאו { $added }, דולגו { $skipped }
settings-tags-exported = { $n ->
    [one] יוצא תג אחד
   *[other] יוצאו { $n } תגים
}

tags-chip-more = { $n ->
    [one] עוד תג אחד
   *[other] עוד { $n } תגים
}

cast-section = צוות הדמויות
cast-add = הוספה לצוות הדמויות…
cast-add-filter-placeholder = סינון תיק הסיפור…
cast-add-empty = אין רשומות בתיק הסיפור להוספה
cast-pin = הוספת { $name } לצוות הדמויות
cast-unpin = הסרת { $name } מצוות הדמויות
cast-empty = עדיין לא הוצמד איש. אפשר להוסיף, או לאשר הצעה
cast-unresolved = כבר לא נמצא בתיק הסיפור

mentions-confirm = הוספה לצוות הדמויות של { $name }
mentions-hit-count = { $n ->
    [one] פעם אחת
   *[other] { $n } פעמים
}
mentions-point-of-view-badge = נקודת מבט
mentions-point-of-view-badge-tooltip = הוגדר כאן כנקודת המבט, ידנית, ולא זוהה בטקסט.

mentions-roster = צוות הדמויות
mentions-pin = הוספת { $name } לצוות הדמויות

pov-section = נקודת מבט
pov-empty = לא הוגדרה נקודת מבט
pov-add = הגדרת נקודת מבט…
pov-multiple = לסצנה הזו יש שתי נקודות מבט.
pov-remove = הסרת { $name } כנקודת מבט
pov-unresolved = נקודת מבט הוצמדה כאן, אבל הרשומה הזו כבר לא קיימת או שאיבדה את תג תיק הסיפור שלה.

books-section = מתויק תחת
books-empty = עדיין לא מתויק תחת ספר
books-add = תיוק תחת ספר…
books-remove = הסרה מ־{ $name }
books-apply-to-children = החלת התיוק על הצאצאים

note-details-name-placeholder = שם…
note-details-tags = תגים
note-details-aliases = שמות נוספים
note-details-books = מתויק תחת
note-details-books-empty = עדיין לא מתויק תחת ספר
note-details-links = קישורים
note-details-links-empty = אין עדיין קישורים. אפשר להוסיף קישור שמפנה לרשומה אחרת בתיק הסיפור שלכם.
note-details-links-add = קישור לפתק…
note-details-pov = נקודת מבט
note-details-pov-empty = לא הוגדרה נקודת מבט
note-details-pov-unresolved = נקודת מבט הוצמדה כאן, אבל הרשומה הזו כבר לא קיימת או שאיבדה את תג תיק הסיפור שלה.
note-details-pov-multiple = לפתק הזה יש שתי נקודות מבט.
note-details-backlinks = מופיע בכתב היד
note-details-backlinks-empty = עדיין כלום. כשהשם הזה יופיע בטקסט שלכם, הוא יוצג כאן.
note-details-backlinks-outside = מחוץ לספרים
note-details-backlinks-confirm-all = אישור כל ההופעות
note-details-backlinks-confirm-all-tooltip = הוספת הרשומה הזו לצוות הדמויות של כל מסמך שמוצג כאן ועדיין לא אישר אותה
note-details-untitled-document = ללא כותרת

note-in-prose-mentions = { $n ->
    [one] אזכור אחד
   *[other] { $n } אזכורים
}
note-in-prose-mention-at = { $current } מתוך { $total }
note-in-prose-mention-previous = האזכור הקודם
note-in-prose-mention-next = האזכור הבא
