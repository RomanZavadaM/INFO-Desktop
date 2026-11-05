# Mode console d'INFO Desktop : manuel court

État : publié dans 0.1.5. Deux modes : via la fenêtre ouverte de l'application et de manière autonome.

## Ce que c'est
Le programme `info-desktop-cli` fait ce que fait la fenêtre : affiche la file d'attente, explique les routes, crée et modifie des éléments, publie, travaille avec les paramètres XLSX. La clé de l'appareil provient du même magasin sécurisé du système que celle de la fenêtre et n'est jamais affichée.

## Modes
- **Via la fenêtre ouverte (connexion partagée).** Dans la fenêtre, cochez « Autoriser la console à passer par cette fenêtre » (désactivé par défaut). Tant que la fenêtre est ouverte, la console utilise sa connexion : VPN et clé sont déjà configurés et la clé n'est jamais transmise. L'entrée est locale à cet ordinateur (127.0.0.1) et protégée par un mot de passe de session à usage unique stocké dans le dossier de paramètres de l'utilisateur. La fenêtre affiche chaque action de la console à côté de la case.
- **Autonome.** Pas besoin de fenêtre : la console se connecte seule au serveur avec le profil de connexion et la clé du magasin du système. La fenêtre écrit le profil à chaque connexion (adresse, port, numéro d'appareil, certificat du serveur ; sans clés) ; il peut aussi être défini à la main (`profile set`).
- Choix : `--mode auto` (par défaut : la fenêtre si elle est ouverte et l'autorise, sinon autonome), `--mode app` (fenêtre seulement), `--mode alone` (autonome seulement). Un `--profile` explicite sans `--mode` signifie autonome.

## Premier lancement
1. Connectez-vous dans la fenêtre comme d'habitude (sous Windows avec notre serveur, le VPN doit être actif).
2. La fenêtre écrit le profil elle-même. Seulement si nécessaire à la main : `info-desktop-cli profile set --server ADRESSE --port PORT --device-id NUMÉRO --ca-file FICHIER` ; le fichier contient la valeur `server_ca_der` du fichier d'enregistrement.
3. Vérifiez la connexion : `info-desktop-cli status`.

## Commandes
`info-desktop-cli` sans arguments affiche l'aide complète.

| Tâche | Commande |
|---|---|
| Liste des canaux | `channels` |
| File d'attente | `queue list [--limit N] [--cursor N] [--channel CODE] [--bucket GROUPE]` |
| Élément avec sources | `queue show ID` |
| Pourquoi dans ce canal | `why ID --channel CODE [--scope route\|auto\|digest\|instant\|relevance]` |
| Créer un élément | `create --channel CODE --title TEXTE (--text TEXTE \| --text-file FICHIER) [--url URL]` |
| Modifier un élément | `save ID --title TEXTE (--text TEXTE \| --text-file FICHIER) [--channel CODE]` |
| Ignorer, reporter, publier | `skip ID`, `defer ID [--minutes N]`, `publish ID` |
| Paramètres XLSX | `settings status`, `settings download --out FICHIER`, `settings check FICHIER`, `settings probe [--hours N]`, `settings probe-status`, `settings apply [--without-probe] [--yes]`, `settings rollback [--yes]` |
| Thèmes et mots | `themes list`, `themes test ...`, `themes save ...` |

## Modifier les paramètres
D'abord `settings check` (vérifie le fichier sans l'appliquer), puis `settings probe` et `settings probe-status` (« ce qui va changer »), puis `settings apply`. Sans `--yes`, la commande montre seulement ce qui serait appliqué. Le fichier vérifié exact est appliqué (par empreinte). Sans essai la commande refuse, sauf avec `--without-probe`. Annulation : `settings rollback --yes`.

La publication (`publish`) et les actions éditoriales ordinaires n'exigent pas de confirmation : à la discrétion et sous la responsabilité de l'exécutant ; une publication peut toujours être supprimée dans le canal.

## Codes de sortie
0 succès, 1 erreur de commande, 2 arguments incorrects, 3 pas de connexion ou de clé, 4 refus du serveur (pas de droits, conflit).

## Pas encore fait
Fonctionnement sur un serveur sans session graphique (reporté par le propriétaire). Vérifié sous Windows le 5 novembre 2026 avec un serveur réel (lecture) : `status`, `channels`, `queue list`, `why` via la fenêtre, `status` autonome et automatique. Non vérifié : commandes d'écriture, XLSX et thèmes, macOS et Linux.
