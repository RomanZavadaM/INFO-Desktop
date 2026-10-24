# INFO Desktop

**INFO Desktop v0.1.4 stable — Windows / Linux / macOS:** [Download installers](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.4). Includes the visual editor, hidden Windows release console, and timestamped XLSX downloads.


> ⬇️ **Download INFO Desktop 0.1.3**: [**Windows EXE / MSI · Linux AppImage / DEB · macOS Intel / Apple Silicon**](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.3)

**INFO Desktop 0.1.3 is now released.** [Download v0.1.3](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.3) for Windows, Linux, macOS Intel and Apple Silicon. XLSX settings and topics are included; complete end-to-end desktop acceptance is still pending.

## What it does

- connects to a real INFO server through `info-desktop-protocol/1`;
- imports a one-time device registration file;
- stores the reusable device secret only in the OS secure credential store;
- reads channels, the full pending queue, item details, sources and cached media;
- edits title, text and destination channel;
- saves, skips and defers editorial items;
- creates new manual items;
- provides a Telegram-style publish preview;
- exposes a scoped publish action for authorized devices;
- supports seven UI languages.

## Production acceptance

On 20 October 2026 INFO Desktop was connected to LIVE INFO `20261020-2040`. A real Lviv item was created through Desktop and appeared in the editorial queue and Telegram editorial bot. This verified Desktop → API → database → editorial queue. No public Telegram SEND was performed during that acceptance.

## Platforms

Windows 10/11 x64 · macOS ARM64 · macOS Intel x86_64 · Linux x64. Windows 7 is not a required target.

## Security

No direct SQLite access, no SSH application protocol, no production secrets in the repository. Device credentials are scoped; the current INFO Desktop gateway remains VPN-only.

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

Source checkpoint `0.1.2` is accepted in `main`. See release status above.

**Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved. Proprietary software.**
