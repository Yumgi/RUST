use crate::fichier::Fichier;
use std::fs::File;
use std::io::{self, Write};
use chrono::Local;

impl Fichier {
    pub fn new(nom: &str, contenu: &str) -> Self {
        Fichier {
            nom: nom.to_string(),
            contenu: contenu.to_string(),
            date_creation: Local::now(),
        }
    }
    pub fn creer_fichier(&self) -> io::Result<()> {
        let mut fichier = File::create(&self.nom)?;
        fichier.write_all(self.contenu.as_bytes())?;
        Ok(())
    }
    pub fn creer_avec_nom(nom: &str, contenu: &str) -> io::Result<()> {
        let mut fichier = File::create(nom)?;
        fichier.write_all(contenu.as_bytes())?;
        Ok(())
    }
}
