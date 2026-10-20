# INFO Desktop — стан продукту

## Поточний стан
- Версія: **0.1.1**
- Default branch: `main`
- PR #1: merged
- Accepted main checkpoint: `852ee086769f2ce02246611d8772dfbb6ddc423d`
- Реальне production-connected acceptance: пройдено

## Реалізовано
- Tauri 2 + TypeScript + React + Rust;
- 7 мов інтерфейсу;
- Windows/macOS/Linux build matrix;
- одноразова реєстрація пристрою;
- OS secure credential storage;
- повна pending-черга з пагінацією;
- картка матеріалу, джерела та cached media preview;
- редагування заголовка, тексту й каналу;
- save / skip / defer;
- manual create;
- Telegram-style preview;
- scoped publish action.

## Production acceptance
На LIVE INFO `20261020-2040` реально створено матеріал Львова через Desktop. Він з'явився в редакційній черзі та Telegram editorial bot. Публічний Telegram SEND у цьому тесті не виконувався.

## Ще не завершено
- публічний GitHub Release з інсталяторами;
- signing/notarization/update policy;
- окремий deliberate publish acceptance;
- TikTok та Facebook adapters;
- подальше UI/UX шліфування.
