# INFO Desktop — Vue d’ensemble du système

INFO Desktop is a separate cross-platform editorial workstation for INFO-compatible servers.

The Desktop client owns UI, localization, navigation, previews and permitted editor actions. The server owns data, permissions, routing, quality, translations, publication, validation and audit.

Desktop is **not a second server**: it has no direct database access and does not duplicate server business logic.

Transport: `info-desktop-protocol/1`, TLS, authenticated GET + scoped POST, OS secure credential storage. The current production gateway remains VPN-only.

La validation de production du 20/10/2026 avec INFO 20261020-2040 a confirmé la création manuelle de bout en bout. Aucun SEND Telegram public n’a été effectué.

## Mode console
La console `info-desktop-cli` (paquet `src-tauri/cli`) et la fenêtre partagent le paquet `src-tauri/core` (enregistrement, transport, chemins, profil, pont local) et le même protocole `info-desktop-protocol/1` ; le protocole n'a pas changé. Via la fenêtre ouverte, la console travaille avec l'accord de l'utilisateur : accès local 127.0.0.1 et mot de passe de session à usage unique, la clé de l'appareil n'est jamais transmise. De manière autonome, la console utilise le profil de connexion (écrit par la fenêtre) et la clé du magasin d'identifiants du système. Le contrôle `scripts/check-static.mjs` impose que tous les modules natifs gardent un seul POST éditorial et deux POST pour les clés.
