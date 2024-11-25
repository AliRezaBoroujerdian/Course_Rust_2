fn main() {
    println!("username: ");
    let mut input: String = String::new();

    /*
     *
     * std => standard library
     * io => input/output
     */
    std::io::stdin().read_line(&mut input);

    println!("input => {input}");

    ////////////

    println!("password: ");
    let mut password: String = String::new();
    std::io::stdin().read_line(&mut password);

    println!("input 2 => {password}");
}
