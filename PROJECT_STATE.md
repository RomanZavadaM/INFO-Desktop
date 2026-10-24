## 2026-10-24 — Visual editor merged (unreleased)
- PR #12 merged into `main` as `8daa226f209258af36facd45d91df63bf315343d` after all seven PR checks succeeded (frontend, docs, Rust, Windows, Linux, macOS arm64/x86_64).
- Rich-text editing, HTML source switch, paste-as-plain-text, selection-aware formatting, recursive HTML sanitization, seven UI localizations. Server draft HTML protocol unchanged.
- **Stable download remains v0.1.3**; code merged after release. Real Windows WebView2 acceptance of caret, link and quote behavior, paste, save/reopen and switching materials remains before new release. Do not claim this testing happened.
- INFO server LIVE remains `20261024-1653`, untouched.

## Checkpoint 2026-10-24 — Desktop v0.1.3 published
- PR #11 merged into main as `ce75608701f3573b03d1ad130b1eaf24bc07d021`; release workflow run `40104833679` finished success.
- GitHub Release: https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.3 (8 application assets + 4 SHA256SUMS).
- Seven UI languages and seven README variants updated. Server INFO LIVE `20261024-1653` confirmed by operator; full Desktop XLSX acceptance and public Telegram publish test still pending.
- Older below checkpoints are historical, not current status.


## Checkpoint 2026-10-24 — XLSX safety
- Desktop PR #10 merged into `main` as `205a0c087777843a5bfad3b7a80e94bb9aba800b`; INFO server PR #30 merged as `7ab849cf77a053bfbcd42d8bbf2d1d04ee40e4dd`.
- Desktop CI: frontend, rust, docs, Windows, Linux, macOS ARM64 and macOS Intel all SUCCESS (Intel DMG first attempt failed with runner `hdiutil ... Resource busy`; rerun SUCCESS).
- New: staged XLSX digest, matching probe result, upload invalidates stale probe, guarded apply checks digest, caution before applying without a completed matching probe.
- Stable downloadable version is still **0.1.2**. Code in `main` is an unreleased 0.1.3 candidate; do NOT pretend current installer includes the XLSX panel.
- Server LIVE is not upgraded by these merges; last confirmed installed INFO version is `20261022-2010`. The `20261022-2250` server package was prepared but not confirmed installed. New Desktop settings/probe/themes require compatible server deployment and real end-to-end acceptance.
- Remaining: safe rollback provenance/parallel edits, full topic/condition editor, UI i18n audit for new warning text, full live acceptance; no release, production SEND or VPN/firewall changes happened in this step.

# PROJECT_STATE — INFO Desktop

## State 2026-10-22 (audit)
- Перевірено фактичний GitHub: `main` = `095bd58` (останній CI `INFO Desktop checks` - success); Release `v0.1.1` Latest, 12 файлів (Windows EXE/MSI, Linux AppImage/DEB, macOS ARM64 і Intel DMG + Portable ZIP, 4 файли SHA256).
- Контрольні суми: вміст Windows EXE/MSI і macOS ARM64 DMG/ZIP збігається з опублікованими `SHA256SUMS-*.txt`.
- Знахідка 1: у `SHA256SUMS-*.txt` імена файлів з пропусками (`INFO Desktop_0.1.1_...`), а файли в Release мають крапки (`INFO.Desktop_0.1.1_...`): перевірка `sha256sum -c` за назвою не спрацює, звіряти треба за хешем. Виправити в наступному release (імена без пропусків).
- Знахідка 2: локальний інсталятор з артефакту CI від 22.10 10:16 (EXE 3 296 677 байт) - не збірка Release (EXE 3 297 393 байт); для встановлення брати файл із Release.
- Знахідка 3: усі 7 мов документації мають по 10 файлів, ключі `src/locales` синхронні (перевіряє CI); старих тверджень (`0.1-r1`, read-only prototype, release pending) у поточних документах немає.
- Серверна частина: INFO PR #18 злито в `main` приватного INFO 21.10 (`fed3ee98`). LIVE INFO тепер `20261022-1730` (було `20261020-2040`); acceptance Desktop на 1730 ще не проводився.

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

## Post-release audit
On 2026-10-20 the public repository was re-audited after `v0.1.1` publication. Old `0.1-r1`/GET-only assumptions were found in project rules, connection architecture and the Rust User-Agent source and were corrected in `main`. The already published `v0.1.1` release is not rewritten; the next published Desktop build must receive a new version.

## Server companion
Private `RomanZavadaM/INFO` PR #18 corresponds to the Desktop server-side work used by LIVE 2040 and still needs repository reconciliation.

## Next
1. Reconcile INFO PR #18.
2. Define signing/notarization/update policy.
3. Deliberate publish test only with owner approval.
4. Continue preview/platform adapters.
