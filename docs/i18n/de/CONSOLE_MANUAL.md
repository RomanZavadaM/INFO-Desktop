# INFO Desktop Konsolenmodus: Kurzanleitung

Status: veröffentlicht in 0.1.5. Zwei Modi: über das geöffnete Anwendungsfenster und eigenständig.

## Was es ist
Das Programm `info-desktop-cli` tut, was das Fenster tut: zeigt die Warteschlange, erklärt Routen, erstellt und bearbeitet Einträge, veröffentlicht, arbeitet mit XLSX-Einstellungen. Der Geräteschlüssel stammt aus demselben geschützten Speicher des Betriebssystems wie beim Fenster und wird nie ausgegeben.

## Modi
- **Über das geöffnete Fenster (gemeinsame Verbindung).** Setzen Sie im Anwendungsfenster das Häkchen „Konsole über dieses Fenster arbeiten lassen“ (standardmäßig aus). Solange das Fenster offen ist, nutzt die Konsole dessen Verbindung: VPN und Schlüssel sind schon eingerichtet, der Schlüssel wird nie weitergegeben. Der Zugang ist lokal auf diesem Computer (127.0.0.1) und durch ein einmaliges Sitzungspasswort im Einstellungsordner des Benutzers geschützt. Das Fenster zeigt jede Konsolenaktion neben dem Kontrollkästchen.
- **Eigenständig.** Kein Fenster nötig: Die Konsole verbindet sich selbst mit dem Server anhand des Verbindungsprofils und des Schlüssels aus dem Betriebssystemspeicher. Das Fenster schreibt das Profil bei jeder Verbindung (Adresse, Port, Gerätenummer, Serverzertifikat; keine Schlüssel); es kann auch manuell gesetzt werden (`profile set`).
- Auswahl: `--mode auto` (Standard: Fenster, wenn offen und erlaubt, sonst eigenständig), `--mode app` (nur Fenster), `--mode alone` (nur eigenständig). Ein ausdrückliches `--profile` ohne `--mode` bedeutet eigenständig.

## Erster Start
1. Verbinden Sie sich im Anwendungsfenster wie gewohnt (unter Windows muss bei unserem Server das VPN aktiv sein).
2. Das Fenster schreibt das Profil selbst. Nur falls manuell nötig: `info-desktop-cli profile set --server ADRESSE --port PORT --device-id NUMMER --ca-file DATEI`; die Datei enthält den Wert `server_ca_der` aus der Registrierungsdatei.
3. Verbindung prüfen: `info-desktop-cli status`.

## Befehle
`info-desktop-cli` ohne Argumente zeigt die vollständige Hilfe.

| Aufgabe | Befehl |
|---|---|
| Kanäle auflisten | `channels` |
| Warteschlange | `queue list [--limit N] [--cursor N] [--channel CODE] [--bucket GRUPPE]` |
| Eintrag mit Quellen | `queue show ID` |
| Warum in diesem Kanal | `why ID --channel CODE [--scope route\|auto\|digest\|instant\|relevance]` |
| Eintrag erstellen | `create --channel CODE --title TEXT (--text TEXT \| --text-file DATEI) [--url URL]` |
| Eintrag ändern | `save ID --title TEXT (--text TEXT \| --text-file DATEI) [--channel CODE]` |
| Überspringen, verschieben, veröffentlichen | `skip ID`, `defer ID [--minutes N]`, `publish ID` |
| XLSX-Einstellungen | `settings status`, `settings download --out DATEI`, `settings check DATEI`, `settings probe [--hours N]`, `settings probe-status`, `settings apply [--without-probe] [--yes]`, `settings rollback [--yes]` |
| Themen und Wörter | `themes list`, `themes test ...`, `themes save ...` |

## Einstellungen ändern
Zuerst `settings check` (prüft die Datei ohne Anwendung), dann `settings probe` und `settings probe-status` („was ändert sich“), dann `settings apply`. Ohne `--yes` zeigt der Befehl nur, was angewendet würde. Es wird genau die geprüfte Datei angewendet (per Prüfsumme). Ohne Probe verweigert der Befehl, außer mit `--without-probe`. Zurücksetzen: `settings rollback --yes`.

Veröffentlichen (`publish`) und gewöhnliche redaktionelle Aktionen brauchen keine Bestätigung: nach Ermessen und in Verantwortung des Ausführenden; ein Beitrag kann jederzeit im Kanal gelöscht werden.

## Exit-Codes
0 Erfolg, 1 Befehlsfehler, 2 falsche Argumente, 3 keine Verbindung oder kein Schlüssel, 4 Server hat abgelehnt (keine Rechte, Konflikt).

## Noch nicht fertig
Betrieb auf einem Server ohne grafische Sitzung (vom Eigentümer zurückgestellt). Am 5. November 2026 unter Windows mit einem Live-Server geprüft (Lesebefehle): `status`, `channels`, `queue list`, `why` über das Fenster, `status` eigenständig und automatisch. Nicht geprüft: Schreibbefehle, XLSX und Themen, macOS und Linux.
