# INFO Desktop — збірка і платформи

## Версія
Поточний accepted source checkpoint: **0.1.1** у гілці `main`.

## Підтримувані цілі
- Windows 10/11 x64;
- macOS Apple Silicon / arm64;
- macOS Intel / x86_64;
- Linux x64.

Windows 7 не входить до обов'язкової матриці нового продукту.

## Перевірка
```bash
npm ci
npm run check
npm run test:protocol
npm run build
npm run tauri build
```

CI-збірка сама по собі не означає підтримку платформи: стабільною ціль вважається після реального запуску та smoke-test на відповідній ОС.

GitHub Actions формують окремі артефакти Windows, Linux і macOS. Імена збірок синхронізовані з версією 0.1.1.
