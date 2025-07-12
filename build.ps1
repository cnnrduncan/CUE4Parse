#!/usr/bin/env pwsh

# CUE4Parse Rust Integration Build Script
# This script helps you build the necessary components for Rust integration

param(
    [switch]$Native,
    [switch]$CLI,
    [switch]$All,
    [switch]$Clean,
    [string]$Configuration = "Release"
)

Write-Host "CUE4Parse Rust Integration Build Script" -ForegroundColor Green
Write-Host "=======================================" -ForegroundColor Green

$rootDir = $PSScriptRoot

if ($Clean) {
    Write-Host "Cleaning previous builds..." -ForegroundColor Yellow
    
    # Clean .NET projects
    if (Test-Path "$rootDir\bin") { Remove-Item "$rootDir\bin" -Recurse -Force }
    if (Test-Path "$rootDir\obj") { Remove-Item "$rootDir\obj" -Recurse -Force }
    
    # Clean native builds
    if (Test-Path "$rootDir\CUE4Parse-Natives\build") { 
        Remove-Item "$rootDir\CUE4Parse-Natives\build" -Recurse -Force 
    }
    
    # Clean Rust builds
    if (Test-Path "$rootDir\cue4parse-rs\target") { 
        Remove-Item "$rootDir\cue4parse-rs\target" -Recurse -Force 
    }
    
    Write-Host "Cleanup completed." -ForegroundColor Green
}

function Build-DotNetProjects {
    Write-Host "Building .NET projects..." -ForegroundColor Yellow
    
    # Build the entire solution
    dotnet build CUE4Parse.sln --configuration $Configuration
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host ".NET projects built successfully." -ForegroundColor Green
        
        # Display CLI executable location
        $cliPath = "$rootDir\CUE4Parse.CLI\bin\$Configuration\net8.0\CUE4Parse.CLI.exe"
        if (Test-Path $cliPath) {
            Write-Host "CLI tool available at: $cliPath" -ForegroundColor Cyan
        }
    } else {
        Write-Host ".NET build failed!" -ForegroundColor Red
        exit 1
    }
}

function Build-NativeLibrary {
    Write-Host "Building native library..." -ForegroundColor Yellow
    
    $nativeDir = "$rootDir\CUE4Parse-Natives"
    $buildDir = "$nativeDir\build"
    
    # Create build directory
    if (-not (Test-Path $buildDir)) {
        New-Item -ItemType Directory -Path $buildDir | Out-Null
    }
    
    Push-Location $buildDir
    
    try {
        # Configure with CMake
        cmake .. -A x64
        
        if ($LASTEXITCODE -eq 0) {
            # Build the library
            cmake --build . --config $Configuration
            
            if ($LASTEXITCODE -eq 0) {
                Write-Host "Native library built successfully." -ForegroundColor Green
                
                # Display library location
                $libPath = "$nativeDir\bin\$Configuration\CUE4Parse-Natives.dll"
                if (Test-Path $libPath) {
                    Write-Host "Native library available at: $libPath" -ForegroundColor Cyan
                }
            } else {
                Write-Host "Native library build failed!" -ForegroundColor Red
            }
        } else {
            Write-Host "CMake configuration failed!" -ForegroundColor Red
        }
    } finally {
        Pop-Location
    }
}

function Build-RustCrate {
    Write-Host "Building Rust crate..." -ForegroundColor Yellow
    
    Push-Location "$rootDir\cue4parse-rs"
    
    try {
        # Build the Rust crate
        cargo build --release
        
        if ($LASTEXITCODE -eq 0) {
            Write-Host "Rust crate built successfully." -ForegroundColor Green
            
            # Run tests
            cargo test
            
            if ($LASTEXITCODE -eq 0) {
                Write-Host "Rust tests passed." -ForegroundColor Green
            } else {
                Write-Host "Some Rust tests failed." -ForegroundColor Yellow
            }
        } else {
            Write-Host "Rust build failed!" -ForegroundColor Red
        }
    } finally {
        Pop-Location
    }
}

function Show-Usage {
    Write-Host ""
    Write-Host "Usage Examples:" -ForegroundColor Cyan
    Write-Host "  .\build.ps1 -All              # Build everything"
    Write-Host "  .\build.ps1 -CLI              # Build only CLI tool"
    Write-Host "  .\build.ps1 -Native           # Build only native library"
    Write-Host "  .\build.ps1 -Clean -All       # Clean and build everything"
    Write-Host ""
    Write-Host "Getting Started:" -ForegroundColor Cyan
    Write-Host "  1. Run: .\build.ps1 -All"
    Write-Host "  2. Navigate to cue4parse-rs directory"
    Write-Host "  3. Run: cargo run --example basic_usage"
    Write-Host ""
}

# Main execution logic
if ($All) {
    Build-DotNetProjects
    if ($Native) { Build-NativeLibrary }
    Build-RustCrate
}
elseif ($CLI) {
    Build-DotNetProjects
}
elseif ($Native) {
    Build-NativeLibrary
}
else {
    Write-Host "Please specify what to build:" -ForegroundColor Yellow
    Write-Host "  -All     : Build everything (recommended)"
    Write-Host "  -CLI     : Build CLI tool only"
    Write-Host "  -Native  : Build native library only"
    Write-Host "  -Clean   : Clean before building"
    Write-Host ""
    Show-Usage
}

Write-Host ""
Write-Host "Build script completed." -ForegroundColor Green
