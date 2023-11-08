pub fn solve() {
    let input = include_str!("../../inputs/two");

    // 1 A X Rock
    // 2 B Y Paper
    // 3 C Z Scissors
    // win 6
    // draw 3
    // loss 0
    //
    // p2

    let parsed: Vec<Vec<&str>> = input
        .split("\n")
        .map(|x| x.split(" ").collect::<Vec<_>>())
        .take_while(|x| x[0] != "")
        .collect();

    let mut points = 0;

    for entry in &parsed {
        match entry[1] {
            "X" => {
                points += 1;
                match entry[0] {
                    "A" => points += 3,
                    "C" => points += 6,
                    _ => (),
                };
            }
            "Y" => {
                points += 2;
                match entry[0] {
                    "A" => points += 6,
                    "B" => points += 3,
                    _ => (),
                };
            }
            "Z" => {
                points += 3;
                match entry[0] {
                    "B" => points += 6,
                    "C" => points += 3,
                    _ => (),
                };
            }
            _ => (),
        };
    }

    println!("{points}");
    points = 0;
    //p2
    // X = loss
    // Y = draw
    // Z = win
    for entry in &parsed {
        match entry[0] {
            "A" => {
                match entry[1] {
                    "X" => points += 3,
                    "Y" => points += 4,
                    "Z" => points += 8,
                    _ => (),
                };
            }
            "B" => {
                match entry[1] {
                    "X" => points += 1,
                    "Y" => points += 5,
                    "Z" => points += 9,
                    _ => (),
                };
            }
            "C" => {
                match entry[1] {
                    "X" => points += 2,
                    "Y" => points += 6,
                    "Z" => points += 7,
                    _ => (),
                };
            }
            _ => (),
        };
    }
    println!("p2 {points}");
}
