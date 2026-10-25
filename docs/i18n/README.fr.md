# INFO Desktop

**Développement après stable :** `main` passe au source **0.1.5 (development)**. Ce n'est pas encore un Release ; les installateurs téléchargeables restent le v0.1.4 vérifié.

**INFO Desktop v0.1.4 stable — Windows / Linux / macOS :** [Télécharger les installateurs](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.4). Éditeur visuel, console Windows masquée et exports XLSX horodatés.




## Fonctionnalités

- connexion à un serveur INFO réel via `info-desktop-protocol/1`;
- fichier d’enregistrement de périphérique à usage unique;
- secret réutilisable stocké uniquement dans le coffre sécurisé du système;
- canaux, file complète, détails, sources et médias mis en cache;
- modification du titre, du texte et du canal cible;
- enregistrement, report et suppression éditoriale;
- création manuelle de nouveaux articles;
- aperçu de publication Telegram;
- publication limitée par les scopes;
- sept langues d’interface.

## Validation en production

Le 20 octobre 2026, INFO Desktop a été connecté à LIVE INFO `20261020-2040`. Un véritable article de Lviv a été créé depuis Desktop et est apparu dans la file éditoriale ainsi que dans le bot Telegram éditorial. Le chemin Desktop → API → base de données → file éditoriale a donc été validé. Aucun SEND Telegram public n’a été effectué pendant ce test.

## Plateformes

Windows 10/11 x64 · macOS ARM64 · macOS Intel x86_64 · Linux x64. Windows 7 n’est pas une cible obligatoire.

## Sécurité

Pas d’accès SQLite direct, pas de SSH comme protocole applicatif, aucun secret de production dans le dépôt. Les identifiants utilisent des scopes; la passerelle actuelle reste VPN-only.

## Technologie et build

Tauri 2 · TypeScript · React · Rust · reqwest/rustls.

```bash
npm install
npm run check
npm run test:protocol
npm run build
npm run tauri build
```

Voir [Build](../BUILD.md), [Manuel utilisateur](../guides/USER_MANUAL.md), [Protocole](../protocol/INFO_DESKTOP_PROTOCOL.md), [Enregistrement](../protocol/INFO_DESKTOP_REGISTRATION.md), [Index des versions](../releases/RELEASE_INDEX.md).

## État de publication

La version stable actuelle est **v0.1.4**. Le GitHub Release est publié pour Windows, Linux et macOS. La validation manuelle Windows WebView2 du nouvel éditeur visuel reste à faire.

**Copyright © 2026 Roman Zavada (Роман Завада). Tous droits réservés. Logiciel propriétaire.**
