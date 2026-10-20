# INFO Desktop — 시스템 개요

INFO Desktop is a separate cross-platform editorial workstation for INFO-compatible servers.

The Desktop client owns UI, localization, navigation, previews and permitted editor actions. The server owns data, permissions, routing, quality, translations, publication, validation and audit.

Desktop is **not a second server**: it has no direct database access and does not duplicate server business logic.

Transport: `info-desktop-protocol/1`, TLS, authenticated GET + scoped POST, OS secure credential storage. The current production gateway remains VPN-only.

2026-10-20 INFO 20261020-2040 운영 검증에서 수동 기사 생성이 end-to-end로 확인되었습니다. 공개 Telegram SEND는 수행하지 않았습니다.
