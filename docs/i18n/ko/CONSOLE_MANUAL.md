# INFO Desktop 콘솔 모드: 간단 설명서

상태: 개발 중(버전 0.1.5, 미출시). 두 가지 모드: 열려 있는 애플리케이션 창을 통한 모드와 독립 모드.

## 개요
`info-desktop-cli` 프로그램은 창과 같은 일을 합니다: 대기열 표시, 경로 설명, 항목 생성 및 편집, 게시, XLSX 설정 작업. 장치 키는 창과 같은 OS 보안 저장소에서 가져오며 출력되지 않습니다.

## 모드
- **열려 있는 창을 통해(공유 연결).** 애플리케이션 창에서 "이 창을 통해 콘솔이 동작하도록 허용"을 체크하세요(기본값은 꺼짐). 창이 열려 있는 동안 콘솔은 창의 연결을 사용합니다: VPN과 키는 이미 설정되어 있고 키는 전달되지 않습니다. 입구는 이 컴퓨터 안에서만(127.0.0.1) 열리며, 사용자 설정 폴더에 저장된 일회용 세션 암호로 보호됩니다. 창은 체크박스 옆에 콘솔의 모든 동작을 표시합니다.
- **독립.** 창이 필요 없습니다: 콘솔이 연결 프로필과 OS 저장소의 키로 서버에 직접 연결합니다. 창은 연결할 때마다 프로필(주소, 포트, 장치 번호, 서버 인증서; 키 없음)을 기록하며, 수동으로 설정할 수도 있습니다(`profile set`).
- 선택: `--mode auto`(기본값: 창이 열려 있고 허용하면 창, 아니면 독립), `--mode app`(창만), `--mode alone`(독립만). `--mode` 없이 명시적인 `--profile`을 쓰면 독립을 뜻합니다.

## 첫 실행
1. 평소처럼 애플리케이션 창에서 연결하세요(Windows에서 우리 서버를 쓸 때는 VPN이 켜져 있어야 합니다).
2. 프로필은 창이 직접 기록합니다. 수동이 필요한 경우만: `info-desktop-cli profile set --server 주소 --port 포트 --device-id 번호 --ca-file 파일`; 파일에는 등록 파일의 `server_ca_der` 값이 들어 있습니다.
3. 연결 확인: `info-desktop-cli status`.

## 명령
인수 없이 `info-desktop-cli`를 실행하면 전체 도움말이 표시됩니다.

| 작업 | 명령 |
|---|---|
| 채널 목록 | `channels` |
| 대기열 | `queue list [--limit N] [--cursor N] [--channel 코드] [--bucket 그룹]` |
| 출처가 있는 항목 | `queue show ID` |
| 이 채널에 있는 이유 | `why ID --channel 코드 [--scope route\|auto\|digest\|instant\|relevance]` |
| 항목 생성 | `create --channel 코드 --title 텍스트 (--text 텍스트 \| --text-file 파일) [--url URL]` |
| 항목 편집 | `save ID --title 텍스트 (--text 텍스트 \| --text-file 파일) [--channel 코드]` |
| 건너뛰기, 보류, 게시 | `skip ID`, `defer ID [--minutes N]`, `publish ID` |
| XLSX 설정 | `settings status`, `settings download --out 파일`, `settings check 파일`, `settings probe [--hours N]`, `settings probe-status`, `settings apply [--without-probe] [--yes]`, `settings rollback [--yes]` |
| 주제와 단어 | `themes list`, `themes test ...`, `themes save ...` |

## 설정 변경
먼저 `settings check`(적용 없이 파일 검사), 그다음 `settings probe`와 `settings probe-status`("무엇이 바뀌는가"), 마지막으로 `settings apply`. `--yes`가 없으면 명령은 적용될 내용만 보여 줍니다. 검사한 파일과 정확히 같은 파일이 적용됩니다(지문 기준). 시험 없이는 `--without-probe`를 추가하지 않으면 거부합니다. 되돌리기: `settings rollback --yes`.

게시(`publish`)와 일반 편집 작업에는 확인이 필요 없습니다: 수행자의 재량과 책임이며, 게시물은 언제든 채널에서 삭제할 수 있습니다.

## 종료 코드
0 성공, 1 명령 오류, 2 잘못된 인수, 3 연결 또는 키 없음, 4 서버가 거부(권한 없음, 충돌).

## 아직 안 된 것
그래픽 세션이 없는 서버에서의 동작(소유자가 보류). 2026년 11월 5일 Windows에서 실제 서버로 확인(읽기): `status`, `channels`, `queue list`, 창을 통한 `why`, 독립 및 자동 `status`. 미확인: 쓰기 명령, XLSX와 주제, macOS와 Linux.
