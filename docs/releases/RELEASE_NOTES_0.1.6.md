# INFO Desktop v0.1.6

First-setup form.

## New
- New "First setup" screen (button in the top bar, works without a server connection): fill in the newsroom name, bot title, access values (bot token, admin id, reader account keys) and channels, with an explanation and an example under every field and immediate format checks (token, ids, channel address `@username`, t.me links, site, time zone).
- The form creates the settings file `info-first-setup.xlsx` on your own computer. Nothing is sent to the server. Put the file on the server (for example `/var/lib/info/inbox/`) and load it with item 1 of `info menu`; delete it afterwards because it contains access values.

## Compatibility
- Works with INFO server `20261116-1300` and newer, where `info menu` lists found XLSX files and offers a question-by-question first setup (item 7). The protocol did not change.
- The form structure comes from the neutral server template `import/INFO-template-blank.xlsx` (copy in `src/firstSetupTemplate.json`); see `docs/FIRST_SETUP_FORM.md`.

## Packages
Windows EXE/MSI, Linux AppImage/DEB, macOS ARM64 and Intel DMG and portable ZIP, console packages for the same platforms, SHA256 checksums.

## Not verified
The form was verified by automated builds only; opening the created file in Excel and loading it on a server was not yet done by the owner.
