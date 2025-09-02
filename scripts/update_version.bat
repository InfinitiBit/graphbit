@echo off
REM GraphBit Version Management Script (Windows Batch wrapper)
REM 
REM This script provides a Windows batch interface to the Python version management script
REM with additional validation and convenience features.

setlocal enabledelayedexpansion

REM Colors for output (Windows 10+ with ANSI support)
set "RED=[91m"
set "GREEN=[92m"
set "YELLOW=[93m"
set "BLUE=[94m"
set "NC=[0m"

REM Script directory and repository root
set "SCRIPT_DIR=%~dp0"
set "REPO_ROOT=%SCRIPT_DIR%.."
set "PYTHON_SCRIPT=%SCRIPT_DIR%update_version.py"

REM Function to print colored output
:print_info
echo %BLUE%[INFO]%NC% %~1
goto :eof

:print_success
echo %GREEN%[SUCCESS]%NC% %~1
goto :eof

:print_warning
echo %YELLOW%[WARNING]%NC% %~1
goto :eof

:print_error
echo %RED%[ERROR]%NC% %~1
goto :eof

REM Function to check prerequisites
:check_prerequisites
call :print_info "Checking prerequisites..."

REM Check if Python is available
python --version >nul 2>&1
if errorlevel 1 (
    call :print_error "Python is not installed or not in PATH"
    exit /b 1
)

REM Get Python version
for /f "tokens=2" %%i in ('python --version 2^>^&1') do set "PYTHON_VERSION=%%i"
call :print_info "Using Python !PYTHON_VERSION!"

REM Check if we're in the right directory
if not exist "%REPO_ROOT%\Cargo.toml" (
    call :print_error "Not in GraphBit repository root (Cargo.toml not found)"
    exit /b 1
)

REM Check if the Python script exists
if not exist "%PYTHON_SCRIPT%" (
    call :print_error "Python version script not found: %PYTHON_SCRIPT%"
    exit /b 1
)

call :print_success "Prerequisites check passed"
goto :eof

REM Function to validate version format
:validate_version
set "version=%~1"
echo %version% | findstr /r "^[0-9][0-9]*\.[0-9][0-9]*\.[0-9][0-9]*" >nul
if errorlevel 1 (
    call :print_error "Invalid version format: %version%"
    call :print_error "Expected semantic versioning format: MAJOR.MINOR.PATCH[-PRERELEASE][+BUILD]"
    call :print_error "Examples: 1.0.0, 1.2.3-beta.1, 2.0.0-rc.1+build.123"
    exit /b 1
)
goto :eof

REM Function to show help
:show_help
echo GraphBit Version Management Script (Windows)
echo.
echo USAGE:
echo     %~nx0 ^<new_version^> [OPTIONS]
echo.
echo ARGUMENTS:
echo     ^<new_version^>    New version number (semantic versioning format)
echo.
echo OPTIONS:
echo     --dry-run        Show what would be changed without making actual changes
echo     --backup         Create backup before making changes
echo     --report         Generate a detailed report file
echo     --force          Skip repository state validation
echo     --help, -h       Show this help message
echo.
echo EXAMPLES:
echo     %~nx0 0.2.0                    # Update to version 0.2.0
echo     %~nx0 1.0.0-beta.1 --dry-run   # Preview changes for beta version
echo     %~nx0 2.1.3 --backup --report  # Update with backup and report
echo.
echo SEMANTIC VERSIONING:
echo     MAJOR.MINOR.PATCH[-PRERELEASE][+BUILD]
echo.    
echo     Examples:
echo     - 1.0.0          (stable release)
echo     - 1.2.3-beta.1   (pre-release)
echo     - 2.0.0-rc.1     (release candidate)
echo     - 1.0.0+build.1  (with build metadata)
echo.
goto :eof

REM Main function
:main
set "new_version="
set "python_args="

REM Parse arguments
:parse_args
if "%~1"=="" goto :args_done
if "%~1"=="--help" goto :show_help_and_exit
if "%~1"=="-h" goto :show_help_and_exit
if "%~1"=="--dry-run" (
    set "python_args=%python_args% --dry-run"
    goto :next_arg
)
if "%~1"=="--backup" (
    set "python_args=%python_args% --backup"
    goto :next_arg
)
if "%~1"=="--report" (
    set "python_args=%python_args% --report"
    goto :next_arg
)
if "%~1"=="--force" (
    set "python_args=%python_args% --force"
    goto :next_arg
)
REM If not a flag, treat as version
if "!new_version!"=="" (
    set "new_version=%~1"
) else (
    call :print_error "Multiple version arguments provided"
    exit /b 1
)

:next_arg
shift
goto :parse_args

:show_help_and_exit
call :show_help
exit /b 0

:args_done
REM Check if version was provided
if "!new_version!"=="" (
    call :print_error "Version number is required"
    call :show_help
    exit /b 1
)

REM Run checks
call :check_prerequisites
if errorlevel 1 exit /b 1

call :validate_version "!new_version!"
if errorlevel 1 exit /b 1

REM Execute Python script
call :print_info "Executing version update..."
cd /d "%REPO_ROOT%"

python "%PYTHON_SCRIPT%" "!new_version!" !python_args!
if errorlevel 1 (
    call :print_error "Version update failed"
    exit /b 1
) else (
    call :print_success "Version update completed successfully"
)

goto :eof

REM Call main function with all arguments
call :main %*
