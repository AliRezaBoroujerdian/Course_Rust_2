const MY_CONSTANT_VALUE: i32 = 10;

fn main() {
    let a = 15;

    const b: i32 = 44;

    println!("{a}");
    println!("{b}");

    let c: i32;
    c = 70;
    println!("{c}");

    // error
    // const d: i32;
    // d = 40;

    let e = 16;
    let f = e;
    println!("{f}");

    // error
    //const g: i32 = f;

    println!("{MY_CONSTANT_VALUE}");
}
