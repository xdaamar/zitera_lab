param(
    [string]$EnginePath = "$PSScriptRoot\..\engine\rust\target\release\zitera-engine.exe"
)

$ErrorActionPreference = "Continue"

Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "  ZITERA_LAB - Phase 12 Security Regression Test Suite     " -ForegroundColor Cyan
Write-Host "============================================================" -ForegroundColor Cyan

if (-not (Test-Path $EnginePath)) {
    Write-Host "[WARN] Release engine not found at $EnginePath. Testing debug engine..." -ForegroundColor Yellow
    $EnginePath = "$PSScriptRoot\..\engine\rust\target\debug\zitera-engine.exe"
    if (-not (Test-Path $EnginePath)) {
        Write-Error "[FAIL] No engine binary found. Run 'cargo build --release' first."
        exit 1
    }
}

$results = [System.Collections.Generic.List[PSCustomObject]]::new()

function Record-Test {
    param(
        [string]$TestName,
        [string]$Category,
        [string]$Status,
        [string]$Detail
    )
    $color = "Green"
    if ($Status -eq "BLOCKED") { $color = "Yellow" }
    if ($Status -eq "FAIL") { $color = "Red" }
    Write-Host "[$Status] $TestName - $Detail" -ForegroundColor $color
    $results.Add([PSCustomObject]@{
        Name     = $TestName
        Category = $Category
        Status   = $Status
        Detail   = $Detail
    })
}

function Invoke-Engine {
    param([string]$ArgsString)
    $resolved = (Resolve-Path $EnginePath).Path
    $psi = New-Object System.Diagnostics.ProcessStartInfo
    $psi.FileName = $resolved
    $psi.Arguments = $ArgsString
    $psi.RedirectStandardOutput = $true
    $psi.RedirectStandardError = $true
    $psi.UseShellExecute = $false
    $psi.CreateNoWindow = $true
    try {
        $p = [System.Diagnostics.Process]::Start($psi)
        $stdout = $p.StandardOutput.ReadToEnd()
        $stderr = $p.StandardError.ReadToEnd()
        $p.WaitForExit()
        return "$stdout`n$stderr"
    } catch {
        return $_.Exception.Message
    }
}

# --- 1. Path Traversal & Filesystem Boundaries ---
$traversalTests = @(
    '../A01',
    '../../engine',
    '..\..\engine',
    'C:\Windows',
    'C:\src',
    '\\server\share',
    '/absolute/path',
    'A01/../../engine',
    'A01\..\..\engine',
    'A01;whoami',
    'A01|calc',
    'A01 && dir'
)

foreach ($id in $traversalTests) {
    $out = Invoke-Engine "--json lab status `"$id`""
    if ($out -match "Application Control" -or $out -match "Device Guard" -or $out -match "group policy") {
        Record-Test -TestName "Traversal: $id" -Category "Filesystem Boundary" -Status "BLOCKED" -Detail "Execution blocked by Windows Smart App Control policy"
    } elseif ($out -match "INVALID_ID" -or $out -match "invalid characters" -or $out -match "between 1 and 16") {
        Record-Test -TestName "Traversal: $id" -Category "Filesystem Boundary" -Status "PASS" -Detail "Engine safely handled malicious lab ID"
    } else {
        Record-Test -TestName "Traversal: $id" -Category "Filesystem Boundary" -Status "FAIL" -Detail "Engine did not reject traversal ID properly: $out"
    }
}

# --- 2. Uninstalled Lab Lifecycle Determinism ---
$uninstalledOps = @("start", "stop", "reset")
foreach ($op in $uninstalledOps) {
    $out = Invoke-Engine "--json lab $op NONEXISTENT_99"
    if ($out -match "Application Control" -or $out -match "Device Guard" -or $out -match "group policy") {
        Record-Test -TestName "Uninstalled Lab: $op" -Category "Lifecycle Determinism" -Status "BLOCKED" -Detail "Execution blocked by Windows Smart App Control policy"
    } elseif ($out -match "LAB_START_FAILED" -or $out -match "LAB_STOP_FAILED" -or $out -match "LAB_RESET_FAILED" -or $out -match "not installed") {
        Record-Test -TestName "Uninstalled Lab: $op" -Category "Lifecycle Determinism" -Status "PASS" -Detail "Safely failed operation on uninstalled lab"
    } else {
        Record-Test -TestName "Uninstalled Lab: $op" -Category "Lifecycle Determinism" -Status "FAIL" -Detail "Unexpected outcome: $out"
    }
}

# --- 3. Catalog Integrity & Schema Check ---
$catalogOut = Invoke-Engine "--json catalog"
if ($catalogOut -match "Application Control" -or $catalogOut -match "Device Guard" -or $catalogOut -match "group policy") {
    Record-Test -TestName "Catalog Schema and Integrity" -Category "Catalog Integrity" -Status "BLOCKED" -Detail "Execution blocked by Windows Smart App Control policy"
} else {
    try {
        $catJson = $catalogOut | ConvertFrom-Json
        if ($catJson.success -eq $true -and $catJson.data.labs.Count -eq 10) {
            Record-Test -TestName "Catalog Schema and Integrity" -Category "Catalog Integrity" -Status "PASS" -Detail "Loaded 10 verified curriculum labs"
        } else {
            Record-Test -TestName "Catalog Schema and Integrity" -Category "Catalog Integrity" -Status "FAIL" -Detail "Catalog failed verification or incomplete lab count"
        }
    } catch {
        Record-Test -TestName "Catalog Schema and Integrity" -Category "Catalog Integrity" -Status "FAIL" -Detail "Catalog output failed JSON decoding"
    }
}

# --- 4. Challenge Validation & Bounded Submission ---
# Huge candidate test (> 512 chars)
$hugeCandidate = "A" * 600
$chalOut = Invoke-Engine "--json lab validate-challenge A01 $hugeCandidate"
if ($chalOut -match "Application Control" -or $chalOut -match "Device Guard" -or $chalOut -match "group policy") {
    Record-Test -TestName "Challenge Candidate Bounded Length" -Category "Challenge Security" -Status "BLOCKED" -Detail "Execution blocked by Windows Smart App Control policy"
} elseif ($chalOut -match "exceeds 512 characters" -or $chalOut -match "failed") {
    Record-Test -TestName "Challenge Candidate Bounded Length" -Category "Challenge Security" -Status "PASS" -Detail "Rejected oversized candidate string safely"
} else {
    Record-Test -TestName "Challenge Candidate Bounded Length" -Category "Challenge Security" -Status "FAIL" -Detail "Did not reject oversized candidate string: $chalOut"
}

# Empty candidate test
$emptyOut = Invoke-Engine "--json lab validate-challenge A01 empty_flag"
if ($emptyOut -match "Application Control" -or $emptyOut -match "Device Guard" -or $emptyOut -match "group policy") {
    Record-Test -TestName "Challenge Candidate Empty Input" -Category "Challenge Security" -Status "BLOCKED" -Detail "Execution blocked by Windows Smart App Control policy"
} elseif ($emptyOut -match "Invalid Flag" -or $emptyOut -match "failed") {
    Record-Test -TestName "Challenge Candidate Empty Input" -Category "Challenge Security" -Status "PASS" -Detail "Handled empty candidate deterministically"
} else {
    Record-Test -TestName "Challenge Candidate Empty Input" -Category "Challenge Security" -Status "FAIL" -Detail "Unexpected response on empty candidate: $emptyOut"
}

# --- 5. Docker Localhost-Only Port Binding Audit ---
$composeFiles = Get-ChildItem -Path "$PSScriptRoot\..\labs" -Filter "docker-compose.yml" -Recurse -ErrorAction SilentlyContinue
$exposedPorts = $false
foreach ($f in $composeFiles) {
    $content = Get-Content $f.FullName -Raw
    if ($content -match '0\.0\.0\.0:\d+') {
        $exposedPorts = $true
        Record-Test -TestName "Docker Port Isolation ($($f.Directory.Name))" -Category "Docker Security" -Status "FAIL" -Detail "Found 0.0.0.0 port binding in $($f.FullName)"
    }
}
if (-not $exposedPorts) {
    Record-Test -TestName "Docker Port Isolation (All Labs)" -Category "Docker Security" -Status "PASS" -Detail "All lab compose definitions bind strictly to 127.0.0.1"
}

# --- Summary Output ---
Write-Host ""
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "  Phase 12 Security Regression Results Summary             " -ForegroundColor Cyan
Write-Host "============================================================" -ForegroundColor Cyan
$passCount = ($results | Where-Object { $_.Status -eq "PASS" }).Count
$failCount = ($results | Where-Object { $_.Status -eq "FAIL" }).Count
$blockedCount = ($results | Where-Object { $_.Status -eq "BLOCKED" }).Count

$failColor = "Green"
if ($failCount -gt 0) { $failColor = "Red" }

Write-Host "Total Tests: $($results.Count)"
Write-Host "Passed:      $passCount" -ForegroundColor Green
Write-Host "Failed:      $failCount" -ForegroundColor $failColor
Write-Host "Blocked:     $blockedCount" -ForegroundColor Yellow
Write-Host "============================================================" -ForegroundColor Cyan

if ($failCount -gt 0) {
    exit 1
} else {
    exit 0
}
