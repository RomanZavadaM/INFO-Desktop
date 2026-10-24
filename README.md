# INFO Desktop

**Архіви й резервні копії:** [реєстр підтверджених релізів та стану резервування](docs/ARCHIVES_AND_BACKUPS.md). Візуальний редактор після 0.1.3 — у `main`, але ще не в завантажуваних інсталяторах.


> ⬇️ **Завантажити INFO Desktop 0.1.3**: [**Windows EXE / MSI · Linux AppImage / DEB · macOS Intel / Apple Silicon**](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.3)

**Мови / Languages / Sprachen / Idiomas / Langues / 언어 / 言語**

[Українська](README.md) · [English](docs/i18n/README.en.md) · [Deutsch](docs/i18n/README.de.md) · [Español](docs/i18n/README.es.md) · [Français](docs/i18n/README.fr.md) · [한국어](docs/i18n/README.ko.md) · [日本語](docs/i18n/README.ja.md)

## Можливості версії 0.1.3

**Ці функції увійшли до опублікованої версії 0.1.3:**
- вкладка **Налаштування**: експорт XLSX, перевірка без застосування, підтверджене застосування та відкат;
- проба **«Що зміниться за 48 год»** на серверній копії бази;
- вкладка **Теми**: редагування ключових слів і перевірка на прикладі тексту;
- прив'язка звіту проби до конкретного XLSX, захист від застосування зміненого файла.

**Важливо:** ручні дії редактора (зберегти, пропустити, відкласти, опублікувати) йдуть **Desktop → API INFO**, а не через Excel. XLSX використовується тільки для налаштувань автоматичних правил. Для функцій потрібен сумісний сервер; оператор підтвердив встановлення INFO LIVE `20261024-1653` 24.10.2026. Повне приймання XLSX із встановленого Desktop ще не проведено. Публічну тестову публікацію не виконували.



> Українська — основна й канонічна мова проєкту. Інтерфейс і публічна документація підтримуються сімома мовами за тією самою моделлю, що й у Taxo.

![INFO Desktop checks](https://github.com/RomanZavadaM/INFO-Desktop/actions/workflows/ci.yml/badge.svg)

**INFO Desktop — окреме настільне робоче місце редактора для INFO-сумісних серверів.**

Поточна стабільна версія: **0.1.3**. Перший робочий клієнт злитий у `main`, пройшов production-connected acceptance та має опублікований GitHub Release.

## Завантаження 0.1.3

**[GitHub Release v0.1.3](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.3)**

| Платформа | Файли |
|---|---|
| Windows 10/11 x64 | EXE installer, MSI |
| macOS Apple Silicon | DMG, Portable ZIP |
| macOS Intel x86_64 | DMG, Portable ZIP |
| Linux x64 | AppImage, DEB |

Для кожної платформи опубліковано окремий `SHA256SUMS-*.txt`.

> Поточні збірки ще не мають завершеної signing/notarization/update policy. Це наступний релізний етап.

## Що вже вміє

- підключення до INFO через `info-desktop-protocol/1`;
- одноразова реєстрація пристрою;
- reusable device secret тільки в OS secure storage;
- TLS із приватним INFO CA;
- канали і повна pending-черга;
- картка матеріалу, джерела, переклади, cached-media preview;
- редагування заголовка, тексту та каналу;
- save / skip / defer;
- ручне створення матеріалу;
- Telegram-style preview;
- scoped publish action;
- 7 мов інтерфейсу;
- Windows 10/11 x64, macOS ARM64, macOS Intel x86_64, Linux x64.

Windows 7 не входить до обов'язкової матриці.

## Реально перевірено

20.10.2026 INFO Desktop був підключений до LIVE INFO `20261020-2040`. Реальний матеріал Львова створено через Desktop і підтверджено в редакційній черзі та Telegram editorial bot. Це перевірило шлях **Desktop → API → база → редакційна черга**. Публічний Telegram SEND у цьому acceptance-тесті не виконувався.

## Безпека

- немає прямого доступу до SQLite;
- SSH не використовується як application protocol;
- production secrets не лежать у репозиторії;
- device secret не зберігається в UI/localStorage;
- сервер видає окремі read/write/create/publish scopes;
- production gateway INFO залишається VPN-only.

## Документація

- [Документація](docs/README.md)
- [START_HERE](START_HERE.md)
- [Стан проєкту](PROJECT_STATE.md)
- [Керівництво користувача](docs/guides/USER_MANUAL.md)
- [Протокол](docs/protocol/INFO_DESKTOP_PROTOCOL.md)
- [Реєстрація](docs/protocol/INFO_DESKTOP_REGISTRATION.md)
- [Індекс релізів](docs/releases/RELEASE_INDEX.md)
- [Roadmap](docs/roadmap/ROADMAP.md)

## Ліцензія

**Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved. Proprietary software.**

[LICENSE.md](LICENSE.md) · [COPYRIGHT.md](COPYRIGHT.md) · [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md)
