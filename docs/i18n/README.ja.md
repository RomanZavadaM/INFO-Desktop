# INFO Desktop

**INFO Desktop v0.1.4 安定版 — Windows / Linux / macOS：** [インストーラーをダウンロード](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.4)。ビジュアルエディター、Windows コンソールの非表示、日時付き XLSX エクスポート。




## 主な機能

- `info-desktop-protocol/1` による実際の INFO サーバー接続;
- 1回限りのデバイス登録ファイル;
- 再利用可能なデバイス秘密情報をOSの安全なストレージにのみ保存;
- チャンネル、全保留キュー、記事詳細、ソース、キャッシュ済みメディア;
- タイトル、本文、配信先チャンネルの編集;
- 保存、スキップ、延期;
- 新規記事の手動作成;
- Telegram形式の公開プレビュー;
- scopeで制限された公開操作;
- 7言語のUI.

## 本番環境での受入確認

2026年10月20日、INFO Desktop を LIVE INFO `20261020-2040` に接続しました。実際の Lviv 記事を Desktop から作成し、編集キューと Telegram 編集ボットに表示されることを確認しました。Desktop → API → データベース → 編集キューの経路を検証済みです。この受入確認では公開 Telegram SEND は実行していません。

## 対応プラットフォーム

Windows 10/11 x64 · macOS ARM64 · macOS Intel x86_64 · Linux x64。Windows 7 は必須対象ではありません。

## セキュリティ

SQLiteへの直接アクセスなし、SSHをアプリケーションプロトコルとして使用しない、production secretsをリポジトリに保存しない。デバイス権限はscopeで制限され、現在のgatewayはVPN-onlyです。

## 技術とビルド

Tauri 2 · TypeScript · React · Rust · reqwest/rustls.

```bash
npm install
npm run check
npm run test:protocol
npm run build
npm run tauri build
```

[Build](../BUILD.md)、[ユーザーマニュアル](../guides/USER_MANUAL.md)、[プロトコル](../protocol/INFO_DESKTOP_PROTOCOL.md)、[登録](../protocol/INFO_DESKTOP_REGISTRATION.md)、[リリース一覧](../releases/RELEASE_INDEX.md)を参照してください。

## リリース状況

現在の安定版は **v0.1.4** です。Windows、Linux、macOS 向け GitHub Release は公開済みです。新しいビジュアルエディターの Windows WebView2 手動受入確認は未完了です。

**Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved. Proprietary software.**
