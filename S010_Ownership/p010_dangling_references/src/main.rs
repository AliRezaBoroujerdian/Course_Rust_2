fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("Rust");

    &s
}

/*
 * * Rules
 * - At any given time, you can have either one mutable reference or any number of immutable references.
 * - References must always be valid.
 */
