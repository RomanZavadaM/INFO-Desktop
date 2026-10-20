# PROJECT_STATE — INFO Desktop

## State 2026-10-20
- Repository: `RomanZavadaM/INFO-Desktop`
- Main: accepted product code
- Product version: `0.1.1`
- PR #1: merged
- GitHub Release: **[v0.1.1](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.1)**
- Release assets: Windows EXE/MSI; Linux AppImage/DEB; macOS ARM64 DMG/ZIP; macOS Intel DMG/ZIP; SHA256 files
- Production acceptance: passed against INFO LIVE `20261020-2040`

## Accepted behavior
Connection, channels, full queue, item details, cached media, editing and manual create work against real INFO. One real Lviv material reached the editorial queue and Telegram editorial bot. No public Telegram SEND was performed in that acceptance.

## Server companion
Private `RomanZavadaM/INFO` PR #18 corresponds to the Desktop server-side work used by LIVE 2040 and still needs repository reconciliation.

## Next
1. Reconcile INFO PR #18.
2. Define signing/notarization/update policy.
3. Deliberate publish test only with owner approval.
4. Continue preview/platform adapters.
