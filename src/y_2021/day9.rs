const INPUT: &str = include_str!("../../input/2021/day9.txt");

fn parse_input() -> Vec<Vec<u8>> {
    INPUT
        .lines()
        .map(|x| x.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

const LIMS: usize = 99usize;

fn is_lowpoint(point: (usize, usize), grid: &[Vec<u8>]) -> bool {
    let x = grid[point.0][point.1];
    (point.0 == 0 || x < grid[point.0 - 1][point.1])
        && (point.1 == 0 || x < grid[point.0][point.1 - 1])
        && (point.0 == LIMS || x < grid[point.0 + 1][point.1])
        && (point.1 == LIMS || x < grid[point.0][point.1 + 1])
}

pub fn sol1() {
    let input = parse_input();

    let result = (0..100)
        .map(|i| {
            (0..100)
                .map(move |j| (i, j))
                .filter(|(i, j)| is_lowpoint((*i, *j), &input))
        })
        .flatten()
        .map(|(i, j)| input[i][j] as u32 + 1)
        .sum::<u32>();

    dbg!(result);
}

use itertools::Itertools;

pub fn sol2() {
    let input = parse_input();

    fn expand(
        point: (usize, usize),
        grid: &[Vec<u8>],
        mut points: Vec<(usize, usize)>,
    ) -> Vec<(usize, usize)> {
        if points.contains(&point) || grid[point.0][point.1] == 9 {
            return points;
        }
        points.push(point);
        if point.0 != 0 {
            points = expand((point.0 - 1, point.1), grid, points);
        }
        if point.1 != 0 {
            points = expand((point.0, point.1 - 1), grid, points);
        }
        if point.0 != LIMS {
            points = expand((point.0 + 1, point.1), grid, points);
        }
        if point.1 != LIMS {
            points = expand((point.0, point.1 + 1), grid, points);
        }
        points
    }
    let result = (0..100)
        .map(|i| {
            (0..100)
                .map(move |j| (i, j))
                .filter(|(i, j)| is_lowpoint((*i, *j), &input))
        })
        .flatten()
        .into_iter()
        .map(|point| expand(point, &input, Vec::new()).len())
        .sorted()
        .rev()
        .take(3)
        .product::<usize>();
    dbg!(result);
}
