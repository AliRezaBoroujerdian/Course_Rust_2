fn main() {
    //let scores: [i32; 4] = [108, 113, 101, 118];

    let mut scores: [i32; 4] = [0; 4];
    scores[0] = 100;
    scores[1] = 113;
    scores[2] = 101;
    scores[3] = 118;

    //scores[4] = 109;

    let mut index = 0;

    // while index < 4 {
    //     println!("{}", scores[index]);
    //     index += 1;
    // }

    while index < scores.len() {
        println!("{}", scores[index]);
        index += 1;
    }

    println!("===========================");

    println!("{}", scores[0]);
    //println!("{}", scores);

    println!("===========================");

    println!("{:?}", scores);

    println!("===========================");

    println!("{:#?}", scores);

    println!("===========================");

    let mut index = 0;

    while index < scores.len() {
        println!("score {} => {}", index + 1, scores[index]);
        index += 1;
    }

    println!("===========================");

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
