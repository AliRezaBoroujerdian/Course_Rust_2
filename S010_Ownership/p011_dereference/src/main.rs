fn main() {
    let mut s: String = String::from("Rust");
    println!("{}", s);

    s = String::from("ferris");
    println!("{}", s);

    let r1 = &mut s;
    *r1 = String::from("value");

    //.push_str(" m");
    println!("{}", r1);
    println!("{}", *r1);
}
