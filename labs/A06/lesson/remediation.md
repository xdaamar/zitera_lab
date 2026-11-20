# Panduan Remediasi: Rekayasa Arsitektur Aman & Mesin Status Otoritatif

Untuk mengeliminasi risiko Insecure Design secara mendasar, terapkan prinsip arsitektur **Secure-by-Design** dan pertahanan berlapis (*defense-in-depth*) berikut:

---

## 1. Terapkan Threat Modeling Sejak Fase Desain Awal
Sebelum menulis satu baris kode pun, lakukan pemodelan ancaman (*threat modeling*) menggunakan metodologi **STRIDE**:
- **Spoofing**: Apakah identitas pemanggil request divalidasi secara kriptografis?
- **Tampering**: Apakah parameter penting (seperti harga, diskon, atau status) dapat diubah oleh klien?
- **Repudiation**: Apakah setiap transisi status penting dicatat ke dalam audit trail yang tidak dapat disangkal?
- **Information Disclosure**: Apakah data rahasia bocor melalui respons status?
- **Denial of Service**: Apakah ada batasan pemanggilan API untuk mencegah kehabisan sumber daya?
- **Elevation of Privilege**: Apakah pengguna biasa dapat memicu aksi persetujuan manajerial?

---

## 2. Implementasikan Authoritative Finite State Machine (FSM)
Jangan pernah mengizinkan klien frontend mengirimkan string `target_state` secara langsung. Buat mesin status terstruktur di mana transisi status hanya dapat dipicu melalui tindakan semantik yang sah:

### Contoh Implementasi Python Aman:
```python
class ProcurementOrder:
    ALLOWED_TRANSITIONS = {
        "DRAFT": ["PENDING_APPROVAL", "CANCELLED"],
        "PENDING_APPROVAL": ["APPROVED", "REJECTED"],
        "APPROVED": ["DISPATCHED"],
        "DISPATCHED": ["DELIVERED"],
        "CANCELLED": [],
        "REJECTED": []
    }

    def __init__(self, order_id, amount, owner):
        self.id = order_id
        self.amount = amount
        self.owner = owner
        self.status = "DRAFT"

    def submit_order(self, user_session):
        if self.status != "DRAFT":
            raise ValueError("Hanya pesanan DRAFT yang dapat disubmit.")

        # Penegakan Invarian Bisnis:
        if self.amount < 500:
            self.status = "APPROVED"  # Auto-approve pesanan kecil
        else:
            self.status = "PENDING_APPROVAL"  # Wajib persetujuan manajer

    def approve_order(self, user_session):
        if self.status != "PENDING_APPROVAL":
            raise ValueError("Pesanan tidak dalam status menunggu persetujuan.")

        # Validasi Peran Otoritatif (Pemisahan Tugas):
        if user_session.get("role") != "ExecutiveManager":
            raise PermissionError("Hanya ExecutiveManager yang berhak menyetujui pesanan ini.")

        self.status = "APPROVED"

    def dispatch_order(self, user_session):
        if self.status != "APPROVED":
            raise ValueError("Pesanan belum disetujui, tidak dapat dikirim.")

        self.status = "DISPATCHED"
```

---

## 3. Terapkan Prinsip Pemisahan Tugas (Separation of Duties / SoD)
- Pembuat pesanan (*requester*) dilarang keras menyetujui pesanannya sendiri, terlepas dari peran yang ia miliki di sistem.
- Tindakan bernilai tinggi wajib menerapkan mekanisme otorisasi ganda (*Dual-Control / Two-Man Rule*).

---

## 4. Validasi Invarian di Lapisan Domain Entity
- Jangan mengandalkan validasi di lapisan antarmuka pengguna (HTML/React/Flutter).
- Seluruh aturan batas nilai (misal: saldo tidak boleh negatif, diskon tidak boleh melebihi nilai total barang) wajib ditegakkan secara mutlak di dalam model domain server.

---

## 5. Audit Logging yang Tidak Dapat Dimanipulasi
Setiap perubahan status transaksi wajib mencatat:
- Identitas pengguna yang melakukan transisi.
- Waktu presisi (timestamp UTC).
- Status awal dan status akhir.
- Alasan atau referensi dokumen pendukung.
