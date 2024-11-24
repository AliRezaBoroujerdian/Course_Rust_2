fn main() {
    let b_true = true;
    let b_false = false;

    println!("true  |   true    =>  {}", b_true | b_true); // true
    println!("true  |   false   =>  {}", b_true | b_false); // true
    println!("false |   true    =>  {}", b_false | b_true); // true
    println!("false |   false   =>  {}", b_false | b_false); // false

    println!("===============================");

    println!("true  &   true    =>  {}", b_true & b_true); // true
    println!("true  &   false   =>  {}", b_true & b_false); // false
    println!("false &   true    =>  {}", b_false & b_true); // false
    println!("false &   false   =>  {}", b_false & b_false); // false

    println!("===============================");

    println!("true  ^   true    =>  {}", b_true ^ b_true); // false
    println!("true  ^   false   =>  {}", b_true ^ b_false); // true
    println!("false ^   true    =>  {}", b_false ^ b_true); // true
    println!("false ^   false   =>  {}", b_false ^ b_false); // false
}
