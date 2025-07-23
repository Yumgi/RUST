// lecture à partir d'un fichier
// dans notre cas on utilise Read et BufReader

use std::fs::File; // stream 
use std::io::{self,BufReader,Read};

fn main() -> io::Result<()>{
     
       let file = File::open("test.txt")?;
       let mut reader = BufReader::new(file); // on crée un lecteur tamponné 
       let mut content = String::new(); 
       reader.read_to_string(&mut content)?;
       
       println!("ceci est le contenu du fichier {}", content);




    Ok(())
}