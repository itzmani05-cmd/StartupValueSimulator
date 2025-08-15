Write-Host "🚀 Startup Value Simulator - Build Script" -ForegroundColor Cyan
Write-Host "=========================================" -ForegroundColor Cyan

Write-Host "`nChecking if Rust is installed..." -ForegroundColor Yellow
try {
    $rustVersion = rustc --version
    Write-Host "✅ Rust is installed: $rustVersion" -ForegroundColor Green
} catch {
    Write-Host "❌ Rust is not installed. Please install Rust from https://rustup.rs/" -ForegroundColor Red
    Read-Host "Press Enter to continue"
    exit 1
}

Write-Host "`nChecking if wasm-pack is installed..." -ForegroundColor Yellow
try {
    $wasmPackVersion = wasm-pack --version
    Write-Host "✅ wasm-pack is available: $wasmPackVersion" -ForegroundColor Green
} catch {
    Write-Host "Installing wasm-pack..." -ForegroundColor Yellow
    cargo install wasm-pack
    Write-Host "✅ wasm-pack installed" -ForegroundColor Green
}

Write-Host "`nChecking if trunk is installed..." -ForegroundColor Yellow
try {
    $trunkVersion = trunk --version
    Write-Host "✅ trunk is available: $trunkVersion" -ForegroundColor Green
} catch {
    Write-Host "Installing trunk..." -ForegroundColor Yellow
    cargo install trunk
    Write-Host "✅ trunk installed" -ForegroundColor Green
}

Write-Host "`nBuilding the project..." -ForegroundColor Yellow
cargo build

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Build failed" -ForegroundColor Red
    Read-Host "Press Enter to continue"
    exit 1
}

Write-Host "✅ Build successful!" -ForegroundColor Green

Write-Host "`n🎉 Setup complete! You can now run:" -ForegroundColor Cyan
Write-Host "   trunk serve" -ForegroundColor White
Write-Host "`nThis will start the development server at http://localhost:8080" -ForegroundColor White
Write-Host "`nPress Enter to continue..." -ForegroundColor Yellow
Read-Host



