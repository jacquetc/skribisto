# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

tags-preset-basic = Βασικό
tags-preset-scifi = Επιστημονική φαντασία
tags-preset-fantasy = Φαντασία
tags-preset-mystery = Μυστήριο
tags-preset-historical = Ιστορικό μυθιστόρημα

tags-preset-needs-research = χρειάζεται έρευνα
tags-preset-continuity-check = έλεγχος συνέχειας
tags-preset-plot-point = σημείο καμπής

tags-preset-character = πρόσωπο
tags-preset-place = τόπος
tags-preset-item = αντικείμενο

tags-preset-vessel = σκάφος
tags-preset-planet = πλανήτης
tags-preset-organization = οργάνωση
tags-preset-creature = πλάσμα
tags-preset-faction = φατρία
tags-preset-artifact = κειμήλιο
tags-preset-realm = βασίλειο
tags-preset-magic-system = μαγικό σύστημα
tags-preset-suspect = ύποπτος
tags-preset-victim = θύμα
tags-preset-clue = ένδειξη
tags-preset-red-herring = παραπλανητικό στοιχείο
tags-preset-historical-figure = ιστορικό πρόσωπο
tags-preset-source = πηγή
tags-preset-period-detail = λεπτομέρεια εποχής

tags-pill-list = Ετικέτες
tags-pill-add = Προσθήκη ετικέτας

tags-none-yet = Χωρίς ετικέτες

tags-pick-list = Ετικέτες προς επιλογή
tags-pill-remove = Αφαίρεση { $name }
tags-pill-filter-placeholder = Φιλτράρισμα ή όνομα νέας ετικέτας
tags-pill-no-match = Καμία ετικέτα δεν αντιστοιχεί
tags-pill-create = Δημιουργία του «{ $name }»
tags-pill-create-failed = Αδυναμία δημιουργίας του «{ $name }»
tags-pill-new-discoverable = Ετικέτα εύρεσης στην πρόζα
tags-pill-new-discoverable-hint = Τα στοιχεία με αυτή την ετικέτα αναζητούνται στην πρόζα σας για να συμπληρωθεί ο κατάλογος προσώπων.

tags-alias-list = Γνωστό και ως
tags-alias-add = Προσθήκη άλλου ονόματος
tags-alias-remove = Αφαίρεση { $name }
tags-alias-placeholder = Ένα άλλο όνομα και μετά Enter
tags-alias-hint = Ονόματα με τα οποία εμφανίζεται στην πρόζα σας, εκτός από τον τίτλο του.
tags-alias-collision = Το { $name } έχει ήδη αυτό το όνομα.

settings-page-tags = Ετικέτες
settings-tags-desc = Οι ετικέτες σημαίνουν τα στοιχεία του ντοσιέ σας. Μια ετικέτα εύρεσης στην πρόζα λέει επίσης στο Skribisto να αναζητά τα ονόματα του στοιχείου στην πρόζα σας.
settings-tags-add = Προσθήκη ετικέτας
settings-tags-add-placeholder = Όνομα νέας ετικέτας
settings-tags-added = Προστέθηκε το «{ $name }»
settings-tags-duplicate = Το «{ $name }» υπάρχει ήδη
settings-tags-filter = Φιλτράρισμα ετικετών
settings-tags-count = { $n ->
    [one] 1 ετικέτα
   *[other] { $n } ετικέτες
}
settings-tags-details-placeholder = Τι σημαίνει αυτή η ετικέτα
settings-tags-discoverable = Εύρεση στην πρόζα
settings-tags-creates-in = Οι νέες σημειώσεις πηγαίνουν στο
settings-tags-creates-in-unset = Ερώτηση την πρώτη φορά
settings-tags-creates-in-untitled = Φάκελος χωρίς τίτλο
settings-tags-creates-in-trashed = { $name } (στον κάδο)
settings-tags-template = Αρχικό πρότυπο
settings-tags-template-unset = Κενή σημείωση
settings-tags-delete = Διαγραφή του { $name }
settings-tags-deleted = Το «{ $name }» διαγράφηκε και αφαιρέθηκε από κάθε στοιχείο
settings-tags-empty = Δεν υπάρχουν ακόμη ετικέτες.
settings-tags-apply-preset = Εφαρμογή προκαθορισμένου συνόλου…
settings-tags-preset-applied = Προστέθηκαν: { $added }, παραλείφθηκαν όσα υπήρχαν ήδη: { $skipped }
settings-tags-csv-filter = Αρχεία CSV
settings-tags-import = Εισαγωγή…
settings-tags-export = Εξαγωγή…
settings-tags-imported = Εισήχθησαν: { $added }, παραλείφθηκαν: { $skipped }
settings-tags-exported = Εξήχθησαν { $n ->
    [one] 1 ετικέτα
   *[other] { $n } ετικέτες
}

tags-chip-more = { $n ->
    [one] 1 ακόμη ετικέτα
   *[other] { $n } ακόμη ετικέτες
}

cast-section = Πρόσωπα
cast-add = Προσθήκη στα πρόσωπα…
cast-add-filter-placeholder = Φιλτράρισμα της βίβλου ιστορίας…
cast-add-empty = Δεν υπάρχουν καταχωρίσεις της βίβλου ιστορίας για προσθήκη
cast-pin = Προσθήκη του { $name } στα πρόσωπα
cast-unpin = Αφαίρεση του { $name } από τα πρόσωπα
cast-empty = Δεν έχει καρφιτσωθεί ακόμη κανείς. Προσθέστε ή κρατήστε μια πρόταση
cast-unresolved = Δεν υπάρχει πια στη βίβλο ιστορίας

mentions-confirm = Προσθήκη στα πρόσωπα του { $name }
mentions-hit-count = { $n ->
    [one] μία φορά
   *[other] { $n } φορές
}

mentions-point-of-view-badge = Οπτική γωνία
mentions-point-of-view-badge-tooltip = Δηλωμένο εδώ ως οπτική γωνία, ορισμένο με το χέρι, όχι εντοπισμένο στην πρόζα.

mentions-roster = Πρόσωπα
mentions-pin = Προσθήκη του { $name } στα πρόσωπα

pov-section = Οπτική γωνία
pov-empty = Δεν έχει οριστεί οπτική γωνία
pov-add = Ορισμός οπτικής γωνίας…
pov-multiple = Αυτή η σκηνή έχει δύο οπτικές γωνίες.
pov-remove = Αφαίρεση του { $name } ως οπτικής γωνίας
pov-unresolved = Είχε καρφιτσωθεί εδώ μια οπτική γωνία, αλλά εκείνη η καταχώριση δεν υπάρχει πια ή έχασε την ετικέτα της βίβλου ιστορίας.

books-section = Ανήκει στο
books-empty = Δεν έχει αρχειοθετηθεί ακόμη σε βιβλίο
books-add = Αρχειοθέτηση σε βιβλίο…
books-remove = Αφαίρεση από το { $name }
books-apply-to-children = Εφαρμογή της αρχειοθέτησης στα θυγατρικά

note-details-name-placeholder = Όνομα…
note-details-tags = Ετικέτες
note-details-aliases = Άλλα ονόματα
note-details-books = Ανήκει στο
note-details-books-empty = Δεν έχει αρχειοθετηθεί ακόμη σε βιβλίο
note-details-links = Σύνδεσμοι
note-details-links-empty = Δεν υπάρχουν ακόμη σύνδεσμοι. Προσθέστε έναν για να παραπέμψετε σε άλλη καταχώριση της βίβλου ιστορίας σας.
note-details-links-add = Σύνδεση με μια σημείωση…
note-details-pov = Οπτική γωνία
note-details-pov-empty = Δεν έχει οριστεί οπτική γωνία
note-details-pov-unresolved = Είχε καρφιτσωθεί εδώ μια οπτική γωνία, αλλά εκείνη η καταχώριση δεν υπάρχει πια ή έχασε την ετικέτα της βίβλου ιστορίας.
note-details-pov-multiple = Αυτή η σημείωση έχει δύο οπτικές γωνίες.
note-details-backlinks = Εμφανίζεται στο χειρόγραφο
note-details-backlinks-empty = Τίποτα ακόμη. Μόλις αυτό το όνομα εμφανιστεί στην πρόζα σας, θα εμφανιστεί εδώ.
note-details-backlinks-outside = Έξω από τα βιβλία
note-details-backlinks-confirm-all = Επιβεβαίωση κάθε εμφάνισης
note-details-backlinks-confirm-all-tooltip = Προσθήκη αυτής της καταχώρισης στα πρόσωπα κάθε εγγράφου που εμφανίζεται εδώ και δεν την έχει επιβεβαιώσει ακόμη
note-details-untitled-document = Χωρίς τίτλο

note-in-prose-mentions = { $n ->
    [one] 1 αναφορά
   *[other] { $n } αναφορές
}
note-in-prose-mention-at = { $current } από { $total }
note-in-prose-mention-previous = Προηγούμενη αναφορά
note-in-prose-mention-next = Επόμενη αναφορά
