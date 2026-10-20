# INFO Desktop — Build et plateformes

Jalon source accepté actuel : 0.1.1 dans main.

Cibles prises en charge : Windows 10/11 x64, macOS arm64, macOS x86_64, Linux x64. Windows 7 n’est pas requis.

```bash
npm ci
npm run check
npm run test:protocol
npm run build
npm run tauri build
```

CI artifact does not by itself prove platform support; a real launch/smoke test is required.
