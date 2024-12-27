struct Rectangle {
    width: u32,
    height: u32,
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(rectangle: Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

impl Rectangle {
    /*
     * All functions defined within an impl block are called associated functions
     * because they’re associated with the type named after the impl.
     */

    // fn area(rectangle: &Rectangle) -> u32 {
    //     rectangle.width * rectangle.height
    // }

    /*
     * Methods are similar to functions.
     * Their first parameter is always self, which represents
     * the instance of the struct the method is being called on.
     */

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn update_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn do_something() -> String {
        String::from("do something")
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // println!(
    //     "The area of the rectangle1 is {} square pixels.",
    //     area(rect1.width, rect1.height)
    // );

    // println!(
    //     "The area of the rectangle1 is {} square pixels.",
    //     area(rect1)
    // );

    // println!(
    //     "The area of the rectangle1 is {} square pixels.",
    //     area(&rect1)
    // );

    // println!(
    //     "The area of the rectangle1 is {} square pixels.",
    //     Rectangle::area(&rect1)
    // );

    println!(
        "The area of the rectangle1 is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect2_area = rect2.area();
    println!(
        "The area of the rectangle2 is {} square pixels.",
        rect2_area
    );

    if rect1.width() {
        println!("The rectangle2 has a nonzero width; it is {}", rect2.width);
    }

    let mut rect3 = Rectangle {
        width: 30,
        height: 20,
    };

    rect3.update_size(60, 45);

    println!("rectangle3 => ({}, {})", rect3.width, rect3.height);

    println!("Can rectangle1 hold rectangle2? {}", rect1.can_hold(&rect2));

    println!("Can rectangle1 hold rectangle3? {}", rect1.can_hold(&rect3));

    let s = Rectangle::do_something();
    println!("{}", s);
}
