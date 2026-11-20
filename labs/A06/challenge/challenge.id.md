# Simulasi Audit: Cacat Desain State Machine AuthShield Identity

### MISI BRIEFING
Tim Arsitektur Keamanan **AuthShield Identity Technologies Inc.** meminta Anda menguji integritas alur kerja permohonan hak istimewa di `http://127.0.0.1:8016`.

Sistem permohonan izin diduga memiliki kelemahan desain fatal pada mesin status (*State Machine Insecure Design*), di mana endpoint backend menerima status transisi dari klien tanpa memverifikasi tanda tangan persetujuan manajer.

---

### RUANG LINGKUP & TARGET
- **Target URL:** `http://127.0.0.1:8016`
- **Tingkat Keparahan:** High (CVSS 8.2)
- **Vektor Kerentanan:** CWE-840 (Business Logic Errors), CWE-653 (Improper Adherence to Architecture)

---

### OBJEKTIF PENELUSURAN
1. Analisis alur persetujuan permohonan entitas pada `/api/orders`.
2. Lakukan manipulasi transisi status pada permohonan eksekutif bernilai tinggi `#9999` menjadi `APPROVED` atau `DISPATCHED`.
3. Dapatkan flag autoritatif berformat `ZITERA{...}`.
4. Kirimkan (*submit*) flag ke Zitera Lab desktop untuk menyelesaikan tantangan!

---

### FORMAT SUBMISSION
`ZITERA{...}`
