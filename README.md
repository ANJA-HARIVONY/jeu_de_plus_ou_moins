# Jeu de plus ou moins

Un jeu de devinette en Rust : trouvez le nombre secret entre 1 et 100.

## Règles

- L’ordinateur choisit un nombre entier entre 1 et 100.
- Vous proposez un nombre.
- L’ordinateur répond **Plus !** si le nombre secret est plus grand, **Moins** s’il est plus petit.
- Vous continuez jusqu’à trouver le bon nombre.

## Prérequis

- [Rust](https://www.rust-lang.org/) (rustc, cargo)

## Installation et exécution

```bash
# Cloner le dépôt (ou se placer dans le dossier du projet)
cd jeu_de_plus_ou_moins

# Lancer le jeu
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
