fn main() {
    // let scores: [i32; 5] = [118, 115, 101, 100];

    let mut scores: [i32; 4] = [0; 4];
    scores[0] = 118;
    scores[1] = 115;
    scores[2] = 101;
    scores[3] = 100;

    //scores[14] = 116;

    let mut index = 0;

    // while index < 4 {
    //     println!("{}", scores[index]);
    //     index += 1;
    // }

    while index < scores.len() {
        println!("{}", scores[index]);
        index += 1;
    }

    println!("======================");

    println!("{}", scores[1]);
    //println!("{}", scores);

    println!("======================");

    println!("{:?}", scores);

    println!("======================");

    println!("{:#?}", scores);

    println!("======================");

    let mut index = 0;

    while index < scores.len() {
        println!("score {} => {}", index + 1, scores[index]);
        index += 1;
    }
    println!("======================");

    let mut index = 0;

    while index < scores.len() {
        scores[index] = 12;

        index += 1;
    }

    let mut index = 0;

    while index < scores.len() {
        println!("score {} => {}", index + 1, scores[index]);
        index += 1;
    }
}
