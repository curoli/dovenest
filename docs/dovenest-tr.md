# DoveNest: İnsan Merkezli Bir Gelecek için Kooperatif Planlama Yapan Yapay Zeka Ajanları

## Yönetici Özeti
Günümüzde yapay zeka genellikle ikiye ayrılır: **konuşma ortakları** (diyalog, bilgi paylaşımı ve yaratıcılıkta güçlü) ve **ajanlar** (araçlar ve arayüzler üzerinden görev yürütmede güçlü). Ancak insanlar, doğal olarak **planlama** ile **eylem** arasında akışkan bir şekilde hareket eder. Bu ayrım sürtüşmeye yol açar, mod değiştirmeyi zorunlu kılar ve parçalı deneyimler doğurur.

**DoveNest**, yeni bir standart öneriyor: **kooperatif planlama yapan ajanlar**. Bunlar **anlama, planlama ve eylemi** tek bir kesintisiz etkileşimde birleştirir. Kalıcı hafızaları, arayüz yetkinliklerini ve engellemeyen eşzamansız yürütmeleri birleştirerek DoveNest kişilikleri şunları yapabilir:
- Görevleri planlayıp yürütürken doğal diyalog kurmaya devam eder.  
- Bağlamı, hafızaları ve özel bilgileri yönetir.  
- Uzun süren işlemleri arka planda çalıştırır, diyalog ise sürer.  
- İnsan onayı, izlenebilirlik ve güvenlik sağlar.  

Bu beyaz kitap, DoveNest’in motivasyonunu, mimarisini ve on somut kullanım senaryosunu, ayrıca bir MVP yol haritası ve tasarım ilkelerini sunmaktadır. Vizyon: **insan merkezli bir yapay zeka platformu**, ajanların güvenilir ortaklar gibi çalıştığı — sürekli, bağlamsal ve güvenli.  

---

## İçindekiler
- [Giriş](#giriş)  
  - [Genel Bakış](#genel-bakış)  
  - [Motivasyon](#motivasyon)  
  - [Mevcut Yapay Zeka Kişiliklerinin Sorunları](#mevcut-yapay-zeka-kişiliklerinin-sorunları)  
  - [DoveNest Çözümü](#dovenest-çözümü)  
  - [DoveNest’in Temel Özellikleri](#dovenestin-temel-özellikleri)  
- [DoveNest Mimarisi](#dovenest-mimarisi)  
  - [Üç Katman](#üç-katman)  
  - [Yapay Zeka Kişilik Platformunun Rolü](#yapay-zeka-kişilik-platformunun-rolü)  
  - [Yapay Zeka Konuşma Ortakları](#yapay-zeka-konuşma-ortakları)  
  - [Yapay Zeka Ajanları](#yapay-zeka-ajanları)  
  - [Kooperatif Planlama Ajanları](#kooperatif-planlama-ajanları)  
- [Kullanım Senaryoları](#kullanım-senaryoları)  
  - [UC-01: GitHub ile Sürekli Pair-Programlama (mod değiştirmeden)](#uc-01-github-ile-sürekli-pair-programlama-mod-değiştirmeden)  
  - [UC-02: Güvenli Depo Çapında Refaktör (Batch + Kılavuzlar)](#uc-02-güvenli-depo-çapında-refaktör-batch--kılavuzlar)  
  - [UC-03: Çok Dilli Dokümantasyon ve Beyaz Kitap Senkronizasyonu](#uc-03-çok-dilli-dokümantasyon-ve-beyaz-kitap-senkronizasyonu)  
  - [UC-04: Araştırmadan PR’a (Derin Okuma → Uygulama)](#uc-04-araştırmadan-pra-derin-okuma--uygulama)  
  - [UC-05: Hata Triage ve Yeniden Üretim Asistanı](#uc-05-hata-triage-ve-yeniden-üretim-asistanı)  
  - [UC-06: Kod ve Kararlar Üzerine Bilgi Portalı](#uc-06-kod-ve-kararlar-üzerine-bilgi-portalı)  
  - [UC-07: Toplantıdan Backlog’a Otomasyon](#uc-07-toplantıdan-backloga-otomasyon)  
  - [UC-08: Veri/Embedding Pipeline Bootstrap](#uc-08-veriembedding-pipeline-bootstrap)  
  - [UC-09: Hafıza Yönetişimi ve Gizlilik Kontrolleri](#uc-09-hafıza-yönetişimi-ve-gizlilik-kontrolleri)  
  - [UC-10: On-Call Runbook Yardımcı Pilot](#uc-10-on-call-runbook-yardımcı-pilot)  
- [MVP Yol Haritası (İlk 4–6 Hafta)](#mvp-yol-haritası-ilk-46-hafta)  
- [Kullanım Senaryolarına Bağlı Tasarım İlkeleri](#kullanım-senaryolarına-bağlı-tasarım-ilkeleri)  
- [Açık Sorular](#açık-sorular)  

---

## Giriş

### Genel Bakış
DoveNest, **kooperatif planlama yapan ajanlar** geliştirir: **konuşma ortaklarının** ve **ajanların** güçlerini tek bir bütünleşik yapıda birleştiren yapay zeka kişilikleri.

### Motivasyon
Yapay zeka giderek daha güçlü hale geliyor ve iş, araştırma, eğitim ve günlük yaşamda merkezi roller üstleniyor. Bunun temelinde, **modeller** ve onların üzerine inşa edilen **yapay zeka kişilikleri** bulunuyor. Bu kişilikler, insanların yapay zekayı nasıl kullandığını belirler.

### Mevcut Yapay Zeka Kişiliklerinin Sorunları
Bugünkü kişilikler ikiye ayrılır:  
1. **Konuşma ortakları** – diyalog, bilgi paylaşımı, karmaşık konuları anlama ve yaratıcı fikirlerde güçlü.  
2. **Ajanlar** – belirlenmiş görevleri araçlar ve arayüzlerle yürütmede uzman.  

İnsanlar doğal olarak planlama ve eylem arasında gidip gelir. Bu ayrım sürtüşme yaratır.

### DoveNest Çözümü
Her iki kategori de aynı temel modellere dayanır; dolayısıyla tek bir kişilik her ikisinde de iyi olabilir. DoveNest, **kooperatif planlama yapan bir ajan** sunar.

### DoveNest’in Temel Özellikleri
- **Kalıcı hafıza** – bilgi, bağlam ve süreklilik için.  
- **Arayüz yetkinliği** – görevleri yerine getirmek için.  
- **Doğrusal olmayan etkileşim** – uzun görevler arka planda yürütülür.  
- **Bağlamsal, isteğe bağlı hafıza** – yalnızca gerektiğinde etkinleşir.  

---

## DoveNest Mimarisi

### Üç Katman
1. **Kullanıcı Arayüzü**  
2. **Yapay Zeka Kişilik Platformu**  
3. **Yapay Zeka Model Altyapısı**  

### Yapay Zeka Kişilik Platformunun Rolü
Bağlamı, hafızayı ve arayüzleri düzenler. DoveNest, konuşma ortağı ve ajanı tek bir kişilikte birleştirir.

### Yapay Zeka Konuşma Ortakları
Diyalog odaklıdır, insan benzeri ve faydalı yanıtlar üretir, hafızalarla desteklenir.

### Yapay Zeka Ajanları
Görevleri arayüzlerle yürütür; asenkron değilse konuşmayı kesebilir.

### Kooperatif Planlama Ajanları
Planlama ve eylemi sorunsuz birleştirir. Hafıza yönetimi, esnek bağlam, asenkron süreçleri destekler.

---

## Kullanım Senaryoları

### UC-01: GitHub ile Sürekli Pair-Programlama (mod değiştirmeden)
**Sorun:** Kod değişiklikleri için mod değiştirmek akışı bozar.  
**Vizyon:** Konuşma devam ederken IA commit atar, CI arka planda çalışır.  
**MVP:** Bir repo, bir branch, lint/fmt/test pipeline.  
**KPI:** CI <1 saatte geçer, ≥%80 PR düzeltmesiz birleşir.  

### UC-02: Güvenli Depo Çapında Refaktör (Batch + Kılavuzlar)
**Sorun:** Büyük refaktörler hata riski taşır.  
**Vizyon:** Küçük commitlerle güvenli refaktör.  
**MVP:** Basit yeniden adlandırma, testler yeşil.  
**KPI:** Ortalama süre, regresyon oranı.  

### UC-03: Çok Dilli Dokümantasyon ve Beyaz Kitap Senkronizasyonu
**Sorun:** Manuel çeviriler farklılaşır.  
**Vizyon:** Tek kaynak, çok dilde senkronizasyon.  
**MVP:** DE↔EN senkronizasyonu.  
**KPI:** <24s gecikme, terminoloji tutarlılığı.  

### UC-04: Araştırmadan PR’a (Derin Okuma → Uygulama)
**Sorun:** Spesifikasyonları koda dönüştürmek yavaş.  
**Vizyon:** IA belgeyi okur, test yazar, kod scaffold eder.  
**MVP:** Küçük modül + unit testler yeşil.  
**KPI:** Kapsam, inceleme döngüleri.  

### UC-05: Hata Triage ve Yeniden Üretim Asistanı
**Sorun:** Issue’lar çoğu zaman yeniden üretim içermez.  
**Vizyon:** IA issue’ları kümeleyip yeniden üretim testi oluşturur.  
**MVP:** Bir hata → test + PR fix.  
**KPI:** % yeniden üretim <1h, ilk fix süresi.  

### UC-06: Kod ve Kararlar Üzerine Bilgi Portalı
**Sorun:** Kararların nedenleri unutulur.  
**Vizyon:** Kod+ADR+PR üzerinden Q&A, kaynaklara bağlantılı.  
**MVP:** “Neden LanceDB?” sorusuna alıntıyla cevap.  
**KPI:** Yanıt doğruluğu, onboarding süresi.  

### UC-07: Toplantıdan Backlog’a Otomasyon
**Sorun:** Toplantı kararları backlog’a girmiyor.  
**Vizyon:** IA transkriptten görev çıkarır, issue açar.  
**MVP:** 5–10 issue tek transkriptten.  
**KPI:** % görev yakalama, paydaş memnuniyeti.  

### UC-08: Veri/Embedding Pipeline Bootstrap
**Sorun:** Embedding servis kurulumları tekrar eder.  
**Vizyon:** IA microservice scaffold (Rust+LanceDB).  
**MVP:** Lokal servis + smoke test.  
**KPI:** İlk sorgu süresi, p95 gecikme.  

### UC-09: Hafıza Yönetişimi ve Gizlilik Kontrolleri
**Sorun:** Kalıcı hafıza güçlü ama riskli.  
**Vizyon:** İnce ayarlı izin, opt-in, denetim logu.  
**MVP:** Konuşma bazlı onay/red, export/temizleme.  
**KPI:** Kullanıcı güveni, sıfır olay.  

### UC-10: On-Call Runbook Yardımcı Pilot
**Sorun:** Olaylara hızlı tepki gerek.  
**Vizyon:** IA alarmı runbook’a eşler, diagnostik çalıştırır.  
**MVP:** Olay simülasyonu → diagnostik + postmortem taslağı.  
**KPI:** MTTR düşüşü, öneri doğruluğu.  

---

## MVP Yol Haritası (İlk 4–6 Hafta)
- Hafta 1–2: UC-01 (repo+CI temel).  
- Hafta 3–4: Çok dilli sync + Bilgi Portalı.  
- Hafta 5–6: Hata yeniden üretim pilotu.  

---

## Kullanım Senaryolarına Bağlı Tasarım İlkeleri
- Mod değiştirme yok: konuşma ana kanal.  
- İnsan döngüde: merge/memory için onay şart.  
- Proveniens: PR, test, ADR, log bağlantıları.  
- Zarif degradasyon: araç yoksa plan devam.  
- Güvenlik: sandbox, minimum yetki token, imzalı eylemler.  

---

## Açık Sorular
- Asenkron güncellemeler için tek event modeli?  
- Diller arası standart test arayüzü?  
- Proje ve kişilikler arası taşınabilir hafıza şeması?  
