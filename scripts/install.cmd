@setlocal
curl -sSf -o "%TEMP%\rustup-init.exe" https://win.rustup.rs
"%TEMP%\rustup-init.exe" --default-toolchain stable -y
@set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"
rustup toolchain install stable     || exit /b 1
rustup toolchain install beta       || exit /b 1
rustup toolchain install nightly    || exit /b 1
@exit /b 0
