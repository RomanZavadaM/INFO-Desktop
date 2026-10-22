# INFO Desktop protocol

**Identifier:** `info-desktop-protocol/1`

Base path: `/api/info-desktop/v1`.

Read operations: handshake, status, channels, paginated queue, item detail and authenticated media preview.

Scoped write operations: save/edit, skip, defer, manual create and publish. Access is controlled server-side through scopes such as `item.write`, `item.create` and `publish.write`.

Queue data is a prepared editorial view; clicks must not trigger heavy routing/quality recomputation. Desktop never writes directly to the database.

Credentials are device secrets and must not be committed to Git or packaged into releases. Unknown fields are ignored; incompatible protocol identifiers are rejected.

- `GET /queue/{id}/why?channel=&scope=` - explains channel decisions for the event (scope `item.read`, read-only): `channels` (code, mode, ok, reason) and `rules` (the XLSX condition tree for the chosen channel)

- XLSX settings (scopes `settings.read`, `settings.write`): `GET /settings/status`, `GET /settings/xlsx`, `POST /settings/xlsx/check`, `POST /settings/xlsx/apply`, `POST /settings/xlsx/rollback` - export, check without applying, apply with one-step rollback; same format and rules as the Telegram bot and `info menu`.

- Probe: `POST /settings/probe` (202), `GET /settings/probe` (state `none|queued|running|done|error` and report text). Topics: `GET /themes`, `POST /themes/test` `{words,text}`, `POST /themes/save` `{topic,words}` (previous state kept for rollback).
