# Panduan Remediasi: Rekayasa Kode Defensif

Untuk memitigasi kerentanan **Python Pickle & YAML Unsafe Object Deserialization** secara tuntas, tim pengembang perangkat lunak wajib menerapkan kontrol defensif berlapis.

---

### Komparasi Kode: Rentan vs Aman

#### [RENTAN] Kode Rentan (Vulnerable Pattern):
```javascript
// Server mempercayai input klien secara naif
app.get('/api/admin/dashboard', (req, res) => {
    const role = req.headers['x-user-role'] || 'user';
    if (role === 'admin') {
        return res.json({ secret: 'UNRESTRICTED_ACCESS' });
    }
    return res.status(403).json({ error: 'Forbidden' });
});
```

#### [AMAN] Kode Kebal & Terlindungi (Secure Patched Pattern):
```javascript
# Gunakan format pertukaran data yang aman seperti JSON:
import json
data = json.loads(untrusted_string)
# Hindari pickle.loads() atau gunakan yaml.safe_load(untrusted_stream)
```

---

### 4 Pilar Pertahanan Berlapis (Defense-in-Depth)
1. **Server-Side Authorization**: Otorisasi harus diverifikasi secara independen di sisi server berdasarkan token sesi terenkripsi.
2. **Strict Input Sanitization**: Seluruh parameter dan header dari peramban wajib dianggap untrusted data.
3. **Audit Logging**: Catat setiap percobaan manipulasi header atau eskalasi wewenang yang mencurigakan ke log audit terpusat.
4. **Automated Security Tests**: Sertakan unit test dan integration test untuk memverifikasi penolakan akses ilegal pada setiap build pipeline.
