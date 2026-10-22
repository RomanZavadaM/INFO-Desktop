# INFO Desktop 0.1.2

Datum: 2026-10-22

Neu:
- Schaltfläche „Warum in diesem Kanal?“ in der Beitragskarte: die Entscheidungen aller Kanäle für das Ereignis und der XLSX-Bedingungsbaum des gewählten Kanals (erfordert INFO-Server `20261022-2010` oder neuer);
- die Anwendungsversion wird jetzt in der Kopfzeile angezeigt;
- Prüfsummendateien (`SHA256SUMS-*.txt`) verwenden dieselben Namen wie die Release-Dateien (ohne Leerzeichen), sodass `sha256sum -c` nach Namen funktioniert;
- der Server liefert Vorschauen nur für kleine Bilder (Videos und große Dateien öffnen sich über den Quelllink).

Unverändert: Protokoll `info-desktop-protocol/1`, Registrierung, Sicherheit. Veröffentlichen aus Desktop wurde in diesem Release nicht getestet.
