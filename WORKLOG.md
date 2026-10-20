# WORKLOG — INFO Desktop

## 0.1.1 — accepted main checkpoint
**Status:** MERGED / LIVE-CONNECTED ACCEPTANCE PASSED  
**Main:** `852ee086769f2ce02246611d8772dfbb6ddc423d`

### DONE
- separate public INFO Desktop repository;
- Tauri 2 + TypeScript + React/Rust foundation;
- Windows/macOS/Linux build workflows;
- seven UI languages;
- universal `info-desktop-protocol/1`;
- secure one-file device registration;
- reusable credential only in OS secure storage;
- real INFO connection and read-only acceptance;
- authenticated cached media preview;
- scoped Desktop write API integration;
- edit title/text and change channel;
- save, skip and defer;
- scoped publish action implemented;
- manual create implemented;
- Telegram-style preview;
- real manual-create production acceptance against INFO `20261020-2040`;
- PR #1 merged to main.

### ACCEPTANCE FACT
One real Lviv item was created through Desktop and appeared in the editorial Telegram bot. This validated Desktop → API → database → editorial queue. No public Telegram SEND was performed by that acceptance.

### NEXT
1. Reconcile private INFO server PR #18 into INFO main.
2. Publish first GitHub Release/installers after packaging/signing decision.
3. Verify one deliberate publish only with explicit owner authorization.
4. Improve Telegram preview and add TikTok/Facebook adapters.

### HARD BOUNDARY
Do not alter production VPN/network/firewall/SSH or expose INFO Desktop publicly without explicit owner approval. Current gateway remains VPN-only.
