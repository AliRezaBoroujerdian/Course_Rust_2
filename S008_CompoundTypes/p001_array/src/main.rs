fn main() {
    let my_array = [1, 5, 66, 5, 33];

    let cities = ["Tehran", "Shiraz", "Isfahan"];

    let number: i32 = 6;
    let scores: [i32; 4] = [108, 113, 101, 118]; // fix 4

    println!("{}", scores[0]);
    println!("{}", scores[1]);

    let third = scores[2];

    println!("{third}");

    let array = [3; 5];

    println!("{}", array[0]);
    println!("{}", array[1]);
    println!("{}", array[2]);

    // println!("{:?}", scores);
    // println!("{:#?}", scores);
}
