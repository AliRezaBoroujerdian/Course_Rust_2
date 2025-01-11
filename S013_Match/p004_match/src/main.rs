fn main() {
    let age = 21;

    match age {
        1 => println!("Happy 1st Birthday!"),
        13..=19 => println!("You are a teenager!"),
        21 | 22 => println!("You are 21 or 22 years old!"),
        x => println!("You are {x} years old!"),
    }
}
