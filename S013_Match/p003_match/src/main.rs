fn main() {
    let day_number = 2;

    let result: String = get_day_name_by_number(day_number);

    println!("{}", result);
}

fn get_day_name_by_number(day_number: i32) -> String {
    match day_number {
        1 => String::from("Sat"),
        2 => String::from("Sun"),
        3 => String::from("Mon"),
        4 => String::from("Tue"),
        5 => String::from("Wed"),
        6 => String::from("Thu"),
        7 => String::from("Fri"),
        _ => String::from("Invalid day number"),
    }
}
