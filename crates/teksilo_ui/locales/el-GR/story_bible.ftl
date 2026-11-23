# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

story-bible-modal-configure-title = Ρύθμιση της νέας καταχώρισης
story-bible-modal-name-label = Όνομα
story-bible-modal-name-placeholder = Ένα πρόσωπο, ένας τόπος, οτιδήποτε αξίζει να καταγραφεί…
story-bible-modal-no-binders = Αυτό το έργο δεν έχει ακόμη ντοσιέ για να την αρχειοθετήσετε
story-bible-modal-template-label = Έναρξη από ένα πρότυπο (προαιρετικό)
story-bible-modal-template-placeholder = Χωρίς πρότυπο
story-bible-modal-body-label = Σώμα
story-bible-modal-cancel = Ακύρωση
story-bible-modal-create = Δημιουργία
story-bible-modal-create-and-open = Δημιουργία και άνοιγμα
story-bible-modal-failed = Αδυναμία δημιουργίας της καταχώρισης

ctx-add-as-note = Προσθήκη ως &σημείωση…
ctx-add-as-note-all-tags = Όλες οι ετικέτες…
ctx-add-as-note-untagged = Χωρίς ετικέτα

story-bible-grid-label = Βίβλος ιστορίας
story-bible-grid-untagged = Δεν έχει ακόμα ετικέτα
story-bible-grid-empty-title = Δεν έχει καταγραφεί ακόμη τίποτα εδώ
story-bible-grid-empty-hint = Βάλτε σε μια σημείωση ετικέτα εύρεσης στην πρόζα ή προσθέστε μία από μια επιλογή, και θα εμφανιστεί εδώ.
story-bible-grid-alias-count = { $count ->
    [0] Κανένα εναλλακτικό όνομα ακόμη
    [one] { $count } εναλλακτικό όνομα
   *[other] { $count } εναλλακτικά ονόματα
}

story-bible-grid-mention-count = { $count ->
    [0] Καμία εμφάνιση ακόμη
    [one] Εμφανίζεται σε { $count } σκηνή σε όλο το έργο
   *[other] Εμφανίζεται σε { $count } σκηνές σε όλο το έργο
}
story-bible-books-filter-all = Όλα τα βιβλία

story-bible-grid-not-searched = Δεν έχει αναζητηθεί ακόμη: χρειάζεται ετικέτα βίβλου ιστορίας

note-in-prose-pov = Οπτική γωνία
note-in-prose-cast = Πρόσωπα
note-in-prose-pov-and-cast = Οπτική γωνία · Πρόσωπα
note-in-prose-no-books = Αυτό το έργο δεν έχει ακόμη βιβλίο
# The page lists only scenes where the writer placed the entry in the cast or the point of
# view, never the ones the scan merely found its name in, so an empty page says so: a bare
# "nothing here" read as a search that had failed.
note-in-prose-empty-book = Καμία σκηνή αυτού του βιβλίου δεν έχει ακόμη αυτή την καταχώριση στα πρόσωπα ή στην οπτική γωνία της.
note-in-prose-empty-viewpoint = Καμία σκηνή αυτού του βιβλίου δεν είναι ακόμη γραμμένη από την οπτική γωνία αυτής της καταχώρισης.
note-in-prose-scope = Η σελίδα «Στην πρόζα» δείχνει μόνο τις σκηνές που επιβεβαιώσατε. Μια σκηνή που κατονομάζει αυτή την καταχώριση μένει πρόταση μέχρι να προσθέσετε την καταχώριση στα πρόσωπά της, στις «Λεπτομέρειες» ή στον επιθεωρητή.
# A switch beside the Book bar: keep only the scenes this entry narrates.
note-in-prose-viewpoint-only = Μόνο σκηνές από την οπτική της
note-in-prose-viewpoint-only-tooltip = Εμφάνιση μόνο των σκηνών που είναι γραμμένες από την οπτική γωνία αυτής της καταχώρισης, και απόκρυψη εκείνων όπου απλώς εμφανίζεται

story-bible-capture-toast = Το «{ $name }» προστέθηκε στη βίβλο ιστορίας
story-bible-capture-open = Άνοιγμα
story-bible-capture-undo = Αναίρεση
story-bible-capture-where-title = Πού πάνε αυτές οι σημειώσεις;
story-bible-capture-where-prompt = Επιλέξτε τον φάκελο στον οποίο θα αρχειοθετούνται οι νέες σημειώσεις.
story-bible-capture-where-confirm = Αρχειοθέτηση εδώ
story-bible-capture-where-needs-folder = Επιλέξτε έναν φάκελο μέσα στον οποίο θα μπουν αυτές οι σημειώσεις.
story-bible-capture-where-tag = Ερωτάται μία φορά. Οι σημειώσεις με την ετικέτα «{ $tag }» θα πηγαίνουν εδώ από τώρα και στο εξής, και μπορείτε να το αλλάξετε αργότερα στις Ρυθμίσεις, Έργο, Ετικέτες.
story-bible-capture-where-untagged = Ερωτάται μία φορά. Οι σημειώσεις χωρίς ετικέτα θα πηγαίνουν εδώ από τώρα και στο εξής, και μπορείτε να το αλλάξετε αργότερα στις Ρυθμίσεις, Έργο, Ετικέτες.

# What a screen reader hears for a binder row that is a story bible entry: its icon
# says so on screen, and this says it aloud. The second form keeps the row's label.
binder-row-story-bible-entry = Καταχώριση βίβλου ιστορίας
binder-row-story-bible-entry-with-label = Καταχώριση βίβλου ιστορίας, { $label }
