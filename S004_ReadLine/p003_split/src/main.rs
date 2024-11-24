fn main() {
    let input: String = String::from("The Old Man and the Sea");
    let mut words = input.split(' ');

    let first_word = words.next().expect("error");

    println!("{first_word}");

    let second_word = words.next().expect("error");

    println!("{second_word}");

    ///////////////////////////////

    let input_2: String = String::from("12,2,3,55");

    let mut numbers = input_2.split(',');

    let first_number = numbers.next().expect("error");

    println!("{first_number}");
}
