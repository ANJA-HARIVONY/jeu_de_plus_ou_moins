# Jeu de plus ou moins

Un jeu de devinette en Rust : trouvez le nombre secret entre 1 et 100.

## Règles

- L’ordinateur choisit un nombre entier entre 1 et 100.
- Vous proposez un nombre.
- L’ordinateur répond **Plus !** si le nombre secret est plus grand, **Moins** s’il est plus petit.
- Vous continuez jusqu’à trouver le bon nombre.

## Prérequis

- [Git](https://git-scm.com/)
- [Rust](https://www.rust-lang.org/) (rustc, cargo)

## Cloner le projet

```bash
# Cloner le dépôt (remplacez par l’URL de votre dépôt GitHub)
git clone https://github.com/VOTRE_UTILISATEUR/jeu_de_plus_ou_moins.git

# Entrer dans le dossier du projet
cd jeu_de_plus_ou_moins

# Télécharger les dépendances et lancer le jeu
cargo run
```

Pour **compiler** sans lancer le jeu : `cargo build`. L’exécutable sera dans `target/debug/jeu_de_plus_ou_moins` (ou `jeu_de_plus_ou_moins.exe` sous Windows).

## Lancer le jeu

Depuis le dossier du projet :

```bash
cargo run
```

## Exemple de partie

```
Devinez le nombre !
Veuillez entrer un nombre.
50
Plus!
Veuillez entrer un nombre.
75
Moins
Veuillez entrer un nombre.
62
Gagné!
Vous avez gagné en 2 tentatives!
```

## Dépendances

- **rand** (0.8.3) : génération du nombre secret aléatoire

## Publier sur GitHub

1. **Créer un dépôt** sur [GitHub](https://github.com/new) (même nom : `jeu_de_plus_ou_moins` ou autre), sans initialiser avec un README.

2. **Dans le dossier du projet**, exécuter :

```bash
git add .
git commit -m "Premier commit : jeu de plus ou moins en Rust"
git branch -M main
git remote add origin https://github.com/VOTRE_UTILISATEUR/jeu_de_plus_ou_moins.git
git push -u origin main
```

Remplacez `VOTRE_UTILISATEUR` et `jeu_de_plus_ou_moins` par votre nom d’utilisateur GitHub et le nom du dépôt.

## Licence

Projet d’apprentissage Rust.
