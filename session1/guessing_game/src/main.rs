use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Number Guessing Game");
    let random_number :u32 = rand::thread_rng().gen_range(1..=10);
    loop {
        print!("Enter your number:");
        let _ = io::stdout().flush();
        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Error while running read_line");
        let number: u32 = number.trim().parse().expect("Enter a valid unsigned number"); // Unsigned int
        println!("The number you entered is -> {number}");
        match number.cmp(&random_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Won!");
                break;
            },
        }
    }
}
