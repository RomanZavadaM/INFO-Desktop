# INFO Desktop — Benutzerhandbuch

1. Install INFO Desktop.
2. Select the interface language.
3. Import the administrator-provided registration file.
4. Activate the device once; the reusable secret is stored in the OS secure credential store.
5. Connect to the INFO server.
6. Select a channel and item from the editorial queue.
7. Review sources/media, edit title/text/channel and save.
8. Use skip/defer/create/publish only when the server grants the required scopes.

Manual create sends title, text, channel and optional source URL to the server and places the result into the normal editorial queue.

Desktop hat keinen direkten SQLite-Zugriff, verwendet SSH nicht als Anwendungsprotokoll, speichert wiederverwendbare Geräteschlüssel nur im sicheren OS-Speicher und nutzt serverseitige Scopes.

Die Produktionsabnahme am 20.10.2026 mit INFO 20261020-2040 bestätigte die manuelle Erstellung vollständig. Kein öffentlicher Telegram-SEND wurde ausgeführt.

## Servereinstellungen (XLSX)
Die Schaltfläche „Einstellungen“ in der Kopfzeile: „Aktuelle Einstellungen herunterladen“ speichert die XLSX-Datei; die bearbeitete Datei wählen Sie mit „XLSX-Datei wählen“ - der Server prüft sie nur und zeigt eine Zusammenfassung; „Geprüfte Datei anwenden“ wendet sie nach Bestätigung an; „Letzte Anwendung zurücknehmen“ stellt den vorherigen Zustand wieder her. Erfordert die Scopes `settings.read` und `settings.write` (bei der Registrierung vergeben). Die Probe „Was ändert sich in 48 Stunden“ läuft vorerst auf dem Server mit `info xlsx try-pending`.

### Probe und Themen
„Probe: was ändert sich in 48 Std.“ startet eine Berechnung auf dem Server (einige Minuten) und zeigt einen Bericht je Kanal. Tab „Themen“: Thema wählen, Wörter bearbeiten (`Wort*` - Wortanfang, `Wort!` - genau diese Form), an einer Beispielüberschrift prüfen und speichern; Zurücknehmen unter „Einstellungen“.

