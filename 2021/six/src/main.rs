//instead of simulating every single fish do it by days [0-8]
fn main() {
    let fishes: Vec<_> = include_str!("input")
        .trim()
        .split(',')
        .map(|x| {
            x.parse::<i32>()
                .expect("all items in list should be numbers")
        })
        .collect();
    println!("{:?}", fishes);

    // init days array
    // [0,1,2,3,4,5,6,7,8]
    let mut days_count: Vec<i64> = vec![0; 9];

    // add initial state
    for fish in fishes {
        days_count[fish as usize - 1] += 1;
    }

    // simulate

    let days_p1 = 80;
    let days_p2 = 256;
    for _ in 1..days_p2 {
        let new_fish = days_count[0];
        days_count[0] = 0;

        for i in 0..8 {
            days_count[i] = days_count[i + 1];
        }
        days_count[6] += new_fish;
        days_count[8] = new_fish;
    }
    let fish_count: i64 = days_count.iter().sum();
    println!("{:?}", fish_count);
}
