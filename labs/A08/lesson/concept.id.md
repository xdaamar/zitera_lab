# Konsep Mendalam: Kegagalan Integritas Perangkat Lunak dan Data

Integritas (*Integrity*) adalah pilar fundamental dari triad keamanan informasi CIA (Confidentiality, Integrity, Availability). Dalam rekayasa perangkat lunak, integritas menjamin bahwa kode sumber, biner yang dapat dieksekusi, pustaka pihak ketiga, dan dokumen konfigurasi sistem tetap berada dalam kondisi **asli, tidak termodifikasi, dan dapat dibuktikan asalnya (*authentic*)** sejak ditandatangani oleh pembuat resmi.

Ketika mekanisme verifikasi integritas tidak diimplementasikan atau dirancang secara cacat, musuh dapat menyisipkan muatan berbahaya ke dalam sistem tanpa perlu membobol kata sandi atau menemukan bug memori tingkat rendah.

---

## 1. Perbedaan Mendasar: Enkripsi vs Hash vs Tanda Tangan Digital

Banyak pengembang keliru mengasumsikan bahwa mengenkripsi data melalui HTTPS/TLS otomatis menyelesaikan masalah integritas data. Faktanya, masing-masing primitif kriptografi memiliki tujuan berbeda:

| Primitif Kriptografi | Tujuan Utama | Apa yang Dijamin? | Apa yang TIDAK Dijamin? |
| :--- | :--- | :--- | :--- |
| **Enkripsi Simetris / TLS** | Kerahasiaan (*Confidentiality*) | Data tidak dapat dibaca oleh pihak ketiga saat transit di jaringan. | Tidak menjamin bahwa data yang dikirim oleh klien tepercaya atau bebas dari malware. |
| **Kriptografis Hash (SHA-256)** | Deteksi Perubahan Tidak Sengaja | Menghasilkan sidik jari unik dari data. | Tidak membuktikan *siapa* pembuat hash (penyerang dapat membuat hash baru untuk file palsu). |
| **Tanda Tangan Digital (Ed25519/RSA)** | Integritas, Otentisitas & Non-Repudiasi | Membuktikan bahwa payload ditandatangani oleh pemegang kunci privat tepercaya dan tidak pernah diubah. | Kunci privat harus dijaga kerahasiaannya di luar jangkauan penyerang. |

```
Plaintext Data ───> [ SHA-256 Hash ] ───> [ Enkripsi Kunci Privat ] ───> Tanda Tangan Digital
                                                                                │
                                                                                ▼
Penerima: Verifikasi Tanda Tangan dengan Kunci Publik ───────────> [ VALID / PALSU ]
```

---

## 2. Taksonomi Kegagalan Integritas (CWE Mapping)

OWASP A08:2025 mengonsolidasikan sejumlah Common Weakness Enumerations (CWE) utama yang berkaitan dengan integritas:

### A. CWE-494: Download of Code Without Integrity Check
Aplikasi mengunduh pembaruan executable, skrip ekstensi, atau modul dinamis melalui jaringan tanpa memverifikasi tanda tangan digital asimetris atau checksum yang ditandatangani. Contoh: Pembaruan aplikasi via HTTP polos atau HTTPS tanpa *certificate pinning* dan tanda tangan biner.

### B. CWE-502: Deserialization of Untrusted Data
Menerima struktur objek yang diserialisasi (seperti Python `pickle`, Java serialized objects, PHP `unserialize()`, atau tag objek Ruby YAML) dari entitas yang tidak tepercaya. Selama proses rekonstruksi objek, method berbahaya (seperti `__reduce__` di Python atau gadget chains di Java) dieksekusi otomatis oleh runtime.

### C. Client-Side Integrity Assertion (Kerentanan Skenario Lab)
Kesalahan arsitektural fatal di mana backend mendelegasikan pemeriksaan integritas kepada klien. Server menerima atribut seperti:
```json
{
  "signature_verified": false,
  "signer": "external"
}
```
dan langsung memproses muatan tersebut. Ini merupakan pelanggaran prinsip bahwa **setiap asersi integritas harus diverifikasi secara kriptografis oleh server itu sendiri menggunakan kunci publik atau sertifikat terpercaya**.

---

## 3. Framework Integritas Industri Modern

Untuk memerangi serangan integritas pada skala global, industri keamanan mengadopsi standar pertahanan berikut:

1. **The Update Framework (TUF)**: Spesifikasi standar keamanan pembaruan software yang tahan terhadap kompromi kunci privat parsial, serangan replay, dan pembekuan pembaruan (*freeze attacks*). TUF digunakan secara luas di Docker Content Trust dan ekosistem Python pip.
2. **SLSA (Supply-chain Levels for Software Artifacts)**: Kerangka kerja bertingkat (Level 1 hingga 4) yang menjamin integritas kode dari commit Git, runner build yang terisolasi, hingga artefak biner yang ditandatangani dengan *provenance* yang tidak dapat disangkal.
3. **Sigstore & Cosign**: Infrastruktur tanda tangan digital modern tanpa kunci statis (*keyless signing*) yang memanfaatkan sertifikat OIDC berumur pendek dan transparansi log (*transparency log - Rekor*) untuk biner dan kontainer container.
