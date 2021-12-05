const INPUT: &str = include_str!("../../input/2021/day5.txt");

#[derive(Debug)]
struct Vent {
    x: usize,
    y: usize,
}

fn parse() -> Vec<(Vent, Vent)> {
    INPUT
        .lines()
        .map(|x| serde_scan::scan!("{},{} -> {},{}" <- x).unwrap())
        .map(|((x1, y1), (x2, y2))| (Vent { x: x1, y: y1 }, Vent { x: x2, y: y2 }))
        .collect()
}

fn mod_strip(grid: &mut Vec<Vec<i32>>, from: &Vent, to: &Vent) {
    grid.iter_mut()
        .take(std::cmp::max(from.x, to.x) + 1)
        .skip(std::cmp::min(from.x, to.x))
        .for_each(|inner| {
            inner
                .iter_mut()
                .take(std::cmp::max(from.y, to.y) + 1)
                .skip(std::cmp::min(from.y, to.y))
                .for_each(|y| *y += 1);
        });
}

pub fn sol1() {
    let mut grid = vec![vec![0; 1000]; 1000];
    parse()
        .iter()
        .filter(|(from, to)| from.x == to.x || from.y == to.y)
        .for_each(|(from, to)| mod_strip(&mut grid, from, to));
    dbg!(grid.iter().flatten().filter(|x| x >= &&2).count());
}

pub fn sol2() {
    let mut grid = vec![vec![0; 1000]; 1000];
    parse().iter().for_each(|(from, to)| {
        if from.x == to.x || from.y == to.y {
            mod_strip(&mut grid, from, to);
        } else {
            let mut incher = (from.x, from.y);
            grid[to.x][to.y] += 1;
            while incher != (to.x, to.y) {
                grid[incher.0][incher.1] += 1;
                if incher.0 > to.x {
                    incher.0 -= 1;
                } else {
                    incher.0 += 1;
                }
                if incher.1 > to.y {
                    incher.1 -= 1;
                } else {
                    incher.1 += 1;
                }
            }
        }
    });
    dbg!(grid.iter().flatten().filter(|x| x >= &&2).count());
}

struct PaddedIter<X: Copy, I: Iterator<Item = X>> {
    iter1: I,
    iter2: I,
    _last_x: Option<X>,
    _last_y: Option<X>,
}

impl<X: Copy, I: Iterator<Item = X>> PaddedIter<X, I> {
    pub fn new(iter1: I, iter2: I) -> Self {
        Self {
            iter1,
            iter2,
            _last_x: None,
            _last_y: None,
        }
    }
}

impl<X: Copy, I: Iterator<Item = X>> std::iter::Iterator for PaddedIter<X, I> {
    type Item = (X, X);

    fn next(&mut self) -> Option<Self::Item> {
        let item = (self.iter1.next(), self.iter2.next());
        match item {
            (Some(x), Some(y)) => {
                self._last_x = Some(x);
                self._last_y = Some(y);
                Some((x, y))
            }
            (None, Some(y)) => {
                self._last_y = Some(y);
                Some((self._last_x.expect("Iter1 is empty."), y))
            }
            (Some(x), None) => {
                self._last_x = Some(x);
                Some((x, self._last_y.expect("Iter 2 is empty.")))
            }
            (None, None) => None,
        }
    }
}

pub fn padded_solutions_1_pass() {
    let mut grid_part_1 = vec![vec![0u8; 1000]; 1000];
    let mut grid_part_2 = vec![vec![0u8; 1000]; 1000];

    use std::cmp::{max, min};

    parse().iter().for_each(|(from, to)| {
        let row_iter = if from.x <= to.x || from.y == to.y {
            min(from.x, to.x) as isize..=max(from.x, to.x) as isize
        } else {
            -(from.x as isize)..=-(to.x as isize)
        };

        let col_iter = if from.y <= to.y || from.x == to.x {
            min(from.y, to.y) as isize..=max(from.y, to.y) as isize
        } else {
            -(from.y as isize)..=-(to.y as isize)
        };

        let true_iter = PaddedIter::new(row_iter, col_iter);

        let adder = |grid: &mut Vec<Vec<u8>>, (a, b): (isize, isize)| {
            grid[a.abs() as usize][b.abs() as usize] += 1
        };

        let part_1_included = from.x == to.x || from.y == to.y;
        for vent in true_iter {
            adder(&mut grid_part_2, vent);
            if part_1_included {
                adder(&mut grid_part_1, vent);
            }
        }
    });
    dbg!(grid_part_1.iter().flatten().filter(|x| x >= &&2).count());
    dbg!(grid_part_2.iter().flatten().filter(|x| x >= &&2).count());
}
