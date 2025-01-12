fn main() {
    let divisor = 5.0;
    let quotient: Result<f64, String> = division(10.0, divisor);

    // match quotient {
    //     Ok(quotient) => println!("{}", quotient),
    //     Err(error) => println!("{}", error),
    // }

    println!("{}", quotient.unwrap());

    let quotient = division(10.0, divisor).unwrap();
    println!("{}", quotient);

    let quotient = division(10.0, divisor).expect("Something went wrong");
    println!("{}", quotient);

    let quotient = division(10.0, 0.0).expect("Something went wrong");
    println!("{}", quotient);
}

fn division(dividend: f64, divisor: f64) -> Result<f64, String> {
    match divisor {
        0.0 => Err(String::from("Cannot divide by zero!")),
        _ => Ok(dividend / divisor),
    }
}
