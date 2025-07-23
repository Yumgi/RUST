# Formation Rust - Notes de cours

## 📚 Ressources
- Documentation officielle : [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

## 1. Présentation de **Cargo**

> **Cargo** est l’outil officiel qui facilite la création, la compilation, la gestion des dépendances, les tests et l’organisation des projets Rust.

Un projet classique avec Cargo contient :
- Un fichier **`Cargo.toml`** (configuration du projet)
- Un dossier **`src/`** avec le point d’entrée **`main.rs`**

## 2. Créer et Gérer un Projet

### ⚒️ Création d'un projet

```sh
cargo new mon_projet
cd mon_projet
```
Cela génère :
- `mon_projet/`
  - `Cargo.toml`
  - `src/main.rs`

### ⚙️ Compilations et exécutions

| Commande            | Action                                               |
|---------------------|------------------------------------------------------|
| `cargo build`       | Compile le projet                                    |
| `cargo run`         | Compile puis exécute le projet                       |

## 3. Structure de Base d’un Projet Cargo

```txt
mon_projet/
 ├─ Cargo.toml       # Configuration et dépendances
 └─ src/
     └─ main.rs      # Code source principal
```

## 4. Ajouter des Dépendances

> Ajoute des bibliothèques externes dans la section `[dependencies]` du fichier `Cargo.toml` :

```toml
[dependencies]
regex = "1.8.4"
```

📌 Utilisation dans le code :

```rust
use regex::Regex;
```
> *La commande `cargo build` ou `cargo run` téléchargera et utilisera automatiquement les dépendances.*

## 5. Organiser le Projet avec des Modules

- Crée un fichier module, par exemple `src/util.rs`
- Déclare-le dans `main.rs` :

```rust
mod util;
```

- Appelle les fonctions du module ainsi :

```rust
util::ma_fonction();
```

## 6. Exécuter les Tests

> Regroupe tes tests dans des modules dédiés :

```rust
#[cfg(test)]
mod tests {
    // Tes tests ici...
}
```

Lance tous les tests du projet :

```sh
cargo test
```

## 7. Bonnes Pratiques d’Organisation

- Structure le code via plusieurs fichiers dans `src/`
- Centralise toutes les dépendances et versions dans `Cargo.toml`
- Utilise :
  - `cargo run` pour le développement rapide
  - `cargo build --release` pour un binaire optimisé


Pour aller plus loin :

  cargo doc : génère la documentation du projet
  cargo check : vérifie rapidement le code sans générer de binaire
  cargo new --lib nom_librairie : crée une bibliothèque réutilisable



## 🌟 Résumé

- **Cargo** structure les projets Rust et automatise la gestion des dépendances, la compilation, les tests, et la documentation.
- L’organisation du code et l’utilisation de modules rend les projets lisibles et maintenables.
- Pour chaque nouvelle bibliothèque : ajoute-la dans `Cargo.toml`, utilise-la dans le code, et Cargo s’occupe du reste !























