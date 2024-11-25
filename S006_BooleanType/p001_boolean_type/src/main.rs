fn main() {
    let b_true = true;
    let b_false = false;

    let mut score = 15;
    let mut result: bool = score > 10;
    println!("line 7 => {result}");

    score = 10;
    result = score >= 10;
    println!("line 11 => {result}");

    score = 20;
    result = score <= 20;
    println!("line 15 => {result}");

    score = 20;
    result = score == 20;
    println!("line 19 => {result}");

    let b_true = true;
    result = !b_true;
    println!("line 23 => {result}");

    score = 7;
    result = !(score >= 10);
    println!("line 27 => {result}");

    let input_1 = String::from("AliReza");
    let input_2 = String::from("AliReza");

    result = input_1 == input_2;
    println!("line 33 => {result}");

    result = !(input_1 == input_2);
    println!("line 36 => {result}");

    result = input_1 != input_2;
    println!("line 39 => {result}");
}
