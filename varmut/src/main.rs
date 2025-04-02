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
*/
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
