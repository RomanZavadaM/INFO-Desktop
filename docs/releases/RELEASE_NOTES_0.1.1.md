# INFO Desktop 0.1.1

**Перший стабільно зафіксований публічний checkpoint INFO Desktop.**

Дата: 2026-10-20

## Що увійшло

- реальне підключення до INFO через `info-desktop-protocol/1`;
- одноразова реєстрація пристрою;
- зберігання reusable device secret тільки в OS secure storage;
- канали та повна pending-черга;
- картка матеріалу, джерела, переклади та cached-media preview;
- редагування заголовка, тексту й каналу;
- save / skip / defer;
- ручне створення нового матеріалу;
- Telegram-style preview;
- scoped publish action для авторизованих пристроїв;
- 7 мов інтерфейсу й багатомовна документація;
- збірки Windows 10/11 x64, macOS ARM64, macOS Intel x86_64 і Linux x64.

## Production acceptance

20.10.2026 INFO Desktop був підключений до LIVE INFO `20261020-2040`.

Реальний матеріал Львова було створено через Desktop і підтверджено в редакційній черзі та Telegram editorial bot. Таким чином перевірено шлях:

**Desktop → API → база → редакційна черга.**

Публічний Telegram SEND у цьому acceptance-тесті не виконувався.

## Безпека

- немає прямого доступу Desktop до SQLite;
- SSH не використовується як робочий протокол програми;
- production secrets не зберігаються у репозиторії;
- device secret не зберігається в UI/localStorage;
- права видаються сервером окремими scopes;
- поточний production gateway лишається VPN-only.

## Інсталяційні файли

GitHub Release містить платформні артефакти та SHA256 checksums, створені GitHub Actions із того самого source checkpoint.

## Мови документації

Українська — канонічна. Синхронні переклади: English, Deutsch, Español, Français, 한국어, 日本語.

## Далі

- deliberate publish acceptance — тільки з явним дозволом власника;
- signing/notarization/update policy;
- Telegram preview polish;
- TikTok і Facebook adapters.
