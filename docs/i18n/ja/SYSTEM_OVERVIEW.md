# INFO Desktop — システム概要

INFO Desktop is a separate cross-platform editorial workstation for INFO-compatible servers.

The Desktop client owns UI, localization, navigation, previews and permitted editor actions. The server owns data, permissions, routing, quality, translations, publication, validation and audit.

Desktop is **not a second server**: it has no direct database access and does not duplicate server business logic.

Transport: `info-desktop-protocol/1`, TLS, authenticated GET + scoped POST, OS secure credential storage. The current production gateway remains VPN-only.

2026-10-20 の INFO 20261020-2040 本番受入で手動記事作成を end-to-end で確認しました。公開 Telegram SEND は実行していません。
