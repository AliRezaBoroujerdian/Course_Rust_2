fn main() {
    let username = String::from("AliRezaBoroujerdian");
    let password = String::from("12@tUz");

    // loop {
    //     println!("==== Login ====");
    //     println!("username: "); // flush stack

    //     let mut input_username = String::new();
    //     std::io::stdin().read_line(&mut input_username);

    //     if username != input_username.trim() {
    //         continue;
    //     }

    //     println!("password: ");

    //     let mut input_password = String::new();
    //     std::io::stdin().read_line(&mut input_password);

    //     if password == input_password.trim() {
    //         break;
    //     }
    // }

    'first_loop: loop {
        println!("==== Login ====");
        println!("username: "); // flush stack

        let mut input_username = String::new();
        std::io::stdin().read_line(&mut input_username);

        if username.to_lowercase() != input_username.trim().to_lowercase() {
            println!("Wrong username");
            continue;
        }

        loop {
            println!("password: ");

            let mut input_password = String::new();
            std::io::stdin().read_line(&mut input_password);

            if password == input_password.trim() {
                break 'first_loop;
            }
        }
    }

    println!("login");
}