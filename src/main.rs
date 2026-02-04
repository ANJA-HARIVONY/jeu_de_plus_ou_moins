use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {

    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut attempts = 1;

    println!("Devinez le nombre !");
    //println!("Le nombre secret est : {}", secret_number);
    loop {  
        println!("Veuillez entrer un nombre.");
       

        let mut supposition = String::new(); // le variable supposition est mutable


        io::stdin()
            .read_line(&mut supposition)
            .expect("Echec de la lecture de l'entrée utilisateur"); // si l'entrée utilisateur est invalide, on affiche un message d'erreur
            let supposition: u32 = match supposition.trim().parse() {
                Ok(nombre) => nombre,
                Err(_) => continue,
            }; // on parse le nombre entré par l'utilisateur
            //println!("Votre nombre est : {}", supposition);


        match supposition.cmp(&secret_number) {
            Ordering::Less => println!("Plus!"),
            Ordering::Greater => println!("Moins"),
            Ordering::Equal => {    
            println!("Gagné!");
            println!("Vous avez gagné en {} tentatives!", attempts);
            break;
            }
        }
        println!("Essai nº {}", attempts);
        attempts += 1;
       
    }

}
