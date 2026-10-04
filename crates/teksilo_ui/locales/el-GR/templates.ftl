# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

settings-page-templates = Πρότυπα
settings-templates-description = Επαναχρησιμοποιήσιμα κομμάτια γραφής μπορείτε να πέσετε σε οτιδήποτε γράφετε: ένα κενό φύλλο χαρακτήρων, ένα προφίλ τοποθεσίας, ένα φύλλο χτυπήματος. Αποθηκεύονται σε αυτό το έργο, έτσι ώστε όποιος το ανοίγει να πάρει το ίδιο σύνολο.
settings-templates-filter = Πρότυπα φιλτραρίσματος
settings-templates-count = { $n ->
    [0] Χωρίς πρότυπα
    [one] 1 πρότυπο
   *[other] { $n } υποδείγματα
}
settings-templates-empty = Αυτό το έργο δεν έχει ακόμη πρότυπα.
settings-templates-empty-hint = Προσθέστε ένα από ένα προκαθορισμένο, εισαγωγή ενός αρχείου .md ή .djot, ή γράψτε κάτι και επιλέξτε Έγγραφο .
settings-templates-presets = Προσθήκη προεπιλογής
settings-templates-import = Εισαγωγή…
settings-templates-export = Εξαγωγή…
settings-templates-star = Εμφάνιση πρώτα στο μενού εισαγωγής
settings-templates-unstar = Σταμάτα να δείχνεις πρώτος.
settings-templates-move-up = Μετακίνηση πάνω
settings-templates-move-down = Μετακίνηση κάτω
settings-templates-delete = Διαγραφή προτύπου
settings-templates-name = Όνομα
settings-templates-name-placeholder = Όνομα προτύπου
settings-templates-body-placeholder = Το κείμενο αυτό το πρότυπο εισάγει
settings-templates-duplicate-name = Ένα άλλο πρότυπο ονομάζεται ήδη -{ $name } -
settings-templates-words = { $n ->
    [one] 1 λέξη
   *[other] { $n } λέξεις
}

templates-imported = { $added ->
    [one] 1 εισαγόμενο πρότυπο
   *[other] { $added } εισαγόμενα πρότυπα
}
templates-imported-renamed = Μετονομασία για αποφυγή σύγκρουσης: { $names }
templates-imported-skipped = Αδυναμία ανάγνωσης: { $names }
templates-import-filter = Υποδείγματα (.md, .djot)
templates-exported = { $n ->
    [one] 1 εξαχθείσα υπόδειγμα
   *[other] { $n } εξαγόμενα πρότυπα
}
templates-preset-applied = Προστέθηκε -{ $name } .

templates-delete-title = Διαγραφή αυτού του προτύπου;
templates-delete-body = "{ $name } Οι σημειώσεις που έχετε ήδη δημιουργήσει από αυτό δεν επηρεάζονται.
templates-delete-confirm = Διαγραφή

menu-document = Έγγραφο (&D)
menu-insert-template = Εισαγωγή προτύπου (&L)
menu-insert-template-none = Δεν υπάρχουν πρότυπα σε αυτό το έργο
menu-save-as-template = Αποθήκευση ως πρότυπο... (&V)

save-as-template-title = Αποθήκευση ως πρότυπο
save-as-template-explain = Το κείμενο που επεξεργάζεστε γίνεται ένα πρότυπο που μπορείτε να μπείτε οπουδήποτε αλλού.
save-as-template-name = Όνομα
save-as-template-placeholder = Φύλλο χαρακτήρων
save-as-template-duplicate = Ένα πρότυπο που ονομάζεται -{ $name } - υπάρχει ήδη
save-as-template-empty-editor = Αυτός ο εκδότης είναι άδειος.
save-as-template-confirm = Αποθήκευση προτύπου
save-as-template-saved = Αποθηκευμένη : { $name } :  ως πρότυπο
template-inserted = Προστέθηκε -{ $name } .

note-template-preset-character-sheet = Φύλλο χαρακτήρων
note-template-preset-location = Τοποθεσία
note-template-preset-artifact = Αντικείμενο
note-template-preset-beat-sheet = Φύλλο χτυπήματος
note-template-preset-faction = Σχήμα
note-template-preset-research-note = Σημείωμα έρευνας

note-template-set-essentials = Απαραίτητα
note-template-set-everything = Κάθε πρότυπο

note-template-section-identity = Ταυτότητα
note-template-section-appearance = Εμφάνιση
note-template-section-voice = Φωνή
note-template-section-psychology = Εσωτερική ζωή
note-template-section-history = Ιστορία
note-template-section-arc = Τόξο
note-template-section-first-impression = Πρώτη εντύπωση
note-template-section-in-the-story = Στην ιστορία
note-template-section-the-thing-itself = Το ίδιο το αντικείμενο
note-template-section-the-scene = Η σκηνή.
note-template-section-the-shape = Το σχήμα
note-template-section-what-it-is = Τι είναι αυτό;
note-template-section-source = Πηγή
note-template-section-what-it-says = Τι λέει;

note-template-field-full-name = Πλήρες όνομα
note-template-field-known-as = Γνωστός και ως, εντοπίστηκε στο πεδίο Alias του Επιθεωρητή.
note-template-field-age = Ηλικία
note-template-field-role-in-story = Ρόλος στην ιστορία
note-template-field-build-and-features = Κατασκευή και διακριτικά χαρακτηριστικά
note-template-field-habitual-bearing = Συνήθης έδραση
note-template-field-speech-patterns = Ομιλίες και προφορικά τικ
note-template-field-what-they-never-say = Αυτό που δεν λένε ποτέ
note-template-field-want = Τι θέλουν;
note-template-field-need = Τι χρειάζονται, αλλά δεν το ξέρουν.
note-template-field-fear = Αυτό που φοβούνται.
note-template-field-flaw = Το ελάττωμα που τους κοστίζει
note-template-field-formative-event = Μορφωτικό γεγονός
note-template-field-relationships = Σχέσεις
note-template-field-starts-as = Εκκινεί ως
note-template-field-ends-as = Τελειώνει όπως
note-template-field-what-you-notice-first = Τι παρατηρείς πρώτα;
note-template-field-sound-and-smell = Ήχος και οσμή
note-template-field-light-and-weather = Φως και καιρός
note-template-field-what-happened-here = Τι συνέβη εδώ;
note-template-field-who-lives-or-works-here = Ποιος ζει ή Εργάζεται εδώ;
note-template-field-scenes-set-here = Σκηνές που ορίστηκαν εδώ
note-template-field-why-it-matters = Γιατί έχει σημασία;
note-template-field-appearance = Εμφάνιση
note-template-field-age-and-origin = Ηλικία και προέλευση
note-template-field-what-it-does = Τι κάνει;
note-template-field-who-holds-it = Ποιος το κρατάει;
note-template-field-who-wants-it = Ποιος το θέλει;
note-template-field-pov = Οπτική γωνία
note-template-field-time-and-place = Χρόνος και τόπος
note-template-field-goal = Στόχος
note-template-field-conflict = Σύγκρουση
note-template-field-turn = Η στροφή.
note-template-field-exit-emotion = Συναίσθημα κατά την έξοδο
note-template-field-purpose = Αντικείμενο
note-template-field-who-leads-it = Ποιος το οδηγεί;
note-template-field-resources = Πόροι
note-template-field-allies-and-enemies = Σύμμαχοι και εχθροί
note-template-field-what-it-wants-now = Αυτό που θέλει τώρα.
note-template-field-where-from = Από πού προήλθε;
note-template-field-page-or-link = Σελίδα ή σύνδεση
note-template-field-key-facts = Βασικά στοιχεία
note-template-field-how-it-is-used = Πώς χρησιμοποιείται στην ιστορία
