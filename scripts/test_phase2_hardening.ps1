# Phase 2 Live Runtime Hardening & Failure Injection Suite
$ErrorActionPreference = "Stop"

$WorkspaceRoot = Split-Path -Parent $PSScriptRoot
Set-Location $WorkspaceRoot

$engineBin = "$WorkspaceRoot\engine\rust\target\release\zitera-engine.exe"
if (-not (Test-Path $engineBin)) {
    Write-Error "zitera-engine.exe not found at $engineBin"
    exit 1
}

Write-Host "`n========================================================" -ForegroundColor Cyan
Write-Host "   ZITERA_LAB // Phase 2 Hardening & Injection Matrix   " -ForegroundColor Cyan
Write-Host "========================================================" -ForegroundColor Cyan

# P2-TEST-1: Port conflict handling on Port 8011 (FAIL-0006)
Write-Host "`n[P2-TEST-1] Injecting port conflict on Port 8011 (A01)..." -ForegroundColor Yellow
$listener = [System.Net.Sockets.TcpListener]::new([System.Net.IPAddress]::Loopback, 8011)
$listener.Start()
try {
    $out = & $engineBin --json lab start A01 | ConvertFrom-Json
    if ($out.success -eq $false -and $out.error.message -like "*already in use*") {
        Write-Host "  -> Port Conflict Guard (8011): PASS (Correctly blocked start with code $($out.error.code))" -ForegroundColor Green
    } else {
        Write-Error "Port conflict on 8011 was NOT caught properly! Output: $(ConvertTo-Json $out)"
        exit 1
    }
} finally {
    $listener.Stop()
}

# P2-TEST-2: Port conflict handling on Port 8015 (A05)
Write-Host "`n[P2-TEST-2] Injecting port conflict on Port 8015 (A05)..." -ForegroundColor Yellow
$listener15 = [System.Net.Sockets.TcpListener]::new([System.Net.IPAddress]::Loopback, 8015)
$listener15.Start()
try {
    $out15 = & $engineBin --json lab start A05 | ConvertFrom-Json
    if ($out15.success -eq $false -and $out15.error.message -like "*already in use*") {
        Write-Host "  -> Port Conflict Guard (8015): PASS (Correctly blocked start with code $($out15.error.code))" -ForegroundColor Green
    } else {
        Write-Error "Port conflict on 8015 was NOT caught properly! Output: $(ConvertTo-Json $out15)"
        exit 1
    }
} finally {
    $listener15.Stop()
}

# P2-TEST-3: Path traversal injection protection
Write-Host "`n[P2-TEST-3] Injecting path traversal attacks (../../secret, ..\..\cmd)..." -ForegroundColor Yellow
$badIds = @("../../secret", "..\..\cmd", "../A01", "A01/../../root", "A01;whoami", "A01|calc")
foreach ($bad in $badIds) {
    $statusOut = & $engineBin --json lab status $bad | ConvertFrom-Json
    $startOut = & $engineBin --json lab start $bad | ConvertFrom-Json
    if ($statusOut.data.status -eq "INVALID_ID" -and $startOut.success -eq $false) {
        Write-Host "  -> Traversal guard for '$bad': PASS (Blocked)" -ForegroundColor Green
    } else {
        Write-Error "Path traversal for '$bad' was not safely rejected!"
        exit 1
    }
}

# P2-TEST-4: Corrupted manifest handling (FAIL-0008)
Write-Host "`n[P2-TEST-4] Testing invalid manifest detection..." -ForegroundColor Yellow
$tempLabDir = "$WorkspaceRoot\labs\CORRUPT_TEST"
New-Item -ItemType Directory -Path $tempLabDir -Force | Out-Null
Set-Content -Path "$tempLabDir\manifest.json" -Value "{ broken_json: [invalid"
try {
    $corruptOut = & $engineBin --json lab status CORRUPT_TEST | ConvertFrom-Json
    if ($corruptOut.data.status -eq "INVALID_MANIFEST") {
        Write-Host "  -> Corrupted manifest handling: PASS (Reported INVALID_MANIFEST cleanly)" -ForegroundColor Green
    } else {
        Write-Error "Corrupted manifest was not recognized! Output: $(ConvertTo-Json $corruptOut)"
        exit 1
    }
} finally {
    Remove-Item -Path $tempLabDir -Recurse -Force -ErrorAction SilentlyContinue
}

# P2-TEST-5: Docker daemon down graceful degradation (FAIL-0001 / FAIL-0002)
Write-Host "`n[P2-TEST-5] Verifying Docker daemon error reporting..." -ForegroundColor Yellow
$daemonDownStart = & $engineBin --json lab start A01 | ConvertFrom-Json
if ($daemonDownStart.success -eq $false -and $daemonDownStart.error.recoverable -eq $true) {
    Write-Host "  -> Daemon failure handling: PASS (Reported recoverable error without crashing: $($daemonDownStart.error.code))" -ForegroundColor Green
} else {
    Write-Error "Daemon failure handling did not return recoverable error structure!"
    exit 1
}

# P2-TEST-6: Idempotent status and stop
Write-Host "`n[P2-TEST-6] Testing operation idempotency..." -ForegroundColor Yellow
$st1 = & $engineBin --json lab status A01 | ConvertFrom-Json
$st2 = & $engineBin --json lab status A01 | ConvertFrom-Json
if ($st1.data.status -eq $st2.data.status -and $st1.data.port -eq $st2.data.port) {
    Write-Host "  -> Idempotent lab status: PASS ($($st1.data.status))" -ForegroundColor Green
} else {
    Write-Error "Status calls produced non-idempotent results!"
    exit 1
}

Write-Host "`n========================================================" -ForegroundColor Cyan
Write-Host "   ALL PHASE 2 HARDENING & INJECTION TESTS PASSED!      " -ForegroundColor Green
Write-Host "========================================================" -ForegroundColor Cyan
