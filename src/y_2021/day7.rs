const INPUT: &str = include_str!("../../input/2021/day7.txt");

fn solve<F: Fn(&[i64], i64) -> i64>(determine_fuel_usage: F) {
    let vec: Vec<i64> = INPUT.split(',').map(|x| x.parse().unwrap()).collect();
    let minimum_usage = (0..*vec.iter().max().unwrap()).fold(i64::MAX, |acc, position| std::cmp::min(acc, determine_fuel_usage(&vec, position)));
    dbg!(minimum_usage);
}

pub fn sol1() {
    solve(|vec, position| vec.iter().map(|x| (x - position).abs()).sum());
}

pub fn sol2() {
    fn addative_factorial(num: i64) -> i64 {
        ((num * num) + num) / 2
    }

    solve(|vec, position| vec.iter().map(|x| addative_factorial((x - position).abs())).sum());
}