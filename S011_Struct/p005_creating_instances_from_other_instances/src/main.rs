struct Book {
    name: String,
    price: f32,
    pages: i32,
    in_stock: bool,
}

fn main() {
    let b1 = Book {
        name: String::from("The Old Man and the Sea"),
        price: 13.99,
        pages: 128,
        in_stock: true,
    };

    let b2 = Book { price: 8.0, ..b1 };

    println!("{}", b2.name);
    //println!("{}", b1.name);

    println!("{}", b1.price);
}
