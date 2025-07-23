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

### ⚙️ Compilation et exécution

| Commande      | Action                                   |
|---------------|------------------------------------------|
| `cargo build` | Compile le projet                        |
| `cargo run`   | Compile et exécute le projet             |

## 3. Structure de Base d’un Projet Cargo

```txt
mon_projet/
 ├─ Cargo.toml       # Configuration et dépendances
 └─ src/
     └─ main.rs      # Code source principal
```

## 4. Ajouter des Dépendances

Ajoute des bibliothèques externes dans la section `[dependencies]` du fichier `Cargo.toml` :

```toml
[dependencies]
regex = "1.8.4"
```

Dans ton code :

```rust
use regex::Regex;
```
_`cargo build` ou `cargo run` téléchargera et utilisera automatiquement les dépendances._

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

Regroupe tes tests dans des modules dédiés :

```rust
#[cfg(test)]
mod tests {
    // Tes tests ici...
}
```

Pour lancer tous les tests du projet :

```sh
cargo test
```

## 7. Bonnes Pratiques d’Organisation

- Structure le code via plusieurs fichiers dans `src/`
- Centralise toutes les dépendances et versions dans `Cargo.toml`
- Utilise :
  - `cargo run` pour le développement rapide
  - `cargo build --release` pour un binaire optimisé

Pour aller plus loin :
- `cargo doc` : génère la documentation
- `cargo check` : vérifie rapidement le code sans générer de binaire
- `cargo new --lib nom_librairie` : crée une bibliothèque réutilisable

## 🧠 Notions fondamentales : **Ownership** & **Membership**

### ⚡ Ownership (propriété)

- **Chaque valeur a un propriétaire unique** (une variable, une structure, etc).
- Quand le propriétaire sort du scope (fin de bloc `{}`), la mémoire est libérée automatiquement : _pas de garbage collector_ !
- Si on affecte la variable à une fonction ou une autre variable **sans `.clone()`**, l’ownership est déplacé. L’ancien propriétaire ne peut plus y accéder.

**Exemple simple :**

```rust
fn greetings(msg: String) {
    println!("Bonjour {}", msg);
}

fn main() {
    let prenom = String::from("Amine"); // `prenom` possède la String
    let secu = String::from("198977787");
    
    let prenom2 = prenom.clone(); // ici on clone, donc deux propriétaires
    greetings(prenom); // ownership transféré à la fonction
    
    // println!("{}", prenom); // ERREUR ! Ownership déplacé

    println!("clone de prenom : {}", prenom2);

    greetings2(&secu); // emprunt immuable (read-only)
    println!("{}", secu); // OK ! encore accessible (non transféré)
}

fn greetings2(msg: &String) {
    println!("Bonjour {}", msg);
}
```

### 🧩 Membership (appartenance à une structure)

- **Membership** décrit les valeurs appartenant à une structure (`struct`).
- La **struct** est propriétaire de ses champs : 
- Exemple :

```rust
struct User {
    nom: String,
    secu: String,
}

fn display(u: User) -> User {
    println!("Nom {}, secu : {}", u.nom, u.secu);
    u // On rend ownership à l'appelant
}

fn main() {
    let user = User {
        nom: String::from("Amine"),
        secu: String::from("187971617100955"),
    };

    display(user); // ownership de `user` transféré à `display`
    // println!("{}", user.nom); // ERREUR ! Ownership déplacé
}
```

## 🏷️ Getters et Setters en Rust

En Rust, il n’est **pas recommandé** d’exposer les champs de struct directement avec `pub`.  
On préfère utiliser des **méthodes getters (pour lire)** et **setters (pour modifier)**, qui permettent de contrôler l’accès et la modification.

### Exemples de getter et setter idiomatiques

```rust
struct CompteBancaire {
    solde: f64,
}

impl CompteBancaire {
    // Getter (lecture, emprunt immuable)
    pub fn get_solde(&self) -> f64 {
        self.solde
    }

    // Setter (modification, emprunt mutable)
    pub fn set_solde(&mut self, nouveau_solde: f64) {
        self.solde = nouveau_solde;
    }
}

fn main() {
    let mut compte = CompteBancaire { solde: 1500.0 };

    // Lire le solde via le getter
    println!("Solde actuel : {}", compte.get_solde());

    // Modifier le solde via le setter
    compte.set_solde(2100.50);
    println!("Solde modifié : {}", compte.get_solde());
}
```

#### Remarques importantes :
- **Getter** : signe `&self` permet de lire sans modifier l’objet.
- **Setter** : signe `&mut self` permet de modifier l’objet (l’instance doit être mutable).
- **Rust** préfère les méthodes explicites plutôt que les champs publics, car cela permet d’encadrer les accès/modifications et d’éviter des usages involontaires ou non sécurisés.
- Pour les structures complexes, tu peux générer ces méthodes automatiquement avec une macro (par exemple la crate [getset]), mais à la main cela reste formateur.

## 🌟 Résumé

- **Cargo** structure les projets Rust et automatise la gestion des dépendances, de la compilation, des tests et de la documentation.
- **Ownership** : principe fondamental pour la sécurité mémoire, chaque valeur a un seul propriétaire.
- **Membership** : relation entre une struct et ses champs (leurs propriétaires).
- Pour chaque nouvelle bibliothèque : ajoute-la dans `Cargo.toml`, utilise-la dans le code — Cargo s’occupe du reste !  
