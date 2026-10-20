# WORKLOG — INFO Desktop

## Active slice: 0.1-r1 / real read-only newsroom + protected registration
**Branch:** `prototype/read-only-client`  
**PR:** #1 draft  
**Status:** DOING

### DONE
- separate public product repository;
- Tauri 2 + TypeScript + React foundation;
- Windows/macOS/Linux build workflows;
- seven UI languages;
- universal `info-desktop-protocol/1`;
- real INFO schema/protocol reconciliation;
- private authenticated image previews through GET-only native layer;
- first private INFO read-only API candidate in PR #18;
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
- cross-platform CI verification for TLS/device-credential activation;
- server-side credential rotation/revoke contract.

### NEXT
1. Finish CI for `0.1-r1` registration/activation stack.
2. Implement silent server-issued credential rotation.
3. Prepare exact TLS gateway/high-port/firewall plan.
4. Update canonical SERVER_REQUIREMENTS with the actual deployed gateway details.
5. Perform real LIVE read-only acceptance after separately approved server deployment.

### HARD BOUNDARY
Do not alter production VPN/network/firewall/SSH, expose the INFO API publicly, or deploy PR #18 without an explicit owner-approved production step.
