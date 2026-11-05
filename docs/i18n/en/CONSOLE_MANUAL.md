# INFO Desktop console mode: short manual

Status: released in 0.1.5. Two modes: through the open application window and standalone.

## What it is
The `info-desktop-cli` program does what the window does: shows the queue, explains routes, creates and edits items, publishes, works with XLSX settings. The device key comes from the same secure OS store as the window's and is never printed.

## Modes
- **Through the open window (shared connection).** In the application window tick "Let the console work through this window" (off by default). While the window is open the console uses its connection: VPN and key are already set up and the key is never passed. The entry is local to this computer (127.0.0.1) and protected by a one-time session password stored in the user's settings folder. The window shows every console action next to the checkbox.
- **Standalone.** No window needed: the console connects to the server itself using the connection profile and the key from the OS store. The window writes the profile on every connect (address, port, device number, server certificate; no keys); it can also be set manually (`profile set`).
- Choice: `--mode auto` (default: the window if it is open and allows the console, otherwise standalone), `--mode app` (window only), `--mode alone` (standalone only). An explicit `--profile` without `--mode` means standalone.

## First run
1. Connect in the application window as usual (on Windows with our server the VPN must be on).
2. The window writes the profile itself. Only if you need it manually: `info-desktop-cli profile set --server ADDRESS --port PORT --device-id ID --ca-file FILE`; the file contains the `server_ca_der` value from the registration file.
3. Check the connection: `info-desktop-cli status`.

## Commands
Run `info-desktop-cli` without arguments for the full help.

| Task | Command |
|---|---|
| List channels | `channels` |
| Queue | `queue list [--limit N] [--cursor N] [--channel CODE] [--bucket GROUP]` |
| Item with sources | `queue show ID` |
| Why is it in this channel | `why ID --channel CODE [--scope route\|auto\|digest\|instant\|relevance]` |
| Create an item | `create --channel CODE --title TEXT (--text TEXT \| --text-file FILE) [--url URL]` |
| Edit an item | `save ID --title TEXT (--text TEXT \| --text-file FILE) [--channel CODE]` |
| Skip, defer, publish | `skip ID`, `defer ID [--minutes N]`, `publish ID` |
| XLSX settings | `settings status`, `settings download --out FILE`, `settings check FILE`, `settings probe [--hours N]`, `settings probe-status`, `settings apply [--without-probe] [--yes]`, `settings rollback [--yes]` |
| Themes and words | `themes list`, `themes test ...`, `themes save ...` |

## Changing settings
First `settings check` (checks the file without applying), then `settings probe` and `settings probe-status` ("what will change"), then `settings apply`. Without `--yes` the command only shows what would be applied. Exactly the checked file is applied (by digest). Without a probe the command refuses unless `--without-probe` is added. Rollback: `settings rollback --yes`.

Publishing (`publish`) and ordinary editorial actions need no confirmation: at the performer's discretion and responsibility; a post can always be removed in the channel.

## Exit codes
0 success, 1 command error, 2 wrong arguments, 3 no connection or key, 4 server refused (no rights, conflict).

## Not done yet
Operation on a server without a graphical session (postponed by the owner). Verified on Windows on 5 Nov 2026 against a live server (read commands): `status`, `channels`, `queue list`, `why` through the window, `status` standalone and automatic. Not verified: write commands, XLSX and themes, macOS and Linux.
