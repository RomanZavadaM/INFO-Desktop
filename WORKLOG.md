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

### AUDIT 2026-10-22
- перевірено GitHub: `main` `095bd58`, CI success, Release `v0.1.1` (12 файлів), версії `0.1.1` у `VERSION.txt`, `package.json`, `Cargo.toml`, `tauri.conf.json`;
- звірено хеші Windows EXE/MSI і macOS ARM64 DMG/ZIP з опублікованими `SHA256SUMS-*.txt`: збігаються;
- знайдено розбіжність імен у `SHA256SUMS-*.txt` (пропуски) та в Release (крапки); локальний інсталятор від 22.10 10:16 не є збіркою Release;
- 7 мов документації: по 10 файлів, ключі локалей синхронні; старих тверджень немає;
- серверна частина: INFO PR #18 злито 21.10; LIVE INFO `20261022-1730`;
- додано `docs/decisions/SIGNING_UPDATES.md` (чернетка політики підпису й оновлень) і оновлено дорожню карту (7 мов).

