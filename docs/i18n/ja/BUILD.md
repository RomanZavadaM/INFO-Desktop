# INFO Desktop — ビルドと対応環境

現在の承認済みソースチェックポイント: main の 0.1.1。

対応対象: Windows 10/11 x64、macOS arm64、macOS x86_64、Linux x64。Windows 7 は必須ではありません。

```bash
npm ci
npm run check
npm run test:protocol
npm run build
npm run tauri build
```

CI artifact does not by itself prove platform support; a real launch/smoke test is required.
