@setlocal
@rustup --version >NUL 2>NUL && goto :skip_rustup
curl -sSf -o "%TEMP%\rustup-init.exe" https://win.rustup.rs
"%TEMP%\rustup-init.exe" --default-toolchain stable -y
@set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"
:skip_rustup
rustup toolchain install 1.39.0     || exit /b 1
rustup toolchain install stable     || exit /b 1
rustup toolchain install beta       || exit /b 1
rustup toolchain install nightly    || exit /b 1
@exit /b 0
