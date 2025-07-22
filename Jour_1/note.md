# Formation Rust - Notes de cours

## 📚 Ressources
- Documentation officielle : [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

## 1. Variables en Rust

### Déclaration de variables
```rust
use std::io; // Bibliothèque de stream input/output

let nom = "Erwan";                    // String slice
let _age: u32 = 23;                   // Entier non signé sur 32 bits
let age_chien = 20;                   // i32 par défaut (inférence de type)
let temperature: f32 = 21.5;          // Nombre flottant
```

### Types numériques en Rust
| Type | Description | Plage de valeurs |
|------|-------------|------------------|
| `i32` | Entier signé | -2³¹ à +2³¹-1 |
| `u32` | Entier non signé | 0 à 2³²-1 |
| `i64` | Entier signé 64 bits | Très grande plage |
| `u8` | Entier non signé 8 bits | 0 à 255 |

### Conventions de nommage
- ✅ Utiliser `snake_case` (convention Rust)
- ❌ Pas de chiffres en début de nom
- ❌ Pas d'espaces ni de tirets

### Affichage avec println!
```rust
println!("Hello la température à Lyon {}", temperature);
println!("Le chien de Lyon a {}", age_chien);
println!("Hello {}, la température à Lyon est {}", nom, temperature);
```

## 2. Fonctions

### Syntaxe de base
```rust
fn nom_fonction(parametre: type) -> type_retour {
    // corps de la fonction
    valeur_retour // sans point-virgule pour le retour
}
```

### Exemples
```rust
// Fonction avec retour
fn addition(a: i32, b: i32) -> i32 {
    a + b  // Pas de point-virgule = valeur de retour
}

// Fonction sans retour
fn say_hello(nom: &str) {
    println!("Bonjour, {}", nom);
}

// Utilisation
let resultat = addition(12, 3);
say_hello("Salma");
```

## 3. Structures de contrôle

### Conditions
```rust
let nombre = 16;
if nombre % 2 == 0 {
    println!("{} est un nombre pair", nombre);
} else {
    println!("Nombre impair");
}
```

### Boucles

#### Boucle for avec intervalles
```rust
// Intervalle inclusif (1 à 10 inclus)
for i in 1..=10 {
    println!("i vaut {}", i);
}

// Intervalle exclusif (1 à 9)
for j in 1..10 {
    println!("j vaut {}", j);
}
```

## 4. Tableaux et collections

### Tableaux statiques
```rust
let voitures = ["citroen", "renault", "jeep"];

// Accès par index
println!("{}", voitures[1]);

// Itération simple
for voiture in voitures {
    println!("Voiture : {}", voiture);
}

// Itération avec index
for (i, voiture) in voitures.iter().enumerate() {
    println!("Index {} : {}", i, voiture);
}
```

### Vecteurs (tableaux dynamiques)
```rust
let noms = vec![
    String::from("Erwan"),
    String::from("Salma"),
    String::from("Pierre Emmanuel")
];

for (i, nom) in noms.iter().enumerate() {
    println!("Nom {} : {}", i, nom);
}
```

## 5. Gestion des entrées utilisateur

### Exemple pratique : Menu interactif
```rust
let options = ["Afficher solde", "Retrait", "Liste des comptes", "Quitter"];

println!("Menu");
for (i, option) in options.iter().enumerate() {
    println!("{}. {}", i + 1, option);
}

println!("Veuillez saisir un numéro de votre choix");
let mut choix = String::new();

// Lecture de l'entrée utilisateur
io::stdin().read_line(&mut choix).expect("Erreur de lecture");

// Conversion et gestion d'erreur
let choix: usize = match choix.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        println!("Veuillez saisir un numéro valide");
        return;
    }
};

// Validation du choix
if choix == 0 || choix > options.len() {
    println!("Choix hors système");
} else {
    println!("Vous avez choisi : {}", options[choix - 1]);
}
```

## 🔑 Points clés à retenir

1. **Sécurité de type** : Rust vérifie les types à la compilation
2. **Inférence de type** : Rust peut déduire le type automatiquement
3. **Gestion d'erreur** : Utilisation de `match` pour gérer les erreurs
4. **Performance** : Pas de garbage collector, gestion mémoire à la compilation
5. **Expressivité** : Syntaxe claire et concise

## 📝 Comparaison avec d'autres langages

### C/C++
```cpp
// C++
public int addition(int a, int b) {
    return a + b;
}
```

### Rust
```rust
// Rust
fn addition(a: i32, b: i32) -> i32 {
    a + b  // Pas de return explicite ni de point-virgule
}
```

## 🚀 Prochaines étapes
- Ownership et borrowing
- Structures et enums
- Gestion d'erreur avancée avec `Result<T, E>`
- Traits et génériques
- Modules et packages
