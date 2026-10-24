## АКТУАЛЬНИЙ СТАН — 24.10.2026
- Стабільний Desktop **v0.1.3**: https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.3 (8 інсталяторів/пакетів та 4 SHA256).
- Візуальний редактор PR #12 злито в main (8daa226f209258af36facd45d91df63bf315343d), але він **не входить** у v0.1.3.
- INFO server LIVE **20261024-1653** за журналом оператора; відкат `/root/backup/info-20261024-174402/rollback.sh`.
- Відкрито: приймання Windows WebView2, XLSX end-to-end; не проводити Telegram SEND чи серверних змін без окремої вказівки.
- Нижчі записи — історичні, навіть якщо містять застарілі твердження про поточну версію.
- Архівний реєстр: [docs/ARCHIVES_AND_BACKUPS.md](docs/ARCHIVES_AND_BACKUPS.md).

## Checkpoint 2026-10-24 — Desktop v0.1.3 published
- PR #11 merged into main as `ce75608701f3573b03d1ad130b1eaf24bc07d021`; release workflow run `40104833679` finished success.
- GitHub Release: https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.3 (8 application assets + 4 SHA256SUMS).
- Seven UI languages and seven README variants updated. Server INFO LIVE `20261024-1653` confirmed by operator; full Desktop XLSX acceptance and public Telegram publish test still pending.
- Older below checkpoints are historical, not current status.

# START_HERE — INFO Desktop

## Checkpoint 2026-10-24 — XLSX safety
- Desktop PR #10 merged into `main` as `205a0c087777843a5bfad3b7a80e94bb9aba800b`; INFO server PR #30 merged as `7ab849cf77a053bfbcd42d8bbf2d1d04ee40e4dd`.
- Desktop CI: frontend, rust, docs, Windows, Linux, macOS ARM64 and macOS Intel all SUCCESS (Intel DMG first attempt failed with runner `hdiutil ... Resource busy`; rerun SUCCESS).
- New: staged XLSX digest, matching probe result, upload invalidates stale probe, guarded apply checks digest, caution before applying without a completed matching probe.
- Stable downloadable version is still **0.1.2**. Code in `main` is an unreleased 0.1.3 candidate; do NOT pretend current installer includes the XLSX panel.
- Server LIVE is not upgraded by these merges; last confirmed installed INFO version is `20261022-2010`. The `20261022-2250` server package was prepared but not confirmed installed. New Desktop settings/probe/themes require compatible server deployment and real end-to-end acceptance.
- Remaining: safe rollback provenance/parallel edits, full topic/condition editor, UI i18n audit for new warning text, full live acceptance; no release, production SEND or VPN/firewall changes happened in this step.


> Перша точка входу для кожної нової робочої сесії.

Перед змінами прочитати `PROJECT_RULES.md`, `PROJECT_STATE.md`, `WORKLOG.md`, перевірити фактичний GitHub: main, PR, CI, releases.

## Контрольний стан
- Repository: `RomanZavadaM/INFO-Desktop`
- Version: `0.1.2`, GitHub Release v0.1.2 опубліковано 2026-10-22 (Windows/Linux/macOS ARM64/Intel + SHA256; відомо: Windows-файл сум має CRLF)
- PR #1 merged
- Production acceptance against INFO `20261020-2040`: passed
- Manual create: verified end-to-end
- GitHub Release: **[v0.1.1](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.1) published**
- Windows/Linux/macOS Intel/macOS ARM assets + SHA256: published

## Межі
- no direct SQLite;
- no SSH application protocol;
- no production secrets;
- Telegram remains parallel;
- do not alter production VPN/network/firewall/SSH.

## Post-release audit 2026-10-20
- public README/release/assets/version files checked;
- stale `0.1-r1` revision policy removed from project rules;
- connection architecture updated from obsolete GET-only wording to current scoped writes;
- Desktop User-Agent source aligned to `0.1.1`;
- published `v0.1.1` remains immutable; `main` now contains post-release audit corrections and must use a new version for the next published release.

## Аудит 2026-10-22
`main` = `095bd58`, Release v0.1.1 цілий, контрольні суми збігаються за хешем; знайдено розбіжність імен у `SHA256SUMS-*.txt` (пропуски проти крапок) - виправити в наступному release. Деталі: `PROJECT_STATE.md`.

## Оновлення 2026-10-22 (вечір)
Кнопка «Чому» (PR #4), політика підпису (чернетка), аудит (PR #3 злито). Доступ без VPN: рішення прийнято, розгортання шлюзу - окремий крок. Запрошення Desktop - `info menu` на сервері. Деталі: `WORKLOG.md`.

## Оновлення 2026-10-22 (ніч)
Злито в main: вкладка «Налаштування» (XLSX: експорт, перевірка, застосування, відкат; PR #7) разом із серверною частиною INFO (PR #28, схема 32, права `settings.read/write`). Опублікований реліз - 0.1.2; наступний 0.1.3 з вкладкою «Налаштування». Далі: проба з Desktop окремою службою, редактор тем.

## Next
1. ~~Reconcile private INFO server PR #18~~ - зроблено 21.10 (INFO `main` `fed3ee98`); LIVE INFO тепер `20261022-1730`.
2. Acceptance Desktop 0.1.1 на INFO `20261022-1730` (підключення, черга, канали «Новини» і «Тривоги» як типи `stream`/`alert`).
3. Нові серверні функції для Desktop: «Чому» (пояснення рішення каналу) і «Що зміниться» (проба XLSX-файлу) - `docs/roadmap/ROADMAP.md`.
4. Політика signing/notarization/updater: чернетка `docs/decisions/SIGNING_UPDATES.md`, чекає рішення власника (витрати).
5. Deliberate publish acceptance лише з явним дозволом власника.
6. Telegram preview polish, далі TikTok/Facebook adapters.

## Новий чат
> **Продовжуємо INFO Desktop. Відкрий у GitHub `START_HERE.md` і продовжуй строго за ним.**
