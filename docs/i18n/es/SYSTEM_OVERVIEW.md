# INFO Desktop — Resumen del sistema

INFO Desktop is a separate cross-platform editorial workstation for INFO-compatible servers.

The Desktop client owns UI, localization, navigation, previews and permitted editor actions. The server owns data, permissions, routing, quality, translations, publication, validation and audit.

Desktop is **not a second server**: it has no direct database access and does not duplicate server business logic.

Transport: `info-desktop-protocol/1`, TLS, authenticated GET + scoped POST, OS secure credential storage. The current production gateway remains VPN-only.

La aceptación en producción del 20/10/2026 con INFO 20261020-2040 confirmó la creación manual de extremo a extremo. No se realizó ningún SEND público de Telegram.

## Modo consola
La consola `info-desktop-cli` (paquete `src-tauri/cli`) y la ventana comparten el paquete `src-tauri/core` (registro, transporte, rutas, perfil, puente local) y el mismo protocolo `info-desktop-protocol/1`; el protocolo no cambió. A través de la ventana abierta la consola trabaja con el consentimiento del usuario: entrada local 127.0.0.1 y contraseña de sesión de un solo uso, la clave del dispositivo nunca se transmite. De forma independiente, la consola usa el perfil de conexión (escrito por la ventana) y la clave del almacén de credenciales del sistema. La comprobación `scripts/check-static.mjs` exige que todos los módulos nativos mantengan un único POST editorial y dos POST de claves.
