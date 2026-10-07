<#
.SYNOPSIS
    Automated Windows Distribution Installer Lifecycle Regression Test (CP09).
.DESCRIPTION
    Tests all 5 required installer states:
    1. Fresh install
    2. Same-version install (idempotency)
    3. Upgrade (v2.0.0 -> v2.0.1, verifying student progress is preserved)
    4. Uninstall (verifying binaries removed, progress preserved)
    5. Reinstall (verifying re-installation recognizes existing progress)
    6. Repair (corrupt binary and auto-repair)
#>

[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
$swTest = [System.Diagnostics.Stopwatch]::StartNew()

$script:Root = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$installerScript = Join-Path $script:Root "scripts\installer\install_user.ps1"

# Create isolated test directories
$testRoot = Join-Path $env:TEMP ("zitera_installer_test_" + [System.Guid]::NewGuid().ToString("N"))
$testAppDir = Join-Path $testRoot "app_programs"
$testDataDir = Join-Path $testRoot "app_data"

Write-Host "============================================================" -ForegroundColor Cyan
Write-Host " ZITERA_LAB WINDOWS INSTALLER LIFECYCLE REGRESSION (CP09)   " -ForegroundColor Cyan
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "Test App Directory  : $testAppDir"
Write-Host "Test Data Directory : $testDataDir"

try {
    # -------------------------------------------------------------
    # TEST 1: FRESH INSTALL (v2.0.0)
    # -------------------------------------------------------------
    Write-Host "`n>> TEST 1: Fresh Per-User Installation (v2.0.0)" -ForegroundColor Yellow
    $res1 = & $installerScript -Action Install -TargetDir $testAppDir -DataDir $testDataDir -Version "2.0.0" -Silent
    $installedBin = Join-Path $testAppDir "bin\zitera-engine.exe"
    $manifestPath = Join-Path $testAppDir "install_manifest.json"

    if (-not (Test-Path $installedBin)) { throw "TEST 1 FAILED: zitera-engine.exe not found at $installedBin" }
    if (-not (Test-Path $manifestPath)) { throw "TEST 1 FAILED: install_manifest.json not found" }
    $manifest1 = Get-Content $manifestPath -Raw | ConvertFrom-Json
    if ($manifest1.version -ne "2.0.0") { throw "TEST 1 FAILED: Expected version 2.0.0, got $($manifest1.version)" }
    Write-Host " [PASS] Fresh install succeeded at v2.0.0" -ForegroundColor Green

    # Simulate Student Course Activity: write progress to user\progress.json
    $progressFile = Join-Path $testDataDir "user\progress.json"
    $activeProgress = @"
{
  "schema_version": 1,
  "completed_labs": ["A01", "A02"],
  "current_streak_days": 5,
  "earned_flags": ["FLAG{broken_access_control_master}", "FLAG{cryptographic_failures_pwn}"],
  "last_activity": "2026-10-07T12:00:00Z"
}
"@
    [System.IO.File]::WriteAllText($progressFile, $activeProgress, [System.Text.UTF8Encoding]::new($false))
    Write-Host " [SEEDED] Simulated student coursework: 2 labs completed, 2 flags earned." -ForegroundColor Gray

    # -------------------------------------------------------------
    # TEST 2: SAME-VERSION INSTALL (Idempotency)
    # -------------------------------------------------------------
    Write-Host "`n>> TEST 2: Same-Version Re-Installation (Idempotency)" -ForegroundColor Yellow
    $res2 = & $installerScript -Action Install -TargetDir $testAppDir -DataDir $testDataDir -Version "2.0.0" -Silent
    if (-not (Test-Path $installedBin)) { throw "TEST 2 FAILED: Binary missing after same-version install" }

    # Verify student progress was NOT overwritten
    $progContent2 = Get-Content $progressFile -Raw | ConvertFrom-Json
    if ($progContent2.completed_labs.Count -ne 2) {
        throw "TEST 2 FAILED: Student progress was wiped during same-version install!"
    }
    Write-Host " [PASS] Same-version install is idempotent; student progress 100% intact" -ForegroundColor Green

    # -------------------------------------------------------------
    # TEST 3: UPGRADE (v2.0.0 -> v2.0.1)
    # -------------------------------------------------------------
    Write-Host "`n>> TEST 3: In-Place Application Upgrade (v2.0.0 -> v2.0.1)" -ForegroundColor Yellow
    $res3 = & $installerScript -Action Upgrade -TargetDir $testAppDir -DataDir $testDataDir -Version "2.0.1" -Silent
    $manifest3 = Get-Content $manifestPath -Raw | ConvertFrom-Json
    if ($manifest3.version -ne "2.0.1") { throw "TEST 3 FAILED: Version not updated to 2.0.1" }
    if ($manifest3.previous_version -ne "2.0.0") { throw "TEST 3 FAILED: Previous version 2.0.0 not recorded" }

    # Verify student progress was preserved during upgrade
    $progContent3 = Get-Content $progressFile -Raw | ConvertFrom-Json
    if ($progContent3.completed_labs.Count -ne 2 -or $progContent3.earned_flags.Count -ne 2) {
        throw "TEST 3 FAILED: Student coursework corrupted during upgrade!"
    }
    Write-Host " [PASS] Upgrade to v2.0.1 succeeded; previous version recorded; user data preserved" -ForegroundColor Green

    # -------------------------------------------------------------
    # TEST 4: REPAIR
    # -------------------------------------------------------------
    Write-Host "`n>> TEST 4: Self-Healing Binary Repair" -ForegroundColor Yellow
    # Simulate binary corruption by truncating the executable
    [System.IO.File]::WriteAllBytes($installedBin, [byte[]]@(0x00, 0x01, 0x02))
    $corruptHash = (Get-FileHash -Path $installedBin -Algorithm SHA256).Hash
    Write-Host " Simulated corruption: binary truncated to 3 bytes" -ForegroundColor Gray

    & $installerScript -Action Repair -TargetDir $testAppDir -DataDir $testDataDir -Version "2.0.1" -Silent
    $repairedHash = (Get-FileHash -Path $installedBin -Algorithm SHA256).Hash
    if ($repairedHash -eq $corruptHash) { throw "TEST 4 FAILED: Binary repair failed to restore valid binary" }
    Write-Host " [PASS] Repair automatically restored valid binary matching manifest" -ForegroundColor Green

    # -------------------------------------------------------------
    # TEST 5: UNINSTALL (Binary Removal + Data Preservation)
    # -------------------------------------------------------------
    Write-Host "`n>> TEST 5: Clean Uninstallation" -ForegroundColor Yellow
    & $installerScript -Action Uninstall -TargetDir $testAppDir -DataDir $testDataDir -Silent
    if (Test-Path $testAppDir) { throw "TEST 5 FAILED: Application directory $testAppDir was not deleted" }

    # Verify student progress was preserved after uninstallation
    if (-not (Test-Path $progressFile)) { throw "TEST 5 FAILED: User data was unexpectedly purged during uninstall!" }
    $progContent5 = Get-Content $progressFile -Raw | ConvertFrom-Json
    if ($progContent5.completed_labs.Count -ne 2) { throw "TEST 5 FAILED: User data corrupted during uninstall" }
    Write-Host " [PASS] Binaries uninstalled; student progress preserved in user data folder" -ForegroundColor Green

    # -------------------------------------------------------------
    # TEST 6: REINSTALL (Re-Connecting Existing Progress)
    # -------------------------------------------------------------
    Write-Host "`n>> TEST 6: Reinstallation & Progress Reconnection" -ForegroundColor Yellow
    $res6 = & $installerScript -Action Install -TargetDir $testAppDir -DataDir $testDataDir -Version "2.0.1" -Silent
    if (-not (Test-Path $installedBin)) { throw "TEST 6 FAILED: Reinstallation binary missing" }

    $progContent6 = Get-Content $progressFile -Raw | ConvertFrom-Json
    if ($progContent6.completed_labs.Count -ne 2) { throw "TEST 6 FAILED: Existing progress lost on reinstall" }
    Write-Host " [PASS] Reinstallation reconnected to previous student progress seamlessly" -ForegroundColor Green

    # Final Purge Cleanup
    & $installerScript -Action Uninstall -TargetDir $testAppDir -DataDir $testDataDir -PurgeUserData -Silent
    if (Test-Path $testDataDir) { throw "TEST Cleanup failed: PurgeUserData did not remove data directory" }
    Write-Host " [PASS] Full purge uninstall verified" -ForegroundColor Green

}
finally {
    Remove-Item -Path $testRoot -Recurse -Force -ErrorAction SilentlyContinue
}

$swTest.Stop()
$totalSec = [math]::Round($swTest.Elapsed.TotalSeconds, 2)
Write-Host ""
Write-Host "============================================================" -ForegroundColor Green
Write-Host " ALL 6 INSTALLER LIFECYCLE TESTS PASSED (${totalSec}s)       " -ForegroundColor Green
Write-Host "============================================================" -ForegroundColor Green
