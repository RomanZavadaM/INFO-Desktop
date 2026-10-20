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
Поточний етап: `0.1-r1 — live editorial workstation`

## Поточні межі

- Читати **реальні** дані сервера.
- Дозволені лише автентифіковані редакторські зміни через scoped Desktop API: save/channel move/skip/defer/publish/manual create.
- Не мати прямого доступу до SQLite.
- Не використовувати SSH як робочий протокол.
- Не містити production-секретів.
- Не чіпати чинний Telegram-функціонал.
- Не змінювати private `RomanZavadaM/INFO`, поки конкретний read-only API не буде звірений і погоджений.

## Architecture checkpoint

- Tauri 2 + TypeScript + React.
- Required OS: Windows 10/11 x64, macOS ARM64, macOS Intel x86_64, Linux x64.
- Windows 7 is not required; possible separate legacy path only if needed.
- Native transport is authenticated GET + scoped POST.
- Significant changes to OS matrix, core stack, storage/security/protocol architecture must be discussed with the owner first.

## Acceptance

Desktop уже підключається до реального INFO-compatible server та показує:
- стан;
- канали;
- prepared pending queue;
- картку;
- ready draft;
- quality/bucket/issues;
- реальні source materials;
- реальні медіа.

Read-only acceptance вже пройдена. Наступна acceptance-точка — повний редакторський цикл з Windows: редагування → preview → зміна каналу → збереження → публікація/пропуск/відкладення/створення.

## Новий чат

> **Продовжуємо INFO Desktop. Відкрий у GitHub `START_HERE.md` і продовжуй строго за ним.**

Repository: `RomanZavadaM/INFO-Desktop`.
