fn main() {
    let nested_tuple: (i32, f64, (i32, i32), String) = (4, 5.2, (3, 16), String::from("Test"));

    let element = nested_tuple.2 .0;

    println!("{}", element);
    println!("{}", nested_tuple.2 .1);
}
