<#
.SYNOPSIS
    Builds the integrated ZITERA_LAB Windows x64 distribution bundle and archive.
.DESCRIPTION
    Phase 22 Release Assembly Pipeline:
    Preflight -> Compile Engine -> Compile Flutter -> Collect CRT DLLs ->
    Collect Runner & Data -> Collect Catalog & Signature -> Collect Packages ->
    Collect Branding -> Generate Manifest, Hashes, Readme -> Create ZIP -> Smoke Test.
#>

[CmdletBinding()]
param(
    [string]$Version = "2.0.0",
    [switch]$SkipBuild,
    [switch]$SkipTests
)

$ErrorActionPreference = 'Stop'
$ProgressPreference = 'SilentlyContinue'
$sw = [System.Diagnostics.Stopwatch]::StartNew()

$repoRoot = (Resolve-Path "$PSScriptRoot\..").Path
$distRoot = Join-Path $repoRoot "dist"
$bundleDir = Join-Path $distRoot "ZITERA_LAB_WINDOWS_X64"
$zipPath = Join-Path $distRoot "ZITERA_LAB_WINDOWS_X64.zip"

function Write-Section([string]$title) {
    Write-Host ""
    Write-Host ("=" * 72) -ForegroundColor Cyan
    Write-Host "  >> $title" -ForegroundColor Cyan
    Write-Host ("=" * 72) -ForegroundColor Cyan
}

Write-Section "STAGE 1: PREFLIGHT & DIRECTORY INITIALIZATION"

Write-Host "Repository Root : $repoRoot"
Write-Host "Distribution Dir: $bundleDir"
Write-Host "Archive Path    : $zipPath"
Write-Host "Target Version  : $Version"

# Clean target bundle directory
if (Test-Path $bundleDir) {
    Write-Host "Cleaning existing bundle directory..." -ForegroundColor Gray
    Remove-Item -Path $bundleDir -Recurse -Force
}
New-Item -ItemType Directory -Path $bundleDir -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $bundleDir "engine") -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $bundleDir "catalog") -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $bundleDir "packages") -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $bundleDir "branding") -Force | Out-Null

if (-not $SkipBuild) {
    Write-Section "STAGE 2: RUST CORE ENGINE RELEASE COMPILATION"
    Push-Location (Join-Path $repoRoot "engine\rust")
    try {
        $env:CARGO_INCREMENTAL = "0"
        Write-Host "Building release zitera-engine.exe..." -ForegroundColor Yellow
        cargo build --release --bin zitera-engine
        if ($LASTEXITCODE -ne 0) { throw "cargo build failed with exit code $LASTEXITCODE" }
    }
    finally {
        Pop-Location
    }

    Write-Section "STAGE 3: FLUTTER WINDOWS RUNNER RELEASE COMPILATION"
    Push-Location (Join-Path $repoRoot "ui\flutter")
    try {
        $env:PATH = "C:\src\flutter\bin;$env:PATH"
        Write-Host "Building Flutter release bundle..." -ForegroundColor Yellow
        flutter build windows --release
        if ($LASTEXITCODE -ne 0) { throw "flutter build windows --release failed with code $LASTEXITCODE" }
    }
    finally {
        Pop-Location
    }
} else {
    Write-Host "Skipping builds as requested (-SkipBuild)." -ForegroundColor Yellow
}

Write-Section "STAGE 4: COLLECT RUNNER & ENGINE EXECUTABLES"

$engineSrc = Join-Path $repoRoot "engine\rust\target\release\zitera-engine.exe"
if (-not (Test-Path $engineSrc)) {
    throw "Compiled engine not found at $engineSrc"
}
$engineDst = Join-Path $bundleDir "engine\zitera-engine.exe"
Copy-Item $engineSrc $engineDst -Force
Write-Host " [COPIED] engine\zitera-engine.exe ($((Get-Item $engineDst).Length) bytes)" -ForegroundColor Green

$flutterReleaseDir = Join-Path $repoRoot "ui\flutter\build\windows\x64\runner\Release"
$runnerExeSrc = Join-Path $flutterReleaseDir "zitera_lab.exe"
$flutterDllSrc = Join-Path $flutterReleaseDir "flutter_windows.dll"
$flutterDataSrc = Join-Path $flutterReleaseDir "data"

if (-not (Test-Path $runnerExeSrc)) { throw "Runner executable not found at $runnerExeSrc" }
if (-not (Test-Path $flutterDllSrc)) { throw "Flutter Windows DLL not found at $flutterDllSrc" }
if (-not (Test-Path $flutterDataSrc)) { throw "Flutter data directory not found at $flutterDataSrc" }

Copy-Item $runnerExeSrc (Join-Path $bundleDir "zitera_lab.exe") -Force
Copy-Item $flutterDllSrc (Join-Path $bundleDir "flutter_windows.dll") -Force
Copy-Item $flutterDataSrc (Join-Path $bundleDir "data") -Recurse -Force
Write-Host " [COPIED] zitera_lab.exe, flutter_windows.dll, and data/" -ForegroundColor Green

Write-Section "STAGE 5: COLLECT NATIVE RUNTIME DEPENDENCIES (MSVC CRT)"

$redistDir = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Redist\MSVC\14.44.35112\x64\Microsoft.VC143.CRT"
$crtDlls = @("vcruntime140.dll", "vcruntime140_1.dll", "msvcp140.dll")

foreach ($dll in $crtDlls) {
    $src = Join-Path $redistDir $dll
    if (Test-Path $src) {
        $dst = Join-Path $bundleDir $dll
        Copy-Item $src $dst -Force
        Write-Host " [BUNDLED] $dll ($((Get-Item $dst).Length) bytes)" -ForegroundColor Green
    } else {
        Write-Warning "Could not find $dll in $redistDir"
    }
}

Write-Section "STAGE 6: COLLECT SIGNED CATALOG & LAB PACKAGES"

# 1. Catalog & Signature
$catalogSrc = Join-Path $repoRoot "catalog\catalog.json"
$catalogDst = Join-Path $bundleDir "catalog\catalog.json"
Copy-Item $catalogSrc $catalogDst -Force

# Obtain canonical signature from engine
$engineRun = & $engineDst --json catalog | Out-String
$parsed = $engineRun | ConvertFrom-Json
$sig = if ($parsed.data.signature) { $parsed.data.signature } elseif ($parsed.catalog.signature) { $parsed.catalog.signature } else { $parsed.signature }
if ([string]::IsNullOrWhiteSpace($sig)) {
    throw "Failed to extract catalog signature from engine"
}
$sigDst = Join-Path $bundleDir "catalog\catalog.signature"
[System.IO.File]::WriteAllText($sigDst, $sig.Trim(), [System.Text.UTF8Encoding]::new($false))

# Also ensure bundled catalog.json contains signature property
$catalogObj = Get-Content $catalogDst -Raw | ConvertFrom-Json
$catalogObj | Add-Member -NotePropertyName "signature" -NotePropertyValue $sig.Trim() -Force
$signedCatalogJson = $catalogObj | ConvertTo-Json -Depth 10
[System.IO.File]::WriteAllText($catalogDst, $signedCatalogJson, [System.Text.UTF8Encoding]::new($false))
Write-Host " [COPIED & SIGNED] catalog\catalog.json and catalog\catalog.signature" -ForegroundColor Green

# 2. Signed Lab Packages (A01 - A10)
$packagesSrcDir = Join-Path $distRoot "packages"
$labIds = @("A01", "A02", "A03", "A04", "A05", "A06", "A07", "A08", "A09", "A10")

foreach ($labId in $labIds) {
    $pkgSrc = Join-Path $packagesSrcDir "$labId.zlab"
    if (Test-Path $pkgSrc) {
        $pkgDst = Join-Path $bundleDir "packages\$labId.zlab"
        Copy-Item $pkgSrc $pkgDst -Force
        Write-Host " [BUNDLED] packages\$labId.zlab ($((Get-Item $pkgDst).Length) bytes)" -ForegroundColor Green
    } else {
        throw "Required lab package missing: $pkgSrc"
    }
}

Write-Section "STAGE 7: COLLECT BRANDING RESOURCES"

$logoSrc = Join-Path $repoRoot "branding\logo.png"
$iconSrc = Join-Path $repoRoot "branding\app_icon.ico"
if (Test-Path $logoSrc) { Copy-Item $logoSrc (Join-Path $bundleDir "branding\logo.png") -Force }
if (Test-Path $iconSrc) { Copy-Item $iconSrc (Join-Path $bundleDir "branding\app_icon.ico") -Force }
Write-Host " [COPIED] branding\logo.png and branding\app_icon.ico" -ForegroundColor Green

Write-Section "STAGE 8: GENERATE FIRST-RUN README & MANIFEST"

$readmeContent = @"
================================================================================
  ZITERA_LAB - Defensive Security & Attack Engineering Platform
  Distribution: Windows x64 (Zero-Elevation Portable Bundle)
  Version: $Version
================================================================================

OVERVIEW:
  ZITERA_LAB is a self-contained, offline-first cybersecurity training and attack
  engineering workstation. It runs completely local on standard Windows 10/11 x64
  user environments without requiring administrative elevation, background daemons,
  Docker, or an internet connection.

QUICK START:
  1. Launch 'zitera_lab.exe' directly from this directory.
  2. On first launch, the First-Run Readiness Checker will automatically audit:
     - Bundled Rust Core Engine (engine/zitera-engine.exe)
     - Windows Native CRT Runtime (Universal CRT / MSVC DLLs)
     - Signed Curriculum Catalog & Ed25519 Cryptographic Trust
     - Offline Lab Packages (A01 - A10 .zlab archives)
  3. Once verified, choose any lab from the catalog to begin learning.

CLI USAGE (OPTIONAL):
  engine\zitera-engine.exe doctor
  engine\zitera-engine.exe catalog
  engine\zitera-engine.exe lab list
  engine\zitera-engine.exe lab start A01

SECURITY & PRIVACY:
  All labs execute inside Windows AppContainer isolation and Job Objects.
  Process monitoring and cleanup are automatic on shell exit.
================================================================================
"@
$readmePath = Join-Path $bundleDir "README_FIRST_RUN.txt"
[System.IO.File]::WriteAllText($readmePath, $readmeContent, [System.Text.UTF8Encoding]::new($false))
Write-Host " [CREATED] README_FIRST_RUN.txt" -ForegroundColor Green

# Generate SHA256SUMS.txt
$allFiles = Get-ChildItem -Path $bundleDir -Recurse -File | Where-Object { $_.Name -ne "SHA256SUMS.txt" -and $_.Name -ne "manifest.json" }
$hashLines = @()
$manifestFiles = @()

foreach ($f in $allFiles) {
    $relPath = ($f.FullName.Substring($bundleDir.Length + 1)).Replace("\", "/")
    $hash = (Get-FileHash -Path $f.FullName -Algorithm SHA256).Hash.ToLower()
    $hashLines += "$hash  $relPath"
    $manifestFiles += [PSCustomObject]@{
        path = $relPath
        size_bytes = $f.Length
        sha256 = $hash
    }
}

$hashPath = Join-Path $bundleDir "SHA256SUMS.txt"
[System.IO.File]::WriteAllLines($hashPath, $hashLines, [System.Text.UTF8Encoding]::new($false))
Write-Host " [GENERATED] SHA256SUMS.txt ($($manifestFiles.Count) entries)" -ForegroundColor Green

# Generate manifest.json
$commitSha = try { (git rev-parse HEAD).Trim() } catch { "UNKNOWN" }
$manifestObj = [PSCustomObject]@{
    product = "ZITERA_LAB"
    version = $Version
    distribution = "WINDOWS_X64"
    architecture = "x86_64"
    platform = "windows"
    target_triple = "x86_64-pc-windows-msvc"
    commit_sha = $commitSha
    build_timestamp = [System.DateTime]::UtcNow.ToString("o")
    offline_packages_count = $labIds.Count
    files_count = $manifestFiles.Count
    files = $manifestFiles
    guarantees = [PSCustomObject]@{
        zero_developer_dependency = $true
        zero_elevation_required = $true
        offline_first = $true
        appcontainer_sandboxed = $true
    }
}
$manifestJson = $manifestObj | ConvertTo-Json -Depth 6
$manifestPath = Join-Path $bundleDir "manifest.json"
[System.IO.File]::WriteAllText($manifestPath, $manifestJson, [System.Text.UTF8Encoding]::new($false))
Write-Host " [GENERATED] manifest.json" -ForegroundColor Green

Write-Section "STAGE 9: CREATE DISTRIBUTION ZIP ARCHIVE"

if (Test-Path $zipPath) {
    Remove-Item $zipPath -Force
}
Write-Host "Compressing bundle to $zipPath (CompressionLevel: Optimal)..." -ForegroundColor Yellow
Compress-Archive -Path "$bundleDir\*" -DestinationPath $zipPath -CompressionLevel Optimal
Write-Host " [ARCHIVED] $zipPath ($((Get-Item $zipPath).Length) bytes)" -ForegroundColor Green

Write-Section "STAGE 10: BUNDLE VERIFICATION & SMOKE TEST"

Write-Host "Testing bundled zitera-engine.exe doctor..." -ForegroundColor Yellow
$doctorOutput = & $engineDst --json doctor | Out-String
$doctorObj = $doctorOutput | ConvertFrom-Json
if (-not $doctorObj.success -or -not $doctorObj.data.all_ready) {
    throw "Bundled engine doctor check failed: $doctorOutput"
}
Write-Host " [PASS] Bundled engine doctor check succeeded (all_ready: $($doctorObj.data.all_ready))." -ForegroundColor Green

$sw.Stop()
$elapsed = [math]::Round($sw.Elapsed.TotalSeconds, 2)
Write-Host ""
Write-Host "========================================================================" -ForegroundColor Green
Write-Host " BUNDLE ASSEMBLY & VERIFICATION COMPLETED IN ${elapsed}s               " -ForegroundColor Green
Write-Host "========================================================================" -ForegroundColor Green
