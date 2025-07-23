use std::io::{self, Write};
use std::fs;
use std::path::Path;
use crate::fichier::Fichier;
use colored::*;

fn show_menu() {
    println!("\n=== MENU FICHIERS ===");
    println!("1. Lister les fichiers du dossier");
    println!("2. Créer un nouveau fichier");
    println!("3. Lire un fichier");
    println!("4. Modifier le contenu d’un fichier");
    println!("5. Ajouter du contenu à un fichier");
    println!("6. Informations sur un fichier");
    println!("7. Quitter");
    print!("\nVotre choix : ");
    io::stdout().flush().unwrap();
}

// Lit une ligne de texte entrée par l'utilisateur
fn read_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdout().flush()?; 
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_string();
    if input == ":q" {
        Err(io::Error::new(io::ErrorKind::Interrupted, "Quit command received"))
    } else {
        Ok(input)
    }
}

pub fn interface() -> io::Result<()> {
    loop {
        show_menu();
        let choix = match read_input() {
            Ok(val) => val,
            Err(e) if e.kind() == io::ErrorKind::Interrupted => {
                println!("{}", "Sortie demandée. Au revoir !".green());
                break;
            }
            Err(e) => return Err(e),
        };

        match choix.as_str() {
            "1" => {
                println!("\n=== Liste des fichiers ===\n");
                let path = ".";
                for entry in fs::read_dir(path)? {
                    let entry = entry?;
                    let file_name = entry.file_name();
                    let file_name = file_name.to_string_lossy();
                    println!("- {}", file_name);
                }
            }
            "2" => {
                // Boucle jusqu'à obtenir un nom qui n'existe pas, ou :q pour sortir
                let nom = loop {
                    println!("Nom du fichier à créer (ou :q pour annuler) :");
                    let nom = match read_input() {
                        Ok(nom) => nom,
                        Err(e) if e.kind() == io::ErrorKind::Interrupted => {
                            println!("{}", "\nCréation annulée par l'utilisateur.".red());
                            break None;
                        }
                        Err(e) => return Err(e),
                    };
                    if Path::new(&nom).exists() {
                        println!(
                            "{}",
                            "\nUn fichier avec ce nom existe déjà ! Veuillez choisir un autre nom.\n"
                                .red()
                        );
                        continue;
                    } else {
                        break Some(nom);
                    }
                };
                if nom.is_none() {
                    continue;
                }
                let nom = nom.unwrap();

                println!("Contenu initial (ou :q pour annuler) : ");
                let contenu = match read_input() {
                    Ok(val) => val,
                    Err(e) if e.kind() == io::ErrorKind::Interrupted => {
                        println!("{}", "\nCréation annulée par l'utilisateur.".red());
                        continue;
                    }
                    Err(e) => return Err(e),
                };
                let fichier = Fichier::new(&nom, &contenu);
                match fichier.creer_fichier() {
                    Ok(_) => println!("{}", format!("Fichier '{}' créé !", nom).green()),
                    Err(e) => println!("{}", format!("Erreur lors de la création : {}", e).red()),
                }
            }
            "3" => {
                println!("Nom du fichier à lire (ou :q pour annuler):");
                let nom = match read_input() {
                    Ok(val) => val,
                    Err(e) if e.kind() == io::ErrorKind::Interrupted => {
                        println!("{}", "\nLecture annulée par l'utilisateur.".red());
                        continue;
                    }
                    Err(e) => return Err(e),
                };
                let mut fichier = Fichier::new(&nom, ""); // contenu initial vide
                match fichier.lire_du_disque() {
                    Ok(_) => println!(
                        "{}\n{}",
                        "\nContenu du fichier :\n",
                        fichier.get_contenu()
                    ),
                    Err(e) => println!("{}", format!("Impossible de lire le fichier : {}", e).red()),
                }
            }
            "4" => {
                println!("Nom du fichier à modifier (ou :q pour annuler):");
                let nom = match read_input() {
                    Ok(val) => val,
                    Err(e) if e.kind() == io::ErrorKind::Interrupted => {
                        println!("{}", "\nModification annulée par l'utilisateur.".red());
                        continue;
                    }
                    Err(e) => return Err(e),
                };
                let mut fichier = Fichier::new(&nom, "");
                if fichier.lire_du_disque().is_ok() {
                    println!("\nAncien contenu : {}", fichier.get_contenu());
                    println!("\nNouveau contenu (ou :q pour annuler) :");
                    let nouveau = match read_input() {
                        Ok(val) => val,
                        Err(e) if e.kind() == io::ErrorKind::Interrupted => {
                            println!("{}", "\nModification annulée.".red());
                            continue;
                        }
                        Err(e) => return Err(e),
                    };
                    fichier.modifier_contenu(&nouveau);
                    fichier.creer_fichier()?;
                    println!("{}", "\nFichier modifié !".green());
                } else {
                    println!("{}", "\nCe fichier n’existe pas !\n".red());
                }
            }
            "5" => {
                println!("Nom du fichier auquel ajouter du contenu (ou :q pour annuler) :");
                let nom = match read_input() {
                    Ok(val) => val,
                    Err(e) if e.kind() == io::ErrorKind::Interrupted => {
                        println!("{}", "\nAjout annulé par l'utilisateur.".red());
                        continue;
                    }
                    Err(e) => return Err(e),
                };
                let mut fichier = Fichier::new(&nom, "");
                if fichier.lire_du_disque().is_ok() {
                    println!("\nContenu actuel : {}", fichier.get_contenu());
                    println!("\nContenu à ajouter (ou :q pour annuler):");
                    let ajout = match read_input() {
                        Ok(val) => val,
                        Err(e) if e.kind() == io::ErrorKind::Interrupted => {
                            println!("{}", "\nAjout annulé.".red());
                            continue;
                        }
                        Err(e) => return Err(e),
                    };
                    fichier.ajouter_contenu(&ajout);
                    fichier.creer_fichier()?;
                    println!("{}", "\nContenu ajouté !".green());
                } else {
                    println!("{}", "\nCe fichier n’existe pas !\n".red());
                }
            }
            "6" => {
                println!("Nom du fichier pour afficher ses informations (ou :q pour annuler) :");
                let nom = match read_input() {
                    Ok(val) => val,
                    Err(e) if e.kind() == io::ErrorKind::Interrupted => {
                        println!("{}", "\nAffichage annulé par l'utilisateur.".red());
                        continue;
                    }
                    Err(e) => return Err(e),
                };

                if Path::new(&nom).exists() {
                    let mut fichier = Fichier::new(&nom, "");
                    if fichier.lire_du_disque().is_ok() {
                        println!("{}", "\n=== Informations sur le fichier ===".green());
                        println!("Nom : {}", fichier.get_nom());
                        println!("Date de création : {}", fichier.get_date_creation());
                        match fichier.get_taille() {
                            Some(taille) => println!("Taille : {} octets", taille),
                            None => println!("{}", "Impossible de lire la taille du fichier.".red())
                        }
                    } else {
                        println!("{}", "Impossible de lire le fichier.".red());
                    }
                    } else {
                    println!("{}", "\nCe fichier n’existe pas !\n".red());
                }
            }


            "7" => {
                println!("{}", "\nAu revoir !\n");
                break;
            }
            _ => println!("{}", "\nChoix invalide !\n".red()),
        }
    }
    Ok(())
}
