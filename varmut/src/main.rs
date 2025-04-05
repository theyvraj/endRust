/*
fn main() {
    let x = 5;
    println!("x is currently 5.");
    let x = x + 1;
    {
        let x = x + 2;
        println!("inner scope value is {x}");
    }
    println!("new value after shadowing is {x}");

    let space = "  ";
    let spacelen = space.len().to_string();
    println!("{spacelen}");
}
fn main() {
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!("{sum}, {difference}, {product}, {quotient}, {truncated}, {remainder}");
}
fn main() {
    let t = true;
    let f: bool = false;
    let c = 'z';
    let z: char = 'ℤ';
    let heart_cat = '😻';
    println!("{t},{f},{c},{z},{heart_cat}");
}
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("{x},{y},{z}");
}

fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundered = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("{five_hundered}, {six_point_four}, {one}");
}

fn main() {
    let a = [1, 2, 3, 4, 5];
    // println!("{:?}", a);
    let first = a[0];
    let second = a[1];
    println!("{first}, {second}");
}

*/
use std::io::{self, Write};
fn main() {
    let a = [1, 2, 3, 4, 5];
    loop {
        print!("Please enter an array index (0-4) : ");
        io::stdout().flush().expect("failed to flush stdout.");

        let mut index = String::new();
        io::stdin()
            .read_line(&mut index)
            .expect("failed to read line.");

        let index: usize = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        if index < a.len() {
            let element = a[index];
            println!("The value of the element at index {index} is : {element}");
            break;
        } else {
            println!("Index is out of bounds. Please enter a number between 0 and 4.");
        }
    }
}
