@echo off
REM Helper batch file to test CUE4Parse compatibility with Oblivion Remastered

REM Configuration
set "GAME_DIR=C:\Games\Steam\steamapps\common\Oblivion Remastered\OblivionRemastered"
set "CONTENT_DIR=%GAME_DIR%\Content\Paks"

echo 🎮 CUE4Parse Oblivion Remastered Test
echo ====================================

REM Check if game directory exists
if not exist "%GAME_DIR%" (
    echo ❌ Game directory not found: %GAME_DIR%
    echo Please update the GAME_DIR variable in this script
    pause
    exit /b 1
)

echo 📂 Game Directory: %GAME_DIR%
echo 📦 Content Directory: %CONTENT_DIR%

REM List available pak files
echo.
echo 📋 Available Package Files:
echo --------------------------
if exist "%CONTENT_DIR%" (
    dir "%CONTENT_DIR%\*.pak" 2>nul || echo No .pak files found
    dir "%CONTENT_DIR%\*.utoc" 2>nul || echo No .utoc files found  
    dir "%CONTENT_DIR%\*.ucas" 2>nul || echo No .ucas files found
) else (
    echo ❌ Content directory not found: %CONTENT_DIR%
)

REM Look for .usmap files
echo.
echo 🗺️ Looking for .usmap mapping files:
echo ------------------------------------

set "FOUND_USMAP="

REM Check common .usmap locations
if exist "%GAME_DIR%\Mappings.usmap" (
    echo ✅ Found .usmap file: %GAME_DIR%\Mappings.usmap
    set "FOUND_USMAP=%GAME_DIR%\Mappings.usmap"
    goto :found_usmap
)

if exist "%GAME_DIR%\Content\Mappings.usmap" (
    echo ✅ Found .usmap file: %GAME_DIR%\Content\Mappings.usmap
    set "FOUND_USMAP=%GAME_DIR%\Content\Mappings.usmap"
    goto :found_usmap
)

if exist "%GAME_DIR%\Binaries\Mappings.usmap" (
    echo ✅ Found .usmap file: %GAME_DIR%\Binaries\Mappings.usmap
    set "FOUND_USMAP=%GAME_DIR%\Binaries\Mappings.usmap"
    goto :found_usmap
)

if exist "%CONTENT_DIR%\Mappings.usmap" (
    echo ✅ Found .usmap file: %CONTENT_DIR%\Mappings.usmap
    set "FOUND_USMAP=%CONTENT_DIR%\Mappings.usmap"
    goto :found_usmap
)

echo ⚠️ No .usmap files found - will test without mappings

:found_usmap

echo.
echo 🚀 Running CUE4Parse compatibility test...
echo =========================================

REM Build the command
set "CMD=cargo run --example real_world_test --features unrealmodding-compat -- --game-dir \"%GAME_DIR%\""

if defined FOUND_USMAP (
    set "CMD=%CMD% --usmap-path \"%FOUND_USMAP%\""
)

echo Command: %CMD%
echo.

REM Run the test
%CMD%

echo.
echo ✅ Test completed!
pause
