fn main() {
    let a: i32 = 11;
    let b: i32 = 2;

    let mut result = a + b;
    println!("a + b = {result}");

    result = a - b;
    println!("a - b = {result}");

    result = a * b;
    println!("a + b = {result}");

    result = a / b;
    println!("a / b = {result}");

    result = a % b;
    println!("a % b = {result}");

    let c: f32 = 11.0;
    let d: f32 = 2.0;

    let result = c / d;
    println!("c / d = {result}");

    let result = 3 + 4 * 5;
    println!("3 + 4 * 5 = {result}");

    let result = (3 + 4) * 5;
    println!("(3 + 4) * 5 = {result}");

    let mut e = 12;
    e = e + 10;
    println!("{e}");

    let mut f = 12;
    f += 10; // f = f + 10;
    println!("{f}");
}
