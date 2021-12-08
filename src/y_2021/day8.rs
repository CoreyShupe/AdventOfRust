const INPUT: &str = include_str!("../../input/2021/day8.txt");

pub fn parse_input() -> Vec<(Vec<String>, Vec<String>)> {
    INPUT
        .lines()
        .map(|line| {
            let (left, right) = line.split_once('|').unwrap();
            (
                left.trim().split(' ').map(|x| x.to_string()).collect(),
                right.trim().split(' ').map(|x| x.to_string()).collect(),
            )
        })
        .collect()
}

pub fn sol1() {
    dbg!(parse_input()
        .iter()
        .map(|x| &x.1)
        .map(|x| x.iter().filter(|y| [2, 4, 3, 7].contains(&y.len())).count())
        .sum::<usize>());
}

use itertools::Itertools;

fn permutations() -> itertools::Permutations<std::ops::RangeInclusive<char>> {
    ('a'..='g').permutations(7)
}

pub fn sol2() {
    const MAGIC: u8 = 48;
    const MAPPING: [&[usize]; 10] = [
        &[0, 1, 2, 4, 5, 6],
        &[2, 5],
        &[0, 2, 3, 4, 6],
        &[0, 2, 3, 5, 6],
        &[1, 2, 3, 5],
        &[0, 1, 3, 5, 6],
        &[0, 1, 3, 4, 5, 6],
        &[0, 2, 5],
        &[0, 1, 2, 3, 4, 5, 6],
        &[0, 1, 2, 3, 5, 6],
    ];

    fn require_permute(uses: &[usize], permutation: &[char], item: &str) -> bool {
        uses.iter().all(|x| item.contains(permutation[*x]))
    }

    fn find_mapping(mapping_range: &[usize], permutation: &[char], item: &str) -> Option<u8> {
        mapping_range
            .iter()
            .find(|local| require_permute(MAPPING[**local], permutation, item))
            .copied()
            .map(|x| x as u8)
    }

    let result = parse_input()
        .iter()
        .map(|(input, output)| {
            'permute: for permutation in permutations() {
                // test the input against the permutation and make sure it works
                let mut new_input = vec![];
                for item in input {
                    let x = item.chars().sorted().collect::<String>();
                    if !new_input.contains(&x) {
                        new_input.push(x);
                    }
                }
                let mut numbers_used = vec![];
                for item in new_input {
                    match item.len() {
                        2 if !require_permute(MAPPING[1], &permutation, &item) => continue 'permute,
                        3 if !require_permute(MAPPING[7], &permutation, &item) => continue 'permute,
                        4 if !require_permute(MAPPING[4], &permutation, &item) => continue 'permute,
                        5 => match find_mapping(&[2, 3, 5], &permutation, &item) {
                            Some(idx) if !numbers_used.contains(&idx) => numbers_used.push(idx),
                            _ => continue 'permute,
                        },
                        6 => match find_mapping(&[0, 6, 9], &permutation, &item) {
                            Some(idx) if !numbers_used.contains(&idx) => numbers_used.push(idx),
                            _ => continue 'permute,
                        },
                        _ => (),
                    }
                }
                return (permutation, output);
            }
            unreachable!();
        })
        .map(|(permutation, input)| {
            fn find_digit(permutation: &[char], input: &str) -> char {
                (match input.len() {
                    2 => 1u8,
                    3 => 7u8,
                    4 => 4u8,
                    5 => find_mapping(&[2, 3, 5], permutation, input).unwrap(),
                    6 => find_mapping(&[0, 6, 9], permutation, input).unwrap(),
                    7 => 8u8,
                    _ => unreachable!(),
                } + MAGIC) as char
            }
            input
                .iter()
                .map(|item| find_digit(&permutation, item))
                .collect::<String>()
                .parse::<i32>()
                .unwrap()
        })
        .sum::<i32>();
    dbg!(result);
}
