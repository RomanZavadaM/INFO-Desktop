# INFO Desktop — 시스템 개요

INFO Desktop is a separate cross-platform editorial workstation for INFO-compatible servers.

The Desktop client owns UI, localization, navigation, previews and permitted editor actions. The server owns data, permissions, routing, quality, translations, publication, validation and audit.

Desktop is **not a second server**: it has no direct database access and does not duplicate server business logic.

Transport: `info-desktop-protocol/1`, TLS, authenticated GET + scoped POST, OS secure credential storage. The current production gateway remains VPN-only.

2026-10-20 INFO 20261020-2040 운영 검증에서 수동 기사 생성이 end-to-end로 확인되었습니다. 공개 Telegram SEND는 수행하지 않았습니다.

## 콘솔 모드
콘솔 `info-desktop-cli`(패키지 `src-tauri/cli`)와 창은 공통 패키지 `src-tauri/core`(등록, 전송, 경로, 프로필, 로컬 브리지)와 같은 프로토콜 `info-desktop-protocol/1`을 사용하며 프로토콜은 변경되지 않았습니다. 열려 있는 창을 통할 때 콘솔은 사용자의 동의로 동작합니다: 로컬 127.0.0.1 입구와 일회용 세션 암호, 장치 키는 전달되지 않습니다. 독립 모드에서 콘솔은 연결 프로필(창이 기록)과 OS 자격 증명 저장소의 키를 사용합니다. `scripts/check-static.mjs` 검사는 모든 네이티브 모듈에서 편집용 POST 하나와 키용 POST 두 개가 유지되도록 확인합니다.
