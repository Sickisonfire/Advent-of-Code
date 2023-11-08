type Parsed = Vec<Vec<Vec<i32>>>;
pub fn solve() {
    // [[[1,2], [3,4]]]
    // [[[3,4], [1,2]]]
    // [[[3,4], [3,4]]]
    // [[[3,4], [3,6]]]
    // [[[3,5], [3,4]]]
    let input = include_str!("../../inputs/input_four");

    // [["1-2", "3-4"],..]
    let mut parsed: Vec<Vec<_>> = input.split("\n").map(|x| x.split(",").collect()).collect();

    let len = parsed.len();
    // [[[1,2], [3,4]],..]
    let parsed: Parsed = parsed[0..len - 1]
        .iter_mut()
        .map(|x| {
            x.iter_mut()
                .map(|y| y.split("-").map(|z| z.parse::<i32>().unwrap()).collect())
                .collect()
        })
        .collect();
    part_one(&parsed);
    part_two(&parsed);
    // println!("{:?}", parsed);
}

fn part_one(parsed: &Parsed) {
    let mut acc = 0;
    for entry in parsed {
        let one = &entry[0];
        let two = &entry[1];
        if one[0] < two[0] {
            if one[1] >= two[1] {
                acc += 1;
            };
        } else if one[0] == two[0] {
            if one[1] >= two[1] || one[1] <= two[1] {
                acc += 1;
            };
        } else {
            if one[1] <= two[1] {
                acc += 1;
            };
        }
        // println!("{:?}", smol_big);
    }
    println!("{:?}", acc);
}

fn part_two(parsed: &Parsed) {
    let mut acc = parsed.len();

    for entry in parsed {
        let one = &entry[0];
        let two = &entry[1];

        if one[0] < two[0] && one[1] < two[0] {
            acc -= 1;
        } else if one[0] > two[0] && one[0] > two[1] {
            acc -= 1;
        }
    }

    println!("{:?}", acc);
}
