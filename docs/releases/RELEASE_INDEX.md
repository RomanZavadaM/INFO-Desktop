## Контрольна точка — INFO Desktop v0.1.4, 24.10.2026
- Ціль релізу: stable v0.1.4, окремий GitHub Release з 8 пакетами Windows/Linux/macOS та 4 SHA256.
- Візуальний редактор PR #12, Windows console fix PR #13, timestamp XLSX export PR #15.
- XLSX HTTP 413: зміну серверного gateway з INFO PR #31 активовано оператором, nginx -t / restart gateway / info check успішні; користувач підтвердив «Перевірка пройдена» на незміненому XLSX. **Налаштування НЕ застосовувалися.** LIVE INFO `20261024-1653`, errors 0.
- Розвиток Facebook/TikTok та інших платформ визначає інший окремий проєкт; перечитати його оновлений план перед реалізацією, не дублювати платформний роутинг у Desktop.
- Заборонено тестові SEND/PIN Telegram, зміну LIVE без окремої процедури, видалення резервних копій.
- Для наступного чату: відкрити `START_HERE.md`, `PROJECT_STATE.md`, `WORKLOG.md`, звірити GitHub `main`, відкриті PR, GitHub Actions і GitHub Release, уточнити фактичну точку.
- Ручний Windows WebView2 UX тест редактора лишається окремим прийманням навіть після green CI.

# INFO Desktop — індекс релізів

## СТАН НА 24.10.2026
**Для тестування:** [INFO Desktop v0.1.4-1 (pre-release)](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.4-1). Перевірено опубліковані Windows EXE, MSI, SHA256; мультиплатформний релізний workflow ще завершується.
**Стабільна версія:** [v0.1.3](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.3). Вона не містить нового візуального редактора; його тестова версія — v0.1.4-1.

---
Старі записи нижче — історія. Заголовки "поточна версія" в них не є актуальними.


## Перевірений стан 24.10.2026
**Стабільний Desktop:** [v0.1.3](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.3); Windows, Linux та macOS, 12 файлів включно із SHA256. **Візуальний редактор із PR #12 — лише у main, не в опублікованому релізі.** Сервер INFO LIVE — `20261024-1653`. [Архівний реєстр](../ARCHIVES_AND_BACKUPS.md).


## 0.1.2 — поточний стабільний GitHub Release
**Дата:** 2026-10-22  
**Статус:** опублікований  
**Завантаження:** [v0.1.2](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.2)

Windows EXE/MSI, Linux AppImage/DEB, macOS Apple Silicon DMG/ZIP, macOS Intel DMG/ZIP та SHA256SUMS. Відповідні інсталятори **не містять** майбутніх вкладок XLSX і «Теми».

## 0.1.3 — стабільний GitHub Release, опубліковано 24.10.2026
**Завантаження:** [v0.1.3](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.3)
У `main` є вкладки XLSX/«Теми», проба 48 год, захист контрольної суми й переклади. Вимагає release, узгодженого з сервером, тестування роботи на справжньому Desktop та перевірки встановлення. INFO LIVE `20261024-1653` підтверджений оператором; Desktop acceptance ще відкритий.



## 0.1.1
**Дата:** 2026-10-20  
**Статус:** stable GitHub Release published / production-connected acceptance passed  
**Release:** [v0.1.1](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.1)

Опубліковані assets:
- Windows x64: EXE + MSI;
- Linux x64: AppImage + DEB;
- macOS ARM64: DMG + Portable ZIP;
- macOS Intel x86_64: DMG + Portable ZIP;
- окремі SHA256SUMS для кожної платформи.

Прийнято:
- реальне INFO connection;
- registration + secure credential storage;
- повну pending queue;
- item/source/media;
- editing і scoped write actions;
- manual create;
- 7 мов UI та документації.

Production acceptance: LIVE INFO `20261020-2040`; manual create підтверджений у редакційному боті; public Telegram SEND не виконувався.

Signing/notarization/update policy лишається наступним етапом.
