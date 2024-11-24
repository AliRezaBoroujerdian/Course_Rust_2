fn main() {
    println!("number: ");

    let mut input_1: String = String::new();
    std::io::stdin().read_line(&mut input_1);

    let number: i32 = input_1
        .trim() // ignore whitespace
        .parse() // convert to integers
        .expect("error");

    let another_number = number + 10;

    println!("{another_number}");

    ///////////////////

    println!("number 2: ");

    let mut input_2: String = String::new();
    std::io::stdin().read_line(&mut input_2);

    let number: i16 = input_2.trim().parse().expect("error");

    let another_number: i16 = number + 10;

    println!("{another_number}");

    ///////////////////

    println!("number 3: ");``

    let mut input_3: String = String::new();
    std::io::stdin().read_line(&mut input_3);

    let number: u64 = input_3.trim().parse::<u64>().expect("Input not an integer");

    let another_number: u64 = number + 10;

    println!("{another_number}");
}
