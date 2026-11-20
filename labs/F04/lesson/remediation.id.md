# F04: Remediasi & Penulisan Kode Aman (Secure Coding)

Untuk mencegah kerentanan ini terjadi pada aplikasi produksi di dunia nyata, terapkan prinsip-prinsip pertahanan berikut:

### 🛡️ 1. Prinsip Least Privilege & Server-Side Enforcement
- Jangan pernah mengandalkan validasi di sisi peramban (JavaScript client-side).
- Selalu periksa izin pengguna (*permission check*) di lapisan handler backend sebelum data diproses atau dikembalikan.

### 🛡️ 2. Validasi & Sanitasi Masukan Ketat
- Terapkan skema validasi tipe data dan batasan nilai (*allowlisting*).
- Tolak karakter atau pola masukan berbahaya sebelum mencapai query database atau parser sistem berkas.

### 🛡️ 3. Penggunaan Library Standar & Teruji
- Gunakan framework keamanan resmi yang telah mengimplementasikan mitigasi secara otomatis (seperti token CSRF bawaan, ORM parameterized query, dan middleware otentikasi).
- Lakukan security code review dan automated vulnerability testing pada pipeline CI/CD.
