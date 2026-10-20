# INFO Desktop — ユーザーマニュアル

1. Install INFO Desktop.
2. Select the interface language.
3. Import the administrator-provided registration file.
4. Activate the device once; the reusable secret is stored in the OS secure credential store.
5. Connect to the INFO server.
6. Select a channel and item from the editorial queue.
7. Review sources/media, edit title/text/channel and save.
8. Use skip/defer/create/publish only when the server grants the required scopes.

Manual create sends title, text, channel and optional source URL to the server and places the result into the normal editorial queue.

Desktop は SQLite に直接アクセスせず、SSH をアプリケーションプロトコルに使わず、再利用可能なデバイス秘密情報を OS の安全なストレージにのみ保存し、サーバーの scope 権限を使用します。

2026-10-20 の INFO 20261020-2040 本番受入で手動記事作成を end-to-end で確認しました。公開 Telegram SEND は実行していません。
