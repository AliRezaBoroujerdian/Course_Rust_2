// use rand::random;
// use rand::Rng;

// use rand::*;

use rand::{random, Rng};

fn main() {
    let x: u8 = random();
    println!("{}", x);

    let mut x = rand::thread_rng().gen_range(-5..5);
    println!("{}", x);
}
