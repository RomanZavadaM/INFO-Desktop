# PROJECT_RULES — постійні правила INFO Desktop

1. **Окремий продукт.** Власні версії, releases, документація, CI, локалізації та життєвий цикл.
2. **Універсальність.** Немає жорсткої прив'язки до newsroom/domain/Telegram/database; взаємодія через versioned INFO Desktop Protocol.
3. **Сервер — джерело правди.** Desktop не дублює routing, permissions, publication rules, translations чи редакційні рішення.
4. **Реальні дані.** Acceptance проходить на реальних текстах, медіа, каналах і живій черзі; write/create/publish перевіряються лише в межах явно дозволеного scope.
5. **Безпека.** Secrets не входять у Git/release; немає прямого SQLite/SSH; Desktop працює через HTTPS API, а mutating actions дозволяються тільки окремими server-side scopes.
6. **Telegram паралельний.** Чинний Telegram INFO не вимикається і не урізається через Desktop.
7. **Платформи.** Windows 10/11 x64, macOS ARM64, macOS Intel x86_64, Linux x64. Windows 7 — лише можливий окремий legacy напрям пізніше.
8. **Стек.** Tauri 2 + TypeScript + React.
9. **Суттєві рішення.** Зміна основного стеку, матриці ОС, storage/security/protocol architecture спершу обговорюється з власником.
10. **Локалізація.** Українська, English, Deutsch, Español, Français, 한국어, 日本語; UI language, server language, channel language і content language — різні поняття.
11. **Версії.** Канонічна версія продукту — SemVer (`MAJOR.MINOR.PATCH`) і має бути однакова у `VERSION.txt`, `package.json`, `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`, назвах release artifacts та release notes. Опубліковану версію не перевикористовувати.
12. **Git.** `main` — працездатний інтегрований стан; development branches + CI; опубліковані releases immutable.
13. **Ліцензія.** Taxo model: proprietary / All rights reserved; public repository ≠ open source.
