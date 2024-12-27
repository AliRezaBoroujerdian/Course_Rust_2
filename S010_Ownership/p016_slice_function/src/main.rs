fn main() {
    let s1 = String::from("Hello, world!");

    let s1_first_5 = first_5_char(&s1);

    let s2 = "Hello, world!";

    let s2_first_5 = first_5_char(s2);

    println!("{s1_first_5}");
    println!("{s2_first_5}");
}

// fn first_5_char(s: &String) -> &str {
//     &s[..5]
// }

fn first_5_char(s: &str) -> &str {
    &s[..5]
}
