# INFO Desktop v0.1.5

Faster daily editorial cycle and a new console mode.

## Editor
- Save, Publish, Skip and Defer no longer reload the whole queue: only the affected item is updated.
- The full paginated queue is still loaded on connect and on manual refresh, but the first pages are shown progressively.
- Local search, bucket filter, explicit "Refresh queue", "Save and next", and a confirmation before unsaved edits are discarded on Skip or Defer.

## Console mode (new)
- New program `info-desktop-cli` for Windows, Linux and macOS (ARM64 and Intel): queue, "why is this in this channel", create/save/skip/defer/publish, settings through XLSX (download, check, probe, apply, rollback) and themes. Manual: `docs/guides/CONSOLE_MANUAL.md`.
- Two modes: through the open application window (opt-in checkbox in the window, local to the computer, one-time session password, the key is never passed) or standalone with the connection profile written by the window and the device key from the OS credential store.
- Confirmation (`--yes`) is required only for applying and rolling back settings.
- Verified by the owner on Windows against a live server for the read commands (status, channels, queue list, why) through the window and standalone. Write, XLSX and themes commands are covered by automated tests only.

## Packages
Windows EXE/MSI, Linux AppImage/DEB, macOS ARM64 and Intel DMG and portable ZIP, console packages for the same platforms, SHA256 checksums.

## Notes
The server (INFO) is a separate project and is not part of these packages; the protocol did not change. No Telegram SEND was performed in testing. Server-side operation of the console without a graphical session is not part of this release.
