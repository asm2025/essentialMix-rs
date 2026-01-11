@echo off
setlocal EnableExtensions

set "check_mode=0"
set "token="
set "publish_opts="
set "root=%~dp0"

REM Parse arguments (token is optional if you've already run: cargo login)
:parse_args
if "%~1"=="" goto args_done
if /i "%~1"=="-c" set "check_mode=1" & shift & goto parse_args
if /i "%~1"=="--check" set "check_mode=1" & shift & goto parse_args
if not defined token set "token=%~1" & shift & goto parse_args
shift
goto parse_args

:args_done
if "%check_mode%"=="1" goto banner_check
echo Publishing EssentialMix crates to crates.io...
goto banner_done
:banner_check
echo Checking EssentialMix crates [dry-run]...
echo NOTE: This uses "cargo publish --dry-run", which simulates a real publish.
echo       That means path deps are treated as crates.io deps, so dependent crates
echo       will FAIL until their dependencies have actually been published.
:banner_done
echo.

REM Build publish options
if "%check_mode%"=="1" set "publish_opts=--dry-run"
if "%check_mode%"=="0" if defined token set "publish_opts=--token %token%"

REM Publish crates in dependency order (do NOT publish the root crate)
call :publish "crates\core" "emixcore" 1 11
if errorlevel 1 goto fail
call :publish "crates\base" "emix" 2 11
if errorlevel 1 goto fail
call :publish "crates\db\common" "emixdb" 3 11
if errorlevel 1 goto fail
call :publish "crates\collections" "emixcollections" 4 11
if errorlevel 1 goto fail
call :publish "crates\crypto" "emixcrypto" 5 11
if errorlevel 1 goto fail
call :publish "crates\threading" "emixthreading" 6 11
if errorlevel 1 goto fail
call :publish "crates\log" "emixlog" 7 11
if errorlevel 1 goto fail
call :publish "crates\net" "emixnet" 8 11
if errorlevel 1 goto fail
call :publish "crates\ai" "emixai" 9 11
if errorlevel 1 goto fail
call :publish "crates\db\diesel" "emixdiesel" 10 11
if errorlevel 1 goto fail
call :publish "crates\db\seaorm" "emixseaorm" 11 11
if errorlevel 1 goto fail

if "%check_mode%"=="1" goto success_check
echo All crates published successfully!
goto done
:success_check
echo All crates checked successfully [dry-run]!
goto done

:fail
echo.
echo Publish/check failed.
goto done

:done
cd /d "%root%"
endlocal
goto :eof

:publish
REM Args: 1=relative path, 2=crate name, 3=step, 4=total
echo [%~3/%~4] Publishing %~2...
cd /d "%root%%~1"
if errorlevel 1 (
	echo Failed to cd into "%root%%~1"
	exit /b 1
)
cargo publish %publish_opts%
if errorlevel 1 (
	echo Failed to publish %~2
	exit /b 1
)
echo.
exit /b 0
