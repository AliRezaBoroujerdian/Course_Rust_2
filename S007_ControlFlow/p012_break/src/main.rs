fn main() {
    let mut inputNumber: i32;
    loop {
        println!("Choose an option:");
        println!();
        println!("1) Start");
        println!("2) About");
        println!("3) Exit");
        println!();
        println!(">>");

        let mut inputString: String = String::new();
        std::io::stdin().read_line(&mut inputString);
        inputNumber = inputString.trim().parse().expect("error");

        if inputNumber == 1 || inputNumber == 2 || inputNumber == 3 {
            break;
        } else {
            println!("Invalid input.")
        }
    }

    println!("{inputNumber}");
}
