use std::io::{self, Write};

fn main() {
    println!("Building an calculator in rust");

    print!("Enter the number 1->");
    let _ = io::stdout().flush();
    let mut number1 = String::new();
    io::stdin()
        .read_line(&mut number1)
        .expect("Failed to run read_line");

    print!("Enter the number 2->");
    let _ = io::stdout().flush();
    let mut number2 = String::new();
    io::stdin()
        .read_line(&mut number2)
        .expect("Failed to run read_line");


    print!("Enter the operator->");
    let mut operator = String::new();
    let _ = io::stdout().flush();

    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to run read_line");

    //Converting strings into i32 numbers
    let number1: i32 = number1.trim().parse().expect("Enter a valid number");
    let number2: i32 = number2.trim().parse().expect("Enter a valid number");
    let operator = operator.trim();
    print!("{}{}{} = ",number1,operator,number2);
    match operator {
        "+" => println!("{}", number1 + number2),
        "-" => println!("{}", number1 - number2),
        "*" => println!("{}", number1 * number2),
        "/" => {
            if number2 == 0 {
                let _ = io::stdout().flush();
                panic!("Cannot divide by zero!");
            }
            println!("{}", number1 / number2);
        },
        "%" => println!("{}", number1 % number2),
        "**" => {
            let u_number1: u32 = u32::try_from(number1).unwrap_or(0);
            let u_number2: u32 = u32::try_from(number2).unwrap_or(0);
            println!("{}", u_number1.pow(u_number2));
        }
        _ => println!("Invalid operator"),
    }
}
