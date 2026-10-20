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
- temporary `wireguard-bootstrap-v1` first-connection model;
- permanent X25519 device key generation into OS secure storage;
- registration token/bootstrap secret removed from UI state;
- Google Drive `INFO Desktop` catalog and subfolders;
- Drive architecture/roadmap document.

### DOING
- cross-platform CI verification for secure-storage/bootstrap code;
- bootstrap → permanent-key enrollment handoff contract.

### NEXT
1. Finish CI for `0.1-r1` secure registration stack.
2. Finish bootstrap → permanent peer handoff contract.
3. Define revoke/rotation and bootstrap cleanup flows.
4. Present exact production WireGuard/port/firewall plan before touching networking.
5. Perform real LIVE read-only acceptance only after protected transport is ready.

### HARD BOUNDARY
Do not alter production VPN/network/firewall/SSH, expose the INFO API publicly, or deploy PR #18 without an explicit owner-approved production step.
