# Protocole INFO Desktop

**Identifier:** `info-desktop-protocol/1`

Base path: `/api/info-desktop/v1`.

Read operations: handshake, status, channels, paginated queue, item detail and authenticated media preview.

Scoped write operations: save/edit, skip, defer, manual create and publish. Access is controlled server-side through scopes such as `item.write`, `item.create` and `publish.write`.

Queue data is a prepared editorial view; clicks must not trigger heavy routing/quality recomputation. Desktop never writes directly to the database.

Credentials are device secrets and must not be committed to Git or packaged into releases. Unknown fields are ignored; incompatible protocol identifiers are rejected.

- `GET /queue/{id}/why?channel=&scope=` - explique les décisions des canaux pour l'événement (scope `item.read`, lecture seule) : `channels` (code, mode, ok, raison) et `rules` (arbre de conditions XLSX du canal choisi)

- Paramètres XLSX (scopes `settings.read`, `settings.write`) : `GET /settings/status`, `GET /settings/xlsx`, `POST /settings/xlsx/check`, `POST /settings/xlsx/apply`, `POST /settings/xlsx/rollback` - export, vérification sans application, application avec annulation en une étape ; même format et mêmes règles que le bot Telegram et `info menu`.

- Essai : `POST /settings/probe` (202), `GET /settings/probe` (état `none|queued|running|done|error` et texte du rapport). Thèmes : `GET /themes`, `POST /themes/test` `{words,text}`, `POST /themes/save` `{topic,words}` (l'état précédent est conservé pour l'annulation).
