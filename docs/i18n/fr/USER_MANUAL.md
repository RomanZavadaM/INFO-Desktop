# INFO Desktop — Manuel utilisateur

1. Install INFO Desktop.
2. Select the interface language.
3. Import the administrator-provided registration file.
4. Activate the device once; the reusable secret is stored in the OS secure credential store.
5. Connect to the INFO server.
6. Select a channel and item from the editorial queue.
7. Review sources/media, edit title/text/channel and save.
8. Use skip/defer/create/publish only when the server grants the required scopes.

Manual create sends title, text, channel and optional source URL to the server and places the result into the normal editorial queue.

Desktop n’accède pas directement à SQLite, n’utilise pas SSH comme protocole applicatif, stocke les secrets réutilisables uniquement dans le coffre sécurisé de l’OS et utilise des scopes serveur.

La validation de production du 20/10/2026 avec INFO 20261020-2040 a confirmé la création manuelle de bout en bout. Aucun SEND Telegram public n’a été effectué.

## Paramètres du serveur (XLSX)
Le bouton « Paramètres » dans l'en-tête : « Télécharger les paramètres actuels » enregistre le XLSX ; le fichier modifié se choisit avec « Choisir un fichier XLSX » - le serveur ne fait que le vérifier et affiche un résumé ; « Appliquer le fichier vérifié » l'applique après confirmation ; « Annuler la dernière application » rétablit l'état précédent. Nécessite les scopes `settings.read` et `settings.write` (délivrés à l'enregistrement). L'essai « ce qui change en 48 heures » s'exécute encore sur le serveur avec `info xlsx try-pending`.

