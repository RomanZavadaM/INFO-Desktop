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
- First live read-only acceptance is complete on real newsroom data.
- Current prototype has moved to authenticated editorial read/write actions: save, channel move, skip, defer, publish and manual create.
- Native transport is authenticated GET + scoped POST over the private INFO TLS gateway.
- Cached media preview remains authenticated and memory-only in Desktop.
- Editor includes Telegram-style publish preview; TikTok and Facebook preview tabs are reserved for the next publishing adapters.

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
- device credentials now carry explicit `item.write`, `item.create` and `publish.write` scopes in schema v18.

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
- authenticated Desktop API with scoped write endpoints;
- LIVE server candidate `20261020-1814` deployed and verified;
- gateway remains bound only to VPN address `10.8.0.1:47443`; public firewall/network exposure was not added.

## Google Drive

Created under the main INFO Drive folder:
- `INFO Desktop/`
  - `Документація/`
  - `Реєстрація і ключі/`
  - `Релізи/`

A Google Doc `INFO Desktop — план, архітектура і реєстрація` records the human-readable architecture and roadmap.

## Next

1. Live-test edit/save/channel-change/skip/defer/publish/manual-create from Windows against `20261020-1814`.
2. Refine Telegram preview against real cards/media.
3. Add TikTok publishing/preview adapter, then Facebook, without coupling them to Telegram.
4. Keep the current VPN-only gateway until a separate owner-approved decision on external access or split tunneling.
