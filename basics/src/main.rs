use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1.0..=100.0);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: f64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };

        match guess.partial_cmp(&secret_number) {
            Some(Ordering::Less) => println!("Too small!"),
            Some(Ordering::Greater) => println!("Too big!"),
            Some(Ordering::Equal) => {
                println!("You win!");
                break;
            }
            None => {
                println!("Comparison failed!!");
            }
        }
    }
}
