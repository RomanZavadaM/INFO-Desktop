# INFO Desktop Protocol

**Identifier:** `info-desktop-protocol/1`  
**Status:** accepted v1 surface for current INFO Desktop

Протокол є семантичним і не залежить від конкретної СУБД.

## Base path
`/api/info-desktop/v1`

## Read endpoints
- `GET /handshake`
- `GET /status`
- `GET /channels`
- `GET /queue`
- `GET /queue/{id}`
- `GET /queue/{id}/why?channel=&scope=` - пояснення рішення каналів для події (scope `item.read`, лише читання): `channels` (код, режим, ok, причина) і `rules` (дерево умов XLSX для вибраного каналу)
- authenticated media preview descriptors/routes

## Scoped write endpoints
Поточний клієнт може використовувати серверні операції:
- save/edit item;
- skip;
- defer;
- manual create;
- publish.

Сервер визначає доступ через scopes, зокрема `item.write`, `item.create`, `publish.write`. Клієнт не має права напряму змінювати БД.

## Налаштування XLSX (scopes `settings.read`, `settings.write`)
- `GET /settings/status` - чи є перевірений файл цього пристрою і чи можна відкотити;
- `GET /settings/xlsx` - поточні налаштування: `{filename,size,base64}`;
- `POST /settings/xlsx/check` `{base64}` - перевірка файлу без застосування (помилка 400 `invalid_xlsx` з `detail`); файл запам'ятовується для цього пристрою;
- `POST /settings/xlsx/apply` - застосувати перевірений файл (409 `no_pending`, якщо файлу немає); попередній стан зберігається;
- `POST /settings/xlsx/rollback` - повернути стан перед останнім застосуванням (409 `no_rollback`).
Той самий формат, перевірки й відкат, що в боті та `info menu`.

## Handshake
Мінімально містить protocol id, instance/server name, server version, timezone, capabilities і languages. Клієнт відхиляє несумісний protocol id.

## Queue
Черга повертає **підготовлений редакційний стан**, а не запускає важкі routing/quality перерахунки на кожен клік. Підтримується bounded pagination і `next_cursor`.

## Item detail
Може містити title, draft_text, channel, state, bucket, quality metadata, eligible channels, provenance, source materials, translation state і media descriptors.

## Media
Сервер не відкриває локальні filesystem paths. Cached media подається через контрольований authenticated route. Desktop тримає preview у пам'яті.

## Authentication
Device credentials є секретами й не потрапляють у Git або release assets. Native client довіряє приватному INFO Desktop CA, отриманому через registration flow.

## Compatibility
Невідомі поля ігноруються; стабільні коди не перекладаються; display labels можуть локалізуватися.

## Audit
Write operations виконуються як серверні бізнес-операції та мають підлягати серверному audit.
