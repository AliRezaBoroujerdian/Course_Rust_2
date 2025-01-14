fn main() {
    let header = get_header(1).unwrap();
    println!("{}", header);
}

fn get_header(user_id: u32) -> Result<String, String> {
    let username: String = get_username(user_id)?;

    let mut header = String::from("=========== Hi ");
    header.push_str(&username);
    header.push_str("! ===========");

    Ok(header)
}

fn get_username(user_id: u32) -> Result<String, String> {
    match user_id {
        1 => Ok(String::from("AliReza")),
        _ => Err(String::from("User not found")),
    }
}
