# ZITERA_LAB: CTF CHALLENGE & FLAG VALIDATION STANDARD
**Document Reference:** `PRD/CTF_CHALLENGE_STANDARD.md`  
**Phase:** 24 (CP05 — Checkpoint Push #6)  
**Author:** AI Pair Programmer (Software Craftsmanship Active)  
**Status:** CHALLENGE & GRADER CANONICAL STANDARD  
**Target Platform:** Windows x64  
**Primary Language:** Bahasa Indonesia (Secondary: English-ready architecture)  

---

## 1. Executive Summary & Educational Mission

Setiap tantangan di ZITERA_LAB dirancang sebagai skenario *hands-on* bergaya **Capture-The-Flag (CTF)** yang realistis dan edukatif. Tantangan bukan bertujuan menguji tebakan liar (*guessing game*), melainkan menguji kemampuan pembelajar dalam mentransfer pemahaman konsep yang telah diajarkan pada modul teori dan latihan terpandu (*guided practice*).

Penyelesaian tantangan dibuktikan melalui flag spesifik atau pembuktian status server (*server-side state verification*) yang dinilai secara otomatis oleh mesin *grader* terisolasi.

---

## 2. Format & Standar Flag (Section 8.1)

Format kanonikal flag ZITERA_LAB:
```text
ZITERA{descriptive_snake_case_identifier}
```

*Contoh ilustratif dokumentasi (bukan flag produksi):*
`ZITERA{sample_training_flag}`

### Aturan Keamanan & Integritas Flag
1. **Spesifik & Unik:** Setiap tantangan memiliki flag unik yang tidak boleh digunakan ulang pada tantangan lain.
2. **Divalidasi Sisi Server (*Server-Side Grader*):** Flag diverifikasi oleh runtime engine Rust (`engine/rust/src/labs.rs`) menggunakan perbandingan waktu konstan (*constant-time comparison* `timing_safe_compare`) guna mencegah serangan *timing analysis*.
3. **Pemisahan dari Konten Publik Klien:** Flag tidak boleh dicantumkan pada berkas `lesson.md`, ringkasan tantangan `challenge.md`, maupun petunjuk bantuan `hints.json` yang dikirim ke antarmuka klien.
4. **Kerahasiaan Diagnostik:** Flag tidak boleh dicetak ke dalam log aplikasi normal, log konsol Flutter, laporan commit git, atau dokumen publik.
5. **Anti-Pencarian Teks Polos:** Grader dan parser konten harus memastikan payload flag dirahasiakan sebelum penyelesaian tantangan.

---

## 3. Siklus Hidup Tantangan (*Challenge Lifecycle*) (Section 8.2)

```text
 ┌──────────────────────┐
 │ Challenge Available  │ Pembelajar membuka detail lab; prasyarat terpenuhi.
 └──────────┬───────────┘
            ▼
 ┌──────────────────────┐
 │  Challenge Started   │ Simulator lab dijalankan di port loopback lokal.
 └──────────┬───────────┘
            ▼
 ┌──────────────────────┐
 │  Target Interaction  │ Pembelajar berinteraksi dengan API / UI target lokal.
 └──────────┬───────────┘
            ▼
 ┌──────────────────────┐
 │ Discovers Weakness   │ Mengidentifikasi celah keamanan yang disimulasikan.
 └──────────┬───────────┘
            ▼
 ┌──────────────────────┐
 │    Flag Obtained     │ Mengekstraksi atau membangkitkan flag dari target.
 └──────────┬───────────┘
            ▼
 ┌──────────────────────┐
 │    Flag Submitted    │ Mengirimkan input flag via form UI ke engine IPC.
 └──────────┬───────────┘
            ▼
 ┌──────────────────────┐
 │   Grader Validates   │ Engine mencocokkan flag secara constant-time.
 └──────────┬───────────┘
            ▼
   ┌─────────────────┐
   │ Valid / Invalid │
   └────────┬────────┘
            ▼
 ┌──────────────────────┐
 │  Progress Recorded   │ Progress manager mencatat status solved secara lokal.
 └──────────────────────┘
```

Jika sebuah tantangan dapat diselesaikan hanya dengan menyalin flag dari berkas publik tanpa berinteraksi dengan target simulasi, maka desain tantangan tersebut dinyatakan cacat dan wajib direvisi.

---

## 4. Sistem Petunjuk Progresif (*Progressive Hints*) (Section 8.3)

Setiap tantangan dilengkapi dengan 3 tingkat petunjuk bertahap untuk mencegah kebuntuan (*learner frustration*) tanpa merusak nilai edukasi eksperimen:

| Tingkatan | Nama Tingkat | Tujuan & Konten | Aturan Penalti / Akses |
|---|---|---|---|
| **Tier 1** | **Arah Konseptual (*Orientation*)** | Mengingatkan konsep atau teori utama yang relevan dari lesson (misal: "Ingat kembali bagaimana server membedakan otentikasi vs kepemilikan data"). | Gratis / Tanpa Penalti |
| **Tier 2** | **Area Teknis (*Technical Area*)** | Menunjukkan komponen, parameter URL, atau header HTTP spesifik yang perlu diperiksa (misal: "Periksa parameter numerik invoice pada path URL"). | Penalti Skor Minor (opsional) |
| **Tier 3** | **Petunjuk Sintaks / Arah (*Syntax Clue*)** | Memberikan format masukan atau rentang nilai target tanpa membocorkan flag langsung (misal: "Coba akses indeks administratif seperti ID 42"). | Penalti Waktu / Skor Sedang |
| **Solution** | **Walkthrough Solusi Lengkap** | Penjelasan komprehensif alur penyelesaian langkah-demi-langkah. | **HANYA TERBUKA** setelah tantangan berhasil diselesaikan. |

*Dilarang keras menaruh flag aktual di dalam teks petunjuk.*

---

## 5. Klasifikasi Tingkat Kesulitan (*Difficulty Levels*) (Section 8.4)

### 1. Beginner (Tingkat Dasar)
- Fokus pada satu konsep kerentanan tunggal (contoh: IDOR langsung pada A01, kredensial default pada A02).
- Target dan titik masuk sangat jelas.
- Petunjuk Tier 1–3 bersifat eksplisit dan membimbing.
- Waktu pengerjaan tipikal: 15–20 menit.

### 2. Intermediate (Tingkat Menengah)
- Menggabungkan dua atau lebih konsep terkait (contoh: bypass autentikasi SQL Injection A05, brute-force rate-limiting A07).
- Pembelajar harus menganalisis request, merangkai payload, atau memeriksa respons galat.
- Waktu pengerjaan tipikal: 25–40 menit.

### 3. Advanced (Tingkat Lanjutan)
- Memerlukan penalaran multi-tahap (*multi-step reasoning*) dan korelasi batas kepercayaan (contoh: Insecure Design A06, Deserialization & Integrity Verification A08, Fail-Open Race A10).
- Membutuhkan pemahaman mendalam tentang state machine, kriptografi, dan arsitektur sistem.
- Waktu pengerjaan tipikal: 40–60 menit.

---

## 6. Matriks Uji Validasi Flag (*Flag Testing Requirements*) (Section 8.5)

Setiap tantangan wajib lolos 12 butir uji otomatis berikut:

```text
1. [FLAG_VALID]            : Flag resmi yang benar wajib diterima dengan status success: true.
2. [FLAG_INVALID]          : String flag acak atau salah wajib ditolak dengan feedback edukatif.
3. [FLAG_EMPTY]            : Input string kosong ("") wajib ditolak tanpa memicu exception/crash.
4. [FLAG_WHITESPACE]       : Leading/trailing whitespace wajib ditangani konsisten (trim default).
5. [FLAG_CASE_SENSITIVE]   : Format ZITERA{...} wajib diverifikasi case-sensitive secara deterministik.
6. [FLAG_CROSS_CHALLENGE]  : Flag valid dari lab lain (misal flag A02 dimasukkan ke A01) wajib ditolak.
7. [FLAG_PROGRESS_REJECT]  : Jawaban salah tidak boleh mencatat progres atau mengubah status lab.
8. [FLAG_PROGRESS_IDEMP]   : Pengiriman flag benar berulang kali bersifat idempotent (tidak dobel skor).
9. [FLAG_RESTART_PERSIST]  : Restart aplikasi tidak menghilangkan riwayat lab yang sudah solved.
10. [FLAG_DIAG_MASK]       : Log engine dan error diagnostics tidak boleh mengekspos nilai flag.
11. [FLAG_TAMPER_PROOF]    : Manipulasi state UI Flutter tanpa verifikasi engine tidak dianggap solved.
12. [FLAG_RESET_CLEAN]     : Tombol reset tantangan mengembalikan status simulator ke baseline awal.
```
