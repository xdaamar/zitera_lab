# Remediasi: Membangun Sistem Logging & Alerting Tangguh

Prinsip utama monitoring keamanan:

## 1. Centralized Log Management (SIEM)
- Kirimkan semua catatan log secara real-time ke penyimpanan terpusat yang terisolasi (seperti Elasticsearch, Splunk, Graylog, atau AWS CloudWatch).
- Pastikan penyimpanan log berstatus *append-only* (hanya bisa menambah, tidak bisa diedit atau dihapus).

## 2. Format Log Terstruktur & Sanitasi Input
- Gunakan format log terstruktur (seperti JSON) dan sanitasi karakter baris baru (`\r`, `\n`) untuk mencegah *Log Injection*.
- Jangan pernah mencatat data pribadi pengguna (seperti nomor kartu kredit atau plain password) ke dalam file log.
