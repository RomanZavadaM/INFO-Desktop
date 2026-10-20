# START_HERE — INFO Desktop

> Перша точка входу для кожної нової робочої сесії.

## Startup protocol

Перед змінами:
1. Прочитати цей файл.
2. Прочитати `PROJECT_RULES.md`.
3. Прочитати `PROJECT_STATE.md`.
4. Прочитати `WORKLOG.md`.
5. Перевірити фактичний GitHub: main SHA, відкриті PR, CI, releases.
6. Продовжити з першого незавершеного `DOING/NEXT`.

GitHub має пріоритет над пам'яттю чату.

## Поточний контрольний стан

- Repository: `RomanZavadaM/INFO-Desktop`.
- Default branch: `main`.
- Version: `0.1.1`.
- PR #1 merged.
- Accepted main commit: `852ee086769f2ce02246611d8772dfbb6ddc423d`.
- First real production-connected acceptance completed against INFO `20261020-2040`.
- Manual create verified end-to-end.
- Public GitHub Release with installers is still pending.

## Поточні межі

- Реальні INFO data only; no mock newsroom behavior in acceptance.
- Authenticated editor changes only through scoped Desktop API.
- No direct SQLite access.
- No SSH as an application protocol.
- No production secrets in repository.
- Telegram remains a parallel working client.
- Do not alter production VPN/network/firewall/SSH from Desktop work.

## Architecture checkpoint

- Tauri 2 + TypeScript + React + Rust.
- Windows 10/11 x64, macOS ARM64, macOS Intel x86_64, Linux x64.
- Windows 7 is not required.
- Authenticated GET + scoped POST.
- One-file device registration, private INFO CA, OS secure credential store.

## Acceptance passed

Desktop now connects to real INFO and can:
- read status/channels/full pending queue;
- open item/source/media details;
- edit title/text/channel;
- save/skip/defer;
- create a manual material;
- show Telegram-style preview;
- expose scoped publish action.

On 2026-10-20 one real Lviv item was created through Desktop and appeared in the editorial bot. No public Telegram SEND was used for that test.

## Next

1. Reconcile/merge private INFO server PR #18 so server source in INFO main matches LIVE 2040.
2. Finish release packaging policy and publish the first GitHub Release installers.
3. Verify one deliberate publish from Desktop only with owner authorization.
4. Continue Telegram preview polish, then TikTok and Facebook adapters.

## Новий чат

> **Продовжуємо INFO Desktop. Відкрий у GitHub `START_HERE.md` і продовжуй строго за ним.**
