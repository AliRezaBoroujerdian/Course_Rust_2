fn main() {
    let mut x: i32 = 5;
    let mut y: Option<i32> = Some(8);

    let result = x + y.unwrap();
    println!("{}", result);

    let result = x + y.expect("Something went wrong");
    println!("{}", result);
}
