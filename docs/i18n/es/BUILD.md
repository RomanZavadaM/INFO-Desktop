# INFO Desktop — Compilación y plataformas

Checkpoint de código aceptado actual: 0.1.2 en main.

Objetivos compatibles: Windows 10/11 x64, macOS arm64, macOS x86_64 y Linux x64. Windows 7 no es obligatorio.

```bash
npm install
npm run check
npm run test:protocol
npm run build
npm run tauri build
```

CI artifact does not by itself prove platform support; a real launch/smoke test is required.

## Modo consola

La consola se compila sin Tauri ni las bibliotecas del sistema de la ventana:

```bash
cd src-tauri
cargo build --release -p info-desktop-cli
cargo test --workspace --lib
```

En GitHub Actions la consola se compila junto con la ventana para Windows, Linux y macOS (ARM64 e Intel); el flujo de publicación `release-0.1.5.yml` la empaqueta como archivos separados con sumas de verificación. Una publicación solo con la palabra expresa del propietario.
