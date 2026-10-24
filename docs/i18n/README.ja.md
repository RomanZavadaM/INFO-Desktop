# INFO Desktop

## 安定版 0.1.2 と次期 0.1.3

**現在公開中の安定版は 0.1.2 です。** Windows、Linux、macOS Intel／Apple Silicon 向けの [v0.1.2 をダウンロード](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.2)。現在の `main` には **未公開の 0.1.3 候補**（XLSX のエクスポート・検査・適用・ロールバック、48時間の変更シミュレーション、トピック別キーワード編集、古い結果の防止）が含まれています。INFO サーバー `20261024-1653` は導入済みですが、Desktop の完全な受入テストは未完了です。記事の保存・スキップ・延期・公開は Excel を経由せず、**サーバー API に直接**送られます。XLSX は自動ルールの設定専用です。Telegram への公開送信は未検証です。


[Українська](../../README.md) · [English](README.en.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Français](README.fr.md) · [한국어](README.ko.md) · [日本語](README.ja.md)

**INFO Desktop は INFO 互換サーバー向けのクロスプラットフォーム編集ワークステーションです。**

バージョン **0.1.2** は `main` に統合された最初の承認済みソースチェックポイントです。

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

`0.1.2` ソースチェックポイントは `main` で承認済みです。See release status above.

**Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved. Proprietary software.**
