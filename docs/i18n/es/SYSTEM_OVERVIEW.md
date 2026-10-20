# INFO Desktop — Resumen del sistema

INFO Desktop is a separate cross-platform editorial workstation for INFO-compatible servers.

The Desktop client owns UI, localization, navigation, previews and permitted editor actions. The server owns data, permissions, routing, quality, translations, publication, validation and audit.

Desktop is **not a second server**: it has no direct database access and does not duplicate server business logic.

Transport: `info-desktop-protocol/1`, TLS, authenticated GET + scoped POST, OS secure credential storage. The current production gateway remains VPN-only.

La aceptación en producción del 20/10/2026 con INFO 20261020-2040 confirmó la creación manual de extremo a extremo. No se realizó ningún SEND público de Telegram.
