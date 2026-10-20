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
- server listens, client initiates connection;
- each Desktop receives a separate one-time registration file through a trusted path;
- Desktop generates its own permanent cryptographic key pair locally;
- permanent device private key is generated locally and is never in the registration file;
- the registration file may contain a separate short-lived bootstrap transport private key delivered only through a trusted out-of-band path;
- server enrolls the public key and can revoke a device independently;
- target protected transport uses digital device keys and a non-standard public port;
- accepted bootstrap profile is `wireguard-bootstrap-v1`; production WireGuard/network deployment is not yet authorized;
- INFO HTTP API itself should remain private behind the protected transport.

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

1. Finish current cross-platform build checks for secure storage/bootstrap registration.
2. Complete bootstrap/permanent key handoff logic without production networking.
3. Complete server-side registration/device-registry tests.
4. Before any VPN/firewall/port change, present exact production transport plan to owner.
