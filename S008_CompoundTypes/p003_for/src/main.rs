fn main() {
    let mut scores: [i32; 4] = [0; 4];
    scores[0] = 118;
    scores[1] = 115;
    scores[2] = 101;
    scores[3] = 100;

    //let mut index = 0;

    // while index < scores.len() {
    //     println!("{}", scores[index]);
    //     index += 1;
    // }

    for element in scores {
        println!("{element}");
    }

    println!("===================");

    for value in scores {
        println!("{value}");
    }

    println!("===================");

    // for element in scores {
    //     element = 12;
    // }
}
