# INFO Desktop

**INFO Desktop v0.1.4 stabil — Windows / Linux / macOS:** [Installationsdateien herunterladen](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.4). Visueller Editor, keine Windows-Konsole, XLSX-Export mit Zeitstempel.




## Funktionen

- Verbindung zu einem echten INFO-Server über `info-desktop-protocol/1`;
- einmalige Geräte-Registrierungsdatei;
- wiederverwendbares Gerätegeheimnis nur im sicheren Speicher des Betriebssystems;
- Kanäle, vollständige Pending-Warteschlange, Beitragsdetails, Quellen und zwischengespeicherte Medien;
- Bearbeitung von Titel, Text und Zielkanal;
- Speichern, Überspringen und Zurückstellen;
- manuelles Erstellen neuer Beiträge;
- Telegram-Vorschau;
- berechtigungsgebundene Veröffentlichung;
- sieben Oberflächensprachen.

## Produktionsabnahme

Am 20.10.2026 wurde INFO Desktop mit LIVE INFO `20261020-2040` verbunden. Ein echter Lwiw-Beitrag wurde über Desktop erstellt und erschien in der Redaktionswarteschlange sowie im Telegram-Redaktionsbot. Damit wurde Desktop → API → Datenbank → Redaktionswarteschlange bestätigt. Während dieses Tests wurde kein öffentlicher Telegram-SEND ausgeführt.

## Plattformen

Windows 10/11 x64 · macOS ARM64 · macOS Intel x86_64 · Linux x64. Windows 7 ist kein Pflichtziel.

## Sicherheit

Kein direkter SQLite-Zugriff, kein SSH als Anwendungsprotokoll, keine Produktionsgeheimnisse im Repository. Geräteberechtigungen sind auf Scopes beschränkt; das aktuelle Gateway bleibt VPN-only.

## Technologie und Build

Tauri 2 · TypeScript · React · Rust · reqwest/rustls.

```bash
npm install
npm run check
npm run test:protocol
npm run build
npm run tauri build
```

Siehe [Build](../BUILD.md), [Benutzerhandbuch](../guides/USER_MANUAL.md), [Protokoll](../protocol/INFO_DESKTOP_PROTOCOL.md), [Registrierung](../protocol/INFO_DESKTOP_REGISTRATION.md), [Release-Index](../releases/RELEASE_INDEX.md).

## Release-Status

Aktuell stabil ist **v0.1.4**. Der GitHub Release für Windows, Linux und macOS ist veröffentlicht. Die manuelle Windows-WebView2-Abnahme des neuen visuellen Editors steht noch aus.

**Copyright © 2026 Roman Zavada (Роман Завада). Alle Rechte vorbehalten. Proprietäre Software.**
