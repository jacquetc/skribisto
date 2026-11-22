# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Kaynaklar: dizindeki öğelere eklenen dosyalar ve web bağlantıları.

## Panel

resources-title = Kaynaklar
shortcut-name-resources-show = Kaynakları göster
resources-add-files = Dosya ekle
resources-add-files-title = Bu öğeye dosya ekle
resources-add-link = Web bağlantısı ekle
resources-drop-hint = Eklemek için dosyaları veya web adreslerini buraya bırakın
resources-no-item = Kaynaklarını görmek için bir öğe açın.
resources-empty = Bu öğeye henüz hiçbir şey eklenmedi. Yukarıdaki düğmelerle dosya veya web bağlantısı ekleyin ya da onları buraya bırakın.
resources-actions = Eylemler
resources-not-found = Bulunamadı
resources-row-linked = Bağlı dosya
resources-row-linked-size = Bağlı dosya, { $size }
resources-row-name = { $title }, { $kind }, { $detail }
resources-row-name-missing = { $title }, { $kind }, { $detail }, bulunamadı

## Dosya türleri, ekran okuyucunun okuduğu biçimiyle

resources-kind-picture = resim
resources-kind-pdf = PDF belgesi
resources-kind-audio = ses kaydı
resources-kind-video = video
resources-kind-document = belge
resources-kind-ebook = e-kitap
resources-kind-archive = arşiv
resources-kind-file = dosya
resources-kind-web-link = web bağlantısı

## Bir satırın menüsü

resources-open = Aç
resources-show-in-folder = Klasörde göster
resources-locate = Yeniden bul…
resources-insert-into-text = Metne ekle
resources-insert-picture = Resmi metne ekle
resources-insert-link = Metne bağlantı ekle
resources-edit = Yeniden adlandır ve açıkla…
resources-detach = Bu öğeden ayır
resources-remove = Projeden kaldır…

## İki iletişim kutusu

resources-link-dialog-title = Web bağlantısı ekle
resources-link-dialog-address = Adres
resources-link-dialog-address-placeholder = https://
resources-link-dialog-name = Başlık
resources-link-dialog-name-placeholder = Boş bırakılırsa sitenin adı
resources-link-dialog-add = Ekle
resources-edit-dialog-title = Yeniden adlandır ve açıkla
resources-edit-dialog-name = Başlık
resources-edit-dialog-description = Açıklama
resources-edit-dialog-description-placeholder = Bu nedir ve neden sakladınız
resources-edit-dialog-save = Kaydet
resources-dialog-cancel = İptal

## Ekleme

resources-large-title = Büyük bir dosya
resources-large-text = “{ $name }” dosyasının boyutu { $size }. Projede saklanırsa projenin her kopyasıyla ve her yedeğiyle birlikte taşınır. Bağlanırsa olduğu yerde kalır ve proje yalnızca onu nerede bulacağını hatırlar.
resources-large-text-many = “{ $name }” dosyasının boyutu { $size } ve { $others ->
    [one] başka bir dosya
   *[other] başka { $others } dosya
} da büyük. Projede saklanırlarsa projenin her kopyasıyla ve her yedeğiyle birlikte taşınırlar. Bağlanırlarsa oldukları yerde kalırlar ve proje yalnızca onları nerede bulacağını hatırlar.
resources-large-store = Projede sakla
resources-large-link = Bunun yerine bağla
resources-large-remember = Bir daha sorma
resources-storing-title = Dosyalar projeye ekleniyor
resources-store-cancel = İptal
resources-store-cancelled = Hiçbir dosya eklenmedi.
resources-store-busy = Dosyalar hâlâ ekleniyor. Eklendiklerinde yeniden deneyin.
resources-store-failed = Dosyalar eklenemedi: { $error }
resources-no-project = Dosya eklemeden önce bir proje açın.
resources-moved-away = Dosya kopyalanırken proje taşındı.
resources-some-failed = { $count ->
    [one] Bir dosya eklenemedi: { $names }
   *[other] { $count } dosya eklenemedi: { $names }
}
resources-add-failed = Kaynaklar eklenemedi: { $error }
resources-added = { $count ->
    [one] Bir kaynak eklendi
   *[other] { $count } kaynak eklendi
}
resources-dropped-added = { $count ->
    [one] Bu öğenin kaynağı olarak eklendi ve metinde bağlandı
   *[other] { $count } dosya bu öğenin kaynakları olarak eklendi ve metinde bağlandı
}
resources-undo = Geri al
resources-link-refused = “{ $url }”, Skribisto'nun saklayabileceği bir web adresi değil. Kullanıcı adı veya parola içermeyen, http:// ya da https:// ile başlayan bir adres kullanın.

## Tek dosyalık proje bildirimi

resources-folder-title = Bu proje büyüyor
resources-folder-text = Saklanan dosyaları artık { $size } yer kaplıyor. Tek dosya olarak tutulan bir proje, dosyalarıyla birlikte her kaydedişte baştan sona yeniden yazılır. Klasör olarak tutulursa yalnızca değişeni kaydeder. Projeyi şimdi klasör olarak kaydedebilir ve bundan sonra klasörde çalışmaya devam edebilirsiniz.
resources-folder-switch = Klasör olarak kaydet…
resources-folder-not-now = Şimdi değil
resources-folder-never = Bu proje için bir daha sorma

## Açma

resources-not-found-toast = “{ $name }”, projenin beklediği yerde değil.
resources-unsafe-title = “{ $name }” açılsın mı?
resources-unsafe-text = Skribisto bu tür bir dosyanın güvenli olup olmadığını bilmiyor. Dosyayı açmak bilgisayarınızda bir program başlatabilir ve bir proje başka birinden gelmiş olabilir. Ne olduğunu görmek için önce onu kendi klasöründe gösterin.
resources-open-anyway = Yine de aç
resources-open-failed = Dosya açılamadı: { $error }
resources-no-data-folder = bu sistem, Skribisto'ya bir kopyayı açabileceği bir veri klasörü sağlamıyor
resources-link-gone = Bu bağlantı, projede artık bulunmayan bir kaynağa işaret ediyor.
resources-locate-title = “{ $name }” şimdi nerede?
resources-relinked-more = { $count ->
    [one] Aynı klasörde eksik bir dosya daha bulundu ve yeniden bağlandı.
   *[other] Aynı klasörde eksik { $count } dosya daha bulundu ve yeniden bağlandı.
}
resources-changed-elsewhere = “{ $name }” başka bir uygulamada değiştirildi. Proje yeni sürümü tutuyor.

## Düzenleme

resources-edit-failed = Kaynak değiştirilemedi: { $error }
resources-remove-title = “{ $title }” projeden kaldırılsın mı?
resources-remove-text = Eklendiği her öğeden çıkarılır. Projede saklanan bir dosya onunla birlikte gider; bağlı bir dosya ve bir web sayfası oldukları yerde kalır. Geri al onu geri getirir.
resources-remove-confirm = Kaldır
resources-remove-failed = Kaynak kaldırılamadı: { $error }

## Metne

resources-no-text = Önce imleci bir metne koyun, ardından kaynağı oraya ekleyin.
resources-drop-insert-picture = Resmi ekle
resources-drop-insert-link = Kaynağa bağlantı ekle

## İlerleme

progress-resources-storing = Dosyalar kopyalanıyor
progress-resources-stored = Dosyalar kopyalandı
