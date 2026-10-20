# INFO Desktop

[Українська](README.md) · [English](docs/i18n/README.en.md) · [Deutsch](docs/i18n/README.de.md) · [Español](docs/i18n/README.es.md) · [Français](docs/i18n/README.fr.md) · [한국어](docs/i18n/README.ko.md) · [日本語](docs/i18n/README.ja.md)

![INFO Desktop checks](https://github.com/RomanZavadaM/INFO-Desktop/actions/workflows/ci.yml/badge.svg)

**INFO Desktop — окреме настільне робоче місце редактора для INFO-сумісних серверів.**

Поточна версія коду: **0.1.1**. Перший робочий клієнт уже злитий у `main` і перевірений на реальному INFO: підключення, реальна редакційна черга, картка матеріалу, медіа та ручне створення матеріалу працюють end-to-end.

> INFO Desktop не замінює Telegram-інтерфейс. Обидва клієнти працюють паралельно з тією самою редакційною системою.

## Що вже вміє

- підключення до INFO через версійований `info-desktop-protocol/1`;
- одноразова реєстрація пристрою;
- секрет пристрою зберігається в системному secure storage;
- TLS із приватним INFO CA без ручного встановлення сертифіката;
- список каналів і повна pending-черга з пагінацією;
- відкриття реального матеріалу, джерел, перекладів і cached-media preview;
- редагування заголовка, тексту та каналу;
- збереження, пропуск, відкладення;
- ручне створення нового матеріалу;
- Telegram-style preview перед публікацією;
- scoped publish action для авторизованих пристроїв;
- 7 мов інтерфейсу;
- Windows 10/11 x64, macOS ARM64, macOS Intel x86_64, Linux x64.

Windows 7 не входить в обов'язкову матрицю цього продукту.

## Реально перевірено

20.10.2026 INFO Desktop був підключений до LIVE INFO `20261020-2040`. Один реальний матеріал для Львова створено через Desktop і підтверджено в редакційній черзі/Telegram-боті. Це перевірило шлях **Desktop → API → база → редакційна черга**. Публічний Telegram SEND у цьому acceptance-тесті не виконувався.

## Безпека

- клієнт не має прямого доступу до SQLite;
- робочий протокол не використовує SSH;
- production-секрети не зберігаються в репозиторії;
- reusable device secret не зберігається в UI/localStorage;
- сервер видає scopes окремо для read/write/create/publish;
- Desktop gateway на поточному INFO залишається VPN-only;
- жоден код цього репозиторію не повинен самовільно змінювати VPN/firewall/SSH.

## Технології

Tauri 2 · TypeScript · React · Rust · reqwest/rustls.

## Збірка

Для frontend/checks:

```bash
npm ci
npm run check
npm run test:protocol
npm run build
```

Native build:

```bash
npm run tauri build
```

Детальніше: [docs/BUILD.md](docs/BUILD.md).

## Документація

- [START_HERE.md](START_HERE.md) — точка входу в розробку;
- [PROJECT_STATE.md](PROJECT_STATE.md) — фактичний стан;
- [USER_MANUAL.md](docs/guides/USER_MANUAL.md) — користувацьке керівництво;
- [SYSTEM_OVERVIEW.md](docs/SYSTEM_OVERVIEW.md) — огляд системи;
- [INFO Desktop Protocol](docs/protocol/INFO_DESKTOP_PROTOCOL.md) — серверний протокол;
- [Registration](docs/protocol/INFO_DESKTOP_REGISTRATION.md) — реєстрація пристроїв;
- [Release Index](docs/releases/RELEASE_INDEX.md) — контрольні релізні точки;
- [Roadmap](docs/roadmap/ROADMAP.md) — подальший розвиток.

## Статус релізу

`0.1.1` — перший accepted checkpoint у `main`. GitHub Release з інсталяційними файлами ще має бути опублікований окремо після фінального release-build/signing рішення; source checkpoint уже стабільно зафіксований у main.

## Ліцензія

**Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved.** Proprietary software.

[LICENSE.md](LICENSE.md) · [COPYRIGHT.md](COPYRIGHT.md) · [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md)
