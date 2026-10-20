# WORKLOG — INFO Desktop

## Active slice: 0.1.0-dev / real read-only newsroom
**Branch:** `prototype/read-only-client`  
**PR:** #1 draft  
**Status:** DOING

### DONE
- public repository, branch, draft PR;
- proprietary legal model;
- universal versioned protocol draft;
- Telegram preserved as parallel working UI;
- read-only first, real-data acceptance;
- Taxo localization/documentation discipline;
- owner decision: Windows 7 removed from required matrix;
- owner decision: Tauri 2 + TypeScript + React;
- first native transport restricted to GET-only.

### DOING
- Tauri/React product skeleton;
- Windows/macOS/Linux CI builds;
- seven-language UI;
- real queue/card/media presentation.

### NEXT
1. Make PR #1 CI/build matrix green.
2. Reconcile protocol with real INFO queue schema.
3. Add a separate read-only API PR to private INFO.
4. Connect to LIVE and inspect ~100+ real pending items.
5. Judge UX before any write API.

### BLOCKED
Live connection requires a read-only server surface. Server INFO remains unchanged until protocol reconciliation.
