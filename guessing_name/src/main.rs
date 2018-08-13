extern crate rand;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Adivina el numero!");

    let secret_number = rand::thread_rng().gen_range(1, 11);

    loop {

        println!("Ingresa tu palpito:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Fallo al leer valor");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Demasiado bajo!"),
            Ordering::Greater => println!("Demasiado alto!"),
            Ordering::Equal => {
                println!("Acertaste!");
                break;
            }
        }

    }

}
