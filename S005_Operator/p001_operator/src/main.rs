fn main() {
    let a: i32 = 11;
    let b: i32 = 2;

    let mut result = a + b;
    println!("a + b = {result}");

    result = a - b;
    println!("a - b = {result}");

    result = a * b;
    println!("a * b = {result}");

    result = a / b;
    println!("a / b = {result}");

    result = a % b;
    println!("a % b = {result}");

    let c: f32 = 11.0;
    let d: f32 = 2.0;

    let result: f32 = c / d;
    println!("c / d = {result}");

    let result: i32 = 3 + 4 * 5;
    println!("3 + 4 * 5 = {result}");

    let result: i32 = (3 + 4) * 5;
    println!("(3 + 4) * 5 = {result}");

    let mut e = 10;
    e = e + 5;
    println!("{e}");

    let mut f = 15;
    f += 5; // f = f + 5;
    println!("{f}");
}
