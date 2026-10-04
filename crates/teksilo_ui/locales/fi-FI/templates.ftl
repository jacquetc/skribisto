# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

settings-page-templates = Mallit
settings-templates-description = Uudelleen käytettävät palaset voit pudottaa mihin tahansa olet kirjoittamassa: tyhjä merkkiarkki, sijainti profiili, beat-arkki. Ne tallennetaan tässä projektissa, joten kuka tahansa, joka avaa sen saa saman sarjan.
settings-templates-filter = Suodatinmallit
settings-templates-count = { $n ->
    [0] Ei malleja
    [one] 1 malli
   *[other] { $n } mallit
}
settings-templates-empty = Tällä projektilla ei ole vielä malleja.
settings-templates-empty-hint = Lisää yksi esiasetetusta, tuoda .md tai .djot tiedosto, tai kirjoita jotain ja valitse asiakirja .
settings-templates-presets = Lisää esiasetettu
settings-templates-import = Tuo...
settings-templates-export = Vie...
settings-templates-star = Näytä ensin lisävalikossa
settings-templates-unstar = Lakkaa näyttämästä ensin
settings-templates-move-up = Ylös.
settings-templates-move-down = Siirry alas
settings-templates-delete = Poista malli
settings-templates-name = Nimi
settings-templates-name-placeholder = Mallin nimi
settings-templates-body-placeholder = Tämä taulukko lisää tekstin
settings-templates-duplicate-name = Toinen malli on jo nimeltään "{ $name }"
settings-templates-words = { $n ->
    [one] 1 sana
   *[other] { $n } words
}

templates-imported = { $added ->
    [one] 1 Tuotu malli
   *[other] { $added } Tuodut mallit
}
templates-imported-renamed = Nimetty välttämään yhteentörmäystä: { $names }
templates-imported-skipped = Ei voitu lukea: { $names }
templates-import-filter = Mallit (.md, .djot)
templates-exported = { $n ->
    [one] 1 Viety malli
   *[other] { $n } Viedyt mallit
}
templates-preset-applied = Lisätty "{ $name }"

templates-delete-title = Poista tämä malli?
templates-delete-body = "{ $name }" poistetaan tästä projektista. Huomioita olet jo luotu siitä ei vaikuta.
templates-delete-confirm = Poista

menu-document = &Asiakirja
menu-insert-template = &Lisää malli
menu-insert-template-none = Ei malleja tässä projektissa
menu-save-as-template = &Tallenna malli...

save-as-template-title = Tallenna mallina
save-as-template-explain = Editoimastasi tekstistä tulee malli, jonka voit pudottaa minne tahansa muualle.
save-as-template-name = Nimi
save-as-template-placeholder = Hahmolomake
save-as-template-duplicate = Malli nimeltään "{ $name }" on jo olemassa
save-as-template-empty-editor = Päätoimittaja on tyhjä.
save-as-template-confirm = Tallenna malli
save-as-template-saved = Säästetty "{ $name }" mallina
template-inserted = Lisätty "{ $name }"

note-template-preset-character-sheet = Hahmolomake
note-template-preset-location = Paikka
note-template-preset-artifact = Esine
note-template-preset-beat-sheet = Tapahtumarunko
note-template-preset-faction = Ryhmittymä
note-template-preset-research-note = Tutkimusmuistiinpano

note-template-set-essentials = Perusmallit
note-template-set-everything = Jokainen malli

note-template-section-identity = Henkilöllisyys
note-template-section-appearance = Ulkoasu
note-template-section-voice = Ääni
note-template-section-psychology = Sisäinen elämä
note-template-section-history = Historia
note-template-section-arc = Tarinakaari
note-template-section-first-impression = Ensivaikutelma
note-template-section-in-the-story = Tarinassa
note-template-section-the-thing-itself = - Itse esine.
note-template-section-the-scene = Kohtaus
note-template-section-the-shape = Muoto
note-template-section-what-it-is = Mitä se on?
note-template-section-source = Lähde
note-template-section-what-it-says = Mitä siinä lukee?

note-template-field-full-name = Koko nimi
note-template-field-known-as = Tunnetaan myös nimellä, jäljitettynä tarkastajan Alias-kentällä
note-template-field-age = Ikä
note-template-field-role-in-story = Rooli tarinassa
note-template-field-build-and-features = Rakentaminen ja erottuvat ominaisuudet
note-template-field-habitual-bearing = Tavallinen laakeri
note-template-field-speech-patterns = Puhemallit ja sanallinen tics
note-template-field-what-they-never-say = Mitä he eivät koskaan sano
note-template-field-want = Mitä he haluavat?
note-template-field-need = Mitä he tarvitsevat, mutta eivät tiedä sitä.
note-template-field-fear = Mitä he pelkäävät?
note-template-field-flaw = Heidän vikansa.
note-template-field-formative-event = Muodollinen tapahtuma
note-template-field-relationships = Suhteet
note-template-field-starts-as = Alkaa nimellä
note-template-field-ends-as = Loppuu nimellä
note-template-field-what-you-notice-first = Mitä huomaat ensin
note-template-field-sound-and-smell = Ääni ja haju
note-template-field-light-and-weather = Valo ja sää
note-template-field-what-happened-here = Mitä täällä tapahtui?
note-template-field-who-lives-or-works-here = Kuka asuu tai työskentelee täällä?
note-template-field-scenes-set-here = Kohtaukset asetettu tähän
note-template-field-why-it-matters = Miksi sillä on väliä?
note-template-field-appearance = Ulkoasu
note-template-field-age-and-origin = Ikä ja alkuperä
note-template-field-what-it-does = Mitä se tekee?
note-template-field-who-holds-it = Kenellä se on?
note-template-field-who-wants-it = Kuka sitä haluaa?
note-template-field-pov = Näkökulma
note-template-field-time-and-place = Aika ja paikka
note-template-field-goal = Tavoite
note-template-field-conflict = Konflikti
note-template-field-turn = Käänne
note-template-field-exit-emotion = Tunteet matkalla ulos
note-template-field-purpose = Aihe
note-template-field-who-leads-it = Kuka sitä johtaa?
note-template-field-resources = Varat
note-template-field-allies-and-enemies = Liittoutuneet ja viholliset
note-template-field-what-it-wants-now = Mitä se haluaa nyt?
note-template-field-where-from = Mistä se tuli?
note-template-field-page-or-link = Sivu tai linkki
note-template-field-key-facts = Lyhyesti
note-template-field-how-it-is-used = Miten sitä käytetään tarinassa
