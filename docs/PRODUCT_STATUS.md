# INFO Desktop — Product Status

## Current

Version: `0.1.1`  
Stage: first accepted newsroom client  
Default branch: `main`  
PR #1: merged 2026-10-20  
Main merge commit: `852ee086769f2ce02246611d8772dfbb6ddc423d`

## Accepted product scope

INFO Desktop is a separate universal desktop newsroom client for INFO-compatible servers.

Implemented:
- Tauri 2 + TypeScript + React/Rust;
- seven UI languages;
- Windows 10/11 x64, macOS ARM64, macOS Intel x86_64, Linux x64 build matrix;
- versioned `info-desktop-protocol/1`;
- one-file device registration and OS secure credential storage;
- authenticated GET + scoped POST transport;
- channels and full pending queue with pagination;
- real item detail, source materials and cached-media preview;
- title/text/channel editing;
- save, skip, defer and scoped publish actions;
- manual material creation;
- Telegram-style publish preview.

## Production acceptance

On 2026-10-20 the client was connected to LIVE INFO `20261020-2040`.
Manual creation of one real Lviv editorial item was accepted end-to-end:
Desktop → API → SQLite → editorial queue → Telegram editorial bot.
The acceptance did not perform a public Telegram SEND.

## Server companion

Private repository `RomanZavadaM/INFO`, PR #18, contains the server-side Desktop API / registration / gateway work used by LIVE 2040. It still needs repository reconciliation/merge into INFO main separately.

## Not yet complete

- public GitHub Release with installers/assets;
- installer signing/notarization/update policy;
- deliberate end-to-end Desktop publish acceptance;
- TikTok publishing/preview adapter;
- Facebook publishing/preview adapter;
- broader UI polish and long-session regression.

Existing Telegram INFO remains parallel and operational.
