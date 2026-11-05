# INFO Desktop — ビルドと対応環境

現在の承認済みソースチェックポイント: main の 0.1.2。

対応対象: Windows 10/11 x64、macOS arm64、macOS x86_64、Linux x64。Windows 7 は必須ではありません。

```bash
npm install
npm run check
npm run test:protocol
npm run build
npm run tauri build
```

CI artifact does not by itself prove platform support; a real launch/smoke test is required.

## コンソールモード

コンソールは Tauri とウィンドウ用のシステムライブラリなしでビルドできます：

```bash
cd src-tauri
cargo build --release -p info-desktop-cli
cargo test --workspace --lib
```

GitHub Actions ではコンソールがウィンドウと共に Windows、Linux、macOS（ARM64 と Intel）向けにビルドされ、リリースワークフロー `release-0.1.5.yml` がチェックサム付きの個別ファイルとしてパッケージ化します。リリースは所有者の明示的な指示があるときのみ行います。
