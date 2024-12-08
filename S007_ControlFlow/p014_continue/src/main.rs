fn main() {
    let mut number = 0;

    while number <= 100 {
        if number % 12 != 0 {
            number += 1;
            continue;
        }

        println!("{number}");
        number += 1;
    }
}
