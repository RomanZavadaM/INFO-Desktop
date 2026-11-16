# INFO Desktop

**INFO Desktop 0.1.6 안정판 — Windows / Linux / macOS:** [다운로드](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.6). 신규: 최초 설정 양식. 이전 안정판: [0.1.5](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.5).

## 기능

- `info-desktop-protocol/1`로 실제 INFO 서버에 연결, 기기 1회 등록;
- 기기 비밀 값은 운영체제의 보안 저장소에만 보관;
- 채널, 전체 대기열, 항목 상세, 출처, 번역, 캐시된 미디어;
- 제목·본문·채널 편집; 저장, 건너뛰기, 보류, 게시;
- 수동 항목 생성, Telegram 스타일 미리보기;
- Excel(XLSX)로 설정: 내보내기, 적용 없이 검사, “48시간 내 변경 사항” 시험, 확인 후 적용, 되돌리기;
- 주제: 키워드 편집과 예시 텍스트로 검사;
- **최초 설정 양식**(0.1.6부터), 콘솔 모드, 7개 인터페이스 언어.

## 최초 설정 양식

상단 막대의 “최초 설정” 버튼은 서버에 연결하지 않아도 동작합니다. 양식은 편집부 이름, 봇 제목, 접근 값, 채널을 묻고, 각 입력란 아래에 설명과 예시가 있으며 형식(봇 토큰, ID, 채널 주소 `@이름`, t.me 링크, 사이트, 시간대)을 즉시 검사합니다. 결과는 사용자 컴퓨터의 `info-first-setup.xlsx` 파일이며 **서버로 아무것도 전송되지 않습니다**. 파일을 서버(예: `/var/lib/info/inbox/`)에 두고 `info menu`의 1번 항목으로 불러온 뒤, 접근 값이 들어 있으므로 삭제하세요. INFO 서버 `20261116-1300` 이상이 필요합니다. 자세한 내용: [FIRST_SETUP_FORM](../FIRST_SETUP_FORM.md).

## 콘솔 모드

`info-desktop-cli` 프로그램은 창과 같은 작업을 수행합니다: 대기열, “이 채널에 있는 이유”, 생성·편집, 게시, XLSX 설정(다운로드, 검사, 시험, 적용, 되돌리기), 주제. 두 가지 방식으로 동작합니다: 열린 창을 통해(앱의 체크박스, 이 컴퓨터에서만, 키는 전달되지 않음) 또는 단독으로(연결 프로필은 창이 기록하고 키는 OS 저장소에 유지). 확인 `--yes`는 설정 적용과 되돌리기에만 필요합니다. 설명서: [CONSOLE_MANUAL](ko/CONSOLE_MANUAL.md).

## 검증된 내용

2026년 10월 20일 Desktop을 실제 INFO 서버 `20261020-2040`에 연결해 실제 항목을 Desktop으로 만들고 편집 대기열과 Telegram 편집 봇에서 확인했습니다(Desktop → API → 데이터베이스 → 대기열). 공개 Telegram 게시는 수행하지 않았습니다. 콘솔 읽기 명령은 소유자가 Windows에서 확인했습니다. 최초 설정 양식은 자동 빌드로만 검증되었으며, 생성된 파일을 Excel에서 열고 서버에 불러오는 것은 아직 확인되지 않았습니다.

## 플랫폼

Windows 10/11 x64 · macOS ARM64 · macOS Intel x86_64 · Linux x64. Windows 7은 필수 대상이 아닙니다.

## 보안

SQLite 직접 접근 없음, SSH를 응용 프로토콜로 사용하지 않음, 저장소에 운영 비밀 값 없음. 기기 자격 증명은 범위가 제한되며 INFO 게이트웨이는 VPN으로만 접근할 수 있습니다.

## 기술 및 빌드

Tauri 2 · TypeScript · React · Rust · reqwest/rustls.

```bash
npm install
npm run check
npm run test:protocol
npm run build
npm run tauri build
```

[빌드](../BUILD.md), [사용자 설명서](../guides/USER_MANUAL.md), [프로토콜](../protocol/INFO_DESKTOP_PROTOCOL.md), [등록](../protocol/INFO_DESKTOP_REGISTRATION.md), [릴리스 색인](../releases/RELEASE_INDEX.md)을 참조하세요.

## 릴리스 상태

현재 안정판은 **v0.1.6**입니다. Windows, Linux, macOS 패키지는 GitHub Releases에 게시되어 있습니다.

**Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved. Proprietary software.**
