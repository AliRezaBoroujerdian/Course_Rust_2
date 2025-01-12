fn main() {
    let username = get_username(1);
    //println!("{}", username);

    match username {
        Some(username) => println!("{}", username),
        None => {
            println!("User not found!")
        } // None => {}
    }
}

// fn get_username(user_id: u32) -> String {
//     if user_id == 1 {
//         String::from("AliReza")
//     } else {
//         String::from("")
//     }
// }

fn get_username(user_id: u32) -> Option<String> {
    if user_id == 1 {
        Some(String::from("AliReza"))
    } else {
        None
    }
}
