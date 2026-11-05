# INFO Desktop — Build et plateformes

Jalon source accepté actuel : 0.1.2 dans main.

Cibles prises en charge : Windows 10/11 x64, macOS arm64, macOS x86_64, Linux x64. Windows 7 n’est pas requis.

```bash
npm install
npm run check
npm run test:protocol
npm run build
npm run tauri build
```

CI artifact does not by itself prove platform support; a real launch/smoke test is required.

## Mode console

La console se compile sans Tauri ni les bibliothèques système de la fenêtre :

```bash
cd src-tauri
cargo build --release -p info-desktop-cli
cargo test --workspace --lib
```

Dans GitHub Actions, la console est compilée avec la fenêtre pour Windows, Linux et macOS (ARM64 et Intel) ; le workflow de publication `release-0.1.5.yml` l'empaquette en fichiers séparés avec sommes de contrôle. Une publication uniquement sur parole explicite du propriétaire.
