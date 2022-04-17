#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50
    };

    println!(
        "The area of the rectangle is {} square pixels",
        area(&rect1)
    );

    //println!("rect is {:#?}", rect1); // stdout
    dbg!(&rect1); // stderr
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}