/*
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("the value of x is {x}");
}
fn main() {
    label_measurement(5, 'h');
}
fn label_measurement(x: i32, unit_label: char) {
    println!("the lenght is {x}{unit_label}");
}
=============================
Statements and Expressions
fn main() {
    let x = {
        let y = 4;
        y + 3
    };

    println!("The value of x is {x}");
}
=======================
Functions with return values
use std::io::{self, Write};

fn times_two(x: i32) -> i32 {
    x * 2
}
fn main() {
    let mut y = String::new();
    print!("Enter your number : ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut y).expect("failed to read line.");
    let x: i32 = y.trim().parse().expect("please enter a valid number.");
    let result = times_two(x);
    println!("Value of {} times two is {}", x, result);
}
===================================
Control Flow
fn main() {
    let number = 3;
    if number < 5 {
        println!("condition was true.");
    } else {
        println!("condition was false.");
    }
}

fn main() {
    let number = 8;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisibile by 2");
    } else {
        println!("number is not divisible by 2, 3 and 4.");
    }
}
================================
IF in a let statement

fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is : {number}");
}
======================
Repeating code with loop

use chrono;

fn main() {
    loop {
        println!("{:#?}", chrono::offset::Local::now());
        println!("again");
    }
}
=======================
Returning values from loops

fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 19 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
===============================
Loops labels to disambiguate between multiple loops

fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 3 {
                break;
            }
            if count == 5 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("end count = {count}");
}
======================================
conditional loops with while

fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
======================================
looping through a collection with for

fn main() {
    let a = [10, 20, 30, 40, 50];

    let mut pos = 1;
    for i in a {
        println!("the value of element at {pos} position is {i}");
        pos += 1;
    }
}
*/

fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
