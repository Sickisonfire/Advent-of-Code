use std::str;
struct Stack<T> {
    items: Vec<T>,
}

impl Stack<Item> {
    fn new() -> Stack<Item> {
        Stack { items: vec![] }
    }
    fn peek(&self) -> Option<&Item> {
        if self.items.len() == 0 {
            return None;
        } else {
            Some(&self.items[self.items.len() - 1])
        }
    }

    fn push(&mut self, item: Item) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<Item> {
        self.items.pop()
    }

    fn pop_bulk(&mut self, number: usize) -> Vec<Item> {
        let mut items = vec![];
        for _ in 0..number {
            if let Some(i) = self.items.pop() {
                items.push(i);
            };
        }
        items
    }
    fn print(&self) {
        let mut s = "|".to_owned();
        for i in &self.items {
            let name = i.name;
            s = format!("{s} [{name}]");
        }
        println!("{s}");
    }
}
#[derive(PartialEq, Eq, Debug)]
struct Item {
    name: char,
}

#[derive(Debug)]
enum Ops {
    Move { from: u32, to: u32, amount: u32 },
}

pub fn solve() {
    let input = include_str!("../../inputs/five");
    let mut parsed: Vec<&str> = input.split("\n").take_while(|x| *x != "").collect();
    parsed.reverse();

    let lut_idxs = vec![1, 5, 9, 13, 17, 21, 25, 29, 33];
    let mut stacks = vec![];
    for _ in 0..9 {
        stacks.push(Stack::new());
    }
    seed_stacks(&parsed, &mut stacks, lut_idxs);

    let parsed: Vec<&str> = input.split("\n").skip(10).collect();
    let mut ops: Vec<Ops> = vec![];

    for op in parsed {
        match op {
            "" => continue,
            op => {
                let p = op
                    .split(" ")
                    .filter_map(|x| x.parse::<u32>().ok())
                    .collect::<Vec<u32>>();
                ops.push(Ops::Move {
                    from: p[1],
                    to: p[2],
                    amount: p[0],
                });
            }
        }
    }

    // part_one(&ops, &mut stacks);
    part_two(&ops, &mut stacks);
}

fn part_one(ops: &Vec<Ops>, stacks: &mut Vec<Stack<Item>>) {
    for op in ops {
        match op {
            Ops::Move { from, to, amount } => {
                for _ in 0..*amount {
                    let item = stacks[*from as usize - 1].pop();
                    if let Some(i) = item {
                        stacks[*to as usize - 1].push(i);
                    }
                }
            }
        }
    }
    let mut solution = "".to_string();
    for s in stacks {
        let v = s.peek().unwrap().name;
        solution = format!("{solution}{v}");
    }
    println!("{solution}");
}

fn part_two(ops: &Vec<Ops>, stacks: &mut Vec<Stack<Item>>) {
    for op in ops {
        match op {
            Ops::Move { from, to, amount } => {
                let items = stacks[*from as usize - 1].pop_bulk(*amount as usize);

                for v in items.into_iter().rev() {
                    stacks[*to as usize - 1].push(v);
                }
            }
        }
    }

    let mut solution = "".to_string();
    for s in stacks {
        let v = s.peek().unwrap().name;
        solution = format!("{solution}{v}");
    }
    println!("{solution}");
}

fn seed_stacks(parsed: &Vec<&str>, stacks: &mut Vec<Stack<Item>>, lut: Vec<usize>) {
    for i in parsed.iter().skip(1) {
        let bytes = i.as_bytes();
        // println!("{:?}", str::from_utf8(bytes));

        for (idx, s) in lut.iter().enumerate() {
            let c = bytes[*s];
            if c != b' ' {
                stacks[idx].push(Item { name: c as char });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_stack_implementation() {
        let mut s = Stack::new();
        s.push(Item { name: 'A' });
        s.push(Item { name: 'B' });
        let b = s.peek().unwrap().name;
        assert_eq!(b, 'B');

        s.push(Item { name: 'C' });
        let c = s.pop().unwrap().name;
        assert_eq!(c, 'C');
        let b = s.pop().unwrap().name;
        assert_eq!(b, 'B');
        let a = s.pop().unwrap().name;
        assert_eq!(a, 'A');
        let n = s.pop();
        assert_eq!(n, None);

        let n = s.peek();
        assert_eq!(n, None);
    }
}
