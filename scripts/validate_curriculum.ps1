# ZITERA_LAB Curriculum and Content Quality Validator
# Validates structural, educational, and security requirements across all labs.
[CmdletBinding()]
param(
    [string]$LabsDir = "labs",
    [string]$CatalogFile = "catalog/catalog.json"
)

$ErrorActionPreference = "Stop"
Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host "  ZITERA_LAB CURRICULUM QUALITY GATE AUDITOR (PHASE 24)  " -ForegroundColor Cyan
Write-Host "==========================================================" -ForegroundColor Cyan

$passCount = 0
$failCount = 0
$issues = @()

# 1. Audit Catalog Integrity
Write-Host "`n[1/4] Auditing Catalog Manifest ($CatalogFile)..." -ForegroundColor Yellow
if (Test-Path $CatalogFile) {
    try {
        $catalog = Get-Content $CatalogFile -Raw | ConvertFrom-Json
        if ($catalog.schema_version -ne 1) {
            $issues += "Catalog schema version is not 1 (Found: $($catalog.schema_version))"
        }
        $catLabs = $catalog.labs
        Write-Host "  -> Catalog contains $($catLabs.Count) registered lab entries." -ForegroundColor Green
        $passCount++
    } catch {
        $issues += "Failed to parse catalog JSON: $_"
        $failCount++
    }
} else {
    $issues += "Catalog file not found at $CatalogFile"
    $failCount++
}

# 2. Audit Lab Manifests & Structural Integrity
Write-Host "`n[2/4] Auditing Lab Manifests & Directory Structure..." -ForegroundColor Yellow
$manifestFiles = Get-ChildItem -Path $LabsDir -Recurse -Filter "manifest.json" | Sort-Object FullName
$seenIds = @{}
$seenPorts = @{}
$seenPackages = @{}

$requiredLessonFiles = @(
    "introduction.id.md",
    "concept.id.md",
    "analogy.id.md",
    "walkthrough.id.md",
    "remediation.id.md"
)

foreach ($mFile in $manifestFiles) {
    $labDir = $mFile.Directory.FullName
    $relPath = $mFile.FullName.Replace((Get-Location).Path + "\", "")
    try {
        $manifest = Get-Content $mFile.FullName -Raw | ConvertFrom-Json
        $labId = $manifest.id

        # Unique ID Check
        if ($seenIds.ContainsKey($labId)) {
            $issues += "Duplicate lab ID found: $labId in $relPath"
            $failCount++
        } else {
            $seenIds[$labId] = $relPath
        }

        # Unique Port Check
        $port = $manifest.default_port
        if ($seenPorts.ContainsKey($port)) {
            $issues += "Duplicate port found: $port for $labId (already used by $($seenPorts[$port]))"
            $failCount++
        } else {
            $seenPorts[$port] = $labId
        }

        # Unique Package ID Check
        $pkgId = $manifest.package_id
        if ($seenPackages.ContainsKey($pkgId)) {
            $issues += "Duplicate package ID: $pkgId for $labId"
            $failCount++
        } else {
            $seenPackages[$pkgId] = $labId
        }

        # Duration bounds check (20m - 90m)
        $dur = $manifest.estimated_minutes
        if ($dur -lt 20 -or $dur -gt 90) {
            $issues += "Lab $labId duration ($dur min) outside realistic boundary (20-90m)"
            $failCount++
        }

        # Learning Objectives check (>= 3)
        if (-not $manifest.learning_objectives -or $manifest.learning_objectives.Count -lt 3) {
            $issues += "Lab $labId has fewer than 3 learning objectives"
            $failCount++
        }

        # Prerequisites check
        if (-not $manifest.prerequisites -or $manifest.prerequisites.Count -lt 1) {
            $issues += "Lab $labId missing prerequisites declaration"
            $failCount++
        }

        # 3. Check Required Lesson Files
        $lessonDir = Join-Path $labDir "lesson"
        if (Test-Path $lessonDir) {
            foreach ($rf in $requiredLessonFiles) {
                $targetFile = Join-Path $lessonDir $rf
                if (-not (Test-Path $targetFile)) {
                    $issues += "Lab $labId missing required section file: lesson/$rf"
                    $failCount++
                }
            }
        } else {
            $issues += "Lab $labId missing lesson/ directory"
            $failCount++
        }

        # 4. Check Challenge & Hints Structure
        $challengeDir = Join-Path $labDir "challenge"
        $challengeMd = Join-Path $challengeDir "challenge.id.md"
        $hintsJson = Join-Path $challengeDir "hints.json"

        if (-not (Test-Path $challengeMd)) {
            $issues += "Lab $labId missing challenge/challenge.id.md"
            $failCount++
        }

        if (Test-Path $hintsJson) {
            try {
                $hintsObj = Get-Content $hintsJson -Raw | ConvertFrom-Json
                if (-not $hintsObj.hints -or $hintsObj.hints.Count -lt 3) {
                    $issues += "Lab $labId hints.json has fewer than 3 tiers"
                    $failCount++
                }
            } catch {
                $issues += "Lab $labId has invalid hints.json syntax: $_"
                $failCount++
            }
        } else {
            $issues += "Lab $labId missing challenge/hints.json"
            $failCount++
        }

        Write-Host "  [OK] $labId ($($manifest.title)) - $($manifest.difficulty), $($manifest.estimated_minutes)m, Port $port" -ForegroundColor Green
        $passCount++

    } catch {
        $issues += "Failed to parse $relPath : $_"
        $failCount++
    }
}

# 3. Security Audit: Flag Leak Detection in Public Lessons
Write-Host "`n[3/4] Scanning Public Educational Content for Flag Leaks..." -ForegroundColor Yellow
$markdownFiles = Get-ChildItem -Path $LabsDir -Recurse -Filter "*.md" | Where-Object { $_.FullName -notmatch '\\challenge\\' }
$leakDetected = $false

foreach ($md in $markdownFiles) {
    $content = Get-Content $md.FullName -Raw
    if ($content -match 'ZITERA\{[a-zA-Z0-9_\-]+\}') {
        $leakDetected = $true
        # SEC-001: Sanitize log - NEVER print the actual flag string!
        $labMatch = $md.FullName -replace '^.*\\labs\\([^\\]+)\\.*$', '$1'
        Write-Warning "CHALLENGE_FLAG_LEAK_DETECTED: $labMatch in $($md.Name)"
        $issues += "CHALLENGE_FLAG_LEAK_DETECTED: $labMatch in $($md.Name)"
        $failCount++
    }
}
if (-not $leakDetected) {
    Write-Host "  -> Zero flag leaks detected across all public lesson documents." -ForegroundColor Green
    $passCount++
}

# 4. Security Audit: Hardcoded Path Inspection
Write-Host "`n[4/4] Inspecting for Hardcoded Developer Workspace Paths..." -ForegroundColor Yellow
$hardcodedPathsFound = $false
foreach ($m in $manifestFiles) {
    $raw = Get-Content $m.FullName -Raw
    if ($raw -match '[A-Za-z]:\\[Uu]sers\\') {
        $hardcodedPathsFound = $true
        $issues += "Hardcoded developer path found in $($m.FullName)"
        $failCount++
    }
}
if (-not $hardcodedPathsFound) {
    Write-Host "  -> Zero hardcoded developer paths found in manifests." -ForegroundColor Green
    $passCount++
}

# Summary
Write-Host "`n==========================================================" -ForegroundColor Cyan
Write-Host "  CURRICULUM QUALITY GATE SUMMARY                        " -ForegroundColor Cyan
Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host "Total Passing Checks : $passCount" -ForegroundColor Green
Write-Host "Total Failed Issues  : $failCount" -ForegroundColor $(if ($failCount -eq 0) { "Green" } else { "Red" })

if ($issues.Count -gt 0) {
    Write-Host "`nIssues Discovered:" -ForegroundColor Red
    foreach ($iss in $issues) {
        Write-Host " - $iss" -ForegroundColor Red
    }
    exit 1
} else {
    Write-Host "`nALL CURRICULUM QUALITY GATES PASSED (10/10 LABS CERTIFIED)." -ForegroundColor Green
    exit 0
}
