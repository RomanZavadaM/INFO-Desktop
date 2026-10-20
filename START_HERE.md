# START_HERE — INFO Desktop

> Перша точка входу для кожної нової робочої сесії.

Перед змінами прочитати `PROJECT_RULES.md`, `PROJECT_STATE.md`, `WORKLOG.md`, перевірити фактичний GitHub: main, PR, CI, releases.

## Контрольний стан
- Repository: `RomanZavadaM/INFO-Desktop`
- Version: `0.1.1`
- PR #1 merged
- Production acceptance against INFO `20261020-2040`: passed
- Manual create: verified end-to-end
- GitHub Release: **[v0.1.1](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.1) published**
- Windows/Linux/macOS Intel/macOS ARM assets + SHA256: published

## Межі
- no direct SQLite;
- no SSH application protocol;
- no production secrets;
- Telegram remains parallel;
- do not alter production VPN/network/firewall/SSH.

## Post-release audit 2026-10-20
- public README/release/assets/version files checked;
- stale `0.1-r1` revision policy removed from project rules;
- connection architecture updated from obsolete GET-only wording to current scoped writes;
- Desktop User-Agent source aligned to `0.1.1`;
- published `v0.1.1` remains immutable; `main` now contains post-release audit corrections and must use a new version for the next published release.

## Next
1. Reconcile private INFO server PR #18 into INFO main.
2. Define signing/notarization/update policy for subsequent Desktop releases.
3. Deliberate publish acceptance only with explicit owner authorization.
4. Telegram preview polish, then TikTok/Facebook adapters.

## Новий чат
> **Продовжуємо INFO Desktop. Відкрий у GitHub `START_HERE.md` і продовжуй строго за ним.**
