use chrono::{DateTime, Local};

pub struct Fichier {
    pub nom: String,
    pub contenu: String,
    pub date_creation: DateTime<Local>,
}
