# INFO Desktop 프로토콜

**Identifier:** `info-desktop-protocol/1`

Base path: `/api/info-desktop/v1`.

Read operations: handshake, status, channels, paginated queue, item detail and authenticated media preview.

Scoped write operations: save/edit, skip, defer, manual create and publish. Access is controlled server-side through scopes such as `item.write`, `item.create` and `publish.write`.

Queue data is a prepared editorial view; clicks must not trigger heavy routing/quality recomputation. Desktop never writes directly to the database.

Credentials are device secrets and must not be committed to Git or packaged into releases. Unknown fields are ignored; incompatible protocol identifiers are rejected.

- `GET /queue/{id}/why?channel=&scope=` - 이벤트에 대한 채널 결정을 설명합니다(scope `item.read`, 읽기 전용): `channels`(코드, 모드, ok, 이유)와 `rules`(선택한 채널의 XLSX 조건 트리)

- XLSX 설정(scope `settings.read`, `settings.write`): `GET /settings/status`, `GET /settings/xlsx`, `POST /settings/xlsx/check`, `POST /settings/xlsx/apply`, `POST /settings/xlsx/rollback` - 내보내기, 적용 없이 확인, 한 단계 되돌리기가 있는 적용; Telegram 봇 및 `info menu`와 같은 형식과 규칙.

- 시험: `POST /settings/probe`(202), `GET /settings/probe`(상태 `none|queued|running|done|error` 및 보고서 텍스트). 주제: `GET /themes`, `POST /themes/test` `{words,text}`, `POST /themes/save` `{topic,words}`(이전 상태는 되돌리기용으로 보관).
