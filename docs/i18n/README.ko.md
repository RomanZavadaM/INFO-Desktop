# INFO Desktop

**안정 버전 이후 개발:** `main`은 source **0.1.5 (development)** 로 이동합니다. 아직 Release가 아니며 다운로드 설치 파일은 검증된 v0.1.4입니다.

**INFO Desktop v0.1.4 안정 버전 — Windows / Linux / macOS:** [설치 파일 다운로드](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.4). 시각적 편집기, Windows 콘솔 숨김, 타임스탬프가 포함된 XLSX 내보내기.




## 주요 기능

- `info-desktop-protocol/1`을 통한 실제 INFO 서버 연결;
- 일회용 기기 등록 파일;
- 재사용 기기 비밀키를 OS 보안 저장소에만 저장;
- 채널, 전체 대기열, 기사 상세, 출처 및 캐시 미디어 조회;
- 제목, 본문, 대상 채널 편집;
- 저장, 건너뛰기, 연기;
- 새 기사 수동 생성;
- Telegram 스타일 게시 미리보기;
- 권한 scope 기반 게시;
- 7개 UI 언어.

## 운영 환경 검증

2026년 10월 20일 INFO Desktop을 LIVE INFO `20261020-2040`에 연결했습니다. 실제 Lviv 기사를 Desktop에서 생성했고 편집 대기열과 Telegram 편집 봇에 나타나는 것을 확인했습니다. Desktop → API → 데이터베이스 → 편집 대기열 경로가 검증되었습니다. 이 검증에서는 공개 Telegram SEND를 수행하지 않았습니다.

## 플랫폼

Windows 10/11 x64 · macOS ARM64 · macOS Intel x86_64 · Linux x64. Windows 7은 필수 지원 대상이 아닙니다.

## 보안

SQLite 직접 접근 없음, 애플리케이션 프로토콜로 SSH 사용 안 함, 저장소에 운영 비밀정보 없음. 기기 권한은 scope로 제한되며 현재 gateway는 VPN-only입니다.

## 기술 및 빌드

Tauri 2 · TypeScript · React · Rust · reqwest/rustls.

```bash
npm install
npm run check
npm run test:protocol
npm run build
npm run tauri build
```

[Build](../BUILD.md), [사용자 매뉴얼](../guides/USER_MANUAL.md), [프로토콜](../protocol/INFO_DESKTOP_PROTOCOL.md), [등록](../protocol/INFO_DESKTOP_REGISTRATION.md), [릴리스 인덱스](../releases/RELEASE_INDEX.md)를 참조하세요.

## 릴리스 상태

현재 안정 버전은 **v0.1.4**입니다. Windows, Linux, macOS용 GitHub Release가 게시되었습니다. 새 시각적 편집기의 Windows WebView2 수동 검수는 아직 남아 있습니다.

**Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved. Proprietary software.**
