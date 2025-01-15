fn main() {
    let s = "AliReza";
    println!("Hi {}!", s);

    let s = format!("Hello"); // => "Hello"
    println!("{}", s);

    let s = format!("Hello, {}!", "world"); // => "Hello, world!"
    println!("{}", s);

    let s = format!("The number is {}", 1); // => "The number is 1"
    println!("{}", s);

    let s = format!("{:?}", (3, 4)); // => "(3, 4)"
    println!("{}", s);

    let s = format!("{value}", value = 4); // => "4"
    println!("{}", s);

    let people = "Rustaceans";
    let s = format!("Hello {people}!"); // => "Hello Rustaceans!"
    println!("{}", s);

    let s = format!("{} {}", 1, 2); // => "1 2"
    println!("{}", s);

    let s = format!("{:04}", 42); // => "0042" with leading zeros
    println!("{}", s);

    let s = format!("{:#?}", (100, 200));
    // => "(
    //       100,
    //       200,
    //     )"
    println!("{}", s);

    let s = format!("{1} {} {0} {} {1}", 1, 2); // => "2 1 1 2 2"
    println!("{}", s);

    let s = format!("{name} {}", 1, name = 2); // => "2 1"
    println!("{}", s);

    let s = format!("{a} {c} {b}", a = "a", b = 'b', c = 3); // => "a 3 b"
    println!("{}", s);

    let s = format!("Hello {{}}"); //"Hello {}"
    println!("{}", s);

    //https://doc.rust-lang.org/std/fmt/

    let username = String::from("AliReza");
    let mut header = String::from("=========== Hi ");
    header.push_str(&username);
    header.push_str("! ===========");
    println!("{}", header);

    let header = format!("=========== Hi {}! ===========", username);
    println!("{}", header);
}
