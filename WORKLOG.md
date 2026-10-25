## 25.10.2026 — наступний редакційний цикл
- Після документаційного узгодження v0.1.4 розпочато source **0.1.5 development**.
- Причина гальмування в Desktop знайдена в клієнті: після кожного Save/Publish/Skip/Defer виконувався повторний handshake + channels + повний paginated queue load.
- Виправлення: повне завантаження лишається для connect/manual refresh/create reconciliation; звичайні редакційні дії оновлюють локальну чергу і точково читають item detail.
- Додані пошук, bucket-фільтр, progressive queue display, save-and-next та захист незбережених змін.
- INFO server, nginx, VPN, Telegram і Storage Box не змінювалися.
- Manual Windows acceptance stable v0.1.4 не позначати завершеним без фактичної перевірки користувачем.

## ВИПУСК ПІДТВЕРДЖЕНО — 2026-10-24
- **GitHub stable release v0.1.4 опублікований**: https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.4 (prerelease=false, published 2026-10-24T20:25:03Z). Release Actions #40128755256: verify, Windows, Linux, macOS arm64/x86_64, publish — ВСЕ SUCCESS.
- 8 пакетів: Windows EXE/MSI; Linux AppImage/DEB; macOS arm64/x86_64 DMG і Portable ZIP. 4 SHA256SUMS. Прямий Windows EXE: https://github.com/RomanZavadaM/INFO-Desktop/releases/download/v0.1.4/INFO-Desktop_0.1.4_x64-setup.exe
- PR #15 (timestamp XLSX download) та PR #16 (stable release) злиті в main; перший пройшов 7/7 CI.
- XLSX HTTP 413 server gateway відновлено; незмінений XLSX перевірено користувачем, **НЕ застосовано**; INFO LIVE `20261024-1653` без змін, errors 0 після restart gateway.
- Залишається ручне приймання Windows WebView2 UX редактора та швидкості черги, пізніші можливості з окремого платформного проєкту потребують звірення його оновленого плану.

## Контрольна точка — INFO Desktop v0.1.4, 24.10.2026
- Ціль релізу: stable v0.1.4, окремий GitHub Release з 8 пакетами Windows/Linux/macOS та 4 SHA256.
- Візуальний редактор PR #12, Windows console fix PR #13, timestamp XLSX export PR #15.
- XLSX HTTP 413: зміну серверного gateway з INFO PR #31 активовано оператором, nginx -t / restart gateway / info check успішні; користувач підтвердив «Перевірка пройдена» на незміненому XLSX. **Налаштування НЕ застосовувалися.** LIVE INFO `20261024-1653`, errors 0.
- Розвиток Facebook/TikTok та інших платформ визначає інший окремий проєкт; перечитати його оновлений план перед реалізацією, не дублювати платформний роутинг у Desktop.
- Заборонено тестові SEND/PIN Telegram, зміну LIVE без окремої процедури, видалення резервних копій.
- Для наступного чату: відкрити `START_HERE.md`, `PROJECT_STATE.md`, `WORKLOG.md`, звірити GitHub `main`, відкриті PR, GitHub Actions і GitHub Release, уточнити фактичну точку.
- Ручний Windows WebView2 UX тест редактора лишається окремим прийманням навіть після green CI.

## ІСТОРІЯ — стан перед stable v0.1.4

> Записи нижче зберігаються як журнал робіт. Поточний стан визначає верхній блок про опублікований stable v0.1.4.
- Усі відкриті на початку завершення PR перевірені; **PR #13 злито в `main`** squash-комітом `7aacfbb1253c3977504d906d09da4fbdc4f99700` після **7/7 SUCCESS** (Rust, frontend, docs, Windows, Linux, macOS arm64/x86_64).
- Причина зайвого вікна консолі: Windows release EXE не мав `windows_subsystem = "windows"`. Виправлено лише для non-debug build у `src-tauri/src/main.rs`. Це працюватиме **тільки в наступному зібраному інсталяторі**; встановлена v0.1.3 не оновлюється автоматично.
- Новий візуальний редактор PR #12 раніше злито, але він також лише у коді `main`; фактичний Windows WebView2 acceptance потрібен перед новим релізом.
- Опублікований stable реліз — **v0.1.3**, новий 0.1.4 НЕ випускався. INFO Server LIVE — `20261024-1653`, без змін.
- Зміни планів Facebook/TikTok та інших платформ живуть **в окремому проєкті**. Перед будь-якою реалізацією перечитати його актуальні джерела; Desktop roadmap не є остаточним міжпроєктним рішенням.
- Ніяких тестових Telegram SEND, серверних deploy, чистки резервних копій, переміщення Storage Box.
- Подальші кроки: Windows тест нового редактора, перевірка зникнення консолі з нової debug/release збірки, XLSX end-to-end без застосування, потім рішення щодо випуску наступної версії.

## ІСТОРІЯ — стан до stable v0.1.4
- Стабільний Desktop **v0.1.3**: https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.3 (8 інсталяторів/пакетів та 4 SHA256).
- Візуальний редактор PR #12 злито в main (8daa226f209258af36facd45d91df63bf315343d), але він **не входить** у v0.1.3.
- INFO server LIVE **20261024-1653** за журналом оператора; відкат `/root/backup/info-20261024-174402/rollback.sh`.
- Відкрито: приймання Windows WebView2, XLSX end-to-end; не проводити Telegram SEND чи серверних змін без окремої вказівки.
- Нижчі записи — історичні, навіть якщо містять застарілі твердження про поточну версію.
- Архівний реєстр: [docs/ARCHIVES_AND_BACKUPS.md](docs/ARCHIVES_AND_BACKUPS.md).

## 2026-10-24 — візуальний редактор (після релізу 0.1.3)
- Реалізовано форматований ввід тексту без показу сирих HTML-тегів, HTML-source toggle, вставлення plain text, локалізовані керування сімома мовами.
- Нову версію застосунку ще не випущено; потрібні GitHub CI та фактичне UI-приймання на Windows. Сервер INFO не змінювали.
- Відоме обмеження: поведінку курсора, цитат та вставлення посилань у WebView2 перевірити вручну перед наступним релізом.


## Checkpoint 2026-10-24 — XLSX safety
- Desktop PR #10 merged into `main` as `205a0c087777843a5bfad3b7a80e94bb9aba800b`; INFO server PR #30 merged as `7ab849cf77a053bfbcd42d8bbf2d1d04ee40e4dd`.
- Desktop CI: frontend, rust, docs, Windows, Linux, macOS ARM64 and macOS Intel all SUCCESS (Intel DMG first attempt failed with runner `hdiutil ... Resource busy`; rerun SUCCESS).
- New: staged XLSX digest, matching probe result, upload invalidates stale probe, guarded apply checks digest, caution before applying without a completed matching probe.
- Stable downloadable version is still **0.1.2**. Code in `main` is an unreleased 0.1.3 candidate; do NOT pretend current installer includes the XLSX panel.
- Server LIVE is not upgraded by these merges; last confirmed installed INFO version is `20261022-2010`. The `20261022-2250` server package was prepared but not confirmed installed. New Desktop settings/probe/themes require compatible server deployment and real end-to-end acceptance.
- Remaining: safe rollback provenance/parallel edits, full topic/condition editor, UI i18n audit for new warning text, full live acceptance; no release, production SEND or VPN/firewall changes happened in this step.

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

### 2026-10-22 (вечір): «Чому в цьому каналі?» і доступ без VPN
- додано `GET /queue/{id}/why` у протокол (сервер INFO `main`, PR #25) і кнопку «Чому в цьому каналі?» у картці матеріалу (PR #4; CI: frontend, rust, docs, збірки Windows/Linux/macOS пройшли); 7 мов, протокол оновлено;
- сервер: прев'ю лише малі зображення (до 8 МБ), відео й великі файли не віддаються;
- рішення власника: доступ Desktop без VPN через окремий TLS-шлюз (механізм: одноразовий файл реєстрації, ключ пристрою, ротація, відкликання); дозвіл на відкриття одного порту (кандидат 47443/tcp) дано 22.10, саме відкриття - окремий крок розгортання;
- запрошення Desktop видає сервер у текстовому меню `info menu` (пункти: запросити Desktop, пристрої, XLSX, відкат, стан); функціонал Desktop живе в самому Desktop, не в Telegram-боті;
- наступний реліз Desktop: 0.1.2 разом із серверним випуском (`why`, обмеження прев'ю, меню).

### 2026-10-22 (ніч): підготовка 0.1.2
- версія 0.1.2 у `VERSION.txt`, `package.json`, `Cargo.toml`, `tauri.conf.json`, User-Agent; версія видна в шапці застосунку;
- новий workflow `release-0.1.2.yml` (замість `release-0.1.1.yml`, щоб не перезаписати опублікований v0.1.1): імена файлів без пропусків, `SHA256SUMS-*.txt` збігаються з іменами в Release;
- нотатки релізу й індекс 7 мовами; статуси й README оновлено до 0.1.2;
- реліз публікується workflow після злиття в `main`; після публікації перевірити assets і SHA256 за хешем та іменами.

### 2026-10-22 (ніч): реліз 0.1.2 опубліковано
- Release `v0.1.2` опубліковано workflow (verify, Windows, Linux, macOS ARM64 і Intel, release - success); 8 файлів + 4 `SHA256SUMS-*.txt`;
- перевірка за іменами: Linux і macOS (обидві архітектури) - OK; Windows - хеші збігаються, але `SHA256SUMS-Windows.txt` має CRLF (PowerShell), на Linux `sha256sum -c` потребує `tr -d ''`; виправити в наступному релізі (LF у Windows-кроці);
- файли тепер `INFO-Desktop_0.1.2_*` (без пропусків), імена в сумах збігаються з Release.

### 2026-10-22 (ніч): вкладка «Налаштування» (XLSX)
- кнопка «Налаштування» в шапці: завантажити поточний XLSX, вибрати файл (сервер перевіряє без застосування), застосувати з підтвердженням, відкотити; пояснення й кнопки 7 мовами;
- серверні запити `settings/*` і права `settings.read`/`settings.write` (INFO, схема 32);
- проба «що зміниться» поки на сервері (`info xlsx try-pending`); наступним кроком - запуск проби з Desktop окремою службою;
- далі: редактор тем (ключові слова за темами).

### 2026-10-22 (ніч): налаштування XLSX злито
- Desktop PR #7 і INFO PR #28 злито (CI: frontend, rust, docs, Windows/Linux/macOS і повна регресія INFO - success);
- інструкція користувача 7 мовами доповнена розділом про налаштування; дорожня карта: далі проба з Desktop і редактор тем;
- публічний реліз із цією вкладкою - 0.1.3 після встановлення серверного випуску зі схемою 32.

### 2026-10-22 (ніч): проба і теми
- кнопка проби «що зміниться за 48 год» у «Налаштуваннях» (запит ставиться в чергу, окрема служба сервера рахує, Desktop опитує стан і показує звіт);
- вкладка «Теми»: перелік тем зі словами, редагування, перевірка слів на тексті (`слово*`, `слово!`), збереження з підтвердженням; відкат через «Налаштування»; 7 мов, протокол і інструкція оновлені;
- серверна частина - INFO PR (проба, служба `info-xlsx-try`, `/themes`); потребує серверного випуску й Desktop 0.1.3.
- попередження про наслідки (рішення власника): червоний блок на «Налаштуваннях» і «Темах» та розгорнуті тексти підтверджень (7 мов): зміна правил живих каналів впливає на те, що публікується автоматично; відкат повертає налаштування, але не прибирає вже опубліковані пости; рекомендація спершу зробити пробу.


### 2026-10-24 — XLSX safety follow-up (review, not released)
- Desktop PR #10 together with INFO PR #30: checked XLSX plan digest, probe report tied to that plan, stale-report reset after uploading another file, warning before unprobed apply, guarded apply with expected digest.
- No production install, public publication, or stable version change.
- Remaining acceptance: CI for both repositories, install compatible server release, real Desktop import/probe/apply/rollback on a disposable test configuration. Rollback history/provenance and fuller theme/condition editor remain separate unfinished scope.
