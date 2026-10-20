# INFO Desktop — Systemübersicht

INFO Desktop is a separate cross-platform editorial workstation for INFO-compatible servers.

The Desktop client owns UI, localization, navigation, previews and permitted editor actions. The server owns data, permissions, routing, quality, translations, publication, validation and audit.

Desktop is **not a second server**: it has no direct database access and does not duplicate server business logic.

Transport: `info-desktop-protocol/1`, TLS, authenticated GET + scoped POST, OS secure credential storage. The current production gateway remains VPN-only.

Die Produktionsabnahme am 20.10.2026 mit INFO 20261020-2040 bestätigte die manuelle Erstellung vollständig. Kein öffentlicher Telegram-SEND wurde ausgeführt.
