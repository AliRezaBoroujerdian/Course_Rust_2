fn main() {
    let result = 'first_loop: loop {
        println!("Loop 1");
        loop {
            println!("Loop 2");
            loop {
                println!("Loop 3");
                loop {
                    println!("Loop 4");
                    break 'first_loop 6;
                }
            }
        }
    };

    println!("{result}");
}
