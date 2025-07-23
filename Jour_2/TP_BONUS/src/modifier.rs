use crate::fichier::Fichier;

impl Fichier {
    pub fn modifier_contenu(&mut self, nouveau: &str) {
        self.contenu = nouveau.to_string();
    }
    pub fn ajouter_contenu(&mut self, supplement: &str) {
        self.contenu.push_str(supplement);
    }
}
