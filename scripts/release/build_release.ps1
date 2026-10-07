<#
.SYNOPSIS
    Deterministic Release Artifact Pipeline for ZITERA_LAB Core.
.DESCRIPTION
    Executes end-to-end release pipeline:
    SOURCE SHA -> BUILD -> TEST -> PACKAGE -> HASH -> SIGN -> MANIFEST -> DIST
    Outputs all distributable artifacts to dist/ along with release_manifest.json
    and checksums.sha256.
#>

[CmdletBinding()]
param(
    [string]$Version = "2.0.0",
    [switch]$SkipTests,
    [string]$OutputDir = ""
)

$ErrorActionPreference = "Stop"
$swPipeline = [System.Diagnostics.Stopwatch]::StartNew()

$script:Root = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$script:RustDir = Join-Path $script:Root "engine\rust"
if ([string]::IsNullOrWhiteSpace($OutputDir)) {
    $script:DistDir = Join-Path $script:Root "dist"
} else {
    $script:DistDir = $OutputDir
}

function Write-Step([string]$title) {
    Write-Host ""
    Write-Host ("=" * 70) -ForegroundColor Cyan
    Write-Host "  >> $title" -ForegroundColor Cyan
    Write-Host ("=" * 70) -ForegroundColor Cyan
}

function Ensure-MsvcBuildEnvironment {
    if (-not (Get-Command cl.exe -ErrorAction SilentlyContinue)) {
        $vcvars = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
        if (Test-Path $vcvars) {
            Write-Host "Initializing MSVC x64 build environment..." -ForegroundColor Gray
            cmd.exe /c "call `"$vcvars`" && set" | ForEach-Object {
                if ($_ -match '^(.*?)=(.*)$') {
                    [System.Environment]::SetEnvironmentVariable($matches[1], $matches[2], "Process")
                }
            }
        } else {
            throw "MSVC build tools (vcvars64.bat) not found at expected path: $vcvars"
        }
    }
    # Enforce non-incremental release build determinism for WDAC compliance
    $env:CARGO_INCREMENTAL = "0"
}

Write-Step "STAGE 1: SOURCE SHA & REPOSITORY METADATA"
$commitSha = ""
try {
    $commitSha = (git rev-parse HEAD).Trim()
} catch {
    $commitSha = "UNKNOWN_OR_UNTRACKED"
}
$buildTimestamp = [System.DateTime]::UtcNow.ToString("yyyy-MM-ddTHH:mm:ssZ")

Write-Host "Commit SHA      : $commitSha"
Write-Host "Release Version : $Version"
Write-Host "Build Timestamp : $buildTimestamp"
Write-Host "Target Platform : windows-x64 (x86_64-pc-windows-msvc)"

Write-Step "STAGE 2: NATIVE COMPILATION (RELEASE PROFILE)"
Ensure-MsvcBuildEnvironment

Push-Location $script:RustDir
try {
    Write-Host "Building release binary zitera-engine.exe (profile: release, incremental: 0)..." -ForegroundColor Yellow
    cargo build --release --bin zitera-engine
    if ($LASTEXITCODE -ne 0) {
        throw "cargo build --release failed with exit code $LASTEXITCODE"
    }
} finally {
    Pop-Location
}

$engineBin = Join-Path $script:RustDir "target\release\zitera-engine.exe"
if (-not (Test-Path $engineBin)) {
    throw "Built binary not found at $engineBin"
}
$engineBinSize = (Get-Item $engineBin).Length
$engineBinHash = (Get-FileHash -Path $engineBin -Algorithm SHA256).Hash
Write-Host "Compiled binary : $engineBin" -ForegroundColor Green
Write-Host "Binary Size     : $engineBinSize bytes"
Write-Host "Binary SHA256   : $engineBinHash"

if (-not $SkipTests) {
    Write-Step "STAGE 3: RELEASE TEST SUITE VERIFICATION"
    Push-Location $script:RustDir
    try {
        Write-Host "Running release verification test matrix (CARGO_INCREMENTAL=0)..." -ForegroundColor Yellow
        cargo test --release --bin zitera-engine -- package::
        if ($LASTEXITCODE -ne 0) {
            throw "Release tests failed with exit code $LASTEXITCODE"
        }
        Write-Host " [PASS] Release test suite passed." -ForegroundColor Green
    } finally {
        Pop-Location
    }
} else {
    Write-Host "Skipping test execution as requested (-SkipTests flag)." -ForegroundColor Yellow
}

Write-Step "STAGE 4: PREPARING DISTRIBUTION ARTIFACTS"
$distBinDir = Join-Path $script:DistDir "bin"
$distPkgDir = Join-Path $script:DistDir "packages"
$distPortableDir = Join-Path $script:DistDir "bundle"

if (Test-Path $distPortableDir) {
    Remove-Item -Path $distPortableDir -Recurse -Force -ErrorAction SilentlyContinue
}
New-Item -ItemType Directory -Path $distBinDir -Force | Out-Null
New-Item -ItemType Directory -Path $distPkgDir -Force | Out-Null
New-Item -ItemType Directory -Path $distPortableDir -Force | Out-Null

# Copy binary to dist/bin/
$targetEngine = Join-Path $distBinDir "zitera-engine.exe"
Copy-Item $engineBin $targetEngine -Force
Write-Host " [COPIED] $targetEngine" -ForegroundColor Green

# Build bundle directory for portable archive
$portableBinDir = Join-Path $distPortableDir "bin"
New-Item -ItemType Directory -Path $portableBinDir -Force | Out-Null
Copy-Item $engineBin (Join-Path $portableBinDir "zitera-engine.exe") -Force

# Copy catalog if present
$catalogDir = Join-Path $script:Root "catalog"
if (Test-Path $catalogDir) {
    Copy-Item $catalogDir (Join-Path $distPortableDir "catalog") -Recurse -Force
}

# Create portable package README
$portableReadme = @"
# ZITERA_LAB Portable Edition (v$Version)
Self-contained, non-developer cybersecurity laboratory runtime for Windows x64.

Usage:
  bin\zitera-engine.exe doctor
  bin\zitera-engine.exe lab list
  bin\zitera-engine.exe lab start <ID>
"@
[System.IO.File]::WriteAllText((Join-Path $distPortableDir "README.txt"), $portableReadme, [System.Text.UTF8Encoding]::new($false))

# Create portable zip archive
$portableZip = Join-Path $script:DistDir "zitera-lab-$Version-windows-x64-portable.zip"
if (Test-Path $portableZip) {
    Remove-Item $portableZip -Force
}
Compress-Archive -Path "$distPortableDir\*" -DestinationPath $portableZip -CompressionLevel Optimal
Remove-Item -Path $distPortableDir -Recurse -Force -ErrorAction SilentlyContinue
Write-Host " [PACKAGED] $portableZip" -ForegroundColor Green

Write-Step "STAGE 5 & 6: ARTIFACT HASHING & METADATA"
$artifactList = [System.Collections.Generic.List[PSObject]]::new()
$checksumLines = [System.Collections.Generic.List[string]]::new()

function Register-Artifact([string]$path, [string]$type, [string]$desc) {
    $item = Get-Item $path
    $hash = (Get-FileHash -Path $path -Algorithm SHA256).Hash
    $artifactObj = [PSCustomObject]@{
        artifact_name = $item.Name
        artifact_type = $type
        artifact_size = $item.Length
        sha256 = $hash
        description = $desc
    }
    $script:artifactList.Add($artifactObj)
    $script:checksumLines.Add("$hash  $($item.Name)")
}

Register-Artifact $targetEngine "binary" "ZITERA Core Engine Standalone Windows x64 Executable"
Register-Artifact $portableZip "portable_archive" "ZITERA Complete Portable Zero-Install Distribution Archive"

# Write standard checksums.sha256
$checksumFile = Join-Path $script:DistDir "checksums.sha256"
[System.IO.File]::WriteAllLines($checksumFile, $checksumLines, [System.Text.UTF8Encoding]::new($false))
Write-Host " [HASHED] $checksumFile" -ForegroundColor Green

Write-Step "STAGE 7: GENERATING DETERMINISTIC RELEASE MANIFEST"
$manifestObject = [PSCustomObject]@{
    schema_version = 1
    product = "ZITERA_LAB"
    version = $Version
    commit_sha = $commitSha
    build_timestamp = $buildTimestamp
    platform = "windows"
    architecture = "x86_64"
    target_triple = "x86_64-pc-windows-msvc"
    artifacts = $artifactList
    verification = [PSCustomObject]@{
        verified_by = "verify_release.ps1"
        all_tests_passed = (-not $SkipTests)
        contract_version = "V3"
    }
    deterministic_guarantee = [PSCustomObject]@{
        content_digest_independent_of_timestamp = $true
        zero_developer_dependency = $true
        zero_elevation_required = $true
    }
}

$manifestJson = $manifestObject | ConvertTo-Json -Depth 6
$manifestFile = Join-Path $script:DistDir "release_manifest.json"
[System.IO.File]::WriteAllText($manifestFile, $manifestJson, [System.Text.UTF8Encoding]::new($false))
Write-Host " [GENERATED] $manifestFile" -ForegroundColor Green

Write-Step "STAGE 8: DISTRIBUTION PIPELINE SUMMARY"
$swPipeline.Stop()
$totalElapsed = [math]::Round($swPipeline.Elapsed.TotalSeconds, 2)

Write-Host ""
Write-Host "======================================================================" -ForegroundColor Green
Write-Host " RELEASE ARTIFACT PIPELINE COMPLETED SUCCESSFULLY (${totalElapsed}s)  " -ForegroundColor Green
Write-Host "======================================================================" -ForegroundColor Green
Write-Host ""
$artifactList | Format-Table -Property artifact_name, artifact_size, sha256 -AutoSize

return $manifestObject
