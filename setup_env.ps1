$env:PATH = "$env:USERPROFILE\.cargo\bin;$PWD\solana-release\bin;$env:PATH"
Write-Host "✅ Environment Loaded!" -ForegroundColor Green
Write-Host "Rust: $(rustc --version)"
Write-Host "Solana: $(solana --version)"
Write-Host "Cargo: $(cargo --version)"
