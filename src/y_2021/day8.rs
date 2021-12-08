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
    let result = parse_input()
        .iter()
        .map(|(input, output)| {
            for permutation in permutations() {
                // test the input against the permutation and make sure it works
                let mut new_input = vec![];
                for item in input {
                    let x = item.chars().sorted().collect::<String>();
                    if !new_input.contains(&x) {
                        new_input.push(x);
                    }
                }
                let mut numbers_used = vec![];
                let mut fail = false;
                for item in new_input {
                    match item.len() {
                        2 => {
                            // 1 = [2, 5]
                            if !item.contains(permutation[2]) || !item.contains(permutation[5]) {
                                fail = true;
                                break;
                            }
                        }
                        3 => {
                            // 7 = [0, 2, 5]
                            if !item.contains(permutation[0])
                                || !item.contains(permutation[2])
                                || !item.contains(permutation[5])
                            {
                                fail = true;
                                break;
                            }
                        }
                        4 => {
                            // 4 = [1, 2, 3, 5]
                            if !item.contains(permutation[1])
                                || !item.contains(permutation[2])
                                || !item.contains(permutation[3])
                                || !item.contains(permutation[5])
                            {
                                fail = true;
                                break;
                            }
                        }
                        5 => {
                            // 2 = [0, 2, 3, 4, 6]
                            // 3 = [0, 2, 3, 5, 6]
                            // 5 = [0, 1, 3, 5, 6]
                            if item.contains(permutation[0])
                                && item.contains(permutation[2])
                                && item.contains(permutation[3])
                                && item.contains(permutation[4])
                                && item.contains(permutation[6])
                            {
                                if numbers_used.contains(&2) {
                                    fail = true;
                                    break;
                                }
                                numbers_used.push(2);
                            } else if item.contains(permutation[0])
                                && item.contains(permutation[2])
                                && item.contains(permutation[3])
                                && item.contains(permutation[5])
                                && item.contains(permutation[6])
                            {
                                if numbers_used.contains(&3) {
                                    fail = true;
                                    break;
                                }
                                numbers_used.push(3);
                            } else if item.contains(permutation[0])
                                && item.contains(permutation[1])
                                && item.contains(permutation[3])
                                && item.contains(permutation[5])
                                && item.contains(permutation[6])
                            {
                                if numbers_used.contains(&5) {
                                    fail = true;
                                    break;
                                }
                                numbers_used.push(5);
                            } else {
                                fail = true;
                                break;
                            }
                        }
                        6 => {
                            // 0 = [0, 1, 2, 4, 5, 6]
                            // 6 = [0, 1, 3, 4, 5, 6]
                            // 9 = [0, 1, 2, 3, 5, 6]
                            if item.contains(permutation[0])
                                && item.contains(permutation[1])
                                && item.contains(permutation[2])
                                && item.contains(permutation[4])
                                && item.contains(permutation[5])
                                && item.contains(permutation[6])
                            {
                                if numbers_used.contains(&0) {
                                    fail = true;
                                    break;
                                }
                                numbers_used.push(0);
                            } else if item.contains(permutation[0])
                                && item.contains(permutation[1])
                                && item.contains(permutation[3])
                                && item.contains(permutation[4])
                                && item.contains(permutation[5])
                                && item.contains(permutation[6])
                            {
                                if numbers_used.contains(&6) {
                                    fail = true;
                                    break;
                                }
                                numbers_used.push(6);
                            } else if item.contains(permutation[0])
                                && item.contains(permutation[1])
                                && item.contains(permutation[2])
                                && item.contains(permutation[3])
                                && item.contains(permutation[5])
                                && item.contains(permutation[6])
                            {
                                if numbers_used.contains(&9) {
                                    fail = true;
                                    break;
                                }
                                numbers_used.push(9);
                            } else {
                                fail = true;
                                break;
                            }
                        }
                        7 => (),
                        _ => unreachable!(),
                    }
                }
                if !fail {
                    return (permutation, output);
                }
            }
            unreachable!();
        })
        .map(|(permutation, input)| {
            fn find_digit(permutation: &[char], input: &str) -> char {
                match input.len() {
                    2 => '1',
                    3 => '7',
                    4 => '4',
                    5 => {
                        // 2 = [0, 2, 3, 4, 6]
                        // 3 = [0, 2, 3, 5, 6]
                        // 5 = [0, 1, 3, 5, 6]
                        if input.contains(permutation[0])
                            && input.contains(permutation[2])
                            && input.contains(permutation[3])
                            && input.contains(permutation[4])
                            && input.contains(permutation[6])
                        {
                            '2'
                        } else if input.contains(permutation[0])
                            && input.contains(permutation[2])
                            && input.contains(permutation[3])
                            && input.contains(permutation[5])
                            && input.contains(permutation[6])
                        {
                            '3'
                        } else if input.contains(permutation[0])
                            && input.contains(permutation[1])
                            && input.contains(permutation[3])
                            && input.contains(permutation[5])
                            && input.contains(permutation[6])
                        {
                            '5'
                        } else {
                            unreachable!()
                        }
                    }
                    6 => {
                        // 0 = [0, 1, 2, 4, 5, 6]
                        // 6 = [0, 1, 3, 4, 5, 6]
                        // 9 = [0, 1, 2, 3, 5, 6]
                        if input.contains(permutation[0])
                            && input.contains(permutation[1])
                            && input.contains(permutation[2])
                            && input.contains(permutation[4])
                            && input.contains(permutation[5])
                            && input.contains(permutation[6])
                        {
                            '0'
                        } else if input.contains(permutation[0])
                            && input.contains(permutation[1])
                            && input.contains(permutation[3])
                            && input.contains(permutation[4])
                            && input.contains(permutation[5])
                            && input.contains(permutation[6])
                        {
                            '6'
                        } else if input.contains(permutation[0])
                            && input.contains(permutation[1])
                            && input.contains(permutation[2])
                            && input.contains(permutation[3])
                            && input.contains(permutation[5])
                            && input.contains(permutation[6])
                        {
                            '9'
                        } else {
                            unreachable!()
                        }
                    }
                    7 => '8',
                    _ => unreachable!(),
                }
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
