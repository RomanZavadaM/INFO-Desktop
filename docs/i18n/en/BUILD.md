# INFO Desktop — Build and platforms

Current accepted source checkpoint: 0.1.1 in main.

Supported targets: Windows 10/11 x64, macOS arm64, macOS x86_64, Linux x64. Windows 7 is not required.

```bash
npm install
npm run check
npm run test:protocol
npm run build
npm run tauri build
```

CI artifact does not by itself prove platform support; a real launch/smoke test is required.
