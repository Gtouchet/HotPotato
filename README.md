# HotPotato

## Composition du groupe

- FLEURET Nathan
- TOUCHET Guillaume
- ROUVILLE Quentin

## Organisation

Dans un premier temps, nous nous sommes concentré sur la connexion au serveur.
Cependant afin de gagner du temps nous nous sommes répartis en deux groupes :
- Le premier, composé de Guillaume et Nathan, est chargé d'établir la communication avec le serveur.
- Le deuxième, composé de Quentin, est chargé de la partie algorithmie pour résoudre les challenges.

Dans un second temps, Guillaume s'est occupé du MD5.

Une fois le travail terminé pour les deux groupes, nous avons rassemblé nos travaux et terminé ensemble les dernières tâches à réaliser (documentation, tests, gestion d'erreurs, etc.).

Nous avons donc réalisés le challenge MD5 avec gestion du timeout et le challenge Recover Secret

## Découpage du projet

La communication avec le serveur se découpe en plusieurs modules (ou fichiers) :
- `main.rs` : Ce module est chargé de récupérer l'adresse du serveur et d'établir la communication avec lui grâce aux autres modules
- `service.rs` : Ce module s'occupe d'envoyer et de recevoir les messages du serveur.
- `client.rs` : Ce module sert à faire le pont entre le module service et le module principal. Il met en forme les messages reçus et prépare les messages à envoyer.
- `messages.rs` : Ce module référence tous les messages que l'on envoie et reçoit du serveur. Cela est nécessaire pour parser automatiquement les messages.
- `random.rs` : Ce module sert à générer des nombres aléatoires pour le joueur.
- `challenge.rs` : Ce module contient le `trait` que doivent respecter les modules de résolution de challenge.
- `md5_resolver.rs` et `recover_secret.rs` : Ce sont les modules de résolution de challenge et sont chargés d'envoyer leur réponse au module client.

## Bonus réalisés
- CI permettant de jouer les tests unitaires

