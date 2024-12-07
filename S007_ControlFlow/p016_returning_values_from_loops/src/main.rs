fn main() {
    let mut number = 5000;
    let result = loop {
        if number % 12 == 0 {
            break number;
        }
        number -= 1;
    };
    println!("{result}");
}
