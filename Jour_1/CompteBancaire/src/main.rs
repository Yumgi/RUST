use std::io::{self, Write};
use colored::*;  // pour colorer le texte

// 1. Création de la structure CompteBancaire
struct CompteBancaire {
    nom: String,
    prenom: String,
    age: u32,
    solde: f64,
    ouvert: bool, // indique si le compte est encore ouvert
}

// 2. Trait pour afficher des infos sur un compte
trait Afficher {
    fn afficher(&self);
}

// 3. Implémentation du trait Afficher pour la struct
impl Afficher for CompteBancaire {
    fn afficher(&self) {
        if self.ouvert {
            println!(
                "Compte de {} {} ({} ans) : {} €",
                self.prenom, self.nom, self.age, self.solde
            );
        } else {
            println!(
                "Compte de {} {} est fermé (solde : {} €)",
                self.prenom, self.nom, self.solde
            );
        }
    }
}

// 4. Fonctions pour gérer les opérations bancaires (avec messages colorés)
impl CompteBancaire {
    fn deposer(&mut self, montant: f64) {
        if self.ouvert {
            self.solde += montant;
            println!("{}", format!("+{} € déposés sur le compte de {}", montant, self.nom).green());
        } else {
            println!("{}", "Impossible de déposer: le compte est fermé !".red());
        }
    }
    fn retirer(&mut self, montant: f64) {
        if !self.ouvert {
            println!("{}", "Impossible de retirer: le compte est fermé !".red());
        } else if self.solde >= montant {
            self.solde -= montant;
            println!("{}", format!("-{} € retirés du compte de {}", montant, self.nom).green());
        } else {
            println!("{}", "Solde insuffisant !".red());
        }
    }
    fn fermer(&mut self) {
        if self.ouvert {
            println!("{}", format!("Le compte de {} est fermé. Dernier solde : {} €", self.nom, self.solde).green());
            self.ouvert = false;
        } else {
            println!("{}", "Ce compte est déjà fermé !".red());
        }
    }
}

// 5. Fonction principale avec menu interactif
fn main() {
    let mut comptes: Vec<CompteBancaire> = vec![
        CompteBancaire {
            nom: String::from("Dupont"),
            prenom: String::from("Alice"),
            age: 30,
            solde: 1500.0,
            ouvert: true,
        },
        CompteBancaire {
            nom: String::from("Martin"),
            prenom: String::from("Bob"),
            age: 42,
            solde: 800.0,
            ouvert: true,
        },
        CompteBancaire {
            nom: String::from("Durant"),
            prenom: String::from("Clara"),
            age: 25,
            solde: 500.0,
            ouvert: true,
        },
    ];

    loop {
        println!("\n========================");
        println!("      MENU BANCAIRE");
        println!("========================");
        println!("1. Afficher les comptes");
        println!("2. Déposer de l'argent");
        println!("3. Retirer de l'argent");
        println!("4. Fermer un compte");
        println!("5. Créer un nouveau compte");
        println!("6. Quitter");

        print!("Votre choix : ");
        io::stdout().flush().unwrap();

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).unwrap();
        let choix = choix.trim();

        match choix {
            "1" => {
                println!("--- Affichage des comptes ---");
                for (i, compte) in comptes.iter().enumerate() {
                    print!("{} : ", i);
                    compte.afficher();
                }
            }
            "2" => {
                println!("Choisissez le numéro du compte où déposer :");
                for (i, compte) in comptes.iter().enumerate() {
                    print!("{} : ", i);
                    compte.afficher();
                }
                let index = lire_index();
                if let Some(compte) = comptes.get_mut(index) {
                    println!("Montant à déposer :");
                    let montant = lire_montant();
                    compte.deposer(montant);
                } else {
                    println!("{}", "Numéro de compte invalide !".red());
                }
            }
            "3" => {
                println!("Choisissez le numéro du compte pour retirer :");
                for (i, compte) in comptes.iter().enumerate() {
                    print!("{} : ", i);
                    compte.afficher();
                }
                let index = lire_index();
                if let Some(compte) = comptes.get_mut(index) {
                    println!("Montant à retirer :");
                    let montant = lire_montant();
                    compte.retirer(montant);
                } else {
                    println!("{}", "Numéro de compte invalide !".red());
                }
            }
            "4" => {
                println!("Choisissez le numéro du compte à fermer :");
                for (i, compte) in comptes.iter().enumerate() {
                    print!("{} : ", i);
                    compte.afficher();
                }
                let index = lire_index();
                if let Some(compte) = comptes.get_mut(index) {
                    compte.fermer();
                } else {
                    println!("{}", "Numéro de compte invalide !".red());
                }
            }
            "5" => {
                println!("Création d'un nouveau compte !");
                println!("Nom ?");
                let nom = lire_chaine();
                println!("Prénom ?");
                let prenom = lire_chaine();
                println!("Âge ?");
                let age = lire_index() as u32;
                println!("Solde initial ?");
                let solde = lire_montant();
                let nouveau = CompteBancaire {
                    nom,
                    prenom,
                    age,
                    solde,
                    ouvert: true,
                };
                comptes.push(nouveau);
                println!("{}", "Compte créé !".green());
            }
            "6" => {
                println!("On quitte l'application. Merci !");
                break;
            }
            _ => {
                println!("{}", "Choix non reconnu, réessaye !".red());
            }
        }
    }
}

// Fonctions utilitaires
fn lire_index() -> usize {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap_or(0)
}
fn lire_montant() -> f64 {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap_or(0.0)
}
fn lire_chaine() -> String {
    let mut buf = String::new();
    print!("> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
