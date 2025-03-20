use std::io;
use std::io::Writing;
use rand::Rng;
fn main() {
    let secret = rand::thread_rng().gen_range(1..=10);
    println!("Welcome to my guessing game");
        print!("Enter your guess! :");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        let guess: i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Enter a numebr, nothing else!");
                continue;
            } 
        };
