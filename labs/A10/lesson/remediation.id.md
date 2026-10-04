# Remediasi: Terapkan Prinsip Fail-Safe Defaults

Aturan penanganan exception yang aman:

## 1. Selalu Terapkan Fail-Closed (Tolak Akses Saat Error)
Bila terjadi kegagalan tak terduga dalam verifikasi keamanan, asumsi default harus selalu menolak akses (`return False` atau `abort(403)`):

```python
# IMPLEMENTASI AMAN: FAIL-CLOSED
def check_access(user_token):
    try:
        response = auth_service.verify(user_token)
        return response.is_valid
    except Exception as e:
        log.error(f"Auth verification failed: {e}")
        # AMAN: Menolak akses secara eksplisit saat terjadi error
        return False
```

## 2. Jangan Bocorkan Stack Trace ke Klien
Tangkap exception tak terduga dan kembalikan pesan error umum yang aman bagi pengguna publik (misal *"An error occurred. Please contact support."*), sementara detail teknis dicatat di file log internal.
