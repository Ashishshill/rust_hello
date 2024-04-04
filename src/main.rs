use colored::*;
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Hello World now i am rust developer :p :p :p");
    println!("Guess Number !");

    let guess_number = rand::thread_rng().gen_range(1, 100);
    println!("The secreate Number: {}", guess_number);

    println!("Please input your number");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Please add number here");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("youer Guess number: {}", guess);

        match guess.cmp(&guess_number) {
            Ordering::Less => println!("{}", "Too week number".red()),
            Ordering::Greater => println!("{}", "Big number".blue()),
            Ordering::Equal => {
                println!("{}", "You win".green());
                break;
            }
        }
    }
}
