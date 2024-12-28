fn main() {
    let age = 25;

    match age {
        1 => println!("Happy 1st Birthday!"),
        13..=19 => println!("You are a teenager!"),
        x => print_age(x),
    }
}

fn print_age(age: i32) {
    println!("You are {age} years old!");
}
