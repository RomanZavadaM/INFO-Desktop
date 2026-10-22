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
