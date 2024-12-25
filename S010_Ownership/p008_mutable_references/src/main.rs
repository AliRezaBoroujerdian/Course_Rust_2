fn main() {
    let mut s: String = String::from("Rust");

    let r1 = &s;
    let r2 = &mut s;

    //println!("{}", s);
    println!("{}", r1);
    //println!("{}", r2);
}
