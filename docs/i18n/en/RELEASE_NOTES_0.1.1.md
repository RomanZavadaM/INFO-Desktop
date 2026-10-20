# INFO Desktop 0.1.1

First stable public checkpoint of INFO Desktop.

Includes real INFO connectivity, one-time device registration, OS secure credential storage, full pending queue, item/source/media views, editing, save/skip/defer, manual item creation, Telegram-style preview, scoped publish action, seven UI languages, multilingual documentation, and Windows/macOS/Linux builds.

Production acceptance on 20 October 2026 against LIVE INFO `20261020-2040` confirmed a real Lviv item created through Desktop and visible in the editorial queue and Telegram editorial bot. No public Telegram SEND was performed during that acceptance.

Security: no direct SQLite access, no SSH application protocol, no production secrets in the repository, device secrets only in OS secure storage, server-side scopes, current production gateway VPN-only.

Ukrainian is the canonical documentation language.
