fn main() {
    println!("username: ");

    let mut username: String = String::new();

    /*
     *
     * std => standard library
     * io => input/output
     *
     */
    std::io::stdin().read_line(&mut username);

    println!("input => {username}");

    println!("password: ");
    let mut password: String = String::new();
    std::io::stdin().read_line(&mut password);

    println!("input => {password}");
}
