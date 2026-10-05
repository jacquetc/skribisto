# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

wm-book = Projenizdeki bir kitap.
wm-book-more = Bir kitap dilediğiniz kadar yalın ya da katmanlı olabilir. Kısa bir öykü yalnızca birkaç [sahneden](:wm-scene) oluşabilir. Daha uzun bir eser [kısımları](:wm-part) ve [bölümleri](:wm-chapter) istediğiniz derinlikte iç içe yerleştirebilir. Biçimini siz belirlersiniz.
wm-part = Bölümleri bir araya getiren bir kısım.
wm-part-more = Bir kısım [bölümleri](:wm-chapter) tek bir başlık altında toplar. Bir bölüm gibi, sonraki kısma ya da [kitabın](:wm-book) sonuna kadar sürer.
wm-chapter = Kitabın bir bölümü.
wm-chapter-more = Bir bölüm buradan sonraki bölüme, sonraki [kısma](:wm-part) ya da [kitap sonuna](:wm-end-of-book) kadar sürer. İki biçimde olabilir: [sahneler](:wm-scene) barındıran bir bölüm klasörü ya da kendi metnini taşıyan düz bir bölüm. Yeni bölümlerin hangisini kullanacağını Ayarlar'da, Yapı altında seçin.
wm-scene = Metin içeren bir sahne.
wm-scene-more = Sahne yazınızı barındırır. Her yazı öğesinin iki yüzü vardır: ana metin ve planlama için bir [özet](:wm-synopsis). İkisi düzenleyicide birlikte açılır.
wm-note = Bir not.
wm-note-more = Not, bir [sahne](:wm-scene) gibi metin ve [özet](:wm-synopsis) taşır; araştırma, ara notlar ya da hatırlatmalar içindir. Bir [bölümün](:wm-chapter) içinde duran bir not da dahil, her öğe derlenen kitabın parçası olabilir.
wm-note-folder = Başka öğeler barındıran bir not.
wm-note-folder-more = Altında öğeler de barındıran bir [not](:wm-note); böylece ilgili notları bir araya toplayabilirsiniz. Kendi [özetini](:wm-synopsis) taşır, içindeki öğeler de kendi metinlerini.
wm-folder = Düzenleme amaçlı bir klasör.
wm-folder-more = Öğeleri dizinde size uygun düştüğü gibi düzenlemek için bir kapsayıcı. Bir [özet](:wm-synopsis) taşır ama kendine ait metni yoktur. Yazı, içindeki öğelerdedir.
wm-end-of-book = Bir kitabın bittiği yeri işaretler.
wm-end-of-book-more = Tüm kitaplar tek ve kesintisiz bir listeyi paylaşır, bu yüzden bir kitabın sonu iç içe yerleşimle belirlenmez. Bu işaret, [kitabın](:wm-book) burada bittiğini söyler. Ondan sonra gelen her şey sonraki kitaba aittir.
wm-story-bible-entry = Hikâyeniz hakkında etiketli, diğer adları olan bir not.
wm-story-bible-entry-more = Bir kişi, bir yer ya da kaydetmeye değer başka bir şey hakkında, adı, etiketleri, diğer adları ve başlangıç şablonuyla tek adımda oluşturulan bir [not](:wm-note). Sonrasında sayfada onu ayıran hiçbir şey yoktur; diğerleri gibi yeniden adlandırabileceğiniz, yeniden etiketleyebileceğiniz ya da dönüştürebileceğiniz sıradan bir not olarak kalır.
wm-synopsis = Bir yazı öğesinin özeti.
wm-synopsis-more = Her yazı öğesine bağlı bir özet; genellikle kısa bir paragraf, ama dilediğiniz kadar uzun olabilir. Metin ortada yokken planlamak ve gezinmek için kullanın. Düzenleyicide ana metnin yanında durur.

scene-break-minor = Sıradan bir sahne arası: bir bölüm içinde zamanın, mekânın ya da bakış açısının değişmesi.
scene-break-minor-more =
    Bir arayı *onu koyduğunuz yerde* işaretler; bir [sahnenin](:wm-scene) ortasında bile. Metni iki öğeye bölmek bir düzenleme tercihidir, bu yüzden kendiliğinden hiçbir zaman ara oluşturmaz.

    Metne aralarında boşluk olan üç yıldız olarak yazılır. Nasıl yazdırılacağına yazdığınız şey değil, dışa aktarma biçemi karar verir: Shunn el yazması biçimi onu tek bir # olarak, ticari cep kitabı bir dinkus olarak, Fransız, Alman, İspanyol, Rus ve İtalyan yayıncılığının çoğu ise hiçbir işaret olmadan yalnızca bir boşluk olarak dizer.
scene-break-major = Daha güçlü bir ayrım: büyük bir zaman atlaması ya da kesin bir bakış açısı değişikliği.
scene-break-major-more =
    Sıradan sahne arasıyla aynı düşünce, bir basamak yukarısı. Sıradan bir ara sıçramayı hafife alacaksa bunu kullanın.

    # # # olarak yazılır ve dışa aktarma biçemi tarafından sıradan aradan farklı yazdırılır. Shunn'un standart el yazması biçimi tam da bu ayrım için tek bir # ile # # # kullanır. Daha güçlü bir işareti olmayan geleneklerde iki ara da aynı yazdırılır.
wm-find-in-prose = Skribisto bu öğeyi metninizde arar.
wm-find-in-prose-more = Bunu hakkında yazdığınız şeyleri adlandıran etiketler için açın: kişiler, yerler, nesneler. Böyle bir etiket taşıyan her öğe, başlığı ve ona verdiğiniz diğer adlarla metninizde aranır; böylece her [sahne](:wm-scene) elle bağlantı kurmanıza gerek kalmadan kimin ve neyin içinde geçtiğini listeler. Bir öğeyi adlandırmayıp niteleyen etiketler için kapalı bırakın, örneğin bir taslak durumu ya da süreklilik denetimi hatırlatması. Eşleşen her öğe [hikâye rehberinize](:wm-story-bible) de katılır.
wm-story-bible = Her kişi, yer ve diğer etiketli kayıt, tek bir ızgarada toplanmış hâlde.
wm-story-bible-more = Herhangi bir not klasörünün kendi sekmesinden açılır. Kartlar etikete göre gruplanır ve her kart, bir kaydın kaç diğer adla anıldığını ve projedeki her [metinde bul](:wm-find-in-prose) etiketinden yola çıkarak kaç sahnede geçtiğini gösterir. Kitaba göre süzmek yalnızca hangi kartların gösterileceğini daraltır; hangi kayıtların var olduğuna asla karar vermez.

tooltip-go-to = Dizindeki herhangi bir öğeye git (Ctrl+G)
synopsis-collapse-tooltip = Özet sütununu gizle

wm-paratext = Öykünün parçası olmayan bir metin.
wm-paratext-more = Önsöz, ithaf, sonsöz, kolofon: kitaba ait olan ama gövdesine ait olmayan yazılar. Nereye koyarsanız orada dışa aktarılır ve el yazmasının sözcük sayısına asla katılmaz. Nereye gideceği size kalmış: gelenekler ülkeden ülkeye ve yayınevinden yayınevine değişir.
wm-paratext-folder = Yan metinler için bir klasör.
wm-paratext-folder-more = Önsözleri ve sonsözleri dizini kalabalıklaştırmadan saklayacak bir yer. Yalnızca düzenleme amaçlıdır: bir [özet](:wm-synopsis) taşır ama dışa aktarılan kitaba hiçbir şey, kendi adını bile eklemez.

goal-target = Bu parçanın ne uzunlukta olması gerektiği.
goal-target-more = Her şeye bir hedef koyabilirsiniz: bir [sahneye](:wm-scene), bir [bölüme](:wm-chapter), bir [kısma](:wm-part), tüm [kitaba](:wm-book). Her biri kendi sayısıdır ve kendi başına durur; bir bölüme hedef koymak kitabın hedefini asla değiştirmez ve bir kitabın hedefi asla içindeki bölümlerin toplamı değildir. Toplanan şey yazının kendisidir: bir kapsayıcının [ilerlemesi](:goal-progress), altında yazılmış her şeydir. Hedefi sıfırda bırakırsanız belirlenmemiş olur. Sayı, bu proje hangisini kullanıyorsa [sözcük ya da karakter](:goal-unit) olarak sayılır.
goal-unit = Bu projenin sözcükle mi karakterle mi saydığı.
goal-unit-more = Proje genelinde bir seçimdir, çünkü uzunluk birimi tek bir bölüme değil, el yazmasına ve onun pazarına aittir: Alman ve Fransız yayıncılığı karakterle, Japon yayıncılığı 400 karakterlik sayfalarla, Çin yayıncılığı bin karakterlik birimlerle, İngilizce yayıncılığın çoğu ise sözcükle ölçer. İki sayı yan yana tutulur ve hiçbiri diğerine dönüştürülmez; bu yüzden birimi değiştirmek her [hedefi](:goal-target) öteki sayıya yöneltir, geri değiştirmek de ilkini geri getirir. Yapmadığı şey herhangi bir dönüştürmedir: önceden girilmiş hedeflerin elle güncellenmesi gerekir.
goal-progress = Hedefe göre ne kadarının yazıldığı.
goal-progress-explained = Kendi metni olan bir parça kendi uzunluğuyla, bir kapsayıcı ise [altında yazılmış her şeyle](:goal-manuscript-words) ölçülür. Yazı geldikçe çubuk kırmızıdan kehribara, oradan yeşile döner ve parça planlananı epeyce aştığında yeniden değişir; uyulması gereken bir uzunluğu olan bir bölümde bunu fark etmek yararlıdır.
goal-manuscript-words = El yazmasına neler sayılır.
goal-manuscript-words-more = Yalnızca gerçekten dışa aktarılacak metin: kendi metnini taşıyan [sahneler](:wm-scene) ve [bölümler](:wm-chapter). Bir [not](:wm-note) ya da bir [yan metin](:wm-paratext) el yazmasının parçası değildir ve ona asla sayılmaz. Çöp kutusundaki her şey ve [dışa aktarmanın dışında bırakılan](:goal-exportable) her şey de sayımdan düşer.
goal-exportable = Bu öğe dışa aktarmanın dışında bırakıldı.
goal-exportable-more = Yazı hâlâ orada olduğu için kendi uzunluğu yine gösterilir, ama hiçbir dışa aktarılan kitapta görünmeyeceği gibi hiçbir toplama da katılmaz. Bir kapsayıcıyı dışarıda bırakmak içindeki parçaları dışarıda bırakmaz: her öğe kendi adına karar verir; anahtarın yanındaki "Alt öğelere uygula" düğmesi bunun içindir.
goal-distribute = Bu hedefi içindekilere dağıtın.
goal-distribute-more = Bir [kitabın](:wm-book) ya da [kısmın](:wm-part) hedefini doğrudan alt öğelerine, parçaların toplamı tam olarak ona eşit olacak biçimde böler. Varsayılan olarak yalnızca henüz hedefi olmayanları doldurur ve sizin sayılarınıza dokunmaz. Yazdığı şeyler sıradan [hedeflerdir](:goal-target); sonradan düzenlemekte ve birbirinden uzaklaşmakta serbesttirler: bu kalıcı bir düzen değil, tek seferlik bir işlemdir.
goal-subtree-total = İçerideki hedeflerin toplamı.
goal-subtree-total-more = Bir hedef değil, bir bilgidir. Bu proje bir kapsayıcının hedefini asla içindekilerin toplamı olarak görmez, çünkü içindeki bir sahneye sayı verildiğinde bir klasörün sayısının kendiliğinden değişmesi tam da böyle olur. Bu satır yalnızca kendi sayılarınızın neye ulaştığını söyler; böylece onu kapsayıcının [hedefiyle](:goal-target) kendiniz karşılaştırabilir ya da [bir hedefi dağıtabilirsiniz](:goal-distribute).
goal-milestone = Bir şeye ulaşmak için belirlenen tarih.
goal-milestone-more = İki türü vardır. Biri, [kitabın](:wm-book) bir tarihe kadar belirli bir uzunluğa ulaşması gerektiğini söyler; kendi eğrisi üzerinde bir duraktır. Öteki, belirli bir [bölümün](:wm-chapter) ya da [kısmın](:wm-part) o tarihe kadar [kendi hedefine](:goal-target) ulaşmış olması gerektiğini söyler ve bu sayının bir kopyasını tutmak yerine onu canlı okur; bu yüzden hedefi düzenlemek ara hedefi de günceller. İkisi de kitabın [tempo planında](:pace-plan) görünür.
pace-plan = Kitabın yazma takvimi.
pace-plan-more = Bir [hedef](:goal-target) ve bir son tarih, yazdığınız günlerle birlikte günde ne kadar gerektiğine dönüştürülür. Kitabın kendi hedefini okur; bu yüzden onu burada ya da Denetçi'de belirlemek aynı sayıyı iki yerde belirlemektir. Yol üzerindeki duraklar [ara hedeflerdir](:goal-milestone).

concept-tag = Herhangi bir öğeye koyabileceğiniz, tüm projede yeniden kullanılabilen renk kodlu bir işaret.
concept-tag-more = Bir öğeyi işaretlemek için etiketleyin: bir durum, bir yer, izlediğiniz bir olay örgüsü. Aynı etiket istediğiniz kadar öğede bulunabilir ve bir öğe birkaç etiket taşıyabilir. Kitabınızdaki bir şeyi, bir kişiyi ya da bir yeri adlandıran bir etiket için [metinde bul](:wm-find-in-prose) seçeneğini açın; Skribisto onu metninizde aramaya başlar.
concept-label = Bir öğenin başlığının altına kendiniz için yazdığınız kısa bir not.
concept-label-more = Bir [etiket](:concept-tag) değildir: açıklama yalnızca tek bir öğeye aittir; arkasında renk ya da katalog olmayan düz bir metindir, örneğin "1. dönüm noktası" ya da "bir ad gerekiyor". Öğenin bağlam menüsünden belirleyin ya da Genel bakış'taki Açıklama sütununda yerinde düzenleyin; hem ana hatta hem akışta başlığın altında küçük bir alt başlık olarak görünür.
concept-point-of-view = Bir sahnenin kimin gözünden anlatıldığı: hikâyenizin kadrosundan bir ya da daha fazla kişi.
concept-point-of-view-more = Denetçi'de, kadronun yanında belirlenir. Henüz kadroda olmayan birini seçmek onu oraya da ekler. Birden fazla bakış açısı geçerli bir seçimdir; ortak bir sahne ya da kitabın ortasında göz değiştirmek için yararlıdır ve Skribisto bunu engellemez. Adaylar, bir [metinde bul](:wm-find-in-prose) etiketinin hikâye rehberinize koyduğu öğelerden gelir.
concept-epigraph = Bir kısmın ya da bölümün başına konan alıntı.
concept-epigraph-more = [Bölümün](:wm-chapter) ya da [kısmın](:wm-part) kendi metninden ayrı, kendine ait bir alandır; bir [sahne](:wm-scene) ya da [not](:wm-note) epigraf taşımaz. Bir tane yazmak için sıradan bir alıntı yazın; birkaç tane yazarsanız her biri kendi blok alıntısı olur ve kendi epigrafı olarak yazdırılır. Sözcükleri asla el yazması sözcüğü olarak sayılmaz.
concept-footnote = Metinde işaretlenen ve numaralı bir not olarak yazdırılan bir kaynak gösterme.
concept-footnote-more = Gönderme, sözcüklerinizin içindeki tek bir işarettir; bu yüzden çevresindeki sözcükler gibi taşınır ve silinir. Numarası asla saklanmaz: Skribisto onu her göndermenin tüm el yazmasındaki yerinden her seferinde yeniden hesaplar; bu yüzden bir tane eklemek ondan sonraki tüm notları yeniden numaralar. Bir dipnotun kendi sözcükleri el yazmasından ayrı sayılır, asla ona katılmaz.
concept-chapter-mode = Yeni bir bölümün sahnelerden oluşan bir klasör mü yoksa tek ve düz bir öğe mi olduğu.
concept-chapter-mode-more = Ayarlar'da, Yapı altında belirlenir. Her iki durumda da doğrudan bir [bölümün](:wm-chapter) içine yazarsınız; tek fark, altında sahne de barındırıp barındıramayacağıdır. İki biçim de aynı kitaba derlenir ve bir projede ikisini dilediğiniz gibi karıştırabilirsiniz. Dönüştür, var olan bir bölümü tek sözcük kaybetmeden ikisi arasında değiştirir.
concept-comment = Metnin belirli bir parçasına bağlanan ve kenar boşluğunda gösterilen not.
concept-comment-more = Sabit bir konumu değil, gösterdiği sözcükleri ve çevrelerini hatırlar; bu yüzden olağan düzenlemelerden ve tam bir yeniden yüklemeden sağ çıkar. Bir tartışmayı sürdürmek için altına yanıt yazın. Alıntının kendisi artık bulunamıyorsa yorum, sessizce yanlış tümceye kaymak yerine bunu söyler.
concept-backup = Tüm projenin belirli bir andaki, yanında tutulan kopyası.
concept-backup-more = Seçtiğiniz bir takvimle alınır: açılışta, kapanışta, belirli aralıklarla ya da bunların bir karışımıyla; eski kopyalar yaşa ya da sayıya göre ayıklanır. Birini açmak onu kendi penceresinde yükler: bakabilir, hatta düzenleyebilirsiniz, ama yalnızca Farklı kaydet ya da Geri yükle bir şeyi korur. Tek bir öğenin kendi geçmişini izleyen [sürümden](:concept-version) farklıdır.
concept-version = Tek bir öğenin kendi geçmişi: metnindeki ya da özetindeki her değişiklik.
concept-version-more = Projenin kendi kaydetme geçmişi ile yedekleri birlikte kullanılarak, dosya başına değil değişiklik başına bir kayıtla oluşturulur. Neyin değiştiğini görmek için bir kayıt seçin ve yalnızca o öğenin metnini ona geri yükleyin; projenin geri kalanına dokunulmaz ve önce bir güvenlik kopyası alınır. Her şeyin kopyası olan [yedekten](:concept-backup) farklıdır.
concept-trash = Çöp kutusuna atılan bir öğe taşınmaz, gizlenir: tam olarak bulunduğu yerde kalır.
concept-trash-more = Çöp kutusuna atmak yalnızca bir işareti değiştirir: öğe dizindeki yerini korur, kaldırılmaz, etkin değil olarak işaretlenir. Geri yüklediğinizde tam o yerde yeniden belirir, yeniden yerleştirmeniz gerekmez. Çöp kutusunu boşalt, çöpteki öğeleri kalıcı olarak kaldırır ve bunların her biri, diğer tüm proje düzenlemeleri gibi geri alınabilir tek bir adımdır.
concept-spellcheck = Kurulu hiçbir sözlüğün tanımadığı sözcüklerin, proje diline göre altını çizer.
concept-spellcheck-more = Her projenin kendi çalışma dili ya da dilleri vardır; başka bir dilde yazılmış bir sahne için bir öğe bunları geçersiz kılabilir. Skribisto'nun bilmediği bir sözcüğü düzenleyicinin bağlam menüsünden, her projeye taşınan kişisel bir sözlüğe ekleyin. Yazım denetimini kapatmak, siz yeniden açana kadar onu her yerde durdurur.
concept-search-replace = Bir sözcüğü ya da ifadeyi yalnızca açık belgede değil, projenin her yerinde bulur.
concept-search-replace-more = Her sahneye, nota, başlığa, özete ve açıklamaya, ayrıca yorumlara ve dipnotlara bakar. Büyük/küçük harf, tam sözcük eşleştirme ve aksan eşleştirme üç ayrı anahtardır. Tümünü değiştir, gözden geçirilen her eşleşmeyi tek seferde ve tek bir geri alma adımında yeniden yazar; bu yüzden Ctrl+Z yalnızca son değişikliği değil, toplu işlemin tamamını geri alır.
concept-note-template = Üzerinde çalıştığınız her şeye ekleyebileceğiniz, yeniden kullanılabilir bir yazı parçası.
concept-note-template-more = Boş bir kişi kartı, bir mekân profili, bir sahne planı: bir kez yazın ve yalnızca notlara değil, gerektiği her öğeye ekleyin. Hazır bir şablonla başlayın, bir .md ya da .djot dosyası içe aktarın veya bir şey yazıp Belge ▸ Şablon olarak kaydet'i seçin. Projede saklanır, böylece onu açan bir ortak yazar aynı şablonları bulur.
concept-text-replacement = Siz yazarken kendiliğinden açılan kendi kısaltmalarınız.
concept-text-replacement-more = Bir tetikleyici ve onun yerine geçecek metni belirleyin, örneğin "bkz" için "bakınız"; ardından bir boşluk ya da noktalama işareti yazdığınız anda açılır. Büyük/küçük harf yazdığınızı izler: tetikleyiciyi büyük harfle başlatırsanız açılan metin de büyük harfle başlar. Açılımdan hemen sonra Geri silme yalnızca o değiştirmeyi geri alır. Proje başına açılır ve [akıllı noktalamadan](:concept-smart-punctuation) ayrıdır.
concept-smart-punctuation = Siz yazarken kendiliğinden uygulanan, dile duyarlı tipografi: tırnaklar, tireler, üç noktalar.
concept-smart-punctuation-more = Düz tırnakları tipografik tırnaklara çevirir, -- dizisini kısa çizgiye ve --- dizisini uzun çizgiye, ... dizisini üç nokta karakterine dönüştürür ve açtığınız yerde "- " ile başlayan bir paragrafı konuşma çizgisiyle açar. Proje başına belirlenir ve .skrib dosyasının içinde taşınır, böylece bir ortak yazar aynı kurallarla yazar. Kendiniz yazdığınız [kendi kurallarınızdan](:concept-text-replacement) ayrıdır.
concept-export-style = Dışa aktarma seçeneklerinin adlandırılmış bir paketi: başlıklar, aralıklar, neyin dahil edileceği.
concept-export-style-more = Hangi biçimi seçerseniz seçin, her dışa aktarmada yeniden kullanılan tek bir biçem. Hazır biçemler salt okunurdur; gerçekten değiştirebileceğiniz bir başlangıç noktası için birini çoğaltın. Seçenekleri arasında, bir editöre gidip dönecek bir dosya için [gidiş-dönüş işaretçilerinin](:concept-round-trip-marks) dahil edilip edilmeyeceği de vardır.
concept-round-trip-marks = Düzenlenmiş bir DOCX ya da ODT dosyasının geri döndüğünde tanınmasını sağlayan görünmez yer işaretleri.
concept-round-trip-marks-more = Yalnızca bir editörün geri gönderebileceği iki biçime, DOCX ve ODT'ye yazılır; başka hiçbir dışa aktarma bunları taşımaz. Özel bir öznitelik değil, bir yer işaretidir: LibreOffice kaydederken özel öznitelikleri siler ama yer işaretlerine dokunmaz. Her [dışa aktarma biçeminde](:concept-export-style), "Editöre gönderme" altında açılır.
concept-status = Bir öğenin kendi iş akışınızdaki yeri: her seferinde tek bir aşama, ileriye doğru.
concept-status-more = Bir [etiket](:concept-tag) değildir: bir öğenin tam olarak tek bir durumu vardır, aşamalar sıralıdır ve liste projeye aittir; bu yüzden onu istediğiniz zaman yeniden adlandırabilir, yeniden sıralayabilir ya da genişletebilirsiniz. Bir bölümün ya da kitabın yanındaki yıldız, altındaki parçaların hepsinin onunla aynı fikirde olmadığı anlamına gelir. Denetçi'den, bir karttan, bir akış öğesinden ya da buradan belirleyin.
