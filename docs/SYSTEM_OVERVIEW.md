# INFO Desktop — System Overview

INFO Desktop — cross-platform newsroom workstation для INFO-compatible servers.

## Основні ролі

### Desktop client
Відповідає за:
- швидке відображення великих черг;
- навігацію;
- фільтри/сортування;
- перегляд тексту й медіа;
- локалізований UI;
- пізніше — керовані редакторські та адміністративні команди.

### INFO-compatible server
Відповідає за:
- дані та їхню актуальність;
- права;
- бізнес-логіку;
- routing;
- translations;
- publication;
- server-side validation;
- audit.

## Головний принцип
Desktop — робоче місце, але не другий сервер.

## Паралельні клієнти
Той самий сервер може одночасно мати:
- INFO Desktop;
- Telegram bot;
- web UI;
- інші сумісні клієнти.

Усі вони повинні використовувати однакову серверну семантику, а не власні дублікати правил.
