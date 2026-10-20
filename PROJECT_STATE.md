# PROJECT_STATE — INFO Desktop

## State 2026-10-20

- Repository: `RomanZavadaM/INFO-Desktop` (public)
- Main: first accepted product code merged
- Main accepted commit: `852ee086769f2ce02246611d8772dfbb6ddc423d`
- PR #1: merged
- Product version: `0.1.1`
- Stable source checkpoint: yes
- Public GitHub Release assets: not yet published

## Architecture accepted

- Tauri 2 + TypeScript + React/Rust.
- Required: Windows 10/11 x64, macOS arm64, macOS x86_64, Linux x64.
- Universal versioned INFO-compatible server protocol.
- Existing Telegram INFO remains parallel and independent.
- Native transport is authenticated GET + scoped POST over private INFO TLS gateway.
- Device credentials use explicit read/write/create/publish scopes.
- Reusable device secret is stored only in OS secure storage.
- Cached media preview is authenticated and memory-only.

## Accepted LIVE behavior

Against INFO LIVE `20261020-2040`:
- connection/handshake works;
- channels/full queue/item detail work;
- cached media preview works;
- manual material creation works end-to-end;
- created Lviv material reached the editorial queue and Telegram editorial bot.

No public Telegram SEND was performed as part of that acceptance.

## Server companion

Private `RomanZavadaM/INFO`:
- branch `feature/desktop-readonly-api`;
- PR #18;
- server API/registration/gateway code corresponding to LIVE 2040;
- gateway remains VPN-only;
- INFO main still needs reconciliation/merge.

## Next

1. Reconcile INFO PR #18 into private INFO main.
2. Publish first GitHub Release installers after release packaging/signing decision.
3. Deliberately test publish only with owner approval.
4. Continue preview/platform adapters.
