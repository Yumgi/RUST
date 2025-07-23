mod ecrire;
mod lire;

use std::io;
use ecrire::Fichier;
use chrono::Local;

fn main() -> io::Result<()> {
    // Appel de la méthode de la structure Fichier
    let fichier = Fichier::creer(
        "test2.txt",
        "Fichier créé avec la structure Fichier - ceci est un exemple",
    );

    fichier.ecrire()?; // Ecrit dans le fichier et gère potentiellement l'erreur
    fichier.lire()?; // Lit le contenu du fichier et gère potentiellement l'erreur

    // Affiche la date et l'heure actuelles
    let now = Local::now();
    println!("\nDate et heure actuelles : {} \n", now.format("%d-%m-%Y %H:%M:%S"));
    Ok(())
}

  // A faire:    créer une structure Fichier et implémenter une fonction qui crée un fichier
                 // et qui prend en paramètre le nom de fichier 
                 // ecrire.rs 
