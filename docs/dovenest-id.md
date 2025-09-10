# DoveNest: Agen AI Perencana Kooperatif untuk Masa Depan Manusia

## Ringkasan Eksekutif
Saat ini, kecerdasan buatan terbagi antara **mitra percakapan** (kuat dalam dialog, berbagi pengetahuan, dan kreativitas) dan **agen** (kuat dalam mengeksekusi tugas melalui alat dan antarmuka). Namun, manusia secara alami mengalir antara **perencanaan** dan **tindakan**. Pemisahan ini menciptakan friksi, memaksa pergantian mode, dan menghasilkan pengalaman yang terfragmentasi.

**DoveNest** mengusulkan standar baru: **agen perencana kooperatif** yang menyatukan **pemahaman, perencanaan, dan tindakan** dalam satu interaksi yang mulus. Dengan menggabungkan memori persisten, kompetensi antarmuka, dan eksekusi asinkron yang tidak memblokir, persona DoveNest dapat:
- Tetap berdialog secara alami sambil merencanakan dan mengeksekusi tugas.  
- Mengelola konteks, memori, dan pengetahuan khusus.  
- Menjalankan operasi panjang di latar belakang sambil percakapan berlanjut.  
- Memastikan persetujuan manusia, jejak audit, dan keamanan.  

Buku putih ini menyajikan motivasi, arsitektur, dan sepuluh kasus penggunaan nyata DoveNest, bersama dengan roadmap MVP dan prinsip desain. Visi kami adalah **platform AI yang berpusat pada manusia**, di mana agen bekerja seperti rekan tepercaya: berkelanjutan, kontekstual, dan aman.  

---

## Daftar Isi
- [Pendahuluan](#pendahuluan)  
  - [Gambaran Umum](#gambaran-umum)  
  - [Motivasi](#motivasi)  
  - [Masalah dengan Persona AI Saat Ini](#masalah-dengan-persona-ai-saat-ini)  
  - [Solusi DoveNest](#solusi-dovenest)  
  - [Karakteristik Utama DoveNest](#karakteristik-utama-dovenest)  
- [Arsitektur DoveNest](#arsitektur-dovenest)  
  - [Tiga Lapisan](#tiga-lapisan)  
  - [Peran Platform Persona AI](#peran-platform-persona-ai)  
  - [Mitra Percakapan AI](#mitra-percakapan-ai)  
  - [Agen AI](#agen-ai)  
  - [Agen Perencana Kooperatif](#agen-perencana-kooperatif)  
- [Kasus Penggunaan](#kasus-penggunaan)  
  - [UC-01: Pair-Programmer Berkelanjutan dengan GitHub (tanpa ganti mode)](#uc-01-pair-programmer-berkelanjutan-dengan-github-tanpa-ganti-mode)  
  - [UC-02: Refaktor Aman Skala Repositori (Batch + Guardrail)](#uc-02-refaktor-aman-skala-repositori-batch--guardrail)  
  - [UC-03: Sinkronisasi Multibahasa Dokumentasi & Buku Putih](#uc-03-sinkronisasi-multibahasa-dokumentasi--buku-putih)  
  - [UC-04: Dari Riset ke PR (Baca Mendalam → Implementasi)](#uc-04-dari-riset-ke-pr-baca-mendalam--implementasi)  
  - [UC-05: Asisten Triage & Reproduksi Bug](#uc-05-asisten-triage--reproduksi-bug)  
  - [UC-06: Portal Pengetahuan atas Kode & Keputusan](#uc-06-portal-pengetahuan-atas-kode--keputusan)  
  - [UC-07: Otomasi Rapat ke Backlog](#uc-07-otomasi-rapat-ke-backlog)  
  - [UC-08: Bootstrap Pipeline Data/Embedding](#uc-08-bootstrap-pipeline-dataembedding)  
  - [UC-09: Tata Kelola Memori & Kontrol Privasi](#uc-09-tata-kelola-memori--kontrol-privasi)  
  - [UC-10: Co-Pilot Runbook On-Call](#uc-10-co-pilot-runbook-on-call)  
- [Roadmap MVP (4–6 Minggu Pertama)](#roadmap-mvp-46-minggu-pertama)  
- [Prinsip Desain Utama Terkait Kasus Penggunaan](#prinsip-desain-utama-terkait-kasus-penggunaan)  
- [Pertanyaan Terbuka](#pertanyaan-terbuka)  

---

## Pendahuluan

### Gambaran Umum
DoveNest mengembangkan **agen perencana kooperatif**: persona AI yang menggabungkan kekuatan **mitra percakapan AI** dan **agen AI** dalam satu kesatuan terintegrasi.

### Motivasi
Kecerdasan buatan semakin kuat dan mengambil peran penting dalam pekerjaan, penelitian, pendidikan, dan kehidupan sehari-hari. Dasarnya adalah **model** yang semakin canggih, yang menjadi fondasi bagi **persona AI**. Persona ini menentukan bagaimana manusia dapat memanfaatkan AI dalam praktik.

### Masalah dengan Persona AI Saat Ini
Persona AI saat ini terbagi dua:
1. **Mitra percakapan** – unggul dalam dialog, berbagi pengetahuan, memahami masalah kompleks, dan ide kreatif.  
2. **Agen** – fokus mengeksekusi tugas spesifik melalui antarmuka & alat.  

Manusia secara alami berpindah antara **perencanaan** dan **aksi**. Pemisahan ini menyebabkan friksi.

### Solusi DoveNest
Karena keduanya didasarkan pada model serupa, tidak ada alasan teknis mengapa satu persona AI tidak bisa melakukan **keduanya**. DoveNest menghadirkan **agen perencana kooperatif** yang menyatukan **pemahaman, perencanaan, dan tindakan**.

### Karakteristik Utama DoveNest
- **Memori persisten** untuk percakapan yang berkesinambungan.  
- **Kompetensi antarmuka** untuk eksekusi tugas.  
- **Interaksi non-linear**: proses panjang berjalan di latar belakang.  
- **Memori kontekstual opsional**: diaktifkan hanya saat diperlukan.  

---

## Arsitektur DoveNest

### Tiga Lapisan
1. **Antarmuka Pengguna** – titik interaksi manusia dengan AI.  
2. **Platform Persona AI** – mengatur konteks, memori, antarmuka, interaksi.  
3. **Infrastruktur Model AI** – menyediakan kecerdasan dan daya komputasi.

### Peran Platform Persona AI
Platform menentukan apa yang benar-benar bisa dilakukan AI. DoveNest menyatukan percakapan & eksekusi dalam satu agen.

### Mitra Percakapan AI
Menjadi teman dialog yang alami & responsif, diperkuat memori.  

### Agen AI
Mengeksekusi tugas via antarmuka; bisa memblokir percakapan jika tidak asinkron.

### Agen Perencana Kooperatif
Mengintegrasikan perencanaan & aksi tanpa friksi. Mendukung proses asinkron & memori fleksibel.

---

## Kasus Penggunaan

### UC-01: Pair-Programmer Berkelanjutan dengan GitHub (tanpa ganti mode)
**Masalah:** Pergantian mode memutus alur percakapan.  
**Visi:** Tetap berdialog sambil IA push commit & CI berjalan di latar belakang.  
**MVP:** Satu repo, satu branch, pipeline lint/fmt/test.  
**KPI:** CI hijau <1 jam, ≥80% PR merged tanpa fix manual.  

### UC-02: Refaktor Aman Skala Repositori (Batch + Guardrail)
**Masalah:** Refaktor besar rentan kesalahan.  
**Visi:** Refaktor terencana, commit kecil, tes memastikan stabilitas.  
**MVP:** Rename sederhana + update call sites.  
**KPI:** Waktu refaktor, tingkat regresi.  

### UC-03: Sinkronisasi Multibahasa Dokumentasi & Buku Putih
**Masalah:** Terjemahan manual cepat menyimpang.  
**Visi:** Sumber tunggal, AI jaga sinkron antar bahasa.  
**MVP:** Sinkronisasi DE↔EN untuk satu halaman.  
**KPI:** Latensi <24h, konsistensi istilah.  

### UC-04: Dari Riset ke PR (Baca Mendalam → Implementasi)
**Masalah:** Konversi spesifikasi ke kode lambat.  
**Visi:** AI baca dokumen, tulis tes, scaffold kode, buat PR.  
**MVP:** Modul kecil dengan unit test lulus.  
**KPI:** Cakupan kriteria, jumlah siklus review.  

### UC-05: Asisten Triage & Reproduksi Bug
**Masalah:** Issue sering tanpa langkah reproduksi.  
**Visi:** AI cluster issue, buat tes gagal minimal, usulkan fix.  
**MVP:** Satu bug → tes gagal + PR fix.  
**KPI:** % issue direpro <1 jam, waktu ke fix pertama.  

### UC-06: Portal Pengetahuan atas Kode & Keputusan
**Masalah:** Alasan keputusan mudah hilang.  
**Visi:** Q&A kode+ADR+PR dengan kutipan.  
**MVP:** Jawaban “Kenapa pakai LanceDB bukan SQLite?” dengan kutipan.  
**KPI:** Akurasi jawaban, efisiensi onboarding.  

### UC-07: Otomasi Rapat ke Backlog
**Masalah:** Keputusan rapat tak terdokumentasi jadi backlog.  
**Visi:** AI ekstrak aksi dari transkrip, buat issue, tautkan dependensi.  
**MVP:** 5–10 issue dari satu transkrip.  
**KPI:** % aksi tercatat, kepuasan stakeholder.  

### UC-08: Bootstrap Pipeline Data/Embedding
**Masalah:** Setup embedding service repetitif.  
**Visi:** AI scaffold microservice Rust+LanceDB dengan CI/CD.  
**MVP:** Service lokal jalan + tes asap.  
**KPI:** Time-to-first-query, latensi p95.  

### UC-09: Tata Kelola Memori & Kontrol Privasi
**Masalah:** Memori persisten bermanfaat tapi rawan.  
**Visi:** Kontrol granular, opt-in, log audit, masa retensi.  
**MVP:** Approve/deny memori per sesi.  
**KPI:** Kepercayaan pengguna, insiden privasi = 0.  

### UC-10: Co-Pilot Runbook On-Call
**Masalah:** Insiden perlu respon cepat.  
**Visi:** AI hubungkan alert ke runbook, jalankan diagnostik, draft postmortem.  
**MVP:** Simulasi insiden → draft postmortem.  
**KPI:** Reduksi MTTR, akurasi saran.  

---

## Roadmap MVP (4–6 Minggu Pertama)
- Minggu 1–2: UC-01 (repo minimal + CI).  
- Minggu 3–4: Sinkronisasi DE↔EN + Portal Pengetahuan.  
- Minggu 5–6: Pilot reproduksi bug.  

---

## Prinsip Desain Utama Terkait Kasus Penggunaan
- Tidak ada ganti mode: percakapan utama, eksekusi asinkron.  
- Human-in-the-loop: persetujuan eksplisit untuk perubahan kode/memori.  
- Proveniens di mana-mana: link ke PR, test, ADR, log.  
- Degradasi elegan: AI tetap merencanakan walau alat tidak tersedia.  
- Guardrail keamanan: sandbox, token least-privilege, aksi tersign.  

---

## Pertanyaan Terbuka
- Model event seragam untuk update asinkron?  
- Antarmuka test standar lintas bahasa?  
- Skema memori portabel lintas proyek & persona?  
