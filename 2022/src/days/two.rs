pub fn solve() {
    let input = include_str!("../../inputs/input_two");

    // 1 A X Rock
    // 2 B Y Paper
    // 3 C Z Scissors
    // win 6
    // draw 3
    // loss 0

    let parsed: Vec<Vec<&str>> = input
        .split("\n")
        .map(|x| x.split(" ").collect::<Vec<_>>())
        .take_while(|x| x[0] != "")
        .collect();

    let mut points = 0;

    for entry in parsed {
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
}
