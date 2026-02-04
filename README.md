# 🎲 Jeu de Plus ou Moins - Application Desktop

Une application desktop en Rust avec interface graphique : trouvez le nombre secret entre 1 et 100 !

![Rust](https://img.shields.io/badge/Rust-1.70+-orange)
![egui](https://img.shields.io/badge/GUI-egui-blue)

## 📋 Règles du Jeu

- L'ordinateur choisit un nombre entier entre 1 et 100
- Vous proposez un nombre
- L'ordinateur répond **Plus !** 📈 si le nombre secret est plus grand, **Moins !** 📉 s'il est plus petit
- Vous continuez jusqu'à trouver le bon nombre
- Tentez de gagner en un minimum de coups !

## ✨ Fonctionnalités

- 🖥️ Interface graphique moderne et intuitive
- 📊 Historique des tentatives
- 🎯 Compteur de tentatives
- 🔄 Bouton pour recommencer une partie
- ✅ Validation par clic ou touche Entrée
- 📜 Instructions intégrées

## 🔧 Prérequis

- [Git](https://git-scm.com/)
- [Rust](https://www.rust-lang.org/) (rustc, cargo) version 1.70 ou supérieure

## 🚀 Installation et Lancement

### Cloner le projet

```bash
# Cloner le dépôt
git clone https://github.com/VOTRE_UTILISATEUR/jeu_de_plus_ou_moins.git

# Entrer dans le dossier du projet
cd jeu_de_plus_ou_moins
```

### Lancer l'application

```bash
# Télécharger les dépendances et lancer le jeu
cargo run
```

### Compiler en mode release (optimisé)

```bash
cargo build --release
```

L'exécutable optimisé sera disponible dans `target/release/jeu_de_plus_ou_moins.exe` (Windows) ou `target/release/jeu_de_plus_ou_moins` (Linux/macOS).

## 🎮 Utilisation

1. **Lancer l'application** : L'interface graphique s'ouvre automatiquement
2. **Entrer un nombre** : Tapez votre supposition dans le champ de texte (entre 1 et 100)
3. **Valider** : Cliquez sur "✓ Valider" ou appuyez sur Entrée
4. **Suivre les indices** : Le jeu vous indique si le nombre secret est plus grand (Plus !) ou plus petit (Moins !)
5. **Nouvelle partie** : Cliquez sur "🔄 Nouvelle Partie" pour recommencer

## 📦 Dépendances

- **rand** (0.8.3) : génération du nombre secret aléatoire
- **eframe** (0.30.0) : framework pour créer des applications desktop
- **egui** (0.30.0) : bibliothèque d'interface graphique immédiate

## 🏗️ Structure du Projet

```
jeu_de_plus_ou_moins/
├── src/
│   └── main.rs          # Code principal de l'application
├── Cargo.toml           # Configuration et dépendances
├── Cargo.lock           # Verrouillage des versions
└── README.md            # Ce fichier
```

## 🛠️ Technologies Utilisées

- **Langage** : Rust
- **GUI Framework** : egui (Immediate Mode GUI)
- **Random** : rand crate

## 📸 Capture d'écran

L'interface comprend :
- Un titre avec emoji 🎲
- Un message d'indication (Plus/Moins/Gagné)
- Un champ de saisie avec validation
- Un compteur de tentatives
- Un historique déroulant des essais
- Des instructions pliables

## 🌐 Publier sur GitHub

1. **Créer un dépôt** sur [GitHub](https://github.com/new) (nom : `jeu_de_plus_ou_moins` ou autre), sans initialiser avec un README.

2. **Dans le dossier du projet**, exécuter :

```bash
git add .
git commit -m "Application desktop : jeu de plus ou moins avec interface graphique"
git branch -M main
git remote add origin https://github.com/VOTRE_UTILISATEUR/jeu_de_plus_ou_moins.git
git push -u origin main
```

Remplacez `VOTRE_UTILISATEUR` par votre nom d'utilisateur GitHub.

## 📝 Licence

Projet d'apprentissage Rust.

## 🤝 Contribution

Les contributions sont les bienvenues ! N'hésitez pas à ouvrir une issue ou une pull request.

---

Développé avec ❤️ en Rust
