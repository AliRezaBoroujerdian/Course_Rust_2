enum Command {
    Undo,
    Redo,
    AddText(String),
    ChangeColor(u8, u8, u8),
    Replace { from: String, to: String },
    Quit,
}

impl Command {
    fn print_menu() {
        println!("1, Undo");
        println!("2, Redo");
        println!("3, AddText");
        println!("4, ChangeColor");
        println!("5, Replace");
        println!("6, Quit");
    }
}

fn main() {
    let cmd = Command::Undo;
    let cmd = Command::AddText(String::from("Rust"));
    let cmd = Command::ChangeColor(255, 0, 0);
    let cmd = Command::Replace {
        from: String::from("a"),
        to: String::from("ba"),
    };

    Command::print_menu();
}
