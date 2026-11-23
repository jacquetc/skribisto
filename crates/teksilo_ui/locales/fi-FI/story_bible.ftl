# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

story-bible-modal-configure-title = Määritä uusi merkintä
story-bible-modal-name-label = Nimi
story-bible-modal-name-placeholder = Henkilö, paikka, mikä tahansa kirjaamisen arvoinen…
story-bible-modal-no-binders = Tässä projektissa ei ole vielä mappia, johon sen voisi tallentaa
story-bible-modal-template-label = Aloita mallista (valinnainen)
story-bible-modal-template-placeholder = Ei mallia
story-bible-modal-body-label = Leipäteksti
story-bible-modal-cancel = Peruuta
story-bible-modal-create = Luo
story-bible-modal-create-and-open = Luo ja avaa
story-bible-modal-failed = Merkintää ei voitu luoda

ctx-add-as-note = Lisää &muistiinpanona…
ctx-add-as-note-all-tags = Kaikki tunnisteet…
ctx-add-as-note-untagged = Ei tunnistetta

story-bible-grid-label = Tarinan tietopankki
story-bible-grid-untagged = Ei vielä tunnistetta
story-bible-grid-empty-title = Tänne ei ole vielä tallennettu mitään
story-bible-grid-empty-hint = Merkitse muistiinpano tunnisteella, jotta se löytyy tekstistä, tai lisää se valinnasta, niin se näkyy tässä.
story-bible-grid-alias-count = { $count ->
       [0] Ei vielä muita nimiä
       [one] { $count } muu nimi
      *[other] { $count } muuta nimeä
   }
story-bible-grid-mention-count = { $count ->
       [0] Ei vielä esiintymiä
       [one] Esiintyy { $count } kohtauksessa koko projektissa
      *[other] Esiintyy { $count } kohtauksessa koko projektissa
   }
story-bible-books-filter-all = Kaikki kirjat
story-bible-grid-not-searched = Ei vielä haettu: tarvitsee tietopankin tunnisteen

note-in-prose-pov = Näkökulma
note-in-prose-cast = Esiintyjät
note-in-prose-pov-and-cast = Näkökulma · Esiintyjät
note-in-prose-no-books = Tässä projektissa ei ole vielä kirjaa
# The page lists only scenes where the writer placed the entry in the cast or the point of
# view, never the ones the scan merely found its name in, so an empty page says so: a bare
# "nothing here" read as a search that had failed.
note-in-prose-empty-book = Tämän kirjan minkään kohtauksen esiintyjissä tai näkökulmassa ei ole vielä tätä merkintää.
note-in-prose-empty-viewpoint = Mitään tämän kirjan kohtausta ei ole vielä kerrottu tämän merkinnän näkökulmasta.
note-in-prose-scope = Tekstissä-näkymä näyttää vain vahvistamasi kohtaukset. Kohtaus, jossa tämä merkintä mainitaan, pysyy ehdotuksena, kunnes lisäät merkinnän sen esiintyjiin Yksityiskohdat-näkymässä tai Ominaisuudet-paneelissa.
# A switch beside the Book bar: keep only the scenes this entry narrates.
note-in-prose-viewpoint-only = Vain näkökulmakohtaukset
note-in-prose-viewpoint-only-tooltip = Näytä vain kohtaukset, jotka kerrotaan tämän merkinnän näkökulmasta, ja piilota ne, joissa se vain esiintyy

story-bible-capture-toast = ”{ $name }” lisätty tarinan tietopankkiin
story-bible-capture-open = Avaa
story-bible-capture-undo = Kumoa
story-bible-capture-where-title = Minne nämä muistiinpanot sijoitetaan?
story-bible-capture-where-prompt = Valitse kansio, johon uudet muistiinpanot tallennetaan.
story-bible-capture-where-confirm = Sijoita tähän
story-bible-capture-where-needs-folder = Valitse kansio, jonka sisään nämä muistiinpanot sijoitetaan.
story-bible-capture-where-tag = Kysytään vain kerran. Tunnisteella ”{ $tag }” merkityt muistiinpanot sijoitetaan tästä lähtien tänne, ja voit muuttaa tätä myöhemmin kohdassa Asetukset ▸ Projekti ▸ Tunnisteet.
story-bible-capture-where-untagged = Kysytään vain kerran. Tunnisteettomat muistiinpanot sijoitetaan tästä lähtien tänne, ja voit muuttaa tätä myöhemmin kohdassa Asetukset ▸ Projekti ▸ Tunnisteet.

# What a screen reader hears for a binder row that is a story bible entry: its icon
# says so on screen, and this says it aloud. The second form keeps the row's label.
binder-row-story-bible-entry = Tietopankin merkintä
binder-row-story-bible-entry-with-label = Tietopankin merkintä, { $label }
