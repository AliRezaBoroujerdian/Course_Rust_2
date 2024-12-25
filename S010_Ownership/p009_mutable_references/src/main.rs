fn main() {
    let mut s: String = String::from("Rust");

    let r1 = &mut s;
    //let r2 = &mut s;

    do_something(&mut s);

    s = String::from("value");
    println!("{}", s);

    //println!("{}", r1);
    //println!("{}", r2);
    //println!("{}, {}", r1, r2);
}

fn do_something(r1: &mut String) {
    // do something
}
