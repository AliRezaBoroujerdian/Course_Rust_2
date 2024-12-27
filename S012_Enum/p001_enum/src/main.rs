enum Genre {
    History,
    Fantasy,
    Fiction,
    Horror,
    Biography,
    Novel,
}

struct Book {
    name: String,
    genre: Genre,
    price: f32,
    pages: i32,
    in_stock: bool,
}

fn main() {
    let genre = Genre::Fantasy;

    let b1 = Book {
        name: String::from("The Old Man and the Sea"),
        genre: Genre::Novel,
        price: 13.99,
        pages: 128,
        in_stock: true,
    };
}
