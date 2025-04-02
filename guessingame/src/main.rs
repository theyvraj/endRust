use rand::Rng;
use std::io::{self, Write};
fn main() {
    loop {
        println!("Welcome to my guessing game");
        let secret = rand::thread_rng().gen_range(1..=10);
        print!("Enter your guess! :");
        io::stdout().flush().expect("Failed to flush stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                return;
            }
        };

        println!("You guessed: {guess}");
        if guess == secret {
            println!("Congratulations! You guessed correctly.");
        } else {
            println!("Sorry, the correct number was : {secret}!");
        }

        println!("Do you want to play again? (type \"yes\" / \"no\")");
        let mut reinput = String::new();
        io::stdin()
            .read_line(&mut reinput)
            .expect("Failed to read line!");
        match reinput.trim().to_lowercase().as_str() {
            "yes" => continue,
            "no" => {
                println!("Thank you for playing! Goodbye...");
                break;
            }
            _ => {
                println!("Invalid response. Enter valid response again.");
                continue;
            }
        }
    }
}
