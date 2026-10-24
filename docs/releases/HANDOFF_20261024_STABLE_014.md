# INFO Desktop — HANDOFF після релізу v0.1.4 (24.10.2026)

## Порядок першого читання
1. `START_HERE.md`
2. `PROJECT_STATE.md`
3. `WORKLOG.md`
4. `docs/roadmap/ROADMAP.md`, `docs/PRODUCT_STATUS.md`
5. Фактичний GitHub `main`, відкриті PR, Actions та GitHub Release (GitHub першочерговий).

## Код/версії
- Репозиторій: `RomanZavadaM/INFO-Desktop`, працювати від `main`.
- Stable target: `v0.1.4`, Windows EXE/MSI, Linux AppImage/DEB, macOS ARM/Intel DMG/ZIP, 4 SHA256.
- Popередні: `v0.1.3` stable, `v0.1.4-1` pre-release для перевірки.
- PR #12: visual editor, PR #13: Windows console, PR #15: XLSX filenames timestamp YYYYMMDD-HHMMSS.
- XLSX імпорт/перевірка незмінного файлу пройшла у Desktop 0.1.4-1 після виправлення nginx HTTP 413.

## INFO server: жорстка межа
- Інший репозиторій `RomanZavadaM/INFO`; LIVE `20261024-1653`.
- INFO PR #31: лише `POST /api/info-desktop/v1/settings/xlsx/check` розширено до 12m в окремому nginx; решта 32k. Оператор зробив backup `/root/backup/info-desktop-gateway-20261024-225022/nginx.conf`, nginx -t SUCCESS, restart Desktop Gateway SUCCESS, `info check`: нуль помилок.
- Нічого з XLSX не застосовували, ніяких SEND/PIN Telegram.
- INFO має власний `INFO_PROTOCOL_CURRENT.md`; команди до сервера давати **одним готовим Bash-блоком** згідно із протоколом.

## Неперевірене
- Manual Windows WebView2 GUI acceptance нового редактора (вставлення, жирний/курсив/підкреслення, цитати, посилання, перемикання HTML та збереження/відкриття).
- Повторна перевірка XLSX-експорту вже з timestamp у stable 0.1.4.
- UX черги: швидке перемикання/масові дії — майбутня робота, не видавати за завершену.
- Окремий проєкт інших платформ оновив плани. Перед Facebook/TikTok інтеграцією **прочитати актуальні рішення цього проєкту**, не вигадувати власні.

## Порядок завершення релізу
1. Звірити що PR #15 і PR #16 злиті та 7/7 CI зелені.
2. Перевірити GitHub Release `v0.1.4`: 8 пакетів + 4 SHA256, прямі посилання.
3. README усіма 7 мовами, release index, START_HERE і WORKLOG звірити з фактичним релізом.
4. Підсумувати на Google Drive в папці INFO Desktop. Storage Box без прямого доступу самостійно не змінювати.
