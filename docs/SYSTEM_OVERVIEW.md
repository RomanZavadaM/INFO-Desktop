# INFO Desktop — огляд системи

INFO Desktop — окреме настільне робоче місце редактора для INFO-сумісних серверів.

## Клієнт
Відповідає за інтерфейс, локалізацію, навігацію по черзі, картку матеріалу, редагування, попередній перегляд, медіа та дозволені редакторські дії.

## Сервер
Відповідає за дані, права, routing, quality, translations, publication, validation, audit та збереження стану.

## Принцип
Desktop — **не другий сервер**. Він не має прямого доступу до SQLite і не дублює серверну бізнес-логіку.

## Паралельні клієнти
Одночасно можуть працювати INFO Desktop, Telegram bot, web UI та інші INFO-compatible клієнти.

## Транспорт
- версійований `info-desktop-protocol/1`;
- TLS;
- автентифіковані GET + scoped POST;
- reusable device secret зберігається тільки в OS secure storage;
- поточний production gateway залишається VPN-only.

## Перевірений шлях
20.10.2026 на LIVE INFO `20261020-2040` підтверджено Desktop → API → база → редакційна черга → Telegram editorial bot для ручного створення матеріалу.
