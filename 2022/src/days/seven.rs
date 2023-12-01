struct FS {}

enum FSEntry {
    File {
        name: &'static str,
        size: u64,
    },
    Directory {
        children: Vec<FSEntry>,
        name: &'static str,
        size: u64,
    },
}

pub fn solve() {
    let input = include_str!("../../inputs/seven");
    let mut input_iter = input.split("\n");
    while let Some(line) = input_iter.next() {
        let mut line_iter = line.split(" ");
        let first = line_iter.next().unwrap();
        match first {
            "$" => {
                //parse command
                //take while first not $
                match line_iter.next().unwrap() {
                    "cd" => {}
                    "ls" => {}
                    x => panic!("unknown cmd: {}", x),
                }
            }

            _ => {}
        }

        // println!("{:?}", line);
    }
}
