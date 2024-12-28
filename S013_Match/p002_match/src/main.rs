fn main() {
    let age = 15;

    match age {
        1 => println!("Happy 1st Birthday!"),
        13..=19 => println!("You are a teenager!"),
        x => println!("You are {x} years old!"),
    }
}
