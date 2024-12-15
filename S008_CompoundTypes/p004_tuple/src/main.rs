fn main() {
    let user = (
        String::from("AliReza"),
        String::from("Boroujerdian"),
        1993,
        82,
    );

    println!("Hi {} {}!", user.0, user.1);

    let age = 2024 - user.2;
    println!("{age}");

    println!("=================");

    let point_2d: (i32, i32) = (3, 4);

    let (x, y) = point_2d;

    println!("x = {}, y = {}", x, y);

    let unit = (); // We can consider unit() to be an empty tuple.

    /*
       Similarly, unit () is a type created by the Move source language in order to
       be expression based. The unit value () does not result in any runtime value.
       We can consider unit() to be an empty tuple, and any restrictions that apply to
       tuples also apply to unit.
    */
}
