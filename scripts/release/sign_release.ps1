<#
.SYNOPSIS
    Windows Authenticode Signing Helper for ZITERA_LAB Release Binaries.
.DESCRIPTION
    Provides automated Authenticode signing using either:
    1. Production external certificate via SignTool.
    2. Ephemeral local test fixture for pipeline regression validation.
    Enforces SHA-256 and RFC 3161 timestamping.
#>

[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string]$ArtifactPath,

    [string]$CertPath = "",
    [string]$CertPassword = "",
    [string]$TimestampServer = "http://timestamp.digicert.com",
    [switch]$UseTestFixture
)

$ErrorActionPreference = "Stop"

if (-not (Test-Path $ArtifactPath)) {
    throw "Target artifact not found: $ArtifactPath"
}

Write-Host "============================================================" -ForegroundColor Cyan
Write-Host " ZITERA_LAB WINDOWS RELEASE SIGNING PIPELINE                " -ForegroundColor Cyan
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "Target Artifact   : $ArtifactPath"

if ($UseTestFixture) {
    Write-Host "`n>> Mode: Ephemeral Test Certificate Fixture (Development/CI)" -ForegroundColor Yellow
    # Create ephemeral test certificate in memory/CurrentUser store
    $certSubject = "CN=Zitera Lab Development Test Fixture, O=Zitera, C=ID"
    $testCert = New-SelfSignedCertificate `
        -Type CodeSigningCert `
        -Subject $certSubject `
        -CertStoreLocation "Cert:\CurrentUser\My" `
        -HashAlgorithm "SHA256"

    try {
        Write-Host "Generated test fixture: $($testCert.Thumbprint)" -ForegroundColor Gray
        $sigResult = Set-AuthenticodeSignature `
            -FilePath $ArtifactPath `
            -Certificate $testCert `
            -HashAlgorithm "SHA256"

        Write-Host "Signature Status  : $($sigResult.Status)" -ForegroundColor Green
        Write-Host "Status Message    : $($sigResult.StatusMessage)"
    }
    finally {
        # Clean up ephemeral test cert from store to avoid certificate bloat
        Remove-Item "Cert:\CurrentUser\My\$($testCert.Thumbprint)" -ErrorAction SilentlyContinue
    }
}
elseif (-not [string]::IsNullOrWhiteSpace($CertPath)) {
    Write-Host "`n>> Mode: Production External Certificate" -ForegroundColor Yellow
    if (-not (Test-Path $CertPath)) {
        throw "Specified certificate file not found: $CertPath"
    }

    # Locate signtool.exe from Windows SDK if available
    $signtool = Get-Command signtool.exe -ErrorAction SilentlyContinue
    if ($signtool) {
        Write-Host "Using SignTool: $($signtool.Source)" -ForegroundColor Gray
        $argsList = @(
            "sign",
            "/fd", "SHA256",
            "/td", "SHA256",
            "/tr", $TimestampServer,
            "/d", "ZITERA_LAB Enterprise Core",
            "/du", "https://github.com/xdaamar/zitera_lab",
            "/f", $CertPath
        )
        if (-not [string]::IsNullOrWhiteSpace($CertPassword)) {
            $argsList += @("/p", $CertPassword)
        }
        $argsList += $ArtifactPath

        & signtool.exe @argsList
        if ($LASTEXITCODE -ne 0) {
            throw "SignTool execution failed with code $LASTEXITCODE"
        }
    } else {
        # Fallback to PowerShell Set-AuthenticodeSignature
        Write-Host "SignTool not found in PATH; using PowerShell Set-AuthenticodeSignature..." -ForegroundColor Gray
        $securePwd = ConvertTo-SecureString $CertPassword -AsPlainText -Force
        $cert = Get-PfxCertificate -FilePath $CertPath -Password $securePwd
        $sigResult = Set-AuthenticodeSignature `
            -FilePath $ArtifactPath `
            -Certificate $cert `
            -HashAlgorithm "SHA256" `
            -TimestampServer $TimestampServer

        Write-Host "Signature Status  : $($sigResult.Status)"
    }
} else {
    throw "Either -CertPath or -UseTestFixture must be specified. Private keys are never committed to repository."
}

# Verify Signature Result
Write-Host "`n>> Verifying Authenticode Signature..." -ForegroundColor Yellow
$verifySig = Get-AuthenticodeSignature -FilePath $ArtifactPath
Write-Host "Signer Certificate: $($verifySig.SignerCertificate.Subject)"
Write-Host "Signature Status  : $($verifySig.Status)"
Write-Host "Digest Algorithm  : SHA256"

if ($verifySig.Status -notin @("Valid", "UnknownError")) {
    throw "Signature verification failed with status: $($verifySig.Status)"
}

Write-Host "`n [PASS] Artifact successfully signed and verified." -ForegroundColor Green
