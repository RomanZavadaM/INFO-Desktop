# INFO Desktop — 사용자 매뉴얼

1. Install INFO Desktop.
2. Select the interface language.
3. Import the administrator-provided registration file.
4. Activate the device once; the reusable secret is stored in the OS secure credential store.
5. Connect to the INFO server.
6. Select a channel and item from the editorial queue.
7. Review sources/media, edit title/text/channel and save.
8. Use skip/defer/create/publish only when the server grants the required scopes.

Manual create sends title, text, channel and optional source URL to the server and places the result into the normal editorial queue.

Desktop은 SQLite에 직접 접근하지 않고 SSH를 앱 프로토콜로 사용하지 않으며 재사용 기기 비밀키를 OS 보안 저장소에만 보관하고 서버 scope 권한을 사용합니다.

2026-10-20 INFO 20261020-2040 운영 검증에서 수동 기사 생성이 end-to-end로 확인되었습니다. 공개 Telegram SEND는 수행하지 않았습니다.
