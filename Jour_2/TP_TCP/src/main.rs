/*Tp : Reséau TCP avec gestion clients
  >> créer un serveur TCP  simple qui accepte plusieurs connexions et garde une liste des clients connectés ( une structure Client )
     vous appliquer  Ownership et Membership 
     consignes  utiliser la bibliothèque  use std::net::TcpStream  et TcpListner  
                                          et std::io:: {Read, Write}
*/                                          

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

// Définition de la structure Client (stocke juste le nom ici)
struct Client {
    name: String,
}

// Fonction qui gère chaque client
fn gestion_client(mut stream: TcpStream, name: String) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("{} s'est déconnecté.", name);
                break;
            }
            Ok(n) => {
                let msg = String::from_utf8_lossy(&buffer[..n]);
                println!("{}: {}", name, msg);
                stream.write_all(msg.as_bytes()).unwrap();
            }
            Err(_) => {
                println!("Erreur réseau avec {}", name);
                break;
            }
        }
    }
}

fn main() {
    // Lancement du serveur sur le port 8080
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Serveur non lancé");
    println!("Serveur lancé sur 127.0.0.1:8080");

    let mut id = 1;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let name = format!("Client{}", id);
                let client = Client { name: name.clone() };
                println!("{} connecté.", client.name);

                // Création du thread pour gérer ce client
                thread::spawn(move || {
                    gestion_client(stream, name);
                });

                id += 1;
            }
            Err(e) => {
                println!("Erreur de connexion: {}", e);
            }
        }
    }
}
