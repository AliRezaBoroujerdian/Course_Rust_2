fn main() {
    // let quotient = division(10.0, 0.0);
    // println!("{}", quotient);

    // let quotient: Result<f64, String> = division(10.0, 0.0);
    // match quotient {
    //     Ok(quotient) => println!("{}", quotient),
    //     Err(error) => println!("{}", error),
    // }

    let quotient: Result<f64, String> = division(10.0, 0.0);
    match quotient {
        Ok(_) => println!("OK"),
        Err(_) => println!("NOK"),
    }
}

// fn division(dividend: f64, divisor: f64) -> f64 {
//     dividend / divisor
// }

// fn division(dividend: f64, divisor: f64) -> Result<f64, String> {
//     if divisor == 0.0 {
//         Err(String::from("Cannot divide by zero!"))
//     } else {
//         Ok(dividend / divisor)
//     }
// }

fn division(dividend: f64, divisor: f64) -> Result<f64, String> {
    match divisor {
        0.0 => Err(String::from("Cannot divide by zero!")),
        _ => Ok(dividend / divisor),
    }
}
