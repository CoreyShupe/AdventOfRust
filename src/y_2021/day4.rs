const INPUT: &str = include_str!("../../input/2021/day4.txt");

struct Piece {
    marked: bool,
    value: u8,
}

impl Piece {
    pub fn mark(&mut self) {
        self.marked = true;
    }
}

struct BingoBoard {
    id: u8,
    rows: Vec<Vec<Piece>>,
}

impl BingoBoard {
    fn has_win(&self) -> bool {
        let mut cols = vec![0; 5];

        for col in &self.rows {
            if col.iter().all(|f| f.marked) {
                return true;
            }

            for i in 0..5 {
                if col[i].marked {
                    cols[i] += 1;
                    if cols[i] == 5 {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn sum(&self) -> i32 {
        let simple_sum = self
            .rows
            .iter()
            .flatten()
            .filter(|x| !x.marked)
            .fold(0, |acc, x: &Piece| acc + x.value as i32);

        simple_sum
    }

    fn mark(&mut self, number: u8) {
        for col in self.rows.iter_mut() {
            for x in col.iter_mut() {
                if x.value == number {
                    x.mark();
                }
            }
        }
    }
}

fn parse_input() -> (Vec<u8>, Vec<BingoBoard>) {
    let mut lines = INPUT.lines();
    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    fn create_boards<I: Iterator<Item = &'static str>>(
        mut iter: I,
        mut vec: Vec<BingoBoard>,
        id: u8,
    ) -> Vec<BingoBoard> {
        if iter.next().is_none() {
            return vec;
        }

        let pieces = vec![
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        ]
        .into_iter()
        .map(|x| {
            x.replace("  ", " ")
                .trim()
                .split(' ')
                .map(|y| y.parse::<u8>().unwrap())
                .map(|uu| Piece {
                    marked: false,
                    value: uu,
                })
                .collect::<Vec<Piece>>()
        })
        .collect::<Vec<Vec<Piece>>>();

        vec.push(BingoBoard { rows: pieces, id });

        create_boards(iter, vec, id + 1)
    }

    (numbers, create_boards(lines, vec![], 0))
}

pub fn sol1() {
    let (numbers, mut boards) = parse_input();

    for x in numbers {
        for board in boards.iter_mut() {
            board.mark(x);
            if board.has_win() {
                dbg!(x as i32 * board.sum());
                return;
            }
        }
    }
}

pub fn sol2() {
    let (numbers, mut boards) = parse_input();

    let board_count = boards.len();
    let mut win_count = 0;
    let mut won = Vec::new();

    for x in numbers {
        for board in boards.iter_mut() {
            board.mark(x);
            if board.has_win() && !won.contains(&board.id) {
                if win_count == board_count - 1 {
                    dbg!(x as i32 * board.sum());
                    return;
                } else {
                    win_count += 1;
                    won.push(board.id);
                }
            }
        }
    }
}
