fn main() {
    let username = get_username(2);

    // match username {
    //     Some(username) => println!("{}", username),
    //     None => {}
    // }

    if let Some(u) = &username {
        println!("{}", u);
    }

    if let Some(u) = username {
        println!("{}", u);
    }
}

fn get_username(user_id: u32) -> Option<String> {
    if user_id == 1 {
        Some(String::from("AliReza"))
    } else {
        None
    }
}
