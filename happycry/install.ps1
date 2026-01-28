Write-Host "💉 Initializing HappyCry Injection Sequence..." -ForegroundColor Cyan

# 1. Dependency Check
if (-not (Get-Command "cargo" -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Rust toolchain missing. System incompatible." -ForegroundColor Red
    Write-Host "Please install Rust first: https://rustup.rs"
    exit 1
}

# 2. Directory Setup
$HappyHome = "$env:USERPROFILE\.happycry"
$BinDir = "$HappyHome\bin"
$LibDir = "$HappyHome\lib"
$ConfigDir = "$HappyHome\config"

Write-Host "📂 constructing neural pathways at $HappyHome..." -ForegroundColor Cyan

New-Item -ItemType Directory -Force -Path $BinDir | Out-Null
New-Item -ItemType Directory -Force -Path $LibDir | Out-Null
New-Item -ItemType Directory -Force -Path $ConfigDir | Out-Null

# 3. Build Process
Write-Host "🔨 Compiling binaries (This may take a moment)..." -ForegroundColor Cyan

# We can't easily do a spinner in simple PS1 without async complexity, 
# so we'll just show the build output or wait.
# Using Start-Process -Wait to ensure it finishes.
$BuildProcess = Start-Process -FilePath "cargo" -ArgumentList "build","--release","--bins" -Wait -NoNewWindow -PassThru

if ($BuildProcess.ExitCode -ne 0) {
    Write-Host "❌ Build Failed." -ForegroundColor Red
    exit 1
}

Write-Host "✅ Compilation Successful." -ForegroundColor Green

# 4. Installation
Write-Host "📦 Moving artifacts to $BinDir..." -ForegroundColor Cyan

$HappySrc = "target\release\happy.exe"
$VirusSrc = "target\release\virus.exe"

if (Test-Path $HappySrc) {
    Copy-Item -Path $HappySrc -Destination $BinDir -Force
} else {
    Write-Host "❌ Error: 'happy.exe' binary not found." -ForegroundColor Red
    exit 1
}

if (Test-Path $VirusSrc) {
    Copy-Item -Path $VirusSrc -Destination $BinDir -Force
} else {
    Write-Host "⚠️  Warning: 'virus.exe' binary not found." -ForegroundColor Yellow
}

# 5. Path Configuration
Write-Host "🔗 Linking to User Path..." -ForegroundColor Cyan

$CurrentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($CurrentPath -notlike "*$BinDir*") {
    $NewPath = "$CurrentPath;$BinDir"
    [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
    Write-Host "✅ Path injected." -ForegroundColor Green
} else {
    Write-Host "⚡ Path already configured." -ForegroundColor Yellow
}

Write-Host "`n=========================================" -ForegroundColor Green
Write-Host " 💉 INJECTION COMPLETE. HAPPYCRY IS ACTIVE." -ForegroundColor Green
Write-Host "=========================================" -ForegroundColor Green
Write-Host "Please restart your terminal to use the 'happy' and 'virus' commands."
