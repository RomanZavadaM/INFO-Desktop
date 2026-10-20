# WORKLOG — INFO Desktop

## Active slice: 0.1.0-dev / real read-only newsroom

**Base:** initial public repository  
**Branch:** `prototype/read-only-client`  
**Status:** DOING

### DONE
- Створено окремий public repository.
- Створено робочу гілку.
- Прийнято модель окремого універсального продукту.
- Прийнято паралельне збереження Telegram INFO.
- Прийнято read-only first.
- Прийнято реальні production data for viewing, без тестової вигаданої черги.
- Прийнято Taxo platform/i18n/license model.
- Додано proprietary license/copyright/third-party skeleton.

### DOING
- Product/bootstrap documentation.
- Protocol v1 read-only contract.
- Localization skeleton.

### NEXT
1. Створити мінімальний app skeleton.
2. Додати UI localization resources.
3. Реалізувати connection profile без secrets у репозиторії.
4. Реалізувати protocol handshake/status/channels/queue/item/media reads.
5. Підключити до реального INFO-compatible server.
6. Перевірити UX на фактичній черзі ~100+ матеріалів.

### BLOCKED
- Серверний read-only endpoint ще не реалізований/не погоджений у `RomanZavadaM/INFO`; до конкретизації protocol v1 сервер не змінювати.
