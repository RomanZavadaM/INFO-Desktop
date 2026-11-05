# INFO Desktop — збірка і платформи

## Версія
Поточний accepted source checkpoint: **0.1.2** у гілці `main`.

## Підтримувані цілі
- Windows 10/11 x64;
- macOS Apple Silicon / arm64;
- macOS Intel / x86_64;
- Linux x64.

Windows 7 не входить до обов'язкової матриці нового продукту.

## Перевірка
```bash
npm install
npm run check
npm run test:protocol
npm run build
npm run tauri build
```

CI-збірка сама по собі не означає підтримку платформи: стабільною ціль вважається після реального запуску та smoke-test на відповідній ОС.

GitHub Actions формують окремі артефакти Windows, Linux і macOS. Імена збірок синхронізовані з версією 0.1.2.

## Консольний режим

Консоль збирається без Tauri і системних бібліотек вікна:

```bash
cd src-tauri
cargo build --release -p info-desktop-cli
cargo test --workspace --lib
```

У GitHub Actions консоль збирається разом із вікном для Windows, Linux і macOS (ARM64 та Intel); випускний процес `release-0.1.5.yml` пакує її окремими файлами з контрольними сумами. Випуск лише за окремим словом власника.
