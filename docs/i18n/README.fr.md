# INFO Desktop

**INFO Desktop 0.1.6 stable — Windows / Linux / macOS :** [Télécharger](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.6). Nouveau : formulaire de première configuration. Version stable précédente : [0.1.5](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.5).

## Fonctionnalités

- connexion à un vrai serveur INFO via `info-desktop-protocol/1`, enregistrement unique de l'appareil ;
- le secret de l'appareil est conservé uniquement dans le coffre sécurisé du système ;
- canaux, file complète, détails des contenus, sources, traductions et médias en cache ;
- modification du titre, du texte et du canal ; enregistrer, ignorer, reporter, publier ;
- création manuelle de contenus, aperçu au style Telegram ;
- réglages via Excel (XLSX) : export, vérification sans application, essai « ce qui change en 48 h », application confirmée, retour arrière ;
- thèmes : édition des mots-clés et vérification sur un texte d'exemple ;
- **formulaire de première configuration** (depuis 0.1.6), mode console, sept langues d'interface.

## Formulaire de première configuration

Le bouton « Première configuration » de la barre supérieure fonctionne même sans connexion au serveur. Le formulaire demande le nom de la rédaction, le titre du bot, les valeurs d'accès et les canaux ; chaque champ est accompagné d'une explication et d'un exemple, et le format (jeton du bot, identifiants, adresse du canal `@nom`, liens t.me, site, fuseau horaire) est vérifié immédiatement. Le résultat est le fichier `info-first-setup.xlsx` sur votre propre ordinateur ; **rien n'est envoyé au serveur**. Placez le fichier sur le serveur (par exemple `/var/lib/info/inbox/`) et chargez-le avec l'option 1 de `info menu` ; supprimez-le ensuite, car il contient des valeurs d'accès. Nécessite le serveur INFO `20261116-1300` ou plus récent. Détails : [FIRST_SETUP_FORM](../FIRST_SETUP_FORM.md).

## Mode console

Le programme `info-desktop-cli` effectue les mêmes actions que la fenêtre : file, « pourquoi dans ce canal », création et modification, publication, réglages XLSX (téléchargement, vérification, essai, application, retour arrière) et thèmes. Il fonctionne de deux façons : via la fenêtre ouverte (case à cocher dans l'application, local à cet ordinateur, la clé n'est pas transmise) ou de manière autonome (le profil de connexion est écrit par la fenêtre, la clé reste dans le coffre du système). La confirmation `--yes` n'est exigée que pour appliquer et annuler des réglages. Manuel : [CONSOLE_MANUAL](fr/CONSOLE_MANUAL.md).

## Ce qui a été vérifié

Le 20 octobre 2026, Desktop a été connecté à un vrai serveur INFO `20261020-2040` : un vrai contenu a été créé via Desktop et confirmé dans la file éditoriale et dans le bot éditorial Telegram (Desktop → API → base de données → file). Aucune publication publique sur Telegram n'a été effectuée. Les commandes de lecture de la console ont été vérifiées par le propriétaire sous Windows. Le formulaire de première configuration n'a été vérifié que par des builds automatiques : l'ouverture du fichier créé dans Excel et son chargement sur un serveur ne sont pas encore confirmés.

## Plateformes

Windows 10/11 x64 · macOS ARM64 · macOS Intel x86_64 · Linux x64. Windows 7 n'est pas une cible obligatoire.

## Sécurité

Pas d'accès direct à SQLite, pas de SSH comme protocole applicatif, aucun secret de production dans le dépôt. Les identifiants de l'appareil sont limités par périmètre ; la passerelle INFO reste accessible uniquement par VPN.

## Technologie et compilation

Tauri 2 · TypeScript · React · Rust · reqwest/rustls.

```bash
npm install
npm run check
npm run test:protocol
npm run build
npm run tauri build
```

Voir [Compilation](../BUILD.md), [Manuel utilisateur](../guides/USER_MANUAL.md), [Protocole](../protocol/INFO_DESKTOP_PROTOCOL.md), [Enregistrement](../protocol/INFO_DESKTOP_REGISTRATION.md), [Index des versions](../releases/RELEASE_INDEX.md).

## État de la version

La version stable actuelle est **v0.1.6**. Les paquets pour Windows, Linux et macOS sont publiés dans GitHub Releases.

**Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved. Proprietary software.**
