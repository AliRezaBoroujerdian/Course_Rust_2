fn main() {
    let a = 5;

    // Error
    // Variables are immutable by default.
    //a = 6

    let mut b: i32 = 11;

    b = 43;
    println!("{b}");

    b = 50;
    println!("{b}");
}
