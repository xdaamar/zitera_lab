# ZITERA_LAB End-to-End System Verification Script

Write-Host "==================================================" -ForegroundColor Cyan
Write-Host "   ZITERA_LAB // Full System Verification Pipeline" -ForegroundColor Cyan
Write-Host "==================================================" -ForegroundColor Cyan

$WorkspaceRoot = Split-Path -Parent $PSScriptRoot
Set-Location $WorkspaceRoot

# Ensure tool paths are available in session
$cargoBin = "C:\Users\Damar\.cargo\bin"
$flutterBin = "C:\src\flutter\bin"
$dockerBin = "C:\Users\Damar\AppData\Local\Programs\DockerDesktop\resources\bin"
$env:Path = "$cargoBin;$flutterBin;$dockerBin;$env:Path"

# 1. Rust Engine Check & Build
Write-Host "`n[1/5] Building & Testing Rust Engine (zitera-engine)..." -ForegroundColor Yellow
Set-Location "$WorkspaceRoot\engine\rust"
cargo check --quiet
if ($LASTEXITCODE -ne 0) { Write-Error "Rust cargo check failed!"; exit 1 }
cargo clippy --all-targets --all-features -- -D warnings
if ($LASTEXITCODE -ne 0) { Write-Error "Rust cargo clippy failed!"; exit 1 }
cargo build --release --quiet
if ($LASTEXITCODE -ne 0) { Write-Error "Rust cargo build release failed!"; exit 1 }
Write-Host "  -> Rust Engine Build & Clippy: PASS" -ForegroundColor Green

# 2. Rust CLI Execution & Security Checks
Write-Host "`n[2/5] Testing zitera-engine CLI, JSON Output & Security Validation..." -ForegroundColor Yellow
$DoctorOut = & ".\target\release\zitera-engine.exe" --json doctor | ConvertFrom-Json
if ($DoctorOut.success -eq $true -and $DoctorOut.action -eq "doctor") {
    Write-Host "  -> Engine doctor JSON IPC: PASS" -ForegroundColor Green
} else {
    Write-Error "Engine doctor JSON IPC output invalid!"
    exit 1
}

$LabsOut = & ".\target\release\zitera-engine.exe" --json lab list | ConvertFrom-Json
if ($LabsOut.success -eq $true -and $LabsOut.data.Count -ge 2) {
    Write-Host "  -> Engine lab list discovery ($($LabsOut.data.Count) labs found): PASS" -ForegroundColor Green
} else {
    Write-Error "Engine lab list output invalid!"
    exit 1
}

# Traversal rejection check
$TraversalCheck = & ".\target\release\zitera-engine.exe" --json lab status "../../secret" | ConvertFrom-Json
if ($TraversalCheck.data.status -eq "INVALID_ID") {
    Write-Host "  -> Engine path-traversal guard: PASS (rejected cleanly with INVALID_ID)" -ForegroundColor Green
} else {
    Write-Error "Engine path-traversal guard failed!"
    exit 1
}

# Lab lifecycle commands (install / update idempotency)
$InstallCheck = & ".\target\release\zitera-engine.exe" --json lab install A01 | ConvertFrom-Json
$UpdateCheck = & ".\target\release\zitera-engine.exe" --json lab update A01 | ConvertFrom-Json
if ($InstallCheck.success -eq $true -and $UpdateCheck.success -eq $true) {
    Write-Host "  -> Engine lab install & update lifecycle: PASS" -ForegroundColor Green
} else {
    Write-Error "Engine lab install or update check failed!"
    exit 1
}

# 3. Central Catalog Check
Write-Host "`n[3/5] Verifying Central Catalog (catalog/catalog.json)..." -ForegroundColor Yellow
Set-Location $WorkspaceRoot
$Catalog = Get-Content ".\catalog\catalog.json" | ConvertFrom-Json
if ($Catalog.schema_version -ge 1 -and $Catalog.labs.Count -ge 2) {
    Write-Host "  -> Central Catalog Schema: PASS ($($Catalog.labs.Count) registered labs)" -ForegroundColor Green
} else {
    Write-Error "Central Catalog validation failed!"
    exit 1
}

# 4. Reference Labs Manifest & Docker Check
Write-Host "`n[4/5] Checking Reference Labs (A01 & A05)..." -ForegroundColor Yellow
foreach ($id in @("A01", "A05")) {
    $manifestPath = ".\labs\$id\manifest.json"
    $composePath = ".\labs\$id\docker\compose.yml"
    if ((Test-Path $manifestPath) -and (Test-Path $composePath)) {
        Write-Host "  -> Lab $id Manifest & Docker Compose: PASS" -ForegroundColor Green
    } else {
        Write-Error "Lab $id missing required files!"
        exit 1
    }
}

# 5. Flutter App Analysis & Unit Tests
Write-Host "`n[5/5] Running Flutter Static Analysis & Unit Tests..." -ForegroundColor Yellow
Set-Location "$WorkspaceRoot\ui\flutter"
flutter analyze
if ($LASTEXITCODE -ne 0) { Write-Error "Flutter analysis failed!"; exit 1 }
flutter test
if ($LASTEXITCODE -ne 0) { Write-Error "Flutter tests failed!"; exit 1 }
Write-Host "  -> Flutter Analyze & Widget Tests: PASS" -ForegroundColor Green

Set-Location $WorkspaceRoot
Write-Host "`n==================================================" -ForegroundColor Cyan
Write-Host "  ALL 5 CRITICAL CHECKPOINTS PASSED SUCCESSFULLY! " -ForegroundColor Green
Write-Host "==================================================" -ForegroundColor Cyan
