# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

tags-preset-basic = Temel
tags-preset-scifi = Bilimkurgu
tags-preset-fantasy = Fantastik
tags-preset-mystery = Polisiye
tags-preset-historical = Tarihî

tags-preset-needs-research = araştırma gerekli
tags-preset-continuity-check = süreklilik denetimi
tags-preset-plot-point = dönüm noktası

tags-preset-character = kişi
tags-preset-place = yer
tags-preset-item = nesne

tags-preset-vessel = gemi
tags-preset-planet = gezegen
tags-preset-organization = kuruluş
tags-preset-creature = yaratık
tags-preset-faction = grup
tags-preset-artifact = eser
tags-preset-realm = diyar
tags-preset-magic-system = büyü sistemi
tags-preset-suspect = şüpheli
tags-preset-victim = kurban
tags-preset-clue = ipucu
tags-preset-red-herring = yanıltıcı ipucu
tags-preset-historical-figure = tarihî kişilik
tags-preset-source = kaynak
tags-preset-period-detail = dönem ayrıntısı

tags-pill-list = Etiketler
tags-pill-add = Etiket ekle
tags-none-yet = Etiket yok
tags-pick-list = Seçilebilecek etiketler
tags-pill-remove = Kaldır: { $name }
tags-pill-filter-placeholder = Süzün ya da yeni bir etiketi adlandırın
tags-pill-no-match = Eşleşen etiket yok
tags-pill-create = "{ $name }" oluştur
tags-pill-create-failed = "{ $name }" oluşturulamadı
tags-pill-new-discoverable = Metinde bul etiketi
tags-pill-new-discoverable-hint = Bu etiketi taşıyan öğeler, kadroyu doldurmak için metninizde aranır.

tags-alias-list = Diğer adları
tags-alias-add = Başka bir ad ekle
tags-alias-remove = Kaldır: { $name }
tags-alias-placeholder = Başka bir ad, ardından Enter
tags-alias-hint = Bunun metninizde başlığının yanı sıra geçtiği adlar.
tags-alias-collision = { $name } zaten bu adla anılıyor.

settings-page-tags = Etiketler
settings-tags-desc = Etiketler dizininizdeki öğeleri işaretler. Bir metinde bul etiketi ayrıca Skribisto'ya o öğenin adlarını metninizde aramasını söyler.
settings-tags-add = Etiket ekle
settings-tags-add-placeholder = Yeni bir etiketi adlandırın
settings-tags-added = "{ $name }" eklendi
settings-tags-duplicate = "{ $name }" zaten var
settings-tags-filter = Etiketleri süz
settings-tags-count = { $n ->
    [one] 1 etiket
   *[other] { $n } etiket
}
settings-tags-details-placeholder = Bu etiketin anlamı
settings-tags-discoverable = Metinde bul
settings-tags-creates-in = Yeni notların gideceği yer
settings-tags-creates-in-unset = İlk seferde bana sor
settings-tags-creates-in-untitled = Adsız klasör
settings-tags-creates-in-trashed = { $name } (çöp kutusunda)
settings-tags-template = Başlangıç şablonu
settings-tags-template-unset = Boş not
settings-tags-delete = Sil: { $name }
settings-tags-deleted = "{ $name }" silindi ve tüm öğelerden kaldırıldı
settings-tags-empty = Henüz etiket yok.
settings-tags-apply-preset = Hazır bir dizi uygula…
settings-tags-preset-applied = Eklenen: { $added }, zaten var olduğu için atlanan: { $skipped }
settings-tags-csv-filter = CSV dosyaları
settings-tags-import = İçe aktar…
settings-tags-export = Dışa aktar…
settings-tags-imported = İçe aktarılan: { $added }, atlanan: { $skipped }
settings-tags-exported = { $n ->
    [one] 1 etiket dışa aktarıldı
   *[other] { $n } etiket dışa aktarıldı
}

tags-chip-more = { $n ->
    [one] 1 etiket daha
   *[other] { $n } etiket daha
}

cast-section = Kadro
cast-add = Kadroya ekle…
cast-add-filter-placeholder = Hikâye rehberini süz…
cast-add-empty = Eklenecek hikâye rehberi kaydı yok
cast-pin = Kadroya ekle: { $name }
cast-unpin = Kadrodan çıkar: { $name }
cast-empty = Henüz kimse sabitlenmedi. Birini ekleyin ya da bir öneriyi tutun
cast-unresolved = Artık hikâye rehberinde değil

mentions-confirm = Şunun kadrosuna ekle: { $name }
mentions-hit-count = { $n ->
    [one] bir kez
   *[other] { $n } kez
}
mentions-point-of-view-badge = Bakış açısı
mentions-point-of-view-badge-tooltip = Burada bakış açısı olarak belirtildi; metinde algılanmadı, elle ayarlandı.

mentions-roster = Kadro
mentions-pin = Kadroya ekle: { $name }

pov-section = Bakış açısı
pov-empty = Bakış açısı belirlenmedi
pov-add = Bakış açısı belirle…
pov-multiple = Bu sahnenin iki bakış açısı var.
pov-remove = Bakış açısı olarak kaldır: { $name }
pov-unresolved = Buraya bir bakış açısı sabitlenmişti, ama o kayıt artık yok ya da hikâye rehberi etiketini yitirdi.

books-section = Bağlı olduğu kitap
books-empty = Henüz bir kitaba bağlanmadı
books-add = Bir kitaba bağla…
books-remove = Şuradan çıkar: { $name }
books-apply-to-children = Kitap bağlantısını alt öğelere uygula

note-details-name-placeholder = Ad…
note-details-tags = Etiketler
note-details-aliases = Diğer adları
note-details-books = Bağlı olduğu kitap
note-details-books-empty = Henüz bir Kitaba bağlanmadı
note-details-links = Bağlantılar
note-details-links-empty = Henüz bağlantı yok. Hikâye rehberinizdeki başka bir kaydı göstermek için bir bağlantı ekleyin.
note-details-links-add = Bir nota bağla…
note-details-pov = Bakış açısı
note-details-pov-empty = Bakış açısı belirlenmedi
note-details-pov-unresolved = Buraya bir bakış açısı sabitlenmişti, ama o kayıt artık yok ya da hikâye rehberi etiketini yitirdi.
note-details-pov-multiple = Bu notun iki bakış açısı var.
note-details-backlinks = El yazmasında geçtiği yerler
note-details-backlinks-empty = Henüz bir şey yok. Bu ad metninizde geçtiğinde burada görünecek.
note-details-backlinks-outside = Kitapların dışında
note-details-backlinks-confirm-all = Her geçişi onayla
note-details-backlinks-confirm-all-tooltip = Bu kaydı, burada gösterilen ve onu henüz onaylamamış her belgenin kadrosuna ekler
note-details-untitled-document = Adsız

note-in-prose-mentions = { $n ->
    [one] 1 anılma
   *[other] { $n } anılma
}
note-in-prose-mention-at = { $current } / { $total }
note-in-prose-mention-previous = Önceki anılma
note-in-prose-mention-next = Sonraki anılma
