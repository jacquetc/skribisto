# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

settings-page-templates = Sablonok
settings-templates-description = Újrahasználható szövegrészek, amelyeket bármibe beilleszthet, amit éppen ír: üres szereplőlap, helyszínprofil, jelenetvázlat. A projektben tárolódnak, így aki megnyitja, ugyanazt a készletet kapja.
settings-templates-filter = Sablonok szűrése
settings-templates-count = { $n ->
    [0] Nincsenek sablonok
    [one] 1 sablon
   *[other] { $n } sablon
}
settings-templates-empty = Ennek a projektnek még nincsenek sablonjai.
settings-templates-empty-hint = Adjon hozzá egyet a mintákból, importáljon egy .md vagy .djot fájlt, vagy írjon valamit, és válassza a Dokumentum ▸ Mentés sablonként parancsot.
settings-templates-presets = Minta hozzáadása
settings-templates-import = Importálás…
settings-templates-export = Exportálás…
settings-templates-star = Megjelenítés elsőként a beszúrási menüben
settings-templates-unstar = Ne jelenjen meg elsőként
settings-templates-move-up = Mozgatás fel
settings-templates-move-down = Mozgatás le
settings-templates-delete = Sablon törlése
settings-templates-name = Név
settings-templates-name-placeholder = A sablon neve
settings-templates-body-placeholder = A szöveg, amelyet a sablon beszúr
settings-templates-duplicate-name = Már van „{ $name }” nevű sablon
settings-templates-words = { $n ->
    [one] 1 szó
   *[other] { $n } szó
}
templates-imported = { $added ->
    [one] 1 sablon importálva
   *[other] { $added } sablon importálva
}
templates-imported-renamed = Az ütközés elkerülése érdekében átnevezve: { $names }
templates-imported-skipped = Nem sikerült beolvasni: { $names }
templates-import-filter = Sablonok (.md, .djot)
templates-exported = { $n ->
    [one] 1 sablon exportálva
   *[other] { $n } sablon exportálva
}
templates-preset-applied = Hozzáadva: „{ $name }”
templates-delete-title = Törli ezt a sablont?
templates-delete-body = A(z) „{ $name }” kikerül a projektből. Az ebből már létrehozott jegyzeteket ez nem érinti.
templates-delete-confirm = Törlés
menu-document = &Dokumentum
menu-insert-template = &Sablon beszúrása
menu-insert-template-none = Ebben a projektben nincsenek sablonok
menu-save-as-template = Mentés s&ablonként…
save-as-template-title = Mentés sablonként
save-as-template-explain = A szerkesztett szövegből sablon lesz, amelyet bárhol máshol beilleszthet.
save-as-template-name = Név
save-as-template-placeholder = Szereplőlap
save-as-template-duplicate = Már létezik „{ $name }” nevű sablon
save-as-template-empty-editor = Ez a szerkesztő üres. Nincs mit menteni.
save-as-template-confirm = Sablon mentése
save-as-template-saved = „{ $name }” mentve sablonként
template-inserted = Beszúrva: „{ $name }”
note-template-preset-character-sheet = Szereplőlap
note-template-preset-location = Helyszín
note-template-preset-artifact = Tárgy
note-template-preset-beat-sheet = Jelenetvázlat
note-template-preset-faction = Frakció
note-template-preset-research-note = Kutatási jegyzet
note-template-set-essentials = Alapkészlet
note-template-set-everything = Minden sablon
note-template-section-identity = Személyazonosság
note-template-section-appearance = Megjelenés
note-template-section-voice = Hang
note-template-section-psychology = Belső világ
note-template-section-history = Előtörténet
note-template-section-arc = Fejlődési ív
note-template-section-first-impression = Első benyomás
note-template-section-in-the-story = A történetben
note-template-section-the-thing-itself = Maga a tárgy
note-template-section-the-scene = A jelenet
note-template-section-the-shape = A felépítés
note-template-section-what-it-is = Mi ez
note-template-section-source = Forrás
note-template-section-what-it-says = Mit mond
note-template-field-full-name = Teljes név
note-template-field-known-as = Más néven (a Tulajdonságok panel Más néven mezőjében nyilvántartva)
note-template-field-age = Életkor
note-template-field-role-in-story = Szerepe a történetben
note-template-field-build-and-features = Testalkat és megkülönböztető jegyek
note-template-field-habitual-bearing = Megszokott tartás
note-template-field-speech-patterns = Beszédmód és szófordulatok
note-template-field-what-they-never-say = Amit sosem mond ki
note-template-field-want = Mit akar
note-template-field-need = Amire szüksége van, de nem tud róla
note-template-field-fear = Mitől fél
note-template-field-flaw = A hibája, amely sokba kerül neki
note-template-field-formative-event = Meghatározó esemény
note-template-field-relationships = Kapcsolatok
note-template-field-starts-as = Kiindulópont
note-template-field-ends-as = Végpont
note-template-field-what-you-notice-first = Ami elsőként feltűnik
note-template-field-sound-and-smell = Hangok és illatok
note-template-field-light-and-weather = Fények és időjárás
note-template-field-what-happened-here = Mi történt itt
note-template-field-who-lives-or-works-here = Ki él vagy dolgozik itt
note-template-field-scenes-set-here = Itt játszódó jelenetek
note-template-field-why-it-matters = Miért fontos
note-template-field-appearance = Megjelenés
note-template-field-age-and-origin = Kor és eredet
note-template-field-what-it-does = Mire képes
note-template-field-who-holds-it = Kinél van
note-template-field-who-wants-it = Ki akarja megszerezni
note-template-field-pov = Nézőpont
note-template-field-time-and-place = Idő és hely
note-template-field-goal = Cél
note-template-field-conflict = Konfliktus
note-template-field-turn = A fordulat
note-template-field-exit-emotion = Érzelem a jelenet végén
note-template-field-purpose = Rendeltetés
note-template-field-who-leads-it = Ki vezeti
note-template-field-resources = Erőforrások
note-template-field-allies-and-enemies = Szövetségesek és ellenségek
note-template-field-what-it-wants-now = Mit akar most
note-template-field-where-from = Honnan származik
note-template-field-page-or-link = Oldalszám vagy hivatkozás
note-template-field-key-facts = Fő tények
note-template-field-how-it-is-used = Hogyan használja a történet
