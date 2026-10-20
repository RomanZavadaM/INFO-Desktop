# INFO Desktop — Product Status

## Current

Version: `0.1-r1`  
Stage: read-only prototype  
Branch: `prototype/read-only-client`  
PR: #1 draft  
Development ledger: Issue #2

## Working product scope

INFO Desktop is a separate universal desktop newsroom client for INFO-compatible servers.

Current implementation:
- Tauri 2 + TypeScript + React;
- seven UI languages;
- prepared-queue three-pane newsroom layout;
- versioned `info-desktop-protocol/1`;
- native GET-only transport;
- Windows/macOS/Linux CI build matrix.

## Not yet accepted

- no stable release;
- no live server endpoint yet;
- no real LIVE queue acceptance yet;
- no write/editorial actions;
- media authentication/fetch details not finalized;
- no updater/signing/notarization policy yet.

## Parallel system

Existing INFO server and Telegram client remain independent and untouched by this prototype.
