# 03 // BUILD, TEST & WORKFLOW COMMANDS

All commands assume PowerShell on Windows from the repository root:
`c:\Users\Damar\Documents\project_pribadi\zitera_lab`

*(For full Rust engine & lab binary build procedures, see [`docs/build.md`](../docs/build.md).)*

---

## 1. Frontend TypeScript Check & Vite Build

```powershell
npm --prefix "ui/tauri" run build
```
- Runs `tsc -b && vite build`.
- Outputs bundled frontend assets to `ui/tauri/dist/`.

---

## 2. Automated Paywall, Integrity & Emoji Test Suite

```powershell
npx tsx scratch/test_ai_overview_paywall.ts
```
- Verifies:
  - Fuzzy semantic normalizer & typo tolerance
  - Structured AI Overview generation across labs
  - Anti-spoiler guardrails & Clue Level 3 paywall
  - Zero-emoji enforcement across core UI files
  - Pure monthly pricing (`70k / 270k / 470k`) & no annual plans
  - Finite Tactical Question Tree (no freeform `<input type="text">`)
  - Tier lab access gating (`trial`, `foundation` 8-lab cap, `pro`, `vip`)
- Current baseline: **79 Passed, 0 Failed**.

---

## 3. Build Desktop App Executable (`app.exe`)

Ensure `app.exe` is not currently running before copying over it, then run:

```powershell
# 1. Build latest frontend first
npm --prefix "ui/tauri" run build

# 2. Build Tauri release binary & update ui/tauri/run/app.exe
$env:Path = "C:\Users\Damar\.cargo\bin;$env:Path"
Set-Location "ui\tauri\src-tauri"
cargo build --release
if ($LASTEXITCODE -eq 0) {
    Copy-Item "target\release\app.exe" -Destination "..\run\app.exe" -Force
}
```

### Launch Built App Directly
```powershell
& "ui\tauri\run\app.exe"
```

### Build Full Windows Installer (NSIS / MSI) for Distribution
```powershell
$env:Path = "C:\Users\Damar\.cargo\bin;$env:Path"
npm --prefix "ui/tauri" run tauri -- build
```

---

## 4. Full System & Rust Engine Verification

```powershell
# Run Rust engine + catalog + lab manifests + Tauri UI verification
.\dev_internal\scripts\verify_all.ps1

# Additional hardening & phase tests
.\dev_internal\scripts\test_phase2_hardening.ps1
.\dev_internal\scripts\test_phase3_phase4.ps1
```

---

## 5. Development Hot-Reload Mode

```powershell
$env:Path = "C:\Users\Damar\.cargo\bin;$env:Path"
npm --prefix "ui/tauri" run dev:tauri
```
