fn main() {
    // loop {
    //     println!("Loop 1");
    //     loop {
    //         println!("Loop 2");
    //         break;
    //     }
    // }

    // 'first_loop: loop {
    //     println!("Loop 1");
    //     loop {
    //         println!("Loop 2");
    //         break 'first_loop;
    //     }
    // }

    'first_loop: loop {
        println!("Loop 1");
        loop {
            println!("Loop 2");
            'third_loop: loop {
                println!("Loop 3");
                loop {
                    println!("Loop 4");
                    break 'third_loop;
                }
            }
        }
    }

    println!("end");
}
