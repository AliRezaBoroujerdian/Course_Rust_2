fn main() {
    let mut s1: String = String::from("Hello");
    do_something_else(&s1);
    println!("s1: {s1}");
}

fn do_something_else(s: &String) {
    //s.push_str(", Word!");
    println!("s: {s}");
}
