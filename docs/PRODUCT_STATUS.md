# INFO Desktop — стан продукту

## Поточний стан — 25.10.2026
- Версія: **0.1.4**
- Default branch: `main`
- GitHub stable Release: **[v0.1.4](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.4)**
- Release workflow: **40128755256 — SUCCESS**
- Пакети: Windows EXE/MSI, Linux AppImage/DEB, macOS ARM64 DMG/Portable ZIP, macOS Intel x86_64 DMG/Portable ZIP + 4 SHA256SUMS.
- Код версії узгоджений у `VERSION.txt`, `package.json`, `src-tauri/Cargo.toml` і `src-tauri/tauri.conf.json`.

## Реалізовано
- Tauri 2 + TypeScript + React + Rust;
- 7 мов UI і публічної документації;
- захищена реєстрація пристрою та credential storage;
- канали, повна pending-черга, картка матеріалу, джерела й cached media;
- створення і редагування матеріалів;
- візуальний rich-text редактор + окремий HTML-режим;
- save / skip / defer та scoped publish action;
- XLSX export/check/probe/apply/rollback із digest-захистом;
- редактор тем/ключових слів;
- timestamp у назві експортованого XLSX;
- release-build Windows без зайвого консольного вікна.

## Фактично перевірено
- 20.10.2026 production-connected acceptance підтвердив шлях Desktop → API → база → редакційна черга на реальному матеріалі; публічний Telegram SEND не виконувався.
- 24.10.2026 після окремого виправлення Desktop Gateway незмінений XLSX успішно пройшов перевірку; **налаштування не застосовувалися**.
- INFO server є окремим проєктом; останній підтверджений LIVE: `20261024-1653`.

## Ще не прийнято вручну
- Windows WebView2 UX stable 0.1.4: форматування, цитати, посилання, вставлення, HTML-режим, save/reopen;
- відсутність постійного консольного вікна на встановленому stable 0.1.4;
- timestamp у фактично завантаженому XLSX зі stable 0.1.4;
- швидкість повної черги та переходів між матеріалами.

## Наступний продуктовий етап
1. Пришвидшити повну редакційну чергу.
2. Додати пошук, фільтрацію й швидке перемикання.
3. Додати «Зберегти й перейти до наступного».
4. Безпечні масові редакційні дії з підтвердженнями.
5. Захист від втрати незбережених змін.
6. Поліпшити теми/ключові слова та медіа.
7. Перевірити універсальність для інших INFO-сумісних серверів.

Facebook, TikTok та інші платформи мають окремий план; не реалізовувати їх із застарілих локальних нотаток Desktop.
