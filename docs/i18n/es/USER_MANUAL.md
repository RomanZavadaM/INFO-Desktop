# INFO Desktop — Manual de usuario

1. Install INFO Desktop.
2. Select the interface language.
3. Import the administrator-provided registration file.
4. Activate the device once; the reusable secret is stored in the OS secure credential store.
5. Connect to the INFO server.
6. Select a channel and item from the editorial queue.
7. Review sources/media, edit title/text/channel and save.
8. Use skip/defer/create/publish only when the server grants the required scopes.

Manual create sends title, text, channel and optional source URL to the server and places the result into the normal editorial queue.

Desktop no accede directamente a SQLite, no usa SSH como protocolo de aplicación, guarda los secretos reutilizables solo en el almacén seguro del SO y usa permisos por scope.

La aceptación en producción del 20/10/2026 con INFO 20261020-2040 confirmó la creación manual de extremo a extremo. No se realizó ningún SEND público de Telegram.
