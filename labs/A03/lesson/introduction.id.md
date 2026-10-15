# A03: Software Supply Chain Failures (Kegagalan Rantai Pasok Perangkat Lunak)

## 1. Apa itu Software Supply Chain Failure?
Dalam era rekayasa perangkat lunak modern, pengembang hampir tidak pernah menulis seluruh basis kode dari nol. Rata-rata aplikasi enterprise kontemporer terdiri dari 80% hingga 90% komponen pihak ketiga: pustaka sumber terbuka (*open-source libraries*), package manager (seperti npm, PyPI, Cargo, Maven, NuGet), base image container, plugin build pipeline, dan modul orkestrasi CI/CD.

OWASP Top 10:2025 A03 menyoroti kerentanan yang menyusup ke dalam aplikasi bukan karena kelalaian logika kode internal pengembang, melainkan akibat kompromi pada ekosistem rantai pasok hulu (*upstream supply chain*). Ketika satu komponen dependensi yang dipercaya disusupi atau dipalsukan, seluruh aplikasi turunan yang mengimpor pustaka tersebut akan mewarisi kerentanan atau muatan berbahaya (*backdoor*) secara otomatis.

## 2. Mengapa Ancaman Rantai Pasok Kritis di 2025/2026?
Serangan terhadap rantai pasok perangkat lunak mengalami peningkatan eksponensial karena penyerang menyadari efisiensi skala:
- **Dampak Berantai (*Blast Radius*) Luas**: Menginfeksi satu pustaka populer dengan jutaan unduhan mingguan memungkinkan penyerang menyusup ke ribuan enterprise sekaligus tanpa perlu membobol firewall masing-masing korban secara langsung.
- **Kepercayaan Tersirat (*Implicit Trust*)**: Lingkungan CI/CD otomatis sering kali diberi hak istimewa (*privileged credentials*) untuk mengakses registry produksi, cloud hosting, dan database internal.
- **Kompleksitas Dependensi Transitif**: Satu paket utama dapat menarik puluhan hingga ratusan pustaka turunan (*transitive dependencies*). Pengembang sering kali tidak menyadari pustaka apa saja yang secara tidak langsung dieksekusi saat proses instalasi.

## 3. Insiden Bersejarah Rantai Pasok
- **Kasus `event-stream` (npm)**: Pustaka populer diambil alih oleh penyerang melalui rekayasa sosial terhadap maintainer asli, lalu disisipi kode pencuri dompet cryptocurrency.
- **Insiden SolarWinds Orion**: Penyerang menyusupkan trojan Sunburst ke dalam sistem kompilasi resmi vendor sebelum pembaruan perangkat lunak ditandatangani dan didistribusikan ke puluhan ribu organisasi pemerintah dan korporasi.
- **Backdoor `xz-utils` (CVE-2024-3094)**: Penyerang menghabiskan waktu bertahun-tahun membangun reputasi sebagai kontributor tepercaya sebelum menyisipkan backdoor bertahap pada mekanisme kompresi Linux guna membajak autentikasi SSH daemon.

## 4. Sasaran Pembelajaran di Lab A03
Melalui lab praktikum ini, Anda akan mempelajari:
1. Memahami mekanisme lockfile (`package-lock.json`) dan bagaimana integritas dependensi dicatat.
2. Mengidentifikasi tanda-tanda paket asing dan hook instalasi mencurigakan (`postinstall_script`).
3. Menganalisis eksfiltrasi token rahasia pipeline melalui dependensi yang tidak diverifikasi.
4. Menerapkan strategi pertahanan berbasis Software Bill of Materials (SBOM) dan pembatasan izin siklus hidup paket.
