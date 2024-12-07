fn main() {
    let mut first_counter = 0;

    while first_counter < 5 {
        let mut second_counter = 0;
        while second_counter < 10 {
            print!("*");
            second_counter += 1;
        }
        println!();
        first_counter += 1;
    }
}
