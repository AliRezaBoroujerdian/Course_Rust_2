fn main() {
    let day_number = 2;

    let result: String = match day_number {
        1 => String::from("Sat"),
        2 => {
            println!("test");
            String::from("Sun")
        } // first simple then turn to this.
        3 => String::from("Mon"),
        4 => String::from("Tue"),
        5 => String::from("Wed"),
        6 => String::from("Thu"),
        7 => String::from("Fri"),
        _ => String::from("Invalid day number"),
    };

    println!("{}", result);
}
