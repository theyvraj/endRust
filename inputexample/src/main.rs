use std::io;
use std::io::Write;
fn main() {
    print!("Welcome to rust! \nPlease Enter your name : ");
    if let Err(e) = io::stdout().flush() {
        eprintln!("error flushing : {e}");
    }
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("invalid input");
    println!("Hello {}", name);
}
