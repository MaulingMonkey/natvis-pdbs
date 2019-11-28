:: Main entry point
@echo on
@pushd "%~dp0.."
cargo +stable   build --all || @exit /b 1
cargo +beta     build --all || @exit /b 1
cargo +nightly  build --all || @exit /b 1
@pushd "%~dp0..\crates\example-usage-metabuild"
cargo +nightly  build --all || @exit /b 1
@popd
@popd
