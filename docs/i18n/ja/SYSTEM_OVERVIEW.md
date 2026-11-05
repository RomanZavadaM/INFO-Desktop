# INFO Desktop — システム概要

INFO Desktop is a separate cross-platform editorial workstation for INFO-compatible servers.

The Desktop client owns UI, localization, navigation, previews and permitted editor actions. The server owns data, permissions, routing, quality, translations, publication, validation and audit.

Desktop is **not a second server**: it has no direct database access and does not duplicate server business logic.

Transport: `info-desktop-protocol/1`, TLS, authenticated GET + scoped POST, OS secure credential storage. The current production gateway remains VPN-only.

2026-10-20 の INFO 20261020-2040 本番受入で手動記事作成を end-to-end で確認しました。公開 Telegram SEND は実行していません。

## コンソールモード
コンソール `info-desktop-cli`（パッケージ `src-tauri/cli`）とウィンドウは共通パッケージ `src-tauri/core`（登録、転送、パス、プロファイル、ローカルブリッジ）と同じプロトコル `info-desktop-protocol/1` を使用し、プロトコルは変更されていません。開いているウィンドウ経由ではユーザーの同意の下で動作します：ローカル 127.0.0.1 の入口と一回限りのセッションパスワード、デバイス鍵は渡されません。単独動作では、コンソールは接続プロファイル（ウィンドウが書き込み）と OS 資格情報ストアの鍵を使用します。`scripts/check-static.mjs` の検査により、すべてのネイティブモジュールで編集用 POST が 1 つ、鍵用 POST が 2 つに保たれます。
