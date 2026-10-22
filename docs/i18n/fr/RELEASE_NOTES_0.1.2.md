# INFO Desktop 0.1.2

Date : 2026-10-22

Nouveautés :
- bouton « Pourquoi dans ce canal ? » dans la fiche de l'élément : les décisions de tous les canaux pour l'événement et l'arbre de conditions XLSX du canal choisi (nécessite le serveur INFO `20261022-2010` ou plus récent) ;
- la version de l'application est désormais affichée dans l'en-tête ;
- les fichiers de sommes de contrôle (`SHA256SUMS-*.txt`) utilisent les mêmes noms que les fichiers de la version (sans espaces), donc `sha256sum -c` fonctionne par nom ;
- le serveur ne fournit des aperçus que pour les petites images (les vidéos et gros fichiers s'ouvrent via le lien de la source).

Inchangé : protocole `info-desktop-protocol/1`, enregistrement, sécurité. La publication depuis Desktop n'a pas été testée dans cette version.
