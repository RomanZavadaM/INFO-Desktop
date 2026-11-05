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

## Konsolenmodus

Die Konsole wird ohne Tauri und die Systembibliotheken des Fensters gebaut:

```bash
cd src-tauri
cargo build --release -p info-desktop-cli
cargo test --workspace --lib
```

In GitHub Actions wird die Konsole zusammen mit dem Fenster für Windows, Linux und macOS (ARM64 und Intel) gebaut; der Release-Workflow `release-0.1.5.yml` packt sie als eigene Dateien mit Prüfsummen. Ein Release nur auf ausdrückliches Wort des Eigentümers.
