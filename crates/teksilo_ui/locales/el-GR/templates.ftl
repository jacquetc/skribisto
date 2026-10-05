# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

settings-page-templates = Πρότυπα
settings-templates-description = Επαναχρησιμοποιήσιμα κομμάτια κειμένου που μπορείτε να βάλετε σε οτιδήποτε γράφετε: ένα κενό φύλλο προσώπου, ένα προφίλ τόπου, έναν πίνακα σκηνών. Αποθηκεύονται σε αυτό το έργο, οπότε όποιος το ανοίγει έχει το ίδιο σύνολο.
settings-templates-filter = Φιλτράρισμα προτύπων
settings-templates-count = { $n ->
    [0] Κανένα πρότυπο
    [one] 1 πρότυπο
   *[other] { $n } πρότυπα
}
settings-templates-empty = Αυτό το έργο δεν έχει ακόμη πρότυπα.
settings-templates-empty-hint = Προσθέστε ένα από τα προκαθορισμένα, εισαγάγετε ένα αρχείο .md ή .djot, ή γράψτε κάτι και επιλέξτε Έγγραφο ▸ Αποθήκευση ως πρότυπο.
settings-templates-presets = Προσθήκη προκαθορισμένου
settings-templates-import = Εισαγωγή…
settings-templates-export = Εξαγωγή…
settings-templates-star = Εμφάνιση πρώτο στο μενού παρεμβολής
settings-templates-unstar = Να μην εμφανίζεται πρώτο
settings-templates-move-up = Μετακίνηση πάνω
settings-templates-move-down = Μετακίνηση κάτω
settings-templates-delete = Διαγραφή προτύπου
settings-templates-name = Όνομα
settings-templates-name-placeholder = Όνομα προτύπου
settings-templates-body-placeholder = Το κείμενο που παρεμβάλλει αυτό το πρότυπο
settings-templates-duplicate-name = Υπάρχει ήδη άλλο πρότυπο με το όνομα «{ $name }»
settings-templates-words = { $n ->
    [one] 1 λέξη
   *[other] { $n } λέξεις
}

templates-imported = { $added ->
    [one] Εισήχθη 1 πρότυπο
   *[other] Εισήχθησαν { $added } πρότυπα
}
templates-imported-renamed = Μετονομάστηκαν για αποφυγή σύγκρουσης: { $names }
templates-imported-skipped = Αδυναμία ανάγνωσης: { $names }
templates-import-filter = Πρότυπα (.md, .djot)
templates-exported = { $n ->
    [one] Εξήχθη 1 πρότυπο
   *[other] Εξήχθησαν { $n } πρότυπα
}
templates-preset-applied = Προστέθηκε το «{ $name }»

templates-delete-title = Διαγραφή αυτού του προτύπου;
templates-delete-body = Το «{ $name }» θα αφαιρεθεί από αυτό το έργο. Οι σημειώσεις που έχετε ήδη δημιουργήσει από αυτό δεν επηρεάζονται.
templates-delete-confirm = Διαγραφή

menu-document = Έ&γγραφο
menu-insert-template = Παρεμβολή &προτύπου
menu-insert-template-none = Δεν υπάρχουν πρότυπα σε αυτό το έργο
menu-save-as-template = Αποθήκευση ως π&ρότυπο…

save-as-template-title = Αποθήκευση ως πρότυπο
save-as-template-explain = Το κείμενο που επεξεργάζεστε γίνεται πρότυπο που μπορείτε να βάλετε οπουδήποτε αλλού.
save-as-template-name = Όνομα
save-as-template-placeholder = Φύλλο προσώπου
save-as-template-duplicate = Υπάρχει ήδη πρότυπο με το όνομα «{ $name }»
save-as-template-empty-editor = Αυτός ο επεξεργαστής είναι κενός. Δεν υπάρχει τίποτα για αποθήκευση.
save-as-template-confirm = Αποθήκευση προτύπου
save-as-template-saved = Το «{ $name }» αποθηκεύτηκε ως πρότυπο
template-inserted = Παρεμβλήθηκε το «{ $name }»

note-template-preset-character-sheet = Φύλλο προσώπου
note-template-preset-location = Τόπος
note-template-preset-artifact = Αντικείμενο
note-template-preset-beat-sheet = Πίνακας σκηνών
note-template-preset-faction = Φατρία
note-template-preset-research-note = Σημείωση έρευνας

note-template-set-essentials = Απαραίτητα
note-template-set-everything = Όλα τα πρότυπα

note-template-section-identity = Ταυτότητα
note-template-section-appearance = Εξωτερική εμφάνιση
note-template-section-voice = Φωνή
note-template-section-psychology = Εσωτερική ζωή
note-template-section-history = Ιστορία
note-template-section-arc = Τόξο εξέλιξης
note-template-section-first-impression = Πρώτη εντύπωση
note-template-section-in-the-story = Στην ιστορία
note-template-section-the-thing-itself = Το ίδιο το αντικείμενο
note-template-section-the-scene = Η σκηνή
note-template-section-the-shape = Η δομή
note-template-section-what-it-is = Τι είναι
note-template-section-source = Πηγή
note-template-section-what-it-says = Τι λέει

note-template-field-full-name = Πλήρες όνομα
note-template-field-known-as = Γνωστό και ως (καταγράφεται στο πεδίο «Γνωστό και ως» του επιθεωρητή)
note-template-field-age = Ηλικία
note-template-field-role-in-story = Ρόλος στην ιστορία
note-template-field-build-and-features = Σωματότυπος και διακριτικά γνωρίσματα
note-template-field-habitual-bearing = Συνήθης στάση και κινήσεις
note-template-field-speech-patterns = Τρόπος ομιλίας και λεκτικοί τικ
note-template-field-what-they-never-say = Τι δεν λέει ποτέ
note-template-field-want = Τι θέλει
note-template-field-need = Τι χρειάζεται χωρίς να το ξέρει
note-template-field-fear = Τι φοβάται
note-template-field-flaw = Το ελάττωμα που του κοστίζει
note-template-field-formative-event = Καθοριστικό γεγονός
note-template-field-relationships = Σχέσεις
note-template-field-starts-as = Ξεκινά ως
note-template-field-ends-as = Καταλήγει ως
note-template-field-what-you-notice-first = Τι προσέχει κανείς πρώτα
note-template-field-sound-and-smell = Ήχος και οσμή
note-template-field-light-and-weather = Φως και καιρός
note-template-field-what-happened-here = Τι συνέβη εδώ
note-template-field-who-lives-or-works-here = Ποιος ζει ή εργάζεται εδώ
note-template-field-scenes-set-here = Σκηνές που διαδραματίζονται εδώ
note-template-field-why-it-matters = Γιατί έχει σημασία
note-template-field-appearance = Εξωτερική εμφάνιση
note-template-field-age-and-origin = Ηλικία και προέλευση
note-template-field-what-it-does = Τι κάνει
note-template-field-who-holds-it = Ποιος το έχει
note-template-field-who-wants-it = Ποιος το θέλει
note-template-field-pov = Οπτική γωνία
note-template-field-time-and-place = Χρόνος και τόπος
note-template-field-goal = Στόχος
note-template-field-conflict = Σύγκρουση
note-template-field-turn = Η ανατροπή
note-template-field-exit-emotion = Συναίσθημα στην έξοδο
note-template-field-purpose = Σκοπός
note-template-field-who-leads-it = Ποιος ηγείται
note-template-field-resources = Πόροι
note-template-field-allies-and-enemies = Σύμμαχοι και εχθροί
note-template-field-what-it-wants-now = Τι θέλει τώρα
note-template-field-where-from = Από πού προέρχεται
note-template-field-page-or-link = Σελίδα ή σύνδεσμος
note-template-field-key-facts = Βασικά στοιχεία
note-template-field-how-it-is-used = Πώς χρησιμοποιείται στην ιστορία
