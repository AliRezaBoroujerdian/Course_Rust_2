fn main() {
    let a = 6;

    println!("{a}");

    {
        let b = 5;
        println!("{b}");
    }

    // error
    //println!("{b}");
}
