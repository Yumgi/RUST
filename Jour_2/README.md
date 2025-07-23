# Formation Rust - Notes de cours

## 📚 Ressources
- Documentation officielle : [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

## Débuter un projet Rust avec Cargo

## 1. Présentation de Cargo

Cargo est l’outil officiel pour gérer les projets Rust : il s’occupe de la compilation, des dépendances, des tests et de l'organisation du code.

Un projet standard Cargo est composé d’un fichier de configuration (Cargo.toml) et d’un dossier source (src/), où se trouve le point d’entrée (main.rs).

## 2. Créer et gérer un projet
### Créer un nouveau projet
```text
cargo new mon_projet
cd mon_projet
```

Cela crée :

- un dossier mon_projet/
- un fichier Cargo.toml pour la configuration
- le dossier src/ contenant le fichier ```main.rs```

### Compiler et exécuter
```text
cargo build   # compile le projet
cargo run     # compile et exécute le projet directement
```

## 3. Structure de base d’un projet Cargo
```text
mon_projet/
 ├─ Cargo.toml
 └─ src/
     └─ main.rs
```
- Cargo.toml : fichier où on déclare le nom du projet, la version, les dépendances, etc.
- src/main.rs : fichier source principal pour l’exécutable.

## 4. Ajouter des dépendances
Pour utiliser une bibliothèque externe, édite la section [dependencies] de Cargo.toml :
```text
[dependencies]
regex = "1.8.4"
```
Ensuite, tu peux utiliser la bibliothèque dans ton code :
```rust
use regex::Regex;
```
Recompile avec cargo build ou cargo run – Cargo gère automatiquement les téléchargements.

## 5. Ajouter et organiser des modules
- Ajouter un fichier module (ex : src/util.rs)
- Dans main.rs, déclarer le module :
```rust
mod util;
```
Les fonctions du module peuvent être appelées comme : util::ma_fonction();

## 6. Exécuter des tests
- Écrire des tests dans des modules marqués par #[cfg(test)].
- Exécuter :
```text
cargo test
```

## 7. Bonnes pratiques d'organisation
- Un projet peut contenir plusieurs fichiers dans src/ pour structurer les fonctions et types.
- Les dépendances, versions et métadonnées sont centralisées dans Cargo.toml.
- Utiliser cargo run en développement, cargo build --release pour les binaires optimisés.
- Cargo rend la gestion des projets Rust ergonomique et fiable, en automatisant l’organisation, l’intégration de bibliothèques et la compilation.
- Pour aller plus loin, explore :
      -- cargo doc (documentation)
      -- cargo check
      -- création de librairies Rust avec cargo new --lib nom_librairie
























