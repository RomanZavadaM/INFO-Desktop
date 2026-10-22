# INFO Desktop — дорожня карта

## DONE — фундамент і реальний INFO
- Tauri 2 / React / TypeScript / Rust;
- Windows/macOS/Linux;
- 7 мов;
- registration v1;
- TLS/private CA;
- реальна черга та картка;
- authenticated media;
- scoped write actions;
- manual create production acceptance;
- merge першого accepted client у `main`.

## NEXT — release discipline
- імена файлів у `SHA256SUMS-*.txt` без пропусків (збігаються з іменами в Release);
- політика signing/notarization/updater (чернетка `docs/decisions/SIGNING_UPDATES.md`, чекає рішення власника);
- зберігати українську як canonical documentation, усі публічні документи синхронізувати 7 мовами.

## NEXT — редакторський цикл
- acceptance на INFO `20261022-1730` (канали «Новини» і «Тривоги»);
- «Чому»: пояснення, чому матеріал потрапив чи не потрапив у канал (серверний запит + кнопка в картці);
- ~~XLSX: експорт, перевірка, застосування, відкат~~ (зроблено 22.10, злито в main; у релізі 0.1.3);
- «Що зміниться»: запуск проби з Desktop окремою службою сервера, звіт по каналах у Desktop (довга операція: запуск і опитування стану);
- редактор тем: ключові слова за темами, їхні відмінки, перевірка слів на прикладах (наступний етап);
- deliberate publish acceptance тільки з явним дозволом власника;
- UI polish.

## LATER
- TikTok preview/publishing adapter;
- Facebook preview/publishing adapter;
- інші INFO-compatible сервери;
- редактор XLSX-налаштувань у Desktop (експорт, правка, проба, застосування, відкат);
- updater після визначення політики підпису.

## Жорстка межа
Не змінювати production VPN/network/firewall/SSH без окремого рішення власника.
