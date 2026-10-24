# INFO Desktop

**INFO Desktop v0.1.4 stable — Windows / Linux / macOS:** [Download installers](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.4). Includes the visual editor, hidden Windows release console, and timestamped XLSX downloads.


> ⬇️ **INFO Desktop 0.1.3 herunterladen**: [**Windows EXE / MSI · Linux AppImage / DEB · macOS Intel / Apple Silicon**](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.3)

**INFO Desktop 0.1.3 ist veröffentlicht.** [v0.1.3 herunterladen](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.3) für Windows, Linux und macOS Intel/Apple Silicon. XLSX-Einstellungen und Themen sind enthalten; die vollständige Desktop-Abnahme steht noch aus.

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

Der Quellcode-Checkpoint `0.1.2` ist in `main` akzeptiert. See release status above.

**Copyright © 2026 Roman Zavada (Роман Завада). Alle Rechte vorbehalten. Proprietäre Software.**
