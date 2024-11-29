fn main() {
    let number = 6;

    if number % 2 == 0 {
        println!("number is divisible by 2");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 2 or 3");
    }
}
