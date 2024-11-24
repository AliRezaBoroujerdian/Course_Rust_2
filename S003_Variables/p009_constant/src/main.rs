const MY_CONSTANT_VALUE: i32 = 12;

fn main() {
    let a: i32 = 345;

    const b: i32 = 256;

    println!("{a}");
    println!("{b}");

    let c: i32;
    c = 77;

    // error
    //const d: i32;
    //d = 18;

    // error
    //const mut e: i32 = 16;

    println!("{MY_CONSTANT_VALUE}");

    let d = 12;

    // error
    //const e: i32 = d;
}
