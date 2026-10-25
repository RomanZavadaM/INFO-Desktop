# INFO Desktop — 제품 상태

stable v0.1.4 이후 source 개발: **0.1.5 / development** 는 아직 공개 Release가 아닙니다. 안정 다운로드는 계속 v0.1.4입니다. Source 0.1.5에는 더 빠른 대기열 작업 흐름, 전체 대기열의 점진적 로딩, 검색/bucket 필터, 명시적 새로고침, «저장 후 다음», 저장하지 않은 변경 보호가 포함됩니다. INFO Server/API는 변경하지 않았습니다. stable v0.1.4의 Windows 수동 검수는 아직 남아 있습니다.

현재 안정 버전: **v0.1.4** (2026-10-24).  
Release: https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.4  
Release workflow **40128755256: SUCCESS**. Windows, Linux, macOS용 앱 패키지 8개와 SHA256 파일 4개가 게시되었습니다.

구현됨: 안전한 기기 등록, 전체 대기열/기사/출처/미디어, 편집 및 수동 생성, HTML 모드를 포함한 시각적 리치 텍스트 편집기, 보호된 XLSX export/check/probe/apply/rollback, 주제/키워드, 7개 UI 언어, 멀티플랫폼 설치 파일.

검증됨: 이전 production-connected 수동 생성 검수; 10월 24일 별도 gateway 수정 후 변경하지 않은 XLSX가 검사를 통과했습니다. XLSX 설정은 **적용하지 않았습니다**. INFO Server는 별도 프로젝트이며 마지막 확인 LIVE는 `20261024-1653`입니다.

남은 작업: stable 0.1.4의 Windows WebView2 수동 검수, 설치된 release의 콘솔 동작, timestamp XLSX 다운로드, 대기열 성능과 빠른 탐색.
