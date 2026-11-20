# Remediasi & Rekayasa Pertahanan Fail-Closed

Untuk mengatasi kerentanan fatal pada **OWASP A10:2025 – Mishandling of Exceptional Conditions**, arsitektur sistem harus mengadopsi prinsip desain **Fail-Closed secara non-negotiable**: **jika sebuah sistem keamanan menghadapi kesalahan runtime, timeout jaringan hulu, atau payload tak terduga, status otorisasi harus tetap DITOLAK (*DENY*)**.

---

## 1. Menegakkan Invarian Keamanan Fail-Closed

Dalam rekayasa perangkat lunak defensif, status otorisasi tidak boleh berupa variabel mengambang yang dapat dibelokkan menjadi `True` di dalam blok penanganan kesalahan.

### Aturan Arsitektur Fail-Closed:
1. **Inisialisasi ke State Teraman**: Variabel penentu izin harus diinisialisasi sebagai `authorized = False` sejak awal fungsi.
2. **Hanya Ubah ke `True` Jika Seluruh Validasi Lulus**: Tidak boleh ada cabang eksepsi (*except block*) yang menetapkan `authorized = True`.
3. **Gunakan Kode Status HTTP Semantik**:
   - Jika data input rusak atau bertipe salah: kembalikan `HTTP 400 Bad Request`.
   - Jika verifikasi otentikasi gagal: kembalikan `HTTP 401 Unauthorized` atau `HTTP 403 Forbidden`.
   - Jika upstream verifier timeout: kembalikan `HTTP 504 Gateway Timeout` dengan status akses tertolak, bukan `HTTP 200 OK`.

```
                    ┌─────────────────────────┐
                    │     authorized = False  │ ──> Nilai Awal Non-Negotiable
                    └────────────┬────────────┘
                                 │
                 ┌───────────────┴───────────────┐
                 ▼                               ▼
       [ Validasi Sukses ]               [ Terjadi Exception / Timeout ]
                 │                               │
                 ▼                               ▼
         authorized = True               authorized = False (TETAP TERKUNCI)
         HTTP 200 OK                     HTTP 504 Gateway Timeout
                                         [SECURITY] AKSES DITOLAK!
```

---

## 2. Refactor Backend: Penanganan Eksepsi Spesifik & Fail-Closed

Berikut adalah implementasi Python/Flask yang telah direfaktor untuk mengamankan `Zitera Payment & Security Gate`:

```python
import logging
from flask import Flask, request, jsonify

app = Flask(__name__)
logger = logging.getLogger("security_gate")

class UpstreamServiceTimeout(Exception):
    """Exception khusus saat upstream gateway mengalami latensi berlebih."""
    pass

class MalformedPayloadException(Exception):
    """Exception saat format payload melanggar skema kontrak."""
    pass

def query_upstream_verifier(token: str) -> bool:
    """
    Simulasi pemanggilan upstream service verifikasi KYC/fraud.
    """
    if not token or not isinstance(token, str):
        raise MalformedPayloadException("Token harus berupa string non-empty yang valid.")

    if token == "super_authorized_token":
        return True
    return False

@app.route('/api/gate/verify', methods=['POST'])
def secure_gate_verify():
    # 1. INVARIAN KEAMANAN: Status otorisasi default selalu FALSE
    authorized = False

    data = request.get_json(silent=True)
    if data is None:
        return jsonify({"error": "Payload JSON tidak valid.", "authorized": False}), 400

    try:
        # Cek simulasi error
        if data.get("simulate_exception") or "trigger_error" in data:
            raise UpstreamServiceTimeout("Upstream KYC provider tidak merespons dalam 2000ms.")

        token = data.get("token")

        # 2. Panggil verifikator hulu
        authorized = query_upstream_verifier(token)

        if authorized:
            return jsonify({
                "authorized": True,
                "status": "cleared",
                "message": "Transaksi berhasil diotorisasi."
            }), 200
        else:
            return jsonify({
                "authorized": False,
                "status": "denied",
                "message": "Kredensial atau token verifikasi tidak valid."
            }), 403

    except MalformedPayloadException as err:
        # Penanganan kesalahan input klien (Client Error)
        logger.warning(f"Permintaan ditolak karena skema rusak: {err}")
        return jsonify({
            "authorized": False,
            "status": "error",
            "message": f"Input tidak valid: {str(err)}"
        }), 400

    except UpstreamServiceTimeout as err:
        # Penanganan kegagalan layanan hulu (Server/Upstream Error)
        # TETAP FAIL-CLOSED: Jangan pernah meloloskan transaksi!
        logger.error(f"Kegagalan upstream terdeteksi: {err}")
        return jsonify({
            "authorized": False,
            "status": "upstream_unavailable",
            "message": "Layanan verifikasi sedang tidak tersedia. Transaksi ditangguhkan untuk keamanan."
        }), 504

    except Exception as unexpected_err:
        # Penanganan kondisi luar biasa tak terduga
        logger.critical(f"Kesalahan internal tak terduga: {unexpected_err}", exc_info=True)
        return jsonify({
            "authorized": False,
            "status": "internal_error",
            "message": "Terjadi kesalahan internal. Akses ditolak secara default."
        }), 500
```

---

## 3. Strategi Antrean Transaksi Tertunda (*Asynchronous Resilient Queueing*)

Untuk industri perbankan dan e-commerce di mana menolak pelanggan secara instan dapat merugikan bisnis, solusi yang aman adalah **antrean asinkron**:

1. Saat upstream verifier mengalami gangguan, simpan transaksi ke dalam antrean terisolasi (`transaction_state = "PENDING_VERIFICATION"`).
2. Kembalikan respons kepada pengguna: *"Transaksi Anda sedang dalam proses peninjauan keamanan. Kami akan mengonfirmasi via notifikasi."*
3. Pekerja latar belakang (*background worker*) mencoba memvalidasi ulang transaksi saat upstream kembali sehat menggunakan *idempotency key*.
4. Barang atau dana **hanya dilepaskan setelah verifikasi benar-benar berhasil lulus**.

---

## 4. Checklist Audit Penanganan Eksepsi

Gunakan checklist ini saat melakukan *code review* pada gerbang otorisasi dan kontrol akses:

- [ ] Seluruh variabel penentu otorisasi diinisialisasi ke status paling aman (`False` / `Deny`).
- [ ] Tidak ada blok `except` atau `catch` yang menetapkan status otorisasi ke `True` atau memberikan akses bypass.
- [ ] Kegagalan layanan hulu (upstream timeouts) menghasilkan kode status HTTP 503/504 dengan status `authorized: false`.
- [ ] Eksepsi diklasifikasikan secara hierarkis dan spesifik (bukan sekadar `except Exception`).
- [ ] Setiap eksepsi yang tertangkap dicatat dalam log audit keamanan lengkap dengan konteks insiden.
- [ ] Respon HTTP kepada pengguna akhir tidak membocorkan stack trace internal atau detail database.
- [ ] Pengujian penetrasi mencakup skenario *fault injection* (input null, timeout simulasi, paket terpotong).
