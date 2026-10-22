# START_HERE — INFO Desktop

> Перша точка входу для кожної нової робочої сесії.

Перед змінами прочитати `PROJECT_RULES.md`, `PROJECT_STATE.md`, `WORKLOG.md`, перевірити фактичний GitHub: main, PR, CI, releases.

## Контрольний стан
- Repository: `RomanZavadaM/INFO-Desktop`
- Version: `0.1.1`
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

## Next
1. ~~Reconcile private INFO server PR #18~~ - зроблено 21.10 (INFO `main` `fed3ee98`); LIVE INFO тепер `20261022-1730`.
2. Acceptance Desktop 0.1.1 на INFO `20261022-1730` (підключення, черга, канали «Новини» і «Тривоги» як типи `stream`/`alert`).
3. Нові серверні функції для Desktop: «Чому» (пояснення рішення каналу) і «Що зміниться» (проба XLSX-файлу) - `docs/roadmap/ROADMAP.md`.
4. Політика signing/notarization/updater: чернетка `docs/decisions/SIGNING_UPDATES.md`, чекає рішення власника (витрати).
5. Deliberate publish acceptance лише з явним дозволом власника.
6. Telegram preview polish, далі TikTok/Facebook adapters.

## Новий чат
> **Продовжуємо INFO Desktop. Відкрий у GitHub `START_HERE.md` і продовжуй строго за ним.**
