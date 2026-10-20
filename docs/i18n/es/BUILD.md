# INFO Desktop — Compilación y plataformas

Checkpoint de código aceptado actual: 0.1.1 en main.

Objetivos compatibles: Windows 10/11 x64, macOS arm64, macOS x86_64 y Linux x64. Windows 7 no es obligatorio.

```bash
npm install
npm run check
npm run test:protocol
npm run build
npm run tauri build
```

CI artifact does not by itself prove platform support; a real launch/smoke test is required.
