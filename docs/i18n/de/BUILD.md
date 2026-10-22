# INFO Desktop — Build und Plattformen

Aktueller akzeptierter Quellcode-Checkpoint: 0.1.2 in main.

Unterstützte Ziele: Windows 10/11 x64, macOS arm64, macOS x86_64, Linux x64. Windows 7 ist nicht erforderlich.

```bash
npm install
npm run check
npm run test:protocol
npm run build
npm run tauri build
```

CI artifact does not by itself prove platform support; a real launch/smoke test is required.
