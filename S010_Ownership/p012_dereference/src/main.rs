fn main() {
    let mut scores: [i32; 4] = [0; 4];
    scores[0] = 118;
    scores[1] = 115;
    scores[2] = 101;
    scores[3] = 100;

    for element in scores {
        println!("{}", element);
    }

    println!("=============================");

    for element in &scores {
        println!("{}", element);
    }

    println!("=============================");

    for element in &mut scores {
        *element = 12;
    }

    println!("=============================");

    for element in scores {
        println!("{}", element);
    }
}
