use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Bienvenido al juego de adivinar el número!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("-----------------");
        println!("Ingresa un número del 1 al 100");
        println!("-----------------");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Algo malio sal leyendo la linea");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Elegiste el número: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Elige uno mayor!"),
            Ordering::Greater => println!("Elige uno menor!"),
            Ordering::Equal => {
                println!("Ganaste! el número era {}", secret_number);
                break;
            }
        }
    }
}
