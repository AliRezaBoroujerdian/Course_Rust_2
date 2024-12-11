fn main() {
    let username = String::from("AliRezaBoroujerdian");
    let password = String::from("12@sR");

    'first_loop: loop {
        println!("==== Login ====");
        println!();
        println!("username: ");

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
    //https://code.visualstudio.com/docs/languages/rust#_debugging
}
