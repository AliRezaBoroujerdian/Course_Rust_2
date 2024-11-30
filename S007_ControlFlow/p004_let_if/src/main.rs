fn main() {
    let authentication = String::from("OK");

    let result = if authentication == "OK" { true } else { false };

    println!("{result}");

    let number = 2;

    // let mut result = String::new();

    // if number % 2 == 0 {
    //     result = String::from("number is divisible by 2");
    // } else {
    //     result = String::from("number is not divisible by 2");
    // }

    // println!("{result}");

    let result = if number % 2 == 0 {
        String::from("number is divisible by 2")
    } else {
        String::from("number is not divisible by 2")
    };

    println!("{result}");
}
