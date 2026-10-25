# INFO Desktop — реєстр релізів, архівів і резервних копій

Стан на 25.10.2026. GitHub є першочерговим джерелом фактичного стану релізів.

## Опубліковані Desktop-релізи
- [v0.1.4](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.4) — **поточний stable**. Release Actions [40128755256](https://github.com/RomanZavadaM/INFO-Desktop/actions/runs/40128755256) SUCCESS. 8 пакетів Windows/Linux/macOS + 4 SHA256SUMS. Візуальний редактор, Windows console fix і timestamp XLSX входять у цей дистрибутив.
- [v0.1.4-1](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.4-1) — історичний pre-release.
- [v0.1.3](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.3) — попередній stable.
- [v0.1.2](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.2) — попередній stable.
- [v0.1.1](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.1) — історичний stable.

## Пов'язаний сервер INFO — окремий проєкт
- Останній підтверджений LIVE: `20261024-1653`.
- Серверний відкат: `bash /root/backup/info-20261024-174402/rollback.sh` — **не запускати без окремої потреби й процедури**.
- Для Desktop Gateway оператор підтвердив backup `/root/backup/info-desktop-gateway-20261024-225022/nginx.conf`.
- XLSX check після виправлення HTTP 413 пройшов; налаштування XLSX не застосовувалися.

## Google Drive
25.10.2026 перевірено папку INFO Desktop. У «Документація» наявні контрольні документи «INFO Desktop — СТАН І ПРОДОВЖЕННЯ — 2026-10-24» та «INFO Desktop — АРХІВИ І БЕКАПИ — 2026-10-24». Вони мають бути синхронізовані з цим GitHub checkpoint після документаційного PR.

## Що не перевірено
- Наявність і SHA256 копій релізних пакетів на Storage Box та локальному ПК не інвентаризовано в цій сесії.
- Відновлення з серверного backup окремо не тестувалося.
- Не заявляти про повне резервування без перевірки вмісту й контрольних сум.

## Правила зберігання
1. GitHub Release з пакунками та SHA256 — дистрибутив застосунку, не резервна копія серверної БД.
2. Конфігурацію, SQLite, токени, журнали й приватні ключі не завантажувати до публічного GitHub.
3. Старі архіви не перезаписувати й не видаляти без перевірки відновлення та явного дозволу.
4. Після релізу звіряти `START_HERE.md`, `PROJECT_STATE.md`, `WORKLOG.md`, `docs/releases/RELEASE_INDEX.md`, цей реєстр і README всіма 7 мовами.
