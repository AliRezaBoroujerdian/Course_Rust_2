fn main() {
    let _a: i32 = 15;

    // error
    // Variables are immutable by default
    //a = 16;

    let mut b: i32 = 15;
    b = 16;

    println!("{b}");
}
