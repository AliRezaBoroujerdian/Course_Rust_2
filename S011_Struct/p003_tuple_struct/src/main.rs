fn main() {
    // tuple
    let rgb_color_red = (255, 0, 0);

    // tuple struct
    struct RGB(i32, i32, i32);
    let red = RGB(255, 0, 0);

    println!("{}", rgb_color_red.0);
    println!("{}", red.0);
}
