fn main() {
    expression();
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

// statements = instructions that perform some action and do not return a value. Actually if you see more, it returns '()' that is a unit type
// expressions = evaluate to a resulting value

fn expression() {
    let y = {
        let x = 3;
        x + 1 // it does not have semicolon because it is a expression. If you add it, you turn into a statement
    };

    println!("The value of y is: {}", y);
}

fn five() -> i32 {
   5 
}

fn plus_one(x: i32) -> i32 {
    x + 1
}