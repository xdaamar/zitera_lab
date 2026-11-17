# A06: Insecure Design — Cacat Desain Arsitektur & Logika Bisnis

Insecure Design menempati peringkat ke-6 (**#6**) dalam daftar risiko web **OWASP Top 10:2025**. Kategori ini merepresentasikan cacat mendasar dalam arsitektur sistem, pemodelan ancaman (*threat modeling*), dan perancangan alur logika bisnis, yang tidak dapat diperbaiki hanya dengan penulisan kode yang rapi.

---

### 1. Alasan Kuat / Mengapa Ini Sangat Penting (Why It Matters)
Sebuah sistem dapat memiliki kode yang 100% bebas dari bug memori, terbebas dari SQL Injection, dan lolos uji linter dengan sempurna, namun tetap hancur total karena arsitektur alur logikanya cacat sejak awal:

- **Perbedaan Krusial: Bug Implementasi vs Cacat Desain**:
  - **Implementation Bug**: Arsitekturnya benar, tetapi programmer salah menulis sintaks kode (misalnya lupa tanda kutip query atau salah ketik nama variabel).
  - **Insecure Design**: Konsep arsitekturnya memang cacat sejak awal. Mengizinkan klien memberi tahu server apa status dokumen berikutnya tanpa verifikasi tanda tangan digital adalah cacat arsitektur. Tidak ada jumlah penulisan kode yang dapat memperbaiki arsitektur yang memang dirancang secara tidak aman.
- **Kerugian Finansial Logika Bisnis Nyata**:
  - **Bypass Transisi Status Pengadaan (Procurement Fraud)**: Pada berbagai platform B2B dan enterprise ERP, manipulasi parameter status dari `DRAFT` langsung menjadi `APPROVED` memungkinkan karyawan curang mengesahkan pengeluaran dana perusahaan jutaan dolar tanpa tanda tangan direksi keuangan.
  - **Manipulasi Harga E-Commerce**: Desain checkout yang memvalidasi harga hanya di antarmuka frontend (JavaScript), sehingga penyerang dapat mengirimkan nilai pesanan bernilai negatif atau melompati gerbang pembayaran langsung ke status `PAID`.

---

### 2. Gambaran Sistem yang Dihadapi (Target Commercial Architecture)
Dalam praktikum ini, Anda mengaudit platform tata kelola identitas enterprise: **AuthShield Identity (Enterprise SSO & Access Management)** yang beroperasi pada port `8016`.

Arsitektur AuthShield Identity mencakup:
- **Access Governance Dashboard**: Konsol antarmuka permohonan hak istimewa (*Resource Entitlement Provisioning Requests*) untuk karyawan.
- **Aturan Bisnis (Intended Business Policy)**:
  - Permohonan di bawah $500 disetujui otomatis.
  - Permohonan bernilai tinggi ($\ge \$500$, seperti akses kluster GPU AI dan kluster datacenter Tier-0 eksekutif) **wajib** melalui persetujuan tanda tangan digital manajer eksekutif.
- **Cacat Desain Arsitektur (Architectural Defect)**:
  Endpoint transisi REST backend `/api/orders/<id>/transition` secara buta menerima payload JSON dari klien berupa `{"target_state": "APPROVED"}` atau `{"target_state": "DISPATCHED"}` tanpa menegakkan invarian mesin status (*State Machine Invariants*) di sisi server!

---

### 3. Sasaran, Temuan & Eksekusi (Search, Find, and Execute)
Metodologi audit Insecure Design:

1. **Apa yang Dicari (What to Search)**:
   - Alur proses multi-tahap: registrasi, checkout, permohonan izin, atau pengadaan barang.
   - Endpoint transisi status: `/api/orders/<id>/transition`, `/api/workflow/advance`.
2. **Apa Temuan yang Menandakan Celah (Vulnerability Indicators)**:
   - Permohonan nomor `#9999` (Executive Tier-0 Datacenter Infrastructure Clearance - senilai $50,000) masih berstatus `DRAFT`.
   - Backend tidak menolak payload yang langsung meminta status `APPROVED` atau `DISPATCHED` tanpa menyertakan token tanda tangan persetujuan manajer.
3. **Eksekusi Eksploitasi Terukur (Strategic Execution)**:
   - Kirimkan permintaan HTTP POST manipulasi status pada pesanan `#9999`:
     ```bash
     curl -X POST http://127.0.0.1:8016/api/orders/9999/transition \
       -H "Content-Type: application/json" \
       -d '{"target_state": "APPROVED"}'
     ```
   - Amati respons server yang memberikan flag autoritatif `ZITERA{...}` atas keberhasilan membuktikan cacat desain logika bisnis.

---

### 4. Pondasi Kuat: Penegakan Invarian Mesin Status
Sistem yang aman harus menerapkan *Finite State Machine (FSM)* yang kaku di tingkat backend:
- Status `DRAFT` **hanya** diizinkan bertransisi ke `PENDING_APPROVAL`.
- Transisi dari `PENDING_APPROVAL` ke `APPROVED` **hanya** dapat dipicu oleh aksi yang ditandatangani secara kriptografis oleh akun dengan peran *Executive Approver*.
- Semua transisi ilegal lainnya harus menghasilkan penolakan mutlak (*Strict Fail-Closed Rejection*).
