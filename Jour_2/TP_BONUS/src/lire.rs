use crate::fichier::Fichier;
use std::fs::File;
use std::io::{self, Read};

impl Fichier {
    pub fn lire_du_disque(&mut self) -> io::Result<()> {
        let mut fichier = File::open(&self.nom)?;
        let mut contenu_lu = String::new();
        fichier.read_to_string(&mut contenu_lu)?;
        self.contenu = contenu_lu;
        Ok(())
    }
}
