fn main() {
    let mut v1: Vec<String> = Vec::new();

    v1.push(String::from("One"));
    v1.push(String::from("Two"));
    v1.push(String::from("Three"));

    let mut v2 = Vec::new();
    v2.push("Rust");

    let mut v3 = Vec::new();
    v3.push(1);
    v3.push(2);

    let v4 = vec![1, 55, 4, 77];

    let v5 = vec!["One", "Two", "Three"];

    let s1 = v5[0];
    println!("s1 => {}", s1);

    let s2 = v5[1];
    println!("s2 => {}", s2);

    //error
    //let s3 = v1[0];

    let s3 = &v1[0]; // can panic
    println!("s3 => {}", s3);

    let s4 = v1.get(0);

    if let Some(s) = s4 {
        println!("s4 => {}", s);
    }

    println!("===================");

    for s in &v1 {
        println!("{}", s);
    }

    println!("===================");

    for s in &mut v1 {
        s.push_str("!");
    }

    for s in &v1 {
        println!("{}", s);
    }
}
