# INFO-Desktop-Protokoll

**Identifier:** `info-desktop-protocol/1`

Base path: `/api/info-desktop/v1`.

Read operations: handshake, status, channels, paginated queue, item detail and authenticated media preview.

Scoped write operations: save/edit, skip, defer, manual create and publish. Access is controlled server-side through scopes such as `item.write`, `item.create` and `publish.write`.

Queue data is a prepared editorial view; clicks must not trigger heavy routing/quality recomputation. Desktop never writes directly to the database.

Credentials are device secrets and must not be committed to Git or packaged into releases. Unknown fields are ignored; incompatible protocol identifiers are rejected.

- `GET /queue/{id}/why?channel=&scope=` - erklärt die Kanalentscheidungen für das Ereignis (Scope `item.read`, nur lesen): `channels` (Code, Modus, ok, Grund) und `rules` (XLSX-Bedingungsbaum des gewählten Kanals)

- XLSX-Einstellungen (Scopes `settings.read`, `settings.write`): `GET /settings/status`, `GET /settings/xlsx`, `POST /settings/xlsx/check`, `POST /settings/xlsx/apply`, `POST /settings/xlsx/rollback` - Export, Prüfung ohne Anwendung, Anwendung mit einstufigem Rollback; gleiches Format und gleiche Regeln wie im Telegram-Bot und in `info menu`.

- Probe: `POST /settings/probe` (202), `GET /settings/probe` (Zustand `none|queued|running|done|error` und Berichtstext). Themen: `GET /themes`, `POST /themes/test` `{words,text}`, `POST /themes/save` `{topic,words}` (vorheriger Stand bleibt für Rollback).
