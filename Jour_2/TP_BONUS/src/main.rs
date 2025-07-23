mod fichier;
mod getter;
mod creer;
mod modifier;
mod lire;
mod interface;
use crate::fichier::Fichier;
use colored::*;

fn main() -> std::io::Result<()> {
    Fichier::creer_avec_nom("statique.txt", "Fichier crée de façon statique\n")?;
    println!("{}", "\nFichier statique créé avec succès.\n".green());

    interface::interface();

    Ok(())
}

