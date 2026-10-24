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

# INFO Desktop — стан продукту

## Поточний стан
- Версія: **0.1.2**
- Default branch: `main`
- PR #1: merged
- Production-connected acceptance: пройдено
- GitHub Release: **[v0.1.2](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.2) published**
- Release assets: Windows x64 / Linux x64 / macOS ARM64 / macOS Intel x86_64 + SHA256

## Реалізовано
- Tauri 2 + TypeScript + React + Rust;
- 7 мов;
- one-file registration;
- OS secure credential storage;
- full pending queue;
- item/source/media;
- editing;
- save / skip / defer;
- manual create;
- Telegram-style preview;
- scoped publish action.

## Production acceptance
На LIVE INFO `20261020-2040` реально створено матеріал Львова через Desktop. Він з'явився в редакційній черзі та Telegram editorial bot. Публічний Telegram SEND у цьому тесті не виконувався.

## Налаштування XLSX у Desktop (злито в main, у релізі 0.1.3)
Експорт, перевірка файлу, застосування й відкат; проба «що зміниться» (службою сервера); вкладка «Теми» (ключові слова за темами з перевіркою на тексті). Далі: додавання тем, групи слів і умови.

## Сервер
Реліз 0.1.2 прийнято на LIVE INFO `20261020-2040`; поточний LIVE INFO - `20261022-1730` (acceptance на ньому ще не проводився).

## Ще не завершено
- signing/notarization/update policy;
- deliberate publish acceptance;
- TikTok та Facebook adapters;
- подальше UI/UX шліфування.
