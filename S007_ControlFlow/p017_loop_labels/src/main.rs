fn main() {
    // loop {
    //     println!("loop 1");
    //     loop {
    //         println!("loop 2");
    //         break;
    //     }
    // }

    // 'first_loop: loop {
    //     println!("loop 1");
    //     loop {
    //         println!("loop 2");
    //         break 'first_loop;
    //     }
    // }

    'first_loop: loop {
        println!("loop 1");
        loop {
            println!("loop 2");
            'third_loop: loop {
                println!("loop 3");
                loop {
                    println!("loop 4");
                    break 'third_loop;
                }
            }
        }
    }
}
