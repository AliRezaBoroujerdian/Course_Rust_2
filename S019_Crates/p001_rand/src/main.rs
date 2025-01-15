use rand::Rng;

fn main() {
    let x: u8 = rand::random();
    println!("{}", x);

    let mut x = rand::thread_rng().gen_range(-5..5);
    println!("{}", x);

    let mut x = rand::thread_rng().gen_range(-5..=5);
    println!("{}", x);

    println!("====================");
    if rand::random() {
        println!("rand!");
    }
    println!("====================");
}
