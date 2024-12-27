fn main() {
    let s1 = String::from("Rust"); // Heap allocated string
    let s2 = "Rust"; // it’s a slice pointing to that specific point of the binary.

    let s3 = "Rust".to_owned();
}
