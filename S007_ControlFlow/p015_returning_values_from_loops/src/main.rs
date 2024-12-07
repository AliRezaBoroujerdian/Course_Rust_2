fn main() {
    let value = loop {
        // while => error
        println!("begin");
        break 6;
        println!("end");
    }; // <= ;;;;;;;;;;;

    println!("{value}");
}
