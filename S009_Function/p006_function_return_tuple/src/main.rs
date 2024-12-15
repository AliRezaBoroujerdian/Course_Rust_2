fn main() {
    let (addition, subtraction, multiplication, division) = calculate_basic_operations(10, 2);

    println!("{addition}");
    println!("{subtraction}");
    println!("{multiplication}");
    println!("{division}");
}

fn calculate_basic_operations(x: i32, y: i32) -> (i32, i32, i32, i32) {
    (x + y, x - y, x * y, x / y)
}
