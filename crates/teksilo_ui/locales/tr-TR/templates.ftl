# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

settings-page-templates = Şablonlar
settings-templates-description = Yeniden yazılabilir parçaları yazabileceğiniz her şeye düşebilir: boş bir karakter sayfası, bir yer profili, bir dövme sayfası. Bu projede depolanırlar, böylece açan herkes aynı seti alır.
settings-templates-filter = Filtre şablonları
settings-templates-count = { $n ->
    [0] Hiçbir şablon yok
    [one] 1 şablon
   *[other] { $n } şablonları
}
settings-templates-empty = Bu proje henüz şablonları yok.
settings-templates-empty-hint = Bir başlangıçtan birini ekleyin, bir tane ithal edin .md veya veya veya .djot Dosya, ya da bir şey yazın ve Doküman .Aptal'ı şablon olarak seçin.
settings-templates-presets = Hazır ayar ekle
settings-templates-import = İçe aktar…
settings-templates-export = Dışa aktar…
settings-templates-star = İlk önce ekleme menüsünde göster
settings-templates-unstar = İlk önce gösteriyi bırak
settings-templates-move-up = Yukarı taşı
settings-templates-move-down = Aşağı taşı
settings-templates-delete = Delete şablon
settings-templates-name = Ad
settings-templates-name-placeholder = Şablon adı
settings-templates-body-placeholder = Metin bu şablon eklentileri
settings-templates-duplicate-name = Başka bir şablon zaten "{ $name }" olarak adlandırılır.
settings-templates-words = { $n ->
    [one] 1 kelime kelime
   *[other] { $n } kelime
}

templates-imported = { $added ->
    [one] 1 şablon ithal edildi
   *[other] { $added } ithal edildi
}
templates-imported-renamed = Bir çatışmadan kaçınmak için yeniden isimlendi: { $names }
templates-imported-skipped = Okuyamadı: { $names }
templates-import-filter = Şablonlar (.md, .djot)
templates-exported = { $n ->
    [one] 1 şablon ihraç edildi
   *[other] { $n } ihraç edilen şablonlar
}
templates-preset-applied = "{ $name }" eklendi.

templates-delete-title = Bu şablonu delete mi?
templates-delete-body = "{ $name }" bu projeden kaldırılacaktır. Bundan zaten yaratılan notlar etkilenmez.
templates-delete-confirm = Sil

menu-document = Doküman Doküman (&D)
menu-insert-template = Ekle şablon (&L)
menu-insert-template-none = Bu projede şablon yok
menu-save-as-template = Şablon olarak tasarruf edin... (&V)

save-as-template-title = Şablon olarak Kaydet
save-as-template-explain = Düzenleme olduğunuz metin başka bir yerde bırakabileceğiniz bir şablon haline gelir.
save-as-template-name = Ad
save-as-template-placeholder = Karakter sayfası
save-as-template-duplicate = "{ $name }" adlı bir şablon zaten var
save-as-template-empty-editor = Bu editör boş. kurtarmak için hiçbir şey yok.
save-as-template-confirm = Kaydet şablon
save-as-template-saved = Bir şablon olarak "{ $name }" tasarruf etti
template-inserted = "{ $name }"

note-template-preset-character-sheet = Karakter sayfası
note-template-preset-location = Konum Konum Location
note-template-preset-artifact = Object Object Object Object
note-template-preset-beat-sheet = Beat page
note-template-preset-faction = Hizip
note-template-preset-research-note = Araştırma notu

note-template-set-essentials = Temel bilgiler
note-template-set-everything = Her şablon

note-template-section-identity = Kimlik Kimlik Kimliği
note-template-section-appearance = Görünüm
note-template-section-voice = Ses Ses Sesi
note-template-section-psychology = İç hayat İçsel
note-template-section-history = Tarih Tarihi Tarihi
note-template-section-arc = Arc Arc
note-template-section-first-impression = İlk izlenim İlk izlenim
note-template-section-in-the-story = Hikayede
note-template-section-the-thing-itself = nesnenin kendisi
note-template-section-the-scene = Sahne The scene the scene
note-template-section-the-shape = Şekil şekli
note-template-section-what-it-is = Nedir?
note-template-section-source = Source Source Source Source
note-template-section-what-it-says = Ne diyor ki

note-template-field-full-name = Full name Full name
note-template-field-known-as = Ayrıca Inspector'un Alias alanında da bilinen
note-template-field-age = Yaş Çağı
note-template-field-role-in-story = Hikayedeki Rolü
note-template-field-build-and-features = Build ve ayırt edici özellikler
note-template-field-habitual-bearing = Habitualating
note-template-field-speech-patterns = Konuşma kalıpları ve sözlü tics
note-template-field-what-they-never-say = Ne söylediklerini asla söylemiyorlar
note-template-field-want = Ne istiyorlar istediklerini
note-template-field-need = Ne ihtiyaçları var ama bunu bilmiyor
note-template-field-fear = Ne korktukları
note-template-field-flaw = Onlara mal olan kusurlar
note-template-field-formative-event = Formative eventive event
note-template-field-relationships = İlişkiler
note-template-field-starts-as = Starts as As
note-template-field-ends-as = Ends as ends
note-template-field-what-you-notice-first = İlk önce fark ettiğiniz şey nedir?
note-template-field-sound-and-smell = Ses ve koku
note-template-field-light-and-weather = Işık ve hava durumu
note-template-field-what-happened-here = Burada ne oldu burada
note-template-field-who-lives-or-works-here = Kim yaşıyor ya da burada
note-template-field-scenes-set-here = Sahneler buraya buraya buraya koydu
note-template-field-why-it-matters = Neden önemli?
note-template-field-appearance = Görünüm
note-template-field-age-and-origin = Yaş ve köken
note-template-field-what-it-does = Ne yapar
note-template-field-who-holds-it = Kim tutar onu
note-template-field-who-wants-it = Kim istiyor
note-template-field-pov = Bakış açısı
note-template-field-time-and-place = Zaman ve yer
note-template-field-goal = Hedef
note-template-field-conflict = Çatışma Çatışma
note-template-field-turn = Dönüm noktası
note-template-field-exit-emotion = Sahne sonundaki duygu
note-template-field-purpose = Amaç
note-template-field-who-leads-it = Kim götürürse onu
note-template-field-resources = Kaynakların Kaynakları Kaynaklar
note-template-field-allies-and-enemies = Bütünler ve Düşmanlar
note-template-field-what-it-wants-now = Şimdi ne istiyor
note-template-field-where-from = Nereden geldi
note-template-field-page-or-link = Sayfa veya bağlantı
note-template-field-key-facts = Anahtar gerçekler
note-template-field-how-it-is-used = Hikayede nasıl kullanılır
