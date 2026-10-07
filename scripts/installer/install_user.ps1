<#
.SYNOPSIS
    ZITERA_LAB Non-Admin Windows Per-User Installer Engine.
.DESCRIPTION
    Implements CP06/CP09 Windows distribution installer lifecycle:
    - Install, Upgrade, Uninstall, Repair, Status
    - Strict per-user boundary: %LOCALAPPDATA%\Programs\ZiteraLab
    - Guaranteed student data preservation: %LOCALAPPDATA%\ZiteraLab\user
    - Zero administrative elevation, zero WSL, zero Docker, zero firewall modification
#>

[CmdletBinding()]
param(
    [ValidateSet("Install", "Uninstall", "Upgrade", "Repair", "Status")]
    [string]$Action = "Install",

    [string]$SourceBundle = "",
    [string]$TargetDir = "",
    [string]$DataDir = "",
    [string]$Version = "2.0.0",
    [switch]$PurgeUserData,
    [switch]$Silent
)

$ErrorActionPreference = "Stop"

$script:Root = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)

# 1. Resolve Target Directories
if ([string]::IsNullOrWhiteSpace($TargetDir)) {
    $script:AppDir = Join-Path $env:LOCALAPPDATA "Programs\ZiteraLab"
} else {
    $script:AppDir = $TargetDir
}

if ([string]::IsNullOrWhiteSpace($DataDir)) {
    $script:UserDataDir = Join-Path $env:LOCALAPPDATA "ZiteraLab"
} else {
    $script:UserDataDir = $DataDir
}

if ([string]::IsNullOrWhiteSpace($SourceBundle)) {
    $script:SourceDir = Join-Path $script:Root "dist"
} else {
    $script:SourceDir = $SourceBundle
}

$script:UserProgressDir = Join-Path $script:UserDataDir "user"
$script:UserLabsDir = Join-Path $script:UserDataDir "labs"
$script:UserCacheDir = Join-Path $script:UserDataDir "cache"
$script:UserLogsDir = Join-Path $script:UserDataDir "logs"
$script:UserDiagnosticsDir = Join-Path $script:UserDataDir "diagnostics"
$script:InstallManifest = Join-Path $script:AppDir "install_manifest.json"
$script:StartMenuDir = Join-Path $env:APPDATA "Microsoft\Windows\Start Menu\Programs\ZiteraLab"

function Write-Info([string]$msg) {
    if (-not $Silent) {
        Write-Host $msg -ForegroundColor Cyan
    }
}

function Write-Success([string]$msg) {
    if (-not $Silent) {
        Write-Host " [SUCCESS] $msg" -ForegroundColor Green
    }
}

function Stop-RunningZiteraProcesses {
    Get-Process -Name "zitera-engine", "lab" -ErrorAction SilentlyContinue | Where-Object {
        $_.Path -like "$script:AppDir*"
    } | Stop-Process -Force -ErrorAction SilentlyContinue
    Start-Sleep -Milliseconds 200
}

function Invoke-InstallStep([bool]$isUpgrade = $false) {
    Stop-RunningZiteraProcesses

    # Locate source binary
    $sourceBin = Join-Path $script:SourceDir "bin\zitera-engine.exe"
    if (-not (Test-Path $sourceBin)) {
        # Fallback to engine release build directory if dist not yet prepared
        $sourceBin = Join-Path $script:Root "engine\rust\target\release\zitera-engine.exe"
    }
    if (-not (Test-Path $sourceBin)) {
        throw "Source binary not found at $sourceBin. Run release build first."
    }

    $binHash = (Get-FileHash -Path $sourceBin -Algorithm SHA256).Hash
    $binSize = (Get-Item $sourceBin).Length

    # Create immutable application directory (Tier 1)
    $targetBinDir = Join-Path $script:AppDir "bin"
    New-Item -ItemType Directory -Path $targetBinDir -Force | Out-Null

    $targetBin = Join-Path $targetBinDir "zitera-engine.exe"
    Copy-Item $sourceBin $targetBin -Force

    # Copy catalog if available
    $sourceCatalog = Join-Path $script:Root "catalog"
    if (Test-Path $sourceCatalog) {
        Copy-Item $sourceCatalog (Join-Path $script:AppDir "catalog") -Recurse -Force
    }

    # Initialize User Data Directories (Tier 2-6: User, Labs, Cache, Logs, Diagnostics)
    New-Item -ItemType Directory -Path $script:UserProgressDir -Force | Out-Null
    New-Item -ItemType Directory -Path $script:UserLabsDir -Force | Out-Null
    New-Item -ItemType Directory -Path $script:UserCacheDir -Force | Out-Null
    New-Item -ItemType Directory -Path $script:UserLogsDir -Force | Out-Null
    New-Item -ItemType Directory -Path $script:UserDiagnosticsDir -Force | Out-Null

    $progressFile = Join-Path $script:UserProgressDir "progress.json"
    if (-not (Test-Path $progressFile)) {
        $initialProgress = @"
{
  "schema_version": 1,
  "completed_labs": [],
  "current_streak_days": 0,
  "earned_flags": [],
  "last_activity": null
}
"@
        [System.IO.File]::WriteAllText($progressFile, $initialProgress, [System.Text.UTF8Encoding]::new($false))
    }

    $preferencesFile = Join-Path $script:UserProgressDir "preferences.json"
    if (-not (Test-Path $preferencesFile)) {
        $initialPrefs = @"
{
  "theme": "dark",
  "font_size": 14,
  "terminal_shell": "internal_sandboxed"
}
"@
        [System.IO.File]::WriteAllText($preferencesFile, $initialPrefs, [System.Text.UTF8Encoding]::new($false))
    }

    # Write install manifest
    $prevVer = $null
    if (Test-Path $script:InstallManifest) {
        try {
            $prev = Get-Content $script:InstallManifest -Raw | ConvertFrom-Json
            $prevVer = $prev.version
        } catch {}
    }

    $manifest = [PSCustomObject]@{
        product = "ZITERA_LAB"
        version = $Version
        previous_version = $prevVer
        installed_at = [System.DateTime]::UtcNow.ToString("yyyy-MM-ddTHH:mm:ssZ")
        app_directory = $script:AppDir
        data_directory = $script:UserDataDir
        binary_sha256 = $binHash
        binary_size = $binSize
        per_user = $true
        elevation_required = $false
    }
    $manifestJson = $manifest | ConvertTo-Json -Depth 4
    [System.IO.File]::WriteAllText($script:InstallManifest, $manifestJson, [System.Text.UTF8Encoding]::new($false))

    # Create Start Menu Shortcuts (non-fatal if environment is restricted or sandboxed)
    try {
        if (-not (Test-Path $script:StartMenuDir)) {
            New-Item -ItemType Directory -Path $script:StartMenuDir -Force -ErrorAction Stop | Out-Null
        }
        $wscript = New-Object -ComObject WScript.Shell
        $shortcut = $wscript.CreateShortcut((Join-Path $script:StartMenuDir "Zitera Lab CLI.lnk"))
        $shortcut.TargetPath = $targetBin
        $shortcut.Arguments = "doctor"
        $shortcut.WorkingDirectory = $script:AppDir
        $shortcut.Description = "ZITERA_LAB Security Education Environment"
        $shortcut.Save()
    } catch {
        Write-Info "Start Menu shortcut creation skipped or restricted: $_"
    }

    if ($isUpgrade) {
        Write-Success "Upgraded ZITERA_LAB to version $Version (Previous: $prevVer)"
    } else {
        Write-Success "Installed ZITERA_LAB version $Version to $script:AppDir"
    }
    Write-Info "Application Path : $targetBin"
    Write-Info "User Data Path   : $script:UserDataDir (Student progress preserved)"

    return $manifest
}

function Invoke-UninstallStep {
    Stop-RunningZiteraProcesses

    # Remove Start Menu Shortcuts
    if (Test-Path $script:StartMenuDir) {
        Remove-Item -Path $script:StartMenuDir -Recurse -Force -ErrorAction SilentlyContinue
    }

    # Remove Application Binaries
    if (Test-Path $script:AppDir) {
        Remove-Item -Path $script:AppDir -Recurse -Force
        Write-Success "Application binaries removed from $script:AppDir"
    }

    # User Data Preservation Policy (CP10 Storage Boundary Contract)
    if ($PurgeUserData) {
        if (Test-Path $script:UserDataDir) {
            Remove-Item -Path $script:UserDataDir -Recurse -Force
            Write-Success "User data purged from $script:UserDataDir as requested."
        }
    } else {
        # Default uninstallation purges disposable cache, but preserves student progress, labs, logs, diagnostics
        if (Test-Path $script:UserCacheDir) {
            Remove-Item -Path $script:UserCacheDir -Recurse -Force -ErrorAction SilentlyContinue
        }
        Write-Success "Student progress & lab data retained in $script:UserDataDir"
    }
}

function Invoke-RepairStep {
    if (-not (Test-Path $script:InstallManifest)) {
        throw "Cannot repair: ZITERA_LAB installation manifest not found at $script:InstallManifest"
    }
    $manifest = Get-Content $script:InstallManifest -Raw | ConvertFrom-Json
    Write-Info "Validating installation integrity against manifest v$($manifest.version)..."

    $targetBin = Join-Path $script:AppDir "bin\zitera-engine.exe"
    $reinstallNeeded = $false

    if (-not (Test-Path $targetBin)) {
        Write-Host "Missing binary detected: $targetBin" -ForegroundColor Yellow
        $reinstallNeeded = $true
    } else {
        $currentHash = (Get-FileHash -Path $targetBin -Algorithm SHA256).Hash
        if ($currentHash -ne $manifest.binary_sha256) {
            Write-Host "Binary hash mismatch! Expected $($manifest.binary_sha256), found $currentHash" -ForegroundColor Yellow
            $reinstallNeeded = $true
        }
    }

    if ($reinstallNeeded) {
        Write-Info "Repairing application binaries..."
        Invoke-InstallStep -isUpgrade $false | Out-Null
        Write-Success "Repair complete: application binaries successfully restored."
    } else {
        Write-Success "Binary integrity confirmed: 100% matching manifest."
    }
}

function Get-InstallationStatus {
    $isInstalled = Test-Path $script:InstallManifest
    $statusObj = [PSCustomObject]@{
        Installed = $isInstalled
        AppDirectory = $script:AppDir
        DataDirectory = $script:UserDataDir
        Version = $null
        BinaryHash = $null
        UserProgressPreserved = (Test-Path (Join-Path $script:UserProgressDir "progress.json"))
    }
    if ($isInstalled) {
        $manifest = Get-Content $script:InstallManifest -Raw | ConvertFrom-Json
        $statusObj.Version = $manifest.version
        $statusObj.BinaryHash = $manifest.binary_sha256
    }
    return $statusObj
}

# Execute Requested Action
switch ($Action) {
    "Install" {
        Invoke-InstallStep -isUpgrade $false
    }
    "Upgrade" {
        Invoke-InstallStep -isUpgrade $true
    }
    "Uninstall" {
        Invoke-UninstallStep
    }
    "Repair" {
        Invoke-RepairStep
    }
    "Status" {
        $st = Get-InstallationStatus
        $st | Format-List
        return $st
    }
}
