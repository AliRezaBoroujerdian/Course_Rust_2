fn main() {
    let result = loop {
        println!("begin");
        break 6;
        println!("end");
    };

    println!("{result}");
}
