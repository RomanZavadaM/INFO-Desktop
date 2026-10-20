# INFO Desktop Registration Protocol v1

**Identifier:** `info-desktop-registration/1`  
**Status:** deployed/accepted for current INFO Desktop

## Користувацький сценарій
```text
встановити INFO Desktop
→ імпортувати registration file
→ активувати пристрій
→ працювати
```

Користувачу не потрібно вручну встановлювати окремий сертифікат або вводити постійний секрет у UI.

## Registration file
Містить server, port, registration_id, одноразовий registration_token, expiry та public INFO Desktop CA certificate.

## Activation
1. Desktop читає файл.
2. Одноразовий token використовується для enrollment.
3. Сервер видає `device_id` + high-entropy `device_secret`.
4. `device_secret` зберігається тільки в OS secure credential store.
5. Одноразовий token видаляється локально.
6. Сервер зберігає hash reusable secret.

## Scopes
Поточна схема підтримує окремі права читання та редакторські scopes, включно з `queue.read`, `item.read`, `media.preview`, `item.write`, `item.create`, `publish.write`.

## Rotation / revoke
Сервер може ротувати credential з коротким grace period і відкликати один конкретний device без впливу на інші.

## TLS trust
Registration містить публічний CA, а не production secret. Native client використовує цей CA тільки для INFO transport і не встановлює його глобально в ОС.

## Production boundary
Hostname/port/firewall/rate-limit не хардкодяться в публічному Desktop repo. Поточний gateway INFO лишається VPN-only.
