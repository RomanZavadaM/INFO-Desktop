# INFO Desktop

[Українська](README.md) · [English](docs/i18n/README.en.md) · [Deutsch](docs/i18n/README.de.md) · [Español](docs/i18n/README.es.md) · [Français](docs/i18n/README.fr.md) · [한국어](docs/i18n/README.ko.md) · [日本語](docs/i18n/README.ja.md)

**INFO Desktop — універсальне настільне робоче місце редактора й адміністратора для INFO-сумісних серверів.**

Поточний етап: **0.1-r1 / read-only prototype**.

Клієнт не прив'язаний до одного newsroom. Він підключається через версійований протокол і показує реальні редакційні дані. У першій версії native transport фізично GET-only.

Чинний Telegram-інтерфейс INFO залишається паралельним працездатним клієнтом.

## Technology
Tauri 2 + TypeScript + React.

Platforms:
- Windows 10/11 x64;
- macOS ARM64;
- macOS Intel x86_64;
- Linux x64.

Windows 7 не входить в обов'язкову матрицю нового продукту.

## Protocol
[INFO Desktop Protocol](docs/protocol/INFO_DESKTOP_PROTOCOL.md)

## Development
Start with [START_HERE.md](START_HERE.md), then `PROJECT_RULES.md`, `PROJECT_STATE.md`, `WORKLOG.md`.

## License
**Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved.** Proprietary software.

[LICENSE.md](LICENSE.md) · [COPYRIGHT.md](COPYRIGHT.md) · [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md)
