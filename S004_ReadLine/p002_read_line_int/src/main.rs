fn main() {
    println!("number: ");

    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input);

    let number: i32 = input
        .trim() // ignore whitespace
        .parse() // convert to int
        .expect("error");

    println!("{number}");
    println!("{}", number + 10);

    ////////////////////////

    println!("number 2: ");

    let mut input_2: String = String::new();
    std::io::stdin().read_line(&mut input_2);

    let number: i8 = input_2.trim().parse().expect("error");

    println!("{number}");
    println!("{}", number + 10);

    ////////////////////////

    println!("number 3: ");

    let mut input_3: String = String::new();
    std::io::stdin().read_line(&mut input_3);

    let number: u64 = input_3.trim().parse::<u64>().expect("error");

    println!("{number}");
    println!("{}", number + 10);
}
