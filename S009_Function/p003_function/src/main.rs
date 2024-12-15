fn main() {
    println!("begin");

    say_hello(String::from("AliReza"));

    let name = String::from("Bruce");
    say_hello(name);

    say_hello(String::from("Harley"));

    println!("end");
}

fn say_hello(name: String) {
    println!("Hello, {name}");
}
