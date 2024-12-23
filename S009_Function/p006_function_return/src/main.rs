fn main() {
    let r: i32 = add(3, 4);
    println!("{r}");

    println!("{}", add(6, 9));
}

// fn add(x: i32, y: i32) -> i32 {
//     let result = x + y;
//     return result;
// }

// fn add(x: i32, y: i32) -> i32 {
//     let result = x + y;
//     result
// }

// fn add(x: i32, y: i32) -> i32 {
//     return x + y;
// }

fn add(x: i32, y: i32) -> i32 {
    x + y
}
