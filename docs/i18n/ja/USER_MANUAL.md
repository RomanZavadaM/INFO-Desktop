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

## サーバー設定（XLSX）
ヘッダーの「設定」ボタン：「現在の設定をダウンロード」で XLSX を保存し、編集したファイルは「XLSX ファイルを選択」で選びます。サーバーは確認のみ行い概要を表示します。「確認済みファイルを適用」は確認後に適用し、「最後の適用を元に戻す」は直前の状態に戻します。スコープ `settings.read` と `settings.write` が必要です（登録時に付与）。「48時間での変更」の試行は現在サーバーで `info xlsx try-pending` により実行します。

### 試行とトピック
「試行：48時間での変更」はサーバーで計算（数分）を開始し、チャンネルごとのレポートを表示します。「トピック」タブ：トピックを選び、単語を編集（`単語*` - 単語の先頭、`単語!` - この形のみ）し、見出しの例で確認して保存します。元に戻す操作は「設定」にあります。

