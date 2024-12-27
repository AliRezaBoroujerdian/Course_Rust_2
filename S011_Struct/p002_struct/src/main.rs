fn main() {
    struct Book {
        name: String,
        price: f32,
        pages: i32,
        in_stock: bool,
    }

    let b1 = Book {
        name: String::from("The Old Man and the Sea"),
        price: 13.99,
        pages: 128,
        in_stock: true,
    }; // b1 => instance

    let price = b1.price;

    println!("{}", b1.name);

    let mut b2 = Book {
        name: String::from("The Stranger"),
        price: 15.0,
        pages: 123,
        in_stock: true,
    };

    b2.price = 9.99;

    let s = b2.name;
    //println!("{}", b2.name);
    println!("{}", b2.price);
}
