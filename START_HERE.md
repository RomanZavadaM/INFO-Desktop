# START_HERE — INFO Desktop

> Перша точка входу для кожної нової робочої сесії.

## Startup protocol

Перед змінами:
1. Прочитати цей файл.
2. Прочитати `PROJECT_RULES.md`.
3. Прочитати `PROJECT_STATE.md`.
4. Прочитати `WORKLOG.md`.
5. Перевірити фактичний GitHub: main SHA, активну гілку/PR, CI, releases.
6. Прочитати останні записи Issue #2 — `INFO Desktop — live development ledger`.
7. Продовжити з першого незавершеного `DOING/NEXT`.

GitHub має пріоритет над пам'яттю чату.

## Продукт

INFO Desktop — окремий універсальний desktop-клієнт для INFO-сумісних серверів.

Поточна гілка: `prototype/read-only-client`  
Поточний PR: #1 draft  
Поточний етап: `0.1-r1 — real read-only newsroom`

## Межі першого етапу

- Читати **реальні** дані сервера.
- Не змінювати серверний стан.
- Не мати прямого доступу до SQLite.
- Не використовувати SSH як робочий протокол.
- Не містити production-секретів.
- Не чіпати чинний Telegram-функціонал.
- Не змінювати private `RomanZavadaM/INFO`, поки конкретний read-only API не буде звірений і погоджений.

## Architecture checkpoint

- Tauri 2 + TypeScript + React.
- Required OS: Windows 10/11 x64, macOS ARM64, macOS Intel x86_64, Linux x64.
- Windows 7 is not required; possible separate legacy path only if needed.
- Native transport in 0.1 is GET-only.
- Significant changes to OS matrix, core stack, storage/security/protocol architecture must be discussed with the owner first.

## Перша acceptance-точка

Desktop підключається до реального INFO-compatible server та швидко показує:
- стан;
- канали;
- prepared pending queue;
- картку;
- ready draft;
- quality/bucket/issues;
- реальні source materials;
- реальні медіа.

Жодних write actions.

## Новий чат

> **Продовжуємо INFO Desktop. Відкрий у GitHub `START_HERE.md` і продовжуй строго за ним.**

Repository: `RomanZavadaM/INFO-Desktop`.
