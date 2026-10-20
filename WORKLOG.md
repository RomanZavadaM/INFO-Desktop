# WORKLOG — INFO Desktop

## Active slice: real read-only newsroom + registration design
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
- Google Drive `INFO Desktop` catalog and subfolders;
- Drive architecture/roadmap document.

### DOING
- cross-platform final CI verification for the current Desktop head;
- Registration v1 implementation scaffolding and tests.

### NEXT
1. Parse/version/validate registration files.
2. Generate device key material locally.
3. Define enrollment request/response and device registry.
4. Define revoke/rotation flows.
5. Decide protected transport implementation with owner before production network changes.
6. Perform real LIVE read-only acceptance only after protected transport is ready.

### HARD BOUNDARY
Do not alter production VPN/network/firewall/SSH, expose the INFO API publicly, or deploy PR #18 without an explicit owner-approved production step.
