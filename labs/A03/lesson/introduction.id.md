# A03: Software Supply Chain Failures (Kegagalan Rantai Pasok Perangkat Lunak)

## Apa itu Software Supply Chain Failure?
Dalam rekayasa perangkat lunak modern, pengembang jarang menulis 100% kode dari nol. Sekitar 80% hingga 90% dari keseluruhan kode aplikasi biasanya terdiri dari library pihak ketiga (*open-source libraries*), package manager (seperti npm, PyPI, Cargo, Maven), pipeline otomatisasi CI/CD, dan image container.

OWASP A03:2025: Software Supply Chain Failures membahas kerentanan keamanan yang masuk ke dalam sistem bukan dari kode buatan tim pengembang sendiri, melainkan disusupkan melalui dependensi pihak ketiga, plugin build pipeline yang terinfeksi, atau repositori paket hulu (*upstream repository*) yang disusupi peretas.

## Mengapa Kerentanan Ini Sangat Berbahaya?
Ketika sebuah library publik yang diunduh jutaan kali disusupi malware atau backdoor (contoh nyata kasus: `event-stream`, `SolarWinds`, atau `xz-utils`), setiap perusahaan yang mengompilasi atau menggunakan library tersebut secara otomatis ikut terinfeksi tanpa mereka sadari.
