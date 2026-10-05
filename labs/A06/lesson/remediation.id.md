# Remediasi: Menerapkan Secure by Design

Cara mengatasi Insecure Design:

## 1. Terapkan Threat Modeling Sejak Tahap Awal
Gunakan metodologi seperti STRIDE atau PASTA pada tahap arsitektur sebelum satu baris kode pun ditulis:
- Spoofing (Penyamaran identitas)
- Tampering (Manipulasi data)
- Repudiation (Penyangkalan tindakan)
- Information Disclosure (Kebocoran informasi)
- Denial of Service (Kelumpuhan layanan)
- Elevation of Privilege (Peningkatan hak akses)

## 2. Tegakkan State Machine yang Ketat di Sisi Server
Pastikan setiap transisi status (misal: *Pending -> Paid -> Shipped*) divalidasi oleh state machine server yang tidak dapat dilewati secara sepihak oleh klien.
