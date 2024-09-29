# Cahier des charges complet : Bot Discord de gestion de cinémathèque personnelle avec Docker et Kubernetes

## 1. Présentation du projet

### 1.1 Objectif
Développer un bot Discord permettant aux utilisateurs de gérer leur collection de films personnelle, incluant un système de notation, directement depuis un serveur Discord. Le projet sera containerisé avec Docker et déployé sur Kubernetes pour assurer sa scalabilité et sa facilité de déploiement.

### 1.2 Public cible
Cinéphiles actifs sur Discord souhaitant organiser, suivre et noter leurs visionnages de films.

## 2. Fonctionnalités

### 2.1 Gestion des films
- Ajouter un film à sa collection
- Supprimer un film de sa collection
- Modifier les informations d'un film
- Lister les films de sa collection
- Rechercher un film dans sa collection
- Afficher les détails d'un film

### 2.2 Système de notation et suivi
- Marquer un film comme vu/non vu
- Noter un film (échelle de 1 à 10 étoiles)
- Ajouter/modifier un commentaire personnel
- Ajouter un film à une liste "À voir"

### 2.3 Statistiques
- Afficher le nombre total de films dans la collection
- Afficher le nombre de films vus/non vus
- Afficher la moyenne des notes
- Afficher les films les mieux notés

### 2.4 Vérification des films dans une base de données en ligne
- Vérifier l'existence d'un film dans une base de données en ligne (TMDB)
- Récupérer automatiquement les informations du film si trouvé
- Permettre à l'utilisateur de choisir entre plusieurs résultats si ambiguïté

### 2.5 Recommandations
- Suggérer un film aléatoire de la collection à regarder
- Recommander des films basés sur les genres préférés de l'utilisateur

## 3. Spécifications techniques

### 3.1 Langage et framework
- Langage : Rust
- Framework Discord : Serenity

### 3.2 Base de données
- SQLite pour la persistance des données

### 3.3 Structure des données
- Table Users : id, discord_id
- Table Movies : id, user_id, title, director, release_year, genre, status, rating, comment, tmdb_id

### 3.4 Intégration API externe
- Utilisation de l'API TMDB (The Movie Database) pour la vérification et la récupération des informations des films

### 3.5 Containerisation
- Docker pour la containerisation de l'application

### 3.6 Orchestration
- Kubernetes pour le déploiement, la mise à l'échelle et la gestion de l'application

### 3.7 Commandes du bot et leur fonctionnement détaillé

1. `!add <titre> | <réalisateur> | <année> | <genre>`
   - Description : Ajoute un nouveau film à la collection de l'utilisateur.
   - Fonctionnement :
     a. Vérifie si le film existe dans la base TMDB.
     b. Si trouvé, récupère automatiquement les informations.
     c. Si plusieurs résultats, demande à l'utilisateur de choisir.
     d. Si non trouvé ou si l'utilisateur le souhaite, ajoute manuellement les informations fournies.
   - Exemple : `!add The Matrix | Wachowski Sisters | 1999 | Science Fiction`

2. `!remove <titre>`
   - Description : Supprime un film de la collection de l'utilisateur.
   - Fonctionnement :
     a. Recherche le film dans la collection de l'utilisateur.
     b. Si trouvé, demande confirmation avant de supprimer.
     c. Si non trouvé, affiche un message d'erreur.
   - Exemple : `!remove The Matrix`

3. `!edit <titre> <champ> <nouvelle valeur>`
   - Description : Modifie les informations d'un film existant.
   - Fonctionnement :
     a. Recherche le film dans la collection de l'utilisateur.
     b. Si trouvé, met à jour le champ spécifié avec la nouvelle valeur.
     c. Si non trouvé, affiche un message d'erreur.
   - Exemple : `!edit The Matrix genre "Science Fiction, Action"`

4. `!list [page]`
   - Description : Affiche la liste des films de l'utilisateur, paginée par 10 films.
   - Fonctionnement :
     a. Récupère tous les films de l'utilisateur.
     b. Les trie par ordre alphabétique.
     c. Affiche la page demandée (par défaut, la première page).
   - Exemple : `!list 2`

5. `!search <terme>`
   - Description : Recherche un film dans la collection de l'utilisateur.
   - Fonctionnement :
     a. Recherche le terme dans les titres, réalisateurs et genres des films de l'utilisateur.
     b. Affiche tous les résultats correspondants.
   - Exemple : `!search Matrix`

6. `!details <titre>`
   - Description : Affiche les détails complets d'un film.
   - Fonctionnement :
     a. Recherche le film dans la collection de l'utilisateur.
     b. Si trouvé, affiche toutes les informations disponibles.
     c. Si non trouvé, affiche un message d'erreur.
   - Exemple : `!details The Matrix`

7. `!markwatched <titre>`
   - Description : Marque un film comme vu.
   - Fonctionnement :
     a. Recherche le film dans la collection de l'utilisateur.
     b. Si trouvé, change son statut à "vu".
     c. Si non trouvé, affiche un message d'erreur.
   - Exemple : `!markwatched The Matrix`

8. `!rate <titre> <note>`
   - Description : Attribue une note à un film (de 1 à 10).
   - Fonctionnement :
     a. Recherche le film dans la collection de l'utilisateur.
     b. Si trouvé, enregistre la note donnée.
     c. Vérifie que la note est entre 1 et 10.
   - Exemple : `!rate The Matrix 9`

9. `!comment <titre> <commentaire>`
   - Description : Ajoute ou modifie un commentaire pour un film.
   - Fonctionnement :
     a. Recherche le film dans la collection de l'utilisateur.
     b. Si trouvé, enregistre ou met à jour le commentaire.
   - Exemple : `!comment The Matrix "Un classique incontournable de la SF"`

10. `!addtowatchlist <titre>`
    - Description : Ajoute un film à la liste "À voir".
    - Fonctionnement :
      a. Vérifie si le film existe dans la base TMDB.
      b. Si trouvé, ajoute à la liste "À voir" avec les infos de TMDB.
      c. Si non trouvé, demande à l'utilisateur de fournir les informations manuellement.
    - Exemple : `!addtowatchlist Inception`

11. `!stats`
    - Description : Affiche les statistiques de la collection de l'utilisateur.
    - Fonctionnement :
      a. Calcule le nombre total de films, films vus, films à voir.
      b. Calcule la note moyenne de tous les films notés.
      c. Détermine les genres les plus présents dans la collection.
    - Exemple : `!stats`

12. `!recommend`
    - Description : Recommande un film de la liste "À voir" ou un film aléatoire de la collection.
    - Fonctionnement :
      a. Vérifie d'abord la liste "À voir".
      b. Si vide, sélectionne un film aléatoire de la collection.
      c. Prend en compte les genres préférés de l'utilisateur basés sur ses notes.
    - Exemple : `!recommend`

13. `!toprated`
    - Description : Affiche les films les mieux notés de la collection.
    - Fonctionnement :
      a. Trie les films par note décroissante.
      b. Affiche les 10 premiers films avec leur note.
    - Exemple : `!toprated`

14. `!verify <titre>`
    - Description : Vérifie l'existence d'un film dans la base TMDB.
    - Fonctionnement :
      a. Recherche le titre dans la base TMDB.
      b. Affiche les résultats trouvés avec leurs détails.
      c. Si plusieurs résultats, permet à l'utilisateur de choisir le bon.
    - Exemple : `!verify The Matrix`


## 4. Sécurité et gestion des erreurs

### 4.1 Authentification
- Vérification que l'utilisateur n'interagit qu'avec sa propre collection

### 4.2 Validation des entrées
- Vérification et nettoyage de toutes les entrées utilisateur
- Validation des notes (1-10)

### 4.3 Gestion des erreurs
- Messages d'erreur clairs pour l'utilisateur
- Journalisation des erreurs côté serveur

### 4.4 Sécurité des conteneurs
- Utilisation d'images de base minimales et sécurisées
- Scan des vulnérabilités des conteneurs

## 5. Performance

### 5.1 Temps de réponse
- Réponse du bot en moins de 2 secondes pour toutes les commandes

### 5.2 Charge
- Support jusqu'à 1000 utilisateurs actifs simultanément
- Capacité à scaler horizontalement grâce à Kubernetes

## 6. Évolutivité

### 6.1 Futures fonctionnalités potentielles
- Intégration avec des APIs de films (ex: TMDB, IMDB)
- Exportation de la collection en format CSV
- Système de tags personnalisés
- Création de listes thématiques (ex: films d'horreur, comédies romantiques)

### 6.2 Scalabilité
- Utilisation de Kubernetes pour faciliter la mise à l'échelle de l'application

## 7. Tests

### 7.1 Tests unitaires
- Couverture de code > 80%

### 7.2 Tests d'intégration
- Scénarios de test pour chaque commande

### 7.3 Tests de charge
- Simulation de multiples requêtes simultanées

### 7.4 Tests de conteneurs
- Vérification de la bonne construction et du bon fonctionnement des conteneurs Docker

## 8. Documentation

### 8.1 Documentation technique
- Commentaires dans le code
- README détaillé pour l'installation et la configuration
- Documentation sur la structure des conteneurs Docker et le déploiement Kubernetes

### 8.2 Documentation utilisateur
- Guide d'utilisation avec exemples pour chaque commande

## 9. Déploiement

### 9.1 Containerisation avec Docker
- Création d'un Dockerfile pour l'application
- Construction de l'image Docker
- Publication de l'image sur un registre de conteneurs (ex: Docker Hub)

### 9.2 Déploiement sur Kubernetes
- Création des fichiers de configuration Kubernetes (Deployments, Services, etc.)
- Configuration des ressources nécessaires (CPU, mémoire)
- Mise en place de la persistance des données pour la base SQLite

### 9.3 Environnement
- Déploiement sur un cluster Kubernetes (ex: Google Kubernetes Engine, Amazon EKS)

### 9.4 Monitoring
- Mise en place de logs pour surveiller l'activité du bot
- Intégration avec des outils de monitoring Kubernetes (ex: Prometheus, Grafana)

## 10. Maintenance

### 10.1 Mises à jour
- Plan pour les mises à jour de sécurité et de fonctionnalités
- Stratégie de déploiement continu avec Kubernetes (ex: rolling updates)

### 10.2 Sauvegarde
- Système de sauvegarde quotidienne de la base de données
- Utilisation de volumes persistants Kubernetes pour les données

### 10.3 Gestion des conteneurs
- Stratégie de mise à jour des images Docker
- Gestion des versions des conteneurs

## 11. Calendrier prévisionnel

- Phase 1 (2 semaines) : Mise en place de la structure de base et des commandes principales
- Phase 2 (1 semaine) : Implémentation de la base de données et de la persistance
- Phase 3 (1 semaine) : Ajout du système de notation et des fonctionnalités avancées (stats, recommandations)
- Phase 4 (1 semaine) : Tests et débogage
- Phase 5 (1 semaine) : Containerisation avec Docker
- Phase 6 (1 semaine) : Configuration et déploiement sur Kubernetes
- Phase 7 (1 semaine) : Tests d'intégration, de charge et optimisation
- Phase 8 (1 semaine) : Documentation et finalisation

Durée totale estimée : 9 semaines