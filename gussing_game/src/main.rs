// use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::process::exit;

use rand::Rng;

fn main() {
    println!("Guess a number!");

    println!("Please input your guess.");

    let number = rand::thread_rng().gen_range(1..=10);

    for i in 1..4 {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read you guess");

        let guest_n: u32 = match guess.trim().parse() {
            Ok(e) => e,
            Err(_) => {
                println!("Invalid input try again");
                continue;
            }
        };

        // println!("Your Guess: {guess} {3}", &number);

        match guest_n.cmp(&number) {
            Ordering::Less => println!("Too small. Try Larger:"),
            Ordering::Equal => {
                println!("exactly you got it write. ");
                exit(0)
            }
            Ordering::Greater => println!("Too large. Try smaller:"),
        }
    }
}
