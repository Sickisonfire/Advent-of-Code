pub fn solve() {
    let input = include_str!("../../inputs/input_one");
    part_one(input);
    part_two(input);
}

fn part_one(input: &str) {
    let iter: Vec<&str> = input.split("\n").collect();
    let mut sums = Vec::new();
    let mut sum = 0;

    for entry in iter {
        match entry {
            "" => {
                sums.push(sum);
                sum = 0;
            }
            n => {
                sum += n.parse::<usize>().unwrap_or(0);
            }
        }
    }
    println!("max is {}", sums.iter().max().unwrap());
}

fn part_two(input: &str) {
    for entry in input.split('\n') {}
}
