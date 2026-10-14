# ZITERA_LAB Curriculum Validator Test Suite
# Tests positive and negative fixture scenarios to guarantee validator correctness.
[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$repoRoot = Split-Path -Parent $scriptDir
$validatorScript = Join-Path $repoRoot "scripts\validate_curriculum.ps1"
$fixturesBase = Join-Path $repoRoot "tests\fixtures\curriculum"

Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host "  CURRICULUM VALIDATOR TEST SUITE (PHASE 1 - CP03)       " -ForegroundColor Cyan
Write-Host "==========================================================" -ForegroundColor Cyan

$testCases = @(
    @{
        Name = "TC01: Valid Lab Fixture"
        FixtureDir = Join-Path $fixturesBase "valid_lab"
        ExpectSuccess = $true
        ExpectedMatch = "ALL CURRICULUM QUALITY GATES PASSED"
    },
    @{
        Name = "TC02: Duplicate Lab ID Detection"
        FixtureDir = Join-Path $fixturesBase "invalid_duplicate_id"
        ExpectSuccess = $false
        ExpectedMatch = "Duplicate lab ID found"
    },
    @{
        Name = "TC03: Unsupported Locale Detection"
        FixtureDir = Join-Path $fixturesBase "invalid_locale"
        ExpectSuccess = $false
        ExpectedMatch = "unsupported locale"
    },
    @{
        Name = "TC04: Challenge Flag Leak Detection"
        FixtureDir = Join-Path $fixturesBase "invalid_flag_leak"
        ExpectSuccess = $false
        ExpectedMatch = "CHALLENGE_FLAG_LEAK_DETECTED"
    },
    @{
        Name = "TC05: Missing Learning Objectives Detection"
        FixtureDir = Join-Path $fixturesBase "invalid_missing_objectives"
        ExpectSuccess = $false
        ExpectedMatch = "fewer than 3 learning objectives"
    },
    @{
        Name = "TC06: Non-Canonical Flag Format Detection"
        FixtureDir = Join-Path $fixturesBase "invalid_flag_format"
        ExpectSuccess = $false
        ExpectedMatch = "contains non-canonical flag format"
    }
)

$passed = 0
$failed = 0

foreach ($tc in $testCases) {
    Write-Host -NoNewline "`n[RUNNING] $($tc.Name)... " -ForegroundColor Yellow

    # Execute validator in isolated sub-process
    $cmd = "powershell -ExecutionPolicy Bypass -File `"$validatorScript`" -LabsDir `"$($tc.FixtureDir)`" -SkipCatalog 2>&1"
    $output = Invoke-Expression $cmd | Out-String
    $exitCode = $LASTEXITCODE

    $isMatch = $output -match [regex]::Escape($tc.ExpectedMatch)

    if ($tc.ExpectSuccess) {
        if ($exitCode -eq 0 -and $isMatch) {
            Write-Host "PASS (Clean Exit 0, Matched expected output)" -ForegroundColor Green
            $passed++
        } else {
            Write-Host "FAIL (Expected Exit 0, Got $exitCode)" -ForegroundColor Red
            Write-Host "Output: $output" -ForegroundColor DarkGray
            $failed++
        }
    } else {
        if ($exitCode -ne 0 -and $isMatch) {
            Write-Host "PASS (Caught expected failure, Exit $exitCode, Matched pattern)" -ForegroundColor Green
            $passed++
        } else {
            Write-Host "FAIL (Expected failure with '$($tc.ExpectedMatch)', Exit $exitCode)" -ForegroundColor Red
            Write-Host "Output: $output" -ForegroundColor DarkGray
            $failed++
        }
    }
}

Write-Host "`n==========================================================" -ForegroundColor Cyan
Write-Host "  TEST RESULTS: $passed PASSED, $failed FAILED" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "==========================================================" -ForegroundColor Cyan

if ($failed -gt 0) {
    exit 1
} else {
    exit 0
}
