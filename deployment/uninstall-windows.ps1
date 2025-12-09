#Requires -RunAsAdministrator
<#
.SYNOPSIS
    ProxyPal Windows Service Uninstallation Script
    Gỡ cài đặt ProxyPal Windows Service

.DESCRIPTION
    Script này thực hiện:
    - Dừng service nếu đang chạy
    - Xóa service khỏi Windows
    - Tùy chọn xóa program files và config

.PARAMETER RemoveAll
    Xóa tất cả files bao gồm config và logs

.PARAMETER KeepConfig
    Giữ lại config files (default: true)

.PARAMETER ServiceName
    Tên service (default: ProxyPal)

.EXAMPLE
    .\uninstall-windows.ps1
    
.EXAMPLE
    .\uninstall-windows.ps1 -RemoveAll

.NOTES
    Yêu cầu: PowerShell 5.1+, Administrator rights
#>

[CmdletBinding()]
param(
    [Parameter()]
    [switch]$RemoveAll,
    
    [Parameter()]
    [switch]$KeepConfig = $true,
    
    [Parameter()]
    [string]$ServiceName = "ProxyPal"
)

# ============================================================================
# Configuration
# ============================================================================

$ErrorActionPreference = "Stop"

$InstallDir = "C:\Program Files\ProxyPal"
$ConfigDir = "$env:APPDATA\ProxyPal"
$NssmDir = "C:\Tools\nssm"
$NssmExe = "$NssmDir\nssm.exe"

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

# ============================================================================
# Main Uninstallation
# ============================================================================

function Main {
    Write-Host ""
    Write-Host "╔══════════════════════════════════════════════════════════╗" -ForegroundColor Yellow
    Write-Host "║          ProxyPal Windows Service Uninstaller            ║" -ForegroundColor Yellow
    Write-Host "╚══════════════════════════════════════════════════════════╝" -ForegroundColor Yellow
    Write-Host ""
    
    # Check admin
    if (-not (Test-IsAdmin)) {
        Write-Status "This script requires Administrator privileges" "ERROR"
        Write-Status "Please run PowerShell as Administrator and try again" "ERROR"
        exit 1
    }
    
    try {
        # Step 1: Stop service
        $service = Get-Service -Name $ServiceName -ErrorAction SilentlyContinue
        
        if ($service) {
            Write-Status "Stopping service..." "INFO"
            
            if ($service.Status -eq "Running") {
                Stop-Service -Name $ServiceName -Force
                Start-Sleep -Seconds 3
            }
            
            Write-Status "Service stopped" "SUCCESS"
        } else {
            Write-Status "Service not found, may already be uninstalled" "WARNING"
        }
        
        # Step 2: Remove service using nssm
        if (Test-Path $NssmExe) {
            Write-Status "Removing service with NSSM..." "INFO"
            & $NssmExe remove $ServiceName confirm 2>$null
            Start-Sleep -Seconds 1
            Write-Status "Service removed from Windows" "SUCCESS"
        } else {
            # Try sc.exe as fallback
            Write-Status "NSSM not found, using sc.exe..." "INFO"
            sc.exe delete $ServiceName 2>$null
        }
        
        # Step 3: Remove program files
        if ($RemoveAll -or -not $KeepConfig) {
            Write-Status "Removing program files..." "INFO"
            
            if (Test-Path $InstallDir) {
                Remove-Item $InstallDir -Recurse -Force
                Write-Status "Removed: $InstallDir" "SUCCESS"
            }
        } else {
            Write-Status "Keeping program files at: $InstallDir" "INFO"
            Write-Status "To remove: Remove-Item '$InstallDir' -Recurse -Force" "INFO"
        }
        
        # Step 4: Handle config
        if ($RemoveAll) {
            Write-Status "Removing config directory..." "INFO"
            
            if (Test-Path $ConfigDir) {
                Remove-Item $ConfigDir -Recurse -Force
                Write-Status "Removed: $ConfigDir" "SUCCESS"
            }
        } else {
            if (Test-Path $ConfigDir) {
                Write-Status "Config preserved at: $ConfigDir" "INFO"
                Write-Status "To remove: Remove-Item '$ConfigDir' -Recurse -Force" "INFO"
            }
        }
        
        # Summary
        Write-Host ""
        Write-Host "╔══════════════════════════════════════════════════════════╗" -ForegroundColor Green
        Write-Host "║              Uninstallation Complete!                    ║" -ForegroundColor Green
        Write-Host "╚══════════════════════════════════════════════════════════╝" -ForegroundColor Green
        Write-Host ""
        
        if (-not $RemoveAll) {
            Write-Status "To completely remove all files, run:" "INFO"
            Write-Host "  .\uninstall-windows.ps1 -RemoveAll" -ForegroundColor Cyan
            Write-Host ""
        }
        
    } catch {
        Write-Status "Uninstallation failed: $_" "ERROR"
        exit 1
    }
}

# Run main
Main
