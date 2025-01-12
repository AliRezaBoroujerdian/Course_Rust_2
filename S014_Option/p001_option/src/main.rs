fn main() {
    let mut x: i32 = 5;

    let mut y: Option<i32> = Some(8);
    let mut z: Option<i32> = None;

    //error - no implementation for `i32 + Option<i32>
    //let result = x + y;

    println!("x = {}", x);
    match y {
        Some(value) => println!("y = {}", value),
        None => {}
    }
    match z {
        Some(value) => println!("z = {}", value),
        None => {}
    }

    let mut result: i32;
    match y {
        Some(value) => {
            result = x + value;
        }
        None => result = 0,
    }
    println!("x + y = {}", result);

    let mut result: i32;
    match z {
        Some(value) => {
            result = x + value;
        }
        None => result = 0,
    }
    println!("x + y = {}", result); // test error
}
