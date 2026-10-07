<#
.SYNOPSIS
    Official Release Verification Matrix for ZITERA_LAB Core.
.DESCRIPTION
    Runs multi-stage verification across Core, Package, Security, Lab,
    Terminal, Curriculum, and Release build integrity.
    Fails fast with non-zero exit code on any failure.
#>

[CmdletBinding()]
param(
    [ValidateSet("ALL", "CORE", "PACKAGE", "SECURITY", "LAB", "TERMINAL", "VALIDATOR", "RELEASE")]
    [string]$Stage = "ALL"
)

$ErrorActionPreference = "Continue"
$script:Root = Split-Path -Parent $PSScriptRoot
$script:RustDir = Join-Path $script:Root "engine\rust"
$script:ReleaseBin = Join-Path $script:Root "engine\rust\target\release\zitera-engine.exe"
$script:StageResults = [ordered]@{}

function Write-Banner([string]$title) {
    Write-Host ""
    Write-Host ("=" * 70) -ForegroundColor Cyan
    Write-Host "  $title" -ForegroundColor Cyan
    Write-Host ("=" * 70) -ForegroundColor Cyan
}

function Write-StageHeader([string]$name, [string]$desc) {
    Write-Host ""
    Write-Host ">> STAGE: $name - $desc" -ForegroundColor Yellow
    Write-Host ("-" * 70) -ForegroundColor Gray
}

function Run-Step([string]$stageName, [scriptblock]$action) {
    $sw = [System.Diagnostics.Stopwatch]::StartNew()
    $global:LASTEXITCODE = 0
    try {
        & $action
        if ($LASTEXITCODE -ne 0) {
            throw "Process exited with code $LASTEXITCODE"
        }
        $sw.Stop()
        $elapsed = [math]::Round($sw.Elapsed.TotalSeconds, 2)
        Write-Host " [PASS] $stageName (${elapsed}s)" -ForegroundColor Green
        $script:StageResults[$stageName] = "PASS (${elapsed}s)"
    }
    catch {
        $sw.Stop()
        Write-Host " [FAIL] $stageName : $_" -ForegroundColor Red
        $script:StageResults[$stageName] = "FAIL"
        Write-Host "`nRelease verification failed at stage: $stageName" -ForegroundColor Red
        exit 1
    }
}

function Ensure-MsvcEnvironment {
    if (-not (Get-Command cl.exe -ErrorAction SilentlyContinue)) {
        $vcvars = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
        if (Test-Path $vcvars) {
            Write-Host "Initializing MSVC x64 build environment..." -ForegroundColor Gray
            $env:CARGO_INCREMENTAL = "0"
            cmd.exe /c "call `"$vcvars`" && set" | ForEach-Object {
                if ($_ -match '^(.*?)=(.*)$') {
                    [System.Environment]::SetEnvironmentVariable($matches[1], $matches[2], "Process")
                }
            }
        }
    }
}

Write-Banner "ZITERA_LAB // FULL REPOSITORY RELEASE TEST MATRIX"
Write-Host "Workspace Root: $script:Root" -ForegroundColor Gray
Write-Host "Target: Windows x64 (Native Sandboxed Runtime)" -ForegroundColor Gray

Ensure-MsvcEnvironment

Push-Location $script:RustDir
try {
    # STAGE 1: CORE ENGINE
    if ($Stage -in @("ALL", "CORE")) {
        Write-StageHeader "CORE" "Host Broker, Catalog, Session & HTTP Routing"
        Run-Step "CORE_BROKER_CATALOG" {
            cargo test --release --bin zitera-engine -- broker:: catalog::
        }
    }

    # STAGE 2: PACKAGE SUBSYSTEM
    if ($Stage -in @("ALL", "PACKAGE")) {
        Write-StageHeader "PACKAGE" "Ed25519 Signing, Archive Bounds, Anti-Downgrade & Rollback"
        Run-Step "PACKAGE_TRUST_MATRIX" {
            cargo test --release --bin zitera-engine -- package::
        }
    }

    # STAGE 3: NATIVE SECURITY & ISOLATION
    if ($Stage -in @("ALL", "SECURITY")) {
        Write-StageHeader "SECURITY" "AppContainer DACL, Job Objects, Kill-on-Close & Network Confinement"
        Run-Step "NATIVE_RUNTIME_ISOLATION" {
            cargo test --release --bin zitera-engine -- native_runtime::
        }
    }

    # STAGE 4: TERMINAL PRODUCTIZATION
    if ($Stage -in @("ALL", "TERMINAL")) {
        Write-StageHeader "TERMINAL" "Lexer, VFS, Injection Defense, Safe curl & ps"
        Run-Step "TERMINAL_SECURITY_SUITE" {
            cargo test --release --bin zitera-engine -- terminal::
        }
    }

    # STAGE 5: LAB CONTRACT & LIFECYCLE
    if ($Stage -in @("ALL", "LAB")) {
        Write-StageHeader "LAB" "Lab Lifecycle, Challenge Flag Validation & Sandbox Execution"
        Run-Step "LAB_CONTRACT_AND_LIFECYCLE" {
            cargo test --release --bin zitera-engine -- labs:: -- --test-threads=2
        }
    }

    # STAGE 6: CURRICULUM VALIDATOR
    if ($Stage -in @("ALL", "VALIDATOR")) {
        Write-StageHeader "VALIDATOR" "Automated 8-point inspection across all 10 canonical labs"
        Run-Step "CANONICAL_LABS_VALIDATION" {
            $labs = @("A01", "A02", "A03", "A04", "A05", "A06", "A07", "A08", "A09", "A10")
            foreach ($lab in $labs) {
                & $script:ReleaseBin lab validate $lab
                if ($LASTEXITCODE -ne 0) {
                    throw "Lab validation failed for $lab"
                }
            }
        }
    }

    # STAGE 7: RELEASE INTEGRITY & CLEAN COMPILATION
    if ($Stage -in @("ALL", "RELEASE")) {
        Write-StageHeader "RELEASE" "Cargo Check & Release Binary Verification"
        Run-Step "RELEASE_BUILD_VERIFICATION" {
            cargo check --release --bin zitera-engine
            if ($LASTEXITCODE -ne 0) {
                throw "cargo check --release failed"
            }
            if (-not (Test-Path $script:ReleaseBin)) {
                throw "Release binary not found at $script:ReleaseBin"
            }
        }
    }
}
finally {
    Pop-Location
}

Write-Banner "RELEASE VERIFICATION SUMMARY"
$allPassed = $true
foreach ($entry in $script:StageResults.GetEnumerator()) {
    $statusColor = if ($entry.Value -match "^PASS") { "Green" } else { "Red" }
    Write-Host (" [{0,-4}] {1,-35} : {2}" -f ($entry.Value.Substring(0, 4)), $entry.Key, $entry.Value) -ForegroundColor $statusColor
    if ($entry.Value -notmatch "^PASS") {
        $allPassed = $false
    }
}
Write-Host ("-" * 70) -ForegroundColor Gray

if ($allPassed) {
    Write-Host "OVERALL STATUS: ALL $($script:StageResults.Count) STAGES PASSED - 100% COMPLIANT" -ForegroundColor Green
    Write-Host ("=" * 70) -ForegroundColor Cyan
    exit 0
}
else {
    Write-Host "OVERALL STATUS: ONE OR MORE STAGES FAILED" -ForegroundColor Red
    Write-Host ("=" * 70) -ForegroundColor Red
    exit 1
}
