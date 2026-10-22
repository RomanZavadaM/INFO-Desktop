# INFO Desktop 0.1.2

Date: 2026-10-22

What's new:
- "Why in this channel?" button in the item card: the decisions of all channels for the event and the XLSX condition tree of the chosen channel (needs INFO server `20261022-2010` or newer);
- the application version is now shown in the header;
- checksum files (`SHA256SUMS-*.txt`) use the same names as the release files (no spaces), so `sha256sum -c` works by name;
- the server serves previews only for small images (videos and large files open through the source link).

Unchanged: protocol `info-desktop-protocol/1`, registration, security. Publishing from Desktop was not tested in this release.
