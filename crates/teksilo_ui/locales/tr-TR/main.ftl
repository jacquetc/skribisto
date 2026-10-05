# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

menu-work = &Proje
menu-new-work = Ye&ni proje
menu-open-work = Proje &aç…
menu-new-window = Yeni &pencere
menu-import-from = İçe ak&tar
menu-create-from = &Kaynaktan oluştur
menu-import-plume = &Plume Creator (.plume)…
menu-import-document = &Belgeler (Markdown, Word, ODT)…
menu-import-manuskript = &Manuskript (.msk)…
menu-export = &Dışa aktar
menu-export-book = Kitabı dışa aktar
menu-export-part = Kısmı dışa aktar
menu-export-chapter = Bölümü dışa aktar
menu-export-scene = Sahneyi dışa aktar
menu-export-note = Notu dışa aktar
menu-export-paratext = Yan metni dışa aktar
menu-export-folder = Klasörü dışa aktar
menu-export-choose = Seç…
menu-export-none = Dışa aktarmak için bir belge açın
menu-save = &Kaydet
menu-save-as-file = Tek do&sya olarak kaydet…
menu-save-as-folder = Klasör &olarak kaydet…
menu-backup = Şimdi &yedekle
menu-close-work = Pro&jeyi kapat
menu-welcome = &Hoş geldiniz…
menu-settings = Aya&rlar
menu-quit = &Çık

menu-view = &Görünüm
menu-outline = &Ana hat
menu-search = Projede a&ra
menu-timeline = &Zamanda geri git
menu-search-preview = Arama &önizlemesi
menu-fullscreen = &Tam ekran
menu-focus-mode = &Dikkat dağıtmayan mod
menu-format = &Biçim
menu-scene-break = Sahne a&rası ekle
menu-major-scene-break = Bü&yük sahne arası ekle

menu-go = Gi&t
menu-go-next-scene = Sonraki &sahne
menu-go-prev-scene = Önceki s&ahne
menu-go-next-chapter = Sonraki &bölüm
menu-go-prev-chapter = Önceki b&ölüm
menu-go-next-note = Sonraki &not
menu-go-prev-note = Önceki n&ot

menu-tools = &Araçlar
menu-help = &Yardım

ctx-add = &Ekle
ctx-new-item = &Yeni öğe
ctx-new-folder = Yeni &klasör
ctx-rename = &Yeniden adlandır
ctx-duplicate = Ç&oğalt
ctx-import-here = Buraya &içe aktar…
ctx-indent = &Girintiyi artır
ctx-outdent = Girintiyi a&zalt
ctx-trash = Çöp kutusuna &taşı
ctx-open-to-side = Ya&nda aç
ctx-open = &Aç
ctx-reveal-in-outline = Ana &hatta göster
ctx-move-up = Yukarı t&aşı
ctx-move-down = Aşağı ta&şı

ctx-tab-close = &Kapat
ctx-tab-close-others = &Diğerlerini kapat
ctx-tab-close-all = &Tümünü kapat
ctx-tab-open-to-side = &Yanda aç
ctx-tab-open-in-main = &Ana bölmede aç
ctx-tab-move-to-side = Ya&na taşı
ctx-tab-move-to-main = A&na bölmeye taşı
ctx-tab-move-to-new-window = Yeni &pencereye taşı
ctx-tab-move-window-unsaved = Önce bu projeyi kaydedin: ikinci pencere diskteki bir dosyayı açar
ctx-tab-pin = Bu sekmeyi &sabitle
ctx-tab-unpin = Bu sekmenin &sabitlemesini kaldır
tab-pinned-tooltip = Sabitlendi: "Diğerlerini kapat" ve "Tümünü kapat" bu sekmeyi açık bırakır

create-book = Kitap
create-part = Kısım
create-chapter = Bölüm
create-scene = Sahne
create-note = Not
create-note-folder = Not klasörü
create-folder = Klasör
create-paratext = Yan metin
create-paratext-folder = Yan metin klasörü
create-book-end = Kitap sonu
create-story-bible-entry = Hikâye rehberi kaydı…
type-book-start = Kitap başı
type-text = Metin

new-item-book = Yeni kitap
new-item-part = Yeni kısım
new-item-chapter = Yeni bölüm
new-item-note = Yeni not
new-item-note-folder = Yeni not klasörü
new-item-folder = Yeni klasör
new-item-paratext = Yeni yan metin
new-item-paratext-folder = Yeni yan metin klasörü
new-item-story-bible-entry = Yeni hikâye rehberi kaydı

placement-inside = içine
placement-after = sonrasına
placement-after-parent = üst öğenin sonrasına
placement-top-level = en üst düzey

ctx-promote = &Dönüştür
promote-chapter-folder = Bölüm klasörü
promote-flat-chapter = Düz bölüm
promote-lossy-title = Metnin saklanacağı yer yok
promote-lossy-text = Bir { $target } öğesinde şunları saklayacak yer yok: { $kinds }. Önce bu metni taşıyın ya da silin, sonra dönüştürün.
content-scene-text = Sahne metni
content-note-text = Not metni
content-book-title = Kitap başlığı
content-book-subtitle = Kitap alt başlığı
content-part-title = Kısım başlığı
content-chapter-title = Bölüm başlığı
content-epigraph-text = Epigraf
content-paratext-text = Yan metin
promote-blocked-title = Bölüm boş değil
promote-blocked-text = { $count ->
    [one] Bu bölümde hâlâ { $count } öğe var. Düz bölüme dönüştürmeden önce onu taşıyın ya da çöp kutusuna atın.
   *[other] Bu bölümde hâlâ { $count } öğe var. Düz bölüme dönüştürmeden önce onları taşıyın ya da çöp kutusuna atın.
}

inspector = Denetçi
inspector-empty = İncelemek için bir öğe açın.
inspector-promote = Dönüştür…
statusbar-toggle-outline = Dizini göster/gizle
statusbar-toggle-inspector = Denetçiyi göster/gizle
statusbar-save-unsaved = Kaydedilmemiş değişiklikler var. Kaydetmek için tıklayın
statusbar-save-saved = Tüm değişiklikler kaydedildi
statusbar-save-autosave = Otomatik kaydetme açık. Değişiklikler siz yazarken kaydedilir
statusbar-saving = Kaydediliyor…
statusbar-word-count = { $count ->
    [one] { $count } sözcük
   *[other] { $count } sözcük
}
statusbar-word-count-tooltip = Düzenlediğiniz sahnedeki sözcükler
statusbar-word-char-count = { $words ->
    [one] { $words } sözcük
   *[other] { $words } sözcük
} · { $chars ->
    [one] { $chars } karakter
   *[other] { $chars } karakter
}
session-toggle = Yazma oturumu: odaklı bir yazma turunu başlatın ya da duraklatın
session-configure = Oturumun sözcük hedefini ve süre sınırını ayarlayın
session-configure-title = Yazma oturumu
session-word-goal = Sözcük hedefi
session-time-limit = Süre sınırı
session-time-limit-unit = dk
session-no-goal = Hedef yok
session-no-limit = Sınır yok
session-reset = Oturumu sıfırla
session-readout = { $words ->
    [one] { $words } sözcük
   *[other] { $words } sözcük
} · { $time }
session-readout-timed = { $words ->
    [one] { $words } sözcük
   *[other] { $words } sözcük
} · { $time } kaldı
statusbar-focus-exit = Dikkat dağıtmayan moddan çık
statusbar-focus-synopsis = Özet
statusbar-focus-go-prev = Önceki öğe (Alt+Yukarı)
statusbar-focus-go-next = Sonraki öğe (Alt+Aşağı)

rail-settings = Ayarlar

language = Dil
theme = Tema
english = İngilizce
french = Fransızca
light = Açık
dark = Koyu
settings-text-width = Metin genişliği
settings-preview-width = Arama önizlemesi genişliği
settings-autosave = Diske otomatik kaydet
settings-show-welcome = Başlangıçta başlatıcıyı göster
settings-show-welcome-tip = Kapalıyken son proje yeniden açılır.
settings-check-for-updates = Yeni sürümleri denetle
settings-check-for-updates-tip = Skribisto günde en fazla bir kez, projenin web sitesine güncel sürümün hangisi olduğunu sorar. İstek hiçbir tanımlayıcı, hatta kullandığınız sürümü bile taşımaz: karşılaştırma bu bilgisayarda yapılır.

settings-title = Ayarlar
settings-close = Kapat
settings-search = Ayarlarda ara
settings-reset = Varsayılanlara sıfırla
settings-done = Bitti
settings-cancel = İptal
settings-apply = Uygula
settings-ok = Tamam
settings-reset-confirm-title = Tüm ayarlar varsayılanlara sıfırlansın mı?
settings-reset-confirm-body = Bu uygulamanın görünüm, düzenleyici ve yazma ayarlarını geri yükler. Projenizin kendi ayarları, klavye kısayolları ve kaydedilmiş biçemler etkilenmez. Bu işlem geri alınamaz.
settings-empty-title = Burada henüz ayar yok
settings-empty-hint = Bu bölüme gelecek bir güncellemede seçenekler eklenecek.

settings-sec-appearance-behaviour = Görünüm ve davranış
settings-sec-editor = Düzenleyici
settings-sec-spelling = Yazım
settings-sec-backup = Yedekleme ve eşitleme
settings-sec-compile = Derleme ve dışa aktarma
settings-page-appearance = Görünüm
settings-field-image-size-policy = Büyük resimler
settings-image-policy-ask = Her seferinde sor
settings-image-policy-keep = Özgün hâlini koru
settings-image-policy-downscale = İyileştir
settings-hint-image-size-policy = Eklenen bir resim el yazmasının gerektirdiğinden büyük olduğunda ne yapılacağı.
settings-page-notifications = Bildirimler
settings-page-scene = Sahne
settings-page-synopsis = Özet
settings-page-notes = Notlar
settings-page-editor-behavior = Düzenleyici davranışı
settings-page-goals = Hedefler ve sözcük sayısı
settings-page-games = Yazma oyunları
settings-page-corkboard = Mantar pano
settings-page-distraction-free = Dikkat dağıtmayan mod
settings-page-dictionaries = Sözlükler
settings-page-autosave = Otomatik kaydetme
settings-page-export = Dışa aktarma biçimleri
settings-page-paratext = Yan metin yapıları
settings-page-keymap = Tuş eşlemesi
settings-keymap-filter = Kısayolları süz

settings-desc-sec-appearance-behaviour = Uygulamanın kendisinin nasıl göründüğü ve açılışta ne yaptığı.
settings-desc-sec-editor = Yazma yüzeyi: nasıl göründüğü ve siz yazarken ne yaptığı.
settings-desc-sec-spelling = Yazım denetimi ve arkasındaki sözlükler.
settings-desc-sec-backup = Çalışmanızın diske nasıl ulaştığı ve hangi kopyaların tutulduğu.
settings-desc-sec-compile = Skribisto'dan neyin, hangi biçimde çıktığı.
settings-desc-sec-work = Bu projeye ait olan ve dosyasının içinde taşınan ayarlar.
settings-desc-sec-extensions = Burada kurulu eklentilerin eklediği sayfalar.
settings-desc-group-typography = Her metin türü için bir sayfa: yazı tipi, boyut, satır yüksekliği ve aralık.
settings-desc-appearance = Arayüz dili, tema, metin boyutu ve açılıştaki başlatıcı.
settings-desc-notifications = Bu oturumda gösterilen her ileti ve yeniden çalıştırabileceğiniz eylemler.
settings-desc-scene = Sahne metninin dizgisi: yazı tipi, boyut, satır yüksekliği, girintiler ve aralık.
settings-desc-synopsis = Özet bölmesinin el yazmasından bağımsız dizgisi.
settings-desc-notes = Notların el yazmasından bağımsız dizgisi.
settings-desc-corkboard = Kart boyutu, bir kartın gösterdikleri ve panonun kartları nasıl dizdiği.
settings-desc-distraction-free = Tam ekran yazma için tipografi, sütun genişliği ve denetim şeridi.
settings-desc-distraction-free-themes = Tam ekran yazma için tema kitaplığı: hazır gelenler ve sizinkiler.
settings-desc-editor-behavior = Metin genişliği, özetin konumu, daktilo kaydırması ve imleç vurgusu.
settings-desc-punctuation = Yeni projelerin başlangıçta izlediği tipografi kuralları. Kendi kurallarını belirlemeyen her proje bunları izler.
settings-desc-goals = Sözcük ve karakter hedefleri, sözcüklerin nasıl sayıldığı.
settings-desc-games = Taslak yazarken kendinize koyduğunuz kısıtlar, örneğin Hep ileri.
settings-desc-spellcheck = Yazım denetimini her yerde açıp kapatan tek anahtar.
settings-desc-dictionaries = Bu bilgisayardaki yazım sözlüklerini kurun, kaldırın ve göz atın.
settings-desc-autosave = Düzenlemelerinizin kendiliğinden diske yazılıp yazılmayacağı.
settings-desc-backup = Kopyaların ne zaman alındığı, nerede ve kaç tane tutulduğu. Kendi kuralını belirlemeyen her proje bunu izler.
settings-desc-export = Her dışa aktarmanın derlendiği biçemler: hazır gelenler ve sizinkiler.
settings-desc-paratext = Yeni bir projenin başlayabileceği ön ve arka kısımlar.
settings-desc-keymap = Her kısayol ve neye atandığı.
settings-page-user = Kullanıcı
settings-desc-user = Yazdığınız yorumlar için kim olduğunuz.
settings-group-identity = Kimlik
settings-field-user-name = Adınız
settings-field-user-name-placeholder = İsteğe bağlı
settings-field-user-name-hint = Yazdığınız yorumları ve yanıtları imzalar. Bir projenin yazar adından farklıdır: yazar adı kitabın künyesidir ve dosyanın içinde taşınır.
settings-field-user-initials = Baş harfleriniz
settings-field-user-initials-placeholder = Adınızdan
settings-field-user-initials-hint = Bir kelime işlemcinin kenar boşluğunda yorumunuzun yanında gösterdiği harfler. Adınızdan alınıp gösterilenleri kullanmak için boş bırakın.
settings-field-user-hint = İkisi de isteğe bağlıdır ve bu bilgisayardaki her projeye uygulanır. Değiştirdiğinizde bundan sonra yazacağınız yorumlar yeni adla imzalanır. Önceden yazılmış yorumlar yazıldıkları adı korur.
settings-desc-author = Bu projede yer alacak ad.
settings-desc-structure = Bu projedeki bölümlerin klasör mü yoksa tek öğe mi olduğu.
settings-desc-language = Bu projenin metninin denetlendiği dil.
settings-desc-work-backup = Bu projenin kendi yedekleme kuralı ya da genel kural.
settings-desc-personal-dictionary = Bu projenin doğru yazılmış saydığı sözcükler.
settings-desc-tags = Bu projenin öğelerini işaretlediği renkli etiketler.
settings-desc-templates = Bu projede yazarken ekleyebileceğiniz not şablonları.
settings-desc-text-replacements = Bu projede siz yazarken açılan kısaltmalar.
settings-desc-work-punctuation = Bu projenin metninin izlediği tırnaklar, tireler ve boşluklar.

settings-group-typography = Tipografi
settings-group-writing-column = Yazı sütunu
settings-group-writing-view = Yazma görünümü
settings-group-distraction-free-strip = Denetim şeridi
settings-group-theme = Tema
settings-group-language = Dil
settings-group-startup = Açılış
settings-group-updates = Güncellemeler
settings-group-autosave = Otomatik kaydetme
settings-field-typeface = Yazı tipi
settings-field-size = Metin boyutu

editor-size-changed-manuscript = El yazması metin boyutu: { $percent }
editor-size-changed-synopsis = Özet metin boyutu: { $percent }
editor-size-changed-notes = Not metin boyutu: { $percent }
editor-size-changed-corkboard = Mantar pano kart metni boyutu: { $percent }
editor-size-changed-corkboard-expanded = Genişletilmiş düzenleyici metin boyutu: { $percent }
editor-size-changed-distraction-free = Dikkat dağıtmayan mod metin boyutu: { $percent }

menu-text-size-increase = Metin boyutunu &büyüt
menu-text-size-decrease = Metin boyutunu küçü&lt
menu-text-size-reset = Metin boyutunu &sıfırla
settings-field-line-height = Satır yüksekliği
settings-field-first-line-indent = İlk satır girintisi
settings-field-paragraph-spacing-before = Paragraf öncesi boşluk
settings-field-paragraph-spacing-after = Paragraf sonrası boşluk
settings-field-column-width = Sütun genişliği
settings-distraction-free-width-hint = Yalnızca dikkat dağıtmayan mod açıkken geçerlidir. Sahne, Özet ve Notlar sütun genişlikleriniz değişmez.
settings-distraction-free-title = Öğenin adını göster
settings-distraction-free-word-count = Sözcük sayısını göster
settings-distraction-free-session = Yazma oturumunu göster
settings-distraction-free-go-to = Git… düğmesini göster
settings-distraction-free-go = Önceki ve Sonraki düğmelerini göster
settings-distraction-free-chrome-hint = Burada ne seçerseniz seçin Çık düğmesi her zaman kalır. Esc tuşu meşgulse çıkış yolunuz odur.
settings-field-app-theme = Tema
settings-theme-light = Açık
settings-theme-dark = Koyu
settings-field-text-scale = Arayüz metin boyutu
settings-field-language = Arayüz dili
settings-synopsis-placement = Özetin konumu
settings-synopsis-placement-none = Yok
settings-synopsis-placement-top = Üstte
settings-synopsis-placement-side = Yanda
settings-typewriter = Daktilo kaydırması
settings-typewriter-tip = El yazması altından kayarken yazdığınız satırı sabit bir yükseklikte tutar. Tıklamak yine de imleci tıkladığınız yere koyar.
settings-typewriter-position = Satırın konumu
settings-typewriter-position-top-third = Üst üçte bir
settings-typewriter-position-middle = Orta
settings-typewriter-position-bottom-quarter = Alt çeyrek
settings-highlight-scope = İmlecin çevresini vurgula
settings-highlight-scope-none = Yok
settings-highlight-scope-sentence = Tümce
settings-highlight-scope-paragraph = Paragraf
settings-highlight-scope-tip-none = Sayfayı sade bırakır. Siz yazarken hiçbir şey gölgelenmez.
settings-highlight-scope-tip-sentence = Yazdığınız tümceyi gölgeler, böylece çevresindekilerden ayrılır.
settings-highlight-scope-tip-paragraph = Yazdığınız paragrafın tamamını gölgeler, böylece üzerinde çalıştığınız bölüm gözünüzün önünde kalır.
settings-group-container-views = Kapsayıcı görünümleri
settings-remember-view = Her öğe türü için son görünümü hatırla
settings-remember-view-tip = Bir kapsayıcıyı, o tür için en son kullandığınız görünümde açar
settings-remember-view-tip-more =
    Kitap, Kısım ve Bölüm birkaç görünüm sunar (kendi sayfası, tüm el yazması,
    tüm özet). Bunu açtığınızda her tür, onun için en son seçtiğiniz görünümde
    yeniden açılır: örneğin bir Bölüm'ü Tüm bölüm görünümüne geçirirseniz açtığınız
    sonraki Bölüm de Tüm bölüm olarak açılır. Her tür kendi görünümünü hatırlar.
settings-group-counting = Sözcük sayımı
settings-counting-auto = Otomatik (dile göre)
settings-counting-whitespace = Boşluklardan böl
settings-counting-unicode-words = Unicode sözcükleri
settings-counting-cjk-hybrid = ÇJK duyarlı (karakter başına)
settings-counting-hint = Otomatik seçenek Çince ve Japoncayı karakter karakter, diğer tüm dilleri sözcük sözcük sayar. Yalnızca sayım dilinize göre yanlış görünüyorsa değiştirin.
settings-group-goals-display = Görüntüleme
settings-show-characters = Durum çubuğunda karakter sayısını göster
settings-autosave-hint = Değişiklikler siz yazarken otomatik olarak diske yazılır.

settings-sec-work = Proje
settings-page-structure = Yapı
settings-page-author = Yazar
settings-field-author-name = Yazar adı
settings-field-author-placeholder = İsteğe bağlı
settings-field-author-hint = Derlenen başlık sayfasında ve dışa aktarılan dosyaların üst verisinde görünür. Kullanmamak için boş bırakın. Bu, kitabın künyesindeki addır. Yorumlarınızın imzalandığı ad Ayarlar ▸ Kullanıcı bölümündedir.
settings-group-chapters = Bölümler
tidy-titles-title = Bölüm başlıklarını düzenle
tidy-titles-none = Hiçbir bölüm ya da kısım başlığı yalnızca kendi numarasını tekrarlamıyor.
tidy-titles-lead = { $count ->
        [one] Bir bölüm ya da kısmın başlığı kendi numarasından başka bir şey içermiyor.
       *[other] { $count } bölüm ve kısmın başlığı kendi numarasından başka bir şey içermiyor.
    }
tidy-titles-explain = Bu başlıkları temizlemek, her birini kitabın zaten bildiği numarayla, dışa aktarmanın yazdırdığı numarayla adlandırılmış bırakır. Başka hiçbir şey değişmez ve tek bir geri alma hepsini eski hâline getirir.
menu-document-tidy-titles = Bölüm başlıklarını &düzenle…
settings-group-numbering = Numaralandırma
settings-number-chapters = Bölümleri ve kısımları numaralandır
settings-number-chapters-tip = Bölümler ve kısımlar, kitaptaki yerlerinden hesaplanan bir numara taşır. Bu numara burada başlıklarının yanında gösterilir ve dışa aktarmada yazdırılır.
settings-number-chapters-tip-more = Numara hiçbir zaman başlığın içinde saklanmaz, bu yüzden yeniden sıraladığınızda, eklediğinizde ya da sildiğinizde doğru kalır. Bunu kapatırsanız dışa aktarma biçemi ne isterse istesin yalnızca başlıklar yazdırılır. Tek bir bölümü (bir önsöz, bir ara bölüm) dışarıda bırakmak için bunun yerine Denetçi'deki Numaralandırma anahtarını kullanın: bu, bölümü kitapta tutar ama numara almasını engeller.
settings-part-resets-chapter = Her kısımda bölüm numaralarını yeniden başlat
settings-part-resets-chapter-tip = Varsayılan olarak kapalı: bölümler bir kitabın kısımları boyunca kesintisiz sürer, böylece "İkinci kısım" On birinci bölümle açılır.
settings-part-resets-chapter-tip-more = Yayıncılıkta olağan uygulama budur ve okurun beklediği de budur. Kısımları ayrı ciltler gibi okunacak, her biri Birinci bölümle açılacak bir kitap için açın.
settings-chapter-flat = Düz bölümler
settings-chapter-flat-hint = Açık: bölüm tek bir öğedir. İçine yazarsınız ve sahne barındırmaz. Kapalı: bölüm bir klasördür. Yine içine yazarsınız ama sahneler de barındırabilir. Yeni bölümler bu ayarı izler; var olanlar Dönüştür ile değiştirilir.

settings-styles-builtin = Hazır biçemler
settings-styles-user = Biçemlerim
settings-styles-builtin-badge = Hazır
settings-styles-duplicate = Çoğalt
settings-styles-edit = Düzenle
settings-styles-delete = Sil
settings-styles-import = İçe aktar…
settings-styles-export = Dışa aktar…
settings-styles-copy-suffix = (kopya)
settings-styles-json-filter = Dışa aktarma biçemi
settings-styles-editor-title = Biçemi düzenle
settings-styles-editor-none = Düzenlemek için özel bir biçem seçin ya da hazır bir biçemi çoğaltın.
settings-styles-imported = Biçem içe aktarıldı
settings-styles-import-failed = Biçem içe aktarılamadı
settings-styles-exported = Biçem dışa aktarıldı
settings-styles-export-failed = Biçem dışa aktarılamadı
settings-styles-field-name = Ad
settings-styles-field-chapters = Bölüm başlıkları
settings-styles-field-parts = Kısım başlıkları
settings-styles-field-scene-break = Sahne arası
settings-styles-field-major-scene-break = Büyük sahne arası
settings-styles-field-spacing = Satır aralığı
settings-styles-field-justify = Metni iki yana yasla
settings-styles-field-notes = Notları dahil et
settings-styles-field-synopses = Özetleri dahil et
settings-styles-field-scene-titles = Sahne başlıklarını dahil et
settings-styles-field-epigraphs = Epigrafları dahil et
settings-styles-field-paratexts = Yan metinleri dahil et
settings-styles-field-epigraph-placement = Epigrafın konumu
settings-styles-epigraph-after = Başlıktan sonra
settings-styles-epigraph-before = Başlıktan önce
settings-styles-field-footnotes = Dipnotları dahil et
settings-styles-field-footnote-numbering = Dipnot numaralandırması
settings-styles-footnote-numbering-continuous = Kesintisiz
settings-styles-footnote-numbering-per-chapter = Her bölümde yeniden başlat
settings-styles-footnote-numbering-per-book = Her kitapta yeniden başlat
settings-styles-field-images = Resimler
menu-image = &Resim
settings-styles-images-beside = Belgenin yanında
settings-styles-images-embed = Belgenin içinde
settings-styles-images-omit = Dışarıda bırak
settings-styles-group-round-trip = Editöre gönderme
settings-styles-field-comments = Yorumları dahil et
settings-styles-field-round-trip-marks = Gidiş-dönüş işaretçilerini dahil et
settings-styles-round-trip-hint = Yalnızca DOCX ve ODT. İşaretçiler, geri dönen bir dosyanın ikinci bir kopya olarak yanına düşmek yerine bu projeyi güncellemesini sağlayan görünmez tanımlayıcılardır.
settings-styles-group-pages = Sayfalar
settings-styles-field-cover = Kapakla aç
settings-styles-field-word-count = Başlık sayfasında sözcük sayısı
settings-styles-field-page-books = Her kitapta yeni sayfa
settings-styles-field-page-parts = Her kısımda yeni sayfa
settings-styles-field-page-chapters = Her bölümde yeni sayfa
settings-styles-field-page-paratexts = Her yan metinde yeni sayfa
settings-styles-heading-none = Başlık yok
settings-styles-heading-numbered = Yalnızca numara
settings-styles-heading-title = Yalnızca başlık
settings-styles-heading-both = Numara + başlık
settings-styles-break-blank = Boş satır
settings-styles-break-none = Yok
settings-styles-spacing-single = Tek
settings-styles-spacing-onehalf = 1½
settings-styles-spacing-double = Çift

welcome-title = Skribisto'ya hoş geldiniz
welcome-version = Sürüm { $version }
welcome-update-available = Sürüm { $version } kullanıma sunuldu
welcome-update-tip = İndirme sayfasını tarayıcınızda açar.
welcome-search = Projelerde ara
welcome-open = Aç
welcome-new-work = Yeni proje
welcome-create-from = Kaynaktan oluştur…
welcome-recent-works = Son projeler
welcome-empty-recents = Henüz son proje yok.
welcome-no-matches = Aramanızla eşleşen son proje yok.
welcome-learn-soon = Kılavuzlar ve ipuçları yakında geliyor.
welcome-about-blurb = Skribisto, yazma uygulamasının Rust + Teksilo ile yeniden yazılmış hâli.
welcome-tagline = *Uzun metinler yazmak için sessiz bir yer.*
welcome-github = GitHub
welcome-discord = Discord
nav-works = Projeler
nav-examples = Örnekler
nav-learn = Öğren
nav-about = Hakkında

synopsis = Özet
epigraph = Epigraf
pane-manuscript = El yazması
corkboard = Mantar pano
overview = Genel bakış

corkboard-card-count = { $count ->
    [one] { $count } kart
   *[other] { $count } kart
}
corkboard-child-count = { $count ->
    [one] { $count } öğe
   *[other] { $count } öğe
}
corkboard-view-nested = İç içe
corkboard-view-flat = Düz
corkboard-layout-hint = İç içe görünüm bir kapsayıcının doğrudan alt öğelerini gösterir. İçine girmek için bir klasör kartını açın. Düz görünüm tüm kapsayıcıdaki bütün sahneleri bir arada gösterir.
corkboard-card-size = Kart boyutu
corkboard-search-placeholder = Kartları süz…
corkboard-empty-title = Burada henüz bir şey yok
corkboard-empty-hint = İlkini eklemek için yukarıdaki “＋ Yeni” düğmesini kullanın.
corkboard-new = Yeni
corkboard-show-card-numbers = Kartları numaralandır
corkboard-modal-size = Genişletilmiş düzenleyici boyutu
corkboard-scope-hint = Her projedeki tüm açık mantar panolara uygulanır.
corkboard-sort-manuscript = El yazması sırası
corkboard-sort-title-asc = Başlık A–Z
corkboard-sort-title-desc = Başlık Z–A
corkboard-card-number = Kart { $number }

duplicate = Çoğalt
duplicate-n = { $count ->
    [one] { $count } kartı çoğalt
   *[other] { $count } kartı çoğalt
}
set-label-n = { $count ->
    [one] { $count } karta açıklama koy
   *[other] { $count } karta açıklama koy
}
move-to-trash-n = { $count ->
    [one] { $count } kartı çöp kutusuna taşı
   *[other] { $count } kartı çöp kutusuna taşı
}
reveal-in-outline = Ana hatta göster

corkboard-move-to = Taşı…
corkboard-move-to-n = { $count ->
    [one] { $count } kartı taşı…
   *[other] { $count } kartı taşı…
}
corkboard-move-picker-title = Taşınacak yer…
corkboard-move-picker-empty = Henüz dizin yok. Önce bir tane oluşturun.
corkboard-move-picker-cancel = İptal
corkboard-move-here = Buraya taşı
corkboard-moved-ok = { $count ->
    [one] { $count } kart taşındı
   *[other] { $count } kart taşındı
}
corkboard-move-into-self = Bir kapsayıcı kendi içine taşınamaz. Onun dışında bir yer seçin.
corkboard-move-failed = Bu kartlar oraya taşınamadı.

overview-col-title = Başlık
overview-col-type = Tür
overview-col-label = Açıklama
overview-col-tags = Etiketler
overview-col-books = Kitaplar
overview-col-own-words = Sözcük
overview-col-total-words = Toplam
overview-row-count = { $count ->
    [one] { $count } öğe
   *[other] { $count } öğe
}
overview-search-placeholder = Öğeleri süz…
overview-expand-all = Tümünü genişlet
overview-collapse-all = Tümünü daralt
overview-table-label = İçindekiler
overview-empty-title = Burada henüz bir şey yok
overview-gone-title = Bu kapsayıcı artık yok
overview-gone-hint = Çöp kutusuna taşındı. Geri yükleyin ya da bu sekmeyi kapatın.
overview-empty-hint = İlkini eklemek için yukarıdaki “＋ Yeni” düğmesini kullanın.
corkboard-grid-label = Mantar pano kartları
corkboard-rename-field = Öğeyi yeniden adlandır
corkboard-expand-synopsis = Özeti genişlet
corkboard-synopsis-modal-title = Özet
corkboard-badge-scene = Sahne
corkboard-badge-chapter = Bölüm
corkboard-badge-part = Kısım
corkboard-badge-book = Kitap
corkboard-badge-note = Not
corkboard-badge-folder = Klasör
corkboard-badge-paratext = Yan metin
corkboard-badge-text = Metin
corkboard-badge-end = Son
settings-group-corkboard-layout = Yerleşim
settings-group-corkboard-cards = Kartlar
corkboard-show-word-count = Kartlarda sözcük sayısını göster
text-heading = Metin
no-content = Bu öğenin düzenlenebilir içeriği yok.
untitled = Adsız
placeholder-title = Başlık…
placeholder-subtitle = Alt başlık…
placeholder-chapter-title = Bölüm başlığı…
split-editor = Düzenleyiciyi böl
close-split-view = Bölünmüş görünümü kapat
drop-open-here = Burada aç
drop-open-to-side = Yanda aç

segment-chapter = Bölüm
segment-part = Kısım
segment-book = Kitap
segment-pace = Tempo
segment-notes = Notlar
segment-story-bible = Hikâye rehberi
segment-note-own = Not
segment-note-details = Ayrıntılar
segment-note-in-prose = Metinde
pace-placeholder = Tempo planlayıcısı burada görünür.
pace-empty-title = Bu kitabın temposunu planlayın
pace-empty-body = Bir sözcük hedefi ve bir son tarih belirleyin, Skribisto oraya varmanız için gereken günlük tempoyu hesaplasın.
pace-start-planning = Planlamaya başla
pace-section-schedule = Takvim
pace-section-progress = İlerleme
pace-advancement = İlerleme durumu
pace-goal = Sözcük hedefi
pace-deadline = Son tarih
pace-active = Tempo etkin
pace-day-mon = Pzt
pace-day-tue = Sal
pace-day-wed = Çar
pace-day-thu = Per
pace-day-fri = Cum
pace-day-sat = Cmt
pace-day-sun = Paz
pace-card-written = yazılan sözcük
pace-card-of-goal = hedefin
pace-card-rate = sözcük / yazma günü
pace-card-days-left = kalan yazma günü
pace-card-streak = günlük seri
pace-card-ahead = sözcük tempo önünde
pace-card-behind = sözcük tempo gerisinde
pace-charts-empty = Sözcük sayınız bir kaydetmede kaydedildiğinde ilerleme grafikleri burada görünür.
pace-chart-progression = Yazılan sözcükler ve öngörülen
pace-chart-words-per-day = Günlük sözcük
pace-series-actual = Gerçekleşen
pace-series-target = Öngörülen
pace-series-words-per-day = Sözcük/gün
pace-daily-target-line = Düzenli tempo: { $count ->
    [one] { $count } sözcük/gün
   *[other] { $count } sözcük/gün
}
pace-section-holidays = Tatiller
pace-section-milestones = Ara hedefler
pace-holidays-none = Tatil yok. Planlanan her gün sayılır.
pace-holiday-label = Tatilin adı
pace-add-holiday = Ekle
pace-remove = Kaldır
pace-milestones-none = Henüz ara hedef yok. Denetçi'de bir Kısım ya da Bölüm için belirleyin.
full-chapter = Tüm bölüm
full-part = Tüm kısım
full-book = Tüm kitap
full-synopsis = Tüm özet
rename = Yeniden adlandır
set-label = Açıklama koy
insert-scene = Sahne ekle
insert-chapter = Bölüm ekle
split-scene = Sahneyi böl
move-up = Yukarı taşı
move-down = Aşağı taşı
merge-with-previous = Öncekiyle birleştir
move-to-trash = Çöp kutusuna taşı
rename-chapter = Bölümü yeniden adlandır
add-scene = Yeni sahne ekle
add-chapter = Yeni bölüm ekle
new-scene-title = Yeni sahne
placeholder-scene-name = Sahne adı…
menu-cut = Kes
menu-copy = Kopyala
menu-paste = Yapıştır
menu-paste-unformatted = Biçimsiz yapıştır
menu-select-all = Tümünü seç
format-bold = Kalın
format-italic = İtalik
format-underline = Altı çizili
format-strikethrough = Üstü çizili

binder = Dizin
no-work = Yüklü proje yok
no-recent-works = Son proje yok
switcher-open-section = Şu anda açık
switcher-recent-section = Son kullanılanlar
switcher-this-window = bu pencere
open-project-title = Projeyi aç
open-project-question = “{ $title }” nasıl açılsın?
open-in-new-window = Yeni pencerede aç
open-here = Burada aç

binder-all = Tüm dizinler
binder-show-all = Tüm dizinleri göster
binder-new = Yeni dizin…
binder-default-name = Yeni dizin
binder-item-count = { $count ->
    [one] { $count } öğe
   *[other] { $count } öğe
}
binder-search-placeholder = Ana hattı süz…
binder-search-scope = Tüm dizinlerde ara
binder-trash-confirm-title = Dizin çöp kutusuna taşınsın mı?
binder-trash-confirm-text = “{ $name }” ve tüm öğeleri çöp kutusuna taşınacak.

dialog-rename = Yeniden adlandır
dialog-new-binder = Yeni dizin
dialog-set-label = Açıklama koy
dialog-new-scene = Yeni sahne
close-work-question = Proje kapatılmadan önce değişiklikler kaydedilsin mi?
quit-question = Çıkmadan önce değişiklikler kaydedilsin mi?
quit-save-work-question = Çıkmadan önce “{ $title }” projesindeki değişiklikler kaydedilsin mi?
unsaved-changes = Bu projede kaydedilmemiş değişiklikler var.
new-work-unsaved-question = Yeni proje oluşturmadan önce değişiklikler kaydedilsin mi?
open-work-unsaved-question = Başka bir proje açmadan önce değişiklikler kaydedilsin mi?
switch-backup-discard-title = Bu yedekteki değişiklikler atılsın mı?
switch-backup-discard-text = Bir yedekte yapılan değişiklikler o yedeğe kaydedilemez. Saklamak için Farklı kaydet ya da Geri yükle'yi kullanın veya değişiklikleri atıp diğer projeyi açın.
switch-save-failed = Proje kaydedilemediği için değiştirilmedi: { $error }
switch-save-not-started = Proje kaydedilemediği için değiştirilmedi.
close-save-failed = Proje kaydedilemediği için kapatılmadı: { $error }
close-save-not-started = Proje kaydedilemediği için kapatılmadı.
save-not-started = Proje kaydedilemedi.

could-not-open-work = "{ $file }" açılamadı
could-not-open-work-too-new = "{ $file }" için daha yeni bir Skribisto gerekiyor
could-not-open-work-too-new-detail = Skribisto biçim sürümü { $written_by } ile kaydedilmiş; açmak için biçim { $requires } ya da daha yenisi gerekiyor, bu sürüm ise en fazla biçim { $supported } destekliyor. Açmak için Skribisto'yu güncelleyin.
could-not-open-work-importing = "{ $file }" şu anda bir içe aktarma tarafından yazılıyor
could-not-open-work-importing-detail = Bu projenin üzerine bir içe aktarma yazıyor ve henüz bitmedi. İçe aktarma tamamlanınca açın: şimdi açılırsa içe aktarmanın yerini aldığı projeyi gösterir ve sonraki kaydetme o projeyi içe aktarılanın üzerine geri yazar.
could-not-open-example = Örnek açılamadı: { $error }
could-not-open-link = { $url } açılamadı: { $error }
could-not-create-work = Proje oluşturulamadı: { $error }
saving-as-file = { $target } olarak kaydediliyor…
saving-as-folder = { $target }/ olarak kaydediliyor…
saved-as = Şuraya kaydedildi: { $target }
save-error = Kaydedilemedi: { $error }
save-flush-failed = Kaydetme tamamlandı ama şu belgeler kaydetmeye aktarılamadı ve hâlâ kaydedilmemiş durumda: { $titles } ({ $error })
backup-error = Yedeklenemedi: { $error }
backing-up = Yedekleniyor…
backup-nothing-open = Yedeklenecek açık proje yok.
backup-already-running = Zaten bir yedekleme sürüyor.
backup-complete = Yedekleme tamamlandı ({ $ok } kaydedildi, { $skipped } zaten güncel)
backup-partial = Yedekleme bitti: { $ok } kaydedildi, { $failed ->
    [one] { $failed } konum başarısız oldu
   *[other] { $failed } konum başarısız oldu
}
backup-no-destination-title = Kullanılabilir yedekleme konumu yok
backup-no-destination-text = Yapılandırılmış yedekleme konumlarının hiçbirine ulaşılamıyor (örneğin harici bir sürücü takılı olmayabilir). Sürücüyü takıp yeniden deneyin ya da yedeklemeden çıkın.
backup-failed-close-title = Yedek kaydedilemedi
backup-failed-close-text = Kapatmadan önce hiçbir yedek kopya yazılamadı. Tüm konumlar başarısız oldu (sürücü çıkarılmış, dolu ya da yazmaya karşı korumalı olabilir). Sorunu giderip yeniden deneyin ya da yedeklemeden çıkın.

backup-choice-title = Yedek dosyası
backup-choice-heading = Bir yedek kopya açtınız
backup-choice-subtitle = Bu, bir projenin belirli bir andaki yedeğidir.
backup-choice-subtitle-dated = Yedeğin alındığı tarih: { $date }.
backup-choice-body = Açıp dilediğiniz gibi düzenleyebilirsiniz, ancak değişiklikler yalnızca Farklı kaydet ile saklanabilir. Özgün proje dosyasına dokunulmaz. Ya da bu projeyi tam olarak bu yedeğe geri yükleyin.
backup-choice-open = Yedeği aç
backup-choice-restore = Bu projeyi bu noktaya geri yükle…
backup-choice-not-a-backup = Hayır, normal şekilde aç
backup-banner-title = Yedek kopya: değişiklikler buraya kaydedilemez
backup-banner-description = Düzenlemelerinizi yeni bir dosyada saklamak için Farklı kaydet'i, özgün projeyi bu yedekle değiştirmek için Geri yükle'yi kullanın.
backup-banner-restore = Geri yükle…
backup-banner-save-as = Farklı kaydet…
backup-restore-original-missing = Üzerine geri yüklenecek özgün proje bulunamıyor. Bunun yerine bu yedeği yeni bir proje olarak saklamak için Farklı kaydet'i kullanın.
backup-restore-close-elsewhere-title = Proje başka bir pencerede açık
backup-restore-close-elsewhere-text = Üzerine geri yüklediğiniz proje başka bir pencerede açık. Lütfen önce orada kapatıp yeniden deneyin.
backup-restore-focus-window = O pencereye geç
backup-restore-confirm-title = Bu yedek geri yüklensin mi?
backup-restore-confirm-text = Projenin şu anki sürümü, bununla değiştirilmeden önce güvenlik yedeği olarak bir kenara kopyalanacak.
backup-restore-confirm-ok = Geri yükle
backup-restore-error = Geri yüklenemedi: { $error }
backup-restored-ok = Proje geri yüklendi.
backup-restored-with-safety = Proje geri yüklendi. Önceki sürümünüz şuraya kaydedildi: { $path }.
close-backup-discard-title = Bu yedekteki değişiklikler atılsın mı?
close-backup-discard-text = Bir yedekte yapılan değişiklikler o yedeğe kaydedilemez. Saklamak için Farklı kaydet'i kullanın ya da değişiklikleri atıp kapatın.
quit-backup-discard-title = Değişiklikler atılıp çıkılsın mı?
quit-backup-discard-work-question = “{ $title }” projesindeki değişiklikler atılıp çıkılsın mı?
quit-backup-discard-text = Bir yedekte yapılan değişiklikler o yedeğe kaydedilemez. Saklamak için Farklı kaydet'i kullanın ya da değişiklikleri atıp çıkın.
backup-nudge-text = Bu proje için ayarlanmış yedekleme yok.
backup-nudge-action = Yedeklemeyi ayarla…

menu-backups-list = Yed&ekler…
backups-title = Yedekler
backups-loading = Yedekler aranıyor…
backups-empty = Bu proje için henüz yedek bulunamadı.
backups-open = Aç
backups-reveal = Klasörde göster
backups-delete = Bu yedeği sil
backups-delete-confirm-title = Bu yedek silinsin mi?
backups-delete-confirm-text = “{ $name }” kalıcı olarak silinecek. Bu işlem geri alınamaz.
backups-delete-error = Yedek silinemedi: { $error }
backups-refresh = Yenile
backups-close = Kapat

settings-page-backup = Yedekleme varsayılanları
settings-page-work-backup = Yedekler
settings-backup-general-title = Varsayılan yedekleme ayarları
settings-backup-work-title = Bu projenin yedekleri
settings-backup-inherit = Genel yedekleme ayarlarını kullan
settings-backup-inheriting = Bu proje genel yedekleme ayarlarını kullanıyor.
settings-backup-none-hint = Otomatik yedekleme ayarlanmamış (tüm tetikleyiciler kapalı).
settings-backup-last = Son yedek: { $date }
settings-backup-last-never = Henüz yedek yok.
settings-backup-open-list = Yedek listesini aç…
settings-backup-triggers = Ne zaman yedeklensin
settings-backup-on-close = Proje kapatılırken yedekle
settings-backup-on-open = Proje açılırken yedekle
settings-backup-interval = Düzenli aralıklarla yedekle, her
settings-backup-destinations = Yedekleme konumları
settings-backup-default-location = Varsayılan konum
settings-backup-usage-since = { $count ->
    [one] { $count } yedek · { $size } · en eskisi { $oldest }
   *[other] { $count } yedek · { $size } · en eskisi { $oldest }
}
settings-backup-usage = { $count ->
    [one] { $count } yedek · { $size }
   *[other] { $count } yedek · { $size }
}
settings-backup-usage-empty = Burada henüz bir şey tutulmuyor
settings-backup-usage-measuring = Ölçülüyor…
settings-backup-reveal-root = Klasörü göster
settings-backup-dest-none = Konum yok. Yedekler projenin yanına kaydedilir.
settings-backup-dest-remove = Kaldır
settings-backup-dest-add = Klasör ekle…
settings-backup-dest-refresh = Yenile
settings-backup-retention = Kaç tane tutulsun
settings-backup-retention-tiered = Kademeli
settings-backup-retention-keep-n = Son N taneyi tut
settings-backup-retention-tip = Eski yedeklerin nasıl ayıklandığı.
settings-backup-retention-tip-more = Kademeli seçenek bir gün boyunca saatte bir, bir hafta boyunca günde bir, bir ay boyunca haftada bir ve sonrasında ayda bir yedek tutar; böylece yakın geçmiş sık, uzak geçmiş seyrek kalır. “Son N taneyi tut” yalnızca en yeni N kopyayı tutar. İki modda da en yeni kopyalar (aşağıdaki en az sayı) her zaman tutulur.
settings-backup-gfs-hourly = Saatlik (son 24 saat)
settings-backup-gfs-daily = Günlük (son hafta)
settings-backup-gfs-weekly = Haftalık (son ay)
settings-backup-gfs-monthly = Aylık
settings-backup-keep-n = Tutulacak sayı
settings-backup-min-keep = Her zaman en az şu kadar tut
settings-backup-dedup = Hiçbir şey değişmediyse yedeği atla

new-work-title = Yeni proje
new-work-close = Kapat
new-work-name = Proje adı
new-work-name-placeholder = Adsız
new-work-author = Yazar
new-work-author-placeholder = İsteğe bağlı
new-work-format = Biçim
new-work-single-file = Tek dosya
new-work-bundle = Klasör paketi
new-work-convert-later = Biçimler arasında daha sonra dönüştürme yapabilirsiniz.
new-work-location = Konum
new-work-will-create = Oluşturulacak
new-work-language = Varsayılan dil
new-work-language-hint = Yeni metinlere ve yazım denetimine uygulanır. Her metin başka bir dile geçirilebilir.
new-work-template = Şablon
new-work-template-none = Yok
new-work-template-empty-novel = Boş roman
new-work-template-light-novel = Hafif roman
new-work-template-novel = Roman
new-work-template-novel-in-parts = Kısımlara ayrılmış roman
new-work-template-notebook = Defter
new-work-cancel = İptal
new-work-create = Projeyi oluştur
new-work-step-details = Ayrıntılar
new-work-step-language = Dil ve yapı
new-work-step-template = Şablon
new-work-back = Geri
new-work-next = İleri
new-work-documents-title = Belgelerden yeni proje
new-work-step-import = İçe aktar
new-work-create-and-import = Oluştur ve içe aktar…
new-work-documents-next-title = Sırada belgeleriniz var
new-work-documents-next-body = Proje oluşturulunca üzerinde içe aktarma sihirbazı açılır: orada belgeleri seçer, Skribisto'nun onlardan okuduğu yapıyı gözden geçirir ve nereye yerleşeceğini belirlersiniz. Orada onaylayana kadar projeye hiçbir şey yazılmaz.
new-work-documents-no-template = Bu proje bilerek boş başlar: şablon yok, böylece içinde yalnızca içe aktarılan belgeler olur.
new-work-documents-chapter-scene-hint = İçe aktarmanın oluşturduğu bölümlere uygulanır. Daha sonra projenin ayarlarından değiştirebilirsiniz.
new-work-single-file-desc = Tek bir .skrib arşivi (zip). Taşınabilir, yedeklemesi kolay.
new-work-bundle-desc = Her metni ve varlığı içeren bir klasör. Sürüm denetimiyle daha uyumlu.
new-work-template-none-count = boş dizin
new-work-template-empty-novel-count = 1 bölüm
new-work-template-light-novel-count = 15 bölüm
new-work-template-novel-count = 20 bölüm
new-work-template-novel-in-parts-count = 3 kısım, 24 bölüm
new-work-template-notebook-count = serbest notlar
new-work-chapter-scene = Düz bölümler
new-work-chapter-scene-tip = Her bölüm, doğrudan içine yazdığınız ve altında sahne olmayan tek bir öğedir. Bölümün bir klasör olduğu klasik düzen için bunu kapalı bırakın: ona da doğrudan yazarsınız ama sahneler de barındırabilir.
new-work-chapter-scene-tip-more = Her iki durumda da bölümün içine yazarsınız. Tek fark, bölümün sahne *barındırıp* barındıramayacağıdır. Skribisto'nun dizin ağacı yalnızca düzenleme amaçlıdır; bu yüzden iki düzen de aynı kitaba derlenir, ikisini dilediğiniz gibi karıştırabilirsiniz ve Dönüştür bir bölümü tek sözcük kaybetmeden ikisi arasında değiştirir.
new-work-name-required = Proje için bir ad girin
new-work-name-invalid = Bu adda kullanılabilir karakter yok
new-work-location-required = Bir konum seçin
new-work-location-missing = Bu klasör yok
new-work-location-not-folder = Bu yol bir klasör değil
new-work-location-readonly = Bu klasöre yazılamıyor
new-work-target-exists = Bu klasörde bu adda bir proje zaten var. Proje oluşturulurken değiştirilmeden önce size sorulacak
new-work-target-is-folder = Bu konumda bu adda bir klasör zaten var. Başka bir ad seçin
new-work-overwrite-title = Var olan proje değiştirilsin mi?
new-work-overwrite-text = “{ $name }” bu klasörde zaten var. Yeni projeyle değiştirilsin mi?
new-work-target-open-title = Bu proje açık
new-work-target-open-text = “{ $name }” Skribisto'da açık. Onun yerine yeni bir proje oluşturmak üzerinde çalıştığınız projeyi değiştirir ve sonraki kaydetme o projeyi yenisinin üzerine geri yazar. Başka bir ad ya da klasör seçin veya önce projeyi kapatın.
target-importing-title = Bu projeye bir içe aktarma yazıyor
target-importing-text = Bir içe aktarma hâlâ “{ $name }” projesini yazıyor. Bitene kadar bekleyin ya da başka bir ad seçin.
import-plume-busy-title = Başka bir Plume Creator projesi içe aktarılıyor
import-plume-busy-text = Plume Creator projeleri birer birer içe aktarılır. Süren içe aktarmanın bitmesini bekleyin ya da onu iptal edin, sonra bunu yeniden başlatın.
import-manuskript-busy-title = Başka bir Manuskript projesi içe aktarılıyor
import-manuskript-busy-text = Manuskript projeleri birer birer içe aktarılır. Süren içe aktarmanın bitmesini bekleyin ya da onu iptal edin, sonra bunu yeniden başlatın.
backup-restore-importing-text = Bir içe aktarma hâlâ, bu yedeğin üzerine geri yükleneceği “{ $name }” projesini yazıyor. Yedeği içe aktarma bittikten sonra geri yükleyin.

new-work-manuscript = El yazması
new-work-notes = Notlar
new-work-research = Araştırma
new-work-notebook = Defter
new-work-scene = Sahne
new-work-note = Not
new-work-front-matter = Ön kısım
new-work-back-matter = Arka kısım
new-work-paratext = Kitap yapısı
new-work-paratext-none = Yapı yok
new-work-paratext-hint = Bir geleneğin kitabı açtığı ve kapattığı ön ve arka kısımlar. Daha sonra herhangi birini taşıyabilir, yeniden adlandırabilir ya da silebilirsiniz.
new-work-tags = Etiketler
new-work-tags-none = Etiket yok
new-work-tags-hint = Kişileri, yerleri ve diğerlerini etiketlemek için bir başlangıç paleti. İsteğe bağlıdır; her etiket daha sonra yeniden adlandırılabilir, yeniden renklendirilebilir ya da silinebilir.
new-work-note-templates = Not şablonları
new-work-note-templates-none = Şablon yok
new-work-note-templates-hint = Bir hikâye rehberi notunun başlangıçtaki biçimi. İsteğe bağlıdır; her şablon daha sonra düzenlenebilir ya da silinebilir.
new-work-characters = Kişiler
new-work-places = Yerler

import-plume-title = Plume Creator projesini içe aktar
import-plume-close = Kapat
import-plume-source = Plume projesi
import-plume-source-hint = Bir .plume ya da .plume_backup dosyası seçin (herhangi bir Plume Creator sürümü).
import-plume-location = Hedef klasör
import-plume-name = Dosya adı
import-plume-name-placeholder = Proje adı
import-plume-will-create = Oluşturulacak
import-plume-trash-warning = ⚠ Çöp kutusundaki / silinmiş öğeler aktarılmaz.
import-plume-cancel = İptal
import-plume-import = İçe aktar
import-plume-source-required = Bir Plume proje dosyası seçin
import-plume-source-missing = Bu dosya yok
import-plume-source-not-file = Bu yol bir dosya değil
import-plume-location-required = Bir hedef klasör seçin
import-plume-location-missing = Bu klasör yok
import-plume-location-not-folder = Bu yol bir klasör değil
import-plume-location-readonly = Bu klasöre yazılamıyor
import-plume-name-required = Bir dosya adı girin
import-plume-name-exists = Burada bu adda bir dosya zaten var. İçe aktarma, üzerine yazmadan önce onay isteyecek
import-plume-overwrite-title = Var olan dosya değiştirilsin mi?
import-plume-overwrite-text = “{ $name }” zaten var. İçe aktarılan projeyle değiştirilsin mi?
import-plume-manuscript-binder = El yazması
import-plume-story-bible-binder = Hikâye rehberi
import-plume-progress-title = Plume projesi içe aktarılıyor…
import-plume-cancel-import = İptal
import-plume-cancelled = İçe aktarma iptal edildi
import-plume-done = { $imported ->
    [one] { $imported } öğe içe aktarıldı.
   *[other] { $imported } öğe içe aktarıldı.
} { $skipped ->
    [one] Çöp kutusundaki { $skipped } öğe aktarılmadı.
   *[other] Çöp kutusundaki { $skipped } öğe aktarılmadı.
}
import-plume-open-now = Şimdi aç
import-plume-warnings = { $count ->
    [one] 1 şey birebir içe aktarılamadı
   *[other] { $count } şey birebir içe aktarılamadı
}
import-plume-details = Ayrıntılar
import-plume-warnings-title = İçe aktarma uyarıları
import-plume-error-title = Proje içe aktarılamadı
import-plume-nested-too-deep = Proje içe aktarılmadı: “{ $part }” bölümü { $limit } düzeyden daha derin iç içe geçmiş. Hiçbir Plume Creator projesi buna yaklaşmaz ve onu okumak Skribisto'yu çökertebilir. Hiçbir şey yazılmadı.
import-plume-declares-entities = Proje içe aktarılmadı: “{ $part }” bölümü XML varlıkları tanımlıyor, oysa hiçbir Plume Creator projesi bunu yapmaz. Bunları açmak bilgisayarın belleğinden fazlasını gerektirip Skribisto'yu çökertebilir. Hiçbir şey yazılmadı.
import-plume-error-details = Ayrıntılar

import-manuskript-title = Manuskript projesini içe aktar
import-manuskript-close = Kapat
import-manuskript-source = Manuskript projesi
import-manuskript-source-hint = .msk dosyasını ya da proje klasörünü seçin (herhangi bir Manuskript sürümü).
import-manuskript-source-file = Dosya seç…
import-manuskript-source-folder = Klasör seç…
import-manuskript-location = Hedef klasör
import-manuskript-name = Dosya adı
import-manuskript-name-placeholder = Proje adı
import-manuskript-will-create = Oluşturulacak
import-manuskript-cancel = İptal
import-manuskript-import = İçe aktar
import-manuskript-source-required = Bir Manuskript projesi seçin
import-manuskript-source-missing = Bu dosya ya da klasör yok
import-manuskript-location-required = Bir hedef klasör seçin
import-manuskript-location-missing = Bu klasör yok
import-manuskript-location-not-folder = Bu yol bir klasör değil
import-manuskript-location-readonly = Bu klasöre yazılamıyor
import-manuskript-name-required = Bir dosya adı girin
import-manuskript-name-exists = Burada bu adda bir dosya zaten var. İçe aktarma, üzerine yazmadan önce onay isteyecek
import-manuskript-overwrite-title = Var olan dosya değiştirilsin mi?
import-manuskript-overwrite-text = “{ $name }” zaten var. İçe aktarılan projeyle değiştirilsin mi?
import-manuskript-manuscript-binder = El yazması
import-manuskript-story-bible-binder = Hikâye rehberi
import-manuskript-characters-group = Kişiler
import-manuskript-world-group = Dünya
import-manuskript-plots-group = Olay örgüleri
import-manuskript-project-info-note = Proje bilgileri
import-manuskript-summary-note = Özet
import-manuskript-importance-minor = Küçük
import-manuskript-importance-secondary = İkincil
import-manuskript-importance-main = Ana
import-manuskript-progress-title = Manuskript projesi içe aktarılıyor…
import-manuskript-cancel-import = İptal
import-manuskript-cancelled = İçe aktarma iptal edildi
import-manuskript-done = { $imported ->
    [one] { $imported } öğe içe aktarıldı.
   *[other] { $imported } öğe içe aktarıldı.
} { $revisions ->
    [0] { "" }
    [one] Onunla birlikte { $revisions } eski sürüm de geldi.
   *[other] Onlarla birlikte { $revisions } eski sürüm de geldi.
}
import-manuskript-open-now = Şimdi aç
import-manuskript-warnings = { $count ->
    [one] Bu içe aktarma hakkında bilinmesi gereken 1 şey
   *[other] Bu içe aktarma hakkında bilinmesi gereken { $count } şey
}
import-manuskript-details = Ayrıntılar
import-manuskript-warnings-title = Bu içe aktarma hakkında
import-manuskript-error-title = Proje içe aktarılamadı
import-manuskript-nested-too-deep = Proje içe aktarılmadı: “{ $part }” dosyası { $limit } düzeyden daha derin iç içe geçmiş. Hiçbir Manuskript projesi buna yaklaşmaz ve onu okumak Skribisto'yu çökertebilir. Hiçbir şey yazılmadı.
import-manuskript-folders-too-deep = Proje içe aktarılmadı: dosyalarından biri { $limit } klasörden daha derinde duruyor. Hiçbir Manuskript projesi buna yaklaşmaz ve onu okumak Skribisto'yu çökertebilir. Hiçbir şey yazılmadı.
import-manuskript-declares-entities = Proje içe aktarılmadı: “{ $part }” dosyası XML varlıkları tanımlıyor, oysa hiçbir Manuskript projesi bunu yapmaz. Bunları açmak bilgisayarın belleğinden fazlasını gerektirip Skribisto'yu çökertebilir. Hiçbir şey yazılmadı.
import-manuskript-error-details = Ayrıntılar

import-target-open-title = Bu proje açık
import-target-open-text = “{ $name }” Skribisto'da açık. Üzerine içe aktarmak üzerinde çalıştığınız projeyi değiştirir ve sonraki kaydetme o projeyi içe aktarılanın üzerine geri yazar. Başka bir dosya adı seçin ya da önce projeyi kapatın.
import-warnings-announcement = { $result } { $notice }.

export-title = Dışa aktar
export-close = Kapat
export-scope-label = Ne
export-format-label = Biçim
export-style-label = Biçem
export-path-label = Dosya
export-browse = Gözat…
export-show-non-exportable = Dışa aktarılmayanları göster
export-choose-empty = Seçilecek proje yok.
export-cancel = İptal
export-export = Dışa aktar
export-save-dialog-title = Dosyaya dışa aktar
export-section-what = Dışa aktarılacaklar
export-section-style = Biçem
export-section-destination = Kayıt yeri
export-custom-selection = Özel seçim
export-selected-count = { $count } seçili
export-chip-chapters-none = Bölümler: yok
export-chip-chapters-numbered = Bölümler: numaralı
export-chip-chapters-title = Bölümler: yalnızca başlık
export-chip-chapters-both = Bölümler: numara + başlık
export-chip-scene-break-glyph = Sahne arası: { $glyph }
export-chip-scene-break-blank = Sahne arası: boş satır
export-chip-scene-break-none = Sahne arası: yok
export-chip-major-break-glyph = Büyük ara: { $glyph }
export-chip-major-break-blank = Büyük ara: boş satır
export-chip-major-break-none = Büyük ara: yok
export-chip-major-break-same = İki ara düzeyi aynı
export-chip-spacing-single = Aralık: tek
export-chip-spacing-onehalf = Aralık: 1½
export-chip-spacing-double = Aralık: çift
export-chip-notes-included = Notlar dahil
export-chip-notes-excluded = Notlar hariç
export-format-docx = Word
export-format-odt = LibreOffice
export-format-html = HTML
export-format-markdown = Markdown
export-format-djot = Djot
export-format-text = Metin
export-format-latex = LaTeX
export-format-epub = EPUB
export-format-pdf = PDF
export-overwrite-title = Var olan dosya değiştirilsin mi?
export-overwrite-text = “{ $name }” zaten var. Değiştirilsin mi?
export-progress-title = Dışa aktarılıyor…
export-cancelled = Dışa aktarma iptal edildi
export-done = { $count ->
    [one] { $count } öğe dışa aktarıldı
   *[other] { $count } öğe dışa aktarıldı
}
export-open-file = Aç
export-show-in-folder = Klasörde göster
export-error-title = Dışa aktarılamadı
export-error-details = Ayrıntılar

backup-progress-start = Başlatılıyor…
backup-progress-retention = Eski yedekler temizleniyor…
backup-progress-done = Bitti
backup-progress-destination = Konum { $i } / { $n }
backup-details = Ayrıntılar
backup-issues-title = Yedekleme sorunları
backup-failed-title = Yedekleme başarısız oldu
backup-complete-prune-warning = Yedekleme tamamlandı ({ $ok } kaydedildi, { $skipped } zaten güncel). Bazı eski yedekler kaldırılamadı

search = Ara
search-query-placeholder = Ara…
search-replace-placeholder = Şununla değiştir…
search-replace-toggle = Değiştirmeyi göster/gizle
search-replace-all = Tümünü değiştir
search-opt-case = Büyük/küçük harf duyarlı
search-opt-whole-word = Tam sözcük
search-opt-diacritics = Aksan duyarlı
search-scope-body = Gövde
search-scope-title = Başlık
search-scope-synopsis = Özet
search-scope-label = Açıklama
search-facet-book = Kitaplar
search-facet-part = Kısımlar
search-facet-chapter = Bölümler
search-facet-scene = Sahneler
search-facet-note = Notlar
search-facet-folder = Klasörler
search-tip-case = Büyük/küçük harf duyarlı: büyük ve küçük harfleri farklı sayar, böylece “Elena” ile “elena” ayrı eşleşmelerdir.
search-tip-whole-word = Tam sözcük: yalnızca tam sözcükleri eşleştirir, böylece “kar” sözcüğü “karpuz” içinde bulunmaz.
search-tip-diacritics = Aksan duyarlı: aksanlı ve şapkalı harfleri ayrı sayar, böylece “kar” ile “kâr” eşleşmez.
search-tip-body = Gövde: sahnelerin ve notların metninde arar.
search-tip-title = Başlık: dizin öğelerinin başlıklarında arar.
search-tip-synopsis = Özet: her yazı öğesinin özetinde arar.
search-tip-label = Açıklama: bir dizin öğesinin başlığının altında gösterilen kısa notta arar.
search-tip-comment = Yorumlar: yorum dizilerinin ve yanıtlarının metninde arar. Yorum el yazmasının bir parçası değil, onun hakkındadır; bu yüzden kendi anahtarı vardır ve değiştirme işlemi siz işaretleyene kadar yorum eşleşmelerini işaretsiz bırakır.
search-tip-book = Kitaplar: kitap kapsayıcısı ve başlangıç / bitiş işaretleri.
search-tip-part = Kısımlar: kısım düzeyindeki ayırıcılar.
search-tip-chapter = Bölümler: proje bölümleri hangi biçimde saklıyorsa.
search-tip-scene = Sahneler: metninizi barındıran öğeler.
search-tip-note = Notlar: serbest notlar.
search-tip-folder = Klasörler: sıradan düzenleme klasörleri ve ayırıcılar.
search-tip-paratext = Kitaba ait olan ama öyküsüne ait olmayan bir metin: önsöz, ithaf, sonsöz. El yazmasında asla sayılmaz.
search-tip-preserve-case = Büyük/küçük harfi koru: değiştirilen metin bulduğu harf biçimini alır, böylece “ELENA” “MARTA”, “Elena” da “Marta” olur.
search-tip-replace = Değiştir: değiştirme alanını ve Tümünü değiştir düğmesini gösterir.
search-error = Arama başarısız oldu: { $message }
search-no-matches = Eşleşme yok
search-count =
    { $items ->
        [one] { $items } belgede
       *[other] { $items } belgede
    } { $matches ->
        [one] { $matches } eşleşme
       *[other] { $matches } eşleşme
    }
search-count-truncated =
    { $items ->
        [one] { $items } belgede
       *[other] { $items } belgede
    } { $matches ->
        [one] { $matches } eşleşme
       *[other] { $matches } eşleşme
    } (yalnızca ilk sonuçlar)
search-occurrences = { $count }
search-field-body = Gövde
search-field-title = Başlık
search-field-synopsis = Özet
search-field-label = Açıklama
search-field-epigraph = Epigraf
search-field-comment = Yorum
search-field-comment-reply = Yanıt
search-field-footnote = Dipnot
search-include-in-replace = Tümünü değiştir'e dahil et
search-collapse-all = Tüm sonuçları daralt
search-replace-here = Bunu değiştir
search-dismiss = Sonuçlardan çıkar
search-undo-dismiss = Son çıkarılan sonucu geri getir
search-replace-nothing = (hiçbir şey)
search-replace-confirm-title = Tüm eşleşmeler değiştirilsin mi?
search-replace-confirm-text =
    { $items ->
        [one] { $items } belgede
       *[other] { $items } belgede
    } "{ $query }" ifadesinin { $occurrences ->
        [one] { $occurrences } geçişi
       *[other] { $occurrences } geçişi
    } "{ $replacement }" ile değiştirilsin mi? Bunu daha sonra bildirimden geri alabilirsiniz.
search-replace-done-title = Değiştirme tamamlandı
search-replace-done =
    { $items ->
        [one] { $items } belgede
       *[other] { $items } belgede
    } { $occurrences ->
        [one] { $occurrences } geçiş
       *[other] { $occurrences } geçiş
    } değiştirildi.
search-replace-done-skipped =
    { $items ->
        [one] { $items } belgede
       *[other] { $items } belgede
    } { $occurrences ->
        [one] { $occurrences } geçiş
       *[other] { $occurrences } geçiş
    } değiştirildi. { $skipped ->
        [one] { $skipped } alan
       *[other] { $skipped } alan
    } atlandı (aramadan sonra değişmiş).
search-replace-undo = Geri al
search-replace-failed-title = Değiştirme başarısız oldu
search-replace-skipped-title = Hiçbir şey değiştirilmedi
search-replace-skipped-body =
    { $fields ->
        [one] Bu metin aramadan sonra değiştiği için olduğu gibi bırakıldı.
       *[other] Bu metinlerden { $fields } tanesi aramadan sonra değiştiği için olduğu gibi bırakıldı.
    } Sözcüklerin şimdi nerede olduğunu görmek için yeniden arayın.
search-replace-undo-failed-title = Geri alma başarısız oldu
search-preview = Önizleme
search-preview-empty = Burada önizlemek için bir sonuç seçin
search-preview-no-prose = Bu eşleşmenin düzenlenebilir metni yok
search-preview-prompt = Burada önizleme görmek için bir arama yapın.
search-preview-open-search = Projede ara
search-preview-footnote-prompt = Bu eşleşme bir dipnotun kendi metninde. Görmek ve düzenlemek için Dipnotlar panelinde açın.
search-preview-open-footnotes = Dipnotları aç

find-placeholder = Bu belgede bul…
find-count = { $current } / { $total }
find-no-results = Sonuç yok
find-close = Bulmayı kapat
find-previous = Önceki eşleşme (Shift+Enter)
find-next = Sonraki eşleşme (Enter)
find-opt-case = Büyük/küçük harf duyarlı
find-opt-whole-word = Tam sözcük
find-replace-toggle = Değiştirmeyi göster/gizle (Ctrl+R)
find-replace-placeholder = Şununla değiştir…
find-replace = Değiştir
find-replace-all = Tümünü değiştir
find-preserve-case = Büyük/küçük harfi koru

dict-download-title = { $name } indiriliyor…
dict-download-done = { $name } kuruldu
dict-download-failed = { $name } indirilemedi: { $error }
dict-removed = { $name } kaldırıldı
dict-accept-first = { $name } indirilmeden önce lisansı kabul edin
settings-dict-tab-installed = Kurulu
settings-dict-tab-get-more = Daha fazlasını edin
settings-dict-tab-personal = Kişisel sözcükler
dict-installed-empty = Bu bilgisayarda henüz sözlük bulunamadı.
dict-get-more-search = Dillerde ara
dict-system-badge = sisteminizde
dict-unusable-badge = kullanılamaz
dict-download-button = İndir
dict-downloading = İndiriliyor…
dict-installed-label = Kurulu
dict-remove = Kaldır
dict-view-license = Lisansı görüntüle
dict-approx-size = ~{ $size }
dict-personal-empty = Bu projede henüz kişisel sözcük yok.
dict-personal-add = Ekle
dict-personal-placeholder = Bir sözcük ekleyin…
dict-missing-toast = { $count ->
    [one] Bu proje, kurulu olmayan { $count } sözlük kullanıyor
   *[other] Bu proje, kurulu olmayan { $count } sözlük kullanıyor
}
dict-missing-action = Sözlükleri edin
dict-license-title = { $name } lisansı
dict-license-accept = Kabul et ve indir
dict-license-cancel = İptal
dict-license-close = Kapat
dict-add-button = Sözlük ekle…
dict-add-title = Sözlük ekle
dict-add-close = Kapat
dict-add-name = Ad
dict-add-name-placeholder = ör. Latince sözlüğüm
dict-add-code = Dil kodu
dict-add-code-placeholder = ör. la ya da fr-FR-x-custom
dict-add-code-hint = Sizin seçtiğiniz kısa bir etiket. Bir belgenin dili olarak seçeceğiniz şey budur.
dict-add-aff = Ek dosyası (.aff)
dict-add-dic = Sözcük listesi (.dic)
dict-add-submit = Ekle
dict-add-cancel = İptal
dict-add-name-required = Sözlüğe bir ad verin.
dict-add-code-required = Bir dil kodu girin.
dict-add-code-invalid = Yalnızca harf, rakam ve - _ . kullanın.
dict-add-code-reserved = Bu, hazır bir sözlüğün kodu. Başka bir kod seçin.
dict-add-file-required = Bir dosya seçin.
dict-add-file-missing = Bu dosya yok.
dict-add-code-taken = Bu kod için zaten bir sözlük kurulu.
dict-add-done = { $name } eklendi
dict-add-unusable = Bu dosyalar kullanılabilir bir sözlük değil: { $error }
dict-add-failed = Sözlük eklenemedi: { $error }
inspector-export = Dışa aktarma
inspector-exportable = Dışa aktarmalara dahil et
ctx-number = Bu bölümü nu&marala
ctx-unnumber = Bu bölümü nu&maralama
inspector-numbering = Numaralandırma
inspector-numbered = Numaralı
inspector-numbered-tip = Bu bölüm kitabın numaralandırmasında yer alır. Önsöz, sonsöz ya da ara bölüm için kapatın.
inspector-numbered-tip-more = Numarasız bir bölüm kitapta olduğu gibi kalır: başlığına, metnine ve sözcük sayısına dokunulmaz. Yalnızca numara yazdırmaz ve bir numara harcamaz: önsözden sonraki bölüm ikinci değil birinci bölümdür. Onu dışa aktarmanın dışında bırakmak ise yukarıdaki ayrı bir anahtardır ve o, bölümü kitaptan tamamen çıkarır.
inspector-apply-to-children = Alt öğelere uygula
inspector-milestone = Ara hedef tarihi
inspector-milestone-none = Tarih yok
inspector-milestone-clear = Temizle
inspector-tags = Etiketler
inspector-aliases = Diğer adları
inspector-dict-language = Dil
inspector-apply-language-to-children = Dili alt öğelere uygula
settings-page-language = Dil
settings-field-dict-language = Diller
dict-tradeoff-hint = Her ek dil daha fazla sözcüğü kabul eder, böylece daha az hata yakalanır.
lang-inherit-hint = Devralındı: bu sahne kitabın ya da projenin dillerini kullanıyor.
lang-pill-list = Diller
lang-pill-add = Dil ekle
lang-pill-remove = Kaldır: { $name }
lang-pill-mute = Yazım denetimini kapat: { $name }
lang-pill-unmute = Yazım denetimini aç: { $name }

settings-page-personal-dictionary = Kişisel sözlük
settings-user-dict-desc = Buraya eklediğiniz sözcükler uygulamanın her yerinde doğru yazılmış sayılır ve bir daha işaretlenmez.
settings-user-dict-add-placeholder = Bir sözcük ekleyin…
settings-user-dict-add = Sözcük ekle
settings-user-dict-search = Sözcükleri süz
settings-user-dict-count = { $count ->
    [one] { $count } sözcük
   *[other] { $count } sözcük
}
settings-user-dict-rename = Yeniden adlandır
settings-user-dict-remove = Kaldır
settings-user-dict-empty = Henüz sözcük yok. Yazım denetiminin yok sayması gereken sözcükleri ekleyin ya da bir liste içe aktarın.
settings-user-dict-import = İçe aktar…
settings-user-dict-export = Dışa aktar…
settings-user-dict-import-tip = Her satıra bir sözcük olacak şekilde düz metin dosyasından sözcük ekler. Var olan sözcükler korunur.
settings-user-dict-export-tip = Listenin tamamını, her satıra bir sözcük olacak şekilde düz metin dosyasına kaydeder.
settings-user-dict-txt-filter = Metin dosyaları
settings-user-dict-duplicate = Zaten sözlükte
settings-user-dict-imported = { $count ->
    [one] { $count } sözcük içe aktarıldı
   *[other] { $count } sözcük içe aktarıldı
} ({ $duplicates } tanesi zaten vardı).
settings-user-dict-import-failed = Sözcük listesi okunamadı: { $error }
settings-user-dict-exported = { $count ->
    [one] { $count } sözcük kaydedildi.
   *[other] { $count } sözcük kaydedildi.
}
settings-user-dict-export-failed = Sözcük listesi kaydedilemedi: { $error }

settings-page-text-replacements = Metin değiştirme
settings-text-repl-desc = Kısaltmayı siz yazarken tam metinle değiştirin: bir boşluk ya da noktalama işareti yazdığınız anda "bkz" "bakınız" olur.
settings-text-repl-enable = Bu projede metin değiştirmeyi kullan
settings-text-repl-disabled-hint = Siz yazarken açılan kısaltmalar tanımlamak için bunu açın.
settings-text-repl-add = Kural ekle
settings-text-repl-trigger-placeholder = Kısaltma
settings-text-repl-replacement-placeholder = Neye dönüşeceği
settings-text-repl-added = "{ $trigger }" eklendi
settings-text-repl-duplicate = "{ $trigger }" için zaten bir kural var
settings-text-repl-filter = Kuralları süz
settings-text-repl-count = { $n ->
    [one] 1 kural
   *[other] { $n } kural
}
settings-text-repl-row-enabled = Bu kuralı kullan
settings-text-repl-delete = Kuralı sil: { $trigger }
settings-text-repl-deleted = "{ $trigger }" kuralı silindi
settings-text-repl-empty = Henüz kural yok.
settings-text-repl-csv-filter = CSV dosyaları
settings-text-repl-import = İçe aktar…
settings-text-repl-export = Dışa aktar…
settings-text-repl-imported = İçe aktarılan: { $added }, atlanan: { $skipped }
settings-text-repl-exported = { $n ->
    [one] 1 kural dışa aktarıldı
   *[other] { $n } kural dışa aktarıldı
}

editor-menu-no-suggestions = Öneri yok
editor-menu-add-to-dictionary = “{ $word }” sözcüğünü sözlüğe ekle
editor-menu-add-words-to-dictionary = Seçili sözcükleri sözlüğe ekle
editor-dict-added = “{ $word }” sözlüğünüze eklendi.
editor-dict-added-multi = { $count ->
    [one] Sözlüğünüze { $count } sözcük eklendi.
   *[other] Sözlüğünüze { $count } sözcük eklendi.
}
toast-undo = Geri al

undo-superseded-title = Bu adım artık sonuncu değil
undo-superseded-body = O zamandan beri bu projede başka bir şey değişti. Geri alma bunun yerine onu geri alacağı için işlem yapılmadı. Bu adımın yaptığı hiçbir şey geri çevrilmedi. Geçmişte kendiniz geri gitmek için Düzen ▸ Geri al'ı kullanın.
undo-failed = Geri alma başarısız oldu: { $error }

menu-edit = &Düzen
menu-edit-undo = &Geri al
menu-edit-redo = Yin&ele
menu-edit-undo-target = &Geri al: { $target }
menu-edit-redo-target = Yin&ele: { $target }
shortcut-name-edit-undo = Geri al
shortcut-name-edit-redo = Yinele
undo-target-typing = yazılan metin
undo-target-project = bu projedeki son değişiklik
prose-history-reset-title = Yazma geçmişi sıfırlandı
prose-history-reset-body = { $count ->
    [one] Açık bir sahne daha önceki bir durumdan geri yüklendi, bu yüzden yazma geçmişi artık geçerli değil.
   *[other] Açık { $count } sahne daha önceki bir durumdan geri yüklendi, bu yüzden yazma geçmişleri artık geçerli değil.
}
undo-target-trash = çöp kutusuna taşıma
undo-target-restore = çöp kutusundan geri yükleme
undo-target-delete-forever = kalıcı silme
undo-target-replace-all = proje genelinde değiştirme
undo-target-import = belge içe aktarma
undo-target-duplicate = çoğaltma
undo-target-move = taşıma
undo-target-merge = iki sahneyi birleştirme
undo-target-split = sahne bölme
undo-target-promote = tür değiştirme
undo-target-tidy-titles = bölüm başlıklarını düzenleme
undo-target-import-tags = etiketleri içe aktarma
undo-target-import-templates = not şablonlarını içe aktarma
undo-target-create = oluşturma
undo-target-remove = silme
undo-target-rename = yeniden adlandırma
undo-target-edit = o düzenleme
menu-edit-find = &Bul…
menu-edit-find-next = So&nrakini bul
menu-edit-find-prev = Öncek&ini bul
menu-edit-replace = Bul ve &değiştir…
menu-edit-find-in-project = &Projede bul…
menu-edit-replace-in-project = Projede değişti&r…
undo-frozen = Geri al (“Hep ileri” açık)
redo-frozen = Yinele (“Hep ileri” açık)
menu-edit-cut = &Kes
menu-edit-copy = K&opyala
menu-edit-paste = &Yapıştır
menu-edit-paste-plain = Biçim&siz yapıştır
menu-edit-select-all = &Tümünü seç
shortcut-name-edit-cut = Kes
shortcut-name-edit-copy = Kopyala
shortcut-name-edit-paste = Yapıştır
shortcut-name-edit-paste-plain = Biçimsiz yapıştır
shortcut-name-edit-select-all = Tümünü seç

titlebar-spellcheck-on = Yazım denetimi açık. Denetimi durdurmak için tıklayın (F7)
titlebar-spellcheck-off = Yazım denetimi kapalı. Yeniden denetlemek için tıklayın (F7)
menu-spellcheck = &Yazımı denetle
menu-comments = Yoru&mlar
settings-page-spellcheck = Yazım denetimi
settings-group-spellcheck = Yazım denetimi
settings-spellcheck-enabled = Yazarken yazımı denetle
settings-spellcheck-hint = Kurulu hiçbir sözlüğün tanımadığı sözcüklerin altını çizer. Bunu kapatmak, siz yeniden açana kadar her projedeki tüm denetimi durdurur. Yalnızca bir dilin denetimini durdurmak için projenin ya da bir öğenin Dil alanında o dilin onay işaretini kaldırın.

menu-trash = &Çöp kutusu
trash-title = Çöp kutusu
trash-empty-state = Çöp kutusu boş.
trash-empty-button = Çöp kutusunu boşalt…
trash-restore = &Geri yükle
trash-restore-to = &Şuraya geri yükle…
trash-delete-forever = &Kalıcı olarak sil
trash-restored-ok = { $count } öğe geri yüklendi.
trash-restore-error = Geri yüklenemedi: { $error }
trash-restore-orphaned = Bu öğenin özgün yeri artık yok. Nereye geri yükleneceğini seçin.
trash-restore-no-project = Açık proje olmadığından geri yüklenecek bir şey yok.
trash-delete-no-project = Açık proje olmadığından silinecek bir şey yok.
trash-empty-confirm-title = Çöp kutusu boşaltılsın mı?
trash-empty-confirm-text = { $count ->
    [one] Çöp kutusundaki { $count } kayıt kalıcı olarak silinecek.
   *[other] Çöp kutusundaki { $count } kayıt kalıcı olarak silinecek.
} Bunlar çöp kutusundan kesin olarak çıkar. Siz başka bir şey yapana kadar Geri al yine de onları geri getirebilir.
trash-emptied-title = Çöp kutusu boşaltıldı
trash-emptied-body = Çöp kutusundaki her şey kalıcı olarak silindi.
trash-empty-no-project = Açık proje olmadığından boşaltılacak çöp kutusu yok.
trash-delete-forever-confirm-title = Kalıcı olarak silinsin mi?
trash-delete-forever-confirm-text = { $count ->
    [one] { $count } öğe kalıcı olarak silinecek.
   *[other] { $count } öğe kalıcı olarak silinecek.
} Bunlar çöp kutusundan kesin olarak çıkar. Siz başka bir şey yapana kadar Geri al yine de onları geri getirebilir.
trash-deleted-title = Kalıcı olarak silindi
trash-deleted-body = { $count } öğe kalıcı olarak silindi.
trash-undo = Geri al
trash-restore-picker-title = Şuraya geri yükle…
trash-restore-to-confirm-title = Buraya geri yüklensin mi?
trash-restore-to-confirm-text = "{ $item }", "{ $destination }" içine geri yüklensin mi?
trash-restore-picker-restore-here = Buraya geri yükle
trash-restore-picker-cancel = İptal
trash-restore-picker-empty = Henüz dizin yok. Önce bir tane oluşturun.
trash-banner-title = Bu öğe çöp kutusunda
trash-banner-description = Siz geri yükleyene kadar ana hatta ve dışa aktarmalarda görünmez.
trash-banner-restore = Geri yükle…
trash-tab-tooltip = Çöp kutusunda

settings-styles-sheet-yes = Evet
settings-styles-sheet-no = Hayır
settings-styles-sheet-font = Yazı tipi
settings-styles-sheet-indent = İlk satır girintisi
settings-styles-sheet-para-spacing = Paragraf aralığı
settings-styles-sheet-page = Sayfa boyutu
settings-styles-sheet-margins = Kenar boşlukları (Ü/S/A/S)
settings-styles-sheet-title-page = Kitap başlık sayfası
settings-styles-sheet-heading-language = Başlık dili
settings-styles-sheet-auto = Metni izler
settings-styles-sheet-digits = Rakamlar
settings-styles-sheet-direction = Yazı yönü
settings-styles-sheet-formats = Biçimler
settings-styles-sheet-all-formats = Tümü

settings-styles-editor-group = Biçem düzenleyici
settings-styles-editor-missing = Bu biçem artık kullanılamıyor.

settings-styles-page-letter = Letter
settings-styles-digits-western = Batı (0–9)
settings-styles-digits-eastern-arabic = Doğu Arap (٠–٩)
settings-styles-direction-ltr = Soldan sağa
settings-styles-direction-rtl = Sağdan sola

format-dock-title = Biçim
format-panel-empty = Biçimlendirme seçeneklerini görmek için imleci bir sahneye, nota ya da özete yerleştirin.
format-group-marks = Metin
format-group-block = Paragraf
format-group-lists = Listeler
format-group-tables = Tablo
format-group-breaks = Sahne araları
format-superscript = Üst simge
format-subscript = Alt simge
format-link = Bağlantı…
link-dialog-insert-title = Bağlantı ekle
link-dialog-edit-title = Bağlantıyı düzenle
link-dialog-text-label = Metin
link-dialog-text-placeholder = okurun gördüğü sözcükler
link-dialog-url-label = Bağlantı adresi
link-dialog-url-placeholder = ornek.com
link-dialog-insert = Ekle
link-dialog-apply = Uygula
link-dialog-cancel = İptal
link-dialog-remove = Bağlantıyı kaldır
link-scheme-refused = Yalnızca web ve e-posta bağlantıları açılabilir. { $url } olduğu gibi bırakıldı.
menu-format-link = Bağla&ntı…
format-clear = Biçimlendirmeyi temizle
format-heading = Başlık düzeyi
format-heading-normal = Normal metin
format-heading-1 = Başlık 1
format-heading-2 = Başlık 2
format-heading-3 = Başlık 3
format-heading-4 = Başlık 4
format-heading-5 = Başlık 5
format-heading-6 = Başlık 6
format-align-left = Sola hizala
format-align-center = Ortala
format-direction-rtl = Sağdan sola paragraf
format-blockquote = Blok alıntı
format-list-bullet = Madde işaretli liste
format-list-numbered = Numaralı liste
format-indent = Girintiyi artır
format-outdent = Girintiyi azalt
format-table-insert = Tablo ekle
format-table-row-above = Üste satır ekle
format-table-row-below = Alta satır ekle
format-table-col-before = Sola sütun ekle
format-table-col-after = Sağa sütun ekle
format-table-row-delete = Satırı sil
format-table-col-delete = Sütunu sil
format-table-remove = Tabloyu kaldır

menu-format-marks-bold = Ka&lın
menu-format-marks-italic = İtali&k
menu-format-marks-underline = &Altı çizili
menu-format-marks-strike = &Üstü çizili
menu-format-marks-superscript = Üst si&mge
menu-format-marks-subscript = Alt sim&ge
menu-format-marks-clear = Biçimlendirmeyi t&emizle
menu-format-heading = &Başlık
menu-format-heading-normal = &Normal metin
menu-format-heading-1 = Başlık &1
menu-format-heading-2 = Başlık &2
menu-format-heading-3 = Başlık &3
menu-format-heading-4 = Başlık &4
menu-format-heading-5 = Başlık &5
menu-format-heading-6 = Başlık &6
menu-format-alignment = &Hizalama
menu-format-align-left = &Sola hizala
menu-format-align-center = &Ortala
menu-format-direction = Yazı y&önü
menu-format-direction-auto = &Otomatik
menu-format-direction-ltr = &Soldan sağa
menu-format-direction-rtl = Sağ&dan sola
menu-format-blockquote = Bl&ok alıntı
menu-format-lists = Li&steler
menu-format-list-bullet = &Madde işaretli liste
menu-format-list-numbered = &Numaralı liste
menu-format-indent = &Girintiyi artır
menu-format-outdent = Girintiyi a&zalt
menu-format-table = &Tablo
menu-format-table-insert = Tablo &ekle
menu-format-table-2x2 = &2 x 2
menu-format-table-3x3 = &3 x 3
menu-format-table-4x4 = &4 x 4
menu-format-table-row-above = Üste &satır ekle
menu-format-table-row-below = &Alta satır ekle
menu-format-table-col-before = Sola sü&tun ekle
menu-format-table-col-after = Sağa s&ütun ekle
menu-format-table-row-delete = Satırı si&l
menu-format-table-col-delete = Sütun&u sil
menu-format-table-remove = Tabloyu &kaldır

menu-format-typo-marks = Biçimlendirme &işareti
menu-format-typo-nbsp = &Bölünmez boşluk
menu-format-typo-nbhyphen = Bölünmez &tire
menu-format-typo-soft-hyphen = &Yumuşak tire
menu-format-typo-nnbsp = &Dar bölünmez boşluk
menu-format-typo-zwsp = &Sıfır genişlikli boşluk
menu-format-typo-word-joiner = Sö&zcük birleştirici

menu-about = Skribisto &hakkında…
about-title = Skribisto hakkında
about-version = Sürüm { $version }
about-update-available = Sürüm { $version } kullanıma sunuldu
about-update-see-notes = Nelerin değiştiğine bakın
about-update-flathub = Bu kopyayı Flathub güncel tutar.
about-update-distro = Bu kopyayı dağıtımınız güncel tutar.
about-tagline = Uzun soluklu kurmaca için, Rust ile Teksilo araç takımı kullanılarak yazılmış bir roman yazma uygulaması.
about-license = GNU Genel Kamu Lisansı, sürüm 3 altında yayımlanmıştır.
about-copyright = © 2026 Cyril Jacquet
about-close = Kapat

native-menu-about = { $app } hakkında
native-menu-hide = { $app } uygulamasını gizle
native-menu-quit = { $app } uygulamasından çık
native-menu-settings = Ayarlar…
native-menu-window = Pencere
native-menu-minimize = Simge durumuna küçült
native-menu-zoom = Yakınlaştır

window-title = { $title } - { $app }
window-title-numbered = { $title } - { $app } (Pencere { $n })
window-title-empty = { $app }

first-run-window-title = Hoş geldiniz
first-run-title = { $app } kurulumu
first-run-body = { $app } ayarlarını Skribisto'dan ayrı tutar, bu yüzden boş başlar. Tercihleriniz, son projeleriniz, sözlükleriniz ve pencere yerleşiminiz şimdi kopyalanabilir.
first-run-from = Şuradan kopyala
first-run-to = Şuraya kopyala
first-run-copy-note = Hiçbir şey taşınmaz ya da kaldırılmaz. Skribisto kendi ayarlarının hepsini korur ve tam olarak eskisi gibi çalışmayı sürdürür.
first-run-import = Ayarları içe aktar
first-run-start-fresh = Sıfırdan başla
first-run-import-failed = Bazı ayarlar içe aktarılamadı: { $error }

settings-page-punctuation = Noktalama varsayılanları
settings-group-punctuation = Akıllı noktalama
settings-page-work-punctuation = Noktalama
settings-punctuation-override = Bu projeye kendi noktalama kurallarını ver
settings-punctuation-override-hint = Kapalı: proje uygulama tercihini izler. Bu kurallar .skrib dosyasının içinde taşınır, böylece dosyayı açan bir ortak yazar aynı tipografiyle yazar.
settings-punctuation-dashes = -- dizisini kısa çizgiye, --- dizisini uzun çizgiye dönüştür
settings-punctuation-ellipsis = ... dizisini üç nokta karakterine dönüştür
settings-punctuation-quotes = Tırnak işaretlerini ve kesme işaretlerini kıvrık yap
settings-quote-style = Tırnak işaretleri
settings-quote-style-locale = Dilin varsayılanı
settings-quote-style-curly = “Çift”
settings-quote-style-curly-single = ‘Tek’
settings-quote-style-guillemets = «Açılı tırnak»
settings-quote-style-low-high = „Alt-üst“
settings-punctuation-spacing = ; : ! ? öncesinde boşluk
settings-punctuation-spacing-hint = Fransız tipografisi ; ! ? öncesine dar bir bölünmez boşluk, : öncesine tam bir bölünmez boşluk koyar. Yalnızca Fransızca yazılmış metne uygulanır. Açılı tırnakların « » içindeki boşluk açılı tırnak biçiminin kendisiyle gelir.
settings-punctuation-sample = Dilinizdeki sonuç
settings-punctuation-app-hint = Proje ▸ Noktalama bölümünde kendi kurallarını belirlemedikçe her projenin yaptığı.
settings-punctuation-dialogue = "- " ile başlayan paragrafı konuşma çizgisiyle aç
settings-punctuation-dialogue-hint = Konuşmayı tırnak yerine çizgiyle gösteren diller için: Fransızca, İspanyolca, Rusça ve diğerleri. Yalnızca paragrafın en başında devreye girer.

statusbar-go-to = Git…
go-to-placeholder = Dizinde ara
go-to-no-matches = Bu aramayla eşleşen öğe yok.
menu-go-to = &Git…

settings-page-distraction-free-themes = Dikkat dağıtmayan mod temaları
settings-themes-builtin = Hazır temalar
settings-themes-builtin-badge = Hazır
settings-themes-user = Temalarım
settings-themes-editor-group = Temayı düzenle
settings-themes-editor-empty = Düzenlemek için "Temalarım" altından bir tema seçin.
settings-themes-use = Kullan
settings-themes-duplicate = Çoğalt
settings-themes-edit = Düzenle
settings-themes-delete = Sil
settings-themes-export = Dışa aktar…
settings-themes-import = İçe aktar…
settings-themes-copy-suffix = kopya
settings-themes-json-filter = Tema (JSON)
settings-themes-imported = Tema içe aktarıldı
settings-themes-import-failed = Bu tema içe aktarılamadı
settings-themes-exported = Tema dışa aktarıldı
settings-themes-export-failed = Bu tema dışa aktarılamadı
settings-themes-low-contrast = düşük karşıtlık
settings-themes-low-contrast-band = vurgu metni gizliyor
settings-themes-field-name = Ad
settings-themes-field-paper = Sayfa
settings-themes-field-ink = Metin
settings-themes-field-general = Arka plan
settings-themes-field-widget-text = Denetim şeridi metni
settings-themes-field-caret-band = İmlecin çevresindeki vurgu

statusbar-focus-settings = Dikkat dağıtmayan mod ayarları
statusbar-focus-manage-themes = Temaları yönet…

comments-title = Yorumlar
comments-document-title = Bu belge
comments-empty-project = Bu projede henüz yorum yok.
comments-empty-document = Bu belgede yorum yok.
comments-filter-all = Tümü
comments-filter-open = Açık
comments-filter-resolved = Çözüldü
comments-filter-orphaned = Sahipsiz
comments-status-open = Açık
comments-status-resolved = Çözüldü
comments-status-orphaned = Metnini yitirdi
comments-orphan-snippet = (yorum yapılan metin artık yok)
comments-status-unplaced = Metin konumu yok
comments-unplaced-snippet = (hiçbir metne bağlı değil)
comments-reply-count = { $count ->
    [one] { $count } yanıt
   *[other] { $count } yanıt
}
comments-menu-resolve = Çözüldü olarak işaretle
comments-menu-reopen = Yeniden aç
comments-menu-delete = Sil
comments-sort-document = Belgedeki sıraya göre
comments-sort-newest = Önce en yeniler
comments-sort-reader = Okura göre gruplanmış
comments-unsigned-author = İmzasız
comments-agreement = { $count ->
    [one] { $count } okur
   *[other] { $count } okur
}
comments-menu-add = Yorum ekle
comments-menu-add-paragraph = Bu paragrafa yorum yap
overview-col-comments = Yorumlar
overview-col-total-comments = Toplam yorum
comments-card-placeholder = Bir yorum yazın…
comments-card-unknown-author = Bilinmeyen yazar
comments-unsigned-toast = Yorumlarınız imzasız: bu bilgisayarda sizin için bir ad belirlenmemiş.
comments-unsigned-action = Adınızı belirleyin
comments-card-reply = Yanıtla
comments-card-reply-placeholder = Yanıt…
comments-card-actions = Yorum eylemleri
comments-reply-actions = Yanıt eylemleri
comments-menu-delete-reply = Yanıtı sil
comments-menu-delete-all = Buradaki tüm yorumları sil
comments-deleted-toast = Yorum silindi
comments-reply-deleted-toast = Yanıt silindi
comments-deleted-all-toast = { $count ->
    [one] { $count } yorum silindi
   *[other] { $count } yorum silindi
}
comments-undo = Geri al

analysis-segment = Çözümleme
analysis-scope-book = Bu kitap çözümleniyor
analysis-run = Çözümlemeyi çalıştır
analysis-stale = Çalıştırıldıktan sonra değişti
analysis-not-run = Henüz çözümlenmedi.
analysis-running = El yazması okunuyor…
analysis-failed = Çözümleme tamamlanamadı.
analysis-no-scenes = Bu kitapta henüz sahne yok.

analysis-shape = Şekil

analysis-arrivals = Gelişler
analysis-arrivals-explainer = Proje açıkken metnin bu projeye nasıl ulaştığı. Karakterlerin hangi yoldan geldiğini söyler, onları kimin yazdığı hakkında ise hiçbir şey söylemez: başka yerde taslak yazıp yapıştıran bir yazar yapıştırmıştır, dikte eden de dikte etmiştir. Burada ulaşılacak bir sayı yok.
analysis-arrivals-scope = Yalnızca bu kitap değil, tüm proje.
analysis-arrivals-session = Projeyi açtığınızdan beri. Projeyi kapatmak sayımı yeniden başlatır.
analysis-arrivals-nothing = Bu oturumda henüz metin gelmedi.
analysis-arrivals-typed = Yazılan
analysis-arrivals-pasted = Yapıştırılan
analysis-arrivals-dictated = Dikte edilen
analysis-arrivals-imported = İçe aktarılan
analysis-arrivals-programmatic = Sizin için eklenen
analysis-arrivals-count = { $count ->
    [one] { $count } karakter
   *[other] { $count } karakter
}
analysis-arrivals-none = yok

analysis-words-per-scene = Sahne başına sözcük
analysis-median-words = Bu kitabın ortanca sahnesi { $count ->
    [one] { $count } sözcük.
   *[other] { $count } sözcük.
}
analysis-median-line = Ortanca: { $count ->
    [one] { $count } sözcük
   *[other] { $count } sözcük
}
analysis-dialogue = Diyalog
analysis-dialogue-unsupported = Bu dil için diyalog henüz ölçülmüyor.

analysis-footnote-words = Dipnot sözcükleri
analysis-footnote-words-count = { $count ->
    [one] Bu kitabın dipnotlarında { $count } sözcük var; el yazması toplamından ayrı tutulur.
   *[other] Bu kitabın dipnotlarında { $count } sözcük var; el yazması toplamından ayrı tutulur.
}
analysis-footnote-words-pending = Dipnotlar sayılıyor…

analysis-ignore-empty = Henüz hiçbir şey yazılmamış metinleri yok say
analysis-empty-hidden = { $count } boş { $count ->
        [one] metin
       *[other] metin
    } gizlendi.
analysis-all-texts-empty = Bu kitaptaki tüm metinler hâlâ boş.

binder-filter-count = { $total } öğeden { $shown } tanesi gösteriliyor
binder-filter-clear = Temizle
binder-filter-none = “{ $query }” ile eşleşen bir şey yok.

settings-paratext-intro = Yeni bir projenin başlayabileceği ön ve arka kısımlar. Her yapı bir yayıncılık geleneğine aittir ve sayfa başlıkları o geleneğin kendi dilinde yazılmıştır. Proje oluşturulduktan sonra dilediğiniz gibi yeniden adlandırın.
settings-paratext-structures = Yapılar
settings-paratext-broken = Okunamadı
settings-paratext-edit = Düzenle
settings-paratext-duplicate = Çoğalt
settings-paratext-delete = Sil
settings-paratext-new = Yeni yapı
settings-paratext-save = Kaydet
settings-paratext-editor-hint = Bir ad, el yazmasından önce oluşturulacak sayfalar ve ondan sonra oluşturulacak sayfalar. Nereye yerleşecekleri size kalmış: proje oluşturulduktan sonra bunlar sıradan öğelerdir.

image-insert = &Resim ekle…
image-no-project = Resim eklemeden önce bir proje açın.
image-choose-title = Bir resim seçin
image-filter-label = Resimler
image-large-title = Bu büyük bir resim
image-large-text =
    { $name } { $megapixels } megapiksel ({ $width }×{ $height }).
    Olduğu gibi tutmak özgün dosyanızı projede saklar: her yedekle ve her
    dışa aktarmayla birlikte taşınır. İyileştirmek ise bunun yerine en uzun
    kenarı en fazla 2560 piksel olan daha küçük bir kopya saklar.
image-large-keep = Özgün hâlini koru
image-large-downscale = İyileştir
image-large-remember = Bundan sonra hep böyle yap, bir daha sorma
image-not-recorded = Resim kaydedildi ama projeye işlenemedi.
image-describe-title = Resmi tanımla
image-describe-explain = Göremeyen bir okur için resmin ne gösterdiği. El yazmasının bir parçası değildir: asla sayılmaz, aranmaz ya da metin olarak dışa aktarılmaz.
image-describe-placeholder = gri bir gökyüzü önünde bir deniz feneri
image-resize-title = Resmi yeniden boyutlandır
image-resize-explain = Şu anda gösterildiği boyutun yüzdesi. 100 olduğu gibi bırakır.
image-resize-invalid = 1 ile 1000 arasında bir sayı girin.
image-menu-describe = Resmi &tanımla…
image-menu-resize = Resmi yeniden &boyutlandır…
image-menu-reset-size = Özgün b&oyut

cover-choose = Kapak res&mi…
cover-clear = Kapağı ka&ldır
cover-choose-title = Bir kapak seçin
cover-set = Kapak ayarlandı.
cover-cleared = Kapak kaldırıldı. Resim hâlâ projede.
card-label = Açıklama
card-exportable = Dışa aktarılır
card-numbered = Numaralı
card-created = Oluşturulma
card-modified = Değiştirilme
card-yes = evet
card-no = hayır
card-type = Tür
card-position = Konum
card-children = Alt öğeler
card-goal = Hedef
card-words = Sözcük
card-synopsis = Özet
card-point-of-view = Bakış açısı
card-aliases = Diğer adları

export-orphan-footnotes-title = Göndermesi olmayan dipnotlar
export-comments-dropped =
    { $count ->
        [one] Bir yorum dışa aktarılan metne yerleştirilemediği için dışarıda bırakıldı.
       *[other] { $count } yorum dışa aktarılan metne yerleştirilemediği için dışarıda bırakıldı.
    }
export-orphan-footnotes =
    { $count ->
        [one] Bir dipnota artık el yazmasının hiçbir yerinden gönderme yapılmıyor. Metni dışa aktarılan kitapta görünmeyecek.
       *[other] { $count } dipnota artık el yazmasının hiçbir yerinden gönderme yapılmıyor. Metinleri dışa aktarılan kitapta görünmeyecek.
    }

menu-footnotes = Di&pnotlar
footnotes-title = Dipnotlar
footnotes-insert = Di&pnot ekle
footnotes-empty = Henüz dipnot yok. İmleci bir sahneye koyun, sonra yukarıdaki + düğmesini ya da Ctrl+Alt+F kısayolunu kullanın.
footnotes-filter-all = Tümü
footnotes-filter-document = Bu belge
footnotes-filter-orphaned = Sahipsiz
footnotes-orphaned = Artık hiçbir şey bu nota gönderme yapmıyor
footnotes-untitled-home = Adsız
footnotes-body-placeholder = notun kendisi
footnotes-insert-tooltip = İmlecin bulunduğu yere dipnot ekle (Ctrl+Alt+F)
footnotes-actions = Dipnot eylemleri
footnotes-delete = Notu ve göndermesini &sil
footnotes-no-project = Dipnot eklemeden önce bir proje açın.
footnotes-no-caret = Oraya dipnot eklemek için imleci bir sahnenin metnine koyun.
footnotes-not-created = Dipnot projeye eklenemedi.
footnotes-deleted-toast = Not silindi
footnotes-undo-delete = Geri al

import-document-title = Belgeleri içe aktar
import-document-close = Kapat
import-document-step-files = Dosyalar
import-document-step-review = Gözden geçir
import-document-step-destination = Varış yeri
import-document-step-reconcile = Birleştir
import-document-reconcile-hint = Bu öğelerin bazıları zaten sizde olanlar. Her biri için ne olacağını belirtin.
import-document-reconcile-all-new = Bu dosyadaki hiçbir şey projenizle eşleşmiyor: her öğe yeni olarak eklenecek.
import-document-duplicate-returns = Bu dosyalar, aynı el yazmasının geri dönen birden fazla kopyası. Bunları birer birer içe aktarın: eşleştirme projenizle yalnızca bir geri dönen kopyayı hizalar, bu yüzden birkaçını birlikte içe aktarmak kitabı birleştirmek yerine ikinci bir kopyasını ekler.
import-document-hunk-added = Bu yeni paragrafı al
import-document-hunk-removed = Bu paragrafı çıkar
import-document-hunk-changed = Bu yeniden yazımı al
import-document-col-stray-prose = Metni
import-document-stray-as-paratext = Yan metin olarak tut
import-document-stray-discard = Metni at
import-document-col-current = Projenizde
import-document-col-incoming = Bu dosyada
import-document-col-status = Durum
import-document-col-action = Ne yapılsın
import-document-status-identical = Aynı
import-document-status-editor-edited = Onlar düzenledi
import-document-status-you-edited = Siz düzenlediniz
import-document-status-conflict = İkiniz de düzenlediniz
import-document-status-different = Farklı
import-document-status-new = Yeni
import-document-status-missing = Bu dosyada yok
import-document-status-moved = Taşınmış
import-document-action-comments-only = Yalnızca yorumlar
import-document-action-take-import = Bu sürümü al
import-document-action-keep-current = Benimkini koru
import-document-action-create-new = Yeni olarak ekle
import-document-action-ignore = Atla
import-document-compare = Karşılaştır
import-document-compare-legend = Sizin sürümünüz bu dosyadakiyle karşılaştırılıyor. Salt okunur.
import-document-compare-close = Kapat
import-document-drop-title = Belgeleri buraya bırakın
import-document-drop-hint = Markdown (.md), Word (.docx), OpenDocument (.odt) ve düz metin (.txt)
import-document-browse = Gözat…
import-document-move-up = Yukarı taşı
import-document-move-down = Aşağı taşı
import-document-remove-file = Kaldır
import-document-file-count = { $count ->
    [one] 1 dosya
   *[other] { $count } dosya
}
import-document-no-files = Henüz dosya seçilmedi.
import-document-col-included = İçe aktar
import-document-col-title = Başlık
import-document-col-type = Tür
import-document-col-words = Sözcük
import-document-col-breaks = Aralar
import-document-col-comments = Yorumlar
import-document-col-footnotes = Dipnotlar
import-document-col-epigraph = Epigraf
import-document-col-source = Kaynak
import-document-level-rules = Başlık düzeyleri
import-document-level-n = Başlık { $level }
import-document-add-top-level = En üst düzey ekle
import-document-add-top-level-tooltip = Kitap başlığı olmayan bölüm dosyaları için, çözümlenen her öğenin üstüne bir Kitap ekler
import-document-destination = Varış yeri
import-document-destination-hint = Bir dizin ya da öğe seçin: yeni öğeler bir klasörün içine ya da bir sahnenin sonrasına yerleşir.
import-document-destination-empty = Henüz dizin yok. Önce bir tane oluşturun.
import-document-plan-empty = Henüz içe aktarılacak bir şey yok.
import-document-summary = { $rows ->
    [one] 1 öğe
   *[other] { $rows } öğe
} · { $breaks ->
    [one] 1 sahne arası
   *[other] { $breaks } sahne arası
}
import-document-back = Geri
import-document-cancel = İptal
import-document-analyse = İleri
import-document-analysing = Belgeler okunuyor…
import-document-step-analysing = Okunuyor
import-document-cancel-analysis = Okumayı durdur
import-document-analyse-failed = Belgeler okunamadı.
import-document-details = Ayrıntılar
import-document-import = İçe aktar
import-document-done = { $count ->
    [one] 1 öğe içe aktarıldı
   *[other] { $count } öğe içe aktarıldı
}
import-document-undo = Geri al
import-diagnostic-file-unreadable = “{ $path }” okunamadı: { $detail }. Diğer dosyalar yine de içe aktarılır.
import-diagnostic-nested-too-deep = “{ $path }” okunmadı: içeriği { $limit } düzeyden daha derin iç içe geçmiş. Hiçbir gerçek belge buna yaklaşmaz ve onu okumak Skribisto'yu çökertebilir, bu yüzden dışarıda bırakıldı. Diğer dosyalar yine de içe aktarılır.
import-diagnostic-archive-too-large = “{ $path }” okunmadı: kendisinden çok daha büyük açılacak biçimde hazırlanmış sıkıştırılmış bir dosya; belleği tüketip Skribisto'yu çökertebileceği için dışarıda bırakıldı. Diğer dosyalar yine de içe aktarılır.
import-diagnostic-lossy-decode = { $count ->
    [one] “{ $path }” içindeki bir karakter çözülemedi. Korumak için dosyayı UTF-8 olarak yeniden kaydedin.
   *[other] “{ $path }” içindeki { $count } karakter çözülemedi. Korumak için dosyayı UTF-8 olarak yeniden kaydedin.
}
import-diagnostic-decoded-from-bom = “{ $path }” UTF-8 olarak değil, { $detail } olarak çözüldü.
import-diagnostic-empty-file = “{ $path }” boş.
import-diagnostic-no-headings = “{ $path }” içinde başlık yok, bu yüzden tek bir öğe olarak gelir.
import-diagnostic-unsupported-format = “.{ $detail }” dosyalarını okuyabilen bir şey yok, bu yüzden “{ $path }” atlandı.
import-diagnostic-front-matter-not-flat = “{ $path }” içindeki ön bilgi: “{ $detail }” basit bir değer değil ve atlandı.
import-diagnostic-footnotes-degraded = { $count ->
    [one] “{ $path }” içindeki bir dipnot düz metin olarak gelir: Markdown'dan dipnot okunmaz.
   *[other] “{ $path }” içindeki { $count } dipnot düz metin olarak gelir: Markdown'dan dipnot okunmaz.
}
import-diagnostic-footnote-not-carried = { $count ->
    [one] “{ $path }” içindeki bir dipnot aktarılamadı. Diğerleri aktarıldı.
   *[other] “{ $path }” içindeki { $count } dipnot aktarılamadı. Diğerleri aktarıldı.
}
import-diagnostic-raw-html-dropped = { $count ->
    [one] “{ $path }” içindeki bir ham HTML bloğu çıkarıldı.
   *[other] “{ $path }” içindeki { $count } ham HTML bloğu çıkarıldı.
}
import-diagnostic-nested-break-dropped = { $count ->
    [one] “{ $path }” içinde bir alıntının ya da listenin içindeki bir sahne arası çıkarıldı. Yalnızca kendi satırındaki bir ara korunur.
   *[other] “{ $path }” içinde alıntıların ya da listelerin içindeki { $count } sahne arası çıkarıldı. Yalnızca kendi satırındaki bir ara korunur.
}
import-diagnostic-image-not-ingested = “{ $path }”, “{ $detail }” resmine gönderme yapıyor. Gönderme metin olarak gelir; resmin kendisi kopyalanmaz.
import-diagnostic-duplicate-title = “{ $title }” { $count } kez geçiyor. Bu dosyaları daha önce içe aktardıysanız bu işlem onları çoğaltacak.
import-diagnostic-heading-level-jump = “{ $title }” { $from }. başlık düzeyinden { $to }. düzeye atlıyor; üst öğesinin bir düzey altına yerleştirildi.
import-diagnostic-illegal-combination = “{ $title }” metin içeriyor, ama { $kind } türündeki bir öğe metin barındıramaz. Türünü değiştirene ya da onu dışarıda bırakana kadar içe aktarma bekletilir. Aksi hâlde hiçbir şey içe aktarılmaz.
import-diagnostic-tracked-changes-flattened = { $path } düzeltme sürecinin ortasındaydı: { $count ->
    [one] { $count } izlenen değişiklik kabul edildi
   *[other] { $count } izlenen değişiklik kabul edildi
} ve silmeler çıkarıldı. Son metin budur. Bunun yerine kendi ifadelerinizden herhangi birini korumak için son adımdaki Karşılaştır'ı kullanın.
import-diagnostic-tracked-changes-flattened-by = { $path } düzeltme sürecinin ortasındaydı: { $count ->
    [one] { $names } tarafından yapılan { $count } izlenen değişiklik kabul edildi
   *[other] { $names } tarafından yapılan { $count } izlenen değişiklik kabul edildi
} ve silmeler çıkarıldı. Son metin budur. Bunun yerine kendi ifadelerinizden herhangi birini korumak için son adımdaki Karşılaştır'ı kullanın.
import-diagnostic-text-box-dropped = { $count ->
    [one] { $path } içinde { $count } metin kutusu var. Metni belgenin akışının dışında durduğu için el yazmasında nereye ait olduğu belirlenemiyor. İçe aktarılmadı.
   *[other] { $path } içinde { $count } metin kutusu var. Metinleri belgenin akışının dışında durduğu için el yazmasında nereye ait oldukları belirlenemiyor. İçe aktarılmadılar.
}
import-diagnostic-embedded-object-dropped = { $count ->
    [one] { $path } içinde { $count } gömülü nesne var: bir grafik, bir denklem ya da benzeri. El yazmasında onu barındırabilecek bir şey yok.
   *[other] { $path } içinde { $count } gömülü nesne var: bir grafik, bir denklem ya da benzeri. El yazmasında onları barındırabilecek bir şey yok.
}
import-diagnostic-field-flattened = { $count ->
    [one] { $path } içinde { $count } alan var: bir sayfa numarası, bir çapraz başvuru, bir tarih. En son gösterdiği metni korur ve bir daha güncellenmez.
   *[other] { $path } içinde { $count } alan var: bir sayfa numarası, bir çapraz başvuru, bir tarih. Her biri en son gösterdiği metni korur ve bir daha güncellenmez.
}
import-diagnostic-unknown-style-level = { $path }, başlığa benzeyen ama düzey belirtmeyen “{ $detail }” biçemini kullanıyor. Bu paragraflar bir derinlik tahmin edilmek yerine metin olarak içe aktarılır.
import-diagnostic-comment-unanchored = { $path } içindeki “{ $detail }” yorumu, ilgili olduğu sözcüklere bağlanamadı. Taşıyabileceğiniz şekilde kendi öğesinde tutuluyor.
import-diagnostic-comment-not-carried = { $path } içindeki “{ $detail }” yorumu içe aktarılmadı, çünkü dosyadan onun tutulabileceği metin olarak hiçbir şey gelmiyor. Gerekirse dosyadan kopyalayın.
import-diagnostic-comment-replies-flattened = { $count ->
    [one] { $path } içindeki { $count } yanıt dosyada olmayan bir yoruma bağlıydı, bu yüzden kendi başına bir yorum olarak gelir.
   *[other] { $path } içindeki { $count } yanıt dosyada olmayan bir yoruma bağlıydı, bu yüzden her biri kendi başına bir yorum olarak gelir.
}
import-diagnostic-prose-not-verbatim = { $count ->
    [one] { $path } içindeki { $count } paragraf, biçimlendirmesi yazıldığı gibi aktarılamadığı için düz metin olarak gelir. Özgün hâliyle karşılaştırın.
   *[other] { $path } içindeki { $count } paragraf, biçimlendirmeleri yazıldığı gibi aktarılamadığı için düz metin olarak gelir. Özgün hâlleriyle karşılaştırın.
}
import-diagnostic-styled-spaces-not-carried = { $count ->
    [one] { $path } içinde { $count } boşluk dizisi, örneğin doldurulmak üzere bırakılmış bir satır gibi, altı ya da üstü çizili. Skribisto yalnızca boşluklardan oluşan bir dizide bu çizgileri korumaz; bu yüzden düz boşluk olarak gelir, paragrafın başında ya da sonundaysa hiç gelmez.
   *[other] { $path } içinde { $count } boşluk dizisi, örneğin doldurulmak üzere bırakılmış satırlar gibi, altı ya da üstü çizili. Skribisto yalnızca boşluklardan oluşan dizilerde bu çizgileri korumaz; bu yüzden düz boşluk olarak gelirler, paragrafın başında ya da sonundaysalar hiç gelmezler.
}
import-diagnostic-list-nesting-flattened = { $count ->
    [one] { $path } içindeki { $count } liste öğesi { $limit } düzeyden daha derin iç içe geçmişti. Sözcükleri ve biçimlendirmesiyle birlikte, Skribisto'nun koruduğu en derin düzey olan { $limit }. düzeyde gelir.
   *[other] { $path } içindeki { $count } liste öğesi { $limit } düzeyden daha derin iç içe geçmişti. Sözcükleri ve biçimlendirmeleriyle birlikte, Skribisto'nun koruduğu en derin düzey olan { $limit }. düzeyde gelirler.
}
import-diagnostic-table-read-as-paragraphs = { $count ->
    [one] { $path } içindeki { $count } tablonun bir satırı diğerlerinden çok daha geniş; ızgara olarak dizilemeyecek kadar dengesiz. Hücreleri, sözcükleri ve biçimlendirmeleriyle birlikte art arda paragraflar olarak gelir.
   *[other] { $path } içindeki { $count } tablonun her birinde diğerlerinden çok daha geniş bir satır var; ızgara olarak dizilemeyecek kadar dengesizler. Hücreleri, sözcükleri ve biçimlendirmeleriyle birlikte art arda paragraflar olarak gelir.
}
import-diagnostic-spaces-shortened = { $count ->
    [one] { $path } içindeki { $count } boşluk dizisi { $limit } boşluktan uzundu ve { $limit } boşluğa kısaltılarak gelir. Çevresindeki sözcüklerin hepsi yerinde.
   *[other] { $path } içindeki { $count } boşluk dizisi { $limit } boşluktan uzundu ve { $limit } boşluğa kısaltılarak gelir. Çevrelerindeki sözcüklerin hepsi yerinde.
}
import-diagnostic-epigraph-not-carried = “{ $title }” bir epigrafla başlıyor, ama { $kind } türündeki bir öğe epigraf barındıramaz. Alıntı bunun yerine metninin en üstünde tutulur.
import-diagnostic-epigraph-placement-ambiguous = Bir epigraf “{ $title }” ile “{ $below }” arasında duruyor ve ikisinin de başında olabilir. Epigrafın genellikle bulunduğu yer olan “{ $title }” öğesine verildi.
import-document-diagnostics = { $errors ->
    [0] { $warnings ->
            [one] Bilinmesi gereken 1 şey
           *[other] Bilinmesi gereken { $warnings } şey
        }
   *[other] { $errors ->
            [one] 1 dosya okunamadı
           *[other] { $errors } dosya okunamadı
        }
}
import-document-diagnostics-none = Bildirilecek bir şey yok.

versions-title = Sürümler
versions-scope-synopsis = Özet
versions-scope-prose = Metin
versions-loading = Yedekleriniz inceleniyor…
versions-empty = Bunun henüz daha eski bir sürümü yok
versions-error = Yedekleriniz okunamadı. Hiçbir şey kaybolmadı ama bu liste eksik olabilir
versions-did-not-exist = { $date } tarihinde henüz yoktu
versions-deleted-after = { $date } tarihinden bir süre sonra silindi
versions-unreadable = { $count ->
    [one] { $count } yedek okunamadı
   *[other] { $count } yedek okunamadı
}
versions-thinned = { $count ->
    [one] Eski sürümler zamanla seyrekleşir. Projenin kendi geçmişi bu metnin daha önceki { $count } durumunu zaten bıraktı.
   *[other] Eski sürümler zamanla seyrekleşir. Projenin kendi geçmişi bu metnin daha önceki { $count } durumunu zaten bıraktı.
}
versions-source-backup = Bir yedekten
versions-source-project = Projenin kendi geçmişinden
versions-list-caption = Yedek başına değil, değişiklik başına bir kayıt
versions-pick-a-version = Neyin değiştiğini görmek için bir sürüm seçin
versions-earliest = Kayıtlı en eski sürüm. Karşılaştırılacak daha eski bir şey yok.
versions-no-change = Öğenin bu kısmında hiçbir şey değişmedi
versions-formatting-only = Burada yalnızca biçimlendirme değişti. Sözcükler aynı
versions-show-unchanged = Değişmeyenleri göster
versions-hide-unchanged = Değişmeyenleri gizle
versions-next-change = Sonraki değişiklik
versions-near = "{ $text }" yakınında
versions-words-added = { $count ->
    [one] { $count } sözcük eklendi
   *[other] { $count } sözcük eklendi
}
versions-words-removed = { $count ->
    [one] { $count } sözcük kaldırıldı
   *[other] { $count } sözcük kaldırıldı
}
versions-blocks-moved = { $count ->
    [one] { $count } paragraf taşındı
   *[other] { $count } paragraf taşındı
}
versions-pin = Bu sürümün geldiği yedeği sabitleyin. Otomatik temizlik onu asla silmez
versions-unpin = Bu sürümün geldiği yedeğin sabitlemesini kaldırın. Otomatik temizlik onu yeniden silebilir
versions-pinned-only = Yalnızca sabitlenmiş sürümleri göster
versions-pin-note = Yalnızca bir yedekten gelen sürümler sabitlenebilir. Sabitleme bir dosyayı korur; projenin kendi geçmişi ise bir dosya değildir.
versions-pinned-empty = Burada henüz sabitlenmiş bir şey yok
versions-pinned-empty-log = Burada henüz sabitlenmiş bir şey yok. Yalnızca bir yedekten gelen sürümler sabitlenebilir. Sabitleme bir dosyayı korur; projenin kendi geçmişi ise bir dosya değildir.
versions-range-filter = Yalnızca iki tarih arasındaki sürümleri göster
versions-filtered-empty = Belirlediğiniz süzgeçlerle eşleşen sürüm yok
versions-clear-filters = Süzgeçleri temizle
versions-last-30-days = Son 30 gün
versions-restore-button = Bu sürümü geri yükle
versions-restore-confirm-title = Bu metin { $date } tarihli sürümle değiştirilsin mi?
versions-restore-confirm-text = Şu anki metin, bu öğenin { $date } tarihindeki metniyle değiştirilecek.
versions-restore-confirm-with-comments = Şu anki metin, bu öğenin { $date } tarihindeki metniyle değiştirilecek. { $count ->
    [one] { $count } yorum şu anki metne bağlı ve sahipsiz kalabilir.
   *[other] { $count } yorum şu anki metne bağlı ve sahipsiz kalabilir.
}
versions-restore-confirm-undo-note = Önce bir yedek alınır ve Ctrl+Z bunu tek adımda geri alır.
versions-restored-toast = { $date } tarihli sürüm geri yüklendi
versions-undo = Geri al
versions-restore-row-gone = Bu öğe artık bu projede değil
versions-restore-no-home = Bu öğenin türü o zamandan beri değişti ve eski metnin onda konabileceği bir yer yok
versions-restore-no-safety-copy = Güvenlik yedeğiniz alınamadı, bu yüzden hiçbir şey değiştirilmedi
versions-restore-failed = Geri yükleme başarısız oldu: { $error }
versions-restore-backup-busy = Zaten bir yedekleme sürüyor. Biraz sonra yeniden deneyin
versions-restore-in-backup-file = Bir yedek dosyasına bakıyorsunuz; içine geri yüklemek için projenin kendisini açın
versions-restore-no-project = Açık proje yok
versions-recreate-button = Bunu geri getir…
versions-recreate-picker-title = Nereye konsun?
versions-recreate-picker-empty = Bu projede onu koyacak bir dizin yok
versions-recreate-picker-confirm = Buraya geri getir
versions-recreate-picker-cancel = İptal
versions-recreate-untitled = bu öğe
versions-recreate-confirm-title = “{ $item }” geri getirilsin mi?
versions-recreate-confirm-text = { $date } tarihindeki metniyle birlikte şuraya eklenecek: { $destination }.
versions-recreate-confirm-undo-note = Ardından gelen iletideki Geri al onu hemen yeniden çıkarır.
versions-recreated-toast = “{ $item }” projenize geri döndü
versions-recreated-partial-toast = { $count ->
    [one] “{ $item }” geri döndü, ama metinlerinden biri okunamadı
   *[other] “{ $item }” geri döndü, ama metinlerinden { $count } tanesi okunamadı
}
versions-recreate-already-here = Bu öğe zaten projenizde
versions-recreate-no-destination = Onu koymak için dizinde bir yer seçin
versions-recreate-unreadable = Bu yedek okunamadı, bu yüzden hiçbir şey eklenmedi
versions-recreate-failed = Geri getirilemedi: { $error }
versions-changed-percent = Bunun %{ $percent } kadarı değişti
versions-hidden-paragraphs = { $count ->
    [one] … değişmeyen { $count } paragraf …
   *[other] … değişmeyen { $count } paragraf …
}

timeline-title = Zamanda geri git
timeline-coverage = { $count ->
    [one] { $oldest } tarihine kadar uzanan { $count } kayıtlı sürüm
   *[other] { $oldest } tarihine kadar uzanan { $count } kayıtlı sürüm
}
timeline-loading = Projenizin geçmişi inceleniyor…
timeline-empty = Bu projenin henüz hiçbir sürümü kaydedilmedi
timeline-no-changes = O zamandan beri hiçbir şey değişmedi
timeline-no-text-changes = O zamandan beri hiçbir metin değişmedi
timeline-slider-label = Kayıtlı sürüm
timeline-series-name = Proje boyutu
timeline-bars-caption = Her çubuk kayıtlı bir sürümdür ve yüksekliği projenin o zamanki boyutu kadardır. Vurgulanan, şu anda baktığınız sürümdür.
timeline-bars-caption-periods = Tek tek gösterilemeyecek kadar çok sürüm var, bu yüzden her çubuk bir { $period } ve yüksekliği projenin o sürenin sonundaki boyutu kadar.
timeline-unit-hour = saat
timeline-unit-day = gün
timeline-unit-week = hafta
timeline-unit-month = ay
timeline-range-filter = Yalnızca iki tarih arasındaki sürümleri göster
timeline-range-empty = Bu tarihler arasında kaydedilmiş sürüm yok
timeline-open-period = Bu dönemi aç
timeline-show-all = Tüm geçmişi göster
timeline-changed-since = { $count ->
    [one] { $date } ile projenizin şu anki hâli arasında { $count } öğe farklı
   *[other] { $date } ile projenizin şu anki hâli arasında { $count } öğe farklı
}
timeline-kind-added = O zamandan beri yazılan
timeline-kind-removed = Artık projede olmayan
timeline-kind-changed = O zamandan beri düzenlenen
timeline-kind-moved = O zamandan beri taşınan
timeline-not-yet-written = Bu, o noktada henüz yoktu
timeline-no-text-of-its-own = Bunun kendine ait metni yok. İçindekilerin başlığıdır
timeline-reader-close = Kapat
timeline-reader-stamp = { $date } tarihindeki hâliyle
timeline-reader-loading = Kayıtlı sürüm açılıyor…
timeline-reader-unreadable = Bu kayıt okunamadı. Yedek taşınmış, silinmiş ya da bağlı olmayan bir sürücüde olabilir.
timeline-reader-compared = { $date } tarihindeki hâli, şu anki hâliyle karşılaştırmalı
timeline-reader-diff-legend = Üstü çizili olanlar o zamandan beri gitti; altı çizili olanlar eklendi.
timeline-reader-view-label = Gösterilecek
timeline-reader-view-diff = Değişiklikler
timeline-reader-view-text = Metin
timeline-reader-deleted = Bu artık projenizde değil. Burada okuyup kopyalayabilirsiniz.
timeline-prose-only-record = Bu nokta, metinden başka bir şey tutmayan projenin kendi geçmişinden geliyor; bu yüzden o zamandan beri neyin silindiği ya da taşındığı gösterilemez. Bunun için bir yedek seçin.

settings-sec-extensions = Eklentiler

settings-group-games-forward = Hep ileri
settings-games-forward-toggle = "Hep ileri" oyununu oyna
settings-games-forward-hint =
    Oynarken yazdığınız hiçbir şey geri alınamaz: aşağıda seçtiğiniz yüzeylerde
    Geri silme, Delete, Kes, sürükle-bırak ve Geri al devre dışıdır. Yine de
    özgürce yazabilir, yapıştırabilir, biçimlendirebilir ve gezinebilirsiniz. Taslak yalnızca büyür.
settings-games-session-warning =
    Bu oturumluk bir seçimdir: asla kaydedilmez. Projeyi kapatmak ya da Skribisto'dan
    çıkmak oyunu her zaman bitirir ve oynarken yazdığınız her şey o anda yeniden
    geri alınabilir hâle gelir.
settings-group-games-scope = Nerede geçerli
settings-games-in-prose = El yazması metni
settings-games-in-synopsis = Özetler
settings-games-scope-hint =
    Yorumlar, dipnotlar ve başlıklar asla dondurulmaz: kendinize yasakladığınız
    düzeltmeyi not ettiğiniz yerler onlardır.
settings-games-inert-warning =
    "Hep ileri" açık ama hiçbir şeye uygulanmıyor. Yukarıda en az bir yüzeyi
    işaretleyin, yoksa siz yazarken hiçbir şeyi değiştirmez.

games-title = Yazma oyunları
games-forward-name = Hep ileri
games-forward-blurb =
    Hiçbir şeyi geri almadan taslak yazın. Oynarken silme, kesme ve geri alma
    devre dışıdır. Sonuncuyu düzeltmek yerine bir sonraki tümceyi yazın.
games-forward-playing = Oynanıyor: silme devre dışı
games-forward-idle = Oynanmıyor
games-session-note = Projeyi kapattığınızda biter.
games-scope-prose-and-synopsis = Metninize ve özetlerinize uygulanır.
games-scope-prose = Metninize uygulanır.
games-scope-synopsis = Özetlerinize uygulanır.
games-scope-nothing = Henüz hiçbir şeye uygulanmıyor. Ayarlar'dan bir yüzey seçin.
games-settings-link = Yazma oyunu ayarları…
statusbar-games-forward = Hep ileri
statusbar-games-forward-tooltip =
    "Hep ileri" açık: taslak yazarken silme, kesme ve geri alma devre dışı.
    Oyunu bırakmak için tıklayın.
goal-progress-words = { $g ->
    [one] { $count } / { $goal } sözcük
   *[other] { $count } / { $goal } sözcük
}
goal-progress-characters = { $g ->
    [one] { $count } / { $goal } karakter
   *[other] { $count } / { $goal } karakter
}
goal-count-words = { $n ->
    [one] { $count } sözcük
   *[other] { $count } sözcük
}
goal-count-characters = { $n ->
    [one] { $count } karakter
   *[other] { $count } karakter
}
goal-unit-words = Sözcük
goal-unit-characters = Karakter

inspector-goal = Hedef
inspector-goal-none = Hedef yok

overview-col-goal = Hedef
overview-excluded-from-export = Dışa aktarmanın dışında bırakıldı, bu yüzden hiçbir toplama katılmaz

goal-subtree-total-words = { $count ->
    [one] İçerideki { $items } hedefin toplamı { $words } sözcük
   *[other] İçerideki { $items } hedefin toplamı { $words } sözcük
}
goal-subtree-total-characters = { $count ->
    [one] İçerideki { $items } hedefin toplamı { $words } karakter
   *[other] İçerideki { $items } hedefin toplamı { $words } karakter
}

new-work-goal-unit = Sayım birimi
new-work-goal-unit-hint = Bu projedeki her sözcük ya da karakter hedefi için kullanılır. Daha sonra değiştirilebilir.

settings-group-goal-unit = Hedefler
settings-goal-unit-switch-title = Sayım birimini değiştir
settings-goal-unit-switch-text = Bu proje için önceden belirlenmiş hedefler ve ara hedefler şu birimle girildi: { $from }. Şu birime geçmek onları dönüştürmez: { $to }; bu yüzden var olan her sayı artık bu birimle okunacak.
settings-goal-unit-switch-informative = Hiçbir şey kaybolmaz. Özgün okumaya dönmek için istediğiniz zaman geri geçebilir, sonra korumak istediğiniz hedefleri güncelleyebilirsiniz.
settings-goal-unit-switch-confirm = Geçiş yap

goal-distribute-action = Dağıt…
distribute-title = Hedefi dağıt
distribute-empty = Bunun içinde hedefin dağıtılabileceği bir şey yok.
distribute-weight = Dağıtım
distribute-weight-length = Uzunluğa göre
distribute-weight-rows = Parça sayısına göre
distribute-weight-even = Eşit olarak
distribute-overwrite = Önceden belirlenmiş hedefleri değiştir
distribute-col-item = Öğe
distribute-col-current = Şimdi
distribute-col-proposed = Sonra
distribute-total = Bunların toplamı { $total }; hedef ise { $goal }.
distribute-over-budget = İçeride önceden belirlenmiş hedefler bu kapsayıcının kendi hedefinden { $over } fazla. Onun hedefini yükseltin, onlarınkini düşürün ya da onları değiştirin.
distribute-apply = Dağıt
distribute-cancel = İptal

milestone-target-gone = Hedef silindi
milestone-no-target = Hedef yok
milestone-add = Ekle
milestone-add-label = Neye ulaşılmalı

pace-summary-title = Kitap ne durumda
pace-summary-remaining = { $words } kaldı
pace-summary-open = Planı aç
pace-summary-close = Kapat
pace-summary-dont-show = Açılışta bunu gösterme
pace-summary-menu = Yazma planı…

menu-help-topics = &Yardım konuları
menu-help-shortcuts = Klavye kı&sayolları…
menu-help-website = &Web'de Skribisto
menu-help-report = Sorun &bildir…
menu-help-check-updates = &Güncellemeleri denetle
menu-command-palette = &Komut paleti…

learn-help-topics = Yardım konuları
learn-shortcuts = Klavye kısayolları
learn-website = Web'de Skribisto

help-window-title = Yardım
help-filter-topics = Konuları süz
help-no-matching-topic = Eşleşen konu yok.
help-topic-missing = Bu konu artık kullanılamıyor.
help-back = Geri
help-not-translated = Bu sayfa henüz çevrilmediği için İngilizce gösteriliyor.

help-section-getting-started = Başlarken
help-section-writing = Yazma
help-section-reviewing = Gözden geçirme
help-section-exchanging = Sözcükleri içeri ve dışarı aktarma
help-section-keeping = Çalışmanızı güvende tutma
help-section-extensions = Eklentiler

help-topic-getting-started = İlk projeniz
help-topic-writing-model = Bir kitap nasıl bir araya gelir
help-topic-drafts-and-old-versions = Eski bir taslağı saklamak
help-topic-goals-and-pace = Hedefler ve tempo
help-topic-comments = Yorumlar
help-topic-round-trip = Kitabınızı bir okura göndermek
help-topic-export = Dışa aktarma
help-topic-import-documents = Belgeleri içe aktarma
help-topic-import-projects = Bütün bir projeyi aktarmak
help-topic-backups-and-versions = Yedekler ve sürümler

help-shortcuts-title = Klavye kısayolları
help-shortcuts-filter = Kısayolları süz
help-shortcuts-no-matches = Eşleşen kısayol yok.
help-shortcuts-rebind = Kısayolları değiştir…
help-shortcuts-close = Kapat

command-palette-placeholder = Bir komut yazın

shortcut-name-binder-duplicate = Çoğalt
shortcut-name-comments-add = Yorum ekle
shortcut-name-comments-add-paragraph = Paragrafa yorum yap
shortcut-name-spellcheck-toggle = Yazımı denetle
shortcut-name-editor-tab-close = Sekmeyi kapat
shortcut-name-editor-tab-pin = Sekmeyi sabitle ya da sabitlemeyi kaldır
shortcut-name-editor-save = Kaydet
shortcut-name-work-export = Dışa aktar…
shortcut-name-work-new = Yeni proje
shortcut-name-work-open = Proje aç
shortcut-name-window-new = Yeni pencere
shortcut-name-work-close = Projeyi kapat
shortcut-name-app-settings = Ayarlar
shortcut-name-app-quit = Çık
shortcut-name-editor-insert-footnote = Dipnot ekle
shortcut-name-format-scene-break = Sahne arası ekle
shortcut-name-format-major-scene-break = Büyük sahne arası ekle
shortcut-name-format-link = Bağlantı ekle
shortcut-name-format-mark-nbsp = Bölünmez boşluk ekle
shortcut-name-format-mark-nbhyphen = Bölünmez tire ekle
shortcut-name-format-mark-nnbsp = Dar bölünmez boşluk ekle
shortcut-name-format-mark-zwsp = Sıfır genişlikli boşluk ekle
shortcut-name-go-next = Sonraki
shortcut-name-go-prev = Önceki
shortcut-name-go-to = Git
shortcut-name-outline-toggle = Ana hattı göster/gizle
shortcut-name-preview-toggle = Önizleme şeridini göster/gizle
shortcut-name-view-fullscreen = Tam ekranı aç/kapat
shortcut-name-view-focus-mode = Dikkat dağıtmayan modu aç/kapat
shortcut-name-editor-size-increase = Metin boyutunu büyüt
shortcut-name-editor-size-decrease = Metin boyutunu küçült
shortcut-name-editor-size-reset = Metin boyutunu sıfırla
shortcut-name-editor-find = Bul
shortcut-name-editor-replace = Değiştir
shortcut-name-editor-find-next = Sonraki eşleşme
shortcut-name-editor-find-prev = Önceki eşleşme
shortcut-name-search-show = Projede ara
shortcut-name-search-replace = Projede değiştir
shortcut-name-outline-open-to-side = Yanda aç
shortcut-name-help-topics = Yardım konuları
shortcut-name-help-shortcuts = Klavye kısayolları
shortcut-name-help-website = Web'de Skribisto
shortcut-name-help-report = Sorun bildir
shortcut-name-help-check-updates = Güncellemeleri denetle
update-check-behind = Sürüm { $version } kullanıma sunuldu.
update-check-current = En yeni sürümü kullanıyorsunuz.
update-check-failed = Daha yeni bir sürüm olup olmadığı denetlenemedi. { $error }
shortcut-name-command-palette = Komut paleti
help-section-reference = Neyin ne olduğu

help-concept-scene-break = Sahne arası
help-concept-major-scene-break = Büyük sahne arası
help-concept-find-in-prose = Metinde bul
help-concept-story-bible = Hikâye rehberi
help-concept-goal-unit = Sözcük ya da karakter
help-concept-goal-progress = İlerleme
help-concept-manuscript-words = El yazmasına neler sayılır
help-concept-exportable = Dışa aktarmanın dışında bırakılan
help-concept-distribute = Bir hedefi dağıtmak
help-concept-subtree-total = İçerideki hedeflerin toplamı
help-concept-milestone = Ara hedef
help-concept-pace-plan = Tempo planı

help-concept-tag = Etiket
help-concept-label = Açıklama
help-concept-point-of-view = Bakış açısı
help-concept-epigraph = Epigraf
help-concept-footnote = Dipnot
help-concept-chapter-mode = Bölüm biçimi
help-concept-comment = Yorum
help-concept-backup = Yedek
help-concept-version = Sürüm
help-concept-trash = Çöp kutusu
help-concept-spellcheck = Yazım denetimi
help-concept-search-replace = Ara ve değiştir
help-concept-note-template = Not şablonu
help-concept-story-bible-entry = Hikâye rehberi kaydı
help-concept-text-replacement = Metin değiştirme
help-concept-smart-punctuation = Akıllı noktalama
help-concept-export-style = Dışa aktarma biçemi
help-concept-round-trip-marks = Gidiş-dönüş işaretçileri

epigraph-mark-attribution = Kaynak satırı
epigraph-mark-attribution-tip = İmlecin bulunduğu satırı alıntının kaynağı olarak işaretler, böylece bir atıf olarak yazdırılır
margin-lane-name = Kenar işaretleri
margin-lane-provider-comments = Yorumlar
margin-lane-provider-story-bible = Hikâye rehberi kaydı
margin-lane-provider-story-bible-hint = Bir notun Metinde görünümünde, o kaydın adının geçtiği yerler ve onun gözünden anlatılan sahneler. Bu adları metnin kendisinde de işaretler.
margin-lane-mark-point-of-view = Buradan anlatılıyor
margin-lane-mark-named-here = Burada adı geçiyor
margin-lane-provider-comments-hint = Metne bir notun bağlandığı yerler
margin-lane-provider-search = Arama sonuçları
margin-lane-provider-search-hint = Son aradığınız şeyin belgedeki tüm sonuçları
margin-lane-provider-boundaries = Her belgenin başladığı yer
margin-lane-provider-boundaries-hint = Bir akıştaki her sahnenin başında bir çizgi, böylece nerede olduğunuzu görürsünüz
margin-lane-provider-spelling = Yazım
margin-lane-provider-spelling-hint = Yazım denetiminin bir sözcüğü işaretlediği yerler. Siz istemedikçe kapalıdır: metniniz hakkında bir makinenin görüşünü işaretler
margin-lane-spelling = { $word }, olası yazım hatası
margin-lane-search-hit = { $text }, eşleşme { $index } / { $total }
margin-lane-search-current = { $text }, eşleşme { $index } / { $total }, bulunduğunuz eşleşme
margin-lane-boundary = Başlangıç: { $title }
margin-lane-boundary-untitled = adsız bir belge
settings-page-margin-lane = Kenar işaretleri
settings-desc-margin-lane = Kaydırma çubuğunun yanındaki şerit ve gösterdikleri
settings-margin-lane-enabled = Kenar şeridini göster
settings-margin-lane-enabled-hint = Görünüm ▸ Kenar işaretleri ile aynı anahtar.
settings-margin-lane-enabled-more = Hiçbir şeyin yanlış olduğunu söylemez: size şeylerin nerede olduğunu gösterir ve bundan ne çıkaracağınızı size bırakır.
settings-group-margin-lane-marks = Neleri işaretler
settings-margin-lane-no-providers = Şeridi henüz hiçbir şey işaretlemiyor.
settings-group-margin-lane-texture = Diyalog dokusu
settings-margin-lane-texture = Diyalog dokusunu göster
settings-margin-lane-texture-hint = Paragraf başına bir çubuk: ne kadar uzun olduğu ve ne kadarının konuşma olduğu.
settings-margin-lane-texture-more = Belirlenmiş bir kuralı olmayan dillerde ölçülmez; orada çubuk yalnızca uzunluğu gösterir.
settings-group-margin-lane-surfaces = Nerede görünür
settings-margin-lane-surface-editor = Metin düzenleyici
settings-margin-lane-surface-stream = Akışlar
settings-margin-lane-surface-search-preview = Arama önizlemesi
menu-margin-lane = &Kenar işaretleri

status-preset-drafting = Taslak yazımı
status-preset-passes = Düzeltme turları
status-preset-plume = Plume Creator
status-todo = Yapılacak
status-draft = Taslak
status-revised = Düzeltildi
status-final = Son hâli
status-outline = Ana hat
status-first-edit = 1. düzeltme
status-second-edit = 2. düzeltme
status-done = Bitti
status-plume-draft-1 = 1. taslak
status-plume-draft-2 = 2. taslak
status-plume-draft-3 = 3. taslak
status-plume-edit-1 = 1. düzeltme
status-plume-edit-2 = 2. düzeltme
status-plume-edit-3 = 3. düzeltme
status-plume-proofread = Son okuma yapıldı
status-plume-finished = Tamamlandı
new-work-statuses = İş akışı
new-work-statuses-hint = Bir sahnenin geçtiği aşamalar. Bunları istediğiniz zaman yeniden adlandırabilir, yeniden sıralayabilir ya da yenilerini ekleyebilirsiniz.
status-none = Durum yok
inspector-status = Durum
overview-col-status = Durum
overview-status-mixed = alttaki parçalar farklı
help-concept-status = Durum
status-completion-title = Kitap ne durumda
status-completion-headline = { $total } sahneden { $done } tanesi tamamlandı
status-completion-empty = Henüz sahne yok. Bu, el yazmasını okur; bu yüzden siz yazdıkça dolar.
status-completion-open = Kitap ne durumda…

settings-page-statuses = Durumlar
settings-desc-statuses = Bir sahnenin geçtiği aşamaları yeniden adlandırın, sıralayın, ekleyin ve kaldırın
settings-statuses-add = Durum ekle
settings-statuses-add-placeholder = Yeni durumun adı
settings-statuses-desc =
    Bir sahnenin sırayla geçtiği aşamalar: en üstte en az bitmiş, en altta en çok
    bitmiş olan. Bu sıra, uygulamanın başka her yerinde "daha az bitmiş" ifadesinin
    anlamıdır; bu yüzden onları çalışma sürecinizin gerçekten işlediği sıraya koyun.
settings-statuses-apply-preset = Hazır bir dizi uygula…
settings-statuses-preset-applied = { $added ->
    [one] 1 durum eklendi
   *[other] { $added } durum eklendi
}
settings-statuses-preset-refused = Bu projenin zaten bir durum basamakları dizisi var. Hazır diziler yalnızca boş bir diziyi doldurur. Önce istemediğiniz basamakları silin.
settings-statuses-duplicate = "{ $name }" bu dizide zaten var
settings-statuses-added = "{ $name }" eklendi
settings-statuses-move-up = Yukarı taşı (daha az bitmiş)
settings-statuses-move-down = Aşağı taşı (daha çok bitmiş)
settings-statuses-details-placeholder = Bu aşamanın anlamı (isteğe bağlı)
settings-statuses-delete = Sil: "{ $name }"
settings-statuses-delete-in-use = { $count ->
    [one] Sil: "{ $name }". Bu durumda 1 öğe var ve durumunu yitirecek
   *[other] Sil: "{ $name }". Bu durumda { $count } öğe var ve durumlarını yitirecekler
}
settings-statuses-deleted = "{ $name }" silindi
settings-statuses-deleted-in-use = { $count ->
    [one] "{ $name }" silindi. 1 öğenin artık durumu yok.
   *[other] "{ $name }" silindi. { $count } öğenin artık durumu yok.
}
settings-statuses-empty-title = Henüz iş akışı yok
settings-statuses-empty-body = Durum, bir sahnenin ne kadar ilerlediğini söyler. Yukarıdan bir tane ekleyin ya da hazır bir diziyle başlayın.
status-category-planned = Planlandı
status-category-drafting = Taslak yazılıyor
status-category-needs-work = Çalışılması gerekiyor
status-category-revised = Düzeltildi
status-category-final = Son hâli

progress-percent = %{ $percent }
progress-percent-with-label = %{ $percent } · { $label }
progress-import-opening = Proje açılıyor…
progress-import-outline = Ana hat okunuyor…
progress-import-converting = Bölümler ve sahneler dönüştürülüyor…
progress-import-writing = .skrib dosyası yazılıyor…
progress-import-done = Bitti
progress-import-structure = Yapı çıkarılıyor…
progress-export-start = Başlatılıyor…
progress-export-writing = Yazılıyor…
progress-project-reading = Proje okunuyor…
settings-sec-work-titled = Proje: { $title }

import-notice-flattened-rich-texts = { $count ->
    [one] Bir metin, bir Skribisto projesinin barındırabileceğinden daha derin iç içe geçmişti; sözcükleri biçimlendirmesiz düz metin olarak korundu.
   *[other] { $count } metin, bir Skribisto projesinin barındırabileceğinden daha derin iç içe geçmişti; sözcükleri biçimlendirmesiz düz metin olarak korundu.
}
import-notice-free-text-badges = { $count ->
    [one] Bir sahne rozeti bir durum listesinden çok serbest metne benziyor, bu yüzden etikete dönüştürülmek yerine öğenin açıklaması olarak korundu.
   *[other] { $count } farklı sahne rozeti bir durum listesinden çok serbest metne benziyor, bu yüzden etikete dönüştürülmek yerine öğe açıklamaları olarak korundu.
}
import-notice-separator-text-dropped = “{ $name }” ayırıcısı metin içeriyordu; bunun bir Skribisto projesinde yeri yok, bu yüzden metin içe aktarılmadı.
import-notice-separator-without-scene = “{ $name }” ayırıcısının bölümünde bağlanabileceği önceki bir sahne yoktu, bu yüzden içe aktarılmadı.
import-notice-chapter-without-scenes = “{ $name }” bölümünde sahne yok, bu yüzden ayırıcıları içe aktarılmadı.
import-notice-rich-text-unconverted = Biçimlendirilmiş bazı metinler dönüştürülemedi ({ $error }); içe aktarılmadı.
import-notice-unresolved-cross-link = #{ $number } numaralı kadro kaydına giden bir bağlantı izlenemedi: kayıt eksik ya da çöp kutusundaydı.
import-notice-read-copy = Bu projenin { $container ->
    [single-file] tek dosya
   *[folder] klasör
} kopyası okundu.
import-notice-read-copy-dated = Bu projenin, en son { $day } tarihinde değiştirilmiş { $container ->
    [single-file] tek dosya
   *[folder] klasör
} kopyası okundu.
import-notice-original-format = Bu proje Manuskript'in 2016'daki özgün biçiminde. Tamamı okundu; içindeki hiçbir şey değiştirilmedi.
import-notice-newer-single-file-beside = “{ $file }” bu klasörden daha yeni ({ $file_day }, klasörünki { $folder_day }). Manuskript bir proje tek dosya kipine geçtiğinde eski klasörü yerinde bırakır, bu yüzden bu klasör terk edilmiş bir kopya olabilir; bunun yerine .msk dosyasını içe aktarmak daha yeni olanı okur.
import-notice-pickle-skipped = “{ $file }” okunmadan bırakıldı. Bu bir Python pickle dosyası ve bir proje dosyasından böyle bir dosya yüklemek Manuskript'in kendisinde bir güvenlik açığıydı (CVE-2021-35196); içerdiği ayarlar yazı değil, görüntüleme tercihleridir.
import-notice-file-unreadable = “{ $file }” okunamadı ({ $error }) ve atlandı; geri kalan her şey içe aktarıldı.
import-notice-not-utf8 = “{ $file }” geçerli bir UTF-8 değil; okunamayan karakterler içe aktarmayı durdurmak yerine değiştirildi.
import-notice-table-unreadable = “{ $file }” okunamadı ({ $error }); içerdikleri içe aktarılmadı.
import-notice-outline-missing = “{ $file }” eksik, bu yüzden bu projede içe aktarılacak el yazması yok.
import-notice-outline-unreadable = “{ $file }” okunamadı ({ $error }); hiçbir el yazması içe aktarılmadı.
import-notice-named-none-row = “{ $file }”, Manuskript'in boş “none” kaydını tuttuğu yerde adlandırılmış bir satırla (“{ $name }”) başlıyor. Bu satır dışarıda bırakıldı, böylece ondan sonraki satırlar el yazmasının kullandığı numaraları korur.
import-notice-unknown-field = “{ $file }” bu içe aktarıcının tanımadığı bir alan içeriyor: “{ $field }”. İçe aktarılmadı.
import-notice-colorless-labels = { $count ->
    [one] “{ $file }” içinde rengi olmayan bir satır var. Renksiz olarak içe aktarıldı; Manuskript böyle bir satırda projeyi yüklemeyi durdurur.
   *[other] “{ $file }” içinde rengi olmayan { $count } satır var. Renksiz olarak içe aktarıldılar; Manuskript böyle bir satırda projeyi yüklemeyi durdurur.
}
import-notice-settings-unreadable = “{ $file }” okunamadı ({ $error }); projenin dili içe aktarılmadı.
import-notice-revisions-unreadable = “{ $file }” okunamadı ({ $error }); projenin geçmişi içe aktarılmadı, ama başka hiçbir şey etkilenmedi.
import-notice-foreign-files = { $count ->
    [one] Bu proje, Manuskript biçiminin parçası olmayan ve içe aktarılmayan bir dosya içeriyor: { $files }.
   *[other] Bu proje, Manuskript biçiminin parçası olmayan ve içe aktarılmayan { $count } dosya içeriyor: { $files }.
}
import-notice-foreign-files-more = Bu proje, Manuskript biçiminin parçası olmayan ve içe aktarılmayan { $count } dosya içeriyor: { $files } ve { $more ->
    [one] bir tane daha
   *[other] { $more } tane daha
}.
import-notice-not-a-character-file = “{ $file }” bir Manuskript kişi dosyası değil ve dışarıda bırakıldı.
import-notice-not-a-text-item = “{ $file }” bir Manuskript metin öğesi değil ve ana hattın dışında bırakıldı.
import-notice-missing-blank-line = “{ $file }” üst bilgisinden sonra boş satır içermiyor. Manuskript son alanını kaybeder, ama burada korundu.
import-notice-folder-without-metadata = “{ $folder }” içinde { $file } yok, bu yüzden dizinin adını taşıyan sıradan bir klasör olarak içe aktarıldı. Manuskript böyle bir dizini ve içindeki her şeyi atlar.
import-notice-rows-given-ids = { $count ->
    [one] Ana hattaki bir satırın kendine ait kimliği yoktu ve ona bir kimlik verildi. Manuskript böyle bir satırı atar ya da satır bir klasörse projeyi yüklemeyi durdurur.
   *[other] Ana hattaki { $count } satırın kendine ait kimliği yoktu ve onlara birer kimlik verildi. Manuskript böyle bir satırı atar ya da satır bir klasörse projeyi yüklemeyi durdurur.
}
import-notice-plots-unreadable = “{ $file }” okunamadı ({ $error }); olay örgüleri içe aktarılmadı.
import-notice-world-unreadable = “{ $file }” okunamadı ({ $error }); dünya kayıtları içe aktarılmadı.
import-notice-world-not-opml = “{ $file }” bir OPML belgesi değil; dünya kayıtları içe aktarılmadı.
import-notice-text-flattened = “{ $title }” metni bir Skribisto projesinin barındırabileceğinden daha derin iç içe geçmiş; sözcükleri biçimlendirmesiz düz metin olarak korundu.
import-notice-text-unconverted = “{ $title }” metni dönüştürülemedi ({ $error }); tam olarak yazıldığı gibi korundu.
import-notice-text-without-file-name = “{ $title }” öğesinin { $kind ->
    [scene] sahne metnine
    [note] not metnine
    [synopsis] özetine
   *[title] başlığına
} bir dosya adı verilemedi, bu yüzden içe aktarılmadı.
import-notice-unknown-label = “{ $title }” öğesinin etiketi { $index }, ama bu numara projenin etiket listesinde yok. Etiketsiz olarak içe aktarıldı.
import-notice-unknown-status = “{ $title }” öğesinin durumu { $index }, ama bu numara projenin durum listesinde yok. Durumsuz olarak içe aktarıldı.
import-notice-unknown-point-of-view = “{ $title }”, bu projenin kadrosunda olmayan bir kişinin gözünden anlatılıyor. Bakış açısı belirlenmedi.
import-notice-unresolved-references = “{ $title }” metnindeki göndermeler çözümlenemedi ({ $error }); metin, gönderme işaretleri dahil dönüştürüldüğü gibi korundu.
import-notice-custom-icons = { $count ->
    [one] Bir satırın özel bir simgesi vardı. Skribisto her satıra kendi türünün simgesini verir, bu yüzden simge içe aktarılmadı; satırla ilgili başka hiçbir şey değişmedi.
   *[other] { $count } satırın özel bir simgesi vardı. Skribisto her satıra kendi türünün simgesini verir, bu yüzden simgeler içe aktarılmadı; bu satırlarla ilgili başka hiçbir şey değişmedi.
}
import-notice-character-colors = { $count ->
    [one] Bir kişinin adının yanında bir renk vardı. Skribisto satırları değil etiketleri renklendirir, bu yüzden renk içe aktarılmadı; kişinin kendisi aktarıldı.
   *[other] { $count } kişinin adının yanında bir renk vardı. Skribisto satırları değil etiketleri renklendirir, bu yüzden renkler içe aktarılmadı; kişilerin hepsi aktarıldı.
}
import-notice-versions-kept = { $found ->
    [one] Bu proje daha önceki bir sürüm kaydetmişti
   *[other] Bu proje daha önceki { $found } sürüm kaydetmişti
}; { $kept ->
    [one] bir tanesi
   *[other] { $kept } tanesi
}, Skribisto'nun kendi geçmişini seyreltme biçimiyle seyreltilerek korundu. Sürümler panelinde bulunuyorlar.
import-notice-versions-unplaced = { $count ->
    [one] Daha önceki bir sürüm artık projede olmayan bir satıra ait ve yerleştirilemedi.
   *[other] Daha önceki { $count } sürüm artık projede olmayan bir satıra ait ve yerleştirilemedi.
}

dialog-open-work-title = Skribisto projesi aç
dialog-save-as-file-title = Tek bir .skrib dosyası olarak kaydet
dialog-save-as-folder-title = Proje için bir üst klasör seçin
dialog-manuskript-file-title = Bir Manuskript projesi seçin
dialog-manuskript-folder-title = Bir Manuskript proje klasörü seçin
dialog-filter-skrib = Skribisto projesi
dialog-filter-manuskript = Manuskript projesi
dialog-filter-plume = Plume Creator projesi
dialog-filter-hunspell-aff = Hunspell ek dosyası (.aff)
dialog-filter-hunspell-dic = Hunspell sözlüğü (.dic)
dialog-filter-export = Dışa aktarılan dosya
shortcut-name-view-margin-lane = Kenar işaretlerini göster/gizle

df-theme-paper = Kâğıt
df-theme-sepia = Sepya
df-theme-night = Gece
df-theme-high-contrast = Yüksek karşıtlık
df-theme-midnight = Gece yarısı

welcome-example-byline = Yazan: { $author }

welcome-sections-a11y = Karşılama bölümleri
comments-a11y-summary-author = { $author }: { $text }
comments-a11y-summary-replies = { $text } ({ $count ->
    [one] 1 yanıt
   *[other] { $count } yanıt
})
comments-a11y-summary-author-replies = { $author }: { $text } ({ $count ->
    [one] 1 yanıt
   *[other] { $count } yanıt
})
margin-lane-a11y-mark = { $label }, belgenin yüzde { $percent } kadarında
margin-lane-a11y-marks = { $label }, { $count } işaret, belgenin yüzde { $percent } kadarında
margin-lane-a11y-fallback = { $count ->
    [one] Kenar işaretleri, 1 işaret
   *[other] Kenar işaretleri, { $count } işaret
}
export-choose-unnamed-binder = Adsız dizin
export-style-neutral = Yalın
export-style-manuscript-shunn = Standart el yazması (Shunn)
export-style-ebook-clean = Sade e-kitap

date-time-short = { $day }.{ $month }.{ $year } { $hour }:{ $minute }
pace-day-label = { $day }.{ $month }

byte-size-bytes = { $value } B
byte-size-kb = { $value } KB
byte-size-mb = { $whole },{ $tenth } MB
byte-size-gb = { $whole },{ $tenth } GB
unit-percent = %{ $value }
unit-pixels = { $value } px
settings-paratext-error-no-name = Bir hazır yapının `name` satırı olmalı.
export-error-no-project = Dışa aktarılacak açık proje yok.

shortcut-group-work = Proje
shortcut-group-edit = Düzen
shortcut-group-view = Görünüm
shortcut-group-document = Belge
shortcut-group-format = Biçim
shortcut-group-go = Git
shortcut-group-tools = Araçlar
shortcut-group-help = Yardım

## Manuskript import: section headings written into the imported notes.
import-manuskript-section-paragraph-summary = Paragraf özeti
import-manuskript-section-full-summary = Tam özet
import-manuskript-section-motivation = Motivasyon
import-manuskript-section-epiphany = Aydınlanma
import-manuskript-section-notes = Notlar
import-manuskript-section-description = Açıklama
import-manuskript-section-passion = Tutku
import-manuskript-section-result = Sonuç
import-manuskript-section-summary = Özet
import-manuskript-section-meta = Üst veri
import-manuskript-section-series = Seri
import-manuskript-section-volume = Cilt
import-manuskript-section-genre = Tür
import-manuskript-section-license = Lisans
import-manuskript-section-email = E-posta
# The snowflake ladder's five tellings, shortest first.
import-manuskript-section-situation = Durum
import-manuskript-section-sentence = Tek cümle
import-manuskript-section-paragraph = Tek paragraf
import-manuskript-section-page = Tek sayfa
import-manuskript-section-full = Tam özet
import-manuskript-untitled-project = İçe aktarılan Manuskript projesi

## Plume import: implied story-bible groups and the title of a nameless project.
import-plume-group-characters = Kişiler
import-plume-group-items = Nesneler
import-plume-group-places = Mekânlar
import-plume-untitled-project = İçe aktarılan Plume projesi
