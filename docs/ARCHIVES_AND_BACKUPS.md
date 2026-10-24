# INFO Desktop — реєстр релізів, архівів і резервних копій

Стан на 24.10.2026. Розділяємо перевірені GitHub-артефакти, свідчення оператора LIVE та зовнішні копії, які не були перевірені.

## Опубліковані Desktop-релізи
- [v0.1.3](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.3) — стабільний реліз. GitHub Actions [40104833679](https://github.com/RomanZavadaM/INFO-Desktop/actions/runs/40104833679) SUCCESS. 8 пакетів Windows (EXE/MSI), Linux (AppImage/DEB), macOS ARM та Intel (DMG/ZIP), а також 4 файли SHA256.
- [v0.1.2](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.2) — попередній стабільний реліз.
- [v0.1.1](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.1) — історичний реліз.
- Візуальний редактор із PR #12 **злитий у main після 0.1.3**, не входить до завантажуваних інсталяторів.

## Пов'язаний сервер INFO (окремий репозиторій)
- `info-20261024-1653.tar.gz` зібрано через [GitHub Actions](https://github.com/RomanZavadaM/INFO/actions/runs/40100124030), регресія [SUCCESS](https://github.com/RomanZavadaM/INFO/actions/runs/40100123957).
- Оператор підтвердив успішне встановлення LIVE 24.10.2026 17:44, версія `20261024-1653`.
- Відкат на сервері: `bash /root/backup/info-20261024-174402/rollback.sh` — **не запускати без потреби**.
- `info-20261022-2250.tar.gz` — старіший пакет, не актуальний LIVE.

## Що не перевірено
- Копії файлів на Storage Box, Google Drive та локальному ПК — **не підтверджені**. Не заявляти про синхронізацію.
- Відновлення з серверного backup окремо не тестувалося.
- Вміст та контрольні суми зовнішніх копій потребують інвентаризації без видалення файлів.

## Правила зберігання
1. GitHub Release з пакунками та SHA256 — дистрибутив застосунку, **не** резервна копія серверної БД.
2. Конфігурацію, SQLite, токени, журнали й приватні ключі не завантажувати до публічного GitHub.
3. Старі архіви не перезаписувати. Не видаляти бекапи без перевірки відновлення та явного дозволу.
4. Після релізу звіряти `START_HERE.md`, `PROJECT_STATE.md`, `WORKLOG.md`, `docs/releases/RELEASE_INDEX.md` і сім мов README.
