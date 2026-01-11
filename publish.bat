@echo off
setlocal EnableExtensions

set "check_mode=0"
set "token="
set "publish_opts="
set "start_step=1"
set "delay_seconds=0"
set "retry_max=0"
set "retry_wait_seconds=60"
set "root=%~dp0"

REM Parse arguments (token is optional if you've already run: cargo login)
:parse_args
if "%~1"=="" goto args_done
if /i "%~1"=="-h" goto usage
if /i "%~1"=="--help" goto usage
if "%~1"=="/?" goto usage
if /i "%~1"=="-c" set "check_mode=1" & shift & goto parse_args
if /i "%~1"=="--check" set "check_mode=1" & shift & goto parse_args
if /i "%~1"=="-s" set "start_step=%~2" & shift & shift & goto parse_args
if /i "%~1"=="--step" set "start_step=%~2" & shift & shift & goto parse_args
if /i "%~1"=="-d" set "delay_seconds=%~2" & shift & shift & goto parse_args
if /i "%~1"=="--delay" set "delay_seconds=%~2" & shift & shift & goto parse_args
if /i "%~1"=="--retry" set "retry_max=%~2" & shift & shift & goto parse_args
if /i "%~1"=="-w" set "retry_wait_seconds=%~2" & shift & shift & goto parse_args
if /i "%~1"=="--wait" set "retry_wait_seconds=%~2" & shift & shift & goto parse_args

set "arg=%~1"
if "%arg:~0,1%"=="-" goto usage
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
echo start_step: %start_step%
echo delay: %delay_seconds%s
echo retry: %retry_max% (wait %retry_wait_seconds%s)
echo.

REM Build publish options
if "%check_mode%"=="1" set "publish_opts=--dry-run"
if "%check_mode%"=="0" if defined token set "publish_opts=--token %token%"

REM Publish crates in dependency order (do NOT publish the root crate)
if %start_step% LEQ 1 call :publish "crates\core" "emixcore" 1 11
if errorlevel 1 goto fail
if %start_step% LEQ 2 call :publish "crates\base" "emix" 2 11
if errorlevel 1 goto fail
if %start_step% LEQ 3 call :publish "crates\db\common" "emixdb" 3 11
if errorlevel 1 goto fail
if %start_step% LEQ 4 call :publish "crates\collections" "emixcollections" 4 11
if errorlevel 1 goto fail
if %start_step% LEQ 5 call :publish "crates\crypto" "emixcrypto" 5 11
if errorlevel 1 goto fail
if %start_step% LEQ 6 call :publish "crates\threading" "emixthreading" 6 11
if errorlevel 1 goto fail
if %start_step% LEQ 7 call :publish "crates\log" "emixlog" 7 11
if errorlevel 1 goto fail
if %start_step% LEQ 8 call :publish "crates\net" "emixnet" 8 11
if errorlevel 1 goto fail
if %start_step% LEQ 9 call :publish "crates\ai" "emixai" 9 11
if errorlevel 1 goto fail
if %start_step% LEQ 10 call :publish "crates\db\diesel" "emixdiesel" 10 11
if errorlevel 1 goto fail
if %start_step% LEQ 11 call :publish "crates\db\seaorm" "emixseaorm" 11 11
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
set "attempt=0"
:publish_try
set /a attempt=attempt+1
set "log_file=%TEMP%\emix_publish_%~2.log"
cargo publish %publish_opts% > "%log_file%" 2>&1
type "%log_file%"
if errorlevel 1 goto publish_failed
if %delay_seconds% GTR 0 call :sleep %delay_seconds%
echo.
exit /b 0

:publish_failed
REM Treat "already published" as success to allow resume without -step
findstr /i /c:"already uploaded" /c:"already exists" /c:"already been uploaded" "%log_file%" >nul
if errorlevel 1 goto publish_failed_429_check
echo.
echo %~2 already published on crates.io. Skipping.
echo.
exit /b 0

:publish_failed_429_check
findstr /i /c:"status 429" "%log_file%" >nul
if errorlevel 1 goto publish_failed_final
echo.
echo Hit crates.io rate limit (429 Too Many Requests).
if %retry_max% LEQ 0 goto publish_failed_final
if %attempt% GEQ %retry_max% goto publish_failed_final
echo Waiting %retry_wait_seconds%s then retrying (%attempt%/%retry_max%)...
call :sleep %retry_wait_seconds%
goto publish_try

:publish_failed_final
echo Failed to publish %~2
exit /b 1

:sleep
REM Args: 1=seconds
if "%~1"=="" goto :eof
timeout /t %~1 /nobreak >nul
goto :eof

:usage
echo.
echo Usage:
echo   publish.bat [token] [options]
echo.
echo Token:
echo   token                     Optional. If omitted, uses credentials from `cargo login`.
echo.
echo Options:
echo   -c, --check                        Dry-run: `cargo publish --dry-run` (no upload).
echo   -s N, --step N                     Resume from step N (1..11). Default: 1
echo   -d SECONDS, --delay SECONDS        Wait between crates. Default: 0
echo   --retry COUNT                      Retry on crates.io 429. Default: 0
echo   -w SECONDS, --wait SECONDS         Wait between retries. Default: 60
echo   -h, --help, /?                     Show this help
echo.
echo Notes:
echo   - On reruns, if a crate/version is already published, the script will skip it.
echo   - `--check` can still fail for dependent crates until deps are actually published.
echo.
echo Examples:
echo   publish.bat
echo   publish.bat -c
echo   publish.bat --step 6 --delay 30
echo   publish.bat YOUR_TOKEN --delay 30 --retry 5 --wait 120
echo.
cd /d "%root%"
endlocal
exit /b 1
