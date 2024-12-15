fn main() {
    let i: i32 = 5;
    do_something(i);
    println!("i1: {i}");

    let s: String = String::from("Rust");
    do_something_else(s);
    println!("s1: {s}");
}

fn do_something(some_integer: i32) /* some_integer makes copy */
{
    // do something
}

fn do_something_else(some_string: String) /* some_string takes ownership */
{
    // do something else
}
