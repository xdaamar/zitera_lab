# Bedah Teknis: IDOR & Otorisasi Berbasis Objek

## 1. Memahami Insecure Direct Object References (IDOR)
Insecure Direct Object Reference (IDOR) terjadi ketika sebuah aplikasi web mengekspos referensi langsung ke objek internal database (seperti kunci primer basis data: `id=1`, `wire_id=42`, `account_no=882194`) dan mempercayai input klien tanpa memverifikasi hak kepemilikan di sisi server.

### Mengapa Developer Sering Terjebak dalam Celah Ini?
1. **UI-Centric Assumption (Asumsi Keliru Frontend)**: Developer sering mengira bahwa karena tombol di layar Alice hanya bertuliskan *"Lihat Transaksi Kawat #1"*, Alice tidak akan pernah tahu atau mencoba mengakses transaksi nomor 42. Padahal penyerang tidak peduli tampilan UI; mereka memodifikasi URL atau request HTTP langsung via address bar atau Burp Suite!
2. **Rasa Aman Palsu dari `@login_required`**: Framework modern menyediakan decorator autentikasi. Ketika fungsi API sudah dipasangi pengecekan sesi login, developer merasa pekerjaannya sudah selesai, padahal otorisasi level objek (*Object-Level Authorization*) belum pernah ditulis.
3. **Pola Query Database yang Cacat**:
   - [RENTAN] **Query Rentan IDOR:**
     ```sql
     SELECT * FROM wire_transfers WHERE id = :target_id;
     ```
     *(Server hanya memeriksa apakah transaksi itu ada, siapa pun yang meminta akan diberikan datanya!)*
   - [AMAN] **Query Aman Kebal IDOR:**
     ```sql
     SELECT * FROM wire_transfers WHERE id = :target_id AND user_id = :session_user_id;
     ```
     *(Server secara mutlak membatasi pencarian hanya pada data milik user yang sedang aktif).*

---

## 2. Pola Pikir Peretas (The Bug Hunter Mindset)
Bagaimana seorang bug hunter profesional memburu IDOR pada target nyata seperti ApexFin Corporate Treasury?

```text
┌────────────────────────────────────────────────────────────────────────┐
│                      ALUR INVESTIGASI IDOR                             │
├────────────────────────────────────────────────────────────────────────┤
│ 1. RECON & MAPPING PARAMETER                                           │
│    • Buka DevTools (F12) -> Network Tab                                │
│    • Identifikasi parameter tertebak: id, ref_id, wire_id, doc_num     │
│    • Catat format: Sekuensial (1, 2, 3) vs Hash vs UUID               │
├────────────────────────────────────────────────────────────────────────┤
│ 2. HIPOTESIS TAMPERING                                                 │
│    • Bagaimana jika id=1 saya ubah menjadi id=2 atau id=42?            │
│    • Apakah server memvalidasi kepemilikan sesi saya?                  │
├────────────────────────────────────────────────────────────────────────┤
│ 3. ANALISIS RESPON (DIFFING)                                           │
│    • Jika HTTP 403 / 401: Kontrol akses ditegakkan (Aman).             │
│    • Jika HTTP 200 dengan data user lain: IDOR TERKONFIRMASI!          │
│    • Dokumentasikan Proof-of-Concept untuk pelaporan bug bounty.       │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 3. Dua Dimensi Broken Access Control

```text
                 [ ADMINISTRATOR / MARCUS REID ]
                                ▲
                                │  Otorisasi Vertikal (Privilege Escalation)
                                │  (Contoh: Alice mengakses /admin atau /audit)
                                ▼
[ ANALIS ALICE ] ◄──────────────────────────────► [ SPESIALIS BOB ]
              Otorisasi Horizontal (IDOR / BOLA)
              (Contoh: Alice membaca voucher transfer /wire/2 milik Bob)
```

1. **Otorisasi Horizontal (Horizontal Access Control):** Terjadi antar pengguna yang memiliki tingkat hak akses setara. Alice dan Bob sama-sama berstatus *Standard Operator*, namun Alice dapat mengintip voucher transfer milik Bob.
2. **Otorisasi Vertikal (Vertical Access Control):** Terjadi ketika pengguna biasa berhasil melompati batas peran dan membaca voucher rahasia kepatuhan milik Marcus Reid (VP Controller) yang memiliki ID #42 (*Classified Sovereign Wire*).

---

## 4. Contoh Anatomi Protokol HTTP pada ApexFin Treasury

### 1. Request Normal (Milik Alice - User ID #1)
```http
GET /wire/1 HTTP/1.1
Host: 127.0.0.1:8011
Cookie: session_user=user_sess_1

HTTP/1.1 200 OK
Content-Type: text/html

<!-- Menampilkan Voucher TRX-APEX-1001 sebesar $4,950.00 milik Alice -->
```

### 2. Request Eksploitasi IDOR (Alice Meminta Data Rahasia Controller Marcus Reid)
```http
GET /wire/42 HTTP/1.1
Host: 127.0.0.1:8011
Cookie: session_user=user_sess_1

HTTP/1.1 200 OK
Content-Type: text/html

<!-- Server rentan mengembalikan voucher kawat bernilai $99,999.00 beserta flag rahasia! -->
```
Perhatikan bahwa cookie sesi yang dikirimkan tetap milik **Alice (`user_sess_1`)**, tetapi server ApexFin mengembalikan dokumen rahasia milik **Marcus Reid** tanpa penolakan HTTP 403 Forbidden.
