# WORKLOG — INFO Desktop

## Active slice: 0.1-r1 / live editorial workstation
**Branch:** `prototype/read-only-client`  
**PR:** #1 draft  
**Status:** LIVE TESTING

### DONE
- separate public product repository;
- Tauri 2 + TypeScript + React foundation;
- Windows/macOS/Linux build workflows;
- seven UI languages;
- universal `info-desktop-protocol/1`;
- real INFO schema/protocol reconciliation;
- private authenticated image previews through GET-only native layer;
- first private INFO API companion in PR #18;
- LIVE read-only acceptance against INFO succeeded;
- scoped Desktop write API deployed with INFO `20261020-1814` / schema v18;
- edit title/text, change channel, save, skip, defer, publish and manual create implemented;
- Telegram-style publish preview implemented;
- Windows/Linux/macOS builds and frontend/Rust checks green for commit `731081f6`;
- Windows artifact contains EXE + MSI + SHA256SUMS;
- media restricted to INFO cache; remote originals not fetched automatically;
- owner-approved one-file device registration model;
- `info-desktop-registration/1` specification;
- one-file registration import;
- server-issued device credential model over TLS;
- reusable device secret stored only in OS secure storage;
- registration/device secrets removed from UI state;
- old WireGuard/X25519 client design retired before merge;
- Google Drive `INFO Desktop` catalog and subfolders;
- Drive architecture/roadmap document.

### DOING
- live Windows acceptance of write actions against production INFO through VPN-only gateway;
- visual refinement of Telegram preview with real newsroom cards/media.

### NEXT
1. Verify save + channel move without publication.
2. Verify one deliberate publish from Desktop and confirm Telegram/journal result.
3. Verify manual material creation.
4. Add TikTok adapter/preview as the next platform, then Facebook.
5. Keep gateway VPN-only while the editor is being stabilized.

### HARD BOUNDARY
Do not alter production VPN/network/firewall/SSH or expose INFO Desktop publicly without explicit owner approval. Current gateway is intentionally VPN-only.
