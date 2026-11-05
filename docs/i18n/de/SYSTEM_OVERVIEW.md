# INFO Desktop — Systemübersicht

INFO Desktop is a separate cross-platform editorial workstation for INFO-compatible servers.

The Desktop client owns UI, localization, navigation, previews and permitted editor actions. The server owns data, permissions, routing, quality, translations, publication, validation and audit.

Desktop is **not a second server**: it has no direct database access and does not duplicate server business logic.

Transport: `info-desktop-protocol/1`, TLS, authenticated GET + scoped POST, OS secure credential storage. The current production gateway remains VPN-only.

Die Produktionsabnahme am 20.10.2026 mit INFO 20261020-2040 bestätigte die manuelle Erstellung vollständig. Kein öffentlicher Telegram-SEND wurde ausgeführt.

## Konsolenmodus
Die Konsole `info-desktop-cli` (Paket `src-tauri/cli`) und das Fenster nutzen das gemeinsame Paket `src-tauri/core` (Registrierung, Transport, Pfade, Profil, lokale Brücke) und dasselbe Protokoll `info-desktop-protocol/1`; das Protokoll wurde nicht geändert. Über das geöffnete Fenster arbeitet die Konsole mit Zustimmung des Benutzers: lokaler Zugang 127.0.0.1 und einmaliges Sitzungspasswort, der Geräteschlüssel wird nie weitergegeben. Eigenständig verwendet die Konsole das Verbindungsprofil (vom Fenster geschrieben) und den Schlüssel aus dem Anmeldedatenspeicher. Die Prüfung `scripts/check-static.mjs` stellt sicher, dass alle nativen Module genau einen redaktionellen POST und zwei POSTs für Schlüssel behalten.
