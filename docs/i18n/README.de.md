# INFO Desktop

**INFO Desktop 0.1.6 stabil — Windows / Linux / macOS:** [Herunterladen](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.6). Neu: Formular für die Ersteinrichtung. Vorherige stabile Version: [0.1.5](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.5).

## Funktionen

- Verbindung zu einem echten INFO-Server über `info-desktop-protocol/1`, einmalige Geräteregistrierung;
- das Gerätegeheimnis liegt nur im sicheren Speicher des Betriebssystems;
- Kanäle, vollständige Warteschlange, Materialdetails, Quellen, Übersetzungen und zwischengespeicherte Medien;
- Titel, Text und Kanal bearbeiten; speichern, überspringen, zurückstellen, veröffentlichen;
- manuelles Anlegen von Materialien, Vorschau im Telegram-Stil;
- Einstellungen über Excel (XLSX): Export, Prüfung ohne Anwendung, Probe „Was ändert sich in 48 Std.“, bestätigte Anwendung, Rollback;
- Themen: Schlüsselwörter bearbeiten und an einem Beispieltext prüfen;
- **Formular für die Ersteinrichtung** (seit 0.1.6), Konsolenmodus, sieben Oberflächensprachen.

## Formular für die Ersteinrichtung

Die Schaltfläche „Ersteinrichtung“ in der oberen Leiste funktioniert auch ohne Serververbindung. Das Formular fragt nach Redaktionsname, Bot-Titel, Zugangswerten und Kanälen; unter jedem Feld stehen eine Erklärung und ein Beispiel, das Format (Bot-Token, IDs, Kanaladresse `@name`, t.me-Links, Website, Zeitzone) wird sofort geprüft. Das Ergebnis ist die Datei `info-first-setup.xlsx` auf Ihrem eigenen Computer; **nichts wird an den Server gesendet**. Legen Sie die Datei auf dem Server ab (z. B. `/var/lib/info/inbox/`) und laden Sie sie mit Punkt 1 von `info menu`; löschen Sie sie danach, da sie Zugangswerte enthält. Erfordert INFO-Server `20261116-1300` oder neuer. Details: [FIRST_SETUP_FORM](../FIRST_SETUP_FORM.md).

## Konsolenmodus

Das Programm `info-desktop-cli` führt dieselben Aktionen aus wie das Fenster: Warteschlange, „Warum in diesem Kanal“, Erstellen und Bearbeiten, Veröffentlichen, XLSX-Einstellungen (Download, Prüfung, Probe, Anwendung, Rollback) und Themen. Es arbeitet auf zwei Wegen: über das geöffnete Fenster (Kontrollkästchen in der App, nur lokal, der Schlüssel wird nicht weitergegeben) oder eigenständig (das Verbindungsprofil schreibt das Fenster, der Schlüssel bleibt im Speicher des Betriebssystems). Die Bestätigung `--yes` ist nur beim Anwenden und Zurücksetzen von Einstellungen nötig. Anleitung: [CONSOLE_MANUAL](de/CONSOLE_MANUAL.md).

## Was geprüft wurde

Am 20. Oktober 2026 war Desktop mit einem echten INFO-Server `20261020-2040` verbunden: Ein echtes Material wurde über Desktop angelegt und in der Redaktionswarteschlange und im Telegram-Redaktionsbot bestätigt (Desktop → API → Datenbank → Warteschlange). Eine öffentliche Telegram-Veröffentlichung erfolgte nicht. Lesebefehle der Konsole wurden vom Eigentümer unter Windows geprüft. Das Ersteinrichtungsformular wurde nur durch automatische Builds geprüft: Das Öffnen der erzeugten Datei in Excel und das Laden auf einem Server sind noch nicht bestätigt.

## Plattformen

Windows 10/11 x64 · macOS ARM64 · macOS Intel x86_64 · Linux x64. Windows 7 ist kein verpflichtendes Ziel.

## Sicherheit

Kein direkter SQLite-Zugriff, kein SSH als Anwendungsprotokoll, keine Produktionsgeheimnisse im Repository. Geräte-Zugangsdaten sind eingeschränkt; das INFO-Gateway bleibt nur über VPN erreichbar.

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

Aktuell stabil ist **v0.1.6**. Pakete für Windows, Linux und macOS sind in den GitHub Releases veröffentlicht.

**Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved. Proprietary software.**
