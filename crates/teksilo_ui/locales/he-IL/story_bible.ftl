# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

story-bible-modal-configure-title = הגדרת הרשומה החדשה
story-bible-modal-name-label = שם
story-bible-modal-name-placeholder = דמות, מקום, כל דבר שכדאי לתעד…
story-bible-modal-no-binders = לפרויקט הזה עדיין אין קלסרים לתייק בהם את הרשומה
story-bible-modal-template-label = התחלה מתבנית (רשות)
story-bible-modal-template-placeholder = ללא תבנית
story-bible-modal-body-label = טקסט
story-bible-modal-cancel = ביטול
story-bible-modal-create = יצירה
story-bible-modal-create-and-open = יצירה ופתיחה
story-bible-modal-failed = לא ניתן היה ליצור את הרשומה

ctx-add-as-note = הוספה כפתק… (&N)
ctx-add-as-note-all-tags = כל התגים…
ctx-add-as-note-untagged = ללא תג

story-bible-grid-label = תיק הסיפור
story-bible-grid-untagged = עדיין ללא תג
story-bible-grid-empty-title = עדיין לא תויק כאן דבר
story-bible-grid-empty-hint = מתייגים פתק כדי לאתר אותו בטקסט, או מוסיפים פתק מתוך בחירה, והוא מופיע כאן.
story-bible-grid-alias-count = { $count ->
    [0] אין עדיין שמות נוספים
    [one] שם נוסף אחד
   *[other] { $count } שמות נוספים
}
story-bible-grid-mention-count = { $count ->
    [0] אין עדיין הופעות
    [one] מופיע בסצנה אחת בכל הפרויקט
   *[other] מופיע ב־{ $count } סצנות בכל הפרויקט
}
story-bible-books-filter-all = כל הספרים
story-bible-grid-not-searched = עדיין לא נבדק: נדרש תג של תיק הסיפור

note-in-prose-pov = נקודת מבט
note-in-prose-cast = צוות הדמויות
note-in-prose-pov-and-cast = נקודת מבט · צוות הדמויות
note-in-prose-no-books = לפרויקט הזה עדיין אין ספר
# The page lists only scenes where the writer placed the entry in the cast or the point of
# view, never the ones the scan merely found its name in, so an empty page says so: a bare
# "nothing here" read as a search that had failed.
note-in-prose-empty-book = עדיין אין בספר הזה סצנה שהרשומה הזו נמצאת בצוות הדמויות או בנקודת המבט שלה.
note-in-prose-empty-viewpoint = עדיין אין בספר הזה סצנה שמסופרת מנקודת המבט של הרשומה הזו.
note-in-prose-scope = הדף "בטקסט" מציג רק את הסצנות שאישרתם. סצנה שמזכירה את הרשומה הזו נשארת הצעה עד שתוסיפו את הרשומה לצוות הדמויות שלה, ב"פרטים" או בחלונית המאפיינים.
# A switch beside the Book bar: keep only the scenes this entry narrates.
note-in-prose-viewpoint-only = סצנות נקודת מבט בלבד
note-in-prose-viewpoint-only-tooltip = הצגה של הסצנות שמסופרות מנקודת המבט של הרשומה הזו בלבד, והסתרה של הסצנות שהיא רק מופיעה בהן

story-bible-capture-toast = "{ $name }" נוסף לתיק הסיפור
story-bible-capture-open = פתיחה
story-bible-capture-undo = ביטול פעולה
story-bible-capture-where-title = לאן ייכנסו הפתקים האלה?
story-bible-capture-where-prompt = יש לבחור את התיקייה שבה יתויקו פתקים חדשים.
story-bible-capture-where-confirm = תיוק כאן
story-bible-capture-where-needs-folder = יש לבחור תיקייה שהפתקים האלה ייכנסו לתוכה.
story-bible-capture-where-tag = השאלה נשאלת פעם אחת. פתקים עם התג "{ $tag }" ייכנסו לכאן מעכשיו, ואפשר לשנות זאת מאוחר יותר בהגדרות, פרויקט, תגים.
story-bible-capture-where-untagged = השאלה נשאלת פעם אחת. פתקים ללא תג ייכנסו לכאן מעכשיו, ואפשר לשנות זאת מאוחר יותר בהגדרות, פרויקט, תגים.

# What a screen reader hears for a binder row that is a story bible entry: its icon
# says so on screen, and this says it aloud. The second form keeps the row's label.
binder-row-story-bible-entry = רשומה בתיק הסיפור
binder-row-story-bible-entry-with-label = רשומה בתיק הסיפור, { $label }
