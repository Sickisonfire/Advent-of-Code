const MAX_BITS: usize = 5;

fn parse_input(input: &str) -> Vec<i32> {
    let mut table: Vec<i32> = vec![0; MAX_BITS];

    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            match c {
                '0' => table[i] -= 1,
                '1' => table[i] += 1,
                _ => (),
            };
        }
    }
    table
}

fn part_one(table: &Vec<i32>) {
    let mut gamma = 0;
    let mut epsilon = 0;
    for (i, &v) in table.into_iter().enumerate() {
        if v > 0 {
            gamma |= 1 << (MAX_BITS - i - 1);
        } else {
            epsilon |= 1 << (MAX_BITS - i - 1);
        };
    }
    println!("{} * {} = {}", gamma, epsilon, gamma * epsilon);
}

// loop over table
//      loop over entries
//          filter entries
//          repeat
//
fn filter_entries(table: Vec<i32>, mut entries: Vec<&str>) -> Vec<&str> {
    for (i, &v) in table.iter().enumerate() {
        let bit_val = if v >= 0 { '1' } else { '0' };
        let mut entries_filtered = Vec::new();
        for line in &entries {
            if line.chars().nth(i).unwrap() == bit_val {
                entries_filtered.push(line);
            }
            entries.clear();
            entries = entries_filtered.clone();
        }

        if entries.len() == 1 {
            return entries;
        }
    }
    entries
}
fn main() {
    let input = include_str!("example");
    let table = parse_input(input);
    //part_one(&table);

    // loop over input
    // determine most common value of first char
    // save entries with most common
    // repeat with next char
    // until one entry left
    //
    let oxygen = 0;
    let c02 = 0;

    //o2
    let mut entries = input.lines().collect::<Vec<_>>();
}
