# System Requirements (Windows Native)

To build and run **HELIOS L1** locally on Windows, you must install the following:

## 1. Visual Studio Build Tools (C++) 🛠️

**Critical Missing Component:** Your system is missing the C++ compiler (`cl.exe`).

### How to Install:

1.  Download [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/).
2.  Run the installer.
3.  Select the **"Desktop development with C++"** workload.
4.  Click **Install**.

## 2. Dependencies (Once VS Tools are installed)

Run the following in PowerShell:

```powershell
# 1. Install Rust
winget install --id Rustlang.Rustup

# 2. Install Solana
& 'C:\Windows\System32\curl.exe' -L -o solana-release.tar.bz2 https://github.com/solana-labs/solana/releases/download/v1.18.18/solana-release-x86_64-pc-windows-msvc.tar.bz2
tar jxf solana-release.tar.bz2

# 3. Install Anchor
cargo install anchor-cli --version 0.30.1
```
