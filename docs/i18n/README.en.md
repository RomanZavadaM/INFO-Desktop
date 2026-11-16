# INFO Desktop

**INFO Desktop 0.1.6 stable — Windows / Linux / macOS:** [Download](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.6). New: first-setup form. Previous stable: [0.1.5](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.5).

## What it does

- connects to a real INFO server through `info-desktop-protocol/1`, one-time device registration;
- the device secret is stored only in the OS secure credential store;
- reads channels, the full pending queue, item details, sources, translations and cached media;
- edits title, text and channel; saves, skips, defers and publishes items;
- creates manual items, Telegram-style preview;
- settings through Excel (XLSX): export, check without applying, "what changes in 48 h" probe, confirmed apply, rollback;
- topics: keyword editing and a check on sample text;
- **first-setup form** (since 0.1.6), console mode, seven UI languages.

## First-setup form

The "First setup" button in the top bar works even without a server connection. The form asks for the newsroom name, bot title, access values and channels, with an explanation and an example under every field and immediate format checks (bot token, ids, channel address `@name`, t.me links, site, time zone). The result is the file `info-first-setup.xlsx` on your own computer; **nothing is sent to the server**. Put the file on the server (for example `/var/lib/info/inbox/`) and load it with item 1 of `info menu`; delete it afterwards because it contains access values. Requires INFO server `20261116-1300` or newer. Details: [FIRST_SETUP_FORM](../FIRST_SETUP_FORM.md).

## Console mode

The `info-desktop-cli` program performs the same actions as the window: queue, "why is this in this channel", create and edit, publish, XLSX settings (download, check, probe, apply, rollback) and themes. It works in two ways: through the open window (a checkbox in the app, local to this computer, the key is never passed) or standalone (the connection profile is written by the window, the key stays in the OS credential store). Confirmation `--yes` is required only for applying and rolling back settings. Manual: [CONSOLE_MANUAL](en/CONSOLE_MANUAL.md).

## What was verified

On 20 October 2026 Desktop was connected to a real INFO server `20261020-2040`: a real item was created through Desktop and confirmed in the editorial queue and the Telegram editorial bot (Desktop → API → database → queue). No public Telegram publish was performed. Console read commands were verified by the owner on Windows. The first-setup form was verified by automated builds only: opening the created file in Excel and loading it on a server is not yet confirmed.

## Platforms

Windows 10/11 x64 · macOS ARM64 · macOS Intel x86_64 · Linux x64. Windows 7 is not a required target.

## Security

No direct SQLite access, no SSH application protocol, no production secrets in the repository. Device credentials are scoped; the INFO gateway remains VPN-only.

## Technology and build

Tauri 2 · TypeScript · React · Rust · reqwest/rustls.

```bash
npm install
npm run check
npm run test:protocol
npm run build
npm run tauri build
```

See [Build](../BUILD.md), [User manual](../guides/USER_MANUAL.md), [Protocol](../protocol/INFO_DESKTOP_PROTOCOL.md), [Registration](../protocol/INFO_DESKTOP_REGISTRATION.md), [Release index](../releases/RELEASE_INDEX.md).

## Release status

Current stable is **v0.1.6**. Packages for Windows, Linux and macOS are published on GitHub Releases.

**Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved. Proprietary software.**
