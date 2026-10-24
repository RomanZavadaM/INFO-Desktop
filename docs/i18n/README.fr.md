# INFO Desktop

## Version 0.1.2 et prochaine version 0.1.3

**0.1.2 est la dernière version stable publiée**. [Télécharger v0.1.2](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.2) pour Windows, Linux et macOS Intel/Apple Silicon. La branche `main` contient un **candidat 0.1.3 non publié** : exportation/vérification/application/retour arrière XLSX, simulation sur 48 heures, édition des mots-clés par thème et protection contre les simulations périmées. Le serveur INFO `20261024-1653` est installé, mais la validation intégrale via Desktop reste à faire. Enregistrer, ignorer, reporter et publier passent **directement par l'API**, et non par Excel. XLSX configure seulement les règles automatiques. Aucun envoi Telegram public de test n'a été validé.


[Українська](../../README.md) · [English](README.en.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Français](README.fr.md) · [한국어](README.ko.md) · [日本語](README.ja.md)

**INFO Desktop est un poste de travail rédactionnel multiplateforme pour les serveurs compatibles INFO.**

La version **0.1.2** est le premier jalon source accepté et fusionné dans `main`.

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

Le checkpoint source `0.1.2` est accepté dans `main`. See release status above.

**Copyright © 2026 Roman Zavada (Роман Завада). Tous droits réservés. Logiciel propriétaire.**
