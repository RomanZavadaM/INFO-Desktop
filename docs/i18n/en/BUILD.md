# INFO Desktop — Build and platforms

Current accepted source checkpoint: 0.1.2 in main.

Supported targets: Windows 10/11 x64, macOS arm64, macOS x86_64, Linux x64. Windows 7 is not required.

```bash
npm install
npm run check
npm run test:protocol
npm run build
npm run tauri build
```

CI artifact does not by itself prove platform support; a real launch/smoke test is required.

## Console mode

The console builds without Tauri and the window's system libraries:

```bash
cd src-tauri
cargo build --release -p info-desktop-cli
cargo test --workspace --lib
```

In GitHub Actions the console is built together with the window for Windows, Linux and macOS (ARM64 and Intel); the release workflow `release-0.1.5.yml` packages it as separate files with checksums. A release only with the owner's explicit word.
