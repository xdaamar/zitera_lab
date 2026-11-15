# Pengenalan: Security Logging & Alerting Failures (OWASP A09:2025)

Dalam dunia pertahanan siber (*cyber defense*), ada adagium terkenal: **"Anda tidak dapat bertahan dari serangan yang tidak dapat Anda lihat."** Banyak organisasi memiliki sistem firewall mutakhir dan enkripsi kuat, namun mengalami kompromi fatal selama berbulan-bulan tanpa disadari (*mean time to detect* / MTTD yang mencapai ratusan hari). **OWASP A09:2025 – Security Logging and Alerting Failures** berfokus pada ketidakmampuan sistem mencatat aktivitas keamanan kritis, kegagalan membunyikan peringatan saat serangan berlangsung, serta ketiadaan jejak audit yang membuat investigasi forensik menjadi mustahil.

Ketika logging dan alerting gagal, insiden keamanan berubah dari gangguan kecil yang dapat segera dibendung menjadi bencana kebocoran data skala penuh.

---

## Mengapa Kategori Ini Sangat Krusial dalam OWASP Top 10 2025?

Laporan ancaman global dari berbagai lembaga forensik digital secara konsisten menunjukkan bahwa penyerang memanfaatkan *telemetry blind spots* (titik buta telemetri):

1. **Eksploitasi Senyap (*Silent Exploitation*)**: Penyerang dapat melakukan ribuan tebakan kredensial (*brute-force* atau *credential stuffing*) atau manipulasi otorisasi tanpa satupun alarm berbunyi di Security Operations Center (SOC).
2. **Eskalasi Hak Akses Siluman**: Perubahan peran pengguna biasa menjadi *Super Admin* yang tidak meninggalkan jejak log membuat administrator resmi tidak memiliki visibilitas atas akun boneka penyerang.
3. **Ketidakmampuan Respons Insiden**: Saat peretasan terungkap ke publik, tim forensik tidak dapat menentukan pintu masuk, data apa saja yang dicuri, atau mesin mana saja yang terinfeksi karena log sistem kosong atau terhapus.
4. **Log Injection & Tampering**: Format log teks biasa yang rentan terhadap karakter baris baru (*CRLF Injection*) dapat dipalsukan oleh penyerang untuk mengaburkan jejak mereka.

---

## Skenario Lab: Zitera SOC Audit & Telemetry Console

Dalam lab ini, Anda akan berhadapan dengan **Zitera SOC Audit & Telemetry Console (Port 8019)**. Layanan ini merepresentasikan modul autentikasi dan administrasi internal sebuah perusahaan:

```
┌─────────────────────────┐          HTTP POST          ┌─────────────────────────┐
│     Untrusted Client    │  ─────────────────────────> │   Zitera SOC Console    │
│  (Attacker / Tester)    │   /api/auth/login           │       (Port 8019)       │
│                         │   /api/admin/role_escalate  │                         │
│                         │                             │ ❌ 0 Alerts Fired       │
│                         │                             │ ❌ No Audit Log Entry   │
└─────────────────────────┘                             └─────────────────────────┘
```

Meskipun sistem menyediakan stream audit log, implementasinya mengalami dua kegagalan fatal:
1. Serangan tebakan kata sandi beruntun pada `/api/auth/login` ditolak dengan status HTTP 401 namun sama sekali tidak dicatat ke log audit dan menghasilkan **0 peringatan SOC**.
2. Operasi eskalasi hak istimewa pada `/api/admin/role_escalate` mengubah status pengguna menjadi administrator tingkat tinggi secara instan tanpa menuliskan satu baris pun ke dalam berkas jejak audit (*silent escalation*).

---

## Sasaran Pembelajaran

Melalui modul ini, Anda akan:
- **Mengidentifikasi Titik Buta Telemetri**: Mengaudit alur kerja aplikasi untuk menemukan peristiwa keamanan kritis yang tidak tercatat.
- **Mendemonstrasikan Eksploitasi Tanpa Jejak**: Menjalankan serangan *brute force* dan eskalasi hak akses tanpa memicu alert pada konsol pengawasan.
- **Merancang Struktur Telemetri Terstruktur**: Mempelajari format log JSON terstruktur yang kaya konteks (IP, user agent, correlation ID, severity).
- **Membangun Aturan Deteksi SIEM**: Merumuskan aturan korelasi dan batas ambang (*thresholds*) untuk mendeteksi anomali perilaku pengguna.
