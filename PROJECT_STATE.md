# PROJECT_STATE — INFO Desktop

## State 2026-10-20

- Repository: `RomanZavadaM/INFO-Desktop` (public)
- Main: initial repository only; prototype not merged
- Active branch: `prototype/read-only-client`
- Active PR: #1 draft
- Development ledger: Issue #2
- Product version: `0.1-r1` (runtime SemVer `0.1.1`)
- Stable release: none

## Architecture accepted

- Tauri 2 + TypeScript + React.
- Required: Windows 10/11 x64, macOS arm64, macOS x86_64, Linux x64.
- Windows 7 is not required.
- Universal versioned INFO-compatible server protocol.
- Existing Telegram INFO remains parallel and independent.
- First product stage is read-only on real newsroom data.
- Native Desktop transport surface is GET-only.
- Cached media preview is authenticated and memory-only in Desktop.

## Registration / connection direction

Owner accepted:
- server-side security may be stronger/complex; editor client stays simple;
- no separate VPN/WireGuard/helper installation for the editor;
- each Desktop receives a one-time registration file through a trusted path;
- registration file contains server, high TLS port, registration ID/token and expiry;
- server issues a reusable high-entropy `device_secret` after one-time activation;
- Desktop stores the reusable secret only in the OS secure credential store;
- UI/localStorage keep only non-secret server URL + `device_id`;
- server stores only credential hashes;
- server controls expiry, revoke and transparent rotation;
- newsroom API remains read-only in 0.1-r1.

Canonical spec: `docs/protocol/INFO_DESKTOP_REGISTRATION.md`.

## Third-party media

- INFO media is editorial source material, not product-owned archive.
- Existing cached photos may live with the editorial material/cache life cycle (currently around the 48-hour material window).
- Remote-only originals are not automatically copied in 0.1.
- Video remains metadata-first.
- Source/provenance remains visible.

## Server companion

Private `RomanZavadaM/INFO`:
- branch `feature/desktop-readonly-api`;
- draft PR #18;
- isolated GET-only read-only API candidate;
- not deployed;
- production network untouched.

## Google Drive

Created under the main INFO Drive folder:
- `INFO Desktop/`
  - `Документація/`
  - `Реєстрація і ключі/`
  - `Релізи/`

A Google Doc `INFO Desktop — план, архітектура і реєстрація` records the human-readable architecture and roadmap.

## Next

1. Finish cross-platform CI for simple TLS/device-credential activation.
2. Complete silent credential rotation endpoint/client flow.
3. Define production TLS gateway/high port/firewall/rate-limit configuration.
4. Before production network changes, take fresh INFO server diagnostics and deploy as a separately approved server step.
