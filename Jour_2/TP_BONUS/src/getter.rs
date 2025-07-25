use std::fs::metadata;
use crate::fichier::Fichier;

impl Fichier {
    pub fn get_nom(&self) -> &str {
        &self.nom
    }
    pub fn get_contenu(&self) -> &str {
        &self.contenu
    }
    pub fn get_date_creation(&self) -> String {
        self.date_creation.format("%Y-%m-%d %H:%M:%S").to_string()
    }
    pub fn get_taille(&self) -> Option<u64> {
        metadata(&self.nom).map(|meta| meta.len()).ok()
    }
}
