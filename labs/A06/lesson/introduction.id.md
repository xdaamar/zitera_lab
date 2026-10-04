# A06: Insecure Design (Desain Arsitektur yang Tidak Aman)

Insecure Design berfokus pada risiko yang berkaitan dengan cacat desain arsitektur sistem (*architectural flaws*). Kategori ini berbeda secara mendasar dari bug implementasi kode.

## Cacat Desain vs Bug Implementasi
- **Bug Implementasi:** Desain arsitekturnya sudah benar, namun pemrogram membuat kesalahan ketik atau salah menerapkan sintaksis kode (*flawed implementation*).
- **Insecure Design:** Penulisan kodenya sempurna tanpa error sintaksis, namun sejak awal logika bisnis dan alur prosesnya memang dirancang tanpa mempertimbangkan model ancaman keamanan (*threat modeling*). Bahkan jika kode ditulis tanpa cacat bug sama sekali, sistem tersebut tetap rentan dibobol karena desain dasarnya memang salah.
