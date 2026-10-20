# PROJECT_STATE — INFO Desktop

## Стан 2026-10-20

- Repository: `RomanZavadaM/INFO-Desktop`
- Visibility: public
- Main initial commit: `146b69c3cd8e7d80b4ef1b8aed3dd6688b543baf`
- Active development branch: `prototype/read-only-client`
- Product version: `0.1.0-dev`
- Stage: architecture/bootstrap
- Release: none
- Stable: none

## Прийняті рішення

- Desktop — окремий продукт.
- Серверний INFO і Telegram не чіпаємо під час bootstrap.
- Перший прототип працює тільки read-only.
- Перший acceptance — на реальній черзі, текстах і медіа.
- Взаємодія універсальна через версійований protocol/capabilities.
- Platform/release/i18n model орієнтується на Taxo.
- License model — Taxo proprietary / All rights reserved.

## Що вже є

- public repository;
- development branch;
- LICENSE.md;
- COPYRIGHT.md;
- THIRD_PARTY_NOTICES.md.

## Наступна ціль

Зафіксувати protocol v1 read-only, після чого створити мінімальний desktop-клієнт, який підключається до реального сумісного сервера та показує живу редакційну чергу без будь-яких змін серверного стану.
