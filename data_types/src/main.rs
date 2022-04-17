use std::io;

fn main() {
    array_invalid_index();
}

fn float() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}

fn numeric_operations() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;
}

fn boolean_type() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}

fn character_type() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}

// compound types: tuples and arrays
fn tuple() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_fouter = x.1;
    let one = x.2;   
}

fn array() {
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5]; // [3, 3, 3, 3, 3]
    
    let first = a[0];
    let second = a[1];
}

fn array_invalid_index() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line.");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number.");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}