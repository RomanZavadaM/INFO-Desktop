# INFO Desktop v0.1.4 — stable

Released after v0.1.4-1 test build.

- Visual rich-text editor instead of editing raw HTML tags; HTML source view remains available.
- Formatting and safer plain-text paste for newsroom drafts; seven supported UI languages.
- Windows: suppress console window in packaged release builds (retain debug terminal).
- Desktop XLSX export now appends **local download timestamp** YYYYMMDD-HHMMSS to avoid duplicate/ambiguous names; does not modify XLSX contents.
- INFO Server Desktop Gateway XLSX check HTTP 413 fix verified end-to-end by owner on 2026-10-24, INFO LIVE 20261024-1653, after scoped nginx route update. Export and unchanged-file reimport reported successful. **No settings were applied**.
- Builds for Windows EXE/MSI, Linux AppImage/DEB, macOS arm64/x86_64 DMG and portable ZIP, SHA256 checksums.

Notes: desktop visual editor manual WebView2 acceptance remains a follow-up; server live is managed in separate repository and is not part of this installer. No Telegram SEND was tested.
