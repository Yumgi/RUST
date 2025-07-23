use std::fs::File;
use std::io::{self, Write};

pub struct Fichier {
    nom: String,
    contenu: String,
}

impl Fichier {
    /// Crée une nouvelle instance de Fichier
    pub fn creer(nom: &str, contenu: &str) -> Self {
        Self {
            nom: nom.to_string(),
            contenu: contenu.to_string(),
        }
    }

    /// Ecrit le contenu dans le fichier
    pub fn ecrire(&self) -> io::Result<()> {
        let mut fichier = File::create(&self.nom)?;
        fichier.write_all(self.contenu.as_bytes())?; // on écrit le contenu fourni, pas du texte codé en dur
        println!("Le fichier {} a été créé avec succès", self.nom);
        Ok(())
    }
}
