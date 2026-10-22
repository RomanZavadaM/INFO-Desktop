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

## Server settings (XLSX)
The "Settings" button in the header: "Download current settings" saves the XLSX; choose the edited file with "Choose XLSX file" - the server only checks it and shows a summary; "Apply the checked file" applies it after confirmation; "Roll back the last apply" returns the previous state. Requires the scopes `settings.read` and `settings.write` (issued at registration). The "what changes in 48 hours" probe still runs on the server with `info xlsx try-pending`.

### Probe and topics
"Probe: what changes in 48 h" starts a calculation on the server (a few minutes) and shows a per-channel report. The "Topics" tab: choose a topic, edit the words (`word*` - beginning of a word, `word!` - exactly this form), check them on a sample headline and save; rollback is in "Settings".

