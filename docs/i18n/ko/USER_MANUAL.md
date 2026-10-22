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

## 서버 설정(XLSX)
머리글의 "설정" 버튼: "현재 설정 내려받기"는 XLSX를 저장합니다. 편집한 파일은 "XLSX 파일 선택"으로 고르며 서버는 확인만 하고 요약을 보여줍니다. "확인된 파일 적용"은 확인 후 적용하고 "마지막 적용 되돌리기"는 이전 상태로 되돌립니다. `settings.read` 및 `settings.write` scope가 필요합니다(등록 시 발급). "48시간 내 변경 사항" 시험은 아직 서버에서 `info xlsx try-pending`로 실행합니다.

