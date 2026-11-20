# Remediasi & Arsitektur Pertahanan Integritas Software

Untuk memitigasi **OWASP A08:2025 – Software and Data Integrity Failures**, arsitektur sistem harus mengadopsi prinsip dasar kriptografi: **tidak ada kode, biner, atau konfigurasi runtime yang boleh dieksekusi tanpa verifikasi tanda tangan digital asimetris publik yang valid**.

---

## 1. Implementasi Tanda Tangan Kunci Publik Asimetris (Ed25519)

Penggunaan algoritma tanda tangan digital modern kurva eliptik seperti **Ed25519 (EdDSA)** memberikan performa tinggi, ukuran kunci dan tanda tangan yang ringkas (kunci publik 32 byte, signature 64 byte), serta resistansi tinggi terhadap serangan *side-channel*.

### Pola Tanda Tangan Terpisah (*Detached Signature Pattern*)

Dalam pola ini, setiap paket rilis atau pembaruan konfigurasi terdiri dari dua file:
1. `payload.json`: Berisi data konfigurasi atau biner mentah.
2. `payload.json.sig`: Berisi tanda tangan digital yang dibuat oleh *build server* menggunakan kunci privat terisolasi.

```
       [ Build / CI Pipeline Server ]
       Kunci Privat Ed25519 (Tersimpan di HSM / KMS)
                      │
                      ▼
         Tanda Tangani payload.json
                      │
         ┌────────────┴────────────┐
         ▼                         ▼
   payload.json             payload.json.sig
         │                         │
         └────────────┬────────────┘
                      ▼
            [ Jaringan Publik ]
                      │
                      ▼
            [ Zitera Edge Device ]
       Kunci Publik Ed25519 Tersemat di Firmware
                      │
                      ▼
         Verifikasi Kriptografis
       ├─ VALID  ──> Terapkan ke Memori
       └─ INVALID ─> TOLAK & BUNYIKAN ALARM!
```

---

## 2. Refactor Kode Backend: Ingesti Aman dengan Validasi Ed25519

Berikut adalah implementasi Python menggunakan pustaka `cryptography` yang memvalidasi integritas dan otentisitas payload sebelum mengubah status runtime:

```python
import base64
import json
from flask import Flask, request, jsonify
from cryptography.hazmat.primitives.asymmetric import ed25519
from cryptography.exceptions import InvalidSignature

app = Flask(__name__)

# Kunci publik tepercaya yang disematkan secara aman saat pembuatan citra firmware
# Hex representasi dari kunci publik Ed25519 resmi Zitera Root Authority
TRUSTED_ROOT_PUBLIC_KEY_HEX = "3d4017c3e843895a92b70aa74d1b7ebc9c982ccf2ec4968cc0cd55f12af4660c"
trusted_public_key = ed25519.Ed25519PublicKey.from_public_bytes(
    bytes.fromhex(TRUSTED_ROOT_PUBLIC_KEY_HEX)
)

active_config = {
    "version": "1.0.0",
    "params": {"telemetry_level": "standard", "enforce_mtls": True}
}

@app.route('/api/deploy_update', methods=['POST'])
def secure_deploy_update():
    data = request.get_json(silent=True) or {}

    # 1. Ekstrak data konfigurasi mentah dan tanda tangan terpisah
    config_data = data.get("config")
    signature_b64 = data.get("signature")

    if not config_data or not signature_b64:
        return jsonify({
            "error": "Permintaan tidak lengkap: atribut 'config' dan 'signature' wajib disertakan."
        }), 400

    try:
        signature_bytes = base64.b64decode(signature_b64)
        # Serialisasi konsisten (canonical JSON) agar representasi byte identik
        canonical_bytes = json.dumps(config_data, sort_keys=True, separators=(',', ':')).encode('utf-8')

        # 2. Verifikasi tanda tangan kriptografis menggunakan kunci publik resmi
        trusted_public_key.verify(signature_bytes, canonical_bytes)

    except (InvalidSignature, ValueError) as err:
        # Peringatan Keamanan: Upaya penyusupan atau kerusakan data terdeteksi!
        app.logger.error(f"Kegagalan verifikasi tanda tangan digital: {err}")
        return jsonify({
            "error": "Validasi integritas gagal: Tanda tangan digital tidak valid atau payload termutasi."
        }), 403

    # 3. Validasi skema parameter sebelum aplikasi ke runtime
    params = config_data.get("params", {})
    if "debug_backdoor" in params or not params.get("enforce_mtls", False):
        return jsonify({"error": "Kebijakan keamanan menolak konfigurasi tidak aman."}), 422

    # 4. Terapkan pembaruan yang telah diverifikasi
    global active_config
    active_config = {
        "version": config_data.get("version"),
        "signature_verified": True,
        "signer": "zitera-root-authority",
        "params": params
    }

    return jsonify({
        "status": "applied",
        "message": "Konfigurasi terverifikasi berhasil diterapkan ke memori operasional.",
        "active_config": active_config
    }), 200
```

---

## 3. Deserialisasi Aman & Pertahanan Insecure Deserialization

Selain penandatanganan payload, proses parsing data harus dibentengi:

1. **Gunakan Format Data Netral**: Hindari pustaka serialisasi bahasa-spesifik seperti Python `pickle`, Java Serialization API, PHP `serialize()`, atau Ruby `Marshal`. Gunakan format netral seperti JSON, Protocol Buffers, atau FlatBuffers.
2. **Validasi Skema Kuat (Strict Schema Typing)**: Gunakan alat validasi seperti Pydantic di Python, Zod di TypeScript, atau protobuf schema untuk memastikan tipe data setiap field sesuai sebelum diproses.
3. **Prinsip Least Privilege Runtime**: Proses penerima pembaruan tidak boleh berjalan dengan hak akses `root` atau administrator sistem.

---

## 4. Checklist Audit Integritas Software

Gunakan checklist ini saat merancang atau mengaudit pipeline pembaruan dan ingesti data:

- [ ] Seluruh endpoint penerima pembaruan mewajibkan tanda tangan digital asimetris (Ed25519/RSA).
- [ ] Kunci publik verifikasi disematkan secara aman (*hardware-backed root of trust*) dan tidak dapat diubah oleh panggilan API biasa.
- [ ] Backend memverifikasi tanda tangan secara independen; tidak pernah mempercayai boolean dari klien.
- [ ] Data yang ditandatangani diserialisasi secara kanonikal (*deterministic JSON sorting*) sebelum pembuatan atau verifikasi tanda tangan.
- [ ] Paket yang gagal verifikasi tanda tangan langsung ditolak (HTTP 403) dan dicatat sebagai insiden keamanan.
- [ ] Komponen CI/CD menggunakan alat seperti Sigstore / Cosign untuk mencatat metadata provenance pembuatan software.
