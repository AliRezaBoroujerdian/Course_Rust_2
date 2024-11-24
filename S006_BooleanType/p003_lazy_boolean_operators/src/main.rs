fn main() {
    let b_true = true;
    let b_false = false;

    println!("true  ||   true    =>  {}", b_true || b_true);
    println!("true  &&   true    =>  {}", b_true && b_true);

    // error
    //println!("true  ^^   true    =>  {}", b_true ^^ b_true);

    println!("=====================");

    //panic!();

    // let result = b_true | panic!();
    // println!("{result}");

    let result = b_true || panic!();
    println!("{result}");

    println!("=====================");

    // let result = b_false & panic!();
    // println!("{result}");

    let result = b_false && panic!();
    println!("{result}");
}
