# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

wm-book = Projenizde bir kitap.
wm-book-more = Bir kitap sizin gibi basit ya da tabakalı. Kısa bir hikaye sadece birkaç [Sahne sahneleri sahneleri](:wm-scene) olabilir. Daha uzun bir Proje nest [parçalar parça parça parça parça parça parça parçaları](:wm-part) ve [Bölümler](:wm-chapter) istediğiniz kadar derin olabilir.
wm-part = Bir parça gruplama bölümü.
wm-part-more = Bir parça toplanır [Bölümler](:wm-chapter) Bir bölüm gibi, bir sonraki parçaya veya sonun sonuna kadar çalışır. [Kitap](:wm-book).
wm-chapter = Kitabın bir bölümü.
wm-chapter-more = Bir bölüm bir sonraki bölüme kadar burada, sonraki [Bölüm](:wm-part)'e veya [Kitabın sonu](:wm-end-of-book)'e geliyor: [Sahne sahneleri sahneleri](:wm-scene)'e sahip olan bir bölüm ya da kendi prototipini taşıyan düz bir bölüm. Yapı altında yeni bölümler hangileri kullanıyor.
wm-scene = Prose bir sahne.
wm-scene-more = Bir sahne yazınızı tutar. Her yazı kitabının iki tarafı vardır: ana metin ve [Özet](:wm-synopsis) ile plan yapmak için.
wm-note = Bir not.
wm-note-more = Bir not metni tutar ve [Özet](:wm-synopsis), araştırma için [Sahne](:wm-scene) gibi, bir kenara veya hatırlatmalar. Herhangi bir öğe [Kısım](:wm-chapter) içinde bir notun parçası olabilir.
wm-note-folder = Diğer eşyaları tutan bir not.
wm-note-folder-more = Aşağıdaki öğeler de içeren [Not](:wm-note), bu yüzden birlikte ilgili notları gruplayabilirsiniz. Kendi [Özet](:wm-synopsis) tutar ve içerideki öğeler kendi metinlerini tutar.
wm-folder = Bir düzenleme klasörü.
wm-folder-more = Ancak binder'deki eşyaları düzenlemek için bir konteyner size uygundur. [Özet](:wm-synopsis) taşır, ancak yazının içindeki öğeler.
wm-end-of-book = Bir kitabın sona erdiği Marks.
wm-end-of-book-more = Her kitap sürekli bir liste paylaşıyor, bu yüzden bir kitap sonunda nesting tarafından karar verilmez. Bu işaret, [Kitap](:wm-book)'in burada durmasına karar verir.Bir sonraki kitaba ait olan her şey.
wm-story-bible-entry = Bir etiket, hikayeniz hakkında bir not.
wm-story-bible-entry-more = Bir karakter hakkında [Not](:wm-note), bir yer ya da başka bir şey filing, bir isim, etiketler, takmalar ve bir başlangıç şablonu ile bir adımdaki her şeyi ayırt eder. sayfada hiçbir şey onu daha sonra ayırt ettiğinden, sıradan bir not olarak yeniden adlandırılabilir ya da başka herhangi bir şekilde dönüştürebilirsiniz.
wm-synopsis = Bir yazı kitabının özeti.
wm-synopsis-more = Herhangi bir yazı kitabına ek olarak, genellikle kısa bir paragraf ama sizin gibi uzun zamandır.prose var önce planlamak ve gezinmek için kullanın. editörün ana metninin yanında oturur.

scene-break-minor = Sıradan bir sahne molası: Zaman, yer veya bir bölüm içinde bakış.
scene-break-minor-more =
    Marks a break *nereye koydun* [Sahne](:wm-scene)'in ortasında da dahil olmak üzere. İki öğeye prose etmek örgütsel bir seçimdir, bu yüzden asla kendi başına bir mola yaratmaz.

    Uzaylar tarafından ayrılmış üç asterisk olarak prose içine alındı. Baskılar ihracat tarzına nasıl karar verildi, ne tür bir şekilde değil: Shunn Bir el yazması onu tek bir #, bir ticaret kağıt gerisi olarak bir dinkus olarak ayarlar ve çoğu Fransız, Almanca, İspanyolca, Rusça ve İtalyanca her şey işaretle açık bir boşluk olarak yayınlamaya karar verir.
scene-break-major = Daha güçlü bir bölüm: büyük bir zaman atlayın veya bakış açısından belirleyici bir değişiklik.
scene-break-major-more =
    Sıradan bir sahne molası olarak aynı fikir, bir adım yukarı. Bir düz molanın attığı zaman kullanın.

    # # # # ve ihracat tarzı tarafından sıradan katmanından farklı olarak yazdırıldı. Shunn'in standart yazma formatı, # # # # tam olarak bu ayrıma karşı tek bir # # # # 'ye karşı.Bir geleneğin daha güçlü bir işareti var, her ikisi de aynı yazdırdı.
wm-find-in-prose = Skribisto bu öğeyi Prose'de arar.
wm-find-in-prose-more = Bu, yazdığınız şeyler için etiketler için tıklayın: karakterler, yerler, nesneler. Böyle bir etiket taşıyan herhangi bir eşya, kendi unvanıyla ve verdiğiniz diğer isimlerle karşılanır. [Sahne](:wm-scene) Herhangi bir şeyi el ile ilişkilendirmeden kim ve görünen şeyleri listeler.Bir isim yerine bir öğe tarif eden etiketler için bırakın, bir taslak statüsü veya sürekliliği kontrol etmek için bir hatırlatma. [Hikâye rehberi](:wm-story-bible).
wm-story-bible = Her karakter, yer ve diğer etiket girişi, bir ağda toplandı.
wm-story-bible-more = Herhangi bir not klasörün kendi sekmesinden açın. Kartlar grubu etiketi ile açın ve her isim kaç tane giriş yanıtları ve kaç sahneler ortaya çıktı, her şeyden çizildi, her şeyden çekildi. [Prose'de bul](:wm-find-in-prose) Projede etiket. Kitap tarafından filtre yalnızca kartların gösterdiği darlar; hangi girişlerin var olduğuna asla karar vermez.

tooltip-go-to = Binder'deki herhangi bir öğeye atlayın (Ctrl+G)
synopsis-collapse-tooltip = sinopsis sütunu

wm-paratext = Hikayenin bir parçası olmayan bir metin.
wm-paratext-more = Bir preface, bir adanmış, bir sonraki kelime, bir colophon: Kitapa ait olan yazı yazmak ama onu nereye koyduğunuza kadar ihracat yapmak ve asla el yazmanın kelimesine saymamak. Nerede olursanız olun ülkeye ve yayıncıya farklılık gösterir.
wm-paratext-folder = Paratexts için bir klasör.
wm-paratext-folder-more = Bir yerlerde önyüzleri tutmak ve kelimelerden sonra, bu yüzden sadece binder'i karıştırmıyorlar: [Özet](:wm-synopsis) taşır, ancak ihraç edilen kitaba hiçbir şey ekler, kendi adı bile.

goal-target = Bu parça ne kadar zamandır olduğu anlamına gelir.
goal-target-more = Her şeye bir hedef oluşturun: bir [Sahne](:wm-scene), a) [Kısım](:wm-chapter), a) [Bölüm](:wm-part)Bütün bunlar [Kitap](:wm-book)Her biri kendi numarasıdır ve kendi başına duruyor, bu yüzden bir bölüm üzerinde bir hedef asla kitabını değiştirmiyor ve bir kitap hedefi asla içinde bölümlerden ibaret değildir.What does up is the writing: a container's. [İlerleme](:goal-progress) Onun altında yazılan her şeydir. Sıfırda bir hedef alın ve sadece set değildir. Sayıda sayılıyor. [kelimeler veya karakterler](:goal-unit)Ancak bu projenin kullandığına göre.
goal-unit = Bu proje kelimelerde veya karakterlerde sayılıyorsa.
goal-unit-more = Bir proje çapında bir seçim, çünkü uzun bir birim bir bölüm yerine el yazma ve pazarına aittir: Alman ve Fransız yayın ölçüleri işaretleri, 400-character çarşaflar, Çin'de binlerce karakter tarafından, çoğu İngilizce yayını yan yana tutulur ve her iki sayı da bir bölüme dönüştürülür, bu yüzden üniteyi diğer rakamdaki [Hedef](:goal-target)'i diğer rakama ayırın ve geri yüklemeyin, ilk olarak geri yüklemeyin.
goal-progress = Hedefe karşı ne kadar yazılır.
goal-progress-explained = Kendi uzunluğuna karşı bir parça ölçülüyor; [Aşağıda yazılan her şey](:goal-manuscript-words)'e karşı bir konteyner kırmızıdan yazılı olarak yeşile kadar ısınıyor ve bir kez daha parça planlandığında iyi bir şekilde ilerliyor, bu da bir bölümün devam etmesi için uzun süreye değer.
goal-manuscript-words = Yazının ne kadar önemli olduğunu.
goal-manuscript-words-more = Sadece ihraç edilecek prose: [Sahne sahneleri sahneleri](:wm-scene) ve [Bölümler](:wm-chapter) kendi metinlerini taşıyor. [Not](:wm-note) veya [Yan metin](:wm-paratext) yazının bir parçası değildir ve asla bir şey hesaba katmıyor. [İhracattan ayrıldıktan sonra](:goal-exportable).
goal-exportable = Bu öğe ihracattan ayrılır.
goal-exportable-more = Kendi uzunluğu hala gösteriyor, çünkü yazı hala orada, ancak toplam olarak değil, tam olarak ihraç edilen bir kitapta görünecektir gibi.Bir konteyner bırakmak parçalara içeriden ayrılmaz: Her bir öğe kendine cevap, bu da "Çocuklara Apply" düğmesine işaret eder.
goal-distribute = Bu hedefin içeride ne olduğunu paylaşın.
goal-distribute-more = Splits a a [Kitabın kitabı](:wm-book) veya veya veya [Bölüm's](:wm-part) Hemen hemen çocukları boyunca hedef, böylece parçalar tam olarak ekleniyor. Varsayılan olarak sadece hedef olmayanları doldurur ve kendi numaralarınızı yalnız bırakır.Yazdığı şey sıradandır. [Hedef hedefler hedefler hedefler hedefler hedefler](:goal-target)Daha sonra düzenlemek ve uzaklaşmak için ücretsiz: Bu bir eylem, ayakta bir düzenleme değil.
goal-subtree-total = İçerideki hedeflerin ne kadar ekleneceği.
goal-subtree-total-more = Aslında, bir hedef değil. Bu proje hiçbir zaman bir konteynerin hedefine ait değildir, bu yüzden konteynerin [Hedef](:goal-target) ile kendinizi karşılaştırabilirsiniz, ya da [Birini paylaş](:goal-distribute).
goal-milestone = Bir şeye ulaşmak için bir tarih.
goal-milestone-more = İki tür.Biri diyor ki, [Kitap](:wm-book) Bir tarihle belirli bir uzunlukta durmalıdır, kendi eğrisinde bir yol noktası. Diğeri belirli bir söylüyor [Kısım](:wm-chapter) veya veya veya [Bölüm](:wm-part) Var olmalıydıya [Kendi hedefi](:goal-target) O zaman, ve bu sayının bir kopyasını tutmak yerine yaşadığını okur, bu yüzden hedef de kilometreyi de güncelliyor. [Hız planı](:pace-plan).
pace-plan = Kitapın yazı programı.
pace-plan-more = [Hedef](:goal-target) ve bir tarih, yazdığınız günler ile, günde ne kadar ihtiyaç duyulana dönüştü.Kitapın kendi hedefini okur, bu yüzden burada ayarlayın ve Inspector'da aynı sayı iki yerdedir. Yol boyunca Waypoints [dönüm noktaları](:goal-milestone).

concept-tag = Herhangi bir öğeye koyabileceğiniz bir renk kodlanmış etiket, tüm proje boyunca yeniden kullanılabilir.
concept-tag-more = Etiket onu işaretlemek için bir öğe: bir statü, bir yer, takip ettiğiniz bir iplik. Aynı etiket herhangi bir sayıda öğe üzerinde oturabilir ve bir öğe birkaç tane taşıyabilir.Kitapınızda bir şey olan [Prose'de bul](:wm-find-in-prose)'de, bir karakter veya bir yer, ve Skribisto onu prose'de aramaya başlar.
concept-label = Bir öğenin başlığı altında yazdığınız kısa bir not, kendi gözleriniz için.
concept-label-more = [Etiket](:concept-tag) değil: bir etiket tek başına bir öğeye ait, arkadaki renkli veya kataloga ait bir yazı, "1st arsa noktası" veya "bir isim" gibi bir şey gösterir.List it from the item's context menu, or edit it inline in the Overview's Label; it show as a small subtitle under the stream both.
concept-point-of-view = Bir sahnenin kim tarafından yorumlanır: hikayenizin bir veya daha fazlası.
concept-point-of-view-more = Inspector'da set, dökümün yanında. Henüz oyuncuda birini seçmek onları orada da ekliyor.Bir bakış açısıyla daha fazla yasal bir seçim, paylaşılan bir sahne veya bir göz orta kitabı değişikliği için faydalı, bir şey değil, bir şey yok Skribisto Bloklar. Adaylar eşyalardan gelir [Prose'de bul](:wm-find-in-prose) etiket, hikayenize bible koyar.
concept-epigraph = Bir parçanın veya bir bölümün başında bir alıntı yapılır.
concept-epigraph-more = Kendi alanı, [Kısım](:wm-chapter)'den veya [Bölüm](:wm-part)'in kendi prosesyonundan ayrı; [Sahne](:wm-scene) veya [Not](:wm-note) sıradan bir alıntı olarak yazmıyor; birkaç tane yaz ve her biri kendi blok alıntısı haline gelir, kendi epigrafi olarak yazdırılır.
concept-footnote = Prose'de işaretlenen bir alıntı, bir sayı olarak yazdırın.
concept-footnote-more = Referans kelimelerinizin içinde tek bir işarettir, bu yüzden bu konudaki kelimeler olarak hareket eder ve silinir. numarası asla depolanmaz: Skribisto Projeler her referansın tüm el yazısında oturduğu yerden tazelenir, bu yüzden bir ayaknote'nin kendi sözleri yazılardan ayrı sayılır, asla içine katlanmaz.
concept-chapter-mode = Yeni bir bölüm sahnelerin veya tek bir düz öğenin bir klasörüdür.
concept-chapter-mode-more = Ayarları, Yapı altında doğrudan [Kısım](:wm-chapter)'e yazıyorsunuz; tek fark da aşağıdaki sahneleri içerebileceğidir. Her iki kodlama da aynı kitaba derlenir ve onları bir projede özgürce karıştırabilirsiniz.
concept-comment = Bir not tam bir prose yayılmasına yol açtı, marjda gösterilen.
concept-comment-more = Bu, işaret ettiği sözleri ve onları çevreleyen şeyleri hatırlıyor, sabit bir pozisyon değil, bu yüzden sıradan bir düzenleme ve tam bir reload hayatta kalıyor.Bir konu daha bulunamazsa, yorum yanlış cümleye sürüklenmeyi tercih ediyor.
concept-backup = Tüm projenin bir noktası-zaman kopyası, bunun yanında tuttu.
concept-backup-more = Seçtiğiniz bir program üzerinde durun: açık, yakın, bir aralıkta veya herhangi bir karışımda, eski kopyalarla yaşlanma ya da sayın. kendi penceresinde bir yük açın: görünebilir ve hatta düzenleyebilirsiniz, ancak sadece Kaydet veya geri yükleme bir şey tutar. [sürüm sürüm sürüm sürüm sürüm sürüm sürüm sürüm sürüm sürüm sürüm sürüm sürüm sürüm](:concept-version), hangi bir öğenin kendi geçmişi izler.
concept-version = Bir öğenin kendi geçmişi: metin veya sinopsis için her değişiklik.
concept-version-more = Projenin kendi tasarruf tarihinin ve yedeklerinin bir araya gelmesinden inşa edilmiş, dosya başına bir değişiklik yerine bir giriş yapın ve sadece bu öğenin metnini ona geri yüklemek için bir giriş yapın; projenin geri kalanı önceden alındı ve bir güvenlik kopyası her şeyden farklı olarak [Yedek](:concept-backup)'den alınmıştır.
concept-trash = Bir çöp eşyası gizlidir, taşındı: tam olarak nerede kaldı.
concept-trash-more = Trashing sadece bir bayrak döndürür: öğe yerini binder'de tutar, kaldırıldıktan ziyade aktif olarak işaret eder. geri yükleyin ve o kesin noktada yeniden filtrelenmez. Boş Trash iyi için çöp eşyaları kaldırır ve bunların her biri başka bir proje düzenleme gibi geri alınabilir.
concept-spellcheck = Alt kelimeler kurulu bir sözlük tanımaz, proje dili başına.
concept-spellcheck-more = Her projenin kendi çalışma dili veya dili vardır; bir eşya onları başka bir dilde yazılmış bir sahne için onarabilir. Skribisto editörün bağlam menüsünden, her projeye devam eden kişisel bir sözlüğe sahip değildir.
concept-search-replace = Projedeki her yerde bir kelime veya cümle bul, sadece açık belge değil.
concept-search-replace-more = Her sahne, not, başlık, sinopsis ve etiket ile bak ve yorumlar ve ayaknotlar da. Vaka, tüm kelime eşleştirme ve aksaklık, üç ayrı anahtardır. Tüm yeniden yazmalar her incelemede tek bir geçiş ve bir adım, o yüzden Ctrl+Z bütünü geri almaz, sadece son değişiklik.
concept-note-template = Üzerinde çalıştığınız her şeye eklediğiniz bir yeniden yazılabilir parça.
concept-note-template-more = Boş bir karakter sayfası, bir yer profili, bir dövme sayfası: bir kez yazın ve ihtiyacınız olan yerde bırakın, herhangi bir öğede, sadece bir not alın. inşa edilmiş bir başlangıçtan başlayın, .md veya .djot dosyasını ithal edin veya bir şey yazın ve Belgeyi seçin Projede, böylece aynı seti açan bir ortak yazar.
concept-text-replacement = Kendi kısa eliniz, tipiniz olarak otomatik olarak genişletildi.
concept-text-replacement-more = Bir tetikleyici ve değiştirme seti, "btw" için "bu şekilde" ayarlayın ve proje başına geçiş yaptıktan sonra, [Akıllı Noktalama](:concept-smart-punctuation)'den farklı bir alan ya da boksajı genişletin. Case what you typed: capitalize the trigger and the growth capitalizes too. A Backspace right after an extensiones only that one substitution.
concept-smart-punctuation = Locale-aware tipografi otomatik olarak sizin türüniz olarak uygulanır: alıntılar, dashes, elpses.
concept-smart-punctuation-more = Curls doğrudan tipografik olanları, dönüşleri - bir en dash ve --- bir em dash'e, dönüş ... elpsis'e ve nereye döndüğünüzde, "-" olarak bir paragraf türü açıyor ve .skrib'te seyahat ediyor, böylece bir ortak yazar, [Kendi kurallarınız](:concept-text-replacement)'den uzaklaşıyor.
concept-export-style = İhracat seçeneklerinin adı: başlıklar, spacing, hangileri dahil edilir.
concept-export-style-more = Bir stil, seçtiğiniz her ihracatta yeniden kullanılabilir.Yapılan stiller sadece okunur; aslında değiştirebilirsiniz bir başlangıç noktası almak için bir tane. Seçenekleri arasında [round-trip işaretleyiciler](:concept-round-trip-marks), bir dosya editörü ve gerisi için bağlı olup olmadığıdır.
concept-round-trip-marks = Düzenlenen DOCX veya ODT'in ev tanımasına izin veren görünmez kitap işaretleri.
concept-round-trip-marks-more = Sadece iki formatta bir editör geri gönderebiliyor, DOCX ve ODT; diğer her ihracat hiçbir şey taşımaz. Özel bir özellik değil: LibreOffice striptizler kurtarma konusunda özel nitelikler değil, [ihracat tarzı](:concept-export-style)'e kadar geri dönülemez.
concept-status = Bir öğe kendi iş akışınızda duruyor: Bir seferde bir aşama, ileriye doğru ilerliyor.
concept-status-more = [Etiket](:concept-tag) Değil: Bir öğe tam olarak bir statüye sahiptir, aşamalar sipariş edilir ve proje listeye sahiptir, böylece istediğiniz zaman yeniden sipariş edebilir veya ekleyebilirsiniz. Bir bölüm veya kitabın yanında bir risk, Inspector'dan gelen parçalar anlamına gelir.
