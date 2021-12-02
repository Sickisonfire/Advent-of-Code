#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

fn part_one(input: &str) {
    let lines: Vec<Vec<&str>> = input
        .lines()
        .map(|x| x.split_whitespace().collect())
        .collect();

    let mut pos = Position { x: 0, y: 0 };

    for line in &lines {
        let val = &line[1].parse().unwrap();
        match line[0] {
            "forward" => pos.x = pos.x + val,
            "down" => pos.y = pos.y + val,
            "up" => pos.y = pos.y - val,
            _ => println!("input not valid"),
        };
    }
    println!("{}, {} = {}", pos.x, pos.y, pos.x * pos.y);
}

fn part_two(input: &str) {
    let lines: Vec<Vec<&str>> = input
        .lines()
        .map(|x| x.split_whitespace().collect())
        .collect();

    let mut pos = Position { x: 0, y: 0 };
    let mut aim = 0;

    for line in &lines {
        let val = &line[1].parse().unwrap();
        match line[0] {
            "forward" => {
                pos.x = pos.x + val;
                pos.y = pos.y + val * aim;
            }
            "down" => aim = aim + val,
            "up" => aim = aim - val,
            _ => println!("input not valid"),
        };
    }
    println!("{}, {} = {}", pos.x, pos.y, pos.x * pos.y);
}
fn main() {
    let input = include_str!("../input");
    part_one(input);
    part_two(input);
}
