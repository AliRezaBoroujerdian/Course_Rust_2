fn main() {
    let mut s1: String = String::from("Hello");
    do_something_else(&mut s1);
    println!("s1: {s1}");
}

fn do_something_else(s: &mut String) {
    s.push_str(", world!");
    println!("s: {s}");
}
