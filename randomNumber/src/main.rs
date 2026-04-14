use std::io::{self, Write};
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Random number guessing game!");
    let random :u32 = rand::thread_rng().gen_range(1..=10);
    loop {
        print!("Enter the number:");
        let _ = io::stdout().flush();
        let mut user_guess = String::new();
        io::stdin()
            .read_line(&mut user_guess)
            .expect("Failed to run read_line");
        let user_guess: u32 = user_guess.trim().parse().expect("Enter a valid number");
        println!("You entered -> {}",user_guess);
        match user_guess.cmp(&random) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You won");
                break;
            },
        }
    }
}
