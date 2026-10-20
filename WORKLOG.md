# WORKLOG — INFO Desktop

## 0.1.1 — STABLE RELEASE
**Status:** MERGED / LIVE-CONNECTED ACCEPTANCE PASSED / GITHUB RELEASE PUBLISHED

### DONE
- public INFO Desktop repository;
- Tauri 2 + React/TypeScript/Rust;
- Windows/macOS/Linux workflows;
- seven UI languages;
- multilingual documentation with Ukrainian canonical;
- protocol + registration;
- real INFO connection;
- secure credential storage;
- full queue/item/source/media;
- edit/save/skip/defer;
- manual create;
- Telegram-style preview;
- scoped publish action;
- real manual-create acceptance against INFO `20261020-2040`;
- PR #1 merged;
- **[GitHub Release v0.1.1](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.1) published**;
- Windows/Linux/macOS Intel/macOS ARM assets and SHA256 published.

### POST-RELEASE AUDIT — 2026-10-20
- verified public README, release assets and canonical version files;
- removed obsolete `0.1-r1` revision policy from `PROJECT_RULES.md`;
- corrected obsolete GET-only connection architecture to scoped write behavior;
- corrected Rust source User-Agent from `INFO-Desktop/0.1-r1` to `INFO-Desktop/0.1.1`;
- kept published `v0.1.1` immutable; next published build requires a new version.

### NEXT
1. Reconcile INFO server PR #18.
2. Signing/notarization/update policy.
3. Deliberate publish only with explicit owner authorization.
4. TikTok/Facebook adapters and UI polish.

### HARD BOUNDARY
Do not alter production VPN/network/firewall/SSH without explicit owner approval.
