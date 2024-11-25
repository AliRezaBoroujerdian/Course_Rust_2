fn main() {
    let b_true = true;
    let b_false = false;

    println!("true  |   true    =>  {}", b_true || b_true); // true
    println!("true  &   true    =>  {}", b_true && b_true); // true

    // error
    //println!("true  ^   true    =>  {}", b_true ^^ b_true); // false

    //panic!();

    // error
    //let result = b_true | panic!();

    let result = b_true || panic!();
    println!("{result}");

    // error
    //let result = b_false & panic!();

    let result = b_false && panic!();
    println!("{result}");
}
