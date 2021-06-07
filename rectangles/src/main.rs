#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// implementation block, used to define functions within the context of Rectangle
// NOTE: you can have multiple impl blocks referring to the same struct, but it's unnecessary for
// now
impl Rectangle {
    // instead of passing "rectangle: &Rectangle", we use &self because Rust knowns the type of
    // self as Rectangle due to this method being inside the "impl Rectangle" context
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated function of Rectangle, accessible by using :: syntax
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

    println!("The area of the rectangle is {} square pixels", rect1.area());

    // prints the struct. the strust must implement Debug, which is shown as "#[derive(Debug)]"
    println!("rect1 is {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels",
        area(&rect1) // pass in reference of rect1 in order for "main" to retain ownership
    );

    // accessing the Rectangle's associated square function
    let square = Rectangle::square(10);

    println!("The area of the square is {}", square.area());

    more_rectangles();
}

// we want to borrow the struct rather than taking ownership. therefore, we need to pass in a
// reference of our struct using &Rectangle
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn more_rectangles() {
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

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
