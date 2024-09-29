1. Mise en place de l'environnement de développement
   - [x] Installer Rust et les outils nécessaires (Cargo, etc.)
   - [x] Configurer un éditeur de code avec les extensions Rust
   - [x] Créer un nouveau projet Rust avec Cargo

2. Développement du cœur du bot Discord
   - [x] Configurer le bot Discord et obtenir un token
   - [x] Implémenter la connexion de base au serveur Discord
   - [ ] Créer une structure de commandes simple

3. Mise en place de la base de données
   - [ ] Configurer SQLite pour le projet
   - [ ] Créer le schéma de base de données (tables Users et Movies)
   - [ ] Implémenter les fonctions CRUD de base pour interagir avec la BDD

4. Développement des fonctionnalités de base
   - [ ] Implémenter la commande d'ajout de film
   - [ ] Implémenter la commande de liste des films
   - [ ] Implémenter la commande de détails d'un film

5. Intégration de l'API TMDB
   - [ ] Configurer l'accès à l'API TMDB
   - [ ] Implémenter la vérification des films via TMDB
   - [ ] Intégrer la récupération des informations de films

6. Développement des fonctionnalités avancées
   - [ ] Implémenter le système de notation
   - [ ] Développer les fonctionnalités de statistiques
   - [ ] Créer le système de recommandation

7. Amélioration de l'interface utilisateur
   - [ ] Formater les messages de sortie pour une meilleure lisibilité
   - [ ] Implémenter l'utilisation des embeds Discord
   - [ ] Ajouter des émojis et du formatage de texte

8. Tests et débogage
   - [ ] Écrire des tests unitaires pour les fonctions clés
   - [ ] Réaliser des tests d'intégration
   - [ ] Effectuer des tests manuels approfondis

9. Dockerisation de l'application
   - [ ] Créer un Dockerfile pour l'application
   - [ ] Construire et tester l'image Docker localement
   - [ ] Optimiser l'image Docker pour la production

10. Mise en place de l'infrastructure Kubernetes
    - [ ] Configurer un cluster Kubernetes local (comme Minikube) pour les tests
    - [ ] Créer les fichiers de configuration Kubernetes (Deployments, Services)
    - [ ] Tester le déploiement sur le cluster local

11. Configuration de la persistance des données
    - [ ] Configurer un volume persistant pour la base de données SQLite
    - [ ] Tester la persistance des données lors des redémarrages de pods

12. Mise en place du monitoring et des logs
    - [ ] Configurer la journalisation des événements du bot
    - [ ] Mettre en place un système de monitoring basique

13. Préparation pour le déploiement en production
    - [ ] Choisir un fournisseur de cloud pour Kubernetes (GKE, EKS, etc.)
    - [ ] Configurer le cluster de production
    - [ ] Mettre en place un pipeline CI/CD pour automatiser le déploiement

14. Documentation
    - [ ] Écrire la documentation technique du projet
    - [ ] Créer un guide d'utilisation pour les utilisateurs finaux
    - [ ] Documenter le processus de déploiement et de maintenance

15. Déploiement final et tests de charge
    - [ ] Déployer l'application sur le cluster de production
    - [ ] Effectuer des tests de charge pour vérifier les performances
    - [ ] Ajuster les ressources Kubernetes si nécessaire

16. Maintenance et itérations
    - [ ] Collecter les retours des utilisateurs
    - [ ] Planifier et implémenter des améliorations futures
    - [ ] Maintenir à jour les dépendances et l'infrastructure
