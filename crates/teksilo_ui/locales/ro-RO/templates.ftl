# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

settings-page-templates = Șabloane
settings-templates-description = Piese reutilizabile de scris puteți picatura în tot ce scrieți: o foaie de caracter gol, un profil de locație, o foaie de beat. Acestea sunt stocate în acest proiect, astfel încât oricine care deschide devine același set.
settings-templates-filter = Șabloane de filtrare
settings-templates-count = { $n ->
    [0] Fără șabloane
    [one] 1 model
   *[other] Șabloane { $n }
}
settings-templates-empty = Acest proiect nu are încă șabloane.
settings-templates-empty-hint = Adăugaţi unul dintr-un preset, importaţi un fişier .md sau .djot, sau scrieţi ceva şi alegeţi Documentul
settings-templates-presets = Adaugă un preset
settings-templates-import = Importare…
settings-templates-export = Exportare…
settings-templates-star = Arată mai întâi în meniul de inserare
settings-templates-unstar = Nu mai arătați mai întâi
settings-templates-move-up = Mutare în sus
settings-templates-move-down = Mutare în jos
settings-templates-delete = Șterge șablonul
settings-templates-name = Nume
settings-templates-name-placeholder = Denumirea șablonului
settings-templates-body-placeholder = Textul introduce acest model
settings-templates-duplicate-name = Un alt șablon este deja numit "{ $name }"
settings-templates-words = { $n ->
    [one] 1 cuvânt
   *[other] { $n } cuvinte
}

templates-imported = { $added ->
    [one] 1 șablon importat
   *[other] { $added } șabloane importate
}
templates-imported-renamed = Renumit pentru a evita un conflict: { $names }
templates-imported-skipped = Nu am putut citi: { $names }
templates-import-filter = Modele (.md, .djot)
templates-exported = { $n ->
    [one] 1 model exportat
   *[other] Modele { $n } exportate
}
templates-preset-applied = Adăugat "{ $name }"

templates-delete-title = Şterge acest şablon?
templates-delete-body = "{ $name }" va fi eliminat din acest proiect. Note pe care le-ați creat deja din acesta nu sunt afectate.
templates-delete-confirm = Ștergere

menu-document = Document (&D)
menu-insert-template = Inserează șablon (&L)
menu-insert-template-none = Niciun șablon în acest proiect
menu-save-as-template = Salvează ca șablon... (&V)

save-as-template-title = Salvează ca șablon
save-as-template-explain = Textul pe care îl editați devine un șablon pe care îl puteți introduce oriunde altundeva.
save-as-template-name = Nume
save-as-template-placeholder = Foaie de caractere
save-as-template-duplicate = Există deja un șablon numit "{ $name }"
save-as-template-empty-editor = Editorul e gol, nu e nimic de salvat.
save-as-template-confirm = Salvează șablonul
save-as-template-saved = Salvat "{ $name }" ca șablon
template-inserted = Se introduce "{ $name }"

note-template-preset-character-sheet = Foaie de caractere
note-template-preset-location = Locație
note-template-preset-artifact = Obiect
note-template-preset-beat-sheet = Foaie de bătaie
note-template-preset-faction = Facţiune
note-template-preset-research-note = Notă de cercetare

note-template-set-essentials = Elemente esenţiale
note-template-set-everything = Fiecare șablon

note-template-section-identity = Identitate
note-template-section-appearance = Aspect
note-template-section-voice = Voce
note-template-section-psychology = Viaţa interioară
note-template-section-history = Istoric
note-template-section-arc = Evoluție narativă
note-template-section-first-impression = Prima impresie
note-template-section-in-the-story = În poveste
note-template-section-the-thing-itself = Obiectul în sine
note-template-section-the-scene = Scena
note-template-section-the-shape = Forma
note-template-section-what-it-is = Ce este?
note-template-section-source = Sursă
note-template-section-what-it-says = Ceea ce spune

note-template-field-full-name = Nume complet
note-template-field-known-as = De asemenea, cunoscut sub numele de, urmărit în câmpul Alias inspectorului
note-template-field-age = Vârstă
note-template-field-role-in-story = Rolul în poveste
note-template-field-build-and-features = Construiește și distinge caracteristicile
note-template-field-habitual-bearing = Rulmenți obişnuiţi
note-template-field-speech-patterns = Tipare de vorbire și ticuri verbale
note-template-field-what-they-never-say = Ceea ce ei nu spun niciodată
note-template-field-want = Ce vor ei
note-template-field-need = Ceea ce au nevoie, dar nu-l cunosc
note-template-field-fear = De ce se tem?
note-template-field-flaw = Defectul care-i costă.
note-template-field-formative-event = Eveniment formativ
note-template-field-relationships = Relaţii
note-template-field-starts-as = Începe ca
note-template-field-ends-as = Se termină ca
note-template-field-what-you-notice-first = Ceea ce observați mai întâi
note-template-field-sound-and-smell = Sunet şi miros
note-template-field-light-and-weather = Lumină și vreme
note-template-field-what-happened-here = Ce s-a întâmplat aici?
note-template-field-who-lives-or-works-here = Cine trăiește sau proiectează aici
note-template-field-scenes-set-here = Scene stabilite aici
note-template-field-why-it-matters = De ce contează?
note-template-field-appearance = Aspect
note-template-field-age-and-origin = Vârsta și originea
note-template-field-what-it-does = Ceea ce face
note-template-field-who-holds-it = Cine o deţine?
note-template-field-who-wants-it = Cine o vrea?
note-template-field-pov = Punct de vedere
note-template-field-time-and-place = Ora și locul
note-template-field-goal = Obiectiv
note-template-field-conflict = Conflict
note-template-field-turn = Rândul
note-template-field-exit-emotion = Emoție pe cale de ieșire
note-template-field-purpose = Scop
note-template-field-who-leads-it = Cine o conduce?
note-template-field-resources = Resurse
note-template-field-allies-and-enemies = Aliaţi şi duşmani
note-template-field-what-it-wants-now = Ce vrea acum
note-template-field-where-from = De unde a venit
note-template-field-page-or-link = Pagina sau linkul
note-template-field-key-facts = Date cheie
note-template-field-how-it-is-used = Cum se utilizează în poveste
