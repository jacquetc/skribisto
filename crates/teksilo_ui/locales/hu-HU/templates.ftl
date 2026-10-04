# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

settings-page-templates = Sablonok
settings-templates-description = Újratölthető írási darabok bármibe beleeshetnek: egy üres karakterlap, egy helyszín profil, egy beat lap. Ebben a projektben tárolják őket, így bárki, aki kinyitja, ugyanazt a készletet kapja.
settings-templates-filter = Szűrősablonok
settings-templates-count = { $n ->
    [0] Nincs sablon
    [one] 1 sablon
   *[other] { $n } Sablonok
}
settings-templates-empty = Ennek a projektnek még nincsenek sablonjai.
settings-templates-empty-hint = Adjunk hozzá egy előzetest, importáljunk egy .md vagy .djot fájlt, vagy írjunk valamit, és válasszuk a Document Save sablont.
settings-templates-presets = Előzetes beállítás hozzáadása
settings-templates-import = Importálás…
settings-templates-export = Exportálás…
settings-templates-star = Az első megjelenítés a menüben
settings-templates-unstar = Ne mutasd meg előbb!
settings-templates-move-up = Mozgatás felfelé
settings-templates-move-down = Mozgatás lefelé
settings-templates-delete = A sablon törlése
settings-templates-name = Név
settings-templates-name-placeholder = Sablon neve
settings-templates-body-placeholder = A sablon beillesztése
settings-templates-duplicate-name = Egy másik sablont már { $name }-nek hívnak.
settings-templates-words = { $n ->
    [one] 1 szó
   *[other] { $n } szó
}

templates-imported = { $added ->
    [one] 1 Behozott sablon
   *[other] { $added } Behozott sablonok
}
templates-imported-renamed = Átnevezve, hogy elkerülje az ütközést: { $names }
templates-imported-skipped = Nem sikerült leolvasni: { $names }
templates-import-filter = Sablonok (.md, .djot)
templates-exported = { $n ->
    [one] 1 Kivitt sablon
   *[other] { $n } Kivitt sablonok
}
templates-preset-applied = Hozzáadva { $name }

templates-delete-title = Törölni ezt a sablont?
templates-delete-body = A projektből eltávolítjuk a { $name }-et. A már létrehozott jegyzeteket nem érinti.
templates-delete-confirm = Törlés

menu-document = Dokumentum (&D)
menu-insert-template = A sablon beszúrása (&L)
menu-insert-template-none = Ebben a projektben nincsenek sablonok
menu-save-as-template = STATISZTÁLY (&V)

save-as-template-title = Mentés sablonként
save-as-template-explain = A szöveg szerkesztése lesz egy sablon lehet ugrani bárhol máshol.
save-as-template-name = Név
save-as-template-placeholder = Karakterlap
save-as-template-duplicate = Már létezik egy { $name } nevű sablon
save-as-template-empty-editor = Ez a szerkesztő üres, nincs mit megmenteni.
save-as-template-confirm = A sablon mentése
save-as-template-saved = Mentett { $name }
template-inserted = Kokszolt{ $name }"

note-template-preset-character-sheet = Karakterlap
note-template-preset-location = Hely
note-template-preset-artifact = Objektum
note-template-preset-beat-sheet = Ütközőlap
note-template-preset-faction = Tevékenység
note-template-preset-research-note = Kutatási jegyzék

note-template-set-essentials = Alapvető tényezők
note-template-set-everything = Minden sablon

note-template-section-identity = Azonosító
note-template-section-appearance = Megjelenés
note-template-section-voice = Hang
note-template-section-psychology = Belső élet
note-template-section-history = Történelem
note-template-section-arc = Fejlődési ív
note-template-section-first-impression = Első benyomás
note-template-section-in-the-story = A történetben
note-template-section-the-thing-itself = Maga az objektum.
note-template-section-the-scene = A jelenet
note-template-section-the-shape = A forma
note-template-section-what-it-is = Mi az?
note-template-section-source = Forrás
note-template-section-what-it-says = Mi áll benne?

note-template-field-full-name = Teljes név
note-template-field-known-as = Más néven, nyomon követi a felügyelő Alias területen
note-template-field-age = Kor
note-template-field-role-in-story = Szerepet a történetben.
note-template-field-build-and-features = Építési és megkülönböztető jellemzők
note-template-field-habitual-bearing = Származási csapágy
note-template-field-speech-patterns = Beszédminták és szóbeli szövegek
note-template-field-what-they-never-say = Amit soha nem mondanak.
note-template-field-want = Mit akarnak?
note-template-field-need = Amire szükségük van, de nem tudnak róla.
note-template-field-fear = Mitől fél
note-template-field-flaw = A hiba, ami nekik jár.
note-template-field-formative-event = Formatív esemény
note-template-field-relationships = Kapcsolatok
note-template-field-starts-as = Kezdődik:
note-template-field-ends-as = Végek:
note-template-field-what-you-notice-first = Amit először észreveszel.
note-template-field-sound-and-smell = Hang és szag
note-template-field-light-and-weather = Fény és időjárás
note-template-field-what-happened-here = Mi történt itt?
note-template-field-who-lives-or-works-here = Ki él itt, vagy a projektek
note-template-field-scenes-set-here = Helyek ide beállítva
note-template-field-why-it-matters = Miért számít ez?
note-template-field-appearance = Megjelenés
note-template-field-age-and-origin = Kor és eredet
note-template-field-what-it-does = Mit csinál?
note-template-field-who-holds-it = Who hold it
note-template-field-who-wants-it = Ki akarja?
note-template-field-pov = Nézőpont
note-template-field-time-and-place = Idő és hely
note-template-field-goal = Cél
note-template-field-conflict = Összeférhetetlenség
note-template-field-turn = A turn
note-template-field-exit-emotion = Érzelem kifelé menet
note-template-field-purpose = Cél
note-template-field-who-leads-it = Ki vezeti?
note-template-field-resources = Erőforrások
note-template-field-allies-and-enemies = Szövetségek és ellenségek
note-template-field-what-it-wants-now = Mit akar most?
note-template-field-where-from = Honnan jött?
note-template-field-page-or-link = Oldal vagy link
note-template-field-key-facts = A legfontosabb tények
note-template-field-how-it-is-used = Hogyan használják a történetben
