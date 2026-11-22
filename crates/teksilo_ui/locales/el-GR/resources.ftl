# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Πόροι: αρχεία και σύνδεσμοι ιστού συνδεδεμένοι με στοιχεία του ντοσιέ.

## Το πλαίσιο

resources-title = Πόροι
shortcut-name-resources-show = Εμφάνιση των πόρων
resources-add-files = Προσθήκη αρχείων
resources-add-files-title = Προσθήκη αρχείων σε αυτό το στοιχείο
resources-add-link = Προσθήκη συνδέσμου ιστού
resources-drop-hint = Αφήστε εδώ αρχεία ή διευθύνσεις ιστού για να τα επισυνάψετε
resources-no-item = Ανοίξτε ένα στοιχείο για να δείτε τους πόρους του.
resources-empty = Δεν έχει επισυναφθεί ακόμη τίποτα σε αυτό το στοιχείο. Προσθέστε αρχεία ή συνδέσμους ιστού με τα κουμπιά παραπάνω, ή αφήστε τα εδώ.
resources-actions = Ενέργειες
resources-not-found = Δεν βρέθηκε
resources-row-linked = Συνδεδεμένο αρχείο
resources-row-linked-size = Συνδεδεμένο αρχείο, { $size }
resources-row-name = { $title }, { $kind }, { $detail }
resources-row-name-missing = { $title }, { $kind }, { $detail }, δεν βρέθηκε

## Οι τύποι αρχείων, όπως τους εκφωνεί ένας αναγνώστης οθόνης

resources-kind-picture = εικόνα
resources-kind-pdf = έγγραφο PDF
resources-kind-audio = ηχογράφηση
resources-kind-video = βίντεο
resources-kind-document = έγγραφο
resources-kind-ebook = ηλεκτρονικό βιβλίο
resources-kind-archive = συμπιεσμένο αρχείο
resources-kind-file = αρχείο
resources-kind-web-link = σύνδεσμος ιστού

## Το μενού μιας γραμμής

resources-open = Άνοιγμα
resources-show-in-folder = Εμφάνιση στον φάκελο
resources-locate = Εντοπισμός…
resources-insert-into-text = Παρεμβολή στο κείμενο
resources-insert-picture = Παρεμβολή της εικόνας στο κείμενο
resources-insert-link = Παρεμβολή συνδέσμου στο κείμενο
resources-edit = Μετονομασία και περιγραφή…
resources-detach = Αποσύνδεση από αυτό το στοιχείο
resources-remove = Αφαίρεση από το έργο…

## Τα δύο παράθυρα διαλόγου

resources-link-dialog-title = Προσθήκη συνδέσμου ιστού
resources-link-dialog-address = Διεύθυνση
resources-link-dialog-address-placeholder = https://
resources-link-dialog-name = Τίτλος
resources-link-dialog-name-placeholder = Το όνομα του ιστότοπου, αν μείνει κενό
resources-link-dialog-add = Προσθήκη
resources-edit-dialog-title = Μετονομασία και περιγραφή
resources-edit-dialog-name = Τίτλος
resources-edit-dialog-description = Περιγραφή
resources-edit-dialog-description-placeholder = Τι είναι αυτό, και γιατί το κρατήσατε
resources-edit-dialog-save = Αποθήκευση
resources-dialog-cancel = Ακύρωση

## Προσθήκη

resources-large-title = Ένα μεγάλο αρχείο
resources-large-text = Το «{ $name }» έχει μέγεθος { $size }. Αποθηκευμένο μέσα στο έργο, ταξιδεύει με κάθε αντίγραφο και κάθε αντίγραφο ασφαλείας του. Συνδεδεμένο, μένει εκεί που είναι, και το έργο θυμάται μόνο πού να το βρει.
resources-large-text-many = Το «{ $name }» έχει μέγεθος { $size }, και { $others ->
    [one] ένα ακόμη αρχείο είναι
   *[other] { $others } ακόμη αρχεία είναι
} επίσης μεγάλα. Αποθηκευμένα μέσα στο έργο, ταξιδεύουν με κάθε αντίγραφο και κάθε αντίγραφο ασφαλείας του. Συνδεδεμένα, μένουν εκεί που είναι, και το έργο θυμάται μόνο πού να τα βρει.
resources-large-store = Αποθήκευση μέσα στο έργο
resources-large-link = Σύνδεση σε αυτό, χωρίς αποθήκευση
resources-large-remember = Να μη γίνει ξανά ερώτηση
resources-storing-title = Προσθήκη αρχείων στο έργο
resources-store-cancel = Ακύρωση
resources-store-cancelled = Δεν προστέθηκε κανένα αρχείο.
resources-store-busy = Προστίθενται ακόμη αρχεία. Δοκιμάστε ξανά όταν ολοκληρωθεί η προσθήκη τους.
resources-store-failed = Δεν ήταν δυνατή η προσθήκη των αρχείων: { $error }
resources-no-project = Ανοίξτε ένα έργο πριν επισυνάψετε αρχεία.
resources-moved-away = Το έργο μετακινήθηκε ενώ αντιγραφόταν το αρχείο.
resources-some-failed = { $count ->
    [one] Δεν ήταν δυνατή η προσθήκη ενός αρχείου: { $names }
   *[other] Δεν ήταν δυνατή η προσθήκη { $count } αρχείων: { $names }
}
resources-add-failed = Δεν ήταν δυνατή η προσθήκη των πόρων: { $error }
resources-added = { $count ->
    [one] Προστέθηκε ένας πόρος
   *[other] Προστέθηκαν { $count } πόροι
}
resources-dropped-added = { $count ->
    [one] Προστέθηκε ως πόρος αυτού του στοιχείου, και συνδέθηκε στο κείμενο
   *[other] { $count } αρχεία προστέθηκαν ως πόροι αυτού του στοιχείου, και συνδέθηκαν στο κείμενο
}
resources-undo = Αναίρεση
resources-link-refused = Η διεύθυνση «{ $url }» δεν είναι διεύθυνση ιστού που μπορεί να κρατήσει το Skribisto. Χρησιμοποιήστε μια διεύθυνση που αρχίζει με http:// ή https://, χωρίς όνομα χρήστη ή κωδικό πρόσβασης.

## Η ειδοποίηση για ένα έργο σε ένα μόνο αρχείο

resources-folder-title = Αυτό το έργο μεγαλώνει
resources-folder-text = Τα αποθηκευμένα αρχεία του καταλαμβάνουν πλέον { $size }. Ένα έργο που φυλάσσεται ως ένα μόνο αρχείο γράφεται ολόκληρο σε κάθε αποθήκευση, μαζί με τα αρχεία του. Ως φάκελος, αποθηκεύει μόνο ό,τι άλλαξε. Μπορείτε να το αποθηκεύσετε ως φάκελο τώρα, και να συνεχίσετε να δουλεύετε στον φάκελο από εκεί και πέρα.
resources-folder-switch = Αποθήκευση ως φάκελος…
resources-folder-not-now = Όχι τώρα
resources-folder-never = Να μη γίνει ξανά ερώτηση για αυτό το έργο

## Άνοιγμα

resources-not-found-toast = Το «{ $name }» δεν βρίσκεται εκεί που το περιμένει το έργο.
resources-unsafe-title = Άνοιγμα του «{ $name }»;
resources-unsafe-text = Το Skribisto δεν ξέρει αν αυτό το είδος αρχείου είναι ασφαλές. Αν το ανοίξετε, μπορεί να ξεκινήσει ένα πρόγραμμα στον υπολογιστή σας, και ένα έργο μπορεί να προέρχεται από κάποιον άλλο. Εμφανίστε το πρώτα στον φάκελό του για να δείτε τι είναι.
resources-open-anyway = Άνοιγμα παρ' όλα αυτά
resources-open-failed = Δεν ήταν δυνατό το άνοιγμα του αρχείου: { $error }
resources-no-data-folder = αυτό το σύστημα δεν δίνει στο Skribisto κανέναν φάκελο δεδομένων για να ανοίξει ένα αντίγραφο
resources-link-gone = Αυτός ο σύνδεσμος οδηγεί σε έναν πόρο που το έργο δεν έχει πια.
resources-locate-title = Πού βρίσκεται τώρα το «{ $name }»;
resources-relinked-more = { $count ->
    [one] Ένα ακόμη αρχείο που έλειπε βρέθηκε στον ίδιο φάκελο, και συνδέθηκε ξανά.
   *[other] { $count } ακόμη αρχεία που έλειπαν βρέθηκαν στον ίδιο φάκελο, και συνδέθηκαν ξανά.
}
resources-changed-elsewhere = Το «{ $name }» άλλαξε σε άλλη εφαρμογή. Το έργο κρατά τη νέα έκδοση.

## Επεξεργασία

resources-edit-failed = Δεν ήταν δυνατή η αλλαγή του πόρου: { $error }
resources-remove-title = Αφαίρεση του «{ $title }» από το έργο;
resources-remove-text = Αφαιρείται από κάθε στοιχείο με το οποίο είναι συνδεδεμένος. Ένα αρχείο αποθηκευμένο μέσα στο έργο φεύγει μαζί του. Ένα συνδεδεμένο αρχείο και μια ιστοσελίδα μένουν εκεί που είναι. Η αναίρεση τον επαναφέρει.
resources-remove-confirm = Αφαίρεση
resources-remove-failed = Δεν ήταν δυνατή η αφαίρεση του πόρου: { $error }

## Στο κείμενο

resources-no-text = Τοποθετήστε πρώτα τον δρομέα σε ένα κείμενο, και μετά παρεμβάλετε εκεί τον πόρο.
resources-drop-insert-picture = Παρεμβολή της εικόνας
resources-drop-insert-link = Παρεμβολή συνδέσμου προς τον πόρο

## Πρόοδος

progress-resources-storing = Αντιγραφή των αρχείων
progress-resources-stored = Τα αρχεία αντιγράφηκαν
