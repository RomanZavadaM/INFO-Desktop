# INFO Desktop プロトコル

**Identifier:** `info-desktop-protocol/1`

Base path: `/api/info-desktop/v1`.

Read operations: handshake, status, channels, paginated queue, item detail and authenticated media preview.

Scoped write operations: save/edit, skip, defer, manual create and publish. Access is controlled server-side through scopes such as `item.write`, `item.create` and `publish.write`.

Queue data is a prepared editorial view; clicks must not trigger heavy routing/quality recomputation. Desktop never writes directly to the database.

Credentials are device secrets and must not be committed to Git or packaged into releases. Unknown fields are ignored; incompatible protocol identifiers are rejected.

- `GET /queue/{id}/why?channel=&scope=` - イベントに対するチャンネルの判断を説明します（スコープ `item.read`、読み取り専用）：`channels`（コード、モード、ok、理由）と `rules`（選択したチャンネルの XLSX 条件ツリー）

- XLSX 設定（スコープ `settings.read`、`settings.write`）：`GET /settings/status`、`GET /settings/xlsx`、`POST /settings/xlsx/check`、`POST /settings/xlsx/apply`、`POST /settings/xlsx/rollback` - エクスポート、適用なしの確認、1 段階のロールバック付き適用；Telegram ボットや `info menu` と同じ形式とルール。

- 試行：`POST /settings/probe`（202）、`GET /settings/probe`（状態 `none|queued|running|done|error` とレポート本文）。トピック：`GET /themes`、`POST /themes/test` `{words,text}`、`POST /themes/save` `{topic,words}`（元の状態はロールバック用に保持）。
