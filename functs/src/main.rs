/*
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("the value of x is {x}");
}
*/

fn main() {
    label_measurement(5, 'h');
}
fn label_measurement(x: i32, unit_label: char) {
    println!("the lenght is {x}{unit_label}");
}
