fn main() {
    let s1: String = String::from("Rust");
    do_something_else(&s1);
    println!("s1: {s1}");
}

fn do_something_else(s: &String) /* s is a reference to a String */
{
    println!("s: {s}");
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

/*
 *
 *
 * A reference is like a pointer in that
 * it’s an address we can follow to access the data stored at that address;
 * that data is owned by some other variable.
 * Unlike a pointer, a reference is guaranteed to point to a valid value
 * of a particular type for the life of that reference.
 *
 *
 * The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it.
 * Because it does not own it, the value it points to will not be dropped when the reference stops being used.
 *
 *
 * We call the action of creating a reference borrowing.
 * As in real life, if a person owns something, you can borrow it from them.
 * When you’re done, you have to give it back. You don’t own it.
 *
 *
 */
