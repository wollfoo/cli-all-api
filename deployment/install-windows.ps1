#Requires -RunAsAdministrator
<#
.SYNOPSIS
    ProxyPal Windows Service Installation Script
    Cài đặt ProxyPal như Windows Service sử dụng NSSM

.DESCRIPTION
    Script này thực hiện:
    - Kiểm tra và download NSSM nếu cần
    - Cài đặt ProxyPal binary vào Program Files
    - Tạo thư mục config
    - Đăng ký Windows Service với recovery options

.PARAMETER BinaryPath
    Đường dẫn đến proxypal.exe (default: current directory)

.PARAMETER ServiceName
    Tên service (default: ProxyPal)

.PARAMETER Port
    Port proxy server (default: 8317)

.EXAMPLE
    .\install-windows.ps1
    
.EXAMPLE
    .\install-windows.ps1 -BinaryPath "C:\Downloads\proxypal.exe" -Port 9000

.NOTES
    Yêu cầu: PowerShell 5.1+, Administrator rights
    NSSM: https://nssm.cc/
#>

[CmdletBinding()]
param(
    [Parameter()]
    [string]$BinaryPath = ".\proxypal.exe",
    
    [Parameter()]
    [string]$ServiceName = "ProxyPal",
    
    [Parameter()]
    [int]$Port = 8317
)

# ============================================================================
# Configuration
# ============================================================================

$ErrorActionPreference = "Stop"

$InstallDir = "C:\Program Files\ProxyPal"
$ConfigDir = "$env:APPDATA\ProxyPal"
$NssmDir = "C:\Tools\nssm"
$NssmExe = "$NssmDir\nssm.exe"
$NssmUrl = "https://nssm.cc/release/nssm-2.24.zip"
$LogDir = "$InstallDir\logs"

# ============================================================================
# Helper Functions
# ============================================================================

function Write-Status {
    param([string]$Message, [string]$Type = "INFO")
    
    $color = switch ($Type) {
        "INFO"    { "Cyan" }
        "SUCCESS" { "Green" }
        "WARNING" { "Yellow" }
        "ERROR"   { "Red" }
        default   { "White" }
    }
    
    $prefix = switch ($Type) {
        "INFO"    { "[*]" }
        "SUCCESS" { "[+]" }
        "WARNING" { "[!]" }
        "ERROR"   { "[-]" }
        default   { "[*]" }
    }
    
    Write-Host "$prefix $Message" -ForegroundColor $color
}

function Test-IsAdmin {
    $currentPrincipal = New-Object Security.Principal.WindowsPrincipal([Security.Principal.WindowsIdentity]::GetCurrent())
    return $currentPrincipal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

function Install-NSSM {
    if (Test-Path $NssmExe) {
        Write-Status "NSSM already installed at $NssmExe" "INFO"
        return
    }
    
    Write-Status "Downloading NSSM..." "INFO"
    
    $zipPath = "$env:TEMP\nssm.zip"
    $extractPath = "$env:TEMP\nssm-extract"
    
    try {
        # Download
        [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
        Invoke-WebRequest -Uri $NssmUrl -OutFile $zipPath -UseBasicParsing
        
        # Extract
        if (Test-Path $extractPath) {
            Remove-Item $extractPath -Recurse -Force
        }
        Expand-Archive -Path $zipPath -DestinationPath $extractPath -Force
        
        # Create tools directory
        if (-not (Test-Path $NssmDir)) {
            New-Item -ItemType Directory -Path $NssmDir -Force | Out-Null
        }
        
        # Copy nssm.exe (64-bit)
        $nssmSource = Get-ChildItem -Path $extractPath -Filter "nssm.exe" -Recurse | 
                      Where-Object { $_.Directory.Name -eq "win64" } | 
                      Select-Object -First 1
        
        if (-not $nssmSource) {
            # Fallback to win32
            $nssmSource = Get-ChildItem -Path $extractPath -Filter "nssm.exe" -Recurse | 
                          Select-Object -First 1
        }
        
        if ($nssmSource) {
            Copy-Item $nssmSource.FullName -Destination $NssmExe -Force
            Write-Status "NSSM installed at $NssmExe" "SUCCESS"
        } else {
            throw "Could not find nssm.exe in downloaded archive"
        }
        
    } finally {
        # Cleanup
        if (Test-Path $zipPath) { Remove-Item $zipPath -Force }
        if (Test-Path $extractPath) { Remove-Item $extractPath -Recurse -Force }
    }
}

function Stop-ExistingService {
    $service = Get-Service -Name $ServiceName -ErrorAction SilentlyContinue
    
    if ($service) {
        Write-Status "Stopping existing service..." "INFO"
        
        if ($service.Status -eq "Running") {
            Stop-Service -Name $ServiceName -Force
            Start-Sleep -Seconds 2
        }
        
        # Remove existing service
        Write-Status "Removing existing service..." "INFO"
        & $NssmExe remove $ServiceName confirm 2>$null
        Start-Sleep -Seconds 1
    }
}

function Install-ProxyPalBinary {
    Write-Status "Installing ProxyPal binary..." "INFO"
    
    # Verify source binary exists
    if (-not (Test-Path $BinaryPath)) {
        throw "Binary not found: $BinaryPath`nPlease specify correct path with -BinaryPath parameter"
    }
    
    # Create install directory
    if (-not (Test-Path $InstallDir)) {
        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    }
    
    # Create log directory
    if (-not (Test-Path $LogDir)) {
        New-Item -ItemType Directory -Path $LogDir -Force | Out-Null
    }
    
    # Copy binary
    Copy-Item $BinaryPath -Destination "$InstallDir\proxypal.exe" -Force
    
    Write-Status "Binary installed at $InstallDir\proxypal.exe" "SUCCESS"
}

function New-ConfigDirectory {
    Write-Status "Creating config directory..." "INFO"
    
    if (-not (Test-Path $ConfigDir)) {
        New-Item -ItemType Directory -Path $ConfigDir -Force | Out-Null
    }
    
    # Create default config if not exists
    $configFile = "$ConfigDir\config.yaml"
    if (-not (Test-Path $configFile)) {
        $defaultConfig = @"
# ProxyPal Configuration File
# Được tạo tự động bởi install-windows.ps1

# Server port
port: $Port

# Debug mode
debug: false

# Logging level (trace, debug, info, warn, error)
log_level: info

# Provider configurations (thêm API keys của bạn)
# providers:
#   gemini:
#     api_key: "YOUR_GEMINI_API_KEY"
#   claude:
#     api_key: "YOUR_CLAUDE_API_KEY"
"@
        Set-Content -Path $configFile -Value $defaultConfig -Encoding UTF8
        Write-Status "Default config created at $configFile" "SUCCESS"
    } else {
        Write-Status "Config file already exists, keeping existing" "INFO"
    }
}

function Register-ProxyPalService {
    Write-Status "Registering Windows Service..." "INFO"
    
    $exePath = "$InstallDir\proxypal.exe"
    $configPath = "$ConfigDir\config.yaml"
    
    # Install service with nssm
    & $NssmExe install $ServiceName $exePath serve --foreground --config $configPath
    
    if ($LASTEXITCODE -ne 0) {
        throw "Failed to install service with NSSM"
    }
    
    # Configure service
    & $NssmExe set $ServiceName AppDirectory $InstallDir
    & $NssmExe set $ServiceName DisplayName "ProxyPal - AI Proxy Server"
    & $NssmExe set $ServiceName Description "ProxyPal proxy server for AI coding agents. Routes requests to Claude, Gemini, OpenAI, etc."
    & $NssmExe set $ServiceName Start SERVICE_AUTO_START
    
    # Environment variables
    & $NssmExe set $ServiceName AppEnvironmentExtra "RUST_LOG=info"
    & $NssmExe set $ServiceName AppEnvironmentExtra "+PROXYPAL_CONFIG=$configPath"
    
    # Logging
    & $NssmExe set $ServiceName AppStdout "$LogDir\proxypal.log"
    & $NssmExe set $ServiceName AppStderr "$LogDir\proxypal-error.log"
    & $NssmExe set $ServiceName AppRotateFiles 1
    & $NssmExe set $ServiceName AppRotateBytes 10485760  # 10MB
    
    # Graceful shutdown
    & $NssmExe set $ServiceName AppStopMethodSkip 0
    & $NssmExe set $ServiceName AppStopMethodConsole 3000
    & $NssmExe set $ServiceName AppStopMethodWindow 3000
    & $NssmExe set $ServiceName AppStopMethodThreads 3000
    
    Write-Status "Service registered successfully" "SUCCESS"
}

function Set-ServiceRecovery {
    Write-Status "Configuring service recovery options..." "INFO"
    
    # Recovery options:
    # First failure: Restart service immediately
    # Second failure: Restart service after 5 seconds
    # Subsequent: Restart service after 5 minutes (300000 ms)
    # Reset fail count after 1 day (86400 seconds)
    
    $recoveryCmd = "sc.exe failure $ServiceName reset= 86400 actions= restart/0/restart/5000/restart/300000"
    Invoke-Expression $recoveryCmd | Out-Null
    
    Write-Status "Recovery options configured" "SUCCESS"
}

function Start-ProxyPalService {
    Write-Status "Starting ProxyPal service..." "INFO"
    
    Start-Service -Name $ServiceName
    Start-Sleep -Seconds 3
    
    $service = Get-Service -Name $ServiceName
    if ($service.Status -eq "Running") {
        Write-Status "Service started successfully!" "SUCCESS"
    } else {
        Write-Status "Service may have failed to start. Check logs at $LogDir" "WARNING"
    }
}

# ============================================================================
# Main Installation
# ============================================================================

function Main {
    Write-Host ""
    Write-Host "╔══════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
    Write-Host "║           ProxyPal Windows Service Installer             ║" -ForegroundColor Cyan
    Write-Host "╚══════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
    Write-Host ""
    
    # Check admin
    if (-not (Test-IsAdmin)) {
        Write-Status "This script requires Administrator privileges" "ERROR"
        Write-Status "Please run PowerShell as Administrator and try again" "ERROR"
        exit 1
    }
    
    try {
        # Step 1: Install NSSM
        Install-NSSM
        
        # Step 2: Stop existing service if any
        Stop-ExistingService
        
        # Step 3: Install binary
        Install-ProxyPalBinary
        
        # Step 4: Create config directory
        New-ConfigDirectory
        
        # Step 5: Register service
        Register-ProxyPalService
        
        # Step 6: Configure recovery
        Set-ServiceRecovery
        
        # Step 7: Start service
        Start-ProxyPalService
        
        # Summary
        Write-Host ""
        Write-Host "╔══════════════════════════════════════════════════════════╗" -ForegroundColor Green
        Write-Host "║              Installation Complete!                      ║" -ForegroundColor Green
        Write-Host "╠══════════════════════════════════════════════════════════╣" -ForegroundColor Green
        Write-Host "║  Service Name:  $ServiceName" -ForegroundColor Green
        Write-Host "║  Install Dir:   $InstallDir" -ForegroundColor Green
        Write-Host "║  Config Dir:    $ConfigDir" -ForegroundColor Green
        Write-Host "║  Log Dir:       $LogDir" -ForegroundColor Green
        Write-Host "║  Port:          $Port" -ForegroundColor Green
        Write-Host "╠══════════════════════════════════════════════════════════╣" -ForegroundColor Green
        Write-Host "║  Useful Commands:" -ForegroundColor Green
        Write-Host "║    Get-Service ProxyPal        - Check status" -ForegroundColor Green
        Write-Host "║    Start-Service ProxyPal      - Start service" -ForegroundColor Green
        Write-Host "║    Stop-Service ProxyPal       - Stop service" -ForegroundColor Green
        Write-Host "║    Restart-Service ProxyPal    - Restart service" -ForegroundColor Green
        Write-Host "╚══════════════════════════════════════════════════════════╝" -ForegroundColor Green
        Write-Host ""
        
    } catch {
        Write-Status "Installation failed: $_" "ERROR"
        exit 1
    }
}

# Run main
Main
