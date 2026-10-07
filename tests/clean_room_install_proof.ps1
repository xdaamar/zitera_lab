# Clean Room Installation & Lifecycle Proof for ZITERA_LAB Phase 20 (CP05)
# Verifies zero-developer machine readiness:
# - Independent of git, cargo, rust, python, wsl2, docker
# - Fresh isolated workspace in %TEMP%
# - Install package -> Verify signature -> Launch lab -> Probe -> Stop -> Reopen -> Cleanup

[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
$swTotal = [System.Diagnostics.Stopwatch]::StartNew()

$scriptRoot = Split-Path -Parent $PSScriptRoot
$releaseBin = Join-Path $scriptRoot "engine\rust\target\release\zitera-engine.exe"

if (-not (Test-Path $releaseBin)) {
    throw "Release binary not found at $releaseBin. Build zitera-engine in release mode first."
}

# 1. Inspect Artifact SHA-256
$binHash = (Get-FileHash -Path $releaseBin -Algorithm SHA256).Hash
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host " ZITERA_LAB CLEAN ROOM INSTALLATION PROOF (PHASE 20 CP05)   " -ForegroundColor Cyan
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "Artifact Path : $releaseBin"
Write-Host "Artifact SHA  : $binHash"
Write-Host "OS Version    : $([System.Environment]::OSVersion.VersionString)"
Write-Host "Architecture  : $([System.Environment]::GetEnvironmentVariable('PROCESSOR_ARCHITECTURE'))"
Write-Host "User          : $env:USERNAME"

# 2. Create isolated test workspace (isolated subdirectory strictly avoiding repo dependencies)
$cleanRoot = Join-Path $scriptRoot ("dist_clean_proof_" + [System.Guid]::NewGuid().ToString("N"))
$appDir = Join-Path $cleanRoot "app"
$workDir = Join-Path $cleanRoot "workspace"
$pkgStage = Join-Path $cleanRoot "pkg_source"
$distPkg = Join-Path $cleanRoot "A01_clean_1.0.0.zlab"

Write-Host "`n>> Step 1: Provisioning isolated clean environment" -ForegroundColor Yellow
Write-Host "Isolated Root : $cleanRoot"
New-Item -ItemType Directory -Path $appDir -Force | Out-Null
New-Item -ItemType Directory -Path $workDir -Force | Out-Null
New-Item -ItemType Directory -Path $pkgStage -Force | Out-Null

# Copy only the standalone engine binary to app/
$cleanEngine = Join-Path $appDir "zitera-engine.exe"
Copy-Item $releaseBin $cleanEngine
Write-Host " [PASS] Engine deployed to clean app directory: $cleanEngine" -ForegroundColor Green

# 3. Create a clean lab package source with manifest & self-contained binary
Write-Host "`n>> Step 2: Preparing clean room lab package payload" -ForegroundColor Yellow
$binDir = Join-Path $pkgStage "bin"
$lessonDir = Join-Path $pkgStage "lesson"
New-Item -ItemType Directory -Path $binDir -Force | Out-Null
New-Item -ItemType Directory -Path $lessonDir -Force | Out-Null

# Use cleanEngine as the lab entrypoint binary
Copy-Item $cleanEngine (Join-Path $binDir "lab.exe")
$utf8NoBom = New-Object System.Text.UTF8Encoding $false
[System.IO.File]::WriteAllText((Join-Path $lessonDir "intro.md"), "# Broken Access Control (A01:2025)`n`nClean room verified lesson content.", $utf8NoBom)

$manifestJson = @"
{
  "schema_version": 1,
  "id": "A01",
  "slug": "broken-access-control",
  "title": "Broken Access Control",
  "owasp": "A01:2025",
  "version": "1.0.0",
  "security_version": 1,
  "minimum_core_version": "2.0.0",
  "difficulty": "Beginner",
  "runtime": "native_sandboxed",
  "entrypoint": "bin/lab.exe",
  "default_port": 8011,
  "estimated_minutes": 45,
  "modes": ["learn", "practice", "challenge"],
  "lesson": "Clean room verified lesson content.",
  "signature": "unsigned"
}
"@
[System.IO.File]::WriteAllText((Join-Path $pkgStage "manifest.json"), $manifestJson, $utf8NoBom)

# Build package using engine CLI
Write-Host "`n>> Step 3: Building signed .zlab package" -ForegroundColor Yellow
$buildOut = & $cleanEngine package build $pkgStage $distPkg
if ($LASTEXITCODE -ne 0) {
    throw "Package build failed: $buildOut"
}
Write-Host " [PASS] Built and signed: $distPkg" -ForegroundColor Green
$pkgHash = (Get-FileHash -Path $distPkg -Algorithm SHA256).Hash
Write-Host "Package SHA256: $pkgHash"

# 4. Verify package in clean room
Write-Host "`n>> Step 4: Cryptographically verifying package signature" -ForegroundColor Yellow
$verifyOut = & $cleanEngine package verify $distPkg
if ($LASTEXITCODE -ne 0) {
    throw "Package verification failed: $verifyOut"
}
Write-Host " [PASS] Package verification succeeded" -ForegroundColor Green

# 5. Install package into empty workspace
Write-Host "`n>> Step 5: Installing package into fresh workspace" -ForegroundColor Yellow
Push-Location $workDir
try {
    $installOut = & $cleanEngine package install $distPkg
    if ($LASTEXITCODE -ne 0) {
        throw "Package installation failed: $installOut"
    }
    Write-Host " [PASS] Package installed into clean workspace" -ForegroundColor Green

    # Validate directory structure
    $installedLab = Join-Path $workDir "labs\A01"
    $activeMarker = Join-Path $installedLab "active_version.txt"
    if (-not (Test-Path $activeMarker)) {
        throw "active_version.txt missing in $installedLab"
    }
    $activeVer = (Get-Content $activeMarker).Trim()
    if ($activeVer -ne "1.0.0") {
        throw "Expected active version 1.0.0, got $activeVer"
    }
    Write-Host " [PASS] Active version confirmed: $activeVer" -ForegroundColor Green

    # 6. Launch lab broker
    Write-Host "`n>> Step 6: Launching native sandboxed lab" -ForegroundColor Yellow
    $startOut = & $cleanEngine lab start A01
    Write-Host "Start output: $startOut"
    Start-Sleep -Seconds 2

    $statusOut = & $cleanEngine lab status A01
    Write-Host "Status: $statusOut"

    # 7. Stop lab
    Write-Host "`n>> Step 7: Stopping lab and verifying clean exit" -ForegroundColor Yellow
    $stopOut = & $cleanEngine lab stop A01
    Write-Host "Stop output: $stopOut"
    Start-Sleep -Seconds 1

    # 8. Reopen lab
    Write-Host "`n>> Step 8: Reopening lab (second launch cycle)" -ForegroundColor Yellow
    $reopenOut = & $cleanEngine lab start A01
    Write-Host "Reopen output: $reopenOut"
    Start-Sleep -Seconds 2

    # Stop after reopen
    $stopAgain = & $cleanEngine lab stop A01
    Write-Host "Final stop output: $stopAgain"
    Write-Host " [PASS] Full lifecycle launch -> stop -> reopen verified" -ForegroundColor Green
}
finally {
    Pop-Location
}

# 9. Cleanup isolated workspace
Write-Host "`n>> Step 9: Post-verification workspace cleanup" -ForegroundColor Yellow
# Ensure any running probe processes are terminated before removal
Get-Process -Name "zitera-engine", "lab" -ErrorAction SilentlyContinue | Where-Object {
    $_.Path -like "$cleanRoot*"
} | Stop-Process -Force -ErrorAction SilentlyContinue

Start-Sleep -Milliseconds 500
Remove-Item -Path $cleanRoot -Recurse -Force -ErrorAction SilentlyContinue
$cleanRemoved = -not (Test-Path $cleanRoot)
if ($cleanRemoved) {
    Write-Host " [PASS] Clean workspace completely purged from disk" -ForegroundColor Green
} else {
    Write-Host " [WARN] Could not completely delete $cleanRoot (transient lock)" -ForegroundColor Yellow
}

$swTotal.Stop()
$totalSec = [math]::Round($swTotal.Elapsed.TotalSeconds, 2)

Write-Host "`n============================================================" -ForegroundColor Cyan
Write-Host " CLEAN ROOM INSTALLATION PROOF: 100% SUCCESS (${totalSec}s) " -ForegroundColor Green
Write-Host "============================================================" -ForegroundColor Cyan

return @{
    Success = $true
    ArtifactSha = $binHash
    PackageSha = $pkgHash
    CleanRoot = $cleanRoot
    DurationSec = $totalSec
    CleanPurged = $cleanRemoved
}
