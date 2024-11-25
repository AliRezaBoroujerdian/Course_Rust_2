fn main() {
    //immutable
    let a = 10;
    println!("line 4 => {a}");

    // error
    //a = 11;

    // mutable
    let mut b = 15;
    b = 100;
    println!("line 12 => {b}");

    b = b + 10;
    println!("line 15 => {b}");

    //Shadowing

    let c = 12;
    let c = 15; // first c is shadowed by the second c.

    println!("line 22 => {c}");

    let d = 12;
    let d = d + 10;

    println!("line 27 => {d}");

    let e: i32 = 17;
    let e: i64 = 3000000000;

    println!("line 32 => {e}");

    let f: i32 = 12;
    let f: String = String::from("Rust");

    println!("line 37 => {f}");

    // scope

    let g: i32 = 10;
    println!("line 42 => {g}");

    {
        let g: i32 = 15;
        println!("line 46 => {g}");
    }

    println!("line 49 => {g}");
}
