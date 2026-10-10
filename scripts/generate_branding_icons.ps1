<#
.SYNOPSIS
    Generates multi-resolution Windows ICO and root branding assets from authoritative logo.png.
.DESCRIPTION
    Reads ui/flutter/assets/images/logo.png, resizes to standard Windows icon dimensions
    (256x256, 128x128, 64x64, 48x48, 32x32, 16x16) using high-quality bicubic interpolation
    and 32bpp ARGB alpha channel transparency, and packages them into a standards-compliant
    multi-frame ICO file for the Windows runner and distribution bundle.
#>

[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'

Add-Type -AssemblyName System.Drawing

$repoRoot = Resolve-Path "$PSScriptRoot\.."
$sourceLogo = Join-Path $repoRoot "ui\flutter\assets\images\logo.png"
$runnerIcon = Join-Path $repoRoot "ui\flutter\windows\runner\resources\app_icon.ico"
$brandingDir = Join-Path $repoRoot "branding"
$brandingLogo = Join-Path $brandingDir "logo.png"
$brandingIcon = Join-Path $brandingDir "app_icon.ico"

if (-not (Test-Path $sourceLogo)) {
    throw "Authoritative logo not found at $sourceLogo"
}

Write-Host "Authoritative logo: $sourceLogo"
Write-Host "Output Windows Runner Icon: $runnerIcon"

# 1. Copy authoritative logo.png to branding/
if (-not (Test-Path $brandingDir)) {
    New-Item -ItemType Directory -Path $brandingDir -Force | Out-Null
}
Copy-Item -Path $sourceLogo -Destination $brandingLogo -Force
Write-Host "Synced authoritative logo to $brandingLogo"

# 2. Generate multi-resolution ICO
$src = [System.Drawing.Image]::FromFile($sourceLogo)
$sizes = @(256, 128, 64, 48, 32, 16)
$pngFrames = @()

try {
    foreach ($sz in $sizes) {
        $bmp = New-Object System.Drawing.Bitmap $sz, $sz, ([System.Drawing.Imaging.PixelFormat]::Format32bppArgb)
        $g = [System.Drawing.Graphics]::FromImage($bmp)
        $g.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
        $g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::HighQuality
        $g.PixelOffsetMode = [System.Drawing.Drawing2D.PixelOffsetMode]::HighQuality
        $g.CompositingQuality = [System.Drawing.Drawing2D.CompositingQuality]::HighQuality
        $g.Clear([System.Drawing.Color]::Transparent)
        $g.DrawImage($src, 0, 0, $sz, $sz)
        $g.Dispose()

        $ms = New-Object System.IO.MemoryStream
        $bmp.Save($ms, [System.Drawing.Imaging.ImageFormat]::Png)
        $bmp.Dispose()
        $pngFrames += ,@($sz, $ms.ToArray())
        $ms.Dispose()
    }
}
finally {
    $src.Dispose()
}

function Write-IcoFile([string]$outputPath, $frames) {
    $dir = Split-Path $outputPath -Parent
    if (-not (Test-Path $dir)) {
        New-Item -ItemType Directory -Path $dir -Force | Out-Null
    }

    $fs = [System.IO.File]::Create($outputPath)
    $bw = New-Object System.IO.BinaryWriter $fs

    try {
        # ICONDIR header: idReserved (0), idType (1 for ICO), idCount
        $bw.Write([uint16]0)
        $bw.Write([uint16]1)
        $bw.Write([uint16]$frames.Count)

        # Directory entries (16 bytes each)
        $offset = 6 + ($frames.Count * 16)
        foreach ($f in $frames) {
            $sz = $f[0]
            $bytes = $f[1]

            $wByte = if ($sz -ge 256) { [byte]0 } else { [byte]$sz }
            $hByte = if ($sz -ge 256) { [byte]0 } else { [byte]$sz }

            $bw.Write($wByte)                    # bWidth
            $bw.Write($hByte)                    # bHeight
            $bw.Write([byte]0)                   # bColorCount
            $bw.Write([byte]0)                   # bReserved
            $bw.Write([uint16]1)                 # wPlanes
            $bw.Write([uint16]32)                # wBitCount
            $bw.Write([uint32]$bytes.Length)     # dwBytesInRes
            $bw.Write([uint32]$offset)           # dwImageOffset

            $offset += $bytes.Length
        }

        # Image raw payloads
        foreach ($f in $frames) {
            $bytes = $f[1]
            $bw.Write($bytes)
        }

        $bw.Flush()
    }
    finally {
        $bw.Dispose()
        $fs.Dispose()
    }
}

Write-IcoFile -outputPath $runnerIcon -frames $pngFrames
Write-Host "Created Runner app_icon.ico ($((Get-Item $runnerIcon).Length) bytes)"

Write-IcoFile -outputPath $brandingIcon -frames $pngFrames
Write-Host "Created Branding app_icon.ico ($((Get-Item $brandingIcon).Length) bytes)"

Write-Host "Branding and Windows Runner icon generation completed successfully."
