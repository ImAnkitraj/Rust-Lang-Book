use colored::*;
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Secret Number {}", secret_number);
    loop {
        println!("Input your guess");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too less".red()),
            Ordering::Equal => {
                println!("{}", "Equal".green());
                break;
            }
            Ordering::Greater => println!("{}", "Too big".red()),
        }
    }
}
