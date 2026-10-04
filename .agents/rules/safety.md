# ZITERA_LAB Safety Rules

Strictly apply these rules to all tasks in this workspace:

1. PROJECT ROOT BOUNDARY
   Semua operasi destruktif hanya boleh terjadi di dalam repository ZITERA_LAB (`c:\Users\Damar\Documents\project_pribadi\zitera_lab`).

2. NO SYSTEM DELETION
   Jangan pernah menghapus:
   C:\, C:\Windows\, C:\Program Files\, C:\ProgramData\, C:\Users\<user>\Documents\, Desktop, Downloads, registry, system services, drivers, partition/disk, atau path Windows lain di luar repository.

3. NO PRIVILEGE ESCALATION
   Jangan menjalankan RunAs, sudo equivalent, Administrator elevation, atau meminta privilege elevated kecuali user memberikan izin eksplisit untuk kebutuhan yang sudah dijelaskan sebelumnya.

4. NO SYSTEM CONFIGURATION
   Jangan mengubah Windows Features, WSL, Hyper-V, Docker, Firewall rules, Registry, Services, Scheduled Tasks, Drivers, Boot configuration, atau Environment variables global.

5. NO DESTRUCTIVE WILDCARDS
   Dilarang menggunakan perintah seperti `Remove-Item -Recurse -Force`, `rm -rf`, `del /s`, `git clean -fdx`, atau equivalent terhadap path yang belum diverifikasi.

6. PATH VERIFICATION
   Sebelum penghapusan:
   - resolve absolute path
   - verify it is under repository root
   - verify it belongs to explicit deletion allowlist.

7. NO FOLLOWING OUTSIDE BOUNDARY
   Junction, symlink, UNC path, drive-letter path, atau path traversal tidak boleh menyebabkan operasi keluar dari repository.

8. GIT CHECKPOINT
   Sebelum cleanup besar:
   - git status
   - commit/checkpoint
   - record current SHA.

9. LEGACY PRESERVATION
   Kode yang akan dihapus karena obsolete harus tetap recoverable melalui Git history/tag sebelum removal.

10. NO BLIND MASS CLEANUP
    Jangan menghapus "semua file lama" tanpa inventory dan deletion manifest.

11. BUILD MUST BE REPRODUCIBLE
    Build hanya memakai source/dependency yang didefinisikan project.

12. NO SILENT INSTALLATION
    Jangan menginstal dependency/tool/driver ke Windows tanpa kebutuhan yang sudah disetujui.

13. NO UNKNOWN DOWNLOAD + EXECUTE
    Jangan download binary dari URL arbitrary lalu langsung execute.

14. STOP ON SYSTEM TOUCH
    Jika task membutuhkan perubahan sistem Windows yang tidak termasuk desain runtime yang disetujui:
    STOP dan laporkan. Jangan improvisasi.

15. RUNTIME VS BUILD SEPARATION
    Build harus non-destructive terhadap host.
    Runtime boleh membuat data aplikasi per-user yang memang diperlukan, tetapi tidak boleh memodifikasi system-wide configuration.

16. SECURITY FAILURE = STOP
    Jika sandbox tidak dapat dibuat:
    JANGAN fallback ke unsandboxed execution.

17. DELETE ONLY WHAT IS PROVEN OBSOLETE
    Tidak ada penghapusan berdasarkan asumsi.

18. AFTER CLEANUP
    Wajib:
    `git diff --check`, `cargo check/build`, `flutter analyze/test`, project-specific regression.

19. REPORT EVERY DESTRUCTIVE OPERATION
    Semua penghapusan besar harus dicatat dalam sprint report:
    - path
    - alasan
    - bukti obsolete
    - replacement
    - recovery source.

20. NEVER CLAIM SAFE WITHOUT VERIFICATION
    Jika belum diverifikasi: tulis `NOT VERIFIED`.
