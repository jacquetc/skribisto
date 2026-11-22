# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Rimedoj: dosieroj kaj retligiloj alkroĉitaj al eroj de la kolektujo.

## La panelo

resources-title = Rimedoj
shortcut-name-resources-show = Montri la rimedojn
resources-add-files = Aldoni dosierojn
resources-add-files-title = Aldoni dosierojn al ĉi tiu ero
resources-add-link = Aldoni retligilon
resources-drop-hint = Demetu dosierojn aŭ retadresojn ĉi tie por alkroĉi ilin
resources-no-item = Malfermu eron por vidi ĝiajn rimedojn.
resources-empty = Nenio estas ankoraŭ alkroĉita al ĉi tiu ero. Aldonu dosierojn aŭ retligilojn per la supraj butonoj, aŭ demetu ilin ĉi tie.
resources-actions = Agoj
resources-not-found = Ne trovita
resources-row-linked = Ligita dosiero
resources-row-linked-size = Ligita dosiero, { $size }
resources-row-name = { $title }, { $kind }, { $detail }
resources-row-name-missing = { $title }, { $kind }, { $detail }, ne trovita

## La dosierspecoj, kiel ekranlegilo anoncas ilin

resources-kind-picture = bildo
resources-kind-pdf = PDF-dokumento
resources-kind-audio = sonregistraĵo
resources-kind-video = video
resources-kind-document = dokumento
resources-kind-ebook = e-libro
resources-kind-archive = arkivo
resources-kind-file = dosiero
resources-kind-web-link = retligilo

## La menuo de vico

resources-open = Malfermi
resources-show-in-folder = Montri en dosierujo
resources-locate = Retrovi…
resources-insert-into-text = Enmeti en la tekston
resources-insert-picture = Enmeti la bildon en la tekston
resources-insert-link = Enmeti ligilon en la tekston
resources-edit = Alinomi kaj priskribi…
resources-detach = Malkroĉi de ĉi tiu ero
resources-remove = Forigi el la projekto…

## La du dialogoj

resources-link-dialog-title = Aldoni retligilon
resources-link-dialog-address = Adreso
resources-link-dialog-address-placeholder = https://
resources-link-dialog-name = Titolo
resources-link-dialog-name-placeholder = La nomo de la retejo, se vi lasas ĝin malplena
resources-link-dialog-add = Aldoni
resources-edit-dialog-title = Alinomi kaj priskribi
resources-edit-dialog-name = Titolo
resources-edit-dialog-description = Priskribo
resources-edit-dialog-description-placeholder = Kio ĝi estas, kaj kial vi konservis ĝin
resources-edit-dialog-save = Konservi
resources-dialog-cancel = Nuligi

## Aldoni

resources-large-title = Granda dosiero
resources-large-text = «{ $name }» grandas { $size }. Konservita en la projekto, ĝi vojaĝas kun ĉiu kopio kaj ĉiu savkopio de ĝi. Ligita, ĝi restas kie ĝi estas, kaj la projekto nur memoras kie trovi ĝin.
resources-large-text-many = «{ $name }» grandas { $size }, kaj { $others ->
    [one] unu alia dosiero estas
   *[other] { $others } aliaj dosieroj estas
} ankaŭ grandaj. Konservitaj en la projekto, ili vojaĝas kun ĉiu kopio kaj ĉiu savkopio de ĝi. Ligitaj, ili restas kie ili estas, kaj la projekto nur memoras kie trovi ilin.
resources-large-store = Konservi ĝin en la projekto
resources-large-link = Prefere ligi al ĝi
resources-large-remember = Ne demandi denove
resources-storing-title = Aldonado de dosieroj al la projekto
resources-store-cancel = Nuligi
resources-store-cancelled = Neniu dosiero estis aldonita.
resources-store-busy = Dosieroj ankoraŭ estas aldonataj. Reprovu, kiam ili estos aldonitaj.
resources-store-failed = La dosieroj ne povis esti aldonitaj: { $error }
resources-no-project = Malfermu projekton antaŭ alkroĉi dosierojn.
resources-moved-away = La projekto estis movita dum la dosiero estis kopiata.
resources-some-failed = { $count ->
    [one] Unu dosiero ne povis esti aldonita: { $names }
   *[other] { $count } dosieroj ne povis esti aldonitaj: { $names }
}
resources-add-failed = La rimedoj ne povis esti aldonitaj: { $error }
resources-added = { $count ->
    [one] Unu rimedo aldonita
   *[other] { $count } rimedoj aldonitaj
}
resources-dropped-added = { $count ->
    [one] Aldonita kiel rimedo de ĉi tiu ero, kaj ligita en la teksto
   *[other] { $count } dosieroj aldonitaj kiel rimedoj de ĉi tiu ero, kaj ligitaj en la teksto
}
resources-undo = Malfari
resources-link-refused = «{ $url }» ne estas retadreso, kiun Skribisto povas konservi. Uzu adreson, kiu komenciĝas per http:// aŭ https://, sen uzantnomo aŭ pasvorto.

## La averto por projekto en unu sola dosiero

resources-folder-title = Ĉi tiu projekto fariĝas granda
resources-folder-text = Ĝiaj konservitaj dosieroj nun okupas { $size }. Projekto tenata kiel unu sola dosiero estas tute reskribita je ĉiu konservo, ankaŭ ĝiaj dosieroj. Tenata kiel dosierujo, ĝi konservas nur tion, kio ŝanĝiĝis. Vi povas konservi ĝin kiel dosierujon nun, kaj de tiam plu labori en la dosierujo.
resources-folder-switch = Konservi kiel dosierujon…
resources-folder-not-now = Ne nun
resources-folder-never = Ne demandi denove por ĉi tiu projekto

## Malfermi

resources-not-found-toast = «{ $name }» ne estas tie, kie la projekto atendas ĝin.
resources-unsafe-title = Ĉu malfermi «{ $name }»?
resources-unsafe-text = Skribisto ne scias, ĉu ĉi tiu speco de dosiero estas sekura. Malfermi ĝin povus lanĉi programon en via komputilo, kaj projekto povas veni de iu alia. Unue montru ĝin en ĝia dosierujo por vidi kio ĝi estas.
resources-open-anyway = Malfermi tamen
resources-open-failed = La dosiero ne povis esti malfermita: { $error }
resources-no-data-folder = ĉi tiu sistemo donas al Skribisto neniun datumdosierujon, en kiu malfermi kopion
resources-link-gone = Ĉi tiu ligilo montras al rimedo, kiun la projekto ne plu havas.
resources-locate-title = Kie estas «{ $name }» nun?
resources-relinked-more = { $count ->
    [one] Ankoraŭ unu mankanta dosiero troviĝis en la sama dosierujo, kaj estis denove ligita.
   *[other] Ankoraŭ { $count } mankantaj dosieroj troviĝis en la sama dosierujo, kaj estis denove ligitaj.
}
resources-changed-elsewhere = «{ $name }» estis ŝanĝita en alia aplikaĵo. La projekto tenas la novan version.

## Redakti

resources-edit-failed = La rimedo ne povis esti ŝanĝita: { $error }
resources-remove-title = Ĉu forigi «{ $title }» el la projekto?
resources-remove-text = Ĝi estas malkroĉita de ĉiu ero, al kiu ĝi estas alkroĉita. Dosiero konservita en la projekto foriras kun ĝi; ligita dosiero kaj retpaĝo restas kie ili estas. Malfari revenigas ĝin.
resources-remove-confirm = Forigi
resources-remove-failed = La rimedo ne povis esti forigita: { $error }

## En la tekston

resources-no-text = Unue metu la kursoron en tekston, poste enmetu la rimedon tie.
resources-drop-insert-picture = Enmeti la bildon
resources-drop-insert-link = Enmeti ligilon al la rimedo

## Progreso

progress-resources-storing = Kopiado de la dosieroj
progress-resources-stored = Dosieroj kopiitaj
