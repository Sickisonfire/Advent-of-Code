use std::fs;
use std::time::Instant;

fn part_one(input: &String) -> i32 {
    //let vec: Vec<i32> = content.lines().map(|x| x.parse::<i32>().unwrap()).collect();
    let mut prev = i32::MAX;
    let mut total = 0;
    for line in input.lines() {
        let current = line.parse::<i32>().unwrap();
        if current > prev {
            total += 1;
        }
        prev = current;
    }
    total
}

fn part_two(input: &String, win_length: i32) -> i32 {
    let mut prev = i32::MAX;
    let mut total = 0;
    let mut current: i32;
    let vec: Vec<i32> = input.lines().map(|x| x.parse::<i32>().unwrap()).collect();
    for win in vec.windows(win_length as usize) {
        current = win.iter().sum();
        if current > prev {
            total += 1;
        }
        prev = current;
    }
    total
}
fn main() {
    let content = fs::read_to_string("input").expect("error loading file");

    let now = Instant::now();
    let p_one = part_one(&content);
    let elapsed = now.elapsed();
    println!("PART 1: {}, in: {:?}", p_one, elapsed);
    let now = Instant::now();
    let p_two = part_two(&content, 3);
    let elapsed = now.elapsed();
    println!("PART 2: {}, in: {:?}", p_two, elapsed);
}
