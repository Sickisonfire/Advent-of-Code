pub fn solve() {
    // part one
    // let win_size = 4;

    // part two
    let win_size = 14;

    let input = include_bytes!("../../inputs/six");
    for i in 0..input.len() - win_size {
        let win_start = i;
        let win_end = i + win_size;
        let win = &input[win_start..win_end];
        if !has_duplicate(win) {
            println!("{}", win_end);
            break;
        }
    }
}

fn has_duplicate(slice: &[u8]) -> bool {
    let mut res = false;
    for i in 0..slice.len() - 1 {
        let b = slice[i];
        if slice[i + 1..].contains(&b) {
            res = true;
        }
    }
    res
}
