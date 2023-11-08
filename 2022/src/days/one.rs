pub fn solve() {
    let input = include_str!("../../inputs/one");
    part_one(input);
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

    sums.sort();
    sums.reverse();

    let top_three_sum: usize = sums.iter().take(3).sum();
    println!("sum of top 3 is: {:?}", top_three_sum);
}
