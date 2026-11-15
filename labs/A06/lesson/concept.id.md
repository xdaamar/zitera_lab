# Pembahasan Teknis: Anatomi Cacat Desain & Mesin Status (Finite State Machine)

Kerentanan Insecure Design berakar pada kegagalan mendefinisikan dan menegakkan **invarian keamanan (*security invariants*)** di seluruh siklus hidup data dan status aplikasi.

---

## 1. Pemodelan Mesin Status Bisnis (Finite State Machine / FSM)
Dalam sistem transaksi dan alur kerja (*procurement workflows*), status pesanan idealnya bergerak mengikuti graf berarah yang deterministik:

```text
[ DRAFT ] ──▶ (Submit Pesanan) ──▶ [ PENDING_APPROVAL ] ──▶ (Tanda Tangan Manajer) ──▶ [ APPROVED ] ──▶ [ DISPATCHED ]
   │                                      ▲
   │ (Pesanan < $500)                     │ (Pesanan >= $500: Wajib Persetujuan)
   └──────────────────────────────────────┘
```

### Invarian Bisnis yang Diinginkan:
1. Pesanan bernilai rendah (< $500) dapat langsung beralih ke status `APPROVED` secara otomatis.
2. Pesanan bernilai tinggi ($\ge$ $500) **WAJIB** masuk ke status `PENDING_APPROVAL` dan hanya dapat diubah ke `APPROVED` oleh akun yang memiliki peran `ExecutiveManager`.
3. Pesanan tidak boleh langsung melompat dari `DRAFT` atau `PENDING_APPROVAL` ke `DISPATCHED` tanpa melalui status `APPROVED`.

---

## 2. Titik Lemah Cacat Desain pada Endpoint API
Perhatikan cuplikan logika backend yang rentan di bawah ini:
```python
# KODE DENGAN CACAT DESAIN ARSITEKTUR
@app.route("/api/orders/<order_id>/transition", methods=["POST"])
def transition_order(order_id):
    payload = request.get_json()
    target_state = payload.get("target_state")
    action = payload.get("action")

    order = orders.get(order_id)
    if not order:
        return jsonify({"error": "Order not found"}), 404

    # CACAT DESAIN: Mengizinkan klien mendikte status tujuan secara bebas!
    if target_state:
        order["status"] = target_state
    elif action == "force_dispatch":
        order["status"] = "DISPATCHED"

    return jsonify({"status": "updated", "order": order})
```

### Mengapa Ini Merupakan Cacat Desain Fatal?
1. **Penerimaan Parameter Status Arbitrer**: Endpoint mempercayai nilai `target_state` yang dikirimkan oleh klien tanpa memeriksa graf transisi yang diizinkan (*allowed transition matrix*).
2. **Ketiadaan Validasi Prasyarat Nilai**: Tidak ada logika di server yang memeriksa apakah pesanan dengan nilai besar (misal: order #9999 senilai $50.000) sudah memiliki tanda tangan persetujuan manajer sebelum dikirimkan (*DISPATCHED*).
3. **Ketiadaan Verifikasi Peran Otoritatif**: Siapa pun yang dapat mengirimkan request HTTP POST dapat memicu aksi `force_dispatch`, mengabaikan pemisahan tugas (*separation of duties*).

---

## 3. Anti-Pattern Desain Perangkat Lunak Populer
- **Client-Driven State Pattern**: Memberikan wewenang kepada aplikasi frontend untuk menentukan urutan langkah transaksi.
- **Happy Path Syndrome**: Tim perekayasa hanya merancang sistem untuk skenario normal di mana pengguna selalu mengikuti instruksi antarmuka, tanpa memikirkan apa yang terjadi jika pengguna mengirim request API di luar urutan (*out-of-order execution*).
- **Missing Negative Test Cases**: Pengujian perangkat lunak hanya memverifikasi *"apakah pesanan sah bisa diproses?"*, bukan *"apakah pesanan $50.000 bisa diselesaikan tanpa persetujuan manajer?"*.
