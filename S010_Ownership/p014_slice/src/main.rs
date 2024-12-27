fn main() {
    let s1 = String::from("Hello, World!");

    /*
     * 0    => H
     * 1    => e
     * 2    => l
     * 3    => l
     * 4    => o
     * 5    => ,
     * 6    =>
     * 7    => W
     * 4    => o
     * 9    => r
     * 10   => l
     * 11   => d
     * 12   => !
     */

    let r1 = &s1;

    let slice: &str = &s1[0..5]; // &str string slice
    println!("{slice}");

    let slice = &s1[..5];
    println!("{slice}");

    let slice = &s1[7..13];
    println!("{slice}");

    let slice = &s1[7..];
    println!("{slice}");

    let len = s1.len();

    let slice = &s1[7..];
    println!("{slice}");

    // let slice = &s1[7..14];
    // println!("{slice}");

    let slice = &s1[..];
    println!("{slice}");
}

/*
 *
 * Slice
 * Slices let you reference a contiguous sequence of elements in a collection
 * rather than the whole collection.
 * A slice is a kind of reference, so it does not have ownership.
 *
 *  */
