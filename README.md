# INFO Desktop

[Українська](README.md) · [English](docs/i18n/README.en.md) · [Deutsch](docs/i18n/README.de.md) · [Español](docs/i18n/README.es.md) · [Français](docs/i18n/README.fr.md) · [한국어](docs/i18n/README.ko.md) · [日本語](docs/i18n/README.ja.md)

**INFO Desktop — універсальне настільне робоче місце редактора й адміністратора для INFO-сумісних серверів.**

Поточний етап: **0.1.0-dev / read-only prototype**.

## Ідея

INFO Desktop не замінює серверне ядро й не прив'язаний до одного конкретного newsroom. Він підключається до сервера через версійований протокол взаємодії та показує реальні редакційні дані: чергу, тексти, медіа, канали, статуси, оцінки й пов'язані матеріали.

На першому етапі клієнт **тільки читає**. Жодних publish/skip/move/config changes.

## Паралельна робота

Чинний Telegram-інтерфейс INFO залишається працездатним і незалежним. Розробка Desktop не є дозволом ламати, урізати або замінювати Telegram.

## Платформи

Модель збірок успадковує перевірений підхід Taxo:
- Windows 10/11 x64 — Setup + Portable;
- Windows 7 SP1 x64 — окрема Portable-збірка сумісності;
- macOS Apple Silicon / ARM64;
- macOS Intel x86_64.

Точна підтримка платформи оголошується тільки після фактичної збірки й перевірки.

## Протокол

Базовий контракт: [INFO Desktop Protocol](docs/protocol/INFO_DESKTOP_PROTOCOL.md).

## Розробка

Точка входу: [START_HERE.md](START_HERE.md). Далі читати `PROJECT_RULES.md`, `PROJECT_STATE.md` і `WORKLOG.md`.

## Ліцензія

**Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved.** INFO Desktop — proprietary software.

[LICENSE.md](LICENSE.md) · [COPYRIGHT.md](COPYRIGHT.md) · [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md)
