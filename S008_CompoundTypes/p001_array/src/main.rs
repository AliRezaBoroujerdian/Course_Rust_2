fn main() {
    let my_array = [12, 15, 9];

    let first_element = my_array[0];

    println!("{first_element}");
    println!("{}", my_array[0]);
    println!("{}", my_array[1]);
    println!("{}", my_array[2]);

    let cities = ["Tehran", "Shiraz"];

    println!("{}", cities[0]);

    let number: i32 = 12;
    let scores: [i32; 5] = [118, 115, 101, 100, 110];

    let numbers = [3; 5];

    println!("{}", numbers[0]);
}
