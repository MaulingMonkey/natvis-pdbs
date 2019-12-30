:: Main entry point
@if defined CI set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"
@echo on
@pushd "%~dp0.."
cargo +1.39.0   build --all || @exit /b 1
cargo +stable   build --all || @exit /b 1
cargo +beta     build --all || @exit /b 1
cargo +nightly  build --all || @exit /b 1
@pushd "%~dp0..\crates\example-usage-metabuild"
cargo +nightly  build --all || @exit /b 1
@popd
@popd
