# PROJECT_RULES — постійні правила INFO Desktop

1. **Окремий продукт.** Власні версії, releases, документація, CI, локалізації та життєвий цикл.
2. **Універсальність.** Немає жорсткої прив'язки до newsroom/domain/Telegram/database; взаємодія через versioned INFO Desktop Protocol.
3. **Сервер — джерело правди.** Desktop не дублює routing, permissions, publication rules, translations чи редакційні рішення.
4. **Реальні дані.** Read-only acceptance проходить на реальних текстах, медіа, каналах і живій черзі.
5. **Безпека.** Secrets не входять у Git/release; немає прямого SQLite/SSH; перша native transport surface фізично GET-only.
6. **Telegram паралельний.** Чинний Telegram INFO не вимикається і не урізається через Desktop.
7. **Платформи.** Windows 10/11 x64, macOS ARM64, macOS Intel x86_64, Linux x64. Windows 7 — лише можливий окремий legacy напрям пізніше.
8. **Стек.** Tauri 2 + TypeScript + React.
9. **Суттєві рішення.** Зміна основного стеку, матриці ОС, storage/security/protocol architecture спершу обговорюється з власником.
10. **Локалізація.** Українська, English, Deutsch, Español, Français, 한국어, 日本語; UI language, server language, channel language і content language — різні поняття.
11. **Версії.** Product versioning 0.1.0 → 1.0.0; видані версії не перевикористовувати.
12. **Git.** main — працездатний інтегрований стан; development branches + CI; releases immutable.
13. **Ліцензія.** Taxo model: proprietary / All rights reserved; public repository ≠ open source.
