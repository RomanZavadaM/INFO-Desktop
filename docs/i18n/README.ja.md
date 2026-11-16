# INFO Desktop

**INFO Desktop 0.1.6 安定版 — Windows / Linux / macOS:** [ダウンロード](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.6)。新機能：初期設定フォーム。前回の安定版：[0.1.5](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.5)。

## 機能

- `info-desktop-protocol/1` で実際の INFO サーバーに接続、端末の一度限りの登録；
- 端末のシークレットは OS の安全な保管領域にのみ保存；
- チャンネル、全待ちキュー、項目の詳細、出典、翻訳、キャッシュ済みメディア；
- タイトル・本文・チャンネルの編集、保存・スキップ・保留・公開；
- 手動での項目作成、Telegram 風プレビュー；
- Excel（XLSX）による設定：エクスポート、適用なしの検証、「48 時間で何が変わるか」の試行、確認付き適用、ロールバック；
- トピック：キーワード編集とサンプル文での確認；
- **初期設定フォーム**（0.1.6 以降）、コンソールモード、7 つの UI 言語。

## 初期設定フォーム

上部バーの「初期設定」ボタンは、サーバーに接続していなくても動作します。フォームは編集部名、ボット名、アクセス値、チャンネルを尋ね、各項目の下に説明と例があり、形式（ボットトークン、ID、チャンネルアドレス `@name`、t.me リンク、サイト、タイムゾーン）をその場で検証します。結果はお使いのコンピューター上の `info-first-setup.xlsx` ファイルで、**サーバーには何も送信されません**。ファイルをサーバー（例：`/var/lib/info/inbox/`）に置き、`info menu` の項目 1 で読み込み、アクセス値を含むため適用後は削除してください。INFO サーバー `20261116-1300` 以降が必要です。詳細：[FIRST_SETUP_FORM](../FIRST_SETUP_FORM.md)。

## コンソールモード

`info-desktop-cli` プログラムはウィンドウと同じ操作を行います：キュー、「このチャンネルに入る理由」、作成と編集、公開、XLSX 設定（ダウンロード、検証、試行、適用、ロールバック）、トピック。動作は 2 通りです：開いているウィンドウ経由（アプリ内のチェックボックス、このコンピューター内のみ、キーは渡されません）、または単独（接続プロファイルはウィンドウが書き込み、キーは OS の保管領域に残ります）。確認 `--yes` は設定の適用とロールバックでのみ必要です。マニュアル：[CONSOLE_MANUAL](ja/CONSOLE_MANUAL.md)。

## 確認済みの内容

2026 年 10 月 20 日に Desktop を実際の INFO サーバー `20261020-2040` に接続し、Desktop から作成した実際の項目を編集キューと Telegram 編集ボットで確認しました（Desktop → API → データベース → キュー）。Telegram への公開は行っていません。コンソールの読み取りコマンドは所有者が Windows で確認しました。初期設定フォームは自動ビルドでのみ検証されており、作成されたファイルを Excel で開くことやサーバーへの読み込みは未確認です。

## 対応プラットフォーム

Windows 10/11 x64 · macOS ARM64 · macOS Intel x86_64 · Linux x64。Windows 7 は必須対象ではありません。

## セキュリティ

SQLite への直接アクセスなし、アプリケーションプロトコルとしての SSH なし、リポジトリに本番シークレットなし。端末の認証情報は権限が限定され、INFO ゲートウェイは VPN 経由のみです。

## 技術とビルド

Tauri 2 · TypeScript · React · Rust · reqwest/rustls.

```bash
npm install
npm run check
npm run test:protocol
npm run build
npm run tauri build
```

[ビルド](../BUILD.md)、[ユーザーマニュアル](../guides/USER_MANUAL.md)、[プロトコル](../protocol/INFO_DESKTOP_PROTOCOL.md)、[登録](../protocol/INFO_DESKTOP_REGISTRATION.md)、[リリース一覧](../releases/RELEASE_INDEX.md)を参照してください。

## リリース状況

現在の安定版は **v0.1.6** です。Windows・Linux・macOS 向けパッケージは GitHub Releases で公開されています。

**Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved. Proprietary software.**
