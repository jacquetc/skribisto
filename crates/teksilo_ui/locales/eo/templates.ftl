# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

settings-page-templates = Ŝablonoj
settings-templates-description = Reuzeblaj tekstoj por enmeti dum verkado: malplena rolulfiŝo, lokpriskribo aŭ scenskemo. Ili konserviĝas en ĉi tiu projekto, do ĉiu malfermanto ricevas la saman aron.
settings-templates-filter = Filtri ŝablonojn
settings-templates-count = { $n ->
    [0] Neniuj ŝablonoj
    [one] 1 ŝablono
   *[other] { $n } ŝablonoj
}
settings-templates-empty = Ĉi tiu projekto ankoraŭ ne havas ŝablonojn.
settings-templates-empty-hint = Aldonu antaŭagordon, importu .md- aŭ .djot-dosieron, aŭ verku tekston kaj elektu Dokumento ▸ Konservi kiel ŝablonon.
settings-templates-presets = Aldoni antaŭagordon
settings-templates-import = Importi…
settings-templates-export = Eksporti…
settings-templates-star = Montri unue en la enmetmenuo
settings-templates-unstar = Ne plu montri unue
settings-templates-move-up = Movi supren
settings-templates-move-down = Movi malsupren
settings-templates-delete = Forigi ŝablonon
settings-templates-name = Nomo
settings-templates-name-placeholder = Ŝablonnomo
settings-templates-body-placeholder = La teksto enmetota de ĉi tiu ŝablono
settings-templates-duplicate-name = Alia ŝablono jam nomiĝas «{ $name }»
settings-templates-words = { $n ->
    [one] 1 vorto
   *[other] { $n } vortoj
}
templates-imported = { $added ->
    [one] 1 ŝablono importita
   *[other] { $added } ŝablonoj importitaj
}
templates-imported-renamed = Alinomitaj por eviti konflikton: { $names }
templates-imported-skipped = Ne eblis legi: { $names }
templates-import-filter = Ŝablonoj (.md, .djot)
templates-exported = { $n ->
    [one] 1 ŝablono eksportita
   *[other] { $n } ŝablonoj eksportitaj
}
templates-preset-applied = Aldonita: «{ $name }»
templates-delete-title = Ĉu forigi ĉi tiun ŝablonon?
templates-delete-body = «{ $name }» estos forigita el ĉi tiu projekto. Notoj jam kreitaj per ĝi restas senŝanĝaj.
templates-delete-confirm = Forigi
menu-document = &Dokumento
menu-insert-template = E&nmeti ŝablonon
menu-insert-template-none = Neniuj ŝablonoj en ĉi tiu projekto
menu-save-as-template = &Konservi kiel ŝablonon…
save-as-template-title = Konservi kiel ŝablonon
save-as-template-explain = La redaktata teksto fariĝas ŝablono, kiun vi povas enmeti aliloke.
save-as-template-name = Nomo
save-as-template-placeholder = Rolulfiŝo
save-as-template-duplicate = Ŝablono «{ $name }» jam ekzistas
save-as-template-empty-editor = Ĉi tiu redaktilo estas malplena. Nenio konservenda.
save-as-template-confirm = Konservi ŝablonon
save-as-template-saved = «{ $name }» konservita kiel ŝablono
template-inserted = Enmetita: «{ $name }»
note-template-preset-character-sheet = Rolulfiŝo
note-template-preset-location = Loko
note-template-preset-artifact = Objekto
note-template-preset-beat-sheet = Scenskemo
note-template-preset-faction = Frakcio
note-template-preset-research-note = Esplornoto
note-template-set-essentials = Bazaj
note-template-set-everything = Ĉiuj ŝablonoj
note-template-section-identity = Identeco
note-template-section-appearance = Aspekto
note-template-section-voice = Voĉo
note-template-section-psychology = Interna vivo
note-template-section-history = Historio
note-template-section-arc = Evoluo
note-template-section-first-impression = Unua impreso
note-template-section-in-the-story = En la rakonto
note-template-section-the-thing-itself = La objekto mem
note-template-section-the-scene = La sceno
note-template-section-the-shape = La formo
note-template-section-what-it-is = Kio ĝi estas
note-template-section-source = Fonto
note-template-section-what-it-says = Kion ĝi diras
note-template-field-full-name = Plena nomo
note-template-field-known-as = Aliaj nomoj, sekvataj en la kampo Ankaŭ nomata de la Inspektilo
note-template-field-age = Aĝo
note-template-field-role-in-story = Rolo en la rakonto
note-template-field-build-and-features = Korpo kaj distingaj trajtoj
note-template-field-habitual-bearing = Kutima sinteno
note-template-field-speech-patterns = Parolmaniero kaj vortaj kutimoj
note-template-field-what-they-never-say = Kion la rolulo neniam diras
note-template-field-want = Kion la rolulo volas
note-template-field-need = Kion la rolulo bezonas sen scii tion
note-template-field-fear = Kion la rolulo timas
note-template-field-flaw = La malforto kun sekvoj
note-template-field-formative-event = Formanta evento
note-template-field-relationships = Rilatoj
note-template-field-starts-as = Komenca stato
note-template-field-ends-as = Fina stato
note-template-field-what-you-notice-first = Kion oni unue rimarkas
note-template-field-sound-and-smell = Sonoj kaj odoroj
note-template-field-light-and-weather = Lumo kaj vetero
note-template-field-what-happened-here = Kio okazis ĉi tie
note-template-field-who-lives-or-works-here = Kiu loĝas aŭ laboras ĉi tie
note-template-field-scenes-set-here = Scenoj okazantaj ĉi tie
note-template-field-why-it-matters = Kial ĝi gravas
note-template-field-appearance = Aspekto
note-template-field-age-and-origin = Aĝo kaj origino
note-template-field-what-it-does = Kion ĝi faras
note-template-field-who-holds-it = Kiu posedas ĝin
note-template-field-who-wants-it = Kiu volas ĝin
note-template-field-pov = Vidpunkto
note-template-field-time-and-place = Tempo kaj loko
note-template-field-goal = Celo
note-template-field-conflict = Konflikto
note-template-field-turn = Turnopunkto
note-template-field-exit-emotion = Fina emocio
note-template-field-purpose = Celo
note-template-field-who-leads-it = Kiu gvidas ĝin
note-template-field-resources = Rimedoj
note-template-field-allies-and-enemies = Aliancanoj kaj malamikoj
note-template-field-what-it-wants-now = Kion ĝi nun volas
note-template-field-where-from = De kie ĝi venis
note-template-field-page-or-link = Paĝo aŭ ligilo
note-template-field-key-facts = Ĉefaj faktoj
note-template-field-how-it-is-used = Kiel ĝi estas uzata en la rakonto
