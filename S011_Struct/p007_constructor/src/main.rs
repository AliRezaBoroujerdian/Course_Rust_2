struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    /*
     * Constructor
     */

    // fn new(width: u32, height: u32) -> Rectangle {
    //     let rect1 = Rectangle {
    //         width: width,
    //         height: height,
    //     };
    //     return rect1;
    // }

    // fn new(width: u32, height: u32) -> Rectangle {
    //     Rectangle {
    //         width: width,
    //         height: height,
    //     }
    // }

    // fn new(width: u32, height: u32) -> Rectangle {
    //     Rectangle { width, height }
    // }

    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /*
     * The Self keywords in the return type and
     * in the body of the function are aliases
     * for the type that appears after the impl keyword,
     * which in this case is Rectangle.
     */

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle::new(30, 50);

    println!(
        "The area of the rectangle1 is {} square pixels.",
        rect1.area()
    );
}
