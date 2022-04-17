#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// all functions inside this block are called 'associated functions'
// the function is not a method if does not have 'self' as the first parameter, for example a construtor
// it is allowed to have multiple 'impl' blocks
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated function (not method)
    fn square(size: u32) -> Rectangle {
        Rectangle { 
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };
    
    let rect2 = Rectangle {
        width: 10,
        height: 40
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45
    };

    let _square = Rectangle::square(3);

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}