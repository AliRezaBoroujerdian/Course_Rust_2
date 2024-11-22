fn main() {
    let a = 12;

    let b = 13_i8;
    println!("{b}");

    let c = 14_u64;
    println!("{c}");

    let d = 12_530;
    println!("{d}");

    let e = 12_530.1;
    println!("{e}");

    let f = 1_2_3_4;
    println!("{f}");

    let g = 0xe; // Hex => e => 14
    println!("{g}");

    let h = 0o10; // Octal => 10 => 8
    println!("{h}");

    let i = 0b0001_0000; // Binary => 00010000 => 16
    println!("{i}");

    let j: u8 = b'A'; // Byte => 'A' => 65
    println!("{j}");
}
