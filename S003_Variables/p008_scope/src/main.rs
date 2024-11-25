fn main() {
    let mut a: i32 = 50;

    {
        let b = 31;
        println!("{b}");

        a = 100;
    }
    println!("{a}");
    // error
    //println!("{b}");
}
