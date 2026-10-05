# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

settings-page-templates = Şablonlar
settings-templates-description = Yazdığınız her şeye ekleyebileceğiniz, yeniden kullanılabilir yazı parçaları: boş bir kişi kartı, bir mekân profili, bir sahne planı. Bu projede saklanırlar, böylece projeyi açan herkes aynı şablonları bulur.
settings-templates-filter = Şablonları süz
settings-templates-count = { $n ->
    [0] Şablon yok
    [one] 1 şablon
   *[other] { $n } şablon
}
settings-templates-empty = Bu projede henüz şablon yok.
settings-templates-empty-hint = Hazır bir şablondan ekleyin, bir .md ya da .djot dosyası içe aktarın veya bir şey yazıp Belge ▸ Şablon olarak kaydet'i seçin.
settings-templates-presets = Hazır şablon ekle
settings-templates-import = İçe aktar…
settings-templates-export = Dışa aktar…
settings-templates-star = Ekle menüsünde en başta göster
settings-templates-unstar = En başta göstermeyi bırak
settings-templates-move-up = Yukarı taşı
settings-templates-move-down = Aşağı taşı
settings-templates-delete = Şablonu sil
settings-templates-name = Ad
settings-templates-name-placeholder = Şablon adı
settings-templates-body-placeholder = Bu şablonun eklediği metin
settings-templates-duplicate-name = "{ $name }" adlı başka bir şablon zaten var
settings-templates-words = { $n ->
    [one] 1 sözcük
   *[other] { $n } sözcük
}

templates-imported = { $added ->
    [one] 1 şablon içe aktarıldı
   *[other] { $added } şablon içe aktarıldı
}
templates-imported-renamed = Çakışmayı önlemek için yeniden adlandırıldı: { $names }
templates-imported-skipped = Okunamadı: { $names }
templates-import-filter = Şablonlar (.md, .djot)
templates-exported = { $n ->
    [one] 1 şablon dışa aktarıldı
   *[other] { $n } şablon dışa aktarıldı
}
templates-preset-applied = "{ $name }" eklendi

templates-delete-title = Bu şablon silinsin mi?
templates-delete-body = "{ $name }" bu projeden kaldırılacak. Ondan önceden oluşturduğunuz notlar etkilenmez.
templates-delete-confirm = Sil

menu-document = B&elge
menu-insert-template = Ş&ablon ekle
menu-insert-template-none = Bu projede şablon yok
menu-save-as-template = Şablon olarak &kaydet…

save-as-template-title = Şablon olarak kaydet
save-as-template-explain = Düzenlediğiniz metin, başka her yere ekleyebileceğiniz bir şablona dönüşür.
save-as-template-name = Ad
save-as-template-placeholder = Kişi kartı
save-as-template-duplicate = "{ $name }" adlı bir şablon zaten var
save-as-template-empty-editor = Bu düzenleyici boş. Kaydedilecek bir şey yok.
save-as-template-confirm = Şablonu kaydet
save-as-template-saved = "{ $name }" şablon olarak kaydedildi
template-inserted = "{ $name }" eklendi

note-template-preset-character-sheet = Kişi kartı
note-template-preset-location = Mekân
note-template-preset-artifact = Nesne
note-template-preset-beat-sheet = Sahne planı
note-template-preset-faction = Grup
note-template-preset-research-note = Araştırma notu

note-template-set-essentials = Temel şablonlar
note-template-set-everything = Tüm şablonlar

note-template-section-identity = Kimlik
note-template-section-appearance = Görünüş
note-template-section-voice = Ses
note-template-section-psychology = İç dünya
note-template-section-history = Geçmiş
note-template-section-arc = Gelişim
note-template-section-first-impression = İlk izlenim
note-template-section-in-the-story = Hikâyede
note-template-section-the-thing-itself = Nesnenin kendisi
note-template-section-the-scene = Sahne
note-template-section-the-shape = Yapı
note-template-section-what-it-is = Ne olduğu
note-template-section-source = Kaynak
note-template-section-what-it-says = Ne söylediği

note-template-field-full-name = Tam adı
note-template-field-known-as = Diğer adları, Denetçi'nin Diğer adları alanında izlenir
note-template-field-age = Yaşı
note-template-field-role-in-story = Hikâyedeki rolü
note-template-field-build-and-features = Yapısı ve ayırt edici özellikleri
note-template-field-habitual-bearing = Alışılmış duruşu
note-template-field-speech-patterns = Konuşma biçimi ve dil alışkanlıkları
note-template-field-what-they-never-say = Asla söylemediği
note-template-field-want = Ne istediği
note-template-field-need = Farkında olmadan neye ihtiyaç duyduğu
note-template-field-fear = Neden korktuğu
note-template-field-flaw = Ona pahalıya patlayan kusuru
note-template-field-formative-event = Onu biçimlendiren olay
note-template-field-relationships = İlişkileri
note-template-field-starts-as = Başta nasıl
note-template-field-ends-as = Sonda nasıl
note-template-field-what-you-notice-first = İlk göze çarpan
note-template-field-sound-and-smell = Ses ve koku
note-template-field-light-and-weather = Işık ve hava
note-template-field-what-happened-here = Burada ne oldu
note-template-field-who-lives-or-works-here = Burada kim yaşıyor ya da çalışıyor
note-template-field-scenes-set-here = Burada geçen sahneler
note-template-field-why-it-matters = Neden önemli
note-template-field-appearance = Görünüşü
note-template-field-age-and-origin = Yaşı ve kökeni
note-template-field-what-it-does = Ne işe yaradığı
note-template-field-who-holds-it = Kimin elinde
note-template-field-who-wants-it = Kim istiyor
note-template-field-pov = Bakış açısı
note-template-field-time-and-place = Zaman ve mekân
note-template-field-goal = Amaç
note-template-field-conflict = Çatışma
note-template-field-turn = Dönüm
note-template-field-exit-emotion = Çıkıştaki duygu
note-template-field-purpose = Amacı
note-template-field-who-leads-it = Kim yönetiyor
note-template-field-resources = Kaynakları
note-template-field-allies-and-enemies = Dostları ve düşmanları
note-template-field-what-it-wants-now = Şu an ne istiyor
note-template-field-where-from = Nereden geldiği
note-template-field-page-or-link = Sayfa ya da bağlantı
note-template-field-key-facts = Temel bilgiler
note-template-field-how-it-is-used = Hikâyede nasıl kullanıldığı
