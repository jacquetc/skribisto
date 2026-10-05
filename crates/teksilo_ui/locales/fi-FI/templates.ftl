# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

settings-page-templates = Mallit
settings-templates-description = Uudelleenkäytettäviä tekstipohjia, jotka voit lisätä mihin tahansa kirjoittamaasi: tyhjä henkilökortti, paikan kuvaus, juonirunko. Ne tallennetaan tähän projektiin, joten jokainen projektin avaaja saa saman valikoiman.
settings-templates-filter = Suodata malleja
settings-templates-count = { $n ->
       [0] Ei malleja
       [one] 1 malli
      *[other] { $n } mallia
   }
settings-templates-empty = Tällä projektilla ei ole vielä malleja.
settings-templates-empty-hint = Lisää malli valmiista malleista, tuo .md- tai .djot-tiedosto, tai kirjoita jotain ja valitse Asiakirja ▸ Tallenna mallina.
settings-templates-presets = Lisää valmis malli
settings-templates-import = Tuo…
settings-templates-export = Vie…
settings-templates-star = Näytä ensimmäisenä lisäysvalikossa
settings-templates-unstar = Älä näytä ensimmäisenä
settings-templates-move-up = Siirrä ylös
settings-templates-move-down = Siirrä alas
settings-templates-delete = Poista malli
settings-templates-name = Nimi
settings-templates-name-placeholder = Mallin nimi
settings-templates-body-placeholder = Teksti, jonka tämä malli lisää
settings-templates-duplicate-name = Toisen mallin nimi on jo ”{ $name }”
settings-templates-words = { $n ->
       [one] 1 sana
      *[other] { $n } sanaa
   }

templates-imported = { $added ->
       [one] 1 malli tuotu
      *[other] { $added } mallia tuotu
   }
templates-imported-renamed = Nimetty uudelleen päällekkäisyyden välttämiseksi: { $names }
templates-imported-skipped = Ei voitu lukea: { $names }
templates-import-filter = Mallit (.md, .djot)
templates-exported = { $n ->
       [one] 1 malli viety
      *[other] { $n } mallia viety
   }
templates-preset-applied = Lisätty ”{ $name }”

templates-delete-title = Poistetaanko tämä malli?
templates-delete-body = ”{ $name }” poistetaan tästä projektista. Sen avulla jo luotuihin muistiinpanoihin tämä ei vaikuta.
templates-delete-confirm = Poista

menu-document = &Asiakirja
menu-insert-template = &Lisää malli
menu-insert-template-none = Ei malleja tässä projektissa
menu-save-as-template = &Tallenna mallina…

save-as-template-title = Tallenna mallina
save-as-template-explain = Muokkaamastasi tekstistä tulee malli, jonka voit lisätä mihin tahansa muualle.
save-as-template-name = Nimi
save-as-template-placeholder = Henkilökortti
save-as-template-duplicate = Malli nimeltä ”{ $name }” on jo olemassa
save-as-template-empty-editor = Tämä muokkain on tyhjä. Tallennettavaa ei ole.
save-as-template-confirm = Tallenna malli
save-as-template-saved = ”{ $name }” tallennettu mallina
template-inserted = Lisätty ”{ $name }”

note-template-preset-character-sheet = Henkilökortti
note-template-preset-location = Paikka
note-template-preset-artifact = Esine
note-template-preset-beat-sheet = Juonirunko
note-template-preset-faction = Ryhmittymä
note-template-preset-research-note = Taustatietomuistiinpano

note-template-set-essentials = Perusmallit
note-template-set-everything = Kaikki mallit

note-template-section-identity = Henkilöllisyys
note-template-section-appearance = Ulkoasu
note-template-section-voice = Ääni
note-template-section-psychology = Sisäinen elämä
note-template-section-history = Historia
note-template-section-arc = Tarinakaari
note-template-section-first-impression = Ensivaikutelma
note-template-section-in-the-story = Tarinassa
note-template-section-the-thing-itself = Itse esine
note-template-section-the-scene = Kohtaus
note-template-section-the-shape = Muoto
note-template-section-what-it-is = Mikä se on
note-template-section-source = Lähde
note-template-section-what-it-says = Mitä siinä sanotaan

note-template-field-full-name = Koko nimi
note-template-field-known-as = Tunnetaan myös nimellä (seurataan Ominaisuudet-paneelin Muut nimet -kentässä)
note-template-field-age = Ikä
note-template-field-role-in-story = Rooli tarinassa
note-template-field-build-and-features = Ruumiinrakenne ja tuntomerkit
note-template-field-habitual-bearing = Tavanomainen olemus
note-template-field-speech-patterns = Puhetapa ja sanalliset maneerit
note-template-field-what-they-never-say = Mitä hän ei koskaan sano
note-template-field-want = Mitä hän haluaa
note-template-field-need = Mitä hän tarvitsee tietämättään
note-template-field-fear = Mitä hän pelkää
note-template-field-flaw = Heikkous, joka tulee hänelle kalliiksi
note-template-field-formative-event = Muovaava tapahtuma
note-template-field-relationships = Suhteet
note-template-field-starts-as = Alussa
note-template-field-ends-as = Lopussa
note-template-field-what-you-notice-first = Mitä huomaat ensin
note-template-field-sound-and-smell = Ääni ja haju
note-template-field-light-and-weather = Valo ja sää
note-template-field-what-happened-here = Mitä täällä tapahtui
note-template-field-who-lives-or-works-here = Kuka täällä asuu tai työskentelee
note-template-field-scenes-set-here = Täällä tapahtuvat kohtaukset
note-template-field-why-it-matters = Miksi sillä on merkitystä
note-template-field-appearance = Ulkoasu
note-template-field-age-and-origin = Ikä ja alkuperä
note-template-field-what-it-does = Mitä se tekee
note-template-field-who-holds-it = Kenen hallussa se on
note-template-field-who-wants-it = Kuka sitä haluaa
note-template-field-pov = Näkökulma
note-template-field-time-and-place = Aika ja paikka
note-template-field-goal = Päämäärä
note-template-field-conflict = Konflikti
note-template-field-turn = Käänne
note-template-field-exit-emotion = Tunne lopussa
note-template-field-purpose = Tarkoitus
note-template-field-who-leads-it = Kuka sitä johtaa
note-template-field-resources = Resurssit
note-template-field-allies-and-enemies = Liittolaiset ja viholliset
note-template-field-what-it-wants-now = Mitä se haluaa nyt
note-template-field-where-from = Mistä se on peräisin
note-template-field-page-or-link = Sivu tai linkki
note-template-field-key-facts = Keskeiset tiedot
note-template-field-how-it-is-used = Miten sitä käytetään tarinassa
