# Protocolo INFO Desktop

**Identifier:** `info-desktop-protocol/1`

Base path: `/api/info-desktop/v1`.

Read operations: handshake, status, channels, paginated queue, item detail and authenticated media preview.

Scoped write operations: save/edit, skip, defer, manual create and publish. Access is controlled server-side through scopes such as `item.write`, `item.create` and `publish.write`.

Queue data is a prepared editorial view; clicks must not trigger heavy routing/quality recomputation. Desktop never writes directly to the database.

Credentials are device secrets and must not be committed to Git or packaged into releases. Unknown fields are ignored; incompatible protocol identifiers are rejected.

- `GET /queue/{id}/why?channel=&scope=` - explica las decisiones de los canales para el evento (scope `item.read`, solo lectura): `channels` (código, modo, ok, motivo) y `rules` (árbol de condiciones XLSX del canal elegido)

- Ajustes XLSX (scopes `settings.read`, `settings.write`): `GET /settings/status`, `GET /settings/xlsx`, `POST /settings/xlsx/check`, `POST /settings/xlsx/apply`, `POST /settings/xlsx/rollback` - exportar, comprobar sin aplicar, aplicar con reversión de un paso; mismo formato y reglas que el bot de Telegram y `info menu`.

- Prueba: `POST /settings/probe` (202), `GET /settings/probe` (estado `none|queued|running|done|error` y texto del informe). Temas: `GET /themes`, `POST /themes/test` `{words,text}`, `POST /themes/save` `{topic,words}` (el estado anterior se conserva para revertir).
