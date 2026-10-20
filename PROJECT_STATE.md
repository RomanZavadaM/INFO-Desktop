# PROJECT_STATE — INFO Desktop

## 2026-10-20
- Repository: `RomanZavadaM/INFO-Desktop` (public)
- main: initial repository only
- active branch: `prototype/read-only-client`
- active PR: #1 draft
- product version: `0.1.0-dev`
- release/stable: none

## Decisions
- Separate universal product.
- Existing server INFO and Telegram stay untouched during bootstrap.
- First prototype is strictly read-only, accepted on real queue/text/media.
- Protocol: `info-desktop-protocol/1`.
- Taxo i18n/license/release discipline.
- Technology: Tauri 2 + TypeScript + React.
- Required targets: Windows 10/11 x64, macOS arm64, macOS Intel, Linux x64.
- Windows 7 removed from required matrix by owner decision; legacy support may be revisited separately.

## Current safety boundary
Native transport exposes only GET for http/https. No queue mutation path exists.

## Next
1. Get CI/build matrix green.
2. Reconcile protocol v1 fields with real INFO data.
3. Only then add a minimal read-only endpoint to private INFO.
4. Connect Desktop to LIVE queue/media without mutation.
