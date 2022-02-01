#[derive(Debug, Clone)]
enum ValueStatus {
    Marked(i32),
    Unmarked(i32),
}

#[derive(Debug, Clone)]
struct Board {
    values: Vec<ValueStatus>,
}
impl Board {
    fn new(rows: &mut Vec<Vec<i32>>) -> Board {
        Board {
            values: rows
                .into_iter()
                .take(5)
                .flatten()
                .map(|x| ValueStatus::Unmarked(*x))
                .collect::<Vec<ValueStatus>>(),
        }
    }
    fn mark_value(&mut self, value: &i32) {
        for v in self.values.iter_mut() {
            if let ValueStatus::Unmarked(val) = v {
                if val == value {
                    *v = ValueStatus::Marked(*val);
                } else {
                    ();
                }
            }
        }
    }

    fn evaluate(&self) -> Option<i32> {
        //
        //  0, 1, 2, 3, 4
        //  5, 6, 7, 8, 9
        // 10,11,12,13,14
        // 15,16,17,18,19
        // 20,21,22,23,24
        let mut unmarked = Vec::new();

        for x in 0..5 {
            if (self.check_row(x) || self.check_col(x)) {
                self.values.iter().for_each(|el| match el {
                    ValueStatus::Unmarked(num) => unmarked.push(num),
                    _ => (),
                });
                break;
            }
        }
        if unmarked.is_empty() {
            return None;
        } else {
            let sum = unmarked.into_iter().sum();
            return Some(sum);
        }
    }
    fn check_row(&self, row_index: i32) -> bool {
        let mut marked = Vec::new();
        let start_idx = row_index * 5;
        let end_idx = start_idx + 5;

        for pos in start_idx..end_idx {
            match self.values[pos as usize] {
                ValueStatus::Marked(num) => marked.push(num),
                ValueStatus::Unmarked(_) => marked.clear(),
            };
        }
        if marked.len() == 5 {
            return true;
        } else {
            return false;
        }
    }

    fn check_col(&self, col_index: i32) -> bool {
        let mut marked = Vec::new();
        let start_idx = col_index;
        let end_idx = start_idx + 4 * 5;

        for pos in (start_idx..=end_idx).step_by(5) {
            match self.values[pos as usize] {
                ValueStatus::Marked(num) => marked.push(num),
                ValueStatus::Unmarked(_) => break,
            };
        }
        if marked.len() == 5 {
            return true;
        } else {
            return false;
        }
    }
}

#[derive(Debug, Clone)]
struct Game {
    boards: Vec<Board>,
    drawn_nums: Vec<i32>,
}
impl Game {
    pub fn new(bs: Vec<Board>) -> Self {
        Game {
            boards: bs,
            drawn_nums: Vec::new(),
        }
    }
    pub fn draw(&mut self, number: i32) {
        self.drawn_nums.push(number);

        self.boards
            .iter_mut()
            .for_each(|board| board.mark_value(&number))
    }

    fn check_winner(&mut self) -> Option<Vec<i32>> {
        let mut winners: Vec<i32> = Vec::new();
        let mut remove_idx = Vec::new();
        if self.boards.len() == 0 {
            return None;
        }
        for (idx, board) in self.boards.iter().enumerate() {
            let b = board.evaluate();
            match b {
                Some(sum) => {
                    remove_idx.push(idx);
                    winners.push(sum);
                }
                None => {}
            };
        }
        if remove_idx.len() > 0 {
            for winner_idx in remove_idx.iter().rev() {
                self.boards.remove(*winner_idx);
            }
        }
        return Some(winners);
    }
}

fn parse_input_new(input: &str) -> (Vec<Vec<i32>>, Vec<i32>) {
    let mut lines = input.lines();
    let drawn_numbers: Vec<i32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let all_rows: Vec<Vec<i32>> = lines
        .map(|x| x.split_whitespace().map(|y| y.parse().unwrap()).collect())
        .collect::<Vec<Vec<i32>>>()
        .into_iter()
        .filter(|x| !x.is_empty())
        .collect();

    (all_rows, drawn_numbers)
}

fn main() {
    let input = include_str!("input");
    let (parsed_rows, drawn_values) = parse_input_new(input);

    let mut board_vec = Vec::new();
    let mut boards = Vec::new();

    for i in parsed_rows {
        board_vec.push(i);
        if board_vec.len() == 5 {
            boards.push(Board::new(&mut board_vec));
            board_vec.clear();
        }
    }
    let mut game = Game::new(boards);
    for (i, number) in drawn_values.into_iter().enumerate() {
        game.draw(number);
        //game.print_boards();
        let winners = game.check_winner();
        match winners {
            Some(ws) => {
                for winner in ws {
                    println!(
                        " sum: {}, drawn {}, solution: {}",
                        winner,
                        number,
                        winner * number
                    );
                }
            }
            None => break,
        }
    }
}
