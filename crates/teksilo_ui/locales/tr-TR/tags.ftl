# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

tags-preset-basic = Temel Temel Temel Temel Temel Temel Temel Temel
tags-preset-scifi = Bilim kurgusu
tags-preset-fantasy = Fantasy Fantasy
tags-preset-mystery = Gizem Gizem
tags-preset-historical = Tarihsel Tarihsel Tarihsel Tarihsel Tarihsel Tarihsel Tarihsel Tarihsel Tarihsel Tarihi


tags-preset-needs-research = Araştırmanın ihtiyaçlara ihtiyacı var
tags-preset-continuity-check = Süreklilik kontrolü
tags-preset-plot-point = arsa noktası

tags-preset-character = karakter karakter karakteri
tags-preset-place = yer yer yer yer yer yer
tags-preset-item = item item

tags-preset-vessel = Gemi teknesi
tags-preset-planet = gezegen gezegeni gezegen gezegeni gezegen gezegeni
tags-preset-organization = Organizasyon organizasyonu
tags-preset-creature = Yararlı yaratık
tags-preset-faction = hizip
tags-preset-artifact = Sanat
tags-preset-realm = alemi
tags-preset-magic-system = sihirli sistem
tags-preset-suspect = şüpheli şüpheli şüpheli şüpheli şüpheli şüpheli şüpheli şüpheli şüpheli şüpheli şüpheli şüpheli
tags-preset-victim = kurbanı kurban kurbanı
tags-preset-clue = ipucu
tags-preset-red-herring = Red herring
tags-preset-historical-figure = Tarihsel figür
tags-preset-source = Kaynak kaynağı kaynağı kaynağı kaynağı kaynağı
tags-preset-period-detail = Dönem detayı

tags-pill-list = Etiketler
tags-pill-add = Bir etiket ekleyin
tags-none-yet = Hiçbir etiket yok
tags-pick-list = Etiketler to pick from from from
tags-pill-remove = { $name }
tags-pill-filter-placeholder = Filtre veya yeni bir etiket
tags-pill-no-match = No tag maçları
tags-pill-create = "{ $name }" oluşturun
tags-pill-create-failed = "{ $name }" yaratamaz
tags-pill-new-discoverable = Find-in-prose etiketi
tags-pill-new-discoverable-hint = Bu etiketle öğeler, roster'ı doldurmanız için prose karşı eşleştirilir.

tags-alias-list = Ayrıca olarak da bilinir
tags-alias-add = Başka bir isim ekleyin
tags-alias-remove = { $name }
tags-alias-placeholder = Başka bir isim, sonra girin
tags-alias-hint = İsimler bu, kendi başlığının yanında Prose’nin altında görünür.
tags-alias-collision = { $name } Bu adı zaten yanıtlar.

settings-page-tags = Etiketler
settings-tags-desc = Etiketler, binder'deki eşyaları etiketleyin. Bir find-in-prose etiketi aynı zamanda Skribisto'e bu öğenin isimlerini prose'lerinizde aramak için de söyler.
settings-tags-add = Etiket ekle
settings-tags-add-placeholder = Yeni bir etiket
settings-tags-added = "{ $name }" eklendi.
settings-tags-duplicate = "{ $name }" zaten var
settings-tags-filter = Filtre etiketleri
settings-tags-count = { $n ->
    [one] 1 etiket
   *[other] { $n } Etiketler
}
settings-tags-details-placeholder = Bu etiket ne anlama geliyor?
settings-tags-discoverable = Prose'de bulun
settings-tags-creates-in = Yeni notlar yeni notlar gider
settings-tags-creates-in-unset = Bana ilk kez sorun
settings-tags-creates-in-untitled = Untitled klasörü
settings-tags-creates-in-trashed = { $name } (içinde)
settings-tags-template = Starting şablon
settings-tags-template-unset = Blank notu
settings-tags-delete = { $name } etiketini sil
settings-tags-deleted = Deleted "{ $name }" ve her öğeden kaldırıldı
settings-tags-empty = Henüz etiket yok.
settings-tags-apply-preset = Bir başlangıç uygulayın...
settings-tags-preset-applied = { $added } eklendi, { $skipped } zaten mevcut
settings-tags-csv-filter = CSV dosyaları
settings-tags-import = İçe aktar…
settings-tags-export = Dışa aktar…
settings-tags-imported = { $added } ithal edildi, { $skipped } atladı
settings-tags-exported = İhracata ihracat yapıldı { $n ->
    [one] 1 etiket
   *[other] { $n } Etiketler
}

tags-chip-more = { $n ->
    [one] 1 daha etiket
   *[other] { $n } daha fazla etiket
}

cast-section = Karakterler
cast-add = Çalmak için ekleyin...
cast-add-filter-placeholder = Filtre hikayesi bible...
cast-add-empty = Eklenecek hikâye rehberi kaydı yok
cast-pin = Oy vermek için { $name } ekleyin
cast-unpin = { $name }'i işten çıkarın
cast-empty = Hiç kimse henüz tamamlanmadı ya da bir öneriye devam et
cast-unresolved = Artık hikayede İncil yok

mentions-confirm = { $name }
mentions-hit-count = { $n ->
    [one] Bir kere bir kez bir kez bir kez bir kez bir kez bir kez bir kez bir kez bir kez bir kez
   *[other] { $n } defa
}
mentions-point-of-view-badge = Bakış açısı
mentions-point-of-view-badge-tooltip = Buradaki görüş noktası olarak, el tarafından belirlenen, prose'de tespit edilmedi.

mentions-roster = Karakterler
mentions-pin = Oy vermek için { $name } ekleyin

pov-section = Bakış açısı
pov-empty = Hiçbir bakış noktası set
pov-add = Set point of view...
pov-multiple = Bu sahnenin iki bakış açısı vardır.
pov-remove = { $name } görüntü noktası olarak
pov-unresolved = Bir bakış noktası burada kilitlendi, ancak bu giriş artık var ya da hikayesini kaybetmiş bir etiket yoktu.

books-section = Filed under çeviri Tagged under:
books-empty = Henüz bir kitap altında yayınlanmadı
books-add = Bir kitap altında Dosya...
books-remove = { $name }'den Çıkarın
books-apply-to-children = Çocuklara dosya uygulayın

note-details-name-placeholder = Ad…
note-details-tags = Etiketler
note-details-aliases = Diğer isimler
note-details-books = Filed under çeviri Tagged under:
note-details-books-empty = Henüz bir kitap altında yayınlanmadı
note-details-links = Linkler Linkler Linkler
note-details-links-empty = Henüz bağlantı yok. Hikayenizde başka bir girişte bir tane ekleyin.
note-details-links-add = Bir nota bağlantı...
note-details-pov = Bakış açısı
note-details-pov-empty = Hiçbir bakış noktası set
note-details-pov-unresolved = Bir bakış noktası burada kilitlendi, ancak bu giriş artık var ya da hikayesini kaybetmiş bir etiket yoktu.
note-details-pov-multiple = Bu notun iki görüşü vardır.
note-details-backlinks = Yazının içinde görünürler
note-details-backlinks-empty = Henüz bir şey yok. Bu isim senin prose'de göründüğünde, burada ortaya çıkacak.
note-details-backlinks-outside = Kitaplar dışında
note-details-backlinks-confirm-all = Her görünümünü onaylayın
note-details-backlinks-confirm-all-tooltip = Burada gösterilen her belgenin dökümüne bu girişi ekleyin, henüz onaylamadı
note-details-untitled-document = Adsız

note-in-prose-mentions = { $n ->
    [one] 1 1 Bahsed 1
   *[other] { $n } Bahsediyor
}
note-in-prose-mention-at = { $current } Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of Of { $total }
note-in-prose-mention-previous = Önceki Önceki Önceki Önceki Önceki Önceki Önceki Önceki yazı
note-in-prose-mention-next = Sonraki Bahsedin
