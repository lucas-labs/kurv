@echo off

REM example kurv plugin for testing purposes

REM Check if --kurv-cfg flag is present
if "%1"=="--kurv-cfg" (
    setlocal enabledelayedexpansion
    set "fullpath=%~f0"
    set "fullpath=!fullpath:\=\\!"
    echo {"name":"hello-world","command":"!fullpath!","args":["run"],"env":{"HELLO_MESSAGE":"Hello from kurv plugin!"}}
    endlocal
    exit /b 0
)

REM If run argument is provided, execute the plugin logic
if "%1"=="run" (
    REM print env variable HELLO_MESSAGE, KURV_API_HOST, KURV_API_PORT, KURV_HOME, KURV_LOGS_DIR
    echo HELLO_MESSAGE: %HELLO_MESSAGE%
    echo KURV_API_HOST: %KURV_API_HOST%
    echo KURV_API_PORT: %KURV_API_PORT%
    echo KURV_HOME: %KURV_HOME%
    echo KURV_LOGS_DIR: %KURV_LOGS_DIR%

    :loop
    REM echo [%date% %time%] %HELLO_MESSAGE%
    timeout /t 5 /nobreak >nul
    goto loop
)

echo Usage: kurv-hello-world.bat [--kurv-cfg^|run]
exit /b 1
