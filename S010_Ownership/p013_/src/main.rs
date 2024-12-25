fn main() {
    let mut x = 10;
    let mut y = 15;

    println!("x = {}, y = {}", x, y);

    swap(&mut x, &mut y);

    println!("x = {}, y = {}", x, y);
}

fn swap(x: &mut i32, y: &mut i32) {
    let temp = *x;
    *x = *y;
    *y = temp;
}
