fn main() {
    // Immutable Variable
    let a = 15;

    // error
    //a = 17;

    // Mutable variable
    let mut b = 15;

    b = 32;
    println!("line 12 => b is {b}");

    b = b + 12;
    println!("line 15 => b is {b}");

    // Shadowing

    let c = 17;
    let c = 18; // first c is shadowed by the second c.

    println!("line 22 => c is {c}");

    let d = 156;
    let d = d + 16;

    println!("line 27 => d is {d}");

    let e: i32 = 12;
    let e: i64 = 3000000000;

    println!("line 32 => e is {e}");

    let f: i32 = 15;
    let f: String = String::from("AliReza");

    println!("line 37 => f is {f}");

    let g = 12;

    {
        let g = g * 2;
        println!("line 43 => g is {g}");
    }

    println!("line 46 => g is {g}");
}
