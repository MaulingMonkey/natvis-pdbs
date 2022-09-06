:: Main entry point
@if defined CI set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"
@echo on
@setlocal && pushd "%~dp0.."

cargo +1.39.0   build --all
@if ERRORLEVEL 1 goto :die

cargo +stable   build --all
@if ERRORLEVEL 1 goto :die

cargo +beta     build --all
@if ERRORLEVEL 1 goto :die

cargo +nightly  build --all
@if ERRORLEVEL 1 goto :die

@cd "%~dp0..\crates\example-usage-metabuild"

cargo +nightly  build --all
@if ERRORLEVEL 1 goto :die

:die
@if ERRORLEVEL 1 echo BUILD ERRORS BUILD ERRORS BUILD ERRORS BUILD ERRORS
@endlocal && popd && exit /b %ERRORLEVEL%
