# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

tags-preset-basic = Βασικό
tags-preset-scifi = Επιστημονική φαντασία
tags-preset-fantasy = Φαντασία
tags-preset-mystery = Μυστήριο
tags-preset-historical = Ιστορικό


tags-preset-needs-research = χρειάζεται έρευνα
tags-preset-continuity-check = έλεγχος συνέχειας
tags-preset-plot-point = σημείο γραφικής παράστασης

tags-preset-character = χαρακτήρα
tags-preset-place = θέση
tags-preset-item = Θέση

tags-preset-vessel = σκάφος
tags-preset-planet = πλανήτης
tags-preset-organization = οργάνωση
tags-preset-creature = πλάσμα
tags-preset-faction = Φατρία
tags-preset-artifact = τεχνουργήματα
tags-preset-realm = βασίλειο
tags-preset-magic-system = μαγικό σύστημα
tags-preset-suspect = ύποπτος
tags-preset-victim = θύμα
tags-preset-clue = στοιχείο
tags-preset-red-herring = ερυθρό ρέγγα
tags-preset-historical-figure = ιστορικό σχήμα
tags-preset-source = πηγή
tags-preset-period-detail = λεπτομέρειες της περιόδου

tags-pill-list = Ετικέτες
tags-pill-add = Προσθήκη ετικέτας
tags-none-yet = Χωρίς ετικέτες
tags-pick-list = Ετικέτες για επιλογή από
tags-pill-remove = Αφαίρεση { $name }
tags-pill-filter-placeholder = Φίλτρο ή όνομα νέας ετικέτας
tags-pill-no-match = Δεν ταιριάζει με ετικέτα
tags-pill-create = Δημιουργήστε το -{ $name }.
tags-pill-create-failed = Δεν μπόρεσα να δημιουργήσω το { $name }.
tags-pill-new-discoverable = Εύρεση στην τιμή
tags-pill-new-discoverable-hint = Τα στοιχεία με αυτή την ετικέτα ταιριάζουν ενάντια στην πρόζα σας για να γεμίσετε το ρόστερ.

tags-alias-list = Γνωστό και ως
tags-alias-add = Προσθήκη άλλου ονόματος
tags-alias-remove = Αφαίρεση { $name }
tags-alias-placeholder = Ένα άλλο όνομα, στη συνέχεια, εισάγετε
tags-alias-hint = Ονόματα αυτά εμφανίζονται στην πεζογραφία σας, εκτός από τον τίτλο του.
tags-alias-collision = { $name } απαντά ήδη σε αυτό το όνομα.

settings-page-tags = Ετικέτες
settings-tags-desc = Ετικέτες ετικέτα τα στοιχεία στο ντοσιέ σας. Μια ετικέτα εύρεσης-σε-προσφορά λέει επίσης στο Skribisto να ψάξουν για τα ονόματα αυτού του αντικειμένου στην πεζογραφία σας.
settings-tags-add = Προσθήκη ετικέτας
settings-tags-add-placeholder = Όνομα μιας νέας ετικέτας
settings-tags-added = Προστέθηκε -{ $name } .
settings-tags-duplicate = -{ $name } - υπάρχει ήδη
settings-tags-filter = Ετικέτα φίλτρου
settings-tags-count = { $n ->
    [one] 1 ετικέτα
   *[other] { $n } ετικέτες
}
settings-tags-details-placeholder = Τι σημαίνει αυτή η ετικέτα
settings-tags-discoverable = Βρείτε στην πεζογραφία
settings-tags-creates-in = Νέες σημειώσεις
settings-tags-creates-in-unset = Ρώτα με την πρώτη φορά.
settings-tags-creates-in-untitled = Χωρίς τίτλο φάκελος
settings-tags-creates-in-trashed = { $name } (στα σκουπίδια)
settings-tags-template = Εκκίνηση προτύπου
settings-tags-template-unset = Κενή σημείωση
settings-tags-delete = Διαγραφή { $name }
settings-tags-deleted = Διαγράφηκε από το --{ $name } - και το αφαίρεσε από κάθε αντικείμενο
settings-tags-empty = Δεν υπάρχουν ακόμα ταμπέλες.
settings-tags-apply-preset = Εφαρμογή προεπιλογής...
settings-tags-preset-applied = Προστέθηκε { $added }, παραλείφθηκε { $skipped } ήδη παρούσα
settings-tags-csv-filter = CSV αρχεία
settings-tags-import = Εισαγωγή…
settings-tags-export = Εξαγωγή…
settings-tags-imported = Εισήχθη { $added }, παραλήφθηκε { $skipped }
settings-tags-exported = Εξαγωγή { $n ->
    [one] 1 ετικέτα
   *[other] { $n } ετικέτες
}

tags-chip-more = { $n ->
    [one] 1 ακόμη ετικέτα
   *[other] { $n } περισσότερες ετικέτες
}

cast-section = Χαρακτήρες
cast-add = Προσθήκη στο cast...
cast-add-filter-placeholder = Bίβλος ιστοριών φίλτρου...
cast-add-empty = Δεν υπάρχουν καταχωρήσεις για την ιστορία-βίβλος
cast-pin = Προσθήκη { $name } στο cast
cast-unpin = Αφαιρέστε το { $name } από το cast
cast-empty = Κανείς δεν έχει καρφωθεί ακόμα.
cast-unresolved = Όχι πια στη Βίβλο της ιστορίας.

mentions-confirm = Προσθήκη στο καστ του { $name }
mentions-hit-count = { $n ->
    [one] μία φορά
   *[other] { $n } φορές
}
mentions-point-of-view-badge = Οπτική γωνία
mentions-point-of-view-badge-tooltip = Δηλώθηκε ως η άποψη εδώ, που τίθεται με το χέρι, δεν ανιχνεύεται στην πεζογραφία.

mentions-roster = Χαρακτήρες
mentions-pin = Προσθήκη { $name } στο cast

pov-section = Οπτική γωνία
pov-empty = Δεν έχει οριστεί άποψη
pov-add = Ορισμός άποψης...
pov-multiple = Αυτή η σκηνή έχει δύο απόψεις.
pov-remove = Αφαίρεση { $name } ως άποψη
pov-unresolved = Μια άποψη ήταν καρφωμένη εδώ, αλλά αυτή η καταχώρηση δεν υπάρχει πια ή έχασε την παραμυθά-βιβλιοθήκη της.

books-section = Κατατίθενται στο πλαίσιο
books-empty = Δεν έχει κατατεθεί ακόμα σε βιβλίο.
books-add = Αρχείο κάτω από ένα βιβλίο...
books-remove = Αφαίρεση από { $name }
books-apply-to-children = Εφαρμογή αρχειοθέτησης σε παιδιά

note-details-name-placeholder = Όνομα…
note-details-tags = Ετικέτες
note-details-aliases = Άλλες ονομασίες
note-details-books = Κατατίθενται στο πλαίσιο
note-details-books-empty = Δεν έχει κατατεθεί ακόμα σε βιβλίο.
note-details-links = Σύνδεσμοι
note-details-links-empty = Δεν υπάρχουν σύνδεσμοι ακόμα. Προσθέστε ένα σημείο σε μια άλλη καταχώρηση στη Βίβλο ιστορία σας.
note-details-links-add = Σύνδεση με ένα σημείωμα...
note-details-pov = Οπτική γωνία
note-details-pov-empty = Δεν έχει οριστεί άποψη
note-details-pov-unresolved = Μια άποψη ήταν καρφωμένη εδώ, αλλά αυτή η καταχώρηση δεν υπάρχει πια ή έχασε την παραμυθά-βιβλιοθήκη της.
note-details-pov-multiple = Αυτό το σημείωμα έχει δύο απόψεις.
note-details-backlinks = Εμφανίζεται στο χειρόγραφο
note-details-backlinks-empty = Μόλις εμφανιστεί αυτό το όνομα στην πρόζα σου, θα εμφανιστεί εδώ.
note-details-backlinks-outside = Έξω από τα βιβλία
note-details-backlinks-confirm-all = Επιβεβαίωση κάθε εμφάνισης
note-details-backlinks-confirm-all-tooltip = Προσθήκη αυτής της καταχώρησης στο cast κάθε εγγράφου που εμφανίζεται εδώ και δεν το έχει επιβεβαιώσει ακόμα
note-details-untitled-document = Χωρίς τίτλο

note-in-prose-mentions = { $n ->
    [one] 1 αναφορά
   *[other] { $n } αναφέρει
}
note-in-prose-mention-at = { $current } από { $total }
note-in-prose-mention-previous = Προηγούμενη αναφορά
note-in-prose-mention-next = Επόμενη αναφορά
