# INFO Desktop

[Українська](../../README.md) · [English](README.en.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Français](README.fr.md) · [한국어](README.ko.md) · [日本語](README.ja.md)

**INFO Desktop is a cross-platform newsroom workstation for INFO-compatible servers.**

Version **0.1.1** is the first accepted source checkpoint merged into `main`.

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

Source checkpoint `0.1.1` is accepted in `main`. Public GitHub Release installers are the next packaging step.

**Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved. Proprietary software.**
