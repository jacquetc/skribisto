# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# משאבים: קבצים וקישורי אינטרנט שמצורפים לפריטים בקלסר.

## הלוח

resources-title = משאבים
shortcut-name-resources-show = הצגת המשאבים
resources-add-files = הוספת קבצים
resources-add-files-title = הוספת קבצים לפריט הזה
resources-add-link = הוספת קישור אינטרנט
resources-drop-hint = אפשר לשחרר כאן קבצים או כתובות אינטרנט כדי לצרף אותם
resources-no-item = יש לפתוח פריט כדי לראות את המשאבים שלו.
resources-empty = עדיין לא צורף דבר לפריט הזה. אפשר להוסיף קבצים או קישורי אינטרנט בכפתורים שלמעלה, או לשחרר אותם כאן.
resources-actions = פעולות
resources-not-found = לא נמצא
resources-row-linked = קובץ מקושר
resources-row-linked-size = קובץ מקושר, { $size }
resources-row-name = { $title }, { $kind }, { $detail }
resources-row-name-missing = { $title }, { $kind }, { $detail }, לא נמצא

## סוגי הקבצים, כפי שקורא מסך מקריא אותם

resources-kind-picture = תמונה
resources-kind-pdf = מסמך PDF
resources-kind-audio = הקלטת קול
resources-kind-video = סרטון
resources-kind-document = מסמך
resources-kind-ebook = ספר אלקטרוני
resources-kind-archive = ארכיון
resources-kind-file = קובץ
resources-kind-web-link = קישור אינטרנט

## התפריט של שורה

resources-open = פתיחה
resources-show-in-folder = הצגה בתיקייה
resources-locate = איתור…
resources-insert-into-text = הוספה לטקסט
resources-insert-picture = הוספת התמונה לטקסט
resources-insert-link = הוספת קישור לטקסט
resources-edit = שינוי שם ותיאור…
resources-detach = ניתוק מהפריט הזה
resources-remove = הסרה מהפרויקט…

## שתי תיבות הדו־שיח

resources-link-dialog-title = הוספת קישור אינטרנט
resources-link-dialog-address = כתובת
resources-link-dialog-address-placeholder = https://
resources-link-dialog-name = כותרת
resources-link-dialog-name-placeholder = שם האתר, אם השדה נשאר ריק
resources-link-dialog-add = הוספה
resources-edit-dialog-title = שינוי שם ותיאור
resources-edit-dialog-name = כותרת
resources-edit-dialog-description = תיאור
resources-edit-dialog-description-placeholder = מה זה, ולמה שמרתם את זה
resources-edit-dialog-save = שמירה
resources-dialog-cancel = ביטול

## הוספה

resources-large-title = קובץ גדול
resources-large-text = הגודל של "{ $name }" הוא { $size }. אם הוא מאוחסן בפרויקט, הוא עובר עם כל עותק וכל גיבוי של הפרויקט. אם הוא מקושר, הוא נשאר במקומו, והפרויקט זוכר רק איפה למצוא אותו.
resources-large-text-many = הגודל של "{ $name }" הוא { $size }, וגם { $others ->
    [one] קובץ אחד נוסף גדול
    [two] שני קבצים נוספים גדולים
   *[other] { $others } קבצים נוספים גדולים
}. אם הם מאוחסנים בפרויקט, הם עוברים עם כל עותק וכל גיבוי של הפרויקט. אם הם מקושרים, הם נשארים במקומם, והפרויקט זוכר רק איפה למצוא אותם.
resources-large-store = לאחסן בפרויקט
resources-large-link = לקשר אליו במקום זאת
resources-large-remember = לא לשאול שוב
resources-storing-title = הוספת קבצים לפרויקט
resources-store-cancel = ביטול
resources-store-cancelled = לא נוספו קבצים.
resources-store-busy = עדיין מתבצעת הוספה של קבצים. אפשר לנסות שוב כשההוספה תסתיים.
resources-store-failed = לא ניתן היה להוסיף את הקבצים: { $error }
resources-no-project = יש לפתוח פרויקט לפני צירוף קבצים.
resources-moved-away = הפרויקט הועבר בזמן העתקת הקובץ.
resources-some-failed = { $count ->
    [one] לא ניתן היה להוסיף קובץ אחד: { $names }
    [two] לא ניתן היה להוסיף שני קבצים: { $names }
   *[other] לא ניתן היה להוסיף { $count } קבצים: { $names }
}
resources-add-failed = לא ניתן היה להוסיף את המשאבים: { $error }
resources-added = { $count ->
    [one] נוסף משאב אחד
    [two] נוספו שני משאבים
   *[other] נוספו { $count } משאבים
}
resources-dropped-added = { $count ->
    [one] נוסף כמשאב של הפריט הזה, וקושר בטקסט
    [two] שני קבצים נוספו כמשאבים של הפריט הזה, וקושרו בטקסט
   *[other] { $count } קבצים נוספו כמשאבים של הפריט הזה, וקושרו בטקסט
}
resources-undo = ביטול פעולה
resources-link-refused = הכתובת "{ $url }" אינה כתובת אינטרנט ש־Skribisto יכול לשמור. יש להשתמש בכתובת שמתחילה ב־http:// או ב־https://, בלי שם משתמש או סיסמה.

## ההודעה לפרויקט שנשמר כקובץ יחיד

resources-folder-title = הפרויקט הזה נעשה גדול
resources-folder-text = הקבצים המאוחסנים בו תופסים עכשיו { $size }. פרויקט שנשמר כקובץ יחיד נכתב מחדש במלואו בכל שמירה, כולל הקבצים שלו. פרויקט שנשמר כתיקייה שומר רק את מה שהשתנה. אפשר לשמור אותו כתיקייה עכשיו, ולהמשיך לעבוד בתיקייה מכאן והלאה.
resources-folder-switch = שמירה כתיקייה…
resources-folder-not-now = לא עכשיו
resources-folder-never = לא לשאול שוב לגבי הפרויקט הזה

## פתיחה

resources-not-found-toast = הקובץ "{ $name }" אינו במקום שבו הפרויקט מצפה למצוא אותו.
resources-unsafe-title = לפתוח את "{ $name }"?
resources-unsafe-text = ‏Skribisto אינו יודע אם סוג הקובץ הזה בטוח. פתיחתו עלולה להפעיל תוכנה במחשב, ופרויקט יכול להגיע ממישהו אחר. כדאי להציג אותו קודם בתיקייה שלו כדי לראות מה הוא.
resources-open-anyway = לפתוח בכל זאת
resources-open-failed = לא ניתן היה לפתוח את הקובץ: { $error }
resources-no-data-folder = המערכת הזו אינה נותנת ל־Skribisto תיקיית נתונים שבה אפשר לפתוח עותק
resources-link-gone = הקישור הזה מפנה למשאב שכבר אינו בפרויקט.
resources-locate-title = איפה נמצא עכשיו "{ $name }"?
resources-relinked-more = { $count ->
    [one] נמצא באותה תיקייה עוד קובץ חסר אחד, והוא קושר מחדש.
    [two] נמצאו באותה תיקייה עוד שני קבצים חסרים, והם קושרו מחדש.
   *[other] נמצאו באותה תיקייה עוד { $count } קבצים חסרים, והם קושרו מחדש.
}
resources-changed-elsewhere = הקובץ "{ $name }" שונה ביישום אחר. הפרויקט שומר את הגרסה החדשה.

## עריכה

resources-edit-failed = לא ניתן היה לשנות את המשאב: { $error }
resources-remove-title = להסיר את "{ $title }" מהפרויקט?
resources-remove-text = הוא מוסר מכל פריט שהוא מצורף אליו. קובץ שמאוחסן בפרויקט הולך איתו; קובץ מקושר ודף אינטרנט נשארים במקומם. ביטול פעולה מחזיר אותו.
resources-remove-confirm = הסרה
resources-remove-failed = לא ניתן היה להסיר את המשאב: { $error }

## לתוך הטקסט

resources-no-text = יש להציב קודם את הסמן בטקסט, ואז להוסיף את המשאב שם.
resources-drop-insert-picture = הוספת התמונה
resources-drop-insert-link = הוספת קישור למשאב

## התקדמות

progress-resources-storing = מעתיק את הקבצים
progress-resources-stored = הקבצים הועתקו
