# INFO Desktop

## Version 0.1.2 und nächste Version 0.1.3

**0.1.2 ist der neueste veröffentlichte stabile Download**. [v0.1.2 herunterladen](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.2) für Windows, Linux und macOS (Intel/Apple Silicon). Der aktuelle `main`-Zweig enthält einen **noch unveröffentlichten 0.1.3-Kandidaten**: XLSX-Export/Prüfung/Anwendung/Rücksetzung, 48-Stunden-Simulation, Themen-Schlüsselwörter und Schutz vor veralteten Simulationen. INFO-Server `20261024-1653` ist installiert; die vollständige Desktop-Abnahme fehlt noch. Redaktionelle Aktionen (Speichern/Überspringen/Zurückstellen/Veröffentlichen) laufen **direkt über die Server-API**, nicht über Excel. XLSX dient nur zur Konfiguration automatischer Regeln. Kein öffentlicher Telegram-Versand wurde abgenommen.


[Українська](../../README.md) · [English](README.en.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Français](README.fr.md) · [한국어](README.ko.md) · [日本語](README.ja.md)

**INFO Desktop ist ein plattformübergreifender Newsroom-Arbeitsplatz für INFO-kompatible Server.**

Version **0.1.2** ist der erste akzeptierte Quellcode-Checkpoint in `main`.

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
