# Remediasi: Menerapkan Secure by Design

Cara mengatasi Insecure Design:

## 1. Terapkan Threat Modeling Sejak Tahap Awal
Gunakan metodologi seperti STRIDE atau PASTA pada tahap arsitektur sebelum satu baris kode pun ditulis:
- **S**poofing (Penyamaran identitas)
- **T**ampering (Manipulasi data)
- **R**epudiation (Penyangkalan tindakan)
- **I**nformation Disclosure (Kebocoran informasi)
- **D**enial of Service (Kelumpuhan layanan)
- **E**levation of Privilege (Peningkatan hak akses)

## 2. Tegakkan State Machine yang Ketat di Sisi Server
Pastikan setiap transisi status (misal: *Pending -> Paid -> Shipped*) divalidasi oleh state machine server yang tidak dapat dilewati secara sepihak oleh klien.
