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

