fn main() {
    let mut scores: [i32; 4] = [0; 4];
    scores[0] = 100;
    scores[1] = 113;
    scores[2] = 101;
    scores[3] = 118;

    // let mut index = 0;

    // while index < scores.len() {
    //     println!("{}", scores[index]);
    //     index += 1;
    // }

    for element in scores {
        println!("{element}");
    }

    println!("==========");

    for value in scores {
        println!("{value}");
    }

    // for element in scores {
    //     element = 12;
    // }
}
