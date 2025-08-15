@echo off
echo 🚀 Startup Value Simulator - Build Script
echo ==========================================

echo.
echo Checking if Rust is installed...
rustc --version >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ Rust is not installed. Please install Rust from https://rustup.rs/
    pause
    exit /b 1
)

echo ✅ Rust is installed

echo.
echo Checking if wasm-pack is installed...
wasm-pack --version >nul 2>&1
if %errorlevel% neq 0 (
    echo Installing wasm-pack...
    cargo install wasm-pack
)

echo ✅ wasm-pack is available

echo.
echo Checking if trunk is installed...
trunk --version >nul 2>&1
if %errorlevel% neq 0 (
    echo Installing trunk...
    cargo install trunk
)

echo ✅ trunk is available

echo.
echo Building the project...
cargo build

if %errorlevel% neq 0 (
    echo ❌ Build failed
    pause
    exit /b 1
)

echo ✅ Build successful!

echo.
echo 🎉 Setup complete! You can now run:
echo    trunk serve
echo.
echo This will start the development server at http://localhost:8080
echo.
pause



