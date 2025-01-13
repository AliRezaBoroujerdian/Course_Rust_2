fn main() {
    let age = 50;

    match age {
        1 => println!("Happy 1st Birthday!"),
        13..=19 => println!("You are a teenager!"),
        21 | 22 => println!("You are 21 or 22 years old!"),
        x => print_age(x),
    }
}

fn print_age(age: i32) {
    println!("You are {age} years old!");
}
