/*fn main() {
    let s = "hello";
    {
        let s = "bruh";
        println!("{s}");
    }
    println!("{s}");
}

fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{s2}");
}

fn main() {
    let s = String::from("hello");
    take_ownsership(s);
    let x = 5;
    makes_copy(x);
}
fn take_ownsership(x: String) {
    println!("{x}");
}
fn makes_copy(x: i32) {
    println!("{x}");
}

fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = take_back_ownership(s2);
    println!("{s1}");
    println!("{s3}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn take_back_ownership(some_string: String) -> String {
    some_string
}


fn main() {
    let s1 = String::from("devraj");
    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
========================================================
REFERENCES AND BORROWING

fn main() {
    let s1 = String::from("hellow");
    let len = calculate_length(&s1);
    println!("the length of '{s1}' is {len}.");
    println!("the length of '{s1}' is {len}.");
}
fn calculate_length(s: &String) -> usize {
    s.len()
}


fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{s}");
}
fn change(some_string: &mut String) {
    some_string.push_str(" world!");
}

fn main() {
    /*let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("{r1}");
    }
    let r2 = &mut s;
    println!("{r2}"); */
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");
}


fn main() {
    let reference_to_nothing = dangle();
    println!("{reference_to_nothing}");
}
fn dangle() -> String {
    let s = String::from("hello");
    s
}

*/
fn main() {
    let mut s = String::from("he l llo ");
    let result = first_word(&s);
    println!("{result}");
    s.clear();
    println!("{result}");
}
fn first_word(s: &String) -> usize {
    let mem = s.as_bytes();

    for (i, &item) in mem.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
