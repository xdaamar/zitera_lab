# Pembahasan Teknis: Fail-Open vs Fail-Closed

## Kode Rentan Contoh (Fail-Open Bug):
```python
def check_access(user_token):
    try:
        # Menghubungi server verifikasi otorisasi eksternal
        response = auth_service.verify(user_token)
        return response.is_valid
    except Exception as e:
        # KESALAHAN FATAL: Jika auth service timeout/error, sistem mengasumsikan valid!
        log.warning(f"Auth service down: {e}")
        return True # Fail-Open (Bypass keamanan!)
```

Bila penyerang sengaja membanjiri server otorisasi dengan trafik tinggi sehingga server tersebut timeout, fungsi di atas akan selalu mengembalikan `True`, membiarkan penyerang masuk sebagai administrator tanpa token yang sah.
