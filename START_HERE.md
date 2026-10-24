## Контрольна точка — INFO Desktop v0.1.4, 24.10.2026
- Ціль релізу: stable v0.1.4, окремий GitHub Release з 8 пакетами Windows/Linux/macOS та 4 SHA256.
- Візуальний редактор PR #12, Windows console fix PR #13, timestamp XLSX export PR #15.
- XLSX HTTP 413: зміну серверного gateway з INFO PR #31 активовано оператором, nginx -t / restart gateway / info check успішні; користувач підтвердив «Перевірка пройдена» на незміненому XLSX. **Налаштування НЕ застосовувалися.** LIVE INFO `20261024-1653`, errors 0.
- Розвиток Facebook/TikTok та інших платформ визначає інший окремий проєкт; перечитати його оновлений план перед реалізацією, не дублювати платформний роутинг у Desktop.
- Заборонено тестові SEND/PIN Telegram, зміну LIVE без окремої процедури, видалення резервних копій.
- Для наступного чату: відкрити `START_HERE.md`, `PROJECT_STATE.md`, `WORKLOG.md`, звірити GitHub `main`, відкриті PR, GitHub Actions і GitHub Release, уточнити фактичну точку.
- Ручний Windows WebView2 UX тест редактора лишається окремим прийманням навіть після green CI.

## АКТУАЛЬНЕ ОНОВЛЕННЯ — тестовий реліз 0.1.4-1, 24.10.2026
- **Опублікований Windows pre-release:** https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.4-1 (EXE, MSI, SHA256). Workflow `40124584839`: Windows build і publish SUCCESS.
- Пряма інсталяція Windows: https://github.com/RomanZavadaM/INFO-Desktop/releases/download/v0.1.4-1/INFO-Desktop_0.1.4-1_x64-setup.exe
- PR #14 злито в main як `2cd9f97d07354b357e6bcf88b3ba1186e8ab1fb5`: версії та артефакти синхронізовано, Linux RPM виключено з тестового пакування. Windows і Linux PR checks SUCCESS. Платформні macOS роботи можуть ще залишатися в черзі; не заявляти повний multiplatform release до перевірки.
- **Стабільна версія досі 0.1.3.** Тестова 0.1.4-1 включає візуальний редактор PR #12 та приховану консоль у Windows PR #13, але фактичний Windows UI acceptance на ПК користувача ще не проведено.
- INFO server LIVE `20261024-1653` без змін. Ніяких Telegram SEND не виконували.

## ФІНАЛЬНА КОНТРОЛЬНА ТОЧКА — 24.10.2026
- Усі відкриті на початку завершення PR перевірені; **PR #13 злито в `main`** squash-комітом `7aacfbb1253c3977504d906d09da4fbdc4f99700` після **7/7 SUCCESS** (Rust, frontend, docs, Windows, Linux, macOS arm64/x86_64).
- Причина зайвого вікна консолі: Windows release EXE не мав `windows_subsystem = "windows"`. Виправлено лише для non-debug build у `src-tauri/src/main.rs`. Це працюватиме **тільки в наступному зібраному інсталяторі**; встановлена v0.1.3 не оновлюється автоматично.
- Новий візуальний редактор PR #12 раніше злито, але він також лише у коді `main`; фактичний Windows WebView2 acceptance потрібен перед новим релізом.
- Опублікований stable реліз — **v0.1.3**, новий 0.1.4 НЕ випускався. INFO Server LIVE — `20261024-1653`, без змін.
- Зміни планів Facebook/TikTok та інших платформ живуть **в окремому проєкті**. Перед будь-якою реалізацією перечитати його актуальні джерела; Desktop roadmap не є остаточним міжпроєктним рішенням.
- Ніяких тестових Telegram SEND, серверних deploy, чистки резервних копій, переміщення Storage Box.
- Подальші кроки: Windows тест нового редактора, перевірка зникнення консолі з нової debug/release збірки, XLSX end-to-end без застосування, потім рішення щодо випуску наступної версії.

## КОНТРОЛЬНА ТОЧКА — 24.10.2026, завершення сесії
- Стабільний реліз **v0.1.3** лишається опублікованим; візуальний редактор PR #12 уже в `main` (merge `8daa226f209258af36facd45d91df63bf315343d`), але в 0.1.3 його немає.
- Прибирання зайвого консольного вікна Windows: PR #13 `fix/windows-console-hidden-release-20261024`, стандартний `windows_subsystem=windows` лише для release, розробницька консоль збережена. PR зливати лише після повних перевірок; до нового інсталятора виправлення не дійде.
- Плани інших платформ належать **окремому проєкту** та вже переглянуті там. Не вважати локальні пропозиції Desktop авторитетними; перед розвитком Facebook/TikTok перечитати актуальні рішення того проєкту, звірити ролі Desktop і сервера.
- Не змінювали INFO сервер LIVE `20261024-1653`, не виконували Telegram SEND і не видаляли backup/архіви.
- Перед новим Desktop release: Windows WebView2 практична перевірка редактора (виділення/посилання/цитати/вставлення/збереження); перегляд релізних збірок; документація і 7 мов; XLSX end-to-end тільки без apply.
- Документи: `docs/ARCHIVES_AND_BACKUPS.md`, `docs/releases/HANDOFF_20261024.md`, `docs/roadmap/ROADMAP.md`.

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
