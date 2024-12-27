fn main() {
    let s1 = String::from("Rust"); // Heap allocated string
    let s2 = "Rust"; // it’s a slice pointing to that specific point of the binary.

    let s3 = "Rust".to_owned();
    let s4 = "Rust".to_string();

    let r1 = &s1;
    let slice: &str = &s1; //deref coercions

    println!("{slice}");
}
