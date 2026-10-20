# INFO Desktop — User manual

1. Install INFO Desktop.
2. Select the interface language.
3. Import the administrator-provided registration file.
4. Activate the device once; the reusable secret is stored in the OS secure credential store.
5. Connect to the INFO server.
6. Select a channel and item from the editorial queue.
7. Review sources/media, edit title/text/channel and save.
8. Use skip/defer/create/publish only when the server grants the required scopes.

Manual create sends title, text, channel and optional source URL to the server and places the result into the normal editorial queue.

Desktop has no direct SQLite access, does not use SSH as an application protocol, stores reusable device secrets only in OS secure storage, and uses scoped server permissions.

Production acceptance on 20 October 2026 against INFO 20261020-2040 confirmed manual create end-to-end. No public Telegram SEND was performed.
