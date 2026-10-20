# INFO Desktop Protocol

**Working identifier:** `info-desktop-protocol/1`  
**Status:** draft / read-only subset

## Мета

Протокол відділяє універсальний Desktop-клієнт від конкретної реалізації INFO Server.

Клієнт не повинен знати:
- структуру SQLite;
- внутрішні Python modules;
- systemd;
- Telegram token/session;
- файлове розташування даних.

## 1. Handshake

Сервер має віддати:
- protocol identifier/version;
- server product/name;
- server instance label;
- server version;
- server time/timezone;
- supported capabilities;
- supported content/localization languages;
- authentication requirements.

Приклад семантики capabilities:
- `status.read`
- `channels.read`
- `queue.read`
- `item.read`
- `media.read`
- майбутні `item.publish`, `item.skip`, `item.move`, `admin.*`

Desktop показує тільки функції, які сервер реально оголосив.

## 2. Read-only v1

Перший subset повинен дозволяти:
- server status;
- channels;
- queue list з pagination/filter metadata;
- one item detail;
- source/provenance metadata;
- media descriptors/access;
- quality/editorial status;
- related/similar items, якщо підтримується;
- localization/translation states.

## 3. Queue

Queue item повинен мати стабільний server ID та мінімально:
- id;
- channel/current route;
- title;
- timestamps;
- state;
- quality/editorial fields;
- source summary;
- media presence;
- language/translation state.

Desktop не повинен виводити сенс із внутрішніх назв таблиць чи Python-станів.

## 4. Item detail

Повна картка може включати:
- original title/text;
- prepared/publication title/text;
- source/provenance;
- media;
- routing;
- quality;
- language variants;
- related items;
- server-side warnings.

## 5. Media

Media не вбудовується у protocol response великими binary blobs без потреби. Сервер віддає descriptor + authenticated fetch mechanism.

Клієнт повинен підтримувати реальні фото/відео/документи без припущення про їхній розмір.

## 6. Read-only guarantee

Для першого етапу клієнт не надсилає жодних mutation commands. Серверний read-only surface теж не повинен мати side effects.

## 7. Authentication

Protocol не визначає production secret у source code. Credentials мають видаватися конкретному installation/device/user і зберігатися у системному secure storage клієнта.

## 8. Compatibility

Server може підтримувати кілька protocol versions. Desktop має відмовитися від небезпечної несумісної операції, а не вгадувати.

## 9. Future write subset

Пізніше окремо описуються:
- publish;
- skip;
- postpone;
- move;
- edit;
- bulk actions;
- admin/configuration.

Усі write actions валідовуються сервером і аудіюються сервером.
