# Panduan Remediasi: Rekayasa Kode Defensif

Untuk memitigasi kerentanan **Multi-Step Workflow Step Skipping** secara tuntas, tim pengembang perangkat lunak wajib menerapkan kontrol defensif berlapis.

---

### Komparasi Kode: Rentan vs Aman

#### ❌ Kode Rentan (Vulnerable Pattern):
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

#### ✅ Kode Kebal & Terlindungi (Secure Patched Pattern):
```javascript
// Verifikasi state machine alur kerja di setiap endpoint:
const state = await db.getWorkflowState(userId);
if (state.currentStep < 3 || !state.idVerified) {
    return res.status(400).json({ error: 'Prasyarat verifikasi KYC belum terpenuhi.' });
}
```

---

### 4 Pilar Pertahanan Berlapis (Defense-in-Depth)
1. **Server-Side Authorization**: Otorisasi harus diverifikasi secara independen di sisi server berdasarkan token sesi terenkripsi.
2. **Strict Input Sanitization**: Seluruh parameter dan header dari peramban wajib dianggap untrusted data.
3. **Audit Logging**: Catat setiap percobaan manipulasi header atau eskalasi wewenang yang mencurigakan ke log audit terpusat.
4. **Automated Security Tests**: Sertakan unit test dan integration test untuk memverifikasi penolakan akses ilegal pada setiap build pipeline.
