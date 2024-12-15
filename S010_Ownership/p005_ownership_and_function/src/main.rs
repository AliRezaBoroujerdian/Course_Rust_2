fn main() {
    let mut s1 = gives_ownership();
    println!("{s1}");
    s1 = String::from("Ferris");
    println!("{s1}");
    let s2 = takes_and_gives_back(s1);
    println!("{s2}");
}

fn gives_ownership() -> String {
    let mut some_string = String::from("Rust");
    some_string
}

fn takes_and_gives_back(mut some_string: String) -> String {
    some_string = String::from("Rustacean");
    some_string
}
