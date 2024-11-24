fn main() {
    let b_true = true;
    let b_false = false;

    let mut score = 16;

    let mut result = score > 10;
    println!("line 8    => {result}");

    score = 10;
    result = score > 10;
    println!("line 12   => {result}");

    score = 10;
    result = score >= 10;
    println!("line 16   => {result}");

    score = 20;
    result = score <= 20;
    println!("line 20   => {result}");

    score = 20;
    result = score == 20;
    println!("line 24   => {result}");

    let b = true;
    result = !b;
    println!("line 28   => {result}");

    score = 2;
    result = !(score > 10);
    println!("line 32   => {result}");

    let name_1: String = String::from("AliReza");
    let name_2: String = String::from("AliReza");

    result = name_1 == name_2;
    println!("line 38   => {result}");

    result = !(name_1 == name_2);
    println!("line 41   => {result}");

    result = name_1 != name_2;
    println!("line 44   => {result}");
}
