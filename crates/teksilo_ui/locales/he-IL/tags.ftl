# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

tags-preset-basic = יסוד
tags-preset-scifi = מדע בדיוני
tags-preset-fantasy = פנטזיה פנטזיה
tags-preset-mystery = המסתורין המסתורי
tags-preset-historical = היסטוריה היסטורית


tags-preset-needs-research = זקוק למחקר
tags-preset-continuity-check = בדיקות רציפות
tags-preset-plot-point = שלב העלילה

tags-preset-character = אופי אופי
tags-preset-place = מקום
tags-preset-item = פריט

tags-preset-vessel = כלי שיט
tags-preset-planet = כוכב הלכת Planet
tags-preset-organization = ארגון
tags-preset-creature = יצור היצור
tags-preset-faction = עובדה
tags-preset-artifact = חפצים
tags-preset-realm = ממלכה
tags-preset-magic-system = מערכת קסם
tags-preset-suspect = החשוד החשוד
tags-preset-victim = הקורבן
tags-preset-clue = רמזים
tags-preset-red-herring = Red Herring
tags-preset-historical-figure = דמות היסטורית
tags-preset-source = מקור מקור מקור
tags-preset-period-detail = זמן פרטים

tags-pill-list = תגיות
tags-pill-add = הוסף תג
tags-none-yet = אין תגים
tags-pick-list = תגיות to pick from
tags-pill-remove = { $name }
tags-pill-filter-placeholder = פילטר או שם חדש
tags-pill-no-match = No tags
tags-pill-create = צור "{ $name }"
tags-pill-create-failed = לא יכול היה ליצור "{ $name }"
tags-pill-new-discoverable = תגית: Find-in-prose tag
tags-pill-new-discoverable-hint = פריטים עם תג זה תואמים נגד הפרוזה שלך כדי למלא את הroster.

tags-alias-list = ידוע גם בשם
tags-alias-add = הוסף שם אחר
tags-alias-remove = { $name }
tags-alias-placeholder = שם אחר, היכנס
tags-alias-hint = שמות אלה מופיעים תחת הכותרת שלך, לצד הכותרת שלו.
tags-alias-collision = { $name } תשובות על שם זה.

settings-page-tags = תגיות
settings-tags-desc = תוויות: הפריטים בחבר שלך.A Find-in-prose tags גם מספר Skribisto כדי לחפש את השמות של פריט זה בפרוזה שלך.
settings-tags-add = הוסף תג
settings-tags-add-placeholder = שם תג חדש
settings-tags-added = { $name }
settings-tags-duplicate = "{ $name }"כבר קיים
settings-tags-filter = תוויות filter
settings-tags-count = { $n ->
    [one] 1 תג
   *[other] { $n } תגים
}
settings-tags-details-placeholder = מה המשמעות של התג הזה
settings-tags-discoverable = Find in Prose
settings-tags-creates-in = הערות חדשות מגיעות
settings-tags-creates-in-unset = שאל אותי בפעם הראשונה
settings-tags-creates-in-untitled = תיקיה ללא תשלום
settings-tags-creates-in-trashed = { $name } (בפח האשפה)
settings-tags-template = החל תבנית
settings-tags-template-unset = תגית: Blank Note
settings-tags-delete = { $name }
settings-tags-deleted = "{ $name }" והוציאו אותו מכל פריט
settings-tags-empty = עדיין אין תגים.
settings-tags-apply-preset = החל מראש...
settings-tags-preset-applied = { $added }, דלגו על { $skipped } כבר עכשיו
settings-tags-csv-filter = CSV קבצים
settings-tags-import = ייבוא…
settings-tags-export = ייצוא…
settings-tags-imported = { $added }, דלג { $skipped }
settings-tags-exported = ייצוא { $n ->
    [one] 1 תג
   *[other] { $n } תגים
}

tags-chip-more = { $n ->
    [one] 1 עוד תג
   *[other] { $n } תגים נוספים
}

cast-section = דמויות
cast-add = הוסף ל-Slece...
cast-add-filter-placeholder = ספר מסנן התנ"ך
cast-add-empty = אין ערך לסיפור כדי להוסיף
cast-pin = עקבו אחרי { $name }
cast-unpin = { $name }
cast-empty = אף אחד עדיין לא נכנס, הוסף או לשמור הצעה.
cast-unresolved = כבר לא בסיפור התנ"ך

mentions-confirm = הוסף ל- { $name }
mentions-hit-count = { $n ->
    [one] פעם אחת
   *[other] { $n } פעמים
}
mentions-point-of-view-badge = נקודת מבט
mentions-point-of-view-badge-tooltip = הובהר כנקודת המבט כאן, שנקבעה ביד, לא מזוהה בפרוזה.

mentions-roster = דמויות
mentions-pin = עקבו אחרי { $name }

pov-section = נקודת מבט
pov-empty = אין נקודת מבט
pov-add = הגדרת נקודת מבט...
pov-multiple = לזירה הזאת יש שתי נקודות מבט.
pov-remove = { $name } כנקודת מבט
pov-unresolved = נקודת מבט הונחה כאן, אבל הכניסה הזאת כבר לא קיימת או איבדה את תג הסיפור שלה.

books-section = משויך אל
books-empty = עדיין לא הוגשו תחת ספר
books-add = File under a book...
books-remove = { $name }
books-apply-to-children = הגשת בקשה לילדים

note-details-name-placeholder = שם…
note-details-tags = תגיות
note-details-aliases = שמות נוספים
note-details-books = משויך אל
note-details-books-empty = עדיין לא הוגשו תחת ספר
note-details-links = קישורים
note-details-links-empty = עדיין אין קישורים, הוסף אחד כדי להצביע על כניסה אחרת בסיפור התנ"ך.
note-details-links-add = קישור להערה...
note-details-pov = נקודת מבט
note-details-pov-empty = אין נקודת מבט
note-details-pov-unresolved = נקודת מבט הונחה כאן, אבל הכניסה הזאת כבר לא קיימת או איבדה את תג הסיפור שלה.
note-details-pov-multiple = להודעה זו יש שתי נקודות מבט.
note-details-backlinks = מופיע בכתב היד
note-details-backlinks-empty = שום דבר אחר עוד לא מופיע שם זה בפרוזה שלך, הוא יופיע כאן.
note-details-backlinks-outside = מחוץ לספרים
note-details-backlinks-confirm-all = אישור כל הופעה
note-details-backlinks-confirm-all-tooltip = הוסף כניסה זו לסט של כל מסמך המוצג כאן כי טרם אישרה זאת.
note-details-untitled-document = ללא שם

note-in-prose-mentions = { $n ->
    [one] 1 אזכור
   *[other] { $n } מזכירות
}
note-in-prose-mention-at = { $current } { $total }
note-in-prose-mention-previous = תגית:
note-in-prose-mention-next = מזכיר הבא
